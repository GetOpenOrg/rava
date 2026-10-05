//! 引擎：字节码调用点接收者值集的版本记忆。
//!
//! 类型集只增不减（环合并后代表的集合是成员集合之并，同样只增），故来源节点的元素数之和不变即接收者值集不变。
//! 同一分析结果下站点的来源列表固定，另以来源身份的哈希校验。读者站点因实参或其他读者节点增长重跑时，
//! 接收者值集未变的调用点跳过取值集、接收者判定与增量登记：
//! - 虚调用：沿用上次的精确接收者与 open 枢纽（后续派发 / 枢纽接入各自去重，结果与重新计算相同）；
//! - 非虚调用（`edge_recv` 的站点去重）：增量为空，整体无事可做。
//!
//! 版本未命中（值集确有增长）时取值集也只做增量（[`Engine::value_since`]）：同一来源身份下记下已见过的精确类，
//! 重跑只取各来源相对它的新增——
//! - 虚调用：精确接收者 = 已见部分的接收者（上次记录）∪ 新增部分中 ⊂ 属主者。接收者判定逐元素、只依赖元素与属主，
//!   且子类型判定首次求得即记忆（`classes.rs::sub`），故与对全集重判逐元素相同、同为升序；
//! - 非虚调用：已见部分在上次都已登记进 `recv_done`（两者同时作废），`recv_delta` 对全集与对新增部分的结果相同。
//! open 部分元素少，每次全取。读者登记（`watch_node`）与全量取值逐来源同序。
//!
//! 记忆与 `recv_done` 同口径：分析重算 / 站点重接时作废（`worklist.rs::reset_offsets` / `reset_sites`）。

use super::*;
use std::hash::{Hash, Hasher};

/// 保留已见精确类的元素数下限（与 `IdSet` 稠密形态同阈值）
const SEEN_MIN: usize = super::idset::DENSE_AT;

/// 调用点接收者值集的版本与（虚调用的）接收者判定结果
pub(super) struct RecvFp {
    /// 来源身份哈希
    key: u64,
    /// 来源节点元素数之和
    ver: usize,
    /// 虚调用：精确接收者与 open 枢纽；非虚调用为 None
    recv: Option<(Rc<[u32]>, Rc<[u32]>)>,
    /// 已见过的来源精确类（增量取值用；只为元素多的站点保留，少时全取更省）
    seen: Option<IdSet>,
}

/// 增量取值的结果
pub(super) struct Since {
    /// 精确部分：增量时只含新增，否则为全集；open 部分恒为全集
    pub(super) s: TypeSet,
    /// 增量时：上次记录的精确接收者（虚调用）
    pub(super) prev: Option<Rc<[u32]>>,
    /// 取值后已见过的精确类（交给 `recv_fp_store`）
    pub(super) seen: Option<IdSet>,
}

impl RecvFp {
    /// 虚调用上次的精确接收者（升序；剖析用）
    pub(super) fn recv_list(&self) -> Option<&[u32]> {
        self.recv.as_ref().map(|r| &r.0[..])
    }
}

impl<'a> Engine<'a> {
    /// 来源列表的（身份哈希, 元素数之和）；同时按 `value_set` 口径登记当前站点 / lambda 调用为来源节点的读者
    pub(super) fn feeds_version(&mut self, fs: &[Feed]) -> (u64, usize) {
        let mut h = sets::FxHasher::default();
        let mut ver = 0usize;
        for f in fs {
            match f {
                Feed::N(n) => {
                    self.watch_node(*n);
                    n.hash(&mut h);
                    ver += self.graph.get(n).map_or(0, |s| s.classes.len() + s.open.len());
                }
                Feed::S(s) => {
                    h.write_u8(0xff);
                    h.write_usize(s.classes.len());
                    h.write_usize(s.open.len());
                    ver += s.classes.len() + s.open.len();
                }
            }
        }
        (h.finish(), ver)
    }

    /// 站点 (m, off) 的接收者值集自上次记录以来未变：返回上次的接收者判定（非虚调用为 `Some(None)`）
    pub(super) fn recv_fp_hit(&mut self, m: usize, off: u32, v: (u64, usize)) -> Option<Option<(Rc<[u32]>, Rc<[u32]>)>> {
        let hit = self.recv_fp.get(&m).and_then(|d| d.get(&off)).filter(|r| (r.key, r.ver) == v).map(|r| r.recv.clone());
        self.ctx.stats.borrow_mut().recv_fp[usize::from(hit.is_none())] += 1;
        hit
    }

    pub(super) fn recv_fp_store(&mut self, m: usize, off: u32, v: (u64, usize), recv: Option<(Rc<[u32]>, Rc<[u32]>)>, seen: Option<IdSet>) {
        self.recv_fp.entry(m).or_default().insert(off, RecvFp { key: v.0, ver: v.1, recv, seen });
    }

    /// 站点 (m, off) 的来源值集，精确部分相对上次记录（来源身份哈希为 key）只取新增。`virt`：虚调用，增量另需上次的精确接收者。
    /// 无可用记录（首次、来源身份变化、上次元素少未保留）时与 `value_set` 相同
    pub(super) fn value_since(&mut self, m: usize, off: u32, fs: &[Feed], key: u64, virt: bool) -> Since {
        let rec = self.recv_fp.get_mut(&m).and_then(|d| d.get_mut(&off)).filter(|r| r.key == key);
        let prev = rec.and_then(|r| {
            let prev = match &r.recv {
                Some((rs, _)) => Some(rs.clone()),
                None if virt => return None,
                None => None,
            };
            Some((r.seen.take()?, prev))
        });
        let Some((mut seen, prev)) = prev else {
            let s = self.value_set(fs);
            let seen = (s.classes.len() >= SEEN_MIN).then(|| s.classes.clone());
            return Since { s, prev: None, seen };
        };
        let mut fresh = IdSet::default();
        let mut open = IdSet::default();
        for f in fs {
            let src = match f {
                Feed::N(n) => {
                    self.watch_node(*n);
                    self.graph.get(n)
                }
                Feed::S(s) => Some(s),
            };
            if let Some(s) = src {
                fresh.union_with(&s.classes.minus(&seen));
                open.union_with(&s.open);
            }
        }
        seen.union_with(&fresh);
        Since { s: TypeSet { classes: fresh, open }, prev, seen: Some(seen) }
    }
}
