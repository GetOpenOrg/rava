//! 发射层类型层（`codegen/type_map.py` / `sig_parse.py` / `type_args.py` /
//! `sig_types.py` / `jvm_type.py` 的 Rust 移植）。
//!
//! 与 Python 的结构差异：
//! - 注册表 [`Registry`]、短名 [`ShortNames`]、清单 [`Manifest`] 是显式上下文，
//!   经 [`TyCtx`] 传递；无模块级全局状态；
//! - Rust 类型是结构化的 [`RsType`]，文本只在 [`RsType::render`] 出口产生；
//! - JVM 侧类型代数是 [`JvmType`]。

pub mod carrier;
pub mod class_params;
pub mod consts;
pub mod ident;
pub mod manifest;
pub mod registry;
pub mod rs_type;
pub mod short_names;
pub mod sig_parse;
pub mod type_args;
pub mod type_map;

#[cfg(test)]
pub(crate) mod testutil;

pub use manifest::Manifest;
pub use registry::{ClassInfo, Registry};
pub use rs_type::{Prim, RsType};
pub use short_names::ShortNames;
pub use sig_parse::MethodSigTypes;

/// 类型层查询上下文：注册表 + 其短名表 + runtime 清单（均不可变）
#[derive(Clone, Copy)]
pub struct TyCtx<'a> {
    pub reg: &'a Registry,
    pub names: &'a ShortNames,
    pub manifest: &'a Manifest,
}

impl<'a> TyCtx<'a> {
    pub fn new(reg: &'a Registry, names: &'a ShortNames, manifest: &'a Manifest) -> TyCtx<'a> {
        TyCtx { reg, names, manifest }
    }
}
