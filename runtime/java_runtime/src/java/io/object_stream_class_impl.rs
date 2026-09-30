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

    /// `private static native boolean hasStaticInitializer(Class<?>)`：类文件是否声明了 `<clinit>`
    /// （仅本类，不含父类——HotSpot 按类自身方法表查找）。默认 serialVersionUID 计算
    /// （`computeDefaultSUID`）的输入之一；数据源为 build.rs 的类元数据表。
    #[jvm_native]
    pub fn hasStaticInitializer(cl: crate::java::lang::Class) -> Result<bool> {
        Ok(cl.__has_static_initializer())
    }
}
