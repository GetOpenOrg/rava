# 从 codegen/instr/sim.py 中拆出

from ...stack import I32, I64, F32, F64
from ...rs_ir import RawExpr, Cast
from ...render import render_expr, render_type
from ..coerce import _to_i32


def sim_arith(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 整数算术 ──
    if op == 'iadd':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_add({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'isub':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_sub({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'imul':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_mul({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'idiv':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}/{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'irem':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}%{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ineg':
        a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_neg()"), I32)
    elif op == 'ishl':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}<<({_to_i32(render_expr(b), bt)}&0x1f))"), I32)
    elif op == 'ishr':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}>>(({_to_i32(render_expr(b), bt)}&0x1f)))"), I32)
    elif op == 'iushr':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"(({_to_i32(render_expr(a), at)} as u32>>({_to_i32(render_expr(b), bt)}&0x1f)) as i32)"), I32)
    elif op == 'iand':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}&{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ior':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}|{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ixor':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}^{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ladd':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_add({b_s})"), I64)
    elif op == 'lsub':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_sub({b_s})"), I64)
    elif op == 'lmul':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_mul({b_s})"), I64)
    elif op == 'ldiv':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}/{b_s})"), I64)
    elif op == 'fadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F32)
    elif op == 'fsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F32)
    elif op == 'fmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F32)
    elif op == 'fdiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F32)
    elif op == 'dadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F64)
    elif op == 'dsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F64)
    elif op == 'dmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F64)
    elif op == 'ddiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F64)

    # ── long 算術（lrem/lneg/land/lor/lxor/lshl/lshr/lushr）──
    elif op == 'lrem':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}%({b_s}))"), I64)
    elif op == 'lneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_neg()"), I64)
    elif op == 'land':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}&({b_s}))"), I64)
    elif op == 'lor':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}|({b_s}))"), I64)
    elif op == 'lxor':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s})^({b_s})"), I64)
    elif op == 'lshl':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_shl(({render_expr(b)}&0x3f) as u32)"), I64)
    elif op == 'lshr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_shr(({render_expr(b)}&0x3f) as u32)"), I64)
    elif op == 'lushr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"(({render_expr(a)} as u64).wrapping_shr(({render_expr(b)}&0x3f) as u32) as i64)"), I64)

    # ── float/double 取余与取负 ──
    elif op == 'frem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}%({render_expr(b)}))"), F32)
    elif op == 'fneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"(-({render_expr(a)}))"), F32)
    elif op == 'drem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}%({render_expr(b)}))"), F64)
    elif op == 'dneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"(-({render_expr(a)}))"), F64)

    # ── 比较指令（lcmp/fcmpl/fcmpg/dcmpl/dcmpg）→ 压 i32 结果 ──
    elif op == 'lcmp':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"(({a_s}>({b_s})) as i32-(({a_s})<({b_s})) as i32)"), I32)
    elif op in ('fcmpl', 'fcmpg', 'dcmpl', 'dcmpg'):
        b, _ = sim.pop(); a, _ = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        sim.push(RawExpr(f"(({a_s}>({b_s})) as i32-(({a_s})<({b_s})) as i32)"), I32)

    # ── 类型转换 ──
    elif op == 'i2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'i2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'i2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    elif op == 'i2b': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as i8 as i32)"), I32)
    elif op == 'i2s': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as i16 as i32)"), I32)
    elif op == 'i2c': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as u16 as i32)"), I32)
    elif op == 'l2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'l2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'l2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    elif op == 'f2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'f2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'd2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'd2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'd2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'f2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    else:
        return False
    return True
