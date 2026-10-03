//! `java/lang/StackStreamFactory` 的 ACC_NATIVE（类 1）。

use crate::prelude::*;
use super::stack_stream_factory::StackStreamFactory;

impl StackStreamFactory {
    /// native `checkStackWalkModes()`：HotSpot `JVM_SupportsStackWalkModes` 式核对——Java 侧模式位
    ///（DEFAULT_MODE / FILL_CLASS_REFS_ONLY / GET_CALLER_CLASS / SHOW_HIDDEN_FRAMES /
    /// FILL_LIVE_STACK_FRAMES）与本数据面（stack_stream_factory_abstract_stack_walker_impl.rs）同一编码，恒 true。
    #[jvm_native]
    pub fn checkStackWalkModes() -> Result<bool> {
        Ok(true)
    }
}
