"""
rava 服务器环境初始化。

自动完成以下所有操作（幂等，已安装的步骤自动跳过）：
  1. 安装 build-essential（C 链接器 cc，Rust 编译必需）
  2. 安装 OpenJDK 21 / 25 与 Maven（lib pilot 取包 scripts/fetch_pilot_deps.sh 需要 mvn）
  3. 安装 Rust / cargo
  4. 安装 uv
  5. git clone / git pull rava 项目
  6. uv sync 初始化 Python 依赖
  7. 语料参考 JDK（rava tools/refjdk.toml 的固定构建）落到数据目录 <refjdk_root>/<tag>/
     （config.REFJDK_ROOT，可按服务器 refjdk_root 覆盖；不动 apt / /usr/lib/jvm）
  8. GraalVM 参照基线依赖（不计入就绪判定，缺失不影响 e2e 派发）：
     apt zlib1g-dev（native-image 链接需要）；Oracle GraalVM（config.GRAALVM_*，钉版本 + sha256）
     落到数据目录 <refjdk_root>/<GRAALVM_DIR>/，与参考 JDK 同盘，不动 /usr/lib/jvm
  9. 验证能建立内存受限 scope（测试 / 作业的内存上限依赖它）。不开 linger：user systemd 随 SSH
     会话存在，会话断开即连带结束其中的测试 / 作业进程，不留无人管的孤儿进程

单独使用：
  uv run --group cluster python scripts/cluster/env_setup.py                     # 初始化所有服务器
  uv run --group cluster python scripts/cluster/env_setup.py --servers kr2 sg1   # 指定服务器
  uv run --group cluster python scripts/cluster/env_setup.py --check-only        # 只检查，不安装
"""

import shlex
import sys
import threading
import argparse
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from ssh_client import run_command
import cluster_config as config
from cluster_config import SERVERS, CONFIG_PATH, REMOTE_DIR, REPO_URL, REFJDK_ROOT, GRAALVM_DIR, GRAALVM_VERSION, GRAALVM_URL, GRAALVM_SHA256
from dist_ctx import _shutdown
from dist_remote import _connect_with_retry, ensure_reference_jdk

_print_lock = threading.Lock()


# ── 工具检查 ──────────────────────────────────────────────────────────────────

# 每项：检查命令（exit 0 = 已安装）
# 固定工具检查（不依赖 remote_dir）
_TOOL_CHECKS_FIXED = {
    "cc":     "command -v cc >/dev/null 2>&1",
    # 检查安装目录而非默认 java 命令，避免 PATH 版本不一致导致误判
    "java21": "find /usr/lib/jvm -maxdepth 1 -name 'java-21-openjdk*' -type d 2>/dev/null | grep -q .",
    "java25": "find /usr/lib/jvm -maxdepth 1 -name 'java-25-openjdk*' -type d 2>/dev/null | grep -q .",
    "mvn":    "command -v mvn >/dev/null 2>&1",
    # native-image 在 Linux 上链接 libz（GraalVM 参照基线）
    "zlib":   "test -f /usr/include/zlib.h",
    "cargo":  "{ command -v cargo || $HOME/.cargo/bin/cargo -V; } >/dev/null 2>&1",
    "uv":     "{ command -v uv || $HOME/.local/bin/uv --version; } >/dev/null 2>&1",
    # 内存受限 scope：MemoryMax / MemorySwapMax 须真正落到 cgroup（memory 控制器未委派给用户时会被静默忽略）
    "memlimit": (
        "systemd-run --user --scope --quiet -p MemoryMax=64M -p MemorySwapMax=0 bash -c "
        "'f=/sys/fs/cgroup$(sed -n \"s/^0:://p\" /proc/self/cgroup); "
        "[ \"$(cat $f/memory.max)\" = 67108864 ] && [ \"$(cat $f/memory.swap.max)\" = 0 ]' >/dev/null 2>&1"
    ),
}

# 用于 --check-only 表格的显示列顺序
_TABLE_COLS = ["cc", "java21", "java25", "mvn", "cargo", "uv", "rava", "uv_sync", "refjdk", "memlimit",
               "zlib", "graalvm"]

# 只服务于 GraalVM 参照基线的项：缺失时照常安装，但不计入就绪判定（不阻塞 e2e 派发）
_OPTIONAL = {"zlib", "graalvm"}


def _exec_check(client, cmd: str, timeout: int = 15) -> bool:
    stdin, stdout, _ = client.exec_command(cmd, timeout=timeout)
    stdin.close()
    return stdout.channel.recv_exit_status() == 0


def check_tools(client, label: str, silent: bool = False,
                remote_dir: str = REMOTE_DIR, refjdk_root: str = REFJDK_ROOT) -> dict[str, bool]:
    checks = {
        **_TOOL_CHECKS_FIXED,
        "rava":    f"test -d {remote_dir}/.git",
        "uv_sync": f"test -d {remote_dir}/.venv",
        # 参考 JDK 按 main 检出里的清单判定（抽查 / 作业另在各自检出后按其清单确保）
        "refjdk":  f"cd {remote_dir} && RAVA_REFJDK_ROOT={refjdk_root} scripts/fetch_reference_jdk.sh --check >/dev/null 2>&1",
        "graalvm": _graalvm_check(refjdk_root),
    }
    result = {name: _exec_check(client, cmd) for name, cmd in checks.items()}
    if not silent:
        with _print_lock:
            print(f"\n[{label}] 环境检查：")
            for name, ok in result.items():
                print(f"  {'✅' if ok else '❌'} {name}")
    return result


def is_fully_ready(status: dict[str, bool]) -> bool:
    return all(ok for name, ok in status.items() if name not in _OPTIONAL)


# ── --check-only 专用：并行检查后输出表格 ────────────────────────────────────

def check_all_table(servers: list[dict]) -> None:
    """并行检查所有服务器，将结果渲染为对齐表格后一次性打印。"""
    # {label: dict[tool, bool] | None}  None 表示连接失败
    results: dict[str, dict[str, bool] | None] = {}
    threads = []

    def _run(s):
        label = s["label"]
        client, _ = _connect_with_retry(s)
        if not client:
            results[label] = None
            return
        try:
            results[label] = check_tools(client, label, silent=True,
                                         remote_dir=s.get("remote_dir", REMOTE_DIR),
                                         refjdk_root=config.refjdk_root(s))
        except Exception:
            results[label] = None
        finally:
            client.close()

    for s in servers:
        t = threading.Thread(target=_run, args=(s,))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()

    # ── 渲染表格 ──────────────────────────────────────────────────────────────
    # 列头宽度：最长列名 + 2 空白
    col_widths = {c: max(len(c), 4) + 1 for c in _TABLE_COLS}
    lbl_w = max(len(s["label"]) for s in servers) + 1

    header = f"  {'服务器':<{lbl_w}}" + "".join(f"  {c:^{col_widths[c]}}" for c in _TABLE_COLS) + "  整体"
    sep    = "  " + "─" * (len(header) - 2)

    print(f"\n{'='*len(header)}")
    print("  环境检查汇总（--check-only）")
    print(sep)
    print(header)
    print(sep)

    ok_icon   = "✅"
    fail_icon = "❌"
    na_icon   = "??"

    for s in servers:
        label  = s["label"]
        status = results.get(label)
        if status is None:
            cols = "".join(f"  {na_icon:^{col_widths[c]}}" for c in _TABLE_COLS)
            overall = "❌ 连接失败"
        else:
            cols = "".join(
                f"  {(ok_icon if status.get(c, False) else fail_icon):^{col_widths[c]}}"
                for c in _TABLE_COLS
            )
            overall = "✅ 就绪" if is_fully_ready(status) else "⚠️  缺失"
        print(f"  {label:<{lbl_w}}{cols}  {overall}")

    print(sep)
    # 汇总：哪些工具在哪些服务器上缺失
    missing: dict[str, list[str]] = {}
    for s in servers:
        label  = s["label"]
        status = results.get(label)
        if status is None:
            missing.setdefault("连接失败", []).append(label)
            continue
        for col in _TABLE_COLS:
            if not status.get(col, False):
                missing.setdefault(col, []).append(label)

    if missing:
        print("\n  缺失明细：")
        for tool, labels in sorted(missing.items()):
            print(f"    {tool:<10} → {', '.join(labels)}")
    else:
        print("\n  所有服务器环境均就绪 ✅")
    print("=" * len(header))


# ── 安装步骤 ──────────────────────────────────────────────────────────────────

def _notify(label: str, text: str, on_status=None):
    """输出步骤信息：有回调时调用回调，否则打印到终端。"""
    if on_status:
        on_status(label, text)
    else:
        with _print_lock:
            print(f"[{label}] {text}")


def install_system_deps(client, label: str, missing_cc: bool,
                        missing_java21: bool, missing_java25: bool = False,
                        missing_mvn: bool = False, missing_zlib: bool = False, on_status=None):
    """安装 build-essential / OpenJDK 21 / OpenJDK 25 / Maven / zlib1g-dev（一条 apt 命令完成）。"""
    pkgs = []
    if missing_cc:
        pkgs.append("build-essential")
    if missing_java21:
        pkgs.append("openjdk-21-jdk")
    if missing_java25:
        pkgs.append("openjdk-25-jdk")
    if missing_mvn:
        pkgs.append("maven")
    if missing_zlib:
        pkgs.append("zlib1g-dev")
    if not pkgs:
        return
    pkg_str = " ".join(pkgs)
    _notify(label, f"apt install {pkg_str}", on_status)
    run_command(
        client,
        f"sudo apt-get update -qq && sudo apt-get install -y -q {pkg_str}",
        timeout=300,
    )


def _graalvm_check(refjdk_root: str) -> str:
    """GraalVM 就位判定：native-image 存在且版本与 GRAALVM_VERSION 一致。"""
    return (f"{refjdk_root}/{GRAALVM_DIR}/bin/native-image --version 2>/dev/null "
            f"| grep -q '^native-image {GRAALVM_VERSION} '")


def install_graalvm(client, label: str, refjdk_root: str, on_status=None):
    """Oracle GraalVM 落到 <refjdk_root>/<GRAALVM_DIR>：下载 → sha256 校验 → 解压到临时目录 → 原子换入。

    以 <refjdk_root>/.graalvm.lock 互斥（作业里的按需安装用同一把锁）；旧版本整目录换掉。"""
    _notify(label, f"安装 GraalVM {GRAALVM_VERSION} → {refjdk_root}/{GRAALVM_DIR}", on_status)
    g = f"{refjdk_root}/{GRAALVM_DIR}"
    script = (
        f"set -e; mkdir -p {refjdk_root}; cd {refjdk_root}; exec 9>.graalvm.lock; flock 9; "
        f"if {_graalvm_check(refjdk_root)}; then exit 0; fi; "
        f"curl -fsSL -o .graalvm.tgz {GRAALVM_URL}; "
        f"echo '{GRAALVM_SHA256}  .graalvm.tgz' | sha256sum -c --quiet; "
        f"rm -rf {g}.tmp {g}.old; mkdir {g}.tmp; "
        f"tar xzf .graalvm.tgz -C {g}.tmp --strip-components=1; rm -f .graalvm.tgz; "
        f"[ -d {g} ] && mv {g} {g}.old; mv {g}.tmp {g}; rm -rf {g}.old"
    )
    run_command(client, f"bash -c {shlex.quote(script)}", timeout=900)


def install_rust(client, label: str, on_status=None):
    _notify(label, "安装 Rust/cargo", on_status)
    run_command(
        client,
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path",
        timeout=300,
    )


def install_uv(client, label: str, on_status=None):
    _notify(label, "安装 uv", on_status)
    run_command(
        client,
        "curl -LsSf https://astral.sh/uv/install.sh | sh",
        timeout=120,
    )


def sync_project(client, label: str, remote_dir: str = REMOTE_DIR, repo_url: str | None = REPO_URL, on_status=None):
    """git clone 或 git pull 项目，并 uv sync。repo_url 为克隆地址（服务器条目 repo_url 覆盖全局值）。"""
    has_git = _exec_check(client, f"test -d {remote_dir}/.git")
    if has_git:
        _notify(label, "git pull", on_status)
        run_command(client, f"cd {remote_dir} && git pull --ff-only", timeout=120)
    else:
        if not repo_url:
            raise RuntimeError(f"{CONFIG_PATH} 缺少 repo_url，无法在 {label} 上克隆项目")
        _notify(label, "git clone", on_status)
        run_command(client, f"git clone {repo_url} {remote_dir}", timeout=300)

    _notify(label, "uv sync", on_status)
    run_command(
        client,
        f"export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH; "
        f"cd {remote_dir} && uv sync",
        timeout=180,
    )


MIN_SWAP_GB = 8  # 低于此值时自动补足


def ensure_swap(client, label: str, min_gb: int = MIN_SWAP_GB, on_status=None):
    """检查 swap 总量，不足 min_gb 时在 Linux 服务器上自动创建 /swapfile。"""
    # 只在 Linux 上操作（macOS 不需要也不支持此方式）
    is_linux = _exec_check(client, "[ $(uname -s) = Linux ]")
    if not is_linux:
        return

    # fstab 去重：每次都修复（先删所有 /swapfile 行再追加一行）
    run_command(
        client,
        "sudo sed -i '/^\\/swapfile /d' /etc/fstab"
        " && echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab",
        timeout=10,
    )

    # /proc/swaps 里已有 /swapfile 说明 swap 已激活，不需要重新创建
    # 不用 free -k 比较大小：swap header 占用会导致报告值略小于 min_gb，引发误判
    if _exec_check(client, "grep -q /swapfile /proc/swaps"):
        return

    _notify(label, f"/swapfile 未激活，创建 {min_gb}GB swap", on_status)
    # 各步骤用 ; 分隔，独立执行，避免 && 链导致 || 误触发
    cmds = "; ".join([
        # 文件不存在才创建并格式化
        f"[ -f /swapfile ] || (sudo fallocate -l {min_gb}G /swapfile"
        f" && sudo chmod 600 /swapfile && sudo mkswap /swapfile)",
        # 激活（已激活则忽略错误）
        "sudo swapon /swapfile 2>/dev/null || true",
    ])
    run_command(client, cmds, timeout=60)


# ── 完整初始化流程 ─────────────────────────────────────────────────────────────

def setup_with_client(client, server: dict, on_status=None) -> bool:
    """
    在已建立的 SSH 连接上执行环境初始化。由 setup_server / server_worker 调用。
    项目目录取 server 的 remote_dir（支持每台服务器独立路径），参考 JDK 根目录取 refjdk_root。
    """
    label = server["label"]
    remote_dir = server.get("remote_dir", REMOTE_DIR)
    try:
        _notify(label, "检查工具", on_status)
        status = check_tools(client, label, remote_dir=remote_dir,
                             refjdk_root=config.refjdk_root(server))

        ensure_swap(client, label, on_status=on_status)

        if not all(status[k] for k in ("cc", "java21", "java25", "mvn", "zlib")):
            install_system_deps(
                client, label,
                missing_cc=not status["cc"],
                missing_java21=not status["java21"],
                missing_java25=not status["java25"],
                missing_mvn=not status["mvn"],
                missing_zlib=not status["zlib"],
                on_status=on_status,
            )

        if not status["cargo"]:
            install_rust(client, label, on_status=on_status)

        if not status["uv"]:
            install_uv(client, label, on_status=on_status)

        if not status["rava"] or not status["uv_sync"]:
            sync_project(client, label, remote_dir=remote_dir, repo_url=server.get("repo_url", REPO_URL), on_status=on_status)
        else:
            _notify(label, "git pull（更新代码）", on_status)
            run_command(client, f"cd {remote_dir} && git pull --ff-only", timeout=120)

        if not status["refjdk"]:
            _notify(label, f"取参考 JDK → {config.refjdk_root(server)}", on_status)
        jhome, err = ensure_reference_jdk(client, server, remote_dir)
        if err:
            _notify(label, f"⚠️ 参考 JDK 未能就位: {err}", on_status)
            return False

        if not status["graalvm"]:
            install_graalvm(client, label, config.refjdk_root(server), on_status=on_status)
            if not _exec_check(client, _graalvm_check(config.refjdk_root(server))):
                # 只影响 GraalVM 参照基线，不阻塞 e2e 就绪
                _notify(label, "⚠️ GraalVM 未能就位（参照基线作业会失败，e2e 不受影响）", on_status)

        _notify(label, "验证环境", on_status)
        ok = _exec_check(
            client,
            f"export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH; "
            f"cd {remote_dir} && uv run python3 -c 'import sys; print(sys.version)'",
            timeout=30,
        )
        if not ok:
            _notify(label, "⚠️ 验证失败", on_status)
            return False
        if not _exec_check(client, _TOOL_CHECKS_FIXED["memlimit"]):
            _notify(label, "⚠️ 无法建立内存受限 scope（systemd-run --user / cgroup memory 委派）", on_status)
            return False
        _notify(label, "✅ 就绪", on_status)
        return True

    except Exception as e:
        _notify(label, f"❌ 异常: {e}", on_status)
        return False


def setup_server(server: dict, check_only: bool = False, on_status=None) -> bool:
    """独立调用入口（env_setup.py 命令行 / check-only 用）。"""
    label = server["label"]
    _notify(label, f"连接 {server['host']}", on_status)
    client, _ = _connect_with_retry(server)
    if not client:
        _notify(label, "❌ 连接失败", on_status)
        return False
    try:
        remote_dir = server.get("remote_dir", REMOTE_DIR)
        if check_only:
            return is_fully_ready(check_tools(client, label, remote_dir=remote_dir,
                                              refjdk_root=config.refjdk_root(server)))
        return setup_with_client(client, server, on_status=on_status)
    finally:
        client.close()


def setup_all(servers: list[dict], check_only: bool = False,
              on_status=None) -> dict[str, bool]:
    """并行初始化多台服务器，返回 {label: ready}。"""
    results: dict[str, bool] = {}
    threads = []

    def _run(s):
        results[s["label"]] = setup_server(s, check_only=check_only,
                                           on_status=on_status)

    for s in servers:
        t = threading.Thread(target=_run, args=(s,))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()
    return results


# ── 命令行入口 ─────────────────────────────────────────────────────────────────

def main():
    try:
        _main()
    except KeyboardInterrupt:
        # 通知仍在重试连接的线程立即放弃，否则要等完整个退避周期才退出
        _shutdown.set()
        print("\n⏹ 已中断")


def _main():
    ap = argparse.ArgumentParser(description="初始化 rava 服务器环境")
    ap.add_argument("--servers", nargs="+", metavar="LABEL",
                    help="只初始化指定服务器（如 kr2 sg1）")
    ap.add_argument("--check-only", action="store_true",
                    help="只检查工具状态，不安装任何东西")
    ap.add_argument("--dry-run", action="store_true",
                    help="显示将要操作的服务器，不实际连接")
    args = ap.parse_args()

    servers = SERVERS
    if args.servers:
        labels  = set(args.servers)
        servers = [s for s in SERVERS if s["label"] in labels]
        unknown = labels - {s["label"] for s in servers}
        if unknown:
            print(f"⚠️  未知服务器: {unknown}")

    if args.dry_run:
        print("将初始化：")
        for s in servers:
            print(f"  {s['label']}  {s['host']}")
        return

    if args.check_only:
        check_all_table(servers)
        return

    results = setup_all(servers)

    print(f"\n{'='*50}")
    print("初始化汇总：")
    for label, ok in sorted(results.items()):
        print(f"  {'✅' if ok else '❌'} {label}")


if __name__ == "__main__":
    main()
