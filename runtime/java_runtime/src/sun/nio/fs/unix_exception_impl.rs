//! `sun/nio/fs/UnixException` 手写伴生：POSIX 原生族档 A（切入序 4）。
//!
//! errno 载体异常：消息语义 = strerror（std::io::Error::from_raw_os_error 的
//! 平台字符串，与报告 §4 映射一致）；rethrowAsIOException 按 JDK 错误映射表
//! 把 errno 翻译为具体 IOException 子类。

use crate::prelude::*;
use super::unix_exception::UnixException;
use super::unix_path::UnixPath;
use crate::java::io::IOException;
use crate::java::lang::String;

impl UnixException {
    /// `<init>(int errno)`。
    #[jvm_boundary]
    pub fn new_i(errno: i32) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_errno(errno);
        Ok(this)
    }

    /// `<init>(String msg)`：非 errno 形态（errno=0）。
    #[jvm_boundary]
    pub fn new_str(msg: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_msg(msg);
        Ok(this)
    }

    /// `errno()`。
    #[jvm_boundary]
    pub fn errno(&self) -> Result<i32> {
        Ok(self.__get_errno())
    }

    /// `setError(int)`。
    #[jvm_boundary]
    pub fn setError(&self, errno: i32) -> Result<()> {
        self.__set_errno(errno);
        Ok(())
    }

    /// `errorString()`：msg 优先，否则 strerror(errno)。
    #[jvm_boundary]
    pub fn errorString(&self) -> Result<String> {
        let msg = self.__get_msg();
        if !_is_jnull(&msg) {
            return Ok(msg);
        }
        Ok(String::from(
            std::io::Error::from_raw_os_error(self.__get_errno()).to_string().as_str(),
        ))
    }

    /// `getMessage()`（Throwable 覆写，vtable 槽位形态）。
    #[jvm_boundary]
    pub fn __impl_getMessage(&self) -> Result<String> {
        self.errorString()
    }

    /// `translateToIOException(String file, String other)`：JDK 错误映射表
    /// （EACCES/EEXIST/ENOENT/ELOOP → 具体子类，其余 → FileSystemException）。
    /// file/other 语义为 null 载体（JVM null String）。
    #[jvm_boundary(upcalls = "java/nio/file/AccessDeniedException.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V java/nio/file/NoSuchFileException.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V java/nio/file/FileAlreadyExistsException.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V java/nio/file/FileSystemException.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V")]
    pub fn translateToIOException(&self, file: String, other: String) -> Result<IOException> {
        let msg = self.__get_msg();
        if !_is_jnull(&msg) {
            return Ok(Clone::clone(&IOException::new_str(Clone::clone(&msg))?).into());
        }
        const EACCES: i32 = 13;
        const ENOENT: i32 = 2;
        const EEXIST: i32 = 17;
        #[cfg(target_os = "macos")]
        const ELOOP: i32 = 62;
        #[cfg(target_os = "linux")]
        const ELOOP: i32 = 40;
        let errno = self.__get_errno();
        let other_arg = |v: &String| -> String {
            if _is_jnull(v) { String::from("") } else { Clone::clone(v) }
        };
        match errno {
            EACCES => Ok(crate::java::nio::file::AccessDeniedException::new_str_str_str(
                Clone::clone(&file),
                other_arg(&other),
                String::from(""),
            )?
            .into()),
            ENOENT => Ok(crate::java::nio::file::NoSuchFileException::new_str_str_str(
                Clone::clone(&file),
                other_arg(&other),
                String::from(""),
            )?
            .into()),
            EEXIST => Ok(crate::java::nio::file::FileAlreadyExistsException::new_str_str_str(
                Clone::clone(&file),
                other_arg(&other),
                String::from(""),
            )?
            .into()),
            ELOOP => Ok(crate::java::nio::file::FileSystemException::new_str_str_str(
                Clone::clone(&file),
                other_arg(&other),
                String::from(format!("{} or unable to access attributes of symbolic link",
                    self.errorString()?).as_str()),
            )?
            .into()),
            _ => Ok(crate::java::nio::file::FileSystemException::new_str_str_str(
                Clone::clone(&file),
                other_arg(&other),
                self.errorString()?,
            )?
            .into()),
        }
    }

    /// `rethrowAsIOException(UnixPath file, UnixPath other)`：翻译并以 IOException
    /// 异常形态抛出（Rust 侧 = JvmError 传播）。
    pub fn rethrowAsIOException_unixpath_unixpath(
        &self,
        file: Option<&UnixPath>,
        other: Option<&UnixPath>,
    ) -> Result<()> {
        let a = match file {
            Some(f) => f.getPathForExceptionMessage()?,
            None => String::default(),
        };
        let b = match other {
            Some(o) => o.getPathForExceptionMessage()?,
            None => String::default(),
        };
        let x = self.translateToIOException(a, b)?;
        Err(JvmError::from(x))
    }

    /// `rethrowAsIOException(UnixPath file)`。
    pub fn rethrowAsIOException_unixpath(&self, file: &UnixPath) -> Result<()> {
        self.rethrowAsIOException_unixpath_unixpath(Some(file), None)
    }

    /// `asIOException(UnixPath file)`：翻译为 IOException 值（不抛出）。
    pub fn asIOException(&self, file: &UnixPath) -> Result<IOException> {
        self.translateToIOException(file.getPathForExceptionMessage()?, String::default())
    }
}
