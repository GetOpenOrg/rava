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


# ── 仓库布局（per-test scratch workspace，见 docs/plans/2026-09-16-per-test-scratch-workspace.md）──
# runtime/：手写代码唯一真源（git 管理）
# build/<test>/：每测试一次性 scratch（gitignore，生成代码 + 手写 overlay 副本）

import os as _os

REPO_ROOT = _os.path.dirname(_os.path.dirname(_os.path.abspath(__file__)))
RUNTIME_DIR = _os.path.join(REPO_ROOT, 'runtime')
RUNTIME_JAVA_RUNTIME = _os.path.join(RUNTIME_DIR, 'java_runtime')
RUNTIME_MACROS_CRATE = _os.path.join(RUNTIME_DIR, 'java_rta_macros')


# java_runtime 手写实现的短类名：这些类的方法名不经过 mangle（hand-written API 已定好名称）
# System/PrintStream/String/Math/ArrayList/HashMap/HashSet/StringBuilder 由 jdk_classes 字节码翻译提供
JAVA_RUNTIME_SHORT_NAMES: frozenset[str] = frozenset({
    'Object',
})

# Rust 原生类型集合（不对应 Java 类，供 codegen 内部判断用）
PRIMITIVE_RUST_TYPES: frozenset[str] = frozenset({
    'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64',
    'f32', 'f64', 'bool', 'usize', '()',
})

# 常用 JVM binary class names
OBJECT_CLASS = 'java/lang/Object'
STRING_CLASS = 'java/lang/String'
CLASS_CLASS = 'java/lang/Class'
THROWABLE_CLASS = 'java/lang/Throwable'
# LambdaMetafactory 默认产物恒实现的标记接口（JVM 语义事实；与 OBJECT_CLASS 同级的
# 架构常量，A-5 函数式接口合成对象的 is_instance_of 静态名单使用）
SERIALIZABLE_CLASS = 'java/io/Serializable'


def scratch_pkg_version(out_dir: str) -> str:
    """为 scratch 工作区内的包生成唯一版本号。

    背景：多个 scratch 共享 CARGO_TARGET_DIR 时，cargo 以「包名+版本+依赖」
    计算元数据哈希命名 artifact。同名同版本的路径包（java_runtime / user）
    哈希相同，会发生跨工作区的陈旧 artifact 复用（编译结果张冠李戴）。

    方案：版本号带 out_dir 的 CRC —— 同一 scratch 复跑版本不变（增量缓存
    有效），不同 scratch 互不碰撞。注册表依赖（syn/quote）与绝对路径的宏
    crate 不受影响，仍全局共享缓存。
    """
    import zlib as _zlib
    _crc = _zlib.crc32(_os.path.abspath(out_dir).encode()) & 0xffffffff
    return f"0.0.{_crc}"
