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
  - 扫描来源：macOS brew Cellar（openjdk@NN 与裸 openjdk 即最新版两种命名）；
    找不到时回退 /usr/libexec/java_home -v N。
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
