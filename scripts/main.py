#!/usr/bin/env python3
"""
Java → Rust 转译器 CLI 入口。
用法：
    python3 scripts/main.py                            # 默认运行 tests/HelloWorld.java 并执行
    python3 scripts/main.py tests/TestP0.java
    python3 scripts/main.py tests/TestP1.java tests/TestP2.java --out output
    python3 scripts/main.py tests/HelloWorld.java --no-run  # 只生成，不执行
"""

import argparse
import subprocess
import sys
import os

# 将 scripts/ 加入 path，使 `import codegen` 可以找到包
sys.path.insert(0, os.path.dirname(__file__))

from codegen import transpile

_DEFAULT_JAVA = os.path.join(os.path.dirname(__file__), '..', 'tests', 'HelloWorld.java')


def main():
    ap = argparse.ArgumentParser(description='Java .class → Rust 转译器')
    ap.add_argument('java_files', nargs='*', help='.java 源文件列表（默认 tests/HelloWorld.java）')
    ap.add_argument('--out', default='output', help='Rust 项目输出目录（默认 output）')
    ap.add_argument('--no-run', action='store_true', help='只生成 Rust 代码，不编译运行')
    args = ap.parse_args()

    java_files = args.java_files or [_DEFAULT_JAVA]
    transpile(java_files, args.out)

    if not args.no_run:
        print(f"\n[run] cargo run --manifest-path {args.out}/Cargo.toml")
        r = subprocess.run(
            ['cargo', 'run', '--manifest-path', os.path.join(args.out, 'Cargo.toml')],
            cwd=os.path.dirname(__file__),
        )
        sys.exit(r.returncode)


if __name__ == '__main__':
    main()
