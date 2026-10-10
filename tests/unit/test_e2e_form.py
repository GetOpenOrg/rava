"""e2e 形态（scripts/e2e_form.py）的单元测试（不依赖 JDK / mvn）：
    python3 -m unittest tests.unit.test_e2e_form
"""

import os
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path

_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(_ROOT, 'scripts'))

import e2e_form as ef


def _jar(path: Path, *classes: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(path, 'w') as z:
        for c in classes:
            z.writestr(c + '.class', b'')


class FormTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.t = Path(self.tmp.name)
        ef.set_pilot_libs(None)

    def tearDown(self):
        self.tmp.cleanup()
        ef.set_pilot_libs(None)

    def _lock(self, d: Path) -> Path:
        _jar(d / 'libs' / 'b-1.0.jar', 'b/B')
        _jar(d / 'libs' / 'a-2.jar', 'a/A')
        lock = d / 'deps.lock.toml'
        lock.write_text('release = 21\n'
                        '[[jar]]\ncoordinate = "g:bee:1.0"\npath = "libs/b-1.0.jar"\nsha256 = "x"\n'
                        '[[jar]]\npath = "libs/a-2.jar"\nsha256 = "y"\n')
        return lock

    def _case_dir(self, deps: str, cp: str, extra: str = '') -> Path:
        d = self.t / 'case'
        d.mkdir(exist_ok=True)
        (d / 'form.toml').write_text(f'deps = "{deps}"\ncp = {cp}\n{extra}')
        (d / 'T.java').write_text('class T {}')
        return d

    def test_no_form(self):
        d = self.t / 'plain'
        d.mkdir()
        (d / 'T.java').write_text('')
        self.assertIsNone(ef.form_of(d / 'T.java'))

    def test_entry_names_and_lock_order(self):
        lock = self._lock(self.t)
        self.assertEqual([n for n, _ in ef.lock_entries(lock)], ['bee', 'a-2'])
        d = self._case_dir(str(lock), '["a-2", "bee"]')
        f = ef.form_of(d / 'T.java')
        self.assertIsNone(f.ensure())
        self.assertEqual(f.deps_args(), ['--deps', str(lock.resolve()), '--cp', 'a-2,bee'])
        # 类路径取锁序，与 cp 给出序无关
        self.assertEqual([p.name for p in f.classpath()], ['b-1.0.jar', 'a-2.jar'])
        cp = ef.java_classpath(Path('/c'), f).split(os.pathsep)
        self.assertEqual(cp[0], '/c')
        self.assertEqual(len(cp), 3)

    def test_missing_lock_fails_without_fetch(self):
        d = self._case_dir(str(self.t / 'nope.toml'), '["x"]')
        err = ef.form_of(d / 'T.java').ensure()
        self.assertIn('依赖锁不存在', err)

    def test_unknown_entry_and_missing_jar(self):
        lock = self._lock(self.t)
        d = self._case_dir(str(lock), '["zzz"]')
        self.assertIn('不在依赖锁中', ef.form_of(d / 'T.java').ensure())
        ef.set_pilot_libs(None)
        (self.t / 'libs' / 'a-2.jar').unlink()
        d = self._case_dir(str(lock), '["a-2"]')
        self.assertIn('jar 不存在', ef.form_of(d / 'T.java').ensure())

    def test_fetch_once_then_ready(self):
        lock_dir = self.t / 'gen'
        script = self.t / 'fetch.sh'
        counter = self.t / 'count'
        src = self.t / 'src'
        self._lock(src)
        script.write_text(f'#!/bin/sh\necho x >> "{counter}"\ncp -r "{src}" "{lock_dir}"\n')
        script.chmod(0o755)
        d = self._case_dir(str(lock_dir / 'deps.lock.toml'), '["bee"]', f'fetch = "{script}"\n')
        f = ef.form_of(d / 'T.java')
        self.assertIsNone(f.ensure())
        self.assertIsNone(f.ensure())
        self.assertEqual(counter.read_text().count('x'), 1)

    def test_bad_form(self):
        d = self.t / 'bad'
        d.mkdir()
        (d / 'form.toml').write_text('deps = ""\ncp = []\n')
        with self.assertRaises(ef.FormError):
            ef.form_of(d / 'T.java')

    def test_pilot_libs_rebases_lock(self):
        alt = self.t / 'alt'
        self._lock(alt)
        ef.set_pilot_libs(alt / 'libs')
        d = self._case_dir('tests/lib_pilot/deps/target/deps.lock.toml', '["bee"]')
        f = ef.form_of(d / 'T.java')
        self.assertEqual(f.lock_path(), (alt / 'deps.lock.toml').resolve())
        self.assertIsNone(f.ensure())


if __name__ == '__main__':
    unittest.main()
