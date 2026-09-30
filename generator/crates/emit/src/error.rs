//! 发射层错误。

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {
    /// 文件系统错误
    Io(String),
    /// 输入不一致（registry 缺类、清单格式等）
    Input(String),
    /// 方法体生成器的硬失败（非兜底类异常，穿透）
    Body(String),
    /// 生成期断言失败（G-10 账本等）
    Assert(String),
}

impl fmt::Display for EmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmitError::Io(s) => write!(f, "发射层读写失败：{s}"),
            EmitError::Input(s) => write!(f, "发射层输入错误：{s}"),
            EmitError::Body(s) => write!(f, "方法体生成失败：{s}"),
            EmitError::Assert(s) => write!(f, "生成期断言失败：{s}"),
        }
    }
}

impl std::error::Error for EmitError {}

pub type Result<T> = std::result::Result<T, EmitError>;

pub(crate) fn io_err(what: &str, e: std::io::Error) -> EmitError {
    EmitError::Io(format!("{what}：{e}"))
}
