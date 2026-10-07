"""多槽（config slots）：槽展开、槽锁、按槽输出根、只清本测试 scratch、只杀本测试进程、物理机初始化只做一次。"""
import subprocess
import sys
import tempfile
import threading
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts" / "cluster"))

import cluster_config as config  # noqa: E402
import dist_ctx  # noqa: E402
import dist_e2e  # noqa: E402
import dist_job  # noqa: E402
import dist_remote  # noqa: E402

ONE = {"label": "a", "host": "h"}
FOUR = {"label": "b", "host": "h2", "slots": 4}


class SlotExpandTest(unittest.TestCase):
    def test_expand(self):
        ss = config.slot_servers([ONE, FOUR])
        self.assertEqual([s["label"] for s in ss], ["a", "b#0", "b#1", "b#2", "b#3"])
        self.assertEqual({config.host_label(s) for s in ss}, {"a", "b"})
        self.assertEqual(ss[0]["slots"], 1)
        self.assertEqual(ss[3]["slot"], 2)

    def test_locks(self):
        self.assertEqual(config.slot_lock_path(0), "/tmp/rava_dist.lock")
        self.assertEqual(config.slot_lock_path(3), "/tmp/rava_dist.slot3.lock")
        s3 = config.slot_servers([FOUR])[3]
        self.assertIn("exec 9>/tmp/rava_dist.slot3.lock", dist_job.server_lock_prefix(s3))
        self.assertIn("exec 9>/tmp/rava_dist.lock", dist_job.server_lock_prefix(ONE))


class SlotCmdTest(unittest.TestCase):
    def test_single_slot_cmd(self):
        cmd = dist_e2e._test_cmd(config.slot_servers([ONE])[0], "Foo", config.DEFAULT_JDK, "/d")
        self.assertIn("-j 1 --filter /Foo.java", cmd)
        self.assertNotIn("--out-dir", cmd)
        self.assertNotIn(".cargo/config.toml", cmd)
        self.assertNotIn("/ 1 ))", cmd)

    def test_multi_slot_cmd(self):
        s2 = config.slot_servers([FOUR])[2]
        cmd = dist_e2e._test_cmd(s2, "Foo", config.DEFAULT_JDK, "/d")
        self.assertIn("-j 1 --out-dir build/slot2 --filter /Foo.java", cmd)
        self.assertIn("/d/build/slot2/.cargo/config.toml", cmd)
        self.assertIn("RAVA_LIM=$(( RAVA_TOTAL / 4 ))", cmd)
        self.assertIn("[ $RAVA_LIM -lt 14336 ] && RAVA_LIM=14336", cmd)
        self.assertIn("--slice=rava.slice", cmd)
        self.assertEqual(subprocess.run(["bash", "-n", "-c", cmd]).returncode, 0)

    def test_job_dirs_and_jobs(self):
        self.assertEqual(dist_remote.job_remote_dir("/data/r", "t", 0), "/data/r-spot-job-t")
        self.assertEqual(dist_remote.job_remote_dir("/data/r", "t", 2), "/data/r-spot-job-t-s2")

    def test_resource_gate(self):
        s = config.slot_servers([FOUR])[0]
        self.assertTrue(dist_remote._resource_ok(20000, 0.85, s))
        self.assertFalse(dist_remote._resource_ok(20000, 0.85, ONE))
        self.assertFalse(dist_remote._resource_ok(8000, 0.1, s))
        self.assertTrue(dist_remote._resource_ok(8000, 0.1, ONE))


class SlotCleanupTest(unittest.TestCase):
    def _tree(self, root: Path, out: str):
        j = root / out / "jdk21"
        for p in ("foo_bar/src/x.rs", "other/src/y.rs", "target/debug/a.rlib",
                  "logs/foo_bar.build.log", "logs/other.build.log", "expected-classes/foo_bar/A.class"):
            (j / p).parent.mkdir(parents=True, exist_ok=True)
            (j / p).write_text("x")
        return j

    def test_only_own_scratch(self):
        with tempfile.TemporaryDirectory() as d:
            j1 = self._tree(Path(d), "build/slot1")
            j2 = self._tree(Path(d), "build/slot2")
            subprocess.run(["bash", "-c", dist_e2e._scratch_rm(d, 21, "build/slot1", "FooBar", False)], check=True)
            self.assertFalse((j1 / "foo_bar").exists())
            self.assertFalse((j1 / "expected-classes/foo_bar").exists())
            self.assertFalse((j1 / "logs/foo_bar.build.log").exists())
            self.assertTrue((j1 / "other/src/y.rs").exists())
            self.assertTrue((j1 / "logs/other.build.log").exists())
            self.assertTrue((j1 / "target/debug/a.rlib").exists())          # 本槽编译缓存保留
            self.assertTrue((j2 / "foo_bar/src/x.rs").exists())             # 其他槽不动
            subprocess.run(["bash", "-c", dist_e2e._scratch_rm(d, 21, "build/slot1", "FooBar", True)], check=True)
            self.assertFalse(j1.exists())
            self.assertTrue((j2 / "target/debug/a.rlib").exists())

    def test_kill_only_own_test(self):
        cmd = dist_ctx._kill_cmd("/d", "KILL", "Foo")
        self.assertIn("--filter /Foo\\.java( |$)", cmd)
        abort = dist_ctx._abort_cmd("/d", "/d/build/dist_runs/k1", "Foo")
        self.assertIn("kill_run /d/build/dist_runs/k1", abort)
        self.assertNotIn("dist_runs/*/", abort)


class HostGateTest(unittest.TestCase):
    def test_init_once(self):
        gate = dist_e2e.HostGate(4)
        calls, results = [], []

        def init():
            calls.append(1)
            return {"work_dir": "/w", "commit": "c"}

        ts = [threading.Thread(target=lambda: results.append(gate.init_once(init))) for _ in range(4)]
        for t in ts:
            t.start()
        for t in ts:
            t.join(5)
        self.assertEqual(len(calls), 1)
        self.assertEqual(results, [{"work_dir": "/w", "commit": "c"}] * 4)
        self.assertEqual([gate.leave() for _ in range(4)], [False, False, False, True])


if __name__ == "__main__":
    unittest.main()


class PickServersTest(unittest.TestCase):
    """缺省服务器按 pools 取：全量 / 抽查用 test 池、作业用 job 池；点名优先，不受池限制。"""

    SERVERS = [{"label": "t", "pools": ["test"]}, {"label": "j", "pools": ["job"]}, {"label": "c"}]

    def setUp(self):
        self._saved = config.SERVERS
        config.SERVERS = self.SERVERS

    def tearDown(self):
        config.SERVERS = self._saved

    def test_default_pools(self):
        self.assertEqual([s["label"] for s in config.pick_servers(None, "test")], ["t"])
        self.assertEqual([s["label"] for s in config.pick_servers(None, "job")], ["j"])

    def test_unpooled_only_by_name(self):
        self.assertEqual([s["label"] for s in config.pick_servers(["c"], "test")], ["c"])


class LoadClusterTest(unittest.TestCase):
    """本机清单读取：示例文件可解析，端口缺省 22，私钥路径展开 ~；文件缺失时清单为空。"""

    def test_example(self):
        servers, proxy, repo_url = config._load_cluster(Path(config.__file__).with_name("cluster.example.toml"))
        self.assertEqual({s["label"]: s.get("pools") for s in servers}, {"job1": ["job"], "test1": ["test"]})
        self.assertTrue(all(s["port"] == 22 and not s["private_key_path"].startswith("~") for s in servers))
        self.assertIsNone(proxy)
        self.assertEqual(repo_url, "git@git.example.internal:team/rava.git")

    def test_missing(self):
        self.assertEqual(config._load_cluster(Path("/nonexistent/cluster.toml")), ([], None, None))

