//! `java/security/AccessControlContext` 手写伴生（java/security/ 内部边界包，按调用链按需实现）。
//! 消费方同 permissions_impl.rs；SecurityManager 恒 null，上下文从不被检查。
use crate::prelude::*;
use super::access_control_context::AccessControlContext;
use super::ProtectionDomain;

impl AccessControlContext {
    /// `<init>(ProtectionDomain[])`。
    #[jvm_boundary]
    pub fn new_arr_protectiondomain(_context: JArray<ProtectionDomain>) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Ok(this)
    }
}
