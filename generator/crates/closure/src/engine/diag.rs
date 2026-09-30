//! 引擎：open 来源诊断（`--flows @openorig:<类型>|<节点子串>` / `@openinj:<类型>`）。
//!
//! open 值只有两种来历：直接注入（手写返回、未知值按声明类型、open 数组的元素读……）与沿流边传播。
//! 注入点在 `add_to` 处登记（[`Engine::open_inj`]），诊断沿流边反向走到注入点，回答「这个节点上的
//! open(T) 从哪些注入点来」。

use super::*;

impl Engine<'_> {
    pub(super) fn diag_open(&self, pat: &str) -> Option<Vec<String>> {
        if let Some(q) = pat.strip_prefix("@openinj:") {
            let Some(&cid) = self.ids.get(q) else { return Some(vec![format!("无此类：{q}")]) };
            let mut v: Vec<String> =
                self.open_inj.iter().filter(|(_, ts)| ts.contains(&cid)).map(|(n, _)| format!("  {}", self.node_str(*n))).collect();
            v.sort();
            return Some(v);
        }
        let (q, np) = pat.strip_prefix("@openorig:")?.split_once('|')?;
        let Some(&cid) = self.ids.get(q) else { return Some(vec![format!("无此类：{q}")]) };
        // 流边按声明类型收窄：open(Object) 经 Object[] 过滤成为 open(Object[])，沿途的 open 类型与目标有子类型关系即算同源
        let q_name = self.names[cid as usize].clone();
        let related = |o: u32| {
            let n = &self.names[o as usize];
            o == cid || self.h.is_subtype(n, &q_name) || self.h.is_subtype(&q_name, n)
        };
        let has = |x: &Node| self.sets.get(x).is_some_and(|s| s.open.iter().any(|&o| related(o)));
        let mut rev: HashMap<Node, Vec<Node>> = HashMap::default();
        for (src, edges) in &self.flows {
            if has(src) {
                for (dst, _) in edges {
                    rev.entry(*dst).or_default().push(*src);
                }
            }
        }
        let mut seen: HashSet<Node> = self.sets.keys().filter(|n| has(n) && self.node_str(**n).contains(np)).copied().collect();
        let mut q: VecDeque<Node> = seen.iter().copied().collect();
        // 注入点 → 到起点的最短距离
        let mut hits: Vec<(Node, usize)> = Vec::new();
        let mut dist: HashMap<Node, usize> = seen.iter().map(|n| (*n, 0)).collect();
        while let Some(n) = q.pop_front() {
            let d = dist[&n];
            if self.open_inj.get(&n).is_some_and(|ts| ts.iter().any(|&o| related(o))) {
                hits.push((n, d));
            }
            for p in rev.get(&n).into_iter().flatten() {
                if seen.insert(*p) {
                    dist.insert(*p, d + 1);
                    q.push_back(*p);
                }
            }
        }
        let mut out: Vec<String> = hits.iter().map(|(n, d)| format!("  [{d:3}] {}", self.node_str(*n))).collect();
        out.sort();
        out.insert(0, format!("  经 {} 个节点，注入点 {} 个", seen.len(), hits.len()));
        Some(out)
    }
}
