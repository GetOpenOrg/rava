# 从 codegen/instr/sim.py 中拆出

from ...stack import I32, I64, F32, F64
from ...rs_ir import Var, RawExpr, RawStmt
from ...render import render_expr
from ..member_naming import _parse_slot


def _refs_local(code: str, name: str) -> bool:
    """Rust 表达式文本是否在字符串/字符字面量之外以整词引用标识符 name。

    用于 iinc 快照判定：复合栈值（如 iadd 已把 iload 的 Var 吸进
    `_pre.wrapping_add(x)`）内嵌同名局部变量的活引用。扫描时跳过 "..." 与
    '...' 字面量（含转义），避免把 format! 模板文本里的同名误判为变量引用。"""
    i, n = 0, len(code)
    while i < n:
        c = code[i]
        if c == '"' or c == "'":
            quote = c
            i += 1
            while i < n and code[i] != quote:
                i += 2 if code[i] == '\\' else 1
            i += 1
        elif c.isalpha() or c == '_':
            j = i
            while j < n and (code[j].isalnum() or code[j] == '_'):
                j += 1
            if code[i:j] == name:
                return True
            i = j
        else:
            i += 1
    return False


def sim_locals(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op.startswith('iload'): sim.push(*sim.load_local(_parse_slot(op, operand)))
    elif op.startswith('lload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, I64)
    elif op.startswith('fload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F32)
    elif op.startswith('dload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F64)
    elif op.startswith('aload'): sim.push(*sim.load_local(_parse_slot(op, operand)))
    elif op.startswith('istore'):
        e, ty = sim.pop_for_store()
        # int 家族（bool/u16/i8/i16/i32）到局部声明类型的对齐统一在 store_local 内完成
        sim.store_local(_parse_slot(op, operand), e, ty)
    elif op.startswith('lstore'): e, _ = sim.pop_for_store(); sim.store_local(_parse_slot(op, operand), e, I64)
    elif op.startswith('fstore'): e, _ = sim.pop_for_store(); sim.store_local(_parse_slot(op, operand), e, F32)
    elif op.startswith('dstore'): e, _ = sim.pop_for_store(); sim.store_local(_parse_slot(op, operand), e, F64)
    elif op.startswith('astore'):
        e, ty = sim.pop_for_store()
        sim.store_local(_parse_slot(op, operand), e, ty)
    elif op == 'iinc':
        parts = operand.replace(',', ' ').split()
        slot, delta = int(parts[0]), int(parts[1])
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", I32, True))
        # iload X; iinc X K 模式（Java 后缀自增 i++）：栈上的 Var(name) 指向递增前的值。
        # Rust 变量在递增后名字不变，故先快照再递增，防止后续使用读到新值。
        # 裸 Var 之外，复合栈值同样内嵌该变量的活引用（如 `x++ + ++x` 的中间和
        # `_pre.wrapping_add(x)`）：凡在文本上引用了该变量的栈值，整体物化为临时
        # 变量、在递增前取值——JVM 语义是 iinc 之前压栈的任何栈值都按旧值参与
        # 后续运算。物化只会提前一步（iinc 与下次消费之间无其他指令），且栈值
        # 的副作用已在压栈时发生，故提前求值无可观察差异。
        for j in range(len(sim.stack)):
            sv_expr, sv_ty = sim.stack[j]
            if isinstance(sv_expr, Var):
                if sv_expr.name == name:
                    snap = sim.fresh_let(f'_{name}_pre', Var(name), sv_ty)
                    sim.stack[j] = (snap, sv_ty)
            elif _refs_local(render_expr(sv_expr), name):
                held = sim.fresh_let(f'_{name}_held', sv_expr, sv_ty)
                sim.stack[j] = (held, sv_ty)
        if delta >= 0: sim.emit(RawStmt(f"{name} = {name}.wrapping_add({delta}i32);"))
        else:          sim.emit(RawStmt(f"{name} = {name}.wrapping_sub({-delta}i32);"))
    else:
        return False
    return True
