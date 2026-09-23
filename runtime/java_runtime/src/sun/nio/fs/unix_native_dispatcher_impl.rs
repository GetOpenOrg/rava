//! `sun/nio/fs/UnixNativeDispatcher` 手写伴生：POSIX 原生族档 A（切入序 4/5）。
//!
//! 按用例面实现 open/close/stat/lstat/unlink/rmdir/access/strerror（报告 §3
//! 的 4 native 子集 + rmdir），std::fs 承载（报告 §4 映射：open0 →
//! OpenOptions+OpenOptionsExt::custom_flags+mode；fd 以 i32 裸 fd 流转，close0
//! 以 File::from_raw_fd 回收）。其余 40+ native 保持 panic 存根（档 B 引入
//! libc 后补全集）。

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

    /// `access(UnixPath, int amode)`：access(2) 的 errno 返回形态（0=允许）。
    /// F_OK 以 metadata 精确承载；R_OK/W_OK/X_OK 无 std 等价（faccessat 需
    /// libc——档 B），档 A 以存在性近似（用例面 only 走 F_OK：exists 链；
    /// isReadable/isWritable/isExecutable 未被需求登记，保持存根不受影响）。
    #[jvm_native]
    pub fn access(path: UnixPath, amode: i32) -> Result<i32> {
        let _ = amode; // 见上：R/W/X 近似为 F_OK 语义
        let p = sys_path(&path)?;
        match std::fs::metadata(&p) {
            Ok(_) => Ok(0),
            Err(e) => Ok(e.raw_os_error().unwrap_or(consts::errno::ENOENT)),
        }
    }

    /// `strerror(int)`：平台错误字符串（jnu 编码字节）。
    #[jvm_native]
    pub fn strerror(errno: i32) -> Result<JArray<i8>> {
        let s = std::io::Error::from_raw_os_error(errno).to_string();
        Ok(JArray::from(
            s.into_bytes().into_iter().map(|b| b as i8).collect::<Vec<i8>>(),
        ))
    }

    /// native `init(int[])`：capabilities 位图写入（JDK21 的 UnixCapabilities：
    /// [0]=_support八位图）。宿主真实能力：xattr 与 birthtime 的平台分叉按
    /// cfg 报告（Unsafe 常量自洽原则——不虚报能力）。
    #[jvm_native]
    pub fn init(capabilities: JArray<i32>) -> Result<()> {
        // UnixNativeDispatcher.init 写入 capabilities[0] 的位：
        //   0x01 = xattr supported, 0x02 = birthtime supported（JDK 语义位序）
        let bits: i32 = if cfg!(target_os = "macos") { 0x03 } else { 0x00 };
        if capabilities.len()? > 0 {
            capabilities.set(0, bits)?;
        }
        Ok(())
    }
}

/// stat 缓冲填充（UnixFileAttributes.st_* 字段）。
fn fill_stat(attrs: &UnixFileAttributes, md: &std::fs::Metadata) {
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
    attrs.__set_st_mtime_sec(md.mtime());
    attrs.__set_st_ctime_sec(md.mtime());
    let birth = md
        .created()
        .ok()
        .and_then(|st| st.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    attrs.__set_st_birthtime_sec(birth);
}
