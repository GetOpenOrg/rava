"""
JVM 类型描述符 → Rust 类型的映射与解析工具。

包含：
  - JVM descriptor → Rust 类型字符串（jvm_to_rust）
  - JVM descriptor / Generic Signature → RsType 节点（jvm_to_rs_type）
  - Generic Signature 解析器（原 sig_parser.py，已内联）
"""

from __future__ import annotations
import re
from .constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST, OBJECT_CLASS as _OBJECT_CLASS


# ── JVM descriptor → Rust 类型 ──────────────────────────────────
# 注：String 是 java::lang::String（通过 prelude 引入），
#     它遮蔽 Rust 的 std::string::String，符合 Java 命名空间同构要求。
JVM_RUST: dict[str, str] = {
    'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64', 'Z': 'bool',
    'B': 'i8',  'S': 'i16', 'C': 'u16', 'V': '()',
    'Ljava/lang/Object;':  'Object',
    # 注：Ljava/lang/Class; 不做硬编码 —— 走 registry 泛型推导得到
    # Class<Object>（jvm_to_rust 的 registry 分支），与 generic_signature
    # 解析（_parse_one_type → Class<Object>）保持一致；registry 缺 Class
    # 时 fallback 到 Object（与反射擦除语义等价）。
    'Ljava/lang/Integer;': 'i32',
    'Ljava/lang/Long;':    'i64',
    'Ljava/lang/Double;':  'f64',
    'Ljava/lang/Boolean;': 'bool',
    '[I': 'JArray<i32>', '[J': 'JArray<i64>',
    '[F': 'JArray<f32>', '[D': 'JArray<f64>',
    '[B': 'JArray<i8>',  '[S': 'JArray<i16>',
    '[C': 'JArray<u16>', '[Z': 'JArray<bool>',
    '[Ljava/lang/String;': 'JArray<String>',
    '[Ljava/lang/Object;': 'JArray<Object>',
}

# newarray 操作数 → (Rust 元素类型, 零值字面量)
NEWARRAY_TYPES: dict[str, tuple[str, str]] = {
    'int':     ('i32',  '0i32'),
    'long':    ('i64',  '0i64'),
    'float':   ('f32',  '0f32'),
    'double':  ('f64',  '0f64'),
    'byte':    ('i8',   '0i8'),
    'short':   ('i16',  '0i16'),
    'char':    ('u16',  '0u16'),
    'boolean': ('bool', 'false'),
}


# 装箱方法（调用端透明：保留栈顶值不变）
BOXING_SKIP_STATIC: set[str] = {
    'java/lang/Integer.valueOf', 'java/lang/Long.valueOf',
    'java/lang/Double.valueOf',  'java/lang/Float.valueOf',
    'java/lang/Boolean.valueOf',
    'Integer.valueOf', 'Long.valueOf', 'Double.valueOf',
    'Float.valueOf',   'Boolean.valueOf',
}

# 拆箱方法（虚方法，返回原始值）
UNBOX_VIRTUAL: set[str] = {
    'intValue', 'longValue', 'doubleValue',
    'floatValue', 'booleanValue', 'byteValue', 'shortValue',
}


# ── 工具函数 ────────────────────────────────────────────────────

def jvm_to_rust(t: str, registry: dict | None = None) -> str:
    if t in JVM_RUST:
        return JVM_RUST[t]
    if t.startswith('L') and t.endswith(';'):
        inner = t[1:-1]
        # 只有 registry 中已翻译的类才用具体名称，否则 fallback 到 Object
        if registry is not None and inner in registry:
            name = short_cls(inner)
            if not name:
                return 'Object'
            ci = registry[inner]
            # Arch-1：接口 = Object 类型别名，用全路径避免与 Rust prelude 冲突
            if ci.is_interface:
                return _iface_full_path(inner)
            if ci.generic_signature:
                tparams = parse_class_type_params(ci.generic_signature)
                if tparams:
                    # 所有类型参数填 Object（JVM 类型擦除的 Rust 表现）
                    objects = ', '.join('Object' for _ in tparams)
                    return f"{name}<{objects}>"
            return name
        return 'Object'
    if t.startswith('['):
        elem = jvm_to_rust(t[1:], registry)
        return f'JArray<{elem}>'
    return 'Object'


def sig_type(rt: str) -> str:
    """函数签名中数组类型直接传递（Rc clone 语义，共享所有权）"""
    return rt


def rust_default(rt: str) -> str:
    return {
        'i32': '0', 'i64': '0', 'f32': '0.0', 'f64': '0.0',
        'bool': 'false',
        'String': 'String::default()',
    }.get(rt, 'Default::default()')


def is_jdk(cls: str) -> bool:
    """判断是否为 JDK 类（纯路径检查：含 / 的是 JDK 类）。"""
    return '/' in cls


def short_cls(cls: str) -> str:
    name = cls.split('/')[-1].split('.')[-1] if cls else ''
    return name.replace('$', '_')


def _iface_full_path(jvm_name: str) -> str:  # noqa: ARG001
    """接口类型 = Object（Arch-1：接口 = Object 类型别名，T-2 合规版本不含硬编码 JDK 名）。
    方法签名中接口类型的擦除由 method_gen.py / codegen.py 用 _registry_iface_shorts 完成。
    """
    return 'Object'


def parse_descriptor_params(desc: str) -> list[str]:
    m = re.match(r'\(([^)]*)\)', desc)
    return _parse_type_list(m.group(1) if m else '')


def parse_descriptor_return(desc: str) -> str:
    m = re.match(r'\([^)]*\)(.*)', desc)
    return m.group(1) if m else 'V'


_PRIM_SUFFIX: dict[str, str] = {
    'I': 'i', 'J': 'l', 'Z': 'z', 'B': 'b',
    'S': 's', 'F': 'f', 'D': 'd', 'C': 'c',
}
_CLS_ABBREV: dict[str, str] = {
    'object': 'obj',   'string': 'str',    'integer': 'int',
    'long': 'lng',     'double': 'dbl',    'boolean': 'bool',
    'charsequence': 'seq', 'stringbuilder': 'sb', 'comparable': 'cmp',
    'iterable': 'iter', 'collection': 'coll', 'list': 'list',
    'map': 'map',      'set': 'set',       'number': 'num',
}


def descriptor_to_suffix(descriptor: str) -> str:
    """把描述符参数部分 '(ITE;)V' 转成后缀字符串（不含 __），如 'i_e'。"""
    m = re.match(r'\(([^)]*)\)', descriptor)
    if not m:
        return ''
    s = m.group(1)
    if not s:
        return ''
    parts: list[str] = []
    i = 0
    while i < len(s):
        c = s[i]
        if c in _PRIM_SUFFIX:
            parts.append(_PRIM_SUFFIX[c]); i += 1
        elif c == 'T':
            try:
                end = s.index(';', i + 1)
                parts.append(s[i+1:end].lower()); i = end + 1
            except ValueError:
                i += 1
        elif c == 'L':
            try:
                end = s.index(';', i + 1)
                short = s[i+1:end].split('/')[-1].lower().replace('$', '_')
                parts.append(_CLS_ABBREV.get(short, short[:6])); i = end + 1
            except ValueError:
                i += 1
        elif c == '[':
            j = i + 1
            while j < len(s) and s[j] == '[':
                j += 1
            if j < len(s) and s[j] == 'L':
                try:
                    end = s.index(';', j + 1)
                    short = s[j+1:end].split('/')[-1].lower().replace('$', '_')
                    parts.append('arr_' + _CLS_ABBREV.get(short, short[:3])); i = end + 1
                except ValueError:
                    i = j + 1
            elif j < len(s) and s[j] == 'T':
                try:
                    end = s.index(';', j + 1)
                    parts.append('arr_' + s[j+1:end].lower()); i = end + 1
                except ValueError:
                    i = j + 1
            elif j < len(s):
                parts.append('arr_' + _PRIM_SUFFIX.get(s[j], 'x')); i = j + 1
            else:
                i += 1
        else:
            i += 1
    return '_'.join(parts)


def mangle_name(name: str, descriptor: str) -> str:
    """方法名 + 描述符 → 含后缀的唯一 Rust 名，无参数时返回原名。
    使用单下划线分隔符（`add_obj` 而非 `add__obj`），留出双下划线给 Rust 保留名。"""
    suffix = descriptor_to_suffix(descriptor)
    return f"{name}_{suffix}" if suffix else name



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


def _parse_type_list(s: str) -> list[str]:
    types, i = [], 0
    while i < len(s):
        c = s[i]
        if c in 'BCDFIJSZV':
            types.append(c); i += 1
        elif c == 'L':
            end = s.index(';', i)
            types.append(s[i:end+1]); i = end + 1
        elif c == '[':
            j = i + 1
            while j < len(s) and s[j] == '[':
                j += 1
            if s[j] == 'L':
                end = s.index(';', j)
                types.append(s[i:end+1]); i = end + 1
            else:
                types.append(s[i:j+1]); i = j + 1
        else:
            i += 1
    return types


# ══════════════════════════════════════════════════════════════════════════════
# Generic Signature 解析器（原 sig_parser.py，已内联）
# 实现 JVMS §4.7.9 Generic Signature 解析
# ══════════════════════════════════════════════════════════════════════════════

# 已知类名 → Rust 类型映射
_CLASSNAME_MAP: dict[str, str] = {
    'java/lang/String':        'String',
    _OBJECT_CLASS:             'Object',
    'java/lang/CharSequence':  'Object',
    # 特判：Class 非泛化（类型参数纯 phantom，类级签名已在 classfile.py
    # 置空）。mapped 分支忽略 type_args，使 Ljava/lang/Class<*>; → Class，
    # 与 jvm_to_rust 的 registry 分支（裸 Class）保持一致。
    'java/lang/Class':         'Class',
    'java/lang/Integer':       'i32',
    'java/lang/Long':          'i64',
    'java/lang/Double':        'f64',
    'java/lang/Float':         'f32',
    'java/lang/Boolean':       'bool',
    'java/lang/StringBuilder': 'String',
    'java/lang/StringBuffer':  'String',
}

# 基本类型映射
_PRIMITIVE_MAP: dict[str, str] = {
    'V': '()', 'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64',
    'Z': 'bool', 'B': 'i8', 'C': 'u16', 'S': 'i16',
}


def _skip_field_type_sig(sig: str, i: int) -> int:
    """跳过一个 FieldTypeSig，返回跳过后的位置。"""
    if i >= len(sig):
        return i
    c = sig[i]
    if c in _PRIMITIVE_MAP:
        return i + 1
    if c == 'T':
        try:
            end = sig.index(';', i + 1)
            return end + 1
        except ValueError:
            return len(sig)
    if c == 'L':
        j = i + 1
        depth = 0
        while j < len(sig):
            ch = sig[j]
            if ch == '<':
                depth += 1
            elif ch == '>':
                depth -= 1
            elif ch == ';' and depth == 0:
                return j + 1
            j += 1
        return j
    if c == '[':
        return _skip_field_type_sig(sig, i + 1)
    if c in ('+', '-'):
        return _skip_field_type_sig(sig, i + 1)
    if c == '*':
        return i + 1
    return i + 1


def _parse_type_args(sig: str, i: int, class_type_params: list[str], registry=None,
                     method_bounds: 'dict | None' = None) -> tuple[list[str], int]:
    """解析 <TypeArgument*>，i 指向 '<'。返回 (类型字符串列表, '>' 之后的位置)。"""
    i += 1  # 跳过 '<'
    args: list[str] = []
    while i < len(sig) and sig[i] != '>':
        if sig[i] == '*':
            args.append('Object')
            i += 1
        elif sig[i] in ('+', '-'):
            t, i = _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)
            args.append(t)
        else:
            t, i = _parse_one_type(sig, i, class_type_params, registry, method_bounds)
            args.append(t)
    if i < len(sig) and sig[i] == '>':
        i += 1  # 跳过 '>'
    return args, i


def _extract_method_tparam_bounds(sig: str, registry=None) -> 'dict[str, str]':
    """从方法级泛型参数段 '<T:Ljava/lang/Number;...>' 提取类型变量 → 上界 Rust 类型的映射。
    传入 registry 时，接口上界经 _iface_full_path 返回 'Object' 被自动过滤，
    只保留具体类上界（如 Number → "Number"，Enum<T> → "Enum<Object>"）。
    """
    if not sig or sig[0] != '<':
        return {}
    bounds: dict[str, str] = {}
    i = 1  # 跳过 '<'
    depth = 1
    while i < len(sig) and depth > 0:
        c = sig[i]
        if c == '<':
            depth += 1
            i += 1
        elif c == '>':
            depth -= 1
            i += 1
        elif c.isalpha() or c == '_':
            # 读取类型变量名（到 ':'）
            j = i
            while j < len(sig) and sig[j] not in (':', '<', '>'):
                j += 1
            name = sig[i:j]
            i = j
            # 解析上界列表：:classbound(:iface1)* 格式
            first_class_bound: 'str | None' = None
            while i < len(sig) and sig[i] == ':':
                i += 1  # 跳过 ':'
                if i < len(sig) and sig[i] in ('L', '['):
                    # 传入 registry：接口类型 → _iface_full_path → 'Object' → 被过滤
                    # 具体类（Number/Enum 等）→ 正确类型（"Number"、"Enum<Object>"）
                    rust_t, i = _parse_one_type(sig, i, [], registry)
                    if first_class_bound is None and rust_t and rust_t != 'Object':
                        first_class_bound = rust_t
                elif i < len(sig) and sig[i] == 'T':
                    # 以另一类型变量作为上界，直接跳过
                    _, i = _parse_one_type(sig, i, [], None)
                # else: 空类上界（'::'）直接继续下一轮循环
            if name and first_class_bound:
                bounds[name] = first_class_bound
        else:
            i += 1
    return bounds


def _parse_one_type(sig: str, i: int, class_type_params: list[str], registry=None,
                    method_bounds: 'dict | None' = None) -> tuple[str, int]:
    """从 sig[i] 起解析一个类型（Generic Signature 格式），返回 (rust_type, next_i)。"""
    if i >= len(sig):
        return 'Object', i

    c = sig[i]

    if c in _PRIMITIVE_MAP:
        return _PRIMITIVE_MAP[c], i + 1

    if c == 'T':
        # TypeVariable
        try:
            end = sig.index(';', i + 1)
        except ValueError:
            return 'Object', len(sig)
        name = sig[i + 1:end]
        if name in class_type_params:
            return name, end + 1
        if method_bounds and name in method_bounds:
            # 方法级类型变量 → 使用其上界（如 T extends Number → Number）
            return method_bounds[name], end + 1
        return 'Object', end + 1

    if c == '[':
        # 数组 → JArray<elem>
        elem_type, next_i = _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)
        return f'JArray<{elem_type}>', next_i

    if c == '+' or c == '-':
        # 上下界通配符 — 取内部类型
        return _parse_one_type(sig, i + 1, class_type_params, registry, method_bounds)

    if c == '*':
        # 无界通配符
        return 'Object', i + 1

    if c == 'L':
        # ClassTypeSig: L<classname>(<TypeArgs>)?;
        j = i + 1
        while j < len(sig) and sig[j] not in ('<', ';', '.'):
            j += 1
        class_name = sig[i + 1:j]

        type_args: list[str] = []
        has_type_args = j < len(sig) and sig[j] == '<'
        if has_type_args:
            type_args, j = _parse_type_args(sig, j, class_type_params, registry, method_bounds)

        # ClassTypeSigSuffix（`Outer<TE;>.Inner<TX;>`）：类型是内部类 Outer$Inner，
        # 不是外部类。实参按内部类的有效形参选取：自带形参 → 本段实参；
        # 形参继承自外部类（非静态内部类）→ 外层段实参。
        if j < len(sig) and sig[j] == '.':
            outer_args = type_args
            while j < len(sig) and sig[j] == '.':
                k = j + 1
                while k < len(sig) and sig[k] not in ('<', ';', '.'):
                    k += 1
                class_name = class_name + '$' + sig[j + 1:k]
                j = k
                type_args = []
                if j < len(sig) and sig[j] == '<':
                    type_args, j = _parse_type_args(sig, j, class_type_params, registry, method_bounds)
            inner_ci = registry.get(class_name) if registry else None
            if inner_ci is not None:
                inner_own = (parse_class_type_params(inner_ci.generic_signature)
                             if inner_ci.generic_signature else [])
                inner_eff = effective_class_type_params(inner_ci, registry)
                if not inner_own:
                    type_args = outer_args
                if len(type_args) != len(inner_eff):
                    type_args = ['Object'] * len(inner_eff)
            has_type_args = bool(type_args)

        # 跳过结尾 ';'
        if j < len(sig) and sig[j] == ';':
            j += 1

        # 映射类名到 Rust 类型
        mapped = _CLASSNAME_MAP.get(class_name)
        if mapped is not None:
            rust_type = mapped
        else:
            short = class_name.rsplit('/', 1)[-1].replace('$', '_')
            # Arch-1：接口 = Object 类型别名，用全路径避免与 Rust prelude 冲突
            if registry and class_name in registry and registry[class_name].is_interface:
                rust_type = _iface_full_path(class_name)
            elif has_type_args:
                rust_type = f"{short}<{', '.join(type_args)}>"
            else:
                rust_type = short

        return rust_type, j

    # 未知 — 前进一步
    return 'Object', i + 1


# ── Generic Signature 公开 API ──────────────────────────────────

def parse_field_type(sig: str, class_type_params: list[str], registry=None) -> str:
    """从字段级 Signature 解析 Rust 类型字符串。"""
    if not sig:
        return ''
    try:
        rust_type, _ = _parse_one_type(sig, 0, class_type_params, registry)
        return rust_type
    except Exception:
        return ''


def parse_class_type_params(sig: str) -> list[str]:
    """从类级 Signature 中提取类型参数名列表。

    示例：
      '<E:Ljava/lang/Object;>...'     → ['E']
      '<K:Ljava/lang/Object;V:...>'   → ['K', 'V']
      'Ljava/lang/Object;'            → []
      ''                              → []
    """
    if not sig or sig[0] != '<':
        return []

    params: list[str] = []
    i = 1  # 跳过开头的 '<'

    try:
        while i < len(sig) and sig[i] != '>':
            j = i
            while j < len(sig) and sig[j] != ':' and sig[j] != '>':
                j += 1
            if j >= len(sig) or sig[j] == '>':
                break
            name = sig[i:j]
            if name:
                params.append(name)
            i = j + 1  # 跳过 ':'

            # 跳过 ClassBound（可为空）
            if i < len(sig) and sig[i] not in (':', '>'):
                i = _skip_field_type_sig(sig, i)

            # 跳过所有 InterfaceBound（每个以 ':' 开头）
            while i < len(sig) and sig[i] == ':':
                i += 1  # 跳过 ':'
                if i < len(sig) and sig[i] not in (':', '>'):
                    i = _skip_field_type_sig(sig, i)
    except Exception:
        pass  # 解析失败时返回已收集的部分

    return params


def effective_class_type_params(ci, registry=None) -> list[str]:
    """类在 Rust 侧的有效类型参数表。

    - 类自身 Signature 声明了形参 → 用自身形参
    - 否则若是内部类（持有 this$N 字段）→ 继承外部类的形参
      （Java 内部类隐式可见外部类类型变量，Rust struct 必须显式声明）
    """
    own = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    if own or not registry:
        return own
    for f in ci.fields:
        if f.is_static or not re.match(r'^this\$\d+$', f.name):
            continue
        m = re.match(r'L([^;]+);', f.descriptor)
        if m:
            outer_ci = registry.get(m.group(1))
            if outer_ci is not None and outer_ci.generic_signature:
                return list(parse_class_type_params(outer_ci.generic_signature))
        break
    return []


def _superclass_sig_segments(sig: str, class_type_params: list[str], registry=None) -> list[list[str]]:
    """解析类级 Signature 中 SuperclassSignature 的各段类型实参。

    `<...>Lpkg/Outer<TE;>.Inner<TX;>;...` → [['E'], ['X']]；无实参的段为 []。
    """
    i = 0
    if sig.startswith('<'):
        depth = 0
        while i < len(sig):
            if sig[i] == '<':
                depth += 1
            elif sig[i] == '>':
                depth -= 1
                if depth == 0:
                    i += 1
                    break
            i += 1
    if i >= len(sig) or sig[i] != 'L':
        return []
    segments: list[list[str]] = []
    j = i + 1
    while j < len(sig):
        while j < len(sig) and sig[j] not in ('<', ';', '.'):
            j += 1
        if j < len(sig) and sig[j] == '<':
            args, j = _parse_type_args(sig, j, class_type_params, registry)
            segments.append(args)
        else:
            segments.append([])
        if j < len(sig) and sig[j] == '.':
            j += 1
            continue
        break
    return segments


def _type_arg_is_resolvable(rust_ty: str, class_type_params: list[str], registry) -> bool:
    """类型实参中出现的每个类型名都必须可解析（类型参数 / 内建 / registry 中的类且元数正确）。"""
    short_names = _registry_short_index(registry)
    mapped = set(_CLASSNAME_MAP.values()) | set(_PRIMITIVE_MAP.values()) | {'JArray'}
    for m in re.finditer(r'([A-Za-z_]\w*)\s*(<)?', rust_ty):
        name, has_args = m.group(1), bool(m.group(2))
        if name in class_type_params or name in mapped:
            continue
        ref_ci = short_names.get(name)
        if ref_ci is None:
            return False
        # 裸用泛型类（raw type）在 Rust 中缺实参 → 不可用
        if bool(effective_class_type_params(ref_ci, registry)) != has_args:
            return False
    return True


_SHORT_INDEX_CACHE: dict[int, tuple[int, dict]] = {}


def _registry_short_index(registry) -> dict:
    """registry 的 Rust 短名 → ClassInfo 索引（按 registry 身份 + 大小缓存）。"""
    if not registry:
        return {}
    cached = _SHORT_INDEX_CACHE.get(id(registry))
    if cached is not None and cached[0] == len(registry):
        return cached[1]
    index = {short_cls(k): v for k, v in registry.items()}
    _SHORT_INDEX_CACHE[id(registry)] = (len(registry), index)
    return index


def superclass_type_args(ci, registry) -> list[str]:
    """直接父类的 Rust 类型实参，以 ci 的有效类型参数表达。

    实参取自 ci 类级 Signature 的 SuperclassSignature（而非按位置把子类形参套给父类）：
      `<P_IN,P_OUT> extends AbstractPipeline<P_IN,P_OUT,Stream<P_OUT>>` → ['P_IN','P_OUT','Object']
      `extends RecursiveTask<BigInteger>`                               → ['BigInteger']
      `extends AbstractList<E>.Itr`（父类形参继承自其外部类）           → ['E']
    无法解析的实参（raw type、通配符、未翻译的类）取 Object（类型擦除的真实多态边界）。
    """
    if not registry or not ci.super_class or ci.super_class not in registry:
        return []
    parent_ci = registry[ci.super_class]
    parent_params = effective_class_type_params(parent_ci, registry)
    if not parent_params:
        return []
    own_params = effective_class_type_params(ci, registry)
    args: list[str] = []
    if ci.generic_signature:
        segments = _superclass_sig_segments(ci.generic_signature, own_params, registry)
        parent_has_own = bool(parse_class_type_params(parent_ci.generic_signature)
                              if parent_ci.generic_signature else [])
        non_empty = [s for s in segments if s]
        if non_empty:
            # 父类自带形参 → 末段实参；父类形参继承自外部类 → 外层段实参
            args = non_empty[-1] if parent_has_own else non_empty[0]
    if len(args) != len(parent_params):
        args = ['Object'] * len(parent_params)
    return [a if _type_arg_is_resolvable(a, own_params, registry) else 'Object' for a in args]


def outer_ref_field_type(f, decl_params: list, registry) -> str:
    """内部类外部引用字段（this$N）的 Rust 类型：外部类 + 声明类从外部类继承的类型参数
    （如 ArrayList$Itr.this$0 → ArrayList<E>）。非 this$N 字段或外部类非泛型 → ''。
    struct 字段定义（class_writer）与 getfield/putfield 的字段类型恢复共用。"""
    if not (re.match(r'^this\$\d+$', f.name) and decl_params and registry):
        return ''
    fm = re.match(r'L([^;]+);', f.descriptor)
    outer_ci = registry.get(fm.group(1)) if fm else None
    if outer_ci is None or not outer_ci.generic_signature:
        return ''
    outer_tp = parse_class_type_params(outer_ci.generic_signature)
    if not outer_tp or len(outer_tp) > len(decl_params):
        return ''
    return short_cls(fm.group(1)) + '<' + ', '.join(decl_params[:len(outer_tp)]) + '>'


def rust_type_with_args(short_name: str, args: list[str]) -> str:
    """`Name` + 实参表 → `Name<A, B>`；无实参时为裸名。"""
    return f"{short_name}<{', '.join(args)}>" if args else short_name


def substitute_type_params(rust_ty: str, mapping: dict) -> str:
    """把 Rust 类型字符串中的类型参数名按 mapping 同步替换。"""
    if not mapping:
        return rust_ty
    return re.sub(r'\b[A-Za-z_]\w*\b', lambda m: mapping.get(m.group(0), m.group(0)), rust_ty)


def ancestor_type_args(ci, registry, self_args: 'list[str] | None' = None) -> 'list[tuple[str, list[str]]]':
    """沿超类链解析每个祖先的 Rust 类型实参：[(ancestor_binary, [args]), ...]，直接父类在前。

    self_args 给出 ci 自身形参的实参（调用点静态类型，如 `X<Object>` → ['Object']）；
    缺省时以 ci 的形参自身表达（类定义内部视角）。不含 java.lang.Object，链在 registry 之外截断。
    """
    chain: list[tuple[str, list[str]]] = []
    if not registry:
        return chain
    own_params = effective_class_type_params(ci, registry)
    mapping: dict = {}
    if self_args is not None and len(self_args) == len(own_params):
        mapping = dict(zip(own_params, self_args))
    elif self_args is not None:
        mapping = {p: 'Object' for p in own_params}
    cur = ci
    seen: set[str] = set()
    while (cur.super_class and cur.super_class != _OBJECT_CLASS
           and cur.super_class in registry and cur.super_class not in seen):
        seen.add(cur.super_class)
        args = [substitute_type_params(a, mapping) for a in superclass_type_args(cur, registry)]
        parent_ci = registry[cur.super_class]
        chain.append((cur.super_class, args))
        mapping = dict(zip(effective_class_type_params(parent_ci, registry), args))
        cur = parent_ci
    return chain


def split_rust_type_args(rust_ty: str) -> list[str]:
    """`Name<A, B<C, D>>` → ['A', 'B<C, D>']（仅拆顶层逗号）；无实参 → []。"""
    lt = rust_ty.find('<')
    if lt < 0 or not rust_ty.rstrip().endswith('>'):
        return []
    inner = rust_ty[lt + 1:rust_ty.rstrip().rfind('>')]
    parts: list[str] = []
    depth = 0
    cur: list[str] = []
    for ch in inner:
        if ch == '<':
            depth += 1
        elif ch == '>':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(''.join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    tail = ''.join(cur).strip()
    if tail:
        parts.append(tail)
    return parts


def ancestor_vtable_args_by_short(ci, rust_ty: str, registry) -> 'dict[str, str]':
    """静态类型为 rust_ty（ci 的实例化，如 `X<Object>`）的接收者，其各祖先 VTable 的
    类型实参串：{祖先 Rust 短名 → '<A, B>' 或 ''}。供 UFCS `<dyn Anc__VTable<..>>::m(..)` 使用。"""
    return {short_cls(anc_bin): (f"<{', '.join(args)}>" if args else '')
            for anc_bin, args in ancestor_type_args(ci, registry, split_rust_type_args(rust_ty))}


def parse_method_param_types(
    sig: str,
    class_type_params: list[str],
    registry=None,
    is_static: bool = True,
) -> tuple[list[str], str]:
    """从方法 Signature 中解析参数类型和返回类型（Rust 类型字符串）。

    is_static：方法是否为 static。实例方法声明的方法级类型形参遮蔽同名的类级
      形参（Java 作用域规则：`class P<S> { <S extends Sink> S wrap(S s) }` 里的 S
      是方法级变量，与类级 S 无关）→ 解析时从类级形参表剔除，按方法级变量
      （上界 / Object）处理，与父类/接口侧同一方法的擦除形态一致。
      static 方法不能引用类级形参，其方法级 `<K,V>` 由所在 impl 块的同名
      形参承载（impl<K,V> X<K,V> { fn f(..: K) }），保留同名解析。
      定义侧与所有调用侧必须传入同一取值。

    class_type_params：类级类型参数名（如 ['E'] 或 ['K', 'V']）
    registry：类注册表，用于 Arch-1 接口擦除（接口类型参数 → Object）

    示例（class_type_params=['E']）：
      '(TE;)Z'    → (['E'], 'bool')
      '(I)TE;'    → (['i32'], 'E')
      '(TE;I)V'   → (['E', 'i32'], '()')

    遇到解析错误时返回 ([], '')。
    """
    if not sig:
        return [], ''

    try:
        i = 0

        # 解析方法级类型参数 <T:Ljava/lang/Number;...>，提取上界用于参数类型解析
        method_bounds: dict[str, str] = {}
        if i < len(sig) and sig[i] == '<':
            method_bounds = _extract_method_tparam_bounds(sig[i:], registry)
            if not is_static:
                _shadowed = set(parse_class_type_params(sig[i:]))
                if _shadowed & set(class_type_params):
                    class_type_params = [p for p in class_type_params if p not in _shadowed]
            depth = 1
            i += 1
            while i < len(sig) and depth > 0:
                if sig[i] == '<':
                    depth += 1
                elif sig[i] == '>':
                    depth -= 1
                i += 1

        # 期望 '('
        if i >= len(sig) or sig[i] != '(':
            return [], ''
        i += 1  # 跳过 '('

        # 解析参数类型（方法级类型变量使用上界替代 Object）
        param_types: list[str] = []
        while i < len(sig) and sig[i] != ')':
            rust_type, i = _parse_one_type(sig, i, class_type_params, registry, method_bounds or None)
            param_types.append(rust_type)

        if i < len(sig) and sig[i] == ')':
            i += 1  # 跳过 ')'

        # 解析返回类型（忽略 ThrowsSignature ^...）
        if i < len(sig) and sig[i] != '^':
            ret_type, _ = _parse_one_type(sig, i, class_type_params, registry, method_bounds or None)
        else:
            ret_type = '()'

        return param_types, ret_type

    except Exception:
        return [], ''


_ACC_PRIVATE = 0x0002


def method_sig_types(ci, m, class_type_params: list[str], registry=None) -> tuple[list[str], str]:
    """方法在 Rust 侧的泛型签名类型（参数表, 返回类型）；([], '') 表示退回描述符擦除形态。

    覆盖方法（超类链上存在同名同描述符的非私有声明）的签名由**最远祖先的声明**决定：
    Rust trait impl 要求与 trait 声明逐字一致，而 Java 允许子类在擦除相同的前提下
    把 `Object key()` 重声明为 `K key()`。祖先声明的类型按祖先形参 → 本类视角实参
    （ancestor_type_args）替换后作为本方法签名；祖先声明无泛型签名 → 退回描述符。
    方法定义侧（方法体 / native 存根）与所有调用侧统一经此函数取签名。
    """
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
