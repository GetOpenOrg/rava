# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawExpr, RawStmt, RsNamed
from ...render import render_expr
from ...type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return
from ...constants import safe_ident as _safe_ident
from ..invoke import _gen_string_concat, _static_call_turbofish
from ..coerce import _mangle_if_overloaded


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
                    _impl_cls_rust  = _impl_cls_bin.rsplit('/', 1)[-1]
                    # 实现方法名与定义侧同规则 mangle（方法引用指向重载方法 / 构造器时必须一致）
                    _impl_mangled = _mangle_if_overloaded(
                        _impl_cls_bin, _impl_mname,
                        f"Method {_impl_cls_bin}.{_impl_mname}:{_impl_desc}", registry)
                    _impl_mname_r   = _safe_ident(_impl_mangled.replace('<init>', 'new'))
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
                    _impl_ci = registry.get(_impl_cls_bin) if registry else None
                    if _impl_ci is not None:
                        for _im in _impl_ci.methods:
                            if _im.name == _impl_mname and _im.descriptor == _impl_desc:
                                _impl_is_instance = not _im.is_static and not _impl_is_ctor
                                _impl_has_generic_sig = bool(_im.generic_signature)
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
                                        # 返回值适配：SAM 返回 void → 丢弃实现方法返回值；
                    # SAM 返回擦除的 Object 而实现方法返回具体类型 → 装箱
                    _impl_ret = f'L{_impl_cls_bin};' if _impl_is_ctor else parse_descriptor_return(_impl_desc)
                    if _sam_ret == 'V':
                        if _impl_ret != 'V':
                            _closure_body = f'{_closure_body}?; Ok(())'
                    elif _is_erased_ref(_sam_ret) and _impl_ret != 'V' and (
                            not _is_erased_ref(_impl_ret) or _impl_has_generic_sig):
                        _closure_body = f'Ok(Object::from_any({_closure_body}?))'
                    _lam_varname = f'__lam_{_lam_idx}'
                    sim.emit(RawStmt(
                        f'let {_lam_varname}: {_fn_type} = std::rc::Rc::new('
                        f'move |{_fn_params_sig}| -> Result<{_sam_rtype}> '
                        f'{{ {_closure_body} }});'
                    ))
                    sim.push(RawExpr(f'Object::from_any({_lam_varname})'), RsNamed('Object'))
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

    # ── switch（弹出 key，线性继续，不跳转）──
    elif op in ('tableswitch', 'lookupswitch'):
        key, _ = sim.pop()
        key_s = render_expr(key)
        sim.emit(RawStmt(f"let _switch_key = {key_s};"))

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
