//! 执行流的栈界：软件栈界（`LIMIT`，Java `StackOverflowError` 的判定依据）与硬件 guard 区间，
//! 以及载体线程上的 `SIGSEGV` / `SIGBUS` 处理器（运行在 `sigaltstack` 上）。
//!
//! 两者都随执行流走：[`crate::switch`] 把当前值存入被挂起的 [`crate::Context`]、换上被恢复者的值。
//!
//! 故障地址落在当前线程正在运行的协程栈的 guard page 内时，输出与 Rust 平台线程栈溢出同形的
//! 「thread '…' (tid) has overflowed its stack」并 abort；其余故障交还先前的处理器（通常是 std 的
//! 平台线程栈溢出处理器）。当前协程的 guard 区间由 [`crate::switch`] 在切换时写入线程局部槽；
//! 处理器只读 const 初始化、无析构的线程局部，满足异步信号安全。

use std::cell::{Cell, RefCell};
use std::sync::Once;

/// 一个执行流的栈界
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Bounds {
    /// 硬件 guard 区间 `[lo, hi)`；(0, 0) = 无（平台线程自己的栈，或协程栈未启用硬件 guard）
    pub guard: (usize, usize),
    /// 软件栈界：栈指针低于此值即判定栈耗尽；0 = 不检查
    pub limit: usize,
}

impl Bounds {
    pub const NONE: Bounds = Bounds { guard: (0, 0), limit: 0 };
}

thread_local! {
    /// 当前线程上正在运行的执行流的栈界
    static CURRENT: Cell<Bounds> = const { Cell::new(Bounds::NONE) };
    /// 处理器读取的载体信息（名称 / OS 线程号），指向 `CARRIER` 中的值
    static INFO: Cell<*const CarrierInfo> = const { Cell::new(std::ptr::null()) };
    /// 本线程的备用信号栈与载体信息（首次切入协程时建立，线程退出时释放）
    static CARRIER: RefCell<Option<Box<CarrierInfo>>> = const { RefCell::new(None) };
}

struct CarrierInfo {
    name: Box<str>,
    tid: u64,
    /// 本模块建立的备用信号栈（线程已有备用栈时为 None）
    alt: Option<(usize, usize)>,
}

impl Drop for CarrierInfo {
    fn drop(&mut self) {
        INFO.with(|c| c.set(std::ptr::null()));
        if let Some((base, len)) = self.alt {
            // SAFETY: 先停用本线程的备用栈，再释放其映射
            unsafe {
                let ss = libc::stack_t { ss_sp: std::ptr::null_mut(), ss_flags: libc::SS_DISABLE, ss_size: 0 };
                libc::sigaltstack(&ss, std::ptr::null_mut());
                libc::munmap(base as *mut libc::c_void, len);
            }
        }
    }
}

static INSTALL: Once = Once::new();
static mut PREV_SEGV: Option<libc::sigaction> = None;
static mut PREV_BUS: Option<libc::sigaction> = None;

/// 进程级安装处理器（幂等）
pub(crate) fn install() {
    INSTALL.call_once(|| {
        for (sig, slot) in [(libc::SIGSEGV, &raw mut PREV_SEGV), (libc::SIGBUS, &raw mut PREV_BUS)] {
            // SAFETY: Once 内单线程写入先前处理器；sigaction 结构零初始化后填写
            unsafe {
                let mut old: libc::sigaction = std::mem::zeroed();
                let mut sa: libc::sigaction = std::mem::zeroed();
                sa.sa_sigaction = on_fault as *const () as usize;
                sa.sa_flags = libc::SA_SIGINFO | libc::SA_ONSTACK;
                libc::sigemptyset(&mut sa.sa_mask);
                if libc::sigaction(sig, &sa, &mut old) == 0 {
                    *slot = Some(old);
                }
            }
        }
    });
}

/// 当前执行流的栈界（切换时存入被保存上下文）
#[inline]
pub(crate) fn current() -> Bounds {
    CURRENT.with(|c| c.get())
}

#[cfg(all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")))]
/// 本线程栈界槽（`Cell<Bounds>`）的地址，并暴露其来源供快路径按地址读取（[`crate::limit`]）
pub(crate) fn current_slot_addr() -> usize {
    CURRENT.with(|c| c.as_ptr().expose_provenance())
}

/// 换上即将切入的执行流的栈界；切入带硬件 guard 的协程栈前确保本线程有备用信号栈与载体信息
#[inline]
pub(crate) fn enter(b: Bounds) {
    if b.guard.0 != 0 && INFO.with(|c| c.get().is_null()) {
        init_carrier();
    }
    CURRENT.with(|c| c.set(b));
    crate::limit::check_tls_offset();
}

/// 改写当前执行流的软件栈界
#[inline]
pub(crate) fn set_limit(limit: usize) {
    CURRENT.with(|c| {
        let mut b = c.get();
        b.limit = limit;
        c.set(b);
    });
}

#[cold]
fn init_carrier() {
    let th = std::thread::current();
    let name: Box<str> = th.name().unwrap_or("<unknown>").into();
    let alt = ensure_altstack();
    let info = Box::new(CarrierInfo { name, tid: os_tid(), alt });
    let ptr: *const CarrierInfo = &*info;
    CARRIER.with(|c| *c.borrow_mut() = Some(info));
    INFO.with(|c| c.set(ptr));
}

/// 本线程没有备用信号栈时建立一块（64 KiB + guard page）
fn ensure_altstack() -> Option<(usize, usize)> {
    // SAFETY: sigaltstack 查询 / 设置本线程备用栈；映射由 CarrierInfo 析构释放
    unsafe {
        let mut cur: libc::stack_t = std::mem::zeroed();
        libc::sigaltstack(std::ptr::null(), &mut cur);
        if cur.ss_flags & libc::SS_DISABLE == 0 {
            return None;
        }
        let page = crate::stack::page_size();
        let size = (64usize << 10).max(libc::SIGSTKSZ);
        let len = size + page;
        let base = libc::mmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON,
            -1,
            0,
        );
        if base == libc::MAP_FAILED {
            return None;
        }
        libc::mprotect(base, page, libc::PROT_NONE);
        let ss = libc::stack_t { ss_sp: (base as usize + page) as *mut libc::c_void, ss_flags: 0, ss_size: size };
        libc::sigaltstack(&ss, std::ptr::null_mut());
        Some((base as usize, len))
    }
}

#[cfg(target_os = "macos")]
fn os_tid() -> u64 {
    let mut tid = 0u64;
    // SAFETY: 查询当前线程号
    unsafe { libc::pthread_threadid_np(0 as libc::pthread_t, &mut tid) };
    tid
}

#[cfg(not(target_os = "macos"))]
fn os_tid() -> u64 {
    // SAFETY: gettid 无副作用
    unsafe { libc::gettid() as u64 }
}

#[cfg(target_os = "macos")]
unsafe fn fault_addr(info: *mut libc::siginfo_t) -> usize {
    (*info).si_addr as usize
}

#[cfg(not(target_os = "macos"))]
unsafe fn fault_addr(info: *mut libc::siginfo_t) -> usize {
    (*info).si_addr() as usize
}

/// 异步信号安全的 stderr 输出
fn write_all(parts: &[&[u8]]) {
    for p in parts {
        let mut s = *p;
        while !s.is_empty() {
            // SAFETY: write(2) 是异步信号安全的
            let n = unsafe { libc::write(2, s.as_ptr().cast(), s.len()) };
            if n <= 0 {
                break;
            }
            s = &s[n as usize..];
        }
    }
}

fn fmt_u64(mut v: u64, buf: &mut [u8; 20]) -> &[u8] {
    let mut i = buf.len();
    loop {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    &buf[i..]
}

unsafe extern "C" fn on_fault(sig: libc::c_int, info: *mut libc::siginfo_t, ctx: *mut libc::c_void) {
    let addr = fault_addr(info);
    let (lo, hi) = CURRENT.with(|c| c.get()).guard;
    if addr != 0 && lo <= addr && addr < hi {
        let ci = INFO.with(|c| c.get());
        let (name, tid) = if ci.is_null() { ("<unknown>", 0) } else { (&*(*ci).name, (*ci).tid) };
        let mut buf = [0u8; 20];
        let tid = fmt_u64(tid, &mut buf);
        write_all(&[
            b"\nthread '",
            name.as_bytes(),
            b"' (",
            tid,
            b") has overflowed its stack\nfatal runtime error: stack overflow, aborting\n",
        ]);
        libc::abort();
    }
    let prev = if sig == libc::SIGSEGV { (*(&raw const PREV_SEGV)).as_ref() } else { (*(&raw const PREV_BUS)).as_ref() };
    match prev {
        Some(p) if p.sa_sigaction != libc::SIG_DFL && p.sa_sigaction != libc::SIG_IGN => {
            if p.sa_flags & libc::SA_SIGINFO != 0 {
                let f: unsafe extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void) =
                    std::mem::transmute(p.sa_sigaction);
                f(sig, info, ctx);
            } else {
                let f: unsafe extern "C" fn(libc::c_int) = std::mem::transmute(p.sa_sigaction);
                f(sig);
            }
        }
        _ => {
            // 恢复缺省处置后返回：故障指令重新执行，按缺省动作终止
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = libc::SIG_DFL;
            libc::sigaction(sig, &sa, std::ptr::null_mut());
        }
    }
}
