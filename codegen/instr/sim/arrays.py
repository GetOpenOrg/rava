# 从 codegen/instr/sim.py 中拆出

import re as _re

from ...stack import I32, I64, F32, F64, erased_base, is_jvm_array
from ...rs_ir import Var, RawExpr, RawStmt, RsNamed, RsGeneric
from ...render import render_expr, render_type
from ...type_map import jvm_to_rust, NEWARRAY_TYPES
from ...constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ..coerce import _to_i32, _coerce_to_object
from ..hierarchy import _is_subtype, _into_super_chain
from ...constants import OBJECT_CLASS as _OBJECT_CLASS
from ... import equiv_audit

# 数组创建指令（neg-array 口径：S-8 潜在负长度路径——长度是运行期值，codegen
# 无法静态判定，当前对这三个指令的全部发射点计数；S-8 修复后转为创建点总量观测）
_ARRAY_CREATE_OPS = frozenset({'newarray', 'anewarray', 'multianewarray'})

# 数组访问指令（null-array 口径：S-2.1 null 表示语义的作用面——这些指令在
# Java 语义里对 null 数组引用抛 NPE（JVMS §6.5），null 表示的行为近似性
# 全部体现在这组发射点上）
_ARRAY_ACCESS_OPS = frozenset({
    'arraylength',
    'iaload', 'laload', 'faload', 'daload', 'aaload', 'baload', 'saload', 'caload',
    'iastore', 'lastore', 'fastore', 'dastore', 'aastore', 'bastore', 'sastore', 'castore',
})


def _pop_index(sim):
    """弹出数组下标：JVM 下标恒为 int，窄类型（char/short/byte 局部）提升为 i32。"""
    idx_expr, idx_ty = sim.pop()
    src = render_expr(idx_expr)
    widened = _to_i32(src, idx_ty)
    return idx_expr if widened == src else RawExpr(widened)


def _is_object_receiver(arr_ty) -> bool:
    """数组指令接收者是否擦除为 Object（S-2.2：多维数组元素经 Object 流转、Object[] 持有
    数组引用等）。是 → 走 Object 的数组访问 API（array_load_*/array_store_*/array_length，
    元素类型由指令操作码决定）；JArray/Vec 接收者保持类型化 get/set/len 快路径。"""
    t = render_type(arr_ty)
    return not (is_jvm_array(t) or t.startswith('Vec<'))


def sim_arrays(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # [equiv-audit] 只读计数，不改发射内容（neg-array / null-array 口径见
    # codegen/equiv_audit.py 模块注释）
    if op in _ARRAY_CREATE_OPS:
        equiv_audit.record('neg-array')
    elif op in _ARRAY_ACCESS_OPS:
        equiv_audit.record('null-array')

    if op == 'newarray':
        count_expr = _pop_index(sim)   # JVM 计数恒为 int：readShort 等窄来源提升为 i32
        elem_t, _zero = NEWARRAY_TYPES.get(operand.strip(), ('i32', '0i32'))
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: JArray<{elem_t}> = JArray::<{elem_t}>::new({render_expr(count_expr)});"))
        sim.push(Var(v), RsNamed(f'JArray<{elem_t}>'))
    elif op == 'anewarray':
        count_expr = _pop_index(sim)
        # 用完整路径（comment）而非 short_cls，避免 'LString;' 等非全限定名映射到 Object
        if comment and comment != _OBJECT_CLASS:
            if comment.startswith('['):
                # 组件本身是数组类（`int[][]` → anewarray class "[I"）：comment 已是
                # 元素描述符，直接映射（`L[I;` 是非法描述符，曾把多维数组的元素类型
                # 擦除为 Object，aaload 后丢失静态元素类型——实参位 E0308 的根因）
                _elem_raw = jvm_to_rust(comment, registry)
                elem_t = _elem_raw
            else:
                _elem_raw = jvm_to_rust(f'L{comment};', registry)
                # 用 _ 替换类型参数中的 Object，让 Rust 从赋值上下文推断泛型（避免 E0308）。
                # A-4 批次 3+：接口载体除外——载体的 Object 实参就是擦除运行时形态，
                # 无赋值上下文可推断（E0283），保持 `I<Object, ..>` 原样
                from ...jvm_type import carrier_type_for_ident as _carrier_of
                if ('<' in _elem_raw
                        and _carrier_of(_elem_raw, registry) != _elem_raw):
                    elem_t = _re.sub(r'\bObject\b', '_', _elem_raw)
                else:
                    elem_t = _elem_raw
        else:
            elem_t = 'Object'
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: JArray<{elem_t}> = JArray::<{elem_t}>::new({render_expr(count_expr)});"))
        sim.push(Var(v), RsNamed(f'JArray<{elem_t}>'))
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [render_expr(_pop_index(sim)) for _ in range(dims)][::-1]
        # 常量池项是数组类描述符（`[[I` / `[[Lpkg/Name;`）：数组类型按描述符映射；
        # 给出长度的各维逐层构造（每行是独立数组对象），未给长度的内层维保持 null
        _arr_desc = comment.strip().strip('"') if comment else ''
        arr_t = jvm_to_rust(_arr_desc, registry) if _arr_desc.startswith('[') else 'JArray<JArray<i32>>'
        _levels = []
        _cur_t = arr_t
        for _ in range(dims):
            _levels.append(_cur_t)
            _m_lv = _re.match(r'JArray<(.+)>$', _cur_t)
            _cur_t = _m_lv.group(1) if _m_lv else 'Object'
        init = f"{_levels[-1].replace('JArray<', 'JArray::<', 1)}::new({sizes[-1]})"
        for _lv in range(dims - 2, -1, -1):
            init = f"{_levels[_lv].replace('JArray<', 'JArray::<', 1)}::new_with({sizes[_lv]}, || {init})"
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: {arr_t} = {init};"))
        sim.push(Var(v), RsNamed(arr_t))
    elif op in ('iastore', 'lastore', 'fastore', 'dastore'):
        val_expr, _val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        val_s = render_expr(val_expr)
        # JVM 操作数栈上 byte/short/char/boolean 都是 int：窄类型局部变量存入宽数组时显式加宽
        _elem_prim = {'iastore': 'i32', 'lastore': 'i64', 'fastore': 'f32', 'dastore': 'f64'}[op]
        _val_prim = render_type(_val_ty)
        if _val_prim in _PRIMITIVE_RUST_TYPES and _val_prim != _elem_prim:
            val_s = f"(({val_s}) as {_elem_prim})"
        _arr_s = render_expr(arr_expr)
        if _is_object_receiver(arr_ty):
            _api = {'iastore': 'array_store_int', 'lastore': 'array_store_long',
                    'fastore': 'array_store_float', 'dastore': 'array_store_double'}[op]
            sim.emit(RawStmt(f"{_arr_s}.{_api}({render_expr(idx_expr)}, {val_s})?;"))
        else:
            sim.emit(RawStmt(f"{_arr_s}.set({render_expr(idx_expr)}, {val_s})?;"))
    elif op == 'aastore':
        val_expr, val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        if _is_object_receiver(arr_ty):
            # 擦除为 Object 的数组接收者：元素类型未知，值统一装箱为 Object 后经
            # Object::array_store_object 转发（协变视图按源元素类型做存储检查）
            val_str = render_expr(val_expr)
            val_ty_str = render_type(val_ty)
            if val_ty_str not in ('Object', '()'):
                val_str = _coerce_to_object(val_str, val_ty_str, registry, sim.class_type_params)
            sim.emit(RawStmt(
                f"{render_expr(arr_expr)}.array_store_object({render_expr(idx_expr)}, {val_str})?;"))
            return True
        _m_aa = _re.match(r'JArray<(.+)>$', arr_ty_str)
        if _m_aa:
            elem_ty = _m_aa.group(1)
        elif arr_ty_str.startswith('Vec<') and arr_ty_str.endswith('>'):
            elem_ty = arr_ty_str[4:-1]
        else:
            elem_ty = 'Object'
        val_str = render_expr(val_expr)
        val_ty_str = render_type(val_ty)
        if (val_ty_str == 'JArray<Object>' and is_jvm_array(elem_ty) and elem_ty != val_ty_str):
            # `spine[i] = (E[]) new Object[n]`：javac 擦除了 unchecked cast，新建数组的元素类型
            # 由它存入的槽位决定 → 数组在创建处即按槽位元素类型实例化（JArray<E>），
            # 而不是先建 JArray<Object> 再转换（两者是不同的运行时类型）
            _fresh_decl = f"let mut {val_str}: JArray<Object> = JArray::<Object>::new("
            for _si in range(len(sim.stmts) - 1, -1, -1):
                _st = sim.stmts[_si]
                if isinstance(_st, RawStmt) and _st.code.startswith(_fresh_decl):
                    _inner_t = elem_ty[len('JArray<'):-1]
                    sim.stmts[_si] = RawStmt(
                        f"let mut {val_str}: {elem_ty} = JArray::<{_inner_t}>::new("
                        + _st.code[len(_fresh_decl):])
                    val_ty_str = elem_ty
                    break
        if elem_ty == 'Object' and val_ty_str not in ('Object', '()'):
            val_str = _coerce_to_object(val_str, val_ty_str, registry, sim.class_type_params)
        elif elem_ty != 'Object' and val_ty_str == 'Object':
            # 元素静态类型比值更具体（checkcast 被验证器省略的位置）：按对象标识还原
            val_str = f"From::from(Clone::clone(&{val_str}))"
        elif val_ty_str not in _PRIMITIVE_RUST_TYPES:
            if (elem_ty != val_ty_str
                    and _is_subtype(erased_base(val_ty_str), erased_base(elem_ty), registry)):
                chain = _into_super_chain(erased_base(val_ty_str), erased_base(elem_ty), registry)
                val_str = f"Clone::clone(&{val_str}){chain}"
            elif elem_ty != val_ty_str:
                # 值静态类型与元素类型无子型关系（`Number[] n = intArr; n[0] = 3.14;`
                # ——元素类型来自值流推断，比 javac 的声明元素类型更精确）：Java 侧按
                # 声明元素类型静态合法，运行时按运行时元素类型检查 → 经 Object 边界
                # 走 aastore 存储检查路径（不满足抛 ArrayStoreException，S-4）
                _val_obj = _coerce_to_object(val_str, val_ty_str, registry,
                                             sim.class_type_params)
                sim.emit(RawStmt(
                    f"Object::from(Clone::clone(&{render_expr(arr_expr)}))"
                    f".array_store_object({render_expr(idx_expr)}, {_val_obj})?;"))
                return True
            else:
                val_str = f"Clone::clone(&{val_str})"
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, {val_str})?;"))
    elif op == 'bastore':
        val_expr, val_ty = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        # boolean[] 在 JVM 中以 bastore 写入，需要 != 0 转换
        if _is_object_receiver(arr_ty):
            # 擦除接收者：boolean[]/byte[] 由 array_store_byte 运行时分派（JVM 两类型共用 bastore）
            sim.emit(RawStmt(
                f"{render_expr(arr_expr)}.array_store_byte({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i32)?;"))
        elif arr_ty_str in ('JArray<bool>', 'Vec<bool>'):
            val_s = render_expr(val_expr)
            coerced = val_s if render_type(val_ty) == 'bool' else f"(({val_s}) as i8 != 0)"
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, {coerced})?;"))
        else:
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i8)?;"))
    elif op == 'sastore':
        val_expr, _ = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        if _is_object_receiver(arr_ty):
            sim.emit(RawStmt(
                f"{render_expr(arr_expr)}.array_store_short({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i32)?;"))
        else:
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i16)?;"))
    elif op == 'castore':
        val_expr, _ = sim.pop(); idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        if _is_object_receiver(arr_ty):
            sim.emit(RawStmt(
                f"{render_expr(arr_expr)}.array_store_char({render_expr(idx_expr)}, ({render_expr(val_expr)}) as i32)?;"))
        else:
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.set({render_expr(idx_expr)}, ({render_expr(val_expr)}) as u16)?;"))
    elif op == 'iaload':
        idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        if _is_object_receiver(arr_ty):
            sim.push(RawExpr(f"{render_expr(arr_expr)}.array_load_int({render_expr(idx_expr)})?"), I32)
        else:
            sim.push(RawExpr(f"{render_expr(arr_expr)}.get({render_expr(idx_expr)})?"), I32)
    elif op in ('baload', 'saload', 'caload'):
        idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        if _is_object_receiver(arr_ty):
            # Object API 已按 JVM 栈形态返回符号/零扩展后的 i32
            _api = {'baload': 'array_load_byte', 'saload': 'array_load_short',
                    'caload': 'array_load_char'}[op]
            sim.push(RawExpr(f"{render_expr(arr_expr)}.{_api}({render_expr(idx_expr)})?"), I32)
        else:
            sim.push(RawExpr(f"({render_expr(arr_expr)}.get({render_expr(idx_expr)})? as i32)"), I32)
    elif op in ('laload', 'faload', 'daload'):
        idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        ty = {'l': I64, 'f': F32, 'd': F64}.get(op[0], I32)
        if _is_object_receiver(arr_ty):
            _api = {'laload': 'array_load_long', 'faload': 'array_load_float',
                    'daload': 'array_load_double'}[op]
            sim.push(RawExpr(f"{render_expr(arr_expr)}.{_api}({render_expr(idx_expr)})?"), ty)
        else:
            sim.push(RawExpr(f"{render_expr(arr_expr)}.get({render_expr(idx_expr)})?"), ty)
    elif op == 'aaload':
        idx_expr = _pop_index(sim); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        if _is_object_receiver(arr_ty):
            sim.push(RawExpr(
                f"{render_expr(arr_expr)}.array_load_object({render_expr(idx_expr)})?"), RsNamed('Object'))
            return True
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
        arr_expr, arr_ty = sim.pop()
        if _is_object_receiver(arr_ty):
            # Object::array_length 返回 Result（null 检查）
            sim.push(RawExpr(f"({render_expr(arr_expr)}.array_length()?)"), I32)
        else:
            # JArray::len 返回 Result：null 数组引用抛 NPE（JVMS §6.5 arraylength）
            sim.push(RawExpr(f"({render_expr(arr_expr)}.len()?)"), I32)
    else:
        return False
    return True
