"""作业模式（--job）：把 e2e 以外的重命令（rava audit、gen_trees / compare_trees、cargo test、
seed_check、lib_pilot_golden 等）分发到各服务器执行，不占本机资源。

- 每条 --cmd 是一个作业；各服务器的每个槽（config slots，缺省 1）空闲时取一个，一个槽同一时刻只跑一个作业
- 每个槽在独立检出目录 <remote_dir>-spot-job-<tag>（槽 i≥1 加 -s<i>）检出 --ref（跑完删检出目录），
  不动全量 / 抽查的检出目录，不清理其他实例 / 其他槽的进程
- 内存上限：作业在 user systemd scope 内运行，上限 = (总内存 − 预留（MEM_RESERVE_GB / mem_reserve_gb）) / 槽数，
  禁用 swap；超限只杀作业自身，summary.json 记 oom（limit / peak）；服务器建不了受限 scope 则不在其上运行
- 资源礼让：每个作业开跑前检查负载 / 内存 / 磁盘（内存门槛 --min-ram）；以 nice 低优先级运行，
  CARGO_BUILD_JOBS = 服务器核数的一半 / 槽数；与 e2e 共用服务器端槽锁（槽 0 为 /tmp/rava_dist.lock），
  同一个槽上作业与 e2e 测试不重叠（锁被占时归还作业、换时机重试）
- 输出流式写入 test_results/job/<tag>/<序号>_<服务器>.log；--fetch 声明的产物（相对检出目录的
  通配）取回到 test_results/job/<tag>/<序号>/；汇总写 summary.json（rc / 耗时 / 提交）
- 同一 tag 重跑时跳过已成功（rc=0）且命令相同的作业
- 远端脱钩执行（remote_run）：作业在 <检出目录>/build/dist_runs/<run_key>/ 以 setsid nohup 运行，
  断线重连后按偏移续读、不重派；summary.json 里 status=running 的条目即派发租约，本地重启后由原服务器接上；
  远端确认已死且无 done 才归还，异常归还超过 MAX_INFRA_RETURNS 次记 infra 失败
"""

import json
import shlex
import signal
import threading
import time
from collections import deque
from datetime import datetime
from pathlib import Path

from cluster_config import (
    SERVERS, REMOTE_DIR, pick_servers, slot_servers, slot_lock_path, host_label, MIN_DISK_GB, WAIT_SECONDS, RETRY_WAITS, MAX_RETRIES, MAX_INFRA_RETURNS,
)
import dist_ctx
from dist_ctx import _shutdown, _log
from dist_remote import (
    _check_resources, _resource_ok, _connect_with_retry, _fetch_git_hash, prepare_spot_checkout,
    job_remote_dir, remove_checkout, ensure_reference_jdk, refjdk_env, mem_limited, parse_oom_line, NO_MEMLIMIT_MARK,
)
import remote_run
from remote_run import make_run_key, runs_root

# 服务器端槽锁（config.slot_lock_path）：e2e 单测与作业都经此锁，一个槽同一时刻只跑一个重任务
BUSY_MARK = "__RAVA_SERVER_BUSY__"


def server_lock_prefix(server: dict) -> str:
    """shell 前缀：非阻塞取本槽的服务器锁（fd 9 由后续命令继承，命令结束即释放）；被占则打印标记退出。
    无 flock 的系统不加锁，仅靠资源检查礼让。"""
    return (
        f"if command -v flock >/dev/null 2>&1; then exec 9>{slot_lock_path(server.get('slot', 0))}; "
        f"flock -n 9 || {{ echo {BUSY_MARK}; exit 75; }}; fi; "
    )


_conns_lock = threading.Lock()
_conns: dict[str, tuple] = {}   # 槽标签 → (client, 本槽作业检出目录)


def _job_signal_handler(signum, frame):
    """Ctrl+C：本地停止轮询；远端只结束本实例作业目录下的运行并删除其结果目录（不触碰其他实例），
    summary.json 中的 running 条目保留，下次启动在原结果目录重新发起。"""
    signal.signal(signal.SIGINT, signal.SIG_DFL)
    signal.signal(signal.SIGTERM, signal.SIG_DFL)
    _log("\n正在停止作业，结束远端运行...")
    _shutdown.set()
    with _conns_lock:
        conns = list(_conns.values())
    for c, work_dir in conns:
        # 连接由 worker 关闭：worker 被 _shutdown 唤醒后用同一连接再同步结束一遍（幂等）
        remote_run.exec_nowait(c, remote_run.abort_runs_script(work_dir))


class JobBoard:
    """作业池 + summary.json 持久化。"""

    def __init__(self, cmds: list[str], out_dir: Path):
        self.out_dir = out_dir
        self.path = out_dir / "summary.json"
        self._lock = threading.Lock()
        old = json.loads(self.path.read_text(encoding="utf-8")) if self.path.exists() else {}
        self.data: dict[str, dict] = {}
        pending = []
        self._leases: dict[str, str] = {}   # 服务器 → 作业序号（上次本地中断时仍在远端运行）
        for i, cmd in enumerate(cmds, 1):
            key = f"{i:02d}"
            prev = old.get(key)
            if prev and prev.get("cmd") == cmd and prev.get("rc") == 0:
                self.data[key] = prev
                _log(f"[job {key}] 已成功（{prev.get('server')} @ {prev.get('commit', '?')[:9]}），跳过")
            elif (prev and prev.get("cmd") == cmd and prev.get("status") == "running"
                  and prev.get("run_dir") and prev.get("server") not in self._leases):
                self.data[key] = prev
                self._leases[prev["server"]] = key
                _log(f"[job {key}] 上次在 {prev['server']} 运行中，由该服务器接上（{prev['run_key']}）")
            else:
                self.data[key] = {"cmd": cmd, "status": "pending"}
                pending.append(key)
        self._deque = deque(pending)
        # 未完成（排队中 + 已被某台服务器取走 + 租约）的作业
        self._open = set(pending) | set(self._leases.values())
        self._save()

    # ── 检出登记：本地进程被杀后，下次同 tag 运行结束时仍能清掉各服务器上的作业检出 ──
    def _checkouts_path(self) -> Path:
        return self.out_dir / "checkouts.json"

    def checkouts(self) -> set[str]:
        with self._lock:
            p = self._checkouts_path()
            return set(json.loads(p.read_text(encoding="utf-8"))) if p.exists() else set()

    def mark_checkout(self, label: str, present: bool):
        with self._lock:
            p = self._checkouts_path()
            cur = set(json.loads(p.read_text(encoding="utf-8"))) if p.exists() else set()
            cur = (cur | {label}) if present else (cur - {label})
            p.write_text(json.dumps(sorted(cur)), encoding="utf-8")

    def take_lease(self, label: str) -> str | None:
        with self._lock:
            return self._leases.pop(label, None)

    def release_leases(self, keep_labels: set[str]):
        """服务器不在本次运行范围内的租约作废，作业回队列。"""
        with self._lock:
            for lbl in [l for l in self._leases if l not in keep_labels]:
                key = self._leases.pop(lbl)
                self.data[key]["status"] = "pending"
                self._deque.appendleft(key)
            self._save()

    def note_return(self, key: str) -> int:
        """记一次异常归还，返回累计次数。"""
        with self._lock:
            n = self.data[key].get("returns", 0) + 1
            self.data[key]["returns"] = n
            self._save()
            return n

    def held(self) -> bool:
        """作业目录下有 HOLD 文件时暂停派发排队项（已在跑 / 租约接上的不受影响），删文件即恢复。"""
        return (self.out_dir / "HOLD").exists()

    def pop(self) -> str | None:
        if self.held():
            return None
        with self._lock:
            return self._deque.popleft() if self._deque else None

    def push_back(self, key: str):
        with self._lock:
            self._deque.appendleft(key)

    def remaining(self) -> int:
        with self._lock:
            return len(self._deque)

    def unfinished(self) -> int:
        with self._lock:
            return len(self._open)

    def record(self, key: str, **kw):
        with self._lock:
            self.data[key].update(kw)
            if kw.get("status") == "done":
                self._open.discard(key)
            self._save()

    def _save(self):
        tmp = self.path.with_suffix(".tmp")
        tmp.write_text(json.dumps(self.data, indent=2, ensure_ascii=False), encoding="utf-8")
        tmp.replace(self.path)


def _fetch_artifacts(client, work_dir: str, globs: list[str], dest: Path) -> list[str]:
    """把检出目录下匹配 globs 的文件取回 dest（保持相对路径）。"""
    if not globs:
        return []
    # 在远端展开通配（bash globstar 支持 **），只列普通文件
    lister = (
        f"cd {work_dir} && bash -c "
        + shlex.quote("shopt -s globstar 2>/dev/null; shopt -s nullglob; "
                      f"for p in {' '.join(globs)}; do [ -f \"$p\" ] && echo \"$p\"; done")
    )
    got = []
    try:
        stdin, stdout, _e = client.exec_command(lister, timeout=60)
        stdin.close()
        files = [l.strip() for l in stdout.read().decode(errors="replace").splitlines() if l.strip()]
        if not files:
            return []
        sftp = client.open_sftp()
        try:
            for rel in files:
                local = dest / rel
                local.parent.mkdir(parents=True, exist_ok=True)
                sftp.get(f"{work_dir}/{rel}", str(local))
                got.append(rel)
        finally:
            sftp.close()
    except Exception as e:
        _log(f"  取回产物失败: {e}")
    return got


def _job_cmd(server: dict, cmd: str, work_dir: str, jobs: int) -> str:
    return (
        server_lock_prefix(server)
        + "RUSTUP_BIN=$(ls -d ~/.rustup/toolchains/stable*/bin 2>/dev/null | head -1); "
        "export PATH=$HOME/.local/bin:$HOME/.cargo/bin:${RUSTUP_BIN}:$PATH; "
        "export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only; "
        # 关掉颜色与分页，日志保持纯文本
        "export TERM=dumb NO_COLOR=1 CARGO_TERM_COLOR=never GIT_PAGER=cat PAGER=cat PYTHONUNBUFFERED=1; "
        f"export CARGO_BUILD_JOBS={jobs}; "
        # 语料参考 JDK 根目录：作业里的 run_tests / gen_trees 等语料脚本经它取参考构建
        + refjdk_env(server)
        + mem_limited(f"cd {work_dir} && exec nice -n 10 bash -c {shlex.quote(cmd)}", server)
        + " 2>&1"
    )


def _run_job(client, server: dict, key: str, entry: dict, timeout: int, out_dir: Path):
    """按租约（entry 里的 run_dir / work_dir / start / jobs）发起或接上作业，输出流式落盘。
    返回 (结果, rc, OOM 信息, 结束时的连接)。结果：done / timeout / busy / nomemlimit / dead / lost / aborted。"""
    label, work_dir, run_dir = server["label"], entry["work_dir"], entry["run_dir"]
    log_path = out_dir / f"{key}_{label}.log"
    st = {"first": True, "busy": False, "nomem": None, "oom": None}
    with log_path.open("w", encoding="utf-8") as fh:
        fh.write(f"# server: {label}  dir: {work_dir}  CARGO_BUILD_JOBS={entry['jobs']}  "
                 f"start: {datetime.fromtimestamp(entry['start']).isoformat(timespec='seconds')}\n"
                 f"# cmd: {entry['cmd']}\n# run: {run_dir}\n\n")

        def on_line(line: str):
            if st["first"] and line.strip() == BUSY_MARK:
                st["busy"] = True
            elif st["first"] and line.startswith(NO_MEMLIMIT_MARK):
                st["nomem"] = line.strip()
            else:
                st["oom"] = parse_oom_line(line) or st["oom"]
                fh.write(line + "\n")
                fh.flush()
            st["first"] = False

        def on_client(c):
            with _conns_lock:
                _conns[label] = (c, work_dir)

        out = remote_run.run_detached(
            server, client, run_dir, _job_cmd(server, entry["cmd"], work_dir, entry["jobs"]),
            on_line=on_line, timeout_at=entry["start"] + timeout, on_client=on_client,
        )
    if out.attached:
        _log(f"[{label}] ↪ job {key} 结果目录已存在，接上（{run_dir}）")
    if out.reconnects:
        _log(f"[{label}] job {key} 断线重连 {out.reconnects} 次，续读未重派")
    if st["busy"] or st["nomem"]:
        log_path.unlink(missing_ok=True)
        if st["nomem"]:
            _log(f"[{label}] ❌ {st['nomem']}")
        return ("busy" if st["busy"] else "nomemlimit"), None, None, out.client
    if out.status == "done" and out.reconnects == 0:
        remote_run.note_clean_run(host_label(server))
    return out.status, out.rc, st["oom"], out.client


def _settle_return(board: JobBoard, key: str, label: str, why: str):
    """异常归还：超过 MAX_INFRA_RETURNS 次记 infra 失败（status=done、infra=True），否则回队列。"""
    n = board.note_return(key)
    if n > MAX_INFRA_RETURNS:
        board.record(key, status="done", rc=None, infra=True, error=why)
        _log(f"[{label}] 🚫 job {key} 异常归还 {n} 次，记为 infra 失败（{why}）")
    else:
        board.push_back(key)
        board.record(key, status="pending")
        _log(f"[{label}] ⚠️  job {key} {why}，已归还（{n}/{MAX_INFRA_RETURNS}）")


def job_worker(server: dict, board: JobBoard, tag: str, ref: str, globs: list[str],
               min_ram: int, timeout: int):
    label = server["label"]
    base_dir = server.get("remote_dir", REMOTE_DIR)
    work_dir = job_remote_dir(base_dir, tag, server.get("slot", 0))
    lease_key = board.take_lease(label)
    # 取到第一个作业时才检出，没分到作业的服务器不占磁盘与资源；接上租约时检出目录已在
    commit = board.data[lease_key].get("commit") if lease_key else None
    orphans_cleaned = False

    fails = 0
    while not _shutdown.is_set():
        key, lease_key = (lease_key, None) if lease_key else (None, None)
        resumed = key is not None
        if key is None:
            if remote_run.cooldown_left(host_label(server)) > 0:
                _shutdown.wait(min(remote_run.cooldown_left(host_label(server)), WAIT_SECONDS))
                continue
            key = board.pop()
        if key is None:
            # 队列空但别的服务器手里还有作业（可能因服务器锁被占而归还）：等着接手，全部完成才退出
            if board.unfinished() == 0:
                break
            _shutdown.wait(WAIT_SECONDS)
            continue
        client, _ = _connect_with_retry(server)
        if client is None:
            board.push_back(key)
            board.record(key, status="pending")
            return
        with _conns_lock:
            _conns[label] = (client, work_dir)
        try:
            if not orphans_cleaned:
                # 本实例作业目录下无租约的残留运行（上次中断留下的）
                keep = [board.data[key]["run_dir"]] if resumed else []
                remote_run.cleanup_orphans(client, work_dir, keep)
                orphans_cleaned = True
            if resumed:
                entry = board.data[key]
                _log(f"[{label}] ↪ 接上 job {key}（{entry['run_key']}）")
            else:
                res = _check_resources(client)
                if res is None:
                    board.push_back(key)
                    fails += 1
                    if fails > MAX_RETRIES:
                        _log(f"[{label}] ❌ 资源检查持续失败，退出")
                        return
                    _shutdown.wait(RETRY_WAITS[min(fails - 1, len(RETRY_WAITS) - 1)])
                    continue
                fails = 0
                ram, load, disk, ncpu = res
                if disk < MIN_DISK_GB:
                    board.push_back(key)
                    _log(f"[{label}] ⏭  磁盘剩余 {disk}GB < {MIN_DISK_GB}GB，退出")
                    return
                if not _resource_ok(ram, load, server) or ram < min_ram:
                    board.push_back(key)
                    _log(f"[{label}] ⏳ 资源紧张（负载 {load:.0%}  RAM {ram}MB），{WAIT_SECONDS}s 后再试")
                    _shutdown.wait(WAIT_SECONDS)
                    continue

                if commit is None:
                    board.mark_checkout(label, True)
                    err = prepare_spot_checkout(client, base_dir, work_dir, ref)
                    if err:
                        board.push_back(key)
                        _log(f"[{label}] ❌ 作业检出失败，退出: {err}")
                        return
                    jhome, err = ensure_reference_jdk(client, server, work_dir)
                    if err:
                        board.push_back(key)
                        _log(f"[{label}] ❌ 参考 JDK 未能就位，退出: {err}")
                        return
                    commit = _fetch_git_hash(client, work_dir, short=False)
                    _log(f"[{label}] 作业目录就绪 {work_dir} @ {commit[:9]}（参考 JDK：{jhome or '该提交无清单'}）")

                cmd = board.data[key]["cmd"]
                run_key = make_run_key(f"job{key}", commit[:12], tag)
                _log(f"[{label}] → job {key}: {cmd}")
                # 先落租约再发起：本地中途退出后可凭 summary.json 接上
                board.record(key, status="running", server=label, commit=commit, run_key=run_key,
                             run_dir=f"{runs_root(work_dir)}/{run_key}", work_dir=work_dir,
                             start=time.time(), jobs=max(1, ncpu // 2 // server.get("slots", 1)))
                entry = board.data[key]

            status, rc, oom, client = _run_job(client, server, key, entry, timeout, board.out_dir)
            if status == "aborted":
                if client is not None:
                    try:
                        remote_run._exec(client, remote_run.abort_runs_script(work_dir), timeout=30)
                    except Exception:
                        pass
                return                          # running 条目保留，下次启动在原结果目录重新发起
            if status in ("dead", "lost"):
                if client is not None and status == "dead":
                    remote_run.remove_run(client, entry["run_dir"])
                _settle_return(board, key, label,
                               "远端进程已死且无 done" if status == "dead" else "长时间失联，远端状态未知")
                continue
            if status == "nomemlimit":
                remote_run.remove_run(client, entry["run_dir"])
                board.push_back(key)
                board.record(key, status="pending")
                _log(f"[{label}] ❌ 无法建立内存受限 scope，停止使用该服务器（env_setup --check-only 查看 memlimit 列）")
                return
            if status == "busy":
                remote_run.remove_run(client, entry["run_dir"])
                board.push_back(key)
                board.record(key, status="pending")
                _log(f"[{label}] ⏳ 服务器上已有测试 / 作业在跑，{WAIT_SECONDS}s 后再试")
                _shutdown.wait(WAIT_SECONDS)
                continue
            got = _fetch_artifacts(client, work_dir, globs, board.out_dir / key)
            # 收回后删结果目录；作业目录独占：恢复到检出状态（保留 build/ 编译缓存），不让上一个作业的改动影响下一个
            remote_run.remove_run(client, entry["run_dir"],
                                  f"cd {shlex.quote(work_dir)} && git reset -q --hard && git clean -fdq -e build;")
            dur = int(time.time() - entry["start"])
            timed_out = status == "timeout"
            board.record(key, status="done", rc=rc, oom=oom, seconds=dur, artifacts=got,
                         timeout=timed_out or None,
                         finished=datetime.now().isoformat(timespec="seconds"))
            mark = "💥 OOM" if oom else ("⏰ 超时" if timed_out else ("✅" if rc == 0 else "❌"))
            _log(f"[{label}] {mark} job {key} rc={rc}  {dur // 60}:{dur % 60:02d}"
                 + (f"  内存上限内被杀（{oom}）" if oom else "")
                 + (f"  取回 {len(got)} 个产物" if got else ""))
        except Exception as e:
            if not _shutdown.is_set():
                _settle_return(board, key, label, f"作业异常: {e}")
        finally:
            with _conns_lock:
                _conns.pop(label, None)
            if client is not None:
                client.close()
    # 本次检出过，或此前被中断的运行在本服务器留下过检出（checkouts.json 登记）：全部作业完成后删除
    if (not _shutdown.is_set() and board.unfinished() == 0
            and (commit is not None or label in board.checkouts())):
        c, _ = _connect_with_retry(server)
        if c is not None:
            remove_checkout(c, work_dir)
            c.close()
            board.mark_checkout(label, False)


def _remove_stale_checkouts(board: JobBoard, tag: str):
    """作业已全部成功，但此前被中断的运行在某些服务器上留下检出：逐台删除。"""
    for label in sorted(board.checkouts()):
        server = next((s for s in slot_servers(SERVERS) if s["label"] == label), None)
        if server is None:
            board.mark_checkout(label, False)
            continue
        c, _ = _connect_with_retry(server, max_rounds=1)
        if c is None:
            continue
        remove_checkout(c, job_remote_dir(server.get("remote_dir", REMOTE_DIR), tag, server.get("slot", 0)))
        c.close()
        board.mark_checkout(label, False)
        _log(f"[{label}] 已删除残留作业检出")


def run_jobs(tag: str, ref: str, cmds: list[str], globs: list[str], servers: list[str] | None,
             min_ram: int, timeout: int):
    from concurrent.futures import ThreadPoolExecutor

    out_dir = dist_ctx.results_dir / "job" / tag
    out_dir.mkdir(parents=True, exist_ok=True)
    signal.signal(signal.SIGINT, _job_signal_handler)
    signal.signal(signal.SIGTERM, _job_signal_handler)
    board = JobBoard(cmds, out_dir)
    if board.unfinished() == 0:
        _log("所有作业均已成功")
        _remove_stale_checkouts(board, tag)
        return
    # --servers 按物理机标签筛选（缺省取 config pools 含 "job" 的服务器），再按槽展开：每槽一个工作线程
    picked = slot_servers(pick_servers(servers, "job"))
    board.release_leases({s["label"] for s in picked})
    _log(f"作业 {board.remaining()} 个，服务器 {[s['label'] for s in picked]}，ref={ref}，结果: {out_dir}")
    with ThreadPoolExecutor(max_workers=len(picked)) as ex:
        for s in picked:
            ex.submit(job_worker, s, board, tag, ref, globs, min_ram, timeout)
    _log(f"\n{'=' * 60}")
    for key, v in board.data.items():
        rc, oom = v.get("rc"), v.get("oom")
        mark = ("🚫" if v.get("infra") else "💥" if oom else "⏰" if v.get("timeout")
                else "✅" if rc == 0 else "❌" if rc is not None else "⏸ ")
        _log(f"{mark} {key} [{v.get('server', '-')}] rc={rc}" + (f" OOM {oom}" if oom else "")
             + (" infra 失败（网络 / 执行异常）" if v.get("infra") else "") + f"  {v['cmd']}")
    _log(f"日志与产物: {out_dir}")
