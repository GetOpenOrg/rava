//! 残差重放的读后写审计（用户决策 U8，2026-10-06）。
//!
//! 残差调用 / 残差区段 / 运行期初始化类在运行期执行时读到的是**映像终态**，JVM 中它们在延迟点读到的是
//! 当时的状态。若构建期在延迟点之后又写了它们读过的位置，重放会读到「未来」的值。审计：
//! - 写入时刻：引导求值的每次写入（静态字段、实例字段、数组、VM 单元）推进逻辑时钟并记位置的最后写入
//!   时刻；撤回时恢复（撤回的写入不进映像）；
//! - 读集：每个残差在构建期执行到延迟点为止（区段含全部探索路径）读过的**外部**位置。窗口内自产的
//!   位置不计：先被窗口内写过再读（含窗口内触发的类初始化写静态字段）、或属于窗口内分配的对象。
//!   重放时这些读取由重放自身的写入或分配供值；窗口内类初始化在运行期因映像中该类已初始化而跳过，
//!   读到的是映像中同一 `<clinit>` 的结果——该 `<clinit>` 在窗口内的外部读取同样在读集内受审计，
//!   映像值与窗口值不同必然经由某个读集位置在登记后被写，交集即报出；
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
    /// 读写日志（位置, 是否写入；最外层标记出栈时清空）
    pub rlog: RefCell<Vec<(Loc, bool)>>,
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
            self.bj.war.rlog.borrow_mut().push((l, false));
        }
    }

    pub(super) fn war_write(&mut self, l: Loc) {
        let w = &mut self.bj.war;
        w.clock += 1;
        let old = w.wtime.insert(l, w.clock);
        if self.bj.marked() {
            self.bj.war.wundo.push((l, old));
            self.bj.war.rlog.borrow_mut().push((l, true));
        }
    }

    /// 登记最近一条残差记录的读集（标记起的读写日志 `rl..`；`heap` 为标记时的堆规模，此后分配的对象属窗口内）
    pub(super) fn war_capture(&mut self, rl: usize, heap: usize) {
        let Some(ri) = self.bj.recs.len().checked_sub(1) else { return };
        let mut v = self.window_reads(rl, heap);
        v.sort_unstable();
        v.dedup();
        let t = self.bj.war.clock;
        self.bj.war.reads.push((ri, t, v));
    }

    /// 撤回到标记时压缩读写日志：撤回段只留外部读，写入作废。
    /// 外层窗口随后探索的兄弟路径看不到被撤回路径的写入，不会把自己的外部读误判为窗口内自产
    pub(super) fn war_rollback(&self, rl: usize, heap: usize) {
        let v = self.window_reads(rl, heap);
        let mut log = self.bj.war.rlog.borrow_mut();
        log.truncate(rl);
        log.extend(v.into_iter().map(|l| (l, false)));
    }

    /// 窗口 `rl..` 的外部读：剔除窗口内先写后读的位置与窗口内分配对象的位置。
    /// 类镜像由 VM 缓存持有、撤回后仍在映像中，按已有对象记（与日志的 `logging` 口径相同）
    fn window_reads(&self, rl: usize, heap: usize) -> Vec<Loc> {
        let fresh = |o: u32| o as usize >= heap && !self.mirror_of.contains_key(&o);
        external_reads(self.bj.war.rlog.borrow().get(rl..).unwrap_or_default(), fresh)
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

/// 读写日志的外部读（保持次序）：剔除先写后读的位置与 `fresh` 对象的位置
fn external_reads(log: &[(Loc, bool)], fresh: impl Fn(u32) -> bool) -> Vec<Loc> {
    let mut wrote = std::collections::HashSet::new();
    let mut v = Vec::new();
    for &(l, w) in log {
        let fresh = matches!(l, Loc::F(o, _) | Loc::A(o) if fresh(o));
        if w {
            wrote.insert(l);
        } else if !fresh && !wrote.contains(&l) {
            v.push(l);
        }
    }
    v
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_reads_drop_own_writes_and_fresh_objects() {
        let log = [
            (Loc::S(1), false),
            (Loc::S(2), true),
            (Loc::S(2), false),
            (Loc::F(9, 0), false),
            (Loc::A(3), false),
            (Loc::S(1), true),
            (Loc::S(1), false),
        ];
        assert_eq!(external_reads(&log, |o| o >= 5), vec![Loc::S(1), Loc::A(3)]);
    }
}
