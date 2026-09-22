# 从 codegen/instr/invoke.py 中拆出

from ..constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_TYPE_NAMES
from ..type_map import short_cls as _short_cls_g
from ..sig_parse import parse_method_param_types as _parse_method_param_types
from ..sig_types import method_sig_types as _method_sig_types
from ..type_args import substitute_type_params as _substitute_type_params
from ..type_map import (
    short_cls,
    parse_class_type_params as _parse_class_type_params,
    effective_class_type_params as _effective_class_type_params,
)
from ..type_args import class_type_param_bounds as _class_type_param_bounds
from ..stack import _clone_moved_var
from ..rs_ir import RawExpr, RsNamed
from ..render import render_expr, render_type
from ..constants import JAVA_RUNTIME_SHORT_NAMES as _JAVA_RUNTIME_SHORT_NAMES
from .hierarchy import _rust_type_to_binary

def type_var_receiver_bound_view(sim, obj_expr, obj_ty):
    """有类上界的类型变量接收者（o: E，E extends B<E>）→ 上界类型视图 (expr, type)。

    成员（字段访问器 / 方法）定义在上界类的 wrapper 上，类型变量本身没有成员（E0599）。
    Java 侧该访问经上界类型静态解析（类型变量擦除为上界）→ Rust 侧经 Object 的 checkcast
    视图转换为上界类型（按运行时类重建上界类视图，保留运行时类型）。不用 `E: Into<B<E>>`
    约束：覆盖方法不能比 vtable 声明多带约束，约束放 struct 头又会使擦除实例化的证明循环。
    getfield / putfield / invokevirtual 共用；无类上界（接口上界 / 无界）时原样返回。
    """
    _bound = sim.type_var_bounds.get(render_type(obj_ty))
    if _bound is None:
        return obj_expr, obj_ty
    _src = _clone_moved_var(obj_expr, obj_ty)
    return (RawExpr(f"Into::<{_bound}>::into(Into::<Object>::into({render_expr(_src)}))"),
            RsNamed(_bound))


def _lookup_method_sig_params(
    cls_short: str | None,
    mname: str,
    descriptor_params: list[str],
    descriptor_ret: str,
    registry: dict | None,
    caller_class_type_params: frozenset[str],
    receiver_targ_map: dict | None = None,
    receiver_is_this: bool = False,
    receiver_type: str | None = None,
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
    # JVM 方法解析：常量池类未声明该方法时，目标是超类链上最近的声明者；
    # 形参类型变量属于声明者，按接收者静态类型视角代入
    if mname != '<init>' and not any(m.name == mname and m.descriptor == full_desc for m in ci.methods):
        _seen_owner: set[str] = {ci.name}
        _cur = registry.get(ci.super_class) if ci.super_class else None
        while _cur is not None and _cur.name not in _seen_owner:
            _seen_owner.add(_cur.name)
            if any(m.name == mname and m.descriptor == full_desc for m in _cur.methods):
                ci = _cur
                if receiver_type:
                    receiver_targ_map = receiver_type_arg_map(receiver_type, short_cls(ci.name), registry)
                break
            _cur = registry.get(_cur.super_class) if _cur.super_class else None
    for m in ci.methods:
        if m.name == mname and m.descriptor == full_desc:
            # 用被调用类的类型参数解析签名（覆盖方法取最远祖先声明，与定义侧同规则）
            callee_tparams_list = _effective_class_type_params(ci, registry)
            callee_tparams = frozenset(callee_tparams_list)
            types, _ = _method_sig_types(ci, m, callee_tparams_list, registry)
            if not types:
                return None
            if receiver_targ_map:
                # 参数化形参（X<E_IN, Object>）中的 callee 类型变量按接收者实参替换；
                # 裸类型变量位置由下方逐项解析
                types = [t if t in callee_tparams else _substitute_type_params(t, receiver_targ_map)
                         for t in types]
            elif ci.is_interface and not receiver_is_this and callee_tparams:
                # 接口调用经擦除载体 `I<Object, ..>` 分派：参数化形参里的接口类型变量
                # 即 Object（与调用方同名的类型变量只是命名巧合）
                _erased_map = {t: 'Object' for t in callee_tparams}
                types = [t if t in callee_tparams else _substitute_type_params(t, _erased_map)
                         for t in types]
            # 将 callee 类型参数映射到 caller 上下文：
            # 优先级：receiver_targ_map（接收者泛型实参）> caller_class_type_params（同名类型参数）
            # 若某参数是 callee 的类型参数但无法解析，设 None（降级到 descriptor）
            resolved: list[str | None] = []
            subst_pos: set[int] = set()   # 经类型变量代入得到该值的位置（非直签接口形态）
            for _ti, t in enumerate(types):
                if t in callee_tparams:
                    if (ci.is_interface and not receiver_is_this
                            and not (receiver_targ_map and t in receiver_targ_map)):
                        # Arch-1：接口在 Rust 侧是 Object 别名，没有类型参数；
                        # 其形参与调用方同名（Function<T,R> 在 Optional<T> 内被调用）
                        # 只是命名巧合，接口方法形参一律是擦除形态。
                        # 例外：接收者是 this（default 方法体被继承进实现类），形参即本类形参；
                        # 接收者静态类型是具体泛型类（Set<String> s = new LinkedHashSet<>()）时
                        # 按接收者实参解析
                        resolved.append(None)
                    elif receiver_targ_map and t in receiver_targ_map:
                        resolved.append(receiver_targ_map[t])
                        subst_pos.add(_ti)
                    elif t in caller_class_type_params:
                        resolved.append(t)
                    else:
                        resolved.append(None)   # 擦除，使用 jvm_to_rust(descriptor) 降级
                else:
                    resolved.append(t)
            # registry 中所有接口的短名在调用点降级为 Object（Arch-1 接口 = Object
            # 类型别名，泛型形态 X<...> 不是合法 Rust 类型）。
            # A-4 批次 3+：两类载体形态例外——
            #   a) 经类型变量代入（构造器 turbofish 绑定 / 接收者实参映射）：被调方
            #      声明的是类型变量，rustc 按绑定展开为载体，实参必须是载体；
            #   b) 直签接口形态且描述符同名（`Ljava/util/Iterator;` ↔ Iterator）：
            #      声明侧（method_gen/emitted_method_sig_types 的接口回退）按描述符
            #      发射同一载体。描述符是桥接擦除（`Ljava/lang/Object;`，javac 对
            #      OfLong 桥接 forEachRemaining(Object)）时维持 Object——与声明侧一致。
            import re as _re_iface
            from ..jvm_type import carrier_type_for_ident as _carrier_keep
            from ..type_map import jvm_to_rust as _jvm_to_rust_g
            _reg_iface_shorts = _registry_iface_shorts(registry)
            # 手写边界方法：接口位置的契约是 Object（其 _impl.rs 签名先于载体化）
            _hw = _handwritten_boundary_method(cls_bin, mname)
            final_resolved: list[str | None] = []
            for _ri, t in enumerate(resolved):
                if _hw and _ri < len(descriptor_params):
                    _t_base = t.split('<')[0] if t else ''
                    _d_rust = _jvm_to_rust_g(descriptor_params[_ri], registry)
                    _d_base = _d_rust.split('<')[0]
                    if (t is not None and _t_base in _reg_iface_shorts) or (
                            t is None and _d_base in _reg_iface_shorts):
                        final_resolved.append('Object')
                        continue
                if t is not None:
                    _m = _re_iface.match(r'^(\w+)(?:<|$)', t)
                    if _m and _m.group(1) in _reg_iface_shorts:
                        _desc_base = (_jvm_to_rust_g(descriptor_params[_ri], registry).split('<')[0]
                                      if _ri < len(descriptor_params) else '')
                        # receiver_is_this（接口 default 方法体内 this.xxx）：发射侧
                        # （virtual_in 语境的存根）按描述符擦除，调用侧不得按接收者
                        # 实参映射发射载体——否则与存根签名发散
                        if ((_carrier_keep(t, registry) == t and _ri in subst_pos
                                and not receiver_is_this)
                                or _desc_base == t.split('<')[0]):
                            final_resolved.append(t)
                        else:
                            final_resolved.append(None)
                        continue
                final_resolved.append(t)
            return final_resolved
    return None


_iface_shorts_cache: dict[int, frozenset[str]] = {}


_HANDWRITTEN_BOUNDARY_CACHE: dict[tuple[str, str], bool] = {}


def _handwritten_boundary_method(cls_bin: str, mname: str) -> bool:
    """callee 是否由 runtime/ 手写 `_impl.rs` 伴生文件提供（`#[jvm_boundary]`）。

    手写边界方法的 Rust 签名是调用契约的真源（早于 A-4 载体化，接口位置一律
    擦除 Object）；载体化后调用点解析须按其签名回退 Object，而非发射载体。
    与 project_writer._is_handwritten 同一存在性判据（runtime/java_runtime/src
    下同相对路径），加 `fn {mname}` 前缀探测（重载后缀缀于 Java 原名之后，
    如 `fn checkIndex_i_i_bifunction`）。"""
    key = (cls_bin, mname)
    hit = _HANDWRITTEN_BOUNDARY_CACHE.get(key)
    if hit is not None:
        return hit
    import os as _os
    from ..constants import RUNTIME_JAVA_RUNTIME
    from ..emitter.attrs import to_snake as _to_snake
    result = False
    parts = cls_bin.split('/')
    if len(parts) >= 2:
        *pkg, cls_name = parts
        parent = _os.path.join(RUNTIME_JAVA_RUNTIME, 'src', *pkg)
        probe = f'fn {mname}'
        for cand in (_to_snake(cls_name), _to_snake(cls_name) + '_t'):
            try:
                with open(_os.path.join(parent, cand + '_impl.rs'), encoding='utf-8') as fh:
                    if probe in fh.read():
                        result = True
                        break
            except OSError:
                continue
    _HANDWRITTEN_BOUNDARY_CACHE[key] = result
    return result


def receiver_type_arg_map(recv_ty: str, owner_short: str | None, registry: dict | None) -> dict | None:
    """接收者静态类型 recv_ty 视角下，方法声明类 owner 的类型形参 → 实参映射。

    - recv_ty 即 owner 的实例化（`HashMap<E, Object>`）→ {K: E, V: Object}
    - recv_ty 是 owner 的后代（`Child` extends `Parent<Child>`）→ 沿超类链代入（{E: Child}）
    无法确定（接收者是 Object / 类型变量 / 接口视角）→ None。"""
    if not registry or not owner_short or not recv_ty:
        return None
    from ..type_args import ancestor_type_args, split_rust_type_args
    recv_base = recv_ty.split('<', 1)[0].strip()
    recv_ci = registry.get(_rust_type_to_binary(recv_base, registry) or '')
    owner_ci = registry.get(_rust_type_to_binary(owner_short, registry) or '')
    if recv_ci is None or owner_ci is None:
        return None
    owner_params = _effective_class_type_params(owner_ci, registry)
    if not owner_params:
        return None
    recv_args = split_rust_type_args(recv_ty)
    if recv_ci.name == owner_ci.name:
        return dict(zip(owner_params, recv_args)) if len(recv_args) == len(owner_params) else None
    recv_params = _effective_class_type_params(recv_ci, registry)
    if len(recv_args) != len(recv_params):
        return None
    for anc_bin, args in ancestor_type_args(recv_ci, registry, recv_args):
        if anc_bin == owner_ci.name:
            return dict(zip(owner_params, args)) if len(args) == len(owner_params) else None
    return None


def _registry_iface_shorts(registry: dict | None) -> frozenset[str]:
    """registry 中所有接口的 Rust 短名（含 $→_ 替换），按 id(registry) 缓存。"""
    if not registry:
        return frozenset()
    _key = id(registry)
    _cached = _iface_shorts_cache.get(_key)
    if _cached is not None:
        return _cached
    _shorts = frozenset(
        _short_cls_g(_bin)
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
        _short_cls_g(_bin)
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
    # 基本类型实参（`AbstractPipeline<Object, i32, Object>`：包装类按基本类型建模）
    _ok |= set(_PRIMITIVE_TYPE_NAMES)
    return all(_n in _ok for _n in _names)


def _is_generated_concrete_class(actual: str, sim: 'StackSim', registry: dict | None) -> bool:
    """actual 是否为 registry 中由字节码生成的具体（非接口）类的 Rust 类型。"""
    if not registry or not actual:
        return False
    _head = actual.split('<', 1)[0].rsplit('::', 1)[-1].strip()
    if not _head or _head in (sim.class_type_params or ()):
        return False
    return _head in _generated_concrete_shorts(registry)


_generated_shorts_cache: dict[int, frozenset[str]] = {}


def _generated_concrete_shorts(registry: dict) -> frozenset[str]:
    _key = id(registry)
    _cached = _generated_shorts_cache.get(_key)
    if _cached is None:
        _cached = frozenset(
            _short_cls_g(_bin)
            for _bin, _ci in registry.items()
            if not getattr(_ci, 'is_interface', False)
        )
        _generated_shorts_cache[_key] = _cached
    return _cached


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
            callee_tparams = _effective_class_type_params(ci, registry)
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
            _reg_shorts = {_short_cls_g(k) for k in registry}
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
            # 手写边界方法（_impl.rs）：接口返回位置的契约是 Object（签名先于载体化）
            if _handwritten_boundary_method(cls_bin, mname) \
                    and sig_ret.split('<')[0] in _registry_iface_shorts(registry):
                return 'Object'
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


def _exact_ancestor_type(actual: str, ancestor_short: str, registry: dict | None) -> str:
    """静态类型 actual（如 `Child<A, B>`）沿超类链到 ancestor_short 的精确实例化
    （`Parent<A, B, Object>`）；不在超类链上 → ''。"""
    from ..type_args import ancestor_type_args, split_rust_type_args, rust_type_with_args
    ci = registry.get(_rust_type_to_binary(actual.split('<', 1)[0], registry)) if registry else None
    if ci is None:
        return ''
    for anc_bin, args in ancestor_type_args(ci, registry, split_rust_type_args(actual)):
        if short_cls(anc_bin) == ancestor_short:
            return rust_type_with_args(ancestor_short, args)
    return ''


def _upcast_to_ancestor_instantiation(src: str, actual: str, expected: str,
                                      sim: 'StackSim', registry: dict | None) -> str | None:
    """子类值上转到祖先类的「另一实例化」（raw type / 通配符位置：`Parent<A, i32, ?>`）。

    宏只为祖先的精确实例化生成 From（Child<A> → Parent<f(A)>）；目标是同一祖先的另一实例化时
    先上转到精确祖先，再经 Object 边界（保持对象标识）重新实例化。目标就是精确祖先 → None。"""
    exact_anc = _exact_ancestor_type(actual, expected.split('<')[0], registry)
    if (exact_anc and exact_anc != expected and '<' in expected
            and _downcast_target_valid(expected, sim, registry)
            and _downcast_target_valid(exact_anc, sim, registry)):
        return f"<{expected} as ::std::convert::From<Object>>::from(Object::from(Into::<{exact_anc}>::into({src})))"
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
    from ..constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
    from .coerce import (
        _coerce_from_null, _coerce_to_object, _coerce_value,
        _render_cast, _same_generic_family,
    )
    from .hierarchy import _is_subtype, _into_super_chain, _rust_type_to_binary
    from ..jvm_type import carrier_type_for_ident
    null_coerce = _coerce_from_null(e, expected)
    if null_coerce is not None:
        return null_coerce
    _carrier_expected = carrier_type_for_ident(expected, registry)
    if _carrier_expected is not None and _carrier_expected == expected:
        # A-4 批次 3+：形参是已铺设的接口载体。实参 → 载体的边界转换：
        #   - 实参已是同载体：Clone 保持 move 语义（与尾部兜底同形）；
        #   - 具体实现类：协变 upcast（interface_gen 的 From<C> for I<Object>，
        #     任意类实例化上转到擦除载体是同一视图，保持对象身份）；
        #   - Object / 其余静态类型：经 From<Object> 的非受检载体包装（javac
        #     unchecked 语义——接口视图按运行时类成立，载体只持 Object 引用）。
        #     From<_> 的 UFCS 形态不可被接口自带的 Java 静态 from 工厂遮蔽。
        if actual == expected and actual not in _PRIMITIVE_RUST_TYPES:
            if e == 'this':
                return f"Clone::clone({e})"
            return f"Clone::clone(&{e})"
        if actual == 'Object':
            return f"<{expected} as ::std::convert::From<_>>::from(Clone::clone(&{e}))"
        _src_c = f"Clone::clone(this)" if e == 'this' else f"Clone::clone(&{e})"
        return f"<{expected} as ::std::convert::From<_>>::from(" \
               f"{_coerce_to_object(_src_c, actual, registry, sim.class_type_params, clone=False)})"
    if expected == 'Object' and actual not in ('Object', '()'):
        # 泛型参数值（如 K: Clone + Default + 'static）传给 Object 参数：
        # Rust 无隐式子类型化，K 类型的值不能直接当 Object 用 → 装箱为
        # Object(Rc<JvmRef<K>>)，callee 内按类型形参的 From<Object> bound 可还原。
        if actual in sim.class_type_params:
            return _coerce_to_object(e, actual, registry, sim.class_type_params)
        if e == 'this':
            # 构造器（fn new）里没有 self 关键字，统一用局部变量 this
            # （实例方法里 let this = self;，两者均可见）
            if _is_generated_concrete_class(actual, sim, registry):
                return "Object::from(Clone::clone(this))"
            # 接口载体（default 方法体落到接口自身块时的 this）等其余形态经
            # _coerce_to_object：载体走 `Object::from`（From 解包 __ref，保持接收者
            # 身份）；from_any 双重包装会使接口查询 / SAM 闭包直调失联。
            return _coerce_to_object('this', actual, registry, sim.class_type_params)
        return _coerce_to_object(e, actual, registry, sim.class_type_params)
    if expected in ('bool', 'i8', 'i16', 'u16') and actual != expected:
        return _coerce_value(e, e_ty_node, expected)
    if expected == 'i32' and actual in ('i8', 'i16', 'u16', 'bool'):
        return f"({e} as i32)"
    # 同一泛型类的不同实例化（raw type / 通配符形参接收精确实例化的实参）：
    # CastExpr 的擦除路径（A-3，替代已删除的 _reinstantiate_generic 字符串发射）
    if _downcast_target_valid(expected, sim, registry):
        if _same_generic_family(actual, expected):
            return _render_cast(e, expected, box_first=True)
    if (expected not in _PRIMITIVE_RUST_TYPES and actual not in _PRIMITIVE_RUST_TYPES
            and expected not in ('Object', '()', actual)
            and _is_subtype(actual.split('<')[0], expected.split('<')[0], registry)):
        # R-2：子类传给父类参数，通过显式 __into_super() 链（替代已删除的 T55 From impl）
        chain = _into_super_chain(actual.split('<')[0], expected.split('<')[0], registry)
        src = 'Clone::clone(this)' if e == 'this' else f"Clone::clone(&{e})"
        # 宏只为「祖先的精确实例化」生成 From（Child<A> → Parent<f(A)>）。形参是同一祖先的
        # 另一实例化（raw type / 通配符形参）时：先向上转换到精确祖先，再经 Object 边界重新实例化。
        _reinst_anc = _upcast_to_ancestor_instantiation(src, actual, expected, sim, registry)
        if _reinst_anc is not None:
            return _reinst_anc
        return f"{src}{chain}"
    # Fix 18：actual 是 Object（运行时多态值）而 expected 是具体引用类型 ——
    # Java 调用点隐式 checkcast 语义（A-3：CastExpr checked 形态，失败返回
    # Err(JvmError::class_cast) 可被 java_try 捕获，S-1）。
    # 覆盖「callee 签名参数是精确泛型形态而调用方局部变量被擦除为 Object」
    # 的场景（如 rotateLeft(root: TreeNode<K,V>) 传入 Object 局部变量）。
    # 目标类型须为具体类（非接口别名）/ 类级类型参数 / 内建容器，接口名
    # （List<..>、Consumer<T>）与方法级类型变量不是合法 checkcast 目标。
    if (expected not in _PRIMITIVE_RUST_TYPES and actual == 'Object'
            and expected not in ('Object', '()')
            and expected not in (sim.class_type_params or ())
            and _downcast_target_valid(expected, sim, registry)):
        _bin18 = _rust_type_to_binary(expected.split('<')[0], registry)
        if _bin18:
            return _render_cast(e, expected, binary_name=_bin18, checked=True)
        return _render_cast(e, expected)
    if actual == 'Object' and expected in (sim.class_type_params or ()):
        # 形参是类型变量而实参经擦除边界（方法级类型变量、Object 局部变量）退化为 Object：
        # javac 的 unchecked cast → 经宏为类型形参补的 From<Object> 取回（与 areturn 同规则）
        src = 'Clone::clone(this)' if e == 'this' else f"Clone::clone(&{e})"
        return f"From::from({src})"
    if (actual in (sim.class_type_params or ()) and expected != actual
            and expected not in _PRIMITIVE_RUST_TYPES and expected not in ('Object', '()')
            and expected not in (sim.class_type_params or ())
            and _downcast_target_valid(expected, sim, registry)):
        # 实参静态类型是类型变量（`S extends SpeciesData`），形参是其上界类：Java 的隐式
        # 子类型转换 → 经 Object 边界按对象标识取回上界类视图
        return f"From::from({_coerce_to_object(e, actual, registry, sim.class_type_params)})"
    if actual == 'Object' and expected.startswith('JArray<'):
        # 擦除为 Object 的数组值流入类型化数组形参（`Object o = intArr; f((int[]) o)`
        # 的实参位；checkcast 被验证器省略或已在上游消费）：经 `From<Object> for
        # JArray<T>` 的数组视图机制取回（R9 协变视图 / S-4 探针，checkcast 语义）
        return f"From::from(Clone::clone(&{e}))"
    if actual.startswith('JArray<') and expected.startswith('JArray<') and actual != expected:
        # 数组协变（`T[]` 擦除为 Object[] 的引用传给元素类型具体化的形参）：Java 数组在运行时
        # 按元素类型具体化，同一数组对象经 Object 边界按形参的元素类型取回（checkcast 语义）
        return f"From::from({_coerce_to_object(e, actual, registry, sim.class_type_params)})"
    if actual not in _PRIMITIVE_RUST_TYPES:
        # `this` 在 Rust 中是 &Self 引用，Clone::clone(this) 得到 Self，无需多余 &
        if e == 'this':
            return f"Clone::clone({e})"
        return f"Clone::clone(&{e})"
    return e
