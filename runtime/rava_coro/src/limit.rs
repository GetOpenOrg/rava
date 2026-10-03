//! 软件栈界检查：Java `StackOverflowError` 的判定（§21.8.2「溢出语义」）。
//!
//! 每个执行流（协程或平台线程）有一个栈界 `limit`：栈指针低于它即「栈耗尽」。协程的栈界为
//! 可用区底 + `SHADOW` + `YELLOW`，平台线程由 [`init_platform_thread`] 按其栈区间设定。
//!
//! - `SHADOW`：检查点之间允许的最大栈用量——一个 Java 方法帧加上它调用的、不经检查点的原生 / 运行时代码；
//! - `YELLOW`：判定耗尽后构造并抛出 `StackOverflowError` 所用的余量，由 [`YellowZone`] 临时放开。
//!
//! 检查本身不内联（[`stack_exhausted`]）：协程可能在两次检查之间换到另一载体，内联会让线程局部地址被
//! 缓存跨越挂起点（§21.8.3）。

/// 检查点之间允许的最大栈用量
pub const SHADOW: usize = 64 << 10;
/// 构造 `StackOverflowError` 的余量
pub const YELLOW: usize = 64 << 10;

/// 栈底 `lo` 对应的软件栈界
pub const fn limit_for(lo: usize) -> usize {
    lo + SHADOW + YELLOW
}

/// 当前栈指针（近似：本函数帧内的地址）
#[inline(always)]
fn stack_pointer() -> usize {
    let probe = 0u8;
    std::hint::black_box(&probe) as *const u8 as usize
}

/// 当前执行流的栈是否已耗尽（栈指针低于栈界）。栈界为 0（未设定）时恒为 false。
#[inline(never)]
pub fn stack_exhausted() -> bool {
    let limit = crate::guard::current().limit;
    limit != 0 && stack_pointer() < limit
}

/// 当前执行流的软件栈界（0 = 未设定）
#[inline(never)]
pub fn stack_limit() -> usize {
    crate::guard::current().limit
}

/// 临时放开 `YELLOW` 余量以构造 `StackOverflowError`；drop 时恢复。
/// 期间执行流可能挂起并换载体：恢复时重新读写当前线程的栈界，不缓存线程局部地址。
pub struct YellowZone {
    saved: usize,
}

impl YellowZone {
    /// 进入余量区（栈界未设定时无操作）
    #[inline(never)]
    pub fn enter() -> YellowZone {
        let saved = crate::guard::current().limit;
        if saved != 0 {
            crate::guard::set_limit(saved - YELLOW);
        }
        YellowZone { saved }
    }
}

impl Drop for YellowZone {
    #[inline(never)]
    fn drop(&mut self) {
        crate::guard::set_limit(self.saved);
    }
}

/// 为当前平台线程按其栈区间设定软件栈界（线程入口处调用一次；在协程内调用无效果）。
/// 返回设定的栈界，取不到栈区间时返回 0（不检查）。
pub fn init_platform_thread() -> usize {
    let cur = crate::guard::current();
    if cur.guard.0 != 0 {
        return cur.limit;
    }
    let limit = match platform_stack_low() {
        Some(lo) => limit_for(lo),
        None => 0,
    };
    // 栈指针已低于栈界（栈过小）时不检查，避免一进入就报耗尽
    let limit = if limit != 0 && stack_pointer() > limit { limit } else { 0 };
    crate::guard::set_limit(limit);
    limit
}

#[cfg(target_os = "macos")]
fn platform_stack_low() -> Option<usize> {
    // SAFETY: 查询本线程栈区间
    unsafe {
        let th = libc::pthread_self();
        let top = libc::pthread_get_stackaddr_np(th) as usize;
        let size = libc::pthread_get_stacksize_np(th);
        (top != 0 && size != 0).then(|| top - size)
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn platform_stack_low() -> Option<usize> {
    // SAFETY: 查询本线程栈区间；attr 用后销毁
    unsafe {
        let mut attr: libc::pthread_attr_t = std::mem::zeroed();
        if libc::pthread_getattr_np(libc::pthread_self(), &mut attr) != 0 {
            return None;
        }
        let mut addr: *mut libc::c_void = std::ptr::null_mut();
        let mut size: libc::size_t = 0;
        let r = libc::pthread_attr_getstack(&attr, &mut addr, &mut size);
        libc::pthread_attr_destroy(&mut attr);
        (r == 0 && !addr.is_null()).then_some(addr as usize)
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
fn platform_stack_low() -> Option<usize> {
    None
}
