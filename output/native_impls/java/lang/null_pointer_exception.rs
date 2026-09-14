use java_runtime::prelude::*;
use super::*;
use crate::java::lang::*;

impl super::NullPointerException {
    // JVM 扩展 NPE 消息需要 HotSpot 内部支持；转译环境返回空字符串
    pub fn getExtendedNPEMessage(&self) -> Result<String> {
        Ok(String::default())
    }
}
