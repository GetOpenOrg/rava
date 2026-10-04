#!/usr/bin/env python3
"""sampler.so 样本的符号化与汇总（a3-T6 剖析）。

用法：sampler_report.py <二进制> <前缀.pid.samples> [top=40]
按函数汇总自身（栈顶）与包含（栈上任一帧，每样本每函数计一次）占比，分「全部线程」与「主线程」两组。
符号取自 `nm -n -C`（只要函数名，不解析 DWARF）；二进制之外的帧按所在模块名记。
"""
import bisect
import collections
import os
import subprocess
import sys


def load_maps(path):
    maps = []
    for line in open(path):
        parts = line.split()
        if len(parts) < 6:
            continue
        lo, hi = (int(x, 16) for x in parts[0].split("-"))
        maps.append((lo, hi, int(parts[2], 16), parts[5]))
    return maps


def main():
    binary, samples = sys.argv[1], sys.argv[2]
    top = int(sys.argv[3]) if len(sys.argv) > 3 else 40
    maps = load_maps(samples[: -len(".samples")] + ".maps")
    real = os.path.realpath(binary)
    base = min(lo for lo, _, off, p in maps if os.path.realpath(p) == real and off == 0)
    addrs, names = [], []
    nm = subprocess.run(["nm", "-n", "-C", "--defined-only", binary], capture_output=True, text=True).stdout
    for line in nm.splitlines():
        parts = line.split(" ", 2)
        if len(parts) == 3 and parts[1] in "tTwW":
            addrs.append(int(parts[0], 16))
            names.append(parts[2])
    starts = [m[0] for m in maps]
    cache = {}

    def sym(pc):
        if pc in cache:
            return cache[pc]
        i = bisect.bisect_right(starts, pc) - 1
        name = "?"
        if i >= 0 and pc < maps[i][1]:
            lo, _, off, path = maps[i]
            if os.path.realpath(path) == real:
                j = bisect.bisect_right(addrs, pc - base - 1) - 1  # 返回地址减 1 落回调用指令
                name = names[j] if j >= 0 else "?"
            else:
                name = "[" + os.path.basename(path) + "]"
        cache[pc] = name
        return name

    lines = open(samples).read().splitlines()
    pid = int(lines[0].split()[1])
    groups = {"all": [collections.Counter(), collections.Counter(), 0],
              "main": [collections.Counter(), collections.Counter(), 0]}
    for line in lines[1:]:
        parts = line.split()
        tid = int(parts[0])
        frames = [sym(int(x, 16)) for x in parts[1:]]
        # 去掉采样器自身与信号返回帧
        while frames and frames[0] in ("[sampler.so]",):
            frames.pop(0)
        if frames and frames[0].startswith("[libc"):
            frames.pop(0)
        if not frames:
            continue
        for g in ("all", "main") if tid == pid else ("all",):
            selfc, incl, _ = groups[g]
            selfc[frames[0]] += 1
            for f in set(frames):
                incl[f] += 1
            groups[g][2] += 1
    for g, (selfc, incl, n) in groups.items():
        if not n:
            continue
        print(f"===== {g}: {n} samples")
        print("--- self")
        for f, c in selfc.most_common(top):
            print(f"{100 * c / n:6.2f}% {f[:200]}")
        print("--- inclusive")
        for f, c in incl.most_common(top * 2):
            print(f"{100 * c / n:6.2f}% {f[:200]}")


if __name__ == "__main__":
    main()
