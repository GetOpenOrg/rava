//! `java/lang/StackStreamFactory` 的 ACC_NATIVE（类 1）。

use crate::prelude::*;
use super::stack_stream_factory::StackStreamFactory;

impl StackStreamFactory {
    /// native `checkStackWalkModes()`：HotSpot 校验 Java 侧 walk 模式位与 VM 常量一致；
    /// 本运行时的栈遍历按 Java 侧常量实现 → 恒一致。
    #[jvm_native]
    pub fn checkStackWalkModes() -> Result<bool> {
        Ok(true)
    }
}
