//! 发射层输入：从 classfile + 闭包分析结果构建发射所需的 Registry 与输入事实（P1）。
//!
//! 取代 Python `codegen/closure_input.py` / `closure_folds.py` / `vm_constants.py` /
//! `runtime_manifest.py` 的发射层消费部分与 `transpile.py` 的 registry 构建步骤。
//!
//! 终态是 `rava build` 单进程：闭包结果以 [`closure::Closure`] 在进程内传入
//! （[`facts::ClosureFacts::from_closure`]）；closure.json 只作调试入口
//! （[`facts::ClosureFacts::from_json`]）。

pub mod boundary;
pub mod build;
pub mod facts;
pub mod handwritten;
pub mod manifest;
pub mod norm;
pub mod plan;
pub mod prune;
mod scan_text;

#[cfg(test)]
mod unit_tests;

pub use boundary::Boundary;
pub use build::{BuildInput, EmitInput, LibCrate, MethodKey, ReflectFacts};
pub use facts::{ClosureFacts, MethodKind};
pub use manifest::RuntimeManifest;
pub use norm::{NInsn, NormCode};
pub use plan::{ClassPlan, MethodPlan, Planner, Role, Verdict};
pub use prune::VmConstants;

/// 输入层错误
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputError {
    /// 闭包结果格式错误
    Format(String),
    /// 折叠点与字节码不一致
    Fold(String),
    /// runtime 清单格式错误
    Manifest(String),
    /// 文件读取失败
    Io(String),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Format(s) => write!(f, "闭包结果格式错误：{s}"),
            InputError::Fold(s) => write!(f, "折叠点不一致：{s}"),
            InputError::Manifest(s) => write!(f, "清单错误：{s}"),
            InputError::Io(s) => write!(f, "读取失败：{s}"),
        }
    }
}

impl std::error::Error for InputError {}
