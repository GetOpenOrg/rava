"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
from .classfile import parse_class
from .emitter import write_cargo_project

# JDK 包前缀（binary name 斜线分隔）
_JDK_PREFIXES = ('java/', 'javax/', 'sun/', 'com/sun/', 'com/oracle/')

# java_runtime 已手写实现的类：这些类不再由 jdk_classes 翻译，避免重复定义和命名冲突
_JAVA_RUNTIME_CLASSES: frozenset[str] = frozenset({
    'java/lang/Object',
    'java/util/ArrayList',
    'java/util/HashMap',
    'java/util/HashSet',
    'java/lang/StringBuilder',  # java_runtime 通过 prelude 导出
    'java/lang/Math',           # java_runtime 有 Math stubs
    # 已迁移到 jdk_classes + native_impls:
    # 'java/lang/System', 'java/io/PrintStream', 'java/lang/String'
})

# BFS 截断规则（见设计文档 §6.3）：进入这些类到此为止，不继续追踪
# 原因：SPI / 反射 / 安全框架 / unsafe / 类加载器 / MethodHandle / NIO 内部都会导致依赖爆炸
_CUTOFF_CLASSES: frozenset[str] = frozenset({
    'java/util/ServiceLoader',
    'java/lang/reflect/Method',
    'java/lang/reflect/Constructor',
    'java/lang/reflect/Field',
    'java/lang/reflect/Array',
    'java/security/AccessController',
    'java/security/AccessControlContext',
    'sun/misc/Unsafe',
    'jdk/internal/misc/Unsafe',
    'java/lang/ClassLoader',
    'java/lang/invoke/MethodHandle',
    'java/lang/invoke/MethodHandles',
    'java/lang/invoke/MethodType',
    'java/lang/invoke/LambdaMetafactory',
    'sun/nio/cs/StreamEncoder',
    'sun/nio/cs/StreamDecoder',
    'java/lang/Thread',
    'java/lang/ThreadLocal',
    'java/lang/ref/Reference',
    'java/lang/ref/SoftReference',
    'java/lang/ref/WeakReference',
    'java/lang/ref/PhantomReference',
    'java/lang/ref/Cleaner',
    'java/util/concurrent/locks/ReentrantLock',
    'java/util/concurrent/locks/AbstractQueuedSynchronizer',
    'jdk/internal/access/SharedSecrets',
    'jdk/internal/vm/annotation/Stable',
})

# BFS 截断包前缀：以这些前缀开头的类全部截断，不展开
# 目标：把传递闭包限制在 ~30–60 个核心类，不拉入 NIO / charset / Stream / regex 等深层链
_CUTOFF_PREFIXES: tuple[str, ...] = (
    'sun/',                 # 所有 sun.* 内部包（nio/ch, nio/cs, nio/fs, security 等）
    'jdk/',                 # 所有 jdk.internal.* 包
    'java/nio/',            # NIO（channels、charset、file 等）—— 依赖爆炸起点
    'java/math/',           # BigDecimal/BigInteger（依赖 MutableBigInteger 等内部类）
    'java/util/concurrent/',# 并发工具框架
    'java/util/stream/',    # Stream API（invokedynamic 密集）
    'java/util/regex/',     # 正则引擎（复杂状态机）
    'java/util/function/',  # Functional interfaces（仅接口，实际 lambda 不翻译）
    'java/text/',           # 文本格式化框架（locale/format 依赖复杂）
    'java/security/',       # 安全框架
    'javax/',               # 扩展 API
    'com/sun/',             # Oracle 内部
    'com/oracle/',          # Oracle 内部
)

# BFS 额外精确截断：这些类即使不在前缀范围也需截断（会通过其他包前缀绕过）
_CUTOFF_EXTRA_CLASSES: frozenset[str] = frozenset({
    'java/lang/Class',                  # 反射入口，171 方法，35 native
    'java/lang/SecurityManager',        # 安全管理（遗留 API）
    'java/lang/Runtime',                # 进程/内存（native 密集）
    'java/lang/ProcessEnvironment',     # 环境变量
    'java/lang/VersionProps',           # JDK 版本信息
    'java/lang/Terminator',             # 关机钩子
    'java/lang/System$LoggerFinder',    # 日志框架
    'java/lang/StringLatin1',           # String 内部编码实现
    'java/lang/StringUTF16',            # String 内部编码实现
    'java/lang/StringCoding',           # String 编码辅助
    'java/lang/StringConcatHelper',     # invokedynamic 字符串拼接辅助
    'java/io/ObjectInputStream',        # 序列化（极其复杂）
    'java/io/ObjectOutputStream',       # 序列化
    'java/io/BufferedWriter',           # I/O 中间层（依赖 OutputStreamWriter → nio）
    'java/io/OutputStreamWriter',       # 编码桥（依赖 java/nio/charset）
    'java/util/Formatter',              # printf 格式化（依赖 locale/regex）
    'java/util/Locale',                 # 国际化（极其复杂）
    'java/util/Properties',             # 属性文件
    'java/util/ResourceBundle',         # 资源包
})

# BFS 上限：防止意外拉入过多类（设计文档估算 Hello World 场景 ~30 类）
_MAX_JDK_CLASSES = 150


def transpile(java_files: list[str], out_dir: str):
    for jf in java_files:
        if not os.path.exists(jf):
            sys.exit(f"File not found: {jf}")

    # .class 统一输出到 <源码目录>/classes/，与 .java 隔离
    src_dir   = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    # 1. javac
    print(f"[1/4] javac {' '.join(java_files)}")
    r = subprocess.run(['javac', '-g', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. 解析用户 .class 二进制
    class_infos = []
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        print(f"[2/4] 解析 {class_name}.class")
        ci = parse_class(class_file)
        ci.methods = [m for m in ci.methods if m.name != '<clinit>']
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)

    # 3. 扫描 JDK 类引用，解析元数据（用于 build.rs 追踪 native 状态）
    print(f"[3/4] 扫描 JDK 类引用...")
    jdk_class_infos = _discover_jdk_classes(class_infos)

    # 4. 生成 Rust
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_infos, jdk_class_infos, java_files)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")


def _is_cutoff(binary_name: str) -> bool:
    """判断某个 JDK 类是否命中截断规则（精确名或包前缀匹配）。
    java_runtime 已手写的类也视为截断：不再重复翻译。"""
    if binary_name in _CUTOFF_CLASSES:
        return True
    if binary_name in _CUTOFF_EXTRA_CLASSES:
        return True
    if binary_name in _JAVA_RUNTIME_CLASSES:
        return True
    return any(binary_name.startswith(p) for p in _CUTOFF_PREFIXES)


def _collect_jdk_refs(class_infos: list, skip_clinit: bool = False) -> set:
    """从 ClassInfo 列表的指令注释中收集引用的 JDK 类 binary name。

    skip_clinit=True 时跳过 <clinit> 方法（用于 JDK 类的传递扫描，
    切断 <clinit> 链污染，见设计文档 §6.1）。
    """
    refs = set()
    for ci in class_infos:
        for m in ci.methods:
            if skip_clinit and m.name == '<clinit>':
                continue
            for instr in (m.instrs or []):
                comment = instr.comment
                if not comment:
                    continue
                if comment.startswith(('Method ', 'InterfaceMethod ', 'Field ')):
                    # e.g. "Method java/lang/System.currentTimeMillis:()J"
                    ref_str = comment.split(' ', 1)[1]
                    dot_pos = ref_str.find('.')
                    if dot_pos > 0:
                        class_part = ref_str[:dot_pos]
                        if class_part.startswith(_JDK_PREFIXES) and '[' not in class_part:
                            refs.add(class_part)
                elif comment.startswith(_JDK_PREFIXES) and '[' not in comment:
                    # new, anewarray, checkcast: comment = class binary name（可能带空格后缀）
                    refs.add(comment.split()[0])
    return refs


def _discover_jdk_classes(class_infos: list) -> list:
    """BFS 传递闭包：发现并解析所有可达 JDK 类（含传递依赖）。

    算法（见设计文档 §6.4）：
    1. 从用户类指令中收集直接 JDK 引用作为 BFS 初始队列
    2. 对队列中每个类：解析字节码，扫描其方法找到新 JDK 引用（跳过 <clinit>）
    3. 将新引用加入队列，重复直到不动点
    4. 截断规则：cutoff 类不展开（SPI/reflect/Unsafe 等），防止依赖爆炸
    5. 上限保护：最多 _MAX_JDK_CLASSES 个类
    """
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver

    # 用户类的直接引用（用户代码允许追踪 <clinit>）
    initial_refs = _collect_jdk_refs(class_infos, skip_clinit=False)
    if not initial_refs:
        print("      无 JDK 类引用")
        return []

    # BFS 状态
    visited: set[str] = set()
    queue: list[str] = sorted(ref for ref in initial_refs if not _is_cutoff(ref))
    jdk_infos: list = []

    try:
        resolver = JdkResolver()
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return []

    with resolver:
        while queue and len(jdk_infos) < _MAX_JDK_CLASSES:
            binary_name = queue.pop(0)
            if binary_name in visited:
                continue
            visited.add(binary_name)

            data = resolver.resolve(binary_name)
            if data is None:
                print(f"      未找到：{binary_name}")
                continue
            try:
                ci = parse_class_bytes(data, binary_name)
                native_count = sum(1 for m in ci.methods if m.is_native)
                print(f"      {binary_name}: {len(ci.methods)} 方法，{native_count} native")
                jdk_infos.append(ci)

                # 传递闭包：扫描这个 JDK 类的方法找新引用，跳过 <clinit>
                new_refs = _collect_jdk_refs([ci], skip_clinit=True)
                for ref in sorted(new_refs):
                    if ref not in visited and not _is_cutoff(ref):
                        queue.append(ref)

            except Exception as e:
                print(f"      解析失败 {binary_name}: {e}")

    if len(jdk_infos) >= _MAX_JDK_CLASSES:
        print(f"      [警告] 已达 JDK 类上限 {_MAX_JDK_CLASSES}，停止 BFS 展开")

    print(f"      共解析 {len(jdk_infos)} 个 JDK 类（传递闭包，{len(visited)} 个已访问）")
    return jdk_infos
