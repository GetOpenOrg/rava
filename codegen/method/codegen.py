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
    parse_method_param_types, method_sig_types, parse_field_type, parse_class_type_params,
)
from ..constants import safe_ident, PRIMITIVE_RUST_TYPES as _PRIM_TYPES
from ..stack import StackSim
from ..cfg import (
    CfgError, analyze, build_structure, simplify, build_dispatch, function_always_returns,
    JumpLedger, CfgAuditError, STATS,
)
from ..cfg import structure as _st
from ..instr import sim_instr
from ..render import render_stmt, render_expr, render_type
from ..rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt,
)
from ..instr.coerce import _is_subtype
from .blocks import simulate_blocks
from .emit import emit_tree
from .vars import _str_to_rs_type, _analyze_mutation, _hoist_loop_vars, _hoist_if_vars, _promote_undeclared_assigns
from .postprocess import _normalize_this_clone, _erase_boxed_ctor_type_args, _remove_trailing_return_ok, _fix_bool_returns, _add_ok_return, _indent


def _structured_entries(method, sim, registry, class_tparams, ledger) -> list:
    """方法指令 → entries。所有跳转指令的消费情况记入 ledger 并在此校验。"""
    if not method.instrs:
        return []
    result = simulate_blocks(method, sim, registry, class_tparams, ledger)
    nodes = result.nodes
    if result.dispatch:
        tree = build_dispatch(nodes, result.entry, list(nodes))
        entries = [('', f"    {line}") for line in result.top_decls]
    else:
        flow = analyze(result.entry, {i: n.successors() for i, n in nodes.items()})
        if not flow.reducible:
            raise CfgError("归约后的 CFG 不可归约")
        tree = simplify(build_structure(nodes, flow))
        _verify_tree(tree, nodes, flow, ledger)
        entries = []
    entries.extend(emit_tree(tree, nodes))
    ledger.verify()
    if ledger.method_id not in STATS._seen:
        live = [n for n in nodes.values() if not n.removed]
        # 异常表声明的处理器 vs 实际挂到 Try 节点（因而进入结构化树）的处理器
        declared = {h for _s, _e, h, _ct in (method.exception_table or [])}
        translated = {nodes[h].start_pc for n in live if n.kind == 'try' for h in n.handlers}
        STATS.record_try_regions(ledger.method_id, sum(1 for n in live if n.kind == 'try'),
                                 len(declared - translated))
    STATS.record(ledger, result.dispatch)
    return entries


def _verify_tree(tree: list, nodes: dict, flow, ledger) -> None:
    """结构树自检：每个活块恰好出现一次，每个条件 / switch 终结都有对应的 if / while / match。
    通过后，块终结所承载的跳转指令记为 structured。"""
    code_blocks: list[int] = []
    branch_origins: set[int] = set()
    for item in _st.walk(tree):
        if isinstance(item, _st.Code):
            code_blocks.append(item.block)
        elif isinstance(item, (_st.If, _st.Switch, _st.Try)):
            branch_origins.add(item.origin)
        elif isinstance(item, _st.Loop) and item.cond_origin is not None:
            branch_origins.add(item.cond_origin)
    if sorted(code_blocks) != sorted(flow.rpo):
        raise CfgAuditError(f"{ledger.method_id}: 结构树与活块集合不一致 "
                            f"tree={sorted(code_blocks)} live={sorted(flow.rpo)}")
    for nid in flow.rpo:
        node = nodes[nid]
        if node.kind in ('cond', 'switch', 'try') and nid not in branch_origins:
            raise CfgAuditError(f"{ledger.method_id}: 块 pc={node.start_pc} 的分支未出现在结构树中 "
                                f"(jump pc={node.pcs})")
        for pc in node.pcs:
            if pc not in ledger.consumed:
                ledger.consume(pc, 'structured')


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
    sig_param_types, sig_ret_type = method_sig_types(class_info, method, _class_tparams, registry)

    instrs           = method.instrs
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

    # 构造器的隐式形参（外部实例 / 匿名类转发形参）类型由 method_sig_types →
    # constructor_sig_types 统一给出，定义侧与调用侧同源。

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
    # 局部变量声明表：slot → [(start_pc, end_pc, name, RsType|None, from_signature)]
    # 类型优先取 LocalVariableTypeTable 泛型签名（from_signature=True），
    # 否则取 LocalVariableTable 描述符；Object / 接口别名不构成有效的精确类型（None）。
    slot_decls: dict[int, list] = {}
    for _lv_slot, _lv_start, _lv_len, _lv_name, _lv_desc, _lv_sig in (method.local_vars or []):
        _lv_ty = None
        _lv_from_sig = False
        if _lv_sig:
            _sig_ty_name = parse_field_type(_lv_sig, _class_tparams, registry)
            if _sig_ty_name and _sig_ty_name != 'Object':
                _lv_ty = RsNamed(_sig_ty_name)
                _lv_from_sig = True
        if _lv_ty is None and _lv_desc:
            _desc_ty_name = jvm_to_rust(_lv_desc, registry)
            if _desc_ty_name and _desc_ty_name != 'Object' and '::' not in _desc_ty_name:
                _lv_ty = _str_to_rs_type(_desc_ty_name)
        slot_decls.setdefault(_lv_slot, []).append(
            (_lv_start, _lv_start + _lv_len, _lv_name, _lv_ty, _lv_from_sig, _lv_sig or ''))
    for _lv_entries in slot_decls.values():
        _lv_entries.sort(key=lambda _e: _e[0])

    def _sim_is_subtype(child: str, parent: str) -> bool:
        return _is_subtype(child, parent, registry)

    def _sim_box_object(expr_s: str, ty_s: str) -> str:
        from ..instr.coerce import _coerce_to_object
        return _coerce_to_object(expr_s, ty_s, registry, _class_tparams, clone=False)

    def _sim_infer_type_args(actual_short: str, declared_sig: str):
        from ..instr.coerce import _rust_type_to_binary
        from ..type_map import infer_type_args_from_declared
        _actual_bin = _rust_type_to_binary(actual_short, registry)
        if not _actual_bin:
            return None
        _solved = infer_type_args_from_declared(_actual_bin, declared_sig, list(_class_tparams or []), registry)
        if _solved is None:
            return None
        # 声明实参引用了生成范围之外的类（调用链未触及）：该类型的值在生成代码中不可能以
        # 静态类型出现，元素只会以 Object 形态流动 → 该实参按擦除取 Object
        import re as _re_scope
        def _in_scope(_arg: str) -> bool:
            return all(_n in (_class_tparams or ()) or _n in _PRIM_TYPES
                       or _n in ('Object', 'JArray') or _rust_type_to_binary(_n, registry)
                       for _n in _re_scope.findall(r'[A-Za-z_][A-Za-z0-9_]*', _arg))
        return [_a if _in_scope(_a) else 'Object' for _a in _solved]

    sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names,
                   slot_decls=slot_decls, is_subtype=_sim_is_subtype,
                   return_type=rust_ret, is_constructor=is_ctor, class_type_params=_class_tparams,
                   in_vtable_body=in_vtable_body, box_object=_sim_box_object,
                   infer_type_args=_sim_infer_type_args)
    _bounds_ci = registry.get(method.class_name) if registry else None
    if _bounds_ci is not None and _class_tparams:
        from ..type_map import class_type_param_bounds
        sim.type_var_bounds = {tv: b[0] for tv, b in class_type_param_bounds(_bounds_ci, registry).items()}
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

    # ── 控制流：逐块模拟 + 归约 → 结构化（或状态机兜底）→ entries ─────────
    ledger = JumpLedger(f"{method.class_name}.{method.name}:{method.descriptor}")
    entries.extend(_structured_entries(method, sim, registry, _class_tparams, ledger))

    # 栈下溢：控制流分析失败，整个方法退化为 panic!("stub: ...") 避免生成无法编译的残缺代码
    if sim.underflow_occurred:
        raise RuntimeError(f"stack underflow in {method.class_name}.{method.name}")

    # ── IR mutation 分析（渲染前）────────────────────────────────────
    ir_stmts = [item for _, item in entries if not isinstance(item, str)]
    _analyze_mutation(ir_stmts)
    _hoist_loop_vars(entries, predeclared)
    # _hoist_if_vars 每次只提升一层，循环直到收敛（处理多层嵌套 if-else）
    for _ in range(64):
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

    lines = _erase_boxed_ctor_type_args(lines)

    # ── 构造器末尾返回 Ok(this) ────────────────────────────────────
    if is_ctor:
        while lines and lines[-1].strip() in ('return;', 'return Ok(());', 'return Ok(this);'):
            lines.pop()
        lines.append("    Ok(this)")
        lines = _normalize_this_clone(lines, this_is_owned=True)

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
