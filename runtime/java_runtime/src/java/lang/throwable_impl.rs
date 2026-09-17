use crate::prelude::*;
use super::*;

impl Throwable {
    // 单线程转译环境无栈帧信息；返回默认实例（no-op 实现）
    #[jvm_native]
    pub fn fillInStackTrace(&self) -> Result<Throwable> {
        Ok(Throwable::default())
    }

    // 未触发异常路径时从不被读取；返回哨兵占位（Object::default() = unit rc）
    #[jvm_native]
    pub fn SUPPRESSED_SENTINEL() -> Object {
        Object::default()
    }
}
