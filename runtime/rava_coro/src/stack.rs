//! 协程栈：`mmap` 保留、底端一页 `PROT_NONE` guard page、按需提交；执行完毕的栈归池复用。
//!
//! - 每块栈 2 个映射（guard + 可写）；Linux 下 10⁶ 块需 `vm.max_map_count ≥ 2.1 × 10⁶`，
//!   建栈失败的错误信息附带首次建栈时读到的 `/proc/sys/vm/max_map_count`；
//! - 归池前把栈顶 `KEEP` 以下的已提交页交还内核（Linux `MADV_DONTNEED`；macOS `MADV_FREE_REUSABLE`，
//!   即 `MADV_FREE` 的计账版本——立即从驻留集扣除，取出复用时以 `MADV_FREE_REUSE` 回收计账）；
//! - 池上限缺省 `DEFAULT_POOL_BLOCKS` 块（池内常驻提交量 ≤ 8 MiB，与核数无关），超出即 `munmap`。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

/// 缺省栈保留大小（不含 guard page）
pub const DEFAULT_RESERVE: usize = 1 << 20;
/// 归池时保留提交的栈顶部分
pub const KEEP: usize = 16 << 10;
/// 缺省池上限：池内每块至多常驻 `KEEP`，512 块 = 8 MiB 常驻预算。按固定预算而非核数定：
/// 多核机上「核数 × 常数」会让空闲后的常驻量随核数线性增长；池只用来摊薄建 / 销栈的系统调用，
/// 512 块足够 64 核下每核 8 块的周转。
pub const DEFAULT_POOL_BLOCKS: usize = 512;

static RESERVE: AtomicUsize = AtomicUsize::new(DEFAULT_RESERVE);
/// 池上限；`usize::MAX` = 未设定（取 `DEFAULT_POOL_BLOCKS`）
static POOL_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
static POOL: Mutex<Vec<RawStack>> = Mutex::new(Vec::new());
/// 当前存活（未归池、未释放）的栈块数
static LIVE: AtomicUsize = AtomicUsize::new(0);

/// 页大小
pub fn page_size() -> usize {
    static PAGE: OnceLock<usize> = OnceLock::new();
    // SAFETY: sysconf 只读查询
    *PAGE.get_or_init(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize })
}

/// 设置此后新建栈的保留大小（向上取整到页；已建与池中的栈不变，池中尺寸不符的栈不再复用）
pub fn set_reserve(bytes: usize) {
    let p = page_size();
    RESERVE.store(bytes.max(4 * p).div_ceil(p) * p, Ordering::Relaxed);
}

/// 当前栈保留大小
pub fn reserve() -> usize {
    RESERVE.load(Ordering::Relaxed)
}

/// 设置栈池上限（块数；0 = 不池化，用完即 munmap）
pub fn set_pool_limit(n: usize) {
    POOL_LIMIT.store(n, Ordering::Relaxed);
    let mut pool = POOL.lock().unwrap_or_else(|e| e.into_inner());
    while pool.len() > n {
        if let Some(s) = pool.pop() {
            s.unmap();
        }
    }
}

/// 当前栈池上限：未设置时为 `DEFAULT_POOL_BLOCKS`
pub fn pool_limit() -> usize {
    match POOL_LIMIT.load(Ordering::Relaxed) {
        usize::MAX => DEFAULT_POOL_BLOCKS,
        n => n,
    }
}

/// 存活栈块数（不含池中）
pub fn live_stacks() -> usize {
    LIVE.load(Ordering::Relaxed)
}

/// 池中栈块数
pub fn pooled_stacks() -> usize {
    POOL.lock().unwrap_or_else(|e| e.into_inner()).len()
}

/// 一块映射：`[base, base + page)` 为 guard，`[base + page, base + len)` 可写
struct RawStack {
    base: usize,
    len: usize,
}

impl RawStack {
    fn map(reserve: usize) -> Result<RawStack, StackError> {
        let page = page_size();
        let len = reserve + page;
        // SAFETY: 匿名私有映射，地址由内核选择；MAP_NORESERVE 只保留地址空间，物理页按需提交
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
        // SAFETY: base 是刚映射的区间起点，长度一页
        if unsafe { libc::mprotect(base, page, libc::PROT_NONE) } != 0 {
            let err = StackError::new("mprotect");
            // SAFETY: 释放刚建立的整段映射
            unsafe { libc::munmap(base, len) };
            return Err(err);
        }
        Ok(RawStack { base: base as usize, len })
    }

    fn unmap(self) {
        // SAFETY: 整段映射由 map 建立，此后不再访问
        unsafe { libc::munmap(self.base as *mut libc::c_void, self.len) };
    }

    /// 栈顶 KEEP 以下的已提交页交还内核
    fn shrink(&self) {
        let page = page_size();
        let lo = self.base + page;
        let hi = (self.base + self.len - KEEP.div_ceil(page) * page).max(lo);
        if hi > lo {
            // SAFETY: [lo, hi) 位于本映射的可写区内，栈已不在任何线程上运行
            unsafe { libc::madvise(lo as *mut libc::c_void, hi - lo, ADVICE_RELEASE) };
        }
    }

    /// 取出复用：macOS 回收 REUSABLE 计账
    fn reuse(&self) {
        #[cfg(target_os = "macos")]
        {
            let page = page_size();
            let lo = self.base + page;
            let hi = (self.base + self.len - KEEP.div_ceil(page) * page).max(lo);
            if hi > lo {
                // SAFETY: 同 shrink
                unsafe { libc::madvise(lo as *mut libc::c_void, hi - lo, libc::MADV_FREE_REUSE) };
            }
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

/// 建栈失败（地址空间 / 映射数耗尽等）。调用方按 JDK 平台线程耗尽的形态抛 `OutOfMemoryError`
#[derive(Debug)]
pub struct StackError {
    /// 失败的系统调用
    pub call: &'static str,
    /// errno
    pub errno: i32,
    /// Linux `vm.max_map_count`（首次建栈时读取；其它平台 None）
    pub max_map_count: Option<u64>,
    /// 失败时存活的栈块数
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
            write!(f, "，vm.max_map_count = {m}，每栈占 2 个映射")?;
        }
        write!(f, "）")
    }
}

impl std::error::Error for StackError {}

/// 首次建栈时读取的 `vm.max_map_count`
fn max_map_count() -> Option<u64> {
    static V: OnceLock<Option<u64>> = OnceLock::new();
    *V.get_or_init(|| {
        if cfg!(any(target_os = "linux", target_os = "android")) {
            std::fs::read_to_string("/proc/sys/vm/max_map_count").ok()?.trim().parse().ok()
        } else {
            None
        }
    })
}

/// 一块协程栈。`Drop` 时归池（池满则释放映射）；调用方保证此时栈上已无执行流。
pub struct Stack {
    raw: Option<RawStack>,
}

impl Stack {
    /// 取一块栈：池中有同尺寸的先复用，否则新建
    pub fn new() -> Result<Stack, StackError> {
        let _ = max_map_count();
        crate::guard::install();
        let reserve = reserve();
        let page = page_size();
        let pooled = {
            let mut pool = POOL.lock().unwrap_or_else(|e| e.into_inner());
            match pool.iter().rposition(|s| s.len == reserve + page) {
                Some(i) => Some(pool.swap_remove(i)),
                None => None,
            }
        };
        let raw = match pooled {
            Some(s) => {
                s.reuse();
                s
            }
            None => RawStack::map(reserve)?,
        };
        LIVE.fetch_add(1, Ordering::Relaxed);
        Ok(Stack { raw: Some(raw) })
    }

    /// 栈顶（最高地址，独占端，16 字节对齐）
    pub fn top(&self) -> usize {
        let r = self.raw.as_ref().expect("stack");
        r.base + r.len
    }

    /// 可用区最低地址（guard page 之上）
    pub fn limit(&self) -> usize {
        self.raw.as_ref().expect("stack").base + page_size()
    }

    /// guard page 区间 `[lo, hi)`
    pub fn guard(&self) -> (usize, usize) {
        let b = self.raw.as_ref().expect("stack").base;
        (b, b + page_size())
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        let Some(raw) = self.raw.take() else { return };
        LIVE.fetch_sub(1, Ordering::Relaxed);
        if raw.len != reserve() + page_size() {
            raw.unmap();
            return;
        }
        // madvise 在锁外做：池满时多做一次无害的回收
        raw.shrink();
        let limit = pool_limit();
        let mut pool = POOL.lock().unwrap_or_else(|e| e.into_inner());
        if pool.len() < limit {
            pool.push(raw);
        } else {
            drop(pool);
            raw.unmap();
        }
    }
}
