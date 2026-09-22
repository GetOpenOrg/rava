//! `sun/invoke/util/VerifyAccess` 手写伴生：内部边界类，按调用链按需实现
//! （K-2 规则），其余保持 panic 存根。
//!
//! 消费链：MethodHandles 的 Lookup 访问检查族（checkSymbolicClass /
//! checkAccess 经 isClassAccessible）。JDK 语义核心是 JPMS 模块导出检查；
//! 原生二进制无模块系统——全部类位于未命名模块（对所有人开放），模块边
//! 界恒穿透。无模块世界的等价语义：allowedModes == 0（_lookup 已被吊销）
//! 不可访问，其余恒可访问。

use crate::prelude::*;
use super::verify_access::VerifyAccess;
use crate::java::lang::Class;

impl VerifyAccess {
    /// static `isClassAccessible(refc, lookupClass, prevLookupClass, allowedModes)`：
    /// 模块导出检查的未命名模块等价物——无模块系统时唯一不可访问形态是
    /// allowedModes == 0（Lookup.revoked 后任何符号类都不可达）。
    pub fn isClassAccessible(_refc: Class, _lookupClass: Class, _prevLookupClass: Class, allowedModes: i32) -> Result<bool> {
        Ok(allowedModes != 0)
    }
}
