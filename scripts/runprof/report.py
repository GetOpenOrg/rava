#!/usr/bin/env python3
"""符号化 sampler.c 的样本（R1 运行性能剖析）。

用法：report.py <runprof.raw> <可执行文件> [--top N]
输出：自耗（首帧函数）、含子调用（样本内任一帧，按样本去重）、首帧行级热点、热点函数的调用者。
"""
import collections
import subprocess
import sys


def main() -> None:
    raw, exe = sys.argv[1], sys.argv[2]
    top = int(sys.argv[sys.argv.index("--top") + 1]) if "--top" in sys.argv else 60
    exe_real = __import__("os").path.realpath(exe)
    base = None
    libs = []  # (lo, hi, name)
    samples = []
    section = None
    for line in open(raw):
        line = line.rstrip("\n")
        if line in ("maps", "samples"):
            section = line
            continue
        if section == "maps":
            parts = line.split()
            if len(parts) < 5:
                continue
            lo, hi = (int(x, 16) for x in parts[0].split("-"))
            path = parts[5] if len(parts) > 5 else ""
            if path and __import__("os").path.realpath(path) == exe_real:
                if base is None:
                    base = lo
                continue
            if "x" in parts[1]:
                libs.append((lo, hi, path.rsplit("/", 1)[-1] or "anon"))
            continue
        addrs = [int(x, 16) for x in line.split()]
        if addrs:
            samples.append(addrs)
    base = base or 0

    def lib_of(a):
        for lo, hi, n in libs:
            if lo <= a < hi:
                return n
        return None

    uniq = set()
    for s in samples:
        for k, a in enumerate(s):
            if lib_of(a) is None:
                uniq.add(a - base - (1 if k > 0 else 0))
    uniq = sorted(uniq)
    sym = {}
    if uniq:
        out = subprocess.run(["addr2line", "-f", "-C", "-e", exe], input="\n".join(hex(a) for a in uniq),
                             capture_output=True, text=True).stdout.splitlines()
        for i, a in enumerate(uniq):
            fn = out[2 * i] if 2 * i < len(out) else "?"
            loc = out[2 * i + 1] if 2 * i + 1 < len(out) else "?"
            sym[a] = (shorten(fn), loc.split(" ")[0])

    def name(a, k):
        lib = lib_of(a)
        if lib:
            return (f"[{lib}]", "")
        return sym.get(a - base - (1 if k > 0 else 0), ("?", "?"))

    n = len(samples)
    selfc = collections.Counter()
    linec = collections.Counter()
    incl = collections.Counter()
    callers = collections.defaultdict(collections.Counter)
    for s in samples:
        frames = [name(a, k) for k, a in enumerate(s)]
        selfc[frames[0][0]] += 1
        linec[frames[0][1] + "  " + frames[0][0][:60]] += 1
        for f in set(f[0] for f in frames):
            incl[f] += 1
        if len(frames) > 1:
            callers[frames[0][0]][frames[1][0]] += 1
    print(f"samples {n}")
    print("\n== self ==")
    for f, c in selfc.most_common(top):
        print(f"{100*c/n:6.2f}%  {f}")
    print("\n== inclusive ==")
    for f, c in incl.most_common(top):
        print(f"{100*c/n:6.2f}%  {f}")
    print("\n== self lines ==")
    for f, c in linec.most_common(top):
        print(f"{100*c/n:6.2f}%  {f}")
    print("\n== callers of top self ==")
    for f, _ in selfc.most_common(15):
        print(f"-- {f}")
        for g, c in callers[f].most_common(5):
            print(f"     {100*c/n:6.2f}%  {g}")


def shorten(fn: str) -> str:
    # 去掉泛型实参与哈希后缀，保留路径末三段
    out, depth = [], 0
    for ch in fn:
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth -= 1
        elif depth == 0:
            out.append(ch)
    s = "".join(out)
    segs = s.split("::")
    if segs and segs[-1].startswith("h") and len(segs[-1]) == 17:
        segs = segs[:-1]
    return "::".join(segs[-4:])


if __name__ == "__main__":
    main()
