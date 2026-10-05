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
        // 全边图 Tarjan（迭代），输出分量按逆拓扑序（汇点先出）
        const UNSEEN: u32 = u32::MAX;
        let mut index = vec![UNSEEN; n];
        let mut low = vec![0u32; n];
        let mut on = vec![false; n];
        let mut st: Vec<u32> = Vec::new();
        let mut call: Vec<(u32, usize)> = Vec::new();
        let mut next = 0u32;
        let mut comps: Vec<Vec<u32>> = Vec::new();
        for &v0 in &reps {
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
                if let Some(&(t, _)) = g.edges[vi].get(ei) {
                    call.last_mut().expect("非空").1 += 1;
                    let w = g.rep(t);
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
}
