//! `sun/nio/ch/UnixFileDispatcherImpl` 的 ACC_NATIVE（类 1，libnio `UnixFileDispatcherImpl.c`）。
//!
//! 缓冲区参数是本地内存绝对地址（`Unsafe.allocateMemory` / 直接缓冲区）。返回值与异常按 JNI 同款：
//! 读写走 `convertReturnVal`（EOF = -1、EAGAIN = -2、EINTR = -3，其余抛 IOException），
//! 其它调用走 `handle`（EINTR = -3，其余抛 IOException，消息为 strerror 文案）。

use crate::prelude::*;
use super::unix_file_dispatcher_impl::UnixFileDispatcherImpl;
use crate::java::io::FileDescriptor;

const IOS_EOF: i64 = -1;
const IOS_UNAVAILABLE: i64 = -2;
const IOS_INTERRUPTED: i64 = -3;

fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// `JNU_ThrowIOExceptionWithLastError`：有系统错误文案用文案，否则用缺省消息。
fn io_exception(default: &str) -> JvmError {
    let err = errno();
    let msg = if err == 0 {
        default.to_owned()
    } else {
        let s = std::io::Error::from_raw_os_error(err).to_string();
        s.split(" (os error").next().unwrap_or(default).to_owned()
    };
    match crate::java::io::IOException::new_str(String::from(msg)) {
        Ok(x) => JvmError::from(x),
        Err(e) => e,
    }
}

/// JNI `convertReturnVal`
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
        _ => Err(io_exception(if reading { "Read failed" } else { "Write failed" })),
    }
}

/// JNI `handle`
fn handle(rv: i64, msg: &str) -> Result<i64> {
    if rv >= 0 {
        return Ok(rv);
    }
    if errno() == libc::EINTR {
        return Ok(IOS_INTERRUPTED);
    }
    Err(io_exception(msg))
}

fn fd_of(fdo: &FileDescriptor) -> i32 {
    fdo.__get_fd()
}

impl UnixFileDispatcherImpl {
    /// native `read0(FileDescriptor, long address, int len)`：read(2)。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn read0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = unsafe { libc::read(fd_of(&fdo), address as *mut libc::c_void, len as usize) };
        Ok(convert(n as i64, true)? as i32)
    }

    /// native `pread0(FileDescriptor, long address, int len, long position)`：pread(2)。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn pread0(fdo: FileDescriptor, address: i64, len: i32, position: i64) -> Result<i32> {
        // SAFETY: 同 read0
        let n = unsafe { libc::pread(fd_of(&fdo), address as *mut libc::c_void, len as usize, position as libc::off_t) };
        Ok(convert(n as i64, true)? as i32)
    }

    /// native `write0(FileDescriptor, long address, int len)`：write(2)。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn write0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = unsafe { libc::write(fd_of(&fdo), address as *const libc::c_void, len as usize) };
        Ok(convert(n as i64, false)? as i32)
    }

    /// native `pwrite0(FileDescriptor, long address, int len, long position)`：pwrite(2)。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn pwrite0(fdo: FileDescriptor, address: i64, len: i32, position: i64) -> Result<i32> {
        // SAFETY: 同 write0
        let n = unsafe { libc::pwrite(fd_of(&fdo), address as *const libc::c_void, len as usize, position as libc::off_t) };
        Ok(convert(n as i64, false)? as i32)
    }

    /// native `seek0(FileDescriptor, long offset)`：offset < 0 取当前位置，否则定位到 offset。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn seek0(fdo: FileDescriptor, offset: i64) -> Result<i64> {
        // SAFETY: lseek 只作用于 fd
        let r = unsafe {
            if offset < 0 {
                libc::lseek(fd_of(&fdo), 0, libc::SEEK_CUR)
            } else {
                libc::lseek(fd_of(&fdo), offset as libc::off_t, libc::SEEK_SET)
            }
        };
        handle(r as i64, "lseek64 failed")
    }

    /// native `size0(FileDescriptor)`：fstat(2) 的 st_size。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn size0(fdo: FileDescriptor) -> Result<i64> {
        // SAFETY: buf 为栈上 stat 结构
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        if unsafe { libc::fstat(fd_of(&fdo), &mut buf) } < 0 {
            return handle(-1, "Size failed");
        }
        Ok(buf.st_size as i64)
    }

    /// native `truncate0(FileDescriptor, long size)`：ftruncate(2)。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn truncate0(fdo: FileDescriptor, size: i64) -> Result<i32> {
        // SAFETY: ftruncate 只作用于 fd
        let r = unsafe { libc::ftruncate(fd_of(&fdo), size as libc::off_t) };
        Ok(handle(r as i64, "Truncation failed")? as i32)
    }

    /// native `allocationGranularity0()`：页大小。
    #[jvm_native]
    pub fn allocationGranularity0() -> Result<i64> {
        // SAFETY: sysconf 无副作用
        Ok(unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as i64)
    }

    /// native `setDirect0(FileDescriptor)`：打开直接 I/O 并返回文件系统块大小。
    /// macOS：fcntl(F_NOCACHE)；Linux：追加 O_DIRECT。失败抛 IOException("DirectIO setup failed")。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn setDirect0(fdo: FileDescriptor) -> Result<i32> {
        let fd = fd_of(&fdo);
        // SAFETY: fcntl / fstatvfs 只作用于 fd，vfs 为栈上结构
        unsafe {
            #[cfg(target_os = "macos")]
            let r = libc::fcntl(fd, libc::F_NOCACHE, 1);
            #[cfg(not(target_os = "macos"))]
            let r = {
                let flags = libc::fcntl(fd, libc::F_GETFL);
                libc::fcntl(fd, libc::F_SETFL, flags | libc::O_DIRECT)
            };
            if r == -1 {
                return Err(io_exception("DirectIO setup failed"));
            }
            let mut vfs: libc::statvfs = std::mem::zeroed();
            if libc::fstatvfs(fd, &mut vfs) == -1 {
                return Err(io_exception("DirectIO setup failed"));
            }
            Ok(vfs.f_frsize as i32)
        }
    }
}
