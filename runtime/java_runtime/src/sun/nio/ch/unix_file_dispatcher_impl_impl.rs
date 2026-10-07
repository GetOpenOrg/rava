//! `sun/nio/ch/UnixFileDispatcherImpl` 的 ACC_NATIVE（类 1，libnio `UnixFileDispatcherImpl.c`）。
//!
//! 缓冲区参数是本地内存绝对地址（`Unsafe.allocateMemory` / 直接缓冲区）。返回值与异常按 JNI 同款：
//! 读写走 `convertReturnVal`（EOF = -1、EAGAIN = -2、EINTR = -3，其余抛 IOException），
//! 其它调用走 `handle`（EINTR = -3，其余抛 IOException，消息为 strerror 文案）。
//! 虚拟句柄（`${java.home}` 虚拟树，`crate::jdk_resources`，NIO `open0` 所得）的 read / pread / seek / size
//! 读嵌入数据；其余调用对负 fd 由系统调用自然得 EBADF（与只读打开的写入同为 IOException）。

use crate::prelude::*;
use super::unix_file_dispatcher_impl::UnixFileDispatcherImpl;
use crate::java::io::FileDescriptor;

const IOS_EOF: i64 = -1;
const IOS_UNAVAILABLE: i64 = -2;
const IOS_INTERRUPTED: i64 = -3;

/// `FileChannelImpl.MAP_RO` / `MAP_RW` / `MAP_PV`（map0 的 prot 实参）
const MAP_RO: i32 = 0;
const MAP_RW: i32 = 1;

pub(super) fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// `JNU_ThrowIOExceptionWithLastError`：有系统错误文案用文案，否则用缺省消息。
pub(super) fn io_exception(default: &str) -> JvmError {
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

pub(super) fn fd_of(fdo: &FileDescriptor) -> i32 {
    fdo.__get_fd()
}

/// 虚拟句柄的读目标缓冲区（本地内存 address 起 len 字节）；非虚拟句柄为 None
fn virtual_buf<'a>(fdo: &FileDescriptor, address: i64, len: i32) -> Option<&'a mut [u8]> {
    if !crate::jdk_resources::is_virtual(fd_of(fdo)) {
        return None;
    }
    // SAFETY: address 指向调用方至少 len 字节的本地缓冲区（IOUtil 临时直接缓冲区 / 直接缓冲区）
    Some(unsafe { std::slice::from_raw_parts_mut(address as *mut u8, len.max(0) as usize) })
}

impl UnixFileDispatcherImpl {
    /// native `read0(FileDescriptor, long address, int len)`：read(2)。
    #[jvm_native]
    pub fn read0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        if let Some(buf) = virtual_buf(&fdo, address, len) {
            let n = crate::jdk_resources::virtual_read(fd_of(&fdo), buf);
            return Ok(if n == 0 { IOS_EOF as i32 } else { n as i32 });
        }
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = unsafe { libc::read(fd_of(&fdo), address as *mut libc::c_void, len as usize) };
        Ok(convert(n as i64, true)? as i32)
    }

    /// native `pread0(FileDescriptor, long address, int len, long position)`：pread(2)。
    #[jvm_native]
    pub fn pread0(fdo: FileDescriptor, address: i64, len: i32, position: i64) -> Result<i32> {
        if let Some(buf) = virtual_buf(&fdo, address, len) {
            let n = crate::jdk_resources::virtual_pread(fd_of(&fdo), buf, position);
            return Ok(if n == 0 { IOS_EOF as i32 } else { n as i32 });
        }
        // SAFETY: 同 read0
        let n = unsafe { libc::pread(fd_of(&fdo), address as *mut libc::c_void, len as usize, position as libc::off_t) };
        Ok(convert(n as i64, true)? as i32)
    }

    /// native `write0(FileDescriptor, long address, int len)`：write(2)。
    #[jvm_native]
    pub fn write0(fdo: FileDescriptor, address: i64, len: i32) -> Result<i32> {
        // SAFETY: address 指向至少 len 字节的本地缓冲区
        let n = unsafe { libc::write(fd_of(&fdo), address as *const libc::c_void, len as usize) };
        Ok(convert(n as i64, false)? as i32)
    }

    /// native `pwrite0(FileDescriptor, long address, int len, long position)`：pwrite(2)。
    #[jvm_native]
    pub fn pwrite0(fdo: FileDescriptor, address: i64, len: i32, position: i64) -> Result<i32> {
        // SAFETY: 同 write0
        let n = unsafe { libc::pwrite(fd_of(&fdo), address as *const libc::c_void, len as usize, position as libc::off_t) };
        Ok(convert(n as i64, false)? as i32)
    }

    /// native `seek0(FileDescriptor, long offset)`：offset < 0 取当前位置，否则定位到 offset。
    #[jvm_native]
    pub fn seek0(fdo: FileDescriptor, offset: i64) -> Result<i64> {
        let fd = fd_of(&fdo);
        if crate::jdk_resources::is_virtual(fd) {
            return Ok(if offset < 0 {
                crate::jdk_resources::virtual_position(fd)
            } else {
                crate::jdk_resources::virtual_seek(fd, offset)
            });
        }
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
    #[jvm_native]
    pub fn size0(fdo: FileDescriptor) -> Result<i64> {
        if let Some(st) = crate::jdk_resources::virtual_fstat(fd_of(&fdo)) {
            return Ok(st.st_size as i64);
        }
        // SAFETY: buf 为栈上 stat 结构
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        if unsafe { libc::fstat(fd_of(&fdo), &mut buf) } < 0 {
            return handle(-1, "Size failed");
        }
        Ok(buf.st_size as i64)
    }

    /// native `truncate0(FileDescriptor, long size)`：ftruncate(2)。
    #[jvm_native]
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
    #[jvm_native]
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

    /// native `map0(FileDescriptor, int prot, long position, long length, boolean isSync)`：mmap(2)，
    /// 返回映射地址。prot：MAP_RO → PROT_READ + MAP_SHARED，MAP_RW → 读写 + MAP_SHARED，
    /// MAP_PV → 读写 + MAP_PRIVATE。isSync 在 Linux 加 MAP_SYNC | MAP_SHARED_VALIDATE（不支持时
    /// IOException「map with mode MAP_SYNC unsupported」）；其余平台 Java 侧不会以 isSync 调用
    /// （JNI 为 InternalError「should never call …」，此处同文案报 IOException）。
    /// ENOMEM 抛 OutOfMemoryError("Map failed")，其余失败经 `handle`（"Map failed"）。
    #[jvm_native]
    pub fn map0(fdo: FileDescriptor, prot: i32, position: i64, length: i64, is_sync: bool) -> Result<i64> {
        let (protections, flags) = match prot {
            MAP_RO => (libc::PROT_READ, libc::MAP_SHARED),
            MAP_RW => (libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED),
            _ => (libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE),
        };
        let flags = if is_sync { flags | map_sync_flags()? } else { flags };
        // SAFETY: 新建映射（地址由内核选择），fd 为调用方持有的描述符
        let addr = unsafe {
            libc::mmap(std::ptr::null_mut(), length as libc::size_t, protections, flags, fd_of(&fdo), position as libc::off_t)
        };
        if addr == libc::MAP_FAILED {
            let err = errno();
            if is_sync && err == libc::ENOTSUP {
                return Err(io_exception("map with mode MAP_SYNC unsupported"));
            }
            if err == libc::ENOMEM {
                return Err(JvmError::out_of_memory("Map failed"));
            }
            return handle(-1, "Map failed");
        }
        Ok(addr as i64)
    }

    /// native `unmap0(long address, long length)`：munmap(2)，经 `handle`（"Unmap failed"）。
    #[jvm_native]
    pub fn unmap0(address: i64, length: i64) -> Result<i32> {
        // SAFETY: address / length 为 map0 返回的映射（Unmapper 持有，只解除一次）
        let r = unsafe { libc::munmap(address as *mut libc::c_void, length as libc::size_t) };
        Ok(handle(r as i64, "Unmap failed")? as i32)
    }
}

/// map0 的 isSync 附加标志：Linux 为 MAP_SYNC | MAP_SHARED_VALIDATE。
#[cfg(target_os = "linux")]
fn map_sync_flags() -> Result<i32> {
    Ok(libc::MAP_SYNC | libc::MAP_SHARED_VALIDATE)
}

/// 非 Linux：Java 侧不会以 isSync 调用 map0（JNI 为 InternalError，此处同文案报 IOException）。
#[cfg(not(target_os = "linux"))]
fn map_sync_flags() -> Result<i32> {
    Err(io_exception("should never call map on platform where MAP_SYNC is unimplemented"))
}
