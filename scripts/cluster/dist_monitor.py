"""实时监控界面：各服务器状态登记（_set_st）、状态表渲染与刷新循环、最终汇总。"""

import sys
import time
import threading
import unicodedata

from cluster_config import SERVERS, MAX_RETRIES
import dist_ctx
from dist_ctx import _log, _print_lock


# ── 服务器实时状态 ─────────────────────────────────────────────────────────────

_svr_lock = threading.Lock()
_svr_st: dict = {}

def _vis(ch: str) -> int:
    """单个字符的终端显示列数（CJK/emoji = 2，其余 = 1）。"""
    eaw = unicodedata.east_asian_width(ch)
    if eaw in ("W", "F"):
        return 2
    # emoji 在 unicodedata 里常被标为 N/Na，用码点范围补判
    cp = ord(ch)
    if 0x1F000 <= cp <= 0x1FFFF or 0x2600 <= cp <= 0x27BF:
        return 2
    return 1


def _col(s: str, width: int) -> str:
    """将字符串填充/截断到 `width` 个终端列（左对齐）。"""
    cols = sum(_vis(c) for c in s)
    if cols < width:
        return s + " " * (width - cols)
    result, acc = [], 0
    for c in s:
        w = _vis(c)
        if acc + w > width:
            break
        result.append(c)
        acc += w
    return "".join(result) + " " * (width - acc)


def _col_r(s: str, width: int) -> str:
    """将字符串填充/截断到 `width` 个终端列（右对齐）。"""
    cols = sum(_vis(c) for c in s)
    if cols < width:
        return " " * (width - cols) + s
    result, acc = [], 0
    for c in reversed(s):
        w = _vis(c)
        if acc + w > width:
            break
        result.append(c)
        acc += w
    return " " * (width - acc) + "".join(reversed(result))


STATUS_ICONS = {
    "idle":       "⏸  空闲",
    "connecting": "🔗 连接中",
    "setup":      "🔧 初始化",
    "checking":   "📊 检查资源",
    "waiting":    "⏳ 等待资源",
    "running":    "▶  运行中",
    "collecting": "📥 收集结果",
    "cleaning":   "🧹 清理中",
    "done":       "✅ 完成",
    "skip":       "⏭  磁盘不足",
    "error":      "❌ 错误",
    "cooldown":   "🧯 断线冷却",
}


def _set_st(label: str, **kw):
    with _svr_lock:
        if label not in _svr_st:
            _svr_st[label] = {
                "status": "connecting", "attempt": 0,
                "passed": 0, "failed": 0, "current": "",
                "ram_mb": 0, "load": 0.0, "git_hash": "?",
            }
        _svr_st[label].update(kw)


# ── 监控界面 ──────────────────────────────────────────────────────────────────

def _render_table() -> str:
    pool  = dist_ctx.pool
    state = dist_ctx.state
    w = 76
    if pool is None or state is None:
        # pool/state 尚未就绪（正在初始化环境）
        lines = [
            "=" * w,
            "  rava 分布式测试  |  🔧 正在初始化服务器环境，请稍候…",
            "-" * w,
            "  " + _col("服务器", 7) + "  " + _col("git", 7) + "  " + _col("状态", 16) +
            "  " + _col_r("✅通过", 6) + "  " + _col_r("❌失败", 6) +
            "  " + _col_r("RAM空闲", 9) + "  " + _col_r("负载", 5) + "  当前测试(耗时)",
            "-" * w,
        ]
        now = time.time()
        with _svr_lock:
            for srv in dist_ctx.slot_servers or SERVERS:
                lbl    = srv["label"]
                st     = _svr_st.get(lbl, {})
                status = st.get("status", "connecting")
                attempt = st.get("attempt", 0)
                if status == "retrying":
                    status_str = f"🔄 未连接({attempt}/{MAX_RETRIES})"
                    current    = f"下次重试 {max(0,int(st.get('retry_until',now)-now))}s 后"
                else:
                    status_str = STATUS_ICONS.get(status, status)
                    current    = st.get("current", "")[:28]
                lines.append(
                    f"  {_col(lbl,7)}  {_col(st.get('git_hash','?'),7)}  {_col(status_str,16)}  "
                    f"{st.get('passed',0):>6}  {st.get('failed',0):>6}  "
                    f"{str(st.get('ram_mb',0))+'MB':>9}  {st.get('load',0.0):>5.0%}  {current}"
                )
        lines.append("=" * w)
        return "\n".join(lines)

    s         = state.summary()
    rem       = pool.remaining()
    tot       = pool.total()
    done_cnt  = s["passed"] + s["failed"]
    in_flight = tot - rem - done_cnt   # 已分发但尚未返回结果的测试数
    finished  = rem == 0 and in_flight == 0
    lines = [
        "=" * w,
        (
            f"  rava 分布式测试  JDK {s['jdk']}  |  "
            f"任务池剩余 {rem}/{tot}  "
            f"{'✅ 完成！' if finished else '进行中…'}"
        ),
        (
            f"  已完成 {done_cnt}/{tot}  "
            f"通过 {s['passed']}  失败 {s['failed']}  "
            f"进行中 {in_flight}"
        ),
        "-" * w,
        "  " + _col("服务器", 7) + "  " + _col("git", 7) + "  " + _col("状态", 16) +
        "  " + _col_r("✅通过", 6) + "  " + _col_r("❌失败", 6) +
        "  " + _col_r("RAM空闲", 9) + "  " + _col_r("负载", 5) + "  当前测试(耗时)",
        "-" * w,
    ]
    now = time.time()
    with _svr_lock:
        for srv in dist_ctx.slot_servers or SERVERS:
            lbl = srv["label"]
            st      = _svr_st.get(lbl, {})
            status  = st.get("status", "connecting")
            attempt = st.get("attempt", 0)
            if status == "retrying":
                status_str = f"🔄 未连接({attempt}/{MAX_RETRIES})"
            else:
                status_str = STATUS_ICONS.get(status, status)

            # 当前测试栏内容
            current = st.get("current", "")
            if status == "retrying":
                countdown = max(0, int(st.get("retry_until", now) - now))
                current = f"下次重试 {countdown}s 后"
            elif status in ("running", "collecting", "cleaning") and st.get("task_start"):
                elapsed = int(now - st["task_start"])
                m, s = divmod(elapsed, 60)
                current = f"{current[:20]} ({m}:{s:02d})"
            else:
                current = current[:28]

            lines.append(
                f"  {_col(lbl, 7)}  "
                f"{_col(st.get('git_hash', '?'), 7)}  "
                f"{_col(status_str, 16)}  "
                f"{st.get('passed',0):>6}  "
                f"{st.get('failed',0):>6}  "
                f"{str(st.get('ram_mb',0))+'MB':>9}  "
                f"{st.get('load',0.0):>5.0%}  "
                f"{current}"
            )
    lines.append("=" * w)
    return "\n".join(lines)


def _monitor_loop(stop: threading.Event):
    dist_ctx.monitor_active = True
    try:
        while not stop.is_set():
            table = _render_table()
            with _print_lock:
                sys.__stdout__.write("\033[H\033[J")
                sys.__stdout__.write(table + "\n")
                sys.__stdout__.flush()
            stop.wait(3)
    finally:
        dist_ctx.monitor_active = False


# ── 最终汇总 ──────────────────────────────────────────────────────────────────

def print_final_summary(state, pool):
    s   = state.summary()
    rem = pool.remaining()
    _log(f"\n{'='*60}")
    _log(f"📊 分布式测试完成")
    _log(f"   JDK {s['jdk']}  总测试 {pool.total()}")
    _log(f"   通过: {s['passed']}  失败: {s['failed']}  未运行: {rem}"
         + (f"  infra 失败（网络 / 执行异常，不计入失败）: {s['infra']}" if s.get("infra") else ""))
    if s.get("infra"):
        _log(f"   infra 清单: {dist_ctx.results_dir / ('infra_failed_jdk' + str(s['jdk']) + '.txt')}")
    if s["failed"]:
        fail_list = dist_ctx.results_dir / f"failed_tests_jdk{s['jdk']}.txt"
        _log(f"   失败清单: {fail_list}")
        _log(f"   详细日志: {dist_ctx.results_dir / 'error_logs'}")
    _log("=" * 60)
