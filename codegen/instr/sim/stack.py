# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawExpr, RawStmt
from ...render import render_expr, render_type

# JVM category-2 计算类型（long/double）在模拟栈中占一个条目
_CAT2_RUST_TYPES = frozenset({'i64', 'f64'})


def _is_cat2(entry) -> bool:
    return render_type(entry[1]) in _CAT2_RUST_TYPES


def sim_stack(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op == 'dup':
        if sim.stack: sim.stack.append(sim.stack[-1])
    elif op == 'dup_x1':
        if len(sim.stack) >= 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'dup2':
        # JVM 规范：Form 2（栈顶为 category-2：long/double）只复制栈顶一项；
        # Form 1（两个 category-1）复制前两项
        if sim.stack and _is_cat2(sim.stack[-1]):
            sim.stack.append(sim.stack[-1])
        elif len(sim.stack) >= 2:
            v1 = sim.stack[-1]; v2 = sim.stack[-2]
            sim.stack += [v2, v1]
        elif sim.stack:
            sim.stack.append(sim.stack[-1])
    elif op == 'dup_x2':
        if len(sim.stack) >= 2 and _is_cat2(sim.stack[-2]):
            # Form 2：v2 为 category-2 → v1, v2, v1
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
        elif len(sim.stack) >= 3:
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v1, v3, v2, v1]
        elif len(sim.stack) == 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'dup2_x1':
        if len(sim.stack) >= 2 and _is_cat2(sim.stack[-1]):
            # Form 2：v1 为 category-2 → v1, v2, v1
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
        elif len(sim.stack) >= 3:
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v2, v1, v3, v2, v1]
    elif op == 'dup2_x2':
        if len(sim.stack) >= 2 and _is_cat2(sim.stack[-1]) and _is_cat2(sim.stack[-2]):
            # Form 4：v1、v2 均为 category-2 → v1, v2, v1
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
        elif len(sim.stack) >= 3 and _is_cat2(sim.stack[-1]):
            # Form 2：v1 为 category-2，v2、v3 为 category-1 → v1, v3, v2, v1
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v1, v3, v2, v1]
        elif len(sim.stack) >= 3 and _is_cat2(sim.stack[-3]):
            # Form 3：v1、v2 为 category-1，v3 为 category-2 → v2, v1, v3, v2, v1
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v2, v1, v3, v2, v1]
        elif len(sim.stack) >= 4:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            v3 = sim.stack.pop(); v4 = sim.stack.pop()
            sim.stack += [v2, v1, v4, v3, v2, v1]
    elif op == 'swap':
        if len(sim.stack) >= 2:
            sim.stack[-1], sim.stack[-2] = sim.stack[-2], sim.stack[-1]
    elif op == 'pop':
        if sim.stack:
            e_expr, _ = sim.pop()
            e = render_expr(e_expr)
            # 只有在弹出的是有副作用的表达式时才发出 let _ = ...
            if any(c in e for c in ['(', 'push', 'insert']):
                sim.emit(RawStmt(f"let _ = {e};"))
    elif op == 'pop2':
        # category-2 栈顶只弹一项；两个 category-1 弹两项
        if sim.stack:
            _top_cat2 = _is_cat2(sim.stack[-1])
            sim.pop()
            if not _top_cat2 and sim.stack:
                sim.pop()
    else:
        return False
    return True
