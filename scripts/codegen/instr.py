"""
JVM 字节码指令 → Rust 语句转换。
每条指令操作 StackSim 的栈与语句列表。
"""

import re
from .stack import StackSim
from .type_map import (
    jvm_to_rust, sig_type, is_jdk, short_cls,
    NEWARRAY_TYPES, JDK_COLL_TYPES,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL,
    parse_descriptor_params, parse_descriptor_return,
)
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

    # <init> / <clinit> 含尖括号，先替换为合法标识符再用正则
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


# ── 主分发函数 ────────────────────────────────────────────────────

def sim_instr(ins: Instr, sim: StackSim, class_name: str):
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 整型常量 ──
    if   op == 'iconst_m1':                      sim.push('-1i32')
    elif op.startswith('iconst_'):               sim.push(f"{op[-1]}i32")
    elif op in ('lconst_0', 'lconst_1'):         sim.push(f"{op[-1]}i64", 'i64')
    elif op in ('fconst_0', 'fconst_1', 'fconst_2'): sim.push(f"{op[-1]}f32", 'f32')
    elif op in ('dconst_0', 'dconst_1'):         sim.push(f"{op[-1]}f64", 'f64')
    elif op == 'bipush':                         sim.push(f"{operand}i32")
    elif op == 'sipush':                         sim.push(f"{operand}i32")
    elif op == 'ldc':
        if operand.startswith('"'):
            sim.push(f"{operand}.to_string()", 'String')
        elif comment.startswith('String '):
            sim.push(f'"{comment[7:].strip()}".to_string()', 'String')
        elif comment.startswith('int '):    sim.push(comment[4:].strip() + 'i32')
        elif comment.startswith('float '): sim.push(comment[6:].strip() + 'f32', 'f32')
        elif comment.startswith('long '):  sim.push(comment[5:].strip() + 'i64', 'i64')
        elif comment.startswith('double '): sim.push(comment[7:].strip() + 'f64', 'f64')
        else: sim.push(f"{operand}i32")
    elif op in ('ldc2_w', 'ldc_w'):
        if comment.startswith('long '):   sim.push(comment[5:].strip() + 'i64', 'i64')
        elif comment.startswith('double '): sim.push(comment[7:].strip() + 'f64', 'f64')
        else: sim.push(f"{operand}i32")

    # ── load ──
    elif op.startswith('iload'): sim.push(*sim.load_local(_parse_slot(op, operand)))
    elif op.startswith('lload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, 'i64')
    elif op.startswith('fload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, 'f32')
    elif op.startswith('dload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, 'f64')
    elif op.startswith('aload'): sim.push(*sim.load_local(_parse_slot(op, operand)))

    # ── store ──
    elif op.startswith('istore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, 'i32')
    elif op.startswith('lstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, 'i64')
    elif op.startswith('fstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, 'f32')
    elif op.startswith('dstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, 'f64')
    elif op.startswith('astore'):
        e, ty = sim.pop()
        sim.store_local(_parse_slot(op, operand), e, ty or 'JvmObject')

    # ── 整数算术 ──
    elif op == 'iadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_add({b})")
    elif op == 'isub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_sub({b})")
    elif op == 'imul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_mul({b})")
    elif op == 'idiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})")
    elif op == 'irem': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}%{b})")
    elif op == 'ineg': a,_=sim.pop();sim.push(f"({a}).wrapping_neg()")
    elif op == 'ishl': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}<<({b}&0x1f))")
    elif op == 'ishr': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}>>({b}&0x1f))")
    elif op == 'iushr': b,_=sim.pop();a,_=sim.pop();sim.push(f"(({a} as u32>>({b}&0x1f)) as i32)")
    elif op == 'iand': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}&{b})")
    elif op == 'ior':  b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}|{b})")
    elif op == 'ixor': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}^{b})")
    elif op == 'iinc':
        parts = operand.replace(',', ' ').split()
        slot, delta = int(parts[0]), int(parts[1])
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", 'i32', True))
        if delta >= 0: sim.emit(f"    {name} = {name}.wrapping_add({delta}i32);")
        else:          sim.emit(f"    {name} = {name}.wrapping_sub({-delta}i32);")
    elif op == 'ladd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_add({b})", 'i64')
    elif op == 'lsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_sub({b})", 'i64')
    elif op == 'lmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_mul({b})", 'i64')
    elif op == 'ldiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})", 'i64')
    elif op == 'fadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}+{b})", 'f32')
    elif op == 'fsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}-{b})", 'f32')
    elif op == 'fmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}*{b})", 'f32')
    elif op == 'fdiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})", 'f32')
    elif op == 'dadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}+{b})", 'f64')
    elif op == 'dsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}-{b})", 'f64')
    elif op == 'dmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}*{b})", 'f64')
    elif op == 'ddiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})", 'f64')

    # ── 类型转换 ──
    elif op == 'i2l': a,_=sim.pop();sim.push(f"({a} as i64)", 'i64')
    elif op == 'i2f': a,_=sim.pop();sim.push(f"({a} as f32)", 'f32')
    elif op == 'i2d': a,_=sim.pop();sim.push(f"({a} as f64)", 'f64')
    elif op == 'l2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op == 'f2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op == 'd2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op == 'd2f': a,_=sim.pop();sim.push(f"({a} as f32)", 'f32')
    elif op == 'f2d': a,_=sim.pop();sim.push(f"({a} as f64)", 'f64')

    # ── dup / pop ──
    elif op == 'dup':
        if sim.stack: sim.stack.append(sim.stack[-1])
    elif op == 'dup_x1':
        if len(sim.stack) >= 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'pop':
        if sim.stack:
            e, _ = sim.pop()
            if any(c in e for c in ['(', 'push', 'insert']):
                sim.emit(f"    let _ = {e};")
    elif op == 'pop2':
        sim.pop()
        if sim.stack: sim.pop()

    # ── 对象创建 ──
    elif op == 'new':
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        cls = short_cls(raw) or raw
        sim.push(f"__new__{cls}__", f"__pending__{cls}")

    # ── invokespecial（含构造器）──
    elif op == 'invokespecial':
        _gen_invokespecial(sim, comment, class_name)

    # ── 字段访问 ──
    elif op == 'getfield':
        obj, obj_ty = sim.pop()
        fm = re.search(r'Field\s+(?:\w+\.)?(\w+):(\S+)', comment)
        if fm:
            fname = fm.group(1)
            ftype = jvm_to_rust(fm.group(2))
            if 'Rc<RefCell<' in obj_ty or obj == 'this':
                sim.push(f"{obj}.borrow().{fname}", ftype)
            else:
                sim.push(f"{obj}.{fname}", ftype)
        else:
            sim.push(f"{obj}.field", 'i32')

    elif op == 'putfield':
        val, _ = sim.pop()
        obj, obj_ty = sim.pop()
        fm = re.search(r'Field\s+(?:\w+\.)?(\w+):(\S+)', comment)
        if fm:
            fname = fm.group(1)
            if 'Rc<RefCell<' in obj_ty or obj == 'this':
                sim.emit(f"    {obj}.borrow_mut().{fname} = {val};")
            else:
                sim.emit(f"    {obj}.{fname} = {val};")
        else:
            sim.emit(f"    /* putfield {val} */")

    elif op == 'getstatic':
        if 'System.out' in comment:   sim.push('__stdout__', 'PrintStream')
        elif 'System.err' in comment: sim.push('__stderr__', 'PrintStream')
        else: sim.push(f"/* getstatic {comment} */", 'JvmObject')
    elif op == 'putstatic':
        sim.pop()

    # ── 数组 ──
    elif op == 'newarray':
        count, _ = sim.pop()
        elem_t, zero = NEWARRAY_TYPES.get(operand.strip(), ('i32', '0i32'))
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<{elem_t}> = vec![{zero}; {count} as usize];")
        sim.push(v, f"Vec<{elem_t}>")
    elif op == 'anewarray':
        count, _ = sim.pop()
        cls = short_cls(comment) or 'JvmObject'
        elem_t = jvm_to_rust(f'L{cls};') if cls != 'JvmObject' else 'JvmObject'
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<{elem_t}> = Vec::with_capacity({count} as usize);")
        sim.push(v, f"Vec<{elem_t}>")
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [sim.pop()[0] for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];")
        sim.push(v, "Vec<Vec<i32>>")
    elif op in ('iastore', 'bastore', 'sastore', 'castore',
                'lastore', 'fastore', 'dastore', 'aastore'):
        val, _ = sim.pop(); idx, _ = sim.pop(); arr, _ = sim.pop()
        sim.emit(f"    {arr}[{idx} as usize] = {val};")
    elif op in ('iaload', 'baload', 'saload', 'caload'):
        idx, _ = sim.pop(); arr, _ = sim.pop()
        sim.push(f"{arr}[{idx} as usize]")
    elif op in ('laload', 'faload', 'daload'):
        idx, _ = sim.pop(); arr, _ = sim.pop()
        ty = {'l': 'i64', 'f': 'f32', 'd': 'f64'}.get(op[0], 'i32')
        sim.push(f"{arr}[{idx} as usize]", ty)
    elif op == 'aaload':
        idx, _ = sim.pop(); arr, arr_ty = sim.pop()
        elem_ty = arr_ty[4:-1] if arr_ty.startswith('Vec<') else 'JvmObject'
        sim.push(f"{arr}[{idx} as usize].clone()", elem_ty)
    elif op == 'arraylength':
        arr, _ = sim.pop()
        sim.push(f"({arr}.len() as i32)")

    # ── 方法调用 ──
    elif op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name)
    elif op in ('invokevirtual', 'invokeinterface'):
        _gen_invokevirtual(sim, comment, class_name)

    # ── 返回 ──
    elif op == 'return': sim.emit('    return;')
    elif op in ('ireturn', 'lreturn', 'freturn', 'dreturn'):
        e, _ = sim.pop(); sim.emit(f"    return {e};")
    elif op == 'areturn':
        e, _ = sim.pop(); sim.emit(f"    return {e};")

    # ── 控制流（循环由 method.py 处理，此处跳过）──
    elif op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        pass

    # ── checkcast / instanceof ──
    elif op == 'checkcast': pass
    elif op == 'instanceof': sim.push('true', 'bool')

    # ── 杂项 ──
    elif op in ('nop', 'wide'): pass
    elif op == 'athrow':
        e, _ = sim.pop(); sim.emit(f'    panic!("{{}}", /* {e} */);')
    else:
        sim.emit(f"    /* TODO: {op} {operand} */")


# ── invoke 生成器 ─────────────────────────────────────────────────

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
        e, _ = sim.pop()
        args.insert(0, e)
    obj_e, obj_ty = sim.pop()

    if obj_e.startswith('__new__') or '__pending__' in obj_ty:
        raw_cls = obj_e.replace('__new__', '').replace('__', '') or cls or class_name
        raw_cls = short_cls(raw_cls) or raw_cls

        if raw_cls in JDK_COLL_TYPES:
            rust_ty, init_expr = JDK_COLL_TYPES[raw_cls]
        elif raw_cls in ('StringBuilder', 'StringBuffer'):
            rust_ty, init_expr = 'String', 'String::new()'
        elif raw_cls and not is_jdk(raw_cls):
            init_expr = f"{raw_cls}::new({', '.join(args)})"
            rust_ty   = f"Rc<RefCell<{raw_cls}>>"
        else:
            init_expr = f"{raw_cls}::new()"
            rust_ty   = f"Rc<RefCell<{raw_cls}>>"

        if sim.stack and sim.stack[-1][0] == obj_e:
            sim.stack[-1] = (init_expr, rust_ty)
        else:
            v = sim.fresh('_obj')
            sim.emit(f"    let mut {v}: {rust_ty} = {init_expr};")
            sim.push(v, rust_ty)
    else:
        sim.emit(f"    /* invokespecial {comment} */")


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str):
    for skip in BOXING_SKIP_STATIC:
        if skip in comment:
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a, _ = sim.pop()
        sim.push(f"{a}.to_string()", 'String')
        return

    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for _ in range(len(params)):
        e, ty = sim.pop()
        args.insert(0, f"&{e}" if ty.startswith('Vec<') else e)

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
    elif is_jdk(cls or ''):
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        call = f"{cls}::{mname}({', '.join(args)})"

    rust_ret = jvm_to_rust(ret)
    if rust_ret == '()':
        sim.emit(f"    {call};")
    else:
        v = sim.fresh()
        sim.emit(f"    let {v}: {rust_ret} = {call};")
        sim.push(v, rust_ret)


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str):
    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for _ in range(len(params)):
        e, _ = sim.pop()
        args.insert(0, e)
    obj_e, obj_ty = sim.pop()

    # 拆箱：identity
    if mname in UNBOX_VIRTUAL:
        sim.push(obj_e, obj_ty)
        return

    # System.out.println / System.err.println
    if obj_e in ('__stdout__', '__stderr__') or obj_ty == 'PrintStream':
        macro = "eprintln!" if 'stderr' in obj_e else "println!"
        if not args:
            sim.emit(f"    {macro}();")
        else:
            sim.emit(f'    {macro}("{{}}", {args[0]});')
        return

    # StringBuilder.append / toString
    if cls in ('StringBuilder', 'StringBuffer') or (obj_ty == 'String' and mname in ('append', 'toString')):
        if mname == 'append':
            a = args[0] if args else '""'
            sim.push(f"{obj_e} + &{a}.to_string()", 'String')
            return
        if mname == 'toString':
            sim.push(obj_e, 'String')
            return
        if mname == '<init>':
            return

    # ArrayList / List
    if cls in ('ArrayList', 'List', 'Collection') or obj_ty.startswith('Vec<'):
        _dispatch_list(sim, obj_e, obj_ty, mname, args)
        return

    # HashMap / Map
    if cls in ('HashMap', 'LinkedHashMap', 'TreeMap', 'Map') or obj_ty.startswith('HashMap<'):
        _dispatch_map(sim, obj_e, mname, args)
        return

    # HashSet / Set
    if cls in ('HashSet', 'TreeSet', 'Set') or obj_ty.startswith('HashSet<'):
        _dispatch_set(sim, obj_e, mname, args)
        return

    # 用户类实例方法
    target_cls = cls or class_name
    if not is_jdk(target_cls):
        arg_str = ', '.join(args)
        tail    = f", {arg_str}" if arg_str else ""
        call    = f"{target_cls}::{mname}(&{obj_e}{tail})"
        rust_ret = jvm_to_rust(ret)
        if rust_ret == '()':
            sim.emit(f"    {call};")
        else:
            v = sim.fresh()
            sim.emit(f"    let {v}: {rust_ret} = {call};")
            sim.push(v, rust_ret)
        return

    sim.emit(f"    /* {cls}.{mname}({', '.join(args)}) */")


def _dispatch_list(sim: StackSim, obj: str, obj_ty: str, mname: str, args: list):
    if mname == 'add':
        sim.emit(f"    {obj}.push({args[0] if args else '0i32'});")
        sim.push('true', 'bool')
    elif mname == 'get':
        idx = args[0] if args else '0i32'
        elem_ty = obj_ty[4:-1] if obj_ty.startswith('Vec<') else 'i32'
        sim.push(f"{obj}[{idx} as usize]", elem_ty)
    elif mname == 'size':
        sim.push(f"({obj}.len() as i32)")
    elif mname == 'isEmpty':
        sim.push(f"{obj}.is_empty()", 'bool')
    elif mname == 'remove':
        sim.emit(f"    {obj}.remove({args[0] if args else '0i32'} as usize);")
    elif mname == 'set':
        idx = args[0]; val = args[1] if len(args) > 1 else '0i32'
        sim.emit(f"    {obj}[{idx} as usize] = {val};")
    elif mname == 'contains':
        sim.push(f"{obj}.contains(&{args[0] if args else '0i32'})", 'bool')
    elif mname == 'clear':
        sim.emit(f"    {obj}.clear();")
    else:
        sim.emit(f"    /* ArrayList.{mname} */")


def _dispatch_map(sim: StackSim, obj: str, mname: str, args: list):
    if mname == 'put':
        k = args[0]; v = args[1] if len(args) > 1 else '0i32'
        sim.emit(f"    {obj}.insert({k}, {v});")
        sim.push('None::<i32>', 'Option<i32>')
    elif mname == 'get':
        k = args[0] if args else '""'
        sim.push(f"{obj}.get(&{k}).copied().unwrap_or(0)", 'i32')
    elif mname == 'size':
        sim.push(f"({obj}.len() as i32)")
    elif mname == 'containsKey':
        sim.push(f"{obj}.contains_key(&{args[0] if args else '\"\"'})", 'bool')
    elif mname == 'containsValue':
        sim.push(f"{obj}.values().any(|x| x == &{args[0] if args else '0i32'})", 'bool')
    elif mname == 'remove':
        sim.emit(f"    {obj}.remove(&{args[0] if args else '\"\"'});")
    elif mname == 'isEmpty':
        sim.push(f"{obj}.is_empty()", 'bool')
    elif mname == 'getOrDefault':
        k = args[0]; d = args[1] if len(args) > 1 else '0i32'
        sim.push(f"{obj}.get(&{k}).copied().unwrap_or({d})", 'i32')
    else:
        sim.emit(f"    /* HashMap.{mname} */")


def _dispatch_set(sim: StackSim, obj: str, mname: str, args: list):
    if mname == 'add':
        sim.emit(f"    {obj}.insert({args[0] if args else '0i32'});")
        sim.push('true', 'bool')
    elif mname == 'contains':
        sim.push(f"{obj}.contains(&{args[0] if args else '0i32'})", 'bool')
    elif mname == 'size':
        sim.push(f"({obj}.len() as i32)")
    elif mname == 'remove':
        sim.emit(f"    {obj}.remove(&{args[0] if args else '0i32'});")
    elif mname == 'isEmpty':
        sim.push(f"{obj}.is_empty()", 'bool')
    else:
        sim.emit(f"    /* HashSet.{mname} */")
