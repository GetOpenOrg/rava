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
from .type_map import jvm_to_rust, sig_type, rust_default
from .stack import StackSim
from .cfg import find_loops, cmp_op
from .instr import sim_instr
from .render import render_stmt


def _infer_rust_type(expr: str) -> str:
    """从 Rust 表达式推断基础类型。"""
    if re.match(r'-?\d+i32$', expr):
        return 'i32'
    if re.match(r'-?\d+i64$', expr):
        return 'i64'
    if re.match(r'-?\d+\.\d*f64$', expr) or re.match(r'^\d+f64$', expr):
        return 'f64'
    if re.match(r'-?\d+\.\d*f32$', expr) or re.match(r'^\d+f32$', expr):
        return 'f32'
    if expr.startswith('String::from') or expr.startswith('String::new()'):
        return 'String'
    if expr in ('true', 'false'):
        return 'bool'
    return ''


def _fix_coll_types(lines: list[str]) -> list[str]:
    """后处理：从 add/put 调用推断集合元素类型，修正默认 String 泛型参数。"""
    result = list(lines)

    # 收集 varname → (集合类型, 声明行索引)
    coll_vars: dict[str, tuple[str, int]] = {}
    for i, line in enumerate(result):
        m = re.match(r'\s*let\s+(?:mut\s+)?(\w+)\s*:\s*(ArrayList|HashMap|HashSet)<', line)
        if m:
            coll_vars[m.group(1)] = (m.group(2), i)

    for var, (ctype, decl_idx) in coll_vars.items():
        vr = re.escape(var)

        if ctype == 'ArrayList':
            # 从 var.add(EXPR) 推断元素类型
            elem = None
            for line in result:
                m = re.search(rf'\b{vr}\.add\((.+?)\)\??', line)
                if m:
                    elem = _infer_rust_type(m.group(1).strip())
                    if elem:
                        break
            if elem and elem != 'String':
                result[decl_idx] = re.sub(
                    r'ArrayList<String>',
                    f'ArrayList<{elem}>',
                    result[decl_idx].replace(
                        'ArrayList::<String>::new()',
                        f'ArrayList::<{elem}>::new()'
                    )
                )
                # 修正 get 返回类型标注（let _e0: String = var.get(...)）
                for j in range(len(result)):
                    result[j] = re.sub(
                        rf'(let \w+): String = ({vr}\.get\()',
                        rf'\1: {elem} = \2',
                        result[j]
                    )

        elif ctype == 'HashMap':
            # 从 var.put(K, V) 推断 V 类型
            val_elem = None
            for line in result:
                m = re.search(rf'\b{vr}\.put\(\s*.+?\s*,\s*(.+?)\s*\);', line)
                if m:
                    val_elem = _infer_rust_type(m.group(1).strip())
                    if val_elem:
                        break
            if val_elem and val_elem != 'String':
                result[decl_idx] = re.sub(
                    r'HashMap<String, String>',
                    f'HashMap<String, {val_elem}>',
                    result[decl_idx].replace(
                        'HashMap::<String, String>::new()',
                        f'HashMap::<String, {val_elem}>::new()'
                    )
                )

        elif ctype == 'HashSet':
            # 从 var.add(EXPR) 推断元素类型
            elem = None
            for line in result:
                m = re.search(rf'\b{vr}\.add\((.+?)\);', line)
                if m:
                    elem = _infer_rust_type(m.group(1).strip())
                    if elem:
                        break
            if elem and elem != 'String':
                result[decl_idx] = re.sub(
                    r'HashSet<String>',
                    f'HashSet<{elem}>',
                    result[decl_idx].replace(
                        'HashSet::<String>::new()',
                        f'HashSet::<{elem}>::new()'
                    )
                )

    return result


def _merge_aliases(lines: list[str]) -> list[str]:
    """合并冗余别名：let T a = init; let T b = a;  →  let T b = init;"""
    result = list(lines)
    i = 0
    while i < len(result) - 1:
        m1 = re.match(r'(\s*)let(?:\s+mut)? (_\w+): (.+?) = (.+);', result[i])
        m2 = re.match(r'(\s*)let(?:\s+mut)? (\w+): .+? = (\w+);', result[i + 1])
        if m1 and m2 and m1.group(2) == m2.group(3):
            src = m1.group(2)
            # 只在没有其他引用时合并
            other = sum(
                1 for j, l in enumerate(result)
                if j != i and j != i + 1 and re.search(r'\b' + re.escape(src) + r'\b', l)
            )
            if other == 0:
                indent   = m2.group(1)
                new_name = m2.group(2)
                ty_str   = m1.group(3)
                init_str = m1.group(4)
                result[i + 1] = f'{indent}let mut {new_name}: {ty_str} = {init_str};'
                result.pop(i)
                continue
        i += 1
    return result


def _simplify_wrapping_add(lines: list[str]) -> list[str]:
    """var = var.wrapping_add(N);  →  var += N;"""
    result = []
    for line in lines:
        m = re.match(r'(\s*)(\w+) = \2\.wrapping_add\((\d+)i32\);', line)
        if m:
            result.append(f'{m.group(1)}{m.group(2)} += {m.group(3)};')
        else:
            result.append(line)
    return result


def _remove_unnecessary_mut(lines: list[str]) -> list[str]:
    """删除从未被突变的 let mut 声明中的 mut。

    突变包括：赋值操作和需要 &mut self 的方法调用（如 append）。
    注意：Field<T>::set / ArrayList::add 等使用内部可变性，不需要 mut。
    """
    result = list(lines)
    for i, line in enumerate(result):
        m = re.match(r'(\s*)let mut (\w+)(?:\s*:|\s*=)', line)
        if not m:
            continue
        var = m.group(2)
        var_re = re.escape(var)
        mutated = any(
            # 赋值：var = / var += / var -= / ...
            re.search(rf'(?<![:\w\.]){var_re}\s*(?:\+|-|\*|/)?\s*=\s*(?!=)', result[j]) or
            # 数组索引赋值：var[...] = value
            re.search(rf'\b{var_re}\s*\[.*\]\s*=', result[j]) or
            # &mut self 方法调用（append 修改 String/StringBuilder）
            re.search(rf'\b{var_re}\.(push|push_str|append|append_str|extend|truncate|drain|reverse|sort|retain|reserve)\s*\(', result[j])
            for j in range(i + 1, len(result))
        )
        if not mutated:
            result[i] = re.sub(r'\blet mut\b', 'let', line, count=1)
    return result


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


def _simplify_format_in_println(lines: list[str]) -> list[str]:
    """System::out().println(format!("X {}", y))  →  System::out().println(format!("X {}", y))
    （目前已无 println! 宏，此函数保留备用）"""
    return lines


TWO_OP_CMP = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt',
    'if_icmpge', 'if_icmple', 'if_icmpgt',
})


def _indent(block: str, n: int = 4) -> str:
    pad = ' ' * n
    return '\n'.join(pad + ln if ln.strip() else '' for ln in block.split('\n'))


def gen_method_body(method: ParsedMethod, class_info: ClassInfo) -> str:
    instrs           = method.instrs
    loop_map         = {lp.start_idx: lp for lp in find_loops(instrs)}
    param_types      = method.param_types
    rust_param_types = [jvm_to_rust(t) for t in param_types]
    rust_ret         = jvm_to_rust(method.return_type)
    is_ctor          = method.is_constructor
    is_static        = method.is_static

    local_names = method.local_names or {}

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
        sig = f"pub fn new({', '.join(params)}) -> Result<Self>"

    elif method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        sig = "pub fn main() -> Result<()>"

    elif is_static:
        params = [
            f"{_param_name(k, f'arg_{k}')}: {sig_type(rt)}"
            for k, rt in enumerate(rust_param_types)
        ]
        sig = f"pub fn {method.name}({', '.join(params)})"
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
        sig = f"pub fn {method.name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> Result<{rust_ret}>"
        else:
            sig += " -> Result<()>"

    sim = StackSim(rust_param_types, is_static, method.class_name, local_names)

    # ── 构造器：创建 this ───────────────────────────────────────────
    lines: list[str] = []
    if is_ctor:
        inst_fields = [f for f in (class_info.fields if class_info else []) if not f.is_static]
        if inst_fields:
            field_inits = ', '.join(
                f"{f.name}: Field::new({rust_default(jvm_to_rust(f.descriptor))})"
                for f in inst_fields
            )
            struct_init = f"Self {{ {field_inits} }}"
        else:
            struct_init = "Self {}"
        lines.append(f"    let this = {struct_init};")
        sim.locals[0] = ('this', method.class_name, False)

    elif not is_static:
        # 实例方法：绑定 this = self，供字节码（aload_0 + getfield/putfield）使用
        lines.append("    let this = self;")

    def flush(s: StackSim):
        for stmt in s.stmts:
            lines.append('    ' + render_stmt(stmt).lstrip())
        s.stmts.clear()

    # ── 指令主循环 ──────────────────────────────────────────────────
    i = 0
    while i < len(instrs):
        ins = instrs[i]

        # 循环头
        if i in loop_map:
            lp = loop_map[i]
            lines.append("    loop {")

            pre_sim = StackSim(rust_param_types, is_static, method.class_name, local_names)
            pre_sim.locals = dict(sim.locals)
            for k in range(lp.start_idx, lp.cond_idx):
                sim_instr(instrs[k], pre_sim, method.class_name)
            sim.locals = pre_sim.locals
            for s in pre_sim.stmts:
                lines.append("        " + render_stmt(s).lstrip())

            ci_ins   = instrs[lp.cond_idx]
            cond_sim = StackSim(rust_param_types, is_static, method.class_name, local_names)
            cond_sim.locals = dict(sim.locals)
            cond_sim.stack  = list(pre_sim.stack)
            if ci_ins.opcode in TWO_OP_CMP:
                b, _ = cond_sim.pop_str(); a, _ = cond_sim.pop_str()
                cond = cmp_op(ci_ins.opcode, a, b)
            else:
                a, _ = cond_sim.pop_str()
                cond = cmp_op(ci_ins.opcode, a, '')
            lines.append(f"        if {cond} {{ break; }}")

            body_sim = StackSim(rust_param_types, is_static, method.class_name, local_names)
            body_sim.locals = dict(sim.locals)
            for k in range(lp.cond_idx + 1, lp.end_idx):
                sim_instr(instrs[k], body_sim, method.class_name)
            sim.locals = body_sim.locals
            for s in body_sim.stmts:
                lines.append("        " + render_stmt(s).lstrip())

            lines.append("    }")
            i = lp.end_idx + 1
            continue

        # 普通指令
        sim_instr(ins, sim, method.class_name)
        flush(sim)
        i += 1

    # ── 构造器末尾返回 Ok(this) ────────────────────────────────────
    if is_ctor:
        while lines and lines[-1].strip() in ('return;', 'return Ok(());'):
            lines.pop()
        lines.append("    Ok(this)")

    # ── 其他方法的后处理 ──────────────────────────────────────────
    if not is_ctor:
        lines = _remove_trailing_return_ok(lines)
        lines = _add_ok_return(lines, rust_ret)

    lines = _fix_coll_types(lines)
    lines = _merge_aliases(lines)
    lines = _simplify_wrapping_add(lines)
    lines = _remove_unnecessary_mut(lines)
    lines = _simplify_format_in_println(lines)
    body = '\n'.join(lines)
    return f"{sig} {{\n{body}\n}}"
