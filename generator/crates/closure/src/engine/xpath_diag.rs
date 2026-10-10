//! 引擎：值集合并点诊断（`--flows @xpath:<方法键子串>|<形参序号>`）。
//!
//! 取匹配方法中该形参值集最大的克隆，从值集里等距抽 4 个元素，各沿流边反向走到源头（最短路径），
//! 回答「某个克隆的形参为什么含有不属于它的对象」——路径上第一个值集突然变大的节点即合并点。
//! 只为所抽元素建反向边（不展开整张图的边表），大图上内存可控。

use super::*;

impl Engine<'_> {
    pub(super) fn xpath_report(&self, q: &str) -> Vec<String> {
        let Some((mq, j)) = q.split_once('|') else { return vec!["用法：@xpath:<方法键子串>|<形参序号>".into()] };
        let Ok(j) = j.parse::<u16>() else { return vec![format!("形参序号无效：{j}")] };
        let size = |n: Node| self.graph.get(&n).map_or(0, |s| s.classes.len());
        let best = (0..self.methods.len())
            .filter(|&i| self.methods[i].key.to_string().contains(mq))
            .max_by_key(|&i| (size(Node::P(i, j)), std::cmp::Reverse(i)));
        let Some(m) = best else { return vec![format!("无匹配方法：{mq}")] };
        let start = Node::P(m, j);
        let set: Vec<u32> = self.graph.get(&start).map(|s| s.classes.iter().collect()).unwrap_or_default();
        let mut out = vec![format!("@xpath {q}: 起点 {}（|{}|）", self.node_str(start), set.len())];
        if set.is_empty() {
            return out;
        }
        let n = set.len();
        let mut picks: Vec<u32> = [0, n / 3, 2 * n / 3, n - 1].iter().map(|&k| set[k]).collect();
        picks.dedup();
        let total = self.graph.edges.len();
        for c in picks {
            let has: Vec<bool> = (0..total as u32).map(|i| self.graph.set(i).classes.contains(&c)).collect();
            let mut rev: HashMap<u32, Vec<u32>> = HashMap::default();
            for (s, es) in self.graph.edges.iter().enumerate() {
                if !has[s] {
                    continue;
                }
                for &(d, _) in es {
                    if has[d as usize] {
                        rev.entry(d).or_default().push(s as u32);
                    }
                }
            }
            let Some(st) = self.graph.rep_of(&start) else { continue };
            let mut prev: HashMap<u32, Option<u32>> = HashMap::default();
            prev.insert(st, None);
            let mut q: VecDeque<u32> = VecDeque::from([st]);
            let mut last = st;
            while let Some(x) = q.pop_front() {
                last = x;
                let Some(ps) = rev.get(&x) else { break };
                let mut ps: Vec<u32> = ps.iter().copied().filter(|p| !prev.contains_key(p)).collect();
                ps.sort_unstable();
                if ps.is_empty() && q.is_empty() {
                    break;
                }
                for p in ps {
                    prev.insert(p, Some(x));
                    q.push_back(p);
                }
            }
            out.push(format!("  元素 {}：", self.names[c as usize]));
            let mut cur = Some(last);
            while let Some(x) = cur {
                let nd = *self.graph.node_at(x);
                out.push(format!("    {} (|{}|)", self.node_str(nd), self.graph.set(x).classes.len()));
                cur = prev.get(&x).copied().flatten();
            }
        }
        out
    }
}
