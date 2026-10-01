//! Java 虚方法分派：vtable trait（含 default impl）、`impl *__VTable for __inner`
//! （字段访问器 + 覆盖方法 + 继承槽位 + 接口 impl 桥接）、`ClassName__method_base`
//! 自由函数（invokespecial super() 调用路由）。
//!
//! 按职责分子模块：`trait_decl`（vtable trait 声明与缺省方法）、`inner_impls`
//! （`impl *__VTable for __inner`）、`base_fns`（`_base` 自由函数）。子模块经 `use super::*;`
//! 共用本文件的导入。

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::super::classify::{vtable_body_kind_gated, VTableBodyKind};
use super::super::erasure::{
    erase_item_signature_with, erase_signature, erase_signature_with, erased_args_of_same_arity,
    erased_hook_call, erasure_set_of, forward_conv_spec, result_inner_ty, same_type_tokens,
};
use super::super::generic_sig::rebuild_sig_with_generics;
use super::super::interface::{erased_impl_call, erased_wrapper_call, expand_interface_impl};
use super::super::parse::{split_type_name_args, FnItem};
use super::super::rewrite::{
    replace_clone_this_in_ok, rewrite_block, rewrite_dropped_params_in_inherited_body,
    rewrite_vtable_calls_ufcs_for_base,
};
use super::super::util::{attr_str, is_basic, strip_meta_attrs};
use super::context::GenContext;

mod base_fns;
mod inner_impls;
mod trait_decl;

pub(crate) use base_fns::base_fns;
pub(crate) use inner_impls::vtable_impls;
pub(crate) use trait_decl::vtable_trait;
