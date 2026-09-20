"""
JDK 类文件解析器：从 JDK jmod 档案中按需提取 .class 字节。

支持 JDK 11+（jmod 格式），不依赖 jar 工具，纯 Python zipfile 读取。
使用方法：
    resolver = JdkResolver()
    data = resolver.resolve('java/util/ArrayList')  # 返回 bytes 或 None
"""

import os
import subprocess
import zipfile
from pathlib import Path
from functools import lru_cache
from typing import Optional

# 按优先级排列：先查最核心的模块，再逐步扩展
_JMOD_PRIORITY = [
    'java.base.jmod',
    'java.desktop.jmod',
    'java.logging.jmod',
    'java.xml.jmod',
    'java.sql.jmod',
    'java.net.http.jmod',
]

# JDK 内置类的包前缀（binary name 斜线分隔）
_JDK_PREFIXES = (
    'java/',
    'javax/',
    'jdk/',
    'sun/',
    'com/sun/',
    'com/oracle/',
    'org/xml/',
    'org/w3c/',
    'org/ietf/',
)


def _installed_jdks() -> list[tuple[int, Path]]:
    """本机已安装的 JDK（macOS brew + Linux /usr/lib/jvm）：
    (主版本, JAVA_HOME)，按版本升序。与 scripts/jdk_select.py 同源逻辑。"""
    import re as _re
    result: list[tuple[int, Path]] = []

    def _add(home: Path) -> None:
        if not (home / 'jmods').is_dir():
            return
        m = (_re.search(r'openjdk@(\d+)', str(home))
             or _re.search(r'Cellar/openjdk/(\d+)', str(home))
             or _re.search(r'java-(\d+)-openjdk', str(home))
             or _re.search(r'/openjdk-(\d+)', str(home)))
        if m:
            result.append((int(m.group(1)), home))

    cellar = Path('/opt/homebrew/Cellar')
    if cellar.is_dir():
        for formula in cellar.iterdir():
            if not formula.name.startswith('openjdk'):
                continue
            for version_dir in formula.iterdir():
                _add(version_dir / 'libexec' / 'openjdk.jdk' / 'Contents' / 'Home')
    jvm_dir = Path('/usr/lib/jvm')
    if jvm_dir.is_dir():
        for jvm in jvm_dir.iterdir():
            if jvm.is_dir():
                _add(jvm)
    return sorted(result)


def find_java_home(prefer_major: int | None = None) -> Path:
    """
    按优先级找到 JAVA_HOME：
    1. JAVA_HOME 环境变量（--jdk 显式指定会写入此变量，用户意图优先）
    2. prefer_major 精确匹配的已安装 JDK（用户 .class 文件版本自动推导，
       见 callchain 的语料选择）
    3. prefer_major 的最小上界已安装 JDK（更新语料是 API 超集）
    4. java 可执行文件路径推断
    5. macOS brew 下的 openjdk（21 优先，然后 17）
    """
    java_home_env = os.environ.get('JAVA_HOME', '')
    if java_home_env:
        p = Path(java_home_env)
        if (p / 'jmods').is_dir():
            return p

    if prefer_major:
        installed = _installed_jdks()
        exact = next((h for m, h in installed if m == prefer_major), None)
        if exact is not None:
            return exact
        upper = next((h for m, h in installed if m >= prefer_major), None)
        if upper is not None:
            return upper

    # 通过 java -XshowSettings:all 取 java.home 系统属性
    try:
        result = subprocess.run(
            ['java', '-XshowSettings:all', '-version'],
            capture_output=True, text=True, timeout=5
        )
        for line in (result.stdout + result.stderr).splitlines():
            if 'java.home' in line:
                home_str = line.split('=', 1)[-1].strip()
                p = Path(home_str)
                # java.home 可能指向 jre 子目录
                if not (p / 'jmods').is_dir():
                    p = p.parent
                if (p / 'jmods').is_dir():
                    return p
    except (FileNotFoundError, subprocess.TimeoutExpired):
        pass

    # 常见安装路径（macOS brew / Linux jvm 目录；不存在的自动跳过）
    brew_candidates = [
        '/opt/homebrew/Cellar/openjdk@21/21.0.11/libexec/openjdk.jdk/Contents/Home',
        '/opt/homebrew/Cellar/openjdk@17/17.0.20.1/libexec/openjdk.jdk/Contents/Home',
        '/opt/homebrew/opt/openjdk/libexec/openjdk.jdk/Contents/Home',
        '/Library/Java/JavaVirtualMachines/openjdk-21.jdk/Contents/Home',
        '/Library/Java/JavaVirtualMachines/openjdk-17.jdk/Contents/Home',
        '/usr/lib/jvm/java-21-openjdk-amd64',
        '/usr/lib/jvm/java-17-openjdk-amd64',
        '/usr/lib/jvm/java-11-openjdk-amd64',
    ]
    for candidate in brew_candidates:
        p = Path(candidate)
        if (p / 'jmods').is_dir():
            return p

    raise RuntimeError(
        "找不到 JAVA_HOME 目录（含 jmods/ 子目录）。"
        "请设置 JAVA_HOME 环境变量或安装 JDK 17+。"
    )


class JdkResolver:
    """
    从 JDK jmod 文件中按需解析 .class 字节码。

    每个 jmod 是 zip 格式，.class 文件路径为 'classes/<binary_name>.class'。
    首次访问某个 jmod 时打开，整个生命周期内保持打开。
    """

    def __init__(self, java_home: Optional[Path | str] = None,
                 prefer_major: int | None = None):
        if java_home is None:
            java_home = find_java_home(prefer_major=prefer_major)
        self._home = Path(java_home)
        self._jmods_dir = self._home / 'jmods'
        # jmod 文件名 → zipfile.ZipFile | None（None 表示打开失败）
        self._opened: dict[str, Optional[zipfile.ZipFile]] = {}
        # 延迟初始化：binary_name → jmod_filename 缓存
        self._name_to_jmod: dict[str, str] = {}

    def _open_jmod(self, jmod_name: str) -> Optional[zipfile.ZipFile]:
        if jmod_name in self._opened:
            return self._opened[jmod_name]
        path = self._jmods_dir / jmod_name
        if not path.exists():
            self._opened[jmod_name] = None
            return None
        try:
            zf = zipfile.ZipFile(path, 'r')
            self._opened[jmod_name] = zf
            return zf
        except zipfile.BadZipFile:
            self._opened[jmod_name] = None
            return None

    def _available_jmods(self) -> list[str]:
        """返回所有 .jmod 文件名，优先级排序。"""
        if not self._jmods_dir.is_dir():
            return []
        all_jmods = [f.name for f in self._jmods_dir.iterdir() if f.suffix == '.jmod']
        # 优先级排序
        ordered = [j for j in _JMOD_PRIORITY if j in all_jmods]
        rest    = sorted(j for j in all_jmods if j not in ordered)
        return ordered + rest

    def resolve(self, binary_name: str) -> Optional[bytes]:
        """
        查找并返回 binary_name 对应的 .class 字节。
        binary_name 形式：'java/util/ArrayList'（斜线分隔，无 .class 后缀）。
        返回 None 表示 JDK 中不存在。
        """
        if binary_name in self._name_to_jmod:
            jmod_name = self._name_to_jmod[binary_name]
            zf = self._open_jmod(jmod_name)
            if zf is None:
                return None
            entry = 'classes/' + binary_name + '.class'
            try:
                return zf.read(entry)
            except KeyError:
                return None

        # 按优先级扫描 jmods
        entry = 'classes/' + binary_name + '.class'
        for jmod_name in self._available_jmods():
            zf = self._open_jmod(jmod_name)
            if zf is None:
                continue
            try:
                data = zf.read(entry)
                self._name_to_jmod[binary_name] = jmod_name
                return data
            except KeyError:
                continue

        return None

    def is_jdk_class(self, binary_name: str) -> bool:
        """判断 binary_name 是否属于 JDK 内置类（无需解析字节码判断）。"""
        return binary_name.startswith(_JDK_PREFIXES)

    def list_module(self, module_jmod: str) -> list[str]:
        """
        列出指定 jmod 中所有 .class 文件的 binary name（去掉 'classes/' 前缀和 '.class' 后缀）。
        用于调试/探索。
        """
        zf = self._open_jmod(module_jmod)
        if zf is None:
            return []
        result = []
        for name in zf.namelist():
            if name.startswith('classes/') and name.endswith('.class'):
                result.append(name[len('classes/'):-len('.class')])
        return sorted(result)

    def close(self):
        """关闭所有打开的 jmod 文件。"""
        for zf in self._opened.values():
            if zf is not None:
                try:
                    zf.close()
                except Exception:
                    pass
        self._opened.clear()

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()
