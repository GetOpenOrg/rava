#!/usr/bin/env python3
"""
端到端测试框架（T51）。

用法：
    python3 scripts/run_tests.py                         # 全量运行（顺序）
    python3 scripts/run_tests.py -j 4                    # 并行运行，最多 4 个并发
    python3 scripts/run_tests.py -j 0                    # 并行运行，并发数 = CPU 核数
    python3 scripts/run_tests.py --filter 01_basics      # 只跑指定目录
    python3 scripts/run_tests.py --filter TestArrayList  # 只跑指定类名
    python3 scripts/run_tests.py --update-expected       # 重新生成 expected/*.txt
    python3 scripts/run_tests.py --no-run                # 只生成 Rust，不执行对比

顺序模式流程（-j 1，默认）：
  对每个 tests/e2e/**/*.java：
  1. 转译 → 生成 output/user/src/*.rs
  2. cargo run --bin <class> 捕获 stdout
  3. 与 tests/expected/<Class>.txt diff

并行模式流程（-j N，N>1）：
  1. 重置批量工作区（清空 user/src/bin/ 和 jdk_classes/src/）
  2. 顺序转译所有测试（--batch，共享工作区，积累 src/bin/<class>.rs）
  3. cargo build --bins（一次性编译所有 binary）
  4. ThreadPoolExecutor(max_workers=N) 并行运行 target/debug/<bin>
"""

import argparse
import difflib
import os
import re
import shutil
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

ROOT   = Path(__file__).parent.parent
TESTS  = ROOT / "tests"
E2E    = TESTS / "e2e"
EXPECT = TESTS / "expected"
OUT    = ROOT / "output"


def _run(cmd: list[str], cwd: Path, capture: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=capture, text=True)


def _discover(filter_str: str | None) -> list[Path]:
    files = sorted(E2E.rglob("*.java"))
    if filter_str:
        files = [f for f in files if filter_str in str(f)]
    return files


def _class_name(java_file: Path) -> str:
    return java_file.stem


def _to_bin_name(class_name: str) -> str:
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', class_name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    return s.lower()


def _transpile(java_file: Path, batch: bool = False) -> tuple[bool, str]:
    """运行转译器生成 Rust 代码。batch=True 时写 src/bin/<class>.rs。"""
    args = [sys.executable, str(ROOT / "scripts" / "main.py"), str(java_file), "--no-run"]
    if batch:
        args.append("--batch")
    r = _run(args, cwd=ROOT)
    return r.returncode == 0, (r.stdout + r.stderr)


def _cargo_run(class_name: str) -> tuple[bool, str]:
    """顺序模式：cargo run --bin <class>。"""
    bin_name = _to_bin_name(class_name)
    r = _run(["cargo", "run", "--bin", bin_name], cwd=OUT)
    return r.returncode == 0, r.stdout


def _run_binary(class_name: str) -> tuple[bool, str]:
    """并行模式：直接执行已编译的 binary（不经过 cargo，避免文件锁竞争）。"""
    bin_name = _to_bin_name(class_name)
    bin_path = OUT / "target" / "debug" / bin_name
    r = subprocess.run([str(bin_path)], capture_output=True, text=True)
    return r.returncode == 0, r.stdout


def _expected_path(class_name: str) -> Path:
    return EXPECT / f"{class_name}.txt"


def _read_expected(class_name: str) -> str | None:
    p = _expected_path(class_name)
    return p.read_text() if p.exists() else None


def _diff(expected: str, actual: str, class_name: str) -> list[str]:
    return list(difflib.unified_diff(
        expected.splitlines(keepends=True),
        actual.splitlines(keepends=True),
        fromfile=f"expected/{class_name}.txt",
        tofile=f"cargo run --bin {_to_bin_name(class_name)}",
    ))


def _update_expected(java_file: Path) -> bool:
    class_name = _class_name(java_file)
    classes_dir = TESTS / "classes"
    classes_dir.mkdir(exist_ok=True)
    r = _run(["javac", "-g", "-d", str(classes_dir), str(java_file)], cwd=ROOT)
    if r.returncode != 0:
        print(f"  javac FAIL: {r.stderr.strip()}")
        return False
    r = _run(["java", "-cp", str(classes_dir), class_name], cwd=ROOT)
    if r.returncode != 0:
        print(f"  java FAIL: {r.stderr.strip()}")
        return False
    _expected_path(class_name).write_text(r.stdout)
    print(f"  updated expected/{class_name}.txt ({r.stdout.count(chr(10))} lines)")
    return True


def _reset_batch_workspace() -> None:
    """批量模式开始前：清空 user/src/bin/、user/src/*.rs、jdk_classes/src/，重置 Cargo.toml。"""
    user_dir = OUT / "user"
    user_src = user_dir / "src"

    # 清空 user/src/bin/
    bin_dir = user_src / "bin"
    if bin_dir.exists():
        shutil.rmtree(bin_dir)
    bin_dir.mkdir(parents=True, exist_ok=True)

    # 删除 user/src/ 顶层所有 .rs 文件（main.rs, 用户类文件等）
    for rs_file in user_src.glob("*.rs"):
        rs_file.unlink(missing_ok=True)

    # 清空 jdk_classes/src/ 下所有 .rs 文件（lib.rs 由转译器重建）
    jdk_src = OUT / "jdk_classes" / "src"
    if jdk_src.exists():
        for rs_file in jdk_src.rglob("*.rs"):
            rs_file.unlink()

    # 重置 user/Cargo.toml（无 [[bin]] 条目，由批量转译追加）
    cargo_content = "\n".join([
        "[package]",
        'name = "user"',
        'version = "0.1.0"',
        'edition = "2021"',
        "",
        "[dependencies]",
        'java_runtime    = { path = "../java_runtime" }',
        'java_rta_macros = { path = "../java_rta_macros" }',
        'jdk_classes     = { path = "../jdk_classes" }',
        "",
    ])
    (user_dir / "Cargo.toml").write_text(cargo_content)


# ── 顺序模式 ─────────────────────────────────────────────────────────

def _run_sequential(filter_str: str | None, no_run: bool) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    passed = failed = skipped = 0

    for java_file in files:
        class_name = _class_name(java_file)
        rel = java_file.relative_to(ROOT)
        expected = _read_expected(class_name)

        if expected is None:
            print(f"[ SKIP ] {rel}  (no expected/{class_name}.txt)")
            skipped += 1
            continue

        ok, log = _transpile(java_file)
        if not ok:
            print(f"[ FAIL ] {rel}  — transpile error")
            print(log[-500:])
            failed += 1
            continue

        if no_run:
            print(f"[NORUN ] {rel}  — transpile OK, skipping run")
            skipped += 1
            continue

        ok, actual = _cargo_run(class_name)
        if not ok:
            print(f"[ FAIL ] {rel}  — cargo run error")
            failed += 1
            continue

        diff = _diff(expected, actual, class_name)
        if diff:
            print(f"[ FAIL ] {rel}")
            print("".join(diff[:40]))
            if len(diff) > 40:
                print(f"  … ({len(diff) - 40} more lines)")
            failed += 1
        else:
            print(f"[ PASS ] {rel}")
            passed += 1

    total = passed + failed + skipped
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    return 0 if failed == 0 else 1


# ── 并行模式 ─────────────────────────────────────────────────────────

def _run_parallel(filter_str: str | None, jobs: int) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    # 过滤无 expected 文件的测试
    pending: list[Path] = []
    skipped = 0
    for f in files:
        if _read_expected(_class_name(f)) is None:
            print(f"[ SKIP ] {f.relative_to(ROOT)}  (no expected/{_class_name(f)}.txt)")
            skipped += 1
        else:
            pending.append(f)

    if not pending:
        print(f"\n{'='*50}")
        print(f"Results: 0 passed, 0 failed, {skipped} skipped / {skipped} total")
        return 0

    # 1. 重置批量工作区
    print(f"\n[batch] 重置工作区…")
    _reset_batch_workspace()

    # 2. 顺序转译（共享工作区，每个测试积累到 src/bin/<class>.rs）
    print(f"[batch] 顺序转译 {len(pending)} 个测试…")
    transpile_ok: list[Path] = []
    transpile_fail: list[Path] = []
    for java_file in pending:
        rel = java_file.relative_to(ROOT)
        print(f"  [transpile] {rel}…", end=" ", flush=True)
        ok, log = _transpile(java_file, batch=True)
        if ok:
            print("OK")
            transpile_ok.append(java_file)
        else:
            print("FAIL")
            print(log[-300:])
            transpile_fail.append(java_file)

    if not transpile_ok:
        print("所有转译均失败，退出。")
        return 1

    # 3. 预清理目标 binary（确保构建后只有新编译成功的才存在）
    for java_file in transpile_ok:
        bin_path = OUT / "target" / "debug" / _to_bin_name(_class_name(java_file))
        if bin_path.exists():
            bin_path.unlink()

    # 4. 一次性 cargo build --bins --keep-going（遇到单个 binary 错误仍继续其余）
    print(f"\n[batch] cargo build --bins --keep-going (cargo -j {jobs})…")
    r = _run(["cargo", "build", f"--jobs={jobs}", "--bins", "--keep-going"], cwd=OUT)
    if r.returncode != 0:
        # 有编译失败，但部分 binary 可能已成功——继续后续步骤
        failed_lines = [ln for ln in r.stderr.splitlines() if ln.startswith("error")]
        print(f"[batch] build 部分失败（{len(failed_lines)} 个 error），继续运行已成功的 binary…")
    else:
        print("[batch] build OK")

    # 4. 并行运行所有 binary（直接执行 target/debug/<bin>，不经 cargo）
    print(f"\n[batch] 并行运行 {len(transpile_ok)} 个 binary (max_workers={jobs})…\n")

    passed = failed = 0

    # 按 binary 是否存在区分编译成功/失败
    build_ok: list[Path] = []
    build_fail: list[Path] = []
    for java_file in transpile_ok:
        bin_path = OUT / "target" / "debug" / _to_bin_name(_class_name(java_file))
        (build_ok if bin_path.exists() else build_fail).append(java_file)

    for java_file in build_fail:
        print(f"[ FAIL ] {java_file.relative_to(ROOT)}  — compile error")
        failed += 1

    def _run_one(java_file: Path) -> tuple[Path, bool, str, list[str]]:
        class_name = _class_name(java_file)
        ok, actual = _run_binary(class_name)
        if not ok:
            return java_file, False, "binary error", []
        expected = _read_expected(class_name)
        diff = _diff(expected, actual, class_name)
        return java_file, len(diff) == 0, "", diff

    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_run_one, f): f for f in build_ok}
        for fut in as_completed(futures):
            java_file, ok, err_msg, diff = fut.result()
            rel = java_file.relative_to(ROOT)
            if not ok and err_msg:
                print(f"[ FAIL ] {rel}  — {err_msg}")
                failed += 1
            elif diff:
                print(f"[ FAIL ] {rel}")
                print("".join(diff[:40]))
                if len(diff) > 40:
                    print(f"  … ({len(diff) - 40} more lines)")
                failed += 1
            else:
                print(f"[ PASS ] {rel}")
                passed += 1

    for java_file in transpile_fail:
        print(f"[ FAIL ] {java_file.relative_to(ROOT)}  — transpile error")
        failed += 1

    total = passed + failed + skipped
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    return 0 if failed == 0 else 1


# ── 入口 ─────────────────────────────────────────────────────────────

def run_tests(filter_str: str | None, no_run: bool, update_expected: bool, jobs: int) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    if update_expected:
        print(f"Updating expected output for {len(files)} file(s)…")
        for f in files:
            print(f"\n[update] {f.relative_to(ROOT)}")
            _update_expected(f)
        return 0

    if jobs > 1:
        return _run_parallel(filter_str, jobs)
    return _run_sequential(filter_str, no_run)


def main():
    ap = argparse.ArgumentParser(description="java_rta 端到端测试框架")
    ap.add_argument("--filter",          metavar="STR", help="只测试路径中包含此字符串的文件")
    ap.add_argument("--no-run",          action="store_true", help="只生成 Rust，不执行对比（仅顺序模式）")
    ap.add_argument("--update-expected", action="store_true", help="重新生成 expected/*.txt（用 java 运行）")
    ap.add_argument("--jobs", "-j",      type=int, default=1, metavar="N",
                    help="并行测试数（默认 1 = 顺序模式；0 = CPU 核数）")
    args = ap.parse_args()

    jobs = args.jobs
    if jobs == 0:
        jobs = os.cpu_count() or 4

    sys.exit(run_tests(args.filter, args.no_run, args.update_expected, jobs))


if __name__ == "__main__":
    main()
