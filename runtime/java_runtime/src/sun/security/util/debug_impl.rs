//! `sun/security/util/Debug` 手写伴生。
//!
//! JDK 语义：`Debug.getInstance(option[, prefix])` 仅当系统属性 `java.security.debug`
//! 开启对应选项时返回实例，否则返回 null；各安全组件以 `debug != null` 守卫调试输出。
//! 原生二进制不设该调试属性：恒返回 null、`isOn` / `isVerbose` 恒 false。
//!
//! 【过渡手写，非准入三类】包已按字节码放行（C1d），本文件四个方法仍覆盖字节码，阻塞在闭包分析精度：
//! 分析器不能把 `java.security.debug` 未设折叠为 `getInstance` 恒 null，删除后 Debug 被视为可分配，
//! 所有 `debug != null` 守卫下的 println → getCallerInfo（StackWalker）/ FormatHolder（DateTimeFormatter）
//! 等路径全部入链：CollectorsDemo 实测 +78 类 / +650 方法、新增 3 个缺失 native（StackStreamFactory
//! callStackWalk / checkStackWalkModes、StackTraceElement.initStackTraceElement），且 Debug 位于 418 / 758
//! 个通过用例的闭包内。待分析器支持系统属性常量（未设属性 → null）后删除本文件。
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
