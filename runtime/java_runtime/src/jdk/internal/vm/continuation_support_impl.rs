//! `jdk/internal/vm/ContinuationSupport` 的 ACC_NATIVE（类 1）。`isSupported()` 按字节码翻译
//!（读 `SUPPORTED` 静态常量，由本 native 初始化）。

use crate::prelude::*;
use super::continuation_support::ContinuationSupport;

impl ContinuationSupport {
    /// native `isSupported0()`：VM 是否支持 continuation（`-XX:+VMContinuations`）。
    /// Continuation 由有栈协程承载（`continuation_impl.rs`）→ true：`ThreadBuilders.newVirtualThread`
    /// 走 `VirtualThread` 分支（false 时退化为 `BoundVirtualThread`，每个虚拟线程一条 OS 线程）。
    #[jvm_native]
    pub fn isSupported0() -> Result<bool> {
        Ok(true)
    }
}
