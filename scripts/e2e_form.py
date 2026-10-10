"""e2e 形态（form）：用例目录级的构建单元形态声明（scripts-into-rava §六 形态接口）。

一个 e2e 目录可放 `form.toml`，把该目录下的用例当作「带第三方依赖的用户项目」构建：

    deps  = "tests/lib_pilot/deps/target/deps.lock.toml"   # 依赖锁（仓库相对路径，V12 §3.2）
    cp    = ["hamcrest", "junit"]                          # 锁条目名（类路径取锁序）
    fetch = "scripts/fetch_pilot_deps.sh --no-scan"        # 锁或 jar 缺失时的取包命令（仓库相对，可省）

形态只是数据：本模块不写任何库名 / 类名；依赖内容全部来自锁文件。无 form.toml 的目录 `form_of` 返回
None，调用方行为与无形态概念时完全一致。

取包：每个形态在一次批次内最多取包一次（首个用到它的用例触发，线程安全）；取包后锁或所选 jar 仍缺失时
该形态全部用例记失败（[`DEPS_FETCH_FAIL`]），不静默跳过。

`--pilot-libs DIR`（run_tests）改 jar 资产根：锁取 `DIR` 上级目录下与 `deps` 同名的锁文件（锁内 jar 路径
相对锁所在目录，与 `fetch_pilot_deps.sh` 的产出布局一致）；非缺省根不自动取包。

S7 拆分 run_tests 时本模块整体搬入 `scripts/e2e/select.py`。
"""
from __future__ import annotations

import fcntl
import os
import shlex
import subprocess
import threading
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FORM_FILE = "form.toml"
DEFAULT_PILOT_LIBS = ROOT / "tests" / "lib_pilot" / "deps" / "target" / "pilot-libs"
# 取包失败的失败类别（结果行 / 失败分类 / 失败棘轮共用）
DEPS_FETCH_FAIL = "deps-fetch-fail"


class FormError(Exception):
    """form.toml 本身不合法（缺字段 / 类型错）：属仓库数据错误，直接报错而非记用例失败。"""


def lock_entries(lock: Path) -> list[tuple[str, Path]]:
    """读依赖锁：[(条目名, jar 绝对路径)]，锁序。

    条目名口径与 rava `driver::deps_lock` 一致：坐标 artifactId，无坐标时 jar 文件名去 `.jar`。"""
    data = tomllib.loads(lock.read_text(encoding="utf-8"))
    out: list[tuple[str, Path]] = []
    for e in data.get("jar", []):
        rel = e["path"]
        coord = e.get("coordinate", "")
        parts = coord.split(":")
        name = parts[1] if len(parts) > 1 and parts[1] else rel.rsplit("/", 1)[-1].removesuffix(".jar")
        out.append((name, (lock.parent / rel).resolve()))
    return out


@dataclass
class Form:
    """一个形态目录的声明与（批次内）取包状态。"""
    dir: Path
    deps: str
    cp: tuple[str, ...]
    fetch: str | None = None
    pilot_libs: Path | None = None
    # 批次内状态：是否已就绪检查过、错误原因（None = 就绪）、取包锁
    _checked: bool = field(default=False, repr=False)
    _error: str | None = field(default=None, repr=False)
    _jars: list[Path] = field(default_factory=list, repr=False)
    _lock: threading.Lock = field(default_factory=threading.Lock, repr=False)

    def lock_path(self) -> Path:
        """依赖锁绝对路径（`--pilot-libs` 非缺省时改取其上级目录下的同名锁）。"""
        if self.pilot_libs is not None and self.pilot_libs.resolve() != DEFAULT_PILOT_LIBS.resolve():
            return self.pilot_libs.resolve().parent / Path(self.deps).name
        return (ROOT / self.deps).resolve()

    def _missing(self) -> str | None:
        lock = self.lock_path()
        if not lock.is_file():
            return f"依赖锁不存在：{lock}"
        try:
            entries = dict(lock_entries(lock))
        except (OSError, ValueError, KeyError) as e:
            return f"依赖锁不可读：{lock}：{e}"
        unknown = [n for n in self.cp if n not in entries]
        if unknown:
            return f"cp 条目不在依赖锁中：{', '.join(unknown)}（{lock}）"
        absent = [str(entries[n]) for n in self.cp if not entries[n].is_file()]
        if absent:
            return f"锁内 jar 不存在：{', '.join(absent)}"
        return None

    def _can_fetch(self) -> bool:
        return bool(self.fetch) and self.lock_path() == (ROOT / self.deps).resolve()

    def _fetch_locked(self, err: str) -> str | None:
        """跨进程互斥取包（同一检出下多槽 run_tests 并发）：持锁后先复查，别的进程已取好则不再取。"""
        lock_file = ROOT / "build" / ".form_fetch.lock"
        lock_file.parent.mkdir(parents=True, exist_ok=True)
        with open(lock_file, "w") as lf:
            fcntl.flock(lf, fcntl.LOCK_EX)
            try:
                if (err := self._missing()) is None:
                    return None
                cmd = shlex.split(self.fetch)
                print(f"[form] {self.dir}：{err}——取包：{self.fetch}", flush=True)
                r = subprocess.run([str(ROOT / cmd[0]), *cmd[1:]], cwd=ROOT,
                                   capture_output=True, text=True)
                err = self._missing()
                if err is not None and r.returncode != 0:
                    tail = (r.stderr.strip() or r.stdout.strip()).splitlines()[-3:]
                    err += f"；取包退出 {r.returncode}：{' | '.join(tail)}"
                return err
            finally:
                fcntl.flock(lf, fcntl.LOCK_UN)

    def ensure(self) -> str | None:
        """就绪检查（批次内只做一次；缺失且可取包时取包一次）。返回 None = 就绪，否则为失败原因。"""
        with self._lock:
            if self._checked:
                return self._error
            err = self._missing()
            if err is not None and self._can_fetch():
                err = self._fetch_locked(err)
            self._error = err
            if err is None:
                sel = set(self.cp)
                self._jars = [p for n, p in lock_entries(self.lock_path()) if n in sel]
            self._checked = True
            return err

    def deps_args(self) -> list[str]:
        """`rava build` 的依赖参数：`--deps <锁> --cp <条目名,…>`。"""
        return ["--deps", str(self.lock_path()), "--cp", ",".join(self.cp)]

    def classpath(self) -> list[Path]:
        """依赖 jar 绝对路径（锁序；须先 [`Self::ensure`] 就绪）。"""
        return list(self._jars)


_FORMS: dict[Path, Form | None] = {}
_FORMS_LOCK = threading.Lock()
_PILOT_LIBS: Path | None = None


def set_pilot_libs(path: Path | None) -> None:
    """run_tests `--pilot-libs`：jar 资产根（缺省 tests/lib_pilot/deps/target/pilot-libs）。"""
    global _PILOT_LIBS
    _PILOT_LIBS = path
    with _FORMS_LOCK:
        _FORMS.clear()


def load_form(d: Path) -> Form | None:
    """读目录 d 的 form.toml（无则 None）。"""
    f = d / FORM_FILE
    if not f.is_file():
        return None
    try:
        data = tomllib.loads(f.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as e:
        raise FormError(f"{f}：{e}") from e
    deps, cp, fetch = data.get("deps"), data.get("cp"), data.get("fetch")
    if not isinstance(deps, str) or not deps:
        raise FormError(f"{f}：deps 须为非空字符串（依赖锁的仓库相对路径）")
    if not isinstance(cp, list) or not cp or not all(isinstance(x, str) and x for x in cp):
        raise FormError(f"{f}：cp 须为非空字符串数组（锁条目名）")
    if fetch is not None and not isinstance(fetch, str):
        raise FormError(f"{f}：fetch 须为字符串（仓库相对命令）")
    return Form(dir=d.resolve(), deps=deps, cp=tuple(cp), fetch=fetch, pilot_libs=_PILOT_LIBS)


def form_of(java_file: Path) -> Form | None:
    """用例所在目录的形态（按目录缓存；无 form.toml 返回 None）。"""
    d = java_file.resolve().parent
    with _FORMS_LOCK:
        if d not in _FORMS:
            _FORMS[d] = load_form(d)
        return _FORMS[d]


def java_classpath(classes: Path, form: Form | None) -> str:
    """javac / java 的 `-cp` 串：用户类目录在前，依赖 jar 按锁序在后。"""
    parts = [str(classes)] + ([str(p) for p in form.classpath()] if form else [])
    return os.pathsep.join(parts)
