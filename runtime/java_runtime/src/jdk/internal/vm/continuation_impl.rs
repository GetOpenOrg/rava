//! `jdk/internal/vm/Continuation` 的 ACC_NATIVE（类 1）。其余方法按字节码翻译。

use crate::prelude::*;
use super::continuation::Continuation;

impl Continuation {
    /// native `registerNatives()`：HotSpot 登记 enterSpecial / doYield 等入口；原生二进制按名链接 → no-op。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }
}
