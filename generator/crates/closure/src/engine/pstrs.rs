//! 引擎：形参字符串常量集——按名查找的名字来自本方法形参时，取流到该形参的全部字符串常量。
//!
//! 单调且与处理顺序无关：
//! - 集合只并不减，不走形参常量（`pvals`）的汇合格。汇合格先到的常量会被后到的调用点抬为 Top，
//!   若按汇合结果取常量，结果就取决于调用点接入的先后；
//! - 实参是字符串常量或含字面量来源的合流值：各字面量并入被调形参槽；实参来自调用方形参（透传，含合流）：登记子集边「调用方形参槽 → 被调形参槽」，
//!   常量集沿边传递（调用方形参上已有与日后新增的常量都会到达）；
//! - 派发枢纽同样有形参槽：调用点实参并入枢纽槽，枢纽槽流向父枢纽槽与各目标的形参槽；
//! - 方法形参槽增长时，读过该槽的按名查找站点入站点队列重跑（不在接边中途重入）。

use super::*;

/// 字符串常量集的槽：方法形参（方法，形参槽）/ 枢纽形参（枢纽，形参序号，不含接收者）
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(super) enum PSlot {
    M(usize, usize),
    H(u32, usize),
}

#[derive(Default)]
pub(super) struct PStrs {
    sets: HashMap<PSlot, BTreeSet<Rc<str>>>,
    succ: HashMap<PSlot, BTreeSet<PSlot>>,
    /// 读过方法形参槽的按名查找站点（偏移）
    sites: HashMap<(usize, usize), BTreeSet<u32>>,
}

impl<'a> Engine<'a> {
    /// 方法 m 形参槽 i 上的字符串常量；登记站点 (m, off) 为读者
    pub(super) fn pstr_read(&mut self, m: usize, i: usize, off: u32) -> Vec<Rc<str>> {
        self.pstr.sites.entry((m, i)).or_default().insert(off);
        self.pstr.sets.get(&PSlot::M(m, i)).into_iter().flatten().cloned().collect()
    }

    /// 读过方法形参槽的按名查找站点（形参槽被污染时重跑，engine/field_names.rs）
    pub(super) fn pstr_readers(&self, m: usize, i: usize) -> Vec<u32> {
        self.pstr.sites.get(&(m, i)).into_iter().flatten().copied().collect()
    }

    /// 常量并入槽 at，沿子集边传递
    fn pstr_add(&mut self, at: PSlot, strs: Vec<Rc<str>>) {
        let mut work = vec![(at, strs)];
        while let Some((s, xs)) = work.pop() {
            let set = self.pstr.sets.entry(s).or_default();
            let new: Vec<Rc<str>> = xs.into_iter().filter(|x| set.insert(x.clone())).collect();
            if new.is_empty() {
                continue;
            }
            if let PSlot::M(t, i) = s {
                for &off in self.pstr.sites.get(&(t, i)).into_iter().flatten() {
                    if self.in_swork.insert((t, off)) {
                        self.swork.push_back((t, off));
                    }
                }
            }
            for &n in self.pstr.succ.get(&s).into_iter().flatten() {
                work.push((n, new.clone()));
            }
        }
    }

    /// 子集边 from → to：from 已有的常量立即传递
    pub(super) fn pstr_edge(&mut self, from: PSlot, to: PSlot) {
        if from == to || !self.pstr.succ.entry(from).or_default().insert(to) {
            return;
        }
        let xs: Vec<Rc<str>> = self.pstr.sets.get(&from).into_iter().flatten().cloned().collect();
        if !xs.is_empty() {
            self.pstr_add(to, xs);
        }
    }

    /// 调用方 m 的调用点实参值 vals（不含接收者）流入槽 to(j)
    pub(super) fn pstr_site(&mut self, m: usize, vals: &[V], to: impl Fn(usize) -> PSlot) {
        for (j, v) in vals.iter().enumerate() {
            let lits = v.lits();
            if !lits.is_empty() {
                self.pstr_add(to(j), lits);
            }
            if let V::Ref { src, .. } = v {
                for s in src.iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), to(j));
                    }
                }
            }
        }
    }
}
