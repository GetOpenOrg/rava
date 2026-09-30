//! 指令翻译错误：未移植的分支与不合法输入显式报错，不 panic、不静默。

use std::fmt;

use sim::SimError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrError {
    /// 栈模拟层错误（标识符非法、下层未移植语义）
    Sim(SimError),
    /// 合法但 javac 不产出的字节码形态（`ldc` MethodType / MethodHandle / 动态常量）：
    /// 生成期显式报错，不发射占位
    OutOfScope(String),
    /// 指令操作数与操作码不符 / 常量池形态不合法
    BadInsn(String),
}

impl fmt::Display for InstrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstrError::Sim(e) => write!(f, "{e}"),
            InstrError::OutOfScope(s) => write!(f, "超出 javac 产出范围的字节码形态：{s}"),
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
