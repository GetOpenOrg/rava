"""rava（`generator/` 的 Rust 生成器）调用形态的唯一定义点。

`rava build --no-run` 在同一 scratch 内完成 overlay → javac → 闭包分析 → 发射；闭包结果进程内直传发射层，
closure.json 只在 `closure_json=True`（main.py `--closure-json`）时落 `<scratch>/closure_input/`
（动态对照、生成树对照、`rava emit` 需要它）。
"""

import os
import subprocess
import sys

_REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_GENERATOR_MANIFEST = os.path.join(_REPO, 'generator', 'Cargo.toml')
# 各脚本共用的 rava 构建目录（同一 rava 二进制，避免重复编译）
_RAVA_TARGET = os.path.join(_REPO, 'build', 'analyzer-target')
_RUNTIME = os.path.join(_REPO, 'runtime', 'java_runtime')
_CLOSURE_CACHE = os.path.join(_REPO, 'build', 'closure_cache')


def rava_cmd(*args: str) -> list[str]:
    """rava 子命令的调用形态（release 构建，各脚本共用构建目录）"""
    return ['cargo', 'run', '--release', '-q', '--manifest-path', _GENERATOR_MANIFEST,
            '--target-dir', _RAVA_TARGET, '--', *args]


def run_rust(java_files: list[str], out_dir: str, *, clean: bool = False, strict: bool, locales: tuple[str, ...],
             libs: tuple[str, ...] = (), batch: bool = False, debug: bool = False, trace_class: str = '',
             precheck_only: bool = False, raw_sites: str = '', closure_json: bool = False,
             extra: list[str] = ()) -> None:
    """`rava build --no-run`：（clean 时先清空 out_dir）overlay → javac → 闭包 → 发射进 out_dir。

    镜像独有 / VM 支持类目录由 rava 自行派生（resolve::image）。

    libs 为 main.py `--lib` 原样规格（NAME=JAR[:seed=FQN,…]），解析与校验在 rava 内完成；
    extra 为闭包诊断参数（--cut / --cut-file / --dump-edges），main.py 已转成 rava 参数"""
    home = os.environ.get('JAVA_HOME', '')
    if not home:
        sys.exit('rava build 需要 JAVA_HOME（main.py 经 jdk_select.apply_jdk 设置）')
    cmd = rava_cmd('build', *[os.path.abspath(f) for f in java_files],
                   '--java-home', home, '--runtime', _RUNTIME, '--out', os.path.abspath(out_dir), '--no-run')
    # 闭包分析跨运行结果缓存（键覆盖分析器、JDK、手写层、用户类与全部分析参数，见 closure::cache）
    cmd += ['--closure-cache', _CLOSURE_CACHE]
    if clean:
        cmd.append('--clean')
    for loc in locales:
        cmd += ['--locale', loc]
    for spec in libs:
        cmd += ['--lib', spec]
    if trace_class:
        cmd += ['--trace-class', trace_class]
    if raw_sites:
        cmd += ['--raw-sites', os.path.abspath(raw_sites)]
    for flag, on in (('--strict', strict), ('--batch', batch), ('--debug', debug), ('--precheck-only', precheck_only),
                     ('--closure-json', closure_json)):
        if on:
            cmd.append(flag)
    cmd += list(extra)
    print(f"[rava] build {' '.join(os.path.basename(f) for f in java_files)} → {out_dir}", flush=True)
    r = subprocess.run(cmd)
    if r.returncode != 0:
        sys.exit(f'rava build 失败（{r.returncode}）')
