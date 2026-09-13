"""
JVM 类型描述符 → Rust 类型的映射与解析工具。
"""

import os
import re
from .sig_parser import parse_class_type_params

# ── Ergonomic JVM 方法重命名注册表（T39）─────────────────────────────────────
# 由 load_ergonomic_renames() 在 codegen 启动前填充。
# 格式：class_binary_name → {java_method_name → rust_method_name}
_ERGONOMIC_JVM_RENAMES: dict[str, dict[str, str]] = {}


def load_ergonomic_renames(workspace_root: str) -> None:
    """扫描 native_impls 中所有 *_ergonomic.rs，提取 @jvm_class 和 @jvm_rename 指令。"""
    native_dir = os.path.join(workspace_root, 'native_impls')
    if not os.path.isdir(native_dir):
        return
    _pat_class  = re.compile(r'//\s*@jvm_class:\s*(\S+)')
    _pat_rename = re.compile(r'//\s*@jvm_rename:\s*(.+)')
    for dirpath, _, filenames in os.walk(native_dir):
        for fname in filenames:
            if not fname.endswith('_ergonomic.rs'):
                continue
            with open(os.path.join(dirpath, fname)) as f:
                content = f.read()
            m_cls = _pat_class.search(content)
            if not m_cls:
                continue
            class_name = m_cls.group(1).strip()
            renames: dict[str, str] = {}
            for m_rn in _pat_rename.finditer(content):
                for pair in m_rn.group(1).split(','):
                    parts = [p.strip() for p in pair.split('->')]
                    if len(parts) == 2 and parts[0] and parts[1]:
                        renames[parts[0]] = parts[1]
            if renames:
                _ERGONOMIC_JVM_RENAMES[class_name] = renames


def get_ergonomic_jvm_rename(class_binary_name: str, java_method_name: str) -> str | None:
    """若 @jvm_rename 指令存在，返回重命名后的 Rust 方法名；否则返回 None。"""
    return _ERGONOMIC_JVM_RENAMES.get(class_binary_name, {}).get(java_method_name)

# ── JVM descriptor → Rust 类型 ──────────────────────────────────
# 注：String 是 java::lang::String（通过 prelude 引入），
#     它遮蔽 Rust 的 std::string::String，符合 Java 命名空间同构要求。
JVM_RUST: dict[str, str] = {
    'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64', 'Z': 'bool',
    'B': 'i8',  'S': 'i16', 'C': 'u16', 'V': '()',
    'Ljava/lang/String;':  'String',     # java.lang.String（不是 std::string::String）
    'Ljava/lang/Object;':  'Object',
    'Ljava/lang/Integer;': 'i32',
    'Ljava/lang/Long;':    'i64',
    'Ljava/lang/Double;':  'f64',
    'Ljava/lang/Boolean;': 'bool',
    '[I': 'Rc<RefCell<Vec<i32>>>', '[J': 'Rc<RefCell<Vec<i64>>>',
    '[F': 'Rc<RefCell<Vec<f32>>>', '[D': 'Rc<RefCell<Vec<f64>>>',
    '[B': 'Rc<RefCell<Vec<i8>>>',  '[S': 'Rc<RefCell<Vec<i16>>>',
    '[C': 'Rc<RefCell<Vec<u16>>>', '[Z': 'Rc<RefCell<Vec<bool>>>',
    '[Ljava/lang/String;': 'Rc<RefCell<Vec<String>>>',
    '[Ljava/lang/Object;': 'Rc<RefCell<Vec<Object>>>',
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
        return f'Rc<RefCell<Vec<{elem}>>>'
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
                short = s[i+1:end].split('/')[-1].lower()
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
                    short = s[j+1:end].split('/')[-1].lower()
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
