#!/usr/bin/env python3
"""生成树依赖图的强连通分量统计（拆 crate 可行性评估，见 docs/plans/2026-10-01-rustc-memory-and-crate-split.md）。

依赖边取生成文件的 `use crate::<路径>::<类型>;` 行，解析到定义该 `pub struct` 的生成文件；
只统计带 java_class 生成标记的文件。输出文件级 / 包级最大 SCC 的规模与泛型占比。

用法：python3 scripts/dep_scc.py build/<test>/java_runtime/src [--dump 最大SCC文件清单路径]
"""
import argparse
import collections
import re
import sys
from pathlib import Path

USE_RE = re.compile(r'^use crate::((?:\w+::)+)(\w+);', re.M)
STRUCT_RE = re.compile(r'pub struct (\w+)')
GENERIC_STRUCT_RE = re.compile(r'pub struct \w+<')
FN_RE = re.compile(r'\bfn \w+')
GENERIC_FN_RE = re.compile(r'\bfn \w+<')
GEN_MARK = 'rava_macros::java_class'


def load(src: Path):
    """→ (文件 → 行数, 文件 → 依赖文件集, 文件 → 包路径)。"""
    texts = {}
    for p in src.rglob('*.rs'):
        t = p.read_text(errors='replace')
        if GEN_MARK in t:
            texts[p] = t
    pkg = {p: '::'.join(p.parent.relative_to(src).parts) for p in texts}
    defs = {}
    for p, t in texts.items():
        for m in STRUCT_RE.finditer(t):
            defs[(pkg[p], m.group(1))] = p
    graph = {p: set() for p in texts}
    for p, t in texts.items():
        for m in USE_RE.finditer(t):
            q = defs.get((m.group(1).rstrip(':'), m.group(2)))
            if q is not None and q != p:
                graph[p].add(q)
    return {p: t.count('\n') for p, t in texts.items()}, graph, pkg


def tarjan(graph):
    """迭代式 Tarjan，返回按规模降序的 SCC 列表。"""
    idx, low, on, st, out, n = {}, {}, set(), [], [], 0
    for s in graph:
        if s in idx:
            continue
        idx[s] = low[s] = n; n += 1; st.append(s); on.add(s)
        work = [(s, iter(graph[s]))]
        while work:
            v, it = work[-1]
            for w in it:
                if w not in idx:
                    idx[w] = low[w] = n; n += 1; st.append(w); on.add(w)
                    work.append((w, iter(graph[w])))
                    break
                if w in on:
                    low[v] = min(low[v], idx[w])
            else:
                work.pop()
                if work:
                    low[work[-1][0]] = min(low[work[-1][0]], low[v])
                if low[v] == idx[v]:
                    comp = []
                    while True:
                        w = st.pop(); on.discard(w); comp.append(w)
                        if w == v:
                            break
                    out.append(comp)
    return sorted(out, key=len, reverse=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument('src', type=Path, help='生成树 java_runtime/src')
    ap.add_argument('--dump', type=Path, help='最大 SCC 文件清单输出路径')
    a = ap.parse_args()
    lines, graph, pkg = load(a.src)
    if not graph:
        sys.exit(f'{a.src} 下没有生成文件')
    n, total = len(graph), sum(lines.values())
    sccs = tarjan(graph)
    big = sccs[0]
    print(f'生成文件 {n}，行 {total}，依赖边 {sum(len(v) for v in graph.values())}')
    print(f'最大 SCC：{len(big)} 文件（{len(big) * 100 // n}%），'
          f'{sum(lines[p] for p in big) * 100 // total}% 行；单点 SCC {sum(1 for c in sccs if len(c) == 1)}')
    pgraph = collections.defaultdict(set)
    for p, qs in graph.items():
        for q in qs:
            if pkg[p] != pkg[q]:
                pgraph[pkg[p]].add(pkg[q])
    for k in set(pkg.values()):
        pgraph.setdefault(k, set())
    pbig = tarjan(pgraph)[0]
    print(f'包 {len(pgraph)} 个；最大包级 SCC {len(pbig)} 个包，'
          f'覆盖 {sum(1 for p in graph if pkg[p] in set(pbig)) * 100 // n}% 文件')
    all_text = ''.join(p.read_text(errors='replace') for p in graph)
    print(f'泛型 struct {len(GENERIC_STRUCT_RE.findall(all_text))} / {len(STRUCT_RE.findall(all_text))}，'
          f'fn {len(FN_RE.findall(all_text))}（其中自带泛型参数 {len(GENERIC_FN_RE.findall(all_text))}）')
    if a.dump:
        a.dump.write_text('\n'.join(sorted(str(p.relative_to(a.src)) for p in big)) + '\n')


if __name__ == '__main__':
    main()
