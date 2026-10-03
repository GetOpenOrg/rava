//! `sun/nio/ch/UnixDispatcher` 的 ACC_NATIVE（类 1，libnio `UnixDispatcher.c`）。
//!
//! 异步关闭协议：init 建一对 UNIX 域套接字、关掉一端，留下的半关闭端作「预关闭」fd；
//! preClose0 以 dup2 把它覆盖到待关闭 fd 上——阻塞在该 fd 上的读写立即见 EOF / EPIPE，
//! 而 fd 号在真正 close0 之前不会被复用。

use std::sync::atomic::{AtomicI32, Ordering};

use crate::prelude::*;
use super::unix_dispatcher::UnixDispatcher;
use crate::java::io::FileDescriptor;
use crate::net_posix::{self, errno};

static PRE_CLOSE_FD: AtomicI32 = AtomicI32::new(-1);

fn io_exception(default: &str) -> JvmError {
    let msg = String::from(net_posix::last_error_message(default));
    crate::java::io::IOException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e)
}

impl UnixDispatcher {
    /// native `init()`：socketpair 后关闭 sp[1]，sp[0] 作预关闭 fd。
    #[jvm_native]
    pub fn init() -> Result<()> {
        let mut sp = [0 as libc::c_int; 2];
        // SAFETY: socketpair 写入两元素数组
        if unsafe { libc::socketpair(libc::PF_UNIX, libc::SOCK_STREAM, 0, sp.as_mut_ptr()) } < 0 {
            return Err(io_exception("socketpair failed"));
        }
        PRE_CLOSE_FD.store(sp[0], Ordering::SeqCst);
        // SAFETY: 关闭本函数创建的另一端
        unsafe { libc::close(sp[1]) };
        Ok(())
    }

    /// native `preClose0(FileDescriptor)`：dup2(预关闭 fd, fd)。
    #[jvm_native]
    pub fn preClose0(fdo: FileDescriptor) -> Result<()> {
        let pre = PRE_CLOSE_FD.load(Ordering::SeqCst);
        // SAFETY: 两个 fd 均为进程内有效描述符
        if pre >= 0 && unsafe { libc::dup2(pre, fdo.__get_fd()) } < 0 {
            return Err(io_exception("dup2 failed"));
        }
        Ok(())
    }

    /// native `close0(FileDescriptor)`：close(2)；EINTR 不报错。
    #[jvm_native]
    pub fn close0(fdo: FileDescriptor) -> Result<()> {
        let fd = fdo.__get_fd();
        // SAFETY: 关闭调用方持有的 fd
        if fd != -1 && unsafe { libc::close(fd) } < 0 && errno() != libc::EINTR {
            return Err(io_exception("Close failed"));
        }
        Ok(())
    }
}
