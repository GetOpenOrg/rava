//! `sun/nio/ch/NativeThread` 的 ACC_NATIVE（类 1，libnio `NativeThread.c`）。
//! 阻塞 I/O 的中断信号：安装空处理器，signal0 以 pthread_kill 打断目标线程的系统调用。

use crate::prelude::*;
use super::native_thread::NativeThread;

#[cfg(target_os = "linux")]
fn interrupt_signal() -> libc::c_int {
    libc::SIGRTMAX() - 2
}

#[cfg(not(target_os = "linux"))]
fn interrupt_signal() -> libc::c_int {
    libc::SIGIO
}

extern "C" fn null_handler(_sig: libc::c_int) {}

fn io_exception(default: &str) -> JvmError {
    let e = std::io::Error::last_os_error();
    let s = e.to_string();
    let msg = match s.split(" (os error").next() {
        Some(m) if !m.is_empty() => m.to_owned(),
        _ => default.to_owned(),
    };
    match crate::java::io::IOException::new_str(String::from(msg)) {
        Ok(x) => JvmError::from(x),
        Err(e) => e,
    }
}

impl NativeThread {
    /// native `init()`：为中断信号安装空处理器（sa_flags = 0：被打断的系统调用返回 EINTR）。
    #[jvm_native]
    pub fn init() -> Result<()> {
        // SAFETY: sigaction 结构按零初始化后填写处理器与空掩码
        let rc = unsafe {
            let mut sa: libc::sigaction = std::mem::zeroed();
            sa.sa_sigaction = null_handler as extern "C" fn(libc::c_int) as libc::sighandler_t;
            sa.sa_flags = 0;
            libc::sigemptyset(&mut sa.sa_mask);
            libc::sigaction(interrupt_signal(), &sa, std::ptr::null_mut())
        };
        if rc < 0 {
            return Err(io_exception("sigaction"));
        }
        Ok(())
    }

    /// native `current0()`：`pthread_self()`。
    #[jvm_native]
    pub fn current0() -> Result<i64> {
        // SAFETY: pthread_self 恒有效
        Ok(unsafe { libc::pthread_self() } as usize as i64)
    }

    /// native `signal0(long)`：向目标线程发中断信号；目标已结束（ESRCH）不报错（macOS JNI 同款）。
    #[jvm_native]
    pub fn signal0(thread: i64) -> Result<()> {
        // SAFETY: thread 为 current0 返回的 pthread_t
        let ret = unsafe { libc::pthread_kill(thread as usize as libc::pthread_t, interrupt_signal()) };
        if ret != 0 && ret != libc::ESRCH {
            return Err(io_exception("Thread signal failed"));
        }
        Ok(())
    }

    /// native `supportPendingSignals0()`：Linux 上发给未处于阻塞调用中的线程的信号会挂起到其进入
    /// 阻塞调用时投递（libnio 同款：仅 Linux / AIX 为 true）。
    #[jvm_native]
    pub fn supportPendingSignals0() -> Result<bool> {
        Ok(cfg!(target_os = "linux"))
    }
}
