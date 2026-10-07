"""已知失败判定单元测试：签名命中 / 同名签名不符 / 未出结果 / infra 失败。

运行：cd rava && python3 -m unittest tests.test_known_failures
"""

import json
import sys
import tempfile
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

import known_failures as kf  # noqa: E402

KNOWN = """
[[known]]
test = "StockTrans"
signature = "stub: L3 反射分派未覆盖 java/util/ArrayList.<alloc>:()V"
owner = "c1d-t2"
since = "2026-10-03"
"""


class ClassifyTest(unittest.TestCase):
    def _spot(self, root: Path, state: dict, logs: dict[str, str]):
        d = root / "spot" / "t1"
        (d / "error_logs").mkdir(parents=True)
        (d / "state_jdk21.json").write_text(json.dumps(state), encoding="utf-8")
        for name, text in logs.items():
            (d / "error_logs" / f"{name}_jp1_jdk21.log").write_text(text, encoding="utf-8")

    def test_matrix(self):
        known = kf.parse_known(KNOWN)
        with tempfile.TemporaryDirectory() as t:
            root = Path(t)
            self._spot(root, {
                "passed": {"A": {}},
                "failed": {"StockTrans": {"error": "test failed"}, "B": {"error": "test failed"}},
                "infra_failed": {"C": {}},
                "leases": {"kr1": {"test": "D"}},
            }, {
                "StockTrans": "[FAIL ] x — run error\nstub: L3 反射分派未覆盖 java/util/ArrayList.<alloc>:()V（…）\n",
                "B": "[FAIL ] 01/B.java — compile error (1s)  error[E0782]: expected a type\n",
            })
            v = kf.classify("t1", ["A", "B", "C", "D", "E", "StockTrans"], known, "x", root)
            self.assertEqual(v.passed, ["A"])
            self.assertEqual([k["test"] for k in v.known], ["StockTrans"])
            self.assertEqual([n["test"] for n in v.new], ["B"])
            self.assertIn("E0782", v.new[0]["summary"])
            self.assertEqual(v.infra, ["C"])
            self.assertEqual(v.pending, ["D", "E"])
            self.assertFalse(v.complete)

    def test_same_test_other_signature_is_new(self):
        known = kf.parse_known(KNOWN)
        with tempfile.TemporaryDirectory() as t:
            root = Path(t)
            self._spot(root, {"passed": {}, "failed": {"StockTrans": {"error": "test failed"}}},
                       {"StockTrans": "stub: L3 反射分派未覆盖 java/util/HashMap.<alloc>:()V\n"})
            v = kf.classify("t1", ["StockTrans"], known, "x", root)
            self.assertTrue(v.complete)
            self.assertFalse(v.clean)
            self.assertIn("签名不符", v.new[0]["summary"])

    def test_bad_entry(self):
        with self.assertRaises(ValueError):
            kf.parse_known('[[known]]\ntest = "X"\n')


if __name__ == "__main__":
    unittest.main()
