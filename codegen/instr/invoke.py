"""
invoke 指令生成器：invokespecial / invokestatic / invokevirtual / invokedynamic(string concat)。
"""

import re
from ..stack import StackSim
from ..rs_ir import (
    Lit, Var, RawExpr, RawStmt, NewPendingExpr, RsNamed,
)
from ..render import render_expr, render_type
from ..type_map import (
    jvm_to_rust, short_cls, parse_descriptor_params, is_jdk,
    parse_class_type_params as _parse_class_type_params,
    parse_method_param_types as _parse_method_param_types,
)
from ..constants import safe_ident as _safe_field
from .coerce import (
    parse_method_ref, _coerce_from_null, _coerce_to_object,
    _coerce_to_interface, _coerce_value, _find_super_chain_to_class,
    _find_method_super_prefix, _find_method_super_prefix_for_type,
    _super_prefix_to_expr, _resolve_method_owner,
    _mangle_if_overloaded, _class_known, _is_subtype, _rust_type_to_binary,
    _get_all_subtypes_ordered,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL, _PRIMITIVE_RUST_TYPES,
    _JAVA_RUNTIME_SHORT_NAMES,
)


def _lookup_method_sig_params(
    cls_short: str | None,
    mname: str,
    descriptor_params: list[str],
    descriptor_ret: str,
    registry: dict | None,
    caller_class_type_params: frozenset[str],
) -> list[str | None] | None:
    """查找被调用方法的 generic_signature，返回真实参数类型列表。

    返回值中 None 表示该位置降级到 jvm_to_rust(descriptor)。
    只有当 callee 的类型参数在 caller 的类型参数集合中可见时，才保留泛型参数名；
    否则（调用方用 raw/擦除类型），该位置设为 None，让调用方降级到 descriptor 推导类型。
    """
    if not cls_short or not registry:
        return None
    cls_bin = _rust_type_to_binary(cls_short, registry)
    if not cls_bin:
        return None
    ci = registry.get(cls_bin)
    if not ci:
        return None
    full_desc = '(' + ''.join(descriptor_params) + ')' + descriptor_ret
    for m in ci.methods:
        if m.name == mname and m.descriptor == full_desc:
            if not m.generic_signature:
                return None
            # 用被调用类的类型参数解析 generic_signature
            callee_tparams_list = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
            callee_tparams = frozenset(callee_tparams_list)
            types, _ = _parse_method_param_types(m.generic_signature, callee_tparams_list, registry)
            if not types:
                return None
            # 将 callee 类型参数映射到 caller 上下文：
            # 若某参数是 callee 的类型参数但不在 caller 的类型参数集合中，
            # 说明 caller 用的是擦除类型（如 raw ArrayList），该位置设 None（降级到 descriptor）
            resolved: list[str | None] = []
            for t in types:
                if t in callee_tparams and t not in caller_class_type_params:
                    resolved.append(None)   # 擦除，使用 jvm_to_rust(descriptor) 降级
                else:
                    resolved.append(t)
            # registry 中所有接口的短名在调用点降级为 Object（Arch-1 接口 = Object
            # 类型别名，泛型形态 X<...> 不是合法 Rust 类型）
            import re as _re_iface
            _reg_iface_shorts = _registry_iface_shorts(registry)
            final_resolved: list[str | None] = []
            for t in resolved:
                if t is not None:
                    _m = _re_iface.match(r'^(\w+)(?:<|$)', t)
                    if _m and _m.group(1) in _reg_iface_shorts:
                        final_resolved.append(None)
                        continue
                final_resolved.append(t)
            return final_resolved
    return None


_iface_shorts_cache: dict[int, frozenset[str]] = {}


def _registry_iface_shorts(registry: dict | None) -> frozenset[str]:
    """registry 中所有接口的 Rust 短名（含 $→_ 替换），按 id(registry) 缓存。"""
    if not registry:
        return frozenset()
    _key = id(registry)
    _cached = _iface_shorts_cache.get(_key)
    if _cached is not None:
        return _cached
    _shorts = frozenset(
        _bin.rsplit('/', 1)[-1].replace('$', '_')
        for _bin, _ci in registry.items()
        if getattr(_ci, 'is_interface', False)
    )
    _iface_shorts_cache[_key] = _shorts
    return _shorts


_concrete_shorts_cache: dict[int, frozenset[str]] = {}


def _concrete_class_shorts(registry: dict | None) -> frozenset[str]:
    """registry 中所有非接口类 + java_runtime 手写类的 Rust 短名，按 id(registry) 缓存。"""
    if not registry:
        return frozenset(_JAVA_RUNTIME_SHORT_NAMES)
    _key = id(registry)
    _cached = _concrete_shorts_cache.get(_key)
    if _cached is not None:
        return _cached
    _shorts = frozenset(
        _bin.rsplit('/', 1)[-1].replace('$', '_')
        for _bin, _ci in registry.items()
        if not getattr(_ci, 'is_interface', False)
    ) | frozenset(_JAVA_RUNTIME_SHORT_NAMES)
    _concrete_shorts_cache[_key] = _shorts
    return _shorts


def _downcast_target_valid(expected: str, sim: 'StackSim', registry: dict | None) -> bool:
    """downcast 目标类型合法性：类型串中所有标识符须为具体类（非接口）、
    类级类型参数或内建容器名。接口名（List/Consumer）的泛型形态与
    方法级类型变量（caller 不可见的 T）不能作为 downcast::<T>() 目标。"""
    import re as _re_t
    _names = _re_t.findall(r'[A-Za-z_][A-Za-z0-9_]*', expected)
    if not _names:
        return False
    _ok = set(sim.class_type_params or ()) | set(_concrete_class_shorts(registry))
    _ok |= {'Object', 'String', 'Class', 'Rc', 'Vec', 'RefCell', 'Option'}
    return all(_n in _ok for _n in _names)


def _lookup_method_sig_ret(
    cls: str | None,
    mname: str,
    descriptor_params: list[str],
    descriptor_ret: str,
    registry: dict | None,
    caller_class: str | None = None,
    caller_tparams=None,
    receiver_type: str | None = None,
) -> str | None:
    """查找被调用方法 generic_signature 解析出的「真实」Rust 返回类型。

    方法签名生成（gen_method_body）在返回类型上优先使用 generic_signature
    （如 Class.elementType 的 ()Ljava/lang/Class<*>; → Class<Object>），
    而调用点此前用 jvm_to_rust(descriptor)（Ljava/lang/Class; → Object），
    两侧不一致导致 let 无标注时 Rust 推断出具体类型、sim 却记录 Object。

    返回规则：
    - sig_ret 无类型变量（如 Class<Object>、Vec<Class<Object>>）→ 直接返回
      （任何接收者实例化下表达式类型一致）
    - sig_ret 含 callee 类型变量（如 Class_ReflectionData<T>）：
      - callee 类 == 调用方当前类且变量可见 → 返回（impl 内同名参数）
      - 接收者是 callee 的参数化形态（SoftReference<X>.get() → T）→
        按接收者实参替换类型变量（Rust 泛型单态化的静态镜像）
    - 其余（跨类 + 含类型变量 + 接收者擦除）→ None，调用方降级到擦除类型
    """
    if not cls or not registry:
        return None
    cls_bin = cls if '/' in cls else (_rust_type_to_binary(cls, registry) or cls)
    ci = registry.get(cls_bin)
    if not ci:
        return None
    full_desc = '(' + ''.join(descriptor_params) + ')' + descriptor_ret
    for m in ci.methods:
        if m.name == mname and m.descriptor == full_desc:
            if not m.generic_signature:
                return None
            callee_tparams = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
            if not callee_tparams:
                return None
            _, sig_ret = _parse_method_param_types(m.generic_signature, callee_tparams, registry)
            if not sig_ret:
                return None
            # 有效性：所有标识符须为已知类型（与 gen_method_body 的 _sig_param_valid 同规则）
            import re as _re_v
            _builtin = frozenset({
                'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
                'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell', 'usize', 'u8',
            })
            _reg_shorts = {k.rsplit('/', 1)[-1].replace('$', '_') for k in registry}
            for name in _re_v.findall(r'[A-Za-z_][A-Za-z0-9_]*', sig_ret):
                if name in _builtin or name in callee_tparams or name in _reg_shorts:
                    continue
                return None
            # 类型变量可见性：sig_ret 引用了 callee 的类型参数时，
            # 仅当 callee == 调用方当前类且变量名一致（同类泛型参数）才安全
            _used_vars = set(_re_v.findall(r'[A-Za-z_][A-Za-z0-9_]*', sig_ret)) & set(callee_tparams)
            if _used_vars:
                if (caller_class and caller_tparams
                        and cls_bin == caller_class
                        and _used_vars <= set(caller_tparams)):
                    return sig_ret
                # 跨类：接收者是 callee 的参数化形态 → 按接收者实参替换
                if receiver_type:
                    _recv_base = receiver_type.split('<')[0].strip()
                    _callee_short = short_cls(cls_bin)
                    if (_recv_base == _callee_short and '<' in receiver_type
                            and receiver_type.endswith('>')):
                        _inner = receiver_type[len(_recv_base) + 1:receiver_type.rfind('>')]
                        _rargs = _split_type_args(_inner)
                        if len(_rargs) == len(callee_tparams):
                            _sub = _substitute_tvars(sig_ret, callee_tparams, _rargs)
                            _caller_set = set(caller_tparams) if caller_tparams else set()
                            if all(_n in _builtin or _n in _reg_shorts or _n in _caller_set
                                   for _n in _re_v.findall(r'[A-Za-z_][A-Za-z0-9_]*', _sub)):
                                return _sub
                return None
            return sig_ret
    return None


def _coerce_arg(
    e: str,
    e_ty_node: object,
    expected: str,
    actual: str,
    sim: 'StackSim',
    registry: dict | None,
) -> str:
    """统一参数强制转换逻辑（替代各 _gen_invoke* 中的重复 elif 链）。

    expected: 期望类型（来自 generic_signature 或 descriptor）
    actual:   实际栈顶类型字符串
    """
    from ..render import render_type as _rt
    from .coerce import (
        _coerce_from_null, _coerce_to_object, _coerce_to_interface,
        _coerce_value, _is_subtype, _PRIMITIVE_RUST_TYPES, _into_super_chain,
    )
    null_coerce = _coerce_from_null(e, expected)
    if null_coerce is not None:
        return null_coerce
    if _coerce_to_interface(actual, expected):
        return 'Default::default()'
    if expected == 'Object' and actual not in ('Object', '()'):
        # 泛型参数值（如 K: Clone + Default + 'static）传给 Object 参数：
        # Rust 无隐式子类型化，K 类型的值不能直接当 Object 用 → 装箱为
        # Object(Rc<JvmRef<K>>)，callee 内 downcast::<K>() 可还原。
        if actual in sim.class_type_params:
            return f"Object::from_any(Clone::clone(&{e}))"
        if e == 'this':
            # 构造器（fn new）里没有 self 关键字，统一用局部变量 this
            # （实例方法里 let this = self;，两者均可见）
            return f"Object::from_any(Clone::clone(this))"
        return _coerce_to_object(e, actual)
    if expected in ('bool', 'i8', 'i16', 'u16') and actual != expected:
        return _coerce_value(e, e_ty_node, expected)
    if expected == 'i32' and actual in ('i8', 'i16', 'u16', 'bool'):
        return f"({e} as i32)"
    if (expected not in _PRIMITIVE_RUST_TYPES and actual not in _PRIMITIVE_RUST_TYPES
            and expected not in ('Object', '()', actual)
            and _is_subtype(actual.split('<')[0], expected.split('<')[0], registry)):
        # R-2：子类传给父类参数，通过显式 __into_super() 链（替代已删除的 T55 From impl）
        chain = _into_super_chain(actual.split('<')[0], expected.split('<')[0], registry)
        return f"Clone::clone(&{e}){chain}"
    # Fix 18：actual 是 Object（运行时多态值）而 expected 是具体引用类型 ——
    # Java 调用点隐式 checkcast 语义 → downcast（运行时校验，不符则 panic）。
    # 覆盖「callee 签名参数是精确泛型形态而调用方局部变量被擦除为 Object」
    # 的场景（如 rotateLeft(root: TreeNode<K,V>) 传入 Object 局部变量）。
    # 目标类型须为具体类（非接口别名）/ 类级类型参数 / 内建容器，接口名
    # （List<..>、Consumer<T>）与方法级类型变量不是合法 downcast 目标。
    if (expected not in _PRIMITIVE_RUST_TYPES and actual == 'Object'
            and expected not in ('Object', '()')
            and expected not in (sim.class_type_params or ())
            and _downcast_target_valid(expected, sim, registry)):
        return f"({e}).downcast::<{expected}>()"
    if actual not in _PRIMITIVE_RUST_TYPES:
        # `this` 在 Rust 中是 &Self 引用，Clone::clone(this) 得到 Self，无需多余 &
        if e == 'this':
            return f"Clone::clone({e})"
        return f"Clone::clone(&{e})"
    return e


def _gen_string_concat(sim: StackSim, comment: str):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for p in reversed(params):
        e_expr, _ = sim.pop()
        raw = render_expr(e_expr)
        # Java 浮点数格式化：整数值需显示 .0（如 5.0 而非 5）
        if p in ('D',):
            raw = f'java_fmt_f64({raw})'
        elif p in ('F',):
            raw = f'java_fmt_f32({raw})'
        elif p in ('C',):
            # Java char (u16) 必须转为 Rust char 才能以字符形式格式化
            raw = f"char::from_u32({raw} as u32).unwrap_or('?')"
        args.insert(0, raw)

    tmpl_m = re.search(r' template:(.+)$', comment)
    if tmpl_m:
        template = tmpl_m.group(1)
        parts = template.split('\x01')
        if len(parts) == len(args) + 1:
            fmt_str = ''.join(
                (p.replace('{', '{{').replace('}', '}}') + '{}' if i < len(args)
                 else p.replace('{', '{{').replace('}', '}}'))
                for i, p in enumerate(parts)
            )
            fmt_args = ', '.join(args)
            if fmt_args:
                # 用 from_owned 避免 From<&str> vs From<std::string::String> 歧义
                sim.push(Lit(f'String::from_owned(format!("{fmt_str}", {fmt_args}))'), RsNamed('String'))
            else:
                sim.push(Lit(f'String::from("{fmt_str}")'), RsNamed('String'))
            return

    # fallback
    if not args:
        sim.push(Lit('String::new()'), RsNamed('String'))
    elif len(args) == 1:
        sim.push(Lit(f'String::from_owned(format!("{{}}", {args[0]}))'), RsNamed('String'))
    else:
        fmt = '{}'.join([''] * (len(args) + 1))  # "{}{}{}" for 3 args
        fmt_args = ', '.join(args)
        sim.push(Lit(f'String::from_owned(format!("{fmt}", {fmt_args}))'), RsNamed('String'))


def _split_type_args(s: str) -> list[str]:
    """按顶层逗号切分泛型实参串（处理嵌套尖括号）。
    'K, HashMap_Node<K, V>' → ['K', 'HashMap_Node<K, V>']"""
    parts, depth, cur = [], 0, ''
    for ch in s:
        if ch == '<':
            depth += 1
        elif ch == '>':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    return parts


def _substitute_tvars(ty: str, tparams: list[str], targs: list[str]) -> str:
    """把 ty 中的类型变量标识符替换为对应实参（逐词替换，不动类名）。"""
    import re as _re_s
    mapping = dict(zip(tparams, targs))
    return _re_s.sub(
        r'[A-Za-z_][A-Za-z0-9_]*',
        lambda m: mapping.get(m.group(0), m.group(0)),
        ty,
    )


def _resolve_ctor_turbofish_args(
    full_cls: str,
    ctor_params: list[str],
    arg_tys: list[str],
    caller_class: str | None,
    sim: 'StackSim',
    registry: dict | None,
) -> list[str] | None:
    """推导泛型类构造器的 turbofish 实参（见 _gen_invokespecial 调用处注释）。

    返回实参列表（如 ['K', 'V'] / ['Class_ReflectionData<T>'] / ['Object']），
    类非泛型返回 None。"""
    if not registry:
        return None
    ci = registry.get(full_cls)
    if not ci:
        return None
    cls_tparams = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    if not cls_tparams:
        return None
    # 规则 1：构造器 generic_signature 的参数位置引用类型变量 → 用实参类型替换
    full_desc = '(' + ''.join(ctor_params) + ')V'
    ctor_sig_params = None
    for m in ci.methods:
        if m.name == '<init>' and m.descriptor == full_desc:
            if m.generic_signature:
                sp, _ = _parse_method_param_types(m.generic_signature, cls_tparams, registry)
                if sp and len(sp) == len(ctor_params):
                    ctor_sig_params = sp
            break
    if ctor_sig_params and arg_tys:
        subst: dict[str, str] = {}
        for si, sp_t in enumerate(ctor_sig_params):
            if (sp_t in cls_tparams and si < len(arg_tys)
                    and arg_tys[si] not in _PRIMITIVE_RUST_TYPES
                    and arg_tys[si] not in ('Object', '()')):
                subst[sp_t] = arg_tys[si]
        if len(subst) == len(cls_tparams):
            return [subst[t] for t in cls_tparams]
    # 规则 2：构造类是当前类 / 当前类的内部类，且类型参数名一致
    if caller_class and sim.class_type_params:
        if (full_cls == caller_class or full_cls.startswith(caller_class + '$')) \
                and set(cls_tparams) == set(sim.class_type_params):
            return list(cls_tparams)
    # 规则 3：兜底擦除
    return ['Object'] * len(cls_tparams)


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    if '<init>' not in comment and '"<init>"' not in comment:
        # super.method() 调用（invokespecial 非构造器）：
        # 不能用 invokevirtual 语义——否则 this.method() 会对被覆盖方法产生无限递归。
        # 必须精确路由到目标父类的 ._super 链，直接调用父类实现，跳过虚拟派发。
        cls_short, mname, params, ret = parse_method_ref(comment)
        sig_params = _lookup_method_sig_params(
            cls_short, mname, params, ret, registry, sim.class_type_params
        )
        args: list[str] = []
        for _idx, param_jvm in enumerate(reversed(params)):
            e_expr, e_ty_node = sim.pop()
            e_str = render_expr(e_expr)
            # 若有 generic_signature 参数类型（非 None），优先使用；否则降级到 descriptor
            _pi = len(params) - 1 - _idx
            _sig_t = sig_params[_pi] if sig_params and _pi < len(sig_params) else None
            expected_rust = _sig_t if _sig_t is not None else jvm_to_rust(param_jvm, registry)
            actual_rust = render_type(e_ty_node)
            e_str = _coerce_arg(e_str, e_ty_node, expected_rust, actual_rust, sim, registry)
            args.insert(0, e_str)
        obj_expr, _ = sim.pop()
        obj_e = render_expr(obj_expr)
        # 找到 super 链：class_name（当前类 binary）→ cls_short（目标父类 Rust 短名）
        # 前缀 `_super.` / `_super._super.` 统一翻译成 `__super()` 调用链（方案 §16）
        super_pfx = _find_super_chain_to_class(class_name, cls_short or '', registry) if registry else '_super.'
        recv_e = _super_prefix_to_expr(obj_e, super_pfx) if super_pfx else obj_e
        rust_mname = _safe_field(_mangle_if_overloaded(cls_short or '', mname, comment, registry))
        arg_str = ', '.join(args)
        rust_ret = jvm_to_rust(ret, registry)
        if rust_ret == '()':
            sim.emit(RawStmt(f"{recv_e}.{rust_mname}({arg_str})?;"))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f"let {v} = {recv_e}.{rust_mname}({arg_str})?;"))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    cls, _mname_ctor, params, _ret_ctor = parse_method_ref(comment)
    sig_params_ctor = _lookup_method_sig_params(
        cls, '<init>', params, 'V', registry, sim.class_type_params
    )
    args = []
    arg_tys = []
    for _idx_c, param_jvm in enumerate(reversed(params)):
        e_expr, e_ty_node = sim.pop()
        e = render_expr(e_expr)
        _pi_c = len(params) - 1 - _idx_c
        _sig_t_c = sig_params_ctor[_pi_c] if sig_params_ctor and _pi_c < len(sig_params_ctor) else None
        expected = _sig_t_c if _sig_t_c is not None else jvm_to_rust(param_jvm, registry)
        ty = render_type(e_ty_node)
        e = _coerce_arg(e, e_ty_node, expected, ty, sim, registry)
        args.insert(0, e)
        arg_tys.insert(0, ty)
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        full_cls = obj_expr.class_name          # e.g. 'java/util/ArrayList'
        raw_cls = full_cls.rsplit('/', 1)[-1]
        raw_cls = short_cls(raw_cls) or raw_cls

        if '/' in full_cls:
            # JDK class（含包路径）→ 用 new() 工厂（@synthetic）
            rust_ty_str = jvm_to_rust(f'L{full_cls};', registry)
            # 自动装箱优化：原始包装类型（Integer→i32等）直接用值，跳过构造器调用
            _PRIM_TYPES = frozenset({'i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'})
            if rust_ty_str in _PRIM_TYPES:
                init_expr = args[0] if args else '0'
                rust_ty = rust_ty_str
                rust_ty_node = RsNamed(rust_ty_str)
            elif rust_ty_str != 'Object' and '<' in rust_ty_str:
                # 泛型类构造：turbofish 实参的推导优先级
                #   1. 构造器 generic_signature 参数引用类型变量（如
                #      SoftReference(T)）→ 用对应实参的 sim 类型替换
                #      （Java 钻石推断的静态近似）
                #   2. 构造类是当前类/当前类的内部类，且类型参数名一致
                #      （如 Class<T> 内构造 Class$ReflectionData<T>）→ 用当前
                #      impl 的类型参数
                #   3. 兜底：Object（擦除）
                _ctor_tparams = _resolve_ctor_turbofish_args(
                    full_cls, params, arg_tys, class_name, sim, registry)
                type_params_str = ('<' + ', '.join(_ctor_tparams) + '>') if _ctor_tparams else ''
                rust_ty = raw_cls + type_params_str
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                turbofish = '::' + type_params_str
                init_expr = f"{raw_cls}{turbofish}::{ctor_name}({', '.join(args)})?"
            else:
                type_params_str = ''
                rust_ty = raw_cls
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                if args:
                    init_expr = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
                else:
                    init_expr = f"{raw_cls}::{ctor_name}()?"
        elif raw_cls and '/' not in raw_cls:
            # 用户类：new()? 返回 Result<Self>，同样 mangle 重载构造器
            _init_mangled2 = _mangle_if_overloaded(raw_cls, '<init>', comment, registry)
            ctor_name    = _safe_field(_init_mangled2.replace('<init>', 'new'))
            init_expr    = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)
        else:
            init_expr    = f"/* {raw_cls}::new() */"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)

        if sim.stack and isinstance(sim.stack[-1][0], NewPendingExpr):
            sim.stack[-1] = (RawExpr(init_expr), rust_ty_node)
        else:
            v = sim.fresh('_obj')
            sim.emit(RawStmt(f"let mut {v}: {rust_ty} = {init_expr};"))
            sim.push(Var(v), rust_ty_node)
    else:
        obj_e = render_expr(obj_expr)
        obj_e = render_expr(obj_expr)
        if obj_e in ('this', 'self') and cls:
            # super(args) 调用：在子类构造器中初始化 _super 字段
            # java/lang/Object 的 super() 是 no-op（Rust 不需要 Object 初始化）
            raw_cls = cls.rsplit('/', 1)[-1]
            raw_cls_rust = short_cls(raw_cls.replace('$', '_')) or raw_cls.replace('$', '_')
            if raw_cls_rust in ('Object',) or cls in ('java/lang/Object',):
                sim.emit(RawStmt(f"/* invokespecial {comment} (Object no-op) */"))
            else:
                _init_mangled = _mangle_if_overloaded(cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                arg_str = ', '.join(args)
                ctor_call = f"{raw_cls_rust}::{ctor_name}({arg_str})"
                super_pfx = _find_super_chain_to_class(class_name, raw_cls_rust, registry) if registry else '_super.'
                if super_pfx:
                    # super(...)：以已构造好的父类值重建 this（宏的 __new_with_super）。
                    # JVM 校验器保证 <init> 的 invokespecial 只指向直接父类或同类，
                    # 所以这里恒为 1 层，不需要按层数拼 _super 路径。
                    sim.emit(RawStmt(f"this = Self::__new_with_super({ctor_call}?);"))
                else:
                    # 同类构造器委托 this(args)：直接替换 this（初始占位值丢弃）
                    sim.emit(RawStmt(f"this = {ctor_call}?;"))
        else:
            sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    for skip in BOXING_SKIP_STATIC:
        if skip in comment:
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a_expr, _ = sim.pop()
        a = render_expr(a_expr)
        sim.push(Lit(f'String::from_owned(format!("{{}}", {a}))'), RsNamed('String'))
        return

    cls, mname, params, ret = parse_method_ref(comment)
    sig_params_s = _lookup_method_sig_params(
        cls, mname, params, ret, registry, sim.class_type_params
    )
    args = []
    for _idx_s, param_jvm in enumerate(reversed(params)):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        _pi_s = len(params) - 1 - _idx_s
        _sig_t_s = sig_params_s[_pi_s] if sig_params_s and _pi_s < len(sig_params_s) else None
        expected = _sig_t_s if _sig_t_s is not None else jvm_to_rust(param_jvm, registry)
        e = _coerce_arg(e, ty_node, expected, ty, sim, registry)
        args.insert(0, e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）

    # 目标类不在 registry（被截断的内部类如 jdk.internal.*）→ 生成 panic 存根
    if cls and not _class_known(cls, registry):
        rust_ret = jvm_to_rust(ret, registry)
        stub_msg = f"stub: {cls}.{mname}"
        if rust_ret == '()':
            sim.emit(RawStmt(f'panic!("{stub_msg}");'))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f'let {v}: {rust_ret} = panic!("{stub_msg}");'))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    if cls is None or cls == class_name:
        rust_mname = _safe_field(_mangle_if_overloaded(class_name, mname, comment, registry))
        call = f"Self::{rust_mname}({', '.join(args)})"
        needs_q = True
    elif cls and '/' in cls:
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        rust_mname = _safe_field(_mangle_if_overloaded(cls, mname, comment, registry))
        # 泛型类静态方法需要 turbofish，避免 E0283 类型推断歧义
        turbofish = ''
        if registry:
            _cls_bin = _rust_type_to_binary(cls, registry)
            if _cls_bin:
                _cls_ci = registry.get(_cls_bin)
                if _cls_ci and _cls_ci.generic_signature:
                    _tparams = _parse_class_type_params(_cls_ci.generic_signature)
                    if _tparams:
                        if _cls_bin == class_name and sim.class_type_params:
                            # 同类静态调用（如 impl<K,V> TreeNode 内调用
                            # TreeNode::checkInvariants）：实参是精确泛型形态
                            # （HashMap_TreeNode<K,V>），turbofish 用当前 impl 的
                            # 类型参数；填 Object 会 E0308（expected X<K,V>,
                            # found X<Object,Object>）。
                            turbofish = '::<' + ', '.join(_tparams) + '>'
                        else:
                            turbofish = '::<' + ', '.join('Object' for _ in _tparams) + '>'
        call = f"{cls}{turbofish}::{rust_mname}({', '.join(args)})"
        needs_q = True

    q = '?' if needs_q else ''
    rust_ret = jvm_to_rust(ret, registry)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{call}{q};"))
    else:
        v = sim.fresh()
        # 签名真实返回（generic_signature）与擦除映射不一致时的对齐：
        # - 擦除映射是 Object（Class→Object 等）：包 from_any 保持 Object 记录
        #   （局部 hint/downcast 负责恢复具体类型）
        # - 擦除映射是擦除实例化（X<Object,Object>）：被调方法声明按
        #   generic_signature 生成（X<K,V> / X<Class<Object>>），调用点记录
        #   必须一致，否则局部标注擦除形态而表达式是精确形态，E0308
        _sig_ret_s = _lookup_method_sig_ret(
            cls, mname, params, ret, registry,
            caller_class=class_name, caller_tparams=sim.class_type_params,
        )
        if rust_ret == 'Object':
            if _sig_ret_s is not None and _sig_ret_s != 'Object':
                sim.emit(RawStmt(f"let {v} = Object::from_any({call}{q});"))
            else:
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
            sim.push(Var(v), RsNamed(rust_ret))
        elif (_sig_ret_s is not None and _sig_ret_s != rust_ret
                and rust_ret not in _PRIMITIVE_RUST_TYPES):
            sim.emit(RawStmt(f"let {v} = {call}{q};"))
            sim.push(Var(v), RsNamed(_sig_ret_s))
        else:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
            sim.push(Var(v), RsNamed(rust_ret))


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    sig_params_v = _lookup_method_sig_params(
        cls, mname, params, ret, registry, sim.class_type_params
    )
    args = []
    for _idx_v, param_jvm in enumerate(reversed(params)):
        e_expr, e_ty_node = sim.pop()
        e_str = render_expr(e_expr)
        _pi_v = len(params) - 1 - _idx_v
        _sig_t_v = sig_params_v[_pi_v] if sig_params_v and _pi_v < len(sig_params_v) else None
        expected_rust = _sig_t_v if _sig_t_v is not None else jvm_to_rust(param_jvm, registry)
        actual_rust = render_type(e_ty_node)
        e_str = _coerce_arg(e_str, e_ty_node, expected_rust, actual_rust, sim, registry)
        args.insert(0, e_str)
    obj_expr, obj_ty_node = sim.pop()
    obj_e = render_expr(obj_expr)
    obj_ty = render_type(obj_ty_node)

    # Fix 16：泛型参数接收者（如 k.equals(pk) 中 k: K）——inherent 方法不在
    # 类型参数上可见（E0599）。装箱为 Object 后：Object 自身的方法
    # （equals/hashCode/toString）直接调用 java_runtime 手写实现，避免
    # dispatch 链枚举 Object 的全部子类；其余方法走 obj_is_bare 的多态
    # dispatch。参数已在上方 pop 循环中完成 Object::from_any 装箱。
    if obj_ty in (sim.class_type_params or ()):
        obj_e = f"Object::from_any(Clone::clone(&{obj_e}))"
        obj_ty_node = RsNamed('Object')
        obj_ty = 'Object'
        if mname in ('equals', 'hashCode', 'toString'):
            _rust_ret_eq = jvm_to_rust(ret, registry)
            _arg_str_eq = ', '.join(args)
            if _rust_ret_eq == '()':
                sim.emit(RawStmt(f"{obj_e}.{mname}({_arg_str_eq})?;"))
            else:
                _v_eq = sim.fresh()
                sim.emit(RawStmt(f"let {_v_eq}: {_rust_ret_eq} = {obj_e}.{mname}({_arg_str_eq})?;"))
                sim.push(Var(_v_eq), RsNamed(_rust_ret_eq))
            return
    if mname in UNBOX_VIRTUAL:
        _UNBOX_TARGET = {
            'intValue': 'i32', 'longValue': 'i64', 'doubleValue': 'f64',
            'floatValue': 'f32', 'booleanValue': 'bool', 'byteValue': 'i8', 'shortValue': 'i16',
            'charValue': 'u16',
        }
        target_ty = _UNBOX_TARGET.get(mname)
        if target_ty is None or obj_ty == target_ty:
            sim.push(obj_expr, obj_ty_node)
            return
        # 接收者是装箱对象（Integer/Double/Number 等），生成实际方法调用完成解箱
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {target_ty} = {obj_e}.{mname}()?;"))
        sim.push(Var(v), RsNamed(target_ty))
        return

    # JVM 数组.getClass() → Object::default()（代表 Class<T[]>）
    # Rust 侧 Vec/数组类型没有 getClass()，但调用方（如 Arrays.copyOf）只用
    # 其结果判断是否为 Object[] 类型；Object::default() 使判断走 Object[] 分支
    if mname == 'getClass' and obj_ty.startswith('Rc<RefCell<Vec<'):
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: Object = Object::default();"))
        sim.push(Var(v), RsNamed('Object'))
        return

    # T38：PrintStream.println 有参版本 → 统一生成 println_v(x)（Display 派发）
    # 注：无参 println() 保持原名；println_v<T: Display> 处理所有有参版本
    # 浮点参数先经 java_fmt_*（Java 语义：3.0 不打成 3；NaN/Infinity 拼写一致）
    if mname == 'println' and cls and cls.endswith('PrintStream') and len(args) == 1:
        if params == ['D']:
            sim.emit(RawStmt(f"{obj_e}.println_v(java_fmt_f64({args[0]}))?;"))
        elif params == ['F']:
            sim.emit(RawStmt(f"{obj_e}.println_v(java_fmt_f32({args[0]}))?;"))
        else:
            sim.emit(RawStmt(f"{obj_e}.println_v({args[0]})?;"))
        return

    # 若接收方 Rust 类型是 java_runtime 手写类，不做 mangle
    obj_base = obj_ty.split('<')[0].strip()  # 去泛型后缀（ArrayList<T> → ArrayList）
    if obj_base in _JAVA_RUNTIME_SHORT_NAMES:
        rust_mname = _safe_field(mname)
    else:
        # 优先用接收者实际类型 mangle（invokeinterface 通过接口调用时 cls 是接口，
        # 接口只有一个方法→漏判重载；用实际 receiver 类型能正确找到重载）
        mangle_cls = obj_base if (obj_base and obj_base not in ('Object', '()')) else (cls or '')
        rust_mname = _safe_field(_mangle_if_overloaded(mangle_cls, mname, comment, registry))

    # T76：若方法定义在父类（继承方法），通过 _super 链路由调用
    # 基于接收者实际 Rust 类型查找方法是否需要通过 _super 路由
    # 用 JVM 描述符精确匹配重载，避免同名但不同参数的方法干扰路由判断
    super_method_pfx = ''
    if registry:
        recv_base = obj_ty.split('<')[0].strip()
        cls_short = class_name.rsplit('/', 1)[-1] if class_name and '/' in class_name else (class_name or '')
        jvm_desc = f"({''.join(params)}){ret}"
        if recv_base == cls_short:
            super_method_pfx = _find_method_super_prefix(class_name, mname, registry, descriptor=jvm_desc)
        elif recv_base and recv_base not in ('Object', '()'):
            super_method_pfx = _find_method_super_prefix_for_type(recv_base, mname, registry, descriptor=jvm_desc)
    if super_method_pfx:
        obj_e = _super_prefix_to_expr(obj_e, super_method_pfx)

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret, registry)
    # E0599 防护：接收者是 Object 类型时，Object 结构体不定义具体子类方法，
    # 直接调用会产生 E0599。若目标类已知且有子类，生成 downcast dispatch 链（多态虚分发）。
    obj_is_bare = (obj_ty == 'Object')
    if obj_is_bare and cls and registry:
        # 多态 dispatch：按继承链（叶→根）依次 downcast，找到实际类型后调用方法
        # cls 是 Rust 短类名（$ 已替换为 _），需转回 binary name 查继承链
        cls_binary = _rust_type_to_binary(cls, registry) or cls
        subtypes = _get_all_subtypes_ordered(cls_binary, registry)
        # 跨 crate 防泄漏：JDK 类文件落在 java_runtime crate，不能引用 user crate
        # 的类型。batch 并集 registry 会把用户测试类也列为 Comparable 等接口的
        # 子类，烘焙进 java_runtime 后产生 E0425（类型不在本 crate 作用域）。
        # 因此 JDK 类的分派链只枚举 JDK 子类；user 类文件可引用两者（依赖方向合法）。
        if is_jdk(cls_binary):
            subtypes = [s for s in subtypes if is_jdk(s)]
        cls_rust = jvm_to_rust(f'L{cls_binary};', registry)
        # Arch-3: 闭包回退 —— Rc<dyn Fn(...)> downcast（lambda / 方法引用）
        # SAM 参数列表对应 invokevirtual/invokeinterface 的实际参数类型
        # Result 用裸名：两个 crate 的生成文件均经 prelude 引入
        # （java_runtime: crate::prelude / user: java_runtime::prelude），
        # 写 crate::error::Result 在 user crate 里是 E0433。
        _sam_ptypes = [jvm_to_rust(p, registry) for p in params]
        _fn_type = (f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)})'
                    f' -> Result<{rust_ret}>>')
        if rust_ret == '()':
            _closure_branch = (f'if let Some(__f) = {obj_e}.0.as_any()'
                               f'.downcast_ref::<{_fn_type}>() {{ (__f)({arg_str})?; }}')
        else:
            _closure_branch = (f'if let Some(__f) = {obj_e}.0.as_any()'
                               f'.downcast_ref::<{_fn_type}>() {{ (__f)({arg_str})? }}')

        # 只有当目标类有已知子类时，才生成 dispatch 链（否则退化为简单 downcast）
        if subtypes:
            all_types = subtypes + [cls_binary]  # 叶→根
            v = sim.fresh('_vdispatch')
            branches = []
            for sub_bin in all_types:
                sub_rust = jvm_to_rust(f'L{sub_bin};', registry)
                if sub_rust == 'Object':
                    # 目标类是接口（jvm_to_rust 对接口返回 Object）：
                    # downcast_ref::<Object>() 恒为 Some，会遮蔽闭包回退分支，
                    # 且 Object 上没有业务方法（E0599），直接丢弃该分支
                    continue
                # Fix 12a：包装类在 JVM_RUST 中映射为基本类型（Boolean→bool、
                # Double→f64 等），downcast_ref::<bool>() 不满足 Any 约束是硬
                # 错误，该分支不可编译 → 丢弃
                if sub_rust in _PRIMITIVE_RUST_TYPES:
                    continue
                # Fix 12b：重载 mangle 按声明类（owner）查 —— 子类继承的重载
                # 方法在子类方法表中查不到同名重载，按子类表 mangle 会得到
                # 错误的方法名（如 collect 声明处 mangle 为 collect_collec，
                # 子类分支按子类表查成 collect）。owner 沿父类链解析不到时
                # （接口 default 方法由 class_writer 注入实现类 impl 块，
                # 父类链上查不到）保留子类名。
                _owner_bin, _ = _resolve_method_owner(
                    sub_bin, mname, registry, descriptor=jvm_desc)
                _mangle_cls = _owner_bin or sub_rust
                sub_mname_r = _mangle_if_overloaded(_mangle_cls, mname, comment, registry)
                sub_mname_r = _safe_field(sub_mname_r)
                # Fix 17：bridge 分派的参数 downcast —— dispatch 的共享 args 按
                # 擦除描述符 coercion（如 Comparable.compareTo(Object) 的参数为
                # Object），但子类分支的真实方法（bridge 的目标，如
                # Byte.compareTo(Byte)）参数是具体类型 → per-branch 包装
                # .downcast::<T>()（等价 bridge 方法内的 checkcast）。
                _barg_str = arg_str
                if registry:
                    _own_ci17 = registry.get(_owner_bin or sub_bin)
                    if _own_ci17 is not None:
                        _bm17 = None
                        for _m in _own_ci17.methods:
                            if (_m.name == mname and not _m.is_synthetic
                                    and _m.descriptor == jvm_desc):
                                _bm17 = _m
                                break
                        if _bm17 is None:
                            # 精确 descriptor 失败（bridge 擦除场景）：
                            # 按名字 + 参数个数唯一匹配真实方法
                            _cands17 = [
                                _m for _m in _own_ci17.methods
                                if _m.name == mname and not _m.is_synthetic
                                and len(parse_descriptor_params(_m.descriptor)) == len(params)
                            ]
                            if len(_cands17) == 1:
                                _bm17 = _cands17[0]
                        if _bm17 is not None:
                            _bp17 = parse_descriptor_params(_bm17.descriptor)
                            if len(_bp17) == len(args) and _bp17 != list(params):
                                _bparts17 = []
                                for _i17, _bd17 in enumerate(_bp17):
                                    _bt17 = jvm_to_rust(_bd17, registry)
                                    _shared17 = jvm_to_rust(params[_i17], registry)
                                    if (_bt17 not in ('Object', '()')
                                            and _shared17 == 'Object'):
                                        _bparts17.append(
                                            f"({args[_i17]}).downcast::<{_bt17}>()")
                                    else:
                                        _bparts17.append(args[_i17])
                                _barg_str = ', '.join(_bparts17)
                # T76：方法定义在父类（如 getKey 定义在 HashMap_Node，TreeNode
                # 经继承获得）时，dispatch 分支同样需要 _super 链路由，
                # 否则 _d.getKey() E0599（no method in &HashMap_TreeNode）
                _d_recv = '_d'
                _sub_pfx = _find_method_super_prefix_for_type(
                    sub_rust.split('<')[0], mname, registry,
                    descriptor=f"({''.join(params)}){ret}",
                )
                if _sub_pfx:
                    _d_recv = _super_prefix_to_expr('_d', _sub_pfx)
                if rust_ret == '()':
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_d_recv}.{sub_mname_r}({_barg_str})?; }}")
                else:
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_d_recv}.{sub_mname_r}({_barg_str})? }}")
            branches.append(_closure_branch)
            if rust_ret == '()':
                dispatch_code = ' else '.join(branches)
                sim.emit(RawStmt(f"{dispatch_code}"))
            else:
                dispatch_expr = ' else '.join(branches) + f" else {{ Default::default() }}"
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
        # 无 subtypes 时（接口无已知实现类）：单独生成闭包 dispatch + 占位
        v = sim.fresh('_vdispatch')
        if rust_ret == '()':
            sim.emit(RawStmt(_closure_branch))
        else:
            dispatch_expr = _closure_branch + f" else {{ Default::default() }}"
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
            sim.push(Var(v), RsNamed(rust_ret))
        return
    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{obj_e}.{rust_mname}({arg_str})?;"))
    else:
        v = sim.fresh()
        if obj_is_bare and rust_ret not in ('Object', '()') and rust_ret not in _PRIMITIVE_RUST_TYPES:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
        elif rust_mname == 'clone' and obj_ty not in ('Object', '()'):
            # invokevirtual Object.clone 调用在具体类型上（如数组）：
            # Rust 的 clone() 不返回 Result，用 Object::from_any 包装匹配 Java 返回类型
            # Clone::clone 而非 .clone()：接收者可能是带 Java clone() 的类
            sim.emit(RawStmt(f"let {v}: Object = Object::from_any(Clone::clone(&{obj_e}));"))
            sim.push(Var(v), RsNamed('Object'))
        else:
            # 签名真实返回类型与擦除类型不一致时的对齐（与 invokestatic 同规则）：
            # - 擦除映射 Object：包 from_any 保持 Object 记录
            # - 擦除实例化 X<Object,Object> → 精确形态 X<K,V>：记录精确类型，
            #   与被调方法声明（gen_method_body 按 generic_signature 生成）一致
            _sig_ret_v = _lookup_method_sig_ret(
                cls, mname, params, ret, registry,
                caller_class=class_name, caller_tparams=sim.class_type_params,
                receiver_type=obj_ty,
            )
            if (rust_ret == 'Object' and _sig_ret_v is not None
                    and _sig_ret_v != 'Object'):
                sim.emit(RawStmt(f"let {v} = Object::from_any({obj_e}.{rust_mname}({arg_str})?);"))
                sim.push(Var(v), RsNamed(rust_ret))
            elif (_sig_ret_v is not None and _sig_ret_v != rust_ret
                    and rust_ret not in _PRIMITIVE_RUST_TYPES):
                sim.emit(RawStmt(f"let {v} = {obj_e}.{rust_mname}({arg_str})?;"))
                sim.push(Var(v), RsNamed(_sig_ret_v))
            else:
                sim.emit(RawStmt(f"let {v} = {obj_e}.{rust_mname}({arg_str})?;"))
                sim.push(Var(v), RsNamed(rust_ret))
