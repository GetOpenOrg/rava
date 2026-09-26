//! `sun/reflect/misc/ReflectUtil` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! `checkPackageAccess` 是 SecurityManager 下的包访问检查（`sm == null` 时 JDK 即空操作）；
//! 原生二进制无 SecurityManager（JEP 486）→ 空操作。消费方：MethodHandles.Lookup（MH-native）。

use crate::prelude::*;
use super::reflect_util::implref::ReflectUtil;
use crate::java::lang::Class;

impl ReflectUtil {
    #[jvm_boundary]
    pub fn checkPackageAccess_class(_clazz: Class) -> Result<()> {
        Ok(())
    }

    #[jvm_boundary]
    pub fn checkPackageAccess_str(_name: String) -> Result<()> {
        Ok(())
    }
}
