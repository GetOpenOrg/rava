"""
布尔条件 + 比较运算符翻译。
"""

from ..types import Instr


def _paren_if_cmp(expr: str) -> str:
    """若表达式顶层含比较运算符，用括号包裹，防止 Rust 链式比较错误（E0308）。"""
    if expr.startswith('(') and expr.endswith(')'):
        return expr
    depth = 0
    i = 0
    while i < len(expr):
        c = expr[i]
        if c in '([':
            depth += 1
        elif c in ')]':
            depth -= 1
        elif depth == 0:
            for sym in ('!=', '==', '<=', '>='):
                if expr[i:i + len(sym)] == sym:
                    return f"({expr})"
            if c in '<>' and expr[i:i + 2] not in ('->', '=>'):
                return f"({expr})"
        i += 1
    return expr


def cmp_op(opcode: str, a: str, b: str) -> str:
    """将 JVM 比较指令翻译为 Rust 条件表达式字符串。"""
    two_ops = {
        'if_icmpeq': '==', 'if_icmpne': '!=',
        'if_icmplt': '<',  'if_icmpge': '>=',
        'if_icmple': '<=', 'if_icmpgt': '>',
        'if_acmpeq': '==', 'if_acmpne': '!=',
    }
    if opcode in two_ops:
        a = _paren_if_cmp(a)
        b = _paren_if_cmp(b)
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
        # 仅 iconst_0 / iconst_1 构成布尔物化；iconst_m1 / iconst_2..5（如 `c ? 1 : -1`）
        # 是普通 int 三元表达式，交给通用 if-else 合并路径处理
        if next1.opcode not in ('iconst_0', 'iconst_1'):
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
        if false_ins.opcode not in ('iconst_0', 'iconst_1'):
            continue
        false_val = int(false_ins.opcode[-1])
        if true_val == false_val:
            continue

        end_idx = off2idx.get(end_offset)
        if end_idx is None:
            continue

        result[i] = (true_val, false_val, false_idx, end_idx)

    return result
