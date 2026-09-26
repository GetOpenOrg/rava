//! `java/security/Permissions` 手写伴生：java/security/ 为内部边界包（boundary_prefixes.txt），
//! 按调用链按需实现（K-2 规则）。
//!
//! 消费方：ForkJoinPool$DefaultForkJoinWorkerThreadFactory.newRegularWithACC——为工作线程
//! 构造带 modifyThread / enableContextClassLoaderOverride 权限的 AccessControlContext。
//! SecurityManager 恒 null（JDK 21 缺省 disallow，24+ 已移除），权限集合从不参与检查：
//! 构造与 add 只承载容器语义，不建模按类型分桶的 permsMap。
use crate::prelude::*;
use super::permissions::Permissions;
use super::Permission;

impl Permissions {
    /// `<init>()`：空权限集合。
    #[jvm_boundary]
    pub fn new() -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }

    /// `add(Permission)`：权限入集合（无检查消费方，不建模存储）。
    pub fn __impl_add(&self, _permission: Permission) -> Result<()> {
        Ok(())
    }
}
