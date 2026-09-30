//! JVM 操作数栈模拟（← Python `codegen/stack.py`）：把字节码的栈 / 局部变量操作归约为
//! [`ir`] 表达式与语句。只依赖 `ty` / `ir`，不依赖闭包分析；公开接口不含字符串表达式。
//!
//! # P4b 稳定 API
//!
//! 方法体生成（P4b）每个方法（或每个 CFG 块）的使用方式：
//!
//! 1. **环境**：实现 [`SimEnv`]（短名、擦除子类型 / 接口查询、接口载体、Object 装箱、
//!    菱形实参求解）。类型一律以 [`ty::RsType`] 传入；类型形参以 `RsType::Param` 承载，
//!    菱形占位实参为 `Param(`[`INFER_PARAM`]`)`（渲染为 `_`）。
//! 2. **构造**：[`StackSim::new`]`(`[`SimConfig`]`, &env)`——按形参类型绑定 `this` 与
//!    `arg_N`（宽类型占两槽；`local_names` / `slot_decls` 提供 LVT 名与声明区间）。
//!    分支 / 块间复制状态用 [`StackSim::from_state`]`(cfg, `[`SimState`]`, env)`；
//!    [`SimState`] 全部字段公开，P4b 可直接读写栈、局部变量、语句缓冲与偏移。
//! 3. **逐指令**：先设置 `state.current_offset` / `state.next_offset`，再调用
//!    - [`StackSim::push`] / [`StackSim::push_entry`]（`dup` 以同一 [`ValueId`] 再压一份）；
//!    - [`StackSim::pop`]（被 dup 的非平凡值物化为 `let _tN`；下溢返回占位值并置 `underflow`）；
//!    - [`StackSim::pop_for_store`] + [`StackSim::store_local`]（xstore 的七阶段对齐与 let / 赋值发射）；
//!    - [`StackSim::load_local`]（xload）；[`StackSim::fresh_let`] / [`StackSim::emit`]（P4b 自身的语句）；
//!    - [`StackSim::enter_scope`] / [`StackSim::exit_scope`]（块嵌套深度，决定 let 阴影）。
//! 4. **产出**：`state.stmts`（[`ir::Stmt`]，let / 赋值带 [`ir::VarOrigin`]：槽位、绑定偏移、值类型）
//!    与 `state.locals`（槽 → [`Local`]），供分支提升 / 汇合变量声明使用。
//!
//! 辅助：[`to_ir_type`] / [`type_text`]（`RsType` → `ir::Type` / 文本）、[`safe_name`]（LVT 名 → 标识符）、
//! [`exprs`] 的结构化表达式构造与判定、[`types`] 的类型判定。
//! 未移植语义返回 [`SimError::Unported`]；全部状态确定性（BTreeMap / Vec），无全局可变状态。

pub mod env;
pub mod error;
pub mod exprs;
pub mod names;
mod state;
mod store;
pub mod types;

pub use env::{erase, erased_base, to_ir_type, type_text, SimEnv, TyNames, INFER_PARAM};
pub use error::{SimError, SimResult};
pub use names::safe_name;
pub use state::{Local, SimConfig, SimState, SlotDecl, StackEntry, StackSim, ValueId};
