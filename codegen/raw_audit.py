"""[raw-audit] Raw 发射与类型字符串手术计数（收敛路线图 L5-b / 阶段 A 仪表）。

规格来源：docs/plans/2026-09-21-codegen-type-convergence.md §二终态表与 §六处方 1。
两个口径：

- ``raw_expr`` / ``raw_stmt``（**运行时**）：RawExpr/RawStmt 构造事件计数——由
  rs_ir.py 的 ``__post_init__`` 埋点，每次转译进程累计。终态 = 0（全部类型化
  IR 节点替代）。趋势只降不升（downcast 链移除 / 转换族 IR 化每步应可见下降）。
- ``type_surgery_sites``（**静态源码卫生**）：codegen 源码里类型字符串手术位点
  数（`split('<')[0]` 类文本解剖、`startswith('JArray<')` 类形态探测）。每次
  转译时扫描源码树自算（毫秒级），随位点删除自动下降。终态 = 0（类型查询全部
  经 JvmType/TypeIR 方法）。

main.py 转译后输出一行 ``[raw-audit] raw_expr=N raw_stmt=N type_surgery_sites=N``，
run_tests.py 两模式解析并在结尾汇总（对齐 readability/equiv 审计线）。
"""

from __future__ import annotations

import re
from pathlib import Path

_counts: dict[str, int] = {'raw_expr': 0, 'raw_stmt': 0}

# 静态口径：类型字符串手术的源码形态（新增形态时同步收录并在模块注释说明）
_SURGERY_PATTERNS = (
    re.compile(r"""\.split\(['"]<['"]\)\[0\]"""),
    re.compile(r"""startswith\(['"]JArray<"""),
)


def record_raw(kind: str, n: int = 1) -> None:
    """RawExpr/RawStmt 构造事件计数（只读计数，不影响发射内容）。"""
    _counts[kind] = _counts.get(kind, 0) + n


def reset() -> None:
    """清零（与 equiv_audit.reset 同约定，供复用进程的场景）。"""
    for k in _counts:
        _counts[k] = 0


def type_surgery_sites() -> int:
    """codegen 源码树的类型字符串手术位点数（静态卫生度量）。"""
    root = Path(__file__).parent
    n = 0
    for p in root.rglob('*.py'):
        if '__pycache__' in p.parts:
            continue
        try:
            text = p.read_text(encoding='utf-8')
        except Exception:
            continue
        for pat in _SURGERY_PATTERNS:
            n += len(pat.findall(text))
    return n


def summary() -> str:
    return (f"[raw-audit] raw_expr={_counts['raw_expr']} "
            f"raw_stmt={_counts['raw_stmt']} "
            f"type_surgery_sites={type_surgery_sites()}")
