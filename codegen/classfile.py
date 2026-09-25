"""
二进制 .class 文件解析器（JVMS Chapter 4）。
支持 class file version 45–70（Java 1–Java 26）。
输出与 types.py 中 ClassInfo / ParsedMethod / Instr / FieldInfo 兼容。
"""

import struct
from dataclasses import dataclass, field
from typing import Optional
from .types import ClassInfo, FieldInfo, ParsedMethod, Instr, InnerClassInfo
from .constants import BOXED_CLASS_BY_DESC, CLASS_CLASS

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

# S-17: typeSwitch 标签中 primitive type pattern 的 Class 常量名（primitive 描述符）
# → wrapper 类二进制名（`case int x` 的运行时判定等价于 java/lang/Integer）
_PRIM_CLASS_TO_WRAPPER = BOXED_CLASS_BY_DESC


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


def _decode_mutf8(b: bytes) -> str:
    """解码常量池的 Modified UTF-8（JVMS §4.4.7）。

    与标准 UTF-8 的差异：U+0000 编码为两字节 `C0 80`；增补字符（非 BMP）
    以 UTF-16 代理对形式逐半编码（CESU-8 式两个 3 字节序列），不存在
    4 字节序列。标准 `bytes.decode('utf-8')` 会把代理对判为非法字节，
    `errors='replace'` 把 😀 的 6 字节吞成 6 个 U+FFFD——非 BMP 字面量
    在转译期即丢码点（S-19 #3）。

    解码出 UTF-16 码元序列后把合法代理对合并为码点；孤立代理与残缺
    序列按 UTF-8 惯例替换为 U+FFFD（javac 产出的常量池不会出现，防御
    性兜底，同时保证产出可写入 UTF-8 文本的合法 str）。"""
    units: list[int] = []
    i, n = 0, len(b)
    while i < n:
        c = b[i]
        if c < 0x80:
            units.append(c)
            i += 1
        elif (c >> 5) == 0b110 and i + 1 < n and (b[i + 1] >> 6) == 0b10:
            units.append(((c & 0x1f) << 6) | (b[i + 1] & 0x3f))
            i += 2
        elif ((c >> 4) == 0b1110 and i + 2 < n
              and (b[i + 1] >> 6) == 0b10 and (b[i + 2] >> 6) == 0b10):
            units.append(((c & 0x0f) << 12) | ((b[i + 1] & 0x3f) << 6) | (b[i + 2] & 0x3f))
            i += 3
        else:
            units.append(0xFFFD)
            i += 1
    out: list[str] = []
    j = 0
    while j < len(units):
        u = units[j]
        if 0xD800 <= u <= 0xDBFF and j + 1 < len(units) and 0xDC00 <= units[j + 1] <= 0xDFFF:
            out.append(chr(0x10000 + ((u - 0xD800) << 10) + (units[j + 1] - 0xDC00)))
            j += 2
        elif 0xD800 <= u <= 0xDFFF:
            out.append('\ufffd')
            j += 1
        else:
            out.append(chr(u))
            j += 1
    return ''.join(out)


def _parse_constant_pool(r: _Reader, count: int) -> list:
    """返回索引从 1 开始的常量池列表（索引 0 为 None）。"""
    pool = [None] * count   # pool[0] 未使用

    i = 1
    while i < count:
        tag = r.u1()
        if tag == TAG_UTF8:
            length = r.u2()
            pool[i] = ('Utf8', _decode_mutf8(r.read(length)))
        elif tag == TAG_INTEGER:
            pool[i] = ('Integer', r.i4())
        elif tag == TAG_FLOAT:
            pool[i] = ('Float', struct.unpack_from('>f', r.read(4))[0])
        elif tag == TAG_LONG:
            hi, lo = r.u4(), r.u4()
            val = (hi << 32) | lo
            if val >= (1 << 63):   # 转换为有符号 i64
                val -= (1 << 64)
            pool[i] = ('Long', val)
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
                # 嵌入 makeConcatWithConstants 模板 或 Arch-3 lambda 信息
                bsm_list = bootstrap_methods or []
                if bsm_list and bsm_idx < len(bsm_list):
                    bsm = bsm_list[bsm_idx]
                    tmpl = bsm.get('template')
                    if tmpl is not None:
                        comment += f' template:{tmpl}'
                    else:
                        # Arch-3: lambda — 嵌入实现方法引用 + SAM 类型描述符
                        impl = bsm.get('impl_method')
                        sam  = bsm.get('sam_type')
                        if impl:
                            comment += f' impl:{impl}'
                        if sam:
                            comment += f' samtype:{sam}'
                    # S-17: typeSwitch — 嵌入 case 标签序列（kind:值，逗号分隔；
                    # 值内 % , 空白百分号编码，comment 按空格分词）
                    labels = bsm.get('switch_labels')
                    if labels is not None:
                        toks = []
                        for lkind, lval in labels:
                            enc = (lval.replace('%', '%25')
                                       .replace(',', '%2C')
                                       .replace(' ', '%20'))
                            toks.append(f'{lkind}:{enc}')
                        comment += ' tslabels:' + ','.join(toks)
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
        return str(entry[1])
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


# ── RuntimeVisibleAnnotations / AnnotationDefault（JVMS §4.7.16 / §4.7.22）──
#
# 注解元数据（反射 L3 段 1）的解析与编码：element_value → `tag:载荷`。
# 该编码是三侧（attrs 发射 → build.rs 造表 → java_runtime::annotation 解码）
# 透传的单一形态——载荷内出现的分隔符（`; # = "`、反斜杠、控制字符）一律
# 以 `\` 前缀转义（控制字符用字母助记 / `\uXXXX`）；数组（tag a）各元素为
# `tag:载荷` 以 `;` 连接；嵌套注解（tag @）为 `binary#name=tag:载荷#...`
#（与外层同构递归）。转义只在载荷层发生——元素名/类型名是标识符，裸传。

_ANNO_RESERVED = set('%;#="\\\n\r\t')


def _anno_esc(s: str) -> str:
    r"""载荷百分号编码：保留字符（分隔符 / 反斜杠 / 引号 / 控制字符）→ %XX。

    属性行字符串必须是合法 Rust 字面量（java_class! 块被 proc-macro tokenize，
    反斜杠转义会炸词法）；百分号编码只产生 [0-9A-F] 与 %，字面量安全。
    """
    out = []
    for ch in s:
        if ch in _ANNO_RESERVED or ord(ch) < 0x20:
            b = ch.encode('utf-8')
            out.append(''.join(f'%{byte:02X}' for byte in b))
        else:
            out.append(ch)
    return ''.join(out)


def _anno_unesc(s: str) -> str:
    import urllib.parse
    return urllib.parse.unquote(s, errors='strict')


def _anno_desc_to_bin(desc: str) -> str:
    """注解类型描述符 `Lx/y/Z;` → binary name（非 L 形态原样返回，防御）。"""
    if desc.startswith('L') and desc.endswith(';'):
        return desc[1:-1]
    return desc


def _anno_split(s: str, sep: str) -> list:
    """按 sep 切分（载荷内保留字符已百分号编码——sep 只以分隔符身份出现）。"""
    return s.split(sep)


def _parse_element_value(r: '_Reader', pool: list) -> tuple:
    """element_value → (tag, 编码载荷)。未支持形态 tag='x'。

    载荷：基本类型为十进制文本；String 已转义；enum 为 `binary#常量名`；
    class 为（已转义的）JVM 描述符；数组为 `tag:载荷;tag:载荷;...`；
    嵌套注解为 `binary#name=tag:载荷#...`。
    """
    tag = chr(r.u1())
    if tag == 's':
        idx = r.u2()
        s = pool[idx][1] if idx < len(pool) else ''
        return (tag, _anno_esc(str(s)))
    if tag in 'BCISZJFD':
        idx = r.u2()
        if tag in 'BCIS':
            v = pool[idx][1] if idx < len(pool) else 0
            return (tag, str(v))
        if tag == 'Z':
            v = pool[idx][1] if idx < len(pool) else 0
            return (tag, 'true' if v else 'false')
        if tag == 'J':
            v = pool[idx][1] if idx < len(pool) else 0
            return (tag, str(v))
        v = pool[idx][1] if idx < len(pool) else 0.0
        return (tag, repr(v))
    if tag == 'e':
        type_idx = r.u2()
        type_name = _utf8(pool, type_idx) if type_idx else ''
        const_idx = r.u2()
        const_name = _utf8(pool, const_idx) if const_idx else ''
        return (tag, f'{_anno_desc_to_bin(type_name)}#{const_name}')
    if tag == 'c':
        idx = r.u2()
        cls_desc = _utf8(pool, idx) if idx else ''
        # class 字面量载荷保持 JVM 描述符形态（`Lx/Y;` / `[I`）——运行时经
        # class_for_descriptor 还原 Class（数组元素类型的原生形态）
        return (tag, _anno_esc(cls_desc))
    if tag == '@':
        # 复合载荷整体再编码一层：内层分隔符（# = ;）不与外层切分冲突，
        # 解码端先整体百分号解码再递归拆分
        return (tag, _anno_esc(_parse_annotation_body(r, pool)))
    if tag == '[':
        n = r.u2()
        parts = []
        for _ in range(n):
            t, v = _parse_element_value(r, pool)
            parts.append(f'{t}:{v}')
        return (tag, _anno_esc(';'.join(parts)))
    return ('x', '')


def _parse_annotation_body(r: '_Reader', pool: list) -> str:
    """annotation 结构体（不含条目数前缀）→ `binary#name=tag:载荷#...`。"""
    type_idx = r.u2()
    type_bin = _anno_desc_to_bin(_utf8(pool, type_idx) if type_idx else '')
    parts = [_anno_esc(type_bin)]
    n = r.u2()
    for _ in range(n):
        name_idx = r.u2()
        name = _utf8(pool, name_idx) if name_idx else ''
        tag, val = _parse_element_value(r, pool)
        parts.append(f'{name}={tag}:{val}')
    return '#'.join(parts)


def _parse_annotations_attr(data: bytes, pool: list) -> list:
    """RuntimeVisibleAnnotations 属性体 → list[AnnoInfo]（声明序）。"""
    from .types import AnnoInfo, AnnoElem
    r = _Reader(data)
    n = r.u2()
    out = []
    for _ in range(n):
        text = _parse_annotation_body(r, pool)
        segs = _anno_split(text, '#')
        type_bin = _anno_unesc(segs[0])
        info = AnnoInfo(type_bin=type_bin)
        for seg in segs[1:]:
            name, _, rest = seg.partition('=')
            tag, _, payload = rest.partition(':')
            info.elements.append(AnnoElem(name=name, tag=tag, value=payload))
        out.append(info)
    return out


def encode_annotations(annos: list) -> str:
    """list[AnnoInfo] → 属性行载荷文本（`binary#name=tag:载荷#...;...`）。

    与 _parse_annotation_body 的编码同一形态（AnnoInfo → 文本），供 attrs
    属性行发射使用；`;` 分隔多条注解。
    """
    parts = []
    for a in annos:
        segs = [_anno_esc(a.type_bin)]
        for e in a.elements:
            segs.append(f'{e.name}={e.tag}:{e.value}')
        parts.append('#'.join(segs))
    return ';'.join(parts)


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
        # Arch-3: 对 LambdaMetafactory 提取 SAM 类型 + 实现方法信息
        sam_type = None    # args[0]: SAM 方法类型描述符
        impl_method = None  # args[1]: 实现方法 ClassName.name:desc
        # S-17: 对 SwitchBootstraps.typeSwitch 提取 case 标签序列
        # 每项 (kind, value)：kind 'c' = Class 标签（value 为二进制名，primitive
        # 描述符已归一化为 wrapper 类），'s' = String 常量，'i' = Integer 常量，
        # '?' = 其余形态（EnumDesc 的 CONSTANT_Dynamic 等，翻译侧归类不支持）
        switch_labels = None
        bsm_entry = pool[method_ref] if method_ref < len(pool) else None
        if bsm_entry and bsm_entry[0] == 'MethodHandle':
            bsm_ref_str = _ref_to_str(pool, bsm_entry[2])
            if 'LambdaMetafactory' in bsm_ref_str and len(arg_indices) >= 2:
                # arg[0] = samMethodType（MethodType）
                arg0 = pool[arg_indices[0]] if arg_indices[0] < len(pool) else None
                if arg0 and arg0[0] == 'MethodType':
                    sam_type = _utf8(pool, arg0[1])
                # arg[1] = implMethod（MethodHandle → Methodref）
                arg1 = pool[arg_indices[1]] if arg_indices[1] < len(pool) else None
                if arg1 and arg1[0] == 'MethodHandle':
                    impl_method = _ref_to_str(pool, arg1[2])
            elif 'SwitchBootstraps' in bsm_ref_str and 'typeSwitch' in bsm_ref_str:
                switch_labels = []
                for ai in arg_indices:
                    ent = pool[ai] if ai < len(pool) else None
                    if ent and ent[0] == 'Class':
                        name = _utf8(pool, ent[1])
                        # primitive type pattern 以 primitive 描述符的 Class 常量编码
                        # （`case int x` → Integer.TYPE）：运行时判定等价于 wrapper 类
                        name = _PRIM_CLASS_TO_WRAPPER.get(name, name)
                        switch_labels.append(('c', name))
                    elif ent and ent[0] == 'String':
                        switch_labels.append(('s', _utf8(pool, ent[1])))
                    elif ent and ent[0] in ('Integer', 'Long', 'Float', 'Double'):
                        switch_labels.append(('i', str(ent[1])))
                    else:
                        switch_labels.append(('?', ''))
        result.append({
            'method_ref': method_ref,
            'arg_indices': arg_indices,
            'template': template,
            'sam_type': sam_type,
            'impl_method': impl_method,
            'switch_labels': switch_labels,
        })
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
    # 条目：(start_pc, end_pc, handler_pc, catch_type)；catch_type 为 binary name，
    # None 表示 catch-any（finally / synchronized 的兜底处理器）。顺序即匹配优先级。
    exc_count = r.u2()
    exception_table: list[tuple[int, int, int, Optional[str]]] = []
    for _ in range(exc_count):
        _exc_start = r.u2()
        _exc_end = r.u2()
        _exc_handler = r.u2()
        _exc_type_idx = r.u2()
        exception_table.append((_exc_start, _exc_end, _exc_handler,
                                _utf8(pool, _exc_type_idx) if _exc_type_idx else None))

    # sub-attributes：解析 LocalVariableTable 和 LocalVariableTypeTable，跳过其他
    sub_attr_count = r.u2()
    local_names: dict[int, str] = {}
    local_types: dict[int, str] = {}  # slot → generic Signature string
    # slot → [(start_pc, length, descriptor, name)]：LVT 逐变量的声明类型、名字与作用域
    # 先收集两表原始条目再统一处理：LVT 与 LVTT 的 sub-attribute 顺序不保证
    # （LVTT 可能先于 LVT 出现），且同一 slot 可被多个不同作用域的变量复用
    # （如 resize 的 float ft 与 Node<K,V>[] newTab 共用 slot 6）。
    _lvt_entries: list[tuple[int, int, str, int]] = []    # (start_pc, length, name, slot)
    _lvt_descs: dict[tuple[int, int, str], str] = {}      # (slot, start_pc, name) → descriptor
    _lvtt_entries: list[tuple[str, str, int, int]] = []   # (name, sig, slot, start_pc)
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
                _lvt_entries.append((_start_pc, _length, _utf8(pool, name_idx), slot))
                _lvt_descs[(slot, _start_pc, _utf8(pool, name_idx))] = _utf8(pool, _desc_idx)
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
                _lvtt_entries.append((_utf8(pool, name_idx), _utf8(pool, sig_idx), slot, _start_pc))
        else:
            r.skip(sub_len)

    # LVT：slot 代表名取作用域最长（length 最大）的条目 —— slot 复用时
    # 长作用域变量更可能是该槽的主要用途；同时记录全部名字供 LVTT 匹配
    _slot_all_names: dict[int, set[str]] = {}
    _slot_best: dict[int, tuple[int, str]] = {}
    for _start, _len, _name, _slot in _lvt_entries:
        _slot_all_names.setdefault(_slot, set()).add(_name)
        if _slot not in _slot_best or _len > _slot_best[_slot][0]:
            _slot_best[_slot] = (_len, _name)
    for _slot, (_len, _name) in _slot_best.items():
        local_names[_slot] = _name

    # LVTT：只要 LVTT 变量名与该 slot 的任一 LVT 名字一致即采用精确类型
    # （原逻辑仅比对唯一代表名，slot 复用时短作用域泛型变量的 hint 被误杀，
    # 如 newTab 的 [Ljava/util/HashMap$Node<TK;TV;>; → Vec<HashMap_Node<K,V>>）
    for _lvtt_name, _sig, _slot, _lvtt_start in _lvtt_entries:
        if _slot not in local_types and _lvtt_name in _slot_all_names.get(_slot, ()):
            local_types[_slot] = (_sig, _lvtt_start)

    # 按作用域区间的完整局部变量声明表：(slot, start_pc, length, name, descriptor, signature)
    # 同一 slot 可被多个不同作用域、不同类型的变量复用；store/load 按字节码偏移查表，
    # 得到该偏移处真实的 Java 声明名与声明类型（signature 为 LVTT 泛型签名，无则为 ''）。
    _lvtt_by_key = {(_s, _st, _n): _sg for _n, _sg, _s, _st in _lvtt_entries}
    local_vars: list[tuple[int, int, int, str, str, str]] = [
        (_slot, _start, _len, _name,
         _lvt_descs.get((_slot, _start, _name), ''),
         _lvtt_by_key.get((_slot, _start, _name), ''))
        for _start, _len, _name, _slot in _lvt_entries
    ]

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
        local_vars=local_vars,
        exception_table=exception_table,
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
        f_annotations: list = []
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
            elif a_name == 'RuntimeVisibleAnnotations':
                f_annotations = _parse_annotations_attr(r.read(a_len), pool)
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
            runtime_annotations=f_annotations,
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
        m_annotations: list = []
        m_anno_default: tuple = None
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
            elif attr_name == 'RuntimeVisibleAnnotations':
                m_annotations = _parse_annotations_attr(r.read(attr_len), pool)
            elif attr_name == 'AnnotationDefault':
                # 注解类型方法的元素默认值（ JVMS §4.7.22：单个 element_value）
                ad_r = _Reader(r.read(attr_len))
                m_anno_default = _parse_element_value(ad_r, pool)
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
                            m_deprecated, m_parameters, m_annotations,
                            m_anno_default))

    # ── 类级 attribute（含 BootstrapMethods、Signature、SourceFile）────────
    bootstrap_methods: list[dict] = []
    cls_generic_sig  = ''
    cls_source_file  = ''
    cls_deprecated   = False
    cls_inner_classes: list[InnerClassInfo] = []
    cls_enclosing_class = ''
    cls_enclosing_method: tuple | None = None
    cls_has_enclosing = False
    cls_annotations: list = []
    cls_is_record = False
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
        elif attr_name == 'RuntimeVisibleAnnotations':
            cls_annotations = _parse_annotations_attr(r.read(attr_len), pool)
        elif attr_name == 'SourceFile':
            sf_idx = struct.unpack_from('>H', r.read(2))[0]
            cls_source_file = _utf8(pool, sf_idx)
        elif attr_name == 'Deprecated':
            cls_deprecated = True
        elif attr_name == 'Record':
            # JVMS §4.7.30：record 类的组件声明（isRecord 查询的唯一判据——
            # JVM Class.isRecord = 有 Record 属性的类）。载荷（组件表）不消费，
            # 但必须跳过——否则后续属性读位错位（record 类解析崩溃）。
            cls_is_record = True
            r.skip(attr_len)
        elif attr_name == 'EnclosingMethod':
            # JVMS §4.7.7：局部类 / 匿名类的直接外围类与外围方法
            # （method_index 为 0 → 位于初始化器 / 字段初始化表达式中）
            em_class_idx, em_method_idx = struct.unpack_from('>HH', r.read(4))
            cls_has_enclosing = True
            cls_enclosing_class = _utf8(pool, pool[em_class_idx][1]) if em_class_idx else ''
            if em_method_idx:
                _nat = pool[em_method_idx]
                cls_enclosing_method = (_utf8(pool, _nat[1]), _utf8(pool, _nat[2]))
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
         m_deprecated, m_parameters, m_annotations,
         m_anno_default) in raw_methods:
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
                runtime_annotations=m_annotations,
                annotation_default=m_anno_default,
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
                parsed.runtime_annotations = m_annotations
                parsed.annotation_default  = m_anno_default
                methods.append(parsed)

    # 泛型擦除特判：java/lang/Class 的类型参数 <T> 是纯 phantom（反射类型
    # 指针，转译中所有存储均为 Object）。保留 <T> 会在 Class<T>（泛型上下
    # 文，如 Class 自身方法体内 this）与 Class<Object>（通配符 Class<?> 擦
    # 除形态）之间产生大量不可转换的 E0308 —— Java 语义中 Class<?> 接受任
    # 何 Class<T>，Rust 泛型则要求精确匹配。置空类级签名后，所有下游
    # （jvm_to_rust / parse_class_type_params / 宏展开）统一为裸 Class。
    _cls_sig = '' if class_name == CLASS_CLASS else cls_generic_sig

    return ClassInfo(
        name=class_name,
        fields=fields,
        methods=methods,
        super_class=super_class,
        interfaces=interfaces,
        access_flags=access_flags,
        major_version=major,
        is_interface=bool(access_flags & ACC_INTERFACE),
        is_abstract=bool(access_flags & ACC_ABSTRACT),
        is_enum=bool(access_flags & ACC_ENUM),
        is_record=cls_is_record,
        generic_signature=_cls_sig,
        source_file=cls_source_file,
        inner_classes=cls_inner_classes,
        is_deprecated=cls_deprecated,
        enclosing_class=cls_enclosing_class if cls_has_enclosing else '',
        enclosing_method=cls_enclosing_method,
        runtime_annotations=cls_annotations,
    )
