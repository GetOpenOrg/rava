//! 软件栈界检查：Java `StackOverflowError` 的判定（§21.8.2「溢出语义」）。
//!
//! 每个执行流（协程或平台线程）有一个栈界 `limit`：栈指针低于它即「栈耗尽」。协程的栈界为
//! 可用区底 + `SHADOW` + `YELLOW`，平台线程由 [`init_platform_thread`] 按其栈区间设定。
//!
//! - `SHADOW`：检查点之间允许的最大栈用量——一个 Java 方法帧加上它调用的、不经检查点的原生 / 运行时代码；
//! - `YELLOW`：判定耗尽后构造并抛出 `StackOverflowError` 所用的余量，由 [`YellowZone`] 临时放开；
//!   余量区内再次耗尽（构造异常本身溢出）不再放开，由调用方按致命错误处理。
//!
//! 栈界的最低位标记「余量区已放开」（栈界按页对齐，最低位空闲）。
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

/// 余量区已放开标记（栈界最低位）
const IN_YELLOW: usize = 1;

/// 当前栈指针（近似：本函数帧内的地址）
#[inline(always)]
fn stack_pointer() -> usize {
    let probe = 0u8;
    std::hint::black_box(&probe) as *const u8 as usize
}

/// 当前执行流的栈是否已耗尽（栈指针低于栈界）。栈界为 0（未设定）时恒为 false。
#[inline(never)]
pub fn stack_exhausted() -> bool {
    let limit = crate::guard::current().limit & !IN_YELLOW;
    limit != 0 && stack_pointer() < limit
}

/// 当前执行流的软件栈界（0 = 未设定；余量区放开期间为放开后的值）
#[inline(never)]
pub fn stack_limit() -> usize {
    crate::guard::current().limit & !IN_YELLOW
}

/// 临时放开 `YELLOW` 余量以构造 `StackOverflowError`；drop 时恢复。
/// 期间执行流可能挂起并换载体：恢复时重新读写当前线程的栈界，不缓存线程局部地址。
pub struct YellowZone {
    saved: usize,
}

impl YellowZone {
    /// 进入余量区；已在余量区内（构造异常途中再次耗尽）返回 None。栈界未设定时无操作
    #[inline(never)]
    pub fn enter() -> Option<YellowZone> {
        let saved = crate::guard::current().limit;
        if saved & IN_YELLOW != 0 {
            return None;
        }
        if saved != 0 {
            crate::guard::set_limit((saved - YELLOW) | IN_YELLOW);
        }
        Some(YellowZone { saved })
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
    // 平台线程栈大（主线程 8 MiB、Java 线程 256 MiB）：检查点间余量取栈的 1/16 与 SHADOW 的较大者，
    // 给未优化构建的大帧留足空间
    let limit = match platform_stack() {
        Some((lo, size)) => lo + SHADOW.max(size / 16) + YELLOW,
        None => 0,
    };
    // 栈指针已低于栈界（栈过小）时不检查，避免一进入就报耗尽
    let limit = if limit != 0 && stack_pointer() > limit { limit } else { 0 };
    crate::guard::set_limit(limit);
    limit
}

/// 本线程栈区间（低端, 大小）
#[cfg(target_os = "macos")]
fn platform_stack() -> Option<(usize, usize)> {
    // SAFETY: 查询本线程栈区间
    unsafe {
        let th = libc::pthread_self();
        let top = libc::pthread_get_stackaddr_np(th) as usize;
        let size = libc::pthread_get_stacksize_np(th);
        (top != 0 && size != 0).then(|| (top - size, size))
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn platform_stack() -> Option<(usize, usize)> {
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
        (r == 0 && !addr.is_null()).then_some((addr as usize, size))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "android")))]
fn platform_stack() -> Option<(usize, usize)> {
    None
}
