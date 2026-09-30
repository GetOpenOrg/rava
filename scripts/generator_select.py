"""生成器选择：Python codegen 与 Rust 生成器（`rava build`）的唯一定义点。

优先级：命令行 `--generator` > 环境变量 `RAVA_GENERATOR` > DEFAULT_GENERATOR。
P5b 验收（验收集 27 例生成树逐字节一致）后把 DEFAULT_GENERATOR 改为 'rust'，随后删除 Python 路径
与本开关（docs/plans/2026-09-20-rust-generator-rewrite.md）。

Rust 路径只替换「转译」段：overlay（main.py prepare_scratch）与 cargo 流程与 Python 路径共用；
`rava build --no-run` 在同一 scratch 内完成 javac → 闭包分析 → 发射（closure.json 同样落
`<scratch>/closure_input/`，动态对照照常可用）。
"""

import os
import subprocess
import sys

DEFAULT_GENERATOR = 'python'
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


def run_rust(java_files: list[str], out_dir: str, *, strict: bool, locales: tuple[str, ...]) -> None:
    """`rava build --no-run`：javac → 闭包 → 发射进 out_dir（overlay 已由调用方完成，rava 内幂等重放）"""
    home = os.environ.get('JAVA_HOME', '')
    if not home:
        sys.exit('Rust 生成器需要 JAVA_HOME（main.py 经 jdk_select.apply_jdk 设置）')
    # 镜像独有 / VM 支持类目录：Rust 侧尚无 jimage 提取，沿用 Python 解析器的缓存产物
    sys.path.insert(0, _REPO)
    from codegen.jdk_resolver import JdkResolver
    images = JdkResolver(java_home=home).image_class_dirs()
    cmd = ['cargo', 'run', '--release', '-q', '--manifest-path', _GENERATOR_MANIFEST,
           '--target-dir', _RAVA_TARGET, '--', 'build', *[os.path.abspath(f) for f in java_files],
           '--java-home', home, '--runtime', _RUNTIME, '--out', os.path.abspath(out_dir), '--no-run']
    for d in images:
        cmd += ['--image', d]
    for loc in locales:
        cmd += ['--locale', loc]
    if strict:
        cmd.append('--strict')
    print(f"[rava] build {' '.join(os.path.basename(f) for f in java_files)} → {out_dir}", flush=True)
    r = subprocess.run(cmd)
    if r.returncode != 0:
        sys.exit(f'rava build 失败（{r.returncode}）')
