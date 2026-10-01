"""scripts/baseline_diff.py：冻结基线对照的集合口径与日志解析。"""
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "scripts"))
import baseline_diff as bd  # noqa: E402


class BaselineDiffTest(unittest.TestCase):
    def test_normalize_accepts_both_forms(self):
        self.assertEqual(bd.normalize("01_basics/A.java"), "tests/e2e/01_basics/A.java")
        self.assertEqual(bd.normalize("tests/e2e/01_basics/A.java"), "tests/e2e/01_basics/A.java")

    def test_transpile_failures_only_transpile_errors(self):
        log = ("[06:31:46] [FAIL ] 01_basics/A.java   — transpile error\n"
               "[06:31:47] [FAIL ] 02_oop/B.java      — compile error\n"
               "[06:31:48] [PASS ] 03_x/C.java        | dyn miss 0\n")
        self.assertEqual(bd.transpile_failures(log), {"tests/e2e/01_basics/A.java"})

    def test_compare_splits_regression_and_removed(self):
        base = {"tests/e2e/a/A.java", "tests/e2e/a/B.java", "tests/e2e/a/Gone.java"}
        passed = {"tests/e2e/a/A.java", "tests/e2e/a/New.java"}
        present = {"tests/e2e/a/A.java", "tests/e2e/a/B.java", "tests/e2e/a/New.java"}
        res = bd.compare(base, passed, set(), present)
        self.assertEqual(res["regressions"], ["tests/e2e/a/B.java"])
        self.assertEqual(res["removed"], ["tests/e2e/a/Gone.java"])
        self.assertEqual(res["new_passes"], ["tests/e2e/a/New.java"])

    def test_verdict_requires_no_transpile_failure(self):
        res = bd.compare({"tests/e2e/a/A.java"}, {"tests/e2e/a/A.java"},
                         {"tests/e2e/b/X.java"}, {"tests/e2e/a/A.java"})
        self.assertIn("❌", bd.report(res, 1, 1, "abc"))
        res = bd.compare({"tests/e2e/a/A.java"}, {"tests/e2e/a/A.java"}, set(),
                         {"tests/e2e/a/A.java"})
        self.assertIn("✅", bd.report(res, 1, 1, "abc"))

    def test_single_commit_rejects_mixed(self):
        self.assertEqual(bd.single_commit(["abc123", None], "abc"), "abc123")
        self.assertEqual(bd.single_commit([None], ""), "")
        with self.assertRaises(bd.InputError):
            bd.single_commit(["abc123", "def456"], "")
        with self.assertRaises(bd.InputError):
            bd.single_commit(["abc123"], "def")

    def test_verdict_requires_complete_run(self):
        res = bd.compare(set(), set(), set(), set(), not_run=["tests/e2e/a/A.java"])
        self.assertIn("❌", bd.report(res, 0, 0, "abc"))
        res = bd.compare(set(), set(), set(), set(), no_result=["tests/e2e/a/A.java"])
        self.assertFalse(bd.satisfied(res))

    def _results(self, d: Path, commit: str, completed, failed, passed_lines, logs=()):
        (d / "state_jdk21.json").write_text(json.dumps(
            {"jdk": 21, "commit": commit, "completed": completed, "failed": failed}))
        (d / f"passed_jdk21_{commit[:12]}.txt").write_text(
            f"# rava 分布式测试通过清单  JDK 21  commit {commit}  2026-10-01T00:00:00\n"
            + "".join(l + "\n" for l in passed_lines))
        (d / "error_logs").mkdir()
        for name, text in logs:
            (d / "error_logs" / name).write_text(text)

    def test_load_results(self):
        c = "a" * 40
        by_stem = {s: f"tests/e2e/x/{s}.java" for s in ("A", "B", "C", "D", "E")}
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            self._results(d, c, ["A"], {
                "B": {"error": "test failed"},
                "C": {"error": "no result（输出无 PASS/FAIL 行）"},
                "E": {"error": "test failed"},
            }, ["tests/e2e/x/A.java"], logs=[
                ("B_s_jdk21.log", "server: s  jdk: 21  git: aaaaaaa  time: t\n"
                                  "[FAIL ] x/B.java — transpile error\n"),
                ("E_s_jdk21.log", "server: s  jdk: 21  git: bbbbbbb  time: t\n"
                                  "[FAIL ] x/E.java — transpile error\n"),
            ])
            r = bd.load_results(d, 21, by_stem)
        self.assertEqual(r["commit"], c)
        self.assertEqual(r["passed"], {"tests/e2e/x/A.java"})
        self.assertEqual(r["tfail"], {"tests/e2e/x/B.java"})  # 旧提交日志 E 不计
        self.assertEqual(r["no_result"], ["tests/e2e/x/C.java"])
        self.assertEqual(r["not_run"], ["tests/e2e/x/D.java"])

    def test_load_results_rejects_legacy_and_mismatch(self):
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            (d / "state_jdk21.json").write_text(json.dumps({"jdk": 21, "completed": [], "failed": {}}))
            with self.assertRaises(bd.InputError):
                bd.load_results(d, 21, {})
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            self._results(d, "a" * 40, ["A", "B"], {}, ["tests/e2e/x/A.java"])
            with self.assertRaises(bd.InputError):
                bd.load_results(d, 21, {"A": "tests/e2e/x/A.java", "B": "tests/e2e/x/B.java"})


if __name__ == "__main__":
    unittest.main()
