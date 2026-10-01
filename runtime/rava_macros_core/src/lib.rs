//! `java_class!` / `java_try!` 的展开与分析逻辑。
//!
//! 宏 crate `rava_macros` 是 proc-macro 入口，只做 `proc_macro` ↔ `proc_macro2` 转换；
//! 生成器（`plan` 特性）复用同一份判定为声明层剥去下沉方法体（拆 crate 文档 §7.5.2）。

mod block;
mod try_macro;

pub use block::{augment_generic_bounds, expand as expand_class};
#[cfg(feature = "plan")]
pub use block::plan;
pub use try_macro::expand as expand_try;
