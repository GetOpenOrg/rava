#!/usr/bin/env python3
"""
Java .class → Rust 转译器
P0: 整数运算、静态方法调用、System.out.println、while 循环
"""

import subprocess, sys, re, os, argparse
from dataclasses import dataclass, field
from typing import Optional

# ─────────────────────────────────────────────────────────────
# 1. javap -verbose 解析
# ─────────────────────────────────────────────────────────────

@dataclass
class Instr:
    offset: int
    opcode: str
    operand: Optional[str] = None   # 原始操作数字符串（已去掉注释）
    comment: Optional[str] = None   # // 后面的注释

@dataclass
class ParsedMethod:
    class_name: str
    name: str
    descriptor: str
    is_static: bool
    locals_count: int
    args_size: int
    instrs: list  # list[Instr]

    @property
    def param_types(self): return _parse_params(self.descriptor)

    @property
    def return_type(self): return _parse_return(self.descriptor)


def _parse_params(desc: str) -> list:
    m = re.match(r'\(([^)]*)\)', desc)
    params_str = m.group(1) if m else ''
    return _parse_type_list(params_str)

def _parse_return(desc: str) -> str:
    m = re.match(r'\([^)]*\)(.*)', desc)
    return m.group(1) if m else 'V'

def _parse_type_list(s: str) -> list:
    types, i = [], 0
    while i < len(s):
        c = s[i]
        if c in 'BCDFIJSZV':
            types.append(c); i += 1
        elif c == 'L':
            end = s.index(';', i)
            types.append(s[i:end+1]); i = end + 1
        elif c == '[':
            j = i + 1
            while j < len(s) and s[j] == '[': j += 1
            if s[j] == 'L':
                end = s.index(';', j)
                types.append(s[i:end+1]); i = end + 1
            else:
                types.append(s[i:j+1]); i = j + 1
        else:
            i += 1
    return types

JVM_RUST = {
    'I': 'i32', 'J': 'i64', 'F': 'f32', 'D': 'f64',
    'Z': 'bool', 'B': 'i8', 'S': 'i16', 'C': 'u16', 'V': '()',
    'Ljava/lang/String;': 'String',
    '[I': 'Vec<i32>', '[J': 'Vec<i64>', '[F': 'Vec<f32>', '[D': 'Vec<f64>',
    '[Ljava/lang/String;': 'Vec<String>',
}

def jvm_to_rust(t: str) -> str:
    return JVM_RUST.get(t, 'JvmObject')

def default_val(rt: str) -> str:
    return {'i32':'0i32','i64':'0i64','f32':'0f32','f64':'0f64','bool':'false'}.get(rt,'Default::default()')


def parse_javap(text: str, class_name: str) -> list:
    """javap -verbose 文本 → list[ParsedMethod]"""
    lines = text.split('\n')
    methods = []

    # 状态机：SCAN → IN_METHOD → IN_CODE
    i = 0
    while i < len(lines):
        line = lines[i]

        # ── 寻找方法头（含 descriptor 的行组）──
        # 先找 descriptor: 行
        if 'descriptor:' not in line:
            i += 1
            continue

        desc_m = re.search(r'descriptor:\s*(\S+)', line)
        if not desc_m:
            i += 1
            continue
        descriptor = desc_m.group(1)

        # 向上找方法签名行（最近的含方法名的行）
        method_name = None
        is_static = False
        for k in range(i-1, max(i-5, -1), -1):
            sig_line = lines[k]
            # flags 行
            if 'ACC_STATIC' in sig_line:
                is_static = True
            # 方法名行：包含 "methodName(" 或 "<init>"
            nm = re.search(r'(\w+|<init>|<clinit>)\s*\(', sig_line)
            if nm and nm.group(1) not in ('if','for','while','class','new','return','try','catch'):
                method_name = nm.group(1)
                if 'ACC_STATIC' in sig_line:
                    is_static = True
                break

        if not method_name:
            i += 1
            continue

        # flags 行（在 descriptor 行附近）
        for k in range(i, min(i+3, len(lines))):
            if 'ACC_STATIC' in lines[k]:
                is_static = True

        # ── 找 Code: 段 ──
        code_idx = None
        for k in range(i, min(i+20, len(lines))):
            if re.match(r'\s+Code:', lines[k]):
                code_idx = k
                break

        if code_idx is None:
            i += 1
            continue

        # stack=N, locals=N, args_size=N
        locals_count, args_size = 1, 1
        stack_line = lines[code_idx + 1] if code_idx + 1 < len(lines) else ''
        lm = re.search(r'locals=(\d+)', stack_line)
        am = re.search(r'args_size=(\d+)', stack_line)
        if lm: locals_count = int(lm.group(1))
        if am: args_size = int(am.group(1))

        # ── 解析指令 ──
        instrs = []
        j = code_idx + 2
        while j < len(lines):
            # 指令行格式：   {offset}: {opcode} {operand}   // comment
            instr_m = re.match(r'\s+(\d+):\s+(\w+)(.*)', lines[j])
            if instr_m:
                offset = int(instr_m.group(1))
                opcode = instr_m.group(2)
                rest   = instr_m.group(3).strip()
                comment = None
                if '//' in rest:
                    rest, comment = rest.split('//', 1)
                    rest = rest.strip()
                    comment = comment.strip()
                operand = rest.strip() or None
                instrs.append(Instr(offset=offset, opcode=opcode,
                                    operand=operand, comment=comment))
                j += 1
            elif re.match(r'\s+(LineNumberTable|StackMapTable|frame_type|offset_delta|locals\s*=)', lines[j]):
                break
            elif lines[j].strip() == '':
                j += 1
            else:
                j += 1
                if instrs:
                    # 遇到非指令非空行，可能是下一个方法
                    break

        if instrs:
            methods.append(ParsedMethod(
                class_name=class_name,
                name=method_name,
                descriptor=descriptor,
                is_static=is_static,
                locals_count=locals_count,
                args_size=args_size,
                instrs=instrs,
            ))
        i = j

    return methods


# ─────────────────────────────────────────────────────────────
# 2. 常量池解析（从 javap -verbose 中提取注释中的引用信息）
# ─────────────────────────────────────────────────────────────

def parse_cp_from_comments(methods: list) -> dict:
    """
    javap 在每条引用指令后附有 // 注释，例如：
      invokestatic  #7   // Method add:(II)I
      getstatic     #13  // Field java/lang/System.out:Ljava/io/PrintStream;
    从注释里提取目标信息。
    """
    cp = {}
    for m in methods:
        for instr in m.instrs:
            if instr.operand and instr.operand.startswith('#') and instr.comment:
                idx_s = instr.operand.split()[0].lstrip('#')
                if idx_s.isdigit():
                    cp[int(idx_s)] = instr.comment
    return cp


# ─────────────────────────────────────────────────────────────
# 3. 控制流分析：识别 while 循环
# ─────────────────────────────────────────────────────────────

@dataclass
class LoopInfo:
    start_idx: int    # 循环头的指令下标
    end_idx: int      # goto 指令的下标
    cond_idx: int     # 条件分支指令的下标
    exit_offset: int  # 跳出循环的目标 offset

def find_loops(instrs: list) -> list:
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    loops = []
    for i, ins in enumerate(instrs):
        if ins.opcode == 'goto' and ins.operand:
            target_off = int(ins.operand)
            if target_off < ins.offset:
                start_idx = off2idx.get(target_off)
                if start_idx is None: continue
                # 在循环体内找条件退出指令
                cond_idx, exit_off = None, None
                for j in range(start_idx, i + 1):
                    op = instrs[j].opcode
                    if op.startswith('if') and instrs[j].operand:
                        off = int(instrs[j].operand)
                        if off > ins.offset:
                            cond_idx = j
                            exit_off = off
                            break
                if cond_idx is not None:
                    loops.append(LoopInfo(
                        start_idx=start_idx,
                        end_idx=i,
                        cond_idx=cond_idx,
                        exit_offset=exit_off,
                    ))
    return loops


# ─────────────────────────────────────────────────────────────
# 4. JVM 栈模拟器 → SSA 变量
# ─────────────────────────────────────────────────────────────

class StackSim:
    def __init__(self, args_size: int, param_rust_types: list):
        self.stack = []          # (expr_str, rust_type)
        self._ctr = 0
        self.locals = {}         # slot → (name, rust_type, is_mut)
        self.stmts = []          # 生成的 Rust 语句行

        # 参数：slot 0..args_size-1
        for slot, rt in enumerate(param_rust_types):
            self.locals[slot] = (f"arg_{slot}", rt, False)

    def fresh(self, prefix='_t') -> str:
        v = f"{prefix}{self._ctr}"; self._ctr += 1; return v

    def push(self, expr: str, ty: str = 'i32'):
        self.stack.append((expr, ty))

    def pop(self) -> tuple:
        return self.stack.pop() if self.stack else ('/* UNDERFLOW */', 'i32')

    def peek(self) -> tuple:
        return self.stack[-1] if self.stack else ('/* EMPTY */', 'i32')

    def emit(self, s: str):
        self.stmts.append(s)

    def store_local(self, slot: int, expr: str, ty: str):
        if slot in self.locals:
            name, _, _ = self.locals[slot]
            self.emit(f"    {name} = {expr};")
        else:
            name = f"local_{slot}"
            self.locals[slot] = (name, ty, True)
            self.emit(f"    let mut {name}: {ty} = {expr};")

    def load_local(self, slot: int) -> tuple:
        if slot in self.locals:
            name, ty, _ = self.locals[slot]
            return (name, ty)
        return (f"local_{slot}", 'i32')


# 条件分支反转：用于 loop { if !cond { break; } ... }
COND_NEGATE = {
    'if_icmpeq': 'if_icmpne', 'if_icmpne': 'if_icmpeq',
    'if_icmplt': 'if_icmpge', 'if_icmpge': 'if_icmplt',
    'if_icmple': 'if_icmpgt', 'if_icmpgt': 'if_icmple',
    'ifeq': 'ifne', 'ifne': 'ifeq',
    'iflt': 'ifge', 'ifge': 'iflt',
    'ifle': 'ifgt', 'ifgt': 'ifle',
    'ifnull': 'ifnonnull', 'ifnonnull': 'ifnull',
}

def cmp_op(opcode: str, a: str, b: str) -> str:
    """生成比较表达式（正向，用于 break 条件）"""
    ops = {
        'if_icmpeq': '==', 'if_icmpne': '!=',
        'if_icmplt': '<',  'if_icmpge': '>=',
        'if_icmple': '<=', 'if_icmpgt': '>',
    }
    if opcode in ops:
        return f"{a} {ops[opcode]} {b}"
    # ifeq/ifne/iflt... 单操作数
    single = {
        'ifeq': f"{a} == 0i32", 'ifne': f"{a} != 0i32",
        'iflt': f"{a} < 0i32",  'ifge': f"{a} >= 0i32",
        'ifle': f"{a} <= 0i32", 'ifgt': f"{a} > 0i32",
    }
    return single.get(opcode, f"/* {opcode} */ true")


# ─────────────────────────────────────────────────────────────
# 5. 方法体代码生成
# ─────────────────────────────────────────────────────────────

def gen_method_body(method: ParsedMethod, cp: dict) -> str:
    """将字节码指令序列翻译为 Rust 函数体（带 4-space 缩进）"""
    instrs = method.instrs
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    loops = find_loops(instrs)

    # 循环覆盖的 idx 范围
    loop_ranges = {}  # start_idx → LoopInfo
    loop_exit_offsets = set()
    for lp in loops:
        loop_ranges[lp.start_idx] = lp
        loop_exit_offsets.add(lp.exit_offset)

    # 参数的 Rust 类型（跳过 this 指针：非静态方法 slot 0 是 this）
    param_types = method.param_types
    rust_param_types = [jvm_to_rust(t) for t in param_types]
    rust_ret = jvm_to_rust(method.return_type)

    # 函数签名
    if method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        # Java main → Rust main
        sig = "pub fn main()"
    else:
        params = []
        for k, rt in enumerate(rust_param_types):
            params.append(f"arg_{k}: {rt}")
        sig_params = ", ".join(params)
        if rust_ret == '()':
            sig = f"pub fn {method.name}({sig_params})"
        else:
            sig = f"pub fn {method.name}({sig_params}) -> {rust_ret}"

    sim = StackSim(method.args_size, rust_param_types)
    lines = []
    in_loop = False

    def flush_stack_to_var(sim: StackSim, ty='i32') -> str:
        expr, _ = sim.pop()
        return expr

    # 遍历指令
    i = 0
    while i < len(instrs):
        ins = instrs[i]
        op = ins.opcode
        operand = ins.operand or ''

        # ── 进入循环头 ──
        if i in loop_ranges:
            lp = loop_ranges[i]
            lines.append("    loop {")
            in_loop = True

            # 条件指令在 cond_idx，跳过循环头到条件之前的指令（正常模拟）
            # 先把 [start_idx, cond_idx) 的指令都模拟了
            sub_sim = StackSim(method.args_size, rust_param_types)
            sub_sim.locals = sim.locals.copy()

            for k in range(lp.start_idx, lp.cond_idx):
                _sim_instr(instrs[k], sub_sim, cp, method.class_name)

            sim.locals = sub_sim.locals
            for stmt in sub_sim.stmts:
                lines.append("    " + stmt.strip())

            # 条件指令 → break 语句
            cond_ins = instrs[lp.cond_idx]
            if cond_ins.opcode in ('if_icmpeq','if_icmpne','if_icmplt','if_icmpge','if_icmple','if_icmpgt'):
                b_expr, _ = sub_sim.pop()
                a_expr, _ = sub_sim.pop()
                cond_str = cmp_op(cond_ins.opcode, a_expr, b_expr)
            else:
                a_expr, _ = sub_sim.pop()
                cond_str = cmp_op(cond_ins.opcode, a_expr, '')

            lines.append(f"        if {cond_str} {{ break; }}")

            # 模拟循环体（cond_idx+1 到 end_idx-1）
            body_sim = StackSim(method.args_size, rust_param_types)
            body_sim.locals = sim.locals.copy()

            for k in range(lp.cond_idx + 1, lp.end_idx):
                _sim_instr(instrs[k], body_sim, cp, method.class_name)

            sim.locals = body_sim.locals
            for stmt in body_sim.stmts:
                lines.append("    " + stmt.strip())

            lines.append("    }  // loop")
            in_loop = False
            i = lp.end_idx + 1  # 跳过 goto
            continue

        # ── 循环退出目标偏移处不做特殊处理，正常继续 ──

        # ── 普通指令 ──
        _sim_instr(ins, sim, cp, method.class_name)
        for stmt in sim.stmts:
            lines.append(stmt)
        sim.stmts.clear()

        i += 1

    # 组合输出
    body = "\n".join(lines)
    return f"{sig} {{\n{body}\n}}"


def _sim_instr(ins: Instr, sim: StackSim, cp: dict, class_name: str):
    """单条指令 → 栈操作 + emit Rust 语句"""
    op = ins.opcode
    operand = ins.operand or ''

    # ── 整型常量入栈 ──
    if op == 'iconst_m1': sim.push('-1i32')
    elif op.startswith('iconst_'):  sim.push(f"{op[-1]}i32")
    elif op == 'lconst_0': sim.push('0i64', 'i64')
    elif op == 'lconst_1': sim.push('1i64', 'i64')
    elif op == 'fconst_0': sim.push('0f32', 'f32')
    elif op == 'fconst_1': sim.push('1f32', 'f32')
    elif op == 'dconst_0': sim.push('0f64', 'f64')
    elif op == 'dconst_1': sim.push('1f64', 'f64')
    elif op == 'bipush': sim.push(f"{operand}i32")
    elif op == 'sipush': sim.push(f"{operand}i32")
    elif op == 'ldc':
        # 字符串或整数
        if operand.startswith('"'):
            sim.push(f"{operand}.to_string()", 'String')
        else:
            sim.push(f"{operand}i32")

    # ── load local ──
    elif op.startswith('iload'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.load_local(slot)
        sim.push(expr, ty)
    elif op.startswith('lload'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.load_local(slot)
        sim.push(expr, ty or 'i64')
    elif op.startswith('fload'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.load_local(slot)
        sim.push(expr, ty or 'f32')
    elif op.startswith('dload'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.load_local(slot)
        sim.push(expr, ty or 'f64')
    elif op.startswith('aload'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.load_local(slot)
        sim.push(expr, ty or 'JvmObject')

    # ── store local ──
    elif op.startswith('istore'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.pop()
        sim.store_local(slot, expr, 'i32')
    elif op.startswith('lstore'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.pop()
        sim.store_local(slot, expr, 'i64')
    elif op.startswith('fstore'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.pop()
        sim.store_local(slot, expr, 'f32')
    elif op.startswith('dstore'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.pop()
        sim.store_local(slot, expr, 'f64')
    elif op.startswith('astore'):
        slot = _parse_slot(op, operand)
        expr, ty = sim.pop()
        sim.store_local(slot, expr, ty or 'JvmObject')

    # ── 整数算术 ──
    elif op == 'iadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_add({b})")
    elif op == 'isub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_sub({b})")
    elif op == 'imul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_mul({b})")
    elif op == 'idiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a} / {b})")
    elif op == 'irem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a} % {b})")
    elif op == 'ineg':
        a, _ = sim.pop()
        sim.push(f"({a}).wrapping_neg()")
    elif op == 'ishl':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a} << ({b} & 0x1f))")
    elif op == 'ishr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a} >> ({b} & 0x1f))")
    elif op == 'iushr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"(({a} as u32 >> ({b} & 0x1f)) as i32)")
    elif op == 'iand':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} & {b})")
    elif op == 'ior':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} | {b})")
    elif op == 'ixor':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} ^ {b})")
    elif op == 'iinc':
        # iinc slot, const
        parts = operand.split(',') if ',' in operand else operand.split()
        slot = int(parts[0].strip())
        delta = int(parts[1].strip())
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", 'i32', True))
        if delta >= 0:
            sim.emit(f"    {name} = {name}.wrapping_add({delta}i32);")
        else:
            sim.emit(f"    {name} = {name}.wrapping_sub({-delta}i32);")

    # ── long 算术 ──
    elif op == 'ladd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_add({b})", 'i64')
    elif op == 'lsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_sub({b})", 'i64')
    elif op == 'lmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(f"({a}).wrapping_mul({b})", 'i64')
    elif op == 'ldiv':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} / {b})", 'i64')
    elif op == 'lrem':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} % {b})", 'i64')

    # ── float/double 算术 ──
    elif op == 'fadd':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} + {b})", 'f32')
    elif op == 'fsub':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} - {b})", 'f32')
    elif op == 'fmul':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} * {b})", 'f32')
    elif op == 'fdiv':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} / {b})", 'f32')
    elif op == 'dadd':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} + {b})", 'f64')
    elif op == 'dsub':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} - {b})", 'f64')
    elif op == 'dmul':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} * {b})", 'f64')
    elif op == 'ddiv':
        b, _ = sim.pop(); a, _ = sim.pop(); sim.push(f"({a} / {b})", 'f64')

    # ── 类型转换 ──
    elif op == 'i2l': a, _ = sim.pop(); sim.push(f"({a} as i64)", 'i64')
    elif op == 'i2f': a, _ = sim.pop(); sim.push(f"({a} as f32)", 'f32')
    elif op == 'i2d': a, _ = sim.pop(); sim.push(f"({a} as f64)", 'f64')
    elif op == 'l2i': a, _ = sim.pop(); sim.push(f"({a} as i32)", 'i32')
    elif op == 'f2i': a, _ = sim.pop(); sim.push(f"({a} as i32)", 'i32')
    elif op == 'd2i': a, _ = sim.pop(); sim.push(f"({a} as i32)", 'i32')
    elif op == 'd2f': a, _ = sim.pop(); sim.push(f"({a} as f32)", 'f32')
    elif op == 'f2d': a, _ = sim.pop(); sim.push(f"({a} as f64)", 'f64')

    # ── 字段访问 ──
    elif op == 'getstatic':
        # getstatic #N  // Field java/lang/System.out:Ljava/io/PrintStream;
        comment = ins.comment or ''
        if 'System.out' in comment:
            sim.push('__stdout__', 'PrintStream')
        elif 'System.err' in comment:
            sim.push('__stderr__', 'PrintStream')
        else:
            # 其他 static field，暂用 todo!()
            sim.push(f"/* getstatic {comment} */ todo!()", 'JvmObject')
    elif op == 'putstatic':
        sim.pop()  # 暂不处理

    # ── 方法调用 ──
    elif op == 'invokestatic':
        comment = ins.comment or ''
        _gen_invokestatic(sim, comment, class_name)

    elif op == 'invokevirtual':
        comment = ins.comment or ''
        _gen_invokevirtual(sim, comment)

    elif op == 'invokespecial':
        comment = ins.comment or ''
        if '"<init>"' in comment or '<init>' in comment:
            # super.<init>() or Object.<init>() → drop
            if sim.stack and sim.stack[-1][1] in ('JvmObject', 'PrintStream'):
                sim.pop()
        else:
            _gen_invokestatic(sim, comment, class_name)

    # ── 返回 ──
    elif op == 'return':
        sim.emit('    return;')
    elif op in ('ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn'):
        expr, _ = sim.pop()
        sim.emit(f"    return {expr};")

    # ── dup / pop ──
    elif op == 'dup':
        if sim.stack:
            top = sim.stack[-1]
            sim.stack.append(top)
    elif op == 'pop':
        if sim.stack:
            expr, _ = sim.pop()
            # 可能是有副作用的表达式，emit 为 let _=...
            if any(c in expr for c in ['(', 'println', 'todo']):
                sim.emit(f"    let _ = {expr};")
    elif op == 'pop2':
        sim.pop(); sim.pop() if sim.stack else None

    # ── 控制流（简单 forward if，非循环内） ──
    elif op.startswith('if_icmp') or op.startswith('if'):
        # 在此简化处理：emit 注释，不影响 P0 主流程
        if op in ('if_icmpeq','if_icmpne','if_icmplt','if_icmpge','if_icmple','if_icmpgt'):
            b, _ = sim.pop(); a, _ = sim.pop()
            sim.emit(f"    /* {op} {a} {b} → offset {operand} */")
        else:
            a, _ = sim.pop()
            sim.emit(f"    /* {op} {a} → offset {operand} */")

    elif op == 'goto':
        pass  # 由 find_loops 处理

    elif op in ('nop', 'wide'):
        pass

    else:
        # 未实现指令：emit 注释占位
        sim.emit(f"    /* TODO: {op} {operand} */")


def _parse_slot(op: str, operand: str) -> int:
    """iload_0 → 0, iload 3 → 3"""
    if '_' in op:
        return int(op.split('_')[-1])
    return int(operand.strip())


def _parse_method_ref(comment: str) -> tuple:
    """
    'Method TestP0.add:(II)I'   → ('TestP0', 'add', ['I','I'], 'I')
    'Method add:(II)I'          → (None, 'add', ['I','I'], 'I')
    'Method java/io/PrintStream.println:(I)V' → ('PrintStream', 'println', ['I'], 'V')
    """
    comment = comment.strip()
    for prefix in ('Method ', 'InterfaceMethod '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]

    # class.name:desc  or  name:desc
    m = re.match(r'(?:([^.]+)\.)?(\w+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m:
        return (None, comment, [], 'V')

    raw_class = m.group(1)
    mname = m.group(2)
    desc = m.group(3)

    # 简化类名：java/io/PrintStream → PrintStream
    if raw_class:
        raw_class = raw_class.split('/')[-1].split('.')[-1]

    params = _parse_params(desc)
    ret = _parse_return(desc)
    return (raw_class, mname, params, ret)


def _gen_invokestatic(sim: StackSim, comment: str, this_class: str):
    cls, mname, params, ret = _parse_method_ref(comment)
    n_args = len(params)
    args = []
    for _ in range(n_args):
        expr, _ = sim.pop()
        args.insert(0, expr)

    # Math 类特殊处理
    if cls == 'Math':
        rust_fn = {
            'abs': 'i32::abs', 'max': 'i32::max', 'min': 'i32::min',
        }.get(mname, f"java_runtime::math::{mname}")
        call = f"{rust_fn}({', '.join(args)})"
    elif cls is None or cls == this_class:
        call = f"Self::{mname}({', '.join(args)})"
    else:
        call = f"{cls}::{mname}({', '.join(args)})"

    rust_ret = jvm_to_rust(ret)
    if rust_ret == '()':
        sim.emit(f"    {call};")
    else:
        v = sim.fresh()
        sim.emit(f"    let {v}: {rust_ret} = {call};")
        sim.push(v, rust_ret)


def _gen_invokevirtual(sim: StackSim, comment: str):
    cls, mname, params, ret = _parse_method_ref(comment)
    n_args = len(params)
    args = []
    for _ in range(n_args):
        expr, _ = sim.pop()
        args.insert(0, expr)
    obj_expr, obj_ty = sim.pop()

    # System.out.println 系列
    if obj_ty == 'PrintStream' or obj_expr in ('__stdout__', '__stderr__'):
        stream = "eprintln!" if 'stderr' in obj_expr else "println!"
        if len(args) == 1:
            sim.emit(f'    {stream}("{{}}", {args[0]});')
        elif len(args) == 0:
            sim.emit(f'    {stream}("");')
        else:
            sim.emit(f'    {stream}("{{}}", {args[0]});')
        return

    # StringBuilder.append
    if cls == 'StringBuilder' and mname == 'append':
        sim.push(f"{obj_expr}.append({args[0] if args else ''})", 'StringBuilder')
        return

    # StringBuilder.toString
    if cls == 'StringBuilder' and mname == 'toString':
        sim.push(f"{obj_expr}.build()", 'String')
        return

    # 通用虚方法
    call = f"{obj_expr}.{mname}({', '.join(args)})"
    rust_ret = jvm_to_rust(ret)
    if rust_ret == '()':
        sim.emit(f"    {call};")
    else:
        v = sim.fresh()
        sim.emit(f"    let {v}: {rust_ret} = {call};")
        sim.push(v, rust_ret)


# ─────────────────────────────────────────────────────────────
# 6. Cargo 项目生成
# ─────────────────────────────────────────────────────────────

RUNTIME_MOD = '''\
// java_runtime/mod.rs — JVM 运行时存根
pub mod math;
pub mod io;
pub mod error;
'''

RUNTIME_ERROR = '''\
// java_runtime/error.rs
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    Custom(String),
}
'''

RUNTIME_MATH = '''\
// java_runtime/math.rs — java.lang.Math 存根
pub fn abs_i32(a: i32) -> i32  { a.abs() }
pub fn abs_i64(a: i64) -> i64  { a.abs() }
pub fn abs_f64(a: f64) -> f64  { a.abs() }
pub fn max_i32(a: i32, b: i32) -> i32 { a.max(b) }
pub fn min_i32(a: i32, b: i32) -> i32 { a.min(b) }
pub fn sqrt(a: f64) -> f64    { a.sqrt() }
pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
pub fn floor(a: f64) -> f64   { a.floor() }
pub fn ceil(a: f64) -> f64    { a.ceil() }
pub fn log(a: f64) -> f64     { a.ln() }
pub fn log10(a: f64) -> f64   { a.log10() }
pub fn random() -> f64        { 0.5 }  // TODO: rand crate
'''

RUNTIME_IO = '''\
// java_runtime/io.rs — System.out / System.err 存根
pub fn println_int(v: i32)    { println!("{}", v); }
pub fn println_str(v: &str)   { println!("{}", v); }
pub fn println_long(v: i64)   { println!("{}", v); }
pub fn println_double(v: f64) { println!("{}", v); }
pub fn println_bool(v: bool)  { println!("{}", v); }
pub fn println_empty()        { println!(); }
'''

CARGO_TOML = '''\
[package]
name = "java_transpiled"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 3
lto       = true
codegen-units = 1
strip     = "symbols"
'''


def _indent(block: str, n: int = 4) -> str:
    """每行加 n 个空格缩进（空行不加）"""
    pad = ' ' * n
    return '\n'.join(pad + line if line.strip() else '' for line in block.split('\n'))


def write_cargo_project(out_dir: str, class_name: str, methods: list, cp: dict):
    src_dir  = os.path.join(out_dir, 'src')
    rt_dir   = os.path.join(src_dir, 'java_runtime')
    os.makedirs(rt_dir, exist_ok=True)

    # runtime stubs
    for fname, content in [('mod.rs', RUNTIME_MOD), ('error.rs', RUNTIME_ERROR),
                            ('math.rs', RUNTIME_MATH), ('io.rs', RUNTIME_IO)]:
        with open(os.path.join(rt_dir, fname), 'w') as f:
            f.write(content)

    # Cargo.toml
    with open(os.path.join(out_dir, 'Cargo.toml'), 'w') as f:
        f.write(CARGO_TOML)

    # 过滤构造器（name == '<init>' 或 name == class_name）
    code_blocks = []
    for m in methods:
        if m.name in ('<init>', '<clinit>', class_name):
            continue
        try:
            body = gen_method_body(m, cp)
        except Exception as e:
            body = f"/* codegen error for {m.name}: {e} */"
        code_blocks.append(body)

    impl_body = '\n\n'.join(_indent(b) for b in code_blocks)

    main_rs = (
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]\n"
        "mod java_runtime;\n\n"
        f"pub struct {class_name};\n\n"
        f"impl {class_name} {{\n"
        f"{impl_body}\n"
        "}\n\n"
        f"fn main() {{ {class_name}::main(); }}\n"
    )

    with open(os.path.join(src_dir, 'main.rs'), 'w') as f:
        f.write(main_rs)
    print(f"[codegen] Cargo project written to: {out_dir}/")


# ─────────────────────────────────────────────────────────────
# 7. 主入口
# ─────────────────────────────────────────────────────────────

def transpile(java_file: str, out_dir: str):
    if not os.path.exists(java_file):
        sys.exit(f"File not found: {java_file}")

    class_name = os.path.splitext(os.path.basename(java_file))[0]
    work_dir   = os.path.dirname(os.path.abspath(java_file))

    # 1. 编译 .java → .class
    print(f"[1/4] javac {java_file}")
    r = subprocess.run(['javac', java_file], capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. javap -verbose
    class_file = os.path.join(work_dir, class_name + '.class')
    print(f"[2/4] javap -verbose {class_name}")
    r = subprocess.run(['javap', '-verbose', '-p', class_file],
                       capture_output=True, text=True)
    javap_text = r.stdout

    # 3. 解析字节码
    print(f"[3/4] 解析字节码 + 构建常量池")
    methods = parse_javap(javap_text, class_name)
    cp = parse_cp_from_comments(methods)
    print(f"      解析到 {len(methods)} 个方法")
    for m in methods:
        print(f"      {'static ' if m.is_static else ''}  {m.name}{m.descriptor}  ({len(m.instrs)} 条指令)")

    # 4. 生成 Rust 项目
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_name, methods, cp)

    print("\n✓ 完成。运行方式：")
    print(f"  cd {out_dir} && cargo run --release")


if __name__ == '__main__':
    ap = argparse.ArgumentParser(description='Java → Rust 转译器 (P0)')
    ap.add_argument('java_file', help='.java 源文件')
    ap.add_argument('--out', default='output', help='输出目录（默认 output）')
    args = ap.parse_args()
    transpile(args.java_file, args.out)
