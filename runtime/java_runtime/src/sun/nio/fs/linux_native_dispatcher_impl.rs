//! `sun/nio/fs/LinuxNativeDispatcher` 的 ACC_NATIVE（类 1，libnio `LinuxNativeDispatcher.c`）。
//!
//! 本类只在 Linux 的 JDK 中存在。包装方法（`setmntent` / `getmntent` 把 byte[] 拷进 NativeBuffer）按字节码
//! 翻译；本文件只承载 JNI 对应物：
//! - `init`：JNI 侧为 UnixMountEntry 的四个 byte[] 字段缓存字段 ID，并 `dlsym` 取 `copy_file_range`
//!   函数指针。本模型按名访问字段、原生二进制直接链接 libc，无状态需初始化，也没有能力位要设置
//!   （返回 void；xattr / openat 等能力位属 `UnixNativeDispatcher.init`）。
//! - `setmntent0` / `getmntent0` / `endmntent`：挂载表枚举（LinuxFileSystem.getMountEntries）；
//! - `posix_fadvise`：直接返回 posix_fadvise 的结果码（不抛异常，与 JNI 同）；
//! - `directCopy0`：Files.copy 的内核内拷贝（先 copy_file_range，不支持时退回 sendfile）。

use crate::prelude::*;
use super::linux_native_dispatcher::LinuxNativeDispatcher;
use super::unix_mount_entry::UnixMountEntry;
use super::unix_native_dispatcher_impl::unix_exception;
#[cfg(target_os = "linux")]
use super::unix_native_dispatcher_impl::{errno, restartable};

/// `sun/nio/ch/IOStatus` 的返回码（directCopy0 的约定）
const IOS_UNAVAILABLE: i32 = -2;
const IOS_UNSUPPORTED_CASE: i32 = -6;

/// NUL 结尾 C 串 → Java byte[]（不含 NUL）
#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn c_bytes(p: *const libc::c_char) -> JArray<i8> {
    // SAFETY: p 指向 getmntent_r 写入调用方缓冲的 NUL 结尾串
    let s = unsafe { std::ffi::CStr::from_ptr(p) };
    JArray::from(s.to_bytes().iter().map(|b| *b as i8).collect::<Vec<i8>>())
}

/// JNI `throwUnixException(env, ECANCELED)` 的取消检查：cancel 地址非 0 且所指 int 非 0
#[cfg(target_os = "linux")]
fn cancelled(cancel: i64) -> bool {
    // SAFETY: cancel 非 0 时为调用方（Files.copy 的可取消任务）持有的直接内存 int
    cancel != 0 && unsafe { std::ptr::read_volatile(cancel as *const i32) } != 0
}

impl LinuxNativeDispatcher {
    /// native `init()`：见模块注释，无状态需初始化。
    #[jvm_native]
    pub fn init() -> Result<()> {
        Ok(())
    }

    /// native `setmntent0(long pathAddress, long modeAddress)`：setmntent(3)，失败抛 UnixException(errno)。
    #[jvm_native]
    pub fn setmntent0(path: i64, mode: i64) -> Result<i64> {
        set_mount_table(path, mode)
    }

    /// native `getmntent0(long fp, UnixMountEntry entry, long buffer, int bufLen)`：getmntent_r(3)，
    /// 读到条目时填 name / dir / fstype / opts 并返回 0，表尾返回 -1。
    #[jvm_native]
    pub fn getmntent0(fp: i64, entry: UnixMountEntry, buffer: i64, buf_len: i32) -> Result<i32> {
        next_mount_entry(fp, &entry, buffer, buf_len)
    }

    /// native `endmntent(long stream)`：endmntent(3)（恒返回 1，JNI 不检查结果）。
    #[jvm_native]
    pub fn endmntent(stream: i64) -> Result<()> {
        end_mount_table(stream);
        Ok(())
    }

    /// native `posix_fadvise(int fd, long offset, long len, int advice)`：直接返回 posix_fadvise 结果码。
    #[jvm_native]
    pub fn posix_fadvise(fd: i32, offset: i64, len: i64, advice: i32) -> Result<i32> {
        Ok(fadvise(fd, offset, len, advice))
    }

    /// native `directCopy0(int dst, int src, long cancelAddress)`：成功返回 0；sendfile 会阻塞返回
    /// IOS_UNAVAILABLE、参数不受支持返回 IOS_UNSUPPORTED_CASE；其余失败抛异常。
    #[jvm_native]
    pub fn directCopy0(dst: i32, src: i32, cancel: i64) -> Result<i32> {
        direct_copy(dst, src, cancel)
    }
}

#[cfg(target_os = "linux")]
fn set_mount_table(path: i64, mode: i64) -> Result<i64> {
    // SAFETY: path / mode 指向 NativeBuffer 中 NUL 结尾的串
    let fp = unsafe { libc::setmntent(path as *const libc::c_char, mode as *const libc::c_char) };
    if fp.is_null() {
        return Err(unix_exception(errno()));
    }
    Ok(fp as i64)
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn next_mount_entry(fp: i64, entry: &UnixMountEntry, buffer: i64, buf_len: i32) -> Result<i32> {
    // SAFETY: mntent 为纯数据结构，零值合法
    let mut ent: libc::mntent = unsafe { std::mem::zeroed() };
    // SAFETY: fp 为 setmntent 返回的流；buffer 为调用方 NativeBuffer，长度 buf_len
    let m = unsafe { libc::getmntent_r(fp as *mut libc::FILE, &mut ent, buffer as *mut libc::c_char, buf_len) };
    if m.is_null() {
        return Ok(-1);
    }
    entry.__set_name(c_bytes(ent.mnt_fsname));
    entry.__set_dir(c_bytes(ent.mnt_dir));
    entry.__set_fstype(c_bytes(ent.mnt_type));
    entry.__set_opts(c_bytes(ent.mnt_opts));
    Ok(0)
}

#[cfg(target_os = "linux")]
fn end_mount_table(stream: i64) {
    // SAFETY: stream 为 setmntent 返回的流
    unsafe { libc::endmntent(stream as *mut libc::FILE) };
}

#[cfg(target_os = "linux")]
fn fadvise(fd: i32, offset: i64, len: i64, advice: i32) -> i32 {
    // SAFETY: 纯系统调用，fd 为调用方持有的描述符
    unsafe { libc::posix_fadvise(fd, offset as libc::off_t, len as libc::off_t, advice) }
}

/// JNI `directCopy0`：copy_file_range 循环到 0（EINVAL / ENOSYS / EXDEV 退回 sendfile，其余失败抛
/// IOException）；sendfile 循环到 0（EAGAIN / EINVAL / ENOSYS 返回状态码，其余抛 UnixException）。
/// 每轮后检查取消标记（ECANCELED）。有取消标记时每轮 1 MB，给取消留出机会。
#[cfg(target_os = "linux")]
fn direct_copy(dst: i32, src: i32, cancel: i64) -> Result<i32> {
    let count: usize = if cancel != 0 { 1_048_576 } else { 0x7fff_f000 };
    let mut fallback = false;
    loop {
        // SAFETY: src / dst 为调用方持有的描述符，偏移取文件当前位置（NULL）
        let n = restartable(|| unsafe {
            libc::copy_file_range(src, std::ptr::null_mut(), dst, std::ptr::null_mut(), count, 0) as i32
        });
        match n {
            Err(libc::EINVAL | libc::ENOSYS | libc::EXDEV) => fallback = true,
            Err(e) => return Err(copy_failed(e)),
            Ok(_) => {}
        }
        if cancelled(cancel) {
            return Err(unix_exception(libc::ECANCELED));
        }
        match n {
            Ok(0) => return Ok(0),
            Ok(_) => {}
            Err(_) => break,
        }
    }
    debug_assert!(fallback);
    loop {
        // SAFETY: 同上
        let n = restartable(|| unsafe { libc::sendfile(dst, src, std::ptr::null_mut(), count) as i32 });
        match n {
            Err(libc::EAGAIN) => return Ok(IOS_UNAVAILABLE),
            Err(libc::EINVAL | libc::ENOSYS) => return Ok(IOS_UNSUPPORTED_CASE),
            Err(e) => return Err(unix_exception(e)),
            Ok(_) => {}
        }
        if cancelled(cancel) {
            return Err(unix_exception(libc::ECANCELED));
        }
        if n == Ok(0) {
            return Ok(0);
        }
    }
}

/// `JNU_ThrowIOExceptionWithLastError(env, "Copy failed")`
#[cfg(target_os = "linux")]
fn copy_failed(err: i32) -> JvmError {
    let s = std::io::Error::from_raw_os_error(err).to_string();
    let msg = s.split(" (os error").next().unwrap_or("Copy failed").to_owned();
    match crate::java::io::IOException::new_str(String::from(msg)) {
        Ok(x) => JvmError::from(x),
        Err(e) => e,
    }
}

// 非 Linux 宿主：本类只在 Linux 的 JDK 中存在，下列落点不会被翻译体调用，只为保持文件可编译。
#[cfg(not(target_os = "linux"))]
fn set_mount_table(_path: i64, _mode: i64) -> Result<i64> {
    Err(unix_exception(libc::ENOTSUP))
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn next_mount_entry(_fp: i64, _entry: &UnixMountEntry, _buffer: i64, _buf_len: i32) -> Result<i32> {
    Err(unix_exception(libc::ENOTSUP))
}

#[cfg(not(target_os = "linux"))]
fn end_mount_table(_stream: i64) {}

#[cfg(not(target_os = "linux"))]
fn fadvise(_fd: i32, _offset: i64, _len: i64, _advice: i32) -> i32 {
    libc::ENOTSUP
}

#[cfg(not(target_os = "linux"))]
fn direct_copy(_dst: i32, _src: i32, _cancel: i64) -> Result<i32> {
    Ok(IOS_UNSUPPORTED_CASE)
}
