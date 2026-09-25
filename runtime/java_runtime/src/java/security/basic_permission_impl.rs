//! `java/security/BasicPermission` 手写伴生：java/security/ 为内部边界包
//! （runtime/java_runtime/boundary_prefixes.txt），按调用链按需实现（K-2 规则）。
//!
//! 消费方：`ThreadPerTaskExecutor.<clinit>` 的 `new RuntimePermission("modifyThread")`
//! （RuntimePermission 为 java/lang 生成类，super(name) 落本类）。本档位
//! SecurityManager 恒 null（JDK 24+ 已移除），权限对象从不参与检查——构造只需
//! 承载 Permission.name（getName 可观察）；wildcard / path 的名字解析（init）
//! 无消费方，不建模。

use crate::prelude::*;
use super::basic_permission::BasicPermission;

impl BasicPermission {
    /// `<init>(String name)`。
    #[jvm_boundary]
    pub fn new_str(name: String) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        Self::__init_on_str(this, name)
    }

    /// 构造器双入口的 `this` 形态（子类 invokespecial super(name) 的落点）。
    #[doc(hidden)]
    pub fn __init_on_str(this: Self, name: String) -> Result<Self> {
        this.__set_name(name);
        Ok(this)
    }
}
