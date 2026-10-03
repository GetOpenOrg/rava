//! `sun/nio/ch/KQueue` 的 ACC_NATIVE（类 1，libnio `KQueue.c`，macOS）。
//! 结构布局（kevent 大小与字段偏移）供 Java 侧经 Unsafe 读写本地事件数组。
#![cfg(target_os = "macos")]

use crate::prelude::*;
use super::k_queue::KQueue;

const IOS_INTERRUPTED: i32 = -3;

fn io_exception(default: &str) -> JvmError {
    let msg = String::from(crate::net_posix::last_error_message(default));
    crate::java::io::IOException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

impl KQueue {
    #[jvm_native]
    pub fn keventSize() -> Result<i32> {
        Ok(std::mem::size_of::<libc::kevent>() as i32)
    }

    #[jvm_native]
    pub fn identOffset() -> Result<i32> {
        Ok(std::mem::offset_of!(libc::kevent, ident) as i32)
    }

    #[jvm_native]
    pub fn filterOffset() -> Result<i32> {
        Ok(std::mem::offset_of!(libc::kevent, filter) as i32)
    }

    #[jvm_native]
    pub fn flagsOffset() -> Result<i32> {
        Ok(std::mem::offset_of!(libc::kevent, flags) as i32)
    }

    /// native `create()`：kqueue(2)。
    #[jvm_native]
    pub fn create() -> Result<i32> {
        // SAFETY: kqueue 无参数
        let kq = unsafe { libc::kqueue() };
        if kq < 0 {
            return Err(io_exception("kqueue failed"));
        }
        Ok(kq)
    }

    /// native `register(int kqfd, int fd, int filter, int flags)`：提交单个变更，返回 errno 或 0。
    #[jvm_native]
    pub fn register(kqfd: i32, fd: i32, filter: i32, flags: i32) -> Result<i32> {
        let change = libc::kevent {
            ident: fd as libc::uintptr_t,
            filter: filter as i16,
            flags: flags as u16,
            fflags: 0,
            data: 0,
            udata: std::ptr::null_mut(),
        };
        // SAFETY: 单个变更、无事件输出
        let res = unsafe { libc::kevent(kqfd, &change, 1, std::ptr::null_mut(), 0, std::ptr::null()) };
        Ok(if res == -1 { crate::net_posix::errno() } else { 0 })
    }

    /// native `poll(int kqfd, long pollAddress, int nevents, long timeout)`：timeout < 0 无限等待；
    /// 被信号打断返回 INTERRUPTED。
    #[jvm_native]
    pub fn poll(kqfd: i32, address: i64, nevents: i32, timeout: i64) -> Result<i32> {
        let ts = libc::timespec { tv_sec: (timeout / 1000) as libc::time_t, tv_nsec: ((timeout % 1000) * 1_000_000) as libc::c_long };
        let tsp: *const libc::timespec = if timeout >= 0 { &ts } else { std::ptr::null() };
        // SAFETY: address 指向至少 nevents 个 kevent 的本地数组
        let res = crate::gil::blocking(|| unsafe {
            libc::kevent(kqfd, std::ptr::null(), 0, address as *mut libc::kevent, nevents, tsp)
        });
        if res < 0 {
            if crate::net_posix::errno() == libc::EINTR {
                return Ok(IOS_INTERRUPTED);
            }
            return Err(io_exception("kqueue failed"));
        }
        Ok(res)
    }
}
