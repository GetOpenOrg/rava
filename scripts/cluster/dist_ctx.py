"""分布式调度的共享运行期上下文：关闭信号、活跃连接、线程本地日志捕获，以及 main() 运行期赋值的
可变状态（结果目录、监控标志、任务池 / 状态引用）。可变状态一律以 `dist_ctx.xxx` 属性访问。"""

import re
import shlex
import sys
import signal
import threading
from pathlib import Path

from cluster_config import RESULTS_DIR

# main() 按模式改写（抽查 → spot/<tag>，作业 → job/<tag>）
results_dir: Path = RESULTS_DIR
# e2e 单例设置（main() 按命令行改写；缺省与旧行为一致）：
# run_tests_args 原样追加到服务器上的 run_tests.py 命令行（如放宽三段超时）；
# task_timeout 为单例从发起到结束的总时限（秒，None = config.TASK_TIMEOUT）
run_tests_args: str = ""
task_timeout: int | None = None
# 监控运行时静默 _log，避免内容打到表格下方
monitor_active = False
# 监控用全局引用（在 main() 里赋值，使监控可在 pool/state 就绪前提前启动）
pool = None
state = None
# 本次运行的槽工作条目（config.slot_servers 展开后，main() 赋值）：状态表按槽一行
slot_servers: list = []

# ── 全局关闭信号 + 活跃连接注册（用于 Ctrl+C 时清理远程进程）────────────────

_shutdown          = threading.Event()
_active_conns_lock = threading.Lock()
_active_conns: dict[str, tuple] = {}    # 槽标签 → (正在执行任务时的 SSH client, 检出目录, 结果目录, 测试名)


def _register_conn(label: str, client, work_dir: str, run_dir: str = "", test: str = ""):
    with _active_conns_lock:
        _active_conns[label] = (client, work_dir, run_dir, test)


def _unregister_conn(label: str):
    with _active_conns_lock:
        _active_conns.pop(label, None)


def _kill_cmd(work_dir: str, sig: str, test: str = "") -> str:
    """结束工作目录为 work_dir 的 run_tests 进程（同一服务器上其他实例 / 其他检出目录的测试不受影响）；
    给出 test 时只结束跑该测试的那一个（--filter /<test>.java），同一检出目录下其他槽的测试不受影响。"""
    d = work_dir.rstrip("/")
    pat = "scripts/run_tests.py" + (f" .*--filter /{re.escape(test)}\\.java( |$)" if test else "")
    return (
        f"for p in $(pgrep -f {shlex.quote(pat)} 2>/dev/null); do "
        f"case \"$(readlink /proc/$p/cwd 2>/dev/null)\" in {d}|{d}/*) kill -{sig} $p 2>/dev/null;; esac; done; "
    )


def _kill_remote_tests(client, label: str, work_dir: str):
    """终止 work_dir 下残留的 run_tests.py（阻塞等待完成，用于启动时清理）。"""
    try:
        stdin, stdout, _ = client.exec_command(
            _kill_cmd(work_dir, "TERM") + "sleep 2; " + _kill_cmd(work_dir, "KILL") + "true",
            timeout=15,
        )
        stdin.close()
        stdout.channel.recv_exit_status()
    except Exception:
        pass


def _abort_cmd(work_dir: str, run_dir: str, test: str) -> str:
    """结束本槽的脱钩运行（进程组 + 结果目录）与其残留 run_tests；同一检出目录下其他槽的运行不受影响。"""
    from remote_run import kill_run_script
    return (kill_run_script(run_dir) + f"; rm -rf {shlex.quote(run_dir)}; "
            + _kill_cmd(work_dir, "TERM", test) + _kill_cmd(work_dir, "KILL", test) + "true")


def _kill_remote_tests_nowait(client, label: str, work_dir: str, run_dir: str, test: str):
    """发出终止命令即返回（Ctrl+C 信号处理用）；连接由 worker 自行关闭——worker 被 _shutdown 唤醒后
    还会用同一连接同步执行一遍 abort_remote，两者幂等。
    测试脱钩运行在 <work_dir>/build/dist_runs/<run_key>/：一并结束其进程组并删除结果目录，
    与原先「断开即终止本实例远端测试」的语义一致；本地租约保留，下次启动在原结果目录重新发起。"""
    from remote_run import exec_nowait
    exec_nowait(client, _abort_cmd(work_dir, run_dir, test))


def abort_remote(client, work_dir: str, run_dir: str, test: str):
    """同步结束本槽远端运行（worker 在 Ctrl+C 后调用）；连接不可用则留待下次启动处理。"""
    from remote_run import _exec
    try:
        _exec(client, _abort_cmd(work_dir, run_dir, test), timeout=30)
    except Exception:
        pass


def _cleanup_handler(signum, frame):
    """
    Ctrl+C / SIGTERM 处理：
      1. 将 SIGINT 重置为默认（再次 Ctrl+C 可强制退出）
      2. 设置关闭标志，让各 worker 在下次循环时自行退出
      3. 对活跃连接 fire-and-forget 结束本实例远端运行（worker 的轮询随 _shutdown 结束，并同步再结束一遍）
      4. 返回——不抛异常，不 sys.exit，让主线程自然走完收尾流程
    """
    # 第二次 Ctrl+C 时走默认行为（强制退出），不再进这个函数
    signal.signal(signal.SIGINT,  signal.SIG_DFL)
    signal.signal(signal.SIGTERM, signal.SIG_DFL)

    _log("\n正在停止，终止远程测试进程...")
    _shutdown.set()

    with _active_conns_lock:
        conns = list(_active_conns.items())

    # 并行发出终止命令（按 PID，不关连接；worker 醒来后再同步结束一次），信号处理函数几乎立即返回
    threads = []
    for label, (client, work_dir, run_dir, test) in conns:
        _log(f"  [{label}] 发送终止信号...")
        t = threading.Thread(target=_kill_remote_tests_nowait, args=(client, label, work_dir, run_dir, test),
                             daemon=True)
        t.start()
        threads.append(t)
    # 最多等 1 秒让线程启动，不阻塞
    for t in threads:
        t.join(timeout=1.0)

    _log("远程进程已终止，等待工作线程退出...\n")


# ── 线程本地 stdout 捕获 ──────────────────────────────────────────────────────

_tls        = threading.local()
_print_lock = threading.Lock()


class _WorkerStdout:
    def write(self, text):
        buf = getattr(_tls, "buf", None)
        if buf is not None:
            buf.write(text)
        elif not monitor_active:
            sys.__stdout__.write(text)
        # 监控模式下且不在 run_task 内：静默，避免杂乱日志打到表格下方
    def flush(self):
        buf = getattr(_tls, "buf", None)
        if buf is not None:
            buf.flush()
        elif not monitor_active:
            sys.__stdout__.flush()


_worker_stdout = _WorkerStdout()


def _install_capture():
    if not isinstance(sys.stdout, _WorkerStdout):
        sys.stdout = _worker_stdout


def _log(msg: str):
    if monitor_active:
        return
    with _print_lock:
        sys.__stdout__.write(msg + "\n")
        sys.__stdout__.flush()
