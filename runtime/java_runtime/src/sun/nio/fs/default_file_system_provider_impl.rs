//! `sun/nio/fs/DefaultFileSystemProvider` 手写伴生：POSIX 原生族档 A（切入序 2）。
//!
//! JVM 语义：类初始化时创建平台 provider 单例（`INSTANCE = new 平台Provider()`），
//! `instance()` 返回单例、`theFileSystem()` 返回其默认文件系统。本运行时以
//! thread_local 惰性单例承载（进程内一次构造，与 clinit 恰一次语义等价；
//! 单线程 main 模型下无可观测差异）。
//!
//! 平台差异（报告 §4 ①）：单例类型随平台 JDK classfile 变——
//! - macOS：`MacOSXFileSystemProvider`（MacOSX → Bsd → Unix 三层）；
//! - Linux：`LinuxFileSystemProvider`（直接继承 UnixFileSystemProvider）。
//! 生成侧类型名随语料变化，impl 按 cfg(target_os) 分文件形态对齐；Linux 分支
//! 以 openjdk/jdk21u 源核对书写，待服务器轮编译验证（本机语料为 macOS）。

use crate::prelude::*;
use super::default_file_system_provider::DefaultFileSystemProvider;

#[cfg(target_os = "macos")]
mod platform {
    use crate::prelude::*;
    use crate::sun::nio::fs::MacOSXFileSystemProvider;

    /// 平台 provider 单例：macOS = MacOSXFileSystemProvider。
    /// 构造链语义见 mac_osx_file_system_provider_impl（theFileSystem 在 ctor 内建立）。
    crate::__process_static! {
        pub(crate) static INSTANCE: crate::sync_model::__RefSlot<std::option::Option<MacOSXFileSystemProvider>> =
            const { crate::sync_model::__RefSlot::new(std::option::Option::None) };
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use crate::prelude::*;
    use crate::sun::nio::fs::LinuxFileSystemProvider;

    /// 平台 provider 单例：Linux = LinuxFileSystemProvider（直接继承
    /// UnixFileSystemProvider，构造链同源）。
    crate::__process_static! {
        pub(crate) static INSTANCE: crate::sync_model::__RefSlot<std::option::Option<LinuxFileSystemProvider>> =
            const { crate::sync_model::__RefSlot::new(std::option::Option::None) };
    }
}

impl DefaultFileSystemProvider {
    /// `instance()`: 平台默认 provider 单例（首次触达时构造，等价 clinit 恰一次）。
    #[cfg(target_os = "macos")]
    #[jvm_boundary(upcalls = "sun/nio/fs/MacOSXFileSystemProvider.<init>:()V")]
    pub fn instance() -> Result<crate::sun::nio::fs::MacOSXFileSystemProvider> {
        platform::INSTANCE.with(|cell| {
            if crate::sync_model::__RefSlot::borrow(cell).is_none() {
                let provider = crate::sun::nio::fs::MacOSXFileSystemProvider::new()?;
                *cell.borrow_mut() = std::option::Option::Some(provider);
            }
            Ok(Clone::clone(cell.borrow().as_ref().unwrap()))
        })
    }

    /// `instance()`: Linux 形态（服务器轮验证）。
    #[cfg(target_os = "linux")]
    #[jvm_boundary(upcalls = "sun/nio/fs/LinuxFileSystemProvider.<init>:()V")]
    pub fn instance() -> Result<crate::sun::nio::fs::LinuxFileSystemProvider> {
        platform::INSTANCE.with(|cell| {
            if crate::sync_model::__RefSlot::borrow(cell).is_none() {
                let provider = crate::sun::nio::fs::LinuxFileSystemProvider::new()?;
                *cell.borrow_mut() = std::option::Option::Some(provider);
            }
            Ok(Clone::clone(cell.borrow().as_ref().unwrap()))
        })
    }

    /// `theFileSystem()`: 默认文件系统（JDK: INSTANCE.theFileSystem()）。
    /// 以 theFileSystem 字段访问器直取（macOS/Linux provider 未声明包装方法，
    /// 字段为 UnixFileSystemProvider 的继承平铺成员，子类 wrapper 可直访）。
    #[jvm_boundary(upcalls = "sun/nio/fs/DefaultFileSystemProvider.instance:()Lsun/nio/fs/LinuxFileSystemProvider;")]
    pub fn theFileSystem() -> Result<crate::java::nio::file::file_system::implref::FileSystem> {
        let provider = Self::instance()?;
        let fs = provider.__get_theFileSystem();
        Ok(Clone::clone(&fs).into())
    }
}
