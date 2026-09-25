"""
逐块栈模拟 + CFG 级归约。

输入：方法指令序列的基本块 CFG 与一个 StackSim。
输出：归约后的节点图（每个节点 = 一段 Rust 语句 + 一个终结：cond / goto / switch / exit），
      交给 cfg.structure 结构化（不可归约时交给 cfg.dispatch 状态机）。

与 StackSim 的接口：
  - 每个块以「入口状态」（操作数栈 + 局部变量表）启动模拟；入口状态由已处理的前向前驱汇合得到
  - 汇合点上各前驱留下的不同栈值 → 合并变量 `_mergedN`（前驱末尾赋值）
  - 跳转指令由本层解释（弹操作数、构造 Cond），不经过 sim_instr

归约（全部保语义，且每条被吸收的跳转都记入账本）：
  fuse           单前驱直线块并入前驱
  short-circuit  `a && b` / `a || b` 的级联条件块并入前驱条件
  ternary        两臂各留一个值的菱形 → 条件表达式（0/1 两臂 → 布尔表达式）
  const-fold     编译期常量条件 → 无条件边（死臂不再模拟）
"""

from __future__ import annotations
import re
from dataclasses import dataclass, field

from ..cfg import (
    CfgError, build_blocks, analyze, reachable,
    COND_BRANCH_OPS, GOTO_OPS, SWITCH_OPS, JUMP_OPS,
    Cond, render_cond,
    JumpLedger,
)
from ..instr import sim_instr
from ..render import render_expr, render_stmt, render_type
from ..rs_ir import LetStmt, AssignStmt, RawExpr, RawStmt, RsNamed, Var
from ..stack import StackSim
from .vars import _str_to_rs_type
from .try_catch import TryCatchPlan, _binding_type
from .fusion import try_fuse, try_short_circuit, try_ternary
from .unify import (
    jump_condition, arm_value, unify_values, inline_temps, _same_entry, _same_stack, _parse_temp_let,
)


# ─────────────────────────────────────────────────────────────────────────────
# 数据模型
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class Node:
    id: int
    start_pc: int
    kind: str                          # 'cond' | 'goto' | 'switch' | 'exit' | 'try'
    target: int | None = None
    fallthrough: int | None = None
    cases: list = field(default_factory=list)
    default: int | None = None
    cond: Cond | None = None
    key: str = ''
    pcs: list = field(default_factory=list)      # 本节点终结所承载的跳转指令 pc
    stmts: list = field(default_factory=list)
    decls: list = field(default_factory=list)    # 本节点入口合并变量的声明行
    entry_stack: list = field(default_factory=list)
    entry_locals: dict = field(default_factory=dict)
    exit_stack: list = field(default_factory=list)
    exit_locals: dict = field(default_factory=dict)
    processed: bool = False
    removed: bool = False
    # try 区域：ctx = 覆盖本节点的 try 组编号集合；kind == 'try' 的合成节点另有
    # target = try 体入口、handlers = 各 catch 子句的处理器入口、catches = [(clause, 绑定名, 绑定类型)]
    ctx: frozenset = frozenset()
    group: int | None = None
    handlers: list = field(default_factory=list)
    catches: list = field(default_factory=list)
    catch_ends: list = field(default_factory=list)   # 各 catch 体的文本终点 pc（None = 未知）

    def successors(self) -> list[int]:
        if self.kind == 'try':
            out = [self.target]
            for h in self.handlers:
                if h not in out:
                    out.append(h)
            return out
        if self.kind == 'cond':
            return [self.target] if self.target == self.fallthrough else [self.target, self.fallthrough]
        if self.kind == 'goto':
            return [self.target]
        if self.kind == 'switch':
            out: list[int] = []
            for _, tgt in self.cases:
                if tgt not in out:
                    out.append(tgt)
            if self.default not in out:
                out.append(self.default)
            return out
        return []


@dataclass
class SimResult:
    nodes: dict
    entry: int
    dispatch: bool
    top_decls: list = field(default_factory=list)   # 状态机模式下提升到函数顶部的声明行


# ─────────────────────────────────────────────────────────────────────────────
# 模拟器
# ─────────────────────────────────────────────────────────────────────────────

class BlockSimulator:
    def __init__(self, method, sim: StackSim, registry, class_tparams, ledger: JumpLedger):
        self.method = method
        self.instrs = method.instrs
        self.sim = sim
        self.registry = registry
        self.class_tparams = list(class_tparams or [])
        self.ledger = ledger
        self.plan = TryCatchPlan(method.exception_table, self.instrs, getattr(method, 'local_vars', None))
        self.blocks = build_blocks(self.instrs, method.exception_table, self.plan.catch_body_ends())
        self.nodes: dict[int, Node] = {}

    # ── 入口 ────────────────────────────────────────────────────────────────

    def run(self) -> SimResult:
        blocks = self.blocks
        staged: dict[int, Node] = {}
        for b in blocks:
            t = b.term
            kind = 'goto' if t.kind == 'fall' else t.kind
            staged[b.id] = Node(
                id=b.id, start_pc=b.start_pc, kind=kind, target=t.target,
                fallthrough=t.fallthrough, cases=[(list(v), tg) for v, tg in t.cases],
                default=t.default, pcs=[t.pc] if t.pc is not None else [])
        self._install_try_nodes(staged)

        raw_reach = reachable(self.entry, {nid: n.successors() for nid, n in staged.items()})
        self._register_jumps(raw_reach)
        self._thread_jumps(staged, raw_reach)

        succs0 = {nid: n.successors() for nid, n in staged.items()}
        flow0 = analyze(self.entry, succs0)
        self.flow0 = flow0
        live = set(flow0.rpo)
        for nid in flow0.rpo:
            self.nodes[nid] = staged[nid]
        for nid, n in staged.items():
            # 线程化后失去全部前驱的纯 goto 块：其跳转已由前驱的边承载
            if nid in raw_reach and nid not in live:
                self._consume(n.pcs, 'structured')

        if not flow0.reducible:
            if self.handler_bind:
                raise CfgError("含异常处理器的不可归约 CFG")
            return self._run_dispatch()
        return self._run_structured()

    # ── try 区域 ────────────────────────────────────────────────────────────

    def _install_try_nodes(self, staged: dict) -> None:
        """异常表 → CFG：每个 try 组一个合成节点 T（kind='try'）。

        T 的后继 = try 体入口 + 各处理器入口：处理器由此成为 CFG 的真实块。
        进入受保护区间的边（源不在该区间内）改指 T；区间内部回到体入口的边保持原样。
        同一起点的多个组由外到内串成 T_outer → T_inner → 体入口。
        """
        blocks = self.blocks
        self.entry = 0
        self.handler_bind: dict[int, tuple] = {}     # 处理器入口节点 → (T, 绑定名, 绑定类型)
        self.try_entries: set[int] = set()           # try 体入口块（不参与跳转线程化）
        self.catch_exits: set[int] = set()           # catch 体文本终点处的块（不并入 catch 体）
        plan = self.plan
        planned = {c.handler_pc for g in plan.groups for c in g.clauses}
        for start_pc, _end, handler_pc, _ct in (self.method.exception_table or []):
            if start_pc < handler_pc and handler_pc not in planned:
                raise CfgError(f"异常处理器 pc={handler_pc} 无法归入任何 try 区域")
        if not plan.groups:
            return
        gid = {id(g): k for k, g in enumerate(plan.groups)}
        for b in blocks:
            staged[b.id].ctx = frozenset(gid[id(g)] for g in plan.groups if g.covers(b.start_pc))
        self._adopt_bare_returns(staged)
        by_start_idx = {b.start_idx: b.id for b in blocks}
        by_start_pc = {b.start_pc: b.id for b in blocks}
        self.catch_exits = {by_start_pc[pc] for pc in plan.catch_body_ends()}

        chains: dict[int, list[Node]] = {}           # 体入口块 → [T_outer, ..., T_inner]
        next_id = len(blocks)
        for start_idx, groups in plan.groups_by_start().items():
            body = by_start_idx[start_idx]
            self.try_entries.add(body)
            chain: list[Node] = []
            inner = set()
            for g in reversed(groups):               # 内 → 外：ctx(T_g) 不含 g 及其内层同起点组
                inner.add(gid[id(g)])
                t = Node(id=0, start_pc=blocks[body].start_pc, kind='try',
                         ctx=staged[body].ctx - inner)
                t.group = gid[id(g)]
                for clause in g.clauses:
                    t.handlers.append(by_start_idx[clause.handler_idx])
                    t.catch_ends.append(clause.body_end_pc)
                    t.catches.append((clause, self.sim.fresh('_caught'),
                                      _binding_type(clause, self.registry)))
                chain.insert(0, t)
            for t in chain:
                t.id = next_id
                next_id += 1
                staged[t.id] = t
            for t, nxt in zip(chain, chain[1:] + [None]):
                t.target = nxt.id if nxt is not None else body
            chains[body] = chain

        def redirect(src: Node, tgt):
            for t in chains.get(tgt, ()):
                if t.group not in src.ctx and t is not src:
                    return t.id
            return tgt

        for n in list(staged.values()):
            if n.kind == 'try':
                n.handlers = [redirect(n, h) for h in n.handlers]
                continue
            n.target = redirect(n, n.target)
            n.fallthrough = redirect(n, n.fallthrough)
            n.default = redirect(n, n.default)
            n.cases = [(v, redirect(n, tg)) for v, tg in n.cases]
        for t in (n for n in staged.values() if n.kind == 'try'):
            for h, (_clause, bind, bind_ty) in zip(t.handlers, t.catches):
                if h in self.handler_bind:
                    raise CfgError(f"处理器 pc={staged[h].start_pc} 被多个 try 区域共用")
                self.handler_bind[h] = (t.id, bind, bind_ty)
        if 0 in chains:
            self.entry = chains[0][0].id

    def _adopt_bare_returns(self, staged: dict) -> None:
        """javac 把 `try { return f(); }` 的受保护区间收在 xreturn 之前（返回指令不会抛出异常）。
        只含一条返回指令、且只有一个前驱的块随其前驱归入同一组 try 区域：
        return 留在 try 体内，与 Java 源码形状一致；语义不变。"""
        preds: dict[int, list[int]] = {}
        for n in staged.values():
            for s in n.successors():
                preds.setdefault(s, []).append(n.id)
        for b in self.blocks:
            if b.term.kind != 'exit' or b.end_idx - b.start_idx != 1 \
                    or not self.instrs[b.start_idx].opcode.endswith('return'):
                continue
            sources = preds.get(b.id, [])
            if len(sources) == 1:      # 处理器入口此时尚无前驱（try 节点还未安装），不会被收编
                staged[b.id].ctx = staged[sources[0]].ctx

    def _thread_jumps(self, staged: dict, raw_reach: set) -> None:
        """跳转线程化：只含一条 goto 的块不产生任何代码，指向它的边直接改指其最终目标。
        （`break` 经由中转 goto 到达循环出口时，&& / || 的两个出口才会落在同一目标上。）"""
        trampoline: dict[int, int] = {}
        for b in self.blocks:
            if b.id != 0 and b.id not in self.try_entries and b.id not in self.handler_bind \
                    and b.end_idx - b.start_idx == 1 and b.term.kind == 'goto' \
                    and b.term.pc is not None and b.term.target != b.id:
                trampoline[b.id] = staged[b.id].target      # 已含 try 入口改指

        def resolve(nid: int) -> int:
            seen = set()
            while nid in trampoline and nid not in seen:
                seen.add(nid)
                nid = trampoline[nid]
            return nid if nid not in trampoline else -1

        final = {nid: resolve(nid) for nid in trampoline}
        final = {nid: tgt for nid, tgt in final.items() if tgt != -1}
        if not final:
            return
        for n in staged.values():
            if n.target in final:
                n.target = final[n.target]
            if n.fallthrough in final:
                n.fallthrough = final[n.fallthrough]
            if n.default in final:
                n.default = final[n.default]
            n.cases = [(v, final.get(tg, tg)) for v, tg in n.cases]

    def _register_jumps(self, reach: set) -> None:
        """登记全部跳转指令；不可达块（含处理器保护自身的重试条目所指向的死代码）记为 dead。"""
        for b in self.blocks:
            for i in range(b.start_idx, b.end_idx):
                ins = self.instrs[i]
                if ins.opcode not in JUMP_OPS:
                    continue
                self.ledger.expect(ins.offset)
                if b.id not in reach:
                    self.ledger.consume(ins.offset, 'dead')

    # ── 图查询 ──────────────────────────────────────────────────────────────

    def _all_preds(self, nid: int) -> list[Node]:
        return [p for p in self.nodes.values() if not p.removed and nid in p.successors()]

    def _live_preds(self, nid: int) -> list[Node]:
        return [self.nodes[p] for p in self.flow0.rpo
                if self.nodes[p].processed and not self.nodes[p].removed
                and nid in self.nodes[p].successors()]

    # ── 单块模拟 ────────────────────────────────────────────────────────────

    def _simulate(self, node: Node) -> None:
        if node.kind == 'try':
            # 合成节点没有指令：出口状态 = 入口状态
            node.exit_stack = list(node.entry_stack)
            node.exit_locals = dict(node.entry_locals)
            node.processed = True
            return
        sim = self.sim
        blk = self.blocks[node.id]
        instrs = self.instrs
        sim.stack = list(node.entry_stack)
        sim.locals = dict(node.entry_locals)
        sim.stmts = []
        last = blk.end_idx - 1
        for i in range(blk.start_idx, blk.end_idx):
            ins = instrs[i]
            sim.current_offset = ins.offset
            sim.next_offset = instrs[i + 1].offset if i + 1 < len(instrs) else 0
            op = ins.opcode
            if i == last and op in JUMP_OPS:
                if op in COND_BRANCH_OPS:
                    node.cond = jump_condition(op, sim, self.registry)
                elif op in SWITCH_OPS:
                    key_e, key_t = sim.pop()
                    key_s = render_expr(key_e)
                    node.key = key_s if render_type(key_t) == 'i32' else f"(({key_s}) as i32)"
                break
            sim_instr(ins, sim, self.method.class_name, registry=self.registry)
        if sim.underflow_occurred:
            raise CfgError(f"操作数栈下溢（块 pc={node.start_pc}）")
        node.stmts = sim.stmts
        node.exit_stack = list(sim.stack)
        node.exit_locals = dict(sim.locals)
        sim.stmts = []
        node.processed = True

    # ── 入口状态汇合 ────────────────────────────────────────────────────────

    def _merge_entry(self, node: Node, preds: list[Node]) -> None:
        first = preds[0]
        depth = len(first.exit_stack)
        for p in preds[1:]:
            if len(p.exit_stack) != depth:
                raise CfgError(f"汇合点 pc={node.start_pc} 各前驱栈深不一致")
        stack = []
        for k in range(depth):
            column = [p.exit_stack[k] for p in preds]
            if all(_same_entry(column[0], c) for c in column[1:]):
                stack.append(column[0])
                continue
            values, ty = unify_values(column, self.class_tparams, self.registry)
            name = self.sim.fresh('_merged')
            for p, v in zip(preds, values):
                p.stmts.append(RawStmt(f"{name} = {v};"))
            # 前置声明以 LetStmt 进入 entries：合并值在其声明所在块之外被消费时
            # （如 try 体内汇合、try 之后 return），由变量提升 pass 移到外层
            node.decls.append(LetStmt(name, ty, True, None))
            stack.append((Var(name), ty))
        node.entry_stack = stack

        locals_ = dict(first.exit_locals)
        idom_node = self.nodes.get(self.flow0.idom.get(node.id))
        for slot in list(locals_):
            name, ty, _ = locals_[slot]
            _dom_used = False
            for p in preds[1:]:
                other = p.exit_locals.get(slot)
                if other is None or other[0] != name:
                    del locals_[slot]
                    break
                if render_type(other[1]) != render_type(ty) and idom_node is not None and idom_node.processed:
                    dom_entry = idom_node.exit_locals.get(slot)
                    if dom_entry is not None and dom_entry[0] == name:
                        locals_[slot] = dom_entry
                        _dom_used = True
            else:
                if _dom_used:
                    continue
                # 汇合点各前驱给同名局部不同引用类型、支配者处尚无该绑定（兄弟分支各自首绑定，
                # 如 switch 各臂存入同一接口的不同实现类）：按 JVM 校验器合并语义取公共祖先，
                # 无公共类祖先回退根类——与变量提升阶段的合并类型（vars._merged_slot_type）一致，
                # 汇合后的调用点按合并类型发射（此前沿用首前驱的具体类，提升后变量已是根类 →
                # 方法调用 E0599，TestBranchLocalMerge 实证）
                if slot in locals_:
                    merged = self._join_ref_type(slot, preds)
                    if merged is not None:
                        locals_[slot] = (locals_[slot][0], merged, locals_[slot][2])
        node.entry_locals = locals_

    def _join_ref_type(self, slot: int, preds: list):
        """前驱出口同名局部的引用类型全等 → None（不变）；否则公共类祖先 / 根类。
        任一侧为基本类型或无类型 → None（不在本规则内）。"""
        from ..constants import PRIMITIVE_RUST_TYPES as _PRIM
        from ..instr.hierarchy import _common_ref_type_widening
        seen: list[str] = []
        for p in preds:
            ent = p.exit_locals.get(slot)
            if ent is None or ent[1] is None:
                return None
            r = render_type(ent[1])
            if r in _PRIM or r == '()':
                return None
            if r not in seen:
                seen.append(r)
        if len(seen) < 2:
            return None
        # 与提升阶段同一判据（vars._all_alignable）：其余类型均可按值侧对齐到首个类型
        #（子类型 / 接口载体实现类）时维持首前驱类型，不退化为根类
        from .vars import _forms_alignable
        if all(_forms_alignable(t, seen[0], self.registry) for t in seen[1:]):
            return None
        common = seen[0]
        for other in seen[1:]:
            common = _common_ref_type_widening(common, other, self.registry)
            if common is None:
                return RsNamed('Object')
        return RsNamed(common)

    # ── 归约 ────────────────────────────────────────────────────────────────

    def _consume(self, pcs: list, kind: str) -> None:
        for pc in pcs:
            self.ledger.consume(pc, kind)

    def _fold_const(self, node: Node) -> None:
        if node.kind != 'cond':
            return
        if node.cond.is_const:
            node.target = node.target if node.cond.value else node.fallthrough
            node.kind, node.cond, node.fallthrough = 'goto', None, None
            self._consume(node.pcs, 'const-fold')
            node.pcs = []
        elif node.target == node.fallthrough:
            # 两臂同一目标：条件只为副作用求值
            node.stmts.append(RawStmt(f"let _ = {render_cond(node.cond)};"))
            node.kind, node.cond, node.fallthrough = 'goto', None, None
            self._consume(node.pcs, 'structured')
            node.pcs = []

    # ── 可归约路径 ──────────────────────────────────────────────────────────

    def _run_structured(self) -> SimResult:
        nodes = self.nodes
        flow0 = self.flow0
        entry = self.entry
        live: set[int] = {entry}
        for nid in flow0.rpo:
            node = nodes[nid]
            if nid not in live:
                self._consume(node.pcs, 'dead')
                node.removed = True
                continue
            if nid != entry:
                try_ternary(self, nid)
            preds = self._live_preds(nid)
            if nid == entry:
                node.entry_stack, node.entry_locals = [], dict(self.sim.locals)
            elif nid in self.handler_bind:
                # 处理器入口：操作数栈只有被捕获的异常对象；局部变量表取 try 入口处的状态
                t_id, bind, bind_ty = self.handler_bind[nid]
                if [p.id for p in preds] != [t_id]:
                    raise CfgError(f"异常处理器 pc={node.start_pc} 同时是正常控制流的目标")
                node.entry_stack = [(Var(bind), RsNamed(bind_ty))]
                node.entry_locals = dict(preds[0].exit_locals)
            elif not preds:
                raise CfgError(f"活块 pc={node.start_pc} 没有已处理的前驱")
            elif len(preds) == 1:
                node.entry_stack = list(preds[0].exit_stack)
                node.entry_locals = dict(preds[0].exit_locals)
            else:
                self._merge_entry(node, preds)
            self._simulate(node)
            self._fold_const(node)
            for s in node.successors():
                live.add(s)
                target = nodes[s]
                if target.processed and not target.removed and s != nid \
                        and not _same_stack(node.exit_stack, target.entry_stack):
                    raise CfgError(f"回边 pc={node.start_pc}→{target.start_pc} 两端操作数栈不一致")
            if node.id in node.successors() and not _same_stack(node.exit_stack, node.entry_stack):
                raise CfgError(f"自环 pc={node.start_pc} 两端操作数栈不一致")
            node = try_fuse(self, node)
            try_short_circuit(self, node)

        self._inline_loop_header_temps()
        self._promote_cross_block_temps()
        kept = {n.id: n for n in nodes.values() if n.processed and not n.removed}
        for h, (t_id, _bind, _ty) in self.handler_bind.items():
            if h in kept and any(h in n.successors() and n.id != t_id for n in kept.values()):
                raise CfgError(f"异常处理器 pc={kept[h].start_pc} 同时是正常控制流的目标")
        return SimResult(nodes=kept, entry=entry, dispatch=False)

    def _inline_loop_header_temps(self) -> None:
        """循环头若只由可回填的临时变量构成，回填进条件（使 `while cond {` 形态成立）。"""
        kept = {n.id: n for n in self.nodes.values() if n.processed and not n.removed}
        flow = analyze(self.entry, {i: n.successors() for i, n in kept.items()})
        for h in flow.loops:
            node = kept[h]
            if node.kind != 'cond' or not node.stmts or node.decls:
                continue
            cond = inline_temps(node.stmts, node.cond, node.exit_stack)
            if cond is not None:
                node.cond, node.stmts = cond, []

    def _promote_cross_block_temps(self) -> None:
        """在某节点声明、被其他节点引用的临时变量：RawStmt → LetStmt，交给变量提升 pass 管理作用域。"""
        kept = [n for n in self.nodes.values() if n.processed and not n.removed]
        declared: dict[str, tuple[Node, int]] = {}
        for n in kept:
            for k, stmt in enumerate(n.stmts):
                parsed = _parse_temp_let(stmt)
                if parsed is not None:
                    declared[parsed[0]] = (n, k)
        if not declared:
            return
        name_re = re.compile(r'\b_[A-Za-z]\w*?\d+\b')
        for n in kept:
            parts = [render_stmt(s) for s in n.stmts]
            if n.cond is not None:
                parts.append(render_cond(n.cond))
            parts.append(n.key)
            for name in set(name_re.findall('\n'.join(parts))):
                owner = declared.get(name)
                if owner is None or owner[0] is n:
                    continue
                o_node, k = owner
                parsed = _parse_temp_let(o_node.stmts[k])
                if parsed is None:
                    continue
                _, mutable, ty_text, value = parsed
                ty = _str_to_rs_type(ty_text) if ty_text else None
                o_node.stmts[k] = LetStmt(name, ty, mutable, RawExpr(value))

    # ── 状态机路径（不可归约 CFG）───────────────────────────────────────────

    def _run_dispatch(self) -> SimResult:
        nodes = self.nodes
        sim = self.sim
        spill: dict[int, list] = {}
        top_decls: list[str] = []
        entry_locals: dict[int, dict] = {0: dict(sim.locals)}

        for nid in self.flow0.rpo:
            node = nodes[nid]
            node.entry_stack = list(spill.get(nid, []))
            node.entry_locals = dict(entry_locals.get(nid, {}))
            self._simulate(node)
            values = []
            for expr, ty in node.exit_stack:
                if len(node.successors()) > 1 and not isinstance(expr, Var):
                    tmp = sim.fresh('_spill')
                    node.stmts.append(RawStmt(f"let {tmp} = {render_expr(expr)};"))
                    expr = Var(tmp)
                values.append((expr, ty))
            for s in node.successors():
                if s not in spill:
                    spill[s] = [(Var(f"__s{s}_{k}"), ty) for k, (_, ty) in enumerate(values)]
                    for var, ty in spill[s]:
                        top_decls.append(f"let mut {var.name}: {render_type(ty)} = Default::default();")
                    entry_locals[s] = dict(node.exit_locals)
                if len(spill[s]) != len(values):
                    raise CfgError(f"状态机：pc={nodes[s].start_pc} 各前驱栈深不一致")
                for (var, _), entry in zip(spill[s], values):
                    node.stmts.append(RawStmt(f"{var.name} = {arm_value(entry)};"))
            self._consume(node.pcs, 'dispatch')

        self._promote_cross_block_temps()
        hoisted: dict[str, str | None] = {}
        for node in nodes.values():
            new_stmts = []
            for stmt in node.stmts:
                if not isinstance(stmt, LetStmt):
                    new_stmts.append(stmt)
                    continue
                ty_s = render_type(stmt.ty) if stmt.ty is not None else None
                if ty_s is None:
                    for lname, lty, _ in node.exit_locals.values():
                        if lname == stmt.name:
                            ty_s = render_type(lty)
                            break
                prev = hoisted.get(stmt.name)
                if prev is not None and ty_s is not None and prev != ty_s:
                    raise CfgError(f"状态机：变量 {stmt.name} 类型冲突 {prev} / {ty_s}")
                hoisted[stmt.name] = prev or ty_s
                if stmt.value is not None:
                    new_stmts.append(AssignStmt(Var(stmt.name), stmt.value))
            node.stmts = new_stmts
        for name, ty_s in hoisted.items():
            ann = f": {ty_s}" if ty_s else ""
            top_decls.append(f"let mut {name}{ann} = Default::default();")
        return SimResult(nodes=dict(nodes), entry=0, dispatch=True, top_decls=top_decls)


def simulate_blocks(method, sim: StackSim, registry, class_tparams, ledger: JumpLedger) -> SimResult:
    return BlockSimulator(method, sim, registry, class_tparams, ledger).run()
