"""
结构化：可归约 CFG → 结构树（带标签 block / loop / if / match / break / continue）。

算法：支配树驱动的结构化（Ramsey, "Beyond Relooper", ICFP 2022）的 Rust 适配。
  - 每个节点 Y 有唯一的「放置父节点」：
      * 若存在包含 idom(Y) 而不包含 Y 的循环，取最外层循环头 H：Y 是 H 的 out follower，
        放在 `loop` 之后（Rust 的 loop 落到体尾会重复执行，循环出口后继必须在 loop 之外）
      * 否则父节点为 idom(Y)：前向入边 ≥ 2 → in follower（merge 节点）；否则在分支点内联
  - follower 用带标签 block 包裹其前驱所在的子树，到它的跳转即 `break 'label`
  - 回边即 `continue 'header`
  - try 区域（kind == 'try' 的合成节点 T，见 method/blocks.py）与循环同构：
      * T 的后继 = try 体入口 + 各处理器入口，分别内联为 `Try` 的 try 体与 catch 体
      * 节点 Y 离开受保护区间（idom(Y) 受 T 的组保护而 Y 不受），或 Y 是 try 体与处理器
        之后的汇合点（idom(Y) == T）：Y 是 T 的 try follower，放在 `Try` 之后
        （javac 内联的 finally 副本、try 语句之后的代码因此在词法上位于 try 之外，不受其保护）
      * try 体不正常落出（以 return / athrow 结尾）时，try 语句之后的代码只经由 catch 体可达；
        越过 catch 体文本终点（节点的 catch_ends）的节点同样是 T 的 try follower，
        生成形状与 Java 源码一致（catch 之后的平级代码）
      * 循环与 try 同时满足时取更外层者；循环体内的 try 所保护的出口块留在该 try 之内
    放置完成后逐节点校验「词法所处的 try 组集合 == 异常表给出的 try 组集合」，
    不一致（区间与控制流不成嵌套结构）→ CfgError。
任何跳转都被翻译为 内联 / break / continue 三者之一，不存在「匹配不上」的形态。

结构树随后经过一组保语义的可读性规整（simplify）。
"""

from __future__ import annotations
import os
import sys
from dataclasses import dataclass, field

from .conditions import Cond, negate
from .graph import FlowAnalysis, CfgError, dominates

sys.setrecursionlimit(max(sys.getrecursionlimit(), 20000))


# ─────────────────────────────────────────────────────────────────────────────
# 结构树节点
# ─────────────────────────────────────────────────────────────────────────────

@dataclass
class Code:
    block: int
    exits: bool = False        # 以 return / athrow 结束
    empty: bool = False        # 无语句


@dataclass
class Decl:
    lines: list


@dataclass
class Block:
    label: int
    body: list


@dataclass
class Loop:
    header: int
    body: list
    exit_label: int | None = None      # 合并进来的出口 block 标签：Break(exit_label) = 跳出本循环
    while_cond: Cond | None = None     # 非 None → while 形态
    cond_origin: int | None = None     # while 条件来源块


@dataclass
class If:
    cond: Cond
    then: list
    else_: list
    origin: int


@dataclass
class Switch:
    key: str
    arms: list                 # [(values | None, body)]，None = default
    origin: int


@dataclass
class Try:
    body: list
    catches: list              # [(catch 描述, body)]，描述 = 节点 catches 的元素
    origin: int


@dataclass
class Break:
    label: int


@dataclass
class Continue:
    label: int


# ─────────────────────────────────────────────────────────────────────────────
# 结构化
# ─────────────────────────────────────────────────────────────────────────────

def structure(nodes: dict, flow: FlowAnalysis) -> list:
    """nodes: id → 具有 kind/cond/target/fallthrough/key/cases/default/stmts/decls 属性的块。"""
    if not flow.reducible:
        raise CfgError("structure() 只接受可归约 CFG")

    # 循环按体大小降序（外层在前）
    loops_outer_first = sorted(flow.loops.items(), key=lambda kv: -len(kv[1]))
    in_followers: dict[int, list[int]] = {}
    out_followers: dict[int, list[int]] = {}
    try_followers: dict[int, list[int]] = {}
    follower_set: set[int] = set()

    def ctx_of(x: int) -> frozenset:
        return getattr(nodes[x], 'ctx', frozenset())

    try_nodes = [x for x in flow.rpo if nodes[x].kind == 'try']   # RPO 序 = 外层在前
    try_slots = {x: [nodes[x].target] + list(nodes[x].handlers) for x in try_nodes}

    past_catch: dict[int, set] = {t: set() for t in try_nodes}   # 已位于 t 的 catch 体之后的节点

    def leaves_catch(t: int, d: int, y: int) -> bool:
        if ctx_of(t) != ctx_of(y):
            return False
        ends = getattr(nodes[t], 'catch_ends', None) or []
        for h, end_pc in zip(nodes[t].handlers, ends):
            if end_pc is not None and nodes[y].start_pc >= end_pc > nodes[d].start_pc \
                    and dominates(flow.idom, h, d):
                return True
        return False

    for y in flow.rpo:
        if y == flow.entry:
            continue
        d = flow.idom[y]
        if os.environ.get('JAVA_RTA_CFG_DEBUG'):
            print(f"[cfg-dbg] y={y} pc={nodes[y].start_pc} kind={nodes[y].kind} d={d} "
                  f"ctx_y={sorted(ctx_of(y))} ctx_d={sorted(ctx_of(d))} "
                  f"in_try_slots={y in {s for sl in try_slots.values() for s in sl}}")
        if nodes[d].kind == 'try' and y in try_slots[d]:
            # try 体入口 / 处理器入口：恒内联为 Try 的 try 体 / catch 体
            lexical = ctx_of(d) | {nodes[d].group} if y == nodes[d].target else ctx_of(d)
            if lexical != ctx_of(y):
                raise CfgError(f"块 pc={nodes[y].start_pc} 的 try 区域与控制流不成嵌套结构")
            continue
        parent_loop = None
        for h, body in loops_outer_first:
            # 循环体内的 try 所保护的出口块（try 里的 throw / return 不属于自然循环体）必须留在
            # 该 try 之内：只有词法 try 组集合与 y 一致的循环才能把 y 作为出口后继放到 loop 之后，
            # 否则 y 留在循环内按支配关系就位（y 不可达循环头，其子树不会落回循环体）
            if d in body and y not in body and ctx_of(h) == ctx_of(y):
                if nodes[y].kind == 'exit' and len(flow.preds[y]) == 1:
                    # 单前驱的 return / athrow 块留在循环体内（随其唯一前驱的分支臂
                    # 内联渲染），不提升为循环 follower：提升会把语句搬到循环之后，
                    # 若它词法上位于 catch 体（try 节点经异常边把处理器拉进循环体
                    # 闭包，处理器里的 rethrow athrow 块因此落在 body 外），catch
                    # 绑定变量的引用就落到作用域外（E0425，如
                    # ZoneRulesProvider.<clinit> 的 instanceof→continue / athrow）。
                    # return / athrow 本身终结控制流，单前驱时无需 label 跳转。
                    # 多前驱的 exit 块仍是其他前驱 break 的目标，保持 follower 形态。
                    continue
                parent_loop = h
                break
        parent_try = None
        matched = False
        for t in try_nodes:
            if y in try_slots[t]:
                continue
            g = nodes[t].group
            if (d == t or g in ctx_of(d)) and g not in ctx_of(y) and dominates(flow.idom, t, d):
                matched = True
                # 候选须与 y 同词法层（ctx 相等——follower 放在 t 的 Try 之后，
                # 词法位置即 t 所在层；不等者由放置后的一致性校验兜底报错）。
                # 同层多候选（同一 try 组的不相连受护区间各一个节点，见
                # codegen._split_disjoint_try_ranges 的补装）取支配链最深者：
                # y 离开的是离它最近的区间；取首个（RPO 最外层）会把 follower
                # 挂到别的子树，Break 越过层级界线，simplify 拆掉无引用 Block
                # 后即成悬空标签。支配链上祖先必然 RPO 靠前，max rpo 即最近。
                if ctx_of(t) == ctx_of(y) and (parent_try is None
                                              or flow.rpo_index[t] > flow.rpo_index[parent_try]):
                    parent_try = t
                continue
            if matched:
                continue          # 已有支配匹配：与原「匹配即止」语义一致，不做后续记账
            if d in past_catch[t]:
                past_catch[t].add(y)
            elif leaves_catch(t, d, y):
                # catch 体的文本终点之后：try 体不正常落出时，try 语句之后的代码只经由 catch 体可达，
                # 它仍是 try 语句之后的平级代码，而不是 catch 体的一部分
                past_catch[t].add(y)
                parent_try = t
                break
        if parent_try is not None and parent_loop is not None \
                and flow.rpo_index[parent_loop] <= flow.rpo_index[parent_try]:
            parent_try = None          # 循环更外层（或同一节点：loop 包住 try）
        if parent_try is not None:
            try_followers.setdefault(parent_try, []).append(y)
            follower_set.add(y)
            lexical = ctx_of(parent_try)
        elif parent_loop is not None:
            out_followers.setdefault(parent_loop, []).append(y)
            follower_set.add(y)
            lexical = ctx_of(parent_loop)
        else:
            if flow.forward_in_degree(y) >= 2:
                in_followers.setdefault(d, []).append(y)
                follower_set.add(y)
            lexical = ctx_of(d)
            if nodes[d].kind == 'try':
                raise CfgError(f"控制流绕过 try 入口进入受保护区间（pc={nodes[y].start_pc}）")
        if lexical != ctx_of(y):
            raise CfgError(f"块 pc={nodes[y].start_pc} 的 try 区域与控制流不成嵌套结构"
                           f"（kind={nodes[y].kind} 词法={sorted(lexical)} 实际={sorted(ctx_of(y))}）")
        if os.environ.get('JAVA_RTA_CFG_DEBUG'):
            _which = ('try_follower' if parent_try is not None else
                      'loop_follower' if parent_loop is not None else
                      'in_follower' if y in follower_set else 'inline')
            print(f"[cfg-dbg] y={y} pc={nodes[y].start_pc} kind={nodes[y].kind} d={d} -> {_which} "
                  f"parent_try={parent_try} parent_loop={parent_loop} "
                  f"ctx_y={sorted(ctx_of(y))} ctx_d={sorted(ctx_of(d))}")

    emitted: list[int] = []

    def do_branch(src: int, tgt: int) -> list:
        if flow.is_back_edge(src, tgt):
            return [Continue(tgt)]
        if tgt in follower_set:
            return [Break(tgt)]
        return do_tree(tgt)

    def try_arm(src: int, tgt: int) -> list:
        if tgt in follower_set or flow.is_back_edge(src, tgt):
            raise CfgError(f"try 区域 pc={nodes[src].start_pc} 的体 / 处理器入口 pc={nodes[tgt].start_pc} "
                           f"同时是其他控制流的汇合点")
        return do_tree(tgt)

    def terminator(x: int) -> list:
        n = nodes[x]
        if n.kind == 'exit':
            return []
        if n.kind == 'try':
            body = try_arm(x, n.target)
            return [Try(body, [(c, try_arm(x, h)) for c, h in zip(n.catches, n.handlers)], origin=x)]
        if n.kind == 'goto':
            return do_branch(x, n.target)
        if n.kind == 'cond':
            # 源码顺序：不跳转（fall-through）臂在前
            return [If(negate(n.cond), do_branch(x, n.fallthrough), do_branch(x, n.target), origin=x)]
        if n.kind == 'switch':
            # 同一目标的 case 合并为一个臂；与 default 同目标的 case 由 `_` 臂覆盖。
            # （目标块只有这一个前驱时会被内联，每个目标只能展开一次）
            grouped: dict[int, list] = {}
            for vals, tgt in n.cases:
                if tgt != n.default:
                    grouped.setdefault(tgt, []).extend(vals)
            arms = [(vals, do_branch(x, tgt)) for tgt, vals in grouped.items()]
            arms.append((None, do_branch(x, n.default)))
            return [Switch(n.key, arms, origin=x)]
        raise CfgError(f"未知终结类型 {n.kind}")

    def wrap(inner: list, followers: list[int]) -> tuple[list, list]:
        """followers 按 RPO 升序依次输出；最先输出者的 block 在最内层。
        返回 (汇合变量声明, 包好的序列)。"""
        ordered = sorted(followers, key=lambda f: flow.rpo_index[f])
        decls = [ln for f in ordered for ln in nodes[f].decls]
        body = inner
        for f in ordered:
            body = [Block(f, body)] + do_tree(f)
        return ([Decl(decls)] if decls else []), body

    def do_tree(x: int) -> list:
        emitted.append(x)
        n = nodes[x]
        # 块自身的直线代码不含跳转，放在标签块之外：局部变量的声明因此与 Java 同层可见，
        # 标签块只包住真正需要 break 的分支部分
        code = Code(x, exits=(n.kind == 'exit'), empty=not n.stmts)
        in_decls, branch = wrap(terminator(x), in_followers.get(x, []))
        core = in_decls + [code] + branch
        if x in try_followers:
            try_decls, core = wrap(core, try_followers[x])
            core = try_decls + core
        if x in flow.loops:
            core = [Loop(x, core)]
        out_decls, body = wrap(core, out_followers.get(x, []))
        return out_decls + body

    tree = do_tree(flow.entry)
    # 汇合变量声明：follower 的声明已随其标签块就位；归约后不再是 follower 的节点，声明置于函数顶部
    orphan_decls = [ln for n in flow.rpo if n not in follower_set for ln in nodes[n].decls]
    if orphan_decls:
        tree = [Decl(orphan_decls)] + tree

    if sorted(emitted) != sorted(flow.rpo):
        missing = sorted(set(flow.rpo) - set(emitted))
        dup = sorted({b for b in emitted if emitted.count(b) > 1})
        raise CfgError(f"结构化未恰好输出每个块一次：缺失={missing} 重复={dup}")
    return tree
