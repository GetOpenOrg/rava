"""任务池、进度状态与结果归档（state / master 通过清单 / by_commit / reset 归档）。"""

import json
import random
import shutil
import threading
from collections import deque
from datetime import datetime
from pathlib import Path

import cluster_config as config
from cluster_config import DEFAULT_JDK
import dist_ctx
from dist_ctx import _log, _print_lock


# ── 测试列表枚举 ──────────────────────────────────────────────────────────────

def load_test_list() -> list[str]:
    """从本地 tests/e2e/ 枚举测试（按路径排序，与服务器端顺序一致）。"""
    if config.LOCAL_E2E_DIR is None:
        raise FileNotFoundError(
            "找不到本地 tests/e2e 目录。\n"
            "请用 --e2e-dir /path/to/tests/e2e 指定（缺省取主检出的 tests/e2e）"
        )
    return sorted(f.stem for f in config.LOCAL_E2E_DIR.rglob("*.java"))


# ── 抽查模式（--spot）─────────────────────────────────────────────────────────
# 全量 600+ / 1000+ 测试要跑数天；抽查模式按目录分层抽取已通过清单，在各服务器的独立检出目录
# （<remote_dir>-spot-<tag>）上运行指定分支，结果归档到 test_results/spot/<tag>/，不影响全量跑批的
# 检出目录、state 与 master 通过清单。

def sample_passed(jdk: int, per_dir: int, seed: int) -> list[str]:
    """从 master_passed_jdk{N}.txt 按 tests/e2e/<目录> 分层抽样，每目录至多 per_dir 个（固定种子可复现）。"""
    if per_dir <= 0:
        return []
    master = dist_ctx.results_dir / f"master_passed_jdk{jdk}.txt"
    groups: dict[str, list[str]] = {}
    for line in master.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        p = Path(line)
        groups.setdefault(p.parent.name, []).append(p.stem)
    rng = random.Random(seed)
    picked = []
    for d in sorted(groups):
        names = sorted(set(groups[d]))
        picked += rng.sample(names, min(per_dir, len(names)))
    return sorted(picked)


# ── 任务池 ─────────────────────────────────────────────────────────────────────

class TaskPool:
    def __init__(self, all_tests: list[str], completed: set[str], failed: set[str],
                 leased: set[str] = frozenset()):
        self._lock  = threading.Lock()
        # 失败的测试不自动重试：从池中排除，须手动清除 state.json 中的记录后才会重新入队
        # 持有派发租约的测试由原服务器的 worker 先接上，不进公共池
        pending     = [t for t in all_tests
                       if t not in completed and t not in failed and t not in leased]
        self._deque = deque(pending)
        self._total = len(all_tests)

    def pop(self, n: int) -> list[str]:
        with self._lock:
            chunk = []
            for _ in range(min(n, len(self._deque))):
                chunk.append(self._deque.popleft())
            return chunk

    def push_back(self, tests: list[str]):
        with self._lock:
            self._deque.extendleft(reversed(tests))

    def remaining(self) -> int:
        with self._lock:
            return len(self._deque)

    def total(self) -> int:
        return self._total


# ── 状态持久化 ─────────────────────────────────────────────────────────────────

class State:
    """进度状态。passed / failed 按测试记录运行时的提交，跨提交累计（不随 main 前进而清零）。"""

    def __init__(self, path: Path):
        self.path  = path
        self._lock = threading.Lock()
        self._migrated: dict[str, list[str]] = {}
        self.data  = self._load()
        for commit, tests in self._migrated.items():
            record_by_commit(self.data.get("jdk", DEFAULT_JDK), commit, "passed", tests)
        if self._migrated:
            self._save()

    def _load(self):
        data = None
        if self.path.exists():
            try:
                data = json.loads(self.path.read_text(encoding="utf-8"))
            except Exception:
                pass
        if not data:
            data = {"jdk": DEFAULT_JDK}
        # 旧格式迁移：completed 列表（+ 可选的全局 commit）→ passed 按测试记提交
        if "completed" in data:
            commit = data.pop("commit", None)
            data["passed"] = {t: {"commit": commit} for t in data.pop("completed")}
            self._migrated[commit] = sorted(data["passed"])
        data.pop("commit", None)
        data.setdefault("passed", {})
        data.setdefault("failed", {})
        # 派发租约：服务器标签 → {test, run_key, run_dir, work_dir, commit, start}；本地重启后据此接上远端运行
        data.setdefault("leases", {})
        # 因网络 / 执行异常归还的次数（本轮），以及超过上限记为 infra 失败的测试（不计入 failed）
        data.setdefault("infra_returns", {})
        data.setdefault("infra_failed", {})
        return data

    def _save(self):
        tmp = self.path.with_suffix(".tmp")
        tmp.write_text(json.dumps(self.data, indent=2, ensure_ascii=False), encoding="utf-8")
        tmp.replace(self.path)

    def mark_passed(self, tests: list[str], server: str, commit: str, timing: dict | None = None):
        now = datetime.now().isoformat(timespec="seconds")
        with self._lock:
            for t in tests:
                self.data["passed"][t] = {"commit": commit, "server": server, "time": now}
                if timing:
                    self.data["passed"][t]["timing"] = timing
                self.data["failed"].pop(t, None)
            self._save()
        record_by_commit(self.data["jdk"], commit, "passed", tests)
        write_master(self)
        if timing:
            write_timings(self)

    def mark_failed(self, tests: list[str], server: str, error: str, commit: str,
                    timing: dict | None = None):
        with self._lock:
            for t in tests:
                prev = self.data["failed"].get(t, {"attempts": 0})
                self.data["failed"][t] = {
                    "server":   server,
                    "attempts": prev["attempts"] + 1,
                    "error":    str(error)[:200],
                    "commit":   commit,
                }
                if timing:
                    self.data["failed"][t]["timing"] = timing
            self._save()
        record_by_commit(self.data["jdk"], commit, "failed", tests)
        if timing:
            write_timings(self)

    # ── 派发租约 ──

    def set_lease(self, label: str, **lease):
        with self._lock:
            self.data["leases"][label] = lease
            self._save()

    def clear_lease(self, label: str):
        with self._lock:
            if self.data["leases"].pop(label, None) is not None:
                self._save()

    def leases(self) -> dict[str, dict]:
        with self._lock:
            return {k: dict(v) for k, v in self.data["leases"].items()}

    # ── infra 失败（网络 / 执行异常，区别于测试失败） ──

    def note_infra_return(self, test: str, server: str, reason: str, limit: int) -> bool:
        """记一次异常归还；超过 limit 次则记为 infra 失败并返回 True（调用方不再归还任务池）。"""
        with self._lock:
            n = self.data["infra_returns"].get(test, 0) + 1
            self.data["infra_returns"][test] = n
            over = n > limit
            if over:
                self.data["infra_returns"].pop(test, None)
                self.data["infra_failed"][test] = {
                    "server": server, "returns": n, "reason": str(reason)[:200],
                    "time": datetime.now().isoformat(timespec="seconds"),
                }
            self._save()
        if over:
            with _print_lock, open(dist_ctx.results_dir / f"infra_failed_jdk{self.data['jdk']}.txt",
                                   "a", encoding="utf-8") as fh:
                fh.write(f"{test}\t[{server}]\t[{datetime.now().strftime('%H:%M:%S')}]\t"
                         f"归还 {n} 次\t{str(reason)[:200]}\n")
        return over

    def requeue_infra(self) -> int:
        """新一轮开始：上轮的 infra 失败与归还计数清零，测试重新入队（infra 失败与测试本身无关）。"""
        with self._lock:
            n = len(self.data["infra_failed"])
            self.data["infra_failed"] = {}
            self.data["infra_returns"] = {}
            self._save()
        return n

    def completed_set(self) -> set[str]:
        with self._lock:
            return set(self.data["passed"])

    def failed_set(self) -> set[str]:
        with self._lock:
            return set(self.data["failed"])

    def commit_counts(self) -> dict[str, int]:
        """通过集按提交分布（短哈希 → 例数）。"""
        with self._lock:
            out: dict[str, int] = {}
            for v in self.data["passed"].values():
                c = (v.get("commit") or "unknown")[:9]
                out[c] = out.get(c, 0) + 1
            return out

    def set_jdk(self, jdk: int):
        with self._lock:
            self.data["jdk"] = jdk
            self._save()

    def summary(self) -> dict:
        with self._lock:
            return {
                "passed": len(self.data["passed"]),
                "failed": len(self.data["failed"]),
                "infra":  len(self.data["infra_failed"]),
                "jdk":    self.data["jdk"],
            }


# ── 结果合并 ──────────────────────────────────────────────────────────────────

_test_paths_cache: dict[str, str] | None = None


def _test_paths() -> dict[str, str]:
    """类名 → tests/e2e/ 相对路径（state 里记类名，通过清单记路径）。"""
    global _test_paths_cache
    if _test_paths_cache is None:
        root = config.LOCAL_E2E_DIR
        _test_paths_cache = {} if root is None else {
            f.stem: f"tests/e2e/{f.relative_to(root).as_posix()}" for f in root.rglob("*.java")
        }
    return _test_paths_cache


def record_by_commit(jdk: int, commit: str, kind: str, tests: list[str]):
    """追加到 by_commit/<commit>/{passed,failed}_jdk{N}.txt：同一提交上的运行记录（tests/e2e/ 路径）。"""
    d = dist_ctx.results_dir / "by_commit" / (commit or "unknown")[:12]
    d.mkdir(parents=True, exist_ok=True)
    paths = _test_paths()
    with _print_lock, (d / f"{kind}_jdk{jdk}.txt").open("a", encoding="utf-8") as f:
        for t in tests:
            f.write(paths.get(t, t) + "\n")


def write_master(state: "State"):
    """master_passed_jdk{N}.txt = state 的累计通过集（tests/e2e/ 路径），头部注明提交分布。"""
    with state._lock:
        jdk, names = state.data["jdk"], list(state.data["passed"])
    paths = _test_paths()
    dist = ", ".join(f"{c}×{n}" for c, n in sorted(state.commit_counts().items(), key=lambda x: -x[1]))
    master = dist_ctx.results_dir / f"master_passed_jdk{jdk}.txt"
    tmp = master.with_suffix(".tmp")
    tmp.write_text(
        f"# rava 分布式测试累计通过清单  JDK {jdk}  {datetime.now().isoformat(timespec='seconds')}\n"
        f"# 提交分布: {dist}\n" + "".join(t + "\n" for t in sorted(paths.get(n, n) for n in names)),
        encoding="utf-8",
    )
    tmp.replace(master)
    return master


def write_timings(state: "State"):
    """timings_jdk{N}.tsv = state 中带耗时记录的测试，按运行段耗时降序（再按总耗时）。
    耗时取自 run_tests.py 每例结尾的 Elapsed 行（transpile / build / run 三段，秒）。"""
    with state._lock:
        jdk = state.data["jdk"]
        rows = [(t, "pass", v) for t, v in state.data["passed"].items() if v.get("timing")]
        rows += [(t, "fail", v) for t, v in state.data["failed"].items() if v.get("timing")]
    paths = _test_paths()
    rows.sort(key=lambda r: (-r[2]["timing"].get("run_s", 0), -r[2]["timing"].get("total_s", 0)))
    out = dist_ctx.results_dir / f"timings_jdk{jdk}.tsv"
    tmp = out.with_suffix(".tmp")
    lines = ["test\tpath\tresult\trun_s\ttranspile_s\tbuild_s\ttotal_s\tserver\tcommit"]
    for t, res, v in rows:
        tm = v["timing"]
        lines.append("\t".join([t, paths.get(t, t), res] +
                                [f"{tm.get(k, 0):.2f}" for k in ("run_s", "transpile_s", "build_s", "total_s")] +
                                [v.get("server", ""), (v.get("commit") or "")[:12]]))
    tmp.write_text("\n".join(lines) + "\n", encoding="utf-8")
    tmp.replace(out)
    return out


def archive_for_reset(state_path: Path, jdk: int):
    """--reset：把 state / 失败清单 / 失败日志 / master 归档到 archive/reset_<时间>/（by_commit/ 保留）。"""
    dest = dist_ctx.results_dir / "archive" / datetime.now().strftime("reset_%Y%m%d%H%M%S")
    moved = []
    for p in (state_path, dist_ctx.results_dir / f"failed_tests_jdk{jdk}.txt",
              dist_ctx.results_dir / f"master_passed_jdk{jdk}.txt"):
        if p.exists():
            dest.mkdir(parents=True, exist_ok=True)
            shutil.move(str(p), dest / p.name)
            moved.append(p.name)
    err_dir = dist_ctx.results_dir / "error_logs"
    logs = list(err_dir.glob(f"*_jdk{jdk}.log")) if err_dir.exists() else []
    if logs:
        (dest / "error_logs").mkdir(parents=True, exist_ok=True)
        for p in logs:
            shutil.move(str(p), dest / "error_logs" / p.name)
        moved.append(f"error_logs/×{len(logs)}")
    if moved:
        _log(f"🗄  已归档到 {dest}：{', '.join(moved)}")
