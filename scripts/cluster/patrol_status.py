#!/usr/bin/env python3
"""协调巡检一次性状态汇总（只读，不做任何修改）。

覆盖：测试进展（新完成 / 在跑的作业与抽查）、服务器资源（dev 与云服务器负载 / 内存 / 磁盘）、
子代理进展（运行时长 / 最近活动）、本机项目进程与资源、分支归属与远端遗留分支。

用法：python3 scripts/cluster/patrol_status.py [--no-remote]
- 服务器清单取自 cluster.toml（RAVA_CLUSTER_CONFIG 可覆盖），远端探测并行进行；
- 新完成判定以 ~/.cache/rava_patrol_seen 记录已汇报过的结果目录；
- 子代理进展读 Claude Code 任务输出目录（RAVA_AGENT_TASKS_GLOB 可覆盖），无则跳过。
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import shlex
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from cluster_config import CONFIG_PATH, REPO_ROOT, RESULTS_DIR  # noqa: E402

REPO = REPO_ROOT
CR = RESULTS_DIR
SEEN = Path.home() / ".cache" / "rava_patrol_seen"
INTEGRATION = "rust-closure-analyzer"
AGENT_TASKS_GLOB = os.environ.get(
    "RAVA_AGENT_TASKS_GLOB", f"/private/tmp/claude-{os.getuid()}/*rava*/*/tasks")
AGENT_RECENT_HOURS = 3        # 只列最近活动在此范围内的子代理
AGENT_MAX_HOURS = 6           # 超过即需收尾续作
AGENT_IDLE_MINUTES = 30       # 超过即需关注
REMOTE_DISK_MIN_G = 20
LOCAL_DISK_MIN_G = 25


def sh(cmd: str, cwd: Path | None = None, timeout: int = 60) -> str:
    try:
        return subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True,
                              timeout=timeout).stdout
    except subprocess.TimeoutExpired:
        return ""


def section(title: str) -> None:
    print(f"== {title}")


def load_servers() -> list[dict]:
    import tomllib
    try:
        return tomllib.loads(CONFIG_PATH.read_text()).get("servers", [])
    except OSError:
        return []


# ── 服务器占用（本机分发器进程） ──────────────────────────────────────────────

def dispatchers() -> list[dict]:
    """本机在跑的分发器：每个 .venv python 进程一条（uv 包装进程不计）。"""
    out = []
    for line in sh("pgrep -fl 'distribute_tests'").splitlines():
        pid, _, cmd = line.partition(" ")
        if not re.match(r"/\S*\.venv/bin/python3?\s", cmd):
            continue
        argv = cmd.split()
        servers = []
        if "--servers" in argv:
            for a in argv[argv.index("--servers") + 1:]:
                if a.startswith("-"):
                    break
                servers.append(a)
        m = re.search(r"--(job|spot) (\S+)", cmd)
        wt = re.match(r"(/\S*)/\.venv/", cmd)
        out.append({"pid": pid, "servers": servers, "kind": m.group(1) if m else "full",
                    "tag": m.group(2) if m else "全量", "wt": Path(wt.group(1)).name if wt else "?"})
    return out


def report_occupancy(disp: list[dict], servers: list[dict]) -> None:
    section("服务器占用（分发器进程）")
    busy = set()
    for d in sorted(disp, key=lambda d: (d["servers"], d["tag"])):
        print(f"  {' '.join(d['servers']) or '缺省'}  {d['kind']} {d['tag']}  pid={d['pid']}  {d['wt']}")
        busy.update(d["servers"])
    free = [s["label"] for s in servers if s["label"] not in busy]
    print(f"  空闲: {' '.join(free) or '无'}")
    per_wt: dict[str, set] = {}
    for d in disp:
        if d["wt"] != REPO.name:
            per_wt.setdefault(d["wt"], set()).update(d["servers"])
    for wt, ss in per_wt.items():
        if len(ss) > 2:
            print(f"  !! {wt} 同时占 {len(ss)} 台（上限 2）：{' '.join(sorted(ss))}")


# ── 测试进展 ──────────────────────────────────────────────────────────────────

def job_line(d: Path) -> str | None:
    s = d / "summary.json"
    if not s.exists():
        return None
    j = json.loads(s.read_text())
    entries = {k: v for k, v in j.items() if isinstance(v, dict)}
    if any(v.get("status") not in ("done", "failed", "timeout") for v in entries.values()):
        return None
    parts = []
    for k, v in entries.items():
        mark = "OOM" if v.get("oom") else ("超时" if v.get("timeout") else f"rc={v.get('rc', v.get('exit_code'))}")
        parts.append(f"{k}={mark}")
    return f"  job {d.name}: " + ", ".join(parts)


def spot_counts(d: Path) -> tuple[int, dict, int] | None:
    st = d / "state_jdk21.json"
    if not st.exists():
        return None
    j = json.loads(st.read_text())
    return len(j.get("passed", [])), j.get("failed", {}), len(j.get("leases", {}))


def report_results(disp: list[dict]) -> None:
    alive = {d["tag"] for d in disp}
    SEEN.parent.mkdir(parents=True, exist_ok=True)
    seen = set(SEEN.read_text().splitlines()) if SEEN.exists() else set()
    section("新完成结果（上次巡检后）")
    newly = []
    for d in sorted(list(CR.glob("job/*")) + list(CR.glob("spot/*"))):
        if not d.is_dir() or str(d) in seen or d.name in alive:
            continue
        if d.parent.name == "job":
            line = job_line(d)
        else:
            c = spot_counts(d)
            line = None if c is None else (
                f"  spot {d.name}: 通过 {c[0]}，失败 {len(c[1])}" + (f"，中断 {c[2]}" if c[2] else "")
                + (f"：{', '.join(list(c[1])[:12])}" if c[1] else ""))
        if line:
            print(line)
            newly.append(str(d))
    if newly:
        with SEEN.open("a") as fh:
            fh.write("\n".join(newly) + "\n")
    else:
        print("  无")
    section("在跑作业 / 抽查")
    for d in disp:
        base = CR / d["kind"] / d["tag"]
        if d["kind"] == "spot" and (c := spot_counts(base)):
            print(f"  spot {d['tag']}: 通过 {c[0]} 失败 {len(c[1])} 在跑 {c[2]}"
                  + (f"：{', '.join(list(c[1])[:8])}" if c[1] else ""))
        elif d["kind"] == "job" and (base / "summary.json").exists():
            j = json.loads((base / "summary.json").read_text())
            st = []
            for k, v in j.items():
                if not isinstance(v, dict):
                    continue
                s = v.get("status")
                if s == "running" and v.get("start"):
                    s += f" {int((time.time() - v['start']) / 60)}m@{v.get('server', '?')}"
                elif s == "done":
                    s = f"rc={v.get('rc')}"
                st.append(f"{k}={s}")
            print(f"  job {d['tag']}: {', '.join(st)}")


# ── 服务器资源 ────────────────────────────────────────────────────────────────

REMOTE_PROBE = (
    "echo \"$(nproc) $(cut -d' ' -f1 /proc/loadavg) "
    "$(free -g | awk '/Mem/{print $7, $2}') "
    "$(df -BG /data 2>/dev/null | awk 'NR==2{print $4}' | tr -d G) "
    "$(ls -d /data/rava-spot-* 2>/dev/null | wc -l) "
    "$(pgrep -c -f 'rava (build|closure)|cargo (test|build)|rustc')\""
)


def ssh_argv(server: dict, by_label: dict) -> list[str]:
    def target(s: dict) -> list[str]:
        a = ["-p", str(s.get("port", 22))]
        if s.get("private_key_path"):
            a += ["-i", os.path.expanduser(s["private_key_path"])]
        return a + [f"{s['username']}@{s['host']}"]
    argv = ["ssh", "-o", "ConnectTimeout=8", "-o", "BatchMode=yes", "-o", "StrictHostKeyChecking=accept-new"]
    if (j := server.get("jump")) and j in by_label:
        js = target(by_label[j])
        argv += ["-o", "ProxyCommand=" + " ".join(
            ["ssh", "-o", "BatchMode=yes", *map(shlex.quote, js[:-1]), "-W", "%h:%p", shlex.quote(js[-1])])]
    return argv + target(server)


def probe(server: dict, by_label: dict) -> tuple[str, list[str] | None]:
    try:
        r = subprocess.run(ssh_argv(server, by_label) + [REMOTE_PROBE], capture_output=True, text=True, timeout=25)
        f = r.stdout.split()
        return server["label"], (f if r.returncode == 0 and len(f) == 7 else None)
    except subprocess.TimeoutExpired:
        return server["label"], None


def report_servers(servers: list[dict]) -> None:
    section(f"服务器资源（测试优先放 dev；/data <{REMOTE_DISK_MIN_G}G 需清理）")
    by_label = {s["label"]: s for s in servers}
    with ThreadPoolExecutor(len(servers) or 1) as ex:
        res = dict(ex.map(lambda s: probe(s, by_label), servers))
    for s in servers:
        f = res.get(s["label"])
        if f is None:
            print(f"  {s['label']}: 不可达")
            continue
        ncpu, load, avail, total, disk, dirs, procs = f
        warn = []
        if disk.isdigit() and int(disk) < REMOTE_DISK_MIN_G:
            warn.append("磁盘不足")
        if s["label"] == "dev" and float(load) < int(ncpu) / 2:
            warn.append("有余量，新测试放这里")
        print(f"  {s['label']}: 核 {ncpu} 负载 {load} 可用内存 {avail}G/{total}G /data 空闲 {disk}G "
              f"作业目录 {dirs} 在跑进程 {procs}" + (f"  ← {'，'.join(warn)}" if warn else ""))


# ── 子代理进展 ────────────────────────────────────────────────────────────────

def agent_info(path: str) -> dict | None:
    real = os.path.realpath(path)
    try:
        mtime = os.path.getmtime(real)
    except OSError:
        return None
    if time.time() - mtime > AGENT_RECENT_HOURS * 3600:
        return None
    start, last, wts = None, "", {}
    with open(real, encoding="utf-8", errors="ignore") as fh:
        for line in fh:
            try:
                o = json.loads(line)
            except ValueError:
                continue
            content = (o.get("message") or {}).get("content")
            if start is None and o.get("timestamp"):
                start = o["timestamp"]
            if isinstance(content, list):
                for b in content:
                    if isinstance(b, dict) and b.get("type") == "tool_use":
                        inp = b.get("input") or {}
                        last = inp.get("description") or b.get("name", "")
                        for w in re.findall(r"workspace/(rava_\w+)", json.dumps(inp, ensure_ascii=False)):
                            wts[w] = wts.get(w, 0) + 1
    t0 = datetime.fromisoformat(start.replace("Z", "+00:00")).timestamp() if start else mtime
    return {"id": Path(path).stem, "hours": (time.time() - t0) / 3600,
            "idle": (time.time() - mtime) / 60,
            "desc": max(wts, key=wts.get) if wts else "rava（主检出）", "last": last[:60]}


def report_agents() -> None:
    section(f"子代理进展（最近 {AGENT_RECENT_HOURS}h 有活动；>{AGENT_MAX_HOURS}h 收尾续作，"
            f">{AGENT_IDLE_MINUTES}m 无活动需关注）")
    infos = [i for p in glob.glob(AGENT_TASKS_GLOB + "/a*.output") if (i := agent_info(p))]
    for i in sorted(infos, key=lambda i: i["idle"]):
        warn = []
        if i["hours"] > AGENT_MAX_HOURS:
            warn.append("超时长")
        if i["idle"] > AGENT_IDLE_MINUTES:
            warn.append("无活动")
        print(f"  {i['id'][:9]}  运行 {i['hours']:4.1f}h  上次活动 {i['idle']:3.0f}m 前  {i['desc']}"
              f"  | {i['last']}" + (f"  ← {'，'.join(warn)}" if warn else ""))
    if not infos:
        print("  无")


# ── 本机进程与资源 ────────────────────────────────────────────────────────────

def report_local(disp: list[dict]) -> None:
    section("本机项目进程")
    counts = {
        "分发器": len(disp),
        "heavy_lock": len(sh("pgrep -f '[h]eavy_lock.py'").split()),
        "cargo": len(sh("pgrep -x cargo").split()),
        "rustc": len(sh("pgrep -x rustc").split()),
        "rava": len(sh("pgrep -f 'release/[r]ava '").split()),
    }
    print("  " + "  ".join(f"{k} {v}" for k, v in counts.items()))
    heavy = []
    for line in sh("ps -axo pid=,ppid=,rss=,etime=,comm=").splitlines():
        f = line.split(None, 4)
        if len(f) == 5 and int(f[2]) > 300 * 1024:
            heavy.append((int(f[2]) // 1024, f))
    for mb, f in sorted(heavy, reverse=True)[:8]:
        print(f"  重进程 {f[0]} ppid={f[1]} {mb}MB {f[3]} {Path(f[4]).name}")
    section(f"本机资源（磁盘 <{LOCAL_DISK_MIN_G}G 需清理）")
    df = sh("df -g / | tail -1").split()
    free_g = int(df[3]) if len(df) > 3 and df[3].isdigit() else -1
    print(f"  磁盘空闲 {free_g}G" + ("  ← 低于阈值" if 0 <= free_g < LOCAL_DISK_MIN_G else ""))
    print("  " + sh("memory_pressure | tail -1").strip())
    print("  swap " + sh("sysctl -n vm.swapusage").strip())


# ── 分支 ──────────────────────────────────────────────────────────────────────

def git(*args: str) -> str:
    return sh("git " + " ".join(map(shlex.quote, args)), cwd=REPO)


def is_ancestor(a: str, b: str) -> bool:
    return subprocess.run(["git", "merge-base", "--is-ancestor", a, b], cwd=REPO).returncode == 0


def report_branches() -> None:
    section(f"分支（相对集成分支 {INTEGRATION}）")
    git("fetch", "-q", "--prune", "origin")
    worktrees, cur = {}, None
    for line in git("worktree", "list", "--porcelain").splitlines():
        if line.startswith("worktree "):
            cur = line[9:]
        elif line.startswith("branch refs/heads/"):
            worktrees[line[18:]] = cur
    local = git("for-each-ref", "--format=%(refname:short)", "refs/heads").split()
    batches = [b for b in local if b.startswith("batch-")]
    for b in local:
        if b in ("main", INTEGRATION):
            continue
        if is_ancestor(b, INTEGRATION):
            st = "已在集成分支"
        else:
            st = "未合入"
            for x in batches:
                if x != b and is_ancestor(b, x):
                    st = f"已在 {x}"
        wt = worktrees.get(b)
        dirty = ""
        if wt and any(not l.startswith("??") for l in sh("git status --porcelain", cwd=Path(wt)).splitlines()):
            dirty = " 有未提交改动"
        print(f"  {b} {git('log', '-1', '--format=%h %cr', b).strip()}  {st}"
              + (f"  wt={Path(wt).name}" if wt else "") + dirty)
    remote = {r[7:] for r in git("branch", "-r", "--format=%(refname:short)").split()
              if r.startswith("origin/") and r != "origin/HEAD"}
    stale = sorted(remote - set(local))
    print(f"  worktree 数 {len(worktrees)}，origin 分支数 {len(remote)}"
          + (f"；origin 有、本地无（遗留）：{' '.join(stale)}" if stale else ""))


def main() -> int:
    ap = argparse.ArgumentParser(description="协调巡检状态汇总（只读）")
    ap.add_argument("--no-remote", action="store_true", help="不探测远端服务器")
    args = ap.parse_args()
    servers = load_servers()
    disp = dispatchers()
    report_occupancy(disp, servers)
    report_results(disp)
    if not args.no_remote:
        report_servers(servers)
    report_agents()
    report_local(disp)
    report_branches()
    return 0


if __name__ == "__main__":
    sys.exit(main())
