"""
共享常量：Rust 关键字集合与标识符安全化函数。
"""

RUST_KEYWORDS: frozenset[str] = frozenset({
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn',
    'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
    'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
    'self', 'Self', 'static', 'struct', 'super', 'trait', 'true', 'type',
    'union', 'unsafe', 'use', 'where', 'while', 'abstract', 'become',
    'box', 'do', 'final', 'macro', 'override', 'priv', 'try', 'typeof',
    'unsized', 'virtual', 'yield',
})


def safe_ident(name: str) -> str:
    """将 Java 标识符转为合法的 Rust 标识符。

    - 替换 Java 内部类分隔符 $ → _
    - 数字开头加 _ 前缀
    - Rust 关键字加 _ 后缀
    """
    name = name.replace('$', '_')
    if name and name[0].isdigit():
        name = '_' + name
    if name in RUST_KEYWORDS:
        name = name + '_'
    return name
