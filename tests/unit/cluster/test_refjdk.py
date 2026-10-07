"""语料参考 JDK 的服务器侧：检出后幂等确保（ensure_reference_jdk）与 e2e / 作业命令的根目录注入。

运行：cd rava && python3 -m unittest tests.test_refjdk
"""

import shutil
import sys
import tempfile
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

from test_remote_run import FakeClient  # noqa: E402
import cluster_config as config  # noqa: E402
import dist_e2e  # noqa: E402
import dist_job  # noqa: E402
from dist_remote import ensure_reference_jdk  # noqa: E402


class EnsureReferenceJdkTest(unittest.TestCase):
    def setUp(self):
        self.work = Path(tempfile.mkdtemp())
        self.root = Path(tempfile.mkdtemp())
        self.server = {"label": "t", "refjdk_root": str(self.root)}

    def tearDown(self):
        shutil.rmtree(self.work, ignore_errors=True)
        shutil.rmtree(self.root, ignore_errors=True)

    def _script(self, body: str):
        (self.work / "scripts").mkdir(exist_ok=True)
        p = self.work / "scripts" / "fetch_reference_jdk.sh"
        p.write_text("#!/bin/sh\n" + body)
        p.chmod(0o755)

    def test_ensures_under_server_root(self):
        # 取包脚本收到的根目录来自服务器配置；stdout 末行即 JAVA_HOME
        self._script('echo "[refjdk] 下载" >&2; echo "$RAVA_REFJDK_ROOT/jdk-21.0.11+10"\n')
        home, err = ensure_reference_jdk(FakeClient(), self.server, str(self.work))
        self.assertIsNone(err)
        self.assertEqual(home, f"{self.root}/jdk-21.0.11+10")

    def test_failure_reports_stderr(self):
        self._script('echo "[refjdk] sha256 不符" >&2; exit 1\n')
        home, err = ensure_reference_jdk(FakeClient(), self.server, str(self.work))
        self.assertIsNone(home)
        self.assertIn("sha256 不符", err)

    def test_old_commit_without_script(self):
        self.assertEqual(ensure_reference_jdk(FakeClient(), self.server, str(self.work)), (None, None))


class CommandInjectionTest(unittest.TestCase):
    def test_default_jdk_uses_reference_build(self):
        cmd = dist_e2e._test_cmd({"label": "x"}, "HelloWorld", config.DEFAULT_JDK, "/data/rava")
        self.assertIn(f"RAVA_REFJDK_ROOT={config.REFJDK_ROOT}", cmd)
        self.assertNotIn("--jdk", cmd)

    def test_other_major_is_explicit_override(self):
        cmd = dist_e2e._test_cmd({"label": "x"}, "HelloWorld", 25, "/data/rava")
        self.assertIn("--jdk 25", cmd)

    def test_server_root_override(self):
        s = {"label": "ubuntu", "refjdk_root": "/mnt/d/workspace/rava-jdk"}
        self.assertIn("RAVA_REFJDK_ROOT=/mnt/d/workspace/rava-jdk", dist_e2e._test_cmd(s, "A", 21, "/r"))
        self.assertIn("RAVA_REFJDK_ROOT=/mnt/d/workspace/rava-jdk", dist_job._job_cmd(s, "true", "/r", 2))

    def test_configured_roots_are_data_dirs(self):
        for s in config.SERVERS:
            root = config.refjdk_root(s)
            remote = s.get("remote_dir", config.REMOTE_DIR)
            # 参考 JDK 与项目检出同盘（数据目录），不落系统目录
            self.assertEqual(Path(root).parent, Path(remote).parent, s["label"])


if __name__ == "__main__":
    unittest.main()
