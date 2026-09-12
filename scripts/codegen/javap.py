"""
javap -verbose 输出解析器：文本 → ClassInfo（字段 + 方法 + 字节码指令）。
"""

import re
from .types import Instr, FieldInfo, ParsedMethod, ClassInfo

_JAVA_KEYWORDS = frozenset({
    'class', 'public', 'private', 'protected', 'static', 'final',
    'int', 'long', 'float', 'double', 'boolean', 'byte', 'short', 'char',
    'if', 'for', 'while', 'new',
})


def parse_javap(text: str, class_name: str) -> ClassInfo:
    lines  = text.split('\n')
    fields:  list[FieldInfo]    = []
    methods: list[ParsedMethod] = []

    i = 0
    while i < len(lines):
        line = lines[i]

        if 'descriptor:' not in line:
            i += 1
            continue

        desc_m = re.search(r'descriptor:\s*(\S+)', line)
        if not desc_m:
            i += 1
            continue

        descriptor = desc_m.group(1)

        # ACC_STATIC 检测（向上扫描）
        is_static = any('ACC_STATIC' in lines[k] for k in range(max(0, i - 5), i))

        # ── 字段（descriptor 不含括号）──
        if '(' not in descriptor:
            for k in range(i - 1, max(i - 5, -1), -1):
                nm = re.search(r'(\w+)\s*;', lines[k])
                if nm and nm.group(1) not in _JAVA_KEYWORDS:
                    fields.append(FieldInfo(nm.group(1), descriptor, is_static))
                    break
            i += 1
            continue

        # ── 方法 ──
        method_name = None
        for k in range(i - 1, max(i - 6, -1), -1):
            nm = re.search(r'(\w+|<init>|<clinit>)\s*\(', lines[k])
            if nm:
                method_name = nm.group(1)
                if 'ACC_STATIC' in lines[k]:
                    is_static = True
                break
            if 'ACC_STATIC' in lines[k]:
                is_static = True

        if not method_name or method_name in _JAVA_KEYWORDS:
            i += 1
            continue

        # flags 行（向下）
        for k in range(i, min(i + 3, len(lines))):
            if 'ACC_STATIC' in lines[k]:
                is_static = True

        # 定位 Code: 段
        code_idx = next(
            (k for k in range(i, min(i + 20, len(lines)))
             if re.match(r'\s+Code:', lines[k])),
            None,
        )
        if code_idx is None:
            i += 1
            continue

        # locals / args_size
        locals_count, args_size = 1, 1
        sl = lines[code_idx + 1] if code_idx + 1 < len(lines) else ''
        lm = re.search(r'locals=(\d+)', sl)
        am = re.search(r'args_size=(\d+)', sl)
        if lm: locals_count = int(lm.group(1))
        if am: args_size    = int(am.group(1))

        # 指令列表
        instrs: list[Instr] = []
        j = code_idx + 2
        while j < len(lines):
            im = re.match(r'\s+(\d+):\s+(\w+)(.*)', lines[j])
            if im:
                offset = int(im.group(1))
                opcode = im.group(2)
                rest   = im.group(3).strip()
                comment = None
                if '//' in rest:
                    rest, comment = rest.split('//', 1)
                    rest    = rest.strip()
                    comment = comment.strip()
                instrs.append(Instr(
                    offset=offset, opcode=opcode,
                    operand=rest or None, comment=comment,
                ))
                j += 1
            elif re.match(r'\s+(LineNumberTable|StackMapTable|frame_type|offset_delta)', lines[j]):
                break
            elif lines[j].strip() == '':
                j += 1
            else:
                j += 1
                if instrs:
                    break

        if instrs:
            methods.append(ParsedMethod(
                class_name=class_name, name=method_name,
                descriptor=descriptor, is_static=is_static,
                locals_count=locals_count, args_size=args_size,
                instrs=instrs,
            ))
        i = j

    return ClassInfo(name=class_name, fields=fields, methods=methods)
