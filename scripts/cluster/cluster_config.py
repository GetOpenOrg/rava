"""rava 集群测试（分布式全量 / 抽查 / 作业）共享配置。

服务器清单是本机配置，不入库：缺省读 ~/.config/rava/cluster.toml（环境变量 RAVA_CLUSTER_CONFIG 覆盖），
格式见同目录 cluster.example.toml。
"""

import os
import subprocess
import tomllib
from pathlib import Path

# ── 路径 ──────────────────────────────────────────────────────────────────────

_HERE        = Path(__file__).resolve().parent                # scripts/cluster/


def _main_repo_root() -> Path:
    """主检出根目录：各 worktree 共用主检出的结果目录与语料清单（git 公共目录的上级）。"""
    try:
        common = subprocess.run(["git", "-C", str(_HERE), "rev-parse", "--path-format=absolute", "--git-common-dir"],
                                capture_output=True, text=True, check=True).stdout.strip()
        return Path(common).parent
    except (OSError, subprocess.CalledProcessError):
        return _HERE.parent.parent


REPO_ROOT    = _main_repo_root()
CONFIG_PATH  = Path(os.environ.get("RAVA_CLUSTER_CONFIG", Path.home() / ".config" / "rava" / "cluster.toml"))
# 本地结果目录（主检出下，gitignore）；环境变量 RAVA_CLUSTER_RESULTS 覆盖
RESULTS_DIR  = Path(os.environ.get("RAVA_CLUSTER_RESULTS", REPO_ROOT / "cluster_results"))
REMOTE_DIR   = "/data/rava"                                   # 服务器项目目录（服务器条目 remote_dir 覆盖）
# 语料参考 JDK 根目录（tools/refjdk.toml 的固定构建落在 <根>/<tag>/；数据目录，不碰系统 JDK）。
# 服务器可用 refjdk_root 键覆盖（放在 remote_dir 所在盘）；抽查 / 作业的独立检出共用这一份
REFJDK_ROOT  = "/data/rava-jdk"
REPO_URL     = "https://github.com/GetOpenOrg/rava"           # 公开仓库

# GraalVM 参照基线（scripts/graalvm_bench.sh）：Oracle GraalVM 落到 <refjdk_root>/<GRAALVM_DIR>，
# 版本与本机 macOS 基线同版（native-image 21.0.12）；归档地址固定版本，sha256 取 Oracle 同名 .sha256
GRAALVM_DIR     = "graalvm-21"
GRAALVM_VERSION = "21.0.12"
GRAALVM_URL     = "https://download.oracle.com/graalvm/21/archive/graalvm-jdk-21.0.12_linux-x64_bin.tar.gz"
GRAALVM_SHA256  = "b007ff64c425f85bbe0e686107044fba6ca5054a7e89271a473767f546aaddc1"

# 主检出的 tests/e2e（用于枚举测试列表）
LOCAL_E2E_DIR = REPO_ROOT / "tests" / "e2e" if (REPO_ROOT / "tests" / "e2e").exists() else None

# ── 服务器列表（本机配置文件） ────────────────────────────────────────────────

# 条目键：label / host / port（缺省 22）/ username / private_key_path（~ 展开）/ remote_dir（缺省 REMOTE_DIR）；
# 可选键：skip_setup 跳过环境初始化；direct_only 只直连不降级走代理（内网服务器）；
#         mem_reserve_gb 覆盖 MEM_RESERVE_GB；
#         jump 同区跳板服务器标签：直连失败后先经跳板（direct-tcpip 通道）再降级走代理；
#         refjdk_root 覆盖 REFJDK_ROOT（参考 JDK 根目录，放在 remote_dir 所在的数据盘）；
#         slots 每台服务器同时跑的测试 / 作业数（缺省 1）。槽 i 的工作线程标签为 <label>#i（slots=1 时仍为
#         <label>），各持服务器端槽锁 slot_lock_path(i)（e2e 与作业共用），内存上限 = (总内存 − 预留) / slots，
#         rustc 并行度 = 核数 / slots（e2e 写槽输出根 .cargo/config.toml，见 dist_remote.slot_cargo_config；
#         作业为核数一半 / slots）；slots>1 时槽 i 的 run_tests 用 --out-dir build/slot<i>
#         （scratch / 编译缓存 / 日志按槽分开），作业按槽各用一个检出目录。
#         内存两级上限：同机全部槽合计 ≤ 总内存 − 预留（user 级 MEM_SLICE），每槽 = max(均分额, MIN_SLOT_MEM_GB)，
#         允许超分；槽数受 CPU 约束（rustc 并行度 = 核数 / 槽数，至少 1）
#         pools 不加 --servers 时默认参与的分发模式："test" = 全量 / 抽查，"job" = 作业（gate 等）；
#         未列入任何池的服务器（云服务器：跑着业务）只在 --servers 显式点名时使用


def _load_cluster(path: Path) -> tuple[list[dict], dict | None]:
    """读本机集群配置：返回 (服务器条目列表, 直连失败时的降级 SOCKS5 代理或 None)。文件缺失时清单为空。"""
    if not path.exists():
        return [], None
    data = tomllib.loads(path.read_text())
    servers = []
    for s in data.get("servers", []):
        s = {"port": 22, **s}
        if "private_key_path" in s:
            s["private_key_path"] = str(Path(s["private_key_path"]).expanduser())
        servers.append(s)
    return servers, data.get("fallback_proxy")


SERVERS, FALLBACK_PROXY = _load_cluster(CONFIG_PATH)

# ── 连接重试策略（2s → 5s → 稳定 10s，共 30 次） ─────────────────────────────

RETRY_WAITS      = [2, 5] + [10] * 28   # 2→5→10×28，共 30 次，累计等待约 4.8 分钟
MAX_RETRIES      = len(RETRY_WAITS)
CONNECT_TIMEOUT  = 10   # 单次 TCP / SOCKS5 建连超时（秒），每轮直连、代理各尝试一次


# ── 资源阈值 ──────────────────────────────────────────────────────────────────

CPU_LOAD_LIMIT  = 0.6    # load_avg_1m / nproc 超过此值时等待
MIN_RAM_MB      = 1500   # 空闲内存低于此值时等待
MIN_DISK_GB     = 5      # 磁盘剩余低于此值时退出
# 测试 / 作业进程内存上限 = 总内存 − 预留（cgroup MemoryMax，禁用 swap）；服务器可用 mem_reserve_gb 覆盖
MEM_RESERVE_GB  = 4
# 多槽服务器每槽内存上限的下限（GB）：均分额低于它时按它取（超分，合计由 MEM_SLICE 封顶，见 dist_remote.mem_limited）；
# 也是新开一槽要求的空闲内存（OOM 例单例峰值约 12G）
MIN_SLOT_MEM_GB = 14
MEM_SLICE       = "rava.slice"   # 测试 / 作业 scope 所在的 user slice，MemoryMax = 总内存 − 预留
WAIT_SECONDS    = 30     # 资源紧张时等待时间（秒）
MIN_CHUNK       = 5      # 每次最少取的测试数
MAX_CHUNK       = 20     # 每次最多取的测试数
TASK_TIMEOUT    = 1800   # 单个测试从发起到结束的上限（秒），超时杀掉远端进程记为失败

# ── 远端脱钩执行（remote_run） ────────────────────────────────────────────────

POLL_INTERVAL        = 12    # 轮询远端结果目录的间隔（秒）
MAX_INFRA_RETURNS    = 3     # 单个测试因网络 / 执行异常归还超过此次数即记为 infra 失败，不再派发
BREAKER_DISCONNECTS  = 3     # 服务器连续断线达此次数后冷却
BREAKER_COOLDOWN     = 600   # 冷却时长（秒），期间该服务器不领新任务
# 缺省 JDK 主版本 = java_rta 语料参考构建的主版本：run_tests 不传 --jdk 即用参考构建；
# 其他主版本（如 --jdk 25）传 --jdk N，属实验覆盖（run_tests 标记非参考构建）
DEFAULT_JDK     = 21


def refjdk_root(server: dict) -> str:
    """服务器上的参考 JDK 根目录（RAVA_REFJDK_ROOT）。"""
    return server.get("refjdk_root", REFJDK_ROOT)


# ── 多槽（服务器条目 slots 键） ───────────────────────────────────────────────

def server_slots(server: dict) -> int:
    """服务器的槽数（同时跑的测试 / 作业数，缺省 1）。"""
    return max(1, int(server.get("slots", 1)))


def pick_servers(labels: list[str] | None, mode: str) -> list[dict]:
    """分发用的服务器：点名了按点名取（物理机标签），否则取 pools 含 mode 的服务器（mode 为 "test" / "job"）。"""
    if labels:
        return [s for s in SERVERS if s["label"] in labels]
    return [s for s in SERVERS if mode in s.get("pools", ())]


def slot_servers(servers: list[dict]) -> list[dict]:
    """服务器条目 → 每槽一个工作条目：复制原条目，加 host_label（物理机标签）/ slot（槽号，0 起）/
    slots（槽数）；slots=1 时标签不变，slots>1 时标签为 <label>#<槽号>。状态、租约、日志按此标签区分槽。"""
    out = []
    for s in servers:
        n = server_slots(s)
        for i in range(n):
            out.append({**s, "label": s["label"] if n == 1 else f"{s['label']}#{i}",
                        "host_label": s["label"], "slot": i, "slots": n})
    return out


def host_label(server: dict) -> str:
    """工作条目所在物理机的标签（未经 slot_servers 展开的条目即其自身标签）。"""
    return server.get("host_label", server["label"])


def slot_lock_path(slot: int) -> str:
    """服务器端槽锁：e2e 与作业共用，一槽同一时刻只跑一个重任务。槽 0 沿用单槽时代的锁名，
    与仍在运行的旧版分发实例互斥。"""
    return "/tmp/rava_dist.lock" if slot == 0 else f"/tmp/rava_dist.slot{slot}.lock"
