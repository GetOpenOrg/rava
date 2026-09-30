//! IR 构造期错误：不合法的标识符 / 字面量记号在构造时拒绝，而不是渲染出非法 Rust。

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrError {
    /// 不是合法的 Rust 标识符（`[A-Za-z_][A-Za-z0-9_]*` 或 `r#` 前缀形态）
    BadIdent(String),
    /// 浮点字面量记号不是十进制数字形态
    BadFloat(String),
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrError::BadIdent(s) => write!(f, "非法 Rust 标识符：{s:?}"),
            IrError::BadFloat(s) => write!(f, "非法浮点字面量记号：{s:?}"),
        }
    }
}

impl std::error::Error for IrError {}
