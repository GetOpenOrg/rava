//! `java/util/concurrent/atomic/AtomicLong` 手写 native 实现。

use crate::prelude::*;
use super::*;

impl AtomicLong {
    /// native VMSupportsCS8：JVM 是否支持 8 字节原子 compare-and-swap。
    /// HotSpot 在 64 位平台恒为 true（VM_Version::supports_cx8）；
    /// 原生二进制同理——目标平台均为 64 位。
    #[jvm_native]
    pub fn VMSupportsCS8() -> Result<bool> {
        Ok(true)
    }
}
