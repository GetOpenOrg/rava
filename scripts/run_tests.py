#!/usr/bin/env python3
"""
端到端测试框架（T51）— per-test scratch workspace 版。

用法：
    python3 scripts/run_tests.py                         # 全量运行（顺序）
    python3 scripts/run_tests.py -j 4                    # 并行运行，最多 4 个并发
    python3 scripts/run_tests.py -j 0                    # 并行运行，并发数 = CPU 核数
    python3 scripts/run_tests.py --filter 01_basics      # 只跑指定目录
    python3 scripts/run_tests.py --filter TestArrayList  # 只跑指定类名
    python3 scripts/run_tests.py --update-expected       # 重新生成 expected/*.txt
    python3 scripts/run_tests.py --no-run                # 只生成 Rust，不执行对比

工作区模型（见 docs/plans/2026-09-16-per-test-scratch-workspace.md）：
    build/<test>/   每测试独立 scratch（手写 overlay + 该测试的生成代码）
    build/target/   共享编译缓存（CARGO_TARGET_DIR：syn/quote/宏 crate 指纹稳定，
                    跨测试复用；java_runtime 因生成内容不同各自编译）

流程：
  对每个 tests/e2e/**/*.java：
  1. main.py 转译 → overlay 手写 + 生成 build/<test>/{java_runtime,user}
  2. cargo run --bin <class>（共享 target 缓存）捕获 stdout
  3. 与 tests/expected/<Class>.txt diff
"""

import argparse
import difflib
import os
import re
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

ROOT   = Path(__file__).parent.parent
TESTS  = ROOT / "tests"
E2E    = TESTS / "e2e"
EXPECT = TESTS / "expected"
OUT    = ROOT / "build"     # scratch 根（可被 --out-dir 覆盖）
SHARED_TARGET = OUT / "target"


def fmt_dur(sec: float) -> str:
    """格式化耗时：<60s 用秒（两位小数），≥60s 用 m 分 s 秒。"""
    if sec < 60:
        return f"{sec:.2f}s"
    m, s = divmod(sec, 60)
    return f"{int(m)}m{s:04.1f}s"


def _cargo_env() -> dict:
    """共享编译缓存环境变量。"""
    return dict(os.environ, CARGO_TARGET_DIR=str(SHARED_TARGET))


def _run(cmd: list[str], cwd: Path, capture: bool = True,
         env: dict | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=capture, text=True, env=env)


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


def _test_workspace(bin_name: str) -> Path:
    """每测试独立 scratch 工作区。"""
    return OUT / bin_name


def _transpile(java_file: Path, out_dir: Path) -> tuple[bool, str]:
    """运行转译器：overlay 手写代码 + 生成该测试的 Rust 代码。"""
    args = [sys.executable, str(ROOT / "scripts" / "main.py"), str(java_file),
            "--no-run", "--out", str(out_dir)]
    r = _run(args, cwd=ROOT)
    return r.returncode == 0, (r.stdout + r.stderr)


def _cargo_run(class_name: str, out_dir: Path) -> tuple[bool, str]:
    """cargo run --bin <class>（共享 target 缓存）。"""
    bin_name = _to_bin_name(class_name)
    r = _run(["cargo", "run", "--bin", bin_name], cwd=out_dir, env=_cargo_env())
    if r.returncode != 0:
        for line in r.stderr.splitlines():
            if line.startswith('error'):
                print(f"  stderr: {line[:120]}", flush=True)
                break
    return r.returncode == 0, r.stdout


def _run_binary(class_name: str) -> tuple[bool, str]:
    """直接执行已编译的 binary（共享 target 目录下，不经 cargo 避免锁竞争）。"""
    bin_name = _to_bin_name(class_name)
    bin_path = SHARED_TARGET / "debug" / bin_name
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


# ── 顺序模式 ─────────────────────────────────────────────────────────

def _run_sequential(filter_str: str | None, no_run: bool) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    passed = failed = skipped = 0
    t_all = time.perf_counter()
    t_transpile_total = t_run_total = 0.0

    for java_file in files:
        class_name = _class_name(java_file)
        bin_name   = _to_bin_name(class_name)
        rel        = java_file.relative_to(ROOT)
        expected   = _read_expected(class_name)

        if expected is None:
            print(f"[ SKIP ] {rel}  (no expected/{class_name}.txt)")
            skipped += 1
            continue

        ws = _test_workspace(bin_name)
        t0 = time.perf_counter()
        ok, log = _transpile(java_file, out_dir=ws)
        t_transpile = time.perf_counter() - t0
        t_transpile_total += t_transpile
        if not ok:
            print(f"[ FAIL ] {rel}  — transpile error ({fmt_dur(t_transpile)})")
            print(log[-500:])
            failed += 1
            continue

        if no_run:
            print(f"[NORUN ] {rel}  — transpile OK, skipping run ({fmt_dur(t_transpile)})")
            skipped += 1
            continue

        t0 = time.perf_counter()
        ok, actual = _cargo_run(class_name, out_dir=ws)
        t_run = time.perf_counter() - t0
        t_run_total += t_run
        if not ok:
            print(f"[ FAIL ] {rel}  — cargo run error (transpile {fmt_dur(t_transpile)}, run {fmt_dur(t_run)})")
            failed += 1
            continue

        diff = _diff(expected, actual, class_name)
        if diff:
            print(f"[ FAIL ] {rel}  (transpile {fmt_dur(t_transpile)}, run {fmt_dur(t_run)})")
            print("".join(diff[:40]))
            if len(diff) > 40:
                print(f"  … ({len(diff) - 40} more lines)")
            failed += 1
        else:
            print(f"[ PASS ] {rel}  (transpile {fmt_dur(t_transpile)}, run {fmt_dur(t_run)})")
            passed += 1

    total = passed + failed + skipped
    elapsed = time.perf_counter() - t_all
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    print(f"Elapsed: {fmt_dur(elapsed)}"
          f"  (transpile {fmt_dur(t_transpile_total)}, run {fmt_dur(t_run_total)})")
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

    # 1. 并行转译（每测试独立 scratch，无共享状态，可安全并发）
    print(f"\n[batch] 并行转译 {len(pending)} 个测试（max_workers={jobs}）…")
    t_all = time.perf_counter()

    def _transpile_one(java_file: Path) -> tuple[Path, bool, str]:
        ws = _test_workspace(_to_bin_name(_class_name(java_file)))
        return java_file, *_transpile(java_file, out_dir=ws)

    transpile_ok: list[Path] = []
    transpile_fail: list[Path] = []
    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_transpile_one, f): f for f in pending}
        for fut in as_completed(futures):
            java_file, ok, log = fut.result()
            rel = java_file.relative_to(ROOT)
            if ok:
                print(f"  [transpile] {rel} OK", flush=True)
                transpile_ok.append(java_file)
            else:
                print(f"  [transpile] {rel} FAIL", flush=True)
                print(log[-300:])
                transpile_fail.append(java_file)

    if not transpile_ok:
        print("所有转译均失败，退出。")
        return 1

    t_transpile = time.perf_counter() - t_all
    print(f"[time] 转译阶段 {fmt_dur(t_transpile)}（OK {len(transpile_ok)} / FAIL {len(transpile_fail)}）")

    # 2. 逐测试 cargo build --bin <name>
    #    共享 CARGO_TARGET_DIR 下 cargo 以文件锁串行化构建——并发调用只会互相
    #    等待，因此这里顺序构建；syn/quote/宏依赖缓存命中后每个测试只编译自己的
    #    窄语料 java_runtime + user bin。
    print(f"\n[batch] 顺序构建 {len(transpile_ok)} 个测试的 binary（共享 target 缓存）…")
    t_build_start = time.perf_counter()
    build_ok: list[Path] = []
    build_fail: list[Path] = []
    build_durations: dict[str, float] = {}
    for java_file in transpile_ok:
        class_name = _class_name(java_file)
        bin_name = _to_bin_name(class_name)
        ws = _test_workspace(bin_name)
        print(f"  [build] {bin_name}…", end=" ", flush=True)
        t0 = time.perf_counter()
        r = _run(["cargo", "build", "--bin", bin_name], cwd=ws, env=_cargo_env())
        dur = time.perf_counter() - t0
        build_durations[bin_name] = dur
        if r.returncode == 0:
            print(f"OK ({fmt_dur(dur)})", flush=True)
            build_ok.append(java_file)
        else:
            err = next((ln for ln in r.stderr.splitlines() if ln.startswith("error")),
                       "unknown error")
            print(f"FAIL ({fmt_dur(dur)})  {err[:100]}", flush=True)
            build_fail.append(java_file)

    t_build = time.perf_counter() - t_build_start
    slowest = max(build_durations.items(), key=lambda kv: kv[1], default=("", 0.0))
    print(f"[time] 构建阶段 {fmt_dur(t_build)}（OK {len(build_ok)} / FAIL {len(build_fail)}"
          f"，最慢 {slowest[0]} {fmt_dur(slowest[1])}）")

    # 3. 并行运行所有 binary（直接执行 target/debug/<bin>，不经 cargo）
    print(f"\n[batch] 并行运行 {len(build_ok)} 个 binary (max_workers={jobs})…\n")
    t_run_start = time.perf_counter()

    passed = failed = 0

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
    elapsed = time.perf_counter() - t_all
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    print(f"Elapsed: {fmt_dur(elapsed)}"
          f"  (transpile {fmt_dur(t_transpile)}, build {fmt_dur(t_build)},"
          f" run {fmt_dur(time.perf_counter() - t_run_start)})")
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
    global OUT, SHARED_TARGET
    ap = argparse.ArgumentParser(description="java_rta 端到端测试框架")
    ap.add_argument("--filter",          metavar="STR", help="只测试路径中包含此字符串的文件")
    ap.add_argument("--no-run",          action="store_true", help="只生成 Rust，不执行对比（仅顺序模式）")
    ap.add_argument("--update-expected", action="store_true", help="重新生成 expected/*.txt（用 java 运行）")
    ap.add_argument("--out-dir",         metavar="DIR", default=None,
                    help="scratch 根目录（默认 build/；每测试在其下建独立子目录）")
    ap.add_argument("--jobs", "-j",      type=int, default=1, metavar="N",
                    help="并行测试数（默认 1 = 顺序模式；0 = CPU 核数）")
    args = ap.parse_args()

    if args.out_dir is not None:
        OUT = Path(args.out_dir)
        if not OUT.is_absolute():
            OUT = ROOT / OUT
        SHARED_TARGET = OUT / "target"

    jobs = args.jobs
    if jobs == 0:
        jobs = os.cpu_count() or 4

    sys.exit(run_tests(args.filter, args.no_run, args.update_expected, jobs))


if __name__ == "__main__":
    main()
