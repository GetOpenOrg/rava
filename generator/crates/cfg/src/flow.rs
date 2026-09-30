//! 抽象图算法（← `graph.py` 后半）：可达性 / 逆后序 / 支配树 / 自然循环 / 可归约性。
//!
//! 输入是 `(entry, succs)` 形式的抽象图，供基本块图与归约后的节点图复用。
//! 迭代序全部确定：后继表按调用方给出的顺序，集合与映射用 `BTreeSet` / `BTreeMap`。

use std::collections::{BTreeMap, BTreeSet};

use crate::graph::NodeId;

/// 后继表：节点 → 有序后继列表。
pub type Succs = BTreeMap<NodeId, Vec<NodeId>>;

fn succ_of(succs: &Succs, n: NodeId) -> &[NodeId] {
    succs.get(&n).map(Vec::as_slice).unwrap_or(&[])
}

pub fn reachable(entry: NodeId, succs: &Succs) -> BTreeSet<NodeId> {
    let mut seen = BTreeSet::from([entry]);
    let mut work = vec![entry];
    while let Some(n) = work.pop() {
        for &s in succ_of(succs, n) {
            if seen.insert(s) {
                work.push(s);
            }
        }
    }
    seen
}

/// 逆后序。后继按编号降序访问，使 RPO 尽量贴近字节码（= 源码）顺序。
pub fn reverse_postorder(entry: NodeId, succs: &Succs) -> Vec<NodeId> {
    let sorted_desc = |n: NodeId| -> Vec<NodeId> {
        let mut v = succ_of(succs, n).to_vec();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v
    };
    let mut order: Vec<NodeId> = Vec::new();
    let mut seen = BTreeSet::from([entry]);
    // pending 以「下一个待访问下标」推进，与 Python 的 pop(0) 等价
    let mut stack: Vec<(NodeId, Vec<NodeId>, usize)> = vec![(entry, sorted_desc(entry), 0)];
    while let Some(top) = stack.last_mut() {
        let node = top.0;
        let mut next = None;
        while top.2 < top.1.len() {
            let s = top.1[top.2];
            top.2 += 1;
            if seen.insert(s) {
                next = Some(s);
                break;
            }
        }
        match next {
            Some(s) => stack.push((s, sorted_desc(s), 0)),
            None => {
                order.push(node);
                stack.pop();
            }
        }
    }
    order.reverse();
    order
}

/// 前驱表（只含 `nodes` 内的节点；前驱按 `nodes` 序、去重）。
pub fn predecessors(nodes: &[NodeId], succs: &Succs) -> BTreeMap<NodeId, Vec<NodeId>> {
    let mut preds: BTreeMap<NodeId, Vec<NodeId>> = nodes.iter().map(|&n| (n, Vec::new())).collect();
    for &n in nodes {
        for &s in succ_of(succs, n) {
            if let Some(p) = preds.get_mut(&s) {
                if !p.contains(&n) {
                    p.push(n);
                }
            }
        }
    }
    preds
}

/// Cooper-Harvey-Kennedy 迭代支配算法。返回 idom（entry 的 idom 为自身）。
pub fn immediate_dominators(entry: NodeId, succs: &Succs, rpo: &[NodeId]) -> BTreeMap<NodeId, NodeId> {
    let index: BTreeMap<NodeId, usize> = rpo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let preds = predecessors(rpo, succs);
    let mut idom: BTreeMap<NodeId, NodeId> = BTreeMap::from([(entry, entry)]);
    let intersect = |idom: &BTreeMap<NodeId, NodeId>, mut a: NodeId, mut b: NodeId| -> NodeId {
        while a != b {
            while index[&a] > index[&b] {
                a = idom[&a];
            }
            while index[&b] > index[&a] {
                b = idom[&b];
            }
        }
        a
    };
    let mut changed = true;
    while changed {
        changed = false;
        for &n in rpo {
            if n == entry {
                continue;
            }
            let mut new: Option<NodeId> = None;
            for &p in &preds[&n] {
                if idom.contains_key(&p) {
                    new = Some(match new {
                        None => p,
                        Some(cur) => intersect(&idom, p, cur),
                    });
                }
            }
            if let Some(v) = new {
                if idom.get(&n) != Some(&v) {
                    idom.insert(n, v);
                    changed = true;
                }
            }
        }
    }
    idom
}

/// a 是否支配 b（自反）。
pub fn dominates(idom: &BTreeMap<NodeId, NodeId>, a: NodeId, mut b: NodeId) -> bool {
    loop {
        if a == b {
            return true;
        }
        match idom.get(&b) {
            Some(&p) if p != b => b = p,
            _ => return false,
        }
    }
}

/// 图分析结果。
#[derive(Debug, Clone)]
pub struct FlowAnalysis {
    pub entry: NodeId,
    /// 限定在可达节点上的后继表
    pub succs: Succs,
    pub rpo: Vec<NodeId>,
    pub rpo_index: BTreeMap<NodeId, usize>,
    pub preds: BTreeMap<NodeId, Vec<NodeId>>,
    pub idom: BTreeMap<NodeId, NodeId>,
    /// {(src, header)}
    pub back_edges: BTreeSet<(NodeId, NodeId)>,
    /// header → 循环体节点集（含 header）
    pub loops: BTreeMap<NodeId, BTreeSet<NodeId>>,
    pub reducible: bool,
}

impl FlowAnalysis {
    pub fn is_back_edge(&self, src: NodeId, dst: NodeId) -> bool {
        self.back_edges.contains(&(src, dst))
    }

    pub fn forward_in_degree(&self, n: NodeId) -> usize {
        self.preds.get(&n).map_or(0, |ps| ps.iter().filter(|&&p| !self.is_back_edge(p, n)).count())
    }

    pub fn dominates(&self, a: NodeId, b: NodeId) -> bool {
        dominates(&self.idom, a, b)
    }

    pub fn succs_of(&self, n: NodeId) -> &[NodeId] {
        succ_of(&self.succs, n)
    }
}

pub fn analyze(entry: NodeId, succs: &Succs) -> FlowAnalysis {
    let live = reachable(entry, succs);
    let succs: Succs = live.iter().map(|&n| (n, succ_of(succs, n).to_vec())).collect();
    let rpo = reverse_postorder(entry, &succs);
    let rpo_index: BTreeMap<NodeId, usize> = rpo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let preds = predecessors(&rpo, &succs);
    let idom = immediate_dominators(entry, &succs, &rpo);

    let mut back_edges = BTreeSet::new();
    let mut reducible = true;
    for &u in &rpo {
        for &v in &succs[&u] {
            if rpo_index[&v] <= rpo_index[&u] {
                if dominates(&idom, v, u) {
                    back_edges.insert((u, v));
                } else {
                    reducible = false;
                }
            }
        }
    }

    let mut loops: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for &(u, h) in &back_edges {
        let body = loops.entry(h).or_insert_with(|| BTreeSet::from([h]));
        let mut work = vec![u];
        while let Some(n) = work.pop() {
            if !body.insert(n) {
                continue;
            }
            work.extend(preds[&n].iter().copied());
        }
    }
    FlowAnalysis { entry, succs, rpo, rpo_index, preds, idom, back_edges, loops, reducible }
}
