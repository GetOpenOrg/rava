//! `sun/nio/ch/EPoll` 的 ACC_NATIVE（类 1，libnio `EPoll.c`，Linux）。
//! 结构布局（epoll_event 大小与字段偏移）供 Java 侧经 Unsafe 读写本地事件数组。
#![cfg(target_os = "linux")]

use crate::prelude::*;
use super::e_poll::EPoll;

const IOS_INTERRUPTED: i32 = -3;

fn io_exception(default: &str) -> JvmError {
    let msg = String::from(crate::net_posix::last_error_message(default));
    crate::java::io::IOException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

impl EPoll {
    #[jvm_native]
    pub fn eventSize() -> Result<i32> {
        Ok(std::mem::size_of::<libc::epoll_event>() as i32)
    }

    #[jvm_native]
    pub fn eventsOffset() -> Result<i32> {
        Ok(std::mem::offset_of!(libc::epoll_event, events) as i32)
    }

    #[jvm_native]
    pub fn dataOffset() -> Result<i32> {
        Ok(std::mem::offset_of!(libc::epoll_event, u64) as i32)
    }

    /// native `create()`：epoll_create1(EPOLL_CLOEXEC)。
    #[jvm_native]
    pub fn create() -> Result<i32> {
        // SAFETY: 无指针参数
        let epfd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
        if epfd < 0 {
            return Err(io_exception("epoll_create1 failed"));
        }
        Ok(epfd)
    }

    /// native `ctl(int epfd, int opcode, int fd, int events)`：data 携带 fd；返回 0 或 errno。
    #[jvm_native]
    pub fn ctl(epfd: i32, opcode: i32, fd: i32, events: i32) -> Result<i32> {
        let mut event = libc::epoll_event { events: events as u32, u64: fd as u32 as u64 };
        // SAFETY: event 为栈上结构
        let res = unsafe { libc::epoll_ctl(epfd, opcode, fd, &mut event) };
        Ok(if res == 0 { 0 } else { crate::net_posix::errno() })
    }

    /// native `wait(int epfd, long pollAddress, int numfds, int timeout)`：被信号打断返回 INTERRUPTED。
    #[jvm_native]
    pub fn wait_i_l_i_i(epfd: i32, address: i64, numfds: i32, timeout: i32) -> Result<i32> {
        // SAFETY: address 指向至少 numfds 个 epoll_event 的本地数组
        let res = crate::gil::blocking(|| unsafe { libc::epoll_wait(epfd, address as *mut libc::epoll_event, numfds, timeout) });
        if res < 0 {
            if crate::net_posix::errno() == libc::EINTR {
                return Ok(IOS_INTERRUPTED);
            }
            return Err(io_exception("epoll_wait failed"));
        }
        Ok(res)
    }
}
