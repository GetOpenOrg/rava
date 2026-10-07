"""remote_run 单元测试：用本机 bash 模拟 SSH 客户端，验证发起 / 轮询 / 断线续读 / 接上 / 超时 / 已死判定。

运行：cd rava && python3 -m unittest tests.test_remote_run
"""

import os
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

import remote_run  # noqa: E402

remote_run.POLL_INTERVAL = 0.2


class _Chan:
    def __init__(self, rc):
        self.rc = rc
    def settimeout(self, t): pass
    def recv_exit_status(self): return self.rc
    def close(self): pass


class _Out:
    def __init__(self, data, rc):
        self.data, self.channel = data, _Chan(rc)
    def read(self): return self.data


class _In:
    def close(self): pass


class _Transport:
    def __init__(self, client):
        self.client = client
    def close(self):
        self.client.closed = True
    def is_active(self):
        return not self.client.closed


class FakeClient:
    """exec_command 在本机 sh 执行；transport 关闭后再执行即抛 EOFError（模拟断线）。"""
    instances = 0

    def __init__(self):
        self.closed = False
        FakeClient.instances += 1

    def exec_command(self, cmd, timeout=None, get_pty=False):
        if self.closed:
            raise EOFError("SSH session not active")
        r = subprocess.run(["/bin/sh", "-c", cmd], capture_output=True, timeout=timeout)
        return _In(), _Out(r.stdout, r.returncode), _Out(r.stderr, 0)

    def get_transport(self):
        return _Transport(self)

    def close(self):
        self.closed = True


def _connect(server):
    return FakeClient(), "测试"


SERVER = {"label": "t"}


class RemoteRunTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.mkdtemp()
        self.run_dir = f"{self.tmp}/build/dist_runs/X-jdk21-abc-tag"
        os.environ.pop(remote_run.INJECT_ENV, None)

    def _run(self, cmd, **kw):
        lines = []
        out = remote_run.run_detached(SERVER, kw.pop("client", FakeClient()), self.run_dir, cmd,
                                      on_line=lines.append, connect=_connect,
                                      timeout_at=kw.pop("timeout_at", time.time() + 60), **kw)
        return out, lines

    def test_launch_and_collect(self):
        out, lines = self._run("echo a; sleep 0.5; echo b; exit 3",
                               post=f'echo copied > {self.run_dir}/extra.log')
        self.assertEqual((out.status, out.rc, out.attached), ("done", 3, False))
        self.assertEqual(lines, ["a", "b"])
        self.assertTrue(Path(self.run_dir, "extra.log").exists())
        self.assertTrue(Path(self.run_dir, "done").exists())

    def test_drop_reconnect_resume(self):
        os.environ[remote_run.INJECT_ENV] = "3"
        out, lines = self._run("for i in 1 2 3 4 5 6; do echo L$i; sleep 0.3; done")
        self.assertEqual((out.status, out.rc), ("done", 0))
        self.assertEqual(out.reconnects, 1)
        self.assertEqual(lines, [f"L{i}" for i in range(1, 7)])   # 不丢不重
        # 只发起过一次：runner.sh 仅一份、out.log 内容与读回一致
        self.assertEqual(Path(self.run_dir, "out.log").read_text().split(), lines)

    def test_attach_running_and_done(self):
        c = FakeClient()
        remote_run.launch(c, self.run_dir, "echo x; sleep 1; echo y")
        out, lines = self._run("echo SHOULD_NOT_RUN")
        self.assertEqual((out.status, out.attached), ("done", True))
        self.assertEqual(lines, ["x", "y"])
        # 已 done：再跟随直接收结果，不重新发起
        out2, lines2 = self._run("echo SHOULD_NOT_RUN")
        self.assertEqual((out2.status, out2.attached, lines2), ("done", True, ["x", "y"]))

    def test_dead_without_done(self):
        c = FakeClient()
        remote_run.launch(c, self.run_dir, "sleep 30")
        remote_run._exec(c, remote_run.kill_run_script(self.run_dir))
        p = remote_run.probe(c, self.run_dir)
        self.assertEqual(p.state, "dead")
        out, _ = self._run("echo SHOULD_NOT_RUN")
        self.assertEqual((out.status, out.attached), ("dead", True))

    def test_timeout_kills(self):
        out, lines = self._run("echo start; sleep 30; echo never", timeout_at=time.time() + 1)
        self.assertEqual(out.status, "timeout")
        self.assertEqual(lines, ["start"])
        self.assertEqual(remote_run.probe(FakeClient(), self.run_dir).state, "dead")

    def test_cleanup_orphans_keep(self):
        c = FakeClient()
        keep = self.run_dir
        other = f"{self.tmp}/build/dist_runs/Other"
        remote_run.launch(c, keep, "sleep 5")
        remote_run.launch(c, other, "sleep 30")
        remote_run.cleanup_orphans(c, self.tmp, [keep])
        self.assertFalse(Path(other).exists())
        self.assertEqual(remote_run.probe(c, keep).state, "running")
        remote_run._exec(c, remote_run.kill_run_script(keep))

    def test_busy_mark_line(self):
        out, lines = self._run("echo __RAVA_SERVER_BUSY__; exit 75")
        self.assertEqual((out.status, out.rc, lines), ("done", 75, ["__RAVA_SERVER_BUSY__"]))


if __name__ == "__main__":
    unittest.main()
