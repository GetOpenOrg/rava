#!/usr/bin/env python3
"""逐 crate rustc 峰值 / 墙钟 / 分阶段 RSS（拆 crate 线，docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.7）。

可移植（Linux / macOS），供服务器作业使用；峰值取 wait4 的 ru_maxrss，被 OOM 杀掉时同样记录。

  scripts/crate_mem_profile.py run <scratch> <out_dir> [--passes] [--rava PATH]
      统计各 crate 源码体量，再以本脚本为 RUSTC_WRAPPER 执行 `rava compile <scratch>`（作业数等设置与
      run_tests 一致），输出 <out_dir>/crates.jsonl、<out_dir>/report.md。
      --passes：对 java_runtime 加 `-Z time-passes`（RUSTC_BOOTSTRAP=1，stable 可用），阶段行落
      <out_dir>/java_runtime.passes.log；另每秒采样 RSS 落 <out_dir>/<crate>.rss.log。

  作为 RUSTC_WRAPPER 被 cargo 调用时：argv[1] = rustc，其余为 rustc 参数；PROFILE_LOG 指定 jsonl。
"""

import json
import os
import subprocess
import sys
import threading
import time
from pathlib import Path

WS_PREFIXES = ("java_runtime", "java_meta", "java_body_", "user")


def crate_name(args):
    for i, a in enumerate(args):
        if a == "--crate-name" and i + 1 < len(args):
            return args[i + 1]
    return ""


def rss_mb(pid):
    try:
        with open(f"/proc/{pid}/status") as f:
            for line in f:
                if line.startswith("VmRSS:"):
                    return int(line.split()[1]) // 1024
    except OSError:
        pass
    try:
        out = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
        return int(out) // 1024 if out else None
    except Exception:
        return None


def wrap(argv):
    rustc, args = argv[0], argv[1:]
    name = crate_name(args)
    tracked = os.environ.get("PROFILE_CRATES", "").split()
    if not name or not (name in tracked or name.startswith(WS_PREFIXES)):
        os.execvp(rustc, [rustc, *args])
    out_dir = Path(os.environ["PROFILE_DIR"])
    env = dict(os.environ)
    passes = os.environ.get("PROFILE_PASSES") == "1" and name == "java_runtime"
    if passes:
        args = [*args, "-Z", "time-passes"]
        env["RUSTC_BOOTSTRAP"] = "1"
    t0 = time.time()
    p = subprocess.Popen([rustc, *args], env=env, stderr=subprocess.PIPE if passes else None, close_fds=False)
    samples = []
    stop = threading.Event()

    def sample():
        while not stop.is_set():
            r = rss_mb(p.pid)
            if r is not None:
                samples.append((round(time.time() - t0, 1), r))
            stop.wait(1.0)

    th = threading.Thread(target=sample, daemon=True)
    th.start()
    if passes:
        with open(out_dir / f"{name}.passes.log", "w") as pf:
            for raw in p.stderr:
                line = raw.decode("utf-8", "replace")
                if line.startswith("time:"):
                    pf.write(line)
                    pf.flush()
                else:
                    sys.stderr.write(line)
    _, status, ru = os.wait4(p.pid, 0)
    stop.set()
    th.join()
    wall = time.time() - t0
    rc = os.waitstatus_to_exitcode(status)
    maxrss = ru.ru_maxrss // (1024 * 1024 if sys.platform == "darwin" else 1024)
    rec = {"crate": name, "wall_s": round(wall, 1), "peak_mb": maxrss, "rc": rc,
           "user_s": round(ru.ru_utime, 1), "sys_s": round(ru.ru_stime, 1)}
    with open(os.environ["PROFILE_LOG"], "a") as f:
        f.write(json.dumps(rec) + "\n")
    with open(out_dir / f"{name}.rss.log", "w") as f:
        f.writelines(f"{t} {r}\n" for t, r in samples)
    sys.exit(rc if rc >= 0 else 128 - rc)


def src_stats(scratch):
    rows = []
    for d in sorted(scratch.iterdir()):
        if not (d / "Cargo.toml").exists() or not (d / "src").is_dir():
            continue
        files = list((d / "src").rglob("*.rs"))
        total = sum(f.stat().st_size for f in files)
        gen = [f for f in files if b"rava_macros::java_class" in f.read_bytes()[:4096] or b"java_class!" in f.read_bytes()]
        rows.append({"crate": d.name, "files": len(files), "bytes": total,
                     "gen_files": len(gen), "gen_bytes": sum(f.stat().st_size for f in gen)})
    return rows


def run(argv):
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("scratch")
    ap.add_argument("out")
    ap.add_argument("--passes", action="store_true")
    ap.add_argument("--rava", default="build/analyzer-target/release/rava")
    a = ap.parse_args(argv)
    scratch, out = Path(a.scratch).resolve(), Path(a.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    stats = src_stats(scratch)
    (out / "src_stats.json").write_text(json.dumps(stats, indent=1))
    log = out / "crates.jsonl"
    log.write_text("")
    env = dict(os.environ, RUSTC_WRAPPER=str(Path(__file__).resolve()), PROFILE_LOG=str(log),
               PROFILE_DIR=str(out), PROFILE_PASSES="1" if a.passes else "0")
    t0 = time.time()
    rc = subprocess.run([a.rava, "compile", str(scratch)], env=env).returncode
    wall = time.time() - t0
    recs = [json.loads(l) for l in log.read_text().splitlines() if l.strip()]
    lines = [f"# crate_mem_profile {scratch.name}", "", f"rava compile rc={rc}，总墙钟 {wall:.1f} s", "",
             "| crate | 文件 | 源码 MB | 生成类文件 | 墙钟 s | 峰值 MB | rc |", "|---|---:|---:|---:|---:|---:|---:|"]
    by = {s["crate"]: s for s in stats}
    for r in recs:
        s = by.get(r["crate"], {})
        lines.append(f"| {r['crate']} | {s.get('files', '')} | {s.get('bytes', 0) / 1e6:.2f} | {s.get('gen_files', '')} "
                     f"| {r['wall_s']} | {r['peak_mb']} | {r['rc']} |")
    if a.passes and (out / "java_runtime.passes.log").exists():
        lines += ["", "## java_runtime 阶段", "", "```", (out / "java_runtime.passes.log").read_text()[-20000:], "```"]
    (out / "report.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))
    return rc


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "run":
        sys.exit(run(sys.argv[2:]))
    wrap(sys.argv[1:])
