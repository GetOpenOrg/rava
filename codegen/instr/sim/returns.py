# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawStmt
from ...render import render_expr, render_type
from ...constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ...stack import erased_base, erased_class_of, is_jvm_array
from ..coerce import _coerce_to_object, _coerce_value
from ..hierarchy import _is_subtype, _into_super_chain, _rust_type_to_binary

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
        # 擦除基名（TypeIR 查询边界）：下方裸名判定 / 子类型判定共用
        _ret_base = erased_base(ret_ty)
        _actual_base = erased_base(actual_ty)
        # 实例方法返回 this 时，this 是 &Self 引用，需要 Clone::clone 才能返回 owned 值
        _returns_this = expr_s == 'this' and not sim.is_static
        if _returns_this:
            expr_s = 'Clone::clone(this)'
        from ...jvm_type import carrier_type_for_ident
        _carrier_ret = carrier_type_for_ident(ret_ty, registry)
        if _returns_this and ret_ty in _ctparams:
            # 声明返回类型变量（`return (S) this`，javac 在擦除层省略 checkcast——
            # S 擦除为边界接口、this 已是其实现类，如 AbstractPipeline.sequential）：
            # 经 Object 边界取回，类型形参由宏补 From<Object> bound（身份保持的
            # 视图重建，Java unchecked cast 语义）
            expr_s = f"From::from({_coerce_to_object(expr_s, actual_ty, registry, _ctparams, clone=False)})"
        elif _returns_this and ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            # 声明返回 Object / 接口（`return this` 于返回接口类型的方法）：身份保持的向上转型
            expr_s = _coerce_to_object(expr_s, actual_ty, registry, _ctparams, clone=False)
        elif (_returns_this and '<' in ret_ty and _ctparams
                and registry and _rust_type_to_binary(_ret_base, registry) == class_name):
            # 泛型类 `return this` 且声明返回本类实例化（combine 的 PairBox<...>——
            # 发射签名按擦除形态 Object 化，体上下文的 this 却是泛型 Self）：
            # 经 Object 边界按返回形态重建（From<Object> for X<A> 任意 A 成立，身份保持）
            from ...type_map import short_cls as _sc_rt
            if _ret_base == _sc_rt(class_name):
                expr_s = (f"<{ret_ty} as ::std::convert::From<Object>>"
                          f"::from(Object::from({expr_s}))")
        elif _carrier_ret is not None and ret_ty == _carrier_ret:
            # A-4 批次 3+：声明返回类型是已铺设的接口载体。
            #   - 值已是同载体：原样返回；
            #   - 值是 Object（擦除边界）：From::from 取回（javac unchecked cast 语义，
            #     接口视图按运行时类成立；返回位有类型上下文，From::from 目标可推断）；
            #   - 具体类 / 其余静态类型（含 return this）：经 Object 边界的协变 upcast
            #     （保持对象身份与运行时类，与 _coerce_arg 的载体分支同源）。
            if actual_ty == ret_ty:
                pass
            elif actual_ty == 'Object':
                expr_s = f"::std::convert::From::from({expr_s})"
            else:
                expr_s = f"::std::convert::From::from({_coerce_to_object(expr_s, actual_ty, registry, _ctparams)})"
        elif _returns_this and (ret_ty == actual_ty
                                or (_ret_base == _actual_base
                                    and '<' not in ret_ty and '<' not in actual_ty)
                                or not _is_subtype(_actual_base, _ret_base, registry)):
            # 返回类型就是本类（同形态）：this 的克隆即返回值。
            # 返回类型是祖先类（`return this` 于声明返回父类的方法）→ 落到下方子类型上转分支；
            # 同类但另一实例化（声明返回擦除形态 X<Object,..>、this 是泛型 Self——
            # combine 的 `return this`）同样下落，经 _upcast_to_ancestor_instantiation
            # 的 Object 边界重建（身份保持）
            pass
        elif ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            expr_s = _coerce_to_object(expr_s, actual_ty, registry, _ctparams)
        elif ret_ty != 'Object' and actual_ty == 'Object':
            # Object 引用按声明的返回类型返回 = javac 的 checkcast（类型变量位置为 unchecked cast）：
            # 类型变量（宏补 From<Object> bound）与类 wrapper 经 From<Object> 取回；
            # 数组目标（JArray 不在 registry）走 From<Object> for JArray<T> 的数组视图
            # （R9/S-4），不能落到 null 零值；null 字面量 / 无运行时类的返回类型取 null 值
            _ret_ref = erased_class_of(ret_ty, registry)
            if expr_s != 'Object::default()' and (
                    ret_ty in _ctparams or is_jvm_array(ret_ty)
                    or (_ret_ref is not None and not _ret_ref.is_interface)):
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
              and _is_subtype(_actual_base, _ret_base, registry)):
            # vtable 架构：返回值是子类型，用 From trait（.into()）
            chain = _into_super_chain(_actual_base, _ret_base, registry)
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
