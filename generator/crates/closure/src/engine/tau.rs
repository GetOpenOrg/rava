//! 引擎：类型恒等边（V11）。
//!
//! 节点 Y 的**封闭类型** τ(Y)：Y 的每一个输入都已按 τ(Y) 收窄——形参节点 `P(t, i)` 的全部入边都按被调
//! 声明类型过滤（`invoke.rs::edge`、`hub.rs`、`reflect_call.rs`），直接注入的值（接收者、open(声明类型)）
//! 也属于该类型。同理：返回值 `R` 按返回类型汇入、字段节点 `F` / `U` / `O` 按字段声明类型写入、
//! 枢纽 `HP` / `HR` 按枢纽的实参 / 返回声明类型中转，都以声明类型为封闭类型。于是 Y 的值集 s 满足 filter(s, f) = s 对一切 τ(Y) ⊑ f 成立（类 / open 的收窄逐项保持，
//! 见 `classes.rs::open_narrow`）：出边 Y → X（过滤 f，τ(Y) ⊑ f）在任何时刻都与 Object 边推送相同的值，
//! 称为**恒等边**。只经 Object 边与恒等边构成的强连通分量在不动点处类型集必然相等（沿环每条边目标 ⊇ 源），
//! 可与 Object 环一样合并（`scc.rs`）。
//!
//! 封闭性由入口校验守护：按非 τ 子类型过滤的入边、或含非 τ 成员的直接注入使节点失去封闭类型
//! （`tau_broken`，单调）；失去前已据此合并的计数进 `perf.tau`（期望为 0）。

use super::*;

impl<'a> Engine<'a> {
    /// 节点（序号）自身的封闭类型
    pub(super) fn node_tau(&self, i: u32) -> Option<u32> {
        if self.graph.tau_broken[i as usize] {
            return None;
        }
        match self.graph.node(i) {
            Node::P(t, k) => self.methods[t].ptypes.get(k as usize).copied().flatten(),
            Node::R(t) => self.methods[t].rtype,
            Node::F(f) | Node::U(f) | Node::O(_, f) => self.fields.get_index(f).and_then(|x| *x.1),
            Node::HP(h, j) => self.hubs.get(h as usize).and_then(|x| x.ptypes.get(j as usize).copied().flatten()),
            Node::HR(h) => self.hubs.get(h as usize).and_then(|x| x.ret),
            _ => None,
        }
    }

    /// 代表 r 的封闭类型（合并过的代表取各成员的封闭类型；成员值集在不动点处相等，任一成员的 τ 都成立）
    fn rep_taus(&self, r: u32) -> Vec<u32> {
        match self.graph.rep_taus.get(&r) {
            Some(v) => v.clone(),
            None if self.graph.members.contains_key(&r) => Vec::new(),
            None => self.node_tau(r).into_iter().collect(),
        }
    }

    /// 合并后的代表 r：按成员重算封闭类型表
    pub(super) fn refresh_rep_taus(&mut self, r: u32) {
        let Some(ms) = self.graph.members.get(&r) else { return };
        let mut v: Vec<u32> = ms.iter().filter_map(|&m| self.node_tau(m)).collect();
        v.sort_unstable();
        v.dedup();
        if v.is_empty() {
            self.graph.rep_taus.remove(&r);
        } else {
            self.graph.rep_taus.insert(r, v);
        }
    }

    /// 代表 rs 出发、过滤 f 的边是否恒等（Object 边或 τ(rs) ⊑ f）
    pub(super) fn ident_edge(&mut self, rs: u32, f: u32, obj: u32) -> bool {
        if f == obj {
            return true;
        }
        // 反射数组元素写入（OPEN_EXACT）按非恒等处理：只少合并，不改结果
        if f & (NOT_SUB | OPEN_EXACT) != 0 {
            return false;
        }
        let ts = self.rep_taus(rs);
        ts.into_iter().any(|t| t == f || self.sub(t, f))
    }

    /// 节点 i 失去封闭类型
    fn tau_break(&mut self, i: u32) {
        if std::mem::replace(&mut self.graph.tau_broken[i as usize], true) {
            return;
        }
        self.graph.tau_stats[0] += 1;
        let r = self.graph.rep(i);
        if self.graph.members.contains_key(&r) {
            self.graph.tau_stats[1] += 1;
            self.refresh_rep_taus(r);
        }
    }

    /// 入边 → 节点 i（过滤 f）：f 不是 τ(i) 的子类型时 i 失去封闭类型
    pub(super) fn tau_check_flow(&mut self, i: u32, f: u32) {
        let Some(t) = self.node_tau(i) else { return };
        if f == t || (f & (NOT_SUB | OPEN_EXACT) == 0 && self.sub(f, t)) || self.names[t as usize].as_ref() == OBJECT {
            return;
        }
        self.tau_break(i);
    }

    /// 直接注入节点 i：含不属于 τ(i) 的成员时 i 失去封闭类型
    pub(super) fn tau_check_add(&mut self, i: u32, s: &TypeSet) {
        let Some(t) = self.node_tau(i) else { return };
        if self.names[t as usize].as_ref() == OBJECT {
            return;
        }
        let ok = s.classes.iter().all(|x| self.sub(x, t)) && s.open.iter().all(|o| self.sub(o, t));
        if !ok {
            self.tau_break(i);
        }
    }
}
