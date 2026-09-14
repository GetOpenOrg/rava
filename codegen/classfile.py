"""
二进制 .class 文件解析器（JVMS Chapter 4）。
支持 class file version 45–70（Java 1–Java 26）。
输出与 types.py 中 ClassInfo / ParsedMethod / Instr / FieldInfo 兼容。
"""

import struct
from dataclasses import dataclass, field
from typing import Optional
from .types import ClassInfo, FieldInfo, ParsedMethod, Instr, InnerClassInfo

# ── 常量池 tag ───────────────────────────────────────────────────────────────
TAG_UTF8               = 1
TAG_INTEGER            = 3
TAG_FLOAT              = 4
TAG_LONG               = 5
TAG_DOUBLE             = 6
TAG_CLASS              = 7
TAG_STRING             = 8
TAG_FIELDREF           = 9
TAG_METHODREF          = 10
TAG_INTERFACE_METHODREF= 11
TAG_NAME_AND_TYPE      = 12
TAG_METHOD_HANDLE      = 15
TAG_METHOD_TYPE        = 16
TAG_DYNAMIC            = 17
TAG_INVOKE_DYNAMIC     = 18
TAG_MODULE             = 19
TAG_PACKAGE            = 20

# ── Access flags ─────────────────────────────────────────────────────────────
ACC_PUBLIC    = 0x0001
ACC_PRIVATE   = 0x0002
ACC_PROTECTED = 0x0004
ACC_STATIC    = 0x0008
ACC_FINAL     = 0x0010
ACC_NATIVE    = 0x0100
ACC_INTERFACE = 0x0200
ACC_ABSTRACT  = 0x0400
ACC_SYNTHETIC = 0x1000
ACC_ANNOTATION= 0x2000
ACC_ENUM      = 0x4000

# ── 字节码 opcode 名称表（子集，覆盖常用指令）───────────────────────────────

_OPCODE_NAMES = {
    0x00: 'nop',
    0x01: 'aconst_null',
    0x02: 'iconst_m1', 0x03: 'iconst_0', 0x04: 'iconst_1',
    0x05: 'iconst_2',  0x06: 'iconst_3', 0x07: 'iconst_4', 0x08: 'iconst_5',
    0x09: 'lconst_0',  0x0a: 'lconst_1',
    0x0b: 'fconst_0',  0x0c: 'fconst_1', 0x0d: 'fconst_2',
    0x0e: 'dconst_0',  0x0f: 'dconst_1',
    0x10: 'bipush',    0x11: 'sipush',
    0x12: 'ldc',       0x13: 'ldc_w',   0x14: 'ldc2_w',
    0x15: 'iload',     0x16: 'lload',   0x17: 'fload',  0x18: 'dload',  0x19: 'aload',
    0x1a: 'iload_0',   0x1b: 'iload_1', 0x1c: 'iload_2', 0x1d: 'iload_3',
    0x1e: 'lload_0',   0x1f: 'lload_1', 0x20: 'lload_2', 0x21: 'lload_3',
    0x22: 'fload_0',   0x23: 'fload_1', 0x24: 'fload_2', 0x25: 'fload_3',
    0x26: 'dload_0',   0x27: 'dload_1', 0x28: 'dload_2', 0x29: 'dload_3',
    0x2a: 'aload_0',   0x2b: 'aload_1', 0x2c: 'aload_2', 0x2d: 'aload_3',
    0x2e: 'iaload',    0x2f: 'laload',  0x30: 'faload',  0x31: 'daload',  0x32: 'aaload',
    0x33: 'baload',    0x34: 'caload',  0x35: 'saload',
    0x36: 'istore',    0x37: 'lstore',  0x38: 'fstore',  0x39: 'dstore',  0x3a: 'astore',
    0x3b: 'istore_0',  0x3c: 'istore_1', 0x3d: 'istore_2', 0x3e: 'istore_3',
    0x3f: 'lstore_0',  0x40: 'lstore_1', 0x41: 'lstore_2', 0x42: 'lstore_3',
    0x43: 'fstore_0',  0x44: 'fstore_1', 0x45: 'fstore_2', 0x46: 'fstore_3',
    0x47: 'dstore_0',  0x48: 'dstore_1', 0x49: 'dstore_2', 0x4a: 'dstore_3',
    0x4b: 'astore_0',  0x4c: 'astore_1', 0x4d: 'astore_2', 0x4e: 'astore_3',
    0x4f: 'iastore',   0x50: 'lastore', 0x51: 'fastore', 0x52: 'dastore',
    0x53: 'aastore',   0x54: 'bastore', 0x55: 'castore', 0x56: 'sastore',
    0x57: 'pop',       0x58: 'pop2',
    0x59: 'dup',       0x5a: 'dup_x1', 0x5b: 'dup_x2',
    0x5c: 'dup2',      0x5d: 'dup2_x1', 0x5e: 'dup2_x2',
    0x5f: 'swap',
    0x60: 'iadd',  0x61: 'ladd',  0x62: 'fadd',  0x63: 'dadd',
    0x64: 'isub',  0x65: 'lsub',  0x66: 'fsub',  0x67: 'dsub',
    0x68: 'imul',  0x69: 'lmul',  0x6a: 'fmul',  0x6b: 'dmul',
    0x6c: 'idiv',  0x6d: 'ldiv',  0x6e: 'fdiv',  0x6f: 'ddiv',
    0x70: 'irem',  0x71: 'lrem',  0x72: 'frem',  0x73: 'drem',
    0x74: 'ineg',  0x75: 'lneg',  0x76: 'fneg',  0x77: 'dneg',
    0x78: 'ishl',  0x79: 'lshl',  0x7a: 'ishr',  0x7b: 'lshr',
    0x7c: 'iushr', 0x7d: 'lushr',
    0x7e: 'iand',  0x7f: 'land',
    0x80: 'ior',   0x81: 'lor',
    0x82: 'ixor',  0x83: 'lxor',
    0x84: 'iinc',
    0x85: 'i2l',  0x86: 'i2f',  0x87: 'i2d',
    0x88: 'l2i',  0x89: 'l2f',  0x8a: 'l2d',
    0x8b: 'f2i',  0x8c: 'f2l',  0x8d: 'f2d',
    0x8e: 'd2i',  0x8f: 'd2l',  0x90: 'd2f',
    0x91: 'i2b',  0x92: 'i2c',  0x93: 'i2s',
    0x94: 'lcmp',
    0x95: 'fcmpl', 0x96: 'fcmpg',
    0x97: 'dcmpl', 0x98: 'dcmpg',
    0x99: 'ifeq',  0x9a: 'ifne',  0x9b: 'iflt',
    0x9c: 'ifge',  0x9d: 'ifgt',  0x9e: 'ifle',
    0x9f: 'if_icmpeq', 0xa0: 'if_icmpne', 0xa1: 'if_icmplt',
    0xa2: 'if_icmpge', 0xa3: 'if_icmpgt', 0xa4: 'if_icmple',
    0xa5: 'if_acmpeq', 0xa6: 'if_acmpne',
    0xa7: 'goto',  0xa8: 'jsr',   0xa9: 'ret',
    0xaa: 'tableswitch', 0xab: 'lookupswitch',
    0xac: 'ireturn', 0xad: 'lreturn', 0xae: 'freturn',
    0xaf: 'dreturn', 0xb0: 'areturn', 0xb1: 'return',
    0xb2: 'getstatic',   0xb3: 'putstatic',
    0xb4: 'getfield',    0xb5: 'putfield',
    0xb6: 'invokevirtual', 0xb7: 'invokespecial',
    0xb8: 'invokestatic',  0xb9: 'invokeinterface',
    0xba: 'invokedynamic',
    0xbb: 'new',      0xbc: 'newarray', 0xbd: 'anewarray',
    0xbe: 'arraylength',
    0xbf: 'athrow',
    0xc0: 'checkcast', 0xc1: 'instanceof',
    0xc2: 'monitorenter', 0xc3: 'monitorexit',
    0xc4: 'wide',
    0xc5: 'multianewarray',
    0xc6: 'ifnull', 0xc7: 'ifnonnull',
    0xc8: 'goto_w', 0xc9: 'jsr_w',
}

# newarray 类型代码
_NEWARRAY_TYPES = {4: 'boolean', 5: 'char', 6: 'float', 7: 'double',
                   8: 'byte', 9: 'short', 10: 'int', 11: 'long'}


# ── 解析器主体 ───────────────────────────────────────────────────────────────

class _Reader:
    """字节流顺序读取器。"""

    def __init__(self, data: bytes):
        self._data = data
        self._pos  = 0

    def u1(self) -> int:
        v = self._data[self._pos]
        self._pos += 1
        return v

    def u2(self) -> int:
        v = struct.unpack_from('>H', self._data, self._pos)[0]
        self._pos += 2
        return v

    def u4(self) -> int:
        v = struct.unpack_from('>I', self._data, self._pos)[0]
        self._pos += 4
        return v

    def i2(self) -> int:
        v = struct.unpack_from('>h', self._data, self._pos)[0]
        self._pos += 2
        return v

    def i4(self) -> int:
        v = struct.unpack_from('>i', self._data, self._pos)[0]
        self._pos += 4
        return v

    def read(self, n: int) -> bytes:
        v = self._data[self._pos:self._pos + n]
        self._pos += n
        return v

    def skip(self, n: int):
        self._pos += n

    @property
    def pos(self) -> int:
        return self._pos


def _parse_constant_pool(r: _Reader, count: int) -> list:
    """返回索引从 1 开始的常量池列表（索引 0 为 None）。"""
    pool = [None] * count   # pool[0] 未使用

    i = 1
    while i < count:
        tag = r.u1()
        if tag == TAG_UTF8:
            length = r.u2()
            pool[i] = ('Utf8', r.read(length).decode('utf-8', errors='replace'))
        elif tag == TAG_INTEGER:
            pool[i] = ('Integer', r.i4())
        elif tag == TAG_FLOAT:
            pool[i] = ('Float', struct.unpack_from('>f', r.read(4))[0])
        elif tag == TAG_LONG:
            hi, lo = r.u4(), r.u4()
            pool[i] = ('Long', (hi << 32) | lo)
            pool[i + 1] = None   # long/double 占两个槽
            i += 1
        elif tag == TAG_DOUBLE:
            pool[i] = ('Double', struct.unpack_from('>d', r.read(8))[0])
            pool[i + 1] = None
            i += 1
        elif tag == TAG_CLASS:
            pool[i] = ('Class', r.u2())   # name_index
        elif tag == TAG_STRING:
            pool[i] = ('String', r.u2())  # string_index
        elif tag == TAG_FIELDREF:
            pool[i] = ('Fieldref', r.u2(), r.u2())   # class_idx, nat_idx
        elif tag == TAG_METHODREF:
            pool[i] = ('Methodref', r.u2(), r.u2())
        elif tag == TAG_INTERFACE_METHODREF:
            pool[i] = ('InterfaceMethodref', r.u2(), r.u2())
        elif tag == TAG_NAME_AND_TYPE:
            pool[i] = ('NameAndType', r.u2(), r.u2())  # name_idx, desc_idx
        elif tag == TAG_METHOD_HANDLE:
            pool[i] = ('MethodHandle', r.u1(), r.u2())
        elif tag == TAG_METHOD_TYPE:
            pool[i] = ('MethodType', r.u2())
        elif tag == TAG_DYNAMIC:
            pool[i] = ('Dynamic', r.u2(), r.u2())
        elif tag == TAG_INVOKE_DYNAMIC:
            pool[i] = ('InvokeDynamic', r.u2(), r.u2())
        elif tag in (TAG_MODULE, TAG_PACKAGE):
            pool[i] = ('ModuleOrPackage', r.u2())
        else:
            raise ValueError(f"未知常量池 tag: {tag} at position {r.pos - 1}")
        i += 1

    return pool


def _utf8(pool: list, idx: int) -> str:
    entry = pool[idx]
    if entry is None:
        return ''
    if entry[0] == 'Utf8':
        return entry[1]
    if entry[0] == 'Class':
        return _utf8(pool, entry[1])
    return str(entry)


def _ref_to_str(pool: list, idx: int) -> str:
    """将 Fieldref/Methodref/InterfaceMethodref 转为 'ClassName.name:descriptor' 字符串。"""
    entry = pool[idx]
    if entry is None or entry[0] not in ('Fieldref', 'Methodref', 'InterfaceMethodref'):
        return ''
    class_name = _utf8(pool, pool[entry[1]][1])
    nat = pool[entry[2]]
    name = _utf8(pool, nat[1])
    desc = _utf8(pool, nat[2])
    return f'{class_name}.{name}:{desc}'


def _ldc_str(pool: list, idx: int) -> str:
    """将 ldc 索引解析为 javap 风格注释字符串（用于 comment）。"""
    entry = pool[idx]
    if entry is None:
        return str(idx)
    tag = entry[0]
    if tag == 'Utf8':
        return f'String {entry[1]}'
    if tag == 'String':
        return f'String {_utf8(pool, entry[1])}'
    if tag == 'Integer':
        return f'int {entry[1]}'
    if tag == 'Float':
        return f'float {entry[1]}'
    if tag == 'Long':
        return f'long {entry[1]}'
    if tag == 'Double':
        return f'double {entry[1]}'
    if tag == 'Class':
        return f'class {_utf8(pool, entry[1])}'
    return str(entry)


def _decode_bytecode(code: bytes, pool: list, bootstrap_methods: list[dict] | None = None) -> list[Instr]:
    """将 Code attribute 中的字节码解码为 Instr 列表。"""
    instrs: list[Instr] = []
    pos = 0
    n = len(code)

    while pos < n:
        pc = pos
        op = code[pos]
        pos += 1
        name = _OPCODE_NAMES.get(op, f'unknown_0x{op:02x}')
        operand = None
        comment = None

        if op in (0x10,):  # bipush
            operand = str(struct.unpack_from('>b', code, pos)[0])
            pos += 1
        elif op == 0x11:   # sipush
            operand = str(struct.unpack_from('>h', code, pos)[0])
            pos += 2
        elif op == 0x12:   # ldc
            idx = code[pos]; pos += 1
            operand = str(idx)
            comment = _ldc_str(pool, idx)
        elif op in (0x13, 0x14):  # ldc_w, ldc2_w
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            operand = str(idx)
            comment = _ldc_str(pool, idx)
        elif op in (0x15, 0x16, 0x17, 0x18, 0x19,  # xload
                    0x36, 0x37, 0x38, 0x39, 0x3a):  # xstore
            operand = str(code[pos]); pos += 1
        elif op == 0x84:   # iinc
            idx = code[pos]; pos += 1
            const = struct.unpack_from('>b', code, pos)[0]; pos += 1
            operand = f'{idx} {const}'
        elif op in (0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e,   # ifXX
                    0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4,    # if_icmpXX
                    0xa5, 0xa6,                              # if_acmpXX
                    0xa7, 0xa8,                              # goto, jsr
                    0xc6, 0xc7):                             # ifnull, ifnonnull
            offset = struct.unpack_from('>h', code, pos)[0]; pos += 2
            target = pc + offset
            operand = str(target)
        elif op in (0xc8, 0xc9):   # goto_w, jsr_w
            offset = struct.unpack_from('>i', code, pos)[0]; pos += 4
            operand = str(pc + offset)
        elif op in (0xb2, 0xb3, 0xb4, 0xb5,            # getstatic/putstatic/getfield/putfield
                    0xb6, 0xb7, 0xb8,                   # invokevirtual/invokespecial/invokestatic
                    0xbb, 0xbd, 0xc0, 0xc1):            # new, anewarray, checkcast, instanceof
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            operand = str(idx)
            if op in (0xb2, 0xb3):   # getstatic / putstatic
                comment = 'Field ' + _ref_to_str(pool, idx)
            elif op in (0xb4, 0xb5): # getfield / putfield
                comment = 'Field ' + _ref_to_str(pool, idx)
            elif op in (0xb6, 0xb7, 0xb8):  # invoke*
                comment = 'Method ' + _ref_to_str(pool, idx)
            else:                    # new, anewarray, checkcast, instanceof
                comment = _utf8(pool, pool[idx][1])
        elif op == 0xb9:   # invokeinterface
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            count = code[pos]; pos += 1
            pos += 1  # 0 byte
            operand = str(idx)
            comment = 'InterfaceMethod ' + _ref_to_str(pool, idx)
        elif op == 0xba:   # invokedynamic
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            pos += 2  # two 0 bytes
            operand = str(idx)
            entry = pool[idx]
            if entry and entry[0] == 'InvokeDynamic':
                bsm_idx = entry[1]  # bootstrap_method_attr_index
                nat = pool[entry[2]]
                mname_dyn = _utf8(pool, nat[1])
                mdesc_dyn = _utf8(pool, nat[2])
                comment = f'InvokeDynamic {mname_dyn}:{mdesc_dyn}'
                # 嵌入 makeConcatWithConstants 模板
                bsm_list = bootstrap_methods or []
                if bsm_list and bsm_idx < len(bsm_list):
                    tmpl = bsm_list[bsm_idx].get('template')
                    if tmpl is not None:
                        comment += f' template:{tmpl}'
        elif op == 0xbc:   # newarray
            atype = code[pos]; pos += 1
            operand = _NEWARRAY_TYPES.get(atype, str(atype))
        elif op == 0xc4:   # wide
            wide_op = code[pos]; pos += 1
            wide_name = _OPCODE_NAMES.get(wide_op, f'unknown_0x{wide_op:02x}')
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            if wide_op == 0x84:  # iinc wide
                const = struct.unpack_from('>h', code, pos)[0]; pos += 2
                instrs.append(Instr(pc, f'wide_{wide_name}', f'{idx} {const}'))
                continue
            instrs.append(Instr(pc, f'wide_{wide_name}', str(idx)))
            continue
        elif op == 0xc5:   # multianewarray
            idx = struct.unpack_from('>H', code, pos)[0]; pos += 2
            dims = code[pos]; pos += 1
            operand = f'{idx} {dims}'
            comment = _utf8(pool, pool[idx][1])
        elif op == 0xa9:   # ret
            operand = str(code[pos]); pos += 1
        elif op == 0xaa:   # tableswitch (variable length, padded)
            pad = (4 - (pos % 4)) % 4
            pos += pad
            default = pc + struct.unpack_from('>i', code, pos)[0]; pos += 4
            low  = struct.unpack_from('>i', code, pos)[0]; pos += 4
            high = struct.unpack_from('>i', code, pos)[0]; pos += 4
            offsets = []
            for _ in range(high - low + 1):
                offsets.append(pc + struct.unpack_from('>i', code, pos)[0]); pos += 4
            operand = f'default:{default} low:{low} high:{high} offs:{",".join(str(o) for o in offsets)}'
        elif op == 0xab:   # lookupswitch
            pad = (4 - (pos % 4)) % 4
            pos += pad
            default = pc + struct.unpack_from('>i', code, pos)[0]; pos += 4
            npairs = struct.unpack_from('>i', code, pos)[0]; pos += 4
            pairs = []
            for _ in range(npairs):
                k = struct.unpack_from('>i', code, pos)[0]; pos += 4
                v = pc + struct.unpack_from('>i', code, pos)[0]; pos += 4
                pairs.append(f'{k}:{v}')
            operand = f'default:{default} ' + ' '.join(pairs)

        instrs.append(Instr(pc, name, operand, comment))

    return instrs


def _constant_value_str(pool: list, cv_idx: int) -> str:
    """将 ConstantValue attribute 的常量池索引转换为可读字符串（作为元数据存储）。"""
    entry = pool[cv_idx] if cv_idx < len(pool) else None
    if entry is None:
        return ''
    tag = entry[0]
    if tag == 'Integer':
        return str(entry[1])
    if tag == 'Long':
        # 常量池以无符号 u64 存储，转换为有符号 i64
        val = entry[1]
        if val >= (1 << 63):
            val -= (1 << 64)
        return str(val)
    if tag == 'Float':
        v = entry[1]
        if v != v:           # NaN
            return 'NaN'
        return repr(float(v))
    if tag == 'Double':
        v = entry[1]
        if v != v:
            return 'NaN'
        return repr(v)
    if tag == 'String':
        s = _utf8(pool, entry[1])
        parts = []
        for ch in s:
            cp = ord(ch)
            if ch == '\\':
                parts.append('\\\\')
            elif ch == '"':
                parts.append('\\"')
            elif ch == '\n':
                parts.append('\\n')
            elif ch == '\r':
                parts.append('\\r')
            elif ch == '\t':
                parts.append('\\t')
            elif cp < 0x20 or (0x7f <= cp <= 0x9f):
                # Rust lexer rejects raw control chars in source — escape them
                parts.append(f'\\u{{{cp:04x}}}')
            else:
                parts.append(ch)
        return ''.join(parts)
    return ''


def _parse_bootstrap_methods(data: bytes, pool: list) -> list[dict]:
    """解析 BootstrapMethods attribute，返回 bootstrap method 信息列表。"""
    r = _Reader(data)
    num = r.u2()
    result = []
    for _ in range(num):
        method_ref = r.u2()
        num_args = r.u2()
        arg_indices = [r.u2() for _ in range(num_args)]
        # 对 makeConcatWithConstants 提取第一个 String 参数作为模板
        template = None
        if arg_indices:
            cp_entry = pool[arg_indices[0]] if arg_indices[0] < len(pool) else None
            if cp_entry and cp_entry[0] == 'String':
                template = _utf8(pool, cp_entry[1])
        result.append({'method_ref': method_ref, 'arg_indices': arg_indices, 'template': template})
    return result


def _parse_code_attribute(r: _Reader, pool: list, class_name: str,
                           method_name: str, descriptor: str,
                           access_flags: int,
                           bootstrap_methods: list[dict] | None = None) -> Optional[ParsedMethod]:
    """解析 Code attribute，返回 ParsedMethod。"""
    max_stack  = r.u2()
    max_locals = r.u2()
    code_len   = r.u4()
    code_bytes = r.read(code_len)

    # exception table
    exc_count = r.u2()
    for _ in range(exc_count):
        r.skip(8)  # start_pc, end_pc, handler_pc, catch_type

    # sub-attributes：解析 LocalVariableTable 和 LocalVariableTypeTable，跳过其他
    sub_attr_count = r.u2()
    local_names: dict[int, str] = {}
    local_types: dict[int, str] = {}  # slot → generic Signature string
    for _ in range(sub_attr_count):
        sub_name_idx = r.u2()
        sub_len      = r.u4()
        sub_name     = _utf8(pool, sub_name_idx)
        if sub_name == 'LocalVariableTable':
            sub_data = r.read(sub_len)
            lvt_r    = _Reader(sub_data)
            count    = lvt_r.u2()
            for _ in range(count):
                _start_pc = lvt_r.u2()
                _length   = lvt_r.u2()
                name_idx  = lvt_r.u2()
                _desc_idx = lvt_r.u2()
                slot      = lvt_r.u2()
                name      = _utf8(pool, name_idx)
                if slot not in local_names:  # 取第一个（作用域最广的）
                    local_names[slot] = name
        elif sub_name == 'LocalVariableTypeTable':
            # 格式与 LocalVariableTable 相同，但 descriptor 换成 Signature
            sub_data = r.read(sub_len)
            lvtt_r   = _Reader(sub_data)
            count    = lvtt_r.u2()
            for _ in range(count):
                _start_pc = lvtt_r.u2()
                _length   = lvtt_r.u2()
                name_idx  = lvtt_r.u2()
                sig_idx   = lvtt_r.u2()
                slot      = lvtt_r.u2()
                sig       = _utf8(pool, sig_idx)
                lvtt_name = _utf8(pool, name_idx)
                # 只有当 LVTT 变量名与 LVT 同 slot 名字一致时才采用精确类型，
                # 避免合成迭代器（无 LVT entry）的 LVTT 污染后续复用该 slot 的变量
                if slot not in local_types and local_names.get(slot) == lvtt_name:
                    local_types[slot] = sig
        else:
            r.skip(sub_len)

    instrs = _decode_bytecode(code_bytes, pool, bootstrap_methods or [])

    # 参数数量：从描述符推算（static 方法不含 this）
    from .type_map import parse_descriptor_params
    params = parse_descriptor_params(descriptor)
    is_static = bool(access_flags & ACC_STATIC)
    args_size = len(params) + (0 if is_static else 1)

    return ParsedMethod(
        class_name=class_name,
        name=method_name,
        descriptor=descriptor,
        is_static=is_static,
        locals_count=max_locals,
        args_size=args_size,
        instrs=instrs,
        local_names=local_names,
        local_types=local_types,
    )


def _skip_attribute(r: _Reader):
    r.skip(2)
    length = r.u4()
    r.skip(length)


def parse_class(path: str) -> ClassInfo:
    """解析 .class 文件路径，返回 ClassInfo。"""
    with open(path, 'rb') as f:
        data = f.read()
    return parse_class_bytes(data, path)


def parse_class_bytes(data: bytes, source_path: str = '<bytes>') -> ClassInfo:
    """
    从内存中的 .class 字节解析，返回 ClassInfo。
    支持直接从 jmod 解包后传入，无需临时文件。
    """
    r = _Reader(data)

    magic = r.u4()
    if magic != 0xCAFEBABE:
        raise ValueError(f"{source_path} 不是合法 .class 文件（magic: 0x{magic:08x}）")

    minor = r.u2()
    major = r.u2()
    if major < 45 or major > 70:
        raise ValueError(f"不支持的 class file version: {major}.{minor}（仅支持 Java 1–26）")

    cp_count = r.u2()
    pool = _parse_constant_pool(r, cp_count)

    access_flags    = r.u2()
    this_class_idx  = r.u2()
    super_class_idx = r.u2()

    iface_count   = r.u2()
    iface_indices = [r.u2() for _ in range(iface_count)]

    class_name  = _utf8(pool, pool[this_class_idx][1])
    super_class = _utf8(pool, pool[super_class_idx][1]) if super_class_idx != 0 else ''
    interfaces  = [_utf8(pool, pool[i][1]) for i in iface_indices]

    # ── fields ───────────────────────────────────────────────────────────────
    fields: list[FieldInfo] = []
    field_count = r.u2()
    for _ in range(field_count):
        f_flags = r.u2()
        f_name  = _utf8(pool, r.u2())
        f_desc  = _utf8(pool, r.u2())
        f_generic_sig  = ''
        f_constant_val = ''
        f_deprecated   = False
        attr_count = r.u2()
        for _ in range(attr_count):
            a_name_idx = r.u2()
            a_len = r.u4()
            a_name = _utf8(pool, a_name_idx)
            if a_name == 'Signature':
                sig_idx = struct.unpack_from('>H', r.read(2))[0]
                f_generic_sig = _utf8(pool, sig_idx)
            elif a_name == 'ConstantValue':
                cv_idx = struct.unpack_from('>H', r.read(2))[0]
                f_constant_val = _constant_value_str(pool, cv_idx)
            elif a_name == 'Deprecated':
                f_deprecated = True  # 属性体长度为 0
            else:
                r.skip(a_len)
        fields.append(FieldInfo(
            name=f_name,
            descriptor=f_desc,
            is_static=bool(f_flags & ACC_STATIC),
            access_flags=f_flags,
            generic_signature=f_generic_sig,
            constant_value=f_constant_val,
            is_deprecated=f_deprecated,
        ))

    # ── methods（第一步：收集原始数据，延迟解码字节码）─────────────────────
    # tuple: (flags, name, desc, code_attr_bytes | None, exceptions, generic_sig, is_synthetic)
    raw_methods: list[tuple] = []
    method_count = r.u2()
    for _ in range(method_count):
        m_flags = r.u2()
        m_name  = _utf8(pool, r.u2())
        m_desc  = _utf8(pool, r.u2())
        attr_count = r.u2()
        code_attr_bytes: Optional[bytes] = None
        m_exceptions: list[str] = []
        m_generic_sig  = ''
        m_is_synthetic = bool(m_flags & 0x1000)  # ACC_SYNTHETIC flag
        m_deprecated   = False
        m_parameters: list[tuple[str, int]] = []  # (name, access_flags)
        for _ in range(attr_count):
            attr_name_idx = r.u2()
            attr_len      = r.u4()
            attr_name     = _utf8(pool, attr_name_idx)
            if attr_name == 'Code':
                code_attr_bytes = r.read(attr_len)
            elif attr_name == 'Exceptions':
                exc_data = r.read(attr_len)
                exc_r = _Reader(exc_data)
                n_exc = exc_r.u2()
                for _ in range(n_exc):
                    exc_idx = exc_r.u2()
                    m_exceptions.append(_utf8(pool, pool[exc_idx][1]))
            elif attr_name == 'Signature':
                sig_idx = struct.unpack_from('>H', r.read(2))[0]
                m_generic_sig = _utf8(pool, sig_idx)
            elif attr_name == 'Synthetic':
                m_is_synthetic = True
            elif attr_name == 'Deprecated':
                m_deprecated = True
            elif attr_name == 'MethodParameters':
                mp_data = r.read(attr_len)
                mp_r = _Reader(mp_data)
                n_params = mp_r.u1()
                for _ in range(n_params):
                    p_name_idx   = mp_r.u2()
                    p_access     = mp_r.u2()
                    p_name       = _utf8(pool, p_name_idx) if p_name_idx else ''
                    m_parameters.append((p_name, p_access))
            else:
                r.skip(attr_len)
        raw_methods.append((m_flags, m_name, m_desc, code_attr_bytes,
                            m_exceptions, m_generic_sig, m_is_synthetic,
                            m_deprecated, m_parameters))

    # ── 类级 attribute（含 BootstrapMethods、Signature、SourceFile）────────
    bootstrap_methods: list[dict] = []
    cls_generic_sig  = ''
    cls_source_file  = ''
    cls_deprecated   = False
    cls_inner_classes: list[InnerClassInfo] = []
    cls_attr_count = r.u2()
    for _ in range(cls_attr_count):
        attr_name_idx = r.u2()
        attr_len      = r.u4()
        attr_name     = _utf8(pool, attr_name_idx)
        if attr_name == 'BootstrapMethods':
            attr_data = r.read(attr_len)
            bootstrap_methods = _parse_bootstrap_methods(attr_data, pool)
        elif attr_name == 'Signature':
            sig_idx = struct.unpack_from('>H', r.read(2))[0]
            cls_generic_sig = _utf8(pool, sig_idx)
        elif attr_name == 'SourceFile':
            sf_idx = struct.unpack_from('>H', r.read(2))[0]
            cls_source_file = _utf8(pool, sf_idx)
        elif attr_name == 'Deprecated':
            cls_deprecated = True
        elif attr_name == 'InnerClasses':
            ic_data = r.read(attr_len)
            ic_r = _Reader(ic_data)
            n_ic = ic_r.u2()
            for _ in range(n_ic):
                inner_idx = ic_r.u2()
                outer_idx = ic_r.u2()
                iname_idx = ic_r.u2()
                ic_flags  = ic_r.u2()
                inner_name_str = _utf8(pool, pool[inner_idx][1]) if inner_idx else ''
                outer_name_str = _utf8(pool, pool[outer_idx][1]) if outer_idx else ''
                simple_name    = _utf8(pool, iname_idx) if iname_idx else ''
                cls_inner_classes.append(InnerClassInfo(
                    inner_class  = inner_name_str,
                    outer_class  = outer_name_str,
                    inner_name   = simple_name,
                    access_flags = ic_flags,
                ))
        else:
            r.skip(attr_len)

    # ── methods（第二步：解码字节码，包含 native/abstract 方法）────────────
    from .type_map import parse_descriptor_params
    methods: list[ParsedMethod] = []
    for (m_flags, m_name, m_desc, code_attr_bytes,
         m_exceptions, m_generic_sig, m_is_synthetic,
         m_deprecated, m_parameters) in raw_methods:
        is_native   = bool(m_flags & ACC_NATIVE)
        is_abstract = bool(m_flags & ACC_ABSTRACT)
        is_static   = bool(m_flags & ACC_STATIC)
        if code_attr_bytes is None:
            # native 或 abstract 方法：无 Code attribute，生成空指令列表
            params    = parse_descriptor_params(m_desc)
            args_size = len(params) + (0 if is_static else 1)
            methods.append(ParsedMethod(
                class_name=class_name,
                name=m_name,
                descriptor=m_desc,
                is_static=is_static,
                locals_count=args_size,
                args_size=args_size,
                instrs=[],
                access_flags=m_flags,
                is_native=is_native,
                is_abstract=is_abstract,
                is_synthetic=m_is_synthetic,
                exceptions=m_exceptions,
                generic_signature=m_generic_sig,
                is_deprecated=m_deprecated,
                method_parameters=m_parameters,
            ))
        else:
            sub_r = _Reader(code_attr_bytes)
            parsed = _parse_code_attribute(
                sub_r, pool, class_name, m_name, m_desc, m_flags, bootstrap_methods
            )
            if parsed is not None:
                parsed.access_flags       = m_flags
                parsed.is_native          = is_native
                parsed.is_abstract        = is_abstract
                parsed.is_synthetic       = m_is_synthetic
                parsed.exceptions         = m_exceptions
                parsed.generic_signature  = m_generic_sig
                parsed.is_deprecated      = m_deprecated
                parsed.method_parameters  = m_parameters
                methods.append(parsed)

    return ClassInfo(
        name=class_name,
        fields=fields,
        methods=methods,
        super_class=super_class,
        interfaces=interfaces,
        access_flags=access_flags,
        is_interface=bool(access_flags & ACC_INTERFACE),
        is_abstract=bool(access_flags & ACC_ABSTRACT),
        is_enum=bool(access_flags & ACC_ENUM),
        generic_signature=cls_generic_sig,
        source_file=cls_source_file,
        inner_classes=cls_inner_classes,
        is_deprecated=cls_deprecated,
    )
