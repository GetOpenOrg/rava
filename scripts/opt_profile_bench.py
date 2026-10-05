#!/usr/bin/env python3
"""release 系构建档位对照（二进制体积 B3 / B4，docs/plans/2026-10-04-binary-size.md）。

每个用例转译一次，再按各档位分别冷编译（每档独立 target 目录，依赖一并重编，构建耗时可比），
然后交替轮流运行各档产物多次，取运行中位数。每个测点同时记三项：运行耗时、构建耗时、二进制大小。
构建内存记三个口径：
- build_peak_mb：单个 rustc 进程的最大常驻集（B3 口径）；
- build_tree_peak_mb：构建进程树常驻集之和的采样峰值（并行 rustc 叠加后的真实占用，B4 判据）；
- build_cgroup_peak_mb：所在 cgroup v2 的 memory.current 采样峰值（含页缓存，仅 Linux 受限 scope 下有）。
另以 scripts/crate_mem_profile.py 为 RUSTC_WRAPPER 记逐 crate 峰值（<out>_<用例>_<档位>_crates.jsonl），
连同各 crate 源码体量（<out>_<用例>_src.json）供内存感知作业数的系数校准。

档位：命名档（release / release-small / release-max，即 rava 的同名开关）或候选组合
（以 CARGO_PROFILE_RELEASE_* 环境变量覆盖 release 档的 opt-level / lto / codegen-units，见 CANDIDATES）。

用法（在仓库根目录）：
    scripts/opt_profile_bench.py [--profiles o3-thin,o3-local16] [--reps 5] [--out b4] 01_basics/HelloWorld ...
用例为 tests/e2e 下的相对路径（不含 .java）。结果写 <out>_<用例>.json，并在 stdout 打印汇总行。
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import statistics
import subprocess
import sys
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RAVA = ROOT / "build" / "analyzer-target" / "release" / "rava"

def _cand(opt: str, lto: str, cgu: int) -> tuple:
    env = {"CARGO_PROFILE_RELEASE_OPT_LEVEL": opt, "CARGO_PROFILE_RELEASE_LTO": lto,
           "CARGO_PROFILE_RELEASE_CODEGEN_UNITS": str(cgu)}
    return ("--release", env)


# 档位名 → (rava 开关, 覆盖 release 档的环境变量)。lto "false" 即 cargo 缺省：只做 crate 内 thin-local LTO
CANDIDATES = {
    "release": ("--release", {}),
    "release-small": ("--release-small", {}),
    "release-max": ("--release-max", {}),
    "o3-fat": _cand("3", "fat", 1),
    "o3-thin": _cand("3", "thin", 1),
    "o3-thin16": _cand("3", "thin", 16),
    "o3-local16": _cand("3", "false", 16),
    "o2-thin": _cand("2", "thin", 1),
    "s-fat": _cand("s", "fat", 1),
}
CRATE_WRAP = ROOT / "scripts" / "crate_mem_profile.py"


def _cgroup_current() -> Path | None:
    """本进程所在 cgroup v2 的 memory.current（不可读则 None）"""
    try:
        line = next(l for l in Path("/proc/self/cgroup").read_text().splitlines() if l.startswith("0::"))
        p = Path("/sys/fs/cgroup") / line[3:].lstrip("/") / "memory.current"
        int(p.read_text())
        return p
    except (OSError, StopIteration, ValueError):
        return None


def _tree_rss_kb(root: int) -> int:
    """root 及其全部后代进程的常驻集之和（KB）"""
    procs: dict[int, tuple[int, int]] = {}
    if Path("/proc/self/stat").exists():
        page_kb = os.sysconf("SC_PAGE_SIZE") // 1024
        for d in Path("/proc").iterdir():
            if not d.name.isdigit():
                continue
            try:
                rest = (d / "stat").read_text().rsplit(")", 1)[1].split()
            except (OSError, IndexError):
                continue
            procs[int(d.name)] = (int(rest[1]), int(rest[21]) * page_kb)
    else:
        out = subprocess.run(["ps", "-A", "-o", "pid=,ppid=,rss="], capture_output=True, text=True).stdout
        for line in out.splitlines():
            f = line.split()
            if len(f) == 3:
                procs[int(f[0])] = (int(f[1]), int(f[2]))
    kids: dict[int, list[int]] = {}
    for pid, (ppid, _) in procs.items():
        kids.setdefault(ppid, []).append(pid)
    total, stack = 0, [root]
    while stack:
        pid = stack.pop()
        total += procs.get(pid, (0, 0))[1]
        stack.extend(kids.get(pid, []))
    return total


class MemSampler:
    """构建期间每 0.5 s 采样一次进程树常驻集之和与 cgroup memory.current，记峰值（MB）"""

    def __init__(self, pid: int):
        self.pid, self.tree, self.cg = pid, 0, 0
        self.cg_path = _cgroup_current()
        self.stop = threading.Event()
        self.th = threading.Thread(target=self._loop, daemon=True)
        self.th.start()

    def _loop(self):
        while not self.stop.is_set():
            self.tree = max(self.tree, _tree_rss_kb(self.pid) // 1024)
            if self.cg_path:
                try:
                    self.cg = max(self.cg, int(self.cg_path.read_text()) // (1024 * 1024))
                except (OSError, ValueError):
                    pass
            self.stop.wait(0.5)

    def done(self) -> tuple[int, int | None]:
        self.stop.set()
        self.th.join()
        return self.tree, (self.cg if self.cg_path else None)


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


def sampled(cmd: list, log: Path, env: dict) -> tuple[int, float, int, int | None]:
    """执行构建命令并采样内存：(rc, 秒, 进程树峰值 MB, cgroup 峰值 MB)"""
    t = time.monotonic()
    with open(log, "w") as f:
        p = subprocess.Popen(cmd, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, env=env)
        ms = MemSampler(p.pid)
        rc = p.wait()
    tree, cg = ms.done()
    return rc, time.monotonic() - t, tree, cg


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
    sys.path.insert(0, str(CRATE_WRAP.parent))
    from crate_mem_profile import src_stats
    Path(f"{out}_{name}_src.json").write_text(json.dumps(src_stats(scratch), indent=1))
    bins = ROOT / "build" / "b3bin"
    bins.mkdir(parents=True, exist_ok=True)
    for p in profiles:
        tdir = ROOT / "build" / "b3t" / f"{name}_{p}"
        shutil.rmtree(tdir, ignore_errors=True)
        flag, over = CANDIDATES[p]
        cmd = [str(RAVA), "compile", str(scratch), flag, "--target-dir", str(tdir), "--build-timeout", "5400"]
        log = Path(f"{out}_{name}_{p}_build.log")
        crates = Path(f"{out}_{name}_{p}_crates.jsonl").resolve()
        crates.write_text("")
        prof_dir = ROOT / "build" / "b4prof" / f"{name}_{p}"
        prof_dir.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, **over, RUSTC_WRAPPER=str(CRATE_WRAP), PROFILE_LOG=str(crates), PROFILE_DIR=str(prof_dir))
        rc, secs, tree, cg = sampled([sys.executable, "-c", PEAK_WRAP, *cmd], log, env)
        st = json.loads((scratch / "build_status.json").read_text())
        r = {"build_s": round(secs, 1), "build_rc": rc, "build_peak_mb": peak_mb(log), "build_tree_peak_mb": tree,
             "build_cgroup_peak_mb": cg, "cargo_jobs": (st.get("heavy") or {}).get("jobs") or os.environ.get("CARGO_BUILD_JOBS"),
             "env": over, "runs": [], "match": None}
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
            f"\tbuild {b['build_s']}→{a['build_s']} s\tpeak {b.get('build_peak_mb')}→{a.get('build_peak_mb')} MB"
            f"\ttree {b.get('build_tree_peak_mb')}→{a.get('build_tree_peak_mb')} MB\tbin {b['bin_bytes']}→{a['bin_bytes']} B (-{size:.1%})"
            f"\ttext {b.get('text_bytes')}→{a.get('text_bytes')}\tmatch {b['match']}/{a['match']}")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("tests", nargs="+")
    ap.add_argument("--profiles", default="release,release-small")
    ap.add_argument("--reps", type=int, default=5)
    ap.add_argument("--out", default="b4")
    ap.add_argument("--run-timeout", type=float, default=900)
    a = ap.parse_args()
    profiles = a.profiles.split(",")
    bad = [p for p in profiles if p not in CANDIDATES]
    if bad:
        ap.error(f"未知档位：{bad}（可选 {list(CANDIDATES)}）")
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
