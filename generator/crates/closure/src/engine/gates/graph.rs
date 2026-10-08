//! 触发图模型：基线运行登记的全部触发边（`cut::Edges`，方法源带偏移）上的「与」可达性。
//!
//! 节点：`R:` 根、`M:方法` 方法节点、`M:方法@偏移` 调用点、`C:` 类、`I:` 类初始化、`A:` 实例化、`H:` 枢纽。
//! 边 `(源, 目标, 条件)`：源可达且未被切除、条件（若有）可达时目标可达；派发边以接收者实例化 `A:` 为条件。
//! 隐含边 `M:方法 → M:方法@偏移`：方法体切除挡住其全部调用点，调用点切除只挡该点。
//! 被切除的节点自身仍可达（与引擎「节点保留、体内事件不执行」同口径），只是不再触发出边。
//! 无入边的节点（根之外经未登记路径到达的）一律视为根——保守：只会少算可减量。
//!
//! 模型是引擎的上近似：值流（字段 / 返回值集合变空后的折叠）不在图上，Δ 由实测重跑校准（见 `gates.rs`）。

use std::collections::HashMap;

use super::super::cut::{Edges, NO_COND};

pub const NONE: u32 = u32::MAX;

/// 节点类别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Class,
    /// 方法节点（计入方法数的：在引擎方法表里、非伪方法）
    Method,
    /// 方法节点但不计数（伪方法 / 不在方法表）
    OtherMethod,
    Site,
    Other,
}

pub struct TriggerGraph {
    pub names: Vec<String>,
    pub kinds: Vec<NodeKind>,
    /// 出边（CSR）：`(目标, 条件)`
    out_start: Vec<u32>,
    out: Vec<(u32, u32)>,
    /// 以节点为条件的边（CSR）：`(源, 目标)`
    cond_start: Vec<u32>,
    cond: Vec<(u32, u32)>,
    roots: Vec<u32>,
}

/// 一次可达性求解：`reached` 按节点；`parent` 为首次触发它的源（根为 [`NONE`]）
pub struct Reach {
    pub reached: Vec<bool>,
    pub parent: Vec<u32>,
}

fn csr(n: usize, items: impl Iterator<Item = (u32, (u32, u32))> + Clone) -> (Vec<u32>, Vec<(u32, u32)>) {
    let mut start = vec![0u32; n + 1];
    for (k, _) in items.clone() {
        start[k as usize + 1] += 1;
    }
    for i in 0..n {
        start[i + 1] += start[i];
    }
    let mut fill = start.clone();
    let mut out = vec![(0u32, 0u32); start[n] as usize];
    for (k, v) in items {
        out[fill[k as usize] as usize] = v;
        fill[k as usize] += 1;
    }
    (start, out)
}

/// `M:方法@偏移` → (`M:方法`, 偏移)
pub fn split_site(name: &str) -> Option<(&str, u32)> {
    let (base, off) = name.rsplit_once('@')?;
    Some((base, off.parse().ok()?))
}

impl TriggerGraph {
    /// `counted(key)`：方法键（不带 `M:`）是否计入方法数
    pub fn build(e: Edges, counted: impl Fn(&str) -> bool) -> TriggerGraph {
        let Edges { mut names, mut edges } = e;
        let mut index: HashMap<String, u32> = names.iter().enumerate().map(|(i, n)| (n.clone(), i as u32)).collect();
        // 隐含边：方法 → 其调用点
        for i in 0..names.len() {
            let Some(base) = names[i].strip_prefix("M:").and_then(split_site).map(|(b, _)| format!("M:{b}")) else { continue };
            let b = match index.get(&base) {
                Some(&b) => b,
                None => {
                    let b = names.len() as u32;
                    names.push(base.clone());
                    index.insert(base, b);
                    b
                }
            };
            edges.push((b, i as u32, NO_COND));
        }
        let n = names.len();
        let kinds = names
            .iter()
            .map(|s| match s.split_at(s.find(':').map_or(0, |p| p + 1)) {
                ("C:", _) => NodeKind::Class,
                ("M:", k) if split_site(k).is_some() => NodeKind::Site,
                ("M:", k) if counted(k) => NodeKind::Method,
                ("M:", _) => NodeKind::OtherMethod,
                _ => NodeKind::Other,
            })
            .collect();
        let mut indeg = vec![false; n];
        for &(_, b, _) in &edges {
            indeg[b as usize] = true;
        }
        let roots = (0..n as u32).filter(|&i| names[i as usize].starts_with("R:") || !indeg[i as usize]).collect();
        let (out_start, out) = csr(n, edges.iter().map(|&(a, b, c)| (a, (b, c))));
        let (cond_start, cond) = csr(n, edges.iter().filter(|e| e.2 != NO_COND).map(|&(a, b, c)| (c, (a, b))));
        TriggerGraph { names, kinds, out_start, out, cond_start, cond, roots }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// 按名取节点（线性查找，测试用）
    #[cfg(test)]
    pub fn node(&self, name: &str) -> Option<u32> {
        self.names.iter().position(|n| n == name).map(|i| i as u32)
    }

    /// 切除 `blocked` 中节点后的可达集
    pub fn reach(&self, blocked: &[bool]) -> Reach {
        let n = self.len();
        let mut reached = vec![false; n];
        let mut parent = vec![NONE; n];
        let mut queue: Vec<u32> = Vec::with_capacity(n);
        for &r in &self.roots {
            reached[r as usize] = true;
            queue.push(r);
        }
        let mut qi = 0;
        while qi < queue.len() {
            let x = queue[qi];
            qi += 1;
            let xi = x as usize;
            if !blocked[xi] {
                for &(y, c) in &self.out[self.out_start[xi] as usize..self.out_start[xi + 1] as usize] {
                    if !reached[y as usize] && (c == NO_COND || reached[c as usize]) {
                        reached[y as usize] = true;
                        parent[y as usize] = x;
                        queue.push(y);
                    }
                }
            }
            // 以 x 为条件、源已先到达的边
            for &(a, y) in &self.cond[self.cond_start[xi] as usize..self.cond_start[xi + 1] as usize] {
                if !reached[y as usize] && reached[a as usize] && !blocked[a as usize] {
                    reached[y as usize] = true;
                    parent[y as usize] = a;
                    queue.push(y);
                }
            }
        }
        Reach { reached, parent }
    }

    /// (类数, 方法数)
    pub fn count(&self, r: &Reach) -> (usize, usize) {
        let mut c = (0, 0);
        for (k, &on) in self.kinds.iter().zip(&r.reached) {
            match (k, on) {
                (NodeKind::Class, true) => c.0 += 1,
                (NodeKind::Method, true) => c.1 += 1,
                _ => {}
            }
        }
        c
    }

    /// 在 `base` 可达、在 `r` 不可达的节点（按类别过滤）
    pub fn removed(&self, base: &Reach, r: &Reach, kind: NodeKind) -> Vec<u32> {
        (0..self.len()).filter(|&i| self.kinds[i] == kind && base.reached[i] && !r.reached[i]).map(|i| i as u32).collect()
    }

    /// 首达树上各节点子树中的类数（只对 `eligible` 节点计数）
    pub fn subtree_classes(&self, r: &Reach, eligible: &[bool]) -> Vec<u32> {
        let mut cnt = vec![0u32; self.len()];
        for i in 0..self.len() {
            if self.kinds[i] != NodeKind::Class || !r.reached[i] {
                continue;
            }
            let mut p = r.parent[i];
            let mut guard = 0;
            while p != NONE && guard < 10_000 {
                if eligible[p as usize] {
                    cnt[p as usize] += 1;
                }
                p = r.parent[p as usize];
                guard += 1;
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(edges: &[(&str, &str, &str)]) -> TriggerGraph {
        let mut ids: HashMap<String, u32> = HashMap::new();
        let mut names = Vec::new();
        let mut id = |s: &str| {
            *ids.entry(s.to_string()).or_insert_with(|| {
                names.push(s.to_string());
                names.len() as u32 - 1
            })
        };
        let es: Vec<_> = edges.iter().map(|(a, b, c)| (id(a), id(b), if c.is_empty() { NO_COND } else { id(c) })).collect();
        TriggerGraph::build(Edges { names, edges: es }, |_| true)
    }

    fn cut(g: &TriggerGraph, nodes: &[&str]) -> (usize, usize) {
        let mut b = vec![false; g.len()];
        for n in nodes {
            b[g.node(n).unwrap() as usize] = true;
        }
        g.count(&g.reach(&b))
    }

    #[test]
    fn site_and_body_cuts_with_dispatch_conditions() {
        // main@3 派发到 X.run（条件 A:X）；main@7 new X；X.run 触发类 Y
        let g = graph(&[
            ("R:main", "M:U.main:()V", ""),
            ("M:U.main:()V@7", "A:X", ""),
            ("M:U.main:()V@7", "C:X", ""),
            ("M:U.main:()V@3", "M:X.run:()V", "A:X"),
            ("M:X.run:()V@0", "C:Y", ""),
        ]);
        let none = vec![false; g.len()];
        assert_eq!(g.count(&g.reach(&none)), (2, 2));
        // 切调用点：X.run 与 Y 消失
        assert_eq!(cut(&g, &["M:U.main:()V@3"]), (1, 1));
        // 切 X.run 体：方法保留、Y 消失
        assert_eq!(cut(&g, &["M:X.run:()V"]), (1, 2));
        // 切分配点：派发条件不成立
        assert_eq!(cut(&g, &["M:U.main:()V@7"]), (0, 1));
    }

    #[test]
    fn condition_reached_after_source() {
        // 源先到、条件后到：经条件侧的边补触发
        let g = graph(&[("R:r", "M:a:()V", ""), ("M:a:()V@1", "M:b:()V", "A:Z"), ("M:a:()V@9", "M:c:()V", ""), ("M:c:()V@0", "A:Z", "")]);
        let none = vec![false; g.len()];
        let r = g.reach(&none);
        assert!(r.reached[g.node("M:b:()V").unwrap() as usize]);
        assert_eq!(cut(&g, &["M:c:()V"]).1, 2);
    }

    #[test]
    fn multi_connected_needs_both_entries() {
        let g = graph(&[
            ("R:r", "M:a:()V", ""),
            ("M:a:()V@1", "M:hub:()V", ""),
            ("M:a:()V@2", "M:hub:()V", ""),
            ("M:hub:()V@0", "C:Big", ""),
        ]);
        assert_eq!(cut(&g, &["M:a:()V@1"]).0, 1);
        assert_eq!(cut(&g, &["M:a:()V@1", "M:a:()V@2"]).0, 0);
        let none = vec![false; g.len()];
        let r = g.reach(&none);
        let elig: Vec<bool> = g.kinds.iter().map(|k| *k != NodeKind::Class).collect();
        let t = g.subtree_classes(&r, &elig);
        assert_eq!(t[g.node("M:hub:()V").unwrap() as usize], 1);
    }
}
