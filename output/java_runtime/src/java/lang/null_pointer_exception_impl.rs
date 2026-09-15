use crate::prelude::*;
use super::*;

impl NullPointerException {
    // JVM 扩展 NPE 消息需要 HotSpot 内部支持；转译环境返回空字符串
    #[jvm_native]
    pub fn getExtendedNPEMessage(&self) -> Result<String> {
        Ok(String::default())
    }
}
