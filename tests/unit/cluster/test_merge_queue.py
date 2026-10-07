"""合入队列读写单元测试：TOML 往返、加锁更新、重复 id。

运行：cd rava && uv run python -m unittest tests.test_merge_queue
"""

import sys
import tempfile
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

from merge_queue import Queue, dumps, loads  # noqa: E402


class QueueTest(unittest.TestCase):
    def test_roundtrip(self):
        e = {"id": "a", "branch": "b", "sha": "0" * 40, "tests": ["X", "Y"], "keep_branch": False,
             "spot_launches": 2, "summary": "含\"引号\"与\\反斜杠\n换行", "new_failures": ["A: x | y"]}
        self.assertEqual(loads(dumps([e])), [e])

    def test_update_append(self):
        with tempfile.TemporaryDirectory() as t:
            q = Queue(Path(t) / "q.toml")
            q.append({"id": "a", "branch": "b", "sha": "1" * 40, "tests": [], "gate": "doc-only"})
            with self.assertRaises(ValueError):
                q.append({"id": "a"})
            q.update("a", status="blocked", reason="r")
            q.update("a", reason=None)
            s = q.snapshot()
            self.assertEqual(s[0]["status"], "blocked")
            self.assertNotIn("reason", s[0])
            self.assertIsNone(q.update("zz", status="x"))


if __name__ == "__main__":
    unittest.main()
