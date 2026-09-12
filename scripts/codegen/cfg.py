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
