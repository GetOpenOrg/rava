"""
if-else 检测（含 merge-point）。
"""

from dataclasses import dataclass
from ..types import Instr, LoopInfo
from .loops import _BRANCH_OPS, _TWO_OP_BRANCH_OPS


@dataclass
class IfElseInfo:
    """
    if/else 结构（或 simple if-then）。

    if-else 模式（has_else=True）：
      ifX <T>        # 条件为 FALSE → 跳到 else 起始 T
        <then_body>  # then_start .. then_end (不含末尾 goto)
        goto <M>     # 跳到 merge M
      <T>:           # else_start = T
        <else_body>  # else_start .. else_end
      <M>:           # merge_idx

    simple if-then 模式（has_else=False）：
      ifX <T>        # 条件为 FALSE → 跳到 T（跳过 then）
        <then_body>  # then_start .. then_end (= target_idx)
      <T>:           # merge_idx = T
    """
    then_start: int    # then body 第一条指令索引（inclusive）
    then_end: int      # then body 最后一条指令索引之后（exclusive）
    else_start: int    # else body 第一条指令索引（= target_idx；has_else=False 时 = merge_idx）
    else_end: int      # else body 最后一条指令索引之后（exclusive；has_else=False 时 = merge_idx）
    merge_idx: int     # if-else 结束后继续执行的指令索引
    is_two_op: bool    # True = 条件需要两个操作数（if_icmp* / if_acmp*）
    has_else: bool     # True = 有 else 分支


def find_if_else(
    instrs: list[Instr],
    loops: list[LoopInfo] | None = None,
    bool_cond_map: dict | None = None,
    if_guard_map: dict | None = None,
) -> dict[int, IfElseInfo]:
    """
    检测 if-else（有 merge-point）和 simple if-then 结构，排除已处理的模式。

    if-else 判定条件：
      - ifX <T> 是前向跳转
      - instrs[T-1] 是 goto <M>（前向，M > T）
      - then body（i+1 .. T-1）非空

    simple if-then 判定条件：
      - ifX <T> 是前向跳转
      - then body（i+1 .. T）不含条件分支（线性）
      - 不满足 if-else 条件

    排除：loops 体内、bool_cond_map、if_guard_map 中的指令。
    """
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}

    # 只排除循环条件和回跳，不排除整个循环体（循环体内可有嵌套 if-else）
    excluded: set[int] = set()
    if loops:
        for lp in loops:
            excluded.add(lp.cond_idx)
            excluded.add(lp.end_idx)

    already: set[int] = set()
    if bool_cond_map:
        already.update(bool_cond_map.keys())
    if if_guard_map:
        already.update(if_guard_map.keys())

    result: dict[int, IfElseInfo] = {}

    for i, ins in enumerate(instrs):
        if i in excluded or i in already:
            continue
        op = ins.opcode
        if op not in _BRANCH_OPS or not ins.operand:
            continue

        target_offset = int(ins.operand)
        if target_offset <= ins.offset:
            continue  # 后向跳转

        target_idx = off2idx.get(target_offset)
        if target_idx is None:
            continue

        body_start = i + 1
        if body_start > target_idx:
            continue

        # ── 尝试 if-else（goto merge）──
        if target_idx > 0:
            last_before = instrs[target_idx - 1]
            if last_before.opcode == 'goto' and last_before.operand:
                merge_offset = int(last_before.operand)
                # goto 必须是前向且超过 target_offset（跳过 else body）
                if merge_offset > target_offset:
                    merge_idx = off2idx.get(merge_offset)
                    then_end = target_idx - 1   # 不含末尾的 goto（exclusive）
                    if merge_idx is not None and then_end >= body_start:
                        # then body（可以为空 goto-only）→ 合法 if-else
                        result[i] = IfElseInfo(
                            then_start=body_start,
                            then_end=then_end,
                            else_start=target_idx,
                            else_end=merge_idx,
                            merge_idx=merge_idx,
                            is_two_op=(op in _TWO_OP_BRANCH_OPS),
                            has_else=True,
                        )
                        continue

        # ── 尝试 simple if-then（fall-through）──
        then_body = instrs[body_start:target_idx]
        # body 中不能有条件分支（线性）
        if any(b.opcode in _BRANCH_OPS for b in then_body):
            continue

        result[i] = IfElseInfo(
            then_start=body_start,
            then_end=target_idx,
            else_start=target_idx,
            else_end=target_idx,
            merge_idx=target_idx,
            is_two_op=(op in _TWO_OP_BRANCH_OPS),
            has_else=False,
        )

    return result
