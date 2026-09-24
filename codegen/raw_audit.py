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

main.py 转译后输出一行 ``[raw-audit] raw_expr=N raw_stmt=N type_surgery_sites=N
type_surgery_ext=N``（后者为扩大口径，见 _SURGERY_EXT_PATTERNS），
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

# 扩大口径（type_surgery_ext，2026-09-24 TypeIR 余项调研 §四 E 批）：原口径外的
# 同类文本解剖——带 maxsplit 的头部切分、partition/find/index 取尖括号、无尖括号
# 的 JArray 前缀探测、Rust 容器前缀探测。独立计数（不改原口径含义，趋势可比），
# 作为 L1 下一轮余量基线；终态同为 0。
_SURGERY_EXT_PATTERNS = (
    re.compile(r"""\.split\(['"]<['"],\s*1\)\[0\]"""),
    re.compile(r"""\.partition\(['"]<['"]\)"""),
    re.compile(r"""\.(?:find|index|rfind)\(['"]<['"]\)"""),
    re.compile(r"""startswith\(['"]JArray['"]\)"""),
    re.compile(r"""startswith\(['"](?:Vec|Rc)<"""),
)

# 扫描豁免：本模块（模式定义与自述文本即命中形态——原口径的 docstring 误报）；
# 语法解析层与擦除查询边界（文本解析的合法唯一落点：sig_types / type_args /
# jvm_type 的 Rust 类型文法解析器、stack 的擦除边界函数）只计入扩大口径的豁免
_SELF = 'raw_audit.py'
_EXT_PARSER_WHITELIST = frozenset({'sig_types.py', 'type_args.py', 'jvm_type.py', 'stack.py'})


def record_raw(kind: str, n: int = 1) -> None:
    """RawExpr/RawStmt 构造事件计数（只读计数，不影响发射内容）。"""
    _counts[kind] = _counts.get(kind, 0) + n


def reset() -> None:
    """清零（与 equiv_audit.reset 同约定，供复用进程的场景）。"""
    for k in _counts:
        _counts[k] = 0


def _scan(patterns, exempt: frozenset) -> int:
    root = Path(__file__).parent
    n = 0
    for p in root.rglob('*.py'):
        if '__pycache__' in p.parts or p.name == _SELF or p.name in exempt:
            continue
        try:
            text = p.read_text(encoding='utf-8')
        except Exception:
            continue
        for pat in patterns:
            n += len(pat.findall(text))
    return n


def type_surgery_sites() -> int:
    """codegen 源码树的类型字符串手术位点数（静态卫生度量，原口径）。"""
    return _scan(_SURGERY_PATTERNS, frozenset())


def type_surgery_ext_sites() -> int:
    """扩大口径的类型字符串手术位点数（解析层 / 擦除边界白名单豁免）。"""
    return _scan(_SURGERY_EXT_PATTERNS, _EXT_PARSER_WHITELIST)


def summary() -> str:
    return (f"[raw-audit] raw_expr={_counts['raw_expr']} "
            f"raw_stmt={_counts['raw_stmt']} "
            f"type_surgery_sites={type_surgery_sites()} "
            f"type_surgery_ext={type_surgery_ext_sites()}")
