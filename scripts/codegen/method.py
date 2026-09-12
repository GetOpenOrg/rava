"""
方法体生成：ParsedMethod + ClassInfo → Rust 函数字符串。
处理构造器、静态方法、实例方法、while 循环。
"""

from .types import ParsedMethod, ClassInfo
from .type_map import jvm_to_rust, sig_type, rust_default
from .stack import StackSim
from .cfg import find_loops, cmp_op
from .instr import sim_instr

TWO_OP_CMP = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt',
    'if_icmpge', 'if_icmple', 'if_icmpgt',
})


def _indent(block: str, n: int = 4) -> str:
    pad = ' ' * n
    return '\n'.join(pad + ln if ln.strip() else '' for ln in block.split('\n'))


def gen_method_body(method: ParsedMethod, class_info: ClassInfo) -> str:
    instrs          = method.instrs
    loop_map        = {lp.start_idx: lp for lp in find_loops(instrs)}
    param_types     = method.param_types
    rust_param_types = [jvm_to_rust(t) for t in param_types]
    rust_ret        = jvm_to_rust(method.return_type)
    is_ctor         = method.is_constructor
    is_static       = method.is_static

    # ── 函数签名 ──
    if is_ctor:
        params = [f"arg_{k}: {rt}" for k, rt in enumerate(rust_param_types)]
        sig    = f"pub fn new({', '.join(params)}) -> Rc<RefCell<Self>>"
    elif method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        sig = "pub fn main()"
    elif is_static:
        params = [f"arg_{k}: {sig_type(rt)}" for k, rt in enumerate(rust_param_types)]
        sig    = f"pub fn {method.name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> {rust_ret}"
    else:
        params = ["this: &Rc<RefCell<Self>>"]
        params += [f"arg_{k}: {rt}" for k, rt in enumerate(rust_param_types)]
        sig    = f"pub fn {method.name}({', '.join(params)})"
        if rust_ret != '()':
            sig += f" -> {rust_ret}"

    sim = StackSim(rust_param_types, is_static, method.class_name)

    # ── 构造器：预先分配 this ──
    lines: list[str] = []
    if is_ctor:
        inst_fields = [f for f in (class_info.fields if class_info else []) if not f.is_static]
        if inst_fields:
            field_inits = ', '.join(
                f"{f.name}: {rust_default(jvm_to_rust(f.descriptor))}"
                for f in inst_fields
            )
            struct_init = f"Self {{ {field_inits} }}"
        else:
            struct_init = "Self {}"
        lines.append(f"    let this: Rc<RefCell<Self>> = Rc::new(RefCell::new({struct_init}));")
        sim.locals[0] = ('this', f"Rc<RefCell<{method.class_name}>>", False)

    def flush(s: StackSim):
        lines.extend(s.stmts)
        s.stmts.clear()

    # ── 指令主循环 ──
    i = 0
    while i < len(instrs):
        ins = instrs[i]

        # 循环头
        if i in loop_map:
            lp = loop_map[i]
            lines.append("    loop {")

            # 条件前指令
            pre_sim = StackSim(rust_param_types, is_static, method.class_name)
            pre_sim.locals = dict(sim.locals)
            for k in range(lp.start_idx, lp.cond_idx):
                sim_instr(instrs[k], pre_sim, method.class_name)
            sim.locals = pre_sim.locals
            for s in pre_sim.stmts:
                lines.append("    " + s.strip())

            # 循环条件
            ci       = instrs[lp.cond_idx]
            cond_sim = StackSim(rust_param_types, is_static, method.class_name)
            cond_sim.locals = dict(sim.locals)
            cond_sim.stack  = list(pre_sim.stack)
            if ci.opcode in TWO_OP_CMP:
                b, _ = cond_sim.pop(); a, _ = cond_sim.pop()
                cond = cmp_op(ci.opcode, a, b)
            else:
                a, _ = cond_sim.pop()
                cond = cmp_op(ci.opcode, a, '')
            lines.append(f"        if {cond} {{ break; }}")

            # 循环体
            body_sim = StackSim(rust_param_types, is_static, method.class_name)
            body_sim.locals = dict(sim.locals)
            for k in range(lp.cond_idx + 1, lp.end_idx):
                sim_instr(instrs[k], body_sim, method.class_name)
            sim.locals = body_sim.locals
            for s in body_sim.stmts:
                lines.append("    " + s.strip())

            lines.append("    }  // end loop")
            i = lp.end_idx + 1
            continue

        # 普通指令
        sim_instr(ins, sim, method.class_name)
        flush(sim)
        i += 1

    # 构造器末尾返回 this
    if is_ctor:
        while lines and lines[-1].strip() == 'return;':
            lines.pop()
        lines.append("    this")

    body = '\n'.join(lines)
    return f"{sig} {{\n{body}\n}}"
