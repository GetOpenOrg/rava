"""语料参考 JDK：scripts/fetch_reference_jdk.sh 的清单解析 / 就位判定，与 run_tests.py 的语料 JDK 选择。"""
import io
import os
import stat
import subprocess
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

REPO = Path(__file__).resolve().parents[2]
SCRIPT = REPO / "scripts" / "fetch_reference_jdk.sh"
MANIFEST = REPO / "tools" / "refjdk.toml"
sys.path.insert(0, str(REPO / "scripts"))
import run_tests as rt  # noqa: E402


def manifest_value(key: str, section: str = "") -> str:
    """测试侧独立解析清单（tomllib），与脚本的 awk 解析互相印证。"""
    import tomllib
    data = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    return (data[section] if section else data)[key]


def fake_install(root: Path, platform: str, sha: str) -> Path:
    """按脚本的落位约定造一个已就位的 JDK 目录（bin/java、bin/javac、jmods、印记）。"""
    home = root / manifest_value("tag")
    (home / "bin").mkdir(parents=True)
    (home / "jmods").mkdir()
    for tool in ("java", "javac"):
        p = home / "bin" / tool
        p.write_text("#!/bin/sh\n")
        p.chmod(p.stat().st_mode | stat.S_IXUSR)
    (home / ".rava-refjdk").write_text(
        f"tag={manifest_value('tag')}\nplatform={platform}\nsha256={sha}\nurl=x\n")
    return home


def run_script(*args: str, env: dict | None = None) -> subprocess.CompletedProcess:
    e = {k: v for k, v in os.environ.items() if k != "RAVA_REFJDK_ROOT"}
    e.update(env or {})
    return subprocess.run([str(SCRIPT), *args], capture_output=True, text=True, env=e)


class FetchScriptTest(unittest.TestCase):
    PLATFORMS = ("linux-x64", "linux-aarch64", "macos-aarch64", "macos-x64")

    def test_manifest_pins_21_0_11_for_all_platforms(self):
        self.assertEqual(manifest_value("version"), "21.0.11")
        for p in self.PLATFORMS:
            self.assertRegex(manifest_value("sha256", p), r"^[0-9a-f]{64}$")
            self.assertIn("21.0.11", manifest_value("url", p))

    def test_tag_matches_manifest(self):
        r = run_script("--tag")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(r.stdout.strip(), manifest_value("tag"))

    def test_check_missing_reports_fetch_command(self):
        with tempfile.TemporaryDirectory() as d:
            r = run_script("--check", "--root", d, "--platform", "linux-x64")
            self.assertEqual(r.returncode, 1)
            self.assertEqual(r.stdout, "")
            self.assertIn("fetch_reference_jdk.sh --root", r.stderr)

    def test_check_installed_prints_home(self):
        for p in self.PLATFORMS:
            with tempfile.TemporaryDirectory() as d:
                home = fake_install(Path(d), p, manifest_value("sha256", p))
                r = run_script("--check", "--root", d, "--platform", p)
                self.assertEqual(r.returncode, 0, r.stderr)
                self.assertEqual(r.stdout.strip(), str(home))

    def test_root_from_environment(self):
        with tempfile.TemporaryDirectory() as d:
            home = fake_install(Path(d), "linux-x64", manifest_value("sha256", "linux-x64"))
            r = run_script("--check", "--platform", "linux-x64", env={"RAVA_REFJDK_ROOT": d})
            self.assertEqual(r.returncode, 0, r.stderr)
            self.assertEqual(r.stdout.strip(), str(home))

    def test_check_rejects_other_build(self):
        # 印记的 sha256 / 平台与清单不符（清单换包、或拷来别的平台）= 未就位
        with tempfile.TemporaryDirectory() as d:
            fake_install(Path(d), "linux-x64", "0" * 64)
            self.assertEqual(run_script("--check", "--root", d, "--platform", "linux-x64").returncode, 1)
        with tempfile.TemporaryDirectory() as d:
            fake_install(Path(d), "linux-aarch64", manifest_value("sha256", "linux-x64"))
            self.assertEqual(run_script("--check", "--root", d, "--platform", "linux-x64").returncode, 1)

    def test_unknown_platform(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertEqual(run_script("--check", "--root", d, "--platform", "windows-x64").returncode, 2)


class CorpusJdkSelectionTest(unittest.TestCase):
    def setUp(self):
        self._saved = (rt.JDK_MAJOR, rt.JDK_REFERENCE, rt.RAVA, os.environ.get("JAVA_HOME"))
        self.tmp = tempfile.TemporaryDirectory()
        tmp = Path(self.tmp.name)
        # 假 rava：`rava jdk --json [--jdk N | --java-home P]` 回显选择结果
        self.fake_rava = tmp / "rava"
        self.fake_rava.write_text(
            "#!/usr/bin/env python3\n"
            "import json, sys\n"
            "a = sys.argv[1:]\n"
            "if '--java-home' in a:\n"
            "    print(json.dumps({'home': a[a.index('--java-home') + 1], 'major': 21, 'source': '--java-home'}))\n"
            "elif '--jdk' in a:\n"
            "    print(json.dumps({'home': '/sys/jdk' + a[a.index('--jdk') + 1], 'major': int(a[a.index('--jdk') + 1]),"
            " 'source': '--jdk'}))\n"
            "else:\n"
            "    sys.exit('unexpected')\n")
        self.fake_rava.chmod(0o755)
        rt.RAVA = self.fake_rava
        self.ref_home = fake_install(tmp / "root", "x", "y")

    def tearDown(self):
        rt.JDK_MAJOR, rt.JDK_REFERENCE, rt.RAVA, jh = self._saved
        if jh is None:
            os.environ.pop("JAVA_HOME", None)
        else:
            os.environ["JAVA_HOME"] = jh
        self.tmp.cleanup()

    def _apply(self, *args):
        out = io.StringIO()
        with redirect_stdout(out):
            rt.apply_jdk_choice(*args)
        return out.getvalue()

    def test_default_is_reference_and_ignores_ambient_java_home(self):
        os.environ["JAVA_HOME"] = "/usr/lib/jvm/java-21-openjdk-amd64"
        with mock.patch.object(rt, "reference_jdk", return_value=(self.ref_home, "jdk-21.0.11+10")):
            out = self._apply(None, None)
        self.assertEqual(os.environ["JAVA_HOME"], str(self.ref_home))
        self.assertEqual(rt.JDK_REFERENCE, "jdk-21.0.11+10")
        self.assertEqual(rt.JDK_MAJOR, 21)
        self.assertIn("参考构建 jdk-21.0.11+10", out)
        self.assertIn("不参与语料选择", out)

    def test_explicit_overrides_are_marked_non_reference(self):
        with mock.patch.object(rt, "reference_jdk", side_effect=AssertionError("不应查参考构建")):
            out = self._apply(25, None)
            self.assertEqual(os.environ["JAVA_HOME"], "/sys/jdk25")
            self.assertIsNone(rt.JDK_REFERENCE)
            self.assertIn("非参考构建", out)
            out = self._apply(None, "/opt/jdk")
            self.assertEqual(os.environ["JAVA_HOME"], "/opt/jdk")
            self.assertIn("非参考构建", out)

    def test_jdk_and_java_home_are_exclusive(self):
        with self.assertRaises(SystemExit):
            rt.corpus_jdk_request(21, "/opt/jdk")

    def test_missing_reference_exits_without_fallback(self):
        missing = subprocess.CompletedProcess([], 1, "", "[refjdk] 未就位\n[refjdk] 取包：scripts/fetch_reference_jdk.sh")
        with mock.patch.object(rt, "_refjdk", return_value=missing):
            with self.assertRaises(SystemExit) as cm:
                rt.corpus_jdk_request(None, None)
        self.assertIn("fetch_reference_jdk.sh", str(cm.exception.code))
        self.assertIn("--java-home", str(cm.exception.code))

    def test_reference_lookup_uses_script(self):
        with tempfile.TemporaryDirectory() as d:
            p = manifest_value("sha256", "linux-x64")
            home = fake_install(Path(d), "linux-x64", p)
            real = rt._refjdk
            with mock.patch.object(rt, "_refjdk",
                                   side_effect=lambda *a: real(*a, "--root", d, "--platform", "linux-x64")):
                self.assertEqual(rt.reference_jdk(), (home, manifest_value("tag")))


if __name__ == "__main__":
    unittest.main()
