"""
JVM 字节码指令 → Rust 语句转换。
每条指令操作 StackSim 的栈与语句列表。

新设计：
- String（java.lang.String）而非 Rust std::string::String
- 字段通过 Field<T>::get()/set() 访问
- println → System::out().println(...)?
- 所有用户类方法调用加 ?（返回 Result<T>）
- ArrayList::add()?  HashMap::put  HashSet::add
"""

import re
from .stack import StackSim, I32, I64, F32, F64, BOOL, UNIT
from .rs_ir import (
    Lit, Var, BinOp, UnOp, Call, MethodCall, FieldAccess, Index,
    Cast, RawExpr, RawStmt, LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    IfStmt, LoopStmt, BreakStmt, RsNamed, RsGeneric, RsRef, RsSlice, RsInfer,
    NewPendingExpr, StaticFieldRef,
)
from .render import render_expr, render_type
from .type_map import (
    jvm_to_rust, sig_type, short_cls,
    NEWARRAY_TYPES,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL,
    parse_descriptor_params, parse_descriptor_return,
)

# 集合类构造时的 IR 类型节点和初始化表达式
# 用 RsInfer() 占位，Rust 编译器从首次 add/put 调用推断实际元素类型
_COLL_IR_TYPES: dict[str, tuple] = {
    'ArrayList':           (RsGeneric('ArrayList', [RsInfer()]),         'ArrayList::<_>::new()?'),
    'java/util/ArrayList': (RsGeneric('ArrayList', [RsInfer()]),         'ArrayList::<_>::new()?'),
    'HashMap':             (RsGeneric('HashMap',   [RsInfer(), RsInfer()]), 'HashMap::<_, _>::new()?'),
    'java/util/HashMap':   (RsGeneric('HashMap',   [RsInfer(), RsInfer()]), 'HashMap::<_, _>::new()?'),
    'HashSet':             (RsGeneric('HashSet',   [RsInfer()]),         'HashSet::<_>::new()?'),
    'java/util/HashSet':   (RsGeneric('HashSet',   [RsInfer()]),         'HashSet::<_>::new()?'),
}
from .types import Instr


# ── 辅助：解析 javap 注释中的方法引用 ────────────────────────────

def parse_method_ref(comment: str) -> tuple[str | None, str, list, str]:
    """
    解析 'Method Foo.bar:(II)I' 或 'InterfaceMethod ...' 格式。
    返回 (class_short_name, method_name, param_jvm_types, return_jvm_type)
    """
    comment = comment.strip()
    for prefix in ('Method ', 'InterfaceMethod '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]

    comment = (comment
               .replace('"<init>"',   '__init__')
               .replace('"<clinit>"', '__clinit__')
               .replace('<init>',     '__init__')
               .replace('<clinit>',   '__clinit__'))

    m = re.match(r'(?:([^.]+)\.)?(\w+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m:
        return (None, comment, [], 'V')

    raw_cls = m.group(1)
    mname   = m.group(2).replace('__init__', '<init>').replace('__clinit__', '<clinit>')
    desc    = m.group(3)

    if raw_cls:
        raw_cls = raw_cls.split('/')[-1].split('.')[-1]

    return (raw_cls, mname, parse_descriptor_params(desc), parse_descriptor_return(desc))


def _parse_slot(op: str, operand: str) -> int:
    if '_' in op:
        return int(op.split('_')[-1])
    return int(operand.strip()) if operand else 0


def _parse_field_ref(comment: str) -> tuple[str, str, str]:
    """解析 'Field java/lang/System.out:Ljava/io/PrintStream;' 格式。
    返回 (class_binary_name, field_name, descriptor)。"""
    comment = comment.strip()
    for prefix in ('Field ', 'InterfaceField '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]
    # 格式：java/lang/System.out:Ljava/io/PrintStream;
    if ':' in comment:
        ref_part, descriptor = comment.split(':', 1)
    else:
        ref_part, descriptor = comment, 'Ljava/lang/Object;'
    if '.' in ref_part:
        cls, field = ref_part.rsplit('.', 1)
    else:
        cls, field = '', ref_part
    return cls, field, descriptor


# ── 主分发函数 ────────────────────────────────────────────────────

def sim_instr(ins: Instr, sim: StackSim, class_name: str, registry: dict | None = None):
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 整型常量 ──
    if   op == 'iconst_m1':                      sim.push(Lit('-1i32'), I32)
    elif op.startswith('iconst_'):               sim.push(Lit(f"{op[-1]}i32"), I32)
    elif op in ('lconst_0', 'lconst_1'):         sim.push(Lit(f"{op[-1]}i64"), I64)
    elif op in ('fconst_0', 'fconst_1', 'fconst_2'): sim.push(Lit(f"{op[-1]}f32"), F32)
    elif op in ('dconst_0', 'dconst_1'):         sim.push(Lit(f"{op[-1]}f64"), F64)
    elif op == 'bipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'sipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'ldc':
        if operand.startswith('"'):
            # 字符串字面量 → java.lang.String
            sim.push(Lit(f"String::from({operand})"), RsNamed('String'))
        elif comment.startswith('String '):
            lit = comment[7:].strip()
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('int '):    sim.push(Lit(comment[4:].strip() + 'i32'), I32)
        elif comment.startswith('float '): sim.push(Lit(comment[6:].strip() + 'f32'), F32)
        elif comment.startswith('long '):  sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(comment[7:].strip() + 'f64'), F64)
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op in ('ldc2_w', 'ldc_w'):
        if comment.startswith('long '):   sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(comment[7:].strip() + 'f64'), F64)
        elif comment.startswith('String '):
            lit = comment[7:].strip()
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        else: sim.push(Lit(f"{operand}i32"), I32)

    # ── load ──
    elif op.startswith('iload'): sim.push(*sim.load_local(_parse_slot(op, operand)))
    elif op.startswith('lload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, I64)
    elif op.startswith('fload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F32)
    elif op.startswith('dload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F64)
    elif op.startswith('aload'): sim.push(*sim.load_local(_parse_slot(op, operand)))

    # ── store ──
    elif op.startswith('istore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, I32)
    elif op.startswith('lstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, I64)
    elif op.startswith('fstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F32)
    elif op.startswith('dstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F64)
    elif op.startswith('astore'):
        e, ty = sim.pop()
        sim.store_local(_parse_slot(op, operand), e, ty)

    # ── 整数算术 ──
    elif op == 'iadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_add({render_expr(b)})"), I32)
    elif op == 'isub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_sub({render_expr(b)})"), I32)
    elif op == 'imul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_mul({render_expr(b)})"), I32)
    elif op == 'idiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), I32)
    elif op == 'irem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}%{render_expr(b)})"), I32)
    elif op == 'ineg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_neg()"), I32)
    elif op == 'ishl':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}<<({render_expr(b)}&0x1f))"), I32)
    elif op == 'ishr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}>>(({render_expr(b)}&0x1f)))"), I32)
    elif op == 'iushr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"(({render_expr(a)} as u32>>({render_expr(b)}&0x1f)) as i32)"), I32)
    elif op == 'iand':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}&{render_expr(b)})"), I32)
    elif op == 'ior':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}|{render_expr(b)})"), I32)
    elif op == 'ixor':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}^{render_expr(b)})"), I32)
    elif op == 'iinc':
        parts = operand.replace(',', ' ').split()
        slot, delta = int(parts[0]), int(parts[1])
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", I32, True))
        if delta >= 0: sim.emit(RawStmt(f"{name} = {name}.wrapping_add({delta}i32);"))
        else:          sim.emit(RawStmt(f"{name} = {name}.wrapping_sub({-delta}i32);"))
    elif op == 'ladd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_add({render_expr(b)})"), I64)
    elif op == 'lsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_sub({render_expr(b)})"), I64)
    elif op == 'lmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_mul({render_expr(b)})"), I64)
    elif op == 'ldiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), I64)
    elif op == 'fadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F32)
    elif op == 'fsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F32)
    elif op == 'fmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F32)
    elif op == 'fdiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F32)
    elif op == 'dadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F64)
    elif op == 'dsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F64)
    elif op == 'dmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F64)
    elif op == 'ddiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F64)

    # ── 类型转换 ──
    elif op == 'i2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'i2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'i2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    elif op == 'l2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'f2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'd2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'd2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'f2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)

    # ── dup / pop ──
    elif op == 'dup':
        if sim.stack: sim.stack.append(sim.stack[-1])
    elif op == 'dup_x1':
        if len(sim.stack) >= 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'pop':
        if sim.stack:
            e_expr, _ = sim.pop()
            e = render_expr(e_expr)
            # 只有在弹出的是有副作用的表达式时才发出 let _ = ...
            if any(c in e for c in ['(', 'push', 'insert']):
                sim.emit(RawStmt(f"let _ = {e};"))
    elif op == 'pop2':
        sim.pop()
        if sim.stack: sim.pop()

    # ── 对象创建 ──
    elif op == 'new':
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        sim.push(NewPendingExpr(raw), RsNamed(raw.split('/')[-1]))

    # ── invokespecial（含构造器）──
    elif op == 'invokespecial':
        _gen_invokespecial(sim, comment, class_name)

    # ── 字段访问 ──
    elif op == 'getfield':
        obj_expr, obj_ty = sim.pop()
        if comment:
            _, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc) if fdesc else 'JvmObject'
            sim.push(RawExpr(f"{render_expr(obj_expr)}.{fname}.get()"), RsNamed(ftype))
        else:
            sim.push(RawExpr(f"{render_expr(obj_expr)}.field"), I32)

    elif op == 'putfield':
        val_expr, _ = sim.pop()
        obj_expr, obj_ty = sim.pop()
        if comment:
            _, fname, _ = _parse_field_ref(comment)
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.{fname}.set({render_expr(val_expr)});"))
        else:
            sim.emit(RawStmt(f"/* putfield {render_expr(val_expr)} */"))

    elif op == 'getstatic':
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if field_name:
            ty_str = jvm_to_rust(descriptor) if descriptor else 'JvmObject'
            sim.push(StaticFieldRef(cls, field_name, RsNamed(ty_str)), RsNamed(ty_str))
        else:
            sim.push(RawExpr(f"/* getstatic {comment} */"), RsNamed('JvmObject'))
    elif op == 'putstatic':
        val_expr, _ = sim.pop()
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        cls_simple = cls.split('/')[-1] if cls else 'UnknownClass'
        sim.emit(RawStmt(f"{cls_simple}::{field_name}({render_expr(val_expr)});"))

    # ── 数组 ──
    elif op == 'newarray':
        count_expr, _ = sim.pop()
        elem_t, zero = NEWARRAY_TYPES.get(operand.strip(), ('i32', '0i32'))
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<{elem_t}> = vec![{zero}; {render_expr(count_expr)} as usize];"))
        sim.push(Var(v), RsGeneric('Vec', [RsNamed(elem_t)]))
    elif op == 'anewarray':
        count_expr, _ = sim.pop()
        cls = short_cls(comment) or 'JvmObject'
        elem_t = jvm_to_rust(f'L{cls};') if cls != 'JvmObject' else 'JvmObject'
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<{elem_t}> = Vec::with_capacity({render_expr(count_expr)} as usize);"))
        sim.push(Var(v), RsGeneric('Vec', [RsNamed(elem_t)]))
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [render_expr(sim.pop()[0]) for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];"))
        sim.push(Var(v), RsGeneric('Vec', [RsGeneric('Vec', [I32])]))
    elif op in ('iastore', 'bastore', 'sastore', 'castore',
                'lastore', 'fastore', 'dastore', 'aastore'):
        val_expr, _ = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}[{render_expr(idx_expr)} as usize] = {render_expr(val_expr)};"))
    elif op in ('iaload', 'baload', 'saload', 'caload'):
        idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"{render_expr(arr_expr)}[{render_expr(idx_expr)} as usize]"), I32)
    elif op in ('laload', 'faload', 'daload'):
        idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        ty = {'l': I64, 'f': F32, 'd': F64}.get(op[0], I32)
        sim.push(RawExpr(f"{render_expr(arr_expr)}[{render_expr(idx_expr)} as usize]"), ty)
    elif op == 'aaload':
        idx_expr, _ = sim.pop(); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        elem_ty_str = arr_ty_str[4:-1] if arr_ty_str.startswith('Vec<') else 'JvmObject'
        sim.push(RawExpr(f"{render_expr(arr_expr)}[{render_expr(idx_expr)} as usize].clone()"), RsNamed(elem_ty_str))
    elif op == 'arraylength':
        arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(arr_expr)}.len() as i32)"), I32)

    # ── 方法调用 ──
    elif op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name)
    elif op in ('invokevirtual', 'invokeinterface'):
        _gen_invokevirtual(sim, comment, class_name, registry=registry)

    # ── 返回 ──
    elif op == 'return':
        sim.emit(RawStmt('return Ok(());'))
    elif op in ('ireturn', 'lreturn', 'freturn', 'dreturn'):
        e_expr, _ = sim.pop()
        sim.emit(RawStmt(f"return Ok({render_expr(e_expr)});"))
    elif op == 'areturn':
        e_expr, _ = sim.pop()
        sim.emit(RawStmt(f"return Ok({render_expr(e_expr)});"))

    # ── 控制流（循环由 method.py 处理，此处跳过）──
    elif op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        pass

    # ── checkcast / instanceof ──
    elif op == 'checkcast': pass
    elif op == 'instanceof': sim.push(Lit('true'), BOOL)

    # ── invokedynamic ──
    elif op == 'invokedynamic':
        if comment and 'makeConcatWithConstants' in comment:
            _gen_string_concat(sim, comment)
        else:
            sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))

    # ── 杂项 ──
    elif op in ('nop', 'wide'): pass
    elif op == 'athrow':
        e_expr, _ = sim.pop()
        sim.emit(RawStmt(f'panic!("{{}}", /* {render_expr(e_expr)} */);'))
    else:
        sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))


# ── invoke 生成器 ─────────────────────────────────────────────────

def _gen_string_concat(sim: StackSim, comment: str):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for _ in range(len(params)):
        e_expr, _ = sim.pop()
        args.insert(0, render_expr(e_expr))

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


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str):
    if '<init>' not in comment and '"<init>"' not in comment:
        cls, _, _, _ = parse_method_ref(comment)
        if not cls or cls == 'Object':
            return  # super() 忽略
        _gen_invokestatic(sim, comment, class_name)
        return

    cls, _, params, _ = parse_method_ref(comment)
    args = []
    for _ in range(len(params)):
        e_expr, _ = sim.pop()
        args.insert(0, render_expr(e_expr))
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        raw_cls = obj_expr.class_name.rsplit('/', 1)[-1]
        raw_cls = short_cls(raw_cls) or raw_cls

        if raw_cls in _COLL_IR_TYPES:
            rust_ty_node, init_expr = _COLL_IR_TYPES[raw_cls]
            rust_ty = render_type(rust_ty_node)
        elif raw_cls in ('StringBuilder', 'StringBuffer'):
            rust_ty_node = RsNamed('String')
            rust_ty      = 'String'
            init_expr    = 'String::new()'
        elif raw_cls and '/' not in raw_cls:
            # 用户类：new()? 返回 Result<Self>
            init_expr    = f"{raw_cls}::new({', '.join(args)})?"
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
        sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str):
    for skip in BOXING_SKIP_STATIC:
        if skip in comment:
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a_expr, _ = sim.pop()
        a = render_expr(a_expr)
        sim.push(Lit(f'String::from_owned(format!("{{}}", {a}))'), RsNamed('String'))
        return

    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for _ in range(len(params)):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        args.insert(0, f"&{e}" if ty.startswith('Vec<') else e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）

    if cls in ('Math', 'java/lang/Math'):
        fn_map = {
            'abs': 'abs', 'max': 'max', 'min': 'min',
            'sqrt': 'sqrt', 'pow': 'powf',
            'floor': 'floor', 'ceil': 'ceil',
            'log': 'ln', 'log10': 'log10',
        }
        rust_fn = fn_map.get(mname, mname)
        if mname in ('max', 'min') and len(args) >= 2:
            call = f"({args[0]}).{rust_fn}({args[1]})"
        elif mname == 'pow':
            call = f"({args[0]} as f64).powf({args[1]} as f64)"
        elif mname in ('sqrt', 'floor', 'ceil', 'log', 'log10'):
            call = f"({args[0]} as f64).{rust_fn}()"
        else:
            call = f"({args[0]}).abs()"
    elif cls is None or cls == class_name:
        call = f"Self::{mname}({', '.join(args)})"
        needs_q = True
    elif cls and '/' in cls:
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        call = f"{cls}::{mname}({', '.join(args)})"
        needs_q = True

    q = '?' if needs_q else ''
    rust_ret = jvm_to_rust(ret)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{call}{q};"))
    else:
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
        sim.push(Var(v), RsNamed(rust_ret))


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for _ in range(len(params)):
        e_expr, _ = sim.pop()
        args.insert(0, render_expr(e_expr))
    obj_expr, obj_ty_node = sim.pop()
    obj_e = render_expr(obj_expr)
    obj_ty = render_type(obj_ty_node)

    # 拆箱：identity
    if mname in UNBOX_VIRTUAL:
        sim.push(obj_expr, obj_ty_node)
        return

    # StringBuilder.append / toString（保留：Rust 引用语义特殊处理）
    if cls in ('StringBuilder', 'StringBuffer') or (obj_ty == 'String' and mname in ('append', 'toString')):
        if mname == 'append':
            a = args[0] if args else 'String::new()'
            sim.emit(RawStmt(f"{obj_e}.append(&{a})?;"))
            sim.push(RawExpr(obj_e), RsNamed('String'))
            return
        if mname == 'toString':
            sim.push(RawExpr(obj_e), RsNamed('String'))
            return
        if mname == '<init>':
            return

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{obj_e}.{mname}({arg_str})?;"))
    else:
        v = sim.fresh()
        # 不写出显式类型注解，让 Rust 从方法返回类型推断（避免 JDK 类型擦除问题）
        sim.emit(RawStmt(f"let {v} = {obj_e}.{mname}({arg_str})?;"))
        sim.push(Var(v), RsNamed(rust_ret))


