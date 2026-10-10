//! 根模块声明层分段（D8）：按签名引用的强连通分量把声明层切成多个 crate。
//!
//! Rust crate 之间不得循环依赖，固有 impl 必须与 struct 同 crate，故同一 SCC 的类只能同段。
//! 本模块是纯分析：输入各类的声明层文本与手写文件文本，输出分段（段 0 为底段）。
//!
//! - **图**：节点是根模块各类，另加伪节点 INFRA 代表手写基础设施。每类 → INFRA（`Object`、
//!   `Result`、GIL 等任何类都可能用到）；INFRA → 手写文件正文出现的类（按标识符保守匹配）
//!   与钉底的类（共置手写伴生文件的宿主、手写类文件）。类 → 类的边取声明层文本里的
//!   `crate::<包>::<名>` 路径（含 `use` 花括号组与 `X__VTable` 等派生名）。
//! - **规划**：Tarjan 求 SCC，凝聚图按「被依赖者在前」做 Kahn 拓扑序（同层按最小键名优先，确定性）；
//!   含 INFRA 的 SCC 是唯一汇点，为底段（不可再切，原样保留）；其余分量按拓扑序连续切段，
//!   每段源码体量（各类权重之和）≤ 上限，段数取最少，在最少段数下再使最大段最小（均衡），
//!   第 j 段只依赖 < j 的段，各段经镜像链（段 j 只依赖段 j−1）对外呈现完整视图。
//! - 整个声明层（含手写基础量）体量不超过上限、或底段之外无类时不分段（与现状布局一致）。
//!
//! 分段只依赖类集合、引用关系与各类权重（权重 = 落盘源码字节的保守上界，由调用方给出；
//! 上限为常量，不随机器内存变化），与 hash 种子、遍历顺序无关。

use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::cmp::Reverse;

/// 每个声明段 crate 的源码体量上限（字节，按 10^6 计 MB 的 5.5 MB）。
///
/// 标定来源：S7 计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` §9.8.3 第 1 条与
/// §9.8.1「标定修正（10-11，s7d-prof1-f0b2c9de）」——现形态声明 crate 峰值对源码体量分段线性插值，
/// 1.3 GB 约对应 5.9 MB 源码（实测点 7.39 MB → 1.46 GB），扣插值误差留约 7% 余量取 5.5 MB。
/// 峰值由展开后 AST/HIR/MIR 规模决定（rustc 拆分计划 §7.8），按体量而非类数切分
pub const DECL_SEGMENT_BYTES: usize = 5_500_000;

/// 声明层的一个类节点
#[derive(Debug, Clone)]
pub struct DeclNode<'a> {
    /// 确定性排序键（binary name）
    pub key: &'a str,
    /// Rust 包路径（`java::lang::ref`，不带 `r#`）
    pub pkg: String,
    /// 类在包内的 Rust 声明名
    pub ident: String,
    /// 声明层文本（生成类）；手写类为空
    pub text: &'a str,
}

/// 声明层引用图：节点 0..n 为类，n 为 INFRA
#[derive(Debug, Clone, Default)]
pub struct DeclGraph {
    keys: Vec<String>,
    edges: Vec<Vec<usize>>,
}

impl DeclGraph {
    /// 由类节点、手写文件文本（基础设施与共置伴生，引用一律挂 INFRA）与钉底类下标建图
    pub fn build(nodes: &[DeclNode<'_>], handwritten: &[&str], pinned: &[usize]) -> DeclGraph {
        let n = nodes.len();
        let mut by_path: HashMap<(&str, &str), usize> = HashMap::new();
        let mut by_ident: HashMap<&str, Vec<usize>> = HashMap::new();
        let by_key: HashMap<&str, usize> = nodes.iter().enumerate().map(|(i, nd)| (nd.key, i)).collect();
        for (i, nd) in nodes.iter().enumerate() {
            by_path.insert((nd.pkg.as_str(), nd.ident.as_str()), i);
            by_ident.entry(nd.ident.as_str()).or_default().push(i);
        }
        let mut edges: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n + 1];
        for (i, nd) in nodes.iter().enumerate() {
            edges[i].insert(n);
            for path in crate_paths(nd.text) {
                if let Some(j) = resolve(&path, &by_path).filter(|&j| j != i) {
                    edges[i].insert(j);
                }
            }
            // 宏属性里的 binary name（`super_class` / `all_supertypes` / `virtual_in` 等）：宏展开按它生成路径
            for lit in string_literals(nd.text) {
                for j in lit.split(';').filter_map(|b| by_key.get(b).copied()).filter(|&j| j != i) {
                    edges[i].insert(j);
                }
            }
        }
        for text in handwritten {
            for id in idents(text) {
                for cand in [id.as_str(), stem(&id)] {
                    if let Some(js) = by_ident.get(cand) {
                        edges[n].extend(js.iter().copied());
                    }
                }
            }
        }
        edges[n].extend(pinned.iter().copied().filter(|&p| p < n));
        DeclGraph {
            keys: nodes.iter().map(|nd| nd.key.to_string()).collect(),
            edges: edges.into_iter().map(|s| s.into_iter().collect()).collect(),
        }
    }

    fn infra(&self) -> usize {
        self.keys.len()
    }

    fn key(&self, v: usize) -> &str {
        self.keys.get(v).map(String::as_str).unwrap_or("")
    }
}

/// 分段结果：`segments[0]` 为底段（含全部钉底类），段内按键名排序；不分段时只有一段。
///
/// `weights[v]` 为类 v 的体量权重；`base` 为底段 crate 在类之外的固定体量（手写文件等），只参与
/// 「是否分段」判定；`cap` 为每段体量上限
pub fn plan(g: &DeclGraph, weights: &[usize], base: usize, cap: usize) -> Vec<Vec<usize>> {
    let n = g.keys.len();
    let all = || {
        let mut v: Vec<usize> = (0..n).collect();
        v.sort_by(|&a, &b| g.key(a).cmp(g.key(b)));
        v
    };
    let weight = |v: usize| weights.get(v).copied().unwrap_or(0);
    let total: usize = (0..n).map(weight).sum();
    if base.saturating_add(total) <= cap {
        return vec![all()];
    }
    let (comp, ncomp) = tarjan(&g.edges);
    let order = topo_deps_first(g, &comp, ncomp);
    let bottom = comp[g.infra()];
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); ncomp];
    for v in 0..n {
        members[comp[v]].push(v);
    }
    let rest: Vec<usize> = order.into_iter().filter(|&c| c != bottom && !members[c].is_empty()).collect();
    if rest.is_empty() {
        return vec![all()];
    }
    let sizes: Vec<usize> = rest.iter().map(|&c| members[c].iter().map(|&v| weight(v)).sum()).collect();
    let bins = cut(&sizes, cap);
    let k = bins.last().map_or(0, |b| b + 1);
    let mut segs: Vec<Vec<usize>> = vec![Vec::new(); k + 1];
    segs[0] = members[bottom].clone();
    for (c, b) in rest.iter().zip(bins) {
        segs[b + 1].extend(members[*c].iter().copied());
    }
    for s in &mut segs {
        s.sort_by(|&a, &b| g.key(a).cmp(g.key(b)));
    }
    segs
}

/// 贪心连续装箱：当前箱非空且装入后超过 `bound` 即开新箱（单项超过 `bound` 时独占一箱）
fn greedy(sizes: &[usize], bound: usize) -> Vec<usize> {
    let (mut bin, mut load) = (0usize, 0usize);
    let mut out = Vec::with_capacity(sizes.len());
    for (i, &s) in sizes.iter().enumerate() {
        if i > 0 && load.saturating_add(s) > bound {
            bin += 1;
            load = 0;
        }
        load += s;
        out.push(bin);
    }
    out
}

/// 连续切段：段数取上限 `cap` 下的最少段数 k（贪心即最优），再二分求 k 段内最小的段体量上界并按其贪心切，
/// 使各段均衡。每段 ≤ `cap`（单个分量本身超过 `cap` 时独占一段）；段号单调不减、无空段
fn cut(sizes: &[usize], cap: usize) -> Vec<usize> {
    let cap = cap.max(1);
    let count = |b: &[usize]| b.last().map_or(0, |x| x + 1);
    let k = count(&greedy(sizes, cap));
    let total: usize = sizes.iter().sum();
    let (mut lo, mut hi) = (total.div_ceil(k.max(1)).min(cap), cap);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(&greedy(sizes, mid)) <= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    greedy(sizes, hi)
}

/// 迭代式 Tarjan：返回 (各节点分量号, 分量数)
fn tarjan(edges: &[Vec<usize>]) -> (Vec<usize>, usize) {
    const UNSET: usize = usize::MAX;
    let n = edges.len();
    let (mut index, mut low, mut comp) = (vec![UNSET; n], vec![0usize; n], vec![UNSET; n]);
    let mut on_stack = vec![false; n];
    let (mut stack, mut next, mut ncomp) = (Vec::new(), 0usize, 0usize);
    for root in 0..n {
        if index[root] != UNSET {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(root, 0)];
        index[root] = next;
        low[root] = next;
        next += 1;
        stack.push(root);
        on_stack[root] = true;
        while let Some(&mut (v, ref mut ei)) = call.last_mut() {
            if let Some(&w) = edges[v].get(*ei) {
                *ei += 1;
                if index[w] == UNSET {
                    index[w] = next;
                    low[w] = next;
                    next += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    call.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
                continue;
            }
            call.pop();
            if let Some(&(u, _)) = call.last() {
                low[u] = low[u].min(low[v]);
            }
            if low[v] == index[v] {
                while let Some(w) = stack.pop() {
                    on_stack[w] = false;
                    comp[w] = ncomp;
                    if w == v {
                        break;
                    }
                }
                ncomp += 1;
            }
        }
    }
    (comp, ncomp)
}

/// 凝聚图拓扑序，被依赖者在前；同时就绪的分量按最小键名优先
fn topo_deps_first(g: &DeclGraph, comp: &[usize], ncomp: usize) -> Vec<usize> {
    let mut pending = vec![0usize; ncomp];
    let mut dependents: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); ncomp];
    let mut deps: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); ncomp];
    let mut min_key: Vec<Option<&str>> = vec![None; ncomp];
    for (v, outs) in g.edges.iter().enumerate() {
        let c = comp[v];
        let k = g.key(v);
        if min_key[c].is_none_or(|m| k < m) {
            min_key[c] = Some(k);
        }
        for &w in outs {
            let d = comp[w];
            if d != c && deps[c].insert(d) {
                dependents[d].insert(c);
            }
        }
    }
    let mut heap = BinaryHeap::new();
    for c in 0..ncomp {
        pending[c] = deps[c].len();
        if pending[c] == 0 {
            heap.push(Reverse((min_key[c].unwrap_or(""), c)));
        }
    }
    let mut order = Vec::with_capacity(ncomp);
    while let Some(Reverse((_, c))) = heap.pop() {
        order.push(c);
        for &d in &dependents[c] {
            pending[d] -= 1;
            if pending[d] == 0 {
                heap.push(Reverse((min_key[d].unwrap_or(""), d)));
            }
        }
    }
    order
}

/// 派生名的类名主干（`X__VTable` / `X__foo_base` → `X`；`__` 开头的名字不是类派生名）
fn stem(ident: &str) -> &str {
    match ident.find("__") {
        Some(p) if p > 0 => &ident[..p],
        _ => ident,
    }
}

/// 路径首个能对上类节点的前缀（`包::名` 或 `包::名的派生名`）
fn resolve(path: &[String], by_path: &HashMap<(&str, &str), usize>) -> Option<usize> {
    (1..path.len()).find_map(|i| {
        let pkg = path[..i].join("::");
        let id = path[i].as_str();
        by_path.get(&(pkg.as_str(), id)).or_else(|| by_path.get(&(pkg.as_str(), stem(id)))).copied()
    })
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// 文本中全部 `crate::` 路径（展开 `use` 花括号组、去 `r#` 与 `as` 别名、去 `*`）
pub fn crate_paths(text: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(p) = text[from..].find("crate::") {
        let at = from + p;
        from = at + "crate::".len();
        let prev = text[..at].chars().next_back();
        if prev.is_some_and(|c| is_ident_char(c) || c == '$') {
            continue;
        }
        let (paths, end) = parse_tree(text, from, Vec::new());
        out.extend(paths);
        from = end.max(from);
    }
    out
}

/// 从 `pos` 解析一棵 use 树（`a::b::C`、`a::{B, c::D}`），返回完整路径与结束位置
fn parse_tree(text: &str, mut pos: usize, mut prefix: Vec<String>) -> (Vec<Vec<String>>, usize) {
    let bytes = text.as_bytes();
    loop {
        if text[pos..].starts_with('{') {
            let close = matching_brace(text, pos);
            let inner = &text[pos + 1..close];
            let mut out = Vec::new();
            let (mut depth, mut start) = (0usize, 0usize);
            let mut items = Vec::new();
            for (i, c) in inner.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => depth = depth.saturating_sub(1),
                    ',' if depth == 0 => {
                        items.push((start, i));
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            items.push((start, inner.len()));
            for (s, e) in items {
                let item = &inner[s..e];
                let lead = item.len() - item.trim_start().len();
                if item.trim().is_empty() {
                    continue;
                }
                let (paths, _) = parse_tree(&text[..pos + 1 + e], pos + 1 + s + lead, prefix.clone());
                out.extend(paths);
            }
            return (out, close + 1);
        }
        let rest = &text[pos..];
        let raw = rest.strip_prefix("r#").unwrap_or(rest);
        let skip = rest.len() - raw.len();
        let len = raw.find(|c: char| !is_ident_char(c)).unwrap_or(raw.len());
        if len == 0 {
            return (if prefix.is_empty() { Vec::new() } else { vec![prefix] }, pos);
        }
        prefix.push(raw[..len].to_string());
        pos += skip + len;
        if bytes.get(pos..pos + 2) == Some(b"::") {
            pos += 2;
            continue;
        }
        return (vec![prefix], pos);
    }
}

fn matching_brace(text: &str, open: usize) -> usize {
    let mut depth = 0usize;
    for (i, c) in text[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return open + i;
                }
            }
            _ => {}
        }
    }
    text.len().saturating_sub(1).max(open)
}

/// 文本中含 `/` 的字符串字面量内容（binary name 形态的候选）
fn string_literals(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(p) = rest.find('"') {
        let body = &rest[p + 1..];
        let mut end = None;
        let mut esc = false;
        for (i, c) in body.char_indices() {
            match c {
                '\\' if !esc => esc = true,
                '"' if !esc => {
                    end = Some(i);
                    break;
                }
                _ => esc = false,
            }
        }
        let Some(e) = end else { break };
        if body[..e].contains('/') {
            out.push(&body[..e]);
        }
        rest = &body[e + 1..];
    }
    out
}

/// 手写文本中的标识符（去行注释、块注释与字符串字面量）
pub fn idents(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let chars: Vec<char> = text.chars().collect();
    let (mut i, mut cur) = (0usize, String::new());
    let flush = |cur: &mut String, out: &mut BTreeSet<String>| {
        if cur.chars().next().is_some_and(|c| !c.is_ascii_digit()) {
            out.insert(std::mem::take(cur));
        }
        cur.clear();
    };
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            flush(&mut cur, &mut out);
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == '/' && next == Some('*') {
            flush(&mut cur, &mut out);
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if c == '"' {
            flush(&mut cur, &mut out);
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                i += if chars[i] == '\\' { 2 } else { 1 };
            }
            i += 1;
            continue;
        }
        if is_ident_char(c) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out);
        }
        i += 1;
    }
    flush(&mut cur, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node<'a>(key: &'a str, pkg: &str, ident: &str, text: &'a str) -> DeclNode<'a> {
        DeclNode { key, pkg: pkg.into(), ident: ident.into(), text }
    }

    /// 各类权重均为 1（按类数切）
    fn plan1(g: &DeclGraph, cap: usize) -> Vec<Vec<usize>> {
        plan(g, &vec![1; g.keys.len()], 0, cap)
    }

    fn keys(g: &DeclGraph, segs: &[Vec<usize>]) -> Vec<Vec<String>> {
        segs.iter().map(|s| s.iter().map(|&v| g.key(v).to_string()).collect()).collect()
    }

    #[test]
    fn crate_paths_expand_groups_aliases_and_raw_idents() {
        let t = "use crate::java::lang::{Object, ObjectVTable as OV, r#ref::Cleaner};\n\
                 let x = crate::java::util::List__VTable::f(); $crate::no::Way; foocrate::no::Way;\n\
                 use crate::prelude::*;";
        let got: Vec<String> = crate_paths(t).into_iter().map(|p| p.join("::")).collect();
        assert_eq!(
            got,
            vec![
                "java::lang::Object",
                "java::lang::ObjectVTable",
                "java::lang::ref::Cleaner",
                "java::util::List__VTable::f",
                "prelude",
            ]
        );
    }

    #[test]
    fn idents_skip_comments_and_strings() {
        let t = "// Foo\nlet a: Bar = \"Baz \\\" Qux\"; /* Zed */ x1";
        let got: Vec<String> = idents(t).into_iter().collect();
        assert_eq!(got, vec!["Bar", "a", "let", "x1"]);
    }

    #[test]
    fn attribute_binary_names_are_edges() {
        let nodes = vec![
            node("a/A", "a", "A", ""),
            node("a/B", "a", "B", "#[super_class = \"a/A\"] #[all_supertypes = \"a/C;x/Y\"]"),
            node("a/C", "a", "C", ""),
        ];
        let g = DeclGraph::build(&nodes, &[], &[]);
        assert_eq!(g.edges[1], vec![0, 2, 3]);
        assert_eq!(string_literals("f(\"a\\\"/b\", \"c\") \"d/e\""), vec!["a\\\"/b", "d/e"]);
    }

    #[test]
    fn small_set_is_not_split() {
        let nodes = vec![node("a/A", "a", "A", ""), node("a/B", "a", "B", "")];
        let g = DeclGraph::build(&nodes, &[], &[]);
        assert_eq!(keys(&g, &plan1(&g, 10)), vec![vec!["a/A", "a/B"]]);
    }

    #[test]
    fn cycles_share_a_segment_and_bottom_holds_infra_closure() {
        // Obj 被基础设施引用（钉底），Str → Obj；P ↔ Q 成环，R → P；S、T 独立
        let nodes = vec![
            node("l/Obj", "l", "Obj", ""),
            node("l/Str", "l", "Str", "use crate::l::Obj;"),
            node("u/P", "u", "P", "use crate::u::Q;"),
            node("u/Q", "u", "Q", "fn f() -> crate::u::P__VTable {}"),
            node("u/R", "u", "R", "use crate::{u::P, l::Str};"),
            node("u/S", "u", "S", ""),
            node("u/T", "u", "T", ""),
        ];
        let g = DeclGraph::build(&nodes, &["fn infra(o: Obj) -> Str { todo() }"], &[]);
        let segs = keys(&g, &plan1(&g, 3));
        assert_eq!(segs[0], vec!["l/Obj", "l/Str"]);
        // 余 5 类、上限 3 → 2 段；P、Q 同段，R 不早于 P
        assert_eq!(segs.len(), 3);
        let seg_of = |k: &str| segs.iter().position(|s| s.iter().any(|x| x == k)).unwrap();
        assert_eq!(seg_of("u/P"), seg_of("u/Q"));
        assert!(seg_of("u/R") >= seg_of("u/P"));
    }

    #[test]
    fn pinned_hosts_pull_their_closure_to_bottom() {
        let nodes = vec![
            node("a/A", "a", "A", "use crate::a::B;"),
            node("a/B", "a", "B", ""),
            node("a/C", "a", "C", ""),
            node("a/D", "a", "D", ""),
        ];
        let g = DeclGraph::build(&nodes, &[], &[0]);
        let segs = keys(&g, &plan1(&g, 2));
        assert_eq!(segs[0], vec!["a/A", "a/B"]);
        assert_eq!(segs[1..].concat(), vec!["a/C", "a/D"]);
    }

    #[test]
    fn segments_respect_dependency_order_and_are_deterministic() {
        // 链 E → D → C → B → A，无钉底：底段只有 INFRA（空），其余按被依赖者在前切段
        let nodes = vec![
            node("A", "p", "A", ""),
            node("B", "p", "B", "crate::p::A"),
            node("C", "p", "C", "crate::p::B"),
            node("D", "p", "D", "crate::p::C"),
            node("E", "p", "E", "crate::p::D"),
        ];
        let g = DeclGraph::build(&nodes, &[], &[]);
        let segs = keys(&g, &plan1(&g, 2));
        assert_eq!(segs, vec![vec![], vec!["A", "B"], vec!["C", "D"], vec!["E"]]);
        assert_eq!(segs, keys(&g, &plan1(&g, 2)));
    }

    #[test]
    fn everything_in_bottom_means_no_split() {
        let nodes = vec![node("a/A", "a", "A", ""), node("a/B", "a", "B", ""), node("a/C", "a", "C", "")];
        let g = DeclGraph::build(&nodes, &["A B C"], &[]);
        assert_eq!(plan1(&g, 1).len(), 1);
    }

    #[test]
    fn cut_balances_and_never_skips() {
        assert_eq!(cut(&[3, 3, 3, 3, 3, 3, 3, 4], 10), vec![0, 0, 0, 1, 1, 1, 2, 2]);
        assert_eq!(cut(&[1, 30, 1], 10), vec![0, 1, 2]);
        assert_eq!(cut(&[4], 10), vec![0]);
        // 中点均衡会把 6 + 6 落进同一段（12 > 10）；硬上限下须三段
        assert_eq!(cut(&[6, 6, 6], 10), vec![0, 1, 2]);
        assert_eq!(cut(&[6, 4, 6, 4], 10), vec![0, 0, 1, 1]);
        // 最少段数下取均衡：贪心按 10 切为 10|2，均衡后为 6|6
        assert_eq!(cut(&[2, 2, 2, 2, 2, 2], 10), vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn cut_never_exceeds_cap_unless_single_oversized() {
        let sizes: Vec<usize> = (0..200).map(|i| (i * 7919 % 97) + 1).collect();
        for cap in [97, 100, 150, 333, 1000] {
            let bins = cut(&sizes, cap);
            let k = bins.last().unwrap() + 1;
            let mut load = vec![0usize; k];
            for (s, b) in sizes.iter().zip(&bins) {
                load[*b] += s;
            }
            assert!(load.iter().all(|&l| l <= cap), "cap {cap}: {load:?}");
            assert!(bins.windows(2).all(|w| w[1] == w[0] || w[1] == w[0] + 1));
            assert_eq!(k, greedy(&sizes, cap).last().unwrap() + 1, "段数最少");
        }
    }

    #[test]
    fn split_by_bytes_not_class_count() {
        // 3 类、无钉底：体量 4 + 4 + 4 > 10 须切；每段体量 ≤ 10
        let nodes = vec![node("a/A", "a", "A", ""), node("a/B", "a", "B", ""), node("a/C", "a", "C", "")];
        let g = DeclGraph::build(&nodes, &[], &[]);
        let segs = plan(&g, &[4, 4, 4], 0, 10);
        assert_eq!(keys(&g, &segs), vec![vec![], vec!["a/A", "a/B"], vec!["a/C"]]);
        // 总体量 ≤ 上限不切；手写基础量计入判定
        assert_eq!(plan(&g, &[3, 3, 3], 0, 10).len(), 1);
        assert_eq!(keys(&g, &plan(&g, &[3, 3, 3], 2, 10)), vec![vec![], vec!["a/A", "a/B", "a/C"]]);
    }
}
