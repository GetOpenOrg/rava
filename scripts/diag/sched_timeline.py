#!/usr/bin/env python3
"""主线程调度时间线：启动命令，每 INTERVAL 秒读 /proc/<pid>/task/<pid>/schedstat 与 status，
输出主线程在 CPU 上 / 就绪排队（CPU 争用）/ 其余（阻塞）三段时间与自愿 / 非自愿切换增量，
另给全进程线程数与全部线程 CPU 合计。用于判断主线程墙钟里不在 CPU 的部分是争用还是阻塞。

用法：sched_timeline.py [--interval 0.25] -- <cmd...>；程序 stdout / stderr 透传到本脚本的 stderr。
"""
import os
import subprocess
import sys
import time


def read(path):
    try:
        with open(path) as f:
            return f.read()
    except OSError:
        return None


def main_stat(pid):
    ss = read(f"/proc/{pid}/task/{pid}/schedstat")
    st = read(f"/proc/{pid}/task/{pid}/status")
    if ss is None or st is None:
        return None
    run, wait, _ = (int(x) for x in ss.split())
    vol = nvol = 0
    for ln in st.splitlines():
        if ln.startswith("voluntary_ctxt_switches"):
            vol = int(ln.split()[1])
        elif ln.startswith("nonvoluntary_ctxt_switches"):
            nvol = int(ln.split()[1])
    return run, wait, vol, nvol


def all_cpu(pid):
    total = 0
    n = 0
    try:
        tids = os.listdir(f"/proc/{pid}/task")
    except OSError:
        return 0, 0
    for t in tids:
        ss = read(f"/proc/{pid}/task/{t}/schedstat")
        if ss:
            total += int(ss.split()[0])
            n += 1
    return total, n


def main():
    args = sys.argv[1:]
    interval = 0.25
    if args and args[0] == "--interval":
        interval = float(args[1])
        args = args[2:]
    if args and args[0] == "--":
        args = args[1:]
    p = subprocess.Popen(args, stdout=sys.stderr, stderr=sys.stderr)
    t0 = time.monotonic()
    prev = None
    prev_t = t0
    prev_all = 0
    print(f"{'t(s)':>6} {'run%':>6} {'wait%':>6} {'block%':>7} {'vol':>6} {'nvol':>6} {'thr':>4} {'allcpu':>7}")
    while p.poll() is None:
        time.sleep(interval)
        now = time.monotonic()
        s = main_stat(p.pid)
        a, n = all_cpu(p.pid)
        if s is None:
            break
        if prev is not None:
            dt = (now - prev_t) * 1e9
            run = (s[0] - prev[0]) / dt * 100
            wait = (s[1] - prev[1]) / dt * 100
            print(f"{now - t0:6.2f} {run:6.1f} {wait:6.1f} {max(0.0, 100 - run - wait):7.1f} "
                  f"{s[2] - prev[2]:6d} {s[3] - prev[3]:6d} {n:4d} {(a - prev_all) / dt:7.2f}", flush=True)
        prev, prev_t, prev_all = s, now, a
    p.wait()
    print(f"rc={p.returncode} wall={time.monotonic() - t0:.2f}s")


if __name__ == "__main__":
    main()
