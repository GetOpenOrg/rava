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
from ..sig_parse import parse_field_type, parse_method_param_types
from ..sig_types import emitted_method_sig_types
from ..type_map import (
    jvm_to_rust, sig_type, rust_default, mangle_name, short_cls,
    parse_class_type_params,
)
from ..constants import safe_ident, PRIMITIVE_RUST_TYPES as _PRIM_TYPES
from ..stack import StackSim
from ..cfg import (
    CfgError, analyze, build_structure, simplify, build_dispatch, function_always_returns,
    JumpLedger, CfgAuditError, STATS,
)
from ..cfg import structure as _st
from ..cfg.simplify import walk as _simplify_walk
from ..instr import sim_instr
from ..render import render_stmt, render_expr, render_type
from ..rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt,
)
from ..instr.hierarchy import _is_subtype
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
    for item in _simplify_walk(tree):
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
    # 与 descriptor 擦除一致，由 sig_type_string_valid 兜底。
    # 发射签名单一来源（K-6）：method_sig_types 结果逐位过滤（类型串无效 /
    # 首标识符是接口 → 回退描述符形态），与 vtable 擦除名单共用同一函数 ——
    # 擦除条目按 token 全等匹配发射签名，两侧必须逐字一致。
    # 构造器的隐式形参（外部实例 / 匿名类转发形参）类型由 method_sig_types →
    # constructor_sig_types 统一给出，定义侧与调用侧同源。
    rust_param_types, rust_ret = emitted_method_sig_types(
        class_info, method, _class_tparams, registry)

    instrs           = method.instrs
    param_types      = method.param_types

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

    def _sim_is_interface(short_name: str) -> bool:
        from ..instr.hierarchy import _is_interface as _is_iface
        return _is_iface(short_name, registry)

    def _sim_box_object(expr_s: str, ty_s: str) -> str:
        from ..instr.coerce import _coerce_to_object
        return _coerce_to_object(expr_s, ty_s, registry, _class_tparams, clone=False)

    def _sim_infer_type_args(actual_short: str, declared_sig: str):
        from ..instr.hierarchy import _rust_type_to_binary
        from ..sig_types import infer_type_args_from_declared
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
                   is_interface=_sim_is_interface,
                   return_type=rust_ret, is_constructor=is_ctor, class_type_params=_class_tparams,
                   in_vtable_body=in_vtable_body, box_object=_sim_box_object,
                   infer_type_args=_sim_infer_type_args)
    _bounds_ci = registry.get(method.class_name) if registry else None
    if _bounds_ci is not None and _class_tparams:
        from ..type_args import class_type_param_bounds
        sim.type_var_bounds = {tv: b[0] for tv, b in class_type_param_bounds(_bounds_ci, registry).items()}
    # 记录参数和 this 的名字（在函数签名中已声明，无需提升）
    predeclared: set[str] = {name for name, _, _ in sim.locals.values()}

    # entries: list of (indent: str, item: RsStmt | str)
    # - str 条目是已缩进的原始代码行（loop {, if cond { break; }, } 等）
    # - RsStmt 条目是 IR 节点，等待 mutation 分析后再渲染
    entries: list = []

    # ── 构造器：双入口拆分（K-5，JVM 单一对象模型）──────────────────
    # `new`（最外层 new 的具体类构造入口）建立对象身份一次：Self::default()
    # 创建唯一 inner（__identity 单元 + any Rc），_init_not_null 清 null 标志，
    # 随即把 this 传入 `__init_on`。构造器体全部落在 `__init_on`：super(...) 发射为
    # `Parent::__init_on(<Parent as From<Self>>::from(Clone::clone(&this)), args)`
    # ——this 经父类视图（vtable 上转 + any 共享部件）传入，父类体内的 putfield 经
    # 访问器落在唯一身份上、this.m() 虚分派命中最终子类 override；this(...) 委托
    # 同样传入同一 this。body 模拟阶段 this 仍以局部变量身份参与（sim.locals[0]）。
    if is_ctor:
        _this_rust = short_cls(method.class_name)
        if _this_rust and _class_tparams:
            _this_rust = f"{_this_rust}<{', '.join(_class_tparams)}>"
        sim.locals[0] = ('this', RsNamed(_this_rust), False)

    elif not is_static:
        # 实例方法：绑定 this = self，供字节码（aload_0 + getfield/putfield）使用
        entries.append(('', "    let this = self;"))

    # ── ACC_SYNCHRONIZED 前导（S-20 真实化，0x0020 == emitter.attrs._ACC_SYNCHRONIZED）──
    # 同步方法（JLS §8.4.3.6 / JVMS §2.11.10）字节码无 monitorenter/exit，由
    # 方法标志承载：进入时获取监视器，任何完成路径（return / 异常 / panic 展开）
    # 释放——RAII 守卫。实例方法锁 this，静态方法锁声明类的 Class 对象；
    # 可重入，单线程语义不变。构造器 / <clinit> 不可同步（JLS），天然缺席。
    if not is_ctor and method.name != '<clinit>' and (method.access_flags & 0x0020):
        if is_static:
            entries.append(('', f'    let __sync_guard = '
                                f'MonitorGuard::acquire(&class_monitor("{method.class_name}"))?;'))
        else:
            entries.append(('', '    let __sync_guard = '
                                'MonitorGuard::acquire(&Object::from(Clone::clone(this)))?;'))

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
    # LocalVariableTable 里出现过的名字：其余变量是 javac 合成的无名槽
    from ..stack import _safe_name as _safe_local_name
    _lvt_names = frozenset(_safe_local_name(_d[2]) for _ds in sim._slot_decls.values() for _d in _ds)
    for _ in range(64):
        if not _hoist_if_vars(entries, predeclared, sim._box_object, _lvt_names, registry):
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

    if is_ctor:
        # K-5 双入口返回：`new` 建身份 + 转发；`__init_on` 持有构造器体（this 为首参，
        # owned Self —— super()/this() 委托在其上传入同一身份）。名字与 invoke.py
        # 调用侧同源（new / __init_on + 同一重载后缀）。
        _fwd_params: list[str] = []
        _fwd_args: list[str] = []
        for _p in params:
            _pn, _pt = _p.split(':', 1)
            _pn = _pn.removeprefix('mut ').strip()
            _fwd_params.append(f"{_pn}: {_pt.strip()}")
            _fwd_args.append(_pn)
        _init_on_name = ('__init_on' + rust_fn_name[3:]
                         if rust_fn_name.startswith('new')
                         else f'__init_on_{rust_fn_name}')
        _new_sig = f"pub fn {rust_fn_name}({', '.join(_fwd_params)}) -> Result<Self>"
        _init_on_sig = (f"pub fn {_init_on_name}(mut this: Self"
                        f"{', ' if params else ''}{', '.join(params)}) -> Result<Self>")
        _fwd_call = (f"Self::{_init_on_name}(this"
                     f"{', ' + ', '.join(_fwd_args) if _fwd_args else ''})")
        new_fn = (
            f"{prefix}{_new_sig} {{\n"
            f"    let mut this = Self::default();\n"
            f"    this._init_not_null();\n"
            f"    {_fwd_call}\n"
            f"}}"
        )
        init_on_fn = f"#[doc(hidden)]\n{_init_on_sig} {{\n{body}\n}}"
        return new_fn + "\n\n" + init_on_fn

    return f"{prefix}{sig} {{\n{body}\n}}"
