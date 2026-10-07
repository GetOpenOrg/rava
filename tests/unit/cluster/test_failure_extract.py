"""失败日志保全单元测试：诊断块 / panic 摘取、上限截断、runner 收尾拷贝片段（本机 bash 模拟服务器目录）。

运行：cd rava && uv run python -m unittest tests.test_failure_extract
"""

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE.parents[2] / "scripts" / "cluster"))

import failure_extract as fx  # noqa: E402

BUILD_LOG = """\
   Compiling java_runtime v0.1.0 (/data/x/build/jdk21/hello_world/java_runtime)
warning: unused variable: `a`
  --> java_runtime/src/a.rs:1:5
   |
1  |     let a = 1;
   |         ^ help: prefix it with an underscore: `_a`

error[E0782]: expected a type, found a trait
  --> java_runtime/src/java/lang/thread.rs:120:22
   |
120|     fn carrier(&self) -> Runnable {
   |                          ^^^^^^^^
   |
help: you can add the `dyn` keyword if you want a trait object
   |
120|     fn carrier(&self) -> dyn Runnable {
   |                          +++

error[E0308]: mismatched types
  --> java_runtime/src/b.rs:9:1
   |
9  | x
   | ^ expected `i32`, found `i64`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0782.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `java_runtime` (lib) due to 2 previous errors
"""

RUN_LOG = """\
thread 'main' (3019181) panicked at java_runtime/src/reflect_dispatch.rs:186:5:
stub: L3 反射分派未覆盖 java/util/ArrayList.<alloc>:()V（分派闭包缺席 / 方法未发射）
stack backtrace:
""" + "".join(f"  {i}: frame_{i}\n             at src/x.rs:{i}:1\n" for i in range(2000))


class ExtractTest(unittest.TestCase):
    def test_error_blocks(self):
        blocks, summary = fx.build_error_blocks(BUILD_LOG)
        self.assertEqual(len(blocks), 2)
        self.assertIn("--> java_runtime/src/java/lang/thread.rs:120:22", blocks[0])
        self.assertIn("help: you can add the `dyn` keyword", blocks[0])
        self.assertIn("fn carrier(&self) -> dyn Runnable", blocks[0])
        self.assertNotIn("unused variable", "\n".join(blocks))
        self.assertTrue(blocks[1].startswith("error[E0308]"))
        self.assertIn("error: could not compile `java_runtime` (lib) due to 2 previous errors", summary)
        self.assertIn("error: aborting due to 2 previous errors", summary)

    def test_dedupe_and_many(self):
        blk = "error[E0599]: no method named `x{}` found\n  --> a.rs:{}:1\n   |\n\n"
        text = "".join(blk.format(i, i) for i in range(6000)) + blk.format(1, 1)
        blocks, _ = fx.build_error_blocks(text)
        self.assertEqual(len(blocks), 6000)
        out = fx.compose("hdr\n", "out\n", text, None)
        self.assertLessEqual(len(out.encode()), fx.MAX_TOTAL_BYTES)
        self.assertIn("error 块 6000 个", out)
        self.assertIn("超出单例上限", out)

    def test_panic(self):
        secs = fx.panic_sections(RUN_LOG)
        self.assertEqual(len(secs), 1)
        self.assertIn("stub: L3 反射分派未覆盖 java/util/ArrayList.<alloc>:()V", secs[0])
        self.assertIn("frame_0", secs[0])
        self.assertNotIn("frame_150", secs[0])
        out = fx.compose("hdr\n", "out\n", None, RUN_LOG)
        self.assertIn("run.log 摘要（panic 1 处", out)

    def test_small_run_log_kept(self):
        out = fx.compose("hdr\n", "out\n", None, "thread 'main' panicked at a.rs:1:1:\nboom\n")
        self.assertIn("=== run.log ===", out)
        self.assertIn("boom", out)

    def test_out_log_head_tail(self):
        big = "A" * 100_000 + "TAIL_MARK"
        out = fx.compose("hdr\n", big, None, None)
        self.assertIn("TAIL_MARK", out)
        self.assertIn("字节省略", out)


class CollectPostTest(unittest.TestCase):
    """dist_e2e._collect_post 片段在本机 bash 下的行为（不 import dist_e2e：它依赖 paramiko）。"""

    def _post(self, remote_dir, bin_name):
        import importlib.util
        import types
        # 只取函数源码执行，避免连带 import 整个调度栈
        src = (_HERE.parents[2] / "scripts" / "cluster" / "dist_e2e.py").read_text(encoding="utf-8")
        start = src.index("def _logs_dir(")
        end = src.index("def new_lease(")
        start2 = src.index("def _collect_post(")
        end2 = src.index("def _write_error_log(")
        ns = {"shlex": __import__("shlex"), "MAX_REMOTE_LOG": 64}
        exec(src[start:end] + src[start2:end2], ns)
        return ns["_collect_post"](remote_dir, 21, bin_name)

    def test_new_layout_and_fallback(self):
        with tempfile.TemporaryDirectory() as t:
            wd = Path(t) / "wd"
            scratch = wd / "build/jdk21/hello_world/logs"
            scratch.mkdir(parents=True)
            (scratch / "build.log").write_text("error[E0782]: x\n" + "y" * 200)
            (wd / "build/jdk21/hello_world/build_status.json").write_text('{"ok": false}')
            logs = wd / "build/jdk21/logs"
            logs.mkdir(parents=True)
            (logs / "hello_world.run.log").write_text("panic\n")
            d = Path(t) / "run"
            d.mkdir()
            (d / "out.log").write_text(f"[FAIL] x（全文 → {scratch}/build.log）\n")
            subprocess.run(["bash", "-c", f'd={d}; ' + self._post(str(wd), "hello_world")], check=True)
            got = (d / "hello_world.build.log").read_text()
            self.assertTrue(got.startswith("error[E0782]"))
            self.assertIn("服务器端截断", got)
            self.assertEqual((d / "hello_world.run.log").read_text(), "panic\n")
            self.assertEqual((d / "hello_world.build_status.json").read_text(), '{"ok": false}')

    def test_old_layout(self):
        with tempfile.TemporaryDirectory() as t:
            wd = Path(t) / "wd"
            logs = wd / "build/jdk21/logs"
            logs.mkdir(parents=True)
            (logs / "foo.build.log").write_text("error: old\n")
            d = Path(t) / "run"
            d.mkdir()
            (d / "out.log").write_text("")
            subprocess.run(["bash", "-c", f'd={d}; ' + self._post(str(wd), "foo")], check=True)
            self.assertEqual((d / "foo.build.log").read_text(), "error: old\n")
            self.assertFalse((d / "foo.run.log").exists())


if __name__ == "__main__":
    unittest.main()
