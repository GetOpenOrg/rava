# 从 codegen/instr/sim.py 中拆出

import re as _re

from ...stack import I32, I64, F32, F64
from ...rs_ir import Var, RawExpr, RawStmt, RsNamed, RsGeneric
from ...render import render_expr, render_type
from ...type_map import jvm_to_rust, NEWARRAY_TYPES
from ..coerce import _to_i32, _coerce_to_object, _is_subtype, _PRIMITIVE_RUST_TYPES, _into_super_chain
from ...constants import OBJECT_CLASS as _OBJECT_CLASS


def _pop_index(sim):
    """弹出数组下标：JVM 下标恒为 int，窄类型（char/short/byte 局部）提升为 i32。"""
    idx_expr, idx_ty = sim.pop()
    src = render_expr(idx_expr)
    widened = _to_i32(src, idx_ty)
    return idx_expr if widened == src else RawExpr(widened)


def sim_arrays(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op == 'newarray':
        count_expr, _ = sim.pop()
        elem_t, _zero = NEWARRAY_TYPES.get(operand.strip(), ('i32', '0i32'))
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: JArray<{elem_t}> = JArray::<{elem_t}>::new({render_expr(count_expr)});"))
        sim.push(Var(v), RsNamed(f'JArray<{elem_t}>'))
    elif op == 'anewarray':
        count_expr, _ = sim.pop()
        # 用完整路径（comment）而非 short_cls，避免 'LString;' 等非全限定名映射到 Object
        if comment and comment != _OBJECT_CLASS:
            _elem_raw = jvm_to_rust(f'L{comment};', registry)
            # 用 _ 替换类型参数中的 Object，让 Rust 从赋值上下文推断泛型（避免 E0308）
            elem_t = _re.sub(r'\bObject\b', '_', _elem_raw) if '<' in _elem_raw else _elem_raw
        else:
            elem_t = 'Object'
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: JArray<{elem_t}> = JArray::<{elem_t}>::new({render_expr(count_expr)});"))
        sim.push(Var(v), RsNamed(f'JArray<{elem_t}>'))
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [render_expr(sim.pop()[0]) for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];"))
        sim.push(Var(v), RsGeneric('Vec', [RsGeneric('Vec', [I32])]))
    elif op in ('iastore', 'lastore', 'fastore', 'dastore'):
        val_expr, _val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, {render_expr(val_expr)})?;"))
    elif op == 'aastore':
        val_expr, val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        _m_aa = _re.match(r'JArray<(.+)>$', arr_ty_str)
        if _m_aa:
            elem_ty = _m_aa.group(1)
        elif arr_ty_str.startswith('Vec<') and arr_ty_str.endswith('>'):
            elem_ty = arr_ty_str[4:-1]
        else:
            elem_ty = 'Object'
        val_str = render_expr(val_expr)
        val_ty_str = render_type(val_ty)
        if elem_ty == 'Object' and val_ty_str not in ('Object', '()'):
            val_str = _coerce_to_object(val_str, val_ty_str)
        elif elem_ty != 'Object' and val_ty_str == 'Object':
            val_str = "Default::default()"
        elif val_ty_str not in _PRIMITIVE_RUST_TYPES:
            if (elem_ty != val_ty_str
                    and _is_subtype(val_ty_str.split('<')[0], elem_ty.split('<')[0], registry)):
                chain = _into_super_chain(val_ty_str.split('<')[0], elem_ty.split('<')[0], registry)
                val_str = f"Clone::clone(&{val_str}){chain}"
            else:
                val_str = f"Clone::clone(&{val_str})"
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, {val_str})?;"))
    elif op == 'bastore':
        val_expr, val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        # boolean[] 在 JVM 中以 bastore 写入，需要 != 0 转换
        if arr_ty_str in ('JArray<bool>', 'Vec<bool>'):
            val_s = render_expr(val_expr)
            coerced = val_s if render_type(val_ty) == 'bool' else f"(({val_s}) as i8 != 0)"
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, {coerced})?;"))
        else:
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i8)?;"))
    elif op == 'sastore':
        val_expr, _ = sim.pop(); idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i16)?;"))
    elif op == 'castore':
        val_expr, _ = sim.pop(); idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as u16)?;"))
    elif op == 'iaload':
        idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"{render_expr(arr_expr)}.get({render_expr(idx_expr)})?"), I32)
    elif op in ('baload', 'saload', 'caload'):
        idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(arr_expr)}.get({render_expr(idx_expr)})? as i32)"), I32)
    elif op in ('laload', 'faload', 'daload'):
        idx_expr = _pop_index(sim); arr_expr, _ = sim.pop()
        ty = {'l': I64, 'f': F32, 'd': F64}.get(op[0], I32)
        sim.push(RawExpr(f"{render_expr(arr_expr)}.get({render_expr(idx_expr)})?"), ty)
    elif op == 'aaload':
        idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        _m = _re.match(r'JArray<(.+)>$', arr_ty_str)
        if _m:
            elem_ty_str = _m.group(1)
        elif arr_ty_str.startswith('Vec<'):
            elem_ty_str = arr_ty_str[4:-1]
        else:
            elem_ty_str = 'Object'
        _arr_s = render_expr(arr_expr)
        # JArray::get 内部已 clone，直接使用返回值
        sim.push(RawExpr(f"{_arr_s}.get({render_expr(idx_expr)})?"), RsNamed(elem_ty_str))
    elif op == 'arraylength':
        arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(arr_expr)}.len())"), I32)
    else:
        return False
    return True
