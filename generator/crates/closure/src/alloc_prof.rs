//! 分配剖析：包装全局分配器的计数层（只读诊断，不影响分配行为与分析结果）。
//!
//! 定位 S0 / sb3 每 ~240 s 一次 +5~8 GB 的 RSS 尖峰（性能计划 §4.12 / §4.13）：
//! 累计字节入出常开（每次分配 / 释放各一条原子加，单线程引擎上开销可忽略），给出任意时刻的
//! 「活跃字节」与峰值；尺寸分桶（2 的幂）与分配次数计数由 [`ENABLED`] 门控（`--lambda-prof` 开启时置位，
//! 中途置位只影响分桶与次数的口径，字节入出恒一致）。读数经 `--lambda-prof` 的 census 行打印。
//!
//! RSS 与活跃字节之差即分配器层的滞留（mimalloc 段 / 碎片）；活跃字节的跳变再对照引擎
//! census（lambda_prof.rs）即可把尖峰归因到具体结构。

use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// 2 的幂分桶数（按请求字节数最高位分桶；64 位平台上够用）
const BUCKETS: usize = 64;

const ZERO: AtomicU64 = AtomicU64::new(0);

/// 分配剖析门控（`Engine::set_lambda_prof` 置位）：关时只保持字节入出与活跃计数
pub static ENABLED: AtomicBool = AtomicBool::new(false);

/// 全局计数器（引擎 census 直接读；与实际安装的 [`CountingAlloc`] 共用）
pub static COUNTERS: Counters = Counters::new();

pub struct Counters {
    /// 累计请求分配 / 释放字节（恒一致，差值即活跃字节）
    pub alloc_bytes: AtomicU64,
    pub free_bytes: AtomicU64,
    /// 分配 / 释放次数（门控）
    pub allocs: AtomicU64,
    pub frees: AtomicU64,
    /// 活跃字节的峰值（门控；分配路径上更新，单线程引擎上精确）
    pub peak_live: AtomicU64,
    /// 各尺寸桶的活跃字节（门控）
    pub live: [AtomicU64; BUCKETS],
}

impl Counters {
    pub const fn new() -> Self {
        Counters {
            alloc_bytes: ZERO,
            free_bytes: ZERO,
            allocs: ZERO,
            frees: ZERO,
            peak_live: ZERO,
            live: [ZERO; BUCKETS],
        }
    }
    /// 活跃字节（累计分配 − 累计释放）
    pub fn live_bytes(&self) -> u64 {
        self.alloc_bytes.load(Ordering::Relaxed).wrapping_sub(self.free_bytes.load(Ordering::Relaxed))
    }
}

/// 包装分配器：字节计数透传给内层。安装方式见 driver：`#[global_allocator] static GLOBAL: CountingAlloc<MiMalloc>`
pub struct CountingAlloc<A> {
    inner: A,
}

impl<A> CountingAlloc<A> {
    pub const fn new(inner: A) -> Self {
        CountingAlloc { inner }
    }
}

// SAFETY：内层分配器线程安全（mimalloc），本层只做原子计数
unsafe impl<A: Sync> Sync for CountingAlloc<A> {}

#[inline]
fn bucket(size: usize) -> usize {
    (usize::BITS - size.leading_zeros()) as usize
}

// SAFETY：逐方法转发内层分配器，计数不改指针与布局语义；内层 A 自证其安全性
unsafe impl<A: GlobalAlloc> GlobalAlloc for CountingAlloc<A> {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = self.inner.alloc(l);
        if !p.is_null() {
            self.count(l.size() as u64, 0);
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        self.count(0, l.size() as u64);
        self.inner.dealloc(p, l)
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        let p = self.inner.alloc_zeroed(l);
        if !p.is_null() {
            self.count(l.size() as u64, 0);
        }
        p
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new_size: usize) -> *mut u8 {
        let q = self.inner.realloc(p, l, new_size);
        if !q.is_null() {
            self.count(new_size as u64, l.size() as u64);
        }
        q
    }
}

impl<A> CountingAlloc<A> {
    /// 记一笔：add 字节进 / sub 字节出（恒开字节入出；其余门控）
    #[inline]
    fn count(&self, add: u64, sub: u64) {
        let c = &COUNTERS;
        if add != 0 {
            c.alloc_bytes.fetch_add(add, Ordering::Relaxed);
        }
        if sub != 0 {
            c.free_bytes.fetch_add(sub, Ordering::Relaxed);
        }
        if !ENABLED.load(Ordering::Relaxed) {
            return;
        }
        if add != 0 {
            c.allocs.fetch_add(1, Ordering::Relaxed);
            c.live[bucket(add as usize)].fetch_add(add, Ordering::Relaxed);
            let l = c.alloc_bytes.load(Ordering::Relaxed).wrapping_sub(c.free_bytes.load(Ordering::Relaxed));
            let mut pk = c.peak_live.load(Ordering::Relaxed);
            while l > pk {
                match c.peak_live.compare_exchange_weak(pk, l, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => pk = x,
                }
            }
        }
        if sub != 0 {
            c.frees.fetch_add(1, Ordering::Relaxed);
            c.live[bucket(sub as usize)].fetch_sub(sub, Ordering::Relaxed);
        }
    }
}
