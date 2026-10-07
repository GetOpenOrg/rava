"""远端脱钩执行：发起 → 轮询 → 断线续接 → 收结果。e2e 单测（dist_e2e）与作业（dist_job）共用。

远端每次运行一个结果目录 <检出目录>/build/dist_runs/<run_key>/：
  cmd.sh     要执行的命令（服务器锁前缀 + mem_limited 等，由调用方拼好）
  runner.sh  包装层：写 pid → bash cmd.sh > out.log → 执行 post 片段 → 原子写 rc → 建 done
  out.log    流式输出（stdout + stderr）
  pid        runner 进程号（setsid 后即进程组 / 会话号）
  rc         退出码（先写 rc.tmp 再 mv）
  done       结束标记（rc 写好之后才建）

runner 以 setsid nohup 启动，脱离 SSH 会话：断线不再连带杀掉远端测试。
runner 自身留在 SSH 登录会话的 cgroup（session-N.scope）里一直等到命令结束——logind 在会话 scope
仍有进程时把会话保持为 closing 而不回收，user systemd 实例（mem_limited 的 scope 挂在它下面）
因此不会在断线后随最后一个会话停掉；这样无需 loginctl enable-linger。

本地按字节偏移每 POLL_INTERVAL 秒读 out.log 增量并检查 done / rc；连接断开就重连，按同一 run_dir
从原偏移续读，不重新派发。远端确认已死（runner 不在且无 done）才由调用方归还任务。
"""

import os
import re
import shlex
import threading
import time
from dataclasses import dataclass, field

from cluster_config import POLL_INTERVAL, BREAKER_DISCONNECTS, BREAKER_COOLDOWN, host_label
from dist_ctx import _shutdown, _log

READ_CAP = 4 * 1024 * 1024   # 单次轮询最多读回的字节数（超出则立即再读）
RUNS_SUBDIR = "build/dist_runs"
# 调试开关：RAVA_DIST_INJECT_DROP=N[:label] 在每次运行的第 N 次轮询前强制关闭连接（只注入一次），
# 用来验证断线后重连续读、不重派。缺省关闭
INJECT_ENV = "RAVA_DIST_INJECT_DROP"


def runs_root(work_dir: str) -> str:
    return f"{work_dir.rstrip('/')}/{RUNS_SUBDIR}"


def make_run_key(*parts: str) -> str:
    """run_key：各部分（测试名 / jdk / 提交 / 实例标签 …）以 '-' 连接，非法字符替换为 '_'。"""
    return "-".join(re.sub(r"[^A-Za-z0-9_.]", "_", str(p)) for p in parts if p)


# ── 远端 shell 片段 ────────────────────────────────────────────────────────────
# 一律经 bash -c 执行：服务器登录 shell 可能是 zsh（通配 / 分词规则不同）

_SH_KILL_FN = (
    # kill_run <run_dir>：runner 仍是本目录的 runner 时，终止其进程组与会话内的全部进程
    'kill_run() { d=$1; p=$(cat "$d/pid" 2>/dev/null); [ -n "$p" ] || return 0; '
    'ps -ww -p "$p" -o args= 2>/dev/null | grep -qF "$d/runner.sh" || return 0; '
    'for sig in TERM KILL; do '
    'kill -$sig -- -"$p" 2>/dev/null; '
    'for q in $( { ps -eo pid=,pgid= 2>/dev/null; ps -eo pid=,sid= 2>/dev/null; } '
    '| awk -v s="$p" \'$2==s{print $1}\'); do kill -$sig "$q" 2>/dev/null; done; '
    '[ $sig = TERM ] && sleep 2; done; }; '
)


def _probe_script(run_dir: str, offset: int, cap: int) -> str:
    d = shlex.quote(run_dir)
    return (
        f"d={d}; s=absent; rc=-; pid=-; "
        'if [ -d "$d" ]; then '
        '  if [ -f "$d/done" ]; then s=done; rc=$(cat "$d/rc" 2>/dev/null || echo -); '
        '  else pid=$(cat "$d/pid" 2>/dev/null || echo -); '
        '    if [ "$pid" != - ] && ps -ww -p "$pid" -o args= 2>/dev/null | grep -qF "$d/runner.sh"; then s=running; '
        # runner 恰在两次检查之间结束：done 已建
        '    elif [ -f "$d/done" ]; then s=done; rc=$(cat "$d/rc" 2>/dev/null || echo -); '
        # 刚发起、runner 尚未写 pid（发起命令中途断线）：1 分钟内视为启动中
        '    elif [ "$pid" = - ] && [ -z "$(find "$d" -maxdepth 0 -mmin +1 2>/dev/null)" ]; then s=starting; '
        '    else s=dead; fi; '
        '  fi; '
        'fi; '
        'sz=0; [ -f "$d/out.log" ] && sz=$(wc -c < "$d/out.log" | tr -d " "); '
        'echo "RUNSTATE $s $rc $pid $sz"; '
        f'if [ "$sz" -gt {offset} ]; then tail -c +{offset + 1} "$d/out.log" | head -c {cap}; fi; true'
    )


def _launch_script(run_dir: str, cmd: str, post: str) -> str:
    d = shlex.quote(run_dir)
    runner = (
        f"d={d}\n"
        'echo $$ > "$d/pid.tmp" && mv -f "$d/pid.tmp" "$d/pid"\n'
        'bash "$d/cmd.sh" > "$d/out.log" 2>&1 < /dev/null\n'
        "rc=$?\n"
        + (f"( {post} ) > /dev/null 2>&1\n" if post else "")
        + 'echo $rc > "$d/rc.tmp" && mv -f "$d/rc.tmp" "$d/rc"\n'
        ': > "$d/done"\n'
    )
    return (
        f"d={d}; "
        'mkdir -p "$d" && cd "$d" && rm -f done rc rc.tmp pid pid.tmp out.log && '
        f"printf '%s\\n' {shlex.quote(cmd)} > cmd.sh && "
        f"printf '%s' {shlex.quote(runner)} > runner.sh || exit 1; "
        # setsid：新会话、无控制终端；nohup 兜底忽略 SIGHUP；标准流全部重定向，SSH 通道可立即关闭
        'if command -v setsid >/dev/null 2>&1; then '
        'setsid nohup bash "$d/runner.sh" < /dev/null > /dev/null 2>&1 & '
        'else set -m; nohup bash "$d/runner.sh" < /dev/null > /dev/null 2>&1 & fi; '
        'i=0; while [ ! -f "$d/pid" ] && [ $i -lt 100 ]; do sleep 0.1; i=$((i+1)); done; '
        'cat "$d/pid"'
    )


def kill_run_script(run_dir: str) -> str:
    return _SH_KILL_FN + f"kill_run {shlex.quote(run_dir)}; true"


def abort_runs_script(work_dir: str, keep: list[str] = ()) -> str:
    """结束并删除 work_dir 下全部运行（keep 中的结果目录除外）。"""
    root = shlex.quote(runs_root(work_dir))
    keep_s = shlex.quote(" " + " ".join(keep) + " ")
    return (
        _SH_KILL_FN
        + f"for d in {root}/*/; do d=${{d%/}}; [ -d \"$d\" ] || continue; "
        f"case {keep_s} in *\" $d \"*) continue;; esac; kill_run \"$d\"; rm -rf \"$d\"; done; true"
    )


# ── 执行原语 ──────────────────────────────────────────────────────────────────

def _exec(client, script: str, timeout: int = 30) -> tuple[int, bytes]:
    """bash -c 执行脚本，返回 (退出码, stdout 字节)。连接异常原样抛出（调用方据此重连）。"""
    stdin, stdout, _e = client.exec_command("bash -c " + shlex.quote(script), timeout=timeout)
    stdout.channel.settimeout(timeout)
    stdin.close()
    out = stdout.read()
    return stdout.channel.recv_exit_status(), out


def exec_nowait(client, script: str):
    """发出命令即返回，不等结果（Ctrl+C 处理用）。命令在远端脱离会话执行，随后关闭连接也不会打断它。"""
    detached = f"nohup bash -c {shlex.quote(script)} < /dev/null > /dev/null 2>&1 &"
    try:
        stdin, stdout, _e = client.exec_command("bash -c " + shlex.quote(detached))
        stdin.close()
        stdout.channel.close()
    except Exception:
        pass


@dataclass
class Probe:
    state: str          # absent / starting / running / done / dead
    rc: int | None
    pid: str | None
    size: int
    data: bytes


def probe(client, run_dir: str, offset: int = 0, cap: int = READ_CAP) -> Probe:
    _, out = _exec(client, _probe_script(run_dir, offset, cap), timeout=60)
    head, _, data = out.partition(b"\n")
    parts = head.decode(errors="replace").split()
    if len(parts) != 5 or parts[0] != "RUNSTATE":
        raise RuntimeError(f"结果目录探测输出异常: {head[:200]!r}")
    _, st, rc, pid, sz = parts
    return Probe(st, int(rc) if re.fullmatch(r"-?\d+", rc) else None,
                 None if pid == "-" else pid, int(sz), data)


def launch(client, run_dir: str, cmd: str, post: str = "") -> str:
    """在 run_dir 发起 cmd（脱离 SSH 会话），返回 runner pid。"""
    rc, out = _exec(client, _launch_script(run_dir, cmd, post), timeout=60)
    pid = out.decode(errors="replace").strip()
    if rc != 0 or not pid.isdigit():
        raise RuntimeError(f"发起远端运行失败 rc={rc}: {pid[-200:]}")
    return pid


def read_file(client, path: str) -> bytes | None:
    try:
        rc, out = _exec(client, f"cat {shlex.quote(path)}", timeout=120)
        return out if rc == 0 else None
    except Exception:
        return None


def remove_run(client, run_dir: str, extra: str = ""):
    """收回结果后删除结果目录（extra：同一条命令里顺带执行的清理片段）。"""
    try:
        _exec(client, f"rm -rf {shlex.quote(run_dir)}; {extra} true", timeout=120)
    except Exception:
        pass


def cleanup_orphans(client, work_dir: str, keep: list[str] = ()):
    """结束并删除 work_dir 下不在 keep 中的残留运行（上次实例中断留下的）。"""
    try:
        _exec(client, abort_runs_script(work_dir, keep), timeout=120)
    except Exception:
        pass


# ── 熔断：服务器连续断线后冷却（按物理机标签计，同机各槽共享） ─────────────────

_health_lock = threading.Lock()
_disconnects: dict[str, int] = {}
_cool_until: dict[str, float] = {}


def note_disconnect(label: str):
    with _health_lock:
        n = _disconnects.get(label, 0) + 1
        _disconnects[label] = n
        if n >= BREAKER_DISCONNECTS:
            _cool_until[label] = time.time() + BREAKER_COOLDOWN
            _disconnects[label] = 0
            _log(f"[{label}] 🧯 连续断线 {n} 次，冷却 {BREAKER_COOLDOWN // 60} 分钟后再领任务")


def note_clean_run(label: str):
    """一次运行全程未断线：连续断线计数归零。"""
    with _health_lock:
        _disconnects[label] = 0


def cooldown_left(label: str) -> float:
    with _health_lock:
        return max(0.0, _cool_until.get(label, 0.0) - time.time())


# ── 跟随运行至结束 ─────────────────────────────────────────────────────────────

@dataclass
class Outcome:
    status: str                 # done / timeout / dead / lost / aborted
    rc: int | None = None
    client: object = None       # 结束时可用的连接（可能是重连后的新连接；lost / aborted 时可能为 None）
    attached: bool = False      # 发起前结果目录已存在（接上已有运行或直接收结果）
    reconnects: int = 0
    offset: int = 0
    notes: list[str] = field(default_factory=list)


def _inject_at(label: str) -> int:
    v = os.environ.get(INJECT_ENV, "")
    if not v:
        return 0
    n, _, only = v.partition(":")
    return int(n) if n.isdigit() and (not only or only == label) else 0


def run_detached(server: dict, client, run_dir: str, cmd: str, *, on_line, timeout_at: float,
                 post: str = "", kill_extra: str = "", on_client=None, connect=None) -> Outcome:
    """
    确保 run_dir 上的运行存在并跟随到结束：
      已 done → 直接读全部输出；进程仍活 → 接上；不存在 → 发起；已死且无 done → 返回 dead。
    on_line(str) 逐行接收输出（不含换行）；on_client(client) 在重连换连接后回调。
    过 timeout_at 仍在运行 → 结束远端进程（kill_extra 为附加的结束片段），返回 timeout。
    断线 → 重连续读；到 timeout_at 仍连不上 → lost（远端可能仍在运行，由下次启动清理）。
    """
    if connect is None:
        from dist_conn import _connect_with_retry as connect
    label = server["label"]
    out = Outcome("lost", client=client)
    buf = b""
    polls = 0
    inject = _inject_at(label)
    started = False

    def _emit(data: bytes, final: bool):
        nonlocal buf
        buf += data
        *lines, buf = buf.split(b"\n")
        if final and buf:
            lines.append(buf)
            buf = b""
        for ln in lines:
            on_line(ln.decode(errors="replace").rstrip("\r"))

    while True:
        if _shutdown.is_set():
            out.status = "aborted"
            return out
        try:
            if out.client is None:
                raise ConnectionError("无可用连接")
            polls += 1
            if inject and polls == inject:
                inject = 0
                _log(f"  [{label}] 🧪 注入断线（第 {polls} 次轮询前关闭 transport）")
                out.client.get_transport().close()
            p = probe(out.client, run_dir, out.offset)
            if not started:
                started = True
                if p.state == "absent":
                    launch(out.client, run_dir, cmd, post)
                    continue
                out.attached = True
                out.notes.append(f"结果目录已存在（{p.state}），接上")
            out.offset += len(p.data)
            if p.state in ("running", "starting") and time.time() > timeout_at:
                _exec(out.client, kill_run_script(run_dir) + "; " + kill_extra, timeout=60)
                p = probe(out.client, run_dir, out.offset)
                out.offset += len(p.data)
                _emit(p.data, final=True)
                out.status = "timeout"
                return out
            if p.state in ("done", "dead"):
                _emit(p.data, final=False)
                if len(p.data) >= READ_CAP:
                    continue                    # 还有未读完的输出
                _emit(b"", final=True)
                out.status, out.rc = p.state, p.rc
                return out
            _emit(p.data, final=False)
            if p.state == "absent":
                # 发起之后目录消失：被外部删除（如另一实例 / Ctrl+C 清理）
                out.status = "dead"
                return out
            if len(p.data) >= READ_CAP:
                continue
            _shutdown.wait(POLL_INTERVAL)
        except Exception as e:
            if _shutdown.is_set():
                out.status = "aborted"
                return out
            note_disconnect(host_label(server))
            out.reconnects += 1
            _log(f"  [{label}] ⚠️  轮询中断（{type(e).__name__}: {str(e)[:120]}），重连后从偏移 {out.offset} 续读")
            try:
                if out.client is not None:
                    out.client.close()
            except Exception:
                pass
            out.client = None
            while out.client is None:
                if _shutdown.is_set():
                    out.status = "aborted"
                    return out
                if time.time() > timeout_at:
                    out.status = "lost"
                    return out
                c, via = connect(server)
                if c is not None:
                    out.client = c
                    _log(f"  [{label}] 🔁 已重连{'（' + via + '）' if via else ''}，续读 {run_dir}")
                    if on_client:
                        on_client(c)
