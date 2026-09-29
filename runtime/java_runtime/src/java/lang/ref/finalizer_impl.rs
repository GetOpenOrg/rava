//! `java/lang/ref/Finalizer` 的 native 方法（与生成的 finalizer.rs 共置）。
//!
//! 原生二进制以引用计数即时回收，不存在 finalize() 队列与 finalizer 线程：
//! 终结化恒禁用（等价 JDK 18+ 的 `--finalization=disabled`）。

use crate::prelude::*;
use super::finalizer::Finalizer;

impl Finalizer {
    /// native `isFinalizationEnabled()`：false——Finalizer.<clinit> 据此不启动 finalizer 线程，
    /// register 不登记（JDK 禁用终结化的同一路径）。
    #[jvm_native]
    pub fn isFinalizationEnabled() -> Result<bool> {
        Ok(false)
    }

    /// native `reportComplete(Object)`：JFR / 诊断计数——no-op。
    #[jvm_native]
    pub fn reportComplete(_finalizee: Object) -> Result<()> {
        Ok(())
    }
}
