"""
基本块构建：将方法指令序列划分为基本块，构建控制流图（CFG）。

基本块：连续的指令序列，只有一个入口（第一条指令）和一个出口（最后一条指令）。

CFG 主要用途（Arch-8）：
- 可达性分析：确定函数是否在所有路径上都有 return/throw
- 消除 unreachable!() 补丁（I-5）：发散函数（所有路径 return/throw）无需尾部占位
- 支配树分析基础设施（为后续 Arch-1 等提供支撑）
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional, Set

from ..types import Instr


# 条件/无条件跳转指令集
_BRANCH_OPS: frozenset[str] = frozenset([
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifle', 'ifgt', 'ifnull', 'ifnonnull',
    'goto', 'goto_w',
])

# 返回/抛出指令集（所有退出指令）
_EXIT_OPS: frozenset[str] = frozenset([
    'return', 'ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn', 'athrow',
])

# switch 指令集
_SWITCH_OPS: frozenset[str] = frozenset(['tableswitch', 'lookupswitch'])


@dataclass
class BasicBlock:
    """基本块：连续指令 + 后继/前驱边。"""
    id: int
    start_idx: int           # 在原始指令列表中的起始索引（inclusive）
    end_idx: int             # 在原始指令列表中的结束索引（exclusive）
    succs: list[int] = field(default_factory=list)  # 后继块 ID 列表
    preds: list[int] = field(default_factory=list)  # 前驱块 ID 列表


def build_basic_blocks(instrs: list[Instr]) -> list[BasicBlock]:
    """
    从指令序列构建基本块 CFG。

    算法：
    1. 确定块起始点（leader）：第一条指令、所有跳转目标、所有跳转后的下一条
    2. 划分基本块
    3. 构建后继/前驱边
    """
    if not instrs:
        return []

    off2idx: dict[int, int] = {ins.offset: i for i, ins in enumerate(instrs)}
    leaders: set[int] = {0}  # 块起始索引集合

    for i, ins in enumerate(instrs):
        op = ins.opcode

        if op in ('goto', 'goto_w') and ins.operand:
            target_off = int(ins.operand)
            if (target_idx := off2idx.get(target_off)) is not None:
                leaders.add(target_idx)
            if i + 1 < len(instrs):
                leaders.add(i + 1)

        elif op in _BRANCH_OPS and ins.operand:
            # 条件分支：跳转目标 + fall-through 均是新块起始
            target_off = int(ins.operand)
            if (target_idx := off2idx.get(target_off)) is not None:
                leaders.add(target_idx)
            if i + 1 < len(instrs):
                leaders.add(i + 1)

        elif op in _EXIT_OPS:
            if i + 1 < len(instrs):
                leaders.add(i + 1)

        elif op in _SWITCH_OPS and ins.operand:
            # switch：解析所有 case/default 目标
            for part in ins.operand.split():
                if ':' not in part:
                    continue
                _, v = part.split(':', 1)
                try:
                    target_off = int(v)
                    if (target_idx := off2idx.get(target_off)) is not None:
                        leaders.add(target_idx)
                except ValueError:
                    pass
            if i + 1 < len(instrs):
                leaders.add(i + 1)

    # 按 leader 划分基本块
    leader_list = sorted(leaders)
    blocks: list[BasicBlock] = []
    for b_num, start in enumerate(leader_list):
        end = leader_list[b_num + 1] if b_num + 1 < len(leader_list) else len(instrs)
        blocks.append(BasicBlock(id=b_num, start_idx=start, end_idx=end))

    # 构建后继/前驱边
    block_by_start: dict[int, BasicBlock] = {b.start_idx: b for b in blocks}

    for b in blocks:
        if b.end_idx <= b.start_idx:
            continue
        last = instrs[b.end_idx - 1]
        op = last.opcode

        if op in _EXIT_OPS:
            pass  # 无后继（终止块）

        elif op in ('goto', 'goto_w') and last.operand:
            target_off = int(last.operand)
            if (target_idx := off2idx.get(target_off)) is not None:
                succ = block_by_start.get(target_idx)
                if succ and succ.id not in b.succs:
                    b.succs.append(succ.id)
                    succ.preds.append(b.id)

        elif op in _SWITCH_OPS and last.operand:
            seen: set[int] = set()
            for part in last.operand.split():
                if ':' not in part:
                    continue
                _, v = part.split(':', 1)
                try:
                    target_off = int(v)
                    if target_off in seen:
                        continue
                    seen.add(target_off)
                    if (target_idx := off2idx.get(target_off)) is not None:
                        succ = block_by_start.get(target_idx)
                        if succ and succ.id not in b.succs:
                            b.succs.append(succ.id)
                            succ.preds.append(b.id)
                except ValueError:
                    pass

        elif op in _BRANCH_OPS and last.operand:
            # 条件分支：跳转目标 + fall-through
            target_off = int(last.operand)
            if (target_idx := off2idx.get(target_off)) is not None:
                succ = block_by_start.get(target_idx)
                if succ and succ.id not in b.succs:
                    b.succs.append(succ.id)
                    succ.preds.append(b.id)
            # fall-through：下一个块
            if b.end_idx in block_by_start:
                ft = block_by_start[b.end_idx]
                if ft.id not in b.succs:
                    b.succs.append(ft.id)
                    ft.preds.append(b.id)

        else:
            # 非跳转指令（普通指令）：fall-through
            if b.end_idx in block_by_start:
                ft = block_by_start[b.end_idx]
                if ft.id not in b.succs:
                    b.succs.append(ft.id)
                    ft.preds.append(b.id)

    return blocks


def function_always_returns(instrs: list[Instr]) -> bool:
    """
    判断函数是否在所有路径上都通过 return/throw 退出。

    原理：
    - 构建 CFG，找出所有"终止块"（无后继）
    - 若所有终止块的最后一条指令是 return/throw → 函数在所有路径上必然退出 → True
    - 若有终止块最后一条指令不是 return/throw → 函数可能"掉出"末尾 → False

    使用场景（Arch-8 / I-5 补丁删除）：
    - 对于有返回值的非 void 方法，若 function_always_returns = True，
      函数末尾无需追加 unreachable!()——要么有 return Ok(e)，要么是发散循环（loop { ... } 无 break）。
    - 对于 void 方法，总是加 Ok(()) 即可，与此函数无关。

    注意：Java 语言保证合法的非 void 方法在所有控制流路径上都有 return/throw，
    因此对合法 Java 字节码此函数理论上总是返回 True。
    在翻译后的 Rust 代码层面，loop { ... }（无 break）的类型是 !，满足任意返回类型约束。
    """
    if not instrs:
        return True  # 空方法视为无需 unreachable!()

    blocks = build_basic_blocks(instrs)
    if not blocks:
        return True

    for b in blocks:
        if b.succs:
            continue  # 有后继，不是终止块
        # 终止块：无后继
        if b.end_idx <= b.start_idx:
            return False  # 空块（异常情况）
        last_op = instrs[b.end_idx - 1].opcode
        if last_op not in _EXIT_OPS:
            return False  # 终止块最后一条不是 return/throw → 可能掉出末尾

    return True
