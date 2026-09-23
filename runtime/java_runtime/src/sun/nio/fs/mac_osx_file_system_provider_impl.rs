//! `sun/nio/fs/MacOSXFileSystemProvider` 手写伴生：POSIX 原生族档 A（切入序 2，
//! macOS 平台薄层）。Linux 对应薄层（LinuxFileSystemProvider，直接继承
//! UnixFileSystemProvider）见 linux_file_system_provider_impl.rs。

use crate::prelude::*;
use super::mac_osx_file_system_provider::MacOSXFileSystemProvider;
use super::unix_file_system::UnixFileSystem;
use crate::java::lang::String;
use crate::jdk::internal::util::StaticProperty;

impl MacOSXFileSystemProvider {
    /// `<init>()`：JDK 构造链 MacOSX → Bsd → UnixFileSystemProvider，最终执行
    /// `theFileSystem = newFileSystem(StaticProperty.userDir())`——虚分派解析到
    /// 本类的 `newFileSystem(dir)` → `new MacOSXFileSystem(this, dir)`。
    ///
    /// 档 A 落差（如实记录）：MacOSXFileSystem / BsdFileSystem 不在 BFS 闭包，
    /// theFileSystem 以基类 `UnixFileSystem` 落地。MacOSXFileSystem 仅覆盖
    /// NFD 原生路径规范化（normalizeNativePath，非 ASCII 路径形态）与 mount
    /// 枚举（getMountEntries）——均在档 A 用例面（ASCII 路径 + exists/读写）
    /// 之外。NFD 差异留档 B（MacOSXNativeDispatcher.normalizepath）。
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

    /// `newFileSystem(String)`：macOS 平台覆写（返回 MacOSXFileSystem）。
    /// 档 A 以基类 UnixFileSystem 承载（落差见 new 的注释）；静态类型沿声明
    /// 形态返回 Object（MacOSXFileSystem 不在闭包）。
    #[jvm_boundary]
    pub fn newFileSystem_str(&self, dir: String) -> Result<Object> {
        let provider: super::unix_file_system_provider::UnixFileSystemProvider =
            Clone::clone(self).into();
        let fs = UnixFileSystem::new(provider, dir)?;
        Ok(Object::from(fs))
    }
}
