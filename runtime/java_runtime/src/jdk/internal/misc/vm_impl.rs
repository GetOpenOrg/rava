use crate::prelude::*;
use super::vm::VM;

impl VM {
    /// 原生二进制进入 main 时运行时已完成初始化（对应 initLevel == SYSTEM_BOOTED）。
    #[jvm_boundary]
    pub fn isBooted() -> Result<bool> {
        Ok(true)
    }
}
