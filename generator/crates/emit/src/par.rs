//! 保序并行映射：逐类发射的并行执行器。
//!
//! 工作线程按原子游标领取下标（负载自平衡：类体耗时差异两三个数量级），结果按下标归位，
//! 输出顺序与串行映射逐项一致。工作线程栈与主线程同量级（方法体生成有深递归）。

use std::sync::atomic::{AtomicUsize, Ordering};

/// 工作线程栈大小：不小于主线程（macOS 8 MiB），留一倍余量
const WORKER_STACK: usize = 16 << 20;

/// 并行度：0 = 可用核数
pub fn resolve_jobs(jobs: usize) -> usize {
    if jobs > 0 {
        return jobs;
    }
    std::thread::available_parallelism().map_or(1, |n| n.get())
}

/// 对 `items` 逐项求 `f`，结果与 `items` 同序；`jobs <= 1` 时在当前线程串行求值
pub fn par_map<T: Sync, R: Send>(jobs: usize, items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let n = items.len();
    let jobs = jobs.min(n);
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
        let handles: Vec<_> = (0..jobs)
            .map(|_| {
                std::thread::Builder::new()
                    .stack_size(WORKER_STACK)
                    .spawn_scoped(s, work)
                    .expect("创建发射工作线程")
            })
            .collect();
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
    slots.into_iter().map(|r| r.expect("每个下标恰好领取一次")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_order() {
        let items: Vec<usize> = (0..1000).collect();
        let got = par_map(4, &items, |x| x * 2);
        assert_eq!(got, items.iter().map(|x| x * 2).collect::<Vec<_>>());
        assert_eq!(par_map(1, &items, |x| x + 1)[999], 1000);
        assert!(par_map(4, &Vec::<usize>::new(), |x| *x).is_empty());
    }
}
