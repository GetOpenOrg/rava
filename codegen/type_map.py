"""
JVM 类型描述符 → Rust 类型的基础映射与解析工具（type_map 四分后的底层模块）。

包含：
  - JVM descriptor → Rust 类型字符串（jvm_to_rust）与基础映射表
  - 短名消歧（configure_short_names / short_cls）与 registry 短名索引
  - descriptor 解析（parse_descriptor_* / descriptor_to_suffix / mangle_name）
  - 类级泛型签名形参提取与有效形参表（parse_class_type_params /
    effective_class_type_params 及外围作用域簇 outer_instance_class /
    enclosing_method_info —— 属 jvm_to_rust 的依赖闭包，须留在本层）

依赖方向（下层，禁止 import 上层）：sig_parse → 本模块；
type_args → sig_parse → 本模块；sig_types → type_args → sig_parse → 本模块。
"""

from __future__ import annotations
import re


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
    # S-3.1：装箱类型（Integer/Long/...）不再拆平为原生值 —— 描述符
    # Ljava/lang/Integer; 是引用类型，走 registry 分支得到字节码翻译类；
    # 描述符 I（int）与 Ljava/lang/Integer;（Integer）在字节码里本就区分清晰。
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
            # Arch-1：接口 = Object 类型别名，用全路径避免与 Rust prelude 冲突。
            # A-4 批次 3+：已铺设载体化的接口发射擦除载体 `I<Object, ..>`
            # （判定单一来源 jvm_type.carrier_type，惰性 import 防回环）
            if ci.is_interface:
                from .jvm_type import carrier_type
                _carrier = carrier_type(inner, registry)
                if _carrier is not None:
                    return _carrier
                return _iface_full_path(inner)
            tparams = effective_class_type_params(ci, registry)
            if tparams:
                # 所有类型参数填 Object（JVM 类型擦除的 Rust 表现）；
                # 含内部类从外围作用域继承的类型参数
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


# 跨包同简单名的类：binary name → 带包限定的 Rust 类型名（由 configure_short_names 从 registry 算出）
_QUALIFIED_SHORT_NAMES: dict[str, str] = {}


def configure_short_names(registry: dict | None) -> None:
    """按 registry 计算 Rust 类型名的消歧表。

    Rust 类型名 = 类的简单名（`$` → `_`）。不同包的类简单名相同时（Java 靠包名区分，
    Rust 侧类型名字符串是类型身份的唯一载体，跨文件流动），同名组内按 binary name
    字典序最小者保留简单名，其余以完整 binary name（`/`、`$` → `_`）为 Rust 类型名——
    结构体、vtable、`use` 导入、类型串 ↔ binary 反查全部经 short_cls 取同一名字。
    """
    _QUALIFIED_SHORT_NAMES.clear()
    _SHORT_INDEX_CACHE.clear()
    groups: dict[str, list[str]] = {}
    for binary in (registry or {}):
        if '/' not in binary:
            continue
        groups.setdefault(binary.rsplit('/', 1)[-1].replace('$', '_'), []).append(binary)
    for members in groups.values():
        if len(members) < 2:
            continue
        for binary in sorted(members)[1:]:
            _QUALIFIED_SHORT_NAMES[binary] = binary.replace('/', '_').replace('$', '_')


def short_cls(cls: str) -> str:
    qualified = _QUALIFIED_SHORT_NAMES.get(cls)
    if qualified is not None:
        return qualified
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
    """把描述符参数部分 '(ITE;)V' 转成后缀字符串（不含 __），如 'i_e'。

    类类型用完整的小写简单名（内部类 $ → _），不截断：截断会让同前缀的不同类
    （如同一外部类的多个内部类、同词根的接口）映射到同一后缀，重载名撞名（E0428/E0201/E0592）。"""
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
                parts.append(_CLS_ABBREV.get(short, short)); i = end + 1
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
                    parts.append('arr_' + _CLS_ABBREV.get(short, short)); i = end + 1
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
    except (ValueError, IndexError):
        # B 组收窄（fallback-audit 方案 §4.1）：只兜签名残缺形态并计数；
        # 解析失败时返回已收集的部分（既有降级行为不变）
        from . import fallback_audit
        fallback_audit.record('type-map-params')

    return params


_ACC_MANDATED = 0x8000


def outer_instance_class(ci) -> str:
    """内部类实例所绑定的外部实例所属类（binary name）；无外部实例（静态上下文）→ ''。

    依据（全部来自字节码）：
      - 合成字段 this$N 的描述符；
      - javac 在外部实例未被使用时省略 this$N，但构造器仍保留该形参，并在
        MethodParameters 中标记 ACC_MANDATED —— 以 EnclosingMethod（局部 / 匿名类）
        或 InnerClasses（成员类）记录的直接外围类核对首个形参描述符。
    """
    for f in ci.fields:
        if not f.is_static and re.match(r'^this\$\d+$', f.name):
            m = re.match(r'L([^;]+);$', f.descriptor)
            if m:
                return m.group(1)
    enclosing = getattr(ci, 'enclosing_class', '')
    if not enclosing:
        for ic in ci.inner_classes or ():
            if ic.inner_class == ci.name:
                enclosing = ic.outer_class
                break
    if enclosing:
        for m in ci.methods:
            if m.name != '<init>' or not m.method_parameters:
                continue
            params = parse_descriptor_params(m.descriptor)
            if (params and params[0] == f'L{enclosing};'
                    and m.method_parameters[0][1] & _ACC_MANDATED):
                return enclosing
    return ''


def enclosing_method_info(ci, registry):
    """局部 / 匿名类的外围方法（EnclosingMethod attribute）→ ParsedMethod；无 → None。"""
    enclosing = getattr(ci, 'enclosing_class', '')
    em = getattr(ci, 'enclosing_method', None)
    if not (enclosing and em and registry):
        return None
    outer_ci = registry.get(enclosing)
    if outer_ci is None:
        return None
    for m in outer_ci.methods:
        if m.name == em[0] and m.descriptor == em[1]:
            return m
    return None


def effective_class_type_params(ci, registry=None) -> list[str]:
    """类在 Rust 侧的有效类型参数表 = 外围作用域的类型变量（被自身同名形参遮蔽者除外）+ 自身形参。

    Java 内部类隐式可见外围作用域的类型变量，Rust struct 必须显式声明：
      - 局部 / 匿名类（带 EnclosingMethod）→ 外部实例所属类的有效形参（实例上下文）
        + 外围方法的方法级形参（遮蔽同名外层形参）
      - 成员内部类（持有外部实例）→ 外部实例所属类的有效形参
    """
    own = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    if not registry:
        return own
    is_local = bool(getattr(ci, 'enclosing_class', ''))
    inherited: list[str] = []
    outer_bin = outer_instance_class(ci)
    outer_ci = registry.get(outer_bin) if outer_bin else None
    if outer_ci is not None and outer_ci is not ci:
        inherited = list(effective_class_type_params(outer_ci, registry))
    if is_local:
        em = enclosing_method_info(ci, registry)
        if em is not None and em.generic_signature:
            method_params = parse_class_type_params(em.generic_signature)
            inherited = [p for p in inherited if p not in method_params] + method_params
    return [p for p in inherited if p not in own] + own


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
