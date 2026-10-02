//! 方法体生成：**类 + 方法 → 完整 Rust 函数文本**。
//!
//! 入口 [`gen_method_body`]`(env, &MethodRequest, &mut MethodSink) -> MethodResult<String>`：
//! - `env`：[`instr::InstrEnv`]，以发射所在类构造（`InstrCtx::new(.., class)`，继承展开时
//!   `.with_code_owner(字节码出处类)`），`tparams` 为该类的有效类型形参；
//! - [`MethodRequest`]：发射类 [`ty::ClassInfo`]、方法视图 [`classfile::Method`]（名字 / 描述符 /
//!   访问标志 / 泛型签名；继承展开时为适配后的视图）、出处类的规范化字节码
//!   [`input::NormCode`]、局部变量表、是否重载、调用方给定的 Rust 名、是否在 vtable 默认体内；
//! - 返回值：函数文本（构造器为 `new` + `__init_on` 双入口，重载方法前置 `// java:` 注释）；
//! - [`MethodSink`]：调用方持有的外部登记 [`instr::InstrLog`] 与控制流审计数据 [`CfgStats`]；
//!   生成失败时已发生的登记同样保留（与 Python 全局登记时序一致），由调用方汇总；
//!   本 crate 无全局可变状态。
//!
//! 错误：[`MethodError::Cfg`] 为控制流语义限制（调用方退化为 `panic!("stub: ..")` 存根），
//! [`MethodError::Audit`] 为账本 / 结构树自检失败（须中止），其余见 [`error`]。
//! 本 crate 不依赖发射层 trait；发射层以适配器包装上述函数。

pub mod blocks;
pub mod body;
pub mod coerce_text;
pub mod cond_text;
pub mod emit;
pub mod entry;
pub mod error;
pub mod fold_array;
pub mod lines;
pub mod node;
pub mod postprocess;
pub mod sig;
pub mod split_try;
pub mod text;
pub mod try_plan;
pub mod types;
pub mod unify;
pub mod vars;

pub use body::{gen_method_body, CfgStats, MethodRequest, MethodSink};
pub use error::{MethodError, MethodResult};
