//! 方法体生成错误。
//!
//! - [`MethodError::Cfg`]：控制流无法结构化 / 栈下溢等「控制流语义限制」（Python `CfgError`），
//!   调用方把方法退化为 `panic!("stub: ...")` 存根；
//! - [`MethodError::Audit`]：跳转账本 / 结构树自检失败（Python `CfgAuditError`），必须中止；
//! - [`MethodError::Instr`]：指令翻译失败（含未移植分支）；
//! - [`MethodError::Unported`]：本 crate 尚未移植的分支（显式报错，登记于 `GOLDEN_DIFF.md`）。

use std::fmt;

use cfg::{CfgAuditError, CfgError};
use instr::InstrError;
use sim::SimError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodError {
    Cfg(String),
    Audit(String),
    Instr(InstrError),
    Unported(String),
}

impl fmt::Display for MethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MethodError::Cfg(s) => write!(f, "CfgError: {s}"),
            MethodError::Audit(s) => write!(f, "CfgAuditError: {s}"),
            MethodError::Instr(e) => write!(f, "{e}"),
            MethodError::Unported(s) => write!(f, "方法体生成未移植的分支：{s}"),
        }
    }
}

impl std::error::Error for MethodError {}

impl From<CfgError> for MethodError {
    fn from(e: CfgError) -> MethodError {
        MethodError::Cfg(e.0)
    }
}

impl From<CfgAuditError> for MethodError {
    fn from(e: CfgAuditError) -> MethodError {
        MethodError::Audit(e.0)
    }
}

impl From<InstrError> for MethodError {
    fn from(e: InstrError) -> MethodError {
        MethodError::Instr(e)
    }
}

impl From<SimError> for MethodError {
    fn from(e: SimError) -> MethodError {
        MethodError::Instr(InstrError::Sim(e))
    }
}

impl From<ir::IrError> for MethodError {
    fn from(e: ir::IrError) -> MethodError {
        MethodError::Instr(InstrError::from(e))
    }
}

pub type MethodResult<T> = Result<T, MethodError>;

/// `CfgError` 的简写
pub fn cfg_err<T>(msg: impl Into<String>) -> MethodResult<T> {
    Err(MethodError::Cfg(msg.into()))
}

/// 未移植分支的简写
pub fn unported<T>(what: impl Into<String>) -> MethodResult<T> {
    Err(MethodError::Unported(what.into()))
}
