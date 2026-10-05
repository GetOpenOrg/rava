#!/usr/bin/env python3
"""release 系构建档位对照（二进制体积 B3，docs/plans/2026-10-04-binary-size.md）。

每个用例转译一次，再按各档位分别冷编译（每档独立 target 目录，依赖一并重编，构建耗时可比），
然后交替轮流运行各档产物多次，取运行中位数。每个测点同时记三项：运行耗时、构建耗时、二进制大小
（另记构建峰值内存：单个 rustc 进程的最大常驻集）。

用法（在仓库根目录）：
    scripts/opt_profile_bench.py [--profiles release,release-small] [--reps 5] [--out b3] 01_basics/HelloWorld ...
用例为 tests/e2e 下的相对路径（不含 .java）。结果写 <out>_<用例>.json，并在 stdout 打印汇总行。
"""
from __future__ import annotations

import argparse
import json
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RAVA = ROOT / "build" / "analyzer-target" / "release" / "rava"
FLAGS = {"release": "--release", "release-small": "--release-small"}


# 构建峰值内存：经一层 Python 包装取其全部已回收后代进程的最大常驻集（rustc 单进程峰值）
PEAK_MARK = "__B3_PEAK_RSS_KB__"
PEAK_WRAP = (
    "import resource,subprocess,sys\n"
    "rc=subprocess.call(sys.argv[1:])\n"
    "kb=resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss\n"
    f"print('{PEAK_MARK}', kb//1024 if sys.platform=='darwin' else kb, flush=True)\n"
    "sys.exit(rc)"
)


def peak_mb(log: Path) -> int | None:
    for line in log.read_text(errors="replace").splitlines():
        if line.startswith(PEAK_MARK):
            return int(line.split()[1]) // 1024
    return None


def timed(cmd: list, log: Path, timeout: float | None = None) -> tuple[int, float]:
    t = time.monotonic()
    with open(log, "w") as f:
        try:
            rc = subprocess.run(cmd, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, timeout=timeout).returncode
        except subprocess.TimeoutExpired:
            rc = -9
    return rc, time.monotonic() - t


def newest_status() -> Path:
    found = list((ROOT / "build").glob("*/build_status.json")) + list((ROOT / "build").glob("*/*/build_status.json"))
    return max(found, key=lambda p: p.stat().st_mtime)


def text_bytes(exe: Path) -> int | None:
    """代码节大小：ELF `.text`（size -A）/ Mach-O `__TEXT,__text`（size -m）"""
    try:
        if sys.platform == "darwin":
            out = subprocess.run(["size", "-m", str(exe)], capture_output=True, text=True).stdout
            for line in out.splitlines():
                if "Section __text:" in line:
                    return int(line.split(":")[1].split()[0])
        else:
            out = subprocess.run(["size", "-A", str(exe)], capture_output=True, text=True).stdout
            for line in out.splitlines():
                parts = line.split()
                if parts and parts[0] == ".text":
                    return int(parts[1])
    except (OSError, ValueError):
        pass
    return None


def bench(test: str, profiles: list, reps: int, out: str, run_timeout: float) -> dict:
    name = Path(test).name
    rec: dict = {"test": test, "profiles": {}}
    rc, secs = timed([str(RAVA), "build", f"tests/e2e/{test}.java", "--stop-after", "emit"], Path(f"{out}_{name}_emit.log"))
    rec["transpile_s"] = round(secs, 2)
    if rc != 0:
        rec["error"] = f"emit rc={rc}"
        return rec
    scratch = newest_status().parent
    bins = ROOT / "build" / "b3bin"
    bins.mkdir(parents=True, exist_ok=True)
    for p in profiles:
        tdir = ROOT / "build" / "b3t" / f"{name}_{p}"
        shutil.rmtree(tdir, ignore_errors=True)
        cmd = [str(RAVA), "compile", str(scratch), FLAGS[p], "--target-dir", str(tdir), "--build-timeout", "5400"]
        log = Path(f"{out}_{name}_{p}_build.log")
        rc, secs = timed([sys.executable, "-c", PEAK_WRAP, *cmd], log)
        st = json.loads((scratch / "build_status.json").read_text())
        r = {"build_s": round(secs, 1), "build_rc": rc, "build_peak_mb": peak_mb(log), "runs": [], "match": None}
        if rc == 0 and st.get("exe"):
            exe = bins / f"{name}.{p}"
            shutil.copy2(st["exe"], exe)
            r["bin_bytes"] = exe.stat().st_size
            r["text_bytes"] = text_bytes(exe)
        shutil.rmtree(tdir, ignore_errors=True)
        rec["profiles"][p] = r
    expected = ROOT / "tests" / "expected" / f"{name}.txt"
    want = expected.read_text() if expected.exists() else None
    live = [p for p in profiles if "bin_bytes" in rec["profiles"][p]]
    for i in range(reps):
        # 交替轮转档位次序，抵消机器状态随时间的漂移
        for p in live[i % len(live):] + live[: i % len(live)]:
            r = rec["profiles"][p]
            t = time.monotonic()
            try:
                cp = subprocess.run([str(bins / f"{name}.{p}")], cwd=ROOT, capture_output=True, text=True, timeout=run_timeout)
                ok_rc, stdout = cp.returncode, cp.stdout
            except subprocess.TimeoutExpired:
                ok_rc, stdout = -9, ""
            r["runs"].append(round(time.monotonic() - t, 3))
            same = ok_rc == 0 and (want is None or stdout == want)
            r["match"] = same if r["match"] is None else (r["match"] and same)
    for p in live:
        r = rec["profiles"][p]
        r["run_median_s"] = round(statistics.median(r["runs"]), 3)
    return rec


def summary(rec: dict, base: str, alt: str) -> str:
    ps = rec["profiles"]
    if base not in ps or alt not in ps or "run_median_s" not in ps[base] or "run_median_s" not in ps[alt]:
        brief = {p: {k: v for k, v in r.items() if k != "runs"} for p, r in ps.items()}
        return f"{rec['test']}\tINCOMPLETE\t{json.dumps(brief, ensure_ascii=False)}"
    b, a = ps[base], ps[alt]
    run = a["run_median_s"] / b["run_median_s"] - 1 if b["run_median_s"] else 0.0
    size = 1 - a["bin_bytes"] / b["bin_bytes"]
    return (f"{rec['test']}\trun {b['run_median_s']}→{a['run_median_s']} s ({run:+.1%})"
            f"\tbuild {b['build_s']}→{a['build_s']} s\tpeak {b.get('build_peak_mb')}→{a.get('build_peak_mb')} MB\tbin {b['bin_bytes']}→{a['bin_bytes']} B (-{size:.1%})"
            f"\ttext {b.get('text_bytes')}→{a.get('text_bytes')}\tmatch {b['match']}/{a['match']}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("tests", nargs="+")
    ap.add_argument("--profiles", default="release,release-small")
    ap.add_argument("--reps", type=int, default=5)
    ap.add_argument("--out", default="b3")
    ap.add_argument("--run-timeout", type=float, default=900)
    a = ap.parse_args()
    profiles = a.profiles.split(",")
    bad = [p for p in profiles if p not in FLAGS]
    if bad:
        ap.error(f"未知档位：{bad}（可选 {list(FLAGS)}）")
    fail = 0
    for t in a.tests:
        rec = bench(t, profiles, a.reps, a.out, a.run_timeout)
        Path(f"{a.out}_{Path(t).name}.json").write_text(json.dumps(rec, ensure_ascii=False, indent=1))
        line = summary(rec, profiles[0], profiles[-1])
        print(line, flush=True)
        fail |= "INCOMPLETE" in line or "False" in line
    return 1 if fail else 0


if __name__ == "__main__":
    sys.exit(main())
