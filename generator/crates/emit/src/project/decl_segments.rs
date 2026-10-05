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
//!   含 INFRA 的 SCC 是唯一汇点，为底段；其余分量按拓扑序连续切成 k = ⌈余量 / 上限⌉ 个均衡段，
//!   第 j 段只依赖 < j 的段，各段经镜像链（段 j 只依赖段 j−1）对外呈现完整视图。
//! - 总类数不超过上限、或底段之外无类时不分段（与现状布局一致）。
//!
//! 分段只依赖类集合与引用关系（每段类数上限为常量，不随机器内存变化）。

use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::cmp::Reverse;

/// 每个声明段的类数上限
pub const DECL_SEGMENT_CLASSES: usize = 650;

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

/// 分段结果：`segments[0]` 为底段（含全部钉底类），段内按键名排序；不分段时只有一段
pub fn plan(g: &DeclGraph, cap: usize) -> Vec<Vec<usize>> {
    let n = g.keys.len();
    let all = || {
        let mut v: Vec<usize> = (0..n).collect();
        v.sort_by(|&a, &b| g.key(a).cmp(g.key(b)));
        v
    };
    if n <= cap.max(1) {
        return vec![all()];
    }
    let (comp, ncomp) = tarjan(&g.edges);
    let order = topo_deps_first(g, &comp, ncomp);
    let bottom = comp[g.infra()];
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); ncomp];
    for v in 0..n {
        members[comp[v]].push(v);
    }
    let rest: Vec<usize> = order.into_iter().filter(|&c| c != bottom).collect();
    let rest_total: usize = rest.iter().map(|&c| members[c].len()).sum();
    if rest_total == 0 {
        return vec![all()];
    }
    let sizes: Vec<usize> = rest.iter().map(|&c| members[c].len()).collect();
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

/// 连续均衡切段：k = ⌈总量 / 上限⌉，每项按其中点落入的 1/k 区间定段（单调不减、无空段）
fn cut(sizes: &[usize], cap: usize) -> Vec<usize> {
    let total: usize = sizes.iter().sum();
    let k = total.div_ceil(cap.max(1)).clamp(1, sizes.len().max(1));
    let mut prefix = 0usize;
    let mut last = 0usize;
    let mut out = Vec::with_capacity(sizes.len());
    for &s in sizes {
        let mid = prefix + s / 2;
        prefix += s;
        let bin = ((mid as u128 * k as u128 / total.max(1) as u128) as usize).min(k - 1);
        let bin = if out.is_empty() { 0 } else { bin.clamp(last, last + 1) };
        out.push(bin);
        last = bin;
    }
    out
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
    fn small_set_is_not_split() {
        let nodes = vec![node("a/A", "a", "A", ""), node("a/B", "a", "B", "")];
        let g = DeclGraph::build(&nodes, &[], &[]);
        assert_eq!(keys(&g, &plan(&g, 10)), vec![vec!["a/A", "a/B"]]);
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
        let segs = keys(&g, &plan(&g, 3));
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
        let segs = keys(&g, &plan(&g, 2));
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
        let segs = keys(&g, &plan(&g, 2));
        assert_eq!(segs, vec![vec![], vec!["A", "B"], vec!["C", "D"], vec!["E"]]);
        assert_eq!(segs, keys(&g, &plan(&g, 2)));
    }

    #[test]
    fn everything_in_bottom_means_no_split() {
        let nodes = vec![node("a/A", "a", "A", ""), node("a/B", "a", "B", ""), node("a/C", "a", "C", "")];
        let g = DeclGraph::build(&nodes, &["A B C"], &[]);
        assert_eq!(plan(&g, 1).len(), 1);
    }

    #[test]
    fn cut_balances_and_never_skips() {
        assert_eq!(cut(&[3, 3, 3, 3, 3, 3, 3, 4], 10), vec![0, 0, 0, 1, 1, 1, 2, 2]);
        assert_eq!(cut(&[1, 30, 1], 10), vec![0, 1, 2]);
        assert_eq!(cut(&[4], 10), vec![0]);
    }
}
