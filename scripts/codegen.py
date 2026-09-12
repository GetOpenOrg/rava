#!/usr/bin/env python3
"""
Java .class → Rust 转译器
P0: 整数运算、静态方法、while 循环、System.out.println
P1: 类实例化、字段读写、实例方法
P2: 数组（newarray / iaload / iastore / arraylength）
P3: ArrayList / HashMap / HashSet（自动装箱透明处理）
"""

import subprocess, sys, re, os, argparse
from dataclasses import dataclass, field
from typing import Optional

# ─────────────────────────────────────────────────────────────
# 1. 数据结构
# ─────────────────────────────────────────────────────────────

@dataclass
class Instr:
    offset:  int
    opcode:  str
    operand: Optional[str] = None
    comment: Optional[str] = None

@dataclass
class FieldInfo:
    name:      str
    descriptor: str
    is_static: bool = False

@dataclass
class ParsedMethod:
    class_name:   str
    name:         str
    descriptor:   str
    is_static:    bool
    locals_count: int
    args_size:    int
    instrs:       list

    @property
    def param_types(self): return _parse_params(self.descriptor)
    @property
    def return_type(self): return _parse_return(self.descriptor)
    @property
    def is_constructor(self): return self.name in ('<init>', self.class_name)

@dataclass
class ClassInfo:
    name:    str
    fields:  list   # list[FieldInfo]
    methods: list   # list[ParsedMethod]

# ─────────────────────────────────────────────────────────────
# 2. 类型工具
# ─────────────────────────────────────────────────────────────

def _parse_params(desc: str) -> list:
    m = re.match(r'\(([^)]*)\)', desc)
    return _parse_type_list(m.group(1) if m else '')

def _parse_return(desc: str) -> str:
    m = re.match(r'\([^)]*\)(.*)', desc)
    return m.group(1) if m else 'V'

def _parse_type_list(s: str) -> list:
    types, i = [], 0
    while i < len(s):
        c = s[i]
        if c in 'BCDFIJSZV': types.append(c); i += 1
        elif c == 'L':
            end = s.index(';', i); types.append(s[i:end+1]); i = end+1
        elif c == '[':
            j = i+1
            while j < len(s) and s[j] == '[': j += 1
            if s[j] == 'L': end = s.index(';',j); types.append(s[i:end+1]); i=end+1
            else: types.append(s[i:j+1]); i=j+1
        else: i += 1
    return types

JVM_RUST = {
    'I':'i32','J':'i64','F':'f32','D':'f64','Z':'bool',
    'B':'i8','S':'i16','C':'u16','V':'()',
    'Ljava/lang/String;':'String', 'Ljava/lang/Object;':'JvmObject',
    'Ljava/lang/Integer;':'i32',   'Ljava/lang/Long;':'i64',
    'Ljava/lang/Double;':'f64',    'Ljava/lang/Boolean;':'bool',
    '[I':'Vec<i32>','[J':'Vec<i64>','[F':'Vec<f32>','[D':'Vec<f64>',
    '[Ljava/lang/String;':'Vec<String>',
}

NEWARRAY_TYPES = {
    'int':('i32','0i32'),'long':('i64','0i64'),'float':('f32','0f32'),
    'double':('f64','0f64'),'byte':('i8','0i8'),'short':('i16','0i16'),
    'char':('u16','0u16'),'boolean':('bool','false'),
}

def jvm_to_rust(t: str) -> str:
    return JVM_RUST.get(t, 'JvmObject')

def _sig_type(rt: str) -> str:
    """函数参数中 Vec<T> → &[T]（借用语义匹配 Java 数组传引用）"""
    if rt.startswith('Vec<'): return f"&[{rt[4:-1]}]"
    return rt

def rust_default(rt: str) -> str:
    return {'i32':'0','i64':'0','f32':'0.0','f64':'0.0',
            'bool':'false','String':'String::new()'}.get(rt,'Default::default()')

JDK_CLASSES = {
    'Object','String','Integer','Long','Double','Float','Boolean','Byte','Short','Character',
    'StringBuilder','StringBuffer','Math','System','Arrays','Collections',
    'PrintStream','InputStream','OutputStream','BufferedReader',
    'ArrayList','LinkedList','HashMap','LinkedHashMap','TreeMap',
    'HashSet','TreeSet','List','Map','Set','Collection','Iterator','Optional',
}

def _is_jdk(cls: str) -> bool:
    return cls in JDK_CLASSES or '/' in cls

def _short_cls(cls: str) -> str:
    return cls.split('/')[-1].split('.')[-1] if cls else ''

# ─────────────────────────────────────────────────────────────
# 3. javap 解析
# ─────────────────────────────────────────────────────────────

def parse_javap(text: str, class_name: str) -> ClassInfo:
    lines = text.split('\n')
    fields:  list[FieldInfo]    = []
    methods: list[ParsedMethod] = []

    i = 0
    while i < len(lines):
        line = lines[i]

        # ── 字段声明 ──
        # 特征：下一行有 "descriptor:" 且没有 Code:
        if 'descriptor:' in line:
            desc_m = re.search(r'descriptor:\s*(\S+)', line)
            if desc_m:
                descriptor = desc_m.group(1)
                # 判断 flags
                is_static = False
                for k in range(max(0,i-5), i):
                    if 'ACC_STATIC' in lines[k]: is_static = True

                # 找是字段还是方法：字段的 descriptor 不含 '('
                if '(' not in descriptor:
                    # 找字段名（向上找）
                    for k in range(i-1, max(i-5,-1), -1):
                        nm = re.search(r'(\w+)\s*;', lines[k])
                        if nm:
                            fname = nm.group(1)
                            if fname not in ('class','public','private','protected',
                                             'static','final','int','long','float',
                                             'double','boolean','byte','short','char'):
                                fields.append(FieldInfo(fname, descriptor, is_static))
                                break
                    i += 1
                    continue

                # 是方法
                method_name = None
                for k in range(i-1, max(i-6,-1), -1):
                    nm = re.search(r'(\w+|<init>|<clinit>)\s*\(', lines[k])
                    if nm:
                        method_name = nm.group(1)
                        if 'ACC_STATIC' in lines[k]: is_static = True
                        break
                    if 'ACC_STATIC' in lines[k]: is_static = True

                if not method_name or method_name in ('if','for','while','class','new'):
                    i += 1; continue

                # flags 行
                for k in range(i, min(i+3,len(lines))):
                    if 'ACC_STATIC' in lines[k]: is_static = True

                # Code:
                code_idx = None
                for k in range(i, min(i+20, len(lines))):
                    if re.match(r'\s+Code:', lines[k]):
                        code_idx = k; break
                if code_idx is None:
                    i += 1; continue

                locals_count, args_size = 1, 1
                sl = lines[code_idx+1] if code_idx+1 < len(lines) else ''
                lm = re.search(r'locals=(\d+)', sl)
                am = re.search(r'args_size=(\d+)', sl)
                if lm: locals_count = int(lm.group(1))
                if am: args_size    = int(am.group(1))

                instrs = []
                j = code_idx + 2
                while j < len(lines):
                    im = re.match(r'\s+(\d+):\s+(\w+)(.*)', lines[j])
                    if im:
                        offset = int(im.group(1)); opcode = im.group(2)
                        rest = im.group(3).strip(); comment = None
                        if '//' in rest:
                            rest, comment = rest.split('//',1)
                            rest=rest.strip(); comment=comment.strip()
                        instrs.append(Instr(offset=offset,opcode=opcode,
                                            operand=rest.strip() or None,comment=comment))
                        j += 1
                    elif re.match(r'\s+(LineNumberTable|StackMapTable|frame_type|offset_delta)',lines[j]):
                        break
                    elif lines[j].strip() == '': j += 1
                    else:
                        j += 1
                        if instrs: break

                if instrs:
                    methods.append(ParsedMethod(
                        class_name=class_name, name=method_name,
                        descriptor=descriptor, is_static=is_static,
                        locals_count=locals_count, args_size=args_size,
                        instrs=instrs))
                i = j; continue
        i += 1

    return ClassInfo(name=class_name, fields=fields, methods=methods)


# ─────────────────────────────────────────────────────────────
# 4. 控制流分析（while 循环检测）
# ─────────────────────────────────────────────────────────────

@dataclass
class LoopInfo:
    start_idx:   int
    end_idx:     int
    cond_idx:    int
    exit_offset: int

def find_loops(instrs: list) -> list:
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    loops = []
    for i, ins in enumerate(instrs):
        if ins.opcode == 'goto' and ins.operand:
            target_off = int(ins.operand)
            if target_off < ins.offset:
                start_idx = off2idx.get(target_off)
                if start_idx is None: continue
                cond_idx = exit_off = None
                for j in range(start_idx, i+1):
                    op = instrs[j].opcode
                    if (op.startswith('if') or op.startswith('if_')) and instrs[j].operand:
                        off = int(instrs[j].operand)
                        if off > ins.offset:
                            cond_idx = j; exit_off = off; break
                if cond_idx is not None:
                    loops.append(LoopInfo(start_idx,i,cond_idx,exit_off))
    return loops

def cmp_op(opcode: str, a: str, b: str) -> str:
    ops2 = {'if_icmpeq':'==','if_icmpne':'!=','if_icmplt':'<',
            'if_icmpge':'>=','if_icmple':'<=','if_icmpgt':'>'}
    if opcode in ops2: return f"{a} {ops2[opcode]} {b}"
    ops1 = {'ifeq':f"{a}==0i32",'ifne':f"{a}!=0i32",'iflt':f"{a}<0i32",
            'ifge':f"{a}>=0i32",'ifle':f"{a}<=0i32",'ifgt':f"{a}>0i32",
            'ifnull':f"{a}.is_none()",'ifnonnull':f"!{a}.is_none()"}
    return ops1.get(opcode, f"/* {opcode} */ true")

# ─────────────────────────────────────────────────────────────
# 5. JVM 栈模拟器
# ─────────────────────────────────────────────────────────────

class StackSim:
    def __init__(self, param_rust_types: list, is_static: bool, class_name: str):
        self.stack = []
        self._ctr  = 0
        self.locals: dict = {}
        self.stmts: list  = []
        self.is_static = is_static
        self.class_name = class_name

        if is_static:
            for slot, rt in enumerate(param_rust_types):
                self.locals[slot] = (f"arg_{slot}", rt, False)
        else:
            # slot 0 = this
            this_ty = f"Rc<RefCell<{class_name}>>" if class_name else "JvmObject"
            self.locals[0] = ("this", this_ty, False)
            for slot, rt in enumerate(param_rust_types):
                self.locals[slot+1] = (f"arg_{slot}", rt, False)

    def fresh(self, prefix='_t') -> str:
        v = f"{prefix}{self._ctr}"; self._ctr += 1; return v

    def push(self, expr: str, ty: str = 'i32'):
        self.stack.append((expr, ty))

    def pop(self) -> tuple:
        return self.stack.pop() if self.stack else ('/* UNDERFLOW */', 'i32')

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

# ─────────────────────────────────────────────────────────────
# 6. 单条指令翻译
# ─────────────────────────────────────────────────────────────

BOXING_SKIP_STATIC = {
    'java/lang/Integer.valueOf', 'java/lang/Long.valueOf',
    'java/lang/Double.valueOf',  'java/lang/Float.valueOf',
    'java/lang/Boolean.valueOf', 'Integer.valueOf', 'Long.valueOf',
    'Double.valueOf', 'Float.valueOf', 'Boolean.valueOf',
}
UNBOX_VIRTUAL = {
    'intValue','longValue','doubleValue','floatValue','booleanValue',
    'byteValue','shortValue',
}

JDK_COLL_TYPES = {
    'ArrayList': ('Vec<i32>', 'Vec::new()'),
    'java/util/ArrayList': ('Vec<i32>', 'Vec::new()'),
    'HashMap': ('HashMap<String,i32>', 'HashMap::new()'),
    'java/util/HashMap': ('HashMap<String,i32>', 'HashMap::new()'),
    'HashSet': ('HashSet<i32>', 'HashSet::new()'),
    'java/util/HashSet': ('HashSet<i32>', 'HashSet::new()'),
}


def _parse_slot(op: str, operand: str) -> int:
    if '_' in op: return int(op.split('_')[-1])
    return int(operand.strip()) if operand else 0

def _parse_method_ref(comment: str) -> tuple:
    comment = comment.strip()
    for p in ('Method ', 'InterfaceMethod '):
        if comment.startswith(p): comment = comment[len(p):]
    # 把 "<init>" / "<clinit>"（带引号或不带）统一替换为合法标识符
    comment = comment.replace('"<init>"', '__init__').replace('"<clinit>"', '__clinit__')
    comment = comment.replace('<init>', '__init__').replace('<clinit>', '__clinit__')
    m = re.match(r'(?:([^.]+)\.)?(\w+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m: return (None, comment, [], 'V')
    raw_cls = m.group(1); mname = m.group(2); desc = m.group(3)
    # 还原为 <init>
    mname = mname.replace('__init__', '<init>').replace('__clinit__', '<clinit>')
    if raw_cls: raw_cls = raw_cls.split('/')[-1].split('.')[-1]
    return (raw_cls, mname, _parse_params(desc), _parse_return(desc))

def _make_jdk_constructor(cls: str) -> tuple:
    short = cls.split('/')[-1]
    if short in JDK_COLL_TYPES: return JDK_COLL_TYPES[short]
    if short == 'StringBuilder': return ('String', 'String::new()')
    return ('JvmObject', 'JvmObject::default()')


def _sim_instr(ins: Instr, sim: StackSim, cp: dict, class_name: str):
    op = ins.opcode; operand = ins.operand or ''; comment = ins.comment or ''

    # ── 整型常量 ──
    if   op == 'iconst_m1': sim.push('-1i32')
    elif op.startswith('iconst_'): sim.push(f"{op[-1]}i32")
    elif op in ('lconst_0','lconst_1'): sim.push(f"{op[-1]}i64",'i64')
    elif op in ('fconst_0','fconst_1','fconst_2'): sim.push(f"{op[-1]}f32",'f32')
    elif op in ('dconst_0','dconst_1'): sim.push(f"{op[-1]}f64",'f64')
    elif op == 'bipush': sim.push(f"{operand}i32")
    elif op == 'sipush': sim.push(f"{operand}i32")
    elif op == 'ldc':
        if operand.startswith('"'):
            sim.push(f"{operand}.to_string()", 'String')
        else:
            # 可能是 #N（常量池引用）或直接值
            # 从 comment 解析
            if comment.startswith('String '):
                s = comment[7:].strip()
                sim.push(f'"{s}".to_string()', 'String')
            elif comment.startswith('int '): sim.push(comment[4:].strip() + 'i32')
            elif comment.startswith('float '): sim.push(comment[6:].strip() + 'f32','f32')
            elif comment.startswith('long '): sim.push(comment[5:].strip() + 'i64','i64')
            elif comment.startswith('double '): sim.push(comment[7:].strip() + 'f64','f64')
            else: sim.push(f"{operand}i32")
    elif op in ('ldc2_w','ldc_w'):
        if comment.startswith('long '): sim.push(comment[5:].strip()+'i64','i64')
        elif comment.startswith('double '): sim.push(comment[7:].strip()+'f64','f64')
        else: sim.push(f"{operand}i32")

    # ── load ──
    elif op.startswith('iload'): sim.push(*sim.load_local(_parse_slot(op,operand)))
    elif op.startswith('lload'): e,_ = sim.load_local(_parse_slot(op,operand)); sim.push(e,'i64')
    elif op.startswith('fload'): e,_ = sim.load_local(_parse_slot(op,operand)); sim.push(e,'f32')
    elif op.startswith('dload'): e,_ = sim.load_local(_parse_slot(op,operand)); sim.push(e,'f64')
    elif op.startswith('aload'): sim.push(*sim.load_local(_parse_slot(op,operand)))

    # ── store ──
    elif op.startswith('istore'):
        e,_ = sim.pop(); sim.store_local(_parse_slot(op,operand), e, 'i32')
    elif op.startswith('lstore'):
        e,_ = sim.pop(); sim.store_local(_parse_slot(op,operand), e, 'i64')
    elif op.startswith('fstore'):
        e,_ = sim.pop(); sim.store_local(_parse_slot(op,operand), e, 'f32')
    elif op.startswith('dstore'):
        e,_ = sim.pop(); sim.store_local(_parse_slot(op,operand), e, 'f64')
    elif op.startswith('astore'):
        e, ty = sim.pop()
        sim.store_local(_parse_slot(op,operand), e, ty or 'JvmObject')

    # ── 整数算术 ──
    elif op=='iadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_add({b})")
    elif op=='isub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_sub({b})")
    elif op=='imul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_mul({b})")
    elif op=='idiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})")
    elif op=='irem': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}%{b})")
    elif op=='ineg': a,_=sim.pop();sim.push(f"({a}).wrapping_neg()")
    elif op=='ishl': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}<<({b}&0x1f))")
    elif op=='ishr': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}>>({b}&0x1f))")
    elif op=='iushr': b,_=sim.pop();a,_=sim.pop();sim.push(f"(({a} as u32>>({b}&0x1f)) as i32)")
    elif op=='iand': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}&{b})")
    elif op=='ior':  b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}|{b})")
    elif op=='ixor': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}^{b})")
    elif op=='iinc':
        parts = operand.replace(',',' ').split(); slot=int(parts[0]); delta=int(parts[1])
        name,_,_ = sim.locals.get(slot,(f"local_{slot}",'i32',True))
        if delta>=0: sim.emit(f"    {name} = {name}.wrapping_add({delta}i32);")
        else:        sim.emit(f"    {name} = {name}.wrapping_sub({-delta}i32);")
    elif op=='ladd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_add({b})","i64")
    elif op=='lsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_sub({b})","i64")
    elif op=='lmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}).wrapping_mul({b})","i64")
    elif op=='ldiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})","i64")
    elif op=='fadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}+{b})","f32")
    elif op=='fsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}-{b})","f32")
    elif op=='fmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}*{b})","f32")
    elif op=='fdiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})","f32")
    elif op=='dadd': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}+{b})","f64")
    elif op=='dsub': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}-{b})","f64")
    elif op=='dmul': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}*{b})","f64")
    elif op=='ddiv': b,_=sim.pop();a,_=sim.pop();sim.push(f"({a}/{b})","f64")

    # ── 类型转换 ──
    elif op=='i2l': a,_=sim.pop();sim.push(f"({a} as i64)","i64")
    elif op=='i2f': a,_=sim.pop();sim.push(f"({a} as f32)","f32")
    elif op=='i2d': a,_=sim.pop();sim.push(f"({a} as f64)","f64")
    elif op=='l2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op=='f2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op=='d2i': a,_=sim.pop();sim.push(f"({a} as i32)")
    elif op=='d2f': a,_=sim.pop();sim.push(f"({a} as f32)","f32")
    elif op=='f2d': a,_=sim.pop();sim.push(f"({a} as f64)","f64")

    # ── dup / pop ──
    elif op == 'dup':
        if sim.stack: sim.stack.append(sim.stack[-1])
    elif op == 'dup_x1':
        if len(sim.stack) >= 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'pop':
        if sim.stack:
            e,_ = sim.pop()
            # 有副作用的表达式不能丢弃
            if any(c in e for c in ['(','push','insert','emit','todo']):
                sim.emit(f"    let _ = {e};")
    elif op == 'pop2':
        sim.pop()
        if sim.stack: sim.pop()

    # ── 对象创建：new ──
    elif op == 'new':
        # comment 格式：'class TestP1' 或 'class java/util/ArrayList'
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        cls = _short_cls(raw) or raw
        sim.push(f"__new__{cls}__", f"__pending__{cls}")

    # ── invokespecial（含构造器）──
    elif op == 'invokespecial':
        _gen_invokespecial(sim, comment, class_name)

    # ── 字段访问 ──
    elif op == 'getfield':
        obj, obj_ty = sim.pop()
        # comment: "Field TestP1.x:I" 或 "Field x:I"
        fm = re.search(r'Field\s+(?:\w+\.)?(\w+):(\S+)', comment)
        if fm:
            fname = fm.group(1); ftype = jvm_to_rust(fm.group(2))
            if 'Rc<RefCell<' in obj_ty or obj == 'this':
                sim.push(f"{obj}.borrow().{fname}", ftype)
            else:
                sim.push(f"{obj}.{fname}", ftype)
        else:
            sim.push(f"{obj}.field", 'i32')

    elif op == 'putfield':
        val, vty = sim.pop(); obj, obj_ty = sim.pop()
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
        if 'System.out' in comment:
            sim.push('__stdout__', 'PrintStream')
        elif 'System.err' in comment:
            sim.push('__stderr__', 'PrintStream')
        else:
            sim.push(f"/* getstatic {comment} */", 'JvmObject')
    elif op == 'putstatic':
        sim.pop()

    # ── 数组 ──
    elif op == 'newarray':
        count, _ = sim.pop()
        elem_t, zero = NEWARRAY_TYPES.get(operand.strip(), ('i32','0i32'))
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<{elem_t}> = vec![{zero}; {count} as usize];")
        sim.push(v, f"Vec<{elem_t}>")
    elif op == 'anewarray':
        count, _ = sim.pop()
        cls = _short_cls(comment) or 'JvmObject'
        elem_t = jvm_to_rust(f'L{cls};') if cls != 'JvmObject' else 'JvmObject'
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<{elem_t}> = Vec::with_capacity({count} as usize);")
        sim.push(v, f"Vec<{elem_t}>")
    elif op == 'multianewarray':
        # 简化：两维 int[][]
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [sim.pop()[0] for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(f"    let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];")
        sim.push(v, "Vec<Vec<i32>>")
    elif op in ('iastore','bastore','sastore','castore'):
        val,_=sim.pop(); idx,_=sim.pop(); arr,_=sim.pop()
        sim.emit(f"    {arr}[{idx} as usize] = {val};")
    elif op in ('lastore','fastore','dastore','aastore'):
        val,_=sim.pop(); idx,_=sim.pop(); arr,_=sim.pop()
        sim.emit(f"    {arr}[{idx} as usize] = {val};")
    elif op in ('iaload','baload','saload','caload'):
        idx,_=sim.pop(); arr,_=sim.pop(); sim.push(f"{arr}[{idx} as usize]")
    elif op in ('laload','faload','daload'):
        idx,_=sim.pop(); arr,_=sim.pop()
        ty = {'l':'i64','f':'f32','d':'f64'}.get(op[0],'i32')
        sim.push(f"{arr}[{idx} as usize]", ty)
    elif op == 'aaload':
        idx,_=sim.pop(); arr,arr_ty=sim.pop()
        elem_ty = arr_ty.replace('Vec<','').rstrip('>') if arr_ty.startswith('Vec<') else 'JvmObject'
        sim.push(f"{arr}[{idx} as usize].clone()", elem_ty)
    elif op == 'arraylength':
        arr,_ = sim.pop(); sim.push(f"({arr}.len() as i32)")

    # ── 方法调用 ──
    elif op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name)
    elif op in ('invokevirtual','invokeinterface'):
        _gen_invokevirtual(sim, comment, class_name)

    # ── 返回 ──
    elif op == 'return': sim.emit('    return;')
    elif op in ('ireturn','lreturn','freturn','dreturn'):
        e,_ = sim.pop(); sim.emit(f"    return {e};")
    elif op == 'areturn':
        e,_ = sim.pop(); sim.emit(f"    return {e};")

    # ── 控制流（非循环的前向跳转，暂不完整处理）──
    elif op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        pass  # 由 gen_method_body 的 find_loops 处理

    # ── checkcast / instanceof ──
    elif op == 'checkcast': pass  # 类型擦除后忽略
    elif op == 'instanceof': sim.push('true','bool')  # 简化

    # ── 其他 ──
    elif op in ('nop','wide'): pass
    elif op == 'athrow':
        e,_ = sim.pop(); sim.emit(f"    panic!(\"{{}}\", /* {e} */);")
    else:
        sim.emit(f"    /* TODO: {op} {operand} */")


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str):
    if '<init>' not in comment and '"<init>"' not in comment:
        # 非构造器的 invokespecial（super 方法调用等）
        cls, mname, params, ret = _parse_method_ref(comment)
        if not cls or cls == 'Object': return  # super() 忽略
        _gen_invokestatic(sim, comment, class_name)
        return

    cls, _, params, _ = _parse_method_ref(comment)
    n_args = len(params)
    args = []
    for _ in range(n_args):
        e,_ = sim.pop(); args.insert(0, e)
    obj_e, obj_ty = sim.pop()  # "this" 参数

    if obj_e.startswith('__new__') or '__pending__' in obj_ty:
        raw_cls = obj_e.replace('__new__','').replace('__','') or cls or class_name
        raw_cls = _short_cls(raw_cls) or raw_cls

        if raw_cls in JDK_COLL_TYPES:
            rust_ty, init_expr = JDK_COLL_TYPES[raw_cls]
        elif raw_cls in ('StringBuilder', 'StringBuffer'):
            rust_ty, init_expr = 'String', 'String::new()'
        elif raw_cls and not _is_jdk(raw_cls):
            arg_str = ', '.join(args)
            init_expr = f"{raw_cls}::new({arg_str})"
            rust_ty   = f"Rc<RefCell<{raw_cls}>>"
        else:
            init_expr = f"{raw_cls}::new()"
            rust_ty   = f"Rc<RefCell<{raw_cls}>>"

        # 检查 stack 顶是否还有 dup 的副本
        if sim.stack and sim.stack[-1][0] == obj_e:
            sim.stack[-1] = (init_expr, rust_ty)
        else:
            v = sim.fresh('_obj')
            sim.emit(f"    let mut {v}: {rust_ty} = {init_expr};")
            sim.push(v, rust_ty)
    else:
        # 非 new 模式的 invokespecial（极少见）
        sim.emit(f"    /* invokespecial {comment} */")


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str):
    # 装箱方法透明化：Integer.valueOf(int) → identity
    for skip in BOXING_SKIP_STATIC:
        if skip in comment: return  # 值保留在栈上

    # String.valueOf(int) → .to_string()
    if 'String.valueOf' in comment:
        a, _ = sim.pop()
        sim.push(f"{a}.to_string()", 'String')
        return

    cls, mname, params, ret = _parse_method_ref(comment)
    n_args = len(params)
    args = []
    for _ in range(n_args):
        e, ty = sim.pop()
        args.insert(0, f"&{e}" if ty.startswith('Vec<') else e)

    if cls in ('Math','java/lang/Math'):
        fn_map = {
            'abs':'abs','max':'max','min':'min','sqrt':'sqrt','pow':'powf',
            'floor':'floor','ceil':'ceil','log':'ln','log10':'log10',
        }
        rust_fn = fn_map.get(mname, mname)
        if mname in ('max','min') and args:
            call = f"({args[0]}).{rust_fn}({args[1]})" if len(args)>1 else f"{args[0]}.{rust_fn}()"
        elif mname in ('pow',):
            call = f"({args[0]} as f64).powf({args[1]} as f64)"
        elif mname in ('sqrt','floor','ceil','log','log10'):
            call = f"({args[0]} as f64).{rust_fn}()"
        else:
            call = f"({args[0]}).abs()"
    elif cls is None or cls == class_name:
        call = f"Self::{mname}({', '.join(args)})"
    elif _is_jdk(cls or ''):
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
    cls, mname, params, ret = _parse_method_ref(comment)
    n_args = len(params)
    args = []
    for _ in range(n_args): e,_=sim.pop(); args.insert(0,e)
    obj_e, obj_ty = sim.pop()

    # ── 拆箱方法：intValue / doubleValue 等 ──
    if mname in UNBOX_VIRTUAL:
        sim.push(obj_e, obj_ty)  # identity
        return

    # ── PrintStream.println ──
    if obj_e in ('__stdout__','__stderr__') or obj_ty == 'PrintStream':
        stream = "eprintln!" if 'stderr' in obj_e else "println!"
        if len(args) == 0:
            sim.emit(f'    {stream}();')
        else:
            a = args[0]
            # boolean 打印
            if 'bool' in (sim.stack[-1][1] if sim.stack else '') or 'Z' in (ret or ''):
                sim.emit(f'    {stream}("{{}}", {a});')
            else:
                sim.emit(f'    {stream}("{{}}", {a});')
        return

    # ── StringBuilder / StringBuffer ──
    if cls in ('StringBuilder','StringBuffer') or obj_ty == 'String' and mname in ('append','toString'):
        if mname == 'append':
            a = args[0] if args else '""'
            # 如果是 int 需要转 string
            sim.push(f"{obj_e} + &{a}.to_string()", 'String')
            return
        if mname == 'toString':
            sim.push(obj_e, 'String')
            return
        if mname == '<init>': return

    # ── JDK 集合：ArrayList / Vec ──
    if cls in ('ArrayList','List','Collection') or obj_ty.startswith('Vec<'):
        if mname == 'add':
            a = args[0] if args else '0i32'
            sim.emit(f"    {obj_e}.push({a});")
            sim.push('true', 'bool')
        elif mname == 'get':
            idx = args[0] if args else '0i32'
            elem_ty = obj_ty.replace('Vec<','').rstrip('>') if obj_ty.startswith('Vec<') else 'i32'
            sim.push(f"{obj_e}[{idx} as usize]", elem_ty)
        elif mname == 'size':
            sim.push(f"({obj_e}.len() as i32)")
        elif mname == 'isEmpty':
            sim.push(f"{obj_e}.is_empty()", 'bool')
        elif mname == 'remove':
            idx = args[0] if args else '0i32'
            sim.emit(f"    {obj_e}.remove({idx} as usize);")
        elif mname == 'set':
            idx=args[0]; val=args[1] if len(args)>1 else '0i32'
            sim.emit(f"    {obj_e}[{idx} as usize] = {val};")
        elif mname == 'contains':
            a = args[0] if args else '0i32'
            sim.push(f"{obj_e}.contains(&{a})", 'bool')
        elif mname == 'clear':
            sim.emit(f"    {obj_e}.clear();")
        else:
            sim.emit(f"    /* ArrayList.{mname} */")
        return

    # ── JDK 集合：HashMap ──
    if cls in ('HashMap','LinkedHashMap','TreeMap','Map') or obj_ty.startswith('HashMap<'):
        if mname == 'put':
            k=args[0]; v=args[1] if len(args)>1 else '0i32'
            sim.emit(f"    {obj_e}.insert({k}, {v});")
            sim.push('None::<i32>', 'Option<i32>')
        elif mname == 'get':
            k = args[0] if args else '""'
            sim.push(f"{obj_e}.get(&{k}).copied().unwrap_or(0)", 'i32')
        elif mname == 'size':
            sim.push(f"({obj_e}.len() as i32)")
        elif mname == 'containsKey':
            k = args[0] if args else '""'
            sim.push(f"{obj_e}.contains_key(&{k})", 'bool')
        elif mname == 'containsValue':
            v = args[0] if args else '0i32'
            sim.push(f"{obj_e}.values().any(|x| x == &{v})", 'bool')
        elif mname == 'remove':
            k = args[0] if args else '""'
            sim.emit(f"    {obj_e}.remove(&{k});")
        elif mname == 'isEmpty':
            sim.push(f"{obj_e}.is_empty()", 'bool')
        elif mname == 'getOrDefault':
            k=args[0]; d=args[1] if len(args)>1 else '0i32'
            sim.push(f"{obj_e}.get(&{k}).copied().unwrap_or({d})", 'i32')
        else:
            sim.emit(f"    /* HashMap.{mname} */")
        return

    # ── JDK 集合：HashSet / TreeSet ──
    if cls in ('HashSet','TreeSet','Set') or obj_ty.startswith('HashSet<'):
        if mname == 'add':
            a = args[0] if args else '0i32'
            sim.emit(f"    {obj_e}.insert({a});")
            sim.push('true', 'bool')
        elif mname == 'contains':
            a = args[0] if args else '0i32'
            sim.push(f"{obj_e}.contains(&{a})", 'bool')
        elif mname == 'size':
            sim.push(f"({obj_e}.len() as i32)")
        elif mname == 'remove':
            a = args[0] if args else '0i32'
            sim.emit(f"    {obj_e}.remove(&{a});")
        elif mname == 'isEmpty':
            sim.push(f"{obj_e}.is_empty()", 'bool')
        else:
            sim.emit(f"    /* HashSet.{mname} */")
        return

    # ── 用户类实例方法 ──
    target_cls = cls or class_name
    if not _is_jdk(target_cls):
        arg_str = ', '.join(args)
        call = f"{target_cls}::{mname}(&{obj_e}" + (f", {arg_str}" if arg_str else "") + ")"
        rust_ret = jvm_to_rust(ret)
        if rust_ret == '()':
            sim.emit(f"    {call};")
        else:
            v = sim.fresh()
            sim.emit(f"    let {v}: {rust_ret} = {call};")
            sim.push(v, rust_ret)
        return

    # ── 未识别 JDK 方法 ──
    sim.emit(f"    /* {cls}.{mname}({', '.join(args)}) */")


# ─────────────────────────────────────────────────────────────
# 7. 方法体生成
# ─────────────────────────────────────────────────────────────

def gen_method_body(method: ParsedMethod, class_info: ClassInfo) -> str:
    instrs = method.instrs
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    loops   = find_loops(instrs)
    loop_map = {lp.start_idx: lp for lp in loops}

    param_types     = method.param_types
    rust_param_types= [jvm_to_rust(t) for t in param_types]
    rust_ret        = jvm_to_rust(method.return_type)
    is_ctor         = method.is_constructor
    is_static       = method.is_static

    # ── 构造器特殊处理 ──
    if is_ctor:
        params = [f"arg_{k}: {rt}" for k,rt in enumerate(rust_param_types)]
        ret_ty = f"Rc<RefCell<Self>>"
        sig = f"pub fn new({', '.join(params)}) -> {ret_ty}"
    elif method.name == 'main' and method.descriptor == '([Ljava/lang/String;)V':
        sig = "pub fn main()"
    elif is_static:
        params = [f"arg_{k}: {_sig_type(rt)}" for k,rt in enumerate(rust_param_types)]
        if rust_ret == '()': sig = f"pub fn {method.name}({', '.join(params)})"
        else:                sig = f"pub fn {method.name}({', '.join(params)}) -> {rust_ret}"
    else:
        # 实例方法：第一个参数是 this
        params = [f"this: &Rc<RefCell<Self>>"]
        params += [f"arg_{k}: {rt}" for k,rt in enumerate(rust_param_types)]
        if rust_ret == '()': sig = f"pub fn {method.name}({', '.join(params)})"
        else:                sig = f"pub fn {method.name}({', '.join(params)}) -> {rust_ret}"

    sim = StackSim(rust_param_types, is_static, method.class_name)

    # 构造器：预先初始化 this
    if is_ctor:
        inst_fields = [f for f in (class_info.fields if class_info else []) if not f.is_static]
        if inst_fields:
            field_inits = ', '.join(
                f"{f.name}: {rust_default(jvm_to_rust(f.descriptor))}" for f in inst_fields
            )
            struct_init = f"Self {{ {field_inits} }}"
        else:
            struct_init = "Self {}"
        init_lines = [
            f"    let this: Rc<RefCell<Self>> = Rc::new(RefCell::new({struct_init}));",
        ]
        sim.locals[0] = ('this', f"Rc<RefCell<{method.class_name}>>", False)

    lines = [] if not is_ctor else init_lines

    def _flush(local_sim: StackSim):
        for stmt in local_sim.stmts:
            lines.append(stmt)
        local_sim.stmts.clear()

    i = 0
    while i < len(instrs):
        ins = instrs[i]; op = ins.opcode

        # ── 循环头 ──
        if i in loop_map:
            lp = loop_map[i]
            lines.append("    loop {")

            # 循环条件前的指令
            sub = StackSim(rust_param_types, is_static, method.class_name)
            sub.locals = {k: v for k, v in sim.locals.items()}
            for k in range(lp.start_idx, lp.cond_idx):
                _sim_instr(instrs[k], sub, {}, method.class_name)
            sim.locals = sub.locals
            for s in sub.stmts: lines.append("    " + s.strip())

            # 循环条件
            ci = instrs[lp.cond_idx]
            csub = StackSim(rust_param_types, is_static, method.class_name)
            csub.locals = {k: v for k, v in sim.locals.items()}
            csub.stack  = list(sub.stack)
            if ci.opcode in ('if_icmpeq','if_icmpne','if_icmplt','if_icmpge','if_icmple','if_icmpgt'):
                b,_=csub.pop(); a,_=csub.pop()
                cond = cmp_op(ci.opcode, a, b)
            else:
                a,_=csub.pop(); cond = cmp_op(ci.opcode, a, '')
            lines.append(f"        if {cond} {{ break; }}")

            # 循环体
            body_sim = StackSim(rust_param_types, is_static, method.class_name)
            body_sim.locals = {k: v for k, v in sim.locals.items()}
            for k in range(lp.cond_idx+1, lp.end_idx):
                _sim_instr(instrs[k], body_sim, {}, method.class_name)
            sim.locals = body_sim.locals
            for s in body_sim.stmts: lines.append("    " + s.strip())

            lines.append("    }  // end loop")
            i = lp.end_idx + 1
            continue

        # ── 普通指令 ──
        _sim_instr(ins, sim, {}, method.class_name)
        _flush(sim)
        i += 1

    # 构造器末尾返回 this
    if is_ctor:
        # 去掉最后可能生成的 return;
        while lines and lines[-1].strip() == 'return;':
            lines.pop()
        lines.append("    this")

    body = '\n'.join(lines)
    return f"{sig} {{\n{body}\n}}"


# ─────────────────────────────────────────────────────────────
# 8. Rust 运行时存根
# ─────────────────────────────────────────────────────────────

RUNTIME_MOD = """pub mod math;\npub mod io;\npub mod error;\n"""

RUNTIME_ERROR = """\
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    Custom(String),
}
"""

RUNTIME_MATH = """\
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
"""

RUNTIME_IO = """\
pub fn println_int(v: i32)    { println!("{}", v); }
pub fn println_str(v: &str)   { println!("{}", v); }
pub fn println_long(v: i64)   { println!("{}", v); }
pub fn println_double(v: f64) { println!("{}", v); }
pub fn println_bool(v: bool)  { println!("{}", v); }
pub fn println_empty()        { println!(); }
"""

CARGO_TOML = """\
[package]
name = "java_transpiled"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 3
lto       = true
codegen-units = 1
strip     = "symbols"
"""


def _indent(block: str, n: int = 4) -> str:
    pad = ' ' * n
    return '\n'.join(pad + ln if ln.strip() else '' for ln in block.split('\n'))


def _to_snake(name: str) -> str:
    """PascalCase / camelCase → snake_case（用作 Rust 模块文件名）"""
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    return s.lower()

def _pkg_from_java(java_file: str) -> str:
    """从 .java 源文件读取 package 声明，无则返回空串"""
    try:
        with open(java_file, encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line.startswith('package '):
                    return line[8:].rstrip(';').strip()
                # 遇到非注释、非空行且不是 package 就停止
                if line and not line.startswith('//') and not line.startswith('/*'):
                    if any(line.startswith(k) for k in ('import ','public ','class ','@')):
                        break
    except OSError:
        pass
    return ''

def _gen_class_rs(ci: ClassInfo) -> str:
    """生成单个类的完整 .rs 文件内容"""
    parts: list[str] = []
    parts.append("#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]")
    parts.append("use std::rc::Rc;")
    parts.append("use std::cell::RefCell;")
    parts.append("use std::collections::HashMap;")
    parts.append("use std::collections::HashSet;")
    parts.append("")

    inst_fields = [f for f in ci.fields if not f.is_static]
    has_instance_methods = any(not m.is_static and not m.is_constructor for m in ci.methods)
    if inst_fields:
        field_decls = '\n'.join(f"    pub {f.name}: {jvm_to_rust(f.descriptor)}," for f in inst_fields)
        parts.append(f"#[derive(Debug, Clone, Default)]\npub struct {ci.name} {{\n{field_decls}\n}}\n")
    elif has_instance_methods:
        parts.append(f"#[derive(Debug, Clone, Default)]\npub struct {ci.name};\n")
    else:
        parts.append(f"pub struct {ci.name};\n")

    method_blocks = []
    for m in ci.methods:
        if m.name == '<clinit>':
            continue
        try:
            body = gen_method_body(m, ci)
        except Exception as e:
            body = f"/* codegen error {m.name}: {e} */"
        method_blocks.append(body)

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"impl {ci.name} {{\n{impl_body}\n}}\n")
    return '\n'.join(parts)


def write_cargo_project(out_dir: str, class_infos: list, java_files: list = None):
    src_dir = os.path.join(out_dir, 'src')
    rt_dir  = os.path.join(src_dir, 'java_runtime')
    os.makedirs(rt_dir, exist_ok=True)

    # Runtime stubs
    for fname, content in [('mod.rs',RUNTIME_MOD),('error.rs',RUNTIME_ERROR),
                            ('math.rs',RUNTIME_MATH),('io.rs',RUNTIME_IO)]:
        with open(os.path.join(rt_dir,fname),'w') as f: f.write(content)
    with open(os.path.join(out_dir,'Cargo.toml'),'w') as f: f.write(CARGO_TOML)

    # 1. 提取每个类的包名
    packages: dict[str, str] = {}
    if java_files:
        for jf, ci in zip(java_files, class_infos):
            packages[ci.name] = _pkg_from_java(jf)

    # 2. 计算每个类的文件路径、包路径分段、模块名
    #    layout[class_name] = (abs_file_path, pkg_parts, mod_name)
    layout: dict[str, tuple] = {}
    for ci in class_infos:
        pkg      = packages.get(ci.name, '')
        pkg_parts = pkg.split('.') if pkg else []
        mod_name  = _to_snake(ci.name)
        file_path = os.path.join(src_dir, *pkg_parts, mod_name + '.rs')
        layout[ci.name] = (file_path, pkg_parts, mod_name)

    # 3. 构建模块树：dir → {子模块名}
    #    src/com/example/foo.rs 需要：
    #      src_dir          → 'com'
    #      src_dir/com      → 'example'
    #      src_dir/com/example → 'foo'
    mod_tree: dict[str, set] = {}
    for ci in class_infos:
        _, pkg_parts, mod_name = layout[ci.name]
        parent = src_dir
        for part in pkg_parts:
            mod_tree.setdefault(parent, set()).add(part)
            parent = os.path.join(parent, part)
        mod_tree.setdefault(parent, set()).add(mod_name)

    # 4. 写出每个类的 .rs 文件
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        with open(file_path, 'w') as f:
            f.write(_gen_class_rs(ci))

    # 5. 为中间包目录写 mod.rs（src_dir 本身由 main.rs 承担，跳过）
    for dir_path, children in mod_tree.items():
        if dir_path == src_dir:
            continue
        os.makedirs(dir_path, exist_ok=True)
        mod_rs_path = os.path.join(dir_path, 'mod.rs')
        with open(mod_rs_path, 'w') as f:
            f.write('\n'.join(f"pub mod {c};" for c in sorted(children)) + '\n')

    # 6. 写 main.rs：顶层 mod 声明 + fn main()
    main_class = class_infos[0].name if class_infos else 'Main'
    top_mods   = sorted(mod_tree.get(src_dir, set()))

    main_lines = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "mod java_runtime;",
    ]
    for m in top_mods:
        main_lines.append(f"mod {m};")

    # use 引入入口类
    _, pkg_parts, mod_name = layout[main_class]
    if pkg_parts:
        use_path = '::'.join(pkg_parts + [mod_name, main_class])
    else:
        use_path = f"{mod_name}::{main_class}"
    main_lines.append(f"use {use_path};")
    main_lines.append("")
    main_lines.append(f"fn main() {{ {main_class}::main(); }}")
    main_lines.append("")

    with open(os.path.join(src_dir, 'main.rs'), 'w') as f:
        f.write('\n'.join(main_lines))

    print(f"[codegen] Cargo project → {out_dir}/")


# ─────────────────────────────────────────────────────────────
# 9. 主入口
# ─────────────────────────────────────────────────────────────

def transpile(java_files: list, out_dir: str):
    for jf in java_files:
        if not os.path.exists(jf):
            sys.exit(f"File not found: {jf}")

    # .class 文件统一输出到与 .java 同级的 classes/ 子目录
    src_dir    = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir  = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    # 1. javac 编译
    print(f"[1/4] javac {' '.join(java_files)}")
    r = subprocess.run(['javac', '-d', class_dir] + java_files, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    class_infos = []
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        # 2. javap -verbose
        print(f"[2/4] javap {class_name}")
        r = subprocess.run(['javap','-verbose','-p',class_file],
                           capture_output=True, text=True)

        # 3. 解析
        print(f"[3/4] 解析 {class_name}")
        ci = parse_javap(r.stdout, class_name)
        # 过滤构造器方法名与类名相同的 bug（parse 时可能出现）
        ci.methods = [m for m in ci.methods if m.name != '<clinit>']
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)

    # 4. 生成
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_infos, java_files)
    print("\n✓ 完成。运行方式：")
    print(f"  cd {out_dir} && cargo run --release")


if __name__ == '__main__':
    ap = argparse.ArgumentParser(description='Java → Rust 转译器')
    ap.add_argument('java_files', nargs='+', help='.java 文件列表')
    ap.add_argument('--out', default='output', help='输出目录（默认 output）')
    args = ap.parse_args()
    transpile(args.java_files, args.out)
