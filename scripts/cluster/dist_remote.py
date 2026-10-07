"""远端操作：资源检查、内存上限、检出（main / 抽查 / 作业独立目录）、语料参考 JDK 就位、git 哈希。
SSH 建连（直连 → 跳板 → 代理）在 dist_conn，此处转出 _connect_with_retry 供各模块沿用。"""

import re
from pathlib import Path
import shlex

import cluster_config as config
from cluster_config import CPU_LOAD_LIMIT, MIN_RAM_MB, MEM_RESERVE_GB, MIN_SLOT_MEM_GB, MEM_SLICE
from dist_conn import _connect_with_retry  # noqa: F401  转出


def resolve_spot_ref(ref: str) -> str:
    """短哈希在本地 java_rta 仓库解析为完整 40 位哈希（服务器端 git fetch 只认分支名或完整哈希）；
    分支名与完整哈希原样返回。本地无法唯一解析时报错退出。"""
    if not re.fullmatch(r"[0-9a-fA-F]{4,39}", ref):
        return ref
    repo = config.LOCAL_E2E_DIR.parent.parent if config.LOCAL_E2E_DIR else None
    if repo is None:
        raise SystemExit(f"--ref {ref} 是短哈希，但找不到本地 java_rta 仓库来解析；请改用完整哈希或分支名")
    import subprocess
    r = subprocess.run(["git", "-C", str(repo), "rev-parse", "--verify", "--quiet", f"{ref}^{{commit}}"],
                       capture_output=True, text=True)
    full = r.stdout.strip()
    if r.returncode != 0 or not full:
        raise SystemExit(f"--ref {ref} 在本地仓库 {repo} 中无法解析为提交；请改用完整哈希或分支名")
    return full


def drop_missing_tests(ref: str, names: list[str]) -> tuple[list[str], list[str]]:
    """按 ref 的 tests/e2e 树过滤抽查名单：master_passed 与 --tests 可能含已剪枝的用例（如 acc13430
    冗余剪枝删掉的 184 例），派出去只会得到「No test files found」。返回 (保留, 丢弃)；
    本地仓库无法解析 ref（含 origin/<ref>）时原样保留。"""
    repo = config.LOCAL_E2E_DIR.parent.parent if config.LOCAL_E2E_DIR else None
    if repo is None:
        return names, []
    import subprocess
    for cand in (ref, f"origin/{ref}"):
        r = subprocess.run(["git", "-C", str(repo), "ls-tree", "-r", "--name-only", cand, "--", "tests/e2e"],
                           capture_output=True, text=True)
        if r.returncode == 0 and r.stdout.strip():
            stems = {Path(l).stem for l in r.stdout.splitlines() if l.endswith(".java")}
            return [n for n in names if n in stems], [n for n in names if n not in stems]
    return names, []


def spot_remote_dir(remote_dir: str, tag: str) -> str:
    """抽查 / 作业的检出目录：每个 tag 独立（多个实例可同时运行不同 ref），跑完即删。
    编译缓存不跨目录共享：不同 ref 共用 analyzer-target 曾在 ubuntu 上编出错误的 rava（E0432）。"""
    return f"{remote_dir.rstrip('/')}-spot-{re.sub(r'[^A-Za-z0-9_.-]', '_', tag)}"


def job_remote_dir(remote_dir: str, tag: str, slot: int = 0) -> str:
    """作业检出目录：每槽一个（作业命令会改检出内容，槽间不能共用）；槽 0 沿用单槽时的目录名。"""
    return spot_remote_dir(remote_dir, f"job-{tag}" + (f"-s{slot}" if slot else ""))


def remove_checkout(client, work_dir: str):
    """删除抽查 / 作业的独立检出目录。"""
    try:
        stdin, stdout, _ = client.exec_command(f"rm -rf {work_dir}", timeout=300)
        stdin.close()
        stdout.channel.recv_exit_status()
    except Exception:
        pass


def prepare_spot_checkout(client, base_dir: str, spot_dir: str, ref: str) -> str | None:
    """在 spot_dir 检出 origin 上的 ref（分支或提交），首次从 base_dir 本地克隆以复用对象。
    返回错误信息；成功返回 None。spot_dir 由抽查模式独占，检出时强制对齐远端。"""
    q = ref.replace("'", "")
    cmd = (
        "export PATH=$HOME/.local/bin:$HOME/.cargo/bin:$PATH; set -e; "
        f"if [ ! -d {spot_dir}/.git ]; then "
        f"git clone -q {base_dir} {spot_dir}; "
        f"git -C {spot_dir} remote set-url origin $(git -C {base_dir} remote get-url origin); fi; "
        f"cd {spot_dir}; git fetch -q origin '{q}'; git checkout -q -f -B spot FETCH_HEAD; mkdir -p build; "
        "uv sync -q"
    )
    try:
        stdin, stdout, stderr = client.exec_command(cmd, timeout=600)
        stdin.close()
        rc = stdout.channel.recv_exit_status()
        if rc != 0:
            return (stderr.read().decode(errors="replace") or f"exit {rc}").strip()[-300:]
        return None
    except Exception as e:
        return str(e)


def prepare_main_checkout(client, remote_dir: str) -> str | None:
    """强制检出 origin/main 最新提交，并删掉旧版 --record-passed 留下的远端通过清单
    （会让 run_tests 跳过测试）。返回错误信息；成功返回 None。"""
    # 用 find 而非 rm 通配：zsh 下未匹配的通配符直接报错，会中断后续命令
    cmd = (
        f"cd {remote_dir} && "
        "{ find build -maxdepth 1 -name 'passed_tests_jdk*.txt' -delete 2>/dev/null; true; } && "
        "git fetch -q origin && git checkout -q -B main origin/main"
    )
    try:
        stdin, stdout, stderr = client.exec_command(cmd, timeout=120)
        stdin.close()
        rc = stdout.channel.recv_exit_status()
        if rc != 0:
            return (stderr.read().decode(errors="replace") or f"exit {rc}").strip()[-300:]
        return None
    except Exception as e:
        return str(e)


# ── 语料参考 JDK（java_rta tools/refjdk.toml） ───────────────────────────────

NO_REFJDK_MARK = "__RAVA_NO_REFJDK_SCRIPT__"
REFJDK_TIMEOUT = 1200   # 首次下载约 200MB + 解压；已就位时为空操作


def refjdk_env(server: dict) -> str:
    """shell 前缀：导出参考 JDK 根目录，run_tests.py / 语料脚本经 fetch_reference_jdk.sh --check 取 JAVA_HOME。"""
    return f"export RAVA_REFJDK_ROOT={shlex.quote(config.refjdk_root(server))}; "


def ensure_reference_jdk(client, server: dict, work_dir: str) -> tuple[str | None, str | None]:
    """在检出目录 work_dir 里执行 scripts/fetch_reference_jdk.sh（清单随被测提交走），幂等确保参考 JDK
    落在数据目录 <refjdk_root>/<tag>/。返回 (JAVA_HOME, 错误)；该提交无取包脚本（参考 JDK 之前的旧提交）
    时返回 (None, None)，由 run_tests 按旧逻辑选 JDK。只写数据目录，不动系统 JDK。"""
    cmd = (
        refjdk_env(server)
        + f"cd {shlex.quote(work_dir)} && "
        f"if [ -x scripts/fetch_reference_jdk.sh ]; then scripts/fetch_reference_jdk.sh; "
        f"else echo {NO_REFJDK_MARK}; fi"
    )
    try:
        stdin, stdout, stderr = client.exec_command(cmd, timeout=REFJDK_TIMEOUT)
        stdin.close()
        out = stdout.read().decode(errors="replace").strip()
        err = stderr.read().decode(errors="replace").strip()
        rc = stdout.channel.recv_exit_status()
        if rc != 0:
            return None, (err or f"exit {rc}")[-300:]
        if out == NO_REFJDK_MARK:
            return None, None
        return out.splitlines()[-1] if out else None, None
    except Exception as e:
        return None, str(e)


def _fetch_git_hash(client, remote_dir: str, short: bool = True) -> str:
    """读取远端服务器项目的 git 哈希，失败返回 '?'。"""
    try:
        stdin, stdout, _ = client.exec_command(
            f"git -C {remote_dir} rev-parse {'--short ' if short else ''}HEAD", timeout=5
        )
        stdin.close()
        return stdout.read().decode().strip() or "?"
    except Exception:
        return "?"


# ── 内存上限（user systemd scope + cgroup v2） ─────────────────────────────────

OOM_MARK         = "__RAVA_OOM_KILL__"
NO_MEMLIMIT_MARK = "__RAVA_NO_MEMLIMIT__"
NO_MEMLIMIT_RC   = 76

# 在 scope 内运行：先核对上限确已生效（memory 控制器未委派时 MemoryMax 会被静默忽略），
# （systemd ≥ 254 的 systemd-run 会展开命令行里的 ${...}，包装层不用 ${var%...} 之类的写法）；
# 执行命令后读 memory.events，上限内发生过 OOM kill 则输出 OOM_MARK 行；退出码透传
_SCOPE_WRAPPER = (
    "exec 2>&1; f=/sys/fs/cgroup$(sed -n 's/^0:://p' /proc/self/cgroup); "
    "if [ \"$(cat $f/memory.max 2>/dev/null)\" != \"$((RAVA_LIM * 1048576))\" ] "
    "|| [ \"$(cat $f/memory.swap.max 2>/dev/null)\" != 0 ] "
    "|| [ \"$(cat $(dirname $f)/memory.max 2>/dev/null)\" != \"$((RAVA_TOTAL * 1048576))\" ]; then "
    f"echo {NO_MEMLIMIT_MARK} memory.max 未生效; exit {NO_MEMLIMIT_RC}; fi; "
    "bash -c \"$RAVA_CMD\"; rc=$?; "
    "k=$(awk '$1==\"oom_kill\"{print $2}' $f/memory.events); "
    "if [ \"${k:-0}\" -gt 0 ]; then "
    f"echo \"{OOM_MARK} limit=${{RAVA_LIM}}M peak=$(( $(cat $f/memory.peak 2>/dev/null || echo 0) / 1048576 ))M\"; fi; "
    "exit $rc"
)


def mem_reserve_gb(server: dict) -> int:
    return server.get("mem_reserve_gb", MEM_RESERVE_GB)


def mem_limited(cmd: str, server: dict) -> str:
    """shell 片段：在 user systemd scope 内执行 cmd（bash 语法），禁用 swap；子进程一并计入。
    两级上限：
    - 全部槽的 scope 都放在 user 级 MEM_SLICE 下，slice 的 MemoryMax = 总内存 − 预留，同机各槽合计不越过它；
    - 每槽 scope 的 MemoryMax = max((总内存 − 预留) / 槽数, MIN_SLOT_MEM_GB)，允许超分——多数测试峰值远低于
      均分额，单例峰值（约 12G）仍放得下；合计吃紧时由 slice 内的 OOM kill 落到某个测试上（标 OOM_MARK，
      属资源类失败），不波及服务器上其他服务。
    无法建立受限 slice / scope 时输出 NO_MEMLIMIT_MARK 并以 NO_MEMLIMIT_RC 退出，不在无上限的状态下运行。"""
    reserve_mb = mem_reserve_gb(server) * 1024
    slots = server.get("slots", 1)
    total = f"$(awk '/MemTotal/{{print int($2/1024)}}' /proc/meminfo) - {reserve_mb}"
    # 多槽：关掉 rustc 包装（如 ~/.cargo/config.toml 的 rustc-wrapper = "sccache"）——sccache 服务进程常驻在
    # 首个启动它的 scope 里，经它编译的 rustc 不计入本槽 scope，每槽内存上限对编译形同虚设
    split = "" if slots == 1 else (
        f"[ $RAVA_LIM -lt {MIN_SLOT_MEM_GB * 1024} ] && RAVA_LIM={MIN_SLOT_MEM_GB * 1024}; "
        "export RUSTC_WRAPPER=; ")
    return (
        (f"RAVA_TOTAL=$(( {total} )); RAVA_LIM=$RAVA_TOTAL; " if slots == 1
         else f"RAVA_TOTAL=$(( {total} )); RAVA_LIM=$(( RAVA_TOTAL / {slots} )); ") + split +
        # 总量 slice：--runtime 写运行期 drop-in，重启后由下一次派发重新设置
        f"systemctl --user set-property --runtime {MEM_SLICE} MemoryMax=${{RAVA_TOTAL}}M MemorySwapMax=0 "
        f">/dev/null 2>&1 || {{ echo {NO_MEMLIMIT_MARK} {MEM_SLICE} 总量上限设置失败; exit {NO_MEMLIMIT_RC}; }}; "
        # systemd ≥ 253 的 scope 默认 OOMPolicy=stop：一个进程被 OOM kill 就停掉整个 scope（连同包装层）；
        # 更早的版本不认此属性，也没有该行为。写成单个参数：zsh 不对未加引号的变量分词
        "RAVA_OOMP=--property=OOMPolicy=continue; "
        f"systemd-run --user --scope --quiet --slice={MEM_SLICE} $RAVA_OOMP -p MemoryMax=64M true >/dev/null 2>&1 || RAVA_OOMP=; "
        f"if ! systemd-run --user --scope --quiet --slice={MEM_SLICE} $RAVA_OOMP -p MemoryMax=64M -p MemorySwapMax=0 true >/dev/null 2>&1; then "
        f"echo {NO_MEMLIMIT_MARK} systemd-run --user --scope 不可用; exit {NO_MEMLIMIT_RC}; fi; "
        f"export RAVA_LIM RAVA_TOTAL RAVA_CMD={shlex.quote(cmd)}; "
        f"exec systemd-run --user --scope --quiet --slice={MEM_SLICE} $RAVA_OOMP -p MemoryMax=${{RAVA_LIM}}M -p MemorySwapMax=0 "
        f"bash -c {shlex.quote(_SCOPE_WRAPPER)}"
    )


def slot_cargo_config(server: dict, out_root: str) -> str:
    """shell 片段（多槽 e2e）：在槽输出根 out_root 写 .cargo/config.toml，[build] jobs = 核数 / 槽数（至少 1）。
    cargo 从工作目录（scratch 在 out_root 之下）向上找配置，故只作用于本槽的 rava compile。
    用配置而非导出 CARGO_BUILD_JOBS：环境变量优先于配置，rava 对重型闭包（声明层 ≥1700 类）强制的
    CARGO_BUILD_JOBS=1 内存保护照常生效；导出环境变量则会让 rava 视为调用方指定而跳过该保护。
    各槽的 rustc 并行度之和因此不超过核数。"""
    q = shlex.quote(f"{out_root}/.cargo")
    n = server.get("slots", 1)
    return (
        f"mkdir -p {q} && RAVA_J=$(( $(getconf _NPROCESSORS_ONLN 2>/dev/null || echo {n}) / {n} )); "
        "[ $RAVA_J -ge 1 ] || RAVA_J=1; "
        f"printf '[build]\\njobs = %s\\n' $RAVA_J > {q}/config.toml; "
    )


def parse_oom_line(line: str) -> str | None:
    """OOM_MARK 行 → 'limit=…M peak=…M'；其他行返回 None。"""
    s = line.strip()
    return s[len(OOM_MARK):].strip() if s.startswith(OOM_MARK) else None


# ── 资源检查与分块计算 ─────────────────────────────────────────────────────────

def _check_resources(client) -> tuple[int, float, int, int] | None:
    """SSH 执行单条命令，返回 (free_ram_mb, load_per_cpu, disk_free_gb, ncpu)。兼容 Linux/macOS。"""
    cmd = (
        "OS=$(uname -s); "
        "if [ \"$OS\" = \"Darwin\" ]; then "
        "  NCPU=$(sysctl -n hw.logicalcpu 2>/dev/null || echo 1); "
        "  LOAD=$(sysctl -n vm.loadavg 2>/dev/null | awk '{print $2}'); "
        "  PAGE=$(sysctl -n hw.pagesize 2>/dev/null || echo 4096); "
        "  FREE_RAM=$(vm_stat 2>/dev/null | awk -v pg=$PAGE '"
        "/Pages free:/{gsub(/\\./,\"\",$3);f=$3}"
        "/Pages inactive:/{gsub(/\\./,\"\",$3);i=$3}"
        "/Pages speculative:/{gsub(/\\./,\"\",$3);s=$3}"
        "/Pages purgeable:/{gsub(/\\./,\"\",$3);p=$3}"
        "END{printf \"%d\",(f+i+s+p)*pg/1024/1024}'); "
        "  DISK=$(df -k / 2>/dev/null | awk 'NR==2{printf \"%d\",int($4/1048576)}'); "
        "else "
        "  NCPU=$(nproc); "
        "  LOAD=$(awk '{print $1}' /proc/loadavg); "
        "  FREE_RAM=$(awk '/MemAvailable/{printf \"%d\", $2/1024}' /proc/meminfo); "
        "  DISK=$(df -BG / | awk 'NR==2{print $4}' | tr -d G); "
        "fi; "
        "echo $NCPU; echo $LOAD; echo $FREE_RAM; echo $DISK"
    )
    try:
        stdin, stdout, _ = client.exec_command(cmd, timeout=15)
        stdin.close()
        lines = stdout.read().decode(errors="replace").strip().splitlines()
        if len(lines) < 4:
            return None
        ncpu         = int(lines[0])
        load_per_cpu = float(lines[1]) / max(ncpu, 1)
        free_ram_mb  = int(lines[2])
        disk_gb      = int(lines[3])
        return free_ram_mb, load_per_cpu, disk_gb, ncpu
    except Exception:
        return None


def load_limit(server: dict | None = None) -> float:
    """负载门槛（每核）：单槽为 CPU_LOAD_LIMIT；多槽时同机其他槽自身的负载也计在内，
    门槛按槽数放宽到 CPU_LOAD_LIMIT + (1 − CPU_LOAD_LIMIT)·(slots − 1)/slots（4 槽 0.9、8 槽 0.95），
    仍不超过每核 1.0。"""
    slots = (server or {}).get("slots", 1)
    return CPU_LOAD_LIMIT + (1 - CPU_LOAD_LIMIT) * (slots - 1) / slots


def min_free_ram_mb(server: dict | None = None) -> int:
    """空闲内存门槛：单槽为 MIN_RAM_MB；多槽时同机其他槽已占用的内存不在 MemAvailable 内，
    新开一槽要求空闲内存够它跑到每槽上限的下限 MIN_SLOT_MEM_GB（OOM 例峰值约 12G）。"""
    slots = (server or {}).get("slots", 1)
    return MIN_RAM_MB if slots == 1 else max(MIN_RAM_MB, MIN_SLOT_MEM_GB * 1024)


def _resource_ok(free_ram_mb: int, load_per_cpu: float, server: dict | None = None) -> bool:
    """判断当前资源是否允许本槽再开一个测试 / 作业。"""
    return load_per_cpu <= load_limit(server) and free_ram_mb >= min_free_ram_mb(server)
