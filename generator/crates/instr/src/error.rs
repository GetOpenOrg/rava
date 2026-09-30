//! 指令翻译错误：未移植的分支与不合法输入显式报错，不 panic、不静默。

use std::fmt;

use sim::SimError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrError {
    /// 栈模拟层错误（标识符非法、下层未移植语义）
    Sim(SimError),
    /// 本移植未覆盖的 Python 分支（登记于 `GOLDEN_DIFF.md`）
    Unported(String),
    /// 指令操作数与操作码不符 / 常量池形态不合法
    BadInsn(String),
}

impl fmt::Display for InstrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrError::Sim(e) => write!(f, "{e}"),
            InstrError::Unported(s) => write!(f, "指令翻译未移植的分支：{s}"),
            InstrError::BadInsn(s) => write!(f, "指令形态不合法：{s}"),
        }
    }
}

impl std::error::Error for InstrError {}

impl From<SimError> for InstrError {
    fn from(e: SimError) -> InstrError {
        InstrError::Sim(e)
    }
}

impl From<ir::IrError> for InstrError {
    fn from(e: ir::IrError) -> InstrError {
        InstrError::Sim(SimError::from(e))
    }
}

pub type InstrResult<T> = Result<T, InstrError>;

/// 未移植分支的错误构造（统一前缀，便于 golden 分类计数）
pub fn unported<T>(what: impl Into<String>) -> InstrResult<T> {
    Err(InstrError::Unported(what.into()))
}
