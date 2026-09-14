"""
if-guard 检测：条件跳转 + fall-through 必定退出。
"""

from dataclasses import dataclass
from ..types import Instr, LoopInfo
from .loops import _BRANCH_OPS, _TWO_OP_BRANCH_OPS, _EXIT_OPS


@dataclass
class IfGuardInfo:
    """
    一个 if-guard：条件跳转，fall-through 分支必定退出（athrow / *return / goto-past-T）。

    生成模式：
        if <fall_cond> {
            // fall-through body (always exits)
        }
        // continue at continue_idx
    """
    body_start_idx: int   # 第一条 body 指令的索引
    continue_idx: int     # guard 之后继续执行的指令索引（= 条件跳转的目标）
    is_two_op: bool       # True = 条件需要两个操作数（if_icmp* / if_acmp*）


def find_if_guards(instrs: list[Instr], loops: list[LoopInfo] | None = None) -> dict[int, IfGuardInfo]:
    """
    检测 guard 模式：
      ifX <T>        # 条件为 TRUE → 跳到 T（正常路径）
      <body>         # fall-through：必定退出（athrow / *return / forward goto-past-T）
      <T>:           # 继续执行

    返回 {branch_instr_idx: IfGuardInfo}。

    排除循环体内部和循环条件指令，避免与 find_loops 冲突。
    """
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}

    # 只排除循环条件指令和回跳 goto，不排除整个循环体
    # （循环体内可能有 guard / if-else，递归处理时需要能检测到）
    excluded: set[int] = set()
    if loops:
        for lp in loops:
            excluded.add(lp.cond_idx)   # 循环条件
            excluded.add(lp.end_idx)    # back-edge goto

    result: dict[int, IfGuardInfo] = {}

    for i, ins in enumerate(instrs):
        if i in excluded:
            continue
        op = ins.opcode
        if op not in _BRANCH_OPS or not ins.operand:
            continue

        target_offset = int(ins.operand)
        if target_offset <= ins.offset:
            continue  # 后向跳转（已被 find_loops 处理）

        target_idx = off2idx.get(target_offset)
        if target_idx is None:
            continue

        body_start_idx = i + 1
        if body_start_idx >= target_idx:
            continue  # 空 body，无意义

        body_instrs = instrs[body_start_idx:target_idx]

        # body 内不能有条件分支（保持线性，复杂嵌套暂不处理）
        if any(bi.opcode in _BRANCH_OPS for bi in body_instrs):
            continue

        # body 必须必定退出：*return / athrow，或 goto 到循环出口之外（break）
        exits = any(bi.opcode in _EXIT_OPS for bi in body_instrs)

        if not exits and loops:
            # 检查 body 中是否有 goto → 循环出口（break）或循环起始（continue）
            for bi in body_instrs:
                if bi.opcode == 'goto' and bi.operand:
                    goto_tgt = int(bi.operand)
                    for lp in loops:
                        if lp.start_idx <= i <= lp.end_idx:
                            if (lp.exit_offset is not None and goto_tgt >= lp.exit_offset):
                                exits = True
                                break
                            if instrs[lp.start_idx].offset == goto_tgt:
                                exits = True  # continue
                                break
                if exits:
                    break

        if exits:
            result[i] = IfGuardInfo(
                body_start_idx=body_start_idx,
                continue_idx=target_idx,
                is_two_op=(op in _TWO_OP_BRANCH_OPS),
            )

    return result
