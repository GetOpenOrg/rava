"""
循环检测：find_loops + 常量集合。
"""

from ..types import Instr, LoopInfo

# 条件分支指令集
_BRANCH_OPS = frozenset([
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifle', 'ifgt', 'ifnull', 'ifnonnull',
])

_TWO_OP_BRANCH_OPS = frozenset([
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
])

_EXIT_OPS = frozenset([
    'return', 'ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn', 'athrow',
])


def find_loops(instrs: list[Instr]) -> list[LoopInfo]:
    """扫描 back-edge goto 指令，返回识别到的所有循环信息。"""
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    loops: list[LoopInfo] = []

    for i, ins in enumerate(instrs):
        if ins.opcode != 'goto' or not ins.operand:
            continue
        target_off = int(ins.operand)
        if target_off >= ins.offset:
            continue  # 前向跳转，不是循环

        start_idx = off2idx.get(target_off)
        if start_idx is None:
            continue

        cond_idx = exit_off = None
        for j in range(start_idx, i + 1):
            op = instrs[j].opcode
            if (op.startswith('if_icmp') or op.startswith('if')) and instrs[j].operand:
                off = int(instrs[j].operand)
                if off > ins.offset:
                    cond_idx = j
                    exit_off = off
                    break

        if cond_idx is not None:
            loops.append(LoopInfo(start_idx, i, cond_idx, exit_off))
        else:
            # 无条件循环（for(;;) / while(true)）：cond_idx=None，exit_off=None
            # 用 cond_idx=i（goto 自身），exit_off=None 标记为无条件 loop {}
            loops.append(LoopInfo(start_idx, i, None, None))

    # 检测 do-while：back-edge 是条件后向分支（if_icmp* <start>）
    while_starts = {lp.start_idx for lp in loops}
    for i, ins in enumerate(instrs):
        op = ins.opcode
        if op not in _BRANCH_OPS or not ins.operand:
            continue
        target_off = int(ins.operand)
        if target_off >= ins.offset:
            continue  # 前向跳转，不是 do-while
        start_idx = off2idx.get(target_off)
        if start_idx is None or start_idx in while_starts:
            continue  # 已被 while-loop 检测覆盖
        # do-while: cond_idx = end_idx = i, exit_offset = 下一条指令的 offset
        exit_off2 = instrs[i + 1].offset if i + 1 < len(instrs) else None
        loops.append(LoopInfo(start_idx, i, i, exit_off2))

    return loops
