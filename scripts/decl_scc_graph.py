#!/usr/bin/env python3
"""声明层 D8 图的建图、Tarjan 与 D8 同款分段（`decl_scc_sim.py` 的库部分；说明见该脚本）。"""
import os
import re
from collections import defaultdict

from decl_scc_parse import (CAP, IDENT, KINDS, Ctx, crate_paths, hw_idents, kind_of,  # noqa: F401
                            stem, string_literals)


# ── 图 ─────────────────────────────────────────────────────────────────────────

MARKER = "rava_macros::java_class"
STRUCT = re.compile(r"pub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)")
BIN = re.compile(r'#\[binary_name\s*=\s*"([^"]+)"\]')


def load_nodes(scratch, root):
    """[(key, pkg, ident, rel, seg, text)]：各段生成类文件；seg 0 = 底段"""
    nodes = []
    pat = re.compile(r"^%s_decl(?:_(\d+))?$" % re.escape(root))
    for d in sorted(os.listdir(scratch)):
        m = pat.match(d)
        if not m:
            continue
        seg = int(m.group(1) or 0)
        src = os.path.join(scratch, d, "src")
        for dp, _, fs in os.walk(src):
            for f in sorted(fs):
                if not f.endswith(".rs"):
                    continue
                p = os.path.join(dp, f)
                text = open(p, encoding="utf-8", errors="replace").read()
                if MARKER not in text:
                    continue
                b, s = BIN.search(text), STRUCT.search(text)
                if not b or not s:
                    continue
                rel = os.path.relpath(p, src)
                pkg = "::".join(os.path.dirname(rel).split(os.sep)) if os.path.dirname(rel) else ""
                nodes.append((b.group(1), pkg, s.group(1), rel, seg, text))
    nodes.sort(key=lambda x: x[0])
    return nodes


def load_handwritten(runtime_src):
    out = []
    for dp, _, fs in os.walk(runtime_src):
        for f in sorted(fs):
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                out.append((os.path.relpath(p, runtime_src), open(p, encoding="utf-8", errors="replace").read()))
    out.sort()
    return out


def companion_hosts(rel):
    name = os.path.basename(rel)
    for suf in ("_impl.rs", "_ext.rs"):
        if name.endswith(suf):
            base = name[: -len(suf)]
            d = os.path.dirname(rel)
            return [os.path.join(d, base + ".rs"), os.path.join(d, base + "_t.rs")]
    return []


class Graph:
    def __init__(self, nodes, hw):
        self.nodes = nodes
        self.n = len(nodes)
        self.keys = [x[0] for x in nodes]
        self.size = [len(x[5].encode("utf-8")) for x in nodes]
        by_path = {(x[1], x[2]): i for i, x in enumerate(nodes)}
        by_ident = defaultdict(list)
        for i, x in enumerate(nodes):
            by_ident[x[2]].append(i)
        by_key = {x[0]: i for i, x in enumerate(nodes)}
        by_rel = {x[3]: i for i, x in enumerate(nodes)}
        self.keys_ = self.keys

        def resolve(path):
            for k in range(1, len(path)):
                pkg, ident = "::".join(path[:k]), path[k]
                j = by_path.get((pkg, ident))
                if j is None:
                    j = by_path.get((pkg, stem(ident)))
                if j is not None:
                    return j
            return None
        # 类边：(i, j) → {种类}；属性键明细
        self.ekind = defaultdict(set)
        self.implicit = defaultdict(int)  # use 引入、正文未出现、按属性串归类的边数
        self.attr_keys = defaultdict(int)
        for i, x in enumerate(nodes):
            self._class_edges(i, x[5], resolve, by_key)
        # 手写：D8 口径（全部挂 INFRA）与按放置口径（伴生归宿主）
        self.infra_d8, self.infra_placed = set(), set()
        self.hw_bytes_total = 0
        self.hw_bytes_placed = defaultdict(int)  # 宿主 → 伴生字节
        self.placed_edges = defaultdict(set)
        self.pinned = set()
        for rel, text in hw:
            b = len(text.encode("utf-8"))
            self.hw_bytes_total += b
            targets = set()
            for ident in hw_idents(text):
                for cand in (ident, stem(ident)):
                    targets.update(by_ident.get(cand, ()))
            self.infra_d8 |= targets
            hosts = [by_rel[h] for h in companion_hosts(rel) if h in by_rel]
            self.pinned.update(hosts)
            if hosts:
                h = hosts[0]
                self.hw_bytes_placed[h] += b
                for j in targets:
                    if j != h:
                        self.placed_edges[h].add(j)
            else:
                self.infra_placed |= targets
        self.hw_bytes_infra = self.hw_bytes_total - sum(self.hw_bytes_placed.values())

    def _class_edges(self, i, text, resolve, by_key):
        ctx = Ctx(text)
        local = {}  # use 引入的本地名 → 目标
        for pos, path in crate_paths(text):
            j = resolve(path)
            if j is None or j == i:
                continue
            ck = ctx.kind_at(pos)
            if ck == "use":
                stmt_end = text.find(";", pos)
                stmt = text[pos:stmt_end if stmt_end > 0 else len(text)]
                name = path[-1]
                m = re.search(r"\b%s\s+as\s+([A-Za-z_][A-Za-z0-9_]*)" % re.escape(name), stmt)
                local[m.group(1) if m else name] = j
            else:
                kind, key = kind_of(ck)
                self._add(i, j, kind, key)
        lit_refs = defaultdict(set)  # 属性串里提到的 binary name → {(种类, 键)}
        for pos, lit in string_literals(text):
            kk = kind_of(ctx.kind_at(pos))
            for tok in re.split(r"[;,:()\[\]<>]", lit):
                lit_refs[tok[1:] if tok.startswith("L") and tok[1:] in by_key else tok].add(kk)
        if local:
            seen = defaultdict(set)
            for m in IDENT.finditer(text):
                j = local.get(m.group(0))
                if j is None:
                    continue
                if ctx.noncode[m.start()]:
                    continue
                ck = ctx.kind_at(m.start())
                if ck == "use":
                    continue
                seen[m.group(0)].add(kind_of(ck))
            for name, j in local.items():
                derived = "__" in name.lstrip("_")
                if derived and (name.endswith("__VTable") or name.endswith("_base")):
                    self._add(i, j, "inherit", None)
                    continue
                if not seen.get(name):
                    if derived:
                        self._add(i, j, "inherit", None)
                        continue
                    # 宏按属性串展开用名：取提到该类的属性种类（inner_classes → nest，exceptions → exc …）
                    hits = lit_refs.get(self.keys_[j], ())
                    for kind, key in hits or [("macro", None)]:
                        if kind == "attr" and key in ("descriptor", "generic_signature", "bridge_to"):
                            kind = "sig"  # 宏按描述符 / 泛型签名展开的签名类型
                        self.implicit[kind] += 1
                        self._add(i, j, kind, key)
                    continue
                for kind, key in seen[name]:
                    self._add(i, j, kind, key)
        for pos, lit in string_literals(text):
            ck = ctx.kind_at(pos)
            kind, key = kind_of(ck)
            for b in lit.split(";"):
                j = by_key.get(b)
                if j is not None and j != i:
                    self._add(i, j, kind, key)

    def _add(self, i, j, kind, key):
        self.ekind[(i, j)].add(kind)
        if key:
            self.attr_keys[key] += 1

    def adjacency(self, keep, infra="d8", drop_nodes=()):
        """keep：保留的种类集合；返回 (邻接表, INFRA 下标)。节点 n 为 INFRA"""
        n = self.n
        adj = [set() for _ in range(n + 1)]
        for (i, j), ks in self.ekind.items():
            if ks & keep:
                adj[i].add(j)
        if infra == "d8":
            adj[n] |= self.infra_d8 | self.pinned
        else:
            adj[n] |= self.infra_placed
            for h, js in self.placed_edges.items():
                adj[h] |= js
        for v in drop_nodes:
            adj[v] = set()
            for a in adj:
                a.discard(v)
        for i in range(n):
            adj[i].add(n)
        return adj, n


def reach(adj, s):
    seen, st = {s}, [s]
    while st:
        v = st.pop()
        for w in adj[v]:
            if w not in seen:
                seen.add(w)
                st.append(w)
    return seen


def tarjan(adj):
    n = len(adj)
    index, low, comp = [-1] * n, [0] * n, [-1] * n
    on, stack, nxt, nc = [False] * n, [], 0, 0
    order = [sorted(a) for a in adj]
    for r in range(n):
        if index[r] >= 0:
            continue
        call = [(r, 0)]
        index[r] = low[r] = nxt
        nxt += 1
        stack.append(r)
        on[r] = True
        while call:
            v, ei = call[-1]
            if ei < len(order[v]):
                call[-1] = (v, ei + 1)
                w = order[v][ei]
                if index[w] < 0:
                    index[w] = low[w] = nxt
                    nxt += 1
                    stack.append(w)
                    on[w] = True
                    call.append((w, 0))
                elif on[w]:
                    low[v] = min(low[v], index[w])
                continue
            call.pop()
            if call:
                u = call[-1][0]
                low[u] = min(low[u], low[v])
            if low[v] == index[v]:
                while True:
                    w = stack.pop()
                    on[w] = False
                    comp[w] = nc
                    if w == v:
                        break
                nc += 1
    return comp, nc


def plan(g, adj, infra):
    """D8 `plan`：返回段列表（段 0 为底段，元素为类下标）"""
    import heapq
    n = g.n
    comp, nc = tarjan(adj)
    members = defaultdict(list)
    for v in range(n):
        members[comp[v]].append(v)
    deps, dependents, min_key = defaultdict(set), defaultdict(set), {}
    for v in range(n + 1):
        c = comp[v]
        k = g.keys[v] if v < n else ""
        if c not in min_key or k < min_key[c]:
            min_key[c] = k
        for w in adj[v]:
            d = comp[w]
            if d != c and d not in deps[c]:
                deps[c].add(d)
                dependents[d].add(c)
    pending = {c: len(deps[c]) for c in range(nc)}
    heap = [(min_key[c], c) for c in range(nc) if pending[c] == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        _, c = heapq.heappop(heap)
        order.append(c)
        for d in sorted(dependents[c]):
            pending[d] -= 1
            if pending[d] == 0:
                heapq.heappush(heap, (min_key[d], d))
    bottom = comp[infra]
    rest = [c for c in order if c != bottom]
    sizes = [len(members[c]) for c in rest]
    total = sum(sizes)
    if total == 0:
        return [sorted(range(n))]
    k = max(1, min(-(-total // CAP), len(sizes)))
    bins, prefix, last = [], 0, 0
    for s in sizes:
        mid = prefix + s // 2
        prefix += s
        b = min(mid * k // total, k - 1)
        b = 0 if not bins else max(last, min(b, last + 1))
        bins.append(b)
        last = b
    segs = [list(members[bottom])] + [[] for _ in range(max(bins) + 1)]
    for c, b in zip(rest, bins):
        segs[b + 1].extend(members[c])
    return segs
