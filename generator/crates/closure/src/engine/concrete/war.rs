//! 残差重放的读后写审计（用户决策 U8，2026-10-06）。
//!
//! 残差调用 / 残差区段 / 运行期初始化类在运行期执行时读到的是**映像终态**，JVM 中它们在延迟点读到的是
//! 当时的状态。若构建期在延迟点之后又写了它们读过的位置，重放会读到「未来」的值。审计：
//! - 写入时刻：引导求值的每次写入（静态字段、实例字段、数组、VM 单元）推进逻辑时钟并记位置的最后写入
//!   时刻；撤回时恢复（撤回的写入不进映像）；
//! - 读集：每个残差在构建期执行到延迟点为止（区段含全部探索路径）读过的位置；
//! - 交集：读集中最后写入时刻晚于该残差登记时刻的位置。非空即构建失败。
//!
//! 已知边界：延迟点之后的读构建期观测不到（与第 1 步「残差的写集」边界相同）；启动重放 native 的读集
//! 由 native 语义决定，不在审计内。

use std::cell::RefCell;
use std::fmt::Write as _;

use super::journal::Rec;
use super::vm::*;
use super::*;

/// 映像位置
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub(super) enum Loc {
    S(u32),
    F(u32, u32),
    A(u32),
    C(u32),
}

#[derive(Default)]
pub(super) struct War {
    /// 读日志（最外层标记出栈时清空）
    pub rlog: RefCell<Vec<Loc>>,
    /// 位置 → 最后写入时刻
    pub wtime: HashMap<Loc, u64>,
    /// 写入时刻的撤销日志（位置, 原时刻）
    pub wundo: Vec<(Loc, Option<u64>)>,
    pub clock: u64,
    /// 残差的读集：(残差记录下标, 登记时刻, 读集)
    pub reads: Vec<(usize, u64, Vec<Loc>)>,
}

/// 审计结果：残差数、读集总规模、交集（残差描述, 位置描述）
pub(super) struct WarReport {
    pub residuals: usize,
    pub reads: usize,
    pub hits: Vec<(String, String)>,
}

impl Vm {
    pub(super) fn war_read(&self, l: Loc) {
        if self.bj.marked() {
            self.bj.war.rlog.borrow_mut().push(l);
        }
    }

    pub(super) fn war_write(&mut self, l: Loc) {
        let w = &mut self.bj.war;
        w.clock += 1;
        let old = w.wtime.insert(l, w.clock);
        if self.bj.marked() {
            self.bj.war.wundo.push((l, old));
        }
    }

    /// 登记最近一条残差记录的读集（标记 m 起的读日志）
    pub(super) fn war_capture(&mut self, rl: usize) {
        let Some(ri) = self.bj.recs.len().checked_sub(1) else { return };
        let mut v: Vec<Loc> = self.bj.war.rlog.borrow().get(rl..).map(<[Loc]>::to_vec).unwrap_or_default();
        v.sort_unstable();
        v.dedup();
        let t = self.bj.war.clock;
        self.bj.war.reads.push((ri, t, v));
    }

    fn loc_text(&self, l: Loc) -> String {
        let f = |k: u32| self.fnames.get(k as usize).map_or_else(|| format!("#{k}"), |(d, n)| format!("{d}.{n}"));
        match l {
            Loc::S(k) => format!("静态 {}", f(k)),
            Loc::F(o, k) => format!("{} 对象的字段 {}", self.heap[o as usize].ty, f(k)),
            Loc::A(o) => format!("数组 {}", self.heap[o as usize].ty),
            Loc::C(i) => format!("VM 单元 {}", self.cells.get(i as usize).map_or("?", |c| &*c.0)),
        }
    }

    pub(super) fn war_audit(&self) -> WarReport {
        let mut hits = Vec::new();
        let mut reads = 0;
        for (ri, t, locs) in &self.bj.war.reads {
            reads += locs.len();
            for &l in locs {
                if self.bj.war.wtime.get(&l).is_some_and(|w| w > t) {
                    let r = match self.bj.recs.get(*ri) {
                        Some(Rec::RuntimeInit { class, .. }) => format!("运行期初始化 {class}"),
                        Some(Rec::Call { phase, off, callee, .. }) => format!("残差调用 {phase}@{off} → {callee}"),
                        Some(Rec::Region { phase, start, end, .. }) => format!("残差区段 {phase} [{start}, {end:?})"),
                        Some(r) => format!("{r:?}"),
                        None => "?".to_string(),
                    };
                    hits.push((r, self.loc_text(l)));
                }
            }
        }
        WarReport { residuals: self.bj.war.reads.len(), reads, hits }
    }
}

impl WarReport {
    pub(super) fn markdown(&self, out: &mut String) {
        let _ = writeln!(out, "\n## 残差重放读后写审计（U8，{} 个残差，读集 {} 个位置，交集 {}）\n", self.residuals, self.reads, self.hits.len());
        for (r, l) in self.hits.iter().take(40) {
            let _ = writeln!(out, "- {r}：{l}");
        }
        if self.hits.len() > 40 {
            let _ = writeln!(out, "- ……共 {} 项", self.hits.len());
        }
    }
}
