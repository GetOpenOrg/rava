# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawStmt
from ...render import render_expr, render_type
from ..coerce import _coerce_to_object, _coerce_value, _is_subtype, _into_super_chain, _PRIMITIVE_RUST_TYPES


def sim_returns(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op == 'return':
        # 构造函数 return 指令：返回 Ok(this) 而非 Ok(())
        if sim.is_constructor:
            sim.emit(RawStmt('return Ok(this);'))
        else:
            sim.emit(RawStmt('return Ok(());'))
    elif op in ('ireturn', 'lreturn', 'freturn', 'dreturn'):
        e_expr, e_ty = sim.pop()
        expr_s = render_expr(e_expr)
        # 若返回类型与栈类型不匹配（窄类型/bool→i32），做显式转换
        ret_ty = getattr(sim, 'return_type', 'i32')
        actual_ty = render_type(e_ty)
        if actual_ty != ret_ty and ret_ty in ('i8', 'i16', 'u16', 'bool', 'i32'):
            expr_s = _coerce_value(expr_s, e_ty, ret_ty)
        sim.emit(RawStmt(f"return Ok({expr_s});"))
    elif op == 'areturn':
        e_expr, e_ty = sim.pop()
        expr_s = render_expr(e_expr)
        actual_ty = render_type(e_ty)
        ret_ty = getattr(sim, 'return_type', 'Object')
        _ctparams = getattr(sim, 'class_type_params', frozenset())
        # 实例方法返回 this 时，this 是 &Self 引用，需要 Clone::clone 才能返回 owned 值
        if expr_s == 'this' and not sim.is_static:
            expr_s = 'Clone::clone(this)'
        elif ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            expr_s = _coerce_to_object(expr_s, actual_ty)
        elif ret_ty != 'Object' and actual_ty == 'Object':
            # 泛型类型参数（如 T、K、V、E）不实现 Default，用 panic!("null") 代替
            if ret_ty in _ctparams:
                expr_s = 'panic!("null")'
            else:
                expr_s = f"Default::default()"
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty)
              and _is_subtype(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)):
            # vtable 架构：返回值是子类型，用 From trait（.into()）
            chain = _into_super_chain(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)
            expr_s = f"{expr_s}{chain}"
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty) and actual_ty != 'Object'):
            # 类型不兼容（actual 不是 ret 的子类型时，如 checkcast Serializable → return Comparator<Object>）：
            # 条件 4 已处理 actual→ret 子类型，到这里说明 _is_subtype 未匹配，
            # 降级为 Default::default() 保证编译通过
            expr_s = 'Default::default()'
        sim.emit(RawStmt(f"return Ok({expr_s});"))
    else:
        return False
    return True
