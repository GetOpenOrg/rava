//! `jdk/internal/reflect/AccessorUtils` 的栈帧依赖判定（intrinsics.txt 第三类准入：栈帧查询点）。
//!
//! JDK 体经 `Throwable.getStackTrace` 判断 CCE / NPE / WMTE 的抛出点是否在访问器类或
//! java.base 句柄适配层（实参转换失败 → IllegalArgumentException）。原生二进制无 Java 栈帧，
//! 由 L3 分派登记的「目标抛出」身份等价应答：未经目标方法体逃逸者即实参转换失败。

use crate::prelude::*;
use super::accessor_utils::AccessorUtils;
use crate::java::lang::{Class, RuntimeException};

impl AccessorUtils {
    #[jvm_native]
    pub fn isIllegalArgument(_accessor_type: Class, e: RuntimeException) -> Result<bool> {
        Ok(!crate::reflect_dispatch::thrown_by_target(&Object::from(e)))
    }
}
