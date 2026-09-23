//! `java/nio/file/spi/FileSystemProvider` 手写伴生：POSIX 原生族档 A。
//!
//! 抽象槽位的缺省承接：typed 调用点（静态类型 FileSystemProvider）对抽象
//! 声明的虚分派，平台 provider 子类（sun/ 存根）的继承成员需求登记不覆盖
//! （与 FileSystems 收编 vm_boundary 同一根因，见其注释）。isSameFile 在
//! 此以声明类默认体承接——本运行时唯一的具体 provider 族为 Unix 形态，
//! 经精确视图直调 UnixFileSystemProvider 的 JDK 语义体（观察行为等价；
//! 其他 provider 形态不在档 A 语料面）。

use crate::prelude::*;
use super::file_system_provider::FileSystemProvider;

impl FileSystemProvider {
    /// `isSameFile(Path, Path)`：委托 UnixFileSystemProvider 的 JDK 语义体。
    #[jvm_boundary]
    pub fn __impl_isSameFile(&self, obj1: Object, obj2: Object) -> Result<bool> {
        let provider: crate::sun::nio::fs::UnixFileSystemProvider =
            Object::from(Clone::clone(self)).try_cast::<crate::sun::nio::fs::UnixFileSystemProvider>(
                "sun/nio/fs/UnixFileSystemProvider",
            )?;
        provider.__impl_isSameFile(obj1, obj2)
    }
}
