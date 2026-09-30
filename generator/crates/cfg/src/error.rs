//! 错误类型（← `graph.CfgError` / `audit.CfgAuditError`）。

use std::fmt;

/// 控制流图无法构建 / 无法结构化（调用方把方法退化为 stub，并计入统计）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgError(pub String);

impl CfgError {
    pub fn new(msg: impl Into<String>) -> CfgError {
        CfgError(msg.into())
    }
}

impl fmt::Display for CfgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CfgError {}

/// 存在未被消费的跳转指令 / 结构树自检失败。必须中止转译，不得被 stub 降级吞掉。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CfgAuditError(pub String);

impl fmt::Display for CfgAuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CfgAuditError {}
