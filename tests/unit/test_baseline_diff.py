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

    def test_verdict_requires_complete_run(self):
        res = bd.compare(set(), set(), set(), set(), not_run=["tests/e2e/a/A.java"])
        self.assertIn("❌", bd.report(res, 0, 0, "abc"))
        res = bd.compare(set(), set(), set(), set(), no_result=["tests/e2e/a/A.java"])
        self.assertFalse(bd.satisfied(res))

    def test_load_results(self):
        by_stem = {s: f"tests/e2e/x/{s}.java" for s in ("A", "B", "C", "D", "E")}
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            (d / "state_jdk21.json").write_text(json.dumps({"jdk": 21, "passed": {
                "A": {"commit": "a" * 40}, "E": {"commit": "b" * 40},
            }, "failed": {
                "B": {"error": "test failed"},
                "C": {"error": "no result（输出无 PASS/FAIL 行）"},
            }}))
            (d / "error_logs").mkdir()
            (d / "error_logs" / "B_s_jdk21.log").write_text("[FAIL ] x/B.java — transpile error\n")
            # E 早先转译失败、之后已通过：旧日志不计
            (d / "error_logs" / "E_s_jdk21.log").write_text("[FAIL ] x/E.java — transpile error\n")
            r = bd.load_results(d, 21, by_stem)
        self.assertEqual(r["passed"], {"tests/e2e/x/A.java", "tests/e2e/x/E.java"})
        self.assertEqual(r["commits"], {"a" * 9: 1, "b" * 9: 1})
        self.assertEqual(r["tfail"], {"tests/e2e/x/B.java"})
        self.assertEqual(r["no_result"], ["tests/e2e/x/C.java"])
        self.assertEqual(r["not_run"], ["tests/e2e/x/D.java"])

    def test_load_results_legacy_format(self):
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            (d / "state_jdk21.json").write_text(json.dumps(
                {"jdk": 21, "commit": "c" * 40, "completed": ["A"], "failed": {}}))
            r = bd.load_results(d, 21, {"A": "tests/e2e/x/A.java"})
        self.assertEqual(r["commits"], {"c" * 9: 1})


if __name__ == "__main__":
    unittest.main()
