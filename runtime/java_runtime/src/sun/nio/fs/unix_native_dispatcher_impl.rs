//! `sun/nio/fs/UnixNativeDispatcher` 的 ACC_NATIVE（类 1，docs/reference/handwritten-boundary.md）。
//!
//! 包装方法（`open` / `stat` / `unlink` …：UnixPath → NativeBuffer → `*0(address)`）按字节码翻译；
//! 本文件只承载 JNI 对应物（libnio `UnixNativeDispatcher.c`）：路径参数是 NativeBuffer 的绝对地址
//! （以 NUL 结尾的 C 串，`Unsafe.allocateMemory` 所得），失败时按 JNI 同款抛 `UnixException(errno)`
//! 或返回 errno。本类 49 个 native 全部承载（续见 `unix_native_dispatcher_ext.rs`）。

use crate::prelude::*;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_exception::UnixException;
use super::unix_file_attributes::UnixFileAttributes;

// UnixNativeDispatcher 的能力位（JDK 常量值）
const SUPPORTS_OPENAT: i32 = 1 << 1;
const SUPPORTS_FUTIMES: i32 = 1 << 2;
const SUPPORTS_FUTIMENS: i32 = 1 << 3;
const SUPPORTS_LUTIMES: i32 = 1 << 4;
const SUPPORTS_XATTR: i32 = 1 << 5;
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
    /// native `init()`：能力位图，与 JDK 21 libnio `UnixNativeDispatcher.c` 在两平台的探测结果一致——
    /// 本类全部 native 均已承载，能力位不再受实现面限制：
    /// - Linux：OPENAT（openat / fstatat / unlinkat / renameat / futimesat / fdopendir 齐备，
    ///   `Files.newDirectoryStream` 得 SecureDirectoryStream）、FUTIMES、FUTIMENS、LUTIMES、XATTR；
    /// - macOS：无 futimesat，故不报 OPENAT；另报 BIRTHTIME（stat 提供 st_birthtime）。
    ///
    /// 不报 LUTIMES 时，不跟随链接的 setTimes 退回 `openForAttributeAccess(false)`（O_NOFOLLOW 打开链接
    /// 本身）得 ELOOP，与 JDK 行为不符（边界用例 TestSymlinkNoFollowAttrs）。
    #[jvm_native]
    pub fn init() -> Result<i32> {
        let common = SUPPORTS_FUTIMES | SUPPORTS_FUTIMENS | SUPPORTS_LUTIMES | SUPPORTS_XATTR;
        Ok(if cfg!(target_os = "macos") { common | SUPPORTS_BIRTHTIME } else { common | SUPPORTS_OPENAT })
    }

    /// native `openat0(int dfd, long path, int flags, int mode)`：openat(2)。
    #[jvm_native]
    pub fn openat0(dfd: i32, path_address: i64, flags: i32, mode: i32) -> Result<i32> {
        // SAFETY: path_address 指向 NUL 结尾路径；dfd 为调用方持有的目录描述符
        restartable(|| unsafe { libc::openat(dfd, c_path(path_address), flags, mode as libc::c_uint) })
            .map_err(unix_exception)
    }

    /// native `rewind(long stream)`：rewind(3)；rewind 无返回值，以 ferror 判错（JNI 同款先清 errno）。
    #[jvm_native]
    pub fn rewind(stream: i64) -> Result<()> {
        let fp = stream as *mut libc::FILE;
        // SAFETY: stream 为 setmntent / fopen 返回的 FILE*
        unsafe {
            *errno_location() = 0;
            libc::rewind(fp);
            let saved = errno();
            if libc::ferror(fp) != 0 {
                return Err(unix_exception(saved));
            }
        }
        Ok(())
    }

    /// native `getlinelen(long stream)`：getline(3) 读一行，返回该行字节数（含换行）；流已到尾返回 -1
    /// （JNI 同款先判 feof：末行无换行时读到该行也返回 -1）。LinuxFileSystem.getMountEntries 用它
    /// 求 /proc/mounts 最长行定 getmntent 缓冲。
    #[jvm_native]
    pub fn getlinelen(stream: i64) -> Result<i32> {
        let fp = stream as *mut libc::FILE;
        let mut line: *mut libc::c_char = std::ptr::null_mut();
        let mut size: libc::size_t = 0;
        // SAFETY: fp 为有效 FILE*；getline 分配的 line 由本函数 free（无论成败，man page 约定）
        let (res, saved, eof) = unsafe {
            let res = libc::getline(&mut line, &mut size, fp);
            let saved = errno();
            if !line.is_null() {
                libc::free(line as *mut libc::c_void);
            }
            (res, saved, libc::feof(fp) != 0)
        };
        if eof {
            return Ok(-1);
        }
        if res == -1 {
            return Err(unix_exception(saved));
        }
        if res > i32::MAX as isize {
            return Err(unix_exception(libc::EOVERFLOW));
        }
        Ok(res as i32)
    }

    /// native `link0(long existing, long newfile)`：link(2)。
    #[jvm_native]
    pub fn link0(existing: i64, newfile: i64) -> Result<()> {
        // SAFETY: 两个地址均指向 NUL 结尾路径
        restartable(|| unsafe { libc::link(c_path(existing), c_path(newfile)) }).map(drop).map_err(unix_exception)
    }

    /// native `unlinkat0(int dfd, long path, int flag)`：unlinkat(2)。
    #[jvm_native]
    pub fn unlinkat0(dfd: i32, path_address: i64, flag: i32) -> Result<()> {
        // SAFETY: 同 openat0
        if unsafe { libc::unlinkat(dfd, c_path(path_address), flag) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `mknod0(long path, int mode, long dev)`：mknod(2)。
    #[jvm_native]
    pub fn mknod0(path_address: i64, mode: i32, dev: i64) -> Result<()> {
        // SAFETY: 同 open0
        restartable(|| unsafe { libc::mknod(c_path(path_address), mode as libc::mode_t, dev as libc::dev_t) })
            .map(drop)
            .map_err(unix_exception)
    }

    /// native `rename0(long from, long to)`：rename(2)。
    #[jvm_native]
    pub fn rename0(from: i64, to: i64) -> Result<()> {
        // SAFETY: 两个地址均指向 NUL 结尾路径
        if unsafe { libc::rename(c_path(from), c_path(to)) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `renameat0(int fromfd, long from, int tofd, long to)`：renameat(2)。
    #[jvm_native]
    pub fn renameat0(fromfd: i32, from: i64, tofd: i32, to: i64) -> Result<()> {
        // SAFETY: 两个地址均指向 NUL 结尾路径；两 fd 为调用方持有的目录描述符
        if unsafe { libc::renameat(fromfd, c_path(from), tofd, c_path(to)) } == -1 {
            return Err(unix_exception(errno()));
        }
        Ok(())
    }

    /// native `fstatat0(int dfd, long path, int flag, UnixFileAttributes)`：fstatat(2) 后填 st_* 字段。
    #[jvm_native]
    pub fn fstatat0(dfd: i32, path_address: i64, flag: i32, attrs: UnixFileAttributes) -> Result<()> {
        // SAFETY: 同 openat0；buf 为栈上 stat 结构
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        restartable(|| unsafe { libc::fstatat(dfd, c_path(path_address), &mut buf, flag) }).map_err(unix_exception)?;
        fill_stat(&attrs, &buf);
        Ok(())
    }

    /// native `read0(int fd, long address, int nbytes)`：read(2) 到直接内存，返回读到的字节数。
    #[jvm_native]
    pub fn read0(fd: i32, address: i64, nbytes: i32) -> Result<i32> {
        // SAFETY: address 为调用方 NativeBuffer，至少 nbytes 字节可写
        restartable(|| unsafe { libc::read(fd, address as *mut libc::c_void, nbytes as libc::size_t) as i32 })
            .map_err(unix_exception)
    }

    /// native `write0(int fd, long address, int nbytes)`：write(2) 自直接内存，返回写出的字节数。
    #[jvm_native]
    pub fn write0(fd: i32, address: i64, nbytes: i32) -> Result<i32> {
        // SAFETY: address 为调用方 NativeBuffer，至少 nbytes 字节可读
        restartable(|| unsafe { libc::write(fd, address as *const libc::c_void, nbytes as libc::size_t) as i32 })
            .map_err(unix_exception)
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
