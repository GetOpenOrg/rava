"""SSH 建连：每轮依次尝试 直连 → 同区跳板（服务器条目的 jump 键）→ FALLBACK_PROXY（本机清单配置了才试），带退避重试。

跳板：在跳板服务器的 SSH transport 上开 direct-tcpip 通道，作为目标连接的 sock。
跳板连接按跳板服务器标签缓存、各线程共用；失效时重建。跳板自身只试 直连 → 代理，不再递归跳板。
"""

import threading
import time

from ssh_client import create_ssh_client
import cluster_config as config
from cluster_config import RETRY_WAITS, MAX_RETRIES, FALLBACK_PROXY, CONNECT_TIMEOUT
from dist_ctx import _shutdown, _log
from dist_monitor import _set_st

# 单次建连等待上限：TCP 建连之后 SSH 握手 / 认证仍可能卡住，留出余量
CONNECT_WAIT_CAP = CONNECT_TIMEOUT * 3


def _server_by_label(label: str) -> dict | None:
    return next((s for s in config.SERVERS if s["label"] == label), None)


def _direct_cfg(server: dict) -> dict:
    return {k: v for k, v in server.items() if k != "socks5_proxy"}


def _proxy_cfg(server: dict) -> dict:
    return {**_direct_cfg(server), "socks5_proxy": FALLBACK_PROXY}


# ── 跳板连接缓存 ──────────────────────────────────────────────────────────────

_jump_lock = threading.Lock()
_jump_clients: dict[str, object] = {}


def _jump_client(jump_label: str):
    """取跳板服务器的可用连接（缓存、线程共用）；失效时按 直连 → 代理 重建一次，失败返回 None。"""
    with _jump_lock:
        c = _jump_clients.get(jump_label)
        t = c.get_transport() if c is not None else None
        if t is not None and t.is_active():
            return c
        if c is not None:
            try:
                c.close()
            except Exception:
                pass
            _jump_clients.pop(jump_label, None)
        jump = _server_by_label(jump_label)
        if jump is None:
            return None
        methods = [_direct_cfg(jump)] + ([] if jump.get("direct_only") or not FALLBACK_PROXY else [_proxy_cfg(jump)])
        for cfg in methods:
            if _shutdown.is_set():
                return None
            c = create_ssh_client(cfg, max_retries=1, verbose=False, timeout=CONNECT_TIMEOUT)
            if c is not None:
                _jump_clients[jump_label] = c
                return c
        return None


def _jump_sock_factory(jump_label: str, host: str, port: int):
    """返回 sock_factory：在跳板 transport 上开到 host:port 的 direct-tcpip 通道。"""
    def _open():
        jc = _jump_client(jump_label)
        if jc is None:
            raise ConnectionError(f"跳板 {jump_label} 不可用")
        return jc.get_transport().open_channel(
            "direct-tcpip", (host, port), ("127.0.0.1", 0), timeout=CONNECT_TIMEOUT)
    return _open


def close_jump_clients():
    with _jump_lock:
        for c in _jump_clients.values():
            try:
                c.close()
            except Exception:
                pass
        _jump_clients.clear()


# ── 带重试的建连 ──────────────────────────────────────────────────────────────

def _methods(server: dict) -> list[tuple[dict, str, object]]:
    """(连接配置, 方式描述, sock_factory)：直连 → 跳板（若配置）→ 代理；direct_only 只直连。"""
    out: list[tuple[dict, str, object]] = [(_direct_cfg(server), "", None)]
    if server.get("direct_only"):
        return out
    jump = server.get("jump")
    if jump and _server_by_label(jump) is not None:
        out.append((_direct_cfg(server), f"跳板 {jump}",
                    _jump_sock_factory(jump, server["host"], server["port"])))
    if FALLBACK_PROXY:
        out.append((_proxy_cfg(server), f"代理 {FALLBACK_PROXY['host']}:{FALLBACK_PROXY['port']}", None))
    return out


def _connect_with_retry(server: dict, max_rounds: int | None = None) -> tuple:
    """
    每轮依次尝试 直连 → 跳板 → 代理，哪个先通用哪个。
    返回 (client, via)：via 为 "" 表示直连，否则为方式描述（如 "跳板 jp1"）；
    超过 max_rounds（缺省 MAX_RETRIES）轮仍失败或收到关闭信号时返回 (None, "")。
    """
    label = server["label"]
    host  = server["host"]
    waits = RETRY_WAITS[:max_rounds] if max_rounds else RETRY_WAITS
    methods = _methods(server)

    for attempt, wait in enumerate(waits):
        if _shutdown.is_set():
            return None, ""

        for srv_cfg, via, factory in methods:
            if _shutdown.is_set():
                return None, ""

            # 开始尝试连接：更新倒计时显示，让监控页面有视觉反馈
            _set_st(label, status="retrying", attempt=attempt + 1,
                    retry_until=time.time() + CONNECT_WAIT_CAP)

            # create_ssh_client 内部 connect() 是 C 层阻塞，无法被信号直接打断。
            # 放到 daemon 线程里，主循环每 1s 检查 _shutdown，保证 Ctrl+C 后 ≤2s 响应。
            _result: list = [None]
            def _do_connect(cfg=srv_cfg, f=factory, out=_result):
                out[0] = create_ssh_client(cfg, max_retries=1, verbose=False,
                                           timeout=CONNECT_TIMEOUT, sock_factory=f)

            t = threading.Thread(target=_do_connect, daemon=True)
            t.start()
            for _ in range(CONNECT_WAIT_CAP):
                t.join(timeout=1.0)
                if not t.is_alive() or _shutdown.is_set():
                    break

            client = _result[0]
            if _shutdown.is_set():
                if client:
                    try: client.close()
                    except Exception: pass
                return None, ""

            if client:
                if attempt > 0:
                    _log(f"  [{label}] ✅ {host} {via or '直连'} 连接成功（第 {attempt+1} 次重试）")
                _set_st(label, attempt=0)   # 连接成功，重试计数归零
                return client, via
            _log(f"  [{label}] {host} {via or '直连'} 失败 ({attempt+1}/{len(waits)})...")

        # 本轮各方式都失败，更新状态显示重试次数与下次重试时间
        _set_st(label, status="retrying", attempt=attempt + 1,
                retry_until=time.time() + wait)
        if _shutdown.wait(timeout=wait):
            return None, ""

    if not _shutdown.is_set():
        _log(f"  [{label}] ❌ {host} 连续 {len(waits)} 轮全部方式失败，放弃")
    return None, ""
