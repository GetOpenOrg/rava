"""e2e 单例设置透传：--run-tests-args 追加到服务器 run_tests.py 命令行、--task-timeout 记入租约（缺省行为不变）。"""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts" / "cluster"))

import cluster_config as config  # noqa: E402
import dist_ctx  # noqa: E402
import dist_e2e  # noqa: E402

SERVER = {"label": "t", "host": "h"}


class RunTestsArgsTest(unittest.TestCase):
    def tearDown(self):
        dist_ctx.run_tests_args = ""
        dist_ctx.task_timeout = None

    def test_default_cmd_unchanged(self):
        cmd = dist_e2e._test_cmd(SERVER, "Foo", config.DEFAULT_JDK, "/d")
        self.assertIn("-j 1 --filter /Foo.java", cmd)

    def test_args_appended(self):
        dist_ctx.run_tests_args = "--transpile-timeout 1800 --run-timeout 900"
        cmd = dist_e2e._test_cmd(SERVER, "Foo", config.DEFAULT_JDK, "/d")
        self.assertIn("-j 1 --transpile-timeout 1800 --run-timeout 900 --filter /Foo.java", cmd)

    def test_task_timeout(self):
        self.assertEqual(dist_e2e.task_timeout(), config.TASK_TIMEOUT)
        self.assertEqual(dist_e2e.task_timeout({"start": 0}), config.TASK_TIMEOUT, "旧租约无该键")
        dist_ctx.task_timeout = 9000
        lease = dist_e2e.new_lease("Foo", 21, "/d", "abc", "tag")
        self.assertEqual(lease["task_timeout"], 9000)
        dist_ctx.task_timeout = None
        self.assertEqual(dist_e2e.task_timeout(lease), 9000, "接上旧运行时沿用派发时的时限")


if __name__ == "__main__":
    unittest.main()
