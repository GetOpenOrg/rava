"""SSH 客户端：建连（私钥 / 密码、SOCKS5 代理或自定义 sock、心跳保活）与执行命令。"""

import random
import socket
import time

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
        client = paramiko.SSHClient()
        client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
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


