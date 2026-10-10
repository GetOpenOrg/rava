#!/usr/bin/env python3
"""底段扇出咽喉（S7 计划 §9.8 第 2 项）：支配树求「切掉一个入口，底段少多少类」（`decl_scc_sim.py` 用）。

底段 = INFRA 出发可达集。切入口的两种粒度：
  类入口：断开底段内指向类 X 的全部引用 → 底段失去 X 的支配子树（含 X）；
  边入口：只断开一条引用 u → v → 底段失去该边中点在「边拆点图」上的支配子树。
逐个切（贪心累计）：每轮取当前收缩最大的边入口切掉，重算底段，记录累计收缩、体量与估峰。
"""
from decl_scc_graph import reach


def dominators(adj, root):
    """root 出发可达子图的直接支配者与支配子树规模（Cooper–Harvey–Kennedy 迭代）"""
    order, seen, st = [], {root}, [(root, iter(sorted(adj[root])))]
    while st:
        v, it = st[-1]
        w = next((w for w in it if w not in seen), None)
        if w is None:
            order.append(v)
            st.pop()
            continue
        seen.add(w)
        st.append((w, iter(sorted(adj[w]))))
    rpo = order[::-1]
    num = {v: i for i, v in enumerate(rpo)}
    preds = {v: [] for v in rpo}
    for v in rpo:
        for w in adj[v]:
            if w in preds:
                preds[w].append(v)
    idom = {root: root}

    def meet(a, b):
        while a != b:
            while num[a] > num[b]:
                a = idom[a]
            while num[b] > num[a]:
                b = idom[b]
        return a
    changed = True
    while changed:
        changed = False
        for v in rpo[1:]:
            ps = [p for p in preds[v] if p in idom]
            d = ps[0]
            for p in ps[1:]:
                d = meet(p, d)
            if idom.get(v) != d:
                idom[v] = d
                changed = True
    return idom, rpo


def subtree(idom, rpo, weight):
    sub = {v: weight(v) for v in rpo}
    for v in reversed(rpo[1:]):
        sub[idom[v]] += sub[v]
    return sub


def class_entries(adj, inf):
    """类入口：支配子树规模（类数）"""
    idom, rpo = dominators(adj, inf)
    return idom, subtree(idom, rpo, lambda v: 0 if v == inf else 1)


def edge_entries(adj, inf, n):
    """边入口：每条底段边 (u, v) 拆成 u → m → v，m 的支配子树类数即切该边的收缩。返回 {(u, v): 收缩}"""
    bot = reach(adj, inf)
    split = {v: [] for v in bot}
    mids = {}
    for u in bot:
        for v in adj[u]:
            if v in bot and v != inf:
                m = n + 1 + len(mids)
                mids[m] = (u, v)
                split[u].append(m)
                split[m] = [v]
    idom, rpo = dominators(split, inf)
    sub = subtree(idom, rpo, lambda v: 1 if v < n else 0)  # 只计类节点（INFRA = n、中点 > n）
    return {mids[m]: sub[m] for m in mids if sub.get(m, 0) > 0}


def entry_rows(g, adj, inf, top):
    """类入口与边入口各取前 top；边入口附该边的引用种类"""
    idom, sub = class_entries(adj, inf)
    roots = set(adj[inf])
    cls = sorted(((sub[v], g.keys[v], v in roots) for v in sub if v != inf), key=lambda r: (-r[0], r[1]))[:top]
    edges = edge_entries(adj, inf, g.n)
    er = sorted(((k, "INFRA" if u == inf else g.keys[u], g.keys[v], sorted(g.ekind.get((u, v), ())))
                 for (u, v), k in edges.items()), key=lambda r: (-r[0], r[1], r[2]))[:top]
    direct = sum(1 for v in sub if v != inf and idom[v] == inf)
    total = len(sub) - 1
    kids = sorted((sub[v] for v in sub if v != inf and idom[v] == inf), reverse=True)
    cover = [sum(kids[:k]) for k in (1, 5, 10, 25)]
    return {"bottom": total, "idom_infra": direct, "idom_class": total - direct,
            "infra_children_cover_top_1_5_10_25": cover, "class_entries": cls, "edge_entries": er}


def greedy_cuts(g, adj, inf, rounds, volume):
    """逐个切边入口（每轮取收缩最大者），记录累计底段类数与体量"""
    adj = [set(a) for a in adj]
    rows = []
    for _ in range(rounds):
        edges = edge_entries(adj, inf, g.n)
        if not edges:
            break
        (u, v), k = min(edges.items(), key=lambda x: (-x[1], x[0]))
        adj[u].discard(v)
        bot = reach(adj, inf) - {inf}
        rows.append({"cut": ("INFRA" if u == inf else g.keys[u], g.keys[v]),
                     "kinds": sorted(g.ekind.get((u, v), ())), "gain": k,
                     "bottom": len(bot), "mb": round(volume(bot) / 1e6, 2)})
    return rows
