"""
invoke 指令生成器：invokespecial / invokestatic / invokevirtual / invokedynamic(string concat)。
"""

from ..type_map import short_cls as _short_cls_g
import re
from ..stack import StackSim
from ..rs_ir import (
    Lit, Var, RawExpr, RawStmt, NewPendingExpr, RsNamed,
)
from ..render import render_expr, render_type
from ..sig_parse import parse_method_param_types as _parse_method_param_types
from ..sig_types import method_sig_types as _method_sig_types
from ..type_args import (ancestor_type_args as _ancestor_type_args,
                         enclosing_scope_type_args as _enclosing_scope_type_args,
                         superclass_type_args as _superclass_type_args)
from ..type_map import (
    jvm_to_rust, short_cls, parse_descriptor_params, is_jdk,
    parse_class_type_params as _parse_class_type_params,
    effective_class_type_params as _effective_class_type_params,
)
from ..constants import safe_ident as _safe_field, OBJECT_CLASS as _OBJECT_CLASS, RUST_KEYWORDS as _RUST_KEYWORDS
from ..constants import (
    PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES,
    JAVA_RUNTIME_SHORT_NAMES as _JAVA_RUNTIME_SHORT_NAMES,
)
from .coerce import (
    _coerce_from_null, _coerce_to_object,
    _coerce_value, _escape_str,
)
from .hierarchy import (
    _find_super_chain_to_class,
    _super_prefix_to_expr, _is_subtype, _rust_type_to_binary,
    _get_all_subtypes_ordered,
)
from .member_owner import (
    parse_method_ref,
    _find_method_super_prefix, _find_method_super_prefix_for_type,
    _resolve_method_owner,
    _resolve_static_method_owner,
    _resolve_special_method_owner,
    _resolve_interface_special_target, interface_special_member_name,
)
from .member_naming import (
    _mangle_if_overloaded, _class_known,
    _method_ref_binary_class,
    _method_ref_descriptor,
)
from .invoke_sig import (
    _registry_iface_shorts, _concrete_class_shorts, _downcast_target_valid,
    _lookup_method_sig_params, _lookup_method_sig_ret, _split_type_args, _erased_ret_is_type_var,
    _substitute_tvars, _coerce_arg, receiver_type_arg_map,
)
from .invoke_virtual import _gen_invokevirtual




def _gen_string_concat(sim: StackSim, comment: str, registry: dict | None = None):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for p in reversed(params):
        e_expr, e_ty = sim.pop()
        raw = render_expr(e_expr)
        # Java 浮点数格式化：整数值需显示 .0（如 5.0 而非 5）
        if p in ('D',):
            raw = f'java_fmt_f64({raw})'
        elif p in ('F',):
            raw = f'java_fmt_f32({raw})'
        elif p in ('C',):
            # Java char (u16) 必须转为 Rust char 才能以字符形式格式化
            raw = f"char::from_u32({raw} as u32).unwrap_or('?')"
        elif p in ('Z',):
            # Java 布尔拼接（JLS §5.1.11）呈现 true/false：布尔短路表达式经
            # 分支合并以 i32（1/0）流动，栈类型仍为 bool 时保持原样
            if render_type(e_ty) != 'bool':
                raw = f'({raw} != 0)'
        elif (p.startswith('L') or p.startswith('[')) and p != 'Ljava/lang/String;':
            # 引用类型参数：Java 语义是 String.valueOf(x)（虚 toString 分派，S-3.1
            # 后装箱值是翻译对象）。预物化为临时变量（toString 返回 Result，
            # format! 内不能传播 ?）；Object::toString 经 vtable __obj_str 桥接，
            # null 给出 "null"，与 JVM 一致。String 自身走 Display 快速路径不变。
            _boxed = _coerce_to_object(raw, render_type(e_ty), registry, sim.class_type_params)
            _sv = sim.fresh()
            sim.emit(RawStmt(f"let {_sv}: String = {_boxed}.toString()?;"))
            raw = _sv
        args.insert(0, raw)

    tmpl_m = re.search(r' template:(.+)$', comment)
    if tmpl_m:
        template = tmpl_m.group(1)
        parts = template.split('\x01')

        def _tmpl_seg(p: str) -> str:
            # 模板是常量池解码值：先按 Rust 字面量转义（反斜杠双写、控制字符），
            # 再双写 format! 占位花括号。顺序不可反——_escape_str 不产生花括号，
            # 反过来先双写会把 `{{` 里的 `\` 处理乱。与字面量同一解码值契约。
            return _escape_str(p).replace('{', '{{').replace('}', '}}')

        if len(parts) == len(args) + 1:
            fmt_str = ''.join(
                (_tmpl_seg(p) + '{}' if i < len(args)
                 else _tmpl_seg(p))
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


def _bind_type_args(sig_types: list, arg_tys: list[str], tparams) -> dict[str, str]:
    """形参签名类型与实参静态类型结构化合一，得到类型变量 → 实参的绑定
    （Java 钻石 / 泛型方法推断的静态近似）。

    裸类型变量直接绑定实参类型；同一泛型类的实例化（G<T> ← G<X>）逐类型实参递归。
    基本类型 / Object / 未知类型不产生绑定；同一变量得到互相矛盾的绑定时取 Object。"""
    bound: dict[str, str] = {}

    def _unify(sig_t: str, arg_t: str, nested: bool = False) -> None:
        if sig_t in tparams:
            if arg_t in _PRIMITIVE_RUST_TYPES or arg_t in ('()', '_') or '_' == arg_t.strip():
                return
            if arg_t == 'Object' and not nested:
                # 顶层 Object 实参可经 From<Object> 进入任意类型变量，不构成约束；
                # 类型实参位置（JArray<E> ← JArray<Object>）不变，E 只能是 Object
                return
            if bound.setdefault(sig_t, arg_t) != arg_t:
                bound[sig_t] = 'Object'
            return
        if '<' in sig_t and '<' in arg_t and sig_t.split('<', 1)[0] == arg_t.split('<', 1)[0]:
            s_args = _split_type_args(sig_t[sig_t.index('<') + 1:sig_t.rfind('>')])
            a_args = _split_type_args(arg_t[arg_t.index('<') + 1:arg_t.rfind('>')])
            if len(s_args) == len(a_args):
                for _s, _a in zip(s_args, a_args):
                    _unify(_s, _a, True)

    for _i, _sig_t in enumerate(sig_types):
        if _sig_t and _i < len(arg_tys):
            _unify(_sig_t, arg_tys[_i])
    return bound


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
    cls_tparams = _effective_class_type_params(ci, registry)
    if not cls_tparams:
        return None
    # 规则 1：构造器 generic_signature 的参数位置引用类型变量 → 用实参类型替换
    full_desc = '(' + ''.join(ctor_params) + ')V'
    ctor_sig_params = None
    for m in ci.methods:
        if m.name == '<init>' and m.descriptor == full_desc:
            # 与定义侧同一张形参类型表（含编译器注入的外部实例形参 Outer<E>）
            sp, _ = _method_sig_types(ci, m, cls_tparams, registry)
            if sp and len(sp) == len(ctor_params):
                ctor_sig_params = sp
            break
    subst: dict[str, str] = {}
    if ctor_sig_params and arg_tys:
        subst = _bind_type_args(ctor_sig_params, arg_tys, cls_tparams)
        if len(subst) == len(cls_tparams):
            return [subst[t] for t in cls_tparams]
    # 规则 2：构造类是当前类 / 当前类的内部类，且类型参数名一致
    if caller_class and sim.class_type_params:
        # 同一顶层类之下的内部类共享外部类的类型变量（内部类经 this$N 继承）
        _same_outer = full_cls.split('$', 1)[0] == caller_class.split('$', 1)[0]
        if (full_cls == caller_class or full_cls.startswith(caller_class + '$') or _same_outer) \
                and set(cls_tparams) == set(sim.class_type_params):
            return list(cls_tparams)
    # 规则 3：实参已确定的类型变量取其绑定；未确定的优先取调用方同名类型变量
    # （EnumSet<E> 内构造 RegularEnumSet<E>），否则取 Object——A-1 存储擦除后
    # 实例化只是视图（From 跨实例化成立），Object 恒可行；`_` 会因祖先 From 的
    # 任意实例化拓宽（γ'）失去推断锚点
    out: list[str] = []
    for t in cls_tparams:
        if t in subst:
            out.append(subst[t])
        elif caller_class and sim.class_type_params and t in sim.class_type_params:
            out.append(t)
        else:
            out.append('Object')
    return out


def _ctor_outer_ref_base(cls_short: str | None, params: list[str], registry: dict | None) -> str:
    """内部类构造器的首个形参若是编译器注入的外部类引用（this$N），且定义侧按
    「外部类 + 内部类继承的类型参数」生成（Outer<E>，见 gen_method_body / outer_ref_field_type），
    返回外部类 Rust 短名，否则 ''。此形参的实例化由实参决定（Rust 从实参推断内部类的
    类型参数），调用侧不得按擦除形态 Outer<Object> 转换实参。"""
    if not (cls_short and params and registry):
        return ''
    from ..type_args import outer_instance_rust_type
    from ..type_map import effective_class_type_params, outer_instance_class
    ci = registry.get(_rust_type_to_binary(cls_short, registry) or '')
    if ci is None:
        return ''
    outer_bin = outer_instance_class(ci)
    if not outer_bin or params[0] != f'L{outer_bin};':
        return ''
    outer = outer_instance_rust_type(outer_bin, effective_class_type_params(ci, registry), registry)
    return outer.split('<', 1)[0] if outer else ''


def _super_ctor_view_args(class_name: str, comment: str, registry: dict | None) -> list[str]:
    """super(...) 目标父类在本类定义内部视角下的类型实参（K-5 父类视图实例化）。

    与宏 superclass 属性 / import 扫描同源（SuperclassSignature 逐级代入），直接
    父类为 ancestor_type_args 链首；目标不在链上（兜底路径）时按擦除语义取全
    Object（vtable 非泛型，任意实例化行为一致）。非泛型父类返回 []。"""
    if not registry or not class_name:
        return []
    _ci = registry.get(class_name)
    _tgt = _method_ref_binary_class(comment)
    if _ci is None or not _tgt:
        return []
    from ..type_args import ancestor_type_args as _anc_targs
    for _anc_bin, _anc_as in _anc_targs(_ci, registry):
        if _anc_bin == _tgt:
            return list(_anc_as)
    _tgt_ci = registry.get(_tgt)
    if _tgt_ci is not None:
        _tps = _effective_class_type_params(_tgt_ci, registry)
        if _tps:
            return ['Object'] * len(_tps)
    return []


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    if '<init>' not in comment and '"<init>"' not in comment:
        # super.method() 调用（invokespecial 非构造器）：
        # 不能用 invokevirtual 语义——否则 this.method() 会对被覆盖方法产生无限递归。
        # 必须精确路由到目标父类的 ._super 链，直接调用父类实现，跳过虚拟派发。
        cls_short, mname, params, ret = parse_method_ref(comment)
        # super.method()：声明类的类型形参按本类 SuperclassSignature 链代入
        # （OfDouble extends OfPrimitive<Double, double[], DoubleConsumer> → T_ARR = JArray<f64>），
        # 与宏生成的 `Owner__m_base(this, ..)` 形参类型一致
        _self_ci = registry.get(class_name) if registry else None
        _self_ty = ''
        if _self_ci is not None:
            _self_tps = _effective_class_type_params(_self_ci, registry)
            _self_ty = short_cls(class_name) + (f"<{', '.join(_self_tps)}>" if _self_tps else '')
        sig_params = _lookup_method_sig_params(
            cls_short, mname, params, ret, registry, sim.class_type_params,
            receiver_targ_map=receiver_type_arg_map(_self_ty, cls_short, registry) if _self_ty else None,
            receiver_is_this=True,
            receiver_type=_self_ty or None,
        )
        _sp_owner = _resolve_special_method_owner(
            _method_ref_binary_class(comment), mname, _method_ref_descriptor(comment), registry)
        _owner_short = (_short_cls_g(_sp_owner)
                        if _sp_owner else cls_short) or cls_short
        # 基类调用的被调方是 Owner__m_base 自由函数：形参类型是声明类的类型变量
        # （turbofish 已按本类祖先链绑定实例化），不是擦除存根。sig 查询在
        # receiver_is_this 语境会把接口载体形参按描述符降级 None（防 virtual_in
        # 存根与调用侧发散——但 base 函数不存在该发散）→ 对降级位置按「声明签名
        # 的裸类型变量形参 + turbofish 实例化」重建期望类型，实参与 turbofish 同源
        # （Hashtable$EntrySet 的 super.add(e)：E→Map_Entry 载体被降级 → 实参装箱
        # Object 而 base 形参是 E 实例化 → E0308）。
        if (sig_params and None in sig_params and _self_ci is not None
                and _sp_owner and _owner_short != short_cls(class_name)):
            from ..type_args import (ancestor_vtable_args_by_short as _anc_args_by_short,
                                     split_rust_type_args as _split_rust_args)
            _owner_ci = registry.get(_sp_owner)
            if _owner_ci is not None:
                _owner_eff = _effective_class_type_params(_owner_ci, registry)
                # _owner_targs 形如 '<A, B>'（A 自身可含嵌套泛型）：去首尾角括号后按嵌套感知切分
                _owner_targs = _anc_args_by_short(_self_ci, _self_ty, registry).get(_owner_short, '')
                _owner_inst = _split_rust_args(f"T{_owner_targs}") if _owner_targs else []
                if _owner_inst and len(_owner_inst) == len(_owner_eff):
                    _var_inst = dict(zip(_owner_eff, _owner_inst))
                    _full_desc = '(' + ''.join(params) + ')' + ret
                    _decl_m = next((mm for mm in _owner_ci.methods
                                    if mm.name == mname and mm.descriptor == _full_desc), None)
                    if _decl_m is not None:
                        _decl_types, _ = _method_sig_types(_owner_ci, _decl_m, _owner_eff, registry)
                        if _decl_types and len(_decl_types) == len(sig_params):
                            sig_params = [
                                (_var_inst[_decl_types[i]]
                                 if (sp is None and _decl_types[i] in _var_inst) else sp)
                                for i, sp in enumerate(sig_params)
                            ]
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
        # JVM 方法解析：常量池类（直接父类）未声明该方法时，实际目标是最近的祖先声明者
        # `Iface.super.m()` / 接口私有方法（invokespecial InterfaceMethod）：接口方法体展开在
        # 调用者所在的类上（成员 `Iface_super_m`，由 class_writer 按字节码扫描声明）→ 成员调用
        # （常量池类是接口即属此类，由 registry 判定）
        _iface_owner = _resolve_interface_special_target(
            _method_ref_binary_class(comment), mname, _method_ref_descriptor(comment), registry)
        if _iface_owner:
            _member = _safe_field(interface_special_member_name(
                _iface_owner, mname, _method_ref_descriptor(comment), registry))
            _call = f"{obj_e}.{_member}({', '.join(args)})?"
            rust_ret = jvm_to_rust(ret, registry)
            if rust_ret == '()':
                sim.emit(RawStmt(f"{_call};"))
            else:
                v = sim.fresh()
                if rust_ret == 'Object' and _erased_ret_is_type_var(cls_short, mname, params, ret, registry):
                    sim.emit(RawStmt(f"let {v} = Object::from_any({_call});"))
                else:
                    sim.emit(RawStmt(f"let {v} = {_call};"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
        # _sp_owner/_owner_short 已在 sig_params 重建段解析（本函数前部）
        rust_mname = _safe_field(_mangle_if_overloaded(_owner_short or '', mname, comment, registry))
        base_fn = f"{_owner_short}__{rust_mname}_base"
        # base 函数的泛型形参 = 声明类的类型形参 + 接收者类型；实参不提及声明类类型形参时
        # （onCompletion(CountedCompleter<?>)）无处可推断（E0283）→ 按本类视角的祖先实参显式给出
        if _self_ci is not None and _owner_short != short_cls(class_name):
            from ..type_args import ancestor_vtable_args_by_short as _anc_args_by_short
            _owner_targs = _anc_args_by_short(_self_ci, _self_ty, registry).get(_owner_short, '')
            if _owner_targs:
                base_fn += f"::{_owner_targs[:-1]}, _>"
        elif (_self_ci is not None and _owner_short == short_cls(class_name)
                and sim.class_type_params):
            # 同类泛型 base 调用（私有成员经 invokespecial 自调用，如
            # CombinableMatcher.templatedListWith）：base 函数的类形参在 this
            # 擦除形态上不可推断（E0283）→ 按当前 impl 的类型形参显式给出
            base_fn += f"::<{', '.join(sim.class_type_params)}, _>"
        # base 函数首参是 vtable 引用：宏（rewrite.rs）把字面 this/self 接收者
        # 重写为 `&*this.vtable`；非 this 接收者（synthetic access$ 桥的参数局部，
        # access$100 实证）宏不重写——此处按同一形态发射。
        if obj_e not in ('this', 'self'):
            obj_e = f'&*({obj_e}).vtable'
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
    # javac 私有构造器访问桥（pre-nestmates）：synthetic <init>(..., X$1) 委托到
    # 同类真实私有 <init>（Condition$Matched / ComparatorMatcherBuilder$ComparatorMatcher
    # 实证——hamcrest 闭包首见）。调用点按桥描述符解析（栈上含末位 null token），
    # 发射按委托目标建模（桥体仅转发）：comment/params 换成目标构造器，token 弹栈弃置。
    _ctor_bin = _method_ref_binary_class(comment)
    _ctor_ci = registry.get(_ctor_bin) if registry else None
    if _ctor_ci is not None:
        _bridge_desc = '(' + ''.join(params) + ')V'
        _bridge_m = next((m for m in _ctor_ci.methods
                          if m.name == '<init>' and m.descriptor == _bridge_desc), None)
        if _bridge_m is not None and _bridge_m.is_synthetic:
            for _b_ins in (_bridge_m.instrs or []):
                _b_c = _b_ins.comment or ''
                if (_b_ins.opcode == 'invokespecial' and _b_c.startswith('Method ')
                        and '.<init>:' in _b_c):
                    _bt_cls, _bt_m, _bt_p, _ = parse_method_ref(_b_c)
                    if (_bt_m == '<init>' and _bt_cls == short_cls(_ctor_bin)
                            and len(_bt_p) < len(params)):
                        for _ in range(len(params) - len(_bt_p)):
                            sim.pop()   # 桥 token（aconst_null）弃置
                        params = _bt_p
                        comment = _b_c
                    break
    # 构造目标的类型实参在调用点静态可知的两种情形 → 形参类型按实参替换：
    #   - super(...)：父类形参 ← 本类 SuperclassSignature 的实参
    #   - new 局部 / 匿名类：其类型参数全部继承自外围作用域，按当前作用域实例化
    _caller_ci = registry.get(class_name) if registry else None
    _ctor_targ_map: dict | None = None
    _scope_targs: list[str] | None = None
    if _ctor_ci is not None:
        _ctor_eff = _effective_class_type_params(_ctor_ci, registry)
        if _ctor_eff and getattr(_ctor_ci, 'enclosing_class', ''):
            _scope_targs = _enclosing_scope_type_args(_ctor_ci, registry, sim.class_type_params)
            _ctor_targ_map = dict(zip(_ctor_eff, _scope_targs))
        elif (_ctor_eff and _caller_ci is not None and _caller_ci.super_class == _ctor_bin
              and _ctor_bin != class_name):
            _super_targs = _superclass_type_args(_caller_ci, registry)
            if len(_super_targs) == len(_ctor_eff):
                _ctor_targ_map = dict(zip(_ctor_eff, _super_targs))
    _resolved_targs: list[str] | None = None
    if (_ctor_targ_map is None and _ctor_ci is not None and params
            and len(sim.stack) > len(params)
            and isinstance(sim.stack[-len(params) - 1][0], NewPendingExpr)):
        # 实例化先于实参转换确定：由栈上实参的静态类型合一出构造类的类型实参，
        # 形参期望类型与 turbofish 使用同一份实例化（否则二者各自推断会互相矛盾）
        _peek_tys = [render_type(t) for _, t in sim.stack[-len(params):]]
        _resolved_targs = _resolve_ctor_turbofish_args(
            _ctor_bin, params, _peek_tys, class_name, sim, registry)
        _ctor_eff_p = _effective_class_type_params(_ctor_ci, registry)
        if (_resolved_targs and len(_resolved_targs) == len(_ctor_eff_p)
                and any(t != '_' for t in _resolved_targs)):
            _ctor_targ_map = {p: t for p, t in zip(_ctor_eff_p, _resolved_targs) if t != '_'}
    sig_params_ctor = _lookup_method_sig_params(
        cls, '<init>', params, 'V', registry,
        frozenset() if _ctor_targ_map else sim.class_type_params,
        receiver_targ_map=_ctor_targ_map,
    )
    args = []
    arg_tys = []
    _outer_ref_base = _ctor_outer_ref_base(cls, params, registry)
    _ctor_eff_all = _effective_class_type_params(_ctor_ci, registry) if _ctor_ci is not None else []
    for _idx_c, param_jvm in enumerate(reversed(params)):
        e_expr, e_ty_node = sim.pop()
        e = render_expr(e_expr)
        _pi_c = len(params) - 1 - _idx_c
        _sig_t_c = sig_params_ctor[_pi_c] if sig_params_ctor and _pi_c < len(sig_params_ctor) else None
        expected = _sig_t_c if _sig_t_c is not None else jvm_to_rust(param_jvm, registry)
        ty = render_type(e_ty_node)
        if (_pi_c == 0 and _outer_ref_base and ty.split('<', 1)[0] == _outer_ref_base
                and any(_tp not in (_ctor_targ_map or {})
                        for _tp in re.findall(r'[A-Za-z_]\w*', expected) if _tp in _ctor_eff_all)):
            # 外部实例形参里还有未确定的内部类类型变量：其实例化由实参决定（Rust 从实参推断）。
            # 形参不含类型变量（内部类自带形参、不继承外层变量 → Outer<Object, ..>）时按常规转换
            expected = ty
        e = _coerce_arg(e, e_ty_node, expected, ty, sim, registry)
        args.insert(0, e)
        arg_tys.insert(0, ty)
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        full_cls = obj_expr.class_name          # e.g. 'java/util/ArrayList'
        raw_cls = short_cls(full_cls) or full_cls.rsplit('/', 1)[-1]

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
                _ctor_tparams = (list(_scope_targs) if _scope_targs and full_cls == _ctor_bin
                                 else list(_resolved_targs) if _resolved_targs and full_cls == _ctor_bin
                                 else _resolve_ctor_turbofish_args(
                                     full_cls, params, arg_tys, class_name, sim, registry))
                # 后处理：caller 的 class_type_params 为空（如 java/lang/Class 故意抹去
                # <T>）导致 _coerce_arg 误将 Class_ReflectionData<Object> 包裹进
                # Object::from_any。但 turbofish 已由 arg_tys 推出具体类型，多余的
                # Object::from_any 包装会引发 E0308（Object ≠ Class_ReflectionData<Object>）。
                # 若 turbofish 给出的 T 实参不是 Object/_, 则去掉对应参数的包裹。
                if _ctor_tparams and registry:
                    _ci_ctor2 = registry.get(full_cls)
                    _cls_tp_list2 = (_effective_class_type_params(_ci_ctor2, registry)
                                     if _ci_ctor2 else [])
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
                        _BOX_PREFIXES = ('Object::from_any(', 'Object::from(', 'Into::<Object>::into(')
                        for _si2, _sp_t2 in enumerate(_raw_sp2):
                            if not (_sp_t2 in _tparam_to_turbofish_idx and _si2 < len(args)
                                    and args[_si2].endswith('))')):
                                continue
                            _box_pfx = next((bp for bp in _BOX_PREFIXES
                                             if args[_si2].startswith(bp + 'Clone::clone(&')), None)
                            if _box_pfx is None:
                                continue
                            _tidx2 = _tparam_to_turbofish_idx[_sp_t2]
                            if (_tidx2 < len(_ctor_tparams)
                                    and _ctor_tparams[_tidx2] not in ('Object', '_')):
                                # 形参是类型变量且实参已具体化：撤销向 Object 的上转，保留 Clone::clone(&x)
                                args[_si2] = args[_si2][len(_box_pfx):-1]
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
            # 用户类：new()? 返回 Result<Self>，同样 mangle 重载构造器。
            # 泛型类与 JDK 分支同规则给出 turbofish（A-3）：钻石实例化由
            # `_resolve_ctor_turbofish_args` 的静态近似解析（实参合一 > 同类同名
            # 形参 > Object 擦除兜底）——仅靠 Rust 推断时，实参经 From<Object>
            # 进入类型变量形参（unchecked cast）无锚点（E0283）。
            _ctor_tparams_u: list[str] | None = None
            if registry and _ctor_ci is not None:
                _ctor_eff_u = _effective_class_type_params(_ctor_ci, registry)
                if _ctor_eff_u:
                    _ctor_tparams_u = (list(_scope_targs) if _scope_targs and full_cls == _ctor_bin
                                       else list(_resolved_targs) if _resolved_targs and full_cls == _ctor_bin
                                       else _resolve_ctor_turbofish_args(
                                           full_cls, params, arg_tys, class_name, sim, registry))
            type_params_str_u = ('<' + ', '.join(_ctor_tparams_u) + '>') if _ctor_tparams_u else ''
            # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
            _init_mangled2 = _mangle_if_overloaded(raw_cls, '<init>', comment, registry)
            ctor_name    = _safe_field(_init_mangled2.replace('<init>', 'new'))
            init_expr    = f"{raw_cls}{('::' + type_params_str_u) if type_params_str_u else ''}::{ctor_name}({', '.join(args)})?"
            rust_ty      = raw_cls + type_params_str_u
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
            raw_cls_rust = short_cls(cls)
            if raw_cls_rust in ('Object',) or cls in (_OBJECT_CLASS,):
                sim.emit(RawStmt(f"/* invokespecial {comment} (Object no-op) */"))
            else:
                _init_mangled = _mangle_if_overloaded(cls, '<init>', comment, registry)
                init_on_name = _safe_field(_init_mangled.replace('<init>', '__init_on'))
                arg_str = ', '.join(args)
                super_pfx = _find_super_chain_to_class(class_name, raw_cls_rust, registry) if registry else '_super.'
                if super_pfx:
                    # super(...)（K-5，JVM 单一对象模型）：不重建对象。已存在的 this 经
                    # From<Self> for Parent（宏生成的 vtable 上转，any 共享部件）以父类
                    # 视图传入父类 __init_on —— 父类体内的 putfield 经访问器落在唯一
                    # 身份的 inner 上，this.m() 虚分派命中最终子类的 override；super 前
                    # 已赋字段天然保留（取代 G-11 的 __new_with_super 重建保留）。
                    # JVM 校验器保证 <init> 的 invokespecial 只指向直接父类或同类，
                    # 这里恒为 1 层。泛型父类按本类 SuperclassSignature 的实参给出
                    # 视图实例化（与宏 superclass 属性、import 扫描同源）。
                    _sup_args = _super_ctor_view_args(class_name, comment, registry)
                    _sup_ty = raw_cls_rust + (f"<{', '.join(_sup_args)}>" if _sup_args else '')
                    _path = (f"{raw_cls_rust}::<{', '.join(_sup_args)}>::{init_on_name}"
                             if _sup_args else f"{raw_cls_rust}::{init_on_name}")
                    _view = (f"<{_sup_ty} as ::std::convert::From<Self>>::from"
                             f"(::std::clone::Clone::clone(&this))")
                    sim.emit(RawStmt(
                        f"{_path}({_view}{', ' if args else ''}{arg_str})?;"))
                else:
                    # 同类构造器委托 this(args)：同一身份上执行被委托构造器体（K-5）
                    sim.emit(RawStmt(
                        f"this = Self::{init_on_name}(this{', ' if args else ''}{arg_str})?;"))
        else:
            sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _static_call_turbofish(cls: str, class_name: str, sim: StackSim,
                           registry: dict | None,
                           type_bindings: dict | None = None) -> str:
    """泛型类的静态调用路径（`Cls::<...>::method`）所需的 turbofish。

    静态方法不使用类的类型参数，调用点无上下文可推断（E0283），必须显式给出。
    invokestatic 与 invokedynamic（lambda / 方法引用的实现方法调用）共用本规则。
    cls 可为 Rust 短名或 JVM binary name。
    """
    if not registry:
        return ''
    _cls_bin = cls if cls in registry else _rust_type_to_binary(cls, registry)
    if not _cls_bin:
        return ''
    _cls_ci = registry.get(_cls_bin)
    # 接口的静态成员载体与类同构（携带类级类型参数），turbofish 规则一致
    if not _cls_ci:
        return ''
    # 有效形参：含内部 / 局部类从外围作用域继承的类型变量（struct 的泛型形参同源）
    _tparams = _effective_class_type_params(_cls_ci, registry)
    if not _tparams:
        return ''
    if _cls_bin == class_name and sim.class_type_params:
        # 同类静态调用（如 impl<K,V> TreeNode 内调用 TreeNode::checkInvariants）：
        # 实参是精确泛型形态（HashMap_TreeNode<K,V>），turbofish 用当前 impl 的
        # 类型参数；填 Object 会 E0308（expected X<K,V>, found X<Object,Object>）。
        # type_bindings：按实参绑定的类级类型变量（Optional.ofNullable(U) 等）
        _tb = type_bindings or {}
        return '::<' + ', '.join(_tb.get(_tp, _tp) for _tp in _tparams) + '>'
    # 跨类静态调用：用 Object 擦除（无上下文可推断类型参数）
    return '::<' + ', '.join('Object' for _ in _tparams) + '>'


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    # S-3.1：装箱类的 valueOf 不再跳过 —— javac 插入的 Integer.valueOf 等装箱调用
    # 就是普通 invokestatic，走正常翻译路径调用字节码翻译出的工厂方法
    #（Integer.valueOf 自带 -128~127 缓存池语义）。
    cls, mname, params, ret = parse_method_ref(comment)
    # [equiv-audit] identity-hash（S-6）：System.identityHashCode 静态调用点。
    # 按方法名匹配（字节码常量池事实：该名只由 System 声明），只计数不改发射。
    from .. import equiv_audit
    if mname == 'identityHashCode':
        equiv_audit.record('identity-hash')
    # JVM 方法解析：invokestatic 的常量池类可以是子类，static 方法实际声明在祖先类
    # → 沿父类链解析到声明类（Rust 的关联函数不随继承可见，E0599）。
    _cp_cls_bin = _method_ref_binary_class(comment)
    _cls_path = ''  # 非空：短名在 registry 中跨包重名，调用点改用完整模块路径
    if registry and _cp_cls_bin:
        _s_desc = f"({''.join(params)}){ret}"
        _s_owner = _resolve_static_method_owner(_cp_cls_bin, mname, _s_desc, registry)
        if _s_owner and _s_owner != _cp_cls_bin:
            comment = f"Method {_s_owner}.{mname}:{_s_desc}"
            cls = short_cls(_s_owner)
            _cp_cls_bin = _s_owner
        if _cp_cls_bin in registry and _rust_type_to_binary(cls, registry) != _cp_cls_bin:
            _crate = 'crate' if is_jdk(class_name or '') else 'java_runtime'
            _cls_path = _crate + '::' + '::'.join(
                (f'r#{_p}' if _p in _RUST_KEYWORDS else _p)
                for _p in _cp_cls_bin.split('/')[:-1]) + '::'
    # [equiv-audit] class-init（S-10 子缺口 a）：手写静态 native 的调用点——目标
    # 方法在字节码里是 ACC_NATIVE（实现在手写 *_impl.rs，不在 java_class! 宏块
    # 内）时，入口没有宏注入的 `Self::__class_init()?;`，JVMS §5.5 的该触发点
    # 缺失。按（解析后的声明类, 方法名, 描述符）匹配，只计数不改发射。
    if registry and _cp_cls_bin:
        _ci_nat = registry.get(_cp_cls_bin)
        if _ci_nat is not None:
            if any(m.name == mname and m.is_static and m.is_native
                   and m.descriptor.startswith(f"({''.join(params)})")
                   for m in _ci_nat.methods):
                equiv_audit.record('class-init')
    # 跨类静态调用的实例化先于实参转换确定：类级类型变量由实参静态类型合一得到
    # （`EnumSet.of(e)` → `EnumSet::<Characteristics>::of(e)`），未绑定的取 Object；
    # 形参期望类型、turbofish、返回类型三者共用这一份实例化
    _static_inst: list[str] | None = None
    _static_ci = registry.get(_cp_cls_bin) if registry and _cp_cls_bin else None
    if (_static_ci is not None and _cp_cls_bin != class_name and _static_ci.generic_signature
            and len(sim.stack) >= len(params)):
        _static_tps = _parse_class_type_params(_static_ci.generic_signature)
        _static_m = next((m for m in _static_ci.methods
                          if m.name == mname and m.descriptor == f"({''.join(params)}){ret}"), None)
        if _static_tps and _static_m is not None:
            _raw_sig, _ = _method_sig_types(_static_ci, _static_m, _static_tps, registry)
            _peek_s = [render_type(t) for _, t in sim.stack[len(sim.stack) - len(params):]]
            _bound_s = _bind_type_args(_raw_sig or [], _peek_s, _static_tps)
            _static_inst = [_bound_s.get(t, 'Object') for t in _static_tps]
    # 祖先精化：跨类静态调用的常量池类是当前类的祖先（enum 子类的 valueOf 里
    # invokestatic 基类方法）时，实参未绑定的类型参数按子类泛型签名实例化
    # （Day 的 `Ljava/lang/Enum<LTestEnumBasic$Day;>;` → E=Day）。javac 在该位置
    # 的静态推断即此结果（字节码因擦除退化为 Object，调用点需恢复具体形态，
    # 否则返回值无法构造具体实例化的祖先视图）。
    _static_anc_refined = False
    if (_static_inst and registry and class_name and _cp_cls_bin
            and _cp_cls_bin != class_name):
        _cur_ci = registry.get(class_name)
        if _cur_ci is not None:
            for _anc_bin, _anc_args in _ancestor_type_args(_cur_ci, registry):
                if _anc_bin != _cp_cls_bin:
                    continue
                _refined = [
                    (_anc_args[i] if i < len(_anc_args) and _anc_args[i] != 'Object' else a)
                    for i, a in enumerate(_static_inst)
                ]
                if _refined != _static_inst:
                    _static_inst = _refined
                    _static_anc_refined = any(a != 'Object' for a in _static_inst)
                break
    _static_targ_map = (dict(zip(_parse_class_type_params(_static_ci.generic_signature), _static_inst))
                        if _static_inst else None)
    sig_params_s = _lookup_method_sig_params(
        cls, mname, params, ret, registry,
        frozenset() if _static_targ_map else sim.class_type_params,
        receiver_targ_map=_static_targ_map,
    )
    args = []
    # static 泛型方法的类型变量是方法级的（由 impl 块同名形参承载）：形参是裸类型变量而
    # 实参静态类型不同（Optional<T>.map 内 ofNullable(Object)）→ 该变量按实参绑定
    _static_tbind: dict[str, str] = {}
    if (_static_ci is not None and _cp_cls_bin == class_name and _static_ci.generic_signature
            and sim.class_type_params and len(sim.stack) >= len(params)):
        # 同类静态调用：参数化形参（`Version<T>` ← `Version<T>`）的结构合一先于裸类型变量的
        # 实参绑定——否则擦除的 Object 实参会把 T 绑成 Object，与其余实参的实例化冲突
        _same_tps = _parse_class_type_params(_static_ci.generic_signature)
        _same_m = next((m for m in _static_ci.methods
                        if m.name == mname and m.descriptor == f"({''.join(params)}){ret}"), None)
        if _same_tps and _same_m is not None:
            _same_sig, _ = _method_sig_types(_static_ci, _same_m, _same_tps, registry)
            _peek_same = [render_type(t) for _, t in sim.stack[len(sim.stack) - len(params):]]
            _static_tbind.update(_bind_type_args(_same_sig or [], _peek_same, _same_tps))
    for _idx_s, param_jvm in enumerate(reversed(params)):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        _pi_s = len(params) - 1 - _idx_s
        _sig_t_s = sig_params_s[_pi_s] if sig_params_s and _pi_s < len(sig_params_s) else None
        expected = _sig_t_s if _sig_t_s is not None else jvm_to_rust(param_jvm, registry)
        if (expected in (sim.class_type_params or ()) and ty != expected
                and ty not in _PRIMITIVE_RUST_TYPES and ty != '()'):
            _static_tbind.setdefault(expected, ty)
        e = _coerce_arg(e, ty_node, expected, ty, sim, registry)
        args.insert(0, e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）
    turbofish_bound = False  # turbofish 是否采用了 _static_tbind 的实参绑定

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
        # 短名跨包重名时按常量池里的 binary name 定位声明类（短名反查会命中同名的另一个类）
        rust_mname = _safe_field(_mangle_if_overloaded(
            _cp_cls_bin if _cls_path else cls, mname, comment, registry))
        turbofish = ('::<' + ', '.join(_static_inst) + '>' if _static_inst
                     else _static_call_turbofish(cls, class_name, sim, registry, _static_tbind))
        # 同类静态调用的 turbofish 采用了实参绑定 → 返回类型同步替换
        turbofish_bound = bool(turbofish) and _rust_type_to_binary(cls, registry) == class_name and bool(sim.class_type_params)
        call = f"{_cls_path}{cls}{turbofish}::{rust_mname}({', '.join(args)})"
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
            receiver_type=(f"{short_cls(_cp_cls_bin)}<{', '.join(_static_inst)}>" if _static_inst else None),
        )
        if _sig_ret_s is not None and _static_tbind and turbofish_bound:
            from ..type_args import substitute_type_params as _subst_tp
            _sig_ret_s = _subst_tp(_sig_ret_s, _static_tbind)
        if (_static_anc_refined and _sig_ret_s in (None, rust_ret)
                and rust_ret.split('<')[0].strip() == short_cls(_cp_cls_bin)):
            # 擦除返回即被调类自身的擦除实例化（Enum<Object>）且方法级签名无法给出
            # 更具体类型（valueOf 的 <T> 是方法级变量）→ 与 turbofish 同步为祖先
            # 精化后的实例化（Enum<TestEnumBasic_Day>），否则局部标注与表达式 E0308
            _sig_ret_s = f"{short_cls(_cp_cls_bin)}<{', '.join(_static_inst)}>"
        if rust_ret == 'Object':
            if _sig_ret_s is not None and _sig_ret_s != 'Object':
                # 签名真实返回类型装箱（S-3.1）：registry 类走 Object::from —— vtable
                # 桥接（toString/equals/is_instance_of）与 downcast 还原全部可达；
                # 类型变量走 Into；仅未知形态才 from_any 不透明包装
                sim.emit(RawStmt(f"let {v} = {_coerce_to_object(f'{call}{q}', _sig_ret_s, registry, sim.class_type_params)};"))
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



