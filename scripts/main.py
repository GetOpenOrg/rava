#!/usr/bin/env python3
"""
Java → Rust 转译器 CLI 入口。
用法：
    python3 scripts/main.py tests/TestP0.java
    python3 scripts/main.py tests/TestP1.java tests/TestP2.java --out output
"""

import argparse
import sys
import os

# 将 scripts/ 加入 path，使 `import codegen` 可以找到包
sys.path.insert(0, os.path.dirname(__file__))

from codegen import transpile


def main():
    ap = argparse.ArgumentParser(description='Java .class → Rust 转译器')
    ap.add_argument('java_files', nargs='+', help='.java 源文件列表')
    ap.add_argument('--out', default='output', help='Rust 项目输出目录（默认 output）')
    args = ap.parse_args()
    transpile(args.java_files, args.out)


if __name__ == '__main__':
    main()
