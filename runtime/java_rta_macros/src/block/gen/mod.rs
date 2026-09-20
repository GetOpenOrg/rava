//! `expand_inner` 的生成阶段模块（按 Java 语义特性切分，见 docs/plans/2026-09-18-block-rs-refactor.md）。
//!
//! - `context`：GenContext —— 唯一的跨阶段数据传递机制（expand_inner 局部变量的收拢）

pub(crate) mod context;

pub(crate) use context::GenContext;
