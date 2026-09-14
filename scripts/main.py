#!/usr/bin/env python3
"""
Java → Rust 转译器 CLI 入口。
用法：
    python3 scripts/main.py                                        # 默认运行 tests/e2e/01_basics/HelloWorld.java
    python3 scripts/main.py tests/e2e/04_collections/TestArrayList.java
    python3 scripts/main.py tests/e2e/01_basics/TestArithmetic.java --no-run
"""

import argparse
import subprocess
import sys
import os

# 将项目根目录加入 path，使 `import codegen` 可以找到根目录下的 codegen/ 包
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codegen import transpile
from codegen.emitter import to_snake

_DEFAULT_JAVA = os.path.join(os.path.dirname(__file__), '..', 'tests', 'e2e', '01_basics', 'HelloWorld.java')


def main():
    ap = argparse.ArgumentParser(description='Java .class → Rust 转译器')
    ap.add_argument('java_files', nargs='*', help='.java 源文件列表（默认 tests/e2e/01_basics/HelloWorld.java）')
    ap.add_argument('--out', default='output', help='Rust 项目输出目录（默认 output）')
    ap.add_argument('--no-run', action='store_true', help='只生成 Rust 代码，不编译运行')
    ap.add_argument('--batch', action='store_true', help='批量模式：写 src/bin/<class>.rs（供并行测试用）')
    args = ap.parse_args()

    java_files = args.java_files or [_DEFAULT_JAVA]
    transpile(java_files, args.out, batch_bin=args.batch)

    if not args.no_run:
        project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        bin_name = to_snake(os.path.splitext(os.path.basename(java_files[0]))[0])
        print(f"\n[run] cargo run --bin {bin_name}")
        r = subprocess.run(['cargo', 'run', '--bin', bin_name],
                           cwd=os.path.join(project_root, args.out))
        sys.exit(r.returncode)


if __name__ == '__main__':
    main()
