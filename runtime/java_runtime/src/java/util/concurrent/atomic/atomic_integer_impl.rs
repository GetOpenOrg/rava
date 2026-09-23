//! `java/util/concurrent/atomic/AtomicInteger` 手写补全（按 JDK 源语义）。

use crate::prelude::*;
use super::*;

impl AtomicInteger {
    /// `<init>(int initialValue)`：JDK 21 源即 `value = initialValue;`
    /// （该构造器在翻译层停留存根，按源语义补全）。消费方：
    /// `ClassFileDumper.getInstance` 的 dump 计数器初值（MethodHandles$
    /// Lookup.<clinit> 链）。
    #[jvm_boundary]
    pub fn new_i(initialValue: i32) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_value(initialValue);
        Ok(this)
    }
}
