# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import LetStmt, RawExpr, RawStmt, RsNamed, Var
from ...render import render_expr, render_type
from ...sig_types import method_sig_types
from ...type_map import (jvm_to_rust, parse_descriptor_params, parse_descriptor_return, short_cls,
                         effective_class_type_params)
from ..invoke import _gen_string_concat, _static_call_turbofish
from ..coerce import (lambda_impl_rust_name, LAMBDA_NAME_LEDGER, _coerce_to_object,
                      _reinstantiate_generic, _PRIMITIVE_RUST_TYPES)


def sim_dynamic(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── invokedynamic ──
    if op == 'invokedynamic':
        if comment and 'makeConcatWithConstants' in comment:
            _gen_string_concat(sim, comment)
        else:
            # 解析 comment 格式：
            # "InvokeDynamic samName:dynDesc [impl:Cls.method:implDesc] [samtype:samDesc]"
            _dyn_desc = ''
            _impl_method_ref = ''  # "Cls.lambda$main$0:desc"
            _sam_type_desc = ''
            if comment:
                _ctoks = comment.split(' ')
                # 第 2 个词是 "samName:dynDesc"
                if len(_ctoks) >= 2:
                    _nd = _ctoks[1]
                    _ci = _nd.find(':')
                    if _ci >= 0:
                        _dyn_desc = _nd[_ci + 1:]
                for _tok in _ctoks[2:]:
                    if _tok.startswith('impl:'):
                        _impl_method_ref = _tok[5:]
                    elif _tok.startswith('samtype:'):
                        _sam_type_desc = _tok[8:]

            # Arch-3: 若有 impl: 和 samtype:，生成真实 Rust 闭包
            _lam_idx = operand or '0'
            _cap_exprs: list[tuple] = []
            if _dyn_desc.startswith('('):
                _cap_descs = parse_descriptor_params(_dyn_desc)
                for _ci_idx, _cd in enumerate(_cap_descs):
                    if sim.stack:
                        _ce, _cty = sim.pop()
                        _cap_exprs.insert(0, (_ce, _cty, _ci_idx))
                    else:
                        _cap_exprs.insert(0, (RawExpr('Object::default()'), RsNamed('Object'), _ci_idx))

            if _impl_method_ref and _sam_type_desc:
                # 解析实现方法：Cls.method:desc 或 pkg/Cls.method:desc
                _dot = _impl_method_ref.rfind('.')
                _impl_colon = _impl_method_ref.find(':', _dot) if _dot >= 0 else -1
                if _dot >= 0 and _impl_colon > _dot:
                    _impl_cls_bin = _impl_method_ref[:_dot]           # "TestLambda" or "pkg/Cls"
                    _impl_mname   = _impl_method_ref[_dot+1:_impl_colon]  # "lambda$main$0"
                    _impl_desc    = _impl_method_ref[_impl_colon+1:]  # "(I)I"
                    # 转换为 Rust 标识符
                    _impl_cls_rust  = short_cls(_impl_cls_bin)      # 内部类 `$` → `_`，与定义侧一致
                    # G-10：实现方法名取 lambda_impl_rust_name 单一来源（定义侧 class_writer
                    # 同源取名），调用点引用名在此登记，生成收尾由账本断言两侧恒等
                    _impl_mname_r = lambda_impl_rust_name(
                        _impl_cls_bin, _impl_mname, _impl_desc, registry)
                    LAMBDA_NAME_LEDGER.record_reference(_impl_cls_bin, _impl_mname, _impl_mname_r)
                    # SAM 方法参数/返回类型 → Rust 类型
                    _sam_params = parse_descriptor_params(_sam_type_desc)
                    _sam_ret    = parse_descriptor_return(_sam_type_desc)
                    _sam_ptypes = [jvm_to_rust(p, registry) for p in _sam_params]
                    _sam_rtype  = jvm_to_rust(_sam_ret, registry) if _sam_ret != 'V' else '()'
                    # 捕获变量声明
                    _cap_var_stmts: list[str] = []
                    _cap_var_names: list[str] = []
                    for _cv_idx, (_cexpr, _cty, _) in enumerate(_cap_exprs):  # _cap_exprs 已按声明顺序排列（pop 时 insert(0)）
                        _cv_name = f'__lam_cap{_lam_idx}_{_cv_idx}'
                        # Java 捕获的是引用副本：被捕获的局部变量在闭包创建后仍可使用，
                        # 因此按 Clone 捕获而非 move（否则 E0382 use after move）
                        _cap_var_stmts.append(f'let {_cv_name} = Clone::clone(&{render_expr(_cexpr)});')
                        _cap_var_names.append(_cv_name)
                    # SAM 参数名
                    _sam_anames = [f'_la{i}' for i in range(len(_sam_ptypes))]
                    # Fn 类型签名（Result 用裸名：user crate 里 crate::error 是 E0433，两边均经 prelude 引入）
                    _fn_params_sig = ', '.join(f'{_a}: {_t}' for _a, _t in zip(_sam_anames, _sam_ptypes))
                    _fn_type = f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)}) -> Result<{_sam_rtype}>>'
                    # 调用实现方法的参数列表（捕获变量 + SAM 参数）
                    # Clone::clone 而非 .clone()：捕获值可能是带 Java clone() 的类
                    # 实例实现方法（捕获 this 的 lambda / 绑定接收者的方法引用）：
                    # 第一个捕获值是接收者，以 &self 形式传入
                    # 构造器引用（<init>）非 static 但无接收者：全部实参进参数表，返回新实例
                    _impl_is_ctor = _impl_mname == '<init>'
                    _impl_is_instance = False
                    _impl_has_generic_sig = False
                    _impl_tparams: list[str] = []
                    _impl_sig_types: list = []
                    _impl_ci = registry.get(_impl_cls_bin) if registry else None
                    if _impl_ci is not None:
                        for _im in _impl_ci.methods:
                            if _im.name == _impl_mname and _im.descriptor == _impl_desc:
                                _impl_is_instance = not _im.is_static and not _impl_is_ctor
                                _impl_has_generic_sig = bool(_im.generic_signature)
                                _impl_tparams = effective_class_type_params(_impl_ci, registry)
                                _impl_sig_types = (method_sig_types(_impl_ci, _im, _impl_tparams, registry)[0]
                                                   or [])
                                break

                    def _is_erased_ref(_d: str) -> bool:
                        """描述符类型在 Rust 侧是否表现为擦除的 Object（接口别名 / 未翻译类）。"""
                        if not (_d.startswith('L') and _d.endswith(';')):
                            return False
                        _dci = registry.get(_d[1:-1]) if registry else None
                        return _dci is None or _dci.is_interface or jvm_to_rust(_d, registry) == 'Object'

                    # 函数式接口的擦除签名（samtype）与实现方法签名之间的适配：
                    # SAM 实参是擦除的 Object、实现方法形参是具体类 → 拆箱（目标类型由形参推断）
                    _impl_params = parse_descriptor_params(_impl_desc)
                    _call_cap_list = [f'Clone::clone(&{v})' for v in _cap_var_names]
                    _call_sam_list = list(_sam_anames)
                    # 实现方法形参表对应 (捕获值 + SAM 实参) 去掉接收者之后的部分
                    _recv_from_sam = _impl_is_instance and not _call_cap_list
                    _sam_param_offset = len(_call_cap_list) - (1 if _impl_is_instance and _call_cap_list else 0)
                    for _si, _sd in enumerate(_sam_params):
                        if _recv_from_sam and _si == 0:
                            continue
                        _pi = _sam_param_offset + _si - (1 if _recv_from_sam else 0)
                        if 0 <= _pi < len(_impl_params):
                            _pd = _impl_params[_pi]
                            if _pd != _sd and _is_erased_ref(_sd) and not _is_erased_ref(_pd):
                                _call_sam_list[_si] = f'{_sam_anames[_si]}.downcast()'
                            elif (_is_erased_ref(_sd) and not _impl_ci.is_interface
                                  and len(_impl_sig_types) == len(_impl_params)
                                  and _impl_sig_types[_pi] in _impl_tparams):
                                # 实现方法形参是声明类的类型变量（`this::addLast`，addLast(E)）：
                                # SAM 的擦除实参经宏补的 From<Object> bound 取回类型变量视图
                                _call_sam_list[_si] = f'From::from({_sam_anames[_si]})'
                            elif len(_sd) == 1 and jvm_to_rust(_pd, registry) == 'Object':
                                # SAM 实参是基本类型、实现方法形参是引用（metafactory 的装箱适配）
                                _call_sam_list[_si] = f'{_sam_anames[_si]}.into()'
                    # 捕获值 → 实现方法形参：形参是擦除引用（接口 / Object）而捕获值是具体类实例
                    # （List<X> 形参捕获 ArrayList 局部）→ Java 的隐式上转，保持对象标识
                    _cap_recv = 1 if (_impl_is_instance and _call_cap_list) else 0
                    for _ci_idx in range(_cap_recv, len(_call_cap_list)):
                        _cp_idx = _ci_idx - _cap_recv
                        _cap_ty = render_type(_cap_exprs[_ci_idx][1])
                        if (_cp_idx < len(_impl_params) and _is_erased_ref(_impl_params[_cp_idx])
                                and _cap_ty not in ('Object', '()', '_')
                                and _cap_ty not in _PRIMITIVE_RUST_TYPES
                                and not _cap_ty.startswith(('JArray<', 'Vec<', '&'))):
                            _call_cap_list[_ci_idx] = _coerce_to_object(
                                _cap_var_names[_ci_idx], _cap_ty, registry, sim.class_type_params)
                        elif _cp_idx < len(_impl_params):
                            # 合成 lambda 方法的形参是擦除实例化（X<Object, Object>），捕获值是精确
                            # 实例化（X<T, bool>）：经 Object 边界重新实例化。
                            # 形参类型与方法定义侧同源：有泛型签名取签名，否则取描述符擦除形态
                            _cap_expected = (_impl_sig_types[_cp_idx]
                                             if len(_impl_sig_types) == len(_impl_params)
                                             else jvm_to_rust(_impl_params[_cp_idx], registry))
                            _re_inst = _reinstantiate_generic(
                                _cap_var_names[_ci_idx], _cap_ty, _cap_expected or '')
                            if _re_inst is not None:
                                _call_cap_list[_ci_idx] = _re_inst
                    if _impl_is_instance and _call_cap_list:
                        # 捕获 this 的 lambda / 绑定接收者的方法引用：第一个捕获值是接收者
                        _call_cap_list[0] = f'&{_cap_var_names[0]}'
                    elif _recv_from_sam and _call_sam_list:
                        # 未绑定接收者的方法引用（X::method）：第一个 SAM 实参是接收者
                        _recv_desc = f'L{_impl_cls_bin};'
                        if _is_erased_ref(_sam_params[0]) and not _is_erased_ref(_recv_desc):
                            _recv_ty = jvm_to_rust(_recv_desc, registry)
                            _call_sam_list[0] = f'&{_sam_anames[0]}.downcast::<{_recv_ty}>()'
                        else:
                            _call_sam_list[0] = f'&{_sam_anames[0]}'
                    _call_cap_args  = ', '.join(_call_cap_list)
                    _call_sam_args  = ', '.join(_call_sam_list)
                    _all_call_args  = ', '.join(filter(None, [_call_cap_args, _call_sam_args]))
                    # 生成闭包
                    for _s in _cap_var_stmts:
                        sim.emit(RawStmt(_s))
                    _cap_move = ' '.join(f'Clone::clone(&{v}),' for v in _cap_var_names)
                    # 泛型类上的实现方法：静态方法不使用类的类型参数，闭包内无上下文可推断（E0283），
                    # 与 invokestatic 同规则显式给出 turbofish；实例实现方法由接收者类型推断，无需给出
                    _impl_turbofish = ('' if _impl_is_instance else
                                       _static_call_turbofish(_impl_cls_bin, class_name, sim, registry))
                    _closure_body = f'{_impl_cls_rust}{_impl_turbofish}::{_impl_mname_r}({_all_call_args})'
                    if _impl_is_instance and _impl_ci is not None and _impl_ci.is_interface:
                        # 实现方法是接口实例方法（`action::accept`）：接收者是擦除的接口引用，
                        # 经与接口同名的载体分派（与 invokeinterface 同形态）
                        _all_args = _call_cap_list + _call_sam_list
                        _iface_tps = effective_class_type_params(_impl_ci, registry)
                        _iface_targs = f"<{', '.join(['Object'] * len(_iface_tps))}>" if _iface_tps else ''
                        _iface_recv = _all_args[0].lstrip('&')
                        _iface_recv_src = f"Clone::clone(&{_iface_recv})"
                        if _call_cap_list:
                            # 绑定接收者是捕获值：静态类型为具体类（`list::add`，list 是 ArrayList<E>）
                            # 时先按对象标识上转为擦除的接口引用
                            _recv_cap_ty = render_type(_cap_exprs[0][1])
                            if (_recv_cap_ty not in ('Object', '()', '_')
                                    and _recv_cap_ty not in _PRIMITIVE_RUST_TYPES):
                                _iface_recv_src = _coerce_to_object(
                                    _iface_recv, _recv_cap_ty, registry, sim.class_type_params)
                        _closure_body = (f"Into::<{_impl_cls_rust}{_iface_targs}>::into({_iface_recv_src})"
                                         f".{_impl_mname_r}({', '.join(_all_args[1:])})")
                                        # 返回值适配：SAM 返回 void → 丢弃实现方法返回值；
                    # SAM 返回擦除的 Object 而实现方法返回具体类型 → 装箱
                    _impl_ret = f'L{_impl_cls_bin};' if _impl_is_ctor else parse_descriptor_return(_impl_desc)
                    if _sam_ret == 'V':
                        if _impl_ret != 'V':
                            _closure_body = f'{_closure_body}?; Ok(())'
                    elif _is_erased_ref(_sam_ret) and _impl_ret != 'V' and (
                            not _is_erased_ref(_impl_ret) or _impl_has_generic_sig):
                        # 按实现方法的返回类型装箱（S-3.1）：registry 类（Integer 等
                        # 翻译类）走 Object::from —— 对象身份、运行时类与接口 vtable
                        # 全部可达；仅未知形态（闭包等）才 from_any 不透明包装
                        _impl_ret_rust = jvm_to_rust(_impl_ret, registry)
                        _closure_body = (f'Ok({_coerce_to_object(f"{_closure_body}?", _impl_ret_rust, registry, sim.class_type_params)})')
                    _lam_varname = f'__lam_{_lam_idx}'
                    # 函数对象以 Object（函数式接口的擦除形态）绑定为 LetStmt：
                    # 在 try / 分支体内创建、体外消费时由变量提升 pass 管理作用域
                    sim.emit(LetStmt(_lam_varname, RsNamed('Object'), False, RawExpr(
                        f'Object::from_any(std::rc::Rc::new('
                        f'move |{_fn_params_sig}| -> Result<{_sam_rtype}> '
                        f'{{ {_closure_body} }}) as {_fn_type})'
                    )))
                    sim.push(Var(_lam_varname), RsNamed('Object'))
                else:
                    sim.emit(RawStmt(f"/* TODO: {op} {operand} (impl parse failed) */"))
                    if _sam_type_desc:
                        _r2 = parse_descriptor_return(_sam_type_desc)
                        if _r2 != 'V':
                            sim.push(RawExpr('Object::default()'), RsNamed('Object'))
            else:
                # 短期占位（无 impl 信息，如方法引用 REF_invokeVirtual 等）
                sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))
                _ret_desc = parse_descriptor_return(_dyn_desc) if _dyn_desc else 'V'
                if _ret_desc != 'V':
                    sim.push(RawExpr('Object::default()'), RsNamed('Object'))

    # ── 同步（忽略，不支持多线程语义）──
    elif op == 'monitorenter':
        sim.pop()  # pop object reference，忽略 monitor
    elif op == 'monitorexit':
        sim.pop()  # pop object reference，忽略 monitor

    # ── 杂项 ──
    elif op in ('nop', 'wide'): pass
    elif op == 'athrow':
        # 被抛出的就是栈顶对象本身：JvmError 携带该对象，异常表匹配 / getMessage /
        # 未捕获报告都基于它的运行时类（参考文档 §8.3）
        e_expr, _ = sim.pop()
        sim.emit(RawStmt(f'return Err(JvmError::from({render_expr(e_expr)}));'))
    else:
        return False
    return True
