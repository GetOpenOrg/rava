//! `sun/nio/fs/UnixFileAttributes` 手写伴生：POSIX 原生族档 A（切入序 4）。
//!
//! stat/lstat 语义以 `std::fs::metadata` / `symlink_metadata` 承载（报告 §4
//! 映射）；st_* 字段按 MetadataExt 填充。时间戳字段（atime/mtime/ctime/
//! birthtime）以秒承载（nsec=0；std 无纳秒粒度的稳定接口，用例面不消费）。
//! lastModifiedTime 等 FileTime 返回形态成员保持 panic 存根（FileTime 不在
//! 用例闭包）。

use crate::prelude::*;
use super::unix_file_attributes::UnixFileAttributes;
use super::unix_path::UnixPath;

/// `UnixPath → 系统调用路径`（std 侧 String）。getByteArrayForSysCalls 语义。
fn sys_path(path: &UnixPath) -> Result<std::string::String> {
    let bytes = path.getByteArrayForSysCalls()?;
    let raw: Vec<u8> = bytes.to_vec().into_iter().map(|b| b as u8).collect();
    Ok(std::string::String::from_utf8_lossy(&raw).into_owned())
}

fn secs_of(t: std::io::Result<std::time::SystemTime>) -> i64 {
    t.ok()
        .and_then(|st| st.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

impl UnixFileAttributes {
    /// `<init>()`：私有构造（字段由 stat 填充）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `get(UnixPath, boolean followLinks)`：stat / lstat（失败抛 UnixException）。
    #[jvm_boundary]
    pub fn get(path: UnixPath, followLinks: bool) -> Result<UnixFileAttributes> {
        let attrs = Self::new()?;
        let p = sys_path(&path)?;
        let md = if followLinks {
            std::fs::metadata(&p)
        } else {
            std::fs::symlink_metadata(&p)
        };
        match md {
            Ok(md) => {
                fill_from_metadata(&attrs, &md);
                Ok(attrs)
            }
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(2);
                Err(JvmError::from(super::unix_exception::UnixException::new_i(errno)?))
            }
        }
    }

    /// `getIfExists(UnixPath)`：stat2 语义——不存在返回 null 载体，其他错误抛出。
    #[jvm_boundary]
    pub fn getIfExists(path: UnixPath) -> Result<UnixFileAttributes> {
        let attrs = Self::new()?;
        let p = sys_path(&path)?;
        match std::fs::metadata(&p) {
            Ok(md) => {
                fill_from_metadata(&attrs, &md);
                Ok(attrs)
            }
            Err(e) if e.raw_os_error() == Some(2) => Ok(UnixFileAttributes::default()),
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(2);
                Err(JvmError::from(super::unix_exception::UnixException::new_i(errno)?))
            }
        }
    }

    /// `isDirectory()`：S_IFDIR。
    #[jvm_boundary]
    pub fn isDirectory(&self) -> Result<bool> {
        const S_IFMT: i32 = 0o170000;
        const S_IFDIR: i32 = 0o040000;
        Ok(self.__get_st_mode() & S_IFMT == S_IFDIR)
    }

    /// `isSymbolicLink()`：S_IFLNK。
    #[jvm_boundary]
    pub fn isSymbolicLink(&self) -> Result<bool> {
        const S_IFMT: i32 = 0o170000;
        const S_IFLNK: i32 = 0o120000;
        Ok(self.__get_st_mode() & S_IFMT == S_IFLNK)
    }

    /// `isRegularFile()`：S_IFREG。
    #[jvm_boundary]
    pub fn isRegularFile(&self) -> Result<bool> {
        const S_IFMT: i32 = 0o170000;
        const S_IFREG: i32 = 0o100000;
        Ok(self.__get_st_mode() & S_IFMT == S_IFREG)
    }

    /// `mode()`：st_mode。
    #[jvm_boundary]
    pub fn mode(&self) -> Result<i32> {
        Ok(self.__get_st_mode())
    }

    /// `ino()`：st_ino。
    #[jvm_boundary]
    pub fn ino(&self) -> Result<i64> {
        Ok(self.__get_st_ino())
    }

    /// `dev()`：st_dev。
    #[jvm_boundary]
    pub fn dev(&self) -> Result<i64> {
        Ok(self.__get_st_dev())
    }

    /// `size()`：st_size。
    #[jvm_boundary]
    pub fn size(&self) -> Result<i64> {
        Ok(self.__get_st_size())
    }

    /// `isSameFile(UnixFileAttributes)`：(st_ino, st_dev) 相等。
    #[jvm_boundary]
    pub fn isSameFile(&self, attrs: UnixFileAttributes) -> Result<bool> {
        Ok(self.__get_st_ino() == attrs.__get_st_ino()
            && self.__get_st_dev() == attrs.__get_st_dev())
    }
}

/// `struct stat` 填充（UnixNativeDispatcher.stat0 的 std 等价）。
/// ctime：std 无稳定接口，以 mtime 承载（用例面不消费）；birthtime：created()
/// （macOS st_birthtime 语义 / Linux 返回 Err → 0，birthtimeSupported 平台分叉）。
fn fill_from_metadata(attrs: &UnixFileAttributes, md: &std::fs::Metadata) {
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
    attrs.__set_st_birthtime_sec(secs_of(md.created()));
}
