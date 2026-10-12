//! `jdk/internal/misc/ScopedMemoryAccess` 的 ACC_NATIVE（类 1）。其余方法按字节码翻译。

use crate::jdk::internal::misc::ScopedMemoryAccess__Methods as _;
use crate::prelude::*;
use super::scoped_memory_access::ScopedMemoryAccess;
use super::__ScopedMemoryAccessHw;

impl __ScopedMemoryAccessHw for ScopedMemoryAccess {
    /// native `registerNatives()`：HotSpot 登记 closeScope0 的 JNI 入口；原生二进制按名链接 → no-op。
    #[jvm_native]
    fn registerNatives() -> Result<()> {
        Ok(())
    }
}
