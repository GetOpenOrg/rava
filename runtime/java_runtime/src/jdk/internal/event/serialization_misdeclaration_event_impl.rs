use crate::prelude::*;
use super::SerializationMisdeclarationEvent;

// jdk.internal.event.SerializationMisdeclarationEvent（JDK 25：ObjectStreamClass 序列化成员
// 声明错误的 JFR 事件，java.base 占位层）：事件系统未启用（原生二进制无 JFR），与
// DeserializationEvent / Event 同一处置——恒不启用，提交为 no-op。

impl SerializationMisdeclarationEvent {
    /// static `enabled()`：JFR 未启用 → false（ObjectStreamClass 据此跳过声明检查的事件路径）。
    #[jvm_boundary]
    pub fn enabled() -> Result<bool> {
        Ok(false)
    }

    /// static `shouldCommit(long)`：恒不提交。
    #[jvm_boundary]
    pub fn shouldCommit(_start: i64) -> Result<bool> {
        Ok(false)
    }

    /// static `timestamp()`：无事件时钟 → 0。
    #[jvm_boundary]
    pub fn timestamp() -> Result<i64> {
        Ok(0)
    }

    /// static `commit(long, Class, String)`：no-op。
    #[jvm_boundary]
    pub fn commit(_start: i64, _misdeclared: crate::java::lang::Class, _message: String) -> Result<()> {
        Ok(())
    }
}
