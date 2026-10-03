//! 协程栈：大块 slab 预留多栈，映射数与存活协程数脱钩（§21.8.2「栈分配」）。
//!
//! - **slab**：一次 `mmap` 预留一块含多个栈槽的区间（`MAP_NORESERVE`，物理页按需提交），新块槽数取现有总槽数
//!   （容量倍增），夹在 `FIRST_CHUNK_SLOTS` ～ `MAX_CHUNK_SLOTS` 之间。10⁶ 个存活栈约占 1000 个映射，
//!   与 `vm.max_map_count`（缺省 65530）无关；
//! - **槽布局**：`[slot, slot + page)` 为 guard 页，`[slot + page, slot + stride)` 为可用栈；
//! - **硬件 guard 只在不增加映射时启用**：Linux ≥ 6.13 以 `MADV_GUARD_INSTALL` 安装 guard 标记（不拆分映射，
//!   首块建立时探测一次，旧内核 `EINVAL` 即停用）；macOS 逐槽 `mprotect`（无小额映射上限）。其余情况不设硬件
//!   guard，溢出由软件栈界检查（[`crate::stack_exhausted`]，Java `StackOverflowError` 语义）拦截；
//! - **取栈集中**：总是从基址最低、仍有空槽的块取（块内后进先出），存活栈集中在少数块里，突发过后其余块能整块变空；
//! - **归还**：归还的槽至多 `pool_limit()` 个为「热槽」，只交还栈顶 `KEEP` 以下的页；其余为「冷槽」，整槽交还
//!   （Linux `MADV_DONTNEED`；macOS `MADV_FREE_REUSABLE`，复用时 `MADV_FREE_REUSE` 回收计账）。`madvise` 不拆分映射；
//! - **空块整块释放**：块内存活数归零时，若已有另一个空块则 `munmap` 本块——交还地址空间与页表（`madvise` 不回收
//!   页表，10⁵ 栈的页表约数十 MiB），只留一个空块作缓冲，避免单协程反复建销时反复映射。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

/// 缺省栈保留大小（不含 guard 页）
pub const DEFAULT_RESERVE: usize = 1 << 20;
/// 热槽保留提交的栈顶部分
pub const KEEP: usize = 16 << 10;
/// 缺省热槽上限：每槽至多常驻 `KEEP`，512 槽 = 8 MiB 常驻预算（固定预算，不随核数增长）
pub const DEFAULT_POOL_BLOCKS: usize = 512;
/// slab 块的最小槽数
pub const FIRST_CHUNK_SLOTS: usize = 16;
/// slab 块的最大槽数（1 MiB 栈时每块约 1 GiB 地址空间）
pub const MAX_CHUNK_SLOTS: usize = 1024;

static RESERVE: AtomicUsize = AtomicUsize::new(DEFAULT_RESERVE);
/// 热槽上限；`usize::MAX` = 未设定（取 `DEFAULT_POOL_BLOCKS`）
static POOL_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
/// 当前存活（已取出未归还）的栈数
static LIVE: AtomicUsize = AtomicUsize::new(0);
/// 是否启用了硬件 guard（首块建立时确定）
static HW_GUARD: AtomicBool = AtomicBool::new(false);

static SLAB: Mutex<Slab> = Mutex::new(Slab {
    stride: 0,
    probed: false,
    chunks: BTreeMap::new(),
    avail: BTreeSet::new(),
    warm: 0,
    empty: 0,
    total_slots: 0,
});

/// 一块 slab
struct Chunk {
    slots: usize,
    /// 存活栈数
    used: usize,
    /// 空槽：`槽号 << 1 | 热`，后进先出
    free: Vec<u32>,
    /// 空槽中的热槽数
    warm: usize,
}

struct Slab {
    /// 槽跨度（guard 页 + 保留区）；0 = 尚未建块，此后固定
    stride: usize,
    /// 是否已探测硬件 guard
    probed: bool,
    /// 基址 → 块
    chunks: BTreeMap<usize, Chunk>,
    /// 有空槽的块基址
    avail: BTreeSet<usize>,
    /// 热槽总数
    warm: usize,
    /// 存活数为 0 的块数
    empty: usize,
    /// 全部块的槽数之和
    total_slots: usize,
}

/// 页大小
pub fn page_size() -> usize {
    static PAGE: OnceLock<usize> = OnceLock::new();
    // SAFETY: sysconf 只读查询
    *PAGE.get_or_init(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize })
}

/// 最小栈保留大小：软件栈界之上至少还有同样大小的可用区
pub const MIN_RESERVE: usize = 2 * (crate::SHADOW + crate::YELLOW);

/// 设置栈保留大小（不小于 `MIN_RESERVE`，向上取整到页）。槽跨度在首次建栈时固定，此后调用返回 false 且不生效
pub fn set_reserve(bytes: usize) -> bool {
    let p = page_size();
    let slab = SLAB.lock().unwrap_or_else(|e| e.into_inner());
    if slab.stride != 0 {
        return false;
    }
    RESERVE.store(bytes.max(MIN_RESERVE).div_ceil(p) * p, Ordering::Relaxed);
    true
}

/// 栈保留大小
pub fn reserve() -> usize {
    RESERVE.load(Ordering::Relaxed)
}

/// 设置热槽上限（0 = 归还即整槽交还）；超出部分立即转冷
pub fn set_pool_limit(n: usize) {
    POOL_LIMIT.store(n, Ordering::Relaxed);
    let mut slab = SLAB.lock().unwrap_or_else(|e| e.into_inner());
    let stride = slab.stride;
    let mut excess = slab.warm.saturating_sub(n);
    for (&base, c) in slab.chunks.iter_mut() {
        for e in c.free.iter_mut() {
            if excess == 0 {
                break;
            }
            if *e & 1 == 1 {
                *e &= !1;
                c.warm -= 1;
                excess -= 1;
                release(base + (*e >> 1) as usize * stride, stride, 0);
            }
        }
    }
    slab.warm = slab.chunks.values().map(|c| c.warm).sum();
}

/// 热槽上限：未设置时为 `DEFAULT_POOL_BLOCKS`
pub fn pool_limit() -> usize {
    match POOL_LIMIT.load(Ordering::Relaxed) {
        usize::MAX => DEFAULT_POOL_BLOCKS,
        n => n,
    }
}

/// 存活栈数
pub fn live_stacks() -> usize {
    LIVE.load(Ordering::Relaxed)
}

/// 热槽数
pub fn pooled_stacks() -> usize {
    SLAB.lock().unwrap_or_else(|e| e.into_inner()).warm
}

/// 已映射的 slab 块数（即协程栈占用的映射数，不随存活栈数逐个增长）
pub fn mapped_chunks() -> usize {
    SLAB.lock().unwrap_or_else(|e| e.into_inner()).chunks.len()
}

/// 是否启用了硬件 guard（首次建栈后有意义）
pub fn hardware_guard() -> bool {
    HW_GUARD.load(Ordering::Relaxed)
}

/// 交还槽 `[slot + page, slot + stride - keep)` 的已提交页
fn release(slot: usize, stride: usize, keep: usize) {
    let lo = slot + page_size();
    let hi = (slot + stride - keep).max(lo);
    if hi > lo {
        // SAFETY: 区间位于本槽可用区内，栈上已无执行流
        unsafe { libc::madvise(lo as *mut libc::c_void, hi - lo, ADVICE_RELEASE) };
    }
}

/// 取出复用：macOS 回收 REUSABLE 计账（`keep` 以上从未交还）
#[allow(unused_variables)]
fn reclaim(slot: usize, stride: usize, keep: usize) {
    #[cfg(target_os = "macos")]
    {
        let lo = slot + page_size();
        let hi = (slot + stride - keep).max(lo);
        if hi > lo {
            // SAFETY: 同 release
            unsafe { libc::madvise(lo as *mut libc::c_void, hi - lo, libc::MADV_FREE_REUSE) };
        }
    }
}

#[cfg(target_os = "macos")]
const ADVICE_RELEASE: libc::c_int = libc::MADV_FREE_REUSABLE;
#[cfg(not(target_os = "macos"))]
const ADVICE_RELEASE: libc::c_int = libc::MADV_DONTNEED;

#[cfg(any(target_os = "linux", target_os = "android"))]
fn map_noreserve() -> libc::c_int {
    libc::MAP_NORESERVE
}
#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn map_noreserve() -> libc::c_int {
    0
}

/// Linux 6.13 起的 `MADV_GUARD_INSTALL`（libc 尚未导出）
#[cfg(any(target_os = "linux", target_os = "android"))]
const MADV_GUARD_INSTALL: libc::c_int = 102;

/// 在一块新映射的每个槽底装 guard。首块时探测，不支持则此后不再尝试
fn install_guards(base: usize, slots: usize, stride: usize, first: bool) -> Result<(), StackError> {
    let page = page_size();
    #[cfg(target_os = "macos")]
    {
        let _ = first;
        for i in 0..slots {
            // SAFETY: 槽底一页位于刚建立的映射内
            if unsafe { libc::mprotect((base + i * stride) as *mut libc::c_void, page, libc::PROT_NONE) } != 0 {
                return Err(StackError::new("mprotect"));
            }
        }
        HW_GUARD.store(true, Ordering::Relaxed);
        Ok(())
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        if !first && !HW_GUARD.load(Ordering::Relaxed) {
            return Ok(());
        }
        for i in 0..slots {
            // SAFETY: 同上；guard 标记不拆分映射
            if unsafe { libc::madvise((base + i * stride) as *mut libc::c_void, page, MADV_GUARD_INSTALL) } != 0 {
                if first && i == 0 {
                    // 内核不支持（EINVAL）：停用硬件 guard，靠软件栈界检查
                    return Ok(());
                }
                return Err(StackError::new("madvise(MADV_GUARD_INSTALL)"));
            }
        }
        HW_GUARD.store(true, Ordering::Relaxed);
        Ok(())
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
    {
        let _ = (base, slots, stride, first, page);
        Ok(())
    }
}

impl Slab {
    /// 映射一块新 slab，返回基址
    fn grow(&mut self) -> Result<usize, StackError> {
        if self.stride == 0 {
            self.stride = reserve() + page_size();
        }
        let slots = self.total_slots.clamp(FIRST_CHUNK_SLOTS, MAX_CHUNK_SLOTS);
        let len = slots * self.stride;
        // SAFETY: 匿名私有映射，地址由内核选择；MAP_NORESERVE 只保留地址空间
        let base = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANON | map_noreserve(),
                -1,
                0,
            )
        };
        if base == libc::MAP_FAILED {
            return Err(StackError::new("mmap"));
        }
        let base = base as usize;
        let first = !self.probed;
        self.probed = true;
        if let Err(e) = install_guards(base, slots, self.stride, first) {
            // SAFETY: 释放刚建立的整块
            unsafe { libc::munmap(base as *mut libc::c_void, len) };
            return Err(e);
        }
        // 低槽号后出：块内自栈顶一端起用
        let free = (0..slots as u32).map(|i| i << 1).collect();
        self.chunks.insert(base, Chunk { slots, used: 0, free, warm: 0 });
        self.avail.insert(base);
        self.empty += 1;
        self.total_slots += slots;
        Ok(base)
    }
}

/// 建栈失败（地址空间耗尽等）。调用方按 JDK 平台线程耗尽的形态抛 `OutOfMemoryError`
#[derive(Debug)]
pub struct StackError {
    /// 失败的系统调用
    pub call: &'static str,
    /// errno
    pub errno: i32,
    /// Linux `vm.max_map_count`（其它平台 None）
    pub max_map_count: Option<u64>,
    /// 失败时存活的栈数
    pub live: usize,
}

impl StackError {
    fn new(call: &'static str) -> StackError {
        StackError {
            call,
            errno: std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
            max_map_count: max_map_count(),
            live: live_stacks(),
        }
    }
}

impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} 失败（errno {}，存活协程栈 {}", self.call, self.errno, self.live)?;
        if let Some(m) = self.max_map_count {
            write!(f, "，vm.max_map_count = {m}")?;
        }
        write!(f, "）")
    }
}

impl std::error::Error for StackError {}

fn max_map_count() -> Option<u64> {
    if cfg!(any(target_os = "linux", target_os = "android")) {
        std::fs::read_to_string("/proc/sys/vm/max_map_count").ok()?.trim().parse().ok()
    } else {
        None
    }
}

/// 一个协程栈槽。`Drop` 时归还（热槽或冷槽）；调用方保证此时栈上已无执行流。
pub struct Stack {
    slot: usize,
    chunk: usize,
    stride: usize,
}

impl Stack {
    /// 取一个栈槽：基址最低、仍有空槽的块，块内后进先出；都满时扩一块 slab
    pub fn new() -> Result<Stack, StackError> {
        crate::guard::install();
        let (slot, chunk, stride, warm) = {
            let mut guard = SLAB.lock().unwrap_or_else(|e| e.into_inner());
            let slab = &mut *guard;
            let base = match slab.avail.first() {
                Some(&b) => b,
                None => slab.grow()?,
            };
            let c = slab.chunks.get_mut(&base).expect("slab");
            let e = c.free.pop().expect("slab 空槽");
            let warm = e & 1 == 1;
            if warm {
                c.warm -= 1;
                slab.warm -= 1;
            }
            if c.used == 0 {
                slab.empty -= 1;
            }
            c.used += 1;
            if c.free.is_empty() {
                slab.avail.remove(&base);
            }
            (base + (e >> 1) as usize * slab.stride, base, slab.stride, warm)
        };
        reclaim(slot, stride, if warm { KEEP } else { 0 });
        LIVE.fetch_add(1, Ordering::Relaxed);
        Ok(Stack { slot, chunk, stride })
    }

    /// 栈顶（最高地址，独占端，16 字节对齐）
    pub fn top(&self) -> usize {
        self.slot + self.stride
    }

    /// 可用区最低地址（guard 页之上）
    pub fn limit(&self) -> usize {
        self.slot + page_size()
    }

    /// guard 页区间 `[lo, hi)`；未启用硬件 guard 时为 (0, 0)
    pub fn guard(&self) -> (usize, usize) {
        if hardware_guard() {
            (self.slot, self.slot + page_size())
        } else {
            (0, 0)
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        LIVE.fetch_sub(1, Ordering::Relaxed);
        // 栈顶 KEEP 以下在锁外交还；是否保留 KEEP 视热槽余额
        release(self.slot, self.stride, KEEP);
        let idx = ((self.slot - self.chunk) / self.stride) as u32;
        let limit = pool_limit();
        let mut guard = SLAB.lock().unwrap_or_else(|e| e.into_inner());
        let slab = &mut *guard;
        let warm = slab.warm < limit;
        if !warm {
            let top = self.slot + self.stride;
            // SAFETY: 栈顶 KEEP 位于本槽可用区内
            unsafe { libc::madvise((top - KEEP) as *mut libc::c_void, KEEP, ADVICE_RELEASE) };
        }
        let c = slab.chunks.get_mut(&self.chunk).expect("slab");
        c.free.push(idx << 1 | warm as u32);
        if warm {
            c.warm += 1;
            slab.warm += 1;
        }
        c.used -= 1;
        slab.avail.insert(self.chunk);
        if c.used > 0 {
            return;
        }
        slab.empty += 1;
        if slab.empty < 2 {
            return;
        }
        // 已有另一个空块：本块整块释放
        let c = slab.chunks.remove(&self.chunk).expect("slab");
        slab.avail.remove(&self.chunk);
        slab.warm -= c.warm;
        slab.empty -= 1;
        slab.total_slots -= c.slots;
        drop(guard);
        // SAFETY: 块内已无存活栈，此后不再访问
        unsafe { libc::munmap(self.chunk as *mut libc::c_void, c.slots * self.stride) };
    }
}
