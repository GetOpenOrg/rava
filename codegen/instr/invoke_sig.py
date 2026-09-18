# 从 codegen/instr/invoke.py 中拆出

from ..type_map import (
    short_cls,
    parse_class_type_params as _parse_class_type_params,
    parse_method_param_types as _parse_method_param_types,
    method_sig_types as _method_sig_types,
)
from .coerce import (
    _rust_type_to_binary,
    _JAVA_RUNTIME_SHORT_NAMES,
)


def _lookup_method_sig_params(
    cls_short: str | None,
    mname: str,
    descriptor_params: list[str],
    descriptor_ret: str,
    registry: dict | None,
    caller_class_type_params: frozenset[str],
    receiver_targ_map: dict | None = None,
) -> list[str | None] | None:
    """查找被调用方法的 generic_signature，返回真实参数类型列表。

    返回值中 None 表示该位置降级到 jvm_to_rust(descriptor)。
    只有当 callee 的类型参数在 caller 的类型参数集合中可见时，才保留泛型参数名；
    receiver_targ_map 提供 callee 类型参数到接收者实参的映射（如 {K: E, V: Object}），
    用于解析跨类泛型调用（HashSet<E> 调 HashMap<E,Object>.put(K,V) → K=E）。
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
            # 用被调用类的类型参数解析签名（覆盖方法取最远祖先声明，与定义侧同规则）
            callee_tparams_list = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
            callee_tparams = frozenset(callee_tparams_list)
            types, _ = _method_sig_types(ci, m, callee_tparams_list, registry)
            if not types:
                return None
            # 将 callee 类型参数映射到 caller 上下文：
            # 优先级：receiver_targ_map（接收者泛型实参）> caller_class_type_params（同名类型参数）
            # 若某参数是 callee 的类型参数但无法解析，设 None（降级到 descriptor）
            resolved: list[str | None] = []
            for t in types:
                if t in callee_tparams:
                    if t in caller_class_type_params:
                        resolved.append(t)
                    elif receiver_targ_map and t in receiver_targ_map:
                        resolved.append(receiver_targ_map[t])
                    else:
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
            # 非泛型类的方法同样可带泛型签名返回类型（RecursiveTask<BigInteger>）：
            # 方法声明侧（gen_method_body）不要求类有类型参数，调用点必须同规则
            callee_tparams = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
            _, sig_ret = _method_sig_types(ci, m, callee_tparams, registry)
            if not sig_ret:
                return None
            # 有效性：所有标识符须为已知类型（与 gen_method_body 的 _sig_param_valid 同规则）
            import re as _re_v
            _builtin = frozenset({
                'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
                'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell', 'usize', 'u8',
                'JArray',  # Rust 端数组包装，不对应 Java 类
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


def _erased_ret_is_type_var(cls: str | None, mname: str, descriptor_params: list[str],
                            descriptor_ret: str, registry: dict | None) -> bool:
    """被调方法（沿 cls 的超类/接口链查找声明）的泛型签名返回类型是否为裸类型变量（TV;）。
    此时 Rust 端真实返回类型取决于具体实现类（V 或 Object），调用点无法静态确定，
    须用幂等的 Object 装箱对齐到擦除类型。"""
    if not cls or not registry:
        return False
    full_desc = '(' + ''.join(descriptor_params) + ')' + descriptor_ret
    start = cls if '/' in cls else (_rust_type_to_binary(cls, registry) or cls)
    queue, seen = [start], set()
    while queue:
        cur = queue.pop(0)
        if cur in seen:
            continue
        seen.add(cur)
        ci = registry.get(cur)
        if not ci:
            continue
        for m in ci.methods:
            if m.name == mname and m.descriptor == full_desc:
                sig = m.generic_signature or ''
                return ')' in sig and sig.rsplit(')', 1)[1].startswith('T')
        if ci.super_class:
            queue.append(ci.super_class)
        queue.extend(ci.interfaces or [])
    return False


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
        _reinstantiate_generic,
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
    # 同一泛型类的不同实例化（raw type / 通配符形参接收精确实例化的实参）
    if _downcast_target_valid(expected, sim, registry):
        _reinst = _reinstantiate_generic(e, actual, expected)
        if _reinst is not None:
            return _reinst
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
