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
OUT    = ROOT / "output"   # 可被 main() 通过 --out-dir 覆盖


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


def _test_workspace(bin_name: str) -> Path:
    """返回该测试的专属工作区：output/runs/<bin_name>/。"""
    return OUT / "runs" / bin_name


def _init_test_workspace(ws: Path) -> None:
    """首次调用时初始化独立测试工作区（符号链接共享只读 crate，共享 target/）。"""
    if (ws / "Cargo.toml").exists():
        return
    ws.mkdir(parents=True, exist_ok=True)
    default = ROOT / "output"

    # 符号链接指向共享只读 crate（相对路径，目录可移动）
    for shared in ("java_runtime", "java_rta_macros"):
        src = default / shared
        dst = ws / shared
        if src.exists() and not dst.exists():
            dst.symlink_to(os.path.relpath(src, ws))

    # 复制 workspace Cargo.toml
    src_toml = default / "Cargo.toml"
    if src_toml.exists():
        (ws / "Cargo.toml").write_text(src_toml.read_text())

    # jdk_classes/ 和 user/ 骨架目录 + Cargo.toml
    for crate in ("jdk_classes", "user"):
        crate_dir = ws / crate
        (crate_dir / "src").mkdir(parents=True, exist_ok=True)
        cargo_src = default / crate / "Cargo.toml"
        if cargo_src.exists():
            (crate_dir / "Cargo.toml").write_text(cargo_src.read_text())

    # 共享 target/ 目录（避免每个测试重新编译 java_runtime / java_rta_macros）
    shared_target = os.path.relpath(default / "target", ws)
    (ws / ".cargo").mkdir(exist_ok=True)
    (ws / ".cargo" / "config.toml").write_text(
        f'[build]\ntarget-dir = "{shared_target}"\n'
    )


def _transpile(java_file: Path, batch: bool = False,
               out_dir: Path | None = None) -> tuple[bool, str]:
    """运行转译器生成 Rust 代码。batch=True 时写 src/bin/<class>.rs。"""
    out = out_dir or OUT
    args = [sys.executable, str(ROOT / "scripts" / "main.py"), str(java_file),
            "--no-run", "--out", str(out)]
    if batch:
        args.append("--batch")
    r = _run(args, cwd=ROOT)
    return r.returncode == 0, (r.stdout + r.stderr)


def _cargo_run(class_name: str, out_dir: Path | None = None) -> tuple[bool, str]:
    """顺序模式：cargo run --bin <class>。"""
    out = out_dir or OUT
    bin_name = _to_bin_name(class_name)
    r = _run(["cargo", "run", "--bin", bin_name], cwd=out)
    if r.returncode != 0:
        for line in r.stderr.splitlines():
            if line.startswith('error'):
                print(f"  stderr: {line[:120]}", flush=True)
                break
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


_RUST_KEYWORDS = {
    'as','break','const','continue','crate','else','enum','extern','false',
    'fn','for','if','impl','in','let','loop','match','mod','move','mut',
    'pub','ref','return','self','Self','static','struct','super','trait',
    'true','type','unsafe','use','where','while','async','await','dyn',
    'abstract','become','box','do','final','macro','override','priv',
    'typeof','unsized','virtual','yield',
}


def _mod_decl(name: str) -> str:
    safe = f'r#{name}' if name in _RUST_KEYWORDS else name
    return f'pub mod {safe};'


def _use_decl(name: str) -> str:
    safe = f'r#{name}' if name in _RUST_KEYWORDS else name
    return f'pub use {safe}::*;'


def _rebuild_jdk_mod_index() -> None:
    """手动恢复工具：从磁盘全量重建 jdk_classes/src/ 的 lib.rs 和所有 mod.rs。

    正常情况下不需要调用此函数——write_cargo_project(batch_bin=True) 在写完每次
    测试的 stub 文件后已内置磁盘全量扫描重建逻辑（project_writer.py）。
    此函数保留用于工作区损坏时的手动修复。
    """
    jdk_src = OUT / "jdk_classes" / "src"
    if not jdk_src.exists():
        return

    # 第一步：收集所有 class .rs 文件（非 lib.rs/mod.rs）
    mod_tree: dict[Path, set[str]] = {}
    for p in sorted(jdk_src.rglob('*.rs')):
        if p.name not in ('lib.rs', 'mod.rs'):
            mod_tree.setdefault(p.parent, set()).add(p.stem)

    # 第二步：自底向上传播目录（只声明非空目录，避免 E0583）
    changed = True
    while changed:
        changed = False
        for dir_path in list(mod_tree.keys()):
            if dir_path == jdk_src:
                continue
            parent = dir_path.parent
            if dir_path.name not in mod_tree.get(parent, set()):
                mod_tree.setdefault(parent, set()).add(dir_path.name)
                changed = True

    # 重建 lib.rs
    top_mods = sorted(mod_tree.get(jdk_src, set()))
    lib_lines = [
        '#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]',
        *(_mod_decl(m) for m in top_mods),
        '',
    ]
    (jdk_src / 'lib.rs').write_text('\n'.join(lib_lines))

    # 重建各子包 mod.rs（仅限 jdk_src 子目录，顶层用 lib.rs）
    for dir_path, children in mod_tree.items():
        if dir_path == jdk_src:
            continue
        mod_lines = ['#![allow(ambiguous_glob_reexports)]']
        for c in sorted(children):
            mod_lines.append(_mod_decl(c))
            mod_lines.append(_use_decl(c))
        (dir_path / 'mod.rs').write_text('\n'.join(mod_lines) + '\n')

    total = sum(len(v) for v in mod_tree.values())
    print(f"[batch] 重建 jdk_classes mod 索引：{total} 个模块条目")


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
        bin_name   = _to_bin_name(class_name)
        rel        = java_file.relative_to(ROOT)
        expected   = _read_expected(class_name)

        if expected is None:
            print(f"[ SKIP ] {rel}  (no expected/{class_name}.txt)")
            skipped += 1
            continue

        # 每个测试用独立工作区，避免多进程/多次运行互相覆盖
        ws = _test_workspace(bin_name)
        _init_test_workspace(ws)

        ok, log = _transpile(java_file, out_dir=ws)
        if not ok:
            print(f"[ FAIL ] {rel}  — transpile error")
            print(log[-500:])
            failed += 1
            continue

        if no_run:
            print(f"[NORUN ] {rel}  — transpile OK, skipping run")
            skipped += 1
            continue

        ok, actual = _cargo_run(class_name, out_dir=ws)
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


def _ensure_workspace(out: Path) -> None:
    """确保 out_dir 是可用的 Cargo workspace。
    若目录不存在，用符号链接指向 output/ 下的共享只读 crate，新建 jdk_classes/ 和 user/。
    若目录已存在且有 Cargo.toml，直接使用。
    """
    if out == ROOT / "output":
        return  # 默认目录，已完整初始化
    if (out / "Cargo.toml").exists():
        return  # 用户自行准备的目录

    out.mkdir(parents=True, exist_ok=True)
    default = ROOT / "output"

    # 对只读 crate 创建符号链接（不复制，节省磁盘；build 产物隔离在各自 target/）
    for shared in ("java_runtime", "java_rta_macros"):
        src = default / shared
        dst = out / shared
        if src.exists() and not dst.exists():
            dst.symlink_to(src.resolve())

    # 复制 workspace Cargo.toml（内含成员列表，无法跨路径共享）
    src_toml = default / "Cargo.toml"
    if src_toml.exists():
        (out / "Cargo.toml").write_text(src_toml.read_text())

    # 创建 jdk_classes/ 和 user/ 框架（Cargo.toml 内容由转译器填充）
    for crate, cargo_src in (("jdk_classes", default / "jdk_classes" / "Cargo.toml"),
                              ("user",        default / "user"        / "Cargo.toml")):
        crate_dir = out / crate
        (crate_dir / "src").mkdir(parents=True, exist_ok=True)
        dst_toml = crate_dir / "Cargo.toml"
        if not dst_toml.exists() and cargo_src.exists():
            dst_toml.write_text(cargo_src.read_text())

    print(f"[workspace] 初始化新工作区 {out}")


def main():
    global OUT
    ap = argparse.ArgumentParser(description="java_rta 端到端测试框架")
    ap.add_argument("--filter",          metavar="STR", help="只测试路径中包含此字符串的文件")
    ap.add_argument("--no-run",          action="store_true", help="只生成 Rust，不执行对比（仅顺序模式）")
    ap.add_argument("--update-expected", action="store_true", help="重新生成 expected/*.txt（用 java 运行）")
    ap.add_argument("--out-dir",         metavar="DIR", default=None,
                    help="Cargo workspace 目录（默认 output/；多进程并行时指定不同目录避免冲突）")
    ap.add_argument("--jobs", "-j",      type=int, default=1, metavar="N",
                    help="并行测试数（默认 1 = 顺序模式；0 = CPU 核数）")
    args = ap.parse_args()

    if args.out_dir is not None:
        OUT = Path(args.out_dir)
        if not OUT.is_absolute():
            OUT = ROOT / OUT
        _ensure_workspace(OUT)

    jobs = args.jobs
    if jobs == 0:
        jobs = os.cpu_count() or 4

    sys.exit(run_tests(args.filter, args.no_run, args.update_expected, jobs))


if __name__ == "__main__":
    main()
