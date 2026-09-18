"""异常表 → try/catch 区域规划（JVMS §2.10 / §4.7.3）。

输入是 Code attribute 的异常表，输出是与结构化器无关的区域描述：

- **TryGroup**：受保护区间集合完全相同的一组处理器 = 一个 Java `try` 语句的兄弟 catch 子句
  （multi-catch 的多个 catch_type 指向同一 handler_pc，合并为一个子句）。
- 受保护区间之间、以及最后一个区间到首个处理器之间的指令属于 try 体的文本范围，
  但不受该组保护（javac 内联的 finally 副本与尾部 goto）——生成时用 `java_unguarded!` 包裹。

本模块只依赖指令偏移，不依赖任何具体的控制流结构化实现；结构化器在到达
`group.start_idx` 时发射 `java_try!`，并用 `unguarded_layers` 判定豁免层数。
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class CatchClause:
    handler_pc: int
    handler_idx: int
    # catch_type binary names；空列表 = catch-any（finally / monitor 兜底）
    catch_types: list = field(default_factory=list)
    is_catch_any: bool = False


@dataclass
class TryGroup:
    ranges: tuple            # ((start_pc, end_pc), ...) 已排序
    clauses: list            # list[CatchClause]，按异常表顺序（= 匹配优先级）
    start_idx: int = 0       # try 体首指令下标
    body_end_idx: int = 0    # try 体文本范围终点 = 首个处理器下标

    def covers(self, pc: int) -> bool:
        return any(s <= pc < e for s, e in self.ranges)

    @property
    def first_handler_pc(self) -> int:
        return min(c.handler_pc for c in self.clauses)


class TryCatchPlan:
    """一个方法的全部 try 区域；结构化器按指令下标查询。"""

    def __init__(self, exception_table: list, instrs: list):
        self._instrs = instrs
        self._off2idx = {ins.offset: idx for idx, ins in enumerate(instrs)}
        self.groups: list[TryGroup] = self._build(exception_table or [])
        self._by_start: dict[int, list[TryGroup]] = {}
        for g in self.groups:
            self._by_start.setdefault(g.start_idx, []).append(g)
        # 同一起点的多个组：处理器越靠后的越外层
        for lst in self._by_start.values():
            lst.sort(key=lambda g: -g.first_handler_pc)
        self._active: list[TryGroup] = []     # 正在发射 try 体的组（外 → 内）
        self._emitted: set[int] = set()
        self._wrapped_layers = 0              # 当前已由 java_unguarded! 豁免的层数

    # ── 构建 ────────────────────────────────────────────────────────
    def _build(self, table: list) -> list:
        # 1. 按 handler_pc 聚合：同一处理器的全部受保护区间与 catch_type
        by_handler: dict[int, dict] = {}
        order: list[int] = []
        for start_pc, end_pc, handler_pc, catch_type in table:
            if start_pc >= handler_pc:
                # 处理器保护自身（finally / monitorexit 的重试条目）：
                # 该语义由 Rust 侧的作用域释放承担，不构成 try 区域
                continue
            h = by_handler.get(handler_pc)
            if h is None:
                h = by_handler[handler_pc] = {'ranges': set(), 'types': [], 'any': False}
                order.append(handler_pc)
            h['ranges'].add((start_pc, end_pc))
            if catch_type is None:
                h['any'] = True
            elif catch_type not in h['types']:
                h['types'].append(catch_type)

        # 2. 区间集合相同的处理器 = 同一 try 语句的兄弟 catch
        by_ranges: dict[tuple, TryGroup] = {}
        groups: list[TryGroup] = []
        for handler_pc in order:
            h = by_handler[handler_pc]
            handler_idx = self._off2idx.get(handler_pc)
            if handler_idx is None:
                continue
            key = self._merge_ranges(h['ranges'])
            g = by_ranges.get(key)
            if g is None:
                g = by_ranges[key] = TryGroup(ranges=key, clauses=[])
                groups.append(g)
            g.clauses.append(CatchClause(
                handler_pc=handler_pc, handler_idx=handler_idx,
                catch_types=list(h['types']), is_catch_any=h['any']))

        result = []
        for g in groups:
            start_idx = self._off2idx.get(g.ranges[0][0])
            if start_idx is None:
                continue
            g.start_idx = start_idx
            g.body_end_idx = min(c.handler_idx for c in g.clauses)
            if g.body_end_idx <= g.start_idx:
                continue
            result.append(g)
        return result

    @staticmethod
    def _merge_ranges(ranges) -> tuple:
        merged: list[list[int]] = []
        for s, e in sorted(ranges):
            if merged and s <= merged[-1][1]:
                merged[-1][1] = max(merged[-1][1], e)
            else:
                merged.append([s, e])
        return tuple((s, e) for s, e in merged)

    # ── 结构化器查询接口 ────────────────────────────────────────────
    def __bool__(self) -> bool:
        return bool(self.groups)

    def group_starting_at(self, idx: int, end_idx: int) -> Optional[TryGroup]:
        """在 idx 处开始、尚未发射、且完整落在当前块内的最外层组。"""
        for g in self._by_start.get(idx, ()):
            if id(g) in self._emitted:
                continue
            if max(c.handler_idx for c in g.clauses) >= end_idx:
                continue
            return g
        return None

    def enter_body(self, group: TryGroup) -> None:
        self._emitted.add(id(group))
        self._active.append(group)

    def leave_body(self, group: TryGroup) -> None:
        assert self._active and self._active[-1] is group
        self._active.pop()

    def clause_end_idx(self, group: TryGroup, clause_pos: int, block_end_idx: int,
                       handler_var_scope_end: Optional[int]) -> int:
        """第 clause_pos 个处理器体的终点下标。

        - 非末位处理器：到下一个处理器为止；
        - 末位处理器：try 体/前序处理器末尾 `goto END` 的目标、处理器变量的
          LocalVariableTable 作用域终点、当前块终点三者中最近的一个。
          （try 体不以 goto 收尾 = try 体从不正常完成，此时 try 语句之后的代码
          只能经由处理器到达，并入末位处理器体在语义上等价。）
        """
        handler_idxs = sorted(c.handler_idx for c in group.clauses)
        this_idx = group.clauses[clause_pos].handler_idx
        later = [h for h in handler_idxs if h > this_idx]
        if later:
            return later[0]
        candidates = [block_end_idx]
        # 各段（try 体、前序处理器）末尾的 goto END
        boundaries = [group.body_end_idx] + [h for h in handler_idxs if h > group.body_end_idx]
        for b in boundaries:
            if b - 1 < 0:
                continue
            last = self._instrs[b - 1]
            if last.opcode in ('goto', 'goto_w') and last.operand:
                target_idx = self._off2idx.get(int(last.operand))
                if target_idx is not None and target_idx > this_idx:
                    candidates.append(target_idx)
        if handler_var_scope_end is not None:
            scope_idx = self._off2idx.get(handler_var_scope_end)
            if scope_idx is not None and scope_idx > this_idx:
                candidates.append(scope_idx)
        return min(candidates)

    def unguarded_layers(self, pc: int) -> int:
        """pc 处需要的 java_unguarded! 层数 = 不覆盖 pc 的活动组个数。"""
        return sum(1 for g in self._active if not g.covers(pc))

    def unguarded_run(self, idx: int, end_idx: int) -> Optional[tuple]:
        """若 idx 处需要比当前更多的豁免层，返回 (层数增量, 连续段终点下标)。"""
        if not self._active:
            return None
        need = self.unguarded_layers(self._instrs[idx].offset)
        if need <= self._wrapped_layers:
            return None
        j = idx
        while j < end_idx:
            if self.unguarded_layers(self._instrs[j].offset) != need:
                break
            j += 1
        return need - self._wrapped_layers, j

    def push_wrapped(self, layers: int) -> None:
        self._wrapped_layers += layers

    def pop_wrapped(self, layers: int) -> None:
        self._wrapped_layers -= layers



# ── 发射（与结构化器的接口：process_block / make_sub 回调）──────────────

def _throwable_root(registry: dict) -> str:
    """athrow 操作数的静态类型（catch-any 绑定类型）：从 VM 根清单里任一异常类
    沿超类链上溯到根类之下的第一个类。不以字面量出现 JDK 类名。"""
    from ..constants import OBJECT_CLASS
    from ..transpile import _read_manifest
    for line in _read_manifest('vm_roots.txt'):
        cur = line.split('.', 1)[0]
        seen = set()
        while cur in registry and cur not in seen:
            seen.add(cur)
            parent = registry[cur].super_class
            if not parent or parent == OBJECT_CLASS:
                return cur
            cur = parent
    raise RuntimeError("vm_roots.txt 未提供可解析的异常类，无法确定 catch-any 的绑定类型")


def _binding_type(clause: CatchClause, registry: dict) -> str:
    from ..type_map import short_cls
    from ..instr.coerce import _common_ref_type
    if clause.is_catch_any or not clause.catch_types:
        return short_cls(_throwable_root(registry))
    lub = short_cls(clause.catch_types[0])
    for other in clause.catch_types[1:]:
        lub = _common_ref_type(lub, short_cls(other), registry) or short_cls(_throwable_root(registry))
    return lub


def _handler_var_scope_end(method, instrs: list, handler_idx: int) -> Optional[int]:
    first = instrs[handler_idx]
    if not first.opcode.startswith('astore') or handler_idx + 1 >= len(instrs):
        return None
    start_pc = instrs[handler_idx + 1].offset
    for entry in (method.local_vars or []):
        if entry[1] == start_pc:
            return entry[1] + entry[2]
    return None


def emit_unguarded_run(plan: TryCatchPlan, idx: int, end_idx: int, cur_sim, out: list,
                       ind: str, process_block, flush) -> Optional[int]:
    """idx 处若进入不受活动 try 组保护的连续段，发射 java_unguarded! 包裹并返回段终点。"""
    run = plan.unguarded_run(idx, end_idx)
    if run is None:
        return None
    layers, run_end = run
    flush()
    run_out: list = []
    plan.push_wrapped(layers)
    process_block(idx, run_end, cur_sim, run_out, ind + "    ")
    plan.pop_wrapped(layers)
    if run_out:   # 仅含尾部 `goto END` 的段不产生任何语句
        out.append(('', ind + ' '.join(['java_unguarded! {'] * layers)))
        out.extend(run_out)
        out.append(('', ind + ' '.join(['}'] * layers)))
    return run_end


def emit_try_group(plan: TryCatchPlan, group: TryGroup, block_end_idx: int, cur_sim, out: list,
                   ind: str, process_block, make_sub, flush, method, instrs: list,
                   registry: dict) -> int:
    """发射一个 try 语句，返回其后第一条指令的下标。"""
    from ..type_map import short_cls
    from ..rs_ir import LetStmt, Var, RsNamed
    from ..render import render_type, render_expr

    flush()
    out.append(('', f"{ind}java_try! {{"))
    out.append(('', f"{ind}    try {{"))
    body = make_sub()
    body.enter_scope()
    plan.enter_body(group)
    process_block(group.start_idx, group.body_end_idx, body, out, ind + "        ")
    plan.leave_body(group)
    body.exit_scope()
    cur_sim._ctr = max(cur_sim._ctr, body._ctr)

    next_idx = group.body_end_idx
    for pos, clause in enumerate(group.clauses):
        clause_end = plan.clause_end_idx(
            group, pos, block_end_idx,
            _handler_var_scope_end(method, instrs, clause.handler_idx))
        bind_ty = _binding_type(clause, registry)
        bind = cur_sim.fresh('_caught')
        header_pos = len(out)
        out.append(('', ''))   # 占位：处理器体生成后才知道绑定变量名
        handler = make_sub()
        handler._ctr = max(handler._ctr, cur_sim._ctr)
        handler.stack = [(Var(bind), RsNamed(bind_ty))]
        handler.enter_scope()
        process_block(clause.handler_idx, clause_end, handler, out, ind + "        ")
        handler.exit_scope()
        cur_sim._ctr = max(cur_sim._ctr, handler._ctr)

        # 处理器首条 astore 生成的 `let e: T = _caughtN;` 并入 catch 头
        if header_pos + 1 < len(out):
            first = out[header_pos + 1][1]
            if (isinstance(first, LetStmt) and first.value is not None
                    and render_expr(first.value) in (bind, f"Clone::clone(&{bind})")
                    and first.ty is not None
                    and render_type(first.ty) == bind_ty):
                bind = first.name
                del out[header_pos + 1]
        if clause.is_catch_any or not clause.catch_types:
            head = f"catch ({bind})"
        else:
            names = [short_cls(t) for t in clause.catch_types]
            head = f"catch ({bind}: {' | '.join(names)}"
            if len(names) > 1 or names[0] != bind_ty:
                head += f" as {bind_ty}"
            head += ")"
        out[header_pos] = ('', f"{ind}    }} {head} {{")
        next_idx = max(next_idx, clause_end)

    out.append(('', f"{ind}    }}"))
    out.append(('', f"{ind}}}"))
    cur_sim.locals = body.locals
    cur_sim._slot_decl_depth = body._slot_decl_depth
    return next_idx
