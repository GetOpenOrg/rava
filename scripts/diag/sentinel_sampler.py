#!/usr/bin/env python3
"""sentinel16g RSS 采样器（T11）：逐 `rava build` 进程树采样 RSS 曲线，越限杀树。

机制承 scan32g watchdog（/tmp/scan32g/watchdog.py，1368 例次零误杀），差别在：
- 只盯本 worktree 的 `rava build` 树（识别 cmdline[0] basename == rava 且 argv[1] == build、
  cwd 在 --worktree 之下）——JVM golden / dyn 对照的 java 进程不进曲线；
- 曲线按固定粒度落 TSV（t_offset_s\trss_mib），供病理档速率拟合与存档；
- 两种终局：越 --limit-gib 即 SIGKILL 全树（outcome=limit-kill）；固定窗口 --window-s 到点
  杀树（outcome=window-kill；--full 认证档 window=0 = 不设窗口，跑完为止）；
  进程自然消失 = outcome=completed；
- 结果落 JSON（stdout 末行 RESULT <json>）：peak_mib / rate_mib_s（最小二乘斜率）/
  curve_sha256 / samples / outcome / tsv 路径。

断言口径（gatefix 教训）：结论只从本 JSON 文本判，退出码仅表示采样器自身健康。
用法：
  sentinel_sampler.py --worktree <wt> --limit-gib 16 --interval 5 \
      --tsv build/sentinel/<date>/curves/<case>.tsv [--window-s 240] [--quiet-grace 60]
"""
import argparse
import hashlib
import json
import os
import signal
import time

PAGE = os.sysconf("SC_PAGE_SIZE")


def procs():
    """快照 {pid: (ppid, cmdline_list, cwd)}；读失败的条目跳过。"""
    out = {}
    for name in os.listdir("/proc"):
        if not name.isdigit():
            continue
        pid = int(name)
        try:
            with open(f"/proc/{pid}/stat", "rb") as f:
                stat = f.read()
            rest = stat[stat.rindex(b")") + 2:].split()
            ppid = int(rest[1])
            with open(f"/proc/{pid}/cmdline", "rb") as f:
                cmd = f.read().decode("utf-8", "replace").split("\0")
            cmd = [c for c in cmd if c]
            if not cmd:
                continue  # 内核线程
            try:
                cwd = os.readlink(f"/proc/{pid}/cwd")
            except OSError:
                cwd = None
            out[pid] = (ppid, cmd, cwd)
        except (OSError, ValueError, IndexError):
            continue
    return out


def statm_rss_kb(pid):
    try:
        with open(f"/proc/{pid}/statm") as f:
            return int(f.read().split()[1]) * PAGE // 1024
    except (OSError, ValueError, IndexError):
        return 0


def find_roots(snap, wt, wt_pfx):
    """本 worktree 内的全部 `rava build` 根进程 [(pid, tree_pids)]。"""
    children = {}
    for pid, (ppid, _, _) in snap.items():
        children.setdefault(ppid, []).append(pid)

    def tree(pid):
        acc = [pid]
        for c in children.get(pid, ()):
            acc.extend(tree(c))
        return acc

    roots = []
    for pid, (ppid, cmd, cwd) in snap.items():
        if os.path.basename(cmd[0]) == "rava" and len(cmd) >= 2 and cmd[1] == "build":
            if cwd is not None and (cwd == wt or cwd.startswith(wt_pfx)):
                roots.append((pid, tree(pid)))
    return roots


def kill_tree(pids):
    for p in sorted(pids, reverse=True):
        try:
            os.kill(p, signal.SIGKILL)
        except OSError:
            pass


def slope_mib_per_s(ts, rss):
    """最小二乘斜率（MiB/s）；样本 <2 或时间跨度为 0 时返回 0。"""
    n = len(ts)
    if n < 2:
        return 0.0
    mx = sum(ts) / n
    my = sum(rss) / n
    sxx = sum((t - mx) ** 2 for t in ts)
    if sxx <= 0:
        return 0.0
    return sum((t - mx) * (y - my) for t, y in zip(ts, rss)) / sxx


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--worktree", required=True)
    ap.add_argument("--limit-gib", type=float, required=True)
    ap.add_argument("--interval", type=float, default=5.0)
    ap.add_argument("--tsv", required=True)
    ap.add_argument("--window-s", type=float, default=0.0,
                    help="固定窗口秒（0 = 不设窗口，跑到进程自然结束/越限）")
    ap.add_argument("--quiet-grace", type=float, default=60.0,
                    help="未见任何 rava build 的最长等待（等转译启动）；见过之后进程消失即收尾")
    args = ap.parse_args()
    wt = args.worktree.rstrip("/")
    wt_pfx = wt + "/"
    limit_mib = args.limit_gib * 1024
    os.makedirs(os.path.dirname(args.tsv), exist_ok=True)

    t0 = time.monotonic()
    started = None          # 首次见到 rava build 的单调时刻（窗口 / 曲线 t=0）
    ts, rss_hist = [], []
    peak_mib = 0.0
    outcome = "no-run"      # no-run / limit-kill / window-kill / completed
    with open(args.tsv, "w", buffering=1) as f:
        f.write("# t_offset_s\trss_mib\tlimit_gib=%s\twindow_s=%s\n"
                % (args.limit_gib, args.window_s))
        while True:
            now = time.monotonic()
            snap = procs()
            roots = find_roots(snap, wt, wt_pfx)
            if roots:
                if started is None:
                    started = now
                rss_mib = sum(statm_rss_kb(p) for _, tree in roots for p in tree) / 1024.0
                # 已被杀但 /proc 尚存僵尸条目（RSS 归零）不进曲线
                if rss_mib > 0:
                    ts.append(now - started)
                    rss_hist.append(rss_mib)
                    f.write("%.1f\t%.2f\n" % (now - started, rss_mib))
                    peak_mib = max(peak_mib, rss_mib)
                if rss_mib > limit_mib:
                    for _, tree in roots:
                        kill_tree(tree)
                    outcome = "limit-kill"
                    break
                if args.window_s and (now - started) >= args.window_s:
                    for _, tree in roots:
                        kill_tree(tree)
                    outcome = "window-kill"
                    break
            else:
                if started is not None:
                    outcome = "completed"   # 进程树自然消失
                    break
                if now - t0 > args.quiet_grace:
                    outcome = "no-run"      # 全程未见 rava build（命令没到转译段）
                    break
            time.sleep(args.interval)

    # 等被杀树退出后再收尾（SIGKILL 即时，小睡即可）
    time.sleep(1.0)
    curve = open(args.tsv, "rb").read()
    result = {
        "peak_gib": round(peak_mib / 1024.0, 3),
        "rate_mib_s": round(slope_mib_per_s(ts, rss_hist), 3),
        "final_gib": round(rss_hist[-1] / 1024.0, 3) if rss_hist else 0.0,
        "samples": len(ts),
        "dur_s": round(ts[-1], 1) if ts else 0.0,
        "outcome": outcome,
        "curve_sha256": hashlib.sha256(curve).hexdigest(),
        "tsv": args.tsv,
        "limit_gib": args.limit_gib,
        "window_s": args.window_s,
    }
    print("RESULT " + json.dumps(result, ensure_ascii=False))


if __name__ == "__main__":
    main()
