"""
invoke 指令生成器：invokespecial / invokestatic / invokevirtual / invokedynamic(string concat)。
"""

import re
from ..stack import StackSim
from ..rs_ir import (
    Lit, Var, RawExpr, RawStmt, NewPendingExpr, RsNamed,
)
from ..render import render_expr, render_type
from ..type_map import (
    jvm_to_rust, short_cls, parse_descriptor_params, is_jdk,
    parse_class_type_params as _parse_class_type_params,
    parse_method_param_types as _parse_method_param_types,
)
from ..constants import safe_ident as _safe_field, OBJECT_CLASS as _OBJECT_CLASS
from .coerce import (
    parse_method_ref, _coerce_from_null, _coerce_to_object,
    _coerce_to_interface, _coerce_value, _find_super_chain_to_class,
    _find_method_super_prefix, _find_method_super_prefix_for_type,
    _super_prefix_to_expr, _resolve_method_owner,
    _mangle_if_overloaded, _class_known, _is_subtype, _rust_type_to_binary,
    _get_all_subtypes_ordered,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL, _PRIMITIVE_RUST_TYPES,
    _JAVA_RUNTIME_SHORT_NAMES,
)
from .invoke_sig import (
    _registry_iface_shorts, _concrete_class_shorts, _downcast_target_valid,
    _lookup_method_sig_params, _lookup_method_sig_ret, _split_type_args, _erased_ret_is_type_var,
    _substitute_tvars, _coerce_arg,
)
from .invoke_virtual import _gen_invokevirtual




def _gen_string_concat(sim: StackSim, comment: str):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for p in reversed(params):
        e_expr, _ = sim.pop()
        raw = render_expr(e_expr)
        # Java 浮点数格式化：整数值需显示 .0（如 5.0 而非 5）
        if p in ('D',):
            raw = f'java_fmt_f64({raw})'
        elif p in ('F',):
            raw = f'java_fmt_f32({raw})'
        elif p in ('C',):
            # Java char (u16) 必须转为 Rust char 才能以字符形式格式化
            raw = f"char::from_u32({raw} as u32).unwrap_or('?')"
        args.insert(0, raw)

    tmpl_m = re.search(r' template:(.+)$', comment)
    if tmpl_m:
        template = tmpl_m.group(1)
        parts = template.split('\x01')
        if len(parts) == len(args) + 1:
            fmt_str = ''.join(
                (p.replace('{', '{{').replace('}', '}}') + '{}' if i < len(args)
                 else p.replace('{', '{{').replace('}', '}}'))
                for i, p in enumerate(parts)
            )
            fmt_args = ', '.join(args)
            if fmt_args:
                # 用 from_owned 避免 From<&str> vs From<std::string::String> 歧义
                sim.push(Lit(f'String::from_owned(format!("{fmt_str}", {fmt_args}))'), RsNamed('String'))
            else:
                sim.push(Lit(f'String::from("{fmt_str}")'), RsNamed('String'))
            return

    # fallback
    if not args:
        sim.push(Lit('String::new()'), RsNamed('String'))
    elif len(args) == 1:
        sim.push(Lit(f'String::from_owned(format!("{{}}", {args[0]}))'), RsNamed('String'))
    else:
        fmt = '{}'.join([''] * (len(args) + 1))  # "{}{}{}" for 3 args
        fmt_args = ', '.join(args)
        sim.push(Lit(f'String::from_owned(format!("{fmt}", {fmt_args}))'), RsNamed('String'))


def _resolve_ctor_turbofish_args(
    full_cls: str,
    ctor_params: list[str],
    arg_tys: list[str],
    caller_class: str | None,
    sim: 'StackSim',
    registry: dict | None,
) -> list[str] | None:
    """推导泛型类构造器的 turbofish 实参（见 _gen_invokespecial 调用处注释）。

    返回实参列表（如 ['K', 'V'] / ['Class_ReflectionData<T>'] / ['Object']），
    类非泛型返回 None。"""
    if not registry:
        return None
    ci = registry.get(full_cls)
    if not ci:
        return None
    cls_tparams = _parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    if not cls_tparams:
        return None
    # 规则 1：构造器 generic_signature 的参数位置引用类型变量 → 用实参类型替换
    full_desc = '(' + ''.join(ctor_params) + ')V'
    ctor_sig_params = None
    for m in ci.methods:
        if m.name == '<init>' and m.descriptor == full_desc:
            if m.generic_signature:
                sp, _ = _parse_method_param_types(m.generic_signature, cls_tparams, registry, is_static=False)
                if sp and len(sp) == len(ctor_params):
                    ctor_sig_params = sp
            break
    if ctor_sig_params and arg_tys:
        subst: dict[str, str] = {}
        for si, sp_t in enumerate(ctor_sig_params):
            if (sp_t in cls_tparams and si < len(arg_tys)
                    and arg_tys[si] not in _PRIMITIVE_RUST_TYPES
                    and arg_tys[si] not in ('Object', '()')):
                subst[sp_t] = arg_tys[si]
        if len(subst) == len(cls_tparams):
            return [subst[t] for t in cls_tparams]
    # 规则 2：构造类是当前类 / 当前类的内部类，且类型参数名一致
    if caller_class and sim.class_type_params:
        if (full_cls == caller_class or full_cls.startswith(caller_class + '$')) \
                and set(cls_tparams) == set(sim.class_type_params):
            return list(cls_tparams)
    # 规则 3：兜底用 _ 让 Rust 从上下文推断（比 Object 更安全，避免 E0308）
    return ['_'] * len(cls_tparams)


def _ctor_outer_ref_base(cls_short: str | None, params: list[str], registry: dict | None) -> str:
    """内部类构造器的首个形参若是编译器注入的外部类引用（this$N），且定义侧按
    「外部类 + 内部类继承的类型参数」生成（Outer<E>，见 gen_method_body / outer_ref_field_type），
    返回外部类 Rust 短名，否则 ''。此形参的实例化由实参决定（Rust 从实参推断内部类的
    类型参数），调用侧不得按擦除形态 Outer<Object> 转换实参。"""
    if not (cls_short and params and registry):
        return ''
    from ..type_map import effective_class_type_params, outer_ref_field_type
    ci = registry.get(_rust_type_to_binary(cls_short, registry) or '')
    if ci is None:
        return ''
    tparams = effective_class_type_params(ci, registry)
    for f in ci.fields:
        if not f.is_static and f.descriptor == params[0]:
            outer = outer_ref_field_type(f, tparams, registry)
            if outer:
                return outer.split('<', 1)[0]
    return ''


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    if '<init>' not in comment and '"<init>"' not in comment:
        # super.method() 调用（invokespecial 非构造器）：
        # 不能用 invokevirtual 语义——否则 this.method() 会对被覆盖方法产生无限递归。
        # 必须精确路由到目标父类的 ._super 链，直接调用父类实现，跳过虚拟派发。
        cls_short, mname, params, ret = parse_method_ref(comment)
        sig_params = _lookup_method_sig_params(
            cls_short, mname, params, ret, registry, sim.class_type_params
        )
        args: list[str] = []
        for _idx, param_jvm in enumerate(reversed(params)):
            e_expr, e_ty_node = sim.pop()
            e_str = render_expr(e_expr)
            # 若有 generic_signature 参数类型（非 None），优先使用；否则降级到 descriptor
            _pi = len(params) - 1 - _idx
            _sig_t = sig_params[_pi] if sig_params and _pi < len(sig_params) else None
            expected_rust = _sig_t if _sig_t is not None else jvm_to_rust(param_jvm, registry)
            actual_rust = render_type(e_ty_node)
            e_str = _coerce_arg(e_str, e_ty_node, expected_rust, actual_rust, sim, registry)
            args.insert(0, e_str)
        obj_expr, _ = sim.pop()
        obj_e = render_expr(obj_expr)
        # vtable 架构：invokespecial 非构造器 = super.method() 调用
        # 宏为每个虚方法生成自由函数 ClassName__method_base(this, args)，绕过虚拟派发
        rust_mname = _safe_field(_mangle_if_overloaded(cls_short or '', mname, comment, registry))
        base_fn = f"{cls_short}__{rust_mname}_base"
        all_args = [obj_e] + args
        arg_str = ', '.join(all_args)
        rust_ret = jvm_to_rust(ret, registry)
        if rust_ret == '()':
            sim.emit(RawStmt(f"{base_fn}({arg_str})?;"))
        else:
            v = sim.fresh()
            if rust_ret == 'Object' and _erased_ret_is_type_var(cls_short, mname, params, ret, registry):
                # super.method() 返回裸类型变量（Reference<T>.get → T）：幂等装箱对齐 sim 的 Object 记录
                sim.emit(RawStmt(f"let {v} = Object::from_any({base_fn}({arg_str})?);"))
            else:
                sim.emit(RawStmt(f"let {v} = {base_fn}({arg_str})?;"))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    cls, _mname_ctor, params, _ret_ctor = parse_method_ref(comment)
    sig_params_ctor = _lookup_method_sig_params(
        cls, '<init>', params, 'V', registry, sim.class_type_params
    )
    args = []
    arg_tys = []
    _outer_ref_base = _ctor_outer_ref_base(cls, params, registry)
    for _idx_c, param_jvm in enumerate(reversed(params)):
        e_expr, e_ty_node = sim.pop()
        e = render_expr(e_expr)
        _pi_c = len(params) - 1 - _idx_c
        _sig_t_c = sig_params_ctor[_pi_c] if sig_params_ctor and _pi_c < len(sig_params_ctor) else None
        expected = _sig_t_c if _sig_t_c is not None else jvm_to_rust(param_jvm, registry)
        ty = render_type(e_ty_node)
        if _pi_c == 0 and _outer_ref_base and ty.split('<', 1)[0] == _outer_ref_base:
            expected = ty
        e = _coerce_arg(e, e_ty_node, expected, ty, sim, registry)
        args.insert(0, e)
        arg_tys.insert(0, ty)
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        full_cls = obj_expr.class_name          # e.g. 'java/util/ArrayList'
        raw_cls = full_cls.rsplit('/', 1)[-1]
        raw_cls = short_cls(raw_cls) or raw_cls

        if '/' in full_cls:
            # JDK class（含包路径）→ 用 new() 工厂（@synthetic）
            rust_ty_str = jvm_to_rust(f'L{full_cls};', registry)
            # 自动装箱优化：原始包装类型（Integer→i32等）直接用值，跳过构造器调用
            _PRIM_TYPES = _PRIMITIVE_RUST_TYPES
            if rust_ty_str in _PRIM_TYPES:
                init_expr = args[0] if args else '0'
                rust_ty = rust_ty_str
                rust_ty_node = RsNamed(rust_ty_str)
            elif rust_ty_str != 'Object' and '<' in rust_ty_str:
                # 泛型类构造：turbofish 实参的推导优先级
                #   1. 构造器 generic_signature 参数引用类型变量（如
                #      SoftReference(T)）→ 用对应实参的 sim 类型替换
                #      （Java 钻石推断的静态近似）
                #   2. 构造类是当前类/当前类的内部类，且类型参数名一致
                #      （如 Class<T> 内构造 Class$ReflectionData<T>）→ 用当前
                #      impl 的类型参数
                #   3. 兜底：Object（擦除）
                _ctor_tparams = _resolve_ctor_turbofish_args(
                    full_cls, params, arg_tys, class_name, sim, registry)
                # 后处理：caller 的 class_type_params 为空（如 java/lang/Class 故意抹去
                # <T>）导致 _coerce_arg 误将 Class_ReflectionData<Object> 包裹进
                # Object::from_any。但 turbofish 已由 arg_tys 推出具体类型，多余的
                # Object::from_any 包装会引发 E0308（Object ≠ Class_ReflectionData<Object>）。
                # 若 turbofish 给出的 T 实参不是 Object/_, 则去掉对应参数的包裹。
                if _ctor_tparams and registry:
                    _ci_ctor2 = registry.get(full_cls)
                    _cls_tp_list2 = (
                        _parse_class_type_params(_ci_ctor2.generic_signature)
                        if _ci_ctor2 and _ci_ctor2.generic_signature else []
                    )
                    _raw_sp2: list | None = None
                    if _ci_ctor2:
                        _full_desc2 = '(' + ''.join(params) + ')V'
                        for _m2 in _ci_ctor2.methods:
                            if _m2.name == '<init>' and _m2.descriptor == _full_desc2 and _m2.generic_signature:
                                _raw_sp2, _ = _parse_method_param_types(
                                    _m2.generic_signature, _cls_tp_list2, registry, is_static=False)
                                if _raw_sp2 and len(_raw_sp2) != len(params):
                                    _raw_sp2 = None
                                break
                    if _raw_sp2:
                        _tparam_to_turbofish_idx = {t: i for i, t in enumerate(_cls_tp_list2)}
                        _OBJ_FROM_PREFIX = 'Object::from_any(Clone::clone(&'
                        for _si2, _sp_t2 in enumerate(_raw_sp2):
                            if (_sp_t2 in _tparam_to_turbofish_idx
                                    and _si2 < len(args)
                                    and args[_si2].startswith(_OBJ_FROM_PREFIX)
                                    and args[_si2].endswith('))')):
                                _tidx2 = _tparam_to_turbofish_idx[_sp_t2]
                                if (_tidx2 < len(_ctor_tparams)
                                        and _ctor_tparams[_tidx2] not in ('Object', '_')):
                                    # 去掉 Object::from_any(Clone::clone(&x)) → Clone::clone(&x)
                                    args[_si2] = args[_si2][len('Object::from_any('):-1]
                type_params_str = ('<' + ', '.join(_ctor_tparams) + '>') if _ctor_tparams else ''
                rust_ty = raw_cls + type_params_str
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                turbofish = '::' + type_params_str
                init_expr = f"{raw_cls}{turbofish}::{ctor_name}({', '.join(args)})?"
            else:
                type_params_str = ''
                rust_ty = raw_cls
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                if args:
                    init_expr = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
                else:
                    init_expr = f"{raw_cls}::{ctor_name}()?"
        elif raw_cls and '/' not in raw_cls:
            # 用户类：new()? 返回 Result<Self>，同样 mangle 重载构造器
            _init_mangled2 = _mangle_if_overloaded(raw_cls, '<init>', comment, registry)
            ctor_name    = _safe_field(_init_mangled2.replace('<init>', 'new'))
            init_expr    = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)
        else:
            init_expr    = f"/* {raw_cls}::new() */"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)

        if sim.stack and isinstance(sim.stack[-1][0], NewPendingExpr):
            sim.stack[-1] = (RawExpr(init_expr), rust_ty_node)
        else:
            v = sim.fresh('_obj')
            sim.emit(RawStmt(f"let mut {v}: {rust_ty} = {init_expr};"))
            sim.push(Var(v), rust_ty_node)
    else:
        obj_e = render_expr(obj_expr)
        obj_e = render_expr(obj_expr)
        if obj_e in ('this', 'self') and cls:
            # super(args) 调用：在子类构造器中初始化 _super 字段
            # java/lang/Object 的 super() 是 no-op（Rust 不需要 Object 初始化）
            raw_cls = cls.rsplit('/', 1)[-1]
            raw_cls_rust = short_cls(raw_cls.replace('$', '_')) or raw_cls.replace('$', '_')
            if raw_cls_rust in ('Object',) or cls in (_OBJECT_CLASS,):
                sim.emit(RawStmt(f"/* invokespecial {comment} (Object no-op) */"))
            else:
                _init_mangled = _mangle_if_overloaded(cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                arg_str = ', '.join(args)
                ctor_call = f"{raw_cls_rust}::{ctor_name}({arg_str})"
                super_pfx = _find_super_chain_to_class(class_name, raw_cls_rust, registry) if registry else '_super.'
                if super_pfx:
                    # super(...)：以已构造好的父类值重建 this（宏的 __new_with_super）。
                    # JVM 校验器保证 <init> 的 invokespecial 只指向直接父类或同类，
                    # 所以这里恒为 1 层，不需要按层数拼 _super 路径。
                    sim.emit(RawStmt(f"this = Self::__new_with_super({ctor_call}?);"))
                else:
                    # 同类构造器委托 this(args)：直接替换 this（初始占位值丢弃）
                    sim.emit(RawStmt(f"this = {ctor_call}?;"))
        else:
            sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    for skip in BOXING_SKIP_STATIC:
        # 按类名边界匹配：裸子串匹配会把类名以装箱类名结尾的其他类
        # （owner 短名仅是后缀相同）误判为自动装箱，导致真实的静态工厂调用被丢弃
        if re.search(r'(?<![A-Za-z0-9_$])' + re.escape(skip) + r'(?![A-Za-z0-9_$])', comment):
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a_expr, _ = sim.pop()
        a = render_expr(a_expr)
        sim.push(Lit(f'String::from_owned(format!("{{}}", {a}))'), RsNamed('String'))
        return

    cls, mname, params, ret = parse_method_ref(comment)
    sig_params_s = _lookup_method_sig_params(
        cls, mname, params, ret, registry, sim.class_type_params
    )
    args = []
    for _idx_s, param_jvm in enumerate(reversed(params)):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        _pi_s = len(params) - 1 - _idx_s
        _sig_t_s = sig_params_s[_pi_s] if sig_params_s and _pi_s < len(sig_params_s) else None
        expected = _sig_t_s if _sig_t_s is not None else jvm_to_rust(param_jvm, registry)
        e = _coerce_arg(e, ty_node, expected, ty, sim, registry)
        args.insert(0, e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）

    # 目标类不在 registry（被截断的内部类如 jdk.internal.*）→ 生成 panic 存根
    if cls and not _class_known(cls, registry):
        rust_ret = jvm_to_rust(ret, registry)
        stub_msg = f"stub: {cls}.{mname}"
        if rust_ret == '()':
            sim.emit(RawStmt(f'panic!("{stub_msg}");'))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f'let {v}: {rust_ret} = panic!("{stub_msg}");'))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    if cls is None or cls == class_name:
        rust_mname = _safe_field(_mangle_if_overloaded(class_name, mname, comment, registry))
        call = f"Self::{rust_mname}({', '.join(args)})"
        needs_q = True
    elif cls and '/' in cls:
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        rust_mname = _safe_field(_mangle_if_overloaded(cls, mname, comment, registry))
        # 泛型类静态方法需要 turbofish，避免 E0283 类型推断歧义
        turbofish = ''
        if registry:
            _cls_bin = _rust_type_to_binary(cls, registry)
            if _cls_bin:
                _cls_ci = registry.get(_cls_bin)
                # 接口在 Rust 侧是 `pub type Iface = Object;`（Arch-1），元数为 0，不带 turbofish
                if _cls_ci and _cls_ci.generic_signature and not _cls_ci.is_interface:
                    _tparams = _parse_class_type_params(_cls_ci.generic_signature)
                    if _tparams:
                        if _cls_bin == class_name and sim.class_type_params:
                            # 同类静态调用（如 impl<K,V> TreeNode 内调用
                            # TreeNode::checkInvariants）：实参是精确泛型形态
                            # （HashMap_TreeNode<K,V>），turbofish 用当前 impl 的
                            # 类型参数；填 Object 会 E0308（expected X<K,V>,
                            # found X<Object,Object>）。
                            turbofish = '::<' + ', '.join(_tparams) + '>'
                        else:
                            # 跨类静态调用：用 Object 擦除（_引发E0283 —— 无上下文可推断K/V）
                            turbofish = '::<' + ', '.join('Object' for _ in _tparams) + '>'
        call = f"{cls}{turbofish}::{rust_mname}({', '.join(args)})"
        needs_q = True

    q = '?' if needs_q else ''
    rust_ret = jvm_to_rust(ret, registry)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{call}{q};"))
    else:
        v = sim.fresh()
        # 签名真实返回（generic_signature）与擦除映射不一致时的对齐：
        # - 擦除映射是 Object（Class→Object 等）：包 from_any 保持 Object 记录
        #   （局部 hint/downcast 负责恢复具体类型）
        # - 擦除映射是擦除实例化（X<Object,Object>）：被调方法声明按
        #   generic_signature 生成（X<K,V> / X<Class<Object>>），调用点记录
        #   必须一致，否则局部标注擦除形态而表达式是精确形态，E0308
        _sig_ret_s = _lookup_method_sig_ret(
            cls, mname, params, ret, registry,
            caller_class=class_name, caller_tparams=sim.class_type_params,
        )
        if rust_ret == 'Object':
            if _sig_ret_s is not None and _sig_ret_s != 'Object':
                sim.emit(RawStmt(f"let {v} = Object::from_any({call}{q});"))
            else:
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
            sim.push(Var(v), RsNamed(rust_ret))
        elif (_sig_ret_s is not None and _sig_ret_s != rust_ret
                and rust_ret not in _PRIMITIVE_RUST_TYPES):
            sim.emit(RawStmt(f"let {v} = {call}{q};"))
            sim.push(Var(v), RsNamed(_sig_ret_s))
        else:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
            sim.push(Var(v), RsNamed(rust_ret))



