"""
转译流水线：javac → javap → parse → codegen → Cargo 项目。
"""

import subprocess
import sys
import os
from .javap import parse_javap
from .emitter import write_cargo_project


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
    r = subprocess.run(['javac', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2-3. javap → parse
    class_infos = []
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        print(f"[2/4] javap {class_name}")
        r = subprocess.run(['javap', '-verbose', '-p', class_file],
                           capture_output=True, text=True)

        print(f"[3/4] 解析 {class_name}")
        ci = parse_javap(r.stdout, class_name)
        ci.methods = [m for m in ci.methods if m.name != '<clinit>']
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)

    # 4. 生成 Rust
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_infos, java_files)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")
