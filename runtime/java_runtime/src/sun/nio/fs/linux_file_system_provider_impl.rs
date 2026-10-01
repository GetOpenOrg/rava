//! `sun/nio/fs/LinuxFileSystemProvider` 手写伴生：POSIX 原生族档 A（切入序 2，
//! Linux 平台薄层）。macOS 对应薄层（MacOSXFileSystemProvider，经 Bsd 中继）
//! 见 mac_osx_file_system_provider_impl.rs。

use crate::prelude::*;
use super::linux_file_system_provider::LinuxFileSystemProvider;
use super::unix_file_system::implref::UnixFileSystem;
use crate::java::lang::String;
use crate::jdk::internal::util::StaticProperty;

impl LinuxFileSystemProvider {
    /// `<init>()`：JDK 构造链 Linux → UnixFileSystemProvider（无 Bsd 中继，
    /// Linux 特化的最短链），基类构造执行
    /// `theFileSystem = newFileSystem(StaticProperty.userDir())`——虚分派解析到
    /// 本类的 `newFileSystem(dir)` → `new LinuxFileSystem(this, dir)`。
    ///
    /// 档 A 落差（与 macOS 薄层同口径）：LinuxFileSystem 不在 BFS 闭包，
    /// theFileSystem 以基类 `UnixFileSystem` 落地。LinuxFileSystem 相对基类的
    /// 增量（getMountEntries 读 /proc/self/mounts、top/usage 挂 statvfs、
    /// defaultDirectory 与 /proc 同设备的解析豁免）均在档 A 用例面（ASCII
    /// 路径 + exists/读写）之外。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        let dir = StaticProperty::USER_DIR()?;
        let fs = this.newFileSystem_str(dir)?;
        this.__set_theFileSystem(
            Clone::clone(&fs).try_cast::<UnixFileSystem>("sun/nio/fs/UnixFileSystem")?,
        );
        Ok(this)
    }

    /// `newFileSystem(String)`：Linux 平台覆写（返回 LinuxFileSystem）。
    /// 档 A 以基类 UnixFileSystem 承载（落差见 new 的注释）；静态类型沿声明
    /// 形态返回 Object（LinuxFileSystem 不在闭包）。
    #[jvm_boundary]
    pub fn newFileSystem_str(&self, dir: String) -> Result<Object> {
        let provider: super::unix_file_system_provider::UnixFileSystemProvider =
            Clone::clone(self).into();
        let fs = UnixFileSystem::new(provider, dir)?;
        Ok(Object::from(fs))
    }
}
