"""每例耗时解析（run_tests.py 结尾 Elapsed 行）与 timings 表输出。"""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts" / "cluster"))

import dist_e2e  # noqa: E402


class ParseElapsedTest(unittest.TestCase):
    def test_minutes_and_seconds(self):
        tm = dist_e2e.parse_elapsed("Elapsed: 13m59.7s  (transpile 6m51.4s, build 7m07.2s, run 0.00s)")
        self.assertEqual(tm, {"total_s": 839.7, "transpile_s": 411.4, "build_s": 427.2, "run_s": 0.0})

    def test_seconds_only(self):
        tm = dist_e2e.parse_elapsed("Elapsed: 5.12s  (transpile 1.19s, build 3.00s, run 0.59s)")
        self.assertEqual(tm["run_s"], 0.59)

    def test_other_lines(self):
        self.assertIsNone(dist_e2e.parse_elapsed("[  1/1] [PASS ] 01_basics/HelloWorld.java (5.1s)"))


if __name__ == "__main__":
    unittest.main()
