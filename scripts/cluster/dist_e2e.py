"""e2e 单测执行：每个槽一次只跑一个测试（run_task，远端脱钩运行、断线续读），槽工作循环（server_worker）。

多槽（config slots）：一台服务器的各槽共用一个检出目录（初始化由 HostGate 每台物理机只做一次），
槽 i 的 run_tests 用 --out-dir build/slot<i>（slots=1 时仍为缺省 build/），scratch / 编译缓存 target /
日志 / 失败清单按槽隔离——rava compile 在共享 target 内会删「本例拥有」的产物、各测试的档案 crate
单元哈希又相同，两槽共用一个 target 会互删 rlib；按槽隔离后 target 在本槽内跨测试复用（编译缓存）。"""

import re
import shlex
import time
import threading
from datetime import datetime
from pathlib import Path

from cluster_config import (
    host_label, REMOTE_DIR, DEFAULT_JDK, MIN_DISK_GB, WAIT_SECONDS, TASK_TIMEOUT, RETRY_WAITS, MAX_RETRIES, MAX_INFRA_RETURNS,
)
import env_setup
import dist_ctx
from dist_ctx import (
    _shutdown, _log, _tls, _print_lock, _install_capture,
    _register_conn, _unregister_conn, _kill_remote_tests, _kill_cmd, abort_remote,
)
from dist_monitor import _set_st, _svr_lock, _svr_st
from dist_remote import (
    _check_resources, _resource_ok, _connect_with_retry, _fetch_git_hash, slot_cargo_config,
    spot_remote_dir, prepare_spot_checkout, prepare_main_checkout, remove_checkout,
    ensure_reference_jdk, refjdk_env, mem_limited, parse_oom_line, NO_MEMLIMIT_MARK,
)
from dist_state import TaskPool, State
from dist_job import server_lock_prefix, BUSY_MARK
import remote_run
from remote_run import make_run_key, runs_root
import failure_extract

# 从服务器拷回的单个日志上限（尾部）：rustc 全文可达数十 MB，本地只摘关键部分
MAX_REMOTE_LOG = 32 * 1024 * 1024
# 槽内编译缓存（<out>/jdk<N>/target）上限：每例结束后超过即整体删除重建，约束磁盘
TARGET_CAP_GB = 50


# ── 辅助：类名 → bin 名（与 run_tests.py._to_bin_name 保持一致）─────────────

def _to_bin_name(class_name: str) -> str:
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', class_name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    return s.lower()


# ── SSH 辅助 ──────────────────────────────────────────────────────────────────

_run_counter = 0
# run_task 解析出的每例耗时，(label, test) → {total_s, transpile_s, build_s, run_s}；_settle 取走后记入 state
_timings: dict[tuple[str, str], dict] = {}
_DUR = r"((?:\d+m)?[\d.]+s)"
_ELAPSED_RE = re.compile(rf"^Elapsed:\s+{_DUR}\s+\(transpile {_DUR}, build {_DUR}, run {_DUR}\)")


def _dur_secs(s: str) -> float:
    """run_tests.fmt_dur 的逆：'0.59s' / '13m59.7s' → 秒。"""
    m, _, sec = s[:-1].rpartition("m")
    return (int(m) * 60 if m else 0) + float(sec)


def parse_elapsed(line: str) -> dict | None:
    """解析 run_tests.py 结尾的 `Elapsed: 13m59.7s  (transpile 6m51.4s, build 7m07.2s, run 0.00s)`。"""
    m = _ELAPSED_RE.match(line.strip())
    if not m:
        return None
    return {k: round(_dur_secs(v), 2)
            for k, v in zip(("total_s", "transpile_s", "build_s", "run_s"), m.groups())}
_run_lock    = threading.Lock()


def _next_run_id() -> int:
    global _run_counter
    with _run_lock:
        _run_counter += 1
        return _run_counter


# ── 单次任务执行（每次只跑一个测试，远端脱钩运行）────────────────────────────

def slot_out(server: dict) -> str:
    """槽的 run_tests 输出根（相对检出目录）：单槽为缺省 build，多槽为 build/slot<i>。"""
    return "build" if server.get("slots", 1) == 1 else f"build/slot{server.get('slot', 0)}"


def _test_cmd(server: dict, test: str, jdk: int, remote_dir: str) -> str:
    # 扩展 PATH：兼容 rustup 工具链 bin（含非标准安装，如 macOS 无 ~/.cargo/bin/cargo 代理的情况）
    # CARGO_INCREMENTAL=0：分布式测试每次全新目录，增量无缓存可用，关掉节省内存和磁盘
    # CARGO_PROFILE_DEV_DEBUG=line-tables-only：保留行号（backtrace 可读）但内存仅约默认的 1/3
    # 槽锁：同一个槽上全量 / 抽查 / 作业各实例同一时刻只跑一个重任务
    # 语料 JDK：缺省主版本不传 --jdk，run_tests 用参考构建（RAVA_REFJDK_ROOT 下）；其他主版本为实验覆盖
    # 多槽：--out-dir 按槽隔离 scratch / target / 日志；槽输出根下写 cargo 配置限并行度（slot_cargo_config）
    jdk_flag = "" if jdk == DEFAULT_JDK else f"--jdk {jdk} "
    out = slot_out(server)
    out_flag = "" if out == "build" else f"--out-dir {out} "
    return (
        server_lock_prefix(server) +
        "RUSTUP_BIN=$(ls -d ~/.rustup/toolchains/stable*/bin 2>/dev/null | head -1); "
        "export PATH=$HOME/.local/bin:$HOME/.cargo/bin:${RUSTUP_BIN}:$PATH; "
        "export CARGO_INCREMENTAL=0; "
        "export CARGO_PROFILE_DEV_DEBUG=line-tables-only; "
        # 输出写文件：关掉 Python 块缓冲，本地轮询才能及时读到增量
        "export PYTHONUNBUFFERED=1; "
        + refjdk_env(server)
        + (slot_cargo_config(server, f"{remote_dir}/{out}") if out_flag else "")
        + mem_limited(
            f"cd {remote_dir} && "
            f"uv run python3 scripts/run_tests.py "
            f"{jdk_flag}-j 1 {out_flag}"
            + (f"{dist_ctx.run_tests_args} " if dist_ctx.run_tests_args else "")
            # run_tests 的 --filter 是全路径子串匹配：带前导 / 才只命中本测试（Bell.java 会连带 LynchBell.java）
            + f"--filter /{test}.java 2>&1",
            server,
        )
        + " 2>&1"
    )


def _logs_dir(remote_dir: str, jdk: int, out: str = "build") -> str:
    return f"{_jdk_root(remote_dir, jdk, out)}/logs"


def _jdk_root(remote_dir: str, jdk: int, out: str = "build") -> str:
    """槽输出根下的 JDK 版本层（run_tests 的 _versioned(OUT)）：scratch / target / logs 都在其下。"""
    return f"{remote_dir}/{out}/jdk{jdk}"


def _scratch_rm(remote_dir: str, jdk: int, out: str, test: str, whole: bool) -> str:
    """收尾清理片段，只动本槽输出根下的东西。whole=False（正常结束）：删本测试自己的 scratch、
    expected-classes 与日志，保留本槽 target 作编译缓存（超过 TARGET_CAP_GB 才整体删）；
    whole=True（远端已死等异常）：删本槽整个 jdk 根，不留半截产物。"""
    root = shlex.quote(_jdk_root(remote_dir, jdk, out))
    if whole:
        return f"rm -rf {root}/;"
    b = _to_bin_name(test)
    return (
        f"r={root}; rm -rf \"$r/{b}\" \"$r/expected-classes/{b}\" \"$r/logs/{b}.build.log\" "
        f"\"$r/logs/{b}.run.log\" \"$r/logs/dyn/{b}.json\"; "
        f"t=\"$r/target\"; if [ -d \"$t\" ] && [ \"$(du -sm \"$t\" 2>/dev/null | cut -f1)\" -gt {TARGET_CAP_GB * 1024} ]; "
        f"then rm -rf \"$t\"; fi;"
    )


def new_lease(test: str, jdk: int, remote_dir: str, commit: str, tag: str) -> dict:
    """派发租约：run_key 由 测试名 + jdk + 提交 + 实例标签 决定，结果目录在检出目录内。"""
    run_key = make_run_key(test, f"jdk{jdk}", (commit or "unknown")[:12], tag)
    return {"test": test, "jdk": jdk, "run_key": run_key, "work_dir": remote_dir,
            "run_dir": f"{runs_root(remote_dir)}/{run_key}", "commit": commit, "start": time.time(),
            "task_timeout": task_timeout()}


def task_timeout(lease: dict | None = None) -> int:
    """单例总时限（秒）：租约里记下的值优先（接上旧运行时沿用派发时的设置），其次命令行 --task-timeout，
    缺省 config.TASK_TIMEOUT。"""
    if lease and lease.get("task_timeout"):
        return int(lease["task_timeout"])
    return dist_ctx.task_timeout or TASK_TIMEOUT


def _collect_post(remote_dir: str, jdk: int, bin_name: str, out: str = "build") -> str:
    """runner 收尾片段：把失败现场拷进结果目录（$d），文件名统一为 <bin>.build.log / <bin>.run.log /
    <bin>.build_status.json。rustc 全文的位置随 run_tests 版本变化：
    新版在 scratch 下 build/jdk<N>/<bin>/logs/build.log，旧版在 build/jdk<N>/logs/<bin>.build.log；
    另按 run_tests 输出里「全文 → 路径」「backtrace 全量 → 路径」兜底。单个文件超过 MAX_REMOTE_LOG 字节时保留首尾各半。"""
    logs = shlex.quote(_logs_dir(remote_dir, jdk, out))
    scratch = shlex.quote(f"{_jdk_root(remote_dir, jdk, out)}/{bin_name}")
    cap = MAX_REMOTE_LOG
    return (
        f'cpl() {{ [ -f "$1" ] && [ ! -f "$2" ] || return 0; '
        f'if [ "$(wc -c < "$1")" -gt {cap} ]; then '
        f'{{ head -c {cap // 2} "$1"; printf "\\n... （服务器端截断：中间省略）...\\n"; tail -c {cap // 2} "$1"; }} > "$2"; '
        f'else cp -f "$1" "$2"; fi; }}; '
        f'cpl {scratch}/logs/build.log "$d/{bin_name}.build.log"; '
        f'cpl {logs}/{bin_name}.build.log "$d/{bin_name}.build.log"; '
        f'cpl {logs}/{bin_name}.run.log "$d/{bin_name}.run.log"; '
        f'cpl {scratch}/build_status.json "$d/{bin_name}.build_status.json"; '
        # 兜底：run_tests 输出里点名的全文路径（只取本检出目录下的文件）
        f'p=$(grep -o "全文 → [^）]*" "$d/out.log" | head -1 | sed "s/^全文 → //"); '
        f'case "$p" in {shlex.quote(remote_dir)}/*) cpl "$p" "$d/{bin_name}.build.log";; esac; '
        f'p=$(grep -o "backtrace 全量 → [^）]*" "$d/out.log" | head -1 | sed "s/^backtrace 全量 → //"); '
        f'case "$p" in {shlex.quote(remote_dir)}/*) cpl "$p" "$d/{bin_name}.run.log";; esac; true'
    )


def _write_error_log(client, label: str, lease: dict, log_path: Path, oom: str | None, note: str):
    """失败时生成单一日志 {test}_{label}_jdk{N}.log：元信息头 + 运行输出 + 从结果目录拷回的
    build.log 全部 error 块 / run.log panic 与回溯头部 / build_status.json（failure_extract，单例 ≤200KB）。"""
    test, jdk = lease["test"], lease["jdk"]
    error_log_dir = dist_ctx.results_dir / "error_logs"
    error_log_dir.mkdir(parents=True, exist_ok=True)
    log_dest = error_log_dir / f"{test}_{label}_jdk{jdk}.log"
    ts_str = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    bin_name = _to_bin_name(test)
    header = f"server: {label}  jdk: {jdk}  git: {(lease.get('commit') or '?')[:8]}  time: {ts_str}\n"
    if oom:
        header += f"OOM: 超出内存上限被内核杀掉（{oom}）\n"
    if note:
        header += f"{note}\n"
    header += "\n"

    def _remote(suffix: str) -> str | None:
        data = remote_run.read_file(client, f"{lease['run_dir']}/{bin_name}.{suffix}")
        return data.decode(errors="replace") if data else None

    text = failure_extract.compose(
        header, log_path.read_text(errors="replace"),
        _remote("build.log"), _remote("run.log"), _remote("build_status.json"),
    )
    log_dest.write_text(text, encoding="utf-8")
    with _print_lock:
        with open(dist_ctx.results_dir / f"failed_tests_jdk{jdk}.txt", "a", encoding="utf-8") as fh:
            entry = (f"{test}\t[{label}]\t[{datetime.now().strftime('%H:%M:%S')}]"
                     + (f"\t[OOM {oom}]" if oom else "") + f"\t{log_dest}")
            fh.write(entry + "\n")


def run_task(client, server: dict, lease: dict, log_dir: Path) -> tuple[str | None, str | None, object]:
    """
    按租约在远端结果目录上运行（或接上 / 直接收取）单个测试，流式解析 PASS/FAIL。
    返回 (结果, 附加信息, 结束时的连接)。结果为 "pass" / "fail" / "noresult"（输出里既无 PASS 也无 FAIL 行）/
    "oom"（超出内存上限被内核杀掉，附加信息形如 'limit=…M peak=…M'）/ "timeout"（超过单例总时限 task_timeout，远端已结束）/
    "busy"（服务器锁被其他实例占用，调用方归还任务稍后再试）/ "nomemlimit"（建不了内存受限 scope）/
    "aborted"（Ctrl+C，本地停止轮询）/ None=远端确认已死或长时间失联（附加信息为原因，调用方计一次异常归还）。
    不带 --record-passed：通过与否只看本次运行，远端不留跨提交的通过清单。
    """
    label      = server["label"]
    test, jdk  = lease["test"], lease["jdk"]
    remote_dir = lease["work_dir"]
    run_dir    = lease["run_dir"]
    log_path   = log_dir / f"run_{_next_run_id():04d}_{label}.log"
    log_file   = open(log_path, "w", encoding="utf-8")
    _tls.buf   = log_file
    flags = {"pass": False, "fail": False, "busy": False, "nomem": None, "oom": None}

    def on_line(line: str):
        if line.strip() == BUSY_MARK:
            flags["busy"] = True
            return
        if line.startswith(NO_MEMLIMIT_MARK):
            flags["nomem"] = line.strip()
            return
        log_file.write(line + "\n")
        log_file.flush()
        flags["oom"] = parse_oom_line(line) or flags["oom"]
        if (tm := parse_elapsed(line)) is not None:
            _timings[(label, test)] = tm
        # 只认本测试自己的结果行
        m = re.search(r"\[\s*(PASS|FAIL)\s*\]\s+\S+/(\S+)\.java", line.rstrip())
        if m and m.group(2) == test:
            if m.group(1) == "PASS":
                flags["pass"] = True
                _set_st(label, current=f"✅ {test}")
            else:
                flags["fail"] = True
                _set_st(label, current=f"❌ {test}")

    # 失败日志在 runner 内复制进结果目录：本地断线期间结束的运行也能收到
    so = slot_out(server)
    post = _collect_post(remote_dir, jdk, _to_bin_name(test), so)

    def on_client(c):
        _register_conn(label, c, remote_dir, run_dir, test)

    try:
        _set_st(label, status="running", current=test, task_start=lease["start"])
        _register_conn(label, client, remote_dir, run_dir, test)
        try:
            out = remote_run.run_detached(
                server, client, run_dir, _test_cmd(server, test, jdk, remote_dir),
                on_line=on_line, post=post, timeout_at=lease["start"] + task_timeout(lease),
                kill_extra=_kill_cmd(remote_dir, "KILL", test), on_client=on_client,
            )
        finally:
            _unregister_conn(label)
        client = out.client
        if out.attached:
            _log(f"  [{label}] ↪ {test} 结果目录已存在，接上（{run_dir}）")
        if out.status == "aborted":
            if client is not None:
                abort_remote(client, remote_dir, run_dir, test)
            return "aborted", None, client
        if out.status == "lost":
            return None, "长时间失联，远端运行状态未知", client
        if out.status == "dead":
            remote_run.remove_run(client, run_dir, _scratch_rm(remote_dir, jdk, so, test, whole=True))
            return None, "远端进程已死且无 done", client
        if flags["busy"]:
            remote_run.remove_run(client, run_dir)
            return "busy", None, client
        if flags["nomem"]:
            remote_run.remove_run(client, run_dir)
            _log(f"  [{label}] ❌ {flags['nomem']}")
            return "nomemlimit", None, client

        # ── 收集阶段（清理前）────────────────────────────────────────────────
        _set_st(label, status="collecting")
        oom = flags["oom"]
        if out.status == "timeout":
            result, info = "timeout", f"超过 {task_timeout(lease)}s 未结束，已结束远端进程"
        elif oom:
            result, info = "oom", oom
        else:
            result = "pass" if flags["pass"] and not flags["fail"] else ("fail" if flags["fail"] else "noresult")
            info = None
        if result != "pass":
            _write_error_log(client, label, lease, log_path, oom,
                             info if result == "timeout" else "")

        # ── 删除结果目录并清理本测试 scratch（保留本槽 target 编译缓存）────────
        _set_st(label, status="cleaning")
        remote_run.remove_run(client, run_dir, _scratch_rm(remote_dir, jdk, so, test, whole=False))
        if out.reconnects == 0:
            remote_run.note_clean_run(host_label(server))
        _log(f"  [{label}]  {result.upper()}  {test}" + (f"  ({info})" if info else "")
             + (f"  [断线重连 {out.reconnects} 次，续读未重派]" if out.reconnects else ""))
        return result, info, client

    except Exception as e:
        if _shutdown.is_set():
            return "aborted", None, client
        _log(f"  [{label}] ❌ 异常: {e}")
        return None, f"本地异常: {e}", client
    finally:
        del _tls.buf
        log_file.close()
        # 内容已在 error_logs/ 归档，或为空 / 通过
        log_path.unlink(missing_ok=True)


def _settle(server: dict, pool: TaskPool, state: State, lease: dict, ok: str | None,
            info: str | None) -> str:
    """记录 run_task 结果、维护租约与任务池。返回 'stop'（停用该服务器）/ 'wait'（等待后再试）/
    'abort'（关闭中）/ 'next'。"""
    label, test, commit = server["label"], lease["test"], lease.get("commit") or "?"
    timing = _timings.pop((label, test), None)
    if ok == "aborted":
        return "abort"              # 保留租约：Ctrl+C 已结束远端运行，下次启动在原结果目录重新发起
    state.clear_lease(label)
    if ok == "busy":
        pool.push_back([test])
        _log(f"[{label}] ⏳ 服务器上已有其他实例的测试 / 作业在跑，等待 {WAIT_SECONDS}s...")
        _set_st(label, status="waiting", current="服务器锁占用")
        return "wait"
    if ok == "nomemlimit":
        pool.push_back([test])
        _log(f"[{label}] ❌ 无法建立内存受限 scope，停止使用该服务器（env_setup --check-only 查看 memlimit 列）")
        _set_st(label, status="error", current="无内存上限")
        return "stop"
    if ok is None:
        if state.note_infra_return(test, label, info or "?", MAX_INFRA_RETURNS):
            _log(f"[{label}] 🚫 {test} 异常归还超过 {MAX_INFRA_RETURNS} 次，记为 infra 失败（{info}）")
        else:
            pool.push_back([test])
            _log(f"[{label}] ⚠️  {info}，{test} 已归还任务池")
        return "next"
    if ok == "pass":
        state.mark_passed([test], label, commit, timing)
        with _svr_lock:
            _svr_st[label]["passed"] = _svr_st[label].get("passed", 0) + 1
    else:
        reason = {
            "fail": "test failed",
            "noresult": "no result（输出无 PASS/FAIL 行）",
            "oom": f"OOM（{label} 内存上限内被杀：{info}）",
            "timeout": f"timeout（{info}）",
        }[ok]
        state.mark_failed([test], label, reason, commit, timing)
        with _svr_lock:
            _svr_st[label]["failed"] = _svr_st[label].get("failed", 0) + 1
    _set_st(label, status="idle", current="")
    return "next"


# ── 服务器工作循环 ─────────────────────────────────────────────────────────────

def _alive(client) -> bool:
    t = client.get_transport() if client is not None else None
    return t is not None and t.is_active()


def _wait_cooldown(server: dict, pool: TaskPool) -> bool:
    """熔断冷却中（按物理机计）：不领任务，分段等待（响应 Ctrl+C / 任务池清空）。返回是否仍在冷却。"""
    left = remote_run.cooldown_left(host_label(server))
    if left <= 0:
        return False
    _set_st(server["label"], status="cooldown", current=f"剩 {int(left)}s")
    _shutdown.wait(timeout=min(left, WAIT_SECONDS))
    return True


# ── 物理机级初始化：同机各槽会合，只做一次 ──────────────────────────────────────

class HostGate:
    """同一台物理机各槽的会合点。各槽先各自接上自己的租约，再到达会合点；最后到达者执行一次
    物理机级初始化（setup → 清理残留 → 检出 → 参考 JDK），其余槽等它的结果。槽退出时 leave()，
    最后离开者负责物理机级收尾（删抽查检出）。任何槽无论成败都必须到达一次、离开一次。"""

    def __init__(self, n: int):
        self.n = n
        self._cond = threading.Condition()
        self._arrived = 0
        self._left = 0
        self._ready = False
        self._result = None

    def init_once(self, fn):
        with self._cond:
            self._arrived += 1
            leader = self._arrived == self.n
        if leader:
            try:
                res = fn()
            except Exception as e:
                _log(f"  物理机初始化异常: {e}")
                res = None
            with self._cond:
                self._result, self._ready = res, True
                self._cond.notify_all()
            return res
        with self._cond:
            while not self._ready:
                if _shutdown.is_set():
                    return None
                self._cond.wait(timeout=1.0)
            return self._result

    def leave(self) -> bool:
        with self._cond:
            self._left += 1
            return self._left == self.n


_gates_lock = threading.Lock()
_gates: dict[tuple[str, str], HostGate] = {}


def _host_gate(server: dict, instance_tag: str) -> HostGate:
    key = (host_label(server), instance_tag)
    with _gates_lock:
        if key not in _gates:
            _gates[key] = HostGate(server.get("slots", 1))
        return _gates[key]


def _host_labels(server: dict) -> list[str]:
    """同机全部槽的标签（状态表上物理机级步骤同步显示到每个槽）。"""
    n, h = server.get("slots", 1), host_label(server)
    return [h] if n == 1 else [f"{h}#{i}" for i in range(n)]


def _host_init(server: dict, skip_setup: bool, spot_ref: str | None, spot_tag: str | None) -> dict | None:
    """物理机级初始化（每台物理机每个实例一次）：setup → 结束检出目录下无租约的残留运行与 run_tests →
    检出 → 参考 JDK → HEAD。返回 {"work_dir", "commit"}；失败返回 None（同机各槽一起停用）。
    调用时同机各槽都已接上并收完各自的租约，检出目录下不再有本实例在用的运行。"""
    host = host_label(server)
    labels = _host_labels(server)
    remote_dir = server.get("remote_dir", REMOTE_DIR)

    def st(**kw):
        for l in labels:
            _set_st(l, **kw)

    # 命令行 --skip-setup 或 server 配置 skip_setup:True（如 macOS 本机）均跳过初始化
    if not (skip_setup or server.get("skip_setup", False)):
        st(status="setup", current="连接中")
        c, _ = _connect_with_retry(server)
        if c is None:
            st(status="error", current="连接放弃")
            return None
        try:
            ok = env_setup.setup_with_client(c, server, on_status=lambda _l, text: st(status="setup", current=text))
        finally:
            c.close()
        if not ok:
            st(status="error", current="初始化失败")
            return None

    st(status="connecting", current="清理残留进程")
    c, _ = _connect_with_retry(server)
    if c is None:
        st(status="error", current="连接放弃")
        return None
    try:
        spot_dir = spot_remote_dir(remote_dir, spot_tag) if spot_ref else None
        work_dir = spot_dir or remote_dir
        # 本实例检出目录下的残留运行（无租约）与残留 run_tests 一并结束
        remote_run.cleanup_orphans(c, work_dir)
        _kill_remote_tests(c, host, work_dir)
        if spot_ref:
            st(status="connecting", current=f"检出 {spot_ref}")
            err = prepare_spot_checkout(c, remote_dir, spot_dir, spot_ref)
            if err:
                _log(f"[{host}] ❌ 抽查检出失败: {err}")
                st(status="error", current="检出失败")
                return None
        else:
            # 始终检出 main 最新提交（每个结果记录其运行时的 HEAD）
            st(status="connecting", current="检出 main 最新")
            err = prepare_main_checkout(c, remote_dir)
            if err:
                _log(f"[{host}] ⚠️ 检出 main 最新失败，沿用当前代码: {err}")
        # 语料参考 JDK：清单随检出的提交走，每批开头幂等确保（已就位为空操作，不受 --skip-setup 影响）
        st(status="connecting", current="确保参考 JDK")
        jhome, err = ensure_reference_jdk(c, server, work_dir)
        if err:
            _log(f"[{host}] ❌ 参考 JDK 未能就位，停止使用该服务器: {err}")
            st(status="error", current="参考 JDK 失败")
            return None
        _log(f"[{host}] 参考 JDK：{jhome or '该提交无清单，沿用 run_tests 旧选择'}")
        commit = _fetch_git_hash(c, work_dir, short=False)
        st(git_hash=commit[:8])
        _log(f"[{host}] 残留进程已清理，开始分发任务（{server.get('slots', 1)} 槽）")
        return {"work_dir": work_dir, "commit": commit}
    finally:
        c.close()


def _resume_lease(server: dict, pool: TaskPool, state: State, log_dir: Path) -> bool:
    """接上本槽上次本地中断时派发的运行并收完（检出目录此时不能动）。返回本槽是否继续工作。"""
    label = server["label"]
    lease = state.leases().get(label)
    if not lease:
        return True
    _set_st(label, status="connecting", current="接上租约")
    client, _ = _connect_with_retry(server)
    if client is None:
        _set_st(label, status="error", current="连接放弃")
        if not _shutdown.is_set():
            # 本槽用不了：持有租约的测试归还公共池（远端若仍在跑，由物理机初始化清理）
            state.clear_lease(label)
            pool.push_back([lease["test"]])
        return False
    try:
        _log(f"[{label}] ↪ 接上上次派发的 {lease['test']}（{lease['run_key']}）")
        _set_st(label, git_hash=(lease.get("commit") or "?")[:8])
        ok, info, client = run_task(client, server, lease, log_dir)
        return _settle(server, pool, state, lease, ok, info) not in ("abort", "stop")
    finally:
        if client is not None:
            client.close()


def server_worker(server: dict, pool: TaskPool, jdk: int, state: State,
                  log_dir: Path, skip_setup: bool = False, spot_ref: str | None = None,
                  spot_tag: str | None = None):
    """
    每个槽一个工作线程：接上本槽租约 → 同机各槽会合、物理机级初始化只做一次（HostGate）→
    循环执行测试 → 同机最后退出的槽做物理机级收尾。初始化完成即开始取任务，不等待其他服务器。
    """
    label          = server["label"]
    instance_tag   = spot_tag or "full"
    gate           = _host_gate(server, instance_tag)
    init           = None
    _install_capture()
    try:
        try:
            go = _resume_lease(server, pool, state, log_dir)
        except Exception as e:
            _log(f"[{label}] ❌ 接上租约异常: {e}")
            go = False
        init = gate.init_once(lambda: _host_init(server, skip_setup, spot_ref, spot_tag))
        if not go or init is None:
            return
        remote_dir, head_commit = init["work_dir"], init["commit"]
        _set_st(label, git_hash=head_commit[:8])
        _worker_loop(server, pool, jdk, state, log_dir, remote_dir, head_commit, instance_tag)
    finally:
        last = gate.leave()
        if last and init is not None and spot_ref and not _shutdown.is_set() and pool.remaining() == 0:
            # 抽查跑完即删独立检出目录（同机全部槽都已退出）；中断时保留以便续跑
            c, _ = _connect_with_retry(server)
            if c is not None:
                remove_checkout(c, init["work_dir"])
                c.close()


def _worker_loop(server: dict, pool: TaskPool, jdk: int, state: State, log_dir: Path,
                 remote_dir: str, head_commit: str, instance_tag: str):
    """主循环：每次取 1 个测试。"""
    label = server["label"]
    resource_fails = 0
    while not _shutdown.is_set() and pool.remaining() > 0:
        if _wait_cooldown(server, pool):
            continue
        # 先取任务，资源不足时归还后等待（避免空等时占着锁）
        tests = pool.pop(1)
        if not tests:
            break
        test = tests[0]

        # 连接
        _set_st(label, status="connecting", current=test)
        client, via = _connect_with_retry(server)
        if client is None:
            pool.push_back([test])
            _set_st(label, status="error", current="连接放弃")
            return

        try:
            # 资源检查
            _set_st(label, status="checking", current=test)
            res = _check_resources(client)

            if res is None:
                pool.push_back([test])
                resource_fails += 1
                wait = RETRY_WAITS[min(resource_fails - 1, len(RETRY_WAITS) - 1)]
                _log(f"[{label}] ⚠️  资源检查失败 ({resource_fails})，{wait}s 后重试...")
                if resource_fails > MAX_RETRIES:
                    _log(f"[{label}] ❌ 资源检查持续失败，放弃")
                    _set_st(label, status="error", current="")
                    return
                _set_st(label, status="checking", current="")
                _shutdown.wait(timeout=wait)  # 响应 Ctrl+C 立即退出
                continue
            resource_fails = 0

            free_ram_mb, load_per_cpu, disk_gb, _ = res
            _set_st(label, ram_mb=free_ram_mb, load=round(load_per_cpu, 2))

            if disk_gb < MIN_DISK_GB:
                pool.push_back([test])
                _log(f"[{label}] ⏭  磁盘剩余 {disk_gb}GB < {MIN_DISK_GB}GB，退出")
                _set_st(label, status="skip", current="")
                return

            if not _resource_ok(free_ram_mb, load_per_cpu, server):
                pool.push_back([test])
                _log(f"[{label}] ⏳ 资源紧张（CPU {load_per_cpu:.0%}  RAM {free_ram_mb}MB），等待 {WAIT_SECONDS}s...")
                _set_st(label, status="waiting", current="")
                _shutdown.wait(timeout=WAIT_SECONDS)
                continue

            # ── 执行测试：先落租约再发起，本地中途退出后可凭租约接上 ──────────
            _log(f"[{label}]{'（' + via + '）' if via else ''} → {test}  (RAM {free_ram_mb}MB  load {load_per_cpu:.0%})")
            lease = new_lease(test, jdk, remote_dir, head_commit, instance_tag)
            state.set_lease(label, **lease)
            ok, info, client = run_task(client, server, lease, log_dir)
            act = _settle(server, pool, state, lease, ok, info)
            if act == "abort":
                break
            if act == "stop":
                return
            if act == "wait":
                _shutdown.wait(timeout=WAIT_SECONDS)
                continue

        finally:
            if client is not None:
                client.close()

    if _shutdown.is_set():
        _set_st(label, status="idle", current="已停止")
        return
    _set_st(label, status="done", current="")
    _log(f"[{label}] 🏁 任务全部完成")
