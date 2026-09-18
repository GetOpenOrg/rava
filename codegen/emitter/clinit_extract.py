# 从 codegen/emitter/class_writer.py 中拆出

from ..types import ClassInfo

# 简单 const_push 指令 → Python 字面量值映射（用于扫描 <clinit>）
_CONST_PUSH_OPCODES: dict[str, object] = {
    'iconst_m1': -1, 'iconst_0': 0, 'iconst_1': 1, 'iconst_2': 2,
    'iconst_3': 3,   'iconst_4': 4, 'iconst_5': 5,
    'lconst_0': 0,   'lconst_1': 1,
    'fconst_0': 0.0, 'fconst_1': 1.0, 'fconst_2': 2.0,
    'dconst_0': 0.0, 'dconst_1': 1.0,
}


def _extract_clinit_consts(ci: ClassInfo) -> dict[str, str]:
    """扫描 <clinit> 中 const_push → putstatic 的简单模式，
    返回 {field_name: constant_value_str}（格式与 ConstantValue attribute 一致）。
    只识别相邻两条指令构成的最简赋值，复杂初始化不处理。
    """
    clinit = next((m for m in ci.methods if m.name == '<clinit>'), None)
    if not clinit or not clinit.instrs:
        return {}

    result: dict[str, str] = {}
    instrs = clinit.instrs
    for i, instr in enumerate(instrs):
        if instr.opcode != 'putstatic':
            continue
        # comment 格式: "Field java/lang/String.COMPACT_STRINGS:Z"
        comment = instr.comment or ''
        if not comment.startswith('Field ') or '.' not in comment:
            continue
        rest = comment[len('Field '):]          # "java/lang/String.COMPACT_STRINGS:Z"
        if '.' not in rest:
            continue
        cls_part, field_desc = rest.split('.', 1)   # "java/lang/String", "COMPACT_STRINGS:Z"
        if cls_part != ci.name or ':' not in field_desc:
            continue
        fname = field_desc.split(':')[0]

        # 前一条指令（或前两条：iconst_0 → anewarray → putstatic）
        if i == 0:
            continue
        prev = instrs[i - 1]
        # 模式：iconst_0 → anewarray → putstatic（static final Object[] = new T[0]）
        if prev.opcode == 'anewarray' and i >= 2 and instrs[i - 2].opcode == 'iconst_0':
            result[fname] = '__EMPTY_ARRAY__'
        elif prev.opcode in _CONST_PUSH_OPCODES:
            val = _CONST_PUSH_OPCODES[prev.opcode]
            result[fname] = str(int(val)) if isinstance(val, float) and val == int(val) else str(val)
        elif prev.opcode in ('bipush', 'sipush') and prev.operand is not None:
            result[fname] = str(prev.operand)
        elif prev.opcode == 'ldc' and prev.comment:
            # ldc comment 可能是 "String ...", "int 42", "float 1.0" 等
            ldc = prev.comment.strip()
            for prefix in ('String ', 'int ', 'long ', 'float ', 'double '):
                if ldc.startswith(prefix):
                    result[fname] = ldc[len(prefix):]
                    break

    return result


def _push_int_value(instr) -> 'int | None':
    """从 push 指令中提取整数值（iconst_* / bipush / sipush）。"""
    if instr.opcode in _CONST_PUSH_OPCODES:
        return int(_CONST_PUSH_OPCODES[instr.opcode])
    if instr.opcode in ('bipush', 'sipush') and instr.operand is not None:
        return int(instr.operand)
    return None


def _extract_clinit_arrays(ci: ClassInfo) -> 'dict[str, list[int]]':
    """扫描 <clinit> 中 newarray + dup + index + value + xastore 模式，
    提取静态 final 数组（[B/[C/[S/[I）的全部常量元素值。
    返回 {field_name: [val0, val1, ...]}。
    """
    clinit = next((m for m in ci.methods if m.name == '<clinit>'), None)
    if not clinit or not clinit.instrs:
        return {}

    instrs = clinit.instrs
    result: dict[str, list[int]] = {}
    _XASTORE = frozenset({'bastore', 'castore', 'sastore', 'iastore'})

    for put_i, ins in enumerate(instrs):
        if ins.opcode != 'putstatic':
            continue
        comment = ins.comment or ''
        if not comment.startswith('Field ') or '.' not in comment:
            continue
        rest = comment[len('Field '):]
        if '.' not in rest:
            continue
        cls_part, field_desc = rest.split('.', 1)
        if cls_part != ci.name or ':' not in field_desc:
            continue
        fname = field_desc.split(':')[0]
        desc = field_desc.split(':', 1)[1]
        if desc not in ('[B', '[C', '[S', '[I'):
            continue

        # 向前查找对应的 newarray 指令
        na_i = put_i - 1
        while na_i >= 0 and instrs[na_i].opcode != 'newarray':
            na_i -= 1
        if na_i < 0:
            continue

        # newarray 与 putstatic 之间的指令组 = dup + index_push + value_push + xastore
        elem_instrs = instrs[na_i + 1:put_i]
        if not elem_instrs or len(elem_instrs) % 4 != 0:
            continue

        values: dict[int, int] = {}
        valid = True
        for k in range(0, len(elem_instrs), 4):
            grp = elem_instrs[k:k + 4]
            if grp[0].opcode != 'dup' or grp[3].opcode not in _XASTORE:
                valid = False
                break
            idx = _push_int_value(grp[1])
            val = _push_int_value(grp[2])
            if idx is None or val is None:
                valid = False
                break
            values[idx] = val

        if not valid or not values:
            continue
        max_idx = max(values.keys())
        result[fname] = [values.get(i, 0) for i in range(max_idx + 1)]

    return result
