//! raw-audit 构造事件计数（← `codegen/raw_audit.py` 的 `raw_expr` / `raw_stmt` 运行时口径）。
//!
//! [`Raw`](crate::Raw) 逃生舱只能经 [`Expr::raw`](crate::Expr::raw) / [`Stmt::raw`](crate::Stmt::raw) /
//! [`Item::raw`](crate::Item::raw) 构造（字段私有），构造即计数；终态 0。位点剖面（`--raw-sites`）
//! 开启后按构造调用位点（`#[track_caller]`，`文件:行:列`）累计，供按热点收敛。
//!
//! 计数为线程局部（并行单元测试互不干扰）。并行发射的工作线程开工前按调用方状态
//! [`enable_sites`]，收工时 [`take_local`] 取走本线程账本，由调用方 [`absorb`] 并入自身——
//! 计数为加法、位点表有序，合并结果与串行发射一致。

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::panic::Location;

/// 逃生舱种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RawKind {
    Expr,
    Stmt,
    Item,
}

impl RawKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RawKind::Expr => "raw_expr",
            RawKind::Stmt => "raw_stmt",
            RawKind::Item => "raw_item",
        }
    }
}

thread_local! {
    static COUNTS: Cell<[usize; 3]> = const { Cell::new([0; 3]) };
    static SITES: RefCell<Option<BTreeMap<(RawKind, String), usize>>> = const { RefCell::new(None) };
}

/// 登记一次构造（`Location` 为构造调用位点）
pub(crate) fn record(kind: RawKind, at: &'static Location<'static>) {
    COUNTS.with(|c| {
        let mut v = c.get();
        v[kind as usize] += 1;
        c.set(v);
    });
    SITES.with(|s| {
        if let Some(m) = s.borrow_mut().as_mut() {
            let site = format!("{}:{}:{}", at.file(), at.line(), at.column());
            *m.entry((kind, site)).or_default() += 1;
        }
    });
}

/// 本线程累计的构造次数
pub fn count(kind: RawKind) -> usize {
    COUNTS.with(|c| c.get()[kind as usize])
}

/// 开启位点剖面（此后的构造按位点累计）
pub fn enable_sites() {
    SITES.with(|s| {
        s.borrow_mut().get_or_insert_with(BTreeMap::new);
    });
}

/// 位点剖面 `(次数, 种类, 位点)`，次数降序（同次数按种类、位点升序）；未开启时空
pub fn sites() -> Vec<(usize, RawKind, String)> {
    let mut v: Vec<(usize, RawKind, String)> = SITES.with(|s| {
        s.borrow().as_ref().map(|m| m.iter().map(|((k, site), n)| (*n, *k, site.clone())).collect()).unwrap_or_default()
    });
    v.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| (a.1, &a.2).cmp(&(b.1, &b.2))));
    v
}

/// 位点剖面是否已开启（本线程）
pub fn sites_enabled() -> bool {
    SITES.with(|s| s.borrow().is_some())
}

/// 一个线程的账本（计数 + 位点剖面）
#[derive(Debug, Default)]
pub struct Tally {
    counts: [usize; 3],
    sites: BTreeMap<(RawKind, String), usize>,
}

/// 取走本线程账本并清零（位点剖面保持开启状态）
pub fn take_local() -> Tally {
    let counts = COUNTS.with(|c| c.replace([0; 3]));
    let sites = SITES.with(|s| s.borrow_mut().as_mut().map(std::mem::take).unwrap_or_default());
    Tally { counts, sites }
}

/// 把其它线程的账本并入本线程（位点只在本线程已开启剖面时并入）
pub fn absorb(t: Tally) {
    COUNTS.with(|c| {
        let mut v = c.get();
        for (a, b) in v.iter_mut().zip(t.counts) {
            *a += b;
        }
        c.set(v);
    });
    SITES.with(|s| {
        if let Some(m) = s.borrow_mut().as_mut() {
            for (k, n) in t.sites {
                *m.entry(k).or_default() += n;
            }
        }
    });
}

/// 清零计数与位点剖面（位点剖面保持开启状态）
pub fn reset() {
    COUNTS.with(|c| c.set([0; 3]));
    SITES.with(|s| {
        if let Some(m) = s.borrow_mut().as_mut() {
            m.clear();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Item, Stmt};

    #[test]
    fn constructors_count_and_record_sites() {
        reset();
        enable_sites();
        let e = Expr::raw("a");
        let _s = Stmt::raw("b;");
        let _s2 = Stmt::raw("c;");
        let _i = Item::raw("// d");
        let _clone = e.clone();
        assert_eq!((count(RawKind::Expr), count(RawKind::Stmt), count(RawKind::Item)), (1, 2, 1));
        let sites = sites();
        assert_eq!(sites.len(), 4, "{sites:?}");
        assert!(sites.iter().all(|(_, _, s)| s.contains("raw_audit.rs:")), "{sites:?}");
        reset();
        assert_eq!(count(RawKind::Stmt), 0);
        assert!(super::sites().is_empty());
    }

    #[test]
    fn worker_tally_merges_into_caller() {
        reset();
        enable_sites();
        let _a = Expr::raw("a");
        let worker = std::thread::spawn(|| {
            enable_sites();
            let _b = Expr::raw("b");
            let _c = Stmt::raw("c;");
            take_local()
        });
        let t = worker.join().unwrap();
        absorb(t);
        assert_eq!((count(RawKind::Expr), count(RawKind::Stmt)), (2, 1));
        assert_eq!(sites().iter().map(|(n, _, _)| n).sum::<usize>(), 3);
        let t = take_local();
        assert_eq!(t.counts, [2, 1, 0]);
        assert_eq!(count(RawKind::Expr), 0);
        assert!(sites_enabled());
    }
}
