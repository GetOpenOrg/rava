"""合入队列文件（TOML）的读写：加锁读改写、简易 TOML 序列化。merge_daemon.py 与追加入队的子代理共用。

队列文件缺省 <结果目录>/merge_queue.toml（随结果目录不入库）。每条 [[entry]]：

  提交方填写：
    id            条目号（缺省 = spot tag）
    branch        分支名（rava 仓库）
    sha           完整 40 位提交哈希（已推送 origin）
    spot          抽查 tag（结果在 test_results/spot/<tag>/）
    tests         抽查用例名单（空 = 不抽查，仅 doc-only 允许）
    gate          闸门：generator / generator+macros_core / generator+rava_coro /
                  generator+macros_core+rava_coro / doc-only
    keep_branch   合入后是否保留分支（false = 合入后删除本地分支引用）
    summary       合并信息主体（中文，一句话说明改动）
    submitted_by  提交方（子代理 / 协调者）
    added         入队时间
  守护写入：
    status        queued / spot_running / ready / merged / blocked / push_pending / main_pending / dry_run_ok
    spot_launches 守护发起 / 重发抽查的次数
    spot_pid      守护最近一次发起的抽查进程号
    verdict       抽查判定一行摘要
    new_failures  新失败摘要列表
    reason        blocked / pending 的原因
    merge_commit  合并提交哈希
    updated       最近一次状态变化时间
"""

import fcntl
import json
import os
import tomllib
from contextlib import contextmanager
from datetime import datetime
from pathlib import Path

_HERE = Path(__file__).resolve().parent
from cluster_config import RESULTS_DIR  # noqa: E402
DEFAULT_QUEUE = RESULTS_DIR / "merge_queue.toml"

GATES = ("generator", "generator+macros_core", "generator+rava_coro",
         "generator+macros_core+rava_coro", "doc-only")
TERMINAL = ("merged", "blocked")
FIELD_ORDER = ("id", "branch", "sha", "spot", "tests", "gate", "keep_branch", "summary", "submitted_by",
               "added", "status", "spot_launches", "spot_pid", "verdict", "new_failures", "reason",
               "merge_commit", "updated")


def now() -> str:
    return datetime.now().isoformat(timespec="seconds")


def _toml_value(v) -> str:
    if isinstance(v, bool):
        return "true" if v else "false"
    if isinstance(v, int):
        return str(v)
    if isinstance(v, (list, tuple)):
        return "[" + ", ".join(_toml_value(x) for x in v) + "]"
    # TOML 基本字符串与 JSON 字符串转义兼容（ensure_ascii=False 时只转义控制字符、引号、反斜杠）
    return json.dumps(str(v), ensure_ascii=False)


def dumps(entries: list[dict]) -> str:
    out = ["# rava 合入队列（merge_daemon.py 读写；追加请用 merge_daemon.py enqueue，勿手改正在处理的条目）\n"]
    for e in entries:
        out.append("\n[[entry]]\n")
        keys = [k for k in FIELD_ORDER if k in e] + sorted(k for k in e if k not in FIELD_ORDER)
        for k in keys:
            if e[k] is None:
                continue
            out.append(f"{k} = {_toml_value(e[k])}\n")
    return "".join(out)


def loads(text: str) -> list[dict]:
    return list(tomllib.loads(text).get("entry", []))


class Queue:
    """队列文件 + 同目录 .lock 文件锁。所有改动都在 locked() 内读改写。"""

    def __init__(self, path: Path = DEFAULT_QUEUE):
        self.path = Path(path)
        self.lock_path = self.path.with_suffix(self.path.suffix + ".lock")

    @contextmanager
    def locked(self):
        self.path.parent.mkdir(parents=True, exist_ok=True)
        fd = os.open(self.lock_path, os.O_RDWR | os.O_CREAT, 0o644)
        try:
            fcntl.flock(fd, fcntl.LOCK_EX)
            entries = self._read()
            box = {"entries": entries, "dirty": False}
            yield box
            if box["dirty"]:
                self._write(box["entries"])
        finally:
            fcntl.flock(fd, fcntl.LOCK_UN)
            os.close(fd)

    def _read(self) -> list[dict]:
        if not self.path.exists():
            return []
        return loads(self.path.read_text(encoding="utf-8"))

    def _write(self, entries: list[dict]):
        tmp = self.path.with_suffix(".tmp")
        tmp.write_text(dumps(entries), encoding="utf-8")
        tmp.replace(self.path)

    def snapshot(self) -> list[dict]:
        with self.locked() as box:
            return [dict(e) for e in box["entries"]]

    def update(self, entry_id: str, **kw) -> dict | None:
        """按 id 更新字段（值为 None 的键删除），自动写 updated。返回更新后的条目。"""
        with self.locked() as box:
            for e in box["entries"]:
                if e.get("id") == entry_id:
                    for k, v in kw.items():
                        if v is None:
                            e.pop(k, None)
                        else:
                            e[k] = v
                    e["updated"] = now()
                    box["dirty"] = True
                    return dict(e)
        return None

    def append(self, entry: dict) -> dict:
        with self.locked() as box:
            if any(e.get("id") == entry["id"] for e in box["entries"]):
                raise ValueError(f"队列里已有条目 {entry['id']}")
            entry.setdefault("added", now())
            entry.setdefault("status", "queued")
            entry["updated"] = now()
            box["entries"].append(entry)
            box["dirty"] = True
            return dict(entry)
