"""
JVM 类型描述符 → Rust 类型的映射与解析工具。
"""

import re

# ── JVM descriptor → Rust 类型 ──────────────────────────────────
JVM_RUST: dict[str, str] = {
    'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64', 'Z': 'bool',
    'B': 'i8',  'S': 'i16', 'C': 'u16', 'V': '()',
    'Ljava/lang/String;':  'String',
    'Ljava/lang/Object;':  'JvmObject',
    'Ljava/lang/Integer;': 'i32',
    'Ljava/lang/Long;':    'i64',
    'Ljava/lang/Double;':  'f64',
    'Ljava/lang/Boolean;': 'bool',
    '[I': 'Vec<i32>', '[J': 'Vec<i64>',
    '[F': 'Vec<f32>', '[D': 'Vec<f64>',
    '[Ljava/lang/String;': 'Vec<String>',
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

# JDK 集合类 → (Rust 类型, 初始化表达式)
JDK_COLL_TYPES: dict[str, tuple[str, str]] = {
    'ArrayList':           ('Vec<i32>',           'Vec::new()'),
    'java/util/ArrayList': ('Vec<i32>',           'Vec::new()'),
    'HashMap':             ('HashMap<String,i32>', 'HashMap::new()'),
    'java/util/HashMap':   ('HashMap<String,i32>', 'HashMap::new()'),
    'HashSet':             ('HashSet<i32>',        'HashSet::new()'),
    'java/util/HashSet':   ('HashSet<i32>',        'HashSet::new()'),
}

# 已知 JDK 类名（短名，无包路径）
JDK_CLASSES: set[str] = {
    'Object', 'String', 'Integer', 'Long', 'Double', 'Float',
    'Boolean', 'Byte', 'Short', 'Character',
    'StringBuilder', 'StringBuffer',
    'Math', 'System', 'Arrays', 'Collections',
    'PrintStream', 'InputStream', 'OutputStream', 'BufferedReader',
    'ArrayList', 'LinkedList',
    'HashMap', 'LinkedHashMap', 'TreeMap',
    'HashSet', 'TreeSet',
    'List', 'Map', 'Set', 'Collection', 'Iterator', 'Optional',
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

def jvm_to_rust(t: str) -> str:
    return JVM_RUST.get(t, 'JvmObject')


def sig_type(rt: str) -> str:
    """函数签名中 Vec<T> → &[T]（数组按引用传递，避免 ownership 转移）"""
    if rt.startswith('Vec<'):
        return f"&[{rt[4:-1]}]"
    return rt


def rust_default(rt: str) -> str:
    return {
        'i32': '0', 'i64': '0', 'f32': '0.0', 'f64': '0.0',
        'bool': 'false', 'String': 'String::new()',
    }.get(rt, 'Default::default()')


def is_jdk(cls: str) -> bool:
    return cls in JDK_CLASSES or '/' in cls


def short_cls(cls: str) -> str:
    return cls.split('/')[-1].split('.')[-1] if cls else ''


def parse_descriptor_params(desc: str) -> list[str]:
    m = re.match(r'\(([^)]*)\)', desc)
    return _parse_type_list(m.group(1) if m else '')


def parse_descriptor_return(desc: str) -> str:
    m = re.match(r'\([^)]*\)(.*)', desc)
    return m.group(1) if m else 'V'


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
