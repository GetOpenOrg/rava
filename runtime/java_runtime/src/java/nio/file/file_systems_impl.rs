//! `java/nio/file/FileSystems` 手写伴生：POSIX 原生族档 A（见 vm_boundary.txt
//! 的收录理由）。按用例面实现 getDefault（Path.of 的入口），其余成员保持
//! panic 存根。

use crate::prelude::*;
use super::file_systems::FileSystems;
use crate::java::nio::file::file_system::implref::FileSystem;
use crate::sun::nio::fs::DefaultFileSystemProvider;

impl FileSystems {
    /// `getDefault()`：默认文件系统。JDK 语义 = DefaultFileSystemHolder.
    /// defaultFileSystem（doPrivileged 下 getDefaultProvider().getFileSystem(
    /// URI("file:///"))）——无安全管理器时特权动作透明，等价于直接返回平台
    /// provider 的 theFileSystem。
    #[jvm_boundary]
    pub fn getDefault() -> Result<FileSystem> {
        DefaultFileSystemProvider::theFileSystem()
    }
}
