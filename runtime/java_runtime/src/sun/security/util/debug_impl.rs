//! `sun/security/util/Debug` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 语义：`Debug.getInstance(option[, prefix])` 仅当系统属性 `java.security.debug`
//! 开启对应选项时返回实例，否则返回 null；各安全组件以 `debug != null` 守卫调试输出。
//! 原生二进制不读取该调试属性（等价于未设置）：恒返回 null、`isOn` / `isVerbose` 恒 false。
//! 消费方：javax.crypto / java.security 各类的 static `debug` 字段初始化（Rosetta
//! DataEncryptionStandard 实证）。

use crate::prelude::*;
use super::debug::Debug;

impl Debug {
    /// `getInstance(String)`：调试未开启 → null。
    #[jvm_boundary]
    pub fn getInstance_str(_option: String) -> Result<Debug> {
        Ok(Debug::default())
    }

    /// `getInstance(String, String)`：同上。
    #[jvm_boundary]
    pub fn getInstance_str_str(_option: String, _prefix: String) -> Result<Debug> {
        Ok(Debug::default())
    }

    /// `isOn(String)`：调试未开启 → false。
    #[jvm_boundary]
    pub fn isOn(_option: String) -> Result<bool> {
        Ok(false)
    }

    /// `isVerbose()`：同上。
    #[jvm_boundary]
    pub fn isVerbose() -> Result<bool> {
        Ok(false)
    }
}
