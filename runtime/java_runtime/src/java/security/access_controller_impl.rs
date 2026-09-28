//! `java/security/AccessController` 的 native 方法（FS-H0：doPrivileged / getContext 回到 JDK
//! 字节码，closure.toml [release] 放行本类与 AccessControlContext）。
//!
//! 原生二进制无安全管理器（JDK 21 的 SecurityManager 恒 null）且无 Java 栈帧：
//! 栈上 / 继承的访问控制上下文恒为 null，保护域不建模（启动类同 HotSpot 返回 null），
//! 栈遍历物化是 JIT 协作点——均为 HotSpot 在未安装 SM 时的等价应答。

use crate::prelude::*;
use super::AccessController;
use super::AccessControlContext;
use super::ProtectionDomain;
use crate::java::lang::Class;

impl AccessController {
    /// 调用栈上的特权上下文：无 SM / 无 Java 栈帧 → null（JDK getContext 据此构造空上下文）。
    #[jvm_native]
    pub fn getStackAccessControlContext() -> Result<AccessControlContext> {
        Ok(AccessControlContext::default())
    }

    /// 线程继承的上下文：同上 → null。
    #[jvm_native]
    pub fn getInheritedAccessControlContext() -> Result<AccessControlContext> {
        Ok(AccessControlContext::default())
    }

    /// 类的保护域：原生二进制不建模类加载器 / 代码源 → null（HotSpot 对启动类同此）。
    #[jvm_native]
    pub fn getProtectionDomain(_caller: Class) -> Result<ProtectionDomain> {
        Ok(ProtectionDomain::default())
    }

    /// 栈遍历物化（保证 JIT 不消去实参）：原生二进制无栈遍历 → 空操作。
    #[jvm_native]
    pub fn ensureMaterializedForStackWalk(_o: Object) -> Result<()> {
        Ok(())
    }
}
