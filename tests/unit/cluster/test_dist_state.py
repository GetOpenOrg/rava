"""审查即派 A：续跑期限 / 租约提交、测试名校验与引号、状态文件损坏与原子写、state_path 唯一来源、退出码、主机密钥策略。"""
import json
import sys
import tempfile
import time
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts" / "cluster"))

import paramiko  # noqa: E402

import dist_e2e  # noqa: E402
import dist_job  # noqa: E402
import dist_state  # noqa: E402
import patrol_status  # noqa: E402
import ssh_client  # noqa: E402
from distribute_tests import e2e_exit_code  # noqa: E402

SHA_A = "a" * 40
SHA_B = "b" * 40


class ResumeDeadlineTest(unittest.TestCase):
    def test_keeps_later_original(self):
        self.assertEqual(dist_state.resume_deadline(2000, 100, now=1000), 2000)

    def test_extends_from_resume_time(self):
        # 本地停机超过时限：从接上时刻重新计时，而非接上即超时
        self.assertEqual(dist_state.resume_deadline(1100, 600, now=5000), 5600)

    def test_e2e_lease_deadline_fallback(self):
        self.assertEqual(dist_e2e.lease_deadline({"start": 100, "deadline": 999}), 999)
        old = {"test": "HelloWorld", "start": 100}   # 旧租约无 deadline 键
        self.assertEqual(dist_e2e.lease_deadline(old), 100 + dist_e2e.task_timeout(old))


class LeaseRefTest(unittest.TestCase):
    def test_match(self):
        self.assertTrue(dist_state.lease_ref_matches(SHA_A, None))
        self.assertTrue(dist_state.lease_ref_matches(SHA_A, SHA_A))
        self.assertTrue(dist_state.lease_ref_matches("aaaaaaaaa", SHA_A))   # 短 sha 前缀
        self.assertFalse(dist_state.lease_ref_matches(SHA_A, SHA_B))
        self.assertFalse(dist_state.lease_ref_matches(None, SHA_A))
        self.assertFalse(dist_state.lease_ref_matches("aaa", SHA_A))         # 过短不算匹配

    def _board(self, d: Path, target):
        prev = {"01": {"cmd": "echo hi", "status": "running", "run_dir": "/tmp/x", "run_key": "k",
                       "server": "dev", "commit": SHA_A, "start": time.time()}}
        (d / "summary.json").write_text(json.dumps(prev))
        with mock.patch.object(dist_job, "_log"):
            return dist_job.JobBoard(["echo hi"], d, target_commit=target)

    def test_jobboard_recycles_mismatched_lease(self):
        with tempfile.TemporaryDirectory() as t:
            b = self._board(Path(t), SHA_B)
            self.assertEqual(b.data["01"]["status"], "pending")
            self.assertEqual(b._leases, {})

    def test_jobboard_keeps_matching_lease(self):
        with tempfile.TemporaryDirectory() as t:
            b = self._board(Path(t), SHA_A)
            self.assertEqual(b._leases, {"dev": "01"})


class TestNameTest(unittest.TestCase):
    def test_valid(self):
        for n in ("HelloWorld", "MöbiusFunction", "_X1"):
            self.assertTrue(dist_state.valid_test_name(n), n)

    def test_invalid(self):
        for n in ("", "a;rm -rf ~", "../x", "x y", "$(id)", "1abc", "a/b", "a'b", None):
            self.assertFalse(dist_state.valid_test_name(n), repr(n))
        with self.assertRaises(ValueError):
            dist_state.check_test_names(["Ok", "bad;name"], "--tests")

    def test_load_test_list_rejects(self):
        with tempfile.TemporaryDirectory() as t:
            (Path(t) / "HelloWorld.java").write_text("")
            (Path(t) / "sub").mkdir()
            (Path(t) / "sub" / "evil;id.java").write_text("")
            with mock.patch.object(dist_state.config, "LOCAL_E2E_DIR", Path(t)), \
                    self.assertRaises(ValueError):
                dist_state.load_test_list()

    def test_lease_names_validated(self):
        with tempfile.TemporaryDirectory() as t:
            p = Path(t) / "state_jdk21.json"
            p.write_text(json.dumps({"jdk": 21, "leases": {"dev": {"test": "x;reboot"}}}))
            with self.assertRaises(ValueError):
                dist_state.State(p)

    def test_cmd_quoted(self):
        cmd = dist_e2e._test_cmd({"label": "a", "host": "h"}, "HelloWorld", 21, "/r/my dir")
        # 命令整体再被单引号包进 RAVA_CMD：内层引号写成 '"'"'
        self.assertIn("cd '\"'\"'/r/my dir'\"'\"' &&", cmd)
        self.assertNotIn("cd /r/my dir", cmd)
        self.assertIn("--filter /HelloWorld.java", cmd)
        with self.assertRaises(ValueError):
            dist_e2e.new_lease("x;id", "h", "/r", "c", "tag")


class CorruptStateTest(unittest.TestCase):
    def test_backup_and_raise(self):
        with tempfile.TemporaryDirectory() as t:
            p = Path(t) / "state_jdk21.json"
            p.write_text('{"passed": {"A": ')
            with self.assertRaises(dist_state.CorruptStateError):
                dist_state.State(p)
            backups = list(Path(t).glob("state_jdk21.json.corrupt-*"))
            self.assertEqual(len(backups), 1)
            self.assertEqual(backups[0].read_text(), '{"passed": {"A": ')

    def test_non_object_rejected(self):
        with tempfile.TemporaryDirectory() as t:
            p = Path(t) / "state_jdk21.json"
            p.write_text("[1, 2]")
            with self.assertRaises(dist_state.CorruptStateError):
                dist_state.State(p)
            self.assertEqual(p.read_text(), "[1, 2]")   # 不清空

    def test_jobboard_corrupt(self):
        with tempfile.TemporaryDirectory() as t:
            (Path(t) / "summary.json").write_text("{oops")
            with mock.patch.object(dist_job, "_log"), self.assertRaises(dist_state.CorruptStateError):
                dist_job.JobBoard(["echo"], Path(t))

    def test_atomic_write(self):
        with tempfile.TemporaryDirectory() as t:
            p = Path(t) / "s.json"
            dist_state.atomic_write_text(p, "one")
            dist_state.atomic_write_text(p, "two")
            self.assertEqual(p.read_text(), "two")
            self.assertEqual(sorted(x.name for x in Path(t).iterdir()), ["s.json"])   # 无临时文件残留

    def test_roundtrip(self):
        with tempfile.TemporaryDirectory() as t:
            p = dist_state.state_path(Path(t), 21)
            s = dist_state.State(p)
            s.mark_passed(["HelloWorld"], "dev", SHA_A)
            self.assertIn("HelloWorld", dist_state.State(p).data["passed"])


class StatePathTest(unittest.TestCase):
    def test_single_source(self):
        self.assertEqual(dist_state.state_path(Path("/r"), 25), Path("/r/state_jdk25.json"))

    def test_paths_and_patrol_per_jdk(self):
        with tempfile.TemporaryDirectory() as t:
            d = Path(t)
            dist_state.state_path(d, 21).write_text(json.dumps(
                {"passed": {"A": {}, "B": {}}, "failed": {"C": {}}, "leases": {"dev": {"test": "D"}}}))
            dist_state.state_path(d, 25).write_text("{broken")
            (d / "state_jdk25.json.corrupt-1").write_text("")
            (d / "other.json").write_text("{}")
            self.assertEqual(sorted(dist_state.state_paths(d)), [21, 25])
            counts = {jdk: rest for jdk, *rest in patrol_status.spot_counts(d)}
            self.assertEqual(counts[21], [2, {"C": {}}, 1])
            self.assertIn("<状态损坏>", counts[25][1])
            self.assertEqual((d / "state_jdk25.json").read_text(), "{broken")   # 巡检只读
            lines = patrol_status.spot_lines("x", d, False, 3)
            self.assertEqual(len(lines), 2)
            self.assertIn("[JDK21]", lines[0])


class ExitCodeTest(unittest.TestCase):
    def test_job(self):
        ok = {"01": {"status": "done", "rc": 0}}
        self.assertEqual(dist_job.job_exit_code(ok, False), 0)
        self.assertEqual(dist_job.job_exit_code(ok, True), 130)
        self.assertEqual(dist_job.job_exit_code({"01": {"status": "done", "rc": 1}}, False), 1)
        self.assertEqual(dist_job.job_exit_code({"01": {"status": "pending"}}, False), 1)

    def test_e2e(self):
        with tempfile.TemporaryDirectory() as t:
            s = dist_state.State(dist_state.state_path(Path(t), 21))
            s.mark_passed(["A"], "dev", SHA_A)
            pool = mock.Mock(remaining=mock.Mock(return_value=0))
            self.assertEqual(e2e_exit_code(s, ["A"], pool, False), 0)
            self.assertEqual(e2e_exit_code(s, ["A"], pool, True), 130)
            self.assertEqual(e2e_exit_code(s, ["A", "B"], pool, False), 1)


class HostKeyPolicyTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.kh = Path(self.tmp.name) / "known_hosts"
        self.key = paramiko.ECDSAKey.generate()
        p = mock.patch.object(ssh_client, "known_hosts_path", return_value=self.kh)
        p.start()
        self.addCleanup(p.stop)
        self.addCleanup(self.tmp.cleanup)

    def test_reject_by_default(self):
        with mock.patch.object(ssh_client, "TRUST_NEW_HOSTS", False):
            c = ssh_client._new_client()
            self.assertIsInstance(c._policy, ssh_client._RejectUnknownHost)
            with mock.patch("sys.stderr"), self.assertRaises(ssh_client.HostKeyRejected):
                c._policy.missing_host_key(c, "[h]:2222", self.key)
        self.assertFalse(self.kh.exists())

    def test_tofu_records_once(self):
        with mock.patch.object(ssh_client, "TRUST_NEW_HOSTS", True):
            c = ssh_client._new_client()
            with mock.patch("sys.stderr"):
                c._policy.missing_host_key(c, "[h]:2222", self.key)
                c._policy.missing_host_key(c, "[h]:2222", self.key)
        self.assertEqual(len(self.kh.read_text().splitlines()), 1)
        # 登记后的新客户端从 rava known_hosts 认出该主机
        c2 = ssh_client._new_client()
        self.assertEqual(c2.get_host_keys().lookup("[h]:2222")[self.key.get_name()], self.key)

    def test_changed_key_rejected(self):
        ssh_client.record_host_key("h", self.key)
        with self.assertRaises(paramiko.BadHostKeyException):
            ssh_client.record_host_key("h", paramiko.ECDSAKey.generate())

    def test_key_name(self):
        self.assertEqual(ssh_client.host_key_name("1.2.3.4", 22), "1.2.3.4")
        self.assertEqual(ssh_client.host_key_name("1.2.3.4", 2222), "[1.2.3.4]:2222")


if __name__ == "__main__":
    unittest.main()
