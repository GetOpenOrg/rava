"""
控制流分析：检测 while 循环（back-edge goto → loop { if cond { break; } body }）。
"""

from .types import Instr, LoopInfo


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

    return loops


def cmp_op(opcode: str, a: str, b: str) -> str:
    """将 JVM 比较指令翻译为 Rust 条件表达式字符串。"""
    two_ops = {
        'if_icmpeq': '==', 'if_icmpne': '!=',
        'if_icmplt': '<',  'if_icmpge': '>=',
        'if_icmple': '<=', 'if_icmpgt': '>',
    }
    if opcode in two_ops:
        return f"{a} {two_ops[opcode]} {b}"

    one_ops = {
        'ifeq': f"{a}==0i32", 'ifne': f"{a}!=0i32",
        'iflt': f"{a}<0i32",  'ifge': f"{a}>=0i32",
        'ifle': f"{a}<=0i32", 'ifgt': f"{a}>0i32",
        'ifnull':    f"{a}.is_none()",
        'ifnonnull': f"!{a}.is_none()",
    }
    return one_ops.get(opcode, f"/* {opcode} */ true")


_NEGATE_CMP: dict[str, str] = {
    'ifeq': 'ifne', 'ifne': 'ifeq',
    'iflt': 'ifge', 'ifge': 'iflt',
    'ifle': 'ifgt', 'ifgt': 'ifle',
    'if_icmpeq': 'if_icmpne', 'if_icmpne': 'if_icmpeq',
    'if_icmplt': 'if_icmpge', 'if_icmpge': 'if_icmplt',
    'if_icmple': 'if_icmpgt', 'if_icmpgt': 'if_icmple',
    'ifnull': 'ifnonnull', 'ifnonnull': 'ifnull',
}


def neg_cmp_op(opcode: str, a: str, b: str) -> str:
    """返回 fall-through 条件（跳转条件的否定）。"""
    return cmp_op(_NEGATE_CMP.get(opcode, opcode), a, b)


def find_boolean_conditions(instrs: list[Instr]) -> dict[int, tuple]:
    """
    检测 JVM 'condition→boolean' 模式：
      if* <false_offset>    # 跳转条件为真时跳至 false 分支
      iconst_X              # fall-through: push true_val（0 或 1）
      goto <end_offset>     # 跳过 false 分支
      iconst_Y              # 在 false_offset: push false_val（0 或 1）
      ...                   # 在 end_offset: 后续指令

    仅处理简单的 3 指令 true-branch（iconst + goto + iconst 紧挨在一起）。

    返回: {if_idx: (true_val, false_val, false_idx, end_idx)}
    """
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    result: dict[int, tuple] = {}

    for i, ins in enumerate(instrs):
        op = ins.opcode
        if not (op.startswith('if_icmp') or op.startswith('if')):
            continue
        if not ins.operand:
            continue

        false_offset = int(ins.operand)
        if false_offset <= ins.offset:
            continue  # 后向跳转（循环），跳过

        # i+1 必须是 iconst（true_val）
        if i + 1 >= len(instrs):
            continue
        next1 = instrs[i + 1]
        if not next1.opcode.startswith('iconst_'):
            continue
        true_val = int(next1.opcode[-1])

        # i+2 必须是前向 goto
        if i + 2 >= len(instrs):
            continue
        next2 = instrs[i + 2]
        if next2.opcode != 'goto' or not next2.operand:
            continue
        end_offset = int(next2.operand)
        if end_offset <= ins.offset:
            continue

        # false_offset 对应的指令必须是 iconst（false_val），且 index == i+3
        false_idx = off2idx.get(false_offset)
        if false_idx is None or false_idx != i + 3:
            continue
        false_ins = instrs[false_idx]
        if not false_ins.opcode.startswith('iconst_'):
            continue
        false_val = int(false_ins.opcode[-1])

        end_idx = off2idx.get(end_offset)
        if end_idx is None:
            continue

        result[i] = (true_val, false_val, false_idx, end_idx)

    return result
