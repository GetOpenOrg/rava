"""生成器选择：Python codegen 与 Rust 生成器（`rava build`）的唯一定义点。

优先级：命令行 `--generator` > 环境变量 `RAVA_GENERATOR` > DEFAULT_GENERATOR。
缺省为 'rust'（2026-10-01 用户决定；P5b 验收集 27 例生成树与 Python 逐字节一致）。Rust 生成器此后
允许为降低编译成本偏离 Python 基线。Rust 路径已支持 main.py 全部选项；Python 路径保留为对照基线，
删除条件见 docs/plans/2026-10-01-python-generator-deletion.md。

Rust 路径只替换「转译」段：overlay（main.py prepare_scratch）与 cargo 流程与 Python 路径共用；
`rava build --no-run` 在同一 scratch 内完成 javac → 闭包分析 → 发射；闭包结果进程内直传发射层，
closure.json 只在 `closure_json=True`（main.py `--closure-json`）时落 `<scratch>/closure_input/`
（动态对照、生成树对照、`rava emit` 需要它）。
"""

import os
import subprocess
import sys

DEFAULT_GENERATOR = 'rust'
CHOICES = ('python', 'rust')
ENV_VAR = 'RAVA_GENERATOR'

_REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_GENERATOR_MANIFEST = os.path.join(_REPO, 'generator', 'Cargo.toml')
# 与 codegen/closure_input.py 的闭包分析器共用构建目录（同一 rava 二进制，避免重复编译）
_RAVA_TARGET = os.path.join(_REPO, 'build', 'analyzer-target')
_RUNTIME = os.path.join(_REPO, 'runtime', 'java_runtime')


def add_argument(ap) -> None:
    ap.add_argument('--generator', choices=CHOICES, default=None,
                    help=f'生成器实现（缺省取环境变量 {ENV_VAR}，再缺省 {DEFAULT_GENERATOR}）：'
                         'python = codegen/；rust = generator/ 的 rava build')


def resolve(cli_value: str | None) -> str:
    g = cli_value or os.environ.get(ENV_VAR) or DEFAULT_GENERATOR
    if g not in CHOICES:
        sys.exit(f'{ENV_VAR}={g!r} 无效（可选 {", ".join(CHOICES)}）')
    return g


def rava_cmd(*args: str) -> list[str]:
    """rava 子命令的调用形态（release 构建，与闭包分析器共用构建目录）"""
    return ['cargo', 'run', '--release', '-q', '--manifest-path', _GENERATOR_MANIFEST,
            '--target-dir', _RAVA_TARGET, '--', *args]


def run_rust(java_files: list[str], out_dir: str, *, clean: bool = False, strict: bool, locales: tuple[str, ...],
             libs: tuple[str, ...] = (), batch: bool = False, debug: bool = False, trace_class: str = '',
             precheck_only: bool = False, raw_sites: str = '', closure_json: bool = False,
             extra: list[str] = ()) -> None:
    """`rava build --no-run`：（clean 时先清空 out_dir）overlay → javac → 闭包 → 发射进 out_dir。

    镜像独有 / VM 支持类目录由 rava 自行派生（resolve::image），不经 codegen。

    libs 为 main.py `--lib` 原样规格（NAME=JAR[:seed=FQN,…]），解析与校验在 rava 内完成；
    extra 为闭包诊断参数（--cut / --cut-file / --dump-edges），main.py 已转成 rava 参数"""
    home = os.environ.get('JAVA_HOME', '')
    if not home:
        sys.exit('Rust 生成器需要 JAVA_HOME（main.py 经 jdk_select.apply_jdk 设置）')
    cmd = rava_cmd('build', *[os.path.abspath(f) for f in java_files],
                   '--java-home', home, '--runtime', _RUNTIME, '--out', os.path.abspath(out_dir), '--no-run')
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
