"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
import re
from collections import deque
from .classfile import parse_class
from .emitter import write_cargo_project
from .constants import CLASS_CLASS as _CLASS_CLASS
from .callchain import (_JDK_PREFIXES, _JDK_STUB_ONLY_PREFIXES,
                        _read_manifest, _is_boundary_class,
                        _JAVA_RUNTIME_CLASSES, _desc_class_refs,
                        _discover_jdk_classes_method_level)



def transpile(java_files: list[str], out_dir: str, batch_bin: bool = False):
    for jf in java_files:
        if not os.path.exists(jf):
            sys.exit(f"File not found: {jf}")

    # .class 统一输出到 <源码目录>/classes/，与 .java 隔离
    src_dir   = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    # 1. javac（JAVA_HOME 同源解析，与 jdk_resolver 的语料保持同一 JDK）
    _javac = 'javac'
    _home = os.environ.get('JAVA_HOME', '')
    if _home and os.path.exists(os.path.join(_home, 'bin', 'javac')):
        _javac = os.path.join(_home, 'bin', 'javac')
    print(f"[1/4] {_javac} {' '.join(java_files)}")
    r = subprocess.run([_javac, '-g', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. 解析用户 .class 二进制（含内部类 $ 文件）
    class_infos = []
    _seen_class_files: set[str] = set()
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        print(f"[2/4] 解析 {class_name}.class")
        ci = parse_class(class_file)
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)
        _seen_class_files.add(class_file)

        # 发现同目录下的内部类（OuterClass$Inner.class）
        for fname in sorted(os.listdir(class_dir)):
            if fname.startswith(class_name + '$') and fname.endswith('.class'):
                inner_file = os.path.join(class_dir, fname)
                if inner_file in _seen_class_files:
                    continue
                _seen_class_files.add(inner_file)
                inner_ci = parse_class(inner_file)
                print(f"      内部类: {fname[:-6]} (字段: {[f.name for f in inner_ci.fields]}, 方法: {[m.name for m in inner_ci.methods]})")
                class_infos.append(inner_ci)

    # 3. 方法级调用链 BFS 发现 JDK 类
    print(f"[3/4] 扫描 JDK 类引用...", end=' ', flush=True)
    jdk_class_infos, visited_methods, field_stubs = _discover_jdk_classes_method_level(
        class_infos, runtime_src=os.path.join(out_dir, 'java_runtime', 'src'))
    field_stub_count = sum(1 for ci in jdk_class_infos
                           if any(ci.name == cls for cls in field_stubs))
    bfs_count = len(jdk_class_infos) - field_stub_count
    print(f"完成，{bfs_count} 个调用链类 + {field_stub_count} 个 field stub = {len(jdk_class_infos)} 个")

    # 写 JDK 扫描报告
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0 = os.path.splitext(os.path.basename(java_files[0]))[0]
    report_path = os.path.join(reports_dir, f"jdk-scan-{stem0}.md")
    _write_jdk_scan_report(report_path, jdk_class_infos, field_stubs)
    print(f"      JDK 扫描报告 → {report_path}")

    # 4. 生成 Rust
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_infos, jdk_class_infos, java_files,
                        batch_bin=batch_bin, visited_methods=visited_methods)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")


def _write_jdk_scan_report(path: str, jdk_class_infos: list, field_stubs: set):
    """将 JDK 扫描结果写成 Markdown 报告。"""
    from datetime import date
    field_stub_names = {ci.name for ci in jdk_class_infos if ci.name in field_stubs}
    callchain_infos = [ci for ci in jdk_class_infos if ci.name not in field_stubs]
    with open(path, 'w', encoding='utf-8') as f:
        f.write(f"# JDK 扫描报告\n\n生成时间：{date.today()}\n\n")
        f.write("## 摘要\n\n")
        f.write(f"| 来源 | 类数 |\n|------|-----:|\n")
        f.write(f"| 调用链 BFS | {len(callchain_infos)} |\n")
        f.write(f"| Field-only stub | {len(field_stub_names)} |\n")
        f.write(f"| 合计 | {len(jdk_class_infos)} |\n\n")
        f.write("## 调用链 BFS 发现的类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted(callchain_infos, key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")
        f.write("\n## Field-only Stub 类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted((ci for ci in jdk_class_infos if ci.name in field_stubs),
                         key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")
