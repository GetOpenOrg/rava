"""closure.json 折叠点（folds v1）的消费：字节码规范化，classfile 解码后单点执行。

Rust 闭包分析器（generator/crates/closure）剪掉的不可达代码与常量读取点，Python 生成器必须
同样不翻译，否则生成代码会引用闭包外的类。本模块只消费数据、不另做判定；格式约定见
docs/plans/2026-09-29-rust-closure-analyzer.md §7.3「折叠点导出」：

  "folds_version": 1,
  "folds": [{"method": "类.方法:描述符", "dead_pcs": [[start, end), ...],
             "dead_handlers": [pc, ...],
             "consts": [{"pc": 12, "kind": "getfield", "value": false, "type": "Z"}]}]

规范化动作（均在 CFG 结构化之前，生成代码的唯一指令序列）：
  1. dead_pcs 内的指令删除；
  2. 条件跳转只剩一个活后继 → 弹出操作数（`pop`）+ `goto`（或直通），偏移沿用原指令字节；
     switch 的死目标改指向一个活目标，只剩一个活目标时同样改写为 `pop` + `goto`；
  3. dead_handlers 与受保护区间已全死的异常表项删除，其余区间端点收拢到活指令起点；
  4. consts 读取点改写为常量装载：getstatic 直接替换为装载指令；getfield / invoke 替换为
     合成指令 `fold_const`（operand = 弹出的模拟栈条目数，comment = 装载指令），由
     instr/sim/stack.py 弹出 receiver / 实参（有副作用的保留求值）后压入常量。

不认识的 folds_version 忽略全部折叠、按原样翻译（两侧可各自先合入）。
违反格式约定的输入（端点不在指令起点、活指令顺序落入死区、读取点指令不符）直接报错。
"""
from __future__ import annotations

import json
from dataclasses import dataclass, field

from .types import Instr

FOLDS_VERSION = 1

_ONE_OPERAND_BRANCH = frozenset({
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifgt', 'ifle', 'ifnull', 'ifnonnull',
})
_TWO_OPERAND_BRANCH = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmpgt', 'if_icmple',
    'if_acmpeq', 'if_acmpne',
})
_GOTO = frozenset({'goto', 'goto_w'})
_SWITCH = frozenset({'tableswitch', 'lookupswitch'})
# 无顺序后继的指令（其后紧跟死区是合法的）
_NO_FALLTHROUGH = _GOTO | _SWITCH | frozenset({
    'return', 'ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn', 'athrow',
})
_INVOKES = frozenset({'invokevirtual', 'invokespecial', 'invokestatic', 'invokeinterface'})


class FoldError(ValueError):
    """closure.json 的 folds 违反 v1 格式约定。"""


@dataclass
class MethodFold:
    dead_pcs: list = field(default_factory=list)       # [(start, end)]，半开区间
    dead_handlers: frozenset = frozenset()
    consts: dict = field(default_factory=dict)         # pc → {"kind", "value", "type"}


# method key（类.方法:描述符）→ MethodFold；由 load() 单点写入
_FOLDS: dict[str, MethodFold] = {}


def load(path: str) -> int:
    """读取 closure.json 的 folds；返回载入的方法数（版本不认识时为 0）。"""
    _FOLDS.clear()
    with open(path, encoding='utf-8') as f:
        data = json.load(f)
    ver = data.get('folds_version')
    if ver != FOLDS_VERSION:
        if ver is not None or data.get('folds'):
            print(f"[folds] folds_version={ver!r} 不认识（支持 {FOLDS_VERSION}），忽略折叠")
        return 0
    for ent in data.get('folds') or ():
        key = ent['method']
        _FOLDS[key] = MethodFold(
            dead_pcs=[(int(s), int(e)) for s, e in ent.get('dead_pcs') or ()],
            dead_handlers=frozenset(int(h) for h in ent.get('dead_handlers') or ()),
            consts={int(c['pc']): c for c in ent.get('consts') or ()},
        )
    return len(_FOLDS)


def clear() -> None:
    _FOLDS.clear()


def loaded() -> int:
    return len(_FOLDS)


# ── 常量装载指令 ───────────────────────────────────────────────────────────────

def _push_instr(pc: int, value, jtype: str, where: str) -> Instr:
    """常量 → 等价的 JVM 装载指令（复用 sim_consts 的既有发射路径）。"""
    if value is None:
        if not jtype.startswith(('L', '[')):
            raise FoldError(f"{where}: null 常量的 type 必须是引用类型，得到 {jtype!r}")
        return Instr(pc, 'aconst_null')
    if jtype == 'Z':
        if not isinstance(value, bool):
            raise FoldError(f"{where}: Z 常量须为 JSON 布尔值，得到 {value!r}")
        return Instr(pc, 'ldc', '0', f'int {int(value)}')
    if jtype in ('B', 'C', 'S', 'I'):
        if isinstance(value, bool) or not isinstance(value, int):
            raise FoldError(f"{where}: {jtype} 常量须为 JSON 整数，得到 {value!r}")
        return Instr(pc, 'ldc', '0', f'int {value}')
    if jtype == 'J':
        return Instr(pc, 'ldc2_w', '0', f'long {int(str(value))}')
    if jtype == 'F':
        return Instr(pc, 'ldc', '0', f'float {value}')
    if jtype == 'D':
        return Instr(pc, 'ldc2_w', '0', f'double {value}')
    if jtype == 'Ljava/lang/String;':
        if not isinstance(value, str):
            raise FoldError(f"{where}: String 常量须为 JSON 字符串，得到 {value!r}")
        return Instr(pc, 'ldc', '0', f'String {value}')
    raise FoldError(f"{where}: 不支持的常量类型 {jtype!r}")


def _invoke_pops(ins: Instr) -> int:
    """invoke 消耗的模拟栈条目数（long/double 在模拟栈中占一个条目）。"""
    from .type_map import parse_descriptor_params
    desc = (ins.comment or '').split(':', 1)[-1]
    n = len(parse_descriptor_params(desc))
    return n if ins.opcode == 'invokestatic' else n + 1


def _const_instr(ins: Instr, c: dict, where: str) -> Instr:
    kind = c.get('kind')
    op = ins.opcode
    ok = (kind == 'getfield' and op == 'getfield'
          or kind == 'getstatic' and op == 'getstatic'
          or kind == 'invoke' and op in _INVOKES)
    if not ok:
        raise FoldError(f"{where}: const kind={kind!r} 与指令 {op} 不符")
    push = _push_instr(ins.offset, c.get('value'), c.get('type', ''), where)
    npop = 0 if kind == 'getstatic' else 1 if kind == 'getfield' else _invoke_pops(ins)
    if npop == 0:
        return push
    return Instr(ins.offset, 'fold_const', str(npop),
                 f'{push.opcode}\t{push.operand or ""}\t{push.comment or ""}')


def decode_fold_const(ins: Instr) -> tuple[int, Instr]:
    """`fold_const` → (弹出条目数, 装载指令)。"""
    op, operand, comment = (ins.comment or '').split('\t', 2)
    return int(ins.operand), Instr(ins.offset, op, operand or None, comment or None)


# ── 跳转改写 ─────────────────────────────────────────────────────────────────

def _switch_parts(operand: str) -> tuple[list[str], list[tuple[str, int]]]:
    """switch operand → (非目标字段, [(键, 目标)])；键 'default' / 'offs#i' / lookup 键。"""
    keep: list[str] = []
    targets: list[tuple[str, int]] = []
    for part in operand.split():
        k, v = part.split(':', 1)
        if k == 'default':
            targets.append(('default', int(v)))
        elif k in ('low', 'high'):
            keep.append(part)
        elif k == 'offs':
            targets += [(f'offs#{i}', int(x)) for i, x in enumerate(v.split(',')) if x]
        else:
            targets.append((k, int(v)))
    return keep, targets


def _switch_rebuild(op: str, keep: list[str], targets: list[tuple[str, int]]) -> str:
    d = dict(targets)
    parts = [f"default:{d['default']}"]
    if op == 'tableswitch':
        parts += keep
        offs = [t for k, t in targets if k.startswith('offs#')]
        parts.append('offs:' + ','.join(str(t) for t in offs))
    else:
        parts += [f'{k}:{t}' for k, t in targets if k != 'default']
    return ' '.join(parts)


def _pops(pc: int, n: int) -> list[Instr]:
    return [Instr(pc + i, 'pop') for i in range(n)]


def _rewrite_branch(ins: Instr, next_dead: bool, is_dead, where: str) -> list[Instr]:
    op = ins.opcode
    if op in _ONE_OPERAND_BRANCH or op in _TWO_OPERAND_BRANCH:
        n = 1 if op in _ONE_OPERAND_BRANCH else 2
        target_dead = is_dead(int(ins.operand))
        if target_dead and next_dead:
            raise FoldError(f"{where}: 条件跳转 pc={ins.offset} 两个后继都在 dead_pcs 内")
        if target_dead:        # 恒直通
            return _pops(ins.offset, n)
        if next_dead:          # 恒跳转
            return _pops(ins.offset, n) + [Instr(ins.offset + n, 'goto', ins.operand)]
        return [ins]
    if op in _GOTO:
        if is_dead(int(ins.operand)):
            raise FoldError(f"{where}: 活跳转 pc={ins.offset} 的目标在 dead_pcs 内")
        return [ins]
    if op in _SWITCH:
        keep, targets = _switch_parts(ins.operand or '')
        live = [t for _k, t in targets if not is_dead(t)]
        if not live:
            raise FoldError(f"{where}: switch pc={ins.offset} 的目标全在 dead_pcs 内")
        if len(set(live)) == 1:
            return [Instr(ins.offset, 'pop'), Instr(ins.offset + 1, 'goto', str(live[0]))]
        d = dict(targets)
        fallback = d['default'] if not is_dead(d['default']) else live[0]
        if len(live) == len(targets):
            return [ins]
        new = [(k, fallback if is_dead(t) else t) for k, t in targets]
        return [Instr(ins.offset, op, _switch_rebuild(op, keep, new), ins.comment)]
    return [ins]


# ── 主入口 ───────────────────────────────────────────────────────────────────

def apply(method_key: str, instrs: list[Instr], exception_table: list,
          code_len: int) -> list[Instr]:
    """按 folds 规范化一个方法的指令序列；exception_table 原地更新。无折叠时原样返回。"""
    fold = _FOLDS.get(method_key)
    if fold is None:
        return instrs
    where = method_key
    starts = {x.offset for x in instrs}
    for s, e in fold.dead_pcs:
        if s not in starts or not (e in starts or e == code_len) or s >= e:
            raise FoldError(f"{where}: dead_pcs [{s}, {e}) 端点不在指令起点")

    def is_dead(pc: int) -> bool:
        return any(s <= pc < e for s, e in fold.dead_pcs)

    dead_idx = {i for i, x in enumerate(instrs) if is_dead(x.offset)}
    for h in fold.dead_handlers:
        if h not in starts:
            raise FoldError(f"{where}: dead_handlers {h} 不在指令起点")
    for pc in fold.consts:
        if pc not in starts or is_dead(pc):
            raise FoldError(f"{where}: const pc={pc} 不是活指令起点")

    out: list[Instr] = []
    for i, ins in enumerate(instrs):
        if i in dead_idx:
            continue
        next_dead = i + 1 < len(instrs) and (i + 1) in dead_idx
        if next_dead and ins.opcode not in _NO_FALLTHROUGH \
                and ins.opcode not in _ONE_OPERAND_BRANCH | _TWO_OPERAND_BRANCH:
            raise FoldError(f"{where}: 活指令 pc={ins.offset} {ins.opcode} 顺序落入 dead_pcs")
        c = fold.consts.get(ins.offset)
        if c is not None:
            out.append(_const_instr(ins, c, where))
            continue
        out.extend(_rewrite_branch(ins, next_dead, is_dead, where))

    # 异常表：删 dead_handlers 与受保护区间全死的表项，端点收拢到活指令起点
    live_offs = sorted(x.offset for k, x in enumerate(instrs) if k not in dead_idx)

    def first_live_at_or_after(pc: int) -> int | None:
        for o in live_offs:
            if o >= pc:
                return o
        return None

    new_table = []
    for s, e, h, t in exception_table or ():
        if h in fold.dead_handlers:
            continue
        if is_dead(h):
            raise FoldError(f"{where}: handler {h} 在 dead_pcs 内但未列入 dead_handlers")
        ns = first_live_at_or_after(s)
        if ns is None or ns >= e:
            continue           # 区间内已无活指令
        ne = first_live_at_or_after(e)
        new_table.append((ns, e if ne is None else ne, h, t))
    if exception_table is not None:
        exception_table[:] = new_table
    return out
