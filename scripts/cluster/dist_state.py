"""任务池、进度状态与结果归档（state / master 通过清单 / by_commit / reset 归档）。"""

import json
import os
import random
import re
import shutil
import threading
import time
from collections import deque
from datetime import datetime
from pathlib import Path

import cluster_config as config
from cluster_config import DEFAULT_JDK
import dist_ctx
from dist_ctx import _log, _print_lock


# ── 状态文件路径（唯一来源）与原子读写 ────────────────────────────────────────

def state_path(results_dir: Path, jdk: int) -> Path:
    """结果目录下某 JDK 主版本的进度状态文件。全量 / 抽查 / 巡检 / 已知失败分类一律经此取路径。"""
    return Path(results_dir) / f"state_jdk{int(jdk)}.json"


_STATE_RE = re.compile(r"^state_jdk(\d+)\.json$")


def state_paths(results_dir: Path) -> dict[int, Path]:
    """结果目录下全部 JDK 的状态文件：主版本 → 路径（按版本升序）。"""
    d = Path(results_dir)
    found = {}
    if d.is_dir():
        for p in d.iterdir():
            if (m := _STATE_RE.match(p.name)) and p.is_file():
                found[int(m.group(1))] = p
    return dict(sorted(found.items()))


class CorruptStateError(RuntimeError):
    """状态 JSON 无法解析：原文件已备份为 .corrupt-<时间>，须人工核对后再运行。"""


def load_json_strict(path: Path, default=None):
    """读 JSON；文件不存在返回 default。解析失败时把原文件备份为 <name>.corrupt-<时间> 并抛
    CorruptStateError——不静默清空（清空会让续跑把累计通过 / 失败记录全部丢掉）。"""
    path = Path(path)
    if not path.exists():
        return default
    raw = path.read_bytes()
    try:
        return json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as e:
        backup = path.with_name(f"{path.name}.corrupt-{datetime.now().strftime('%Y%m%d%H%M%S')}")
        backup.write_bytes(raw)
        raise CorruptStateError(
            f"{path} 无法解析（{e}）；原文件已备份到 {backup}。核对修复后放回原处再运行，"
            f"或确认放弃该进度后删除 {path.name}") from e


def atomic_write_text(path: Path, text: str):
    """原子写：同目录唯一临时文件写入并 fsync，再 os.replace 覆盖；中途被杀不会留下半截文件。"""
    path = Path(path)
    tmp = path.with_name(f".{path.name}.{os.getpid()}.{threading.get_ident()}.tmp")
    try:
        with open(tmp, "w", encoding="utf-8") as fh:
            fh.write(text)
            fh.flush()
            os.fsync(fh.fileno())
        os.replace(tmp, path)
    finally:
        tmp.unlink(missing_ok=True)


# ── 测试名校验 ────────────────────────────────────────────────────────────────
# 测试名会拼进远端 shell（--filter、rm -rf、pgrep）：所有入口（--tests、load_test_list、sample_passed、
# 读租约）统一校验为 Java 标识符形（字母或下划线开头，后接字母 / 数字 / 下划线；字母含 Unicode，
# 语料有 MöbiusFunction），远端命令再一律 shlex.quote。

_TEST_NAME_RE = re.compile(r"[^\W\d]\w*")


def valid_test_name(name: str) -> bool:
    return isinstance(name, str) and _TEST_NAME_RE.fullmatch(name) is not None


def check_test_names(names, source: str) -> list[str]:
    """校验一批测试名，有非法名即抛 ValueError（指明来源）；全部合法时原样返回列表。"""
    names = list(names)
    bad = [n for n in names if not valid_test_name(n)]
    if bad:
        raise ValueError(f"{source} 含非法测试名（须为 Java 标识符）：{', '.join(map(repr, bad[:10]))}"
                         + (f" 等 {len(bad)} 个" if len(bad) > 10 else ""))
    return names


# ── 续跑租约 ──────────────────────────────────────────────────────────────────

def resume_deadline(orig_deadline: float, limit: float, now: float | None = None) -> float:
    """接上旧租约时的期限：max(原期限, 接上时刻 + 时限)。停机间隔不计入单例时限，
    原期限更晚（刚派发不久就中断）时沿用原期限。"""
    return max(orig_deadline, (time.time() if now is None else now) + limit)


def lease_ref_matches(lease_commit: str | None, target_commit: str | None) -> bool:
    """租约所在提交是否就是本次要测的提交。本次目标解析不出（None）时不判不符。"""
    if not target_commit:
        return True
    if not lease_commit:
        return False
    a, b = lease_commit.lower(), target_commit.lower()
    n = min(len(a), len(b))
    return n >= 7 and a[:n] == b[:n]


# ── 测试列表枚举 ──────────────────────────────────────────────────────────────

def load_test_list() -> list[str]:
    """从本地 tests/e2e/ 枚举测试（按路径排序，与服务器端顺序一致）。"""
    if config.LOCAL_E2E_DIR is None:
        raise FileNotFoundError(
            "找不到本地 tests/e2e 目录。\n"
            "请用 --e2e-dir /path/to/tests/e2e 指定（缺省取主检出的 tests/e2e）"
        )
    return check_test_names(sorted(f.stem for f in config.LOCAL_E2E_DIR.rglob("*.java")),
                            str(config.LOCAL_E2E_DIR))


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
    return check_test_names(sorted(picked), str(master))


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
        data = load_json_strict(self.path)
        if data is not None and not isinstance(data, dict):
            raise CorruptStateError(f"{self.path} 顶层不是对象（{type(data).__name__}），拒绝覆盖")
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
        check_test_names([v.get("test") for v in data["leases"].values()], f"{self.path} 的派发租约")
        # 因网络 / 执行异常归还的次数（本轮），以及超过上限记为 infra 失败的测试（不计入 failed）
        data.setdefault("infra_returns", {})
        data.setdefault("infra_failed", {})
        return data

    def _save(self):
        atomic_write_text(self.path, json.dumps(self.data, indent=2, ensure_ascii=False))

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
            out = {k: dict(v) for k, v in self.data["leases"].items()}
        check_test_names([v.get("test") for v in out.values()], f"{self.path} 的派发租约")
        return out

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
    atomic_write_text(
        master,
        f"# rava 分布式测试累计通过清单  JDK {jdk}  {datetime.now().isoformat(timespec='seconds')}\n"
        f"# 提交分布: {dist}\n" + "".join(t + "\n" for t in sorted(paths.get(n, n) for n in names)),
    )
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
    lines = ["test\tpath\tresult\trun_s\ttranspile_s\tbuild_s\ttotal_s\tserver\tcommit"]
    for t, res, v in rows:
        tm = v["timing"]
        lines.append("\t".join([t, paths.get(t, t), res] +
                                [f"{tm.get(k, 0):.2f}" for k in ("run_s", "transpile_s", "build_s", "total_s")] +
                                [v.get("server", ""), (v.get("commit") or "")[:12]]))
    atomic_write_text(out, "\n".join(lines) + "\n")
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
