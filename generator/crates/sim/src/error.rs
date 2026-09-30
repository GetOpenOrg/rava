//! 栈模拟错误：未移植的语义与不合法的输入显式报错，不 panic。

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimError {
    /// Java 名字安全化后仍不是合法 Rust 标识符（如含非 ASCII 字母）
    BadIdent(String),
    /// IR 节点构造失败（非法字面量文本等，`ir::IrError` 除标识符外的变体）
    Ir(String),
    /// 外部钩子（[`crate::SimEnv`]）报告的失败
    Env(String),
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimError::BadIdent(s) => write!(f, "非法 Rust 标识符：{s:?}"),
            SimError::Ir(s) => write!(f, "IR 构造失败：{s}"),
            SimError::Env(s) => write!(f, "栈模拟环境钩子失败：{s}"),
        }
    }
}

impl std::error::Error for SimError {}

impl From<ir::IrError> for SimError {
    fn from(e: ir::IrError) -> SimError {
        match e {
            ir::IrError::BadIdent(s) => SimError::BadIdent(s),
            other => SimError::Ir(other.to_string()),
        }
    }
}

pub type SimResult<T> = Result<T, SimError>;
