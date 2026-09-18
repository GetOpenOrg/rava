#!/usr/bin/env python3
"""
Java → Rust 转译器 CLI 入口。

per-test scratch workspace（见 docs/plans/2026-09-16-per-test-scratch-workspace.md）：
    手写代码唯一真源在 runtime/，每次转译前 overlay 复制进 scratch；
    生成代码与编译产物全部落在 build/<测试名>/，不进 git。

用法：
    python3 scripts/main.py                                        # 默认运行 tests/e2e/01_basics/HelloWorld.java
    python3 scripts/main.py tests/e2e/04_collections/TestArrayList.java
    python3 scripts/main.py tests/e2e/01_basics/TestArithmetic.java --no-run
    python3 scripts/main.py tests/e2e/01_basics/TestArithmetic.java --clean   # 清空 scratch 后重建
"""

import argparse
import subprocess
import sys
import os
import shutil
import time

# 将项目根目录加入 path，使 `import codegen` 可以找到根目录下的 codegen/ 包
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codegen import transpile
from codegen.constants import (RUNTIME_JAVA_RUNTIME, RUNTIME_MACROS_CRATE,
                               scratch_pkg_version)
from codegen.emitter import to_snake

_REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_DEFAULT_JAVA = os.path.join(_REPO_ROOT, 'tests', 'e2e', '01_basics', 'HelloWorld.java')
_BUILD_ROOT = os.path.join(_REPO_ROOT, 'build')
_SHARED_TARGET = os.path.join(_BUILD_ROOT, 'target')


def fmt_dur(sec: float) -> str:
    """格式化耗时：<60s 用秒（两位小数），≥60s 用 m 分 s 秒。"""
    if sec < 60:
        return f"{sec:.2f}s"
    m, s = divmod(sec, 60)
    return f"{int(m)}m{s:04.1f}s"


def _copy_if_changed_file(src: str, dst: str) -> None:
    """复制单个文件，内容相同则跳过（保留 mtime）。"""
    os.makedirs(os.path.dirname(dst) or '.', exist_ok=True)
    if os.path.exists(dst):
        with open(src, 'rb') as f1, open(dst, 'rb') as f2:
            if f1.read() == f2.read():
                return
    shutil.copy2(src, dst)


def _write_if_changed(path: str, content: str) -> None:
    """写文本文件，内容相同则跳过（保留 mtime）。"""
    if os.path.exists(path):
        try:
            with open(path, encoding='utf-8') as f:
                if f.read() == content:
                    return
        except Exception:
            pass
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)


def _copy_if_changed(src: str, dst: str) -> None:
    """递归复制目录，内容未变的文件跳过（保留 mtime），让 cargo 跳过重编。"""
    for root, dirs, files in os.walk(src):
        rel = os.path.relpath(root, src)
        dst_root = os.path.join(dst, rel) if rel != '.' else dst
        os.makedirs(dst_root, exist_ok=True)
        for fname in files:
            src_file = os.path.join(root, fname)
            dst_file = os.path.join(dst_root, fname)
            if os.path.exists(dst_file):
                with open(src_file, 'rb') as f1, open(dst_file, 'rb') as f2:
                    if f1.read() == f2.read():
                        continue
            shutil.copy2(src_file, dst_file)


def prepare_scratch(out_dir: str, clean: bool = False) -> None:
    """将 runtime/ 手写代码 overlay 进 scratch 工作区。

    必须在 codegen 之前调用：codegen 的 _scan_impl_files 扫描 scratch 里的
    *_impl.rs 生成 new_format_map（决定哪些方法跳过存根生成）。

    overlay 内容：
      - runtime/java_runtime/src/**  → scratch/java_runtime/src/**（手写 .rs）
      - runtime/java_runtime/build.rs → scratch/java_runtime/build.rs
      - runtime/java_runtime/Cargo.toml → 重写宏依赖为 runtime/ 绝对路径
        （宏 crate 不复制：绝对路径稳定 → 共享 CARGO_TARGET_DIR 下指纹不变，
         syn/quote/宏的编译缓存可跨测试复用）
      - java/ jdk/ sun/ 顶层目录保证存在且含占位 mod.rs
        （lib.rs 手写了 `pub mod java; jdk; sun;`，目录缺失会 E0583；
         codegen 在有生成类时会覆写占位文件）
    """
    if clean and os.path.isdir(out_dir):
        shutil.rmtree(out_dir)

    rt_src = os.path.join(RUNTIME_JAVA_RUNTIME, 'src')
    dst_src = os.path.join(out_dir, 'java_runtime', 'src')
    _copy_if_changed(rt_src, dst_src)

    _copy_if_changed_file(os.path.join(RUNTIME_JAVA_RUNTIME, 'build.rs'),
                          os.path.join(out_dir, 'java_runtime', 'build.rs'))

    cargo_toml = open(os.path.join(RUNTIME_JAVA_RUNTIME, 'Cargo.toml'),
                      encoding='utf-8').read()
    cargo_toml = cargo_toml.replace(
        'path = "../java_rta_macros"',
        f'path = "{RUNTIME_MACROS_CRATE}"')
    # 包版本唯一化：共享 CARGO_TARGET_DIR 下避免与其他 scratch 的同名包
    # 元数据哈希碰撞（陈旧 artifact 跨工作区复用）
    cargo_toml = cargo_toml.replace(
        'version = "0.1.0"',
        f'version = "{scratch_pkg_version(out_dir)}"')
    os.makedirs(os.path.join(out_dir, 'java_runtime'), exist_ok=True)
    _write_if_changed(os.path.join(out_dir, 'java_runtime', 'Cargo.toml'), cargo_toml)

    # lib.rs 声明的顶层包目录兜底（占位 mod.rs，codegen 有生成类时覆写）
    for pkg in ('java', 'jdk', 'sun'):
        pkg_dir = os.path.join(dst_src, pkg)
        os.makedirs(pkg_dir, exist_ok=True)
        mod_path = os.path.join(pkg_dir, 'mod.rs')
        if not os.path.exists(mod_path):
            with open(mod_path, 'w', encoding='utf-8') as f:
                f.write('// placeholder（overlay 兜底）：本包无生成类时 lib.rs 的\n'
                        '// `pub mod` 声明仍需可解析；有生成类时被 codegen 覆写。\n')


def main():
    ap = argparse.ArgumentParser(description='Java .class → Rust 转译器')
    ap.add_argument('java_files', nargs='*', help='.java 源文件列表（默认 tests/e2e/01_basics/HelloWorld.java）')
    ap.add_argument('--out', default=None,
                    help='scratch 工作区目录（默认 build/<主类 snake 名>）')
    ap.add_argument('--clean', action='store_true', help='转译前清空 scratch 工作区')
    ap.add_argument('--no-run', action='store_true', help='只生成 Rust 代码，不编译运行')
    ap.add_argument('--batch', action='store_true', help='批量模式：写 src/bin/<class>.rs（供并行测试用）')
    args = ap.parse_args()

    java_files = args.java_files or [_DEFAULT_JAVA]
    stem = os.path.splitext(os.path.basename(java_files[0]))[0]
    out_dir = args.out or os.path.join(_BUILD_ROOT, to_snake(stem))

    t_total = time.perf_counter()

    # 1. overlay 手写代码（必须在 codegen 之前）
    t0 = time.perf_counter()
    prepare_scratch(out_dir, clean=args.clean)
    t_overlay = time.perf_counter() - t0
    print(f"[time] overlay     {fmt_dur(t_overlay)}")

    # 2. codegen
    t0 = time.perf_counter()
    transpile(java_files, out_dir, batch_bin=args.batch)
    t_codegen = time.perf_counter() - t0
    print(f"[time] transpile   {fmt_dur(t_codegen)}")

    if not args.no_run:
        bin_name = to_snake(stem)
        print(f"\n[run] cargo run --bin {bin_name}")
        env = dict(os.environ, CARGO_TARGET_DIR=_SHARED_TARGET)
        t0 = time.perf_counter()
        r = subprocess.run(['cargo', 'run', '--bin', bin_name],
                           cwd=out_dir, env=env)
        t_run = time.perf_counter() - t0
        print(f"[time] cargo run   {fmt_dur(t_run)}")
        print(f"[time] total       {fmt_dur(time.perf_counter() - t_total)}"
              f"  (overlay {fmt_dur(t_overlay)} + transpile {fmt_dur(t_codegen)}"
              f" + run {fmt_dur(t_run)})")
        sys.exit(r.returncode)

    print(f"[time] total       {fmt_dur(time.perf_counter() - t_total)}"
          f"  (overlay {fmt_dur(t_overlay)} + transpile {fmt_dur(t_codegen)})")


if __name__ == '__main__':
    main()
