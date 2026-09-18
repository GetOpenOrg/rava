use crate::prelude::*;
use super::*;

impl Throwable {
    /// native fillInStackTrace(int)：原生二进制不维护 Java 栈帧记录，backtrace 保持为空。
    #[jvm_native]
    pub fn fillInStackTrace_i(&self, _dummy: i32) -> Result<Throwable> {
        Ok(Clone::clone(self))
    }
}
