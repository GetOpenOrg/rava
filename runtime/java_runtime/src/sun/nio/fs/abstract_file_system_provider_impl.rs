//! `sun/nio/fs/AbstractFileSystemProvider` 手写伴生：POSIX 原生族档 A
//! （切入序 5）。delete/deleteIfExists 的 final 壳委托 implDelete——虚分派
//! 经接收者具体类（macOS/Linux provider）的 vtable 槽位转发到本层 base 函数；
//! implDelete 在 UnixFileSystemProvider 声明（子类未覆盖），此处经
//! UnixFileSystemProvider 视图直调其手写体（JVM 方法解析的精确命中形态，
//! 不经子类空槽分派）。

use crate::prelude::*;
use super::abstract_file_system_provider::AbstractFileSystemProvider;
use super::unix_file_system_provider::UnixFileSystemProvider;

impl AbstractFileSystemProvider {
    /// `delete(Path)`：`implDelete(file, true)`。
    #[jvm_boundary]
    pub fn __impl_delete(&self, file: Object) -> Result<()> {
        let provider: UnixFileSystemProvider =
            Object::from(Clone::clone(self)).try_cast::<UnixFileSystemProvider>(
                "sun/nio/fs/UnixFileSystemProvider",
            )?;
        provider.__impl_implDelete(Clone::clone(&file), true)?;
        Ok(())
    }

    /// `deleteIfExists(Path)`：`implDelete(file, false)`。
    #[jvm_boundary]
    pub fn __impl_deleteIfExists(&self, file: Object) -> Result<bool> {
        let provider: UnixFileSystemProvider =
            Object::from(Clone::clone(self)).try_cast::<UnixFileSystemProvider>(
                "sun/nio/fs/UnixFileSystemProvider",
            )?;
        provider.__impl_implDelete(Clone::clone(&file), false)
    }
}
