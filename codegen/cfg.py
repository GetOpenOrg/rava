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
            # 检查 body 中是否有 goto → 循环出口（break）
            for bi in body_instrs:
                if bi.opcode == 'goto' and bi.operand:
                    goto_tgt = int(bi.operand)
                    for lp in loops:
                        if lp.start_idx <= i <= lp.end_idx and lp.exit_offset is not None:
                            if goto_tgt >= lp.exit_offset:
                                exits = True
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


# ── if-else 检测（含 merge-point）──────────────────────────────────────────────

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
                    if merge_idx is not None and then_end > body_start:
                        # then body 非空 → 合法 if-else
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
