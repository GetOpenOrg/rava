#!/usr/bin/env python3
"""声明层底段 SCC 模拟（S7 计划 §9.8）：在 D8 的同一张图上删边 / 改边，求底段与分段，按体量估峰值。

用法：
  scripts/decl_scc_sim.py <scratch> --runtime <runtime/java_runtime/src> [--root java_base]
                          [--calib 源码MB=峰值MB ...] [--hubs N] [--cuts N] [--json out.json]

<scratch>：`rava build --stop-after emit` 的产物目录（含 `<root>_decl/src` 与上层段 `<root>_decl_<k>/src`）。
--runtime：发射该 scratch 的提交的手写真源（D8 读 `ctx.runtime_src()`，即仓库 runtime/java_runtime/src）。
--calib：同口径实测点（crate_mem_profile 报告的「源码 MB」与「峰值 MB」），≥2 个时线性拟合并给每段估峰。

图（与 generator/crates/emit/src/project/decl_segments.rs 逐项一致，见 decl_scc_graph.py）：
  节点 = 声明层各生成类文件（含已移到上层段的）+ 伪节点 INFRA；每类 → INFRA；
  INFRA → 手写文件正文标识符（含 `__` 词干）命中的类 + 伴生 `_impl` / `_ext` 宿主（钉底）；
  类 → 类 = 文本中 `crate::` 路径 + 字符串字面量按 `;` 拆出的 binary name。
  先用此图复现实际分段，与 scratch 各段文件集合逐类比对（不一致则退出码 2）。

边的种类（按引用出现处的上下文；一条边可有多种，删边须删去其全部种类）：
  inherit 继承属性（super_class / interfaces / all_supertypes / declared_by …）、trait impl 头、iface_upcasts!、
          派生名 `X__VTable` / `X__m_base`；sig 方法签名；field 字段 / static / const 类型；
  body 留在声明层的方法体；nest 嵌套属性；exc 方法 exceptions 属性；attr 其他属性字符串；
  macro use 引入而正文未出现（宏展开用名）；top 类块外的其他宏。
INFRA 口径：d8 = 现状；placed = 伴生手写的引用归宿主类、宿主不钉底（固有 impl 与宿主同 crate）。
咽喉（decl_scc_choke.py）：类入口 / 边入口的支配子树规模，及贪心逐个切边入口的累计收缩。
组合：① 全部；② 只留 inherit + sig + field（S7 路线一）；③ ② − sig；④ ② − field；⑤ 只留 inherit（标记 crate 路线）。
"""
import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from decl_scc_choke import entry_rows, greedy_cuts  # noqa: E402
from decl_scc_graph import KINDS, Graph, load_handwritten, load_nodes, plan, reach, tarjan  # noqa: E402

COMBOS = [
    ("①", set(KINDS)),
    ("②", {"inherit", "hidden", "sig", "field"}),
    ("③", {"inherit", "hidden", "field"}),
    ("④", {"inherit", "hidden", "sig"}),
    ("⑤", {"inherit", "hidden"}),
]


def fit(points):
    """最小二乘 peak = a + b·MB；返回 (a, b, 最大绝对残差, 最大相对残差)"""
    if len(points) < 2:
        return None
    n = len(points)
    mx = sum(x for x, _ in points) / n
    my = sum(y for _, y in points) / n
    sxx = sum((x - mx) ** 2 for x, _ in points)
    b = sum((x - mx) * (y - my) for x, y in points) / sxx if sxx else 0.0
    a = my - b * mx
    res = [y - (a + b * x) for x, y in points]
    return a, b, max(abs(r) for r in res), max(abs(r) / y for r, (_, y) in zip(res, points))


def seg_volume(g, seg, bottom, infra):
    """段源码字节：类文件 + 本段手写（底段：d8 全部手写；placed 只基础设施手写）+ placed 下伴生随宿主"""
    v = sum(g.size[i] for i in seg)
    if infra == "placed":
        v += sum(g.hw_bytes_placed.get(i, 0) for i in seg)
        if bottom:
            v += g.hw_bytes_infra
    elif bottom:
        v += g.hw_bytes_total
    return v


def simulate(g, keep, infra, calib):
    adj, inf = g.adjacency(keep, infra)
    segs = plan(g, adj, inf)
    comp, _ = tarjan_no_infra(g, keep, infra)
    sizes = {}
    for c in comp:
        sizes[c] = sizes.get(c, 0) + 1
    rows = []
    for k, seg in enumerate(segs):
        vol = seg_volume(g, seg, k == 0, infra) / 1e6
        est = calib[0] + calib[1] * vol if calib else None
        rows.append({"seg": k, "classes": len(seg), "mb": round(vol, 2), "est_mb": round(est) if est else None})
    return {"segments": rows, "largest_scc_no_infra": max(sizes.values()) if sizes else 0}


def tarjan_no_infra(g, keep, infra):
    adj, inf = g.adjacency(keep, infra)
    adj = [set(a) - {inf} for a in adj[:inf]]
    comp, nc = tarjan(adj)
    return comp, nc


def bottom_size(g, keep, infra):
    adj, inf = g.adjacency(keep, infra)
    return len(reach(adj, inf)) - 1


def bottom_kind_stats(g, keep, infra):
    adj, inf = g.adjacency(keep, infra)
    bot = reach(adj, inf) - {inf}
    cnt = {k: 0 for k in KINDS}
    for (i, j), ks in g.ekind.items():
        if i in bot and j in bot and ks & keep:
            for k in ks & keep:
                cnt[k] += 1
    return cnt


def chokes(g, keep, infra, top, rounds, calib):
    adj, inf = g.adjacency(keep, infra)
    r = entry_rows(g, adj, inf, top)

    def vol(bot):
        return seg_volume(g, bot, True, infra)
    r["greedy"] = greedy_cuts(g, adj, inf, rounds, vol)
    if calib:
        for x in r["greedy"]:
            x["est_mb"] = round(calib[0] + calib[1] * x["mb"])
    return r


def inherit_cycles(g):
    """只留 inherit 边、不加 INFRA（按实际放置：边记在写出该 impl / 属性的文件）的非单点 SCC"""
    adj, inf = g.adjacency({"inherit"}, "placed")
    adj = [set(a) - {inf} for a in adj[:inf]]
    for h in g.placed_edges:  # 手写伴生的引用不是继承边
        adj[h] = {j for j in adj[h] if "inherit" in g.ekind.get((h, j), ())}
    comp, nc = tarjan(adj)
    groups = {}
    for v, c in enumerate(comp):
        groups.setdefault(c, []).append(v)
    out = []
    for c, vs in sorted(groups.items(), key=lambda x: -len(x[1])):
        if len(vs) < 2:
            break
        s = set(vs)
        edges = [(g.keys[i], g.keys[j], sorted(g.ekind[(i, j)])) for i in vs for j in sorted(adj[i]) if j in s]
        out.append({"classes": [g.keys[v] for v in vs], "edges": edges})
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("scratch")
    ap.add_argument("--runtime", required=True)
    ap.add_argument("--root", default="java_base")
    ap.add_argument("--calib", nargs="*", default=[])
    ap.add_argument("--hubs", type=int, default=25, help="类 / 边入口各列前 N")
    ap.add_argument("--cuts", type=int, default=10, help="贪心逐个切边入口的轮数")
    ap.add_argument("--json")
    a = ap.parse_args()
    nodes = load_nodes(a.scratch, a.root)
    g = Graph(nodes, load_handwritten(a.runtime))
    pts = [tuple(float(x) for x in c.split("=")) for c in a.calib]
    f = fit(pts)
    out = {"classes": g.n, "edges": len(g.ekind), "hw_mb": round(g.hw_bytes_total / 1e6, 2),
           "hw_infra_mb": round(g.hw_bytes_infra / 1e6, 2), "pinned_hosts": len(g.pinned),
           "infra_refs_d8": len(g.infra_d8), "infra_refs_placed": len(g.infra_placed),
           "attr_keys": dict(sorted(g.attr_keys.items())),
           "implicit": dict(sorted(g.implicit.items()))}
    if f:
        out["fit"] = {"a": round(f[0]), "b": round(f[1], 1), "max_abs_res": round(f[2]), "max_rel_res": round(f[3], 3),
                      "points": pts, "budget_mb_for_1300": round((1300 - f[0]) / f[1], 2) if f[1] else None}
    # 复现校验
    adj, inf = g.adjacency(set(KINDS), "d8")
    segs = plan(g, adj, inf, a.root)
    actual = {}
    for i, x in enumerate(nodes):
        actual.setdefault(x[4], set()).add(i)
    # D8 按体量切（plan 内含不分段判定）；手写类节点权重 0、只连 INFRA、恒在底段，不影响上段
    sim = {k: set(s) for k, s in enumerate(segs)} if len(segs) > 1 else {0: set(range(g.n))}
    ok = all(sim.get(k, set()) == actual.get(k, set()) for k in set(sim) | set(actual))
    out["reproduce"] = {"ok": ok, "actual": {k: len(v) for k, v in sorted(actual.items())},
                        "simulated": {k: len(v) for k, v in sorted(sim.items())}}
    if not ok:
        diff = sorted(g.keys[i] for k in sim for i in sim[k] ^ actual.get(k, set()))[:40]
        out["reproduce"]["diff_sample"] = diff
    calib = f[:2] if f else None
    out["combos"] = {}
    for infra in ("d8", "placed"):
        for name, keep in COMBOS:
            r = simulate(g, keep, infra, calib)
            r["bottom_edge_kinds"] = bottom_kind_stats(g, keep, infra)
            out["combos"][f"{name}/{infra}"] = r
    out["single_kind_removal"] = {infra: {k: bottom_size(g, set(KINDS) - {k}, infra) for k in KINDS}
                                  for infra in ("d8", "placed")}
    out["only_kind_plus_inherit"] = {infra: {k: bottom_size(g, {"inherit", k}, infra) for k in KINDS}
                                     for infra in ("d8", "placed")}
    out["chokes"] = {f"{n}/{infra}": chokes(g, keep, infra, a.hubs, a.cuts, calib)
                     for infra in ("d8", "placed") for n, keep in COMBOS if n in ("①", "②", "⑤")}
    out["inherit_cycles"] = inherit_cycles(g)
    # 供 decl_scc_full（class 文件图）复用：类声明字节、INFRA 根与手写字节（按 binary name）
    k = g.keys
    out["graph_export"] = {
        "class_bytes": {k[i]: g.size[i] for i in range(g.n)},
        "infra_roots": {"d8": sorted(k[i] for i in g.infra_d8 | g.pinned), "placed": sorted(k[i] for i in g.infra_placed)},
        "placed_edges": {k[h]: sorted(k[j] for j in js) for h, js in sorted(g.placed_edges.items())},
        "hw_bytes_total": g.hw_bytes_total, "hw_bytes_infra": g.hw_bytes_infra,
        "hw_bytes_placed": {k[h]: b for h, b in sorted(g.hw_bytes_placed.items())}}
    if a.json:
        with open(a.json, "w") as fh:
            json.dump(out, fh, ensure_ascii=False, indent=1)
    report(out)
    return 0 if ok else 2


def report(o):
    p = print
    p(f"类 {o['classes']}，类边 {o['edges']}；手写 {o['hw_mb']} MB（其中基础设施 {o['hw_infra_mb']} MB），"
      f"伴生宿主 {o['pinned_hosts']}，INFRA 直连（d8 / placed）{o['infra_refs_d8']} / {o['infra_refs_placed']}")
    p(f"宏隐式引用（use 引入、正文未出现，按属性串归类）：{o['implicit']}")
    r = o["reproduce"]
    p(f"复现：{'一致' if r['ok'] else '不一致'}  实际 {r['actual']}  模拟 {r['simulated']}")
    if "fit" in o:
        f = o["fit"]
        p(f"峰值拟合：峰值 MB = {f['a']} + {f['b']} × 源码 MB；最大残差 {f['max_abs_res']} MB（{f['max_rel_res']:.1%}）；"
          f"1.3 GB 对应源码 ≤ {f['budget_mb_for_1300']} MB")
    p("\n| 组合 / INFRA | 底段类 | 底段源码 MB | 底段估峰 MB | 段数 | 最大上段（类 / MB / 估峰） | 无 INFRA 最大 SCC |")
    p("|---|---:|---:|---:|---:|---|---:|")
    for k, c in o["combos"].items():
        s = c["segments"]
        up = max(s[1:], key=lambda x: x["mb"]) if len(s) > 1 else None
        upt = f"{up['classes']} / {up['mb']} / {up['est_mb']}" if up else "—"
        p(f"| {k} | {s[0]['classes']} | {s[0]['mb']} | {s[0]['est_mb']} | {len(s)} | {upt} | {c['largest_scc_no_infra']} |")
    for infra, d in o["single_kind_removal"].items():
        p(f"\n单删一种（{infra}，底段类数）：" + "，".join(f"{k} {v}" for k, v in d.items()))
    for infra, d in o["only_kind_plus_inherit"].items():
        p(f"只留 inherit + 一种（{infra}）：" + "，".join(f"{k} {v}" for k, v in d.items()))
    for k, c in o["chokes"].items():
        p(f"\n咽喉（{k}）：底段 {c['bottom']} 类，直接受 INFRA 支配 {c['idom_infra']}、受某类支配 {c['idom_class']}；"
          f"INFRA 支配子树前 1/5/10/25 覆盖 {c['infra_children_cover_top_1_5_10_25']}")
        p("  类入口（切断指向它的全部引用，底段少几类；* = INFRA 直连）：" +
          "；".join(f"{key} {n}{'*' if tag else ''}" for n, key, tag in c["class_entries"]))
        p("  边入口（只切一条引用）：" + "；".join(f"{u}→{v} {n} {'/'.join(ks)}" for n, u, v, ks in c["edge_entries"]))
        p("  逐个切（累计）：" + "；".join(f"{x['cut'][0]}→{x['cut'][1]} −{x['gain']} ⇒ {x['bottom']} 类 {x['mb']} MB"
                                     + (f" ≈{x['est_mb']}" if 'est_mb' in x else "") for x in c["greedy"]))
    cyc = o["inherit_cycles"]
    p(f"\n只留 inherit（按放置）非单点 SCC：{len(cyc)} 个" + ("" if not cyc else "；最大 " + str(len(cyc[0]["classes"]))))
    for c in cyc[:5]:
        p("  " + "，".join(c["classes"][:12]))
        for e in c["edges"][:12]:
            p(f"    {e[0]} → {e[1]} {e[2]}")


if __name__ == "__main__":
    sys.exit(main())
