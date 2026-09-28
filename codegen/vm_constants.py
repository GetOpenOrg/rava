"""VM 常量守卫的死分支剪除（字节码规范化，classfile 解码后单点执行）。

原生二进制中部分 VM 边界方法返回恒定值（清单 vm_intrinsics.toml [vm_constants]），以其为守卫的
分支恒不执行。JDK 字节码里这类分支常拉入大族类——如 ThreadLocalRandom.<clinit> 仅在
`java.util.secureRandomSeed` 为真时调用 SecureRandom.getSeed，会把 SUN provider / NativePRNG /
SeedGenerator 整族带进每个触达 ThreadLocalRandom 的闭包。剪除后调用链与生成代码同时不含该分支。

识别形态（均为恒跳转，剪除分支指令与跳转目标之间的直线段）：
  <null 调用> [astore k; aload k] ifnull L
  <null 调用> [astore k; aload k] <null→false 调用> ifeq L
安全条件：段内无外部跳入目标、无异常表端点（段外控制流不受影响，javac 保证 L 处栈形态一致）。
"""
from __future__ import annotations

import re

from .types import Instr

_BRANCH_OPS = frozenset({
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifgt', 'ifle', 'if_icmpeq', 'if_icmpne', 'if_icmplt',
    'if_icmpge', 'if_icmpgt', 'if_icmple', 'if_acmpeq', 'if_acmpne', 'ifnull', 'ifnonnull',
    'goto', 'goto_w', 'jsr', 'jsr_w',
})
_ASTORE_RE = re.compile(r'^astore(?:_(\d)|$)')
_ALOAD_RE = re.compile(r'^aload(?:_(\d)|$)')

_manifest: tuple[frozenset, frozenset] | None = None


def _load() -> tuple[frozenset, frozenset]:
    global _manifest
    if _manifest is None:
        from .runtime_manifest import vm_constant_null_returns, vm_constant_null_to_false
        _manifest = (vm_constant_null_returns(), vm_constant_null_to_false())
    return _manifest


def _call_ref(ins: Instr) -> str:
    """invokestatic 的 `类.方法:描述符`（其余返回空串）。"""
    if ins.opcode != 'invokestatic' or not ins.comment:
        return ''
    c = ins.comment
    for pfx in ('Method ', 'InterfaceMethod '):
        if c.startswith(pfx):
            return c[len(pfx):]
    return ''


def _slot(ins: Instr, pat: re.Pattern) -> int | None:
    m = pat.match(ins.opcode)
    if not m:
        return None
    return int(m.group(1)) if m.group(1) is not None else int(ins.operand or -1)


def _switch_targets(ins: Instr) -> list[int]:
    """switch 的全部跳转目标（operand 形态见 classfile._decode_bytecode）。"""
    ops = ins.operand or ''
    out = [int(re.search(r'default:(-?\d+)', ops).group(1))]
    if ins.opcode == 'tableswitch':
        offs = re.search(r'offs:([-\d,]*)', ops).group(1)
        out += [int(x) for x in offs.split(',') if x]
    else:
        out += [int(p.split(':')[1]) for p in ops.split()[1:]]
    return out


def _dead_ranges(instrs: list[Instr]) -> list[tuple[int, int]]:
    """恒跳转分支的 (分支指令下标, 目标下标) 对。"""
    null_returns, null_to_false = _load()
    if not null_returns:
        return []
    out = []
    n = len(instrs)
    for i, ins in enumerate(instrs):
        if _call_ref(ins) not in null_returns:
            continue
        j = i + 1
        if j + 1 < n and (_slot(instrs[j], _ASTORE_RE) is not None
                          and _slot(instrs[j], _ASTORE_RE) == _slot(instrs[j + 1], _ALOAD_RE)):
            j += 2
        if j < n and instrs[j].opcode == 'ifnull':
            out.append((j, int(instrs[j].operand)))
        elif (j + 1 < n and _call_ref(instrs[j]) in null_to_false
              and instrs[j + 1].opcode == 'ifeq'):
            out.append((j + 1, int(instrs[j + 1].operand)))
    return out


def prune_dead_guards(instrs: list[Instr], exception_table) -> list[Instr]:
    """剪除 VM 常量守卫恒不执行的分支体；不满足安全条件的形态原样保留。"""
    ranges = _dead_ranges(instrs)
    if not ranges:
        return instrs
    offsets = [x.offset for x in instrs]
    drop: set[int] = set()
    for bi, target in ranges:
        lo = instrs[bi].offset
        if target <= lo or target not in offsets:
            continue
        inside = lambda pc: lo < pc < target
        # 段外跳入 / 异常表端点落在段内 → 不剪
        ext = False
        for k, x in enumerate(instrs):
            if inside(x.offset):
                continue
            tg = (_switch_targets(x) if x.opcode in ('tableswitch', 'lookupswitch')
                  else [int(x.operand)] if x.opcode in _BRANCH_OPS else [])
            if any(inside(t) for t in tg):
                ext = True
                break
        if ext or any(inside(s) or inside(e) or inside(h) for s, e, h, _ in (exception_table or ())):
            continue
        drop.update(k for k, x in enumerate(instrs) if inside(x.offset))
    if not drop:
        return instrs
    return [x for k, x in enumerate(instrs) if k not in drop]
