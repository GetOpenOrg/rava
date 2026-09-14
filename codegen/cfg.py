"""
控制流分析：
- find_loops: 检测 while 循环（back-edge goto → loop { if cond { break; } body }）
- find_if_guards: 检测 guard 模式（条件跳转 + fall-through 必定退出）
- find_boolean_conditions: 检测 condition→boolean 赋值模式
"""

from dataclasses import dataclass
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
        'if_acmpeq': '==', 'if_acmpne': '!=',
    }
    if opcode in two_ops:
        return f"{a} {two_ops[opcode]} {b}"

    one_ops = {
        'ifeq': f"({a}==0)", 'ifne': f"({a}!=0)",
        'iflt': f"({a}<0)",  'ifge': f"({a}>=0)",
        'ifle': f"({a}<=0)", 'ifgt': f"({a}>0)",
        'ifnull':    f"_is_jnull(&{a})",
        'ifnonnull': f"!_is_jnull(&{a})",
    }
    return one_ops.get(opcode, f"/* {opcode} */ true")


_NEGATE_CMP: dict[str, str] = {
    'ifeq': 'ifne', 'ifne': 'ifeq',
    'iflt': 'ifge', 'ifge': 'iflt',
    'ifle': 'ifgt', 'ifgt': 'ifle',
    'if_icmpeq': 'if_icmpne', 'if_icmpne': 'if_icmpeq',
    'if_icmplt': 'if_icmpge', 'if_icmpge': 'if_icmplt',
    'if_icmple': 'if_icmpgt', 'if_icmpgt': 'if_icmple',
    'if_acmpeq': 'if_acmpne', 'if_acmpne': 'if_acmpeq',
    'ifnull': 'ifnonnull', 'ifnonnull': 'ifnull',
}


def neg_cmp_op(opcode: str, a: str, b: str) -> str:
    """返回 fall-through 条件（跳转条件的否定）。"""
    return cmp_op(_NEGATE_CMP.get(opcode, opcode), a, b)


# ── if-guard 检测 ──────────────────────────────────────────────────────────────

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

    # 收集所有循环体范围和条件索引，排除它们
    excluded: set[int] = set()
    if loops:
        for lp in loops:
            excluded.update(range(lp.start_idx, lp.end_idx + 1))
            excluded.add(lp.cond_idx)

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

        # body 必须必定退出：包含 *return / athrow
        # 注意：不把 goto 当退出——goto-to-merge 在 boolean-condition 模式中也出现
        exits = any(bi.opcode in _EXIT_OPS for bi in body_instrs)

        if exits:
            result[i] = IfGuardInfo(
                body_start_idx=body_start_idx,
                continue_idx=target_idx,
                is_two_op=(op in _TWO_OP_BRANCH_OPS),
            )

    return result


# ── boolean-condition 检测 ─────────────────────────────────────────────────────

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
