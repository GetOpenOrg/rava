//! `java/security/Permission` 手写伴生：java/security/ 为内部边界包，按调用链按需实现（K-2 规则）。
//!
//! 消费方：`CryptoAllPermission.<clinit>` → `CryptoPermission(String)` → `super(name)`
//! （Cipher.init 取 unlimited 策略的权限单例，K-JCA）。SecurityManager 恒 null，权限对象
//! 从不参与检查——构造只承载 name（JDK `Permission(String)` 的全部语义）。

use crate::prelude::*;
use super::permission::implref::Permission;

impl Permission {
    /// `<init>(String name)`。
    #[jvm_boundary]
    pub fn new(name: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on(this, name)
    }

    /// 构造器双入口的 `this` 形态（子类 invokespecial super(name) 的落点）。
    #[doc(hidden)]
    pub fn __init_on(this: Self, name: String) -> Result<Self> {
        this.__set_name(name);
        Ok(this)
    }

    /// `getName()`。
    pub fn __impl_getName(&self) -> Result<String> {
        Ok(self.__get_name())
    }
}
