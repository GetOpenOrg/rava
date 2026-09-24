//! `java/io/ObjectStreamClass` 手写伴生补充：JUnit Runner 路径的 Result
//! 类初始化触达（`serialPersistentFields` 计算链的 `lookup` → clinit）。

use crate::prelude::*;
use super::object_stream_class::ObjectStreamClass;

impl ObjectStreamClass {
    /// `private static native void initNative()`：JDK 的原生占位（ObjectStreamClass.c
    /// 中为空实现，仅触发库加载）——无操作即 JVM 等价。
    pub fn initNative() -> Result<()> {
        Ok(())
    }
}
