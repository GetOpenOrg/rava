//! `sun/nio/fs/UnixNativeDispatcher` 的 ACC_NATIVE（类 1，docs/reference/handwritten-boundary.md）。
//!
//! 包装方法（`open` / `stat` / `unlink` …：UnixPath → NativeBuffer → `*0(address)`）按字节码翻译；
//! 本文件只承载 JNI 对应物（libnio `UnixNativeDispatcher.c`）：路径参数是 NativeBuffer 的绝对地址
//! （以 NUL 结尾的 C 串，`Unsafe.allocateMemory` 所得），失败时按 JNI 同款抛 `UnixException(errno)`
//! 或返回 errno。未实现的 native 保持 panic 存根。

use crate::prelude::*;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_exception::UnixException;
use super::unix_file_attributes::UnixFileAttributes;

// UnixNativeDispatcher 的能力位（JDK 常量值；init 只报告本运行时实际承载的面）
const SUPPORTS_FUTIMES: i32 = 1 << 2;
const SUPPORTS_FUTIMENS: i32 = 1 << 3;
const SUPPORTS_LUTIMES: i32 = 1 << 4;
const SUPPORTS_BIRTHTIME: i32 = 1 << 16;

pub(super) fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

pub(super) fn unix_exception(err: i32) -> JvmError {
    match UnixException::new_i(err) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// errno 存放位置（平台差异）
#[cfg(target_os = "macos")]
unsafe fn errno_location() -> *mut i32 {
    libc::__error()
}

#[cfg(not(target_os = "macos"))]
unsafe fn errno_location() -> *mut i32 {
    libc::__errno_location()
}

/// 地址 → C 串指针（NativeBuffer 由 copyToNativeBuffer 写入并以 NUL 结尾）。
fn c_path(address: i64) -> *const libc::c_char {
    address as *const libc::c_char
}

/// JNI `RESTARTABLE`：返回 -1 且 errno == EINTR 时重试；其它失败返回 errno。
pub(super) fn restartable(mut f: impl FnMut() -> i32) -> std::result::Result<i32, i32> {
    loop {
        let r = f();
        if r != -1 {
            return Ok(r);
        }
        let err = errno();
        if err != libc::EINTR {
            return Err(err);
        }
    }
}

impl UnixNativeDispatcher {
    /// native `init()`：能力位图（libnio `UnixNativeDispatcher.c` 按平台探测 openat / futimes / futimens /
    /// lutimes / xattr / birthtime）。futimes0 / futimens0 / lutimes0 已承载（libc 在 Linux 与 macOS 都提供），
    /// 两平台同 JDK 报告 FUTIMES / FUTIMENS / LUTIMES——不报 LUTIMES 时，不跟随链接的 setTimes 退回
    /// `openForAttributeAccess(false)`（O_NOFOLLOW 打开符号链接本身）得 ELOOP；birthtime 只有 macOS stat 提供；
    /// *at 系列与 xattr 的 native 未承载，不报告（报告即把调用引到未实现的 native）。
    #[jvm_native]
    pub fn init() -> Result<i32> {
        let times = SUPPORTS_FUTIMES | SUPPORTS_FUTIMENS | SUPPORTS_LUTIMES;
        Ok(if cfg!(target_os = "macos") { times | SUPPORTS_BIRTHTIME } else { times })
    }

    /// native `open0(long path, int flags, int mode)`：open(2)。
    #[jvm_native]
    pub fn open0(path_address: i64, flags: i32, mode: i32) -> Result<i32> {
        // SAFETY: path_address 指向 NativeBuffer 中以 NUL 结尾的路径串
        restartable(|| unsafe { libc::open(c_path(path_address), flags, mode as libc::c_uint) })
            .map_err(unix_exception)
    }

    /// native `close0(int fd)`：close(2)；EINTR 视为已关闭（JNI 同款）。
    #[jvm_native]
    pub fn close0(fd: i32) -> Result<()> {
        // SAFETY: fd 为调用方持有的文件描述符
        if unsafe { libc::close(fd) } == -1 {
            let err = errno();
            if err != libc::EINTR {
                return Err(unix_exception(err));
            }
        }
        Ok(())
    }

    /// native `stat0(long path, UnixFileAttributes attrs)`：stat(2)，返回 errno（0 = 成功）。
    #[jvm_native]
    pub fn stat0(path_address: i64, attrs: UnixFileAttributes) -> Result<i32> {
        // SAFETY: 同 open0；buf 为栈上 stat 结构
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        if let Err(err) = restartable(|| unsafe { libc::stat(c_path(path_address), &mut buf) }) {
            return Ok(err);
        }
        fill_stat(&attrs, &buf);
        Ok(0)
    }

    /// native `lstat0(long path, UnixFileAttributes attrs)`：lstat(2)，失败抛 UnixException。
    #[jvm_native]
    pub fn lstat0(path_address: i64, attrs: UnixFileAttributes) -> Result<()> {
        // SAFETY: 同 stat0
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        restartable(|| unsafe { libc::lstat(c_path(path_address), &mut buf) }).map_err(unix_exception)?;
        fill_stat(&attrs, &buf);
        Ok(())
    }

    /// native `unlink0(long path)`：unlink(2)。
    #[jvm_native]
    pub fn unlink0(path_address: i64) -> Result<()> {
        // SAFETY: 同 open0
        if unsafe { libc::unlink(c_path(path_address)) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `rmdir0(long path)`：rmdir(2)。
    #[jvm_native]
    pub fn rmdir0(path_address: i64) -> Result<()> {
        // SAFETY: 同 open0
        if unsafe { libc::rmdir(c_path(path_address)) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `mkdir0(long path, int mode)`：mkdir(2)。
    #[jvm_native]
    pub fn mkdir0(path_address: i64, mode: i32) -> Result<()> {
        // SAFETY: 同 open0
        if unsafe { libc::mkdir(c_path(path_address), mode as libc::mode_t) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `access0(long path, int amode)`：access(2)，返回 errno（0 = 允许）。
    #[jvm_native]
    pub fn access0(path_address: i64, amode: i32) -> Result<i32> {
        // SAFETY: 同 open0
        Ok(restartable(|| unsafe { libc::access(c_path(path_address), amode) }).err().unwrap_or(0))
    }

    /// native `getcwd()`：当前工作目录（字节形态）；失败抛 UnixException(errno)。
    #[jvm_native]
    pub fn getcwd() -> Result<JArray<i8>> {
        let mut buf = vec![0u8; libc::PATH_MAX as usize + 1];
        // SAFETY: buf 可写 buf.len() 字节
        let p = unsafe { libc::getcwd(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
        if p.is_null() {
            return Err(unix_exception(errno()));
        }
        let n = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        Ok(JArray::from(buf[..n].iter().map(|&b| b as i8).collect::<Vec<i8>>()))
    }

    /// native `dup(int)`：dup(2)。
    #[jvm_native]
    pub fn dup(fd: i32) -> Result<i32> {
        // SAFETY: dup 只作用于 fd
        restartable(|| unsafe { libc::dup(fd) }).map_err(unix_exception)
    }

    /// native `opendir0(long path)`：opendir(3)，返回 DIR* 地址。
    #[jvm_native]
    pub fn opendir0(path_address: i64) -> Result<i64> {
        // SAFETY: 同 open0
        let d = unsafe { libc::opendir(c_path(path_address)) };
        if d.is_null() {
            return Err(unix_exception(errno()));
        }
        Ok(d as i64)
    }

    /// native `fdopendir(int)`：fdopendir(3)，返回 DIR* 地址。
    #[jvm_native]
    pub fn fdopendir(dfd: i32) -> Result<i64> {
        // SAFETY: dfd 为调用方持有的目录描述符
        let d = unsafe { libc::fdopendir(dfd) };
        if d.is_null() {
            return Err(unix_exception(errno()));
        }
        Ok(d as i64)
    }

    /// native `closedir(long)`：closedir(3)；EINTR 视为已关闭（JNI 同款）。
    #[jvm_native]
    pub fn closedir(dir: i64) -> Result<()> {
        // SAFETY: dir 为 opendir0 / fdopendir 返回的 DIR*
        if unsafe { libc::closedir(dir as *mut libc::DIR) } == -1 {
            let err = errno();
            if err != libc::EINTR {
                return Err(unix_exception(err));
            }
        }
        Ok(())
    }

    /// native `readdir0(long)`：下一目录项名（字节形态）；目录读完返回 null。
    #[jvm_native]
    pub fn readdir0(dir: i64) -> Result<JArray<i8>> {
        // SAFETY: dir 为有效 DIR*；清 errno 以区分读完与出错
        unsafe {
            *errno_location() = 0;
            let ent = libc::readdir(dir as *mut libc::DIR);
            if ent.is_null() {
                let err = errno();
                if err != 0 {
                    return Err(unix_exception(err));
                }
                return Ok(JArray::default());
            }
            let name = std::ffi::CStr::from_ptr((*ent).d_name.as_ptr());
            Ok(JArray::from(name.to_bytes().iter().map(|&b| b as i8).collect::<Vec<i8>>()))
        }
    }

    /// native `strerror(int)`：平台错误字符串（jnu 编码字节）。
    #[jvm_native]
    pub fn strerror(err: i32) -> Result<JArray<i8>> {
        let s = std::io::Error::from_raw_os_error(err).to_string();
        let s = s.split(" (os error").next().unwrap_or("").to_owned();
        Ok(JArray::from(s.into_bytes().into_iter().map(|b| b as i8).collect::<Vec<i8>>()))
    }
}

/// stat 结构 → UnixFileAttributes.st_* 字段（JNI `prepAttributes` 同款）。
pub(super) fn fill_stat(attrs: &UnixFileAttributes, st: &libc::stat) {
    attrs.__set_st_mode(st.st_mode as i32);
    attrs.__set_st_ino(st.st_ino as i64);
    attrs.__set_st_dev(st.st_dev as i64);
    attrs.__set_st_rdev(st.st_rdev as i64);
    attrs.__set_st_nlink(st.st_nlink as i32);
    attrs.__set_st_uid(st.st_uid as i32);
    attrs.__set_st_gid(st.st_gid as i32);
    attrs.__set_st_size(st.st_size as i64);
    attrs.__set_st_atime_sec(st.st_atime as i64);
    attrs.__set_st_atime_nsec(st.st_atime_nsec as i64);
    attrs.__set_st_mtime_sec(st.st_mtime as i64);
    attrs.__set_st_mtime_nsec(st.st_mtime_nsec as i64);
    attrs.__set_st_ctime_sec(st.st_ctime as i64);
    attrs.__set_st_ctime_nsec(st.st_ctime_nsec as i64);
    #[cfg(target_os = "macos")]
    {
        attrs.__set_st_birthtime_sec(st.st_birthtime as i64);
        attrs.__set_st_birthtime_nsec(st.st_birthtime_nsec as i64);
    }
}
