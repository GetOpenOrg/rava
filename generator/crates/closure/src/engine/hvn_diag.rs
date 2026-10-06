//! 引擎：等价节点合并潜力诊断（`--flows @hvn`，V11 剖析用，只读）。
//!
//! 在终态流图（代表层）上做离线哈希值编号（HVN）：按全边图的强连通分量拓扑序给每个代表一个标签——
//! 有直接注入（`add_to`）或处在环上的代表取独有标签；其余代表的标签由入边签名
//! `{(源标签, 过滤类型)}` 决定。标签相同的代表在不动点处类型集必然相等。
//! 输出合并前后的边数、按边累计的元素推送量与估算推送次数（源出队次数之和），按源种类拆分。

use super::stats::kind_ix;
use super::*;

const KIND_NAMES: [&str; 19] = ["P", "R", "Spool", "Scatch", "S", "F", "U", "O", "E", "Array", "A", "W", "HP", "HR", "Esc", "G", "Rcall", "K", "NR"];

impl<'a> Engine<'a> {
    pub(super) fn hvn_report(&self) -> Vec<String> {
        let g = &self.graph;
        let n = g.node_count();
        let reps: Vec<u32> = (0..n as u32).filter(|&i| g.rep(i) == i).collect();
        // 入边（代表层）
        let mut ins: Vec<Vec<(u32, u32)>> = vec![Vec::new(); n];
        for &s in &reps {
            for &(d, f) in &g.edges[s as usize] {
                ins[g.rep(d) as usize].push((s, f));
            }
        }
        let injected = |r: u32| -> bool {
            match g.members.get(&r) {
                Some(ms) => ms.iter().any(|&m| g.injected[m as usize]),
                None => g.injected[r as usize],
            }
        };
        // 全边图强连通分量（逆拓扑序：汇点先出）
        let comps = self.rep_sccs(&reps, |_, _, _| true);
        // 拓扑序（源先）编号
        const EMPTY: u32 = 0;
        let mut label = vec![u32::MAX; n];
        let mut sigs: HashMap<Vec<(u32, u32)>, u32> = HashMap::default();
        let mut next_label = 1u32;
        let mut cyclic = 0usize;
        for c in comps.iter().rev() {
            let cyc = c.len() > 1 || g.edges[c[0] as usize].iter().any(|&(d, _)| g.rep(d) == c[0]);
            for &r in c {
                if cyc {
                    cyclic += 1;
                }
                let l = if cyc || injected(r) {
                    next_label += 1;
                    next_label - 1
                } else {
                    let mut sig: Vec<(u32, u32)> = ins[r as usize].iter().map(|&(s, f)| (label[s as usize], f)).filter(|&(l, _)| l != EMPTY).collect();
                    sig.sort_unstable();
                    sig.dedup();
                    if sig.is_empty() {
                        EMPTY
                    } else {
                        *sigs.entry(sig).or_insert_with(|| {
                            next_label += 1;
                            next_label - 1
                        })
                    }
                };
                label[r as usize] = l;
            }
        }
        // 统计：非空代表数 / 类数；边、元素推送量、估算推送次数（合并前后，按源种类）
        let size = |r: u32| -> u64 {
            let s = g.set(r);
            (s.classes.len() + s.open.len()) as u64
        };
        let nonempty: Vec<u32> = reps.iter().copied().filter(|&r| size(r) > 0).collect();
        let classes: HashSet<u32> = nonempty.iter().map(|&r| label[r as usize]).collect();
        // 类 → 最大出队次数（合并后代表的出队次数估计）
        let mut cls_pops: HashMap<u32, u64> = HashMap::default();
        for &r in &nonempty {
            let p = g.push_src[r as usize][0];
            let e = cls_pops.entry(label[r as usize]).or_default();
            *e = (*e).max(p);
        }
        let mut before = [[0u64; 3]; 19];
        let mut after = [[0u64; 3]; 19];
        let mut merged: HashSet<(u32, u32, u32)> = HashSet::default();
        for &s in &nonempty {
            let k = kind_ix(&g.node(s));
            let sz = size(s);
            let pops = g.push_src[s as usize][0];
            let ls = label[s as usize];
            for &(d, f) in &g.edges[s as usize] {
                before[k][0] += 1;
                before[k][1] += sz;
                before[k][2] += pops;
                if merged.insert((ls, label[g.rep(d) as usize], f)) {
                    after[k][0] += 1;
                    after[k][1] += sz;
                    after[k][2] += cls_pops[&ls];
                }
            }
        }
        let tot = |a: &[[u64; 3]; 19], j: usize| a.iter().map(|x| x[j]).sum::<u64>();
        let mut out = vec![
            format!("  代表 {} / 非空 {} / 等价类 {} / 环上 {} / 注入 {}", reps.len(), nonempty.len(), classes.len(), cyclic, reps.iter().filter(|&&r| injected(r)).count()),
            format!("  边 {} → {}；元素推送 {} → {}；估算推送 {} → {}", tot(&before, 0), tot(&after, 0), tot(&before, 1), tot(&after, 1), tot(&before, 2), tot(&after, 2)),
        ];
        let mut ks: Vec<usize> = (0..19).filter(|&k| before[k][0] > 0).collect();
        ks.sort_by_key(|&k| std::cmp::Reverse(before[k][2]));
        for k in ks {
            out.push(format!(
                "  {:6} 边 {} → {}；元素 {} → {}；推送 {} → {}",
                KIND_NAMES[k], before[k][0], after[k][0], before[k][1], after[k][1], before[k][2], after[k][2]
            ));
        }
        // 最大的若干等价类（成员数、种类、样例）
        let mut by: HashMap<u32, Vec<u32>> = HashMap::default();
        for &r in &nonempty {
            by.entry(label[r as usize]).or_default().push(r);
        }
        let mut big: Vec<(u32, Vec<u32>)> = by.into_iter().filter(|(_, v)| v.len() > 1).collect();
        big.sort_by_key(|(l, v)| (std::cmp::Reverse(v.len() as u64 * size(v[0]) * g.edges[v[0] as usize].len().max(1) as u64), *l));
        let merged_nodes: usize = big.iter().map(|(_, v)| v.len() - 1).sum();
        out.push(format!("  可并入的代表 {merged_nodes}，多成员类 {}", big.len()));
        for (_, v) in big.iter().take(30) {
            let ex: Vec<String> = v.iter().take(2).map(|&r| self.node_str(g.node(r)).chars().take(120).collect()).collect();
            out.push(format!("    ×{} |{}| 出 {}：{}", v.len(), size(v[0]), g.edges[v[0] as usize].len(), ex.join(" ｜ ")));
        }
        out
    }

    /// 代表层流图上只取满足 keep(源, 目标, 过滤) 的边求强连通分量（迭代 Tarjan，分量按逆拓扑序）
    fn rep_sccs(&self, reps: &[u32], keep: impl Fn(u32, u32, u32) -> bool) -> Vec<Vec<u32>> {
        let g = &self.graph;
        let n = g.node_count();
        const UNSEEN: u32 = u32::MAX;
        let mut index = vec![UNSEEN; n];
        let mut low = vec![0u32; n];
        let mut on = vec![false; n];
        let mut st: Vec<u32> = Vec::new();
        let mut call: Vec<(u32, usize)> = Vec::new();
        let mut next = 0u32;
        let mut comps: Vec<Vec<u32>> = Vec::new();
        for &v0 in reps {
            if index[v0 as usize] != UNSEEN {
                continue;
            }
            index[v0 as usize] = next;
            low[v0 as usize] = next;
            next += 1;
            st.push(v0);
            on[v0 as usize] = true;
            call.push((v0, 0));
            while let Some(&(v, ei)) = call.last() {
                let vi = v as usize;
                if let Some(&(t, f)) = g.edges[vi].get(ei) {
                    call.last_mut().expect("非空").1 += 1;
                    let w = g.rep(t);
                    if !keep(v, w, f) {
                        continue;
                    }
                    let wi = w as usize;
                    if index[wi] == UNSEEN {
                        index[wi] = next;
                        low[wi] = next;
                        next += 1;
                        st.push(w);
                        on[wi] = true;
                        call.push((w, 0));
                    } else if on[wi] {
                        low[vi] = low[vi].min(index[wi]);
                    }
                    continue;
                }
                call.pop();
                if let Some(&(p, _)) = call.last() {
                    low[p as usize] = low[p as usize].min(low[vi]);
                }
                if low[vi] == index[vi] {
                    let mut c = Vec::new();
                    while let Some(x) = st.pop() {
                        on[x as usize] = false;
                        c.push(x);
                        if x == v {
                            break;
                        }
                    }
                    comps.push(c);
                }
            }
        }
        comps
    }

    /// 类型恒等边的环合并潜力：边 Y→X（过滤 f）在 Y 的值恒属 τ(Y) ⊑ f 时与 Object 边等价。
    /// τ 取两种口径——静态（形参节点取被调声明类型）与终态动态（未受注入、全部入边过滤类型相同）
    pub(super) fn tau_report(&self) -> Vec<String> {
        let g = &self.graph;
        let n = g.node_count();
        let reps: Vec<u32> = (0..n as u32).filter(|&i| g.rep(i) == i).collect();
        let obj = self.ids.get(OBJECT).copied().unwrap_or(u32::MAX);
        let tau_static = |y: u32| -> Option<u32> {
            if g.members.contains_key(&y) {
                return None;
            }
            match g.node(y) {
                Node::P(t, i) => self.methods[t].ptypes.get(i as usize).copied().flatten(),
                _ => None,
            }
        };
        let mut in_f: Vec<Option<u32>> = vec![None; n];
        let mut mixed = vec![false; n];
        for &s in &reps {
            for &(d, f) in &g.edges[s as usize] {
                let d = g.rep(d) as usize;
                match in_f[d] {
                    None => in_f[d] = Some(f),
                    Some(x) if x != f => mixed[d] = true,
                    _ => {}
                }
            }
        }
        let injected = |r: u32| -> bool {
            match g.members.get(&r) {
                Some(ms) => ms.iter().any(|&m| g.injected[m as usize]),
                None => g.injected[r as usize],
            }
        };
        let tau_dyn = |y: u32| -> Option<u32> {
            if injected(y) || mixed[y as usize] {
                return tau_static(y);
            }
            in_f[y as usize].filter(|&f| f & (NOT_SUB | OPEN_EXACT) == 0 && f != obj).or_else(|| tau_static(y))
        };
        let le = |a: u32, f: u32| -> bool { a == f || self.h.is_subtype(&self.names[a as usize], &self.names[f as usize]) };
        let ident = |tau: &dyn Fn(u32) -> Option<u32>, y: u32, f: u32| -> bool {
            f == obj || (f & (NOT_SUB | OPEN_EXACT) == 0 && tau(y).is_some_and(|t| le(t, f)))
        };
        let mut out = Vec::new();
        for (name, which) in [("静态 τ", 0), ("终态动态 τ", 1)] {
            let id = |y: u32, f: u32| -> bool {
                if which == 0 {
                    ident(&tau_static, y, f)
                } else {
                    ident(&tau_dyn, y, f)
                }
            };
            let comps = self.rep_sccs(&reps, |y, _, f| id(y, f));
            let mut comp = vec![u32::MAX; n];
            let mut merged = 0usize;
            for (k, c) in comps.iter().enumerate() {
                if c.len() > 1 {
                    merged += c.len() - 1;
                    for &x in c {
                        comp[x as usize] = k as u32;
                    }
                }
            }
            // 分量内的边（合并后消失）与分量间重复边（合并后去重）的推送
            let mut seen: HashSet<(u32, u32, u32)> = HashSet::default();
            let (mut e0, mut e1, mut p0, mut p1, mut ident_edges) = (0u64, 0u64, 0u64, 0u64, 0u64);
            let key = |x: u32| if comp[x as usize] == u32::MAX { x } else { u32::MAX - 1 - comp[x as usize] };
            for &s in &reps {
                let pops = g.push_src[s as usize][0];
                for &(d, f) in &g.edges[s as usize] {
                    let d = g.rep(d);
                    e0 += 1;
                    p0 += pops;
                    if f != obj && id(s, f) {
                        ident_edges += 1;
                    }
                    let (ks, kd) = (key(s), key(d));
                    let fe = if id(s, f) { obj } else { f };
                    if ks == kd && fe == obj {
                        continue;
                    }
                    if seen.insert((ks, kd, fe)) {
                        e1 += 1;
                        p1 += pops;
                    }
                }
            }
            out.push(format!("  {name}：恒等的非 Object 边 {ident_edges}；可并节点 {merged}；边 {e0} → {e1}；估算推送 {p0} → {p1}（分量内取原源出队次数）"));
        }
        out
    }
}
