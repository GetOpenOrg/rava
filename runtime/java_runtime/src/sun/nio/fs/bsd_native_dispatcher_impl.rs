//! `sun/nio/fs/BsdNativeDispatcher` 手写伴生：macOS 属性列表 native（ACC_NATIVE 准入）。
//!
//! 对标 JDK 21 `BsdNativeDispatcher.c`：setattrlist0 / fsetattrlist0 按 `commonattr` 位
//! （ATTR_CMN_CRTIME / MODTIME / ACCTIME 的位序）依次装入纳秒拆分的 `timespec`；initIDs 只为
//! 挂载表枚举（UnixMountEntry）缓存字段 ID，本模型按名访问字段，无需缓存。
//! 挂载表枚举（getfsstat / fsstatEntry / endfsstat）的迭代器是堆上的 [`FsStatIter`]，地址作 long 句柄；
//! clonefile0 返回 errno（0 = 成功），不抛异常（JNI 同款）。本类 8 个 native 全部承载。

use crate::prelude::*;
use super::bsd_native_dispatcher::BsdNativeDispatcher;
use super::unix_mount_entry::UnixMountEntry;
#[cfg(target_os = "macos")]
use super::unix_native_dispatcher_impl::errno;
use super::unix_native_dispatcher_impl::unix_exception;

/// commonattr 所选时间属性 → timespec 缓冲（创建、修改、访问的位序）。
#[cfg(target_os = "macos")]
fn attr_times(commonattr: i32, modify: i64, access: i64, create: i64) -> Vec<libc::timespec> {
    let ts = |t: i64| libc::timespec { tv_sec: t / 1_000_000_000, tv_nsec: t % 1_000_000_000 };
    let attr = commonattr as libc::attrgroup_t;
    let mut times = Vec::with_capacity(3);
    if attr & libc::ATTR_CMN_CRTIME != 0 {
        times.push(ts(create));
    }
    if attr & libc::ATTR_CMN_MODTIME != 0 {
        times.push(ts(modify));
    }
    if attr & libc::ATTR_CMN_ACCTIME != 0 {
        times.push(ts(access));
    }
    times
}

/// setattrlist 的作用对象。
enum Target {
    Path(i64),
    Fd(i32),
}

/// 以 commonattr 组装 attrlist，对路径（setattrlist）或描述符（fsetattrlist）设置时间属性。
#[cfg(target_os = "macos")]
fn set_times(target: Target, commonattr: i32, modify: i64, access: i64, create: i64, options: i64) -> Result<()> {
    let mut list = libc::attrlist {
        bitmapcount: libc::ATTR_BIT_MAP_COUNT,
        reserved: 0,
        commonattr: commonattr as libc::attrgroup_t,
        volattr: 0,
        dirattr: 0,
        fileattr: 0,
        forkattr: 0,
    };
    let mut times = attr_times(commonattr, modify, access, create);
    let size = times.len() * std::mem::size_of::<libc::timespec>();
    let list_ptr = (&mut list as *mut libc::attrlist).cast();
    let buf = times.as_mut_ptr().cast();
    // SAFETY: 路径地址指向 NUL 结尾路径 / fd 为调用方持有的描述符；list 与 buf 在调用期间有效
    let r = unsafe {
        match target {
            Target::Path(path) => libc::setattrlist(path as *const libc::c_char, list_ptr, buf, size, options as u32),
            Target::Fd(fd) => libc::fsetattrlist(fd, list_ptr, buf, size, options as u32),
        }
    };
    if r != 0 {
        return Err(unix_exception(errno()));
    }
    Ok(())
}

/// 非 macOS 宿主无 setattrlist（本类只在 macOS 的 JDK 中存在）。
#[cfg(not(target_os = "macos"))]
fn set_times(_target: Target, _commonattr: i32, _modify: i64, _access: i64, _create: i64, _options: i64) -> Result<()> {
    Err(unix_exception(libc::ENOTSUP))
}

impl BsdNativeDispatcher {
    /// native `initIDs()`：本模型按名访问字段，无字段 ID 需缓存。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native `setattrlist0(long path, int commonattr, long modTime, long accTime, long createTime, long options)`。
    #[jvm_native]
    pub fn setattrlist0(path: i64, commonattr: i32, modify: i64, access: i64, create: i64, options: i64) -> Result<()> {
        set_times(Target::Path(path), commonattr, modify, access, create, options)
    }

    /// native `fsetattrlist0(int fd, int commonattr, long modTime, long accTime, long createTime, long options)`。
    #[jvm_native]
    pub fn fsetattrlist0(fd: i32, commonattr: i32, modify: i64, access: i64, create: i64, options: i64) -> Result<()> {
        set_times(Target::Fd(fd), commonattr, modify, access, create, options)
    }

    /// native `getmntonname0(long pathAddress)`：statfs(2) 取所在文件系统挂载点 `f_mntonname`。
    #[jvm_native]
    pub fn getmntonname0(path: i64) -> Result<JArray<i8>> {
        mount_point(path)
    }

    /// native `getfsstat()`：getfsstat(2) 快照全部挂载项，返回迭代器句柄。先计数再取（多留 16 项，
    /// 防两次调用之间新挂载），任一次结果 ≤ 0 抛 UnixException(errno)。
    #[jvm_native]
    pub fn getfsstat() -> Result<i64> {
        fs_stat_open()
    }

    /// native `fsstatEntry(long iter, UnixMountEntry entry)`：取下一挂载项填 name（f_mntfromname）/
    /// dir（f_mntonname）/ fstype（f_fstypename）/ opts（MNT_RDONLY ? "ro" : "rw"），返回 0；取尽返回 -1。
    #[jvm_native]
    pub fn fsstatEntry(iter: i64, entry: UnixMountEntry) -> Result<i32> {
        fs_stat_next(iter, &entry)
    }

    /// native `endfsstat(long iter)`：释放迭代器。
    #[jvm_native]
    pub fn endfsstat(iter: i64) -> Result<()> {
        fs_stat_close(iter);
        Ok(())
    }

    /// native `clonefile0(long src, long dst, int flags)`：clonefile(2)，返回 errno（0 = 成功）。
    #[jvm_native]
    pub fn clonefile0(src: i64, dst: i64, flags: i32) -> Result<i32> {
        Ok(clone_file(src, dst, flags))
    }
}

/// getfsstat 快照与游标（`BsdNativeDispatcher.c` 的 `struct fsstat_iter`）。
#[cfg(target_os = "macos")]
struct FsStatIter {
    entries: Vec<libc::statfs>,
    pos: usize,
}

/// `<sys/mount.h>` MNT_RDONLY
#[cfg(target_os = "macos")]
const MNT_RDONLY: u32 = 0x0000_0001;

#[cfg(target_os = "macos")]
fn fs_stat_open() -> Result<i64> {
    // SAFETY: 缓冲为空时只返回挂载项数
    let n = unsafe { libc::getfsstat(std::ptr::null_mut(), 0, libc::MNT_NOWAIT) };
    if n <= 0 {
        return Err(unix_exception(errno()));
    }
    let cap = n as usize + 16;
    // SAFETY: statfs 为纯数据结构，零值合法
    let mut entries: Vec<libc::statfs> = (0..cap).map(|_| unsafe { std::mem::zeroed() }).collect();
    let size = (cap * std::mem::size_of::<libc::statfs>()) as libc::c_int;
    // SAFETY: entries 可写 size 字节
    let n = unsafe { libc::getfsstat(entries.as_mut_ptr(), size, libc::MNT_WAIT) };
    if n <= 0 {
        return Err(unix_exception(errno()));
    }
    entries.truncate(n as usize);
    Ok(Box::into_raw(Box::new(FsStatIter { entries, pos: 0 })) as i64)
}

#[cfg(target_os = "macos")]
fn fs_stat_next(iter: i64, entry: &UnixMountEntry) -> Result<i32> {
    // SAFETY: iter 为 fs_stat_open 返回、尚未 endfsstat 的句柄
    let it = unsafe { &mut *(iter as *mut FsStatIter) };
    let Some(sfs) = it.entries.get(it.pos) else {
        return Ok(-1);
    };
    let bytes = |a: &[libc::c_char]| {
        // SAFETY: statfs 的名字字段为 NUL 结尾的定长数组
        let s = unsafe { std::ffi::CStr::from_ptr(a.as_ptr()) };
        JArray::from(s.to_bytes().iter().map(|b| *b as i8).collect::<Vec<i8>>())
    };
    entry.__set_name(bytes(&sfs.f_mntfromname));
    entry.__set_dir(bytes(&sfs.f_mntonname));
    entry.__set_fstype(bytes(&sfs.f_fstypename));
    let opts: &[u8] = if sfs.f_flags & MNT_RDONLY != 0 { b"ro" } else { b"rw" };
    entry.__set_opts(JArray::from(opts.iter().map(|b| *b as i8).collect::<Vec<i8>>()));
    it.pos += 1;
    Ok(0)
}

#[cfg(target_os = "macos")]
fn fs_stat_close(iter: i64) {
    if iter != 0 {
        // SAFETY: iter 为 fs_stat_open 的 Box 指针，只释放一次（UnixFileSystem 的 finally）
        drop(unsafe { Box::from_raw(iter as *mut FsStatIter) });
    }
}

#[cfg(target_os = "macos")]
fn clone_file(src: i64, dst: i64, flags: i32) -> i32 {
    // SAFETY: src / dst 指向 NUL 结尾路径
    match super::unix_native_dispatcher_impl::restartable(|| unsafe {
        libc::clonefile(src as *const libc::c_char, dst as *const libc::c_char, flags as u32)
    }) {
        Ok(_) => 0,
        Err(e) => e,
    }
}

// 非 macOS 宿主：本类只在 macOS 的 JDK 中存在，下列落点不会被翻译体调用，只为保持文件可编译。
#[cfg(not(target_os = "macos"))]
fn fs_stat_open() -> Result<i64> {
    Err(unix_exception(libc::ENOTSUP))
}

#[cfg(not(target_os = "macos"))]
fn fs_stat_next(_iter: i64, _entry: &UnixMountEntry) -> Result<i32> {
    Ok(-1)
}

#[cfg(not(target_os = "macos"))]
fn fs_stat_close(_iter: i64) {}

#[cfg(not(target_os = "macos"))]
fn clone_file(_src: i64, _dst: i64, _flags: i32) -> i32 {
    libc::ENOTSUP
}

/// 路径所在文件系统的挂载点字节。
#[cfg(target_os = "macos")]
fn mount_point(path: i64) -> Result<JArray<i8>> {
    // SAFETY: statfs 为纯数据结构，零值合法
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: path 指向 NUL 结尾路径，buf 可写
    if unsafe { libc::statfs(path as *const libc::c_char, &mut buf) } != 0 {
        return Err(unix_exception(errno()));
    }
    // SAFETY: f_mntonname 为 NUL 结尾的定长数组
    let name = unsafe { std::ffi::CStr::from_ptr(buf.f_mntonname.as_ptr()) };
    Ok(JArray::from(name.to_bytes().iter().map(|b| *b as i8).collect::<Vec<i8>>()))
}

/// 非 macOS 宿主无 f_mntonname（本类只在 macOS 的 JDK 中存在）。
#[cfg(not(target_os = "macos"))]
fn mount_point(_path: i64) -> Result<JArray<i8>> {
    Err(unix_exception(libc::ENOTSUP))
}
