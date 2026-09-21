# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawStmt
from ...render import render_expr, render_type
from ...constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ..coerce import _coerce_to_object, _coerce_value
from ..hierarchy import _is_subtype, _into_super_chain

import re as _re_ret
from .control import _erased_shape


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
        _returns_this = expr_s == 'this' and not sim.is_static
        if _returns_this:
            expr_s = 'Clone::clone(this)'
        if _returns_this and ret_ty in _ctparams:
            # 声明返回类型变量（`return (S) this`，javac 在擦除层省略 checkcast——
            # S 擦除为边界接口、this 已是其实现类，如 AbstractPipeline.sequential）：
            # 经 Object 边界取回，类型形参由宏补 From<Object> bound（身份保持的
            # 视图重建，Java unchecked cast 语义）
            expr_s = f"From::from({_coerce_to_object(expr_s, actual_ty, registry, _ctparams, clone=False)})"
        elif _returns_this and ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            # 声明返回 Object / 接口（`return this` 于返回接口类型的方法）：身份保持的向上转型
            expr_s = _coerce_to_object(expr_s, actual_ty, registry, _ctparams, clone=False)
        elif _returns_this and (ret_ty.split('<')[0] == actual_ty.split('<')[0]
                                or not _is_subtype(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)):
            # 返回类型就是本类：this 的克隆即返回值。
            # 返回类型是祖先类（`return this` 于声明返回父类的方法）→ 落到下方子类型上转分支
            pass
        elif ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            expr_s = _coerce_to_object(expr_s, actual_ty, registry, _ctparams)
        elif ret_ty != 'Object' and actual_ty == 'Object':
            # Object 引用按声明的返回类型返回 = javac 的 checkcast（类型变量位置为 unchecked cast）：
            # 类型变量（宏补 From<Object> bound）与类 wrapper 经 From<Object> 取回；
            # 数组目标（JArray 不在 registry）走 From<Object> for JArray<T> 的数组视图
            # （R9/S-4），不能落到 null 零值；null 字面量 / 无运行时类的返回类型取 null 值
            from ...type_map import _registry_short_index
            _ret_ci = _registry_short_index(registry).get(ret_ty.split('<')[0].strip()) if registry else None
            if expr_s != 'Object::default()' and (
                    ret_ty in _ctparams or ret_ty.startswith('JArray<')
                    or (_ret_ci is not None and not _ret_ci.is_interface)):
                expr_s = f"From::from({expr_s})"
            else:
                expr_s = 'Default::default()'
        elif ('_' in _re_ret.findall(r'\w+', actual_ty)
              and _erased_shape(actual_ty) == _erased_shape(ret_ty)):
            # 同一擦除类型、类型实参待推断（`return new Entry<?,?>[n]`，声明返回 Entry<K,V>[]）：
            # 值原样返回，`_` 由返回类型推断
            pass
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty)
              and _is_subtype(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)):
            # vtable 架构：返回值是子类型，用 From trait（.into()）
            chain = _into_super_chain(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)
            from ..invoke_sig import _upcast_to_ancestor_instantiation
            _reinst_anc = _upcast_to_ancestor_instantiation(expr_s, actual_ty, ret_ty, sim, registry)
            expr_s = _reinst_anc if _reinst_anc is not None else f"{expr_s}{chain}"
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty) and actual_ty != 'Object'):
            # 静态类型互不为子类型（交叉转型 `(Comparator<T> & Serializable)`、同一泛型类的另一实例化）：
            # javac 在此处的转换是运行时校验 → 经 Object 边界按声明的返回类型取回
            expr_s = f"From::from({_coerce_to_object(expr_s, actual_ty, registry, _ctparams)})"
        sim.emit(RawStmt(f"return Ok({expr_s});"))
    else:
        return False
    return True
