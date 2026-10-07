"""
rava 分布式测试调度器（任务池模式）。

启动时自动完成环境初始化，无需手动 apt / git / uv 操作。
测试放入共享任务池，各服务器根据实时资源（CPU / 内存）动态取任务并行运行。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  全量跑批模式（默认）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  uv run --group cluster python scripts/cluster/distribute_tests.py                     # 自动 setup + 运行/续跑
  uv run --group cluster python scripts/cluster/distribute_tests.py --jdk 25            # 切换 JDK（默认 21）
  uv run --group cluster python scripts/cluster/distribute_tests.py --skip-setup        # 跳过环境初始化检查
  uv run --group cluster python scripts/cluster/distribute_tests.py --reset             # 归档全部进度，在最新代码上从头全量
  uv run --group cluster python scripts/cluster/distribute_tests.py --reset-failed      # 只清除失败记录，重新入队
  uv run --group cluster python scripts/cluster/distribute_tests.py --filter Approx     # 只运行名称含 Approx 的测试
  uv run --group cluster python scripts/cluster/distribute_tests.py --filter Bell Sort  # 多个子串，取并集
  uv run --group cluster python scripts/cluster/distribute_tests.py --status            # 查看当前进度
  uv run --group cluster python scripts/cluster/distribute_tests.py --merge             # 只合并通过清单，不运行测试
  uv run --group cluster python scripts/cluster/distribute_tests.py --no-monitor        # 禁用实时界面，只打印事件日志
  uv run --group cluster python scripts/cluster/distribute_tests.py --cleanup-servers   # 完成后删除各服务器项目目录

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  抽查模式（--spot）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  从 master_passed_jdk{N}.txt 按 tests/e2e/ 子目录分层抽样，在指定分支的独立检出
  目录（<remote_dir>-spot-<tag>，跑完即删）上运行，
  结果归档到 test_results/spot/<tag>/，多个抽查 / 作业实例可同时运行，
  与全量跑批的 state、master 通过清单、工作目录完全隔离。
  适用场景：验证新分支没有破坏已通过的测试，无需等待全量完成。

  uv run --group cluster python scripts/cluster/distribute_tests.py --spot v2-check --ref claude/my-branch
      # 每目录抽 2 个（默认），归档到 spot/v2-check/
  uv run --group cluster python scripts/cluster/distribute_tests.py --spot v2-check --ref claude/my-branch \\
      --per-dir 5 --seed 42
      # 每目录抽 5 个，固定种子 42（可复现）
  uv run --group cluster python scripts/cluster/distribute_tests.py --spot v2-check --ref abc1234 \\
      --tests SpecialTest1 SpecialTest2
      # 抽样基础上额外追加指定测试（精确类名）

  参数说明：
    --spot TAG       抽查标签，结果写入 test_results/spot/TAG/
    --ref BRANCH     要检出的 origin 分支或已推送的 commit hash（必填；短哈希在本地仓库解析为完整哈希）
    --per-dir N      每个子目录抽取的测试数（默认 2）
    --seed N         随机种子（默认 0，相同种子产生相同抽样）
    --tests NAME...  额外强制包含的测试类名（不受 per-dir 限制）
    --run-tests-args ARGS  追加到服务器 run_tests.py 的选项（全量同样适用），如放宽超时：
                     "--transpile-timeout 1800 --run-timeout 900 --build-timeout-scale 2"
    --task-timeout S 单例总时限秒数（缺省 1800；放宽 run_tests 超时时同步放宽，全量同样适用）

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  作业模式（--job）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  把 e2e 以外的重命令（rava audit、gen_trees / compare_trees、cargo test、seed_check 等）
  分发到服务器执行。每条 --cmd 一个作业，在 <remote_dir>-spot-job-<tag>（槽 i≥1 加 -s<i>）独立检出目录中
  运行；一个槽同一时刻只跑一个作业，以 nice 低优先级、CARGO_BUILD_JOBS=核数一半 / 槽数运行；开跑前检查
  负载 / 内存 / 磁盘；与 e2e 共用服务器端槽锁（槽 0 为 /tmp/rava_dist.lock），同一个槽上作业与测试不重叠。
  内存上限同 e2e（见下）；超限在 summary.json 记 oom（limit / peak）。

  uv run --group cluster python scripts/cluster/distribute_tests.py --job audit1 --ref 52bf5311 \
      --cmd "cargo build --release -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target && build/analyzer-target/release/rava audit api" \
      --fetch 'docs/reports/**/*.md'

  参数说明：
    --job TAG        作业标签，结果写入 test_results/job/TAG/（<序号>_<服务器>.log、summary.json、<序号>/ 产物）
    --ref REF        要检出的 origin 分支或提交（必填；短哈希在本地仓库解析）
    --cmd CMD        作业命令（可多次给出；在检出目录下以 bash 执行）
    --fetch GLOB     作业结束后取回的产物（相对检出目录，支持 **；可多次给出）
    --servers L...   只用这些服务器（默认全部）
    --min-ram MB     作业开跑所需空闲内存（默认 4000）
    --job-timeout S  单个作业超时秒数（默认 3600）
  同一 TAG 重跑时跳过已成功且命令未变的作业。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  内存上限（全量 / 抽查 / 作业）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  每个测试 / 作业在 user systemd scope 内运行，scope 都挂在 user 级 rava.slice 下：slice 的 MemoryMax =
  总内存 − MEM_RESERVE_GB（默认 4，服务器可用 mem_reserve_gb 覆盖），同机各槽合计不越过它；每槽 scope 的
  MemoryMax = max(均分额, MIN_SLOT_MEM_GB)（允许超分；服务器条目 slot_mem_gb 或 --slot-mem dev=28 覆盖下限，不超过合计），MemorySwapMax=0，子进程一并计入。超限只杀 scope 内进程，
  结果记为 OOM 失败（失败原因 / 失败日志头 / failed_tests 均写明服务器、上限、峰值）。
  服务器建不了受限 scope 时不在其上运行；env_setup.py --check-only 的 memlimit 列可查。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  多槽（服务器条目 slots 键，缺省 1；见 config.py 顶部说明）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  一台服务器同时跑 slots 个测试 / 作业，每槽一个工作线程，标签 <label>#<槽号>（状态表、租约、
  error_logs 文件名按此区分）；--servers 仍按物理机标签筛选。同机各槽共用一个检出目录，setup /
  清理残留 / 检出 / 参考 JDK 每台物理机只做一次（dist_e2e.HostGate）。槽 i 的 run_tests 用
  --out-dir build/slot<i>，scratch / 编译缓存 target / 日志按槽隔离；槽输出根写 .cargo/config.toml
  限 rustc 并行度为核数 / 槽数。收尾只删本测试的 scratch 与日志、只结束本槽的进程。
  负载门槛按槽数放宽（dist_remote.load_limit），新开一槽要求空闲内存 ≥ MIN_SLOT_MEM_GB。
  --slots dev=6 只对本次运行覆盖槽数（实测调槽数用）；--slot-mem dev=28 只对本次运行覆盖每槽内存下限
  （OOM 例放宽重跑用），新开一槽的空闲内存门槛随之提高。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  远端脱钩执行（全量 / 抽查 / 作业，见 remote_run.py）
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  测试 / 作业在远端 setsid nohup 启动，脱离 SSH 会话；输出与退出码写入检出目录下
  build/dist_runs/<run_key>/（out.log / pid / rc / done），本地每 POLL_INTERVAL 秒按偏移读增量。
  断线后重连、从原偏移续读，不重派；本地进程重启后凭 state 里的派发租约接上。
  远端确认已死且无 done 才归还任务；单测异常归还超过 MAX_INFRA_RETURNS 次记 infra 失败；
  服务器连续断线 BREAKER_DISCONNECTS 次冷却 BREAKER_COOLDOWN 秒。
  建连顺序：直连 → 同区跳板（服务器条目 jump 键）→ SOCKS5 代理。
  调试：RAVA_DIST_INJECT_DROP=N[:label] 在每次运行第 N 次轮询前强制断线（验证续读）。

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  结果目录：cluster_results/
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  state_jdk{N}.json          进度状态：每个已通过 / 已失败测试记录其运行时的提交；跨提交累计续跑，
                             服务器始终检出 main 最新提交，只有 --reset 才从头全量
  master_passed_jdk{N}.txt   累计通过清单（= state 的通过集，tests/e2e/ 路径，每通过一例即重写）
  by_commit/<commit>/        按运行时提交分目录的记录：passed_jdk{N}.txt / failed_jdk{N}.txt
  archive/reset_<时间>/      --reset 时归档的 state / 失败清单 / 失败日志 / master
  failed_tests_jdk{N}.txt    失败测试汇总（测试名 + 服务器 + 时间戳）
  infra_failed_jdk{N}.txt    infra 失败（网络 / 执行异常归还超过 MAX_INFRA_RETURNS 次，不计入失败；下轮重新入队）
  error_logs/                失败日志（每个测试一个文件，含元信息头 + 运行输出 +
                             build.log / run.log）
  spot/<tag>/                抽查模式的独立结果目录（结构同上）
  job/<tag>/                 作业模式的日志、产物与 summary.json
"""

import sys
import time
import signal
import logging
import threading
import argparse
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed

# 将本目录加入路径（以便 import cluster_config / env_setup / dist_*）
sys.path.insert(0, str(Path(__file__).parent))

# 屏蔽 Paramiko 的 WARNING/ERROR 日志（连接关闭时的噪音）
logging.getLogger("paramiko").setLevel(logging.CRITICAL)

from ssh_client import create_ssh_client
import cluster_config as config
from cluster_config import SERVERS, REMOTE_DIR, DEFAULT_JDK, pick_servers, slot_servers
import dist_ctx
from dist_ctx import _shutdown, _cleanup_handler, _log, _print_lock
from dist_monitor import _set_st, _monitor_loop, print_final_summary
from dist_remote import resolve_spot_ref, drop_missing_tests
from dist_state import (
    TaskPool, State, load_test_list, sample_passed, write_master, archive_for_reset,
)
from dist_e2e import server_worker
from dist_job import run_jobs
from dist_cleanup import start_background_cleanup


# ── 清理服务器项目 ─────────────────────────────────────────────────────────────

def cleanup_servers():
    _log(f"\n🗑  删除各服务器 {REMOTE_DIR}...")
    for server in SERVERS:
        client = create_ssh_client(server)
        if not client:
            _log(f"  [{server['label']}] 连接失败，跳过")
            continue
        try:
            client.exec_command(f"rm -rf {REMOTE_DIR}", timeout=60)
            _log(f"  [{server['label']}] ✅ 已清除")
        finally:
            client.close()


# ── 主入口 ────────────────────────────────────────────────────────────────────

def main():
    ap = argparse.ArgumentParser(description="rava 分布式测试调度器")
    ap.add_argument("--jdk",           type=int, default=DEFAULT_JDK)
    ap.add_argument("--e2e-dir",       type=str, default=None,
                    help="本地 tests/e2e 目录（默认自动查找）")
    ap.add_argument("--skip-setup",    action="store_true",
                    help="跳过环境初始化检查（已确认就绪时用）")
    ap.add_argument("--reset",         action="store_true",
                    help="归档全部进度，在 main 最新代码上从头全量（否则跨提交累计续跑）")
    ap.add_argument("--reset-failed",  action="store_true",
                    help="清除失败记录，让这些测试重新入队（保留已通过记录）")
    ap.add_argument("--filter",        nargs="+", metavar="NAME",
                    help="只运行名称含指定字符串的测试（子串匹配，可多个）")
    ap.add_argument("--status",        action="store_true")
    ap.add_argument("--merge",         action="store_true",
                    help="只合并通过清单，不运行测试")
    ap.add_argument("--cleanup-servers", action="store_true",
                    help="全部完成后删除各服务器 /data/rava")
    ap.add_argument("--no-monitor",    action="store_true",
                    help="禁用实时监控界面，只打印事件日志")
    ap.add_argument("--spot",          metavar="TAG",
                    help="抽查模式：结果归档到 test_results/spot/TAG/，服务器用独立检出目录")
    ap.add_argument("--ref",           metavar="BRANCH_OR_SHA",
                    help="抽查模式下检出的 origin 分支或提交（须已推送）")
    ap.add_argument("--per-dir",       type=int, default=2,
                    help="抽查模式：每个 tests/e2e 目录从已通过清单抽取的个数（默认 2）")
    ap.add_argument("--seed",          type=int, default=0,
                    help="抽查模式：抽样随机种子（默认 0，可复现）")
    ap.add_argument("--tests",         nargs="+", metavar="NAME",
                    help="抽查模式：额外追加的测试名（精确类名，如 TestTernary）")
    ap.add_argument("--run-tests-args", metavar="ARGS", default="",
                    help="全量 / 抽查：原样追加到服务器 run_tests.py 命令行的选项（如 "
                         "'--transpile-timeout 1800 --run-timeout 900 --build-timeout-scale 2'；"
                         "须为所检出提交的 run_tests.py 认得的选项）")
    ap.add_argument("--task-timeout",  type=int, default=None, metavar="S",
                    help=f"全量 / 抽查：单个测试从发起到结束的总时限秒数（缺省 {config.TASK_TIMEOUT}；"
                         "放宽 run_tests 超时时须同步放宽）")
    ap.add_argument("--job",           metavar="TAG",
                    help="作业模式：分发 --cmd 命令到服务器，结果写入 test_results/job/TAG/")
    ap.add_argument("--cmd",           action="append", default=[], metavar="CMD",
                    help="作业模式：作业命令（可多次）")
    ap.add_argument("--fetch",         action="append", default=[], metavar="GLOB",
                    help="作业模式：取回的产物通配（相对检出目录，可多次）")
    ap.add_argument("--servers",       nargs="+", metavar="LABEL",
                    help="只用这些服务器（作业 / 全量 / 抽查模式均适用；缺省按 config pools：全量 / 抽查用 test 池、作业用 job 池）")
    ap.add_argument("--slots",         nargs="+", metavar="LABEL=N",
                    help="本次运行覆盖服务器槽数（如 dev=6；实测调槽数用，不改 config）")
    ap.add_argument("--slot-mem",      nargs="+", metavar="LABEL=GB",
                    help="本次运行覆盖多槽服务器每槽内存上限的下限（如 dev=28；OOM 例放宽重跑用，不改 config）")
    ap.add_argument("--min-ram",       type=int, default=4000,
                    help="作业模式：开跑所需空闲内存 MB（默认 4000）")
    ap.add_argument("--job-timeout",   type=int, default=3600,
                    help="作业模式：单个作业超时秒数（默认 3600）")
    ap.add_argument("--no-cleanup",    action="store_true",
                    help="启动时不清理服务器上过期的检出目录与 /tmp 条目（见 dist_cleanup.py）")
    args = ap.parse_args()
    for spec in args.slots or []:
        lbl, _, n = spec.partition("=")
        srv = next((x for x in SERVERS if x["label"] == lbl), None)
        if srv is None or not n.isdigit() or int(n) < 1:
            ap.error(f"--slots {spec}：需为 <服务器标签>=<正整数>")
        srv["slots"] = int(n)
    for spec in args.slot_mem or []:
        lbl, _, n = spec.partition("=")
        srv = next((x for x in SERVERS if x["label"] == lbl), None)
        if srv is None or not n.isdigit() or int(n) < 1:
            ap.error(f"--slot-mem {spec}：需为 <服务器标签>=<正整数 GB>")
        srv["slot_mem_gb"] = int(n)

    # 启动清理：后台删除 24h 未改动且无进程在用的 *-spot-* 检出与 /tmp 条目（服务器端 1 小时节流）
    if not (args.no_cleanup or args.status or args.merge):
        start_background_cleanup(args.servers)

    if args.job:
        if not args.ref or not args.cmd:
            ap.error("--job 需要 --ref 与至少一条 --cmd")
        if args.e2e_dir:
            config.LOCAL_E2E_DIR = Path(args.e2e_dir)
        ref = resolve_spot_ref(args.ref)
        if ref != args.ref:
            _log(f"--ref {args.ref} → {ref}")
        run_jobs(args.job, ref, args.cmd, args.fetch, args.servers, args.min_ram, args.job_timeout)
        return

    if args.task_timeout is not None and args.task_timeout < 60:
        ap.error("--task-timeout 至少 60 秒")
    dist_ctx.run_tests_args = args.run_tests_args.strip()
    dist_ctx.task_timeout = args.task_timeout
    if dist_ctx.run_tests_args or dist_ctx.task_timeout:
        _log(f"run_tests 追加选项: {dist_ctx.run_tests_args or '(无)'}；"
             f"单例总时限 {dist_ctx.task_timeout or config.TASK_TIMEOUT}s")

    spot_tests: list[str] | None = None
    if args.spot:
        if not args.ref:
            ap.error("--spot 需要 --ref 指定要检出的分支或提交")
        resolved = resolve_spot_ref(args.ref)
        if resolved != args.ref:
            _log(f"--ref {args.ref} → {resolved}")
            args.ref = resolved
        spot_tests = sorted(set(sample_passed(args.jdk, args.per_dir, args.seed)) | set(args.tests or ()))
        spot_tests, missing = drop_missing_tests(args.ref, spot_tests)
        if missing:
            _log(f"⚠️ {args.ref} 树中不存在、已从抽查名单剔除 {len(missing)} 例: {' '.join(missing)}")
        dist_ctx.results_dir = dist_ctx.results_dir / "spot" / args.spot

    if args.e2e_dir:
        config.LOCAL_E2E_DIR = Path(args.e2e_dir)

    signal.signal(signal.SIGINT,  _cleanup_handler)
    signal.signal(signal.SIGTERM, _cleanup_handler)

    dist_ctx.results_dir.mkdir(parents=True, exist_ok=True)
    state_path = dist_ctx.results_dir / f"state_jdk{args.jdk}.json"

    if args.reset:
        archive_for_reset(state_path, args.jdk)
        _log(f"🔄 JDK {args.jdk} 进度已重置，本轮在 main 最新代码上从头全量")

    state = State(state_path)

    if args.reset_failed:
        failed_tests = dict(state.data.get("failed", {}))
        n = len(failed_tests)
        state.data["failed"] = {}
        state._save()

        # 删除对应的 error_logs 文件
        error_log_dir = dist_ctx.results_dir / "error_logs"
        removed_logs = 0
        for test in failed_tests:
            for p in error_log_dir.glob(f"{test}_*_jdk{args.jdk}.log"):
                p.unlink()
                removed_logs += 1

        # 清空失败汇总文件
        fail_list = dist_ctx.results_dir / f"failed_tests_jdk{args.jdk}.txt"
        if fail_list.exists():
            fail_list.unlink()

        _log(f"🔄 已清除 {n} 条失败记录、{removed_logs} 个日志文件、failed_tests_jdk{args.jdk}.txt，下次运行将重新调度")
        return
    state.set_jdk(args.jdk)

    if args.status:
        s = state.summary()
        try:
            total = len(load_test_list())
        except Exception:
            total = "?"
        print(f"\n  通过提交分布: {state.commit_counts()}")
        print(f"  总测试: {total}  通过: {s['passed']}  失败: {s['failed']}  "
              f"未运行: {int(total or 0) - s['passed'] - s['failed'] if isinstance(total, int) else '?'}")
        return

    if args.merge:
        _log(f"📋 通过清单: {write_master(state)}  共 {len(state.completed_set())} 个测试")
        return

    # ── 立即预初始化状态 + 启动监控（让状态表从一开始就可见）────────────────────
    # --servers 按物理机标签筛选（缺省取 config pools 含 "test" 的服务器），再按槽展开（config slots）：
    # 每槽一个工作线程、状态表一行
    servers_used = slot_servers(pick_servers(args.servers, "test"))
    dist_ctx.slot_servers = servers_used
    for s in servers_used:
        _set_st(s["label"], status="connecting")

    stop_event = threading.Event()
    if not args.no_monitor:
        threading.Thread(
            target=_monitor_loop, args=(stop_event,), daemon=True
        ).start()

    # ── 初始化任务池（setup 已移入 server_worker，无需在此等待）──────────────
    all_tests = spot_tests if spot_tests is not None else load_test_list()
    if spot_tests is not None:
        _log(f"抽查 {len(all_tests)} 个测试（ref={args.ref}，每目录 {args.per_dir}，seed={args.seed}）")
    if args.filter:
        all_tests = [t for t in all_tests if any(f in t for f in args.filter)]
        if not all_tests:
            _log(f"⚠️  --filter {args.filter} 无匹配测试，退出")
            return
        _log(f"--filter 匹配到 {len(all_tests)} 个测试: {all_tests}")
    completed = state.completed_set()
    failed    = state.failed_set()
    n_infra   = state.requeue_infra()
    if n_infra:
        _log(f"上轮 infra 失败 {n_infra} 个（网络 / 执行异常），本轮重新入队")
    # 派发租约：原服务器的 worker 启动后先接上远端运行；服务器不在本次范围内的租约作废、测试回池
    labels = {s["label"] for s in servers_used}
    leases = state.leases()
    for lbl in [l for l in leases if l not in labels]:
        state.clear_lease(lbl)
    leased = {v["test"] for l, v in leases.items() if l in labels}
    if leased:
        _log(f"接上上次派发的 {len(leased)} 个运行: {sorted(leased)}")
    pool      = TaskPool(all_tests, completed, failed, leased)

    # pool/state 就绪后更新全局引用，监控切换到完整任务显示
    dist_ctx.pool  = pool
    dist_ctx.state = state

    if pool.remaining() == 0 and not leased:
        _log("✅ 所有测试均已完成！")
        print_final_summary(state, pool)
        write_master(state)
        stop_event.set()
        return

    log_dir = dist_ctx.results_dir / "logs"
    log_dir.mkdir(parents=True, exist_ok=True)

    # ── 并行启动所有服务器工作线程（每台独立完成 setup 后立即开始测试）──────
    try:
        with ThreadPoolExecutor(max_workers=len(servers_used)) as executor:
            futures = [
                executor.submit(server_worker, s, pool, args.jdk, state, log_dir,
                                args.skip_setup, args.ref if args.spot else None, args.spot)
                for s in servers_used
            ]
            for f in as_completed(futures):
                pass
    except Exception:
        pass  # shutdown 期间的任何异常静默忽略

    stop_event.set()
    time.sleep(0.3)

    with _print_lock:
        sys.__stdout__.write("\033[H\033[J")
        sys.__stdout__.flush()

    if _shutdown.is_set():
        s = state.summary()
        _log(
            f"\n已停止  JDK {s['jdk']}  "
            f"通过 {s['passed']}  失败 {s['failed']}  "
            f"剩余 {pool.remaining()}/{pool.total()} 未运行\n"
            f"进度已保存，下次运行将从断点续跑。"
        )
        return

    print_final_summary(state, pool)
    write_master(state)

    if args.cleanup_servers:
        cleanup_servers()


if __name__ == "__main__":
    main()
