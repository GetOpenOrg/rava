//! 控制流结构化：基本块 CFG → 支配树 / 自然循环 → 结构树；
//! 不可归约时退到程序计数器状态机。只依赖 classfile / ir，不依赖闭包分析。
//!
//! # P4b 稳定 API
//!
//! 方法体生成（P4b）按以下顺序使用本 crate：
//!
//! 1. **切块**：[`build_blocks`]`(insns, exception_table, boundaries)` → `Vec<`[`Block`]`>`。
//!    `boundaries` 为额外的切块点（try 区间边界等）；块终结 [`Terminator`] 携带跳转指令 pc。
//! 2. **逐块模拟**（P4b 自身，借助 `sim` crate）：把块归约为实现 [`StructNode`] 的节点，
//!    终结为 [`Terminal`]（条件以 [`Cond`] 表达，原子为 [`ir::Expr`]；
//!    [`branch_atom`] / [`cmp_op`] 从比较跳转的操作码与操作数构造原子，
//!    [`Cond::and`] / [`Cond::or`] 做短路合并，[`Cond::to_expr`] 渲染为布尔表达式）。
//!    try 区域以合成的 [`Terminal::Try`] 节点表示，节点 `ctx` 为覆盖它的 try 组集合。
//! 3. **流分析**：[`analyze`]`(entry, &succs)` → [`FlowAnalysis`]（RPO、支配树、回边、自然循环、
//!    可归约性）。`succs` 由各节点 [`Terminal::successors`] 组成。
//! 4. **结构化**：可归约时 [`structure()`]`(&mut nodes, &flow)` → `Vec<`[`Item`]`>`，再经
//!    [`simplify()`] 规整；不可归约时 [`build_dispatch`] + 每块 [`next_pc_stmts`]。
//! 5. **自检**：[`verify_tree`] 校验结构树覆盖全部活块与分支，并把 [`StructNode::jump_pcs`]
//!    记入 [`JumpLedger`]；[`JumpLedger::verify`] 确认全部跳转被消费；[`AuditStats`]
//!    （调用方持有）汇总统计。
//!
//! 结构树 [`Item`] 只以节点编号引用 P4b 的节点（`Code` / `Decl` / catch 子句序号），
//! 渲染（Code → 节点语句、Decl → 汇合变量声明、Break / Continue → 标签）由 P4b 完成。
//! 全部公开函数确定性（BTreeMap / 稳定排序），无全局可变状态。

pub mod audit;
pub mod cond;
pub mod dispatch;
pub mod error;
pub mod flow;
pub mod graph;
pub mod node;
pub mod opcodes;
pub mod simplify;
pub mod structure;
pub mod tree;

pub use audit::{verify_tree, AuditStats, JumpKind, JumpLedger, StubFallback};
pub use cond::{branch_atom, cmp_op, neg_cmp_op, negate_branch, Cond};
pub use dispatch::{build_dispatch, next_pc_stmts, pc_lit, Dispatch, PC_VAR};
pub use error::{CfgAuditError, CfgError};
pub use flow::{analyze, dominates, reachable, FlowAnalysis, Succs};
pub use graph::{build_blocks, function_always_returns, Block, NodeId, Terminator};
pub use node::{FlowNode, StructNode, Terminal};
pub use simplify::{completes_normally, simplify};
pub use structure::structure;
pub use tree::{
    BreakLabel, CatchArm, IfItem, Item, LoopItem, SwitchArm, SwitchItem, TryItem,
};

#[cfg(test)]
mod tests;
