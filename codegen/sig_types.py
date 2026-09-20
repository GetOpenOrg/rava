"""
方法 / 构造器签名类型、层次重载命名判定与 RsType 节点构造。

自 type_map.py 拆出。

依赖方向（顶层，禁止回环）：本模块 → type_args → sig_parse → type_map。
"""

from __future__ import annotations
import re

from .constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST, safe_ident as _safe_ident
from .sig_parse import _parse_one_type, _parse_type_args, parse_method_param_types
from .type_args import (ancestor_type_args, implemented_interface_views,
                        outer_instance_rust_type, substitute_type_params,
                        superclass_type_args)
from .type_map import (_ACC_MANDATED, effective_class_type_params, jvm_to_rust,
                       mangle_name, outer_instance_class, parse_descriptor_params,
                       short_cls as _short_cls_g)


_OVERLOAD_CACHE: dict[tuple, frozenset] = {}


def _class_method_param_sets(ci, registry: dict | None) -> tuple[dict, dict]:
    """ci 在 Rust 侧 impl 块中的方法表：name → 参数列表集合。
    返回 (全部方法, 仅实例方法)。包含 class_writer 注入的接口 default 方法
    （ci 自身未声明同签名时）；synthetic/bridge 不计；协变返回（参数相同）只算一种。"""
    all_sets: dict[str, set[str]] = {}
    inst_sets: dict[str, set[str]] = {}

    def _add(m) -> None:
        params = m.descriptor.split(')')[0]
        all_sets.setdefault(m.name, set()).add(params)
        if not m.is_static and not m.is_constructor:
            inst_sets.setdefault(m.name, set()).add(params)

    for m in ci.methods:
        if not m.is_synthetic and m.name != '<clinit>':
            _add(m)
    if registry and ci.interfaces and not ci.is_interface:
        queue: list[str] = list(ci.interfaces)
        seen: set[str] = set()
        while queue:
            iname = queue.pop(0)
            if iname in seen:
                continue
            seen.add(iname)
            ici = registry.get(iname)
            if ici is None:
                continue
            queue.extend(ici.interfaces or [])
            for m in ici.methods:
                if (not m.is_abstract and not m.is_static and not m.is_synthetic
                        and m.name not in ('<init>', '<clinit>')):
                    _add(m)
    return all_sets, inst_sets


def hierarchy_overloaded_names(ci, registry: dict | None) -> frozenset:
    """类 ci 中需要按描述符 mangle 的方法名集合 —— 定义侧与调用侧的唯一判定来源。

    Rust 侧子类通过 Deref 到父类来继承实例方法；子类的同名 inherent 方法会**按名字**
    遮蔽父类的全部同名方法（Java 按 name+descriptor 覆盖），vtable 覆盖又要求子类方法名
    与祖先 trait 中的名字一致。因此判定必须沿父类链进行：
      1. ci 自身方法表（含注入的接口 default 方法）中同名出现 ≥2 种参数列表
      2. 该名字在父类中已被 mangle → 子类沿用（覆盖方法名与祖先 vtable 一致）
      3. ci 声明了该名字的实例方法，且与祖先链上的同名实例方法合计 ≥2 种参数列表
         （Buffer.position() / ByteBuffer.position(int) → ByteBuffer 用 position_i，
         不遮蔽 Buffer.position()）
    static 方法按路径调用（不经 Deref），不参与跨类判定；<init> 不继承，只看规则 1。
    判定只向上看：祖先总在 registry 中，结果不随翻译范围变化。接口自身只用规则 1。"""
    cache_key = (id(registry), len(registry) if registry else 0, ci.name)
    cached = _OVERLOAD_CACHE.get(cache_key)
    if cached is not None:
        return cached
    own_all, own_inst = _class_method_param_sets(ci, registry)
    result: set[str] = {n for n, ps in own_all.items() if len(ps) > 1}
    if registry and not ci.is_interface:
        parent = registry.get(ci.super_class) if ci.super_class else None
        if parent is not None and parent.name != ci.name:
            # 单调继承：不论 ci 是否重新声明，名字一旦在祖先处 mangle，后代全部沿用
            result |= hierarchy_overloaded_names(parent, registry) - {'<init>'}
        inherited: dict[str, set[str]] = {}
        seen_cls: set[str] = {ci.name}
        anc = parent
        while anc is not None and anc.name not in seen_cls:
            seen_cls.add(anc.name)
            for n, ps in _class_method_param_sets(anc, registry)[1].items():
                inherited.setdefault(n, set()).update(ps)
            anc = registry.get(anc.super_class) if anc.super_class else None
        result |= {n for n, ps in own_inst.items() if len(ps | inherited.get(n, set())) > 1}
    frozen = frozenset(result)
    _OVERLOAD_CACHE[cache_key] = frozen
    return frozen


def instance_field_rust_name(owner_bin: str, safe_name: str, registry: dict | None) -> str:
    """实例字段的 Rust 名（struct 字段 / `__get_` / `__set_` 访问器共用的单一来源）。

    Java 字段按 (声明类, 名字) 静态解析，子类可以声明与祖先同名的字段（隐藏），两者是
    不同的存储槽。继承链字段在 Rust 侧展平进同一个 `__inner`，因此隐藏祖先字段的声明
    取 `<name>_<DeclaringClass>`，祖先字段保持原名。owner_bin 是字段引用的常量池类
    （可以是声明类的子类），沿父类链解析到首个声明该字段的类。"""
    if not registry or not owner_bin:
        return safe_name

    def _declares(ci) -> bool:
        return any(not f.is_static and _safe_ident(f.name) == safe_name for f in ci.fields)

    ci = registry.get(owner_bin)
    seen: set[str] = set()
    while ci is not None and ci.name not in seen and not _declares(ci):
        seen.add(ci.name)
        ci = registry.get(ci.super_class) if ci.super_class else None
    if ci is None or not _declares(ci):
        return safe_name
    decl = ci
    anc = registry.get(decl.super_class) if decl.super_class else None
    seen = {decl.name}
    while anc is not None and anc.name not in seen:
        if _declares(anc):
            return f"{safe_name}_{decl.name.rsplit('/', 1)[-1].replace('$', '_')}"
        seen.add(anc.name)
        anc = registry.get(anc.super_class) if anc.super_class else None
    return safe_name


def interface_member_local_name(ci, mname: str, descriptor: str, registry: dict | None) -> str:
    """类 ci 视角下「只声明在接口上的成员」（类链未声明该 name+descriptor）的 Rust 方法名 ——
    继承成员声明侧与调用侧的唯一判定来源。

    类链上已有同名实例方法（参数列表必然不同：`date(Era,int,int,int)` 对接口的
    `date(TemporalAccessor)`）时，接口成员与之构成重载 → 按描述符 mangle，不与类方法撞名；
    类方法自身的名字不受影响（其名字只由类链决定，见 hierarchy_overloaded_names）。"""
    if mname in hierarchy_overloaded_names(ci, registry):
        return mangle_name(mname, descriptor)
    params = descriptor.split(')')[0]
    seen: set[str] = set()
    cur = ci
    while cur is not None and cur.name not in seen:
        seen.add(cur.name)
        declared = _class_method_param_sets(cur, registry)[1].get(mname)
        if declared is not None:
            # 同参数列表 = 该成员其实就在类的方法表里（注入的接口 default）→ 名字由类链决定
            return mname if params in declared else mangle_name(mname, descriptor)
        cur = registry.get(cur.super_class) if (registry and cur.super_class) else None
    return mname


def method_name_is_mangled(ci, method, registry: dict | None) -> bool:
    """类 ci 声明的方法 method 的 Rust 名是否带描述符后缀 —— 按 (name, descriptor) 判定，
    定义侧与调用侧共用。

    名字在 ci 中需要 mangle（hierarchy_overloaded_names）时，覆盖方法例外：它实现的是
    祖先 vtable trait 中的槽位，Rust 名必须与槽位所属类（virtual_in）中的名字一致。
    槽位所属类未 mangle 该名字、而子类因新增重载 / 注入的接口 default 方法才 mangle 时，
    覆盖方法沿用槽位名（否则 impl 出祖先 trait 没有的方法，E0407）。"""
    if method.name not in hierarchy_overloaded_names(ci, registry):
        return False
    if not registry or ci.is_interface or method.is_constructor or method.is_static:
        return True
    from .emitter.vtable_util import _find_virtual_in, _bin_to_rust
    slot_owner_rust = _find_virtual_in(method, ci, registry)
    if not slot_owner_rust or slot_owner_rust == _bin_to_rust(ci.name):
        return True
    anc = registry.get(ci.super_class) if ci.super_class else None
    seen: set[str] = {ci.name}
    while anc is not None and anc.name not in seen:
        seen.add(anc.name)
        if _bin_to_rust(anc.name) == slot_owner_rust:
            return method.name in hierarchy_overloaded_names(anc, registry)
        anc = registry.get(anc.super_class) if anc.super_class else None
    return True


_ACC_PRIVATE = 0x0002
_ACC_SYNTHETIC = 0x1000
_LOAD_OPCODE_RE = re.compile(r'^[ailfd]load(?:_(\d))?$')


def is_anonymous_class(ci) -> bool:
    """匿名类：带 EnclosingMethod，且 InnerClasses 中自身条目无简单名。"""
    if not getattr(ci, 'enclosing_class', ''):
        return False
    return any(ic.inner_class == ci.name and not ic.inner_name for ic in ci.inner_classes or ())


def _forwarded_super_ctor_params(ci, m) -> 'tuple[str, dict[int, int]]':
    """构造器体内 super(...) 调用的 (父类构造器描述符, {父类形参下标 → 本构造器形参下标})。
    只收录「实参是本构造器形参的直接转发（xload slot）」的位置。"""
    params = parse_descriptor_params(m.descriptor)
    slot_to_param: dict[int, int] = {}
    slot = 1
    for idx, p in enumerate(params):
        slot_to_param[slot] = idx
        slot += 2 if p in ('J', 'D') else 1
    marker = f'Method {ci.super_class}.<init>:'
    for pos, ins in enumerate(m.instrs):
        if ins.opcode != 'invokespecial' or not (ins.comment or '').startswith(marker):
            continue
        super_desc = ins.comment[len(marker):]
        n_super = len(parse_descriptor_params(super_desc))
        forwarded: dict[int, int] = {}
        for k in range(n_super):
            src_pos = pos - n_super + k
            if src_pos < 0:
                break
            lm = _LOAD_OPCODE_RE.match(m.instrs[src_pos].opcode)
            if not lm:
                continue
            src_slot = int(lm.group(1)) if lm.group(1) is not None else int(m.instrs[src_pos].operand or -1)
            if src_slot in slot_to_param:
                forwarded[k] = slot_to_param[src_slot]
        return super_desc, forwarded
    return '', {}


def constructor_sig_types(ci, m, class_type_params: list[str], registry=None) -> list[str]:
    """构造器在 Rust 侧的形参类型表；[] 表示退回描述符擦除形态。
    定义侧（方法体）与所有调用侧（new / super(...)）统一经 method_sig_types 到达此处。

    构造器的描述符含编译器注入的隐式形参，而 Signature attribute 只描述源码声明的形参：
      - 外部实例（内部类首个形参）→ 外部类 + 本类继承的类型变量（Outer<E>）
      - 匿名类构造器（JLS §15.9.5.1：形参与父类构造器一致，无 Signature）
        → 转发给 super(...) 的形参取父类构造器的泛型形参类型（父类形参按本类的
          SuperclassSignature 实参替换）
      - 其余隐式形参（捕获变量 val$x、枚举 name/ordinal）→ 描述符形态
    """
    params = parse_descriptor_params(m.descriptor)
    if not params:
        return []
    types = [jvm_to_rust(p, registry) for p in params]
    recovered = False

    if m.generic_signature:
        declared, _ = parse_method_param_types(m.generic_signature, class_type_params, registry,
                                               is_static=False)
        if len(declared) == len(params):
            types, recovered = list(declared), True
        elif declared and m.method_parameters and len(m.method_parameters) == len(params):
            explicit = [i for i, (_n, flags) in enumerate(m.method_parameters)
                        if not flags & (_ACC_SYNTHETIC | _ACC_MANDATED)]
            if len(explicit) == len(declared):
                for i, t in zip(explicit, declared):
                    types[i] = t
                recovered = True

    outer_bin = outer_instance_class(ci)
    if outer_bin and params[0] == f'L{outer_bin};' and registry:
        outer_ty = outer_instance_rust_type(outer_bin, class_type_params, registry)
        if outer_ty:
            types[0], recovered = outer_ty, True

    if (not m.generic_signature and registry and is_anonymous_class(ci)
            and ci.super_class in registry):
        super_desc, forwarded = _forwarded_super_ctor_params(ci, m)
        parent_ci = registry[ci.super_class]
        parent_m = next((pm for pm in parent_ci.methods
                         if pm.name == '<init>' and pm.descriptor == super_desc), None)
        if parent_m is not None and forwarded:
            parent_params = effective_class_type_params(parent_ci, registry)
            parent_types = constructor_sig_types(parent_ci, parent_m, parent_params, registry)
            parent_args = superclass_type_args(ci, registry)
            if len(parent_args) != len(parent_params):
                parent_args = ['Object'] * len(parent_params)
            mapping = dict(zip(parent_params, parent_args))
            for super_idx, own_idx in forwarded.items():
                if super_idx < len(parent_types):
                    types[own_idx] = substitute_type_params(parent_types[super_idx], mapping)
                    recovered = True

    return types if recovered else []


def receiver_member_name(m_name: str, m_descriptor: str, recv_ci, registry) -> str:
    """继承成员在接收者 wrapper 上的名字 = 接收者重载态下的 mangle 名（K-6）。

    调用侧统一按接收者重载态命名（_mangle_if_overloaded ← 本函数的判定来源
    hierarchy_overloaded_names 单调继承）。跨分支重载发散时（cancel()V 只存在于
    一条祖先分支、cancel(Z)Z 只存在于另一条）接收者态与声明者态不同名：
    wrapper 成员名必须取接收者态（调用点可解析），而 vtable 槽位名是声明者
    trait 的成员名（声明者态），经 vtable_name 属性传给宏（槽位填充分派 /
    wrapper UFCS 分派）。两态一致时（常态）二者同名为 mangle_name 结果或裸名；
    Rust 关键词名（type / match 等）经 safe_ident 转义（type_），与调用侧
    _safe_field 及定义侧 gen_method_body 的 safe_ident 同一约定。"""
    if m_name in hierarchy_overloaded_names(recv_ci, registry):
        return _safe_ident(mangle_name(m_name, m_descriptor))
    return _safe_ident(m_name)


def method_sig_types(ci, m, class_type_params: list[str], registry=None) -> tuple[list[str], str]:
    """方法在 Rust 侧的泛型签名类型（参数表, 返回类型）；([], '') 表示退回描述符擦除形态。

    覆盖方法（超类链上存在同名同描述符的非私有声明）的签名由**最远祖先的声明**决定：
    Rust trait impl 要求与 trait 声明逐字一致，而 Java 允许子类在擦除相同的前提下
    把 `Object key()` 重声明为 `K key()`。祖先声明的类型按祖先形参 → 本类视角实参
    （ancestor_type_args）替换后作为本方法签名；祖先声明无泛型签名 → 退回描述符。
    方法定义侧（方法体 / native 存根）与所有调用侧统一经此函数取签名。
    """
    if m.name == '<init>':
        return constructor_sig_types(ci, m, class_type_params, registry), '()'
    root_ci, root_m, root_args = None, None, []
    if registry and not m.is_static and not m.is_constructor and not (m.access_flags & _ACC_PRIVATE):
        for anc_bin, args in ancestor_type_args(ci, registry):
            anc = registry.get(anc_bin)
            if anc is None:
                break
            for am in anc.methods:
                if (am.name == m.name and am.descriptor == m.descriptor
                        and not am.is_static and not (am.access_flags & _ACC_PRIVATE)):
                    root_ci, root_m, root_args = anc, am, args
                    break
    if root_m is None:
        if not m.generic_signature:
            return [], ''
        return parse_method_param_types(m.generic_signature, class_type_params, registry,
                                        is_static=m.is_static)
    if not root_m.generic_signature:
        return [], ''
    root_tparams = effective_class_type_params(root_ci, registry)
    params, ret = parse_method_param_types(root_m.generic_signature, root_tparams, registry,
                                           is_static=False)
    if len(root_args) != len(root_tparams):
        root_args = ['Object'] * len(root_tparams)
    mapping = dict(zip(root_tparams, root_args))
    return ([substitute_type_params(p, mapping) for p in params],
            substitute_type_params(ret, mapping) if ret else ret)


# ══════════════════════════════════════════════════════════════════════════════
# 发射签名单一来源（K-6）：方法最终写进 Rust impl 块的参数 / 返回类型串
# ══════════════════════════════════════════════════════════════════════════════

_SIG_TYPE_BUILTIN = frozenset({
    'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
    'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell', 'usize', 'u8',
    'JArray',  # Rust 端数组包装，不对应 Java 类
})

_IFACE_SHORTS_CACHE: dict[int, frozenset[str]] = {}


def registry_iface_shorts(registry: 'dict | None') -> frozenset[str]:
    """registry 中所有接口的 Rust 短名（含 $→_ 替换），按 id(registry) 缓存。
    方法签名中的接口类型按 T-2 擦除为描述符形态，判定依赖此集合。"""
    if not registry:
        return frozenset()
    key = id(registry)
    cached = _IFACE_SHORTS_CACHE.get(key)
    if cached is not None:
        return cached
    shorts = frozenset(
        _short_cls_g(bin_name) for bin_name, ci in registry.items()
        if getattr(ci, 'is_interface', False)
    )
    _IFACE_SHORTS_CACHE[key] = shorts
    return shorts


def sig_type_string_valid(sp: str, class_tparams, registry: 'dict | None') -> bool:
    """generic_signature 派生的类型串是否可直接用于发射签名。
    满足以下任一条件则有效：
      1. sp 是类级类型参数（T/E/K/V 等）
      2. sp 中所有标识符均为已知类型（内建 / 类级参数 / 注册表中存在 / 大写开头的 Java 类名）
    大写开头名称视为合法 Java 短类名（可能由 glob import 引入），只拒绝
    全小写且不在内建集合的标识符（如未知 Rust 语法符）。"""
    if sp in class_tparams:
        return True
    import re as _re
    for name in _re.findall(r'[A-Za-z_][A-Za-z0-9_]*', sp):
        if name in _SIG_TYPE_BUILTIN or name in class_tparams:
            continue
        if registry and name in {_short_cls_g(k) for k in registry}:
            continue
        return False
    return True


def sig_type_string_is_iface(sp: str, registry: 'dict | None') -> bool:
    """类型串的首个标识符是否为 registry 中的接口（T-2：方法签名中接口类型
    擦除为描述符形态）。"""
    m = re.match(r'^(\w+)(?:<|$)', sp)
    return bool(m and m.group(1) in registry_iface_shorts(registry))


def emitted_method_sig_types(ci, m, class_type_params: list, registry=None) -> tuple[list[str], str]:
    """方法发射到 Rust impl 块的最终参数 / 返回类型串 —— gen_method_body 与
    vtable 擦除名单（K-6）的单一来源。

    method_sig_types 的结果逐位过滤：类型串无效或首标识符是接口 → 回退该位的
    描述符形态（jvm_to_rust）；泛型签名缺失 / 参数位数不符 → 整体描述符形态。
    vtable_erasure 条目按 token 全等匹配发射签名，必须与这里逐字一致。"""
    sig_params, sig_ret = method_sig_types(ci, m, class_type_params, registry)
    if sig_params and len(sig_params) == len(m.param_types):
        jps = [jvm_to_rust(t, registry) for t in m.param_types]
        params = [
            sp if (sig_type_string_valid(sp, class_type_params, registry)
                   and not sig_type_string_is_iface(sp, registry)) else jp
            for sp, jp in zip(sig_params, jps)
        ]
    else:
        params = [jvm_to_rust(t, registry) for t in m.param_types]
    if sig_ret and sig_type_string_valid(sig_ret, class_type_params, registry) \
            and not sig_type_string_is_iface(sig_ret, registry):
        ret = sig_ret
    else:
        ret = jvm_to_rust(m.return_type, registry)
    return params, ret


# ══════════════════════════════════════════════════════════════════════════════
# RsType 化：jvm_to_rs_type（Arch-7）
# ══════════════════════════════════════════════════════════════════════════════

# _PRIMITIVE_RUST 已统一到 codegen/constants.py 的 PRIMITIVE_RUST_TYPES（顶部 import 为 _PRIMITIVE_RUST）


def _rust_str_to_rs_type(rust_str: str) -> 'RsType':
    """将 Rust 类型字符串转换为 RsType 节点。"""
    from .rs_ir import RsPrimitive, RsNamed, RsGeneric
    if rust_str in _PRIMITIVE_RUST:
        return RsPrimitive(rust_str)
    if '<' not in rust_str:
        return RsNamed(rust_str)
    # 解析泛型类型：找最外层 '<' 的位置
    outer_end = rust_str.index('<')
    outer = rust_str[:outer_end]
    inner = rust_str[outer_end + 1: rust_str.rindex('>')]
    # 在深度 0 处按 ',' 分割参数
    params_strs: list[str] = []
    depth = 0
    start = 0
    for idx, ch in enumerate(inner):
        if ch == '<':
            depth += 1
        elif ch == '>':
            depth -= 1
        elif ch == ',' and depth == 0:
            params_strs.append(inner[start:idx].strip())
            start = idx + 1
    params_strs.append(inner[start:].strip())
    params = [_rust_str_to_rs_type(p) for p in params_strs if p]
    return RsGeneric(outer, params)


def jvm_to_rs_type(
    desc: str,
    generic_sig: str = '',
    class_type_params: list[str] | None = None,
    registry: dict | None = None,
) -> 'RsType':
    """JVM 类型描述符或泛型签名 → RsType 节点。

    优先使用 generic_sig（若非空），降级到 desc。
    generic_sig 是方法/字段级别的 Signature（单个类型，非 '(...) 格式'）。
    """
    _tparams = class_type_params or []
    if generic_sig:
        try:
            rust_str, _ = _parse_one_type(generic_sig, 0, _tparams, registry)
        except Exception:
            rust_str = jvm_to_rust(desc, registry)
    else:
        rust_str = jvm_to_rust(desc, registry)
    return _rust_str_to_rs_type(rust_str)


def infer_type_args_from_declared(actual_bin: str, declared_sig: str,
                                  class_type_params: list[str], registry) -> 'list[str] | None':
    """菱形构造 `List<Class<?>> xs = new ArrayList<>()`：由局部变量声明的泛型签名反推被构造类的
    类型实参（javac 的菱形推断结果不在字节码里，但声明类型在 LocalVariableTypeTable 里）。

    声明类型是被构造类自身 / 祖先类 / 已实现接口的某个实例化；把该超类型在被构造类形参
    视角下的实参（`List<E>`）与声明实参逐位对齐，解出每个形参。任一形参无解 → None。"""
    if not registry or not declared_sig.startswith('L') or '<' not in declared_sig:
        return None
    lt = declared_sig.index('<')
    declared_bin = declared_sig[1:lt]
    ci = registry.get(actual_bin)
    if ci is None or declared_bin not in registry:
        return None
    try:
        declared_args, _ = _parse_type_args(declared_sig, lt, class_type_params, registry)
    except Exception:
        return None
    params = list(effective_class_type_params(ci, registry) or [])
    if not params:
        return None
    views: dict[str, list[str]] = {actual_bin: list(params)}
    for anc_bin, anc_args in ancestor_type_args(ci, registry):
        views.setdefault(anc_bin, list(anc_args))
    for iface_bin, iface_args in implemented_interface_views(ci, registry):
        views.setdefault(iface_bin, list(iface_args))
    view = views.get(declared_bin)
    if view is None or len(view) != len(declared_args):
        return None
    solved: dict[str, str] = {}
    for view_arg, declared_arg in zip(view, declared_args):
        if view_arg in params:
            solved.setdefault(view_arg, declared_arg)
    if any(p not in solved for p in params):
        return None
    return [solved[p] for p in params]
