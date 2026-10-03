//! `jdk/internal/vm/ContinuationSupport` 的 ACC_NATIVE（类 1）。`isSupported()` 按字节码翻译
//!（读 `SUPPORTED` 静态常量，由本 native 初始化）。

use crate::prelude::*;
use super::continuation_support::ContinuationSupport;

impl ContinuationSupport {
    /// native `isSupported0()`：VM 是否支持 continuation（`-XX:+VMContinuations`）。
    /// 虚拟线程由 VM 契约类 `VirtualThread` 承载（映射 OS 线程，closure.toml `[vm_boundary]`），
    /// `ThreadBuilders.newVirtualThread` 须走 `VirtualThread` 分支 → true。
    #[jvm_native]
    pub fn isSupported0() -> Result<bool> {
        Ok(true)
    }
}
