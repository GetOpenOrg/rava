//! `java.lang.Shutdown` 的 native 层：JVM 停机序列的 VM 底座。
//!
//! `System.exit` → `Runtime.exit` → `Shutdown.exit`（翻译体：运行 shutdown hook 槽位）→
//! `halt0`；`main` 正常返回时 `destroy_java_vm` 调 `Shutdown.shutdown()` 运行 hook（JVM
//! DestroyJavaVM 同序）。

use crate::prelude::*;
use super::*;

impl Shutdown {
    /// native `beforeHalt()`：JVM 在 halt 前通知 JVMTI / JFR 等 VM 服务——原生二进制无此类
    /// 服务 → no-op。
    #[jvm_native]
    pub fn beforeHalt() -> Result<()> {
        Ok(())
    }

    /// native `halt0(int)`：以 `status` 终止进程（输出经 FileOutputStream.writeBytes 直写 fd，
    /// 无进程内缓冲待刷）。
    #[jvm_native]
    pub fn halt0(status: i32) -> Result<()> {
        std::process::exit(status)
    }
}
