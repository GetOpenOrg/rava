//! `sun/nio/fs/UnixNativeDispatcher` 手写伴生：POSIX 原生族档 A（切入序 4/5）。
//!
//! JDK 21 形态：native（`init()I`、close0 / dup / opendir0 / fdopendir / closedir / readdir0、
//! strerror）按 ACC_NATIVE 准入由 libc 承载；open/close/stat/lstat/unlink/rmdir/access 为字节码
//! 方法上的手写覆盖（std::fs 承载，fd 以 i32 裸 fd 流转）。其余 native 保持 panic 存根。

use crate::prelude::*;
use super::unix_native_dispatcher::UnixNativeDispatcher;
use super::unix_exception::UnixException;
use super::unix_file_attributes::UnixFileAttributes;
use super::unix_path::UnixPath;
use crate::java::lang::String;

/// 平台 open(2) 标志位与 errno 常量（值随目标平台，与平台 JDK classfile 的
/// UnixConstants 烧入值同源：macOS O_CREAT=0x200 / Linux 0x40——UnixConstants
/// 由生成侧自动取对，此处是 std 侧 syscall 的宿主值）。
pub(crate) mod consts {
    #[cfg(target_os = "macos")]
    pub mod oflags {
        pub const O_WRONLY: i32 = 0x0001;
        pub const O_RDWR: i32 = 0x0002;
        pub const O_APPEND: i32 = 0x0008;
        pub const O_CREAT: i32 = 0x0200;
        pub const O_TRUNC: i32 = 0x0400;
        pub const O_EXCL: i32 = 0x0800;
        pub const O_NOFOLLOW: i32 = 0x0040_0000;
    }
    #[cfg(target_os = "linux")]
    pub mod oflags {
        pub const O_WRONLY: i32 = 0x0001;
        pub const O_RDWR: i32 = 0x0002;
        pub const O_APPEND: i32 = 0x0400;
        pub const O_CREAT: i32 = 0x0040;
        pub const O_TRUNC: i32 = 0x0200;
        pub const O_EXCL: i32 = 0x0080;
        pub const O_NOFOLLOW: i32 = 0x0020_0000;
    }
    pub mod errno {
        pub const ENOENT: i32 = 2;
        pub const EISDIR: i32 = 21;
        #[cfg(target_os = "macos")]
        pub const ELOOP: i32 = 62;
        #[cfg(target_os = "linux")]
        pub const ELOOP: i32 = 40;
        pub const EACCES: i32 = 13;
        pub const EEXIST: i32 = 17;
        #[cfg(target_os = "macos")]
        pub const ENOTEMPTY: i32 = 66;
        #[cfg(target_os = "linux")]
        pub const ENOTEMPTY: i32 = 39;
        /// access(2) 的 amode。
        pub const F_OK: i32 = 0;
        pub const R_OK: i32 = 4;
        pub const W_OK: i32 = 2;
        pub const X_OK: i32 = 1;
    }
}

/// UnixPath → 系统调用路径（std String）。
pub(crate) fn sys_path(path: &UnixPath) -> Result<std::string::String> {
    let bytes = path.getByteArrayForSysCalls()?;
    let raw: Vec<u8> = bytes.to_vec().into_iter().map(|b| b as u8).collect();
    Ok(std::string::String::from_utf8_lossy(&raw).into_owned())
}

/// io::Error → UnixException（errno 保真；无 os 错误码时以 ENOENT 兜底）。
pub(crate) fn as_unix_exception(e: &std::io::Error) -> UnixException {
    UnixException::new_i(e.raw_os_error().unwrap_or(consts::errno::ENOENT))
        .expect("UnixException 构造无失败面")
}

impl UnixNativeDispatcher {
    /// `open(UnixPath, int flags, int mode)`：open(2)。flags 为宿主平台
    /// O_* 位集；fd 以 i32 返回（裸 fd，FileDescriptor/通道层以 int 流转）。
    #[jvm_native]
    pub fn open(path: UnixPath, flags: i32, mode: i32) -> Result<i32> {
        use std::os::fd::IntoRawFd;
        use std::os::unix::fs::OpenOptionsExt;
        use consts::oflags as o;
        let p = sys_path(&path)?;
        let acc = flags & 0x3; // O_ACCMODE（O_RDONLY=0 / O_WRONLY=1 / O_RDWR=2）
        let mut opts = std::fs::OpenOptions::new();
        opts.read(acc != o::O_WRONLY);
        opts.write(acc == o::O_RDWR || acc == o::O_WRONLY);
        if flags & o::O_APPEND != 0 {
            opts.append(true);
        }
        if flags & o::O_CREAT != 0 {
            if flags & o::O_EXCL != 0 {
                opts.create_new(true);
            } else {
                opts.create(true);
            }
        }
        if flags & o::O_TRUNC != 0 {
            opts.truncate(true);
        }
        // 其余标志位（O_NOFOLLOW 等）原样透传
        let passthrough =
            flags & !(0x3 | o::O_APPEND | o::O_CREAT | o::O_EXCL | o::O_TRUNC);
        opts.custom_flags(passthrough);
        opts.mode(mode as u32);
        match opts.open(&p) {
            Ok(f) => Ok(f.into_raw_fd()),
            Err(e) => Err(JvmError::from(as_unix_exception(&e))),
        }
    }

    /// `close(int fd)`：fd==-1 no-op；否则 close(2)（File::from_raw_fd 回收）。
    #[jvm_native]
    pub fn close(fd: i32) -> Result<()> {
        use std::os::fd::FromRawFd;
        if fd == -1 {
            return Ok(());
        }
        // SAFETY: fd 来自本模块 open 的 into_raw_fd（所有权移交至此回收）
        let f = unsafe { std::fs::File::from_raw_fd(fd) };
        drop(f);
        Ok(())
    }

    /// `stat(UnixPath, UnixFileAttributes)`：stat(2)（失败抛 UnixException）。
    #[jvm_native]
    pub fn stat(path: UnixPath, attrs: UnixFileAttributes) -> Result<()> {
        let p = sys_path(&path)?;
        match std::fs::metadata(&p) {
            Ok(md) => {
                fill_stat(&attrs, &md);
                Ok(())
            }
            Err(e) => Err(JvmError::from(as_unix_exception(&e))),
        }
    }

    /// `stat2(UnixPath, UnixFileAttributes)`：stat 的 errno 返回形态（0=成功）。
    #[jvm_native]
    pub fn stat2(path: UnixPath, attrs: UnixFileAttributes) -> Result<i32> {
        let p = sys_path(&path)?;
        match std::fs::metadata(&p) {
            Ok(md) => {
                fill_stat(&attrs, &md);
                Ok(0)
            }
            Err(e) => Ok(e.raw_os_error().unwrap_or(consts::errno::ENOENT)),
        }
    }

    /// `lstat(UnixPath, UnixFileAttributes)`：lstat(2)。
    #[jvm_native]
    pub fn lstat(path: UnixPath, attrs: UnixFileAttributes) -> Result<()> {
        let p = sys_path(&path)?;
        match std::fs::symlink_metadata(&p) {
            Ok(md) => {
                fill_stat(&attrs, &md);
                Ok(())
            }
            Err(e) => Err(JvmError::from(as_unix_exception(&e))),
        }
    }

    /// `unlink(UnixPath)`：unlink(2)。
    #[jvm_native]
    pub fn unlink(path: UnixPath) -> Result<()> {
        let p = sys_path(&path)?;
        std::fs::remove_file(&p).map_err(|e| JvmError::from(as_unix_exception(&e)))
    }

    /// `rmdir(UnixPath)`：rmdir(2)（implDelete 的目录分支）。
    #[jvm_native]
    pub fn rmdir(path: UnixPath) -> Result<()> {
        let p = sys_path(&path)?;
        std::fs::remove_dir(&p).map_err(|e| JvmError::from(as_unix_exception(&e)))
    }

    /// `access(UnixPath, int amode)`：access(2) 的 errno 返回形态（0=允许），F_OK / R_OK /
    /// W_OK / X_OK 均按有效用户权限精确判定（FS-IO1）。
    #[jvm_native]
    pub fn access(path: UnixPath, amode: i32) -> Result<i32> {
        let p = sys_path(&path)?;
        Ok(crate::posix::access(std::path::Path::new(&p), amode))
    }

    /// `strerror(int)`：平台错误字符串（jnu 编码字节）。
    #[jvm_native]
    pub fn strerror(errno: i32) -> Result<JArray<i8>> {
        let s = std::io::Error::from_raw_os_error(errno).to_string();
        Ok(JArray::from(
            s.into_bytes().into_iter().map(|b| b as i8).collect::<Vec<i8>>(),
        ))
    }

    /// native `init()I`：宿主能力位图，写入 `capabilities`（JDK 21 `UnixNativeDispatcher.c` 的
    /// `Java_sun_nio_fs_UnixNativeDispatcher_init` 同口径）：OPENAT(1<<1) / FUTIMES(1<<2) / FUTIMENS(1<<3) /
    /// LUTIMES(1<<4) / XATTR(1<<5) 在 macOS 与 Linux 均可用；BIRTHTIME(1<<16) 仅 macOS（64 位 inode 的
    /// `st_birthtimespec`）——Linux 的 birthtime 依赖 statx，本层 stat 不填，故不报告（不虚报能力）。
    #[jvm_native]
    pub fn init() -> Result<i32> {
        const OPENAT: i32 = 1 << 1;
        const FUTIMES: i32 = 1 << 2;
        const FUTIMENS: i32 = 1 << 3;
        const LUTIMES: i32 = 1 << 4;
        const XATTR: i32 = 1 << 5;
        const BIRTHTIME: i32 = 1 << 16;
        let base = OPENAT | FUTIMES | FUTIMENS | LUTIMES | XATTR;
        Ok(if cfg!(target_os = "macos") { base | BIRTHTIME } else { base })
    }

    /// `close0(int fd)`：close(2)；失败且非 EINTR 时抛 UnixException（JDK 同口径：EINTR 视为已关闭）。
    #[jvm_native]
    pub fn close0(fd: i32) -> Result<()> {
        // SAFETY: fd 为调用方持有的文件描述符（所有权随本调用交还内核）
        if unsafe { libc::close(fd) } == -1 {
            let e = errno();
            if e != libc::EINTR {
                return Err(unix_exception(e));
            }
        }
        Ok(())
    }

    /// `dup(int fd)`：dup(2)（EINTR 重试），失败抛 UnixException。
    #[jvm_native]
    pub fn dup(fd: i32) -> Result<i32> {
        // SAFETY: dup 只读取描述符表
        restartable(|| unsafe { libc::dup(fd) }).map_err(unix_exception)
    }

    /// `opendir0(long path)`：opendir(3)；`path` 为 `copyToNativeBuffer` 写入的 NUL 结尾路径（直接内存地址），
    /// 返回 `DIR*` 地址。失败抛 UnixException。
    #[jvm_native]
    pub fn opendir0(path_address: i64) -> Result<i64> {
        // SAFETY: path_address 指向 NativeBuffer 中以 NUL 结尾的路径字节
        let dir = unsafe { libc::opendir(path_address as *const libc::c_char) };
        if dir.is_null() {
            return Err(unix_exception(errno()));
        }
        Ok(dir as i64)
    }

    /// `fdopendir(int dfd)`：fdopendir(3)，返回 `DIR*` 地址（dfd 所有权移交 DIR）。失败抛 UnixException。
    #[jvm_native]
    pub fn fdopendir(dfd: i32) -> Result<i64> {
        // SAFETY: dfd 为已打开的目录描述符
        let dir = unsafe { libc::fdopendir(dfd) };
        if dir.is_null() {
            return Err(unix_exception(errno()));
        }
        Ok(dir as i64)
    }

    /// `closedir(long dir)`：closedir(3)；失败且非 EINTR 时抛 UnixException。
    #[jvm_native]
    pub fn closedir(dir: i64) -> Result<()> {
        // SAFETY: dir 为 opendir0 / fdopendir 返回的 DIR*（每个流只关闭一次，由 Java 侧状态保证）
        if unsafe { libc::closedir(dir as *mut libc::DIR) } == -1 {
            let e = errno();
            if e != libc::EINTR {
                return Err(unix_exception(e));
            }
        }
        Ok(())
    }

    /// `readdir0(long dir)`：readdir(3)，返回目录项名字节（不过滤 `.` / `..`，由 Java 侧
    /// `isSelfOrParent` 过滤）；读尽返回 null，出错抛 UnixException。
    #[jvm_native]
    pub fn readdir0(dir: i64) -> Result<JArray<i8>> {
        set_errno(0);
        // SAFETY: dir 为有效 DIR*；返回的 dirent 在下次 readdir 前有效，立即复制 d_name
        let ent = unsafe { libc::readdir(dir as *mut libc::DIR) };
        if ent.is_null() {
            let e = errno();
            return if e != 0 { Err(unix_exception(e)) } else { Ok(JArray::default()) };
        }
        // SAFETY: d_name 为 NUL 结尾的定长数组
        let name = unsafe { std::ffi::CStr::from_ptr((*ent).d_name.as_ptr()) };
        Ok(JArray::from(name.to_bytes().iter().map(|b| *b as i8).collect::<Vec<i8>>()))
    }
}

/// 当前线程 errno。
pub(super) fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

/// 置当前线程 errno（readdir 以 NULL + errno 区分读尽与出错）。
fn set_errno(v: i32) {
    // SAFETY: errno 位置为线程局部变量地址
    unsafe {
        #[cfg(target_os = "macos")]
        {
            *libc::__error() = v;
        }
        #[cfg(target_os = "linux")]
        {
            *libc::__errno_location() = v;
        }
    }
}

/// 系统调用返回 -1 且 errno == EINTR 时重试（JDK `RESTARTABLE` 宏）；其它失败返回 errno。
pub(super) fn restartable(mut call: impl FnMut() -> i32) -> std::result::Result<i32, i32> {
    loop {
        let r = call();
        if r != -1 {
            return Ok(r);
        }
        let e = errno();
        if e != libc::EINTR {
            return Err(e);
        }
    }
}

/// errno → `UnixException`（JDK `throwUnixException`）。
pub(super) fn unix_exception(errno: i32) -> JvmError {
    JvmError::from(UnixException::new_i(errno).expect("UnixException 构造无失败面"))
}

/// stat 缓冲填充（UnixFileAttributes.st_* 字段）。
pub(super) fn fill_stat(attrs: &UnixFileAttributes, md: &std::fs::Metadata) {
    use std::os::unix::fs::MetadataExt;
    attrs.__set_st_mode(md.mode() as i32);
    attrs.__set_st_ino(md.ino() as i64);
    attrs.__set_st_dev(md.dev() as i64);
    attrs.__set_st_rdev(md.rdev() as i64);
    attrs.__set_st_nlink(md.nlink() as i32);
    attrs.__set_st_uid(md.uid() as i32);
    attrs.__set_st_gid(md.gid() as i32);
    attrs.__set_st_size(md.size() as i64);
    attrs.__set_st_atime_sec(md.atime());
    attrs.__set_st_atime_nsec(md.atime_nsec());
    attrs.__set_st_mtime_sec(md.mtime());
    attrs.__set_st_mtime_nsec(md.mtime_nsec());
    attrs.__set_st_ctime_sec(md.ctime());
    attrs.__set_st_ctime_nsec(md.ctime_nsec());
    // birthtime 仅在 init() 报告 BIRTHTIME 能力（macOS）时被 creationTime() 读取
    let birth = md.created().ok().and_then(|st| st.duration_since(std::time::UNIX_EPOCH).ok());
    attrs.__set_st_birthtime_sec(birth.map_or(0, |d| d.as_secs() as i64));
    attrs.__set_st_birthtime_nsec(birth.map_or(0, |d| i64::from(d.subsec_nanos())));
}
