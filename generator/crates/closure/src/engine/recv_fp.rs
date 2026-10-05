//! 引擎：字节码调用点接收者值集的版本记忆。
//!
//! 类型集只增不减（环合并后代表的集合是成员集合之并，同样只增），故来源节点的元素数之和不变即接收者值集不变。
//! 同一分析结果下站点的来源列表固定，另以来源身份的哈希校验。读者站点因实参或其他读者节点增长重跑时，
//! 接收者值集未变的调用点跳过取值集、接收者判定与增量登记：
//! - 虚调用：沿用上次的精确接收者与 open 枢纽（后续派发 / 枢纽接入各自去重，结果与重新计算相同）；
//! - 非虚调用（`edge_recv` 的站点去重）：增量为空，整体无事可做。
//!
//! 记忆与 `recv_done` 同口径：分析重算 / 站点重接时作废（`worklist.rs::reset_offsets` / `reset_sites`）。

use super::*;
use std::hash::{Hash, Hasher};

/// 调用点接收者值集的版本与（虚调用的）接收者判定结果
pub(super) struct RecvFp {
    /// 来源身份哈希
    key: u64,
    /// 来源节点元素数之和
    ver: usize,
    /// 虚调用：精确接收者与 open 枢纽；非虚调用为 None
    recv: Option<(Rc<[u32]>, Rc<[u32]>)>,
}

impl<'a> Engine<'a> {
    /// 来源列表的（身份哈希, 元素数之和）；同时按 `value_set` 口径登记当前站点 / lambda 调用为来源节点的读者
    pub(super) fn feeds_version(&mut self, fs: &[Feed]) -> (u64, usize) {
        let mut h = sets::FxHasher::default();
        let mut ver = 0usize;
        for f in fs {
            match f {
                Feed::N(n) => {
                    if let Some(c) = self.cur_call {
                        self.call_watch.entry(*n).or_default().insert(c);
                    } else if let Some(w) = self.cur_site {
                        self.watch.entry(*n).or_default().insert(w);
                    }
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

    pub(super) fn recv_fp_store(&mut self, m: usize, off: u32, v: (u64, usize), recv: Option<(Rc<[u32]>, Rc<[u32]>)>) {
        self.recv_fp.entry(m).or_default().insert(off, RecvFp { key: v.0, ver: v.1, recv });
    }
}
