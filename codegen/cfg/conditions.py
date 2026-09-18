"""
条件代数：JVM 条件跳转 → 可取反 / 可短路合并的布尔条件。

Cond 有四种形态：
  atom   原子条件，同时携带「成立」与「不成立」两种 Rust 文本（取反不产生 `!(..)` 包裹）
  and    全部成立
  or     任一成立
  const  编译期常量（静态 instanceof 等）
取反走德摩根律，始终保持可读形态：!(a || b) → !a && !b。
"""

from __future__ import annotations
from dataclasses import dataclass, field


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
    """将 JVM 比较指令翻译为 Rust 条件表达式字符串（跳转成立的条件）。"""
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
    if opcode not in one_ops:
        raise ValueError(f"非条件跳转指令: {opcode}")
    return one_ops[opcode]


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
    return cmp_op(_NEGATE_CMP[opcode], a, b)


# ─────────────────────────────────────────────────────────────────────────────
# Cond
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class Cond:
    kind: str                       # 'atom' | 'and' | 'or' | 'const'
    pos: str = ''                   # atom：成立文本
    neg: str = ''                   # atom：不成立文本
    items: list = field(default_factory=list)   # and / or
    value: bool = False             # const

    @property
    def is_const(self) -> bool:
        return self.kind == 'const'


def const(value: bool) -> Cond:
    return Cond('const', value=value)


def _is_simple_operand(text: str) -> bool:
    """文本在前缀 `!` 之下是否无需括号（顶层不含空格 / 二元运算符）。"""
    depth = 0
    for ch in text:
        if ch in '([{':
            depth += 1
        elif ch in ')]}':
            depth -= 1
        elif depth == 0 and ch in ' <>=|&+-*/%':
            return False
    return True


def atom(pos: str, neg: str | None = None) -> Cond:
    pos = pos.strip()
    if pos == 'true':
        return const(True)
    if pos == 'false':
        return const(False)
    if neg is None:
        if pos.startswith('!') and _is_simple_operand(pos[1:]):
            neg = pos[1:]
        elif pos.startswith('!(') and pos.endswith(')') and _balanced(pos[2:-1]):
            neg = pos[2:-1]
        elif _is_simple_operand(pos):
            neg = f'!{pos}'
        else:
            neg = f'!({pos})'
    return Cond('atom', pos=pos, neg=neg.strip())


def _balanced(text: str) -> bool:
    depth = 0
    for ch in text:
        if ch == '(':
            depth += 1
        elif ch == ')':
            depth -= 1
            if depth < 0:
                return False
    return depth == 0


def negate(c: Cond) -> Cond:
    if c.kind == 'const':
        return const(not c.value)
    if c.kind == 'atom':
        return Cond('atom', pos=c.neg, neg=c.pos)
    flipped = 'or' if c.kind == 'and' else 'and'
    return Cond(flipped, items=[negate(i) for i in c.items])


def _combine(kind: str, a: Cond, b: Cond) -> Cond:
    absorbing = (kind == 'or')      # or: true 吸收；and: false 吸收
    if a.is_const:
        # 左侧常量：吸收值 → 整体为常量（右侧不求值，与短路语义一致）；否则结果即右侧
        return a if a.value == absorbing else b
    if b.is_const and b.value != absorbing:
        return a
    # 右侧为吸收常量时左侧仍需求值（可能含调用），保留为普通合取/析取项
    items: list[Cond] = []
    for c in (a, b):
        if c.kind == kind:
            items.extend(c.items)
        else:
            items.append(c)
    return Cond(kind, items=items)


def cond_and(a: Cond, b: Cond) -> Cond:
    return _combine('and', a, b)


def cond_or(a: Cond, b: Cond) -> Cond:
    return _combine('or', a, b)


def render_cond(c: Cond, nested: bool = False) -> str:
    """渲染为 Rust 布尔表达式。nested=True 时复合条件自带括号。"""
    if c.kind == 'const':
        return 'true' if c.value else 'false'
    if c.kind == 'atom':
        return c.pos
    sep = ' && ' if c.kind == 'and' else ' || '
    text = sep.join(render_cond(i, nested=True) for i in c.items)
    return f'({text})' if nested else text


def map_atoms(c: Cond, fn) -> Cond:
    """对所有原子文本（pos / neg）应用 fn，返回新 Cond。"""
    if c.kind == 'atom':
        return Cond('atom', pos=fn(c.pos), neg=fn(c.neg))
    if c.kind in ('and', 'or'):
        return Cond(c.kind, items=[map_atoms(i, fn) for i in c.items])
    return c
