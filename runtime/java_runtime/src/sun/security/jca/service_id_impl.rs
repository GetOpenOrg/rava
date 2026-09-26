//! `sun/security/jca/ServiceId` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! K-JCA：(服务类型, 算法) 二元组，`Cipher.getInstance` 按 transformation 候选逐个构造后交给
//! `GetInstance.getServices(List<ServiceId>)`（见 `get_instance_impl.rs`）。

use crate::prelude::*;
use super::service_id::implref::ServiceId;

impl ServiceId {
    /// `<init>(String type, String algorithm)`。
    #[jvm_boundary]
    pub fn new(type_: String, algorithm: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on(this, type_, algorithm)
    }

    #[doc(hidden)]
    pub fn __init_on(this: Self, type_: String, algorithm: String) -> Result<Self> {
        this.__set_type_(type_);
        this.__set_algorithm(algorithm);
        Ok(this)
    }
}
