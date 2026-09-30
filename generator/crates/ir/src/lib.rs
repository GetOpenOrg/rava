//! Rust IR 与渲染单出口（← `codegen/rs_ir.py` + `codegen/render.py`）。
//!
//! 生成器各层之间以本 crate 的 [`Type`] / [`Expr`] / [`Stmt`] / [`Item`] 为唯一货币：
//! 公开 API 不接受、不返回字符串形式的表达式或类型，文本只在 [`Renderer`] 出口产生。
//!
//! 与 Python 版的差异（详见 `GOLDEN_DIFF.md` 与各模块文档）：
//! - 标识符、路径、类型、字面量均结构化（[`Ident`] / [`Path`] / [`Type`] / [`Lit`]）；
//!   Python 里 `RsNamed('HashMap<K, V>')`、`Call('JArray::<u16>::try_new')`、
//!   `Lit('Class::for_class(...)')` 这类文本载体在这里没有对应物；
//! - checkcast 的合法组合由 [`CastMode`] 枚举表达（Python 是三个互相约束的 bool）；
//! - 上转的四种包装与 `UpcastExpr` 节点合并为 [`UpcastWrap`]，原子性判定
//!   （`is_atomic_rs`）改为对 IR 节点判定 [`Renderer::is_atomic`]；
//! - 方法体结构块（Python `BlockStmt` / `StructLine` 文本行 + 花括号深度账）由
//!   结构化控制流语句（[`Stmt::Loop`] / [`Stmt::While`] / [`Stmt::If`] / [`Stmt::Match`] /
//!   [`Stmt::LabeledBlock`] / [`Stmt::JavaTry`]）取代，缩进由渲染器按嵌套深度产生；
//! - 类名短名经 [`ShortNames`] 注入（类型层实现），本 crate 无全局状态、不依赖类型层。
//!
//! [`Raw`] 是唯一的文本逃生舱（raw-audit 计数对象，终态 0），只供尚未结构化的
//! 产出方使用；渲染器对它原样输出。

pub mod anchors;
mod error;
mod expr;
mod ident;
mod item;
mod lit;
pub mod raw_audit;
pub mod render;
mod stmt;
mod ty;

pub use error::IrError;
pub use expr::{
    BinOp, BlockExpr, CastExpr, CastMode, Expr, FnPath, IfExpr, MacroCall, Raw, StaticFieldRef,
    UnOp, UpcastWrap,
};
pub use ident::{Ident, Label};
pub use item::{FnItem, ImplItem, Item, ModItem, Param, StructField, StructItem, TypeAlias, UseTree, Vis};
pub use lit::{FloatLit, FloatTy, IntTy, Lit};
pub use render::Renderer;
pub use stmt::{
    ArmBody, AssignStmt, CatchClause, ElseBranch, IfStmt, LetStmt, LoopStmt, MatchArm, MatchStmt,
    Pattern, Stmt, TryStmt, VarOrigin, WhileStmt,
};
pub use ty::{Path, PathSegment, Prim, Type};

/// 类名短名查询（JVM binary 名 → 生成代码里的 Rust 类型名，如
/// `java/util/HashMap$Node` → `HashMap_Node`）。由类型层实现后注入渲染器，
/// 替代 Python `type_map.short_cls` 的模块级全局配置。
pub trait ShortNames {
    fn short_cls(&self, binary: &str) -> String;
}
