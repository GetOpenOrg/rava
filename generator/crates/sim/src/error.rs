//! 栈模拟错误：未移植的语义与不合法的输入显式报错，不 panic。

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimError {
    /// Java 名字安全化后仍不是合法 Rust 标识符（如含非 ASCII 字母）
    BadIdent(String),
    /// 输入形态在本移植中无对应语义（Python 侧只在理论输入上可达的分支等）
    Unported(String),
    /// 外部钩子（[`crate::SimEnv`]）报告的失败
    Env(String),
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimError::BadIdent(s) => write!(f, "非法 Rust 标识符：{s:?}"),
            SimError::Unported(s) => write!(f, "栈模拟未移植的语义：{s}"),
            SimError::Env(s) => write!(f, "栈模拟环境钩子失败：{s}"),
        }
    }
}

impl std::error::Error for SimError {}

impl From<ir::IrError> for SimError {
    fn from(e: ir::IrError) -> SimError {
        match e {
            ir::IrError::BadIdent(s) => SimError::BadIdent(s),
            other => SimError::Unported(other.to_string()),
        }
    }
}

pub type SimResult<T> = Result<T, SimError>;
