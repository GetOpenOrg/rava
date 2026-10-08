//! 触发图上的反事实量测：单切 Δ、贪心组合（逐步取边际收益最大者）与同机制组合切除。
//!
//! 多连通团块（经多个入口到达）上任何单切 Δ 都是 0，纯贪心会停在原地。此时沿模型首达树依次切下
//! 剩余最大子树的入口（至多 `streak` 步）：其间累计收益达到门槛即整段采纳（曲线上表现为「零收益若干步后跳变」），
//! 否则整段回退、把首个入口排除后重试。

use std::collections::HashSet;

use super::graph::{NodeKind, Reach, TriggerGraph};

/// 一次切除相对基线的减少量
#[derive(Debug, Clone, Default)]
pub struct Delta {
    pub classes: usize,
    pub methods: usize,
    /// 消失的类节点（升序）
    pub removed: Vec<u32>,
}

pub struct Model<'g> {
    pub g: &'g TriggerGraph,
    pub base: Reach,
    pub base_count: (usize, usize),
}

impl<'g> Model<'g> {
    pub fn new(g: &'g TriggerGraph) -> Model<'g> {
        let base = g.reach(&vec![false; g.len()]);
        let base_count = g.count(&base);
        Model { g, base, base_count }
    }

    pub fn eval(&self, cut: &[u32]) -> (Reach, (usize, usize)) {
        let mut b = vec![false; self.g.len()];
        for &n in cut {
            b[n as usize] = true;
        }
        let r = self.g.reach(&b);
        let c = self.g.count(&r);
        (r, c)
    }

    pub fn delta(&self, cut: &[u32]) -> Delta {
        let (r, c) = self.eval(cut);
        Delta {
            classes: self.base_count.0.saturating_sub(c.0),
            methods: self.base_count.1.saturating_sub(c.1),
            removed: self.g.removed(&self.base, &r, NodeKind::Class),
        }
    }
}

/// 贪心曲线上的一步
#[derive(Debug, Clone)]
pub struct Step {
    pub node: u32,
    /// 本步边际减少的类数（多连通入口段内的零收益步为 0，段末一步计整段收益）
    pub gain: usize,
    /// 累计减少（类 / 方法）
    pub classes: usize,
    pub methods: usize,
    /// 多连通入口段内的步（单独切除无收益，与同段其余入口一起才有）
    pub entry: bool,
}

pub struct GreedyParams {
    pub steps: usize,
    /// 单步至少减少的类数
    pub min_gain: usize,
    /// 多连通入口段最长步数
    pub streak: usize,
    /// 每步在候选池之外追加评估的模型首达树大子树节点数
    pub extra: usize,
}

fn top_by_subtree(sub: &[u32], ok: impl Fn(u32) -> bool, k: usize) -> Vec<u32> {
    let mut v: Vec<u32> = (0..sub.len() as u32).filter(|&i| sub[i as usize] > 0 && ok(i)).collect();
    v.sort_by_key(|&i| (std::cmp::Reverse(sub[i as usize]), i));
    v.truncate(k);
    v
}

pub fn greedy(m: &Model, pool: &[u32], eligible: &[bool], p: &GreedyParams) -> Vec<Step> {
    let mut chosen: Vec<u32> = Vec::new();
    let mut out: Vec<Step> = Vec::new();
    let mut excluded: HashSet<u32> = HashSet::new();
    let mut cur = m.base_count;
    'outer: while out.len() < p.steps {
        let (reach, _) = m.eval(&chosen);
        let sub = m.g.subtree_classes(&reach, eligible);
        let free = |i: u32| reach.reached[i as usize] && !chosen.contains(&i) && !excluded.contains(&i);
        let mut cands: Vec<u32> = pool.iter().copied().filter(|&i| free(i)).collect();
        for i in top_by_subtree(&sub, free, p.extra) {
            if !cands.contains(&i) {
                cands.push(i);
            }
        }
        let mut best: Option<(usize, u32, u32, (usize, usize))> = None;
        for &c in &cands {
            let mut cut = chosen.clone();
            cut.push(c);
            let (_, cnt) = m.eval(&cut);
            let gain = cur.0.saturating_sub(cnt.0);
            let key = (gain, sub[c as usize], u32::MAX - c);
            if best.is_none_or(|b| key > (b.0, b.1, u32::MAX - b.2)) {
                best = Some((gain, sub[c as usize], c, cnt));
            }
        }
        if let Some((gain, _, c, cnt)) = best.filter(|b| b.0 >= p.min_gain) {
            chosen.push(c);
            cur = cnt;
            out.push(Step { node: c, gain, classes: m.base_count.0 - cnt.0, methods: m.base_count.1.saturating_sub(cnt.1), entry: false });
            continue;
        }
        // 多连通：沿模型首达树依次切剩余最大子树的入口
        let mut tent = chosen.clone();
        let mut seg: Vec<(u32, (usize, usize))> = Vec::new();
        for _ in 0..p.streak.min(p.steps - out.len()) {
            let (r, _) = m.eval(&tent);
            let s = m.g.subtree_classes(&r, eligible);
            let Some(&c) = top_by_subtree(&s, |i| r.reached[i as usize] && !tent.contains(&i) && !excluded.contains(&i), 1).first() else { break };
            tent.push(c);
            let (_, cnt) = m.eval(&tent);
            seg.push((c, cnt));
            if cur.0.saturating_sub(cnt.0) >= p.min_gain {
                let last = seg.len() - 1;
                for (k, (c, cnt)) in seg.into_iter().enumerate() {
                    let gain = if k == last { cur.0.saturating_sub(cnt.0) } else { 0 };
                    out.push(Step { node: c, gain, classes: m.base_count.0.saturating_sub(cnt.0), methods: m.base_count.1.saturating_sub(cnt.1), entry: k != last });
                }
                chosen = tent;
                cur = cnt;
                continue 'outer;
            }
        }
        match seg.first() {
            Some(&(c, _)) => {
                excluded.insert(c);
            }
            None => break,
        }
        if excluded.len() >= p.streak * 2 {
            break;
        }
    }
    out
}

/// 同机制分组：同包，或消失类集合重合（Jaccard ≥ 0.5）的门并为一组；只返回两个成员以上的组
pub fn groups(pkgs: &[String], removed: &[&[u32]]) -> Vec<Vec<usize>> {
    let n = pkgs.len();
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(p: &mut [usize], mut x: usize) -> usize {
        while p[x] != x {
            p[x] = p[p[x]];
            x = p[x];
        }
        x
    }
    let jaccard = |a: &[u32], b: &[u32]| {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        let (mut i, mut j, mut both) = (0, 0, 0usize);
        while i < a.len() && j < b.len() {
            match a[i].cmp(&b[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j += 1,
                std::cmp::Ordering::Equal => {
                    both += 1;
                    i += 1;
                    j += 1;
                }
            }
        }
        both as f64 / (a.len() + b.len() - both) as f64
    };
    for a in 0..n {
        for b in a + 1..n {
            if pkgs[a] == pkgs[b] || jaccard(removed[a], removed[b]) >= 0.5 {
                let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
                parent[ra.max(rb)] = ra.min(rb);
            }
        }
    }
    let mut by_root: std::collections::BTreeMap<usize, Vec<usize>> = Default::default();
    for i in 0..n {
        let r = find(&mut parent, i);
        by_root.entry(r).or_default().push(i);
    }
    by_root.into_values().filter(|g| g.len() > 1).collect()
}

#[cfg(test)]
mod tests {
    use super::super::super::cut::{Edges, NO_COND};
    use super::*;

    /// 根 → main；main@1 → hub（大团 30 类）、main@2 → hub（第二入口）；main@3 → small（6 类）
    fn fixture() -> TriggerGraph {
        let mut names: Vec<String> = ["R:r", "M:m:()V", "M:m:()V@1", "M:m:()V@2", "M:m:()V@3", "M:hub:()V", "M:hub:()V@0", "M:small:()V", "M:small:()V@0"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut edges = vec![(0, 1, NO_COND), (2, 5, NO_COND), (3, 5, NO_COND), (4, 7, NO_COND)];
        for k in 0..30 {
            names.push(format!("C:Big{k}"));
            edges.push((6, names.len() as u32 - 1, NO_COND));
        }
        for k in 0..6 {
            names.push(format!("C:S{k}"));
            edges.push((8, names.len() as u32 - 1, NO_COND));
        }
        TriggerGraph::build(Edges { names, edges }, |_| true)
    }

    #[test]
    fn greedy_crosses_multi_connected_blob() {
        let g = fixture();
        let m = Model::new(&g);
        assert_eq!(m.base_count.0, 36);
        let n = |s: &str| g.node(s).unwrap();
        assert_eq!(m.delta(&[n("M:m:()V@1")]).classes, 0);
        assert_eq!(m.delta(&[n("M:m:()V@3")]).classes, 6);
        let elig: Vec<bool> = (0..g.len()).map(|i| matches!(g.kinds[i], NodeKind::Site) && g.names[i].starts_with("M:m:")).collect();
        let pool = [n("M:m:()V@1"), n("M:m:()V@3")];
        let steps = greedy(&m, &pool, &elig, &GreedyParams { steps: 5, min_gain: 3, streak: 3, extra: 4 });
        // 第一步只有 small 有单步收益；随后两入口成段采纳
        assert_eq!(steps[0].node, n("M:m:()V@3"));
        assert_eq!(steps.len(), 3);
        assert!(steps[1].entry && !steps[2].entry);
        assert_eq!(steps[2].classes, 36);
        assert_eq!(steps[2].gain, 30);
    }

    #[test]
    fn grouping_by_package_and_overlap() {
        let r = [vec![1, 2, 3], vec![2, 3, 4], vec![9]];
        let refs: Vec<&[u32]> = r.iter().map(|v| v.as_slice()).collect();
        assert_eq!(groups(&["a".into(), "b".into(), "c".into()], &refs), vec![vec![0, 1]]);
        assert_eq!(groups(&["a".into(), "x".into(), "a".into()], &[&[1], &[2], &[3]]), vec![vec![0, 2]]);
    }
}
