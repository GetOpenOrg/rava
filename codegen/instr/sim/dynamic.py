# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import RawExpr, RawStmt, RsNamed
from ...render import render_expr
from ...type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return
from ...constants import safe_ident as _safe_ident
from ..invoke import _gen_string_concat


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
                    _impl_mname_r   = _safe_ident(_impl_mname)
                    # SAM 方法参数/返回类型 → Rust 类型
                    _sam_params = parse_descriptor_params(_sam_type_desc)
                    _sam_ret    = parse_descriptor_return(_sam_type_desc)
                    _sam_ptypes = [jvm_to_rust(p, registry) for p in _sam_params]
                    _sam_rtype  = jvm_to_rust(_sam_ret, registry) if _sam_ret != 'V' else '()'
                    # 捕获变量声明
                    _cap_var_stmts: list[str] = []
                    _cap_var_names: list[str] = []
                    for _cv_idx, (_cexpr, _cty, _) in enumerate(reversed(_cap_exprs)):
                        _cv_name = f'__lam_cap{_lam_idx}_{_cv_idx}'
                        _cap_var_stmts.append(f'let {_cv_name} = {render_expr(_cexpr)};')
                        _cap_var_names.append(_cv_name)
                    # SAM 参数名
                    _sam_anames = [f'_la{i}' for i in range(len(_sam_ptypes))]
                    # Fn 类型签名（Result 用裸名：user crate 里 crate::error 是 E0433，两边均经 prelude 引入）
                    _fn_params_sig = ', '.join(f'{_a}: {_t}' for _a, _t in zip(_sam_anames, _sam_ptypes))
                    _fn_type = f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)}) -> Result<{_sam_rtype}>>'
                    # 调用实现方法的参数列表（捕获变量 + SAM 参数）
                    # Clone::clone 而非 .clone()：捕获值可能是带 Java clone() 的类
                    _call_cap_args  = ', '.join(f'Clone::clone(&{v})' for v in _cap_var_names)
                    _call_sam_args  = ', '.join(_sam_anames)
                    _all_call_args  = ', '.join(filter(None, [_call_cap_args, _call_sam_args]))
                    # 生成闭包
                    for _s in _cap_var_stmts:
                        sim.emit(RawStmt(_s))
                    _cap_move = ' '.join(f'Clone::clone(&{v}),' for v in _cap_var_names)
                    _closure_body = f'{_impl_cls_rust}::{_impl_mname_r}({_all_call_args})'
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
        e_expr, _ = sim.pop()
        # "athrow".to_owned() 使用 std::string::String，避免与 java_runtime::String 遮蔽冲突
        sim.emit(RawStmt(f'return Err(JvmError::Custom("athrow".to_owned()));'))
    else:
        return False
    return True
