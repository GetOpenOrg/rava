use crate::prelude::*;
use super::vm::VM;
use crate::java::lang::String;

impl VM {
    /// 原生二进制进入 main 时运行时已完成初始化（对应 initLevel == SYSTEM_BOOTED）。
    #[jvm_boundary]
    pub fn isBooted() -> Result<bool> {
        Ok(true)
    }

    /// `VM.getSavedProperty(key)`：启动时保存的 JVM 内部属性。
    ///
    /// 保存属性仅在指定 `-XX:` / 归档选项时存在；原生二进制无启动选项，
    /// 所有查询都未设置 → null（如 Integer$IntegerCache 的
    /// `java.lang.Integer.IntegerCache.high`，null 表示沿用默认上界 127）。
    #[jvm_boundary]
    pub fn getSavedProperty(_key: String) -> Result<String> {
        Ok(String::default())
    }
}
