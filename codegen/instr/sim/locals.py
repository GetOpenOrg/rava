# 从 codegen/instr/sim.py 中拆出

from ...stack import I32, I64, F32, F64
from ...rs_ir import Var, RawExpr, RawStmt
from ...render import render_expr
from ..coerce import _parse_slot


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
        e, ty = sim.pop()
        # istore 在 JVM 中存储 int；bool 比较结果需要强制转换
        if getattr(ty, 'name', '') == 'bool':
            e = RawExpr(f"({render_expr(e)}) as i32")
            ty = I32
        sim.store_local(_parse_slot(op, operand), e, ty)
    elif op.startswith('lstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, I64)
    elif op.startswith('fstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F32)
    elif op.startswith('dstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F64)
    elif op.startswith('astore'):
        e, ty = sim.pop()
        sim.store_local(_parse_slot(op, operand), e, ty)
    elif op == 'iinc':
        parts = operand.replace(',', ' ').split()
        slot, delta = int(parts[0]), int(parts[1])
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", I32, True))
        # iload X; iinc X K 模式（Java 后缀自增 i++）：栈上的 Var(name) 指向递增前的值。
        # Rust 变量在递增后名字不变，故先快照再递增，防止后续使用读到新值。
        for j in range(len(sim.stack)):
            sv_expr, sv_ty = sim.stack[j]
            if isinstance(sv_expr, Var) and sv_expr.name == name:
                snap = sim.fresh_let(f'_{name}_pre', Var(name), sv_ty)
                sim.stack[j] = (snap, sv_ty)
        if delta >= 0: sim.emit(RawStmt(f"{name} = {name}.wrapping_add({delta}i32);"))
        else:          sim.emit(RawStmt(f"{name} = {name}.wrapping_sub({-delta}i32);"))
    else:
        return False
    return True
