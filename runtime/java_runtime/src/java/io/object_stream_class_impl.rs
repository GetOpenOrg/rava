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
    /// （`computeDefaultSUID`）的输入之一；数据源为 java_meta 的类元数据表。
    /// HotSpot 以 JNI `GetStaticMethodID` 查找，查找前初始化该类（及超类）：同样先按名初始化再答复
    /// （闭包按 `[facts.reflect] class_initializers` 登记的同一入口把目标类导出为初始化钩子）。
    #[jvm_native]
    pub fn hasStaticInitializer(cl: crate::java::lang::Class) -> Result<bool> {
        let name = format!("{}", cl.__get_name());
        crate::ensure_class_initialized(&name)?;
        Ok(cl.__has_static_initializer())
    }
}
