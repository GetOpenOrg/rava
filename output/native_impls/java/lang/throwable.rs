use java_runtime::prelude::*;
use super::*;

impl super::Throwable {
    // 单线程转译环境无栈帧信息；返回默认实例（no-op 实现）
    pub fn fillInStackTrace(&self) -> Result<Throwable> {
        Ok(Throwable::default())
    }
}
