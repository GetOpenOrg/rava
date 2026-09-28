"""cargo 编译环境：重型闭包自动单作业（N8）。

16G 机器上单 rustc 峰值约 14G：`CARGO_BUILD_JOBS=2` 下约 1750 类闭包即被 OOM 杀
（2026-09-28 实测 TestEnumAdvanced 1777 类、TestCipherDesModes 1807 类）。生成类数达到
阈值的工作区自动设 `CARGO_BUILD_JOBS=1`；调用方已显式设置该变量时尊重调用方。"""
import os
from pathlib import Path

# 生成类文件数阈值（带 java_class 生成标记的 .rs）；显式设置 CARGO_BUILD_JOBS 即可覆盖自动判定
HEAVY_CLASSES = 1700
_GEN_MARKER = b'rava_macros::java_class'


def generated_class_count(ws: Path) -> int:
    """工作区内生成类文件数（java_runtime + user 两个 crate）。"""
    n = 0
    for sub in ('java_runtime/src', 'user/src'):
        root = Path(ws) / sub
        if not root.is_dir():
            continue
        for p in root.rglob('*.rs'):
            try:
                with open(p, 'rb') as f:
                    if _GEN_MARKER in f.read():
                        n += 1
            except OSError:
                pass
    return n


def is_heavy(ws: Path) -> bool:
    """重型工作区：生成类数 ≥ 阈值。"""
    return generated_class_count(ws) >= HEAVY_CLASSES


# 调试信息减量：debuginfo=2 下大闭包 rustc 峰值 13.8G 被 OOM 杀（2026-09-25 实测），
# line-tables-only 通过且二进制约减半；行号回溯保留。调用方显式设置时尊重调用方
_LOW_MEM_DEFAULTS = {'CARGO_PROFILE_DEV_DEBUG': 'line-tables-only'}


def with_heavy_jobs(env: dict, ws: Path) -> dict:
    """编译环境：低内存缺省（调试信息减量）+ 重型工作区（生成类数 ≥ 阈值）且未显式设置时
    追加 CARGO_BUILD_JOBS=1。"""
    missing = {k: v for k, v in _LOW_MEM_DEFAULTS.items() if k not in env}
    if missing:
        env = dict(env, **missing)
    if 'CARGO_BUILD_JOBS' in env:
        return env
    n = generated_class_count(ws)
    if n >= HEAVY_CLASSES:
        print(f"[cargo-env] 生成类 {n} ≥ {HEAVY_CLASSES}：CARGO_BUILD_JOBS=1（N8 内存上限）", flush=True)
        return dict(env, CARGO_BUILD_JOBS='1')
    return env
