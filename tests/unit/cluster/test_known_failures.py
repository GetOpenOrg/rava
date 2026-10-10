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


PATTERNS = """
[[pattern]]
id = "e0283"
stage = "compile"
symptom = "E0283"
match = [["error[E0283]"]]
tests = ["NormalDistribution"]
cause = "无类型缺省值"
introduced = ""
fixed = ["d4f45a58"]
status = "fixed"
owner = ""
diagnose = ""
doc = ""
since = "2026-10-10"

[[pattern]]
id = "clone-noise"
stage = "run"
symptom = "只用于验证 [no-provenance] 行被剔除"
match = [["CloneNotSupportedException"]]
tests = []
cause = "x"
introduced = ""
fixed = []
status = "open"
owner = "y"
diagnose = ""
doc = ""
since = "2026-10-10"

[[pattern]]
id = "npe-or-timeout"
stage = "run"
symptom = "NPE / 超时"
match = [["NullPointerException"], ["run timeout"]]
tests = ["ReflectionAPI"]
cause = "z"
introduced = ""
fixed = []
status = "open"
owner = "w"
diagnose = ""
doc = ""
since = "2026-10-10"

[[pattern]]
id = "unit-only"
stage = "unit"
symptom = "单测类，不参与自动匹配"
match = []
tests = []
cause = "u"
introduced = ""
fixed = []
status = "fixed"
owner = ""
diagnose = ""
doc = ""
since = "2026-10-10"
"""


class PatternTest(unittest.TestCase):
    def test_match_and_noise(self):
        pats = kf.parse_patterns(PATTERNS)
        log = ("[FAIL ] x — compile error  error[E0283]: type annotations needed\n"
               "  [no-provenance] x: java/lang/CloneNotSupportedException\n")
        self.assertEqual([p.id for p in kf.match_patterns("X", log, pats)], ["e0283"])

    def test_or_groups_and_test_rank(self):
        pats = kf.parse_patterns(PATTERNS + PATTERNS.replace('id = "', 'id = "b-').replace(
            'tests = ["ReflectionAPI"]', 'tests = ["Other"]'))
        hits = kf.match_patterns("ReflectionAPI", "[FAIL ] y — run timeout (> 15m)\n", pats)
        self.assertEqual([p.id for p in hits], ["npe-or-timeout", "b-npe-or-timeout"])

    def test_hints_on_new_failures(self):
        with tempfile.TemporaryDirectory() as t:
            root = Path(t)
            d = root / "spot" / "t1" / "error_logs"
            d.mkdir(parents=True)
            (d.parent / "state_jdk21.json").write_text(json.dumps(
                {"passed": {}, "failed": {"N": {"error": "compile error"}}}), encoding="utf-8")
            (d / "N_jp1_jdk21.log").write_text("error[E0283]: type annotations needed\n", encoding="utf-8")
            v = kf.classify("t1", ["N"], [], "x", root, patterns=kf.parse_patterns(PATTERNS))
            self.assertEqual(len(v.new[0]["hints"]), 1)
            self.assertTrue(v.new[0]["hints"][0].startswith("e0283（fixed，修复 d4f45a58）"))

    def test_bad_patterns(self):
        base = PATTERNS.split("[[pattern]]")[1]
        with self.assertRaises(ValueError):  # 缺字段
            kf.parse_patterns('[[pattern]]\nid = "x"\n')
        with self.assertRaises(ValueError):  # id 重复
            kf.parse_patterns("[[pattern]]" + base + "[[pattern]]" + base)
        with self.assertRaises(ValueError):  # 未修复须有 owner
            kf.parse_patterns("[[pattern]]" + base.replace('status = "fixed"', 'status = "open"'))
        with self.assertRaises(ValueError):  # 空匹配组
            kf.parse_patterns("[[pattern]]" + base.replace('[["error[E0283]"]]', '[[]]'))

    def test_repo_file_parses(self):
        repo_file = _HERE.parents[2] / "docs" / "failure_patterns.toml"
        self.assertTrue(kf.parse_patterns(repo_file.read_text(encoding="utf-8")))


if __name__ == "__main__":
    unittest.main()
