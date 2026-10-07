"""合入守护（单次扫描）：抽查 → 已知失败判定 → 合并集成分支 → 本地闸门 → 推送 → 主仓快进 main。

由协调者定时 / 循环驱动（--once），不常驻。队列与字段见 merge_queue.py；动作日志追加到
<结果目录>/merge_daemon.log，每条的闸门 / 抽查输出在 test_results/merge/<id>/。

  uv run --group cluster python scripts/cluster/merge_daemon.py --once [--dry-run]            # 扫描一次
  uv run --group cluster python scripts/cluster/merge_daemon.py enqueue --branch c1d-t2 --sha <40位> --tests StockTrans HelloWorld \\
      --gate generator --summary "T2 序列化回调收窄" [--spot c1dt2-xxxxxxxx] [--no-keep-branch]
  uv run --group cluster python scripts/cluster/merge_daemon.py status                        # 打印队列
  uv run --group cluster python scripts/cluster/merge_daemon.py retry <id> [--reset-spot-failed]   # blocked → queued

每条的处理：
  1. 抽查：名单为空只允许 doc-only；抽查进程不在跑且结果不全 → 发起 / 重发（断线、infra 失败后续跑，
     至多 MAX_SPOT_LAUNCHES 次）；在跑 → 等下一轮；完成 → 按 java_rta docs/known_failures.toml 判定，
     有新失败（含同名签名不符）→ blocked 并写新失败摘要。
  2. 合并（--dry-run 时在独立演练 worktree 里做，不动集成分支、不推送）：集成 worktree 须在
     rust-closure-analyzer 上且无已跟踪改动；git merge -q --no-ff；冲突 → merge --abort，blocked。
  3. 闸门（doc-only 只核对改动文件全在文档内）：失败 → 校验 HEAD 正是本次合并后 reset --hard 回合并前，blocked。
  4. 推送 origin；失败记 push_pending，下轮重推（不回退）。
  5. 主仓 git merge -q --ff-only rust-closure-analyzer 并推 main 到两远端；失败记 main_pending，下轮重试。
  6. keep_branch = false 时 git branch -d 删本地分支引用（检出中 / 未合并则保留并记日志）。
只回退守护自己刚做的那次合并；提交信息不含任何署名行。
"""

import argparse
import fcntl
import os
import re
import shlex
import subprocess
import sys
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE))

import known_failures as kf          # noqa: E402
from merge_queue import Queue, DEFAULT_QUEUE, GATES, TERMINAL, now   # noqa: E402

REPO_ROOT = _HERE.parent.parent          # 本检出根目录（分发器以 scripts/cluster/ 相对路径调用）
from cluster_config import RESULTS_DIR  # noqa: E402
LOG_PATH = RESULTS_DIR / "merge_daemon.log"
from cluster_config import REPO_ROOT as MAIN_REPO  # noqa: E402
WS = MAIN_REPO.parent
INTEG_WT = WS / "java_rta_closure_wt"
DRYRUN_WT = WS / "java_rta_dryrun_wt"
HEAVY_LOCK = WS / "heavy_lock.py"
INTEG_BRANCH = "rust-closure-analyzer"
REMOTES = ("origin",)
MAX_SPOT_LAUNCHES = 4
GATE_TIMEOUT = 3 * 3600
DOC_PATHS = re.compile(r"^(docs/|[^/]+\.md$|.*/[^/]+\.md$)")
# 闸门判定：各步退出码全为 0，且输出没有这些行（与协调者手工闸门的 grep 一致）
GATE_BAD = re.compile(r"^error|test result: F|FAILED|panicked", re.M)


# ── 基础 ────────────────────────────────────────────────────────────────────────

class Ctx:
    def __init__(self, args):
        self.dry = args.dry_run
        self.queue = Queue(Path(args.queue))
        self.integ = Path(args.integ_wt)
        self.main = Path(args.main_repo)
        self.dry_wt = Path(args.dry_run_wt)
        self.known_path = args.known
        self.log_path = Path(args.log)
        self.results = Path(args.results_dir)

    def log(self, entry_id: str, msg: str):
        line = f"[{now()}]{' [dry-run]' if self.dry else ''} [{entry_id}] {msg}"
        print(line, flush=True)
        with self.log_path.open("a", encoding="utf-8") as fh:
            fh.write(line + "\n")

    def work_dir(self, entry_id: str) -> Path:
        d = self.results / "merge" / re.sub(r"[^A-Za-z0-9_.-]", "_", entry_id)
        d.mkdir(parents=True, exist_ok=True)
        return d


def git(repo: Path, *args, check: bool = False, timeout: int = 600) -> subprocess.CompletedProcess:
    r = subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True, timeout=timeout)
    if check and r.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} 失败（{r.returncode}）：{(r.stderr or r.stdout).strip()[-400:]}")
    return r


def head(repo: Path) -> str:
    return git(repo, "rev-parse", "HEAD", check=True).stdout.strip()


# ── 抽查 ────────────────────────────────────────────────────────────────────────

def spot_procs(tag: str) -> list[int]:
    """正在跑 --spot <tag> 的 distribute_tests 进程（含 uv 包装层）。"""
    r = subprocess.run(["ps", "-ax", "-o", "pid=,command="], capture_output=True, text=True)
    pids = []
    for ln in r.stdout.splitlines():
        parts = ln.strip().split(None, 1)
        if len(parts) != 2 or "distribute_tests.py" not in parts[1]:
            continue
        toks = parts[1].split()
        if any(toks[i] == "--spot" and toks[i + 1] == tag for i in range(len(toks) - 1)):
            pids.append(int(parts[0]))
    return pids


def launch_spot(ctx: Ctx, e: dict) -> int:
    n = int(e.get("spot_launches", 0)) + 1
    out = ctx.work_dir(e["id"]) / f"spot_{n}.out"
    cmd = ["uv", "run", "--group", "cluster", "python", "scripts/cluster/distribute_tests.py", "--no-monitor", "--skip-setup",
           "--spot", e["spot"], "--ref", e["sha"], "--per-dir", "0", "--tests", *e["tests"]]
    with out.open("w", encoding="utf-8") as fh:
        fh.write(f"# {now()} {' '.join(shlex.quote(c) for c in cmd)}\n")
        fh.flush()
        p = subprocess.Popen(cmd, cwd=REPO_ROOT, stdout=fh, stderr=subprocess.STDOUT,
                             stdin=subprocess.DEVNULL, start_new_session=True)
    return p.pid


def stale_records(ctx: Ctx, e: dict) -> list[str]:
    """spot tag 下已有结果却属于别的提交（tag 复用）的用例。"""
    import json
    p = ctx.results / "spot" / e["spot"] / "state_jdk21.json"
    if not p.exists():
        return []
    st = json.loads(p.read_text(encoding="utf-8"))
    bad = []
    for kind in ("passed", "failed"):
        for t, v in st.get(kind, {}).items():
            c = v.get("commit")
            if t in e["tests"] and c and c != "?" and not e["sha"].startswith(c) and not c.startswith(e["sha"]):
                bad.append(f"{t}@{c[:9]}")
    return bad


def merge_line(v: kf.Verdict) -> str:
    """合并信息里的抽查结果：8/9（StockTrans 为已知失败）。"""
    s = f"{len(v.passed)}/{len(v.tests)}"
    if v.known:
        s += f"（{'、'.join(sorted({k['test'] for k in v.known}))} 为已知失败）"
    return s


def advance_spot(ctx: Ctx, e: dict, known) -> kf.Verdict | None:
    """推进抽查；返回可合入时的判定，否则 None（已更新状态）。"""
    eid = e["id"]
    if not e.get("tests"):
        if e["gate"] != "doc-only":
            ctx.queue.update(eid, status="blocked", reason="用例名单为空：只有 doc-only 可免抽查")
            ctx.log(eid, "blocked：用例名单为空且闸门不是 doc-only")
            return None
        return kf.Verdict(e["spot"], [])
    stale = stale_records(ctx, e)
    if stale:
        ctx.queue.update(eid, status="blocked", reason=f"spot tag {e['spot']} 已有其他提交的结果：{', '.join(stale[:5])}")
        ctx.log(eid, f"blocked：spot tag 复用（{', '.join(stale[:5])}）")
        return None
    running = spot_procs(e["spot"])
    v = kf.classify(e["spot"], e["tests"], known[0], known[1], ctx.results)
    if running:
        if e.get("status") != "spot_running" or e.get("verdict") != v.line():
            ctx.queue.update(eid, status="spot_running", verdict=v.line())
            ctx.log(eid, f"抽查进行中（pid {running}）：{v.line()}")
        return None
    if v.complete:
        if v.new:
            summ = [f"{n['test']}: {n['summary']}"[:400] for n in v.new]
            ctx.queue.update(eid, status="blocked", verdict=v.line(), new_failures=summ,
                             reason=f"抽查有新失败 {len(v.new)} 例（日志 {ctx.results / 'spot' / e['spot'] / 'error_logs'}）")
            ctx.log(eid, f"blocked：抽查新失败 {v.line()}")
            for s in summ:
                ctx.log(eid, f"  新失败 {s}")
            return None
        if e.get("verdict") != v.line():
            ctx.queue.update(eid, verdict=v.line())
            ctx.log(eid, f"抽查完成：{v.line()}（清单 {v.known_source}）")
        return v
    launches = int(e.get("spot_launches", 0))
    if launches >= MAX_SPOT_LAUNCHES:
        ctx.queue.update(eid, status="blocked", verdict=v.line(),
                         reason=f"抽查已发起 {launches} 次仍未完成（未出结果 {v.pending}，infra {v.infra}）")
        ctx.log(eid, f"blocked：抽查发起 {launches} 次仍未完成")
        return None
    pid = launch_spot(ctx, e)
    ctx.queue.update(eid, status="spot_running", spot_launches=launches + 1, spot_pid=pid, verdict=v.line())
    ctx.log(eid, f"{'发起' if launches == 0 else '重发'}抽查 {e['spot']}（第 {launches + 1} 次，pid {pid}，"
                 f"用例 {len(e['tests'])}，未出结果 {len(v.pending)}，infra {len(v.infra)}）")
    return None


# ── 闸门 ────────────────────────────────────────────────────────────────────────

def gate_steps(gate: str) -> list[tuple[str, str]]:
    steps = [
        ("driver", "cargo build --release -q -p driver --manifest-path generator/Cargo.toml "
                   "--target-dir build/analyzer-target"),
        ("generator-test", "cd generator && cargo test --release -q --target-dir ../build/analyzer-target"),
    ]
    if "macros_core" in gate:
        steps.append(("macros_core-test", "cd runtime/rava_macros_core && "
                                          "cargo test --release -q --target-dir ../../build/analyzer-target"))
    if "rava_coro" in gate:
        steps.append(("rava_coro-test", "cd runtime/rava_coro && cargo test --release -q "
                                        "--target-dir ../../build/coro-target -- --test-threads=1"))
    return steps


def run_gate(ctx: Ctx, e: dict, wt: Path, base: str) -> tuple[bool, str]:
    """返回 (干净, 原因)。doc-only 只核对改动文件。"""
    if e["gate"] == "doc-only":
        files = git(wt, "diff", "--name-only", base, "HEAD", check=True).stdout.split()
        bad = [f for f in files if not DOC_PATHS.match(f)]
        if bad:
            return False, f"doc-only 但改动了非文档文件：{', '.join(bad[:8])}"
        return True, f"doc-only：{len(files)} 个文档文件"
    # 各步在一次 heavy_lock 内顺序执行，每步输出后打印退出码标记；全部跑完才判定
    script = "; ".join(f"( {cmd} ) 2>&1; echo \"__GATE_STEP__ {name} rc=$?\"" for name, cmd in gate_steps(e["gate"]))
    cmd = ["python3", str(HEAVY_LOCK), "bash", "-c", script]
    log = ctx.work_dir(e["id"]) / "gate.log"
    env = dict(os.environ, CARGO_BUILD_JOBS="2", CARGO_TERM_COLOR="never")
    try:
        r = subprocess.run(cmd, cwd=wt, capture_output=True, text=True, env=env, timeout=GATE_TIMEOUT)
        out = r.stdout + r.stderr
    except subprocess.TimeoutExpired as ex:
        out = (ex.stdout or b"").decode(errors="replace") if isinstance(ex.stdout, bytes) else (ex.stdout or "")
        log.write_text(out + "\n[gate] 超时\n", encoding="utf-8")
        return False, f"闸门超时（{GATE_TIMEOUT}s），日志 {log}"
    log.write_text(f"# {now()} cwd={wt}\n# {script}\n\n{out}", encoding="utf-8")
    marks = dict(re.findall(r"^__GATE_STEP__ (\S+) rc=(\d+)$", out, re.M))
    expect = [n for n, _ in gate_steps(e["gate"])]
    missing = [n for n in expect if n not in marks]
    failed = [f"{n} rc={rc}" for n, rc in marks.items() if rc != "0"]
    bad = [ln for ln in out.splitlines() if GATE_BAD.search(ln) and not ln.startswith("__GATE_STEP__")]
    if not missing and not failed and not bad:
        return True, f"闸门 {e['gate']} 干净（{', '.join(expect)}）"
    why = []
    if missing:
        why.append(f"未跑完：{', '.join(missing)}（heavy_lock rc={r.returncode}）")
    if failed:
        why.append(f"失败步骤：{', '.join(failed)}")
    if bad:
        why.append("输出：" + " | ".join(b.strip()[:200] for b in bad[:8]))
    return False, "；".join(why) + f"；日志 {log}"


# ── 合并 ────────────────────────────────────────────────────────────────────────

def ensure_commit(repo: Path, e: dict) -> bool:
    if git(repo, "cat-file", "-e", f"{e['sha']}^{{commit}}").returncode == 0:
        return True
    for remote in REMOTES:
        git(repo, "fetch", "-q", remote, e["branch"], timeout=300)
        if git(repo, "cat-file", "-e", f"{e['sha']}^{{commit}}").returncode == 0:
            return True
    return False


def integ_ready(ctx: Ctx) -> str | None:
    """集成 worktree 前置检查，返回问题描述（None = 就绪）。"""
    br = git(ctx.integ, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip()
    if br != INTEG_BRANCH:
        return f"集成 worktree {ctx.integ} 当前在 {br or '分离头'}，不是 {INTEG_BRANCH}"
    gd = Path(git(ctx.integ, "rev-parse", "--absolute-git-dir", check=True).stdout.strip())
    if (gd / "MERGE_HEAD").exists():
        return "集成 worktree 有未完成的合并（MERGE_HEAD）"
    dirty = git(ctx.integ, "status", "--porcelain", "--untracked-files=no").stdout.strip()
    if dirty:
        return f"集成 worktree 有未提交的已跟踪改动：{dirty.splitlines()[:5]}"
    return None


def prepare_dry_wt(ctx: Ctx, base: str) -> Path:
    """演练 worktree（守护独占）：分离头对齐集成分支当前 HEAD，丢弃上次演练留下的合并。"""
    wt = ctx.dry_wt
    if not (wt / ".git").exists():
        git(ctx.integ, "worktree", "add", "-q", "--detach", str(wt), base, check=True)
    else:
        git(wt, "merge", "--abort")
        git(wt, "reset", "-q", "--hard", check=True)
        git(wt, "checkout", "-q", "--detach", base, check=True)
    return wt


def rollback(ctx: Ctx, wt: Path, eid: str, pre: str, sha: str) -> str:
    """只回退本次合并：有 MERGE_HEAD → merge --abort；HEAD 为 pre 与 sha 的合并 → reset --hard pre。"""
    gd = Path(git(wt, "rev-parse", "--absolute-git-dir").stdout.strip())
    if (gd / "MERGE_HEAD").exists():
        git(wt, "merge", "--abort")
        return "merge --abort"
    cur = head(wt)
    if cur == pre:
        return "无需回退"
    p1 = git(wt, "rev-parse", "HEAD^1").stdout.strip()
    p2 = git(wt, "rev-parse", "-q", "--verify", "HEAD^2").stdout.strip()
    if p1 == pre and p2 == sha:
        git(wt, "reset", "-q", "--hard", pre, check=True)
        return f"reset --hard {pre[:9]}"
    ctx.log(eid, f"⚠ 回退中止：HEAD {cur[:9]} 不是本次合并（父 {p1[:9]} / {p2[:9]}），未动集成分支")
    return "未回退（HEAD 不是本次合并）"


def push_integ(ctx: Ctx, eid: str) -> str | None:
    for r in REMOTES:
        p = git(ctx.integ, "push", "-q", r, INTEG_BRANCH, timeout=300)
        if p.returncode != 0:
            return f"推送 {r} 失败：{(p.stderr or p.stdout).strip()[-300:]}"
    ctx.log(eid, f"已推送 {INTEG_BRANCH} → {' + '.join(REMOTES)}")
    return None


def ff_main(ctx: Ctx, eid: str) -> str | None:
    br = git(ctx.main, "symbolic-ref", "--short", "-q", "HEAD").stdout.strip()
    if br != "main":
        return f"主仓 {ctx.main} 当前在 {br or '分离头'}，不是 main"
    r = git(ctx.main, "merge", "-q", "--ff-only", INTEG_BRANCH)
    if r.returncode != 0:
        return f"主仓快进失败：{(r.stderr or r.stdout).strip()[-300:]}"
    for rm in REMOTES:
        p = git(ctx.main, "push", "-q", rm, "main", timeout=300)
        if p.returncode != 0:
            return f"推送 main → {rm} 失败：{(p.stderr or p.stdout).strip()[-300:]}"
    ctx.log(eid, f"主仓 main 快进到 {head(ctx.main)[:9]} 并推送 {' + '.join(REMOTES)}")
    return None


def finish(ctx: Ctx, e: dict, merge_commit: str):
    eid = e["id"]
    err = push_integ(ctx, eid)
    if err:
        ctx.queue.update(eid, status="push_pending", merge_commit=merge_commit, reason=err)
        ctx.log(eid, f"push_pending：{err}")
        return
    err = ff_main(ctx, eid)
    if err:
        ctx.queue.update(eid, status="main_pending", merge_commit=merge_commit, reason=err)
        ctx.log(eid, f"main_pending：{err}")
        return
    if not e.get("keep_branch", True):
        d = git(ctx.integ, "branch", "-d", e["branch"])
        ctx.log(eid, f"删除本地分支 {e['branch']}：" + ("完成" if d.returncode == 0 else
                                                       f"保留（{(d.stderr or d.stdout).strip()[:200]}）"))
    ctx.queue.update(eid, status="merged", merge_commit=merge_commit, reason=None)
    ctx.log(eid, f"✅ merged {merge_commit[:9]}")


def merge_entry(ctx: Ctx, e: dict, v: kf.Verdict):
    eid, sha = e["id"], e["sha"]
    problem = integ_ready(ctx)
    if problem:
        # 集成 worktree 状态不对不是本条目的问题：保持 ready，下轮再试
        ctx.queue.update(eid, status="ready", reason=f"等待：{problem}")
        ctx.log(eid, f"暂缓合并：{problem}")
        return
    if not ensure_commit(ctx.integ, e):
        ctx.queue.update(eid, status="blocked", reason=f"本地与 {'/'.join(REMOTES)} 都找不到提交 {sha}")
        ctx.log(eid, f"blocked：找不到提交 {sha}")
        return
    base = head(ctx.integ)
    if git(ctx.integ, "merge-base", "--is-ancestor", sha, base).returncode == 0:
        if ctx.dry:
            ctx.queue.update(eid, status="dry_run_ok", reason=f"{sha[:9]} 已在 {INTEG_BRANCH}（无需合并）")
        else:
            ctx.queue.update(eid, status="merged", merge_commit=base, reason="已在集成分支中，未新建合并")
        ctx.log(eid, f"{sha[:9]} 已是 {INTEG_BRANCH} 的祖先，跳过合并")
        return
    wt = prepare_dry_wt(ctx, base) if ctx.dry else ctx.integ
    spot_part = f"抽查 {e['spot']} {merge_line(v)}" if v.tests else "免抽查（doc-only）"
    msg = (f"Merge {e['branch']}（{sha[:8]}）into {INTEG_BRANCH}："
           + (e.get("summary") or "").strip().rstrip("；。") + f"；{spot_part}")
    ctx.queue.update(eid, status="ready", reason=None)
    ctx.log(eid, f"合并 {sha[:9]} 到 {'演练 worktree ' + str(wt) if ctx.dry else INTEG_BRANCH}（基 {base[:9]}）")
    m = git(wt, "merge", "-q", "--no-ff", sha, "-m", msg)
    if m.returncode != 0:
        conflicts = git(wt, "diff", "--name-only", "--diff-filter=U").stdout.split()
        how = rollback(ctx, wt, eid, base, sha)
        reason = (f"合并冲突：{', '.join(conflicts[:10])}" if conflicts
                  else f"合并失败：{(m.stderr or m.stdout).strip()[-300:]}") + f"（已 {how}）"
        ctx.queue.update(eid, status="blocked", reason=reason)
        ctx.log(eid, f"blocked：{reason}")
        return
    merged = head(wt)
    ctx.log(eid, f"已合并 {merged[:9]}，跑闸门 {e['gate']}")
    ok, why = run_gate(ctx, e, wt, base)
    if not ok:
        how = rollback(ctx, wt, eid, base, sha)
        ctx.queue.update(eid, status="blocked", reason=f"闸门不干净：{why}（已 {how}）")
        ctx.log(eid, f"blocked：闸门不干净（已 {how}）：{why}")
        return
    ctx.log(eid, why)
    if ctx.dry:
        ctx.queue.update(eid, status="dry_run_ok", merge_commit=merged,
                         reason=f"演练通过：将推送 {INTEG_BRANCH} → {'+'.join(REMOTES)}，主仓 main 快进")
        ctx.log(eid, f"演练通过（演练合并 {merged[:9]} 留在 {wt}，集成分支与远端未动）")
        return
    finish(ctx, e, merged)


# ── 扫描 ────────────────────────────────────────────────────────────────────────

def scan(ctx: Ctx) -> int:
    try:
        known = kf.load_known(ctx.known_path)
    except Exception as ex:
        ctx.log("-", f"读取已知失败清单失败，本轮不处理：{ex}")
        return 3
    entries = ctx.queue.snapshot()
    active = [e for e in entries if e.get("status") not in TERMINAL
              and not (ctx.dry and e.get("status") == "dry_run_ok")]
    if not active:
        ctx.log("-", f"队列无待处理条目（共 {len(entries)} 条）")
        return 0
    for e in active:
        eid = e["id"]
        try:
            if e["gate"] not in GATES:
                ctx.queue.update(eid, status="blocked", reason=f"未知闸门类型 {e['gate']}（可选 {', '.join(GATES)}）")
                ctx.log(eid, f"blocked：未知闸门 {e['gate']}")
                continue
            st = e.get("status")
            if st == "push_pending" and not ctx.dry:
                finish(ctx, e, e.get("merge_commit", ""))
                continue
            if st == "main_pending" and not ctx.dry:
                err = ff_main(ctx, eid)
                if err:
                    ctx.queue.update(eid, reason=err)
                    ctx.log(eid, f"main_pending：{err}")
                else:
                    ctx.queue.update(eid, status="merged", reason=None)
                    ctx.log(eid, f"✅ merged {e.get('merge_commit', '')[:9]}（main 补快进）")
                continue
            v = advance_spot(ctx, e, known)
            if v is None:
                continue
            merge_entry(ctx, e, v)
        except Exception as ex:
            ctx.queue.update(eid, status="blocked", reason=f"守护异常：{type(ex).__name__}: {str(ex)[:300]}")
            ctx.log(eid, f"blocked：守护异常 {type(ex).__name__}: {ex}")
    counts: dict[str, int] = {}
    for e in ctx.queue.snapshot():
        counts[e.get("status", "?")] = counts.get(e.get("status", "?"), 0) + 1
    ctx.log("-", "扫描完成：" + "，".join(f"{k} {v}" for k, v in sorted(counts.items())))
    return 0


# ── 命令行 ──────────────────────────────────────────────────────────────────────

def cmd_enqueue(ctx: Ctx, a) -> int:
    if a.gate not in GATES:
        print(f"--gate 须为 {', '.join(GATES)}", file=sys.stderr)
        return 2
    r = git(ctx.integ, "rev-parse", "--verify", "-q", f"{a.sha}^{{commit}}")
    sha = r.stdout.strip() if r.returncode == 0 else a.sha
    if not re.fullmatch(r"[0-9a-f]{40}", sha):
        print(f"--sha {a.sha} 无法解析为完整 40 位哈希（先推送并 fetch）", file=sys.stderr)
        return 2
    spot = a.spot or f"{re.sub(r'[^A-Za-z0-9]', '', a.branch)[:12]}-{sha[:8]}"
    entry = {"id": a.id or spot, "branch": a.branch, "sha": sha, "spot": spot, "tests": a.tests or [],
             "gate": a.gate, "keep_branch": not a.no_keep_branch, "summary": a.summary or "",
             "submitted_by": a.by or ""}
    try:
        e = ctx.queue.append(entry)
    except ValueError as ex:
        print(ex, file=sys.stderr)
        return 2
    ctx.log(e["id"], f"入队：{a.branch} {sha[:9]} spot={spot} 用例 {len(entry['tests'])} 闸门 {a.gate}"
                     f"{'' if entry['keep_branch'] else ' 合入后删分支'}（{a.by or '?'}）")
    return 0


def cmd_status(ctx: Ctx) -> int:
    for e in ctx.queue.snapshot():
        print(f"{e.get('status', '?'):13} {e['id']:28} {e['branch']:20} {e['sha'][:9]}  {e.get('verdict', '')}"
              + (f"\n{'':14}原因：{e['reason']}" if e.get("reason") else "")
              + "".join(f"\n{'':14}新失败：{n}" for n in e.get("new_failures", [])))
    return 0


def cmd_retry(ctx: Ctx, a) -> int:
    e = next((x for x in ctx.queue.snapshot() if x["id"] == a.id), None)
    if e is None:
        print(f"无条目 {a.id}", file=sys.stderr)
        return 2
    if a.reset_spot_failed:
        subprocess.run(["uv", "run", "--group", "cluster", "python", "scripts/cluster/distribute_tests.py", "--spot", e["spot"],
                        "--ref", e["sha"], "--per-dir", "0", "--reset-failed"], cwd=REPO_ROOT, check=False)
    ctx.queue.update(a.id, status="queued", reason=None, new_failures=None, spot_launches=0)
    ctx.log(a.id, "retry：重新入队" + ("（已清抽查失败记录）" if a.reset_spot_failed else ""))
    return 0


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description="rava 合入守护（单次扫描）")
    ap.add_argument("--once", action="store_true", help="扫描队列一次（守护本体）")
    ap.add_argument("--dry-run", action="store_true", help="演练：合并与闸门在演练 worktree 里做，不动集成分支、不推送")
    ap.add_argument("--queue", default=str(DEFAULT_QUEUE))
    ap.add_argument("--integ-wt", default=str(INTEG_WT))
    ap.add_argument("--main-repo", default=str(MAIN_REPO))
    ap.add_argument("--dry-run-wt", default=str(DRYRUN_WT))
    ap.add_argument("--known", help="known_failures.toml 路径（缺省读 java_rta 集成分支上的版本）")
    ap.add_argument("--log", default=str(LOG_PATH))
    ap.add_argument("--results-dir", default=str(RESULTS_DIR))
    sub = ap.add_subparsers(dest="sub")
    q = sub.add_parser("enqueue", help="追加待合入条目")
    q.add_argument("--branch", required=True)
    q.add_argument("--sha", required=True)
    q.add_argument("--tests", nargs="*")
    q.add_argument("--gate", required=True, choices=GATES)
    q.add_argument("--spot")
    q.add_argument("--id")
    q.add_argument("--summary")
    q.add_argument("--by")
    q.add_argument("--no-keep-branch", action="store_true")
    sub.add_parser("status", help="打印队列")
    rt = sub.add_parser("retry", help="blocked 条目重新入队")
    rt.add_argument("id")
    rt.add_argument("--reset-spot-failed", action="store_true", help="同时清掉该抽查的失败记录（重跑失败用例）")
    a = ap.parse_args(argv)
    ctx = Ctx(a)
    if a.sub == "enqueue":
        return cmd_enqueue(ctx, a)
    if a.sub == "status":
        return cmd_status(ctx)
    if a.sub == "retry":
        return cmd_retry(ctx, a)
    if not a.once:
        ap.error("需要 --once（或子命令 enqueue / status / retry）")
    # 单实例：同一时刻只有一个扫描在改集成分支
    lock = Path(a.results_dir) / "merge_daemon.lock"
    lock.parent.mkdir(parents=True, exist_ok=True)
    fd = os.open(lock, os.O_RDWR | os.O_CREAT, 0o644)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("另一个 merge_daemon 扫描正在进行，本次退出", file=sys.stderr)
        return 0
    try:
        return scan(ctx)
    finally:
        os.close(fd)


if __name__ == "__main__":
    sys.exit(main())
