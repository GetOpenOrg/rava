"""
控制流图：基本块划分 + 图算法（RPO / 支配树 / 自然循环 / 可归约性）。

本模块只描述「指令序列的形状」，不涉及栈模拟与 Rust 生成。
图算法全部以 (entry, succs) 形式的抽象图为输入，供归约后的图复用。
"""

from __future__ import annotations
from dataclasses import dataclass, field

from ..types import Instr
from ._opcodes import COND_BRANCH_OPS, GOTO_OPS, SWITCH_OPS, EXIT_OPS, SUBROUTINE_OPS


class CfgError(Exception):
    """控制流图无法构建 / 无法模拟（方法退化为 stub，并计入统计）。"""


@dataclass
class Terminator:
    """基本块的出口。

    kind:
      'cond'   条件跳转：target = 跳转目标块，fallthrough = 不跳转时的后继块
      'goto'   无条件跳转：target
      'fall'   自然落入下一块：target
      'switch' cases = [(values, block)]，default = 默认目标块
      'exit'   return / athrow（无后继）
    pc: 跳转指令的字节码偏移（fall / exit 为 None）
    """
    kind: str
    pc: int | None = None
    target: int | None = None
    fallthrough: int | None = None
    cases: list = field(default_factory=list)
    default: int | None = None

    def successors(self) -> list[int]:
        if self.kind == 'cond':
            out = [self.target]
            if self.fallthrough != self.target:
                out.append(self.fallthrough)
            return out
        if self.kind in ('goto', 'fall'):
            return [self.target]
        if self.kind == 'switch':
            out: list[int] = []
            for _, b in self.cases:
                if b not in out:
                    out.append(b)
            if self.default not in out:
                out.append(self.default)
            return out
        return []


@dataclass
class Block:
    id: int
    start_idx: int          # 指令下标（含）
    end_idx: int            # 指令下标（不含）
    start_pc: int
    term: Terminator = None
    is_handler_entry: bool = False


def parse_switch_operand(operand: str) -> tuple[int, list[tuple[int, int]]]:
    """解析 switch 操作数 → (default_offset, [(case_value, target_offset)])。

    两种格式：
      tableswitch : `default:<off> low:<n> high:<n> offs:<o1,o2,..>`
      lookupswitch: `default:<off> <key>:<off> ...`
    """
    default_off = None
    low = None
    offs: list[int] | None = None
    pairs: list[tuple[int, int]] = []
    for part in operand.split():
        if ':' not in part:
            continue
        k, v = part.split(':', 1)
        if k == 'default':
            default_off = int(v)
        elif k == 'low':
            low = int(v)
        elif k == 'high':
            pass
        elif k == 'offs':
            offs = [int(x) for x in v.split(',') if x]
        else:
            pairs.append((int(k), int(v)))
    if offs is not None:
        if low is None:
            raise CfgError(f"tableswitch 缺少 low: {operand}")
        pairs = [(low + i, off) for i, off in enumerate(offs)]
    if default_off is None:
        raise CfgError(f"switch 缺少 default: {operand}")
    return default_off, pairs


def build_blocks(instrs: list[Instr], exception_table: list | None = None) -> list[Block]:
    """指令序列 → 基本块列表（id 即列表下标，按字节码顺序）。"""
    if not instrs:
        return []
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}

    def idx_of(off: int, pc: int) -> int:
        if off not in off2idx:
            raise CfgError(f"跳转目标 {off} 不是指令边界（pc={pc}）")
        return off2idx[off]

    leaders: set[int] = {0}
    handler_idx: set[int] = set()
    for (start_pc, end_pc, handler_pc, _t) in (exception_table or []):
        if handler_pc in off2idx:
            leaders.add(off2idx[handler_pc])
            handler_idx.add(off2idx[handler_pc])
        # 受保护区间的两端是块边界：每个块要么整体受某个处理器保护，要么整体不受
        for pc in (start_pc, end_pc):
            if pc in off2idx:
                leaders.add(off2idx[pc])

    for i, ins in enumerate(instrs):
        op = ins.opcode
        if op in SUBROUTINE_OPS:
            raise CfgError(f"不支持的子例程指令 {op}（pc={ins.offset}）")
        if op in COND_BRANCH_OPS or op in GOTO_OPS:
            leaders.add(idx_of(int(ins.operand), ins.offset))
        elif op in SWITCH_OPS:
            default_off, pairs = parse_switch_operand(ins.operand or '')
            leaders.add(idx_of(default_off, ins.offset))
            for _, off in pairs:
                leaders.add(idx_of(off, ins.offset))
        elif op not in EXIT_OPS:
            continue
        if i + 1 < len(instrs):
            leaders.add(i + 1)

    starts = sorted(leaders)
    blocks: list[Block] = []
    for n, s in enumerate(starts):
        e = starts[n + 1] if n + 1 < len(starts) else len(instrs)
        blocks.append(Block(id=n, start_idx=s, end_idx=e, start_pc=instrs[s].offset,
                            is_handler_entry=(s in handler_idx)))
    by_start = {b.start_idx: b.id for b in blocks}

    for b in blocks:
        last = instrs[b.end_idx - 1]
        op = last.opcode
        nxt = by_start.get(b.end_idx)
        if op in EXIT_OPS:
            b.term = Terminator('exit')
        elif op in GOTO_OPS:
            b.term = Terminator('goto', pc=last.offset,
                                target=by_start[idx_of(int(last.operand), last.offset)])
        elif op in COND_BRANCH_OPS:
            if nxt is None:
                raise CfgError(f"条件跳转位于方法末尾（pc={last.offset}）")
            b.term = Terminator('cond', pc=last.offset,
                                target=by_start[idx_of(int(last.operand), last.offset)],
                                fallthrough=nxt)
        elif op in SWITCH_OPS:
            default_off, pairs = parse_switch_operand(last.operand or '')
            grouped: dict[int, list[int]] = {}
            for val, off in pairs:
                grouped.setdefault(by_start[idx_of(off, last.offset)], []).append(val)
            b.term = Terminator('switch', pc=last.offset,
                                cases=[(vals, tgt) for tgt, vals in grouped.items()],
                                default=by_start[idx_of(default_off, last.offset)])
        else:
            if nxt is None:
                raise CfgError(f"方法末尾缺少 return/athrow（pc={last.offset}）")
            b.term = Terminator('fall', target=nxt)
    return blocks


# ─────────────────────────────────────────────────────────────────────────────
# 抽象图算法：graph = (entry, succs: dict[node, list[node]])
# ─────────────────────────────────────────────────────────────────────────────

def reachable(entry: int, succs: dict[int, list[int]]) -> set[int]:
    seen = {entry}
    work = [entry]
    while work:
        n = work.pop()
        for s in succs.get(n, ()):
            if s not in seen:
                seen.add(s)
                work.append(s)
    return seen


def reverse_postorder(entry: int, succs: dict[int, list[int]]) -> list[int]:
    """逆后序。后继按编号降序访问，使 RPO 尽量贴近字节码（= 源码）顺序。"""
    order: list[int] = []
    seen = {entry}
    stack: list[tuple[int, list[int]]] = [(entry, sorted(succs.get(entry, ()), reverse=True))]
    while stack:
        node, pending = stack[-1]
        advanced = False
        while pending:
            s = pending.pop(0)
            if s not in seen:
                seen.add(s)
                stack.append((s, sorted(succs.get(s, ()), reverse=True)))
                advanced = True
                break
        if not advanced:
            order.append(node)
            stack.pop()
    order.reverse()
    return order


def predecessors(nodes, succs: dict[int, list[int]]) -> dict[int, list[int]]:
    preds: dict[int, list[int]] = {n: [] for n in nodes}
    for n in nodes:
        for s in succs.get(n, ()):
            if s in preds and n not in preds[s]:
                preds[s].append(n)
    return preds


def immediate_dominators(entry: int, succs: dict[int, list[int]], rpo: list[int]) -> dict[int, int]:
    """Cooper-Harvey-Kennedy 迭代支配算法。返回 idom（entry 的 idom 为自身）。"""
    index = {n: i for i, n in enumerate(rpo)}
    preds = predecessors(rpo, succs)
    idom: dict[int, int] = {entry: entry}

    def intersect(a: int, b: int) -> int:
        while a != b:
            while index[a] > index[b]:
                a = idom[a]
            while index[b] > index[a]:
                b = idom[b]
        return a

    changed = True
    while changed:
        changed = False
        for n in rpo:
            if n == entry:
                continue
            new = None
            for p in preds[n]:
                if p in idom:
                    new = p if new is None else intersect(p, new)
            if new is not None and idom.get(n) != new:
                idom[n] = new
                changed = True
    return idom


def dominates(idom: dict[int, int], a: int, b: int) -> bool:
    """a 是否支配 b（自反）。"""
    while True:
        if a == b:
            return True
        parent = idom.get(b)
        if parent is None or parent == b:
            return False
        b = parent


@dataclass
class FlowAnalysis:
    entry: int
    succs: dict[int, list[int]]
    rpo: list[int]
    rpo_index: dict[int, int]
    preds: dict[int, list[int]]
    idom: dict[int, int]
    back_edges: set            # {(src, header)}
    loops: dict[int, set]      # header → 循环体节点集（含 header）
    reducible: bool

    def is_back_edge(self, src: int, dst: int) -> bool:
        return (src, dst) in self.back_edges

    def forward_in_degree(self, n: int) -> int:
        return sum(1 for p in self.preds[n] if (p, n) not in self.back_edges)


def analyze(entry: int, succs: dict[int, list[int]]) -> FlowAnalysis:
    live = reachable(entry, succs)
    succs = {n: [s for s in succs.get(n, ())] for n in live}
    rpo = reverse_postorder(entry, succs)
    rpo_index = {n: i for i, n in enumerate(rpo)}
    preds = predecessors(rpo, succs)
    idom = immediate_dominators(entry, succs, rpo)

    back_edges: set = set()
    reducible = True
    for u in rpo:
        for v in succs[u]:
            if rpo_index[v] <= rpo_index[u]:
                if dominates(idom, v, u):
                    back_edges.add((u, v))
                else:
                    reducible = False

    loops: dict[int, set] = {}
    for (u, h) in back_edges:
        body = loops.setdefault(h, {h})
        work = [u]
        while work:
            n = work.pop()
            if n in body:
                continue
            body.add(n)
            work.extend(preds[n])
    return FlowAnalysis(entry, succs, rpo, rpo_index, preds, idom, back_edges, loops, reducible)


def function_always_returns(instrs: list[Instr]) -> bool:
    """方法的所有出口是否都是 return/athrow（合法字节码恒为 True；保留为后处理的判据）。"""
    if not instrs:
        return True
    try:
        build_blocks(instrs)
    except CfgError:
        return False
    return True
