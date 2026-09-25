"""
窥孔融合三件套（窗口 3 ⑧，自 BlockSimulator 方法纯搬移为接收 sim 的模块级函数，
零逻辑改动）。

- try_fuse           单前驱直线块并入前驱
- try_short_circuit  `a && b` / `a || b` 级联条件块并入前驱条件
- try_ternary        两臂各留一值的菱形 → 条件表达式（0/1 两臂 → 布尔表达式）

每条被吸收的跳转经 sim._consume 记入账本（与 BlockSimulator 其余归约同一账本）。
"""

from __future__ import annotations
from typing import TYPE_CHECKING

from ..cfg import Cond, negate, cond_and, cond_or, render_cond
from ..render import render_expr, render_type
from ..rs_ir import RawExpr
from ..stack import BOOL
from .unify import CondExpr, arm_value, unify_pair, inline_temps

if TYPE_CHECKING:
    from .blocks import Node, BlockSimulator


def try_fuse(sim, node: Node) -> Node:
    """单前驱直线块并入前驱。返回并入后的当前节点。"""
    if node.id == sim.entry or node.decls or node.kind == 'try' or node.id in sim.catch_exits:
        return node
    preds = sim._all_preds(node.id)
    if len(preds) != 1:
        return node
    p = preds[0]
    if p is node or not p.processed or p.kind != 'goto' or p.ctx != node.ctx:
        return node
    p.stmts.extend(node.stmts)
    p.kind, p.cond, p.key = node.kind, node.cond, node.key
    p.target, p.fallthrough = node.target, node.fallthrough
    p.cases, p.default = node.cases, node.default
    p.exit_stack, p.exit_locals = node.exit_stack, node.exit_locals
    sim._consume(p.pcs, 'structured')
    p.pcs = list(node.pcs)
    node.removed = True
    return p

def try_short_circuit(sim, node: Node) -> Node:
    """条件块 B 并入其唯一前驱条件块 P（&& / ||），可级联。"""
    while node.kind == 'cond' and node.id != sim.entry and not node.decls \
            and node.id not in sim.catch_exits:
        preds = sim._all_preds(node.id)
        if len(preds) != 1:
            break
        p = preds[0]
        if p is node or not p.processed or p.kind != 'cond' or p.ctx != node.ctx:
            break
        if not all(x is y or x[0] is y[0] for x, y in zip(p.exit_stack, node.exit_stack)) \
                or len(p.exit_stack) != len(node.exit_stack):
            break
        b_cond = inline_temps(node.stmts, node.cond, node.exit_stack)
        if b_cond is None:
            break
        cp, cb = p.cond, b_cond
        if p.fallthrough == node.id and p.target != node.id:
            if node.target == p.target:            # if (P || B) goto T
                cond, tgt, fall = cond_or(cp, cb), p.target, node.fallthrough
            elif node.fallthrough == p.target:     # if (!P && B) goto Bt
                cond, tgt, fall = cond_and(negate(cp), cb), node.target, p.target
            else:
                break
        elif p.target == node.id and p.fallthrough != node.id:
            if node.target == p.fallthrough:       # if (P && !B) goto Bf
                cond, tgt, fall = cond_and(cp, negate(cb)), node.fallthrough, p.fallthrough
            elif node.fallthrough == p.fallthrough:  # if (P && B) goto Bt
                cond, tgt, fall = cond_and(cp, cb), node.target, p.fallthrough
            else:
                break
        else:
            break
        p.cond, p.target, p.fallthrough = cond, tgt, fall
        p.exit_locals = node.exit_locals
        sim._consume(node.pcs, 'short-circuit')
        p.pcs.extend(node.pcs)
        node.removed = True
        sim._fold_const(p)
        node = p
    return node

def try_ternary(sim, merge_id: int) -> None:
    """汇合点前的菱形：两臂各留一个值 → 条件表达式。"""
    changed = True
    while changed:
        changed = False
        preds = sim._live_preds(merge_id)
        for a in preds:
            if not _is_value_arm(sim, a, merge_id):
                continue
            p = sim._all_preds(a.id)[0]
            if p.kind != 'cond' or not p.processed or p.target == p.fallthrough:
                continue
            other_id = p.fallthrough if p.target == a.id else p.target
            b = sim.nodes[other_id]
            if b is a or b.removed or not b.processed or not _is_value_arm(sim, b, merge_id):
                continue
            if sim._all_preds(b.id)[0] is not p:
                continue
            if not (p.ctx == a.ctx == b.ctx == sim.nodes[merge_id].ctx):
                continue
            base = len(p.exit_stack)
            if not all(len(arm.exit_stack) == base + 1
                       and all(x is y or x[0] is y[0] for x, y in zip(p.exit_stack, arm.exit_stack))
                       for arm in (a, b)):
                continue
            jump_arm = a if p.target == a.id else b
            fall_arm = b if jump_arm is a else a
            p.exit_stack = list(p.exit_stack) + [_ternary_value(sim, p.cond, fall_arm, jump_arm)]
            sim._consume(p.pcs + a.pcs + b.pcs, 'ternary')
            p.kind, p.cond, p.target, p.fallthrough, p.pcs = 'goto', None, merge_id, None, []
            a.removed = b.removed = True
            changed = True
            break

def _is_value_arm(sim, arm: Node, merge_id: int) -> bool:
    if arm.id == sim.entry or arm.id in sim.handler_bind or arm.kind != 'goto' \
            or arm.target != merge_id or arm.stmts or arm.decls:
        return False
    return len(sim._all_preds(arm.id)) == 1

def _ternary_value(sim, jump_cond: Cond, fall_arm: Node, jump_arm: Node):
    fall_entry, jump_entry = fall_arm.exit_stack[-1], jump_arm.exit_stack[-1]
    literals = {render_expr(fall_entry[0]), render_expr(jump_entry[0])}
    if literals == {'0i32', '1i32'} and render_type(fall_entry[1]) == 'i32' \
            and render_type(jump_entry[1]) == 'i32':
        truth = jump_cond if render_expr(jump_entry[0]) == '1i32' else negate(jump_cond)
        return (CondExpr(code=f"({render_cond(truth)})", cond=truth), BOOL)
    tv, ev, ty = unify_pair(arm_value(fall_entry), fall_entry[1],
                            arm_value(jump_entry), jump_entry[1],
                            sim.class_tparams, sim.registry)
    fall_cond = render_cond(negate(jump_cond))
    return (RawExpr(f"(if {fall_cond} {{ {tv} }} else {{ {ev} }})"), ty)
