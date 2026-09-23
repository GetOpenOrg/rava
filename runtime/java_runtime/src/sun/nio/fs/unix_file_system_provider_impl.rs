//! `sun/nio/fs/UnixFileSystemProvider` 手写伴生：POSIX 原生族档 A（切入序 2/4/5）。
//!
//! 按用例面实现（TestFilesApi：writeString/readString/readAllLines/mismatch/
//! exists/deleteIfExists）：构造链、theFileSystem/getFileSystem/getScheme 与
//! checkAccess/exists/readAttributesIfExists/implDelete/newByteChannel 的 Unix
//! 语义体；其余成员保持 panic 存根（档 A 纪律）。

use crate::prelude::*;
use super::unix_file_system_provider::UnixFileSystemProvider;
use super::unix_file_system::UnixFileSystem;
use crate::java::lang::String;
use crate::java::net::URI;
use crate::jdk::internal::util::StaticProperty;

impl UnixFileSystemProvider {
    /// `<init>()`: `theFileSystem = newFileSystem(StaticProperty.userDir())`。
    /// 抽象基类——实际由平台薄层（macOS/Linux）构造，newFileSystem 虚分派
    /// 解析到平台覆写；本形态以基类 UnixFileSystem 承载（档 A 落差见
    /// mac_osx_file_system_provider_impl 的注释）。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        let dir = StaticProperty::USER_DIR()?;
        let fs = UnixFileSystem::new(Clone::clone(&this), dir)?;
        this.__set_theFileSystem(fs);
        Ok(this)
    }

    /// `theFileSystem()`：默认文件系统访问器（本包内直调形态）。
    #[jvm_boundary]
    pub fn theFileSystem(&self) -> Result<UnixFileSystem> {
        Ok(self.__get_theFileSystem())
    }

    /// `getScheme()`: 恒为 "file"。
    #[jvm_boundary]
    pub fn getScheme(&self) -> Result<String> {
        Ok(String::from("file"))
    }

    /// `checkUri(URI)`: file URI 形态校验（scheme/authority/path/query/fragment）。
    #[jvm_boundary(upcalls = "java/net/URI.getScheme:()Ljava/lang/String; java/net/URI.getRawAuthority:()Ljava/lang/String; java/net/URI.getPath:()Ljava/lang/String; java/net/URI.getRawQuery:()Ljava/lang/String; java/net/URI.getRawFragment:()Ljava/lang/String; java/lang/String.equalsIgnoreCase:(Ljava/lang/String;)Z java/lang/IllegalArgumentException.<init>:(Ljava/lang/String;)V")]
    pub fn checkUri(&self, uri: URI) -> Result<()> {
        let scheme_ok = uri.getScheme()?
            .equalsIgnoreCase(Clone::clone(&self.getScheme()?))?;
        if !scheme_ok {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "URI does not match this provider",
                ))?,
            ));
        }
        let authority = uri.getRawAuthority()?;
        if !_is_jnull(&authority) {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Authority component present",
                ))?,
            ));
        }
        let path = uri.getPath()?;
        if _is_jnull(&path) {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Path component is undefined",
                ))?,
            ));
        }
        if !String::from("/") .equals(Object::from(Clone::clone(&path)))? {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Path component should be '/'",
                ))?,
            ));
        }
        let query = uri.getRawQuery()?;
        if !_is_jnull(&query) {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Query component present",
                ))?,
            ));
        }
        let fragment = uri.getRawFragment()?;
        if !_is_jnull(&fragment) {
            return Err(JvmError::from(
                crate::java::lang::IllegalArgumentException::new_str(String::from(
                    "Fragment component present",
                ))?,
            ));
        }
        Ok(())
    }

    /// `getFileSystem(URI)`: checkUri 后返回 theFileSystem（final，声明类即实现体）。
    #[jvm_boundary(upcalls = "java/nio/file/FileSystemAlreadyExistsException.<init>:()V")]
    pub fn __impl_getFileSystem(
        &self,
        uri: URI,
    ) -> Result<crate::java::nio::file::FileSystem> {
        self.checkUri(uri)?;
        Ok(Clone::clone(&self.__get_theFileSystem()).into())
    }
}
