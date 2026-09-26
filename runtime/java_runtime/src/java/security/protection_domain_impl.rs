//! `java/security/ProtectionDomain` 手写伴生（java/security/ 内部边界包，按调用链按需实现）。
//! 消费方同 permissions_impl.rs（ForkJoinPool 工作线程 AccessControlContext）；
//! SecurityManager 恒 null，保护域从不参与检查，构造只产生对象身份。
use crate::prelude::*;
use super::protection_domain::ProtectionDomain;
use super::{CodeSource, PermissionCollection};

impl ProtectionDomain {
    /// `<init>(CodeSource, PermissionCollection)`。
    #[jvm_boundary]
    pub fn new_codesource_permissioncollection(_codesource: CodeSource, _permissions: PermissionCollection) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }
}
