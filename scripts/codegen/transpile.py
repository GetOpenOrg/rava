"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
from .classfile import parse_class
from .emitter import write_cargo_project
from .type_map import load_ergonomic_renames

# JDK 包前缀（binary name 斜线分隔）
_JDK_PREFIXES = ('java/', 'javax/', 'sun/', 'com/sun/', 'com/oracle/')

# java_runtime 已手写实现的类：这些类不再由 jdk_classes 翻译，避免重复定义和命名冲突
_JAVA_RUNTIME_CLASSES: frozenset[str] = frozenset({
    'java/lang/Object',
})


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

    # 3. 方法级调用链 BFS 发现 JDK 类
    print(f"[3/4] 扫描 JDK 类引用...")
    jdk_class_infos = _discover_jdk_classes_method_level(class_infos)

    # 4. 生成 Rust
    print(f"[4/4] 生成 Rust → {out_dir}/")
    # 预加载 ergonomic @jvm_rename 指令（T39：为 ergonomic 层腾出干净方法名）
    load_ergonomic_renames(os.path.abspath(out_dir))
    write_cargo_project(out_dir, class_infos, jdk_class_infos, java_files)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")


def _collect_method_refs(instrs) -> list[tuple[str, str, str]]:
    """从指令注释中提取 (class, method, descriptor) 三元组（仅 JDK 类）。"""
    refs = []
    for instr in (instrs or []):
        c = instr.comment
        if not c:
            continue
        if c.startswith(('Method ', 'InterfaceMethod ')):
            # "Method java/io/PrintStream.println:(Ljava/lang/String;)V"
            rest = c.split(' ', 1)[1]
            dot = rest.find('.')
            colon = rest.find(':', dot)
            if dot > 0 and colon > dot:
                cls = rest[:dot]
                meth = rest[dot+1:colon]
                desc = rest[colon+1:]
                if cls.startswith(_JDK_PREFIXES) and '[' not in cls:
                    refs.append((cls, meth, desc))
        elif c.startswith(_JDK_PREFIXES) and '[' not in c:
            # new / checkcast / anewarray: comment = class binary name
            cls = c.split()[0]
            refs.append((cls, '<init>', '()V'))
    return refs


def _discover_jdk_classes_method_level(class_infos: list) -> list:
    """方法级调用链 BFS：只追踪实际被调用的方法，不展开未调用方法的依赖类。"""
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver

    visited_methods: set[tuple[str, str, str]] = set()
    queue: list[tuple[str, str, str]] = []

    def enqueue_refs(instrs):
        for key in _collect_method_refs(instrs):
            cls = key[0]
            if cls in _JAVA_RUNTIME_CLASSES:
                continue
            if key not in visited_methods:
                visited_methods.add(key)
                queue.append(key)

    # 初始种子：用户类所有方法的引用
    for ci in class_infos:
        for m in ci.methods:
            enqueue_refs(m.instrs or [])

    if not queue:
        print("      无 JDK 类引用")
        return []

    class_cache: dict[str, object] = {}
    jdk_infos: dict[str, object] = {}

    try:
        resolver = JdkResolver()
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return []

    with resolver:
        while queue:
            cls, meth, desc = queue.pop(0)

            # 解析类（首次遇到时）
            if cls not in class_cache:
                data = resolver.resolve(cls)
                if data is None:
                    class_cache[cls] = None
                    continue
                try:
                    ci = parse_class_bytes(data, cls)
                    class_cache[cls] = ci
                    if cls not in jdk_infos:
                        native_count = sum(1 for m in ci.methods if m.is_native)
                        print(f"      {cls}: {len(ci.methods)} 方法, {native_count} native")
                        jdk_infos[cls] = ci
                except Exception as e:
                    print(f"      解析失败 {cls}: {e}")
                    class_cache[cls] = None
                    continue

            ci = class_cache.get(cls)
            if ci is None:
                continue

            if cls not in jdk_infos:
                jdk_infos[cls] = ci

            # 追踪该方法的指令引用
            for m in ci.methods:
                if m.name == meth:
                    enqueue_refs(m.instrs or [])

    print(f"      共解析 {len(jdk_infos)} 个 JDK 类（方法级调用链 BFS）")
    return list(jdk_infos.values())
