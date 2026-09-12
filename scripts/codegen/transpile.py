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


def _collect_jdk_refs(class_infos: list) -> set:
    """从用户类指令注释中收集直接引用的 JDK 类 binary name。"""
    refs = set()
    for ci in class_infos:
        for m in ci.methods:
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
    """发现并解析用户类直接引用的 JDK 类，返回 ClassInfo 列表（含 native 方法信息）。"""
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver

    refs = _collect_jdk_refs(class_infos)
    if not refs:
        print("      无 JDK 类引用")
        return []

    print(f"      发现引用：{sorted(refs)}")

    jdk_infos = []
    try:
        resolver = JdkResolver()
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return []

    with resolver:
        for binary_name in sorted(refs):
            data = resolver.resolve(binary_name)
            if data is None:
                print(f"      未找到：{binary_name}")
                continue
            try:
                ci = parse_class_bytes(data, binary_name)
                native_count = sum(1 for m in ci.methods if m.is_native)
                print(f"      {binary_name}: {len(ci.methods)} 方法，{native_count} native")
                jdk_infos.append(ci)
            except Exception as e:
                print(f"      解析失败 {binary_name}: {e}")

    return jdk_infos
