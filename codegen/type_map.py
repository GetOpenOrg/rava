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

        # 跳过 ClassTypeSigSuffix（.InnerClass…）
        while j < len(sig) and sig[j] == '.':
            j += 1
            while j < len(sig) and sig[j] not in ('<', ';', '.'):
                j += 1
            if j < len(sig) and sig[j] == '<':
                _, j = _parse_type_args(sig, j, class_type_params, registry, method_bounds)

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


def parse_method_param_types(
    sig: str,
    class_type_params: list[str],
    registry=None,
) -> tuple[list[str], str]:
    """从方法 Signature 中解析参数类型和返回类型（Rust 类型字符串）。

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
