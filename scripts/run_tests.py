#!/usr/bin/env python3
"""
端到端测试框架（T51）— per-test scratch workspace 版。

用法：
    python3 scripts/run_tests.py                         # 全量运行（顺序）
    python3 scripts/run_tests.py -j 4                    # 并行运行，最多 4 个并发
    python3 scripts/run_tests.py -j 0                    # 并行运行，并发数 = CPU 核数
    python3 scripts/run_tests.py --filter 01_basics          # 只跑指定目录
    python3 scripts/run_tests.py --filter TestArrayList      # 只跑指定类名
    python3 scripts/run_tests.py --filter 01 02 03           # 多个 filter（任意匹配）
    python3 scripts/run_tests.py --update-expected       # 重新生成 expected/*.txt（并行，-j 控制并发）
    python3 scripts/run_tests.py --no-run                # 只生成 Rust，不执行对比
    python3 scripts/run_tests.py --jdk 25                # 指定 JDK 主版本（javac/java/翻译语料同源）

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

sys.path.insert(0, str(Path(__file__).parent))
from jdk_select import resolve_jdk_home

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


# 每测试运行阶段超时（秒）；编译阶段用更宽的上限捕获病态构建
RUN_TIMEOUT = 300
BUILD_TIMEOUT = 600


def _jdk_tool(name: str) -> str:
    """javac / java 从 JAVA_HOME 同源解析（未设时回退 PATH）。"""
    home = os.environ.get('JAVA_HOME', '')
    if home:
        p = Path(home) / 'bin' / name
        if p.exists():
            return str(p)
    return name


def apply_jdk_choice(major: int) -> None:
    """--jdk N：解析本机安装并写入 JAVA_HOME（javac/java/翻译语料全部同源）。"""
    home = resolve_jdk_home(major)
    if home is None:
        from jdk_select import list_installed_jdks
        installed = '\n'.join(f"  JDK {m}: {h}" for m, h in list_installed_jdks())
        sys.exit(f"未找到 JDK {major}。本机已安装：\n{installed}")
    os.environ['JAVA_HOME'] = str(home)
    print(f"[jdk] JAVA_HOME → {home} (JDK {major})")


def _cargo_env() -> dict:
    """共享编译缓存环境变量。"""
    return dict(os.environ, CARGO_TARGET_DIR=str(SHARED_TARGET))


def _run(cmd: list[str], cwd: Path, capture: bool = True,
         env: dict | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(cmd, cwd=cwd, capture_output=capture, text=True, env=env)


def _discover(filter_str: list[str] | None) -> list[Path]:
    files = sorted(E2E.rglob("*.java"))
    if filter_str:
        files = [f for f in files if any(s in str(f) for s in filter_str)]
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


def _cargo_build(class_name: str, out_dir: Path) -> tuple[bool, str]:
    """cargo build --bin <class>（共享 target 缓存）。返回 (ok, 首个 error 行)。"""
    bin_name = _to_bin_name(class_name)
    try:
        r = _run(["cargo", "build", "--bin", bin_name], cwd=out_dir, env=_cargo_env())
    except subprocess.TimeoutExpired:
        return False, f"build timeout ({fmt_dur(BUILD_TIMEOUT)})"
    if r.returncode != 0:
        err = next((ln for ln in r.stderr.splitlines() if ln.startswith("error")),
                   "unknown build error")
        return False, err[:120]
    return True, ""


# ── V-3 可读性审计汇总 ──────────────────────────────────────────────

_READABILITY_RE = re.compile(r"^\[readability-audit\]\s+(.+)$", re.MULTILINE)


def _parse_readability(log: str) -> dict[str, int]:
    """从转译输出解析 [readability-audit] 行的禁用调用计数（A-2 口径）。"""
    m = _READABILITY_RE.search(log)
    if not m:
        return {}
    counts: dict[str, int] = {}
    for part in m.group(1).split():
        k, _, v = part.partition("=")
        try:
            counts[k] = int(v)
        except ValueError:
            pass
    return counts


def _print_readability_summary(per_test: dict[str, dict[str, int]]) -> None:
    """汇总各测试的可读性审计计数；目标全 0（A-2 禁用调用清零的验收口径）。"""
    if not per_test:
        return
    totals: dict[str, int] = {}
    nonzero: list[str] = []
    for name, counts in sorted(per_test.items()):
        if not counts:
            continue
        bad = {k: v for k, v in counts.items() if v}
        for k, v in counts.items():
            totals[k] = totals.get(k, 0) + v
        if bad:
            nonzero.append(f"{name}({', '.join(f'{k}={v}' for k, v in sorted(bad.items()))})")
    line = " ".join(f"{k}={v}" for k, v in sorted(totals.items()))
    print(f"\n[readability] {line}  (合计 {sum(totals.values())}，"
          f"{len(per_test)} 个测试，目标全 0)")
    if nonzero:
        shown = "; ".join(nonzero[:6])
        print(f"  非零: {shown}{'…' if len(nonzero) > 6 else ''}")


def _run_bin(class_name: str, timeout: int = RUN_TIMEOUT) -> tuple[str, str]:
    """直接执行 binary。返回 (状态, stdout)：状态 ∈ ok / timeout / error。"""
    bin_name = _to_bin_name(class_name)
    bin_path = SHARED_TARGET / "debug" / bin_name
    try:
        r = subprocess.run([str(bin_path)], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return "timeout", ""
    if r.returncode != 0:
        return "error", r.stdout
    return "ok", r.stdout


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


def _update_expected(java_file: Path) -> tuple[str, str]:
    """生成期望输出。返回 (状态, 摘要)：状态 ∈ updated / javac-fail / java-fail / timeout。

    classes 目录按测试独立（build/expected-classes/<bin>），支持并行无覆盖。"""
    class_name = _class_name(java_file)
    classes_dir = OUT / "expected-classes" / _to_bin_name(class_name)
    classes_dir.mkdir(parents=True, exist_ok=True)
    r = _run([_jdk_tool("javac"), "-g", "-d", str(classes_dir), str(java_file)], cwd=ROOT)
    if r.returncode != 0:
        return "javac-fail", r.stderr.strip().splitlines()[-1] if r.stderr.strip() else "javac error"
    try:
        r = subprocess.run([_jdk_tool("java"), "-cp", str(classes_dir), class_name],
                           cwd=ROOT, capture_output=True, text=True, timeout=RUN_TIMEOUT)
    except subprocess.TimeoutExpired:
        return "timeout", f"exceeded {fmt_dur(RUN_TIMEOUT)}"
    if r.returncode != 0:
        return "java-fail", (r.stderr.strip().splitlines() or ["java error"])[-1][:120]
    _expected_path(class_name).write_text(r.stdout)
    return "updated", f"{r.stdout.count(chr(10))} lines"


# ── 顺序模式 ─────────────────────────────────────────────────────────

def _run_sequential(filter_str: list[str] | None, no_run: bool) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    passed = failed = skipped = 0
    t_all = time.perf_counter()
    t_transpile_total = t_build_total = t_run_total = 0.0
    readability_counts: dict[str, dict[str, int]] = {}
    fail_categories: dict[str, list[str]] = {}

    def _fail(cat: str, rel_str: str) -> None:
        nonlocal failed
        failed += 1
        fail_categories.setdefault(cat, []).append(rel_str)

    total_files = len(files)
    for idx, java_file in enumerate(files, 1):
        prog = f"[{idx:>3}/{total_files}]"
        class_name = _class_name(java_file)
        bin_name   = _to_bin_name(class_name)
        rel        = java_file.relative_to(ROOT)
        expected   = _read_expected(class_name)

        if expected is None:
            print(f"{prog} [ SKIP ] {rel}  (no expected/{class_name}.txt)")
            skipped += 1
            continue

        ws = _test_workspace(bin_name)
        t0 = time.perf_counter()
        ok, log = _transpile(java_file, out_dir=ws)
        t_transpile = time.perf_counter() - t0
        t_transpile_total += t_transpile
        if not ok:
            print(f"{prog} [ FAIL ] {rel}  — transpile error ({fmt_dur(t_transpile)})")
            print(log[-500:])
            _fail("transpile", str(rel))
            continue

        _rc = _parse_readability(log)
        if _rc:
            readability_counts[class_name] = _rc

        if no_run:
            print(f"{prog} [NORUN ] {rel}  — transpile OK ({fmt_dur(t_transpile)})")
            skipped += 1
            continue

        t0 = time.perf_counter()
        ok, err = _cargo_build(class_name, out_dir=ws)
        t_build = time.perf_counter() - t0
        t_build_total += t_build
        if not ok:
            print(f"{prog} [ FAIL ] {rel}  — compile error ({fmt_dur(t_build)})  {err}")
            _fail("compile", str(rel))
            continue

        t0 = time.perf_counter()
        status, actual = _run_bin(class_name)
        t_run = time.perf_counter() - t0
        t_run_total += t_run
        timing = f"transpile {fmt_dur(t_transpile)}, build {fmt_dur(t_build)}, run {fmt_dur(t_run)}"
        if status == "timeout":
            print(f"{prog} [ FAIL ] {rel}  — run timeout (> {fmt_dur(RUN_TIMEOUT)})  ({timing})")
            _fail("run-timeout", str(rel))
            continue
        if status == "error":
            print(f"{prog} [ FAIL ] {rel}  — run error  ({timing})")
            _fail("run", str(rel))
            continue

        diff = _diff(expected, actual, class_name)
        if diff:
            print(f"{prog} [ FAIL ] {rel}  — output mismatch  ({timing})")
            print("".join(diff[:40]))
            if len(diff) > 40:
                print(f"  … ({len(diff) - 40} more lines)")
            _fail("output", str(rel))
        else:
            print(f"{prog} [ PASS ] {rel}  ({timing})")
            passed += 1

    total = passed + failed + skipped
    elapsed = time.perf_counter() - t_all
    print(f"\n{'='*50}")
    print(f"Results: {passed} passed, {failed} failed, {skipped} skipped / {total} total")
    if fail_categories:
        for cat, names in fail_categories.items():
            print(f"  {cat}: {len(names)}  ({', '.join(names[:6])}{'…' if len(names) > 6 else ''})")
    print(f"Elapsed: {fmt_dur(elapsed)}"
          f"  (transpile {fmt_dur(t_transpile_total)}, build {fmt_dur(t_build_total)},"
          f" run {fmt_dur(t_run_total)})")
    _print_readability_summary(readability_counts)
    return 0 if failed == 0 else 1


# ── 并行模式 ─────────────────────────────────────────────────────────

def _run_parallel(filter_str: list[str] | None, jobs: int) -> int:
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

    def _transpile_one(java_file: Path) -> tuple[Path, bool, str, dict[str, int]]:
        ws = _test_workspace(_to_bin_name(_class_name(java_file)))
        ok, log = _transpile(java_file, out_dir=ws)
        return java_file, ok, log, _parse_readability(log)

    transpile_ok: list[Path] = []
    transpile_fail: list[Path] = []
    readability_counts: dict[str, dict[str, int]] = {}
    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_transpile_one, f): f for f in pending}
        for fut in as_completed(futures):
            java_file, ok, log, rc = fut.result()
            rel = java_file.relative_to(ROOT)
            if ok:
                print(f"  [transpile] {rel} OK", flush=True)
                transpile_ok.append(java_file)
                if rc:
                    readability_counts[_class_name(java_file)] = rc
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
    _print_readability_summary(readability_counts)
    return 0 if failed == 0 else 1


# ── 期望输出更新（并行） ─────────────────────────────────────────────

def _update_expected_parallel(files: list[Path], jobs: int) -> int:
    """并行生成期望输出（javac/java 独立进程，classes 目录按测试独立）。

    结尾汇总四类：updated / javac-fail / java-fail / timeout——哪些测试
    没有正常生成、Java 侧报什么错，一目了然。"""
    print(f"Updating expected output for {len(files)} file(s) (max_workers={jobs})…")
    t_all = time.perf_counter()

    def _one(f: Path) -> tuple[Path, str, str]:
        return f, *_update_expected(f)

    results: dict[str, list[tuple[str, str]]] = {}
    with ThreadPoolExecutor(max_workers=jobs) as executor:
        futures = {executor.submit(_one, f): f for f in files}
        done = 0
        for fut in as_completed(futures):
            java_file, status, detail = fut.result()
            done += 1
            rel = str(java_file.relative_to(ROOT))
            results.setdefault(status, []).append((rel, detail))
            mark = {"updated": "OK", "javac-fail": "JAVAC-FAIL",
                    "java-fail": "JAVA-FAIL", "timeout": "TIMEOUT"}.get(status, status)
            print(f"[{done:>3}/{len(files)}] [{mark:>10}] {rel}  {detail}", flush=True)

    print(f"\n{'='*50}")
    updated = results.get("updated", [])
    print(f"updated: {len(updated)} / {len(files)}")
    for status in ("javac-fail", "java-fail", "timeout"):
        items = results.get(status, [])
        if items:
            print(f"\n{status}: {len(items)}")
            for rel, detail in items:
                print(f"  {rel}\n    {detail}")
    print(f"Elapsed: {fmt_dur(time.perf_counter() - t_all)}")
    return 0 if len(updated) == len(files) else 1


# ── 入口 ─────────────────────────────────────────────────────────────

def run_tests(filter_str: list[str] | None, no_run: bool, update_expected: bool, jobs: int) -> int:
    files = _discover(filter_str)
    if not files:
        print(f"No test files found (filter={filter_str!r})")
        return 1

    if update_expected:
        return _update_expected_parallel(files, jobs)

    if jobs > 1:
        return _run_parallel(filter_str, jobs)
    return _run_sequential(filter_str, no_run)


def main():
    global OUT, SHARED_TARGET
    ap = argparse.ArgumentParser(description="java_rta 端到端测试框架")
    ap.add_argument("--filter",          metavar="STR", nargs="+", help="只测试路径中包含任意指定字符串的文件（可传多个）")
    ap.add_argument("--no-run",          action="store_true", help="只生成 Rust，不执行对比（仅顺序模式）")
    ap.add_argument("--update-expected", action="store_true", help="重新生成 expected/*.txt（用 java 运行）")
    ap.add_argument("--out-dir",         metavar="DIR", default=None,
                    help="scratch 根目录（默认 build/；每测试在其下建独立子目录）")
    ap.add_argument("--jobs", "-j",      type=int, default=1, metavar="N",
                    help="并行测试数（默认 1 = 顺序模式；0 = CPU 核数）")
    ap.add_argument("--jdk",             type=int, default=None, metavar="N",
                    help="指定 JDK 主版本（javac/java/翻译语料同源；默认沿用 JAVA_HOME 或自动发现）")
    args = ap.parse_args()

    if args.out_dir is not None:
        OUT = Path(args.out_dir)
        if not OUT.is_absolute():
            OUT = ROOT / OUT
        SHARED_TARGET = OUT / "target"

    if args.jdk is not None:
        apply_jdk_choice(args.jdk)

    jobs = args.jobs
    if jobs == 0:
        jobs = os.cpu_count() or 4

    sys.exit(run_tests(args.filter, args.no_run, args.update_expected, jobs))


if __name__ == "__main__":
    main()
