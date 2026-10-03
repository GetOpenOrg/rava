//! `sun/nio/ch/SocketDispatcher` 的 ACC_NATIVE（类 1，libnio `SocketDispatcher.c`）。
//!
//! 缓冲区参数是本地内存绝对地址。读到 ECONNRESET / EPIPE 抛 `sun.net.ConnectionResetException`
//! （「Connection reset」），其余按 `convertReturnVal`：EOF = -1、EAGAIN = -2、EINTR = -3，失败抛 IOException。

use crate::prelude::*;
use super::socket_dispatcher::SocketDispatcher;
use crate::java::io::FileDescriptor;
use crate::net_posix::{self, errno};

const IOS_EOF: i64 = -1;
const IOS_UNAVAILABLE: i64 = -2;
const IOS_INTERRUPTED: i64 = -3;

/// JNI `convertReturnVal` / `convertLongReturnVal`
fn convert(n: i64, reading: bool) -> Result<i64> {
    if n > 0 {
        return Ok(n);
    }
    if n == 0 {
        return Ok(if reading { IOS_EOF } else { 0 });
    }
    match errno() {
        libc::EAGAIN => Ok(IOS_UNAVAILABLE),
        libc::EINTR => Ok(IOS_INTERRUPTED),
        _ => {
            let msg = String::from(net_posix::last_error_message(if reading { "Read failed" } else { "Write failed" }));
            Err(crate::java::io::IOException::new_str(msg).map(JvmError::from).unwrap_or_else(|e| e))
        }
    }
}

/// 读结果：连接被对端重置时抛 ConnectionResetException。
fn read_result(n: i64) -> Result<i64> {
    if n == -1 && matches!(errno(), libc::ECONNRESET | libc::EPIPE) {
        let e = crate::sun::net::ConnectionResetException::new_str(String::from("Connection reset"));
        return Err(e.map(JvmError::from).unwrap_or_else(|e| e));
    }
    convert(n, true)
}

impl SocketDispatcher {
    /// native `read0(FileDescriptor, long address, int len)`：read(2)。
    #[jvm_native]
    pub fn read0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        let fd = fdo.__get_fd();
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = crate::gil::blocking(|| unsafe { libc::read(fd, address as *mut libc::c_void, len as usize) });
        Ok(read_result(n as i64)? as i32)
    }

    /// native `readv0(FileDescriptor, long address, int len)`：address 为 len 个 iovec。
    #[jvm_native]
    pub fn readv0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i64> {
        let fd = fdo.__get_fd();
        // SAFETY: address 指向 len 个有效 iovec
        let n = crate::gil::blocking(|| unsafe { libc::readv(fd, address as *const libc::iovec, len) });
        read_result(n as i64)
    }

    /// native `write0(FileDescriptor, long address, int len)`：write(2)。
    #[jvm_native]
    pub fn write0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        let fd = fdo.__get_fd();
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = crate::gil::blocking(|| unsafe { libc::write(fd, address as *const libc::c_void, len as usize) });
        Ok(convert(n as i64, false)? as i32)
    }

    /// native `writev0(FileDescriptor, long address, int len)`：address 为 len 个 iovec。
    #[jvm_native]
    pub fn writev0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i64> {
        let fd = fdo.__get_fd();
        // SAFETY: address 指向 len 个有效 iovec
        let n = crate::gil::blocking(|| unsafe { libc::writev(fd, address as *const libc::iovec, len) });
        convert(n as i64, false)
    }
}
