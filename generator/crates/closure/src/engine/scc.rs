//! 引擎：类型流图的环合并。
//!
//! 只经 Object 过滤边（恒等推送）构成的强连通分量，不动点处各节点类型集必然相等：沿环每条边
//! 都要求目标 ⊇ 源。合并为一个代表后，类型集、出边、待推增量只存一份，环内推送消失——
//! 手写调用点数组读写（`E→W→E`，见 `hw_mem.rs`）把逃逸数组与各站点写入槽连成的大环
//! 是 DeepCopy 类用例流图的主体（计划 2026-09-30-closure-analyzer-performance.md §4.5）。
//!
//! 成员保留节点身份：读者 / 钩子仍按成员登记，代表增长时逐成员触发（`flow.rs::grown`）。
//! 空数组元素节点暂存而不接收值（`empty_arrays`），不参与合并。合并只提前让成员看到
//! 终态必然含有的值，单调不动点结果不变。

use std::time::Instant;

use super::*;

/// 两次环检测之间至少新接的流边数；且须达到现有边数的 1/4（检测总成本与边数呈对数轮次）
const SCC_MIN_EDGES: usize = 100_000;

impl<'a> Engine<'a> {
    pub(super) fn scc_due(&self) -> bool {
        let e = self.graph.edges_since;
        e >= SCC_MIN_EDGES && e * 4 >= self.graph.edge_count
    }

    /// 可参与合并的节点：代表自身，且不是暂存中的空数组元素节点
    fn scc_eligible(&self, i: u32) -> bool {
        if self.graph.rep(i) != i {
            return false;
        }
        match self.graph.node(i) {
            Node::E(x, _) => !self.empty_arrays.contains_key(&x),
            _ => true,
        }
    }

    /// Tarjan（迭代）求 Object 边子图上的非平凡强连通分量（各分量成员升序，分量按最小成员升序）
    fn object_sccs(&self, obj: u32) -> Vec<Vec<u32>> {
        let n = self.graph.node_count();
        const UNSEEN: u32 = u32::MAX;
        let mut index = vec![UNSEEN; n];
        let mut low = vec![0u32; n];
        let mut on_stack = vec![false; n];
        let mut stack: Vec<u32> = Vec::new();
        let mut call: Vec<(u32, usize)> = Vec::new();
        let mut next = 0u32;
        let mut out: Vec<Vec<u32>> = Vec::new();
        for v0 in 0..n as u32 {
            if index[v0 as usize] != UNSEEN || !self.scc_eligible(v0) || self.graph.edges[v0 as usize].is_empty() {
                continue;
            }
            index[v0 as usize] = next;
            low[v0 as usize] = next;
            next += 1;
            stack.push(v0);
            on_stack[v0 as usize] = true;
            call.push((v0, 0));
            while let Some(&(v, ei)) = call.last() {
                let vi = v as usize;
                if let Some(&(t, f)) = self.graph.edges[vi].get(ei) {
                    call.last_mut().expect("非空").1 += 1;
                    if f != obj {
                        continue;
                    }
                    let w = self.graph.rep(t);
                    if w == v || !self.scc_eligible(w) {
                        continue;
                    }
                    let wi = w as usize;
                    if index[wi] == UNSEEN {
                        index[wi] = next;
                        low[wi] = next;
                        next += 1;
                        stack.push(w);
                        on_stack[wi] = true;
                        call.push((w, 0));
                    } else if on_stack[wi] {
                        low[vi] = low[vi].min(index[wi]);
                    }
                    continue;
                }
                call.pop();
                if let Some(&(p, _)) = call.last() {
                    low[p as usize] = low[p as usize].min(low[vi]);
                }
                if low[vi] == index[vi] {
                    let mut comp = Vec::new();
                    while let Some(x) = stack.pop() {
                        on_stack[x as usize] = false;
                        comp.push(x);
                        if x == v {
                            break;
                        }
                    }
                    if comp.len() > 1 {
                        comp.sort_unstable();
                        out.push(comp);
                    }
                }
            }
        }
        out.sort_unstable_by_key(|c| c[0]);
        out
    }

    /// 检测并合并 Object 边上的环；重写全图出边（目标改指代表、去重、去掉代表内 Object 自环）
    pub(super) fn collapse_cycles(&mut self) {
        let t0 = Instant::now();
        self.graph.edges_since = 0;
        self.graph.scc_stats[0] += 1;
        let obj = self.id(OBJECT);
        let comps = self.object_sccs(obj);
        if comps.is_empty() {
            self.graph.scc_stats[2] += t0.elapsed().as_millis() as u64;
            return;
        }
        // 各分量：并集、各原组相对并集的增量（组成员在合并前取出）、待推增量、出边
        let mut fire: Vec<(Vec<u32>, TypeSet)> = Vec::new();
        let mut push: Vec<(u32, TypeSet)> = Vec::new();
        for comp in &comps {
            let a = comp[0];
            let mut u = TypeSet::default();
            for &c in comp {
                u.add_all(self.graph.set(c));
            }
            let mut pend = TypeSet::default();
            let mut edges: Vec<(u32, u32)> = Vec::new();
            for &c in comp {
                let own = self.graph.take_own(c);
                let d = TypeSet { classes: u.classes.minus(&own.classes), open: u.open.minus(&own.open) };
                if !d.is_empty() {
                    let ms = self.graph.members.get(&c).cloned().unwrap_or_else(|| vec![c]);
                    pend.add_all(&d);
                    fire.push((ms, d));
                }
                pend.add_all(&std::mem::take(&mut self.graph.delta[c as usize]));
                edges.extend(std::mem::take(&mut self.graph.edges[c as usize]));
            }
            for &b in &comp[1..] {
                self.graph.union_into(a, b);
            }
            self.graph.put_own(a, u);
            self.graph.edges[a as usize] = edges;
            self.graph.scc_stats[1] += (comp.len() - 1) as u64;
            if !pend.is_empty() {
                push.push((a, pend));
            }
        }
        // 全图出边改指代表并去重；去重索引按新边重建
        self.graph.clear_seen();
        let mut seen: HashSet<(u32, u32)> = HashSet::default();
        for s in 0..self.graph.node_count() {
            if self.graph.edges[s].is_empty() {
                continue;
            }
            let es = std::mem::take(&mut self.graph.edges[s]);
            let s32 = s as u32;
            seen.clear();
            let kept: Vec<(u32, u32)> = es
                .into_iter()
                .filter_map(|(t, f)| {
                    let r = self.graph.rep(t);
                    (!(r == s32 && f == obj) && seen.insert((r, f))).then_some((r, f))
                })
                .collect();
            self.graph.set_edges(s32, kept);
        }
        for (a, d) in push {
            self.queue_delta(a, &d);
        }
        for (ms, d) in fire {
            for m in ms {
                if !self.graph.hooked(m) {
                    continue;
                }
                let n = self.graph.node(m);
                self.node_grown(n, &d);
            }
        }
        self.graph.scc_stats[2] += t0.elapsed().as_millis() as u64;
    }
}
