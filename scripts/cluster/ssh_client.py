"""SSH 客户端：建连（私钥 / 密码、SOCKS5 代理或自定义 sock、心跳保活）与执行命令。

主机密钥校验：先读系统 ~/.ssh/known_hosts，再读 rava 维护的 known_hosts（cluster_config.KNOWN_HOSTS_PATH，
缺省 ~/.config/rava/known_hosts）。未登记的主机拒绝连接（每主机告警一次），密钥不符由 paramiko 拒绝；
首次接入走显式 TOFU：分发时加 --trust-new-host（或环境变量 RAVA_TRUST_NEW_HOST=1），
或预先登记：python scripts/cluster/ssh_client.py trust [标签 ...]（经 直连 → 跳板 → 代理 链路逐台建连，
把服务器当前密钥写入 rava 的 known_hosts，含已在系统文件登记的主机）。
"""

import base64
import fcntl
import hashlib
import os
import random
import socket
import sys
import threading
import time
from pathlib import Path

import paramiko

# SSH 心跳间隔（秒）：须小于代理 / NAT 的空闲超时（xray 缺省 connIdle=300s）
SSH_KEEPALIVE = 15


def _make_socks5_socket(proxy_host, proxy_port, target_host, target_port, timeout=20):
    """通过 SOCKS5 代理建立到目标的 TCP 连接，返回 socket"""
    import struct
    sock = socket.create_connection((proxy_host, proxy_port), timeout=timeout)
    # SOCKS5 握手
    sock.sendall(b"\x05\x01\x00")
    if sock.recv(2) != b"\x05\x00":
        raise ConnectionError("SOCKS5 代理握手失败")
    # CONNECT 请求（域名类型）
    host_bytes = target_host.encode()
    sock.sendall(b"\x05\x01\x00\x03" + bytes([len(host_bytes)]) + host_bytes + struct.pack("!H", target_port))
    resp = sock.recv(10)
    if resp[1] != 0x00:
        raise ConnectionError(f"SOCKS5 CONNECT 失败，错误码: {resp[1]:#04x}")
    return sock


# 首次接入的主机是否按 TOFU 登记（--trust-new-host 置位）；缺省拒绝
TRUST_NEW_HOSTS = os.environ.get("RAVA_TRUST_NEW_HOST", "") not in ("", "0")

_known_lock = threading.Lock()
_warned_hosts: set[str] = set()


class HostKeyRejected(paramiko.SSHException):
    """主机密钥未登记且未开 TOFU：不重试。"""


def known_hosts_path() -> Path:
    try:
        from cluster_config import KNOWN_HOSTS_PATH
        return Path(KNOWN_HOSTS_PATH)
    except Exception:
        return Path.home() / ".config" / "rava" / "known_hosts"


def host_key_name(host: str, port: int) -> str:
    """known_hosts 中的主机名（与 paramiko 查找口径一致：非 22 端口写成 [host]:port）。"""
    return host if port == 22 else f"[{host}]:{port}"


def fingerprint(key) -> str:
    return "SHA256:" + base64.b64encode(hashlib.sha256(key.asbytes()).digest()).decode().rstrip("=")


def record_host_key(hostname: str, key, path: Path | None = None) -> bool:
    """把 hostname 的密钥追加进 rava known_hosts（进程内锁 + 文件锁；同名同类型已有则不写）。
    返回是否新写入；已登记的同类型密钥与之不符时抛 BadHostKeyException。"""
    path = Path(path or known_hosts_path())
    with _known_lock:
        path.parent.mkdir(parents=True, exist_ok=True)
        fd = os.open(path, os.O_RDWR | os.O_CREAT | os.O_APPEND, 0o600)
        try:
            fcntl.flock(fd, fcntl.LOCK_EX)
            existing = paramiko.HostKeys()
            existing.load(str(path))
            old = (existing.lookup(hostname) or {}).get(key.get_name())
            if old is not None:
                if old != key:
                    raise paramiko.BadHostKeyException(hostname, key, old)
                return False
            line = paramiko.hostkeys.HostKeyEntry([hostname], key).to_line()
            os.write(fd, line.encode())
            os.fsync(fd)
            return True
        finally:
            os.close(fd)


class _RejectUnknownHost(paramiko.MissingHostKeyPolicy):
    def missing_host_key(self, client, hostname, key):
        if hostname not in _warned_hosts:
            _warned_hosts.add(hostname)
            print(f"❌ 主机 {hostname} 的 {key.get_name()} 密钥 {fingerprint(key)} 未登记"
                  f"（系统 known_hosts 与 {known_hosts_path()} 均无），拒绝连接；"
                  f"核实指纹后用 `ssh_client.py trust <标签>` 或分发器 --trust-new-host 登记",
                  file=sys.stderr, flush=True)
        raise HostKeyRejected(f"主机 {hostname} 的密钥未登记")


class _TrustOnFirstUse(paramiko.MissingHostKeyPolicy):
    def missing_host_key(self, client, hostname, key):
        if record_host_key(hostname, key):
            print(f"🔏 TOFU：登记主机 {hostname} 的 {key.get_name()} 密钥 {fingerprint(key)} → {known_hosts_path()}",
                  file=sys.stderr, flush=True)
        client.get_host_keys().add(hostname, key.get_name(), key)


def _new_client() -> paramiko.SSHClient:
    client = paramiko.SSHClient()
    client.load_system_host_keys()
    kh = known_hosts_path()
    if kh.exists():
        # 读入 _host_keys 而不经 load_host_keys：后者会记下文件名，paramiko 关闭时可能回写
        client.get_host_keys().load(str(kh))
    client.set_missing_host_key_policy(_TrustOnFirstUse() if TRUST_NEW_HOSTS else _RejectUnknownHost())
    return client


def create_ssh_client(server, max_retries=3, verbose=True, timeout=20, sock_factory=None):
    """创建 SSH 客户端，支持私钥或密码登录，失败时自动重试（指数退避）。
    server 中可选 socks5_proxy: {"host": "127.0.0.1", "port": 10808} 以走代理。
    sock_factory：可选的无参回调，每次尝试调用一次，返回到目标的已连通 socket 类对象
    （如跳板机上的 direct-tcpip 通道）；给出时优先于 socks5_proxy。
    verbose=False 时静默运行，不输出任何日志（由调用方自行记录状态）。
    """
    def _print(msg):
        if verbose:
            print(msg)

    host = server["host"]
    tag = f"[{server['label']}] " if server.get("label") else ""

    proxy_cfg = server.get("socks5_proxy")
    for attempt in range(max_retries):
        client = _new_client()
        try:
            sock = None
            if sock_factory is not None:
                sock = sock_factory()
            elif proxy_cfg:
                sock = _make_socks5_socket(
                    proxy_cfg["host"], proxy_cfg["port"],
                    server["host"], server["port"], timeout=timeout,
                )
            connect_kwargs = dict(
                hostname=server["host"],
                port=server["port"],
                username=server["username"],
                timeout=timeout,
                sock=sock,
            )
            if "private_key_path" in server:
                msg = f"🔑 {tag}使用私钥登录 {host}" if attempt == 0 else f"🔄 {tag}重试连接 {host} (第 {attempt + 1} 次)"
                _print(msg)
                key_path = server["private_key_path"]
                for _cls in (paramiko.Ed25519Key, paramiko.RSAKey, paramiko.ECDSAKey):
                    try:
                        connect_kwargs["pkey"] = _cls(filename=key_path)
                        break
                    except Exception:
                        continue
            else:
                msg = f"🔐 {tag}使用密码登录 {host}" if attempt == 0 else f"🔄 {tag}重试连接 {host} (第 {attempt + 1} 次)"
                _print(msg)
                connect_kwargs["password"] = server["password"]
            if proxy_cfg and attempt == 0:
                _print(f"🔀 {tag}经由 SOCKS5 {proxy_cfg['host']}:{proxy_cfg['port']} 代理")
            client.connect(**connect_kwargs)
            # 保活：长时间无输出（如 cargo 编译数分钟）时，SOCKS 代理 / NAT 会按空闲超时切断连接，
            # 远端任务随会话一起被杀。SSH 层每 SSH_KEEPALIVE 秒发一次心跳，TCP 层开 SO_KEEPALIVE 兜底
            transport = client.get_transport()
            transport.set_keepalive(SSH_KEEPALIVE)
            # 经跳板时底层是 paramiko Channel，没有 TCP 选项可设（跳板连接自身已开保活）
            if isinstance(transport.sock, socket.socket):
                transport.sock.setsockopt(socket.SOL_SOCKET, socket.SO_KEEPALIVE, 1)
            _print(f"✅ {tag}成功连接到 {host}")
            return client
        except (HostKeyRejected, paramiko.BadHostKeyException) as e:
            # 主机密钥未登记 / 不符：重试无意义
            client.close()
            _print(f"❌ {tag}{host} 主机密钥校验失败: {e}")
            return None
        except Exception as e:
            client.close()
            if attempt < max_retries - 1:
                wait = 2 ** attempt + random.uniform(0, 1)  # 1-2s, 2-3s
                _print(f"⚠️ {tag}连接 {host} 失败: {e}，{wait:.1f} 秒后重试...")
                time.sleep(wait)
            else:
                _print(f"❌ {tag}登录 {host} 失败 (重试 {max_retries} 次后放弃): {e}")
                return None


def run_command(client, cmd, timeout=120):
    """执行 SSH 命令，以 exit code 判断成功失败，返回输出字符串。

    timeout 默认 120 秒，docker compose restart 等命令需要较长时间。
    只在 exit code != 0 时才读取 stderr，避免把正常诊断信息误判为错误。
    """
    print("-" * 80)
    print(f"\n🔹 执行: {cmd}")
    output_lines = []
    try:
        stdin, stdout, stderr = client.exec_command(cmd, timeout=timeout)
        stdout.channel.settimeout(timeout)
        stdin.close()

        for line in iter(stdout.readline, ""):
            print(line, end="")
            output_lines.append(line.strip())

        exit_status = stdout.channel.recv_exit_status()
        if exit_status == 0:
            print(f"✅ 执行成功: {cmd}")
        else:
            err = stderr.read().decode().strip()
            print(f"❌ 执行失败 (exit={exit_status}): {cmd}")
            if err:
                print(f"   错误: {err}")
    except Exception as e:
        print(f"❌ 执行失败: {cmd}，错误: {str(e)}")

    return "\n".join(output_lines).strip()




def trust_servers(labels: list[str]) -> int:
    """逐台经 直连 → 跳板 → 代理 建连（开 TOFU），把服务器当前密钥写入 rava known_hosts。"""
    global TRUST_NEW_HOSTS
    TRUST_NEW_HOSTS = True
    import cluster_config as config
    from dist_conn import _connect_with_retry, close_jump_clients
    servers = [s for s in config.SERVERS if not labels or s["label"] in labels]
    missing = set(labels) - {s["label"] for s in servers}
    if missing:
        print(f"未知标签: {' '.join(sorted(missing))}", file=sys.stderr)
        return 2
    rc = 0
    for srv in servers:
        client, via = _connect_with_retry(srv, max_rounds=1)
        if client is None:
            print(f"❌ [{srv['label']}] 无法建连，未登记")
            rc = 1
            continue
        key = client.get_transport().get_remote_server_key()
        name = host_key_name(srv["host"], srv["port"])
        try:
            new = record_host_key(name, key)
        except paramiko.BadHostKeyException as e:
            print(f"❌ [{srv['label']}] {e}")
            rc = 1
            new = None
        finally:
            client.close()
        if new is not None:
            print(f"{'🔏 登记' if new else '✓ 已登记'} [{srv['label']}] {name} {key.get_name()} {fingerprint(key)}"
                  f"（{via or '直连'}）")
    close_jump_clients()
    return rc


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser(description="rava 集群 SSH 主机密钥登记")
    sub = ap.add_subparsers(dest="cmd", required=True)
    t = sub.add_parser("trust", help="建连并把服务器主机密钥写入 rava known_hosts（缺省全部服务器）")
    t.add_argument("labels", nargs="*")
    a = ap.parse_args()
    sys.exit(trust_servers(a.labels))
