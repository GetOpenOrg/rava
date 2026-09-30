//! 指令翻译层（← Python `codegen/instr/`）：把一条规范化 JVM 指令（[`input::NInsn`]）在
//! [`sim::StackSim`] 上归约为 [`ir`] 语句 / 表达式。不含 CFG 结构化（`cfg` crate）与
//! 方法体组装（P4c）；公开接口不以字符串往返表达式或类型。
//!
//! # P4c 公开 API
//!
//! 方法体生成（P4c，实现 P5a 的 `MethodBodyEmitter`）每个类 / 方法的使用方式：
//!
//! 1. **类级事实**（每次生成一次）：[`InstrFacts::build`]`(registry, root_object_classfile,
//!    runtime_src_dir)`——根类虚方法集、手写根类 API 名面、闭包子类表、手写 `_impl.rs`
//!    的 `fn` 名缓存。
//! 2. **上下文**：[`InstrCtx::new`]`(ty_ctx, runtime_manifest, &facts, &hooks, class_name)`；
//!    `hooks` 实现 [`InstrHooks`]（instr 需要但归方法 / 类生成层所有的查询，如 SAM 构造器
//!    路径），无需时用 [`NoHooks`]。
//! 3. **模拟环境**：[`InstrEnv::new`]`(ctx, class_type_params)`，它实现 [`sim::SimEnv`]
//!    （短名、严格子类型、接口判定、接口载体、Object 装箱、菱形实参求解——与 Python
//!    `method/codegen.py` 注入 `StackSim` 的回调同一语义）；以 `StackSim::new(cfg, &env)`
//!    构造模拟器。
//! 4. **逐指令**：设置 `sim.state.current_offset / next_offset` 后调用
//!    [`sim_instr`]`(&env, &mut sim, &mut log, &ninsn)`；控制流指令（if / goto / switch）由
//!    `cfg` 层处理，不经本入口。
//! 5. **产出**：`sim.state`（语句、栈、局部变量）与 [`InstrLog`]（审计计数 [`Audit`]、
//!    继承调用 / lambda 引用 / SAM 站点等 [`Effect`]，取代 Python 的全局账本）。
//! 6. **层次查询**（P4c 的槽位 widening 等也用）：[`hierarchy::common_ref_type`] /
//!    [`hierarchy::common_ref_type_widening`] / [`hierarchy::is_subtype`]；值强转见 [`coerce`]。
//!
//! 未移植的 Python 分支返回 [`InstrError::Unported`]（清单见 crate 根 `GOLDEN_DIFF.md`）；
//! 全部状态显式传入，无全局可变状态；迭代序确定（BTreeMap / 稳定排序）。

pub mod build;
pub mod coerce;
pub mod ctx;
pub mod env;
pub mod error;
pub mod hierarchy;
pub mod invoke;
pub mod log;
pub mod naming;
pub mod sim;
pub mod text;

pub use ctx::{InstrCtx, InstrFacts, InstrHooks, NoHooks};
pub use env::InstrEnv;
pub use error::{InstrError, InstrResult};
pub use log::{Audit, Effect, InstrLog};
pub use sim::sim_instr;
