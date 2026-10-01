#!/usr/bin/env python3
"""
Java → Rust 转译器 CLI 入口。

per-test scratch workspace（见 docs/plans/2026-09-16-per-test-scratch-workspace.md）：
    手写代码唯一真源在 runtime/，转译由 `rava build --no-run`（generator/ 的 Rust 生成器）完成：
    overlay 手写层 → javac → 闭包分析 → 发射；生成代码与编译产物全部落在 build/<测试名>/，不进 git。

用法：
    python3 scripts/main.py                                        # 默认运行 tests/e2e/01_basics/HelloWorld.java
    python3 scripts/main.py tests/e2e/04_collections/TestArrayList.java
    python3 scripts/main.py tests/e2e/01_basics/TestArithmetic.java --no-run
    python3 scripts/main.py tests/e2e/01_basics/TestArithmetic.java --clean   # 清空 scratch 后重建

    # jar 输入模式（依赖库成 crate，docs/plans/2026-09-23-junit-crate-pilot.md M1/M2）：
    python3 scripts/main.py tests/lib_pilot/HamcrestAssertMain.java --jdk 21 --clean \
        --lib hamcrest=/path/hamcrest-3.0.jar
    python3 scripts/main.py tests/lib_pilot/JunitAssertMain.java --jdk 21 --clean \
        --lib hamcrest=/path/hamcrest-3.0.jar \
        --lib junit4=/path/junit-4.13.2.jar:seed=org.junit.Assert

--lib 规格 NAME=JAR[:seed=FQN[,FQN...]]（可重复，顺序即依赖序）：
  无 seed  = 整包模式（jar 全部类进 lib crate，种子 = 全部 public 类成员）
  有 seed  = 子集模式（只有种子类闭包内的 jar 类进 crate）
"""

import argparse
import re
import subprocess
import sys
import os
import time
import tomllib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rava_cli import run_rust

_REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_DEFAULT_JAVA = os.path.join(_REPO_ROOT, 'tests', 'e2e', '01_basics', 'HelloWorld.java')
_BUILD_ROOT = os.path.join(_REPO_ROOT, 'build')
_SHARED_TARGET = os.path.join(_BUILD_ROOT, 'target')


def fmt_dur(sec: float) -> str:
    """格式化耗时：<60s 用秒（两位小数），≥60s 用 m 分 s 秒。"""
    if sec < 60:
        return f"{sec:.2f}s"
    m, s = divmod(sec, 60)
    return f"{int(m)}m{s:04.1f}s"


def _to_snake(stem: str) -> str:
    """主类名 → scratch 目录名（与 run_tests.py `_to_bin_name` 同一规则）"""
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', stem.replace('$', '_'))
    return re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s).lower()


def _bin_name(out_dir: str, stem: str) -> str:
    """scratch 的 user/Cargo.toml `[[bin]]` 名（生成器是唯一真源）。

    单 bin 直接取；批量模式多 bin 时取与主类同名者。"""
    with open(os.path.join(out_dir, 'user', 'Cargo.toml'), 'rb') as f:
        bins = [b['name'] for b in tomllib.load(f).get('bin', [])]
    if len(bins) == 1:
        return bins[0]
    want = _to_snake(stem)
    for b in bins:
        if b.rstrip('_') == want:
            return b
    sys.exit(f'user/Cargo.toml 无主类 {stem} 的 [[bin]]（现有：{", ".join(bins) or "无"}）')


def main():
    ap = argparse.ArgumentParser(description='Java .class → Rust 转译器')
    ap.add_argument('java_files', nargs='*', help='.java 源文件列表（默认 tests/e2e/01_basics/HelloWorld.java）')
    ap.add_argument('--out', default=None,
                    help='scratch 工作区目录（默认 build/<主类 snake 名>）')
    ap.add_argument('--clean', action='store_true', help='转译前清空 scratch 工作区')
    ap.add_argument('--no-run', action='store_true', help='只生成 Rust 代码，不编译运行')
    ap.add_argument('--batch', action='store_true', help='批量模式：写 src/bin/<class>.rs（供并行测试用）')
    ap.add_argument('--jdk', type=int, default=None, metavar='N',
                    help='指定 JDK 主版本（javac 与翻译语料同源；默认 JAVA_HOME > .jdk-version）')
    ap.add_argument('--lib', action='append', default=[], metavar='NAME=JAR[:seed=FQN]',
                    help='jar 输入模式：依赖库发射为 lib crate（可重复；无 seed=整包，'
                         '有 seed=只收种子类闭包）。顺序即 crate 依赖序')
    ap.add_argument('--locales', default='', metavar='TAG[,TAG...]',
                    help='额外编入的 locale（BCP 47 或下划线形式，逗号分隔；默认只含用户字节码'
                         '静态可见的 locale + en + ROOT，见闭包分析器 generator/crates/closure/src/seeds/locale.rs）')
    ap.add_argument('--debug', action='store_true',
                    help='诊断明细：闭包未解析调用 / 存根兜底逐条输出')
    ap.add_argument('--strict', action='store_true',
                    help='严格模式：转译兜底改为硬失败，缺手写实现的 native 方法编译报错')
    ap.add_argument('--trace-class', default='', metavar='CLASS',
                    help='打印该类或方法（斜线形态，如 java/net/InetAddress 或 类.方法:描述符）入闭包的'
                         '最短 provenance 链（rava closure --why）')
    ap.add_argument('--precheck-only', action='store_true',
                    help='只转译并输出完整编译前预检明细（调用链上的 panic 存根 / 缺失 native），不编译不运行')
    ap.add_argument('--closure-json', action='store_true',
                    help='另写出 <scratch>/closure_input/closure.json（动态对照 / 生成树对照 / rava emit 用），'
                         '并校验由它解析的闭包事实与进程内直传的一致；缺省不写')
    ap.add_argument('--raw-sites', default='', metavar='FILE',
                    help='Raw 逃生舱构造位点剖面追加写入 FILE（FS-Q1 热点排序）')
    args = ap.parse_args()
    if args.lib and args.batch:
        sys.exit('jar 输入模式（--lib）不支持 --batch（单 bin 消费形态）')

    # JDK 选择（jdk_select.apply_jdk 唯一入口）：--jdk > JAVA_HOME >
    # .jdk-version > 最新已安装——多 JDK 并存时不随系统默认 java 漂移。run_tests 子进程
    # 已继承父进程选定的 JAVA_HOME，此处静默沿用
    from jdk_select import apply_jdk
    apply_jdk(args.jdk, quiet=(args.jdk is None and bool(os.environ.get('JAVA_HOME'))))

    java_files = args.java_files or [_DEFAULT_JAVA]
    stem = os.path.splitext(os.path.basename(java_files[0]))[0]
    out_dir = args.out or os.path.join(_BUILD_ROOT, _to_snake(stem))

    t_total = time.perf_counter()

    # 转译：rava build 内完成 overlay 手写代码 → javac → 闭包 → 发射
    t0 = time.perf_counter()
    run_rust(java_files, out_dir, clean=args.clean, strict=args.strict,
             locales=tuple(t for t in args.locales.split(',') if t.strip()),
             libs=tuple(args.lib), batch=args.batch, debug=args.debug, trace_class=args.trace_class,
             precheck_only=args.precheck_only, raw_sites=args.raw_sites, closure_json=args.closure_json)
    if args.precheck_only:
        return
    t_transpile = time.perf_counter() - t0
    print(f"[time] transpile   {fmt_dur(t_transpile)}")

    if not args.no_run:
        bin_name = _bin_name(out_dir, stem)
        print(f"\n[run] cargo run --bin {bin_name}")
        # CARGO_INCREMENTAL=0：宽闭包增量元数据是 OOM 压垮点（服务器 SIGKILL 实证）；scratch 每轮重生成，关闭无损失
        env = dict(os.environ, CARGO_TARGET_DIR=_SHARED_TARGET, CARGO_INCREMENTAL='0')
        # 重型闭包自动单作业（N8：单 rustc 峰值 ~14G）
        from cargo_env import with_heavy_jobs
        env = with_heavy_jobs(env, out_dir)
        t0 = time.perf_counter()
        r = subprocess.run(['cargo', 'run', '--bin', bin_name], cwd=out_dir, env=env)
        t_run = time.perf_counter() - t0
        print(f"[time] cargo run   {fmt_dur(t_run)}")
        print(f"[time] total       {fmt_dur(time.perf_counter() - t_total)}"
              f"  (transpile {fmt_dur(t_transpile)} + run {fmt_dur(t_run)})")
        sys.exit(r.returncode)

    print(f"[time] total       {fmt_dur(time.perf_counter() - t_total)}"
          f"  (transpile {fmt_dur(t_transpile)})")


if __name__ == '__main__':
    main()
