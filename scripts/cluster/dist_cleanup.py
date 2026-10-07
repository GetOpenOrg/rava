"""启动清理：分发脚本每次启动时在各服务器上清理过期的 rava 生成物（后台线程，不阻塞派发）。

清理对象（只删「过期且无进程在用」的条目）：
1. <remote_dir 上级>/<名>-spot-* 检出目录（抽查 / 作业）：
   - 24 小时未改动：目录本身、build/ 前两层、build/dist_runs/*/out.log、.git/FETCH_HEAD / index 都无 24h 内的改动；
   - 不属于在跑任务：无任何进程的 cwd / 打开的文件落在目录内，且不是本地 summary.json 里 running 作业的检出。
2. /tmp 顶层条目（属于登录用户的普通文件 / 目录）：
   - 24 小时未改动（目录取其内最新改动）；
   - 无任何进程的 cwd / 打开的文件落在其中；
   - 跳过隐藏条目、锁文件与系统条目（systemd-private-*、snap-private-tmp、tmux-*、ssh-* 等）。

节流：服务器端 flock /tmp/rava_cleanup.lock 互斥，/tmp/rava_cleanup.stamp 1 小时内已清理则跳过
（多个分发实例同时启动只清一次）。依赖 Linux /proc 与 GNU find。

单独运行（先看清单再删）：
  uv run --group cluster python scripts/cluster/dist_cleanup.py --dry-run [--servers jp1 kr1]
  uv run --group cluster python scripts/cluster/dist_cleanup.py [--servers ...] [--force]   # --force 忽略 1 小时节流
"""

import json
import shlex
import sys
import threading
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from cluster_config import SERVERS, REMOTE_DIR, server_slots
import dist_ctx
from dist_ctx import _log
from dist_conn import _connect_with_retry
from dist_remote import job_remote_dir
from remote_run import _exec

STALE_MINUTES = 1440        # 24 小时未改动才清
THROTTLE_MINUTES = 60       # 同一服务器两次清理的最短间隔
TMP_SKIP = ("rava_dist.lock", "rava_cleanup.*", "systemd-private-*", "snap-private-tmp",
            "tmux-*", "ssh-*", "vscode*", "hsperfdata_*", "*.sock", "*.pid", "*.lock")


def _active_job_tags() -> set[str]:
    """本地 summary.json 中仍有 running 条目的作业 tag（其检出可能正等本地接上，不删）。"""
    tags = set()
    root = dist_ctx.results_dir / "job"
    if not root.is_dir():
        return tags
    for p in root.glob("*/summary.json"):
        try:
            data = json.loads(p.read_text(encoding="utf-8"))
        except Exception:
            continue
        if any(isinstance(v, dict) and v.get("status") == "running" for v in data.values()):
            tags.add(p.parent.name)
    return tags


def cleanup_script(remote_dir: str, keep_dirs: list[str], dry_run: bool, force: bool) -> str:
    """生成服务器端清理脚本；输出 CLEAN 前缀的行。"""
    rd = remote_dir.rstrip("/")
    base, name = rd.rsplit("/", 1)
    keep = " ".join(shlex.quote(d) for d in keep_dirs)
    skip = " ".join(f"-not -name {shlex.quote(p)}" for p in TMP_SKIP)
    act = 'echo "CLEAN would-rm $1"' if dry_run else 'rm -rf -- "$1" 2>/dev/null; echo "CLEAN rm $1"'
    throttle = "" if force or dry_run else (
        f'if [ -f "$stamp" ] && [ -z "$(find "$stamp" -mmin +{THROTTLE_MINUTES})" ]; then '
        f'echo "CLEAN skip 1 小时内已清理"; exit 0; fi; touch "$stamp"; ')
    return (
        "set -u; command -v flock >/dev/null 2>&1 || { echo 'CLEAN skip 无 flock'; exit 0; }; "
        "[ -d /proc/1 ] || { echo 'CLEAN skip 无 /proc'; exit 0; }; "
        "exec 8>/tmp/rava_cleanup.lock; flock -n 8 || { echo 'CLEAN skip 另一实例在清理'; exit 0; }; "
        "stamp=/tmp/rava_cleanup.stamp; " + throttle +
        f"base={shlex.quote(base)}; age={STALE_MINUTES}; "
        "avail() { df -Pk \"$1\" | awk 'NR==2{print int($4/1048576)}'; }; "
        "b0=$(avail \"$base\"); t0=$(avail /tmp); "
        # 在用路径：全部进程的 cwd 与打开的文件（非 root 只看得到自己的进程，也只删得动自己的文件）
        "inuse=$(mktemp); "
        "find /proc/[0-9]*/cwd /proc/[0-9]*/fd -maxdepth 1 -type l -printf '%l\\n' 2>/dev/null"
        " | sed 's/ (deleted)$//' | sort -u > \"$inuse\"; "
        "used() { awk -v c=\"$1\" 'index($0, c \"/\")==1 || $0==c {f=1; exit} END{exit !f}' \"$inuse\"; }; "
        "fresh() { [ -n \"$(find \"$@\" -maxdepth 0 -mmin -$age -print -quit 2>/dev/null)\" ]; }; "
        f"act() {{ {act}; }}; "
        f"keep=' {keep} '; n=0; "
        # 1) 过期检出目录
        f"for d in \"$base\"/{shlex.quote(name)}-spot-*; do [ -d \"$d\" ] && [ ! -L \"$d\" ] || continue; "
        "  case \"$keep\" in *\" $d \"*) echo \"CLEAN keep-lease $d\"; continue;; esac; "
        "  if used \"$d\"; then echo \"CLEAN keep-inuse $d\"; continue; fi; "
        "  if fresh \"$d\" \"$d\"/build \"$d\"/build/* \"$d\"/build/*/* \"$d\"/build/dist_runs/*/out.log"
        " \"$d\"/.git/FETCH_HEAD \"$d\"/.git/index; then continue; fi; "
        "  act \"$d\"; n=$((n+1)); done; "
        # 2) /tmp 顶层条目
        f"for e in $(find /tmp -mindepth 1 -maxdepth 1 -user \"$(id -u)\" \\( -type f -o -type d \\) "
        f"-not -name '.*' {skip} -mmin +$age -print 2>/dev/null); do "
        "  [ \"$e\" = \"$inuse\" ] && continue; "
        "  if used \"$e\"; then echo \"CLEAN keep-inuse $e\"; continue; fi; "
        "  if [ -d \"$e\" ] && [ -n \"$(find \"$e\" -mmin -$age -print -quit 2>/dev/null)\" ]; then continue; fi; "
        "  act \"$e\"; n=$((n+1)); done; "
        "rm -f \"$inuse\"; "
        "echo \"CLEAN done n=$n base_avail=${b0}G→$(avail \"$base\")G tmp_avail=${t0}G→$(avail /tmp)G\""
    )


def cleanup_server(server: dict, keep_tags: set[str], dry_run: bool = False, force: bool = False,
                   verbose: bool = False) -> str:
    label = server["label"]
    rd = server.get("remote_dir", REMOTE_DIR)
    # 作业检出按槽各一个（job_remote_dir）：每个在跑 tag 的全部槽检出都保留
    keep_dirs = [job_remote_dir(rd, t, i) for t in sorted(keep_tags) for i in range(server_slots(server))]
    c, _ = _connect_with_retry(server, max_rounds=1)
    if c is None:
        return f"[{label}] 启动清理：连接失败，跳过"
    try:
        rc, out = _exec(c, cleanup_script(rd, keep_dirs, dry_run, force), timeout=1800)
    except Exception as e:
        return f"[{label}] 启动清理异常: {e}"
    finally:
        c.close()
    lines = [l[6:] for l in out.decode(errors="replace").splitlines() if l.startswith("CLEAN ")]
    if verbose:
        return "\n".join(f"[{label}] {l}" for l in lines) or f"[{label}] 无输出 rc={rc}"
    summary = [l for l in lines if l.startswith(("done", "skip"))]
    return f"[{label}] 启动清理 " + ("; ".join(summary) or f"无输出 rc={rc}")


def start_background_cleanup(servers: list[str] | None = None):
    """分发脚本启动时调用：后台守护线程逐台清理，结果写日志，不阻塞派发。"""
    keep = _active_job_tags()
    picked = [s for s in SERVERS if not servers or s["label"] in servers]

    def _run():
        with ThreadPoolExecutor(max_workers=len(picked) or 1) as ex:
            for msg in ex.map(lambda s: cleanup_server(s, keep), picked):
                _log(msg)

    threading.Thread(target=_run, name="startup-cleanup", daemon=True).start()


if __name__ == "__main__":
    import argparse
    import logging
    logging.getLogger("paramiko").setLevel(logging.CRITICAL)
    ap = argparse.ArgumentParser(description="清理服务器上过期的 rava 检出目录与 /tmp 条目")
    ap.add_argument("--dry-run", action="store_true", help="只列出将删除的条目")
    ap.add_argument("--force", action="store_true", help="忽略 1 小时节流")
    ap.add_argument("--servers", nargs="+", metavar="LABEL")
    a = ap.parse_args()
    keep = _active_job_tags()
    picked = [s for s in SERVERS if not a.servers or s["label"] in a.servers]
    with ThreadPoolExecutor(max_workers=len(picked) or 1) as ex:
        for msg in ex.map(lambda s: cleanup_server(s, keep, a.dry_run, a.force, verbose=True), picked):
            print(msg, flush=True)
