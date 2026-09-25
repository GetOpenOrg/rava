//! `jdk/internal/invoke/MhUtil` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK25 起 `AtomicBoolean` / `AtomicReference` / `FutureTask` 等的 `<clinit>` 不再
//! 直接调 `MethodHandles.lookup().findVarHandle(...)`，改经本工具类转发。按字节码
//! 照写：4 参形态委托 Lookup.findVarHandle；3 参形态以 `lookup.lookupClass()`
//! 为接收者类转 4 参形态。
//!
//! 取舍：JDK 在此捕获 `ReflectiveOperationException` 并包成 `InternalError(e)`
//! 重抛；这里直接传播原异常——成功路径逐字等价，失败路径只差包装类型（语料中
//! 字段均存在，该分支不可达）。

use crate::prelude::*;
use super::mh_util::MhUtil;
use crate::java::lang::Class;
use crate::java::lang::invoke::{MethodHandles_Lookup, VarHandle};

impl MhUtil {
    /// static `findVarHandle(Lookup, String, Class)`：
    /// `findVarHandle(lookup, lookup.lookupClass(), name, type)` 的 Lookup 直调形态。
    #[jvm_boundary(upcalls = "java/lang/invoke/MethodHandles$Lookup.lookupClass:()Ljava/lang/Class; java/lang/invoke/MethodHandles$Lookup.findVarHandle:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;")]
    pub fn findVarHandle_methodhandles_lookup_str_class(
        lookup: MethodHandles_Lookup,
        name: String,
        type_: Class,
    ) -> Result<VarHandle> {
        let recv = lookup.lookupClass()?;
        Self::findVarHandle_methodhandles_lookup_class_str_class(lookup, recv, name, type_)
    }

    /// static `findVarHandle(Lookup, Class recv, String, Class)`：显式接收者类形态
    ///（CompletableFuture 等对嵌套类字段取 VarHandle）——委托 Lookup.findVarHandle。
    #[jvm_boundary(upcalls = "java/lang/invoke/MethodHandles$Lookup.findVarHandle:(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/invoke/VarHandle;")]
    pub fn findVarHandle_methodhandles_lookup_class_str_class(
        lookup: MethodHandles_Lookup,
        recv: Class,
        name: String,
        type_: Class,
    ) -> Result<VarHandle> {
        lookup.findVarHandle(recv, name, type_)
    }
}
