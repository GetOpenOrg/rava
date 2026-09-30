//! 保序并行映射：输入构建中逐类独立步骤（方法体规范化）的并行执行器。
//!
//! 工作线程按原子游标领取下标，结果按下标归位，输出顺序与串行映射逐项一致。

use std::sync::atomic::{AtomicUsize, Ordering};

/// 对 `items` 逐项求 `f`，结果与 `items` 同序；`jobs` 为 0 取可用核数，`<= 1` 时串行
pub(crate) fn par_map<T: Sync, R: Send>(jobs: usize, items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let n = items.len();
    let jobs = if jobs == 0 { std::thread::available_parallelism().map_or(1, |j| j.get()) } else { jobs }.min(n);
    if jobs <= 1 {
        return items.iter().map(f).collect();
    }
    let next = AtomicUsize::new(0);
    let mut slots: Vec<Option<R>> = (0..n).map(|_| None).collect();
    std::thread::scope(|s| {
        let work = || {
            let mut out = Vec::new();
            loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= n {
                    return out;
                }
                out.push((i, f(&items[i])));
            }
        };
        let handles: Vec<_> = (0..jobs).map(|_| s.spawn(work)).collect();
        for h in handles {
            match h.join() {
                Ok(done) => {
                    for (i, r) in done {
                        slots[i] = Some(r);
                    }
                }
                Err(p) => std::panic::resume_unwind(p),
            }
        }
    });
    slots.into_iter().map(|r| r.expect("并行映射结果缺位")).collect()
}
