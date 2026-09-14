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

import re
from .types import ParsedMethod, ClassInfo
from .type_map import jvm_to_rust, sig_type, rust_default, mangle_name, short_cls, get_ergonomic_jvm_rename
from .constants import safe_ident
from .stack import StackSim
from .cfg import (
    find_loops, find_boolean_conditions, find_if_guards, find_if_else,
    find_switches,
    cmp_op, neg_cmp_op, _TWO_OP_BRANCH_OPS,
)
from .instr import sim_instr
from .render import render_stmt, render_expr, render_type
from .rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt,
)
from .stack import BOOL


def _coerce_icmp_operand(expr_str: str, ty_node) -> str:
    """为 if_icmpX 比较的操作数做类型强制转换：u16/i8/i16 → i32"""
    ty = render_type(ty_node)
    if ty in ('u16', 'i8', 'i16'):
        return f"({expr_str} as i32)"
    return expr_str

_PRIMITIVE_TYPES = {'i32', 'i64', 'f32', 'f64', 'bool', 'usize', '()'}

def _str_to_rs_type(s: str) -> RsType:
    """将 jvm_to_rust 返回的字符串转换为 RsType 节点。"""
    if s in _PRIMITIVE_TYPES:
        return RsPrimitive(s)
    return RsNamed(s)


def _analyze_mutation(stmts):
    """扫描 AssignStmt 目标，将对应的 LetStmt.mutable 设为 True。

    递归处理 IfStmt / LoopStmt 内部语句。
    """
    assigned: set[str] = set()

    def collect(ss):
        for stmt in ss:
            if isinstance(stmt, AssignStmt) and isinstance(stmt.target, Var):
                assigned.add(stmt.target.name)
            if isinstance(stmt, IfStmt):
                collect(stmt.then)
                collect(stmt.else_)
            if isinstance(stmt, LoopStmt):
                collect(stmt.body)

    collect(stmts)

    def mark(ss):
        for stmt in ss:
            if isinstance(stmt, LetStmt) and stmt.name in assigned:
                stmt.mutable = True
            if isinstance(stmt, IfStmt):
                mark(stmt.then)
                mark(stmt.else_)
            if isinstance(stmt, LoopStmt):
                mark(stmt.body)

    mark(stmts)


def _hoist_loop_vars(entries: list, predeclared: set[str]):
    """将在 loop{} 内 let-声明但在 loop 外被读取的变量提升到 loop 前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 loop 内首次 let-声明，
    但在 loop 退出后被读取，Rust 报 E0425。
    修复：在 loop 前插入 let mut NAME = Default::default();，loop 内改为赋值。
    """
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置
    nesting = 0
    declared_at: dict[str, tuple[int, int]] = {}  # name → (index, nesting_depth)
    loop_entry_indices: list[int] = []  # loop { 的索引

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            if item.rstrip().endswith('loop {'):
                loop_entry_indices.append(k)
            delta = item.count('{') - item.count('}')
            nesting += delta
        elif isinstance(item, LetStmt):
            if nesting > 0 and item.name not in declared_at and item.name not in predeclared:
                declared_at[item.name] = (k, nesting)

    if not declared_at or not loop_entry_indices:
        return

    # Pass 2: 将 entries 渲染为字符串，检测哪些 loop 内变量在 loop 外被引用
    rendered: list[str] = []
    for _, item in entries:
        if isinstance(item, str):
            rendered.append(item)
        else:
            try:
                rendered.append(render_stmt(item))
            except Exception:
                rendered.append('')

    # 追踪每个条目的嵌套深度
    entry_nesting: list[int] = []
    cur = 0
    for text in rendered:
        entry_nesting.append(cur)
        cur += text.count('{') - text.count('}')

    vars_to_hoist: set[str] = set()
    for name, (decl_k, decl_nesting) in declared_at.items():
        # 找到该变量作用域关闭的第一个索引（nesting 降回 decl_nesting - 1）
        scope_close = None
        for k2 in range(decl_k + 1, len(entries)):
            if entry_nesting[k2] < decl_nesting:
                scope_close = k2
                break
        if scope_close is None:
            continue
        # 检查 scope_close 之后是否有该变量名的引用
        word = re.compile(r'\b' + re.escape(name) + r'\b')
        for k2 in range(scope_close, len(rendered)):
            if word.search(rendered[k2]):
                vars_to_hoist.add(name)
                break

    if not vars_to_hoist:
        return

    # Pass 3: 对需要提升的变量进行修改
    # 先收集所有插入操作（在 loop 前插入 let mut NAME = Default::default();）
    # 用倒序插入，避免索引偏移
    insertions: list[tuple[int, tuple]] = []  # (index, entry) to insert BEFORE
    for name in vars_to_hoist:
        decl_k, decl_nesting = declared_at[name]
        # 找到包含此声明的最近的 loop { 索引
        loop_k = None
        for lk in reversed(loop_entry_indices):
            if lk < decl_k:
                loop_k = lk
                break
        if loop_k is None:
            continue
        # 获取 loop 行的 indent 作为插入位置的 indent
        loop_indent = entries[loop_k][0]
        insertions.append((loop_k, (loop_indent, LetStmt(name, None, True, RawExpr('unsafe { std::mem::MaybeUninit::uninit().assume_init() }')))))
        # 将 loop 内的 LetStmt 改为 AssignStmt
        inner_indent, inner_item = entries[decl_k]
        if isinstance(inner_item, LetStmt):
            entries[decl_k] = (inner_indent, AssignStmt(Var(inner_item.name), inner_item.value))

    # 倒序插入，避免索引偏移（同一 loop 有多个变量时均插入到 loop 前）
    for ins_k, ins_entry in sorted(insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)


def _promote_undeclared_assigns(entries: list, predeclared: set[str]):
    """将在当前词法作用域中无对应 LetStmt 的 AssignStmt 提升为 LetStmt(mutable=True)。

    JVM 局部变量槽可在不同词法作用域中复用同一名字：例如变量 j 在 loop{} 内声明，
    loop 结束后又被赋值 j = high，此时 j 已出作用域，Rust 报 E0425。
    修复：追踪各变量声明时的嵌套深度，退出该层作用域后将同名 AssignStmt 提升为 LetStmt。
    """
    declared: dict[str, int] = {}  # name → 声明时的嵌套深度
    nesting = 0

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            delta = item.count('{') - item.count('}')
            if delta < 0:
                nesting += delta
                # 移除在已退出作用域层声明的变量
                declared = {n: d for n, d in declared.items() if d <= nesting}
            else:
                nesting += delta
            continue

        if isinstance(item, LetStmt):
            declared[item.name] = nesting
        elif isinstance(item, AssignStmt) and isinstance(item.target, Var):
            name = item.target.name
            if name not in predeclared and name not in declared:
                # 变量在当前词法作用域不可见 → 提升为 LetStmt（类型由 Rust 推断）
                entries[k] = (indent, LetStmt(name, None, True, item.value))
                declared[name] = nesting


def _remove_trailing_return_ok(lines: list[str]) -> list[str]:
    """删除函数末尾多余的 return Ok(()); 语句（void 函数）。"""
    result = list(lines)
    for i in reversed(range(len(result))):
        stripped = result[i].strip()
        if stripped:
            if stripped in ('return;', 'return Ok(());'):
                result.pop(i)
            break
    return result


def _fix_bool_returns(lines: list[str]) -> list[str]:
    """将 bool 返回方法中的 Ok(1i32)/Ok(0i32) 转换为 Ok(true)/Ok(false)。
    JVM 中 boolean 用 int 0/1 表示，直接 ireturn 时会生成 Ok(1i32)。"""
    result = []
    for line in lines:
        line = line.replace('Ok(1i32)', 'Ok(true)')
        line = line.replace('Ok(0i32)', 'Ok(false)')
        # 也修正裸 return Ok(1i32)
        result.append(line)
    return result


def _add_ok_return(lines: list[str], rust_ret: str) -> list[str]:
    """在方法末尾添加正确的 Ok(?) 返回表达式。"""
    result = list(lines)
    if rust_ret == '()':
        # void 方法：末尾加 Ok(())
        # 先检查末尾是否已有 Ok(())
        for i in reversed(range(len(result))):
            stripped = result[i].strip()
            if stripped:
                if stripped not in ('Ok(())', 'return Ok(());'):
                    result.append('    Ok(())')
                break
        else:
            result.append('    Ok(())')
    else:
        # 有返回值：将末尾的 return Ok(e); 转换为 Ok(e)（尾表达式形式）
        for i in reversed(range(len(result))):
            stripped = result[i].strip()
            if stripped:
                m = re.match(r'(\s*)return Ok\((.+)\);', result[i])
                if m:
                    result[i] = f"{m.group(1)}Ok({m.group(2)})"
                elif not stripped.startswith('Ok('):
                    # 如果最后一行不是 Ok(...) 也不是 return Ok(...)，
                    # 可能是个普通的 return e; → 已经被 instr 转换了
                    pass
                break
    return result


TWO_OP_CMP = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt',
    'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
})


def _indent(block: str, n: int = 4) -> str:
    pad = ' ' * n
    return '\n'.join(pad + ln if ln.strip() else '' for ln in block.split('\n'))


def gen_method_body(
    method: ParsedMethod,
    class_info: ClassInfo,
    registry: dict | None = None,
    class_type_params: list[str] | None = None,
    overloaded_names: set[str] | None = None,
    rust_name: str | None = None,
) -> str:
    from .sig_parser import parse_method_param_types
    _class_tparams = class_type_params or []

    # 如果方法有泛型签名且类有类型参数，用签名推断参数/返回类型
    if method.generic_signature and _class_tparams:
        sig_param_types, sig_ret_type = parse_method_param_types(
            method.generic_signature, _class_tparams
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

    # 参数类型：如果泛型签名提供了类型变量，优先使用
    if sig_param_types and len(sig_param_types) == len(param_types):
        rust_param_types = [
            sp if sp in _class_tparams else jp
            for sp, jp in zip(sig_param_types, [jvm_to_rust(t, registry) for t in param_types])
        ]
    else:
        rust_param_types = [jvm_to_rust(t, registry) for t in param_types]

    # 返回类型：如果泛型签名返回值是类型变量，优先使用
    if sig_ret_type and sig_ret_type in _class_tparams:
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
        # Java clone() 与 Rust Clone::clone() 同名冲突：重命名为 jvm_clone
        if rust_fn_name == 'clone':
            rust_fn_name = 'jvm_clone'
        # ergonomic @jvm_rename 指令：定义侧同步重命名（与调用侧的 _mangle_if_overloaded 保持一致）
        erg = get_ergonomic_jvm_rename(method.class_name, method.name)
        if erg is not None:
            rust_fn_name = erg

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
    if method.local_types:
        from .sig_parser import parse_field_type
        for _hint_slot, _hint_sig in method.local_types.items():
            rust_ty_name = parse_field_type(_hint_sig, _class_tparams)
            if rust_ty_name and rust_ty_name != 'Object':
                slot_hint_types[_hint_slot] = RsNamed(rust_ty_name)

    sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names,
                   slot_hint_types=slot_hint_types, return_type=rust_ret,
                   is_constructor=is_ctor, class_type_params=_class_tparams)
    # 记录参数和 this 的名字（在函数签名中已声明，无需提升）
    predeclared: set[str] = {name for name, _, _ in sim.locals.values()}

    # entries: list of (indent: str, item: RsStmt | str)
    # - str 条目是已缩进的原始代码行（loop {, if cond { break; }, } 等）
    # - RsStmt 条目是 IR 节点，等待 mutation 分析后再渲染
    entries: list = []

    # ── 构造器：创建 this ───────────────────────────────────────────
    if is_ctor:
        from .sig_parser import parse_class_type_params
        # T76: struct 使用 _super 嵌套，不再展平继承字段
        # 只初始化本类直接字段，父类通过 _super: Default::default() 初始化
        inst_fields = [f for f in (class_info.fields if class_info else []) if not f.is_static]
        _ctor_has_super = bool(
            class_info and class_info.super_class
            and class_info.super_class != 'java/lang/Object'
        )
        class_tparams = parse_class_type_params(class_info.generic_signature) if (class_info and class_info.generic_signature) else []
        _safe_fname = safe_ident
        from .sig_parser import parse_field_type as _pft
        _class_tparams_set = set(class_tparams)

        def _field_init(f) -> str:
            # 若字段的泛型签名解析到类型参数（如 T、K、V），不能用 Default::default()
            # 因为类型参数 T 不一定实现 Default；用 JField::new_uninit() 代替
            gen_ty = _pft(f.generic_signature, class_tparams) if (f.generic_signature and class_tparams) else ''
            if gen_ty and gen_ty in _class_tparams_set:
                return f"{_safe_fname(f.name)}: JField::new_uninit()"
            return f"{_safe_fname(f.name)}: JField::new({rust_default(jvm_to_rust(f.descriptor))})"

        # 按照 emitter.py 的 struct 字段顺序：_super → 本类字段 → _phantom
        # 特例：无 _super、无字段、有泛型参数 → tuple struct，用 Self(PhantomData) 而非具名字段
        _is_tuple_struct = (not _ctor_has_super and not inst_fields and bool(class_tparams))
        if _is_tuple_struct:
            struct_init = "Self(std::marker::PhantomData)"
        else:
            parts_init = []
            if _ctor_has_super:
                parts_init.append("_super: Default::default()")
            parts_init.extend(_field_init(f) for f in inst_fields)
            if class_tparams:
                parts_init.append("_phantom: std::marker::PhantomData")
            if parts_init:
                # 用 ..Default::default() 兜底额外字段（如 native_impls @field 注入的字段）
                struct_init = f"Self {{ {', '.join(parts_init)}, ..Default::default() }}"
            else:
                struct_init = "Self::default()"
        entries.append(('', f"    let this = {struct_init};"))
        sim.locals[0] = ('this', RsNamed(short_cls(method.class_name)), False)

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
                slot_hint_types=slot_hint_types, return_type=rust_ret,
                is_constructor=is_ctor, class_type_params=_class_tparams,
            )
            s.locals = dict(cur_sim.locals)
            s._slot_decl_depth = dict(cur_sim._slot_decl_depth)
            s.stack = list(cur_sim.stack)
            return s

        def pop_fall_cond(op: str) -> str:
            """弹出操作数，返回 fall-through 条件（跳转条件的否定）。"""
            if op in TWO_OP_CMP:
                b_e, b_t = cur_sim.pop(); a_e, a_t = cur_sim.pop()
                a_s = _coerce_icmp_operand(render_expr(a_e), a_t)
                b_s = _coerce_icmp_operand(render_expr(b_e), b_t)
                return neg_cmp_op(op, a_s, b_s)
            else:
                a_e, a_t = cur_sim.pop()
                a_s = render_expr(a_e)
                a_is_b = (str(a_t) == 'bool' or getattr(a_t, 'name', '') == 'bool')
                if a_is_b and op in ('ifeq', 'ifne'):
                    return f'!({a_s})' if op == 'ifne' else a_s
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
                    sim_instr(ins, cur_sim, method.class_name, registry=registry)
                    flush_here()
                    i += 1
                    continue

                out.append(('', f"{ind}loop {{"))

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
                        a_c = _coerce_icmp_operand(render_expr(a_e), a_t)
                        b_c = _coerce_icmp_operand(render_expr(b_e), b_t)
                        cond_str = cmp_op(ci.opcode, a_c, b_c)
                    else:
                        a_e, a_t = cond_s.pop()
                        a_s2 = render_expr(a_e)
                        a_is_b2 = (str(a_t) == 'bool' or getattr(a_t, 'name', '') == 'bool')
                        if a_is_b2 and ci.opcode in ('ifeq', 'ifne'):
                            cond_str = f'!({a_s2})' if ci.opcode == 'ifeq' else a_s2
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
                out.append(('', f"{ind}if {fall_cond} {{"))
                inner = make_sub()
                inner.enter_scope()
                for k in range(guard.body_start_idx, guard.continue_idx):
                    ki = instrs[k]
                    # goto → 循环出口（break）或循环起始（continue）
                    if ki.opcode == 'goto' and ki.operand:
                        goto_tgt = int(ki.operand)
                        enclosing = [lp for lp in _loops if lp.start_idx <= k <= lp.end_idx]
                        is_brk = any(
                            lp.exit_offset is not None and goto_tgt >= lp.exit_offset
                            for lp in enclosing
                        )
                        if is_brk:
                            inner.emit(RawStmt('break;'))
                            continue
                        is_cont = any(
                            instrs[lp.start_idx].offset == goto_tgt
                            for lp in enclosing
                        )
                        if is_cont:
                            inner.emit(RawStmt('continue;'))
                            continue
                    sim_instr(ki, inner, method.class_name, registry=registry)
                inner.exit_scope()
                for s in inner.stmts:
                    out.append((ind + "    ", s))
                out.append(('', f"{ind}}}"))
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
                    ty  = then_s.stack[-1][1]
                    ety = else_s.stack[-1][1]
                    ty_str  = render_type(ty)
                    ety_str = render_type(ety)
                    # 类型不一致时强转 else 侧以匹配 then 侧
                    if ty_str != ety_str:
                        _int_types = {'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64'}
                        if ty_str == 'bool' and ety_str in _int_types:
                            ev = f"({ev} != 0)"
                        elif ety_str == 'bool' and ty_str in _int_types:
                            tv = f"({tv} != 0)"
                            ty = ety; ty_str = ety_str
                        else:
                            ev = f"({ev} as {ty_str})"
                    cur_sim.push(RawExpr(f"(if {fall_cond} {{ {tv} }} else {{ {ev} }})"), ty)
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
            sim_instr(ins, cur_sim, method.class_name, registry=registry)
            flush_here()
            i += 1

    process_block(0, len(instrs), sim, entries, "    ")

    # ── IR mutation 分析（渲染前）────────────────────────────────────
    ir_stmts = [item for _, item in entries if not isinstance(item, str)]
    _analyze_mutation(ir_stmts)
    _hoist_loop_vars(entries, predeclared)
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
        if rust_ret == 'bool':
            lines = _fix_bool_returns(lines)
        lines = _remove_trailing_return_ok(lines)
        lines = _add_ok_return(lines, rust_ret)

    body = '\n'.join(lines)
    # 有重载时在方法前加注释，标注原始 Java 签名
    prefix = f"// java: {method.name}{method.descriptor}\n" if _overloaded else ""
    return f"{prefix}{sig} {{\n{body}\n}}"
