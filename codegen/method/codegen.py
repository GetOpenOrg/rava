"""
方法体生成：ParsedMethod + ClassInfo → Rust 函数字符串。
处理构造器、静态方法、实例方法、while 循环。

新设计要点：
- 所有方法返回 Result<T>
- 构造器返回 Result<Self>，末尾 Ok(this)
- 实例方法用 &self，局部变量 let this = self; 供字节码指令使用
- void 方法末尾加 Ok(())
- return e; → return Ok(e);
"""

from ..types import ParsedMethod, ClassInfo
from ..type_map import (
    jvm_to_rust, sig_type, rust_default, mangle_name, short_cls,
    parse_method_param_types, parse_field_type, parse_class_type_params,
)
from ..constants import safe_ident, PRIMITIVE_RUST_TYPES as _PRIM_TYPES
from ..stack import StackSim
from ..cfg import (
    find_loops, find_boolean_conditions, find_if_guards, find_if_else,
    find_switches,
    cmp_op, neg_cmp_op, _TWO_OP_BRANCH_OPS,
    function_always_returns,
)
from ..instr import sim_instr
from ..render import render_stmt, render_expr, render_type
from ..rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt,
)
from ..stack import BOOL
from .vars import _coerce_icmp_operand, _coerce_acmp_operand, _str_to_rs_type, _analyze_mutation, _hoist_loop_vars, _hoist_if_vars, _promote_undeclared_assigns
from .postprocess import _normalize_this_clone, _remove_trailing_return_ok, _fix_bool_returns, _add_ok_return, _indent


TWO_OP_CMP = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt',
    'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
})


def _uses_jvm_null_method(ty: str) -> bool:
    """判断类型是否需要用 .is_jvm_null() 检测 null（java_class! 生成类）。
    Object / JArray / Rc / 基本类型 / 泛型参数等走 _is_jnull()，生成类走 .is_jvm_null()。"""
    if ty in ('Object', '()', '') or ty in _PRIM_TYPES:
        return False
    if ty.startswith(('JArray<', 'Rc<', 'Vec<', 'Box<', 'std::')):
        return False
    # 泛型类型参数（单或短大写字母，如 T, E, K, V, R, N）不是生成类
    if len(ty) <= 2 and ty[0].isupper() and ty.rstrip('0123456789').isalpha():
        return False
    return True


def gen_method_body(
    method: ParsedMethod,
    class_info: ClassInfo,
    registry: dict | None = None,
    class_type_params: list[str] | None = None,
    overloaded_names: set[str] | None = None,
    rust_name: str | None = None,
    in_vtable_body: bool = False,
) -> str:
    _class_tparams = class_type_params or []

    # 如果方法有泛型签名，用签名推断参数/返回类型。
    # 注：非泛型类的 generic_signature（如 ClassLoader.getInterfaces0 →
    # Vec<Class<Object>>）同样需要采用 —— 门控不要求类有类型参数；
    # 方法级类型变量（<T> m(T)）在空 class_tparams 下解析为 Object，
    # 与 descriptor 擦除一致，由 _sig_param_valid 兜底。
    if method.generic_signature:
        sig_param_types, sig_ret_type = parse_method_param_types(
            method.generic_signature, _class_tparams, registry
        )
    else:
        sig_param_types, sig_ret_type = [], ''

    instrs           = method.instrs
    off2idx          = {ins.offset: idx for idx, ins in enumerate(instrs)}
    _loops           = find_loops(instrs)
    loop_map         = {lp.start_idx: lp for lp in _loops}
    bool_cond_map    = find_boolean_conditions(instrs)
    _raw_guards      = find_if_guards(instrs, _loops)
    # boolean-condition 模式优先级更高，排除重叠的 guard 检测
    if_guard_map     = {k: v for k, v in _raw_guards.items() if k not in bool_cond_map}
    # if-else / simple if-then（排除已处理的模式）
    if_else_map      = find_if_else(instrs, _loops, bool_cond_map, if_guard_map)
    switch_map       = find_switches(instrs)
    param_types      = method.param_types

    # 参数类型：如果泛型签名提供了类型变量或更具体的参数化类型，优先使用
    def _sig_param_valid(sp: str) -> bool:
        """检查 generic_signature 派生的参数类型是否可用。
        满足以下任一条件则有效：
          1. sp 是类级类型参数（T/E/K/V 等）
          2. sp 中所有标识符均为已知类型（内建/类级参数/注册表中存在/大写开头的 Java 类名）
        大写开头名称视为合法 Java 短类名（可能由 glob import 引入），只拒绝
        全小写且不在内建集合的标识符（如未知 Rust 语法符）。
        """
        if sp in _class_tparams:
            return True
        _builtin = frozenset({
            'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
            'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell', 'usize', 'u8',
            'JArray',  # Rust 端数组包装，不对应 Java 类
        })
        _reg_shorts = (
            {k.rsplit('/', 1)[-1].replace('$', '_') for k in registry}
            if registry else set()
        )
        import re as _re
        for name in _re.findall(r'[A-Za-z_][A-Za-z0-9_]*', sp):
            if name in _builtin or name in _class_tparams or name in _reg_shorts:
                continue
            return False
        return True

    # T-2：接口类型在方法签名中擦除为 Object；用 registry 动态检测（无硬编码 JDK 名，Principle 4）。
    from ..instr.invoke import _registry_iface_shorts as _reg_iface_shorts_fn
    _iface_shorts = _reg_iface_shorts_fn(registry)
    import re as _re_iface
    def _is_iface_type(sp: str) -> bool:
        m = _re_iface.match(r'^(\w+)(?:<|$)', sp)
        return bool(m and m.group(1) in _iface_shorts)

    if sig_param_types and len(sig_param_types) == len(param_types):
        jps = [jvm_to_rust(t, registry) for t in param_types]
        rust_param_types = [
            sp if (_sig_param_valid(sp) and not _is_iface_type(sp)) else jp
            for sp, jp in zip(sig_param_types, jps)
        ]
    else:
        rust_param_types = [jvm_to_rust(t, registry) for t in param_types]

    # 内部类构造器 this$N 参数类型修正：
    # 内部类构造器第一个参数（外部类实例）由编译器注入，无 generic_signature。
    # 当内部类继承了外部类的类型参数（如 ArrayList_Itr<E>），将擦除形态（ArrayList<Object>）
    # 替换为带类型参数的版本（ArrayList<E>），使 __set_this_0(arg_0) 类型匹配。
    if method.is_constructor and _class_tparams and registry:
        import re as _re_ctor_ic
        _outer_field_map: dict[str, str] = {}
        for _f in class_info.fields:
            if _f.is_static:
                continue
            if _re_ctor_ic.match(r'^this\$\d+$', _f.name):
                _om = _re_ctor_ic.match(r'L([^;]+);', _f.descriptor)
                if _om:
                    _outer_ci = registry.get(_om.group(1))
                    if _outer_ci and _outer_ci.generic_signature:
                        _outer_tp = parse_class_type_params(_outer_ci.generic_signature)
                        if _outer_tp and len(_outer_tp) <= len(_class_tparams):
                            _outer_short = _om.group(1).rsplit('/', 1)[-1].replace('$', '_')
                            _outer_field_map[_f.descriptor] = (
                                _outer_short + '<' + ', '.join(_class_tparams[:len(_outer_tp)]) + '>'
                            )
        if _outer_field_map:
            rust_param_types = [
                _outer_field_map.get(param_types[k], rt) if k < len(param_types) else rt
                for k, rt in enumerate(rust_param_types)
            ]

    # 返回类型：如果泛型签名返回值是有效类型且非接口，优先使用；接口类型回退到描述符（Object）。
    if sig_ret_type and _sig_param_valid(sig_ret_type) and not _is_iface_type(sig_ret_type):
        rust_ret = sig_ret_type
    else:
        rust_ret = jvm_to_rust(method.return_type, registry)
    is_ctor          = method.is_constructor
    is_static        = method.is_static

    local_names = method.local_names or {}

    # 计算最终 Rust 方法名（有重载则加描述符后缀；Rust 关键字加 _ 后缀）
    # 若 emitter 传入经过去重的 rust_name，直接使用（避免 E0592 重复定义）
    _overloaded = overloaded_names is not None and method.name in overloaded_names
    if rust_name is not None:
        rust_fn_name = safe_ident(rust_name)
    elif is_ctor:
        rust_fn_name = mangle_name('new', method.descriptor) if _overloaded else 'new'
    elif _overloaded:
        rust_fn_name = mangle_name(method.name, method.descriptor)
    else:
        rust_fn_name = safe_ident(method.name)

    def _param_name(slot: int, fallback: str) -> str:
        return safe_ident(local_names.get(slot, fallback))

    # ── 函数签名 ──────────────────────────────────────────────────────
    # JVM wide 类型（J=long, D=double）各占 2 个 slot；签名生成需按实际 slot 查名字
    def _params_with_slot(start_slot: int, type_strings: list[str], use_sig_type: bool) -> list[str]:
        """生成参数列表，正确追踪 wide 类型（J/D）占用的 slot 数量。"""
        result = []
        slot = start_slot
        for k, rt in enumerate(type_strings):
            raw_jvm = param_types[k] if k < len(param_types) else ''
            name = _param_name(slot, f'arg_{k}')
            ty = sig_type(rt) if use_sig_type else rt
            result.append(f"mut {name}: {ty}")
            slot += 2 if raw_jvm in ('J', 'D') else 1
        return result

    if is_ctor:
        # 构造器：Result<Self>，参数从 slot 1 开始
        params = _params_with_slot(1, rust_param_types, use_sig_type=False)
        sig = f"pub fn {rust_fn_name}({', '.join(params)}) -> Result<Self>"

    elif method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        sig = "pub fn main() -> Result<()>"

    elif is_static:
        params = _params_with_slot(0, rust_param_types, use_sig_type=True)
        sig = f"pub fn {rust_fn_name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> Result<{rust_ret}>"
        else:
            sig += " -> Result<()>"

    else:
        # 实例方法：&self，参数从 slot 1 开始
        params = ["&self"] + _params_with_slot(1, rust_param_types, use_sig_type=False)
        sig = f"pub fn {rust_fn_name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> Result<{rust_ret}>"
        else:
            sig += " -> Result<()>"

    # 静态方法参数签名用 sig_type（Vec<T> → &[T]），sim 需与签名一致避免双重引用
    sim_param_types = [sig_type(t) if is_static else t for t in rust_param_types]
    rust_param_type_nodes = [_str_to_rs_type(t) for t in sim_param_types]

    # 从 LocalVariableTypeTable 预计算精确类型提示（slot → RsType）
    # 仅对引用类型（Object 类型擦除后变成 Object 的槽）有意义
    slot_hint_types: dict[int, RsNamed] = {}
    slot_hint_starts: dict[int, int] = {}  # slot → LVTT start_pc，用于检测 slot 复用
    if method.local_types:
        for _hint_slot, (_hint_sig, _hint_start) in method.local_types.items():
            rust_ty_name = parse_field_type(_hint_sig, _class_tparams, registry)
            if rust_ty_name and rust_ty_name != 'Object':
                slot_hint_types[_hint_slot] = RsNamed(rust_ty_name)
                slot_hint_starts[_hint_slot] = _hint_start

    # LVT 逐变量声明类型：slot → [(start_pc, end_pc, RsType)]。
    # - 声明类型为 Object/接口 的变量：生命周期内所有 store 以 Object 形态写入（Java 隐式向上转型）
    # - load 命中当前 sim 未登记的槽（声明在兄弟分支的子 sim 中）时，以声明类型而非 i32 兜底
    # 接口在 Rust 层是 `type Iface = Object` 别名（jvm_to_rust 返回其全路径），同属 Object 形态。
    slot_var_types: dict[int, list[tuple[int, int, RsNamed]]] = {}
    for _lr_slot, _lr_entries in (method.local_ranges or {}).items():
        for _lr_start, _lr_len, _lr_desc in _lr_entries:
            _lr_ci = registry.get(_lr_desc[1:-1]) if (registry and _lr_desc.startswith('L')) else None
            _lr_rust = 'Object' if (_lr_ci is not None and _lr_ci.is_interface) else jvm_to_rust(_lr_desc, registry)
            slot_var_types.setdefault(_lr_slot, []).append(
                (_lr_start, _lr_start + _lr_len, _str_to_rs_type(_lr_rust)))

    sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names,
                   slot_hint_types=slot_hint_types, slot_hint_starts=slot_hint_starts,
                   slot_var_types=slot_var_types,
                   return_type=rust_ret, is_constructor=is_ctor, class_type_params=_class_tparams,
                   in_vtable_body=in_vtable_body)
    # 记录参数和 this 的名字（在函数签名中已声明，无需提升）
    predeclared: set[str] = {name for name, _, _ in sim.locals.values()}

    # entries: list of (indent: str, item: RsStmt | str)
    # - str 条目是已缩进的原始代码行（loop {, if cond { break; }, } 等）
    # - RsStmt 条目是 IR 节点，等待 mutation 分析后再渲染
    entries: list = []

    # ── 构造器：创建 this ───────────────────────────────────────────
    if is_ctor:
        # struct 是 java_class! 宏生成的 newtype（`Name(RefCell/Name__inner)`），
        # 无法用结构体字面量构造。宏为 Inner 派生 Default（各字段取类型默认值，
        # 与 JVM 的零初始化语义一致），因此统一用 `Self::default()` 起手，
        # 后续 putfield 走 `__set_xxx` 访问器逐字段赋值。
        struct_init = "Self::default()"
        entries.append(('', f"    let mut this = {struct_init};"))
        # 标记为非 null（Default 初始化时 _jvm_null=true，构造完成后清零）
        entries.append(('', "    this._init_not_null();"))
        # 泛型类的 this 带类型参数（与 StackSim 实例方法路径一致，避免裸名 E0107/E0308）
        _this_rust = short_cls(method.class_name)
        if _this_rust and _class_tparams:
            _this_rust = f"{_this_rust}<{', '.join(_class_tparams)}>"
        sim.locals[0] = ('this', RsNamed(_this_rust), False)

    elif not is_static:
        # 实例方法：绑定 this = self，供字节码（aload_0 + getfield/putfield）使用
        entries.append(('', "    let this = self;"))

    # ── 递归指令处理器 ──────────────────────────────────────────────────
    # process_block(start, end, cur_sim, out, ind) 处理 instrs[start:end]，
    # 支持任意嵌套的 loop / guard / if-else，正确管理缩进级别。
    def process_block(start: int, end: int, cur_sim: StackSim,
                      out: list, ind: str) -> None:

        def flush_here() -> None:
            for stmt in cur_sim.stmts:
                out.append((ind, stmt))
            cur_sim.stmts.clear()

        def make_sub() -> StackSim:
            s = StackSim(
                rust_param_type_nodes, is_static, method.class_name, local_names,
                slot_hint_types=slot_hint_types, slot_hint_starts=slot_hint_starts,
                slot_var_types=slot_var_types,
                return_type=rust_ret, is_constructor=is_ctor, class_type_params=_class_tparams,
                in_vtable_body=cur_sim.in_vtable_body,
            )
            s.locals = dict(cur_sim.locals)
            s._slot_decl_depth = dict(cur_sim._slot_decl_depth)
            s.stack = list(cur_sim.stack)
            s._ctr = cur_sim._ctr  # 从父 sim 继承计数器，防止嵌套块生成与外层同名的临时变量
            s.current_offset = cur_sim.current_offset
            return s

        def pop_fall_cond(op: str) -> str:
            """弹出操作数，返回 fall-through 条件（跳转条件的否定）。"""
            if op in TWO_OP_CMP:
                b_e, b_t = cur_sim.pop(); a_e, a_t = cur_sim.pop()
                if op in ('if_acmpeq', 'if_acmpne'):
                    a_s = _coerce_acmp_operand(render_expr(a_e), a_t)
                    b_s = _coerce_acmp_operand(render_expr(b_e), b_t)
                else:
                    a_s = _coerce_icmp_operand(render_expr(a_e), a_t)
                    b_s = _coerce_icmp_operand(render_expr(b_e), b_t)
                return neg_cmp_op(op, a_s, b_s)
            else:
                a_e, a_t = cur_sim.pop()
                a_s = render_expr(a_e)
                a_ty = render_type(a_t)
                a_is_b = (str(a_t) == 'bool' or getattr(a_t, 'name', '') == 'bool')
                if a_is_b and op in ('ifeq', 'ifne'):
                    return f'!({a_s})' if op == 'ifne' else a_s
                # ifnull/ifnonnull：fall-through 条件（分支不跳转时成立）
                # ifnull 跳转条件=「为空」→ fall-through=「非空」
                # ifnonnull 跳转条件=「非空」→ fall-through=「为空」
                # java_class! 生成类走 .is_jvm_null()；Object/JArray/Rc 等走 _is_jnull()
                if op == 'ifnull':
                    if _uses_jvm_null_method(a_ty):
                        return f'!{a_s}.is_jvm_null()'
                    return f'!_is_jnull(&{a_s})'
                if op == 'ifnonnull':
                    if _uses_jvm_null_method(a_ty):
                        return f'{a_s}.is_jvm_null()'
                    return f'_is_jnull(&{a_s})'
                return neg_cmp_op(op, a_s, '')

        i = start
        while i < end:
            ins = instrs[i]
            op  = ins.opcode

            # ── 循环 ──────────────────────────────────────────────────
            if i in loop_map:
                lp = loop_map[i]
                if lp.end_idx >= end:
                    # 循环超出当前块范围（不应发生），当普通指令处理
                    cur_sim.current_offset = ins.offset
                    sim_instr(ins, cur_sim, method.class_name, registry=registry)
                    flush_here()
                    i += 1
                    continue

                out.append(('', f"{ind}loop {{"))

                if lp.cond_idx is None:
                    # 无条件循环（for(;;) / while(true)）：直接执行整个循环体，无 break
                    body_s = make_sub()
                    body_s.enter_scope()
                    process_block(lp.start_idx, lp.end_idx, body_s, out, ind + "    ")
                    body_s.exit_scope()
                    cur_sim.locals = body_s.locals
                    cur_sim._slot_decl_depth = body_s._slot_decl_depth
                    out.append(('', f"{ind}}}"))
                    i = lp.end_idx + 1
                    continue

                is_do_while = (lp.cond_idx == lp.end_idx)

                if is_do_while:
                    # do-while：先执行体，再检测后向条件
                    body_s = make_sub()
                    body_s.enter_scope()
                    process_block(lp.start_idx, lp.cond_idx, body_s, out, ind + "    ")
                    body_s.exit_scope()
                    cur_sim.locals = body_s.locals
                    cur_sim._slot_decl_depth = body_s._slot_decl_depth

                    # 条件：后向分支 "if cond goto start" → Rust "if !cond { break; }"
                    ci = instrs[lp.cond_idx]
                    cond_s = make_sub()
                    cond_s.stack = list(body_s.stack)
                    if ci.opcode in TWO_OP_CMP:
                        b_e, b_t = cond_s.pop(); a_e, a_t = cond_s.pop()
                        if ci.opcode in ('if_acmpeq', 'if_acmpne'):
                            a_c = _coerce_acmp_operand(render_expr(a_e), a_t)
                            b_c = _coerce_acmp_operand(render_expr(b_e), b_t)
                        else:
                            a_c = _coerce_icmp_operand(render_expr(a_e), a_t)
                            b_c = _coerce_icmp_operand(render_expr(b_e), b_t)
                        cond_str = neg_cmp_op(ci.opcode, a_c, b_c)
                    else:
                        a_e, a_t = cond_s.pop()
                        a_s2 = render_expr(a_e)
                        a_is_b2 = (str(a_t) == 'bool' or getattr(a_t, 'name', '') == 'bool')
                        if a_is_b2 and ci.opcode in ('ifeq', 'ifne'):
                            cond_str = f'!({a_s2})' if ci.opcode == 'ifne' else a_s2
                        else:
                            cond_str = neg_cmp_op(ci.opcode, a_s2, '')
                    out.append(('', f"{ind}    if {cond_str} {{ break; }}"))
                else:
                    # while：先 pre-condition（递归，支持 body 内嵌 if/guard），再检测条件，再执行体
                    pre = make_sub()
                    pre.enter_scope()
                    pre_out: list = []
                    process_block(lp.start_idx, lp.cond_idx, pre, pre_out, ind + "    ")
                    pre.exit_scope()
                    out.extend(pre_out)
                    for s in pre.stmts:
                        out.append((ind + "    ", s))
                    pre.stmts.clear()
                    cur_sim.locals = pre.locals
                    cur_sim._slot_decl_depth = pre._slot_decl_depth

                    ci = instrs[lp.cond_idx]
                    cond_s = make_sub()
                    cond_s.stack = list(pre.stack)
                    if ci.opcode in TWO_OP_CMP:
                        b_e, b_t = cond_s.pop(); a_e, a_t = cond_s.pop()
                        if ci.opcode in ('if_acmpeq', 'if_acmpne'):
                            a_c = _coerce_acmp_operand(render_expr(a_e), a_t)
                            b_c = _coerce_acmp_operand(render_expr(b_e), b_t)
                        else:
                            a_c = _coerce_icmp_operand(render_expr(a_e), a_t)
                            b_c = _coerce_icmp_operand(render_expr(b_e), b_t)
                        cond_str = cmp_op(ci.opcode, a_c, b_c)
                    else:
                        a_e, a_t = cond_s.pop()
                        a_s2 = render_expr(a_e)
                        a_ty2 = render_type(a_t)
                        a_is_b2 = (str(a_t) == 'bool' or getattr(a_t, 'name', '') == 'bool')
                        if a_is_b2 and ci.opcode in ('ifeq', 'ifne'):
                            cond_str = f'!({a_s2})' if ci.opcode == 'ifeq' else a_s2
                        elif ci.opcode == 'ifnull':
                            if _uses_jvm_null_method(a_ty2):
                                cond_str = f'{a_s2}.is_jvm_null()'
                            else:
                                cond_str = f'_is_jnull(&{a_s2})'
                        elif ci.opcode == 'ifnonnull':
                            if _uses_jvm_null_method(a_ty2):
                                cond_str = f'!{a_s2}.is_jvm_null()'
                            else:
                                cond_str = f'!_is_jnull(&{a_s2})'
                        else:
                            cond_str = cmp_op(ci.opcode, a_s2, '')
                    out.append(('', f"{ind}    if {cond_str} {{ break; }}"))

                    body_s = make_sub()
                    body_s.enter_scope()
                    process_block(lp.cond_idx + 1, lp.end_idx, body_s, out, ind + "    ")
                    body_s.exit_scope()
                    cur_sim.locals = body_s.locals
                    cur_sim._slot_decl_depth = body_s._slot_decl_depth

                out.append(('', f"{ind}}}"))
                i = lp.end_idx + 1
                continue

            # ── condition→boolean ─────────────────────────────────────
            if i in bool_cond_map:
                true_val, false_val, false_idx, end_idx = bool_cond_map[i]
                is_two_op = op in TWO_OP_CMP
                if is_two_op:
                    b_expr, b_ty = cur_sim.pop(); a_expr, a_ty = cur_sim.pop()
                    if op in ('if_acmpeq', 'if_acmpne'):
                        a_str = _coerce_acmp_operand(render_expr(a_expr), a_ty)
                        b_str = _coerce_acmp_operand(render_expr(b_expr), b_ty)
                    else:
                        a_str = _coerce_icmp_operand(render_expr(a_expr), a_ty)
                        b_str = _coerce_icmp_operand(render_expr(b_expr), b_ty)
                else:
                    a_expr, a_type = cur_sim.pop()
                    a_str = render_expr(a_expr); b_str = ''
                    a_is_bool = (str(a_type) == 'bool' or getattr(a_type, 'name', '') == 'bool')
                    if a_is_bool and op in ('ifne', 'ifeq'):
                        base_bool = a_str if op == 'ifne' else f'!({a_str})'
                        bool_expr = f'!({base_bool})' if (true_val == 1 and false_val == 0) else base_bool
                        cur_sim.push(RawExpr(bool_expr), BOOL)
                        flush_here()
                        i = end_idx
                        continue
                if true_val == 1 and false_val == 0:
                    bool_expr = neg_cmp_op(op, a_str, b_str)
                else:
                    bool_expr = cmp_op(op, a_str, b_str)
                cur_sim.push(RawExpr(bool_expr), BOOL)
                flush_here()
                i = end_idx
                continue

            # ── if-guard（fall-through 必定退出）──────────────────────
            if i in if_guard_map:
                guard = if_guard_map[i]
                fall_cond = pop_fall_cond(op)
                flush_here()
                # fall_cond 静态为 false：guard body 是死代码，直接跳过
                if fall_cond == 'false':
                    i = guard.continue_idx
                    continue
                out.append(('', f"{ind}if {fall_cond} {{"))
                inner = make_sub()
                inner.enter_scope()
                inner_out: list = []
                # 用递归 process_block 处理 guard body，支持非线性 body（含嵌套条件分支）
                process_block(guard.body_start_idx, guard.continue_idx, inner, inner_out, ind + "    ")
                inner.exit_scope()
                cur_sim._ctr = max(cur_sim._ctr, inner._ctr)
                out.extend(inner_out)
                for s in inner.stmts:
                    out.append((ind + "    ", s))
                out.append(('', f"{ind}}}"))
                # F-2 fix: 若 guard 的 continue 超过当前块末且超过最内层循环末，
                # else 分支（跳转目标）是循环出口 → 需要 break;
                # 例：while (j>=0 && arr[j]>key) 的第二条件，if-guard 结束后缺 break
                if guard.continue_idx > end:
                    for _glp in _loops:
                        if _glp.start_idx <= i <= _glp.end_idx and guard.continue_idx > _glp.end_idx:
                            out.append(('', f"{ind}break;"))
                            break
                i = guard.continue_idx
                continue

            # ── if-else / simple if-then ──────────────────────────────
            if i in if_else_map:
                ie = if_else_map[i]
                fall_cond = pop_fall_cond(op)
                flush_here()

                # 用递归 process_block 处理 then/else，支持嵌套控制流
                then_out: list = []
                then_s = make_sub()
                then_s.enter_scope()
                process_block(ie.then_start, ie.then_end, then_s, then_out, ind + "    ")
                then_s.exit_scope()

                else_out: list = []
                else_s = None
                if ie.has_else:
                    else_s = make_sub()
                    else_s.enter_scope()
                    process_block(ie.else_start, ie.else_end, else_s, else_out, ind + "    ")
                    else_s.exit_scope()

                outer_len  = len(cur_sim.stack)
                then_extra = len(then_s.stack) - outer_len
                else_extra = (len(else_s.stack) - outer_len) if else_s else 0

                if (ie.has_else and then_extra == 1 and else_extra == 1
                        and not then_s.stmts and not else_s.stmts
                        and not then_out and not else_out):
                    # 三元：两个分支各留一个值在栈上，无语句无嵌套输出
                    tv = render_expr(then_s.stack[-1][0])
                    ev = render_expr(else_s.stack[-1][0])
                    # `this` 是 &Self（let this = self;），不能直接用于值位置，需要 Clone::clone
                    if tv == 'this':
                        tv = 'Clone::clone(this)'
                    if ev == 'this':
                        ev = 'Clone::clone(this)'
                    ty  = then_s.stack[-1][1]
                    ety = else_s.stack[-1][1]
                    ty_str  = render_type(ty)
                    ety_str = render_type(ety)
                    # 类型不一致时强转 else 侧以匹配 then 侧
                    if ty_str != ety_str:
                        _int_types = {'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64'}
                        _prim_types = _int_types | {'bool', 'f32', 'f64'}
                        _null_exprs = {'Object::default()', 'Clone::clone(&Object::default())'}
                        if ty_str == 'bool' and ety_str in _int_types:
                            ev = f"({ev} != 0)"
                        elif ety_str == 'bool' and ty_str in _int_types:
                            tv = f"({tv} != 0)"
                            ty = ety; ty_str = ety_str
                        elif ev in _null_exprs and ty_str not in _prim_types:
                            ev = 'Default::default()'
                        elif tv in _null_exprs and ety_str not in _prim_types:
                            # then 臂是 null：null 转 Default::default()，结果类型取 else 臂的具体类型
                            tv = 'Default::default()'
                            ty = ety; ty_str = ety_str
                        elif ety_str == 'Object' and ty_str not in _prim_types and ty_str != 'Object' and ty_str not in _class_tparams:
                            ev = f"({ev}).downcast::<{ty_str}>()"
                        elif ty_str == 'Object' and ety_str not in _prim_types and ety_str != 'Object' and ety_str not in _class_tparams:
                            ev = f"Object::from_any(Clone::clone(&{ev}))"
                        elif ty_str in _prim_types or ety_str in _prim_types:
                            ev = f"({ev} as {ty_str})"
                        elif (ty_str.split('<')[0] == ety_str.split('<')[0]
                                and '<' in ty_str and '<' in ety_str
                                and 'Object' in ty_str
                                and any(t in ety_str for t in _class_tparams)):
                            # then 侧含 Object（如 ReferenceQueue<Object>），else 侧含类型变量（如 ReferenceQueue<T>）
                            # → then 侧用 Default::default()，合并类型取 else 侧（消除 E0308）
                            tv = 'Default::default()'
                            ty = ety; ty_str = ety_str
                        elif (ty_str.split('<')[0] == ety_str.split('<')[0]
                                and '<' in ty_str and '<' in ety_str
                                and 'Object' in ety_str and 'Object' not in ty_str):
                            # then 侧具体泛型（JArray<Class>），else 侧擦除形态（JArray<Object>）
                            # → then 侧用 Default::default()，合并类型取 else 侧
                            tv = 'Default::default()'
                            ty = ety; ty_str = ety_str
                        # else: both are non-primitive structs, leave as-is and hope types match
                    # fall_cond 静态 false/true：跳过死代码臂
                    if fall_cond == 'false':
                        cur_sim.push(RawExpr(f"({ev})"), ety)
                    elif fall_cond == 'true':
                        cur_sim.push(RawExpr(f"({tv})"), ty)
                    else:
                        cur_sim.push(RawExpr(f"(if {fall_cond} {{ {tv} }} else {{ {ev} }})"), ty)
                elif ie.has_else and then_extra == 1 and else_extra == 1:
                    # 两分支均留一个值在栈上且有语句：声明合并变量，分支内赋值后推入栈
                    ty = then_s.stack[-1][1]
                    ety = else_s.stack[-1][1]
                    ty_str = render_type(ty)
                    ety_str = render_type(ety)
                    then_val = render_expr(then_s.stack[-1][0])
                    else_val = render_expr(else_s.stack[-1][0])
                    # `this` 是 &Self，不能直接用于值位置，需要 Clone::clone
                    if then_val == 'this':
                        then_val = 'Clone::clone(this)'
                    if else_val == 'this':
                        else_val = 'Clone::clone(this)'
                    # 类型不一致时做强转（与三元表达式分支保持一致）
                    if ty_str != ety_str:
                        _int_types = {'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64'}
                        _prim_types = _int_types | {'bool', 'f32', 'f64'}
                        _null_exprs = {'Object::default()', 'Clone::clone(&Object::default())'}
                        if ty_str == 'bool' and ety_str in _int_types:
                            else_val = f"({else_val} != 0)"
                        elif ety_str == 'bool' and ty_str in _int_types:
                            then_val = f"({then_val} != 0)"
                            ty = ety; ty_str = ety_str
                        elif else_val in _null_exprs and ty_str not in _prim_types:
                            else_val = 'Default::default()'
                        elif then_val in _null_exprs and ety_str not in _prim_types:
                            # then 臂是 null：null 转 Default::default()，合并变量
                            # 类型取 else 臂的具体类型（与三元分支对称）
                            then_val = 'Default::default()'
                            ty = ety; ty_str = ety_str
                        elif ety_str == 'Object' and ty_str not in _prim_types and ty_str != 'Object' and ty_str not in _class_tparams:
                            else_val = f"({else_val}).downcast::<{ty_str}>()"
                        elif ty_str == 'Object' and ety_str not in _prim_types and ety_str != 'Object' and ety_str not in _class_tparams:
                            else_val = f"Object::from_any(Clone::clone(&{else_val}))"
                        elif ty_str in _prim_types or ety_str in _prim_types:
                            else_val = f"({else_val} as {ty_str})"
                        elif (ty_str.split('<')[0] == ety_str.split('<')[0]
                                and '<' in ty_str and '<' in ety_str
                                and 'Object' in ty_str
                                and any(t in ety_str for t in _class_tparams)):
                            # then 侧含 Object，else 侧含类型变量 → then 侧 Default::default()
                            then_val = 'Default::default()'
                            ty = ety; ty_str = ety_str
                        elif (ty_str.split('<')[0] == ety_str.split('<')[0]
                                and '<' in ty_str and '<' in ety_str
                                and 'Object' in ety_str and 'Object' not in ty_str):
                            # then 侧具体泛型（JArray<Class>），else 侧擦除形态（JArray<Object>）
                            # → then 侧 Default::default()，合并类型取 else 侧（Java 类型安全近似）
                            then_val = 'Default::default()'
                            ty = ety; ty_str = ety_str
                    # 同步子 sim 的计数器，防止合并变量名与嵌套 if/else 的合并变量名碰撞
                    cur_sim._ctr = max(cur_sim._ctr, then_s._ctr, else_s._ctr)
                    merge_v = cur_sim.fresh('_merged')
                    flush_here()
                    out.append(('', f"{ind}let mut {merge_v}: {ty_str};"))
                    if fall_cond == 'false':
                        # then 臂是死代码，直接渲染 else 臂
                        out.extend(else_out)
                        for s in else_s.stmts:
                            out.append((ind + "    ", s))
                        out.append((ind + "    ", RawStmt(f"{merge_v} = {else_val};")))
                        ty = ety; ty_str = ety_str
                    elif fall_cond == 'true':
                        # else 臂是死代码，直接渲染 then 臂
                        out.extend(then_out)
                        for s in then_s.stmts:
                            out.append((ind + "    ", s))
                        out.append((ind + "    ", RawStmt(f"{merge_v} = {then_val};")))
                    else:
                        out.append(('', f"{ind}if {fall_cond} {{"))
                        out.extend(then_out)
                        for s in then_s.stmts:
                            out.append((ind + "    ", s))
                        out.append((ind + "    ", RawStmt(f"{merge_v} = {then_val};")))
                        out.append(('', f"{ind}}} else {{"))
                        out.extend(else_out)
                        for s in else_s.stmts:
                            out.append((ind + "    ", s))
                        out.append((ind + "    ", RawStmt(f"{merge_v} = {else_val};")))
                        out.append(('', f"{ind}}}"))
                    cur_sim.push(Var(merge_v), ty)
                    cur_sim.locals = then_s.locals
                    cur_sim._slot_decl_depth = then_s._slot_decl_depth
                else:
                    if fall_cond == 'false':
                        # then 臂是死代码，直接渲染 else 臂（若有）
                        if ie.has_else:
                            out.extend(else_out)
                            for s in else_s.stmts:
                                out.append((ind + "    ", s))
                    elif fall_cond == 'true' and ie.has_else:
                        # else 臂是死代码，直接渲染 then 臂
                        out.extend(then_out)
                        for s in then_s.stmts:
                            out.append((ind + "    ", s))
                    else:
                        out.append(('', f"{ind}if {fall_cond} {{"))
                        out.extend(then_out)
                        for s in then_s.stmts:
                            out.append((ind + "    ", s))
                        if ie.has_else:
                            out.append(('', f"{ind}}} else {{"))
                            out.extend(else_out)
                            for s in else_s.stmts:
                                out.append((ind + "    ", s))
                        out.append(('', f"{ind}}}"))
                    cur_sim.locals = then_s.locals
                    cur_sim._slot_decl_depth = then_s._slot_decl_depth

                i = ie.merge_idx
                continue

            # ── switch / tableswitch / lookupswitch ───────────────────
            if i in switch_map:
                sw = switch_map[i]
                # 弹出 switch 操作数
                key_e, key_t = cur_sim.pop()
                key_s = render_expr(key_e)
                flush_here()
                out.append(('', f"{ind}match {key_s} {{"))
                for value, cs, ce in sw.cases:
                    out.append(('', f"{ind}    {value} => {{"))
                    case_s = make_sub()
                    case_s.enter_scope()
                    process_block(cs, ce, case_s, out, ind + "        ")
                    case_s.exit_scope()
                    out.append(('', f"{ind}    }}"))
                # default case
                out.append(('', f"{ind}    _ => {{"))
                def_s = make_sub()
                def_s.enter_scope()
                process_block(sw.default_start, sw.default_end, def_s, out, ind + "        ")
                def_s.exit_scope()
                out.append(('', f"{ind}    }}"))
                out.append(('', f"{ind}}}"))
                i = sw.merge_idx
                continue

            # ── goto: forward-skip / continue / break ─────────────────
            if op == 'goto' and ins.operand:
                target_off = int(ins.operand)
                target_idx = off2idx.get(target_off)
                # 当前块内向前跳转（if-then 末尾跳到 merge）→ 直接跳过
                if target_idx is not None and i < target_idx <= end:
                    flush_here()
                    i = target_idx
                    continue
                # 在循环中：检测 continue / break
                enclosing = None
                for lp in _loops:
                    if lp.start_idx <= i <= lp.end_idx:
                        if enclosing is None or (lp.end_idx - lp.start_idx) < (enclosing.end_idx - enclosing.start_idx):
                            enclosing = lp
                if enclosing is not None:
                    loop_start_off = instrs[enclosing.start_idx].offset
                    if target_off == loop_start_off:
                        flush_here()
                        out.append(('', f"{ind}continue;"))
                        i += 1
                        continue
                    if enclosing.exit_offset is not None and target_off >= enclosing.exit_offset:
                        flush_here()
                        out.append(('', f"{ind}break;"))
                        i += 1
                        continue
                # 其他 goto（向前超出块范围等）：忽略
                i += 1
                continue

            # ── 普通指令 ──────────────────────────────────────────────
            cur_sim.current_offset = ins.offset
            sim_instr(ins, cur_sim, method.class_name, registry=registry)
            flush_here()
            i += 1

    process_block(0, len(instrs), sim, entries, "    ")

    # 栈下溢：控制流分析失败，整个方法退化为 panic!("stub: ...") 避免生成无法编译的残缺代码
    if sim.underflow_occurred:
        raise RuntimeError(f"stack underflow in {method.class_name}.{method.name}")

    # ── IR mutation 分析（渲染前）────────────────────────────────────
    ir_stmts = [item for _, item in entries if not isinstance(item, str)]
    _analyze_mutation(ir_stmts)
    _hoist_loop_vars(entries, predeclared)
    # _hoist_if_vars 每次只提升一层，循环直到收敛（处理多层嵌套 if-else）
    for _ in range(8):
        if not _hoist_if_vars(entries, predeclared):
            break
    _promote_undeclared_assigns(entries, predeclared)

    # ── 渲染 entries → lines ─────────────────────────────────────────
    lines: list[str] = []
    for indent, item in entries:
        if isinstance(item, str):
            lines.append(item)
        else:
            lines.append(indent + render_stmt(item).lstrip())

    # ── 构造器末尾返回 Ok(this) ────────────────────────────────────
    if is_ctor:
        while lines and lines[-1].strip() in ('return;', 'return Ok(());', 'return Ok(this);'):
            lines.pop()
        lines.append("    Ok(this)")

    # ── 其他方法的后处理 ──────────────────────────────────────────
    if not is_ctor:
        if not is_static:
            lines = _normalize_this_clone(lines)
        if rust_ret == 'bool':
            lines = _fix_bool_returns(lines)
        lines = _remove_trailing_return_ok(lines)
        _always_returns = function_always_returns(instrs)
        lines = _add_ok_return(lines, rust_ret, _always_returns)

    body = '\n'.join(lines)
    # 有重载时在方法前加注释，标注原始 Java 签名
    prefix = f"// java: {method.name}{method.descriptor}\n" if _overloaded else ""
    return f"{prefix}{sig} {{\n{body}\n}}"
