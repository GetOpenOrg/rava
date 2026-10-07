"""合入守护正式路径单元测试：在临时 git 仓库（两个裸远端 + 主仓 + 集成 worktree）上走
合并 → 闸门 → 推送 → 主仓快进，以及冲突 / 闸门失败回退 / 推送失败续推 / 集成 worktree 脏时暂缓。
闸门以桩代替（真实闸门由试运行覆盖）；抽查结果直接写 state 文件。

运行：cd rava && uv run python -m unittest tests.test_merge_daemon
"""

import argparse
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

import merge_daemon as md  # noqa: E402


def sh(cwd, *args) -> str:
    r = subprocess.run(["git", *args], cwd=cwd, capture_output=True, text=True)
    if r.returncode != 0:
        raise RuntimeError(f"git {args}: {r.stderr}")
    return r.stdout.strip()


class Repo:
    def __init__(self, root: Path):
        self.root = root
        for name in ("origin.git",):
            sh(root, "init", "-q", "--bare", "-b", "main", name)
        self.main = root / "main"
        sh(root, "init", "-q", "-b", "main", "main")
        for k, v in (("user.name", "t"), ("user.email", "t@t")):
            sh(self.main, "config", k, v)
        (self.main / "docs").mkdir()
        (self.main / "docs" / "a.md").write_text("line1\nline2\n")
        (self.main / "src.rs").write_text("fn a() {}\n")
        sh(self.main, "add", ".")
        sh(self.main, "commit", "-qm", "init")
        sh(self.main, "branch", "rust-closure-analyzer")
        for r in ("origin",):
            sh(self.main, "remote", "add", r, str(root / f"{r}.git"))
            sh(self.main, "push", "-q", r, "main", "rust-closure-analyzer")
        self.integ = root / "integ"
        sh(self.main, "worktree", "add", "-q", str(self.integ), "rust-closure-analyzer")

    def feature(self, name: str, base: str, path: str, text: str) -> str:
        sh(self.main, "branch", name, base)
        wt = self.root / f"wt-{name}"
        sh(self.main, "worktree", "add", "-q", str(wt), name)
        (wt / path).write_text(text)
        sh(wt, "commit", "-qam", f"feat {name}")
        sha = sh(wt, "rev-parse", "HEAD")
        sh(self.main, "worktree", "remove", "--force", str(wt))
        return sha

    def remote_head(self, remote: str, branch: str) -> str:
        return sh(self.root / f"{remote}.git", "rev-parse", branch)


class DaemonTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        root = Path(self.tmp.name)
        self.repo = Repo(root)
        self.results = root / "results"
        a = argparse.Namespace(dry_run=False, queue=str(root / "q.toml"), integ_wt=str(self.repo.integ),
                               main_repo=str(self.repo.main), dry_run_wt=str(root / "dry"), known=None,
                               log=str(root / "daemon.log"), results_dir=str(self.results))
        self.ctx = md.Ctx(a)
        self.known = ([], "test")

    def tearDown(self):
        self.tmp.cleanup()

    def spot(self, tag: str, sha: str, tests: list[str]):
        d = self.results / "spot" / tag
        d.mkdir(parents=True, exist_ok=True)
        (d / "state_jdk21.json").write_text(json.dumps(
            {"passed": {t: {"commit": sha} for t in tests}, "failed": {}, "leases": {}}))

    def enqueue(self, eid, branch, sha, tests=("T1",), gate="generator", keep=True):
        if tests:
            self.spot(eid, sha, list(tests))
        self.ctx.queue.append({"id": eid, "branch": branch, "sha": sha, "spot": eid, "tests": list(tests),
                               "gate": gate, "keep_branch": keep, "summary": f"摘要 {eid}"})

    def scan(self, gate_ok=True):
        with mock.patch.object(md, "run_gate", return_value=(gate_ok, "桩闸门")), \
             mock.patch.object(md.kf, "load_known", return_value=self.known), \
             mock.patch.object(md, "spot_procs", return_value=[]):
            md.scan(self.ctx)
        return {e["id"]: e for e in self.ctx.queue.snapshot()}

    def test_merge_push_ff(self):
        r = self.repo
        sha = self.repo.feature("feat", "rust-closure-analyzer", "src.rs", "fn b() {}\n")
        self.enqueue("e1", "feat", sha, keep=False)
        q = self.scan()
        self.assertEqual(q["e1"]["status"], "merged", q["e1"])
        head = sh(r.integ, "rev-parse", "HEAD")
        self.assertEqual(sh(r.integ, "rev-parse", "HEAD^2"), sha)
        msg = sh(r.integ, "log", "-1", "--format=%B")
        self.assertIn("Merge feat（", msg)
        self.assertIn("抽查 e1 1/1", msg)
        self.assertNotIn("Claude", msg)
        for rem in ("origin",):
            self.assertEqual(r.remote_head(rem, "rust-closure-analyzer"), head)
            self.assertEqual(r.remote_head(rem, "main"), head)
        self.assertEqual(sh(r.main, "rev-parse", "main"), head)
        self.assertEqual(sh(r.main, "branch", "--list", "feat"), "")

    def test_gate_fail_rolls_back(self):
        r = self.repo
        before = sh(r.integ, "rev-parse", "HEAD")
        sha = r.feature("bad", "rust-closure-analyzer", "src.rs", "fn c() {}\n")
        self.enqueue("e2", "bad", sha)
        q = self.scan(gate_ok=False)
        self.assertEqual(q["e2"]["status"], "blocked")
        self.assertIn("闸门不干净", q["e2"]["reason"])
        self.assertEqual(sh(r.integ, "rev-parse", "HEAD"), before)
        self.assertEqual(r.remote_head("origin", "rust-closure-analyzer"), before)

    def test_conflict_aborts(self):
        r = self.repo
        base = sh(r.integ, "rev-parse", "HEAD")
        (r.integ / "docs" / "a.md").write_text("line1\nintegration\n")
        sh(r.integ, "commit", "-qam", "integ change")
        after = sh(r.integ, "rev-parse", "HEAD")
        sha = r.feature("conf", base, "docs/a.md", "line1\nfeature\n")
        self.enqueue("e3", "conf", sha, tests=(), gate="doc-only")
        q = self.scan()
        self.assertEqual(q["e3"]["status"], "blocked")
        self.assertIn("docs/a.md", q["e3"]["reason"])
        self.assertEqual(sh(r.integ, "rev-parse", "HEAD"), after)
        self.assertEqual(sh(r.integ, "status", "--porcelain"), "")

    def test_doc_only_rejects_code(self):
        r = self.repo
        before = sh(r.integ, "rev-parse", "HEAD")
        sha = r.feature("doccode", "rust-closure-analyzer", "src.rs", "fn d() {}\n")
        self.enqueue("e4", "doccode", sha, tests=(), gate="doc-only")
        with mock.patch.object(md.kf, "load_known", return_value=self.known):
            md.scan(self.ctx)   # 不打桩 run_gate：doc-only 走真实核对
        e = self.ctx.queue.snapshot()[0]
        self.assertEqual(e["status"], "blocked")
        self.assertIn("src.rs", e["reason"])
        self.assertEqual(sh(r.integ, "rev-parse", "HEAD"), before)

    def test_push_pending_then_retry(self):
        r = self.repo
        sha = r.feature("p", "rust-closure-analyzer", "src.rs", "fn e() {}\n")
        self.enqueue("e5", "p", sha)
        good = sh(r.main, "remote", "get-url", "origin")
        sh(r.main, "remote", "set-url", "origin", str(r.root / "missing.git"))
        q = self.scan()
        self.assertEqual(q["e5"]["status"], "push_pending")
        sh(r.main, "remote", "set-url", "origin", good)
        q = self.scan()
        self.assertEqual(q["e5"]["status"], "merged")
        self.assertEqual(r.remote_head("origin", "main"), sh(r.integ, "rev-parse", "HEAD"))

    def test_dirty_integ_waits(self):
        r = self.repo
        sha = r.feature("w", "rust-closure-analyzer", "src.rs", "fn f() {}\n")
        self.enqueue("e6", "w", sha)
        (r.integ / "src.rs").write_text("dirty\n")
        q = self.scan()
        self.assertEqual(q["e6"]["status"], "ready")
        self.assertIn("未提交", q["e6"]["reason"])
        sh(r.integ, "checkout", "--", "src.rs")
        q = self.scan()
        self.assertEqual(q["e6"]["status"], "merged")

    def test_new_failure_blocks(self):
        sha = self.repo.feature("nf", "rust-closure-analyzer", "src.rs", "fn g() {}\n")
        self.enqueue("e7", "nf", sha)
        d = self.results / "spot" / "e7"
        (d / "state_jdk21.json").write_text(json.dumps(
            {"passed": {}, "failed": {"T1": {"commit": sha, "error": "test failed"}}, "leases": {}}))
        (d / "error_logs").mkdir()
        (d / "error_logs" / "T1_s_jdk21.log").write_text("error[E0001]: boom\n")
        q = self.scan()
        self.assertEqual(q["e7"]["status"], "blocked")
        self.assertIn("E0001", q["e7"]["new_failures"][0])


if __name__ == "__main__":
    unittest.main()
