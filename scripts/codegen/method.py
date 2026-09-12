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
from .type_map import jvm_to_rust, sig_type, rust_default, mangle_name
from .stack import StackSim
from .cfg import find_loops, find_boolean_conditions, cmp_op, neg_cmp_op
from .instr import sim_instr
from .render import render_stmt, render_expr
from .rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr,
)
from .stack import BOOL

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
    loop_map         = {lp.start_idx: lp for lp in find_loops(instrs)}
    bool_cond_map    = find_boolean_conditions(instrs)
    param_types      = method.param_types

    # 参数类型：如果泛型签名提供了类型变量，优先使用
    if sig_param_types and len(sig_param_types) == len(param_types):
        rust_param_types = [
            sp if sp in _class_tparams else jp
            for sp, jp in zip(sig_param_types, [jvm_to_rust(t) for t in param_types])
        ]
    else:
        rust_param_types = [jvm_to_rust(t) for t in param_types]

    # 返回类型：如果泛型签名返回值是类型变量，优先使用
    if sig_ret_type and sig_ret_type in _class_tparams:
        rust_ret = sig_ret_type
    else:
        rust_ret = jvm_to_rust(method.return_type)
    is_ctor          = method.is_constructor
    is_static        = method.is_static

    local_names = method.local_names or {}

    # 计算最终 Rust 方法名（有重载则加描述符后缀）
    _overloaded = overloaded_names is not None and method.name in overloaded_names
    if is_ctor:
        rust_fn_name = mangle_name('new', method.descriptor) if _overloaded else 'new'
    elif _overloaded:
        rust_fn_name = mangle_name(method.name, method.descriptor)
    else:
        rust_fn_name = method.name

    def _param_name(slot: int, fallback: str) -> str:
        from .stack import _safe_name
        return _safe_name(local_names.get(slot, fallback))

    # ── 函数签名 ──────────────────────────────────────────────────────
    if is_ctor:
        # 构造器：Result<Self>，参数从 slot 1 开始
        params = [
            f"{_param_name(k + 1, f'arg_{k}')}: {rt}"
            for k, rt in enumerate(rust_param_types)
        ]
        sig = f"pub fn {rust_fn_name}({', '.join(params)}) -> Result<Self>"

    elif method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        sig = "pub fn main() -> Result<()>"

    elif is_static:
        params = [
            f"{_param_name(k, f'arg_{k}')}: {sig_type(rt)}"
            for k, rt in enumerate(rust_param_types)
        ]
        sig = f"pub fn {rust_fn_name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> Result<{rust_ret}>"
        else:
            sig += " -> Result<()>"

    else:
        # 实例方法：&self，Result<T>
        params = ["&self"]
        params += [
            f"{_param_name(k + 1, f'arg_{k}')}: {rt}"
            for k, rt in enumerate(rust_param_types)
        ]
        sig = f"pub fn {rust_fn_name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> Result<{rust_ret}>"
        else:
            sig += " -> Result<()>"

    rust_param_type_nodes = [_str_to_rs_type(t) for t in rust_param_types]
    sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names)

    # entries: list of (indent: str, item: RsStmt | str)
    # - str 条目是已缩进的原始代码行（loop {, if cond { break; }, } 等）
    # - RsStmt 条目是 IR 节点，等待 mutation 分析后再渲染
    entries: list = []

    # ── 构造器：创建 this ───────────────────────────────────────────
    if is_ctor:
        from .sig_parser import parse_class_type_params
        inst_fields = [f for f in (class_info.fields if class_info else []) if not f.is_static]
        class_tparams = parse_class_type_params(class_info.generic_signature) if (class_info and class_info.generic_signature) else []
        def _safe_fname(n: str) -> str:
            n = n.replace('$', '_')
            _kw = frozenset({'in', 'type', 'enum', 'mod', 'use', 'fn', 'let',
                             'mut', 'ref', 'pub', 'self', 'static', 'struct'})
            return n + '_' if n in _kw else n
        if inst_fields and class_tparams:
            # 命名 struct，有实例字段且有泛型参数
            parts_init = [
                f"{_safe_fname(f.name)}: Field::new({rust_default(jvm_to_rust(f.descriptor))})"
                for f in inst_fields
            ]
            parts_init.append("_phantom: std::marker::PhantomData")
            struct_init = f"Self {{ {', '.join(parts_init)} }}"
        elif inst_fields:
            # 命名 struct，只有实例字段，无泛型参数
            parts_init = [
                f"{_safe_fname(f.name)}: Field::new({rust_default(jvm_to_rust(f.descriptor))})"
                for f in inst_fields
            ]
            struct_init = f"Self {{ {', '.join(parts_init)} }}"
        elif class_tparams:
            # 无实例字段但有泛型参数：tuple struct，用 Self(PhantomData)
            struct_init = "Self(std::marker::PhantomData)"
        else:
            struct_init = "Self {}"
        entries.append(('', f"    let this = {struct_init};"))
        sim.locals[0] = ('this', RsNamed(method.class_name), False)

    elif not is_static:
        # 实例方法：绑定 this = self，供字节码（aload_0 + getfield/putfield）使用
        entries.append(('', "    let this = self;"))

    def flush(s: StackSim):
        for stmt in s.stmts:
            entries.append(('    ', stmt))
        s.stmts.clear()

    # ── 指令主循环 ──────────────────────────────────────────────────
    i = 0
    while i < len(instrs):
        ins = instrs[i]

        # 循环头
        if i in loop_map:
            lp = loop_map[i]
            entries.append(('', "    loop {"))

            pre_sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names)
            pre_sim.locals = dict(sim.locals)
            for k in range(lp.start_idx, lp.cond_idx):
                sim_instr(instrs[k], pre_sim, method.class_name, registry=registry)
            sim.locals = pre_sim.locals
            for s in pre_sim.stmts:
                entries.append(('        ', s))

            ci_ins   = instrs[lp.cond_idx]
            cond_sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names)
            cond_sim.locals = dict(sim.locals)
            cond_sim.stack  = list(pre_sim.stack)
            if ci_ins.opcode in TWO_OP_CMP:
                b_expr, _ = cond_sim.pop(); a_expr, _ = cond_sim.pop()
                cond = cmp_op(ci_ins.opcode, render_expr(a_expr), render_expr(b_expr))
            else:
                a_expr, _ = cond_sim.pop()
                cond = cmp_op(ci_ins.opcode, render_expr(a_expr), '')
            entries.append(('', f"        if {cond} {{ break; }}"))

            body_sim = StackSim(rust_param_type_nodes, is_static, method.class_name, local_names)
            body_sim.locals = dict(sim.locals)
            for k in range(lp.cond_idx + 1, lp.end_idx):
                sim_instr(instrs[k], body_sim, method.class_name, registry=registry)
            sim.locals = body_sim.locals
            for s in body_sim.stmts:
                entries.append(('        ', s))

            entries.append(('', "    }"))
            i = lp.end_idx + 1
            continue

        # condition→boolean 模式（if* iconst_X goto iconst_Y）
        if i in bool_cond_map:
            true_val, false_val, false_idx, end_idx = bool_cond_map[i]
            is_two_op = ins.opcode in TWO_OP_CMP
            if is_two_op:
                b_expr, _ = sim.pop(); a_expr, _ = sim.pop()
                a_str = render_expr(a_expr); b_str = render_expr(b_expr)
            else:
                a_expr, _ = sim.pop()
                a_str = render_expr(a_expr); b_str = ''
            # true_val=1,false_val=0 → fall-through 为 true → 用 neg_cmp_op（取反跳转条件）
            # true_val=0,false_val=1 → jump 为 true    → 用 cmp_op（跳转条件即为 true）
            if true_val == 1 and false_val == 0:
                bool_expr = neg_cmp_op(ins.opcode, a_str, b_str)
            else:
                bool_expr = cmp_op(ins.opcode, a_str, b_str)
            sim.push(RawExpr(bool_expr), BOOL)
            flush(sim)
            i = end_idx
            continue

        # 普通指令
        sim_instr(ins, sim, method.class_name, registry=registry)
        flush(sim)
        i += 1

    # ── IR mutation 分析（渲染前）────────────────────────────────────
    ir_stmts = [item for _, item in entries if not isinstance(item, str)]
    _analyze_mutation(ir_stmts)

    # ── 渲染 entries → lines ─────────────────────────────────────────
    lines: list[str] = []
    for indent, item in entries:
        if isinstance(item, str):
            lines.append(item)
        else:
            lines.append(indent + render_stmt(item).lstrip())

    # ── 构造器末尾返回 Ok(this) ────────────────────────────────────
    if is_ctor:
        while lines and lines[-1].strip() in ('return;', 'return Ok(());'):
            lines.pop()
        lines.append("    Ok(this)")

    # ── 其他方法的后处理 ──────────────────────────────────────────
    if not is_ctor:
        lines = _remove_trailing_return_ok(lines)
        lines = _add_ok_return(lines, rust_ret)

    body = '\n'.join(lines)
    # 有重载时在方法前加注释，标注原始 Java 签名
    prefix = f"// java: {method.name}{method.descriptor}\n" if _overloaded else ""
    return f"{prefix}{sig} {{\n{body}\n}}"
