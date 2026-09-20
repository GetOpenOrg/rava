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

import re
import subprocess
from pathlib import Path

_CELLAR = Path('/opt/homebrew/Cellar')


def _major_of(home: Path) -> int | None:
    """从安装路径推断主版本：openjdk@21/21.0.11 → 21；openjdk/26.0.2 → 26。"""
    m = re.search(r'openjdk@(\d+)', str(home))
    if m:
        return int(m.group(1))
    m = re.search(r'Cellar/openjdk/(\d+)', str(home))
    if m:
        return int(m.group(1))
    return None


def list_installed_jdks() -> list[tuple[int, Path]]:
    """列出本机 brew 安装的 JDK：(主版本, JAVA_HOME) 按版本升序。"""
    result: list[tuple[int, Path]] = []
    if not _CELLAR.is_dir():
        return result
    for formula in _CELLAR.iterdir():
        if not formula.name.startswith('openjdk'):
            continue
        for version_dir in formula.iterdir():
            home = version_dir / 'libexec' / 'openjdk.jdk' / 'Contents' / 'Home'
            if not (home / 'jmods').is_dir():
                continue
            major = _major_of(home)
            if major is not None:
                result.append((major, home))
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
