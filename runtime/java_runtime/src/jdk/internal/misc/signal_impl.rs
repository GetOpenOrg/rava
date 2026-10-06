//! `jdk/internal/misc/Signal` 的 ACC_NATIVE（类 ①，HotSpot `jvm.cpp` JVM_FindSignal / JVM_RegisterSignal /
//! JVM_RaiseSignal 与 `signals_posix.cpp`）。
//!
//! 信号到 Java 的投递与 HotSpot 同构：C 处理器只做异步信号安全的一件事——把信号号写入自管道；
//! 「Signal Dispatcher」守护系统线程（首次安装处理器时惰性启动）读管道，以 `Signal.dispatch(int)`
//! 回到 Java（dispatch 再为每个信号派生处理线程）。引导期 `Terminator.setup` 经此注册 HUP / INT / TERM。

use crate::prelude::*;
use super::signal::Signal;
use std::sync::OnceLock;

/// 信号名（不带 `SIG` 前缀，与 `Signal(String)` 的入参一致）→ 信号号
const SIGNALS: &[(&str, libc::c_int)] = &[
    ("HUP", libc::SIGHUP),
    ("INT", libc::SIGINT),
    ("QUIT", libc::SIGQUIT),
    ("ILL", libc::SIGILL),
    ("TRAP", libc::SIGTRAP),
    ("ABRT", libc::SIGABRT),
    ("BUS", libc::SIGBUS),
    ("FPE", libc::SIGFPE),
    ("KILL", libc::SIGKILL),
    ("USR1", libc::SIGUSR1),
    ("SEGV", libc::SIGSEGV),
    ("USR2", libc::SIGUSR2),
    ("PIPE", libc::SIGPIPE),
    ("ALRM", libc::SIGALRM),
    ("TERM", libc::SIGTERM),
    ("CHLD", libc::SIGCHLD),
    ("CONT", libc::SIGCONT),
    ("STOP", libc::SIGSTOP),
    ("TSTP", libc::SIGTSTP),
    ("TTIN", libc::SIGTTIN),
    ("TTOU", libc::SIGTTOU),
    ("URG", libc::SIGURG),
    ("XCPU", libc::SIGXCPU),
    ("XFSZ", libc::SIGXFSZ),
    ("VTALRM", libc::SIGVTALRM),
    ("PROF", libc::SIGPROF),
    ("WINCH", libc::SIGWINCH),
    ("IO", libc::SIGIO),
    ("SYS", libc::SIGSYS),
];

/// VM 保留的信号（HotSpot `os::Posix::is_sig_reserved` 与 BREAK_SIGNAL）：Java 不得接管
const RESERVED: &[libc::c_int] = &[libc::SIGSEGV, libc::SIGBUS, libc::SIGFPE, libc::SIGILL, libc::SIGQUIT];

/// 关停信号：启动时已被忽略（如 nohup）则保持忽略（HotSpot JVM_RegisterSignal 同一规则）
const SHUTDOWN: &[libc::c_int] = &[libc::SIGHUP, libc::SIGINT, libc::SIGTERM];

/// handle0 的处理器编码：0 = 缺省、1 = 忽略、2 = 投递给 Java
const H_DFL: i64 = 0;
const H_IGN: i64 = 1;
const H_JAVA: i64 = 2;

/// 自管道写端（C 处理器只读此值）
static PIPE_WRITE: OnceLock<libc::c_int> = OnceLock::new();

extern "C" fn user_handler(sig: libc::c_int) {
    if let Some(&fd) = PIPE_WRITE.get() {
        let b = sig as u8;
        // SAFETY: write 为异步信号安全；写端非阻塞，管道满时丢弃（与 HotSpot 计数饱和同为尽力投递）
        unsafe {
            libc::write(fd, &b as *const u8 as *const libc::c_void, 1);
        }
    }
}

fn user_handler_addr() -> libc::sighandler_t {
    user_handler as extern "C" fn(libc::c_int) as libc::sighandler_t
}

/// 首次投递给 Java 时建立自管道并启动 Signal Dispatcher（HotSpot `os::signal_init` → `signal_thread_entry`）
fn ensure_dispatcher() -> Result<()> {
    if PIPE_WRITE.get().is_some() {
        return Ok(());
    }
    let mut fds = [0 as libc::c_int; 2];
    // SAFETY: fds 为两元素数组
    if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
        return Err(JvmError::out_of_memory("unable to create signal pipe"));
    }
    // SAFETY: 两端均为刚创建的有效描述符
    unsafe {
        for fd in fds {
            libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
        }
        libc::fcntl(fds[1], libc::F_SETFL, libc::fcntl(fds[1], libc::F_GETFL) | libc::O_NONBLOCK);
    }
    if PIPE_WRITE.set(fds[1]).is_err() {
        // 并发初始化的另一方已就绪：关闭本方管道
        // SAFETY: 本方描述符未被他处使用
        unsafe {
            libc::close(fds[0]);
            libc::close(fds[1]);
        }
        return Ok(());
    }
    let read_fd = fds[0];
    crate::java::lang::Thread::__vm_spawn_system("Signal Dispatcher", move || loop {
        let mut b = 0u8;
        // SAFETY: 读入一个字节到栈上缓冲
        let n = crate::gil::blocking(|| unsafe { libc::read(read_fd, &mut b as *mut u8 as *mut libc::c_void, 1) });
        if n == 1 {
            if let Err(e) = Signal::dispatch(i32::from(b)) {
                e.report_uncaught_in("Signal Dispatcher");
            }
        } else if n < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
            continue;
        } else {
            return;
        }
    })
}

impl Signal {
    /// native `findSignal0(String)`：信号名 → 信号号，未知为 -1
    #[jvm_native]
    pub fn findSignal0(arg0: String) -> Result<i32> {
        let name = format!("{arg0}");
        Ok(SIGNALS.iter().find(|(n, _)| *n == name).map_or(-1, |&(_, s)| s))
    }

    /// native `handle0(int, long)`：安装处理器并返回原处理器（同一编码，非本运行时的处理器返回其地址）；
    /// 保留信号与安装失败返回 -1，启动时被忽略的关停信号返回 1 且不改动
    #[jvm_native]
    pub fn handle0(arg0: i32, arg1: i64) -> Result<i64> {
        let sig = arg0 as libc::c_int;
        if RESERVED.contains(&sig) {
            return Ok(-1);
        }
        // SAFETY: 只查询当前处置（新处置为空指针）
        let old = unsafe {
            let mut cur: libc::sigaction = std::mem::zeroed();
            if libc::sigaction(sig, std::ptr::null(), &mut cur) != 0 {
                return Ok(-1);
            }
            cur.sa_sigaction
        };
        if SHUTDOWN.contains(&sig) && old == libc::SIG_IGN {
            return Ok(H_IGN);
        }
        let new = match arg1 {
            H_DFL => libc::SIG_DFL,
            H_IGN => libc::SIG_IGN,
            H_JAVA => {
                ensure_dispatcher()?;
                user_handler_addr()
            }
            raw => raw as libc::sighandler_t,
        };
        // SAFETY: sigaction 结构按零初始化后填写处理器与空掩码
        let ok = unsafe {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = new;
            sa.sa_flags = libc::SA_RESTART;
            libc::sigemptyset(&mut sa.sa_mask);
            libc::sigaction(sig, &sa, std::ptr::null_mut()) == 0
        };
        if !ok {
            return Ok(-1);
        }
        Ok(if old == user_handler_addr() {
            H_JAVA
        } else {
            old as i64
        })
    }

    /// native `raise0(int)`：向本进程发信号
    #[jvm_native]
    pub fn raise0(arg0: i32) -> Result<()> {
        // SAFETY: raise 无内存前置条件
        unsafe {
            libc::raise(arg0 as libc::c_int);
        }
        Ok(())
    }
}
