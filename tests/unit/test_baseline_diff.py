"""scripts/baseline_diff.py：冻结基线对照的集合口径与日志解析。"""
import sys
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


if __name__ == "__main__":
    unittest.main()
