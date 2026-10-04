#!/usr/bin/env python3
"""进程内存时间线采样（Linux，a3-T6 规模剖析用）。

用法：rss_timeline.py <out.tsv> <cmd...>
每 100 ms 读一次被测进程（沿子进程链取最深后代）的 /proc/<pid>/status 与 /proc/<pid>/smaps，按映射分类汇总驻留：
  stack —— 协程栈 slab（匿名映射，大小为槽跨度 1 MiB + 1 页 的整数倍且 ≥16 槽）；
  anon  —— 其余匿名映射（malloc 堆 / arena、平台线程栈等）；
  file  —— 文件映射（二进制代码与只读数据）。
时间线写入 out.tsv（墙钟毫秒、VmRSS、VmPTE、各类 Rss、线程数），结束时在 stderr 打印峰值汇总；退出码同被测进程。
"""
import os
import re
import subprocess
import sys
import time

PAGE = os.sysconf("SC_PAGE_SIZE")
STRIDE = (1 << 20) + PAGE
MAP_RE = re.compile(r"^([0-9a-f]+)-([0-9a-f]+) \S+ \S+ \S+ (\d+)\s*(.*)$")


def sample(pid: int) -> dict | None:
    try:
        status = open(f"/proc/{pid}/status").read()
        smaps = open(f"/proc/{pid}/smaps").read().splitlines()
    except OSError:
        return None
    f = {}
    for line in status.splitlines():
        k, _, v = line.partition(":")
        if k in ("VmRSS", "VmPTE", "Threads", "VmHWM"):
            f[k] = int(v.split()[0])
    acc = {"stack": 0, "anon": 0, "file": 0, "stack_maps": 0, "maps": 0}
    kind = None
    for line in smaps:
        m = MAP_RE.match(line)
        if m:
            lo, hi, inode, path = int(m[1], 16), int(m[2], 16), int(m[3]), m[4]
            size = hi - lo
            acc["maps"] += 1
            if inode == 0 and not path.startswith("/"):
                if size >= 16 * STRIDE and size % STRIDE == 0:
                    kind = "stack"
                    acc["stack_maps"] += 1
                else:
                    kind = "anon"
            else:
                kind = "file"
        elif line.startswith("Rss:") and kind:
            acc[kind] += int(line.split()[1])
    f.update(acc)
    return f


def leaf(pid: int) -> int:
    """沿 /proc 子进程链下行到最深的后代（命令可能经 /usr/bin/time 等包装器启动）。"""
    while True:
        try:
            kids = open(f"/proc/{pid}/task/{pid}/children").read().split()
        except OSError:
            return pid
        if not kids:
            return pid
        pid = int(kids[0])


def main() -> int:
    out, cmd = sys.argv[1], sys.argv[2:]
    p = subprocess.Popen(cmd)
    t_start = time.time()
    peak: dict[str, int] = {}
    cols = ["wall_ms", "VmRSS", "VmPTE", "stack", "anon", "file", "Threads", "stack_maps", "maps"]
    with open(out, "w") as fo:
        fo.write("\t".join(cols) + "\n")
        while p.poll() is None:
            s = sample(leaf(p.pid))
            if s:
                s["wall_ms"] = int(time.time() * 1000)
                fo.write("\t".join(str(s.get(c, "")) for c in cols) + "\n")
                for k, v in s.items():
                    if k != "wall_ms":
                        peak[k] = max(peak.get(k, 0), v)
            time.sleep(0.1)
    rc = p.wait()
    wall = time.time() - t_start
    kb = lambda k: f"{peak.get(k, 0) / 1024:.1f} MiB"
    print(f"[rss_timeline] rc={rc} wall={wall:.2f}s peak VmRSS={kb('VmRSS')} VmHWM={kb('VmHWM')} "
          f"VmPTE={kb('VmPTE')} stack={kb('stack')} anon={kb('anon')} file={kb('file')} "
          f"threads={peak.get('Threads', 0)} stack_maps={peak.get('stack_maps', 0)} maps={peak.get('maps', 0)}",
          file=sys.stderr)
    return rc


if __name__ == "__main__":
    sys.exit(main())
