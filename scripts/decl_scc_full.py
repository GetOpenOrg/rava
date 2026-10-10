#!/usr/bin/env python3
"""可达集 vs 全集（S7 计划 §9.8 第 1 项）：在 class 文件上建同一口径的类依赖图，比较档案可达集与 java.base 全集的底段。

全集的声明层无法直接发射（`--api-package` 全面闭包在 dev 40 GB 槽位被 OOM 杀，见 §9.8），所以两者都在 class 文件图上算：
  节点 = java.base 的类（`jimage extract --include regex:/java.base/.*` 的产物）；
  边种类与 decl_scc_sim 对齐：inherit（super / interfaces）、sig（方法描述符与 Signature）、field（字段描述符与
  Signature）、exc（Exceptions）、nest（InnerClasses / NestHost / NestMembers / PermittedSubclasses / EnclosingMethod）、
  body（常量池其余 Class / 描述符引用，只计入组合 ①）；
  INFRA 根、伴生边、类声明字节与手写字节取自 decl_scc_sim --json 的 graph_export（按 binary name）。
底段 = INFRA 可达集。先在可达集上与生成文件图（decl_scc_sim）对照底段类数，验证本图口径，再算全集。
体量：可达集外的底段类按可达集平均声明字节外推（可达集外的类没有生成文件），在报告中注明。

用法：
  scripts/decl_scc_full.py <java.base 解包目录> --sim-json <decl_scc_sim --json 输出> [--json out.json]
解包：jimage extract --dir <dir> --include 'regex:/java.base/.*' <JAVA_HOME>/lib/modules（<dir>/java.base/...）
"""
import argparse
import json
import os
import re
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from decl_scc_graph import reach  # noqa: E402

COMBOS = [("①", {"inherit", "sig", "field", "exc", "nest", "body"}), ("②", {"inherit", "sig", "field"}),
          ("③", {"inherit", "field"}), ("④", {"inherit", "sig"}), ("⑤", {"inherit"})]
REF = re.compile(r"L([^;<>]+)[;<]")


def parse(path):
    """返回 (name, {kind: set(binary names)})"""
    d = open(path, "rb").read()
    n = struct.unpack(">H", d[8:10])[0]
    cp, i, k = [None] * n, 10, 1
    while k < n:
        t = d[i]
        if t == 1:
            ln = struct.unpack(">H", d[i + 1:i + 3])[0]
            cp[k] = ("u", d[i + 3:i + 3 + ln].decode("utf-8", "replace"))
            i += 3 + ln
        elif t in (7, 8, 16, 19, 20):
            cp[k] = (t, struct.unpack(">H", d[i + 1:i + 3])[0])
            i += 3
        elif t == 15:
            i += 4
        elif t in (3, 4, 9, 10, 11, 12, 17, 18):
            cp[k] = (t, struct.unpack(">HH", d[i + 1:i + 5]))
            i += 5
        elif t in (5, 6):
            i += 9
            k += 1
        else:
            raise ValueError(f"{path}: cp tag {t}")
        k += 1

    def utf(x):
        return cp[x][1]

    def cls(x):
        return utf(cp[x][1]) if x else None
    out = {kk: set() for kk in ("inherit", "sig", "field", "exc", "nest", "body")}

    def refs(s, kind):
        for m in REF.finditer(s):
            out[kind].add(m.group(1))
    _, this, sup, ni = struct.unpack(">HHHH", d[i:i + 8])
    i += 8
    name = cls(this)
    if sup:
        out["inherit"].add(cls(sup))
    for _ in range(ni):
        out["inherit"].add(cls(struct.unpack(">H", d[i:i + 2])[0]))
        i += 2

    def attrs(i, kind):
        cnt = struct.unpack(">H", d[i:i + 2])[0]
        i += 2
        for _ in range(cnt):
            an, al = struct.unpack(">HI", d[i:i + 6])
            a = utf(an)
            body = d[i + 6:i + 6 + al]
            if a == "Signature":
                refs(utf(struct.unpack(">H", body[:2])[0]), kind)
            elif a == "Exceptions":
                for j in range(struct.unpack(">H", body[:2])[0]):
                    out["exc"].add(cls(struct.unpack(">H", body[2 + 2 * j:4 + 2 * j])[0]))
            elif a in ("NestMembers", "PermittedSubclasses"):
                for j in range(struct.unpack(">H", body[:2])[0]):
                    out["nest"].add(cls(struct.unpack(">H", body[2 + 2 * j:4 + 2 * j])[0]))
            elif a == "NestHost":
                out["nest"].add(cls(struct.unpack(">H", body[:2])[0]))
            elif a == "EnclosingMethod":
                out["nest"].add(cls(struct.unpack(">H", body[:2])[0]))
            elif a == "InnerClasses":
                for j in range(struct.unpack(">H", body[:2])[0]):
                    ic, oc = struct.unpack(">HH", body[2 + 8 * j:6 + 8 * j])
                    for x in (ic, oc):
                        if x:
                            out["nest"].add(cls(x))
            i += 6 + al
        return i
    for kind in ("field", "sig"):
        cnt = struct.unpack(">H", d[i:i + 2])[0]
        i += 2
        for _ in range(cnt):
            _, _, di = struct.unpack(">HHH", d[i:i + 6])
            refs(utf(di), kind)
            i = attrs(i + 6, kind)
    attrs(i, "nest")
    for e in cp:
        if e and e[0] == 7:
            s = utf(e[1])
            out["body"].add(s[s.rfind("[") + 2:-1] if s.startswith("[") else s)
        elif e and e[0] == 12:
            refs(utf(e[1][1]), "body")
    for kk in out:
        out[kk].discard(name)
        out[kk].discard(None)
    return name, out


def load(root):
    nodes = {}
    for dp, _, fs in os.walk(root):
        for f in fs:
            if f.endswith(".class") and f not in ("module-info.class", "package-info.class"):
                name, refs = parse(os.path.join(dp, f))
                nodes[name] = refs
    return nodes


def bottom(nodes, members, keep, ex, infra):
    """INFRA 可达集（与 decl_scc_graph.Graph.adjacency 同口径：d8 → 手写引用类 + 钉住宿主；placed → 基础设施引用类、伴生边挂宿主）"""
    keys = sorted(members)
    idx = {k: i for i, k in enumerate(keys)}
    inf = len(keys)
    adj = [set() for _ in range(inf + 1)]
    for k in keys:
        for kind in keep:
            for t in nodes[k][kind]:
                j = idx.get(t)
                if j is not None:
                    adj[idx[k]].add(j)
    adj[inf] = {idx[r] for r in ex["infra_roots"][infra] if r in idx}
    if infra == "placed":
        for h, js in ex["placed_edges"].items():
            if h in idx:
                adj[idx[h]] |= {idx[j] for j in js if j in idx}
    b = reach(adj, inf) - {inf}
    return {keys[i] for i in b}


def volume(b, ex, infra, avg):
    """底段源码字节：类声明（可达集外的类按可达集均值外推）+ 底段手写（同 decl_scc_sim.seg_volume）"""
    cb = ex["class_bytes"]
    v = sum(cb.get(c, avg) for c in b)
    if infra == "d8":
        return v + ex["hw_bytes_total"]
    return v + ex["hw_bytes_infra"] + sum(ex["hw_bytes_placed"].get(c, 0) for c in b)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("jdk_dir")
    ap.add_argument("--sim-json", required=True)
    ap.add_argument("--json")
    a = ap.parse_args()
    sim = json.load(open(a.sim_json))
    ex = sim["graph_export"]
    fit = sim.get("fit")
    nodes = load(a.jdk_dir)
    cb = ex["class_bytes"]
    reach_set = set(cb) & set(nodes)
    avg = sum(cb[c] for c in reach_set) / max(1, len(reach_set))
    out = {"java_base_classes": len(nodes), "reachable": len(reach_set), "avg_decl_bytes": round(avg),
           "reachable_missing_in_jdk": sorted(set(cb) - set(nodes))[:20], "rows": {}}
    for infra in ("d8", "placed"):
        for name, keep in COMBOS:
            r = {"sim_reach": sim["combos"][f"{name}/{infra}"]["segments"][0]["classes"]}
            for label, mem in (("reach", reach_set), ("full", set(nodes))):
                b = bottom(nodes, mem, keep, ex, infra)
                mb = volume(b, ex, infra, avg) / 1e6
                r[label] = {"classes": len(b), "extrapolated": len(b - set(cb)), "mb": round(mb, 2),
                            "est_mb": round(fit["a"] + fit["b"] * mb) if fit else None}
            out["rows"][f"{name}/{infra}"] = r
    if a.json:
        with open(a.json, "w") as fh:
            json.dump(out, fh, ensure_ascii=False, indent=1)
    print(f"java.base 类 {out['java_base_classes']}，可达集（档案声明层）{out['reachable']}，均值 {out['avg_decl_bytes']} B/类")
    print("| 组合 / INFRA | 生成文件图底段 | class 图底段（可达集） | MB / 估峰 | class 图底段（全集） | 其中外推 | MB / 估峰 |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for k, r in out["rows"].items():
        a_, f_ = r["reach"], r["full"]
        print(f"| {k} | {r['sim_reach']} | {a_['classes']} | {a_['mb']} / {a_['est_mb']} | {f_['classes']} | "
              f"{f_['extrapolated']} | {f_['mb']} / {f_['est_mb']} |")
    return 0


if __name__ == "__main__":
    sys.exit(main())
