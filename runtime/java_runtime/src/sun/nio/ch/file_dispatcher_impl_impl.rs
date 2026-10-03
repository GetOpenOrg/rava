//! `sun/nio/ch/FileDispatcherImpl` 的 ACC_NATIVE（类 1，libnio 平台目录下的 `FileDispatcherImpl.c`）。
//! 其余方法按字节码翻译。
//!
//! 该类按平台各有一份字节码：Linux 版有 `init0` / `transferTo0` / `transferFrom0`（`<clinit>` 为
//! `IOUtil.load(); init0();`）；macOS 版有 `transferTo0`（另有 `force0`）。下面的 native 落点只在声明它的
//! 平台上被翻译体调用，另一平台上只是多出一个未被调用的固有函数。
//!
//! 返回值约定同 JNI（`sun.nio.ch.IOStatus`）：成功为字节数；UNAVAILABLE / INTERRUPTED /
//! UNSUPPORTED_CASE 为负状态码，由 FileChannelImpl 退回通用拷贝；其余失败抛 IOException。

use crate::prelude::*;
use super::file_dispatcher_impl::FileDispatcherImpl;
use super::unix_file_dispatcher_impl_impl::{errno, fd_of, io_exception};
use crate::java::io::FileDescriptor;

const IOS_UNAVAILABLE: i64 = -2;
const IOS_INTERRUPTED: i64 = -3;
const IOS_UNSUPPORTED_CASE: i64 = -6;

impl FileDispatcherImpl {
    /// native `init0()`（Linux）：JNI 侧用 `dlsym(RTLD_DEFAULT, "copy_file_range")` 取函数指针，
    /// 留给 transferTo0 / transferFrom0 的快速路径使用；取不到时退回 sendfile。原生二进制直接链接 libc，
    /// 不需要运行期查找函数指针，因此这里没有需要初始化的状态。
    #[jvm_native]
    pub fn init0() -> Result<()> {
        Ok(())
    }

    /// native `transferTo0(FileDescriptor src, long position, long count, FileDescriptor dst, boolean append)`：
    /// 从 src 的 position 起向 dst 拷贝至多 count 字节（不移动 src 的文件位置）。append 目标一律
    /// UNSUPPORTED_CASE（copy_file_range 对 O_APPEND 报 EBADF、sendfile 报 EINVAL）。
    /// Linux：copy_file_range，EINVAL / ENOSYS / EXDEV 退回 sendfile；macOS：sendfile(2)（仅支持套接字目标，
    /// 普通文件得 ENOTSOCK → UNSUPPORTED_CASE）。
    #[jvm_native]
    pub fn transferTo0(src: FileDescriptor, position: i64, count: i64, dst: FileDescriptor, append: bool) -> Result<i64> {
        if append {
            return Ok(IOS_UNSUPPORTED_CASE);
        }
        transfer_to(fd_of(&src), position, count, fd_of(&dst))
    }

    /// native `transferFrom0(FileDescriptor src, FileDescriptor dst, long position, long count, boolean append)`
    /// （Linux）：copy_file_range 从 src 当前位置写到 dst 的 position 处。append 目标、EBADF / EINVAL /
    /// EXDEV / ENOSYS 均为 UNSUPPORTED_CASE（由调用方退回通用拷贝），EAGAIN 为 UNAVAILABLE。
    #[jvm_native]
    pub fn transferFrom0(src: FileDescriptor, dst: FileDescriptor, position: i64, count: i64, append: bool) -> Result<i64> {
        if append {
            return Ok(IOS_UNSUPPORTED_CASE);
        }
        transfer_from(fd_of(&src), fd_of(&dst), position, count)
    }
}

/// 拷贝长度上限：SSIZE_MAX（JNI 同款截断）
#[cfg(target_os = "linux")]
fn clamp(count: i64) -> libc::size_t {
    count.clamp(0, isize::MAX as i64) as libc::size_t
}

#[cfg(target_os = "linux")]
fn transfer_to(src: i32, position: i64, count: i64, dst: i32) -> Result<i64> {
    let mut offset = position as libc::off64_t;
    // SAFETY: src / dst 为调用方持有的描述符；offset 为栈上变量（不移动 src 的文件位置）
    let n = unsafe { libc::copy_file_range(src, &mut offset, dst, std::ptr::null_mut(), clamp(count), 0) };
    if n >= 0 {
        return Ok(n as i64);
    }
    match errno() {
        libc::EINTR => return Ok(IOS_INTERRUPTED),
        libc::EINVAL | libc::ENOSYS | libc::EXDEV => {}
        _ => return Err(io_exception("Copy failed")),
    }
    let mut offset = position as libc::off64_t;
    // SAFETY: 同上
    let n = unsafe { libc::sendfile64(dst, src, &mut offset, count as libc::size_t) };
    if n >= 0 {
        return Ok(n as i64);
    }
    match errno() {
        libc::EAGAIN => Ok(IOS_UNAVAILABLE),
        libc::EINVAL if count >= 0 => Ok(IOS_UNSUPPORTED_CASE),
        libc::EINTR => Ok(IOS_INTERRUPTED),
        _ => Err(io_exception("Transfer failed")),
    }
}

#[cfg(target_os = "linux")]
fn transfer_from(src: i32, dst: i32, position: i64, count: i64) -> Result<i64> {
    let mut offset = position as libc::off64_t;
    // SAFETY: src / dst 为调用方持有的描述符；offset 为栈上变量（不移动 dst 的文件位置）
    let n = unsafe { libc::copy_file_range(src, std::ptr::null_mut(), dst, &mut offset, clamp(count), 0) };
    if n >= 0 {
        return Ok(n as i64);
    }
    match errno() {
        libc::EAGAIN => Ok(IOS_UNAVAILABLE),
        libc::ENOSYS => Ok(IOS_UNSUPPORTED_CASE),
        libc::EBADF | libc::EINVAL | libc::EXDEV if count >= 0 => Ok(IOS_UNSUPPORTED_CASE),
        libc::EINTR => Ok(IOS_INTERRUPTED),
        _ => Err(io_exception("Transfer failed")),
    }
}

#[cfg(target_os = "macos")]
fn transfer_to(src: i32, position: i64, count: i64, dst: i32) -> Result<i64> {
    let mut len: libc::off_t = count;
    // SAFETY: src 为文件描述符、dst 为目标描述符；len 为栈上变量（入：请求字节数，出：实际字节数）
    let r = unsafe { libc::sendfile(src, dst, position, &mut len, std::ptr::null_mut(), 0) };
    if len > 0 {
        return Ok(len);
    }
    if r == -1 {
        return match errno() {
            libc::EAGAIN => Ok(IOS_UNAVAILABLE),
            libc::EOPNOTSUPP | libc::ENOTSOCK | libc::ENOTCONN => Ok(IOS_UNSUPPORTED_CASE),
            libc::EINVAL if count >= 0 => Ok(IOS_UNSUPPORTED_CASE),
            libc::EINTR => Ok(IOS_INTERRUPTED),
            _ => Err(io_exception("Transfer failed")),
        };
    }
    Ok(r as i64)
}

/// macOS 的 FileDispatcherImpl 没有 transferFrom0：此落点不会被翻译体调用，只为保持文件可编译。
#[cfg(not(target_os = "linux"))]
fn transfer_from(_src: i32, _dst: i32, _position: i64, _count: i64) -> Result<i64> {
    Ok(IOS_UNSUPPORTED_CASE)
}
