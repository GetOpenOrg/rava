#!/usr/bin/env python3
"""JDK 版本选择：按主版本号定位本机已安装的 JDK home。

用法（作为模块）：
    from jdk_select import resolve_jdk_home, list_installed_jdks
    home = resolve_jdk_home(25)          # -> Path('/opt/homebrew/Cellar/openjdk@25/...')
    homes = list_installed_jdks()        # -> [(21, Path), (25, Path), ...]

设计要点：
  - 与 codegen/jdk_resolver.py 的关系：resolver 的优先级 1 就是 JAVA_HOME
    环境变量——本模块只负责「把 --jdk N 解析成 JAVA_HOME 并写入环境」，
    javac / java / jmods 语料全部经同一 JAVA_HOME 取得，保证工具链同源。
  - 扫描来源：macOS brew Cellar（openjdk@NN 与裸 openjdk 即最新版两种命名）、
    Linux /usr/lib/jvm；找不到时回退 /usr/libexec/java_home -v N。
  - 自动选择（apply_jdk，main.py / run_tests.py / golden 脚本共用的唯一入口）：
    多 JDK 并存时**不随系统默认 java 或「最新已安装」漂移**——装上 JDK25 不改变
    未指定版本的跑批。优先级：
      1. 显式 --jdk N
      2. 环境变量 JAVA_RTA_JDK=N
      3. 已设置且有效的 JAVA_HOME（用户显式环境）
      4. 仓库根 .jdk-version 固定的默认主版本（语料基线版本，当前 21）
      5. 已安装的最新版（以上都不可用时的兜底）
    选中后写入 JAVA_HOME 并打印来源；javac / java / jmods 语料全部经它取得。
"""

from __future__ import annotations

import os
import re
import subprocess
from pathlib import Path

# macOS brew 前缀两种都要扫：Apple Silicon /opt/homebrew、Intel /usr/local；
# HOMEBREW_PREFIX（brew shellenv 导出）置顶覆盖自定义安装，去重避免双计
_CELLARS: list[Path] = []
for _p in (os.environ.get('HOMEBREW_PREFIX'), '/opt/homebrew', '/usr/local'):
    _c = Path(_p) / 'Cellar' if _p else None
    if _c and _c not in _CELLARS:
        _CELLARS.append(_c)
_JVM_DIR = Path('/usr/lib/jvm')             # Linux（Ubuntu/Debian 系）


def _major_of(home: Path) -> int | None:
    """从安装路径推断主版本，覆盖三种安装命名：
    - brew:  openjdk@21/21.0.11 → 21；Cellar/openjdk/26.0.2 → 26
    - Linux: java-21-openjdk-amd64 → 21；openjdk-21/ → 21（Debian 风格）
    """
    for pat in (r'openjdk@(\d+)', r'Cellar/openjdk/(\d+)',
                r'java-(\d+)-openjdk', r'/openjdk-(\d+)'):
        m = re.search(pat, str(home))
        if m:
            return int(m.group(1))
    return None


def list_installed_jdks() -> list[tuple[int, Path]]:
    """列出本机已安装的 JDK（macOS brew + Linux /usr/lib/jvm）：
    (主版本, JAVA_HOME) 按版本升序。"""
    result: list[tuple[int, Path]] = []
    seen: set[Path] = set()

    def _add(home: Path) -> None:
        if home in seen or not (home / 'jmods').is_dir():
            return
        major = _major_of(home)
        if major is not None:
            seen.add(home)
            result.append((major, home))

    # macOS brew：Cellar/openjdk@NN/<ver>/libexec/openjdk.jdk/Contents/Home
    for cellar in _CELLARS:
        if not cellar.is_dir():
            continue
        for formula in cellar.iterdir():
            if not formula.name.startswith('openjdk'):
                continue
            for version_dir in formula.iterdir():
                _add(version_dir / 'libexec' / 'openjdk.jdk' / 'Contents' / 'Home')

    # Linux：/usr/lib/jvm/java-NN-openjdk-*（Ubuntu）或 openjdk-NN（Debian）
    if _JVM_DIR.is_dir():
        for jvm in _JVM_DIR.iterdir():
            if not jvm.is_dir():
                continue
            _add(jvm)   # Ubuntu/Debian：jmods 直接在 JVM 根下

    return sorted(result)


def resolve_jdk_home(major: int | None = None) -> Path | None:
    """定位 JDK home。major=None 时取已安装的最新版。

    返回 None 表示未找到匹配安装（调用方决定是否回退默认行为）。
    """
    installed = list_installed_jdks()
    if not installed:
        return None
    if major is None:
        return installed[-1][1]
    for m, home in installed:
        if m == major:
            return home
    # brew 未命中时尝试 macOS java_home（覆盖系统安装的 JVM）；
    # java_home 对不存在的版本可能回退默认 JVM，须用 release 文件校验主版本
    # macOS java_home 兜底（不存在该命令的平台上 FileNotFoundError 被吞）
    try:
        r = subprocess.run(['/usr/libexec/java_home', '-v', str(major)],
                           capture_output=True, text=True, timeout=5)
        home = Path(r.stdout.strip())
        if r.returncode == 0 and (home / 'jmods').is_dir():
            release = (home / 'release')
            if release.exists():
                m2 = re.search(r'JAVA_VERSION="(\d+)', release.read_text())
                if m2 and int(m2.group(1)) == major:
                    return home
    except (FileNotFoundError, subprocess.TimeoutExpired):
        pass
    return None


_REPO_ROOT = Path(__file__).resolve().parent.parent
_PIN_FILE = _REPO_ROOT / '.jdk-version'


def pinned_jdk_major() -> int | None:
    """仓库固定的默认 JDK 主版本（.jdk-version 首个整数）；缺失 / 不可解析 → None。"""
    try:
        m = re.search(r'\d+', _PIN_FILE.read_text())
        return int(m.group(0)) if m else None
    except OSError:
        return None


def choose_jdk(explicit: int | None = None) -> tuple[int | None, Path, str]:
    """按优先级选择 JDK：返回 (主版本, JAVA_HOME, 来源说明)。找不到时 SystemExit。"""
    def _need(major: int, source: str) -> tuple[int, Path, str]:
        home = resolve_jdk_home(major)
        if home is None:
            installed = '\n'.join(f"  JDK {m}: {h}" for m, h in list_installed_jdks()) or '  （无）'
            raise SystemExit(f"未找到 JDK {major}（{source}）。本机已安装：\n{installed}")
        return major, home, source

    if explicit is not None:
        return _need(explicit, '--jdk')
    env_major = os.environ.get('JAVA_RTA_JDK', '').strip()
    if env_major.isdigit():
        return _need(int(env_major), 'JAVA_RTA_JDK')
    env_home = os.environ.get('JAVA_HOME', '').strip()
    if env_home and (Path(env_home) / 'jmods').is_dir():
        return _major_of(Path(env_home)), Path(env_home), 'JAVA_HOME'
    pinned = pinned_jdk_major()
    if pinned is not None:
        return _need(pinned, '.jdk-version')
    home = resolve_jdk_home(None)
    if home is None:
        raise SystemExit('未找到任何已安装的 JDK（含 jmods/），请安装 JDK 21 或设置 JAVA_HOME')
    return _major_of(home), home, '最新已安装'


def apply_jdk(explicit: int | None = None, quiet: bool = False) -> tuple[int | None, Path]:
    """选择 JDK 并写入 JAVA_HOME（本进程与子进程同源）。返回 (主版本, JAVA_HOME)。"""
    major, home, source = choose_jdk(explicit)
    os.environ['JAVA_HOME'] = str(home)
    if not quiet:
        print(f"[jdk] JAVA_HOME → {home} (JDK {major}，来源：{source})")
    return major, home


if __name__ == '__main__':
    # 命令行：python3 scripts/jdk_select.py [N]  → 打印选中的 JAVA_HOME（供 shell 脚本取用）
    import sys
    _arg = int(sys.argv[1]) if len(sys.argv) > 1 and sys.argv[1].isdigit() else None
    _m, _h, _src = choose_jdk(_arg)
    print(_h)
