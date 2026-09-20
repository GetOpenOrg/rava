//! `expand_inner` 的生成阶段模块（按 Java 语义特性切分，见 docs/plans/2026-09-18-block-rs-refactor.md）。
//!
//! - `context`：GenContext —— 唯一的跨阶段数据传递机制（expand_inner 局部变量的收拢）
//! - `struct_layout`：Java 字段布局（__inner struct + impl ObjectVTable for __inner）
//! - `virtual_dispatch`：Java 虚方法分派（vtable trait + vtable impls + base 自由函数）
//! - `wrapper`：Java 类型包装（wrapper struct + 方法委托 + 构造器）
//! - `type_conversions`：Java 类型转换（BINARY_NAME / From<Object> / From<Child> for Parent）

pub(crate) mod context;
pub(crate) mod struct_layout;
pub(crate) mod type_conversions;
pub(crate) mod virtual_dispatch;
pub(crate) mod wrapper;

pub(crate) use context::GenContext;
