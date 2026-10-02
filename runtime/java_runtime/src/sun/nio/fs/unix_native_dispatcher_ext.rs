//! `sun/nio/fs/UnixNativeDispatcher` 手写伴生（续）：属性 / 目录 / 链接族 native（ACC_NATIVE 准入）。
//!
//! 对标 JDK 21 `UnixNativeDispatcher.c`：路径实参是 `copyToNativeBuffer` 写入的 NUL 结尾路径
//! （直接内存地址）；系统调用经 `RESTARTABLE`（EINTR 重试），失败按 errno 抛 `UnixException`。
//! 时间实参单位随调用：utimes / futimes / lutimes 为微秒（`struct timeval`），futimens 为纳秒
//! （`struct timespec`），均按 C 整除 / 取余拆分（向零截断）。

use crate::prelude::*;
use super::unix_file_attributes::UnixFileAttributes;
use super::unix_file_store_attributes::UnixFileStoreAttributes;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_native_dispatcher_impl::{errno, fill_stat, restartable, unix_exception};

/// NativeBuffer 地址 → C 路径指针。
fn cpath(address: i64) -> *const libc::c_char {
    address as *const libc::c_char
}

/// 系统调用结果 → `Result<()>`（EINTR 重试，失败抛 UnixException）。
fn check(call: impl FnMut() -> i32) -> Result<()> {
    restartable(call).map(|_| ()).map_err(unix_exception)
}

/// 微秒 → `timeval[2]`（访问时间、修改时间）。
fn timevals(access_us: i64, modify_us: i64) -> [libc::timeval; 2] {
    let tv = |t: i64| libc::timeval {
        tv_sec: (t / 1_000_000) as libc::time_t,
        tv_usec: (t % 1_000_000) as libc::suseconds_t,
    };
    [tv(access_us), tv(modify_us)]
}

/// NUL 结尾的 C 缓冲前 n 字节 → Java byte[]。
fn bytes_of(buf: &[libc::c_char]) -> JArray<i8> {
    JArray::from(buf.iter().map(|c| *c as i8).collect::<Vec<i8>>())
}

impl UnixNativeDispatcher {
    /// `symlink0(long target, long link)`：symlink(2)（name1 为链接内容，name2 为新建链接）。
    #[jvm_native]
    pub fn symlink0(name1: i64, name2: i64) -> Result<()> {
        // SAFETY: 两个地址均指向 NUL 结尾路径
        check(|| unsafe { libc::symlink(cpath(name1), cpath(name2)) })
    }

    /// `readlink0(long path)`：readlink(2)，缓冲 `PATH_MAX + 1`；结果恰好填满缓冲时截掉末字节
    /// （JDK 同口径）。
    #[jvm_native]
    pub fn readlink0(path: i64) -> Result<JArray<i8>> {
        let mut target = vec![0 as libc::c_char; libc::PATH_MAX as usize + 1];
        // SAFETY: target 长度即传入的缓冲大小
        let n = unsafe { libc::readlink(cpath(path), target.as_mut_ptr(), target.len()) };
        if n == -1 {
            return Err(unix_exception(errno()));
        }
        let n = if n as usize == target.len() { n as usize - 1 } else { n as usize };
        Ok(bytes_of(&target[..n]))
    }

    /// `realpath0(long path)`：realpath(3)，返回规范绝对路径字节。
    #[jvm_native]
    pub fn realpath0(path: i64) -> Result<JArray<i8>> {
        let mut resolved = vec![0 as libc::c_char; libc::PATH_MAX as usize + 1];
        // SAFETY: resolved 至少 PATH_MAX 字节（realpath 的缓冲约定）
        if unsafe { libc::realpath(cpath(path), resolved.as_mut_ptr()) }.is_null() {
            return Err(unix_exception(errno()));
        }
        // SAFETY: 成功时 resolved 为 NUL 结尾串
        let len = unsafe { std::ffi::CStr::from_ptr(resolved.as_ptr()) }.to_bytes().len();
        Ok(bytes_of(&resolved[..len]))
    }

    /// `fstat0(int fd, UnixFileAttributes)`：fstat(2) 后填 st_* 字段。
    #[jvm_native]
    pub fn fstat0(fd: i32, attrs: UnixFileAttributes) -> Result<()> {
        // SAFETY: stat 为纯数据结构，零值合法；fd 归调用方所有
        let mut buf: libc::stat = unsafe { std::mem::zeroed() };
        check(|| unsafe { libc::fstat(fd, &mut buf) })?;
        fill_stat(&attrs, &buf);
        Ok(())
    }

    /// `chmod0(long path, int mode)`：chmod(2)。
    #[jvm_native]
    pub fn chmod0(path: i64, mode: i32) -> Result<()> {
        // SAFETY: path 指向 NUL 结尾路径
        check(|| unsafe { libc::chmod(cpath(path), mode as libc::mode_t) })
    }

    /// `fchmod0(int fd, int mode)`：fchmod(2)。
    #[jvm_native]
    pub fn fchmod0(fd: i32, mode: i32) -> Result<()> {
        // SAFETY: 只作用于调用方持有的 fd
        check(|| unsafe { libc::fchmod(fd, mode as libc::mode_t) })
    }

    /// `chown0(long path, int uid, int gid)`：chown(2)。
    #[jvm_native]
    pub fn chown0(path: i64, uid: i32, gid: i32) -> Result<()> {
        // SAFETY: path 指向 NUL 结尾路径
        check(|| unsafe { libc::chown(cpath(path), uid as libc::uid_t, gid as libc::gid_t) })
    }

    /// `lchown0(long path, int uid, int gid)`：lchown(2)（不跟随符号链接）。
    #[jvm_native]
    pub fn lchown0(path: i64, uid: i32, gid: i32) -> Result<()> {
        // SAFETY: path 指向 NUL 结尾路径
        check(|| unsafe { libc::lchown(cpath(path), uid as libc::uid_t, gid as libc::gid_t) })
    }

    /// `fchown0(int fd, int uid, int gid)`：fchown(2)。
    #[jvm_native]
    pub fn fchown0(fd: i32, uid: i32, gid: i32) -> Result<()> {
        // SAFETY: 只作用于调用方持有的 fd
        check(|| unsafe { libc::fchown(fd, uid as libc::uid_t, gid as libc::gid_t) })
    }

    /// `utimes0(long path, long accessUs, long modifyUs)`：utimes(2)，微秒。
    #[jvm_native]
    pub fn utimes0(path: i64, access: i64, modify: i64) -> Result<()> {
        let times = timevals(access, modify);
        // SAFETY: times 为两元素 timeval 数组
        check(|| unsafe { libc::utimes(cpath(path), times.as_ptr()) })
    }

    /// `futimes0(int fd, long accessUs, long modifyUs)`：futimes(2)，微秒。
    #[jvm_native]
    pub fn futimes0(fd: i32, access: i64, modify: i64) -> Result<()> {
        let times = timevals(access, modify);
        // SAFETY: times 为两元素 timeval 数组
        check(|| unsafe { libc::futimes(fd, times.as_ptr()) })
    }

    /// `lutimes0(long path, long accessUs, long modifyUs)`：lutimes(3)，微秒（不跟随符号链接）。
    #[jvm_native]
    pub fn lutimes0(path: i64, access: i64, modify: i64) -> Result<()> {
        let times = timevals(access, modify);
        // SAFETY: times 为两元素 timeval 数组
        check(|| unsafe { libc::lutimes(cpath(path), times.as_ptr()) })
    }

    /// `futimens0(int fd, long accessNs, long modifyNs)`：futimens(2)，纳秒。
    #[jvm_native]
    pub fn futimens0(fd: i32, access: i64, modify: i64) -> Result<()> {
        let ts = |t: i64| libc::timespec {
            tv_sec: (t / 1_000_000_000) as libc::time_t,
            tv_nsec: (t % 1_000_000_000) as _,
        };
        let times = [ts(access), ts(modify)];
        // SAFETY: times 为两元素 timespec 数组
        check(|| unsafe { libc::futimens(fd, times.as_ptr()) })
    }

    /// `statvfs0(long path, UnixFileStoreAttributes)`：文件存储容量。JDK 21 在 macOS 走 statfs(2)
    /// （块大小取 `f_bsize`），其余平台走 statvfs(3)（取 `f_frsize`）。
    #[jvm_native]
    pub fn statvfs0(path: i64, attrs: UnixFileStoreAttributes) -> Result<()> {
        #[cfg(target_os = "macos")]
        let (frsize, blocks, bfree, bavail) = {
            // SAFETY: statfs 为纯数据结构，零值合法
            let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
            // SAFETY: path 指向 NUL 结尾路径，buf 可写
            check(|| unsafe { libc::statfs(cpath(path), &mut buf) })?;
            (buf.f_bsize as i64, buf.f_blocks as i64, buf.f_bfree as i64, buf.f_bavail as i64)
        };
        #[cfg(not(target_os = "macos"))]
        let (frsize, blocks, bfree, bavail) = {
            // SAFETY: statvfs 为纯数据结构，零值合法
            let mut buf: libc::statvfs = unsafe { std::mem::zeroed() };
            // SAFETY: path 指向 NUL 结尾路径，buf 可写
            check(|| unsafe { libc::statvfs(cpath(path), &mut buf) })?;
            (buf.f_frsize as i64, buf.f_blocks as i64, buf.f_bfree as i64, buf.f_bavail as i64)
        };
        attrs.__set_f_frsize(frsize);
        attrs.__set_f_blocks(blocks);
        attrs.__set_f_bfree(bfree);
        attrs.__set_f_bavail(bavail);
        Ok(())
    }

    /// `getpwuid(int uid)`：getpwuid_r(3) 取用户名字节。缓冲取 `_SC_GETPW_R_SIZE_MAX`（不可得时
    /// 1024，JDK `ENT_BUF_SIZE`）；未找到或名字为空时抛 UnixException（errno 缺省 ENOENT）。
    #[jvm_native]
    pub fn getpwuid(uid: i32) -> Result<JArray<i8>> {
        // SAFETY: sysconf 无副作用
        let max = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
        let mut buf = vec![0 as libc::c_char; if max > 0 { max as usize } else { 1024 }];
        // SAFETY: passwd 为纯数据结构，零值合法
        let mut pwent: libc::passwd = unsafe { std::mem::zeroed() };
        let mut found: *mut libc::passwd = std::ptr::null_mut();
        let res = loop {
            // SAFETY: pwent / buf / found 在调用期间有效，buf 长度即传入大小
            let r = unsafe { libc::getpwuid_r(uid as libc::uid_t, &mut pwent, buf.as_mut_ptr(), buf.len(), &mut found) };
            if r != libc::EINTR {
                break r;
            }
        };
        // SAFETY: found 非空时指向 pwent，pw_name 为 NUL 结尾串
        let name = (!found.is_null() && !pwent.pw_name.is_null())
            .then(|| unsafe { std::ffi::CStr::from_ptr(pwent.pw_name) }.to_bytes())
            .filter(|n| !n.is_empty());
        match (res, name) {
            (0, Some(n)) => Ok(JArray::from(n.iter().map(|b| *b as i8).collect::<Vec<i8>>())),
            _ => Err(unix_exception(if res != 0 { res } else { libc::ENOENT })),
        }
    }
}
