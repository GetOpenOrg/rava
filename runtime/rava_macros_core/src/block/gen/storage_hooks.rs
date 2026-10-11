//! 存储钩子（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.3.1）：声明层对存储层
//! `X__inner` 的依赖只经本模块的非泛型自由函数——分配默认存储、实例字段偏移表。S7-2 起句柄统一，按名字段
//! 协议与擦除重建都经句柄所持存储应答，原 `cells` / `from_any` 两个钩子删除；拆层后钩子定义
//! 随存储层进实现 crate，声明层只留调用。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::context::GenContext;

/// 钩子名：`__jb_<类>__<用途>`（`alloc` / `offsets`）
pub(crate) fn hook_ident(ctx: &GenContext, what: &str) -> Ident {
    format_ident!("__jb_{}__{}", ctx.struct_ident, what)
}

/// 钩子为模块级自由函数（拆层时拆为导出定义 + 外壳）
pub(crate) fn generate(ctx: &GenContext) -> Vec<TokenStream2> {
    let inner_ident = &ctx.inner_ident;
    let alloc = hook_ident(ctx, "alloc");
    let offsets = hook_ident(ctx, "offsets");
    let names: Vec<&Ident> = ctx.meta.superclass_fields.iter().map(|(n, _)| n)
        .chain(ctx.fields.iter().map(|(n, _)| n))
        .collect();
    vec![quote! {
        /// 分配一个默认存储，返回其对象句柄（构造器的 `_init_not_null` 调用；S7-2 起 null 不分配）
        pub fn #alloc() -> __Handle {
            __Handle::alloc(__Obj::new(<#inner_ident as ::std::default::Default>::default()))
        }
    }, quote! {
        /// 平铺实例字段（继承字段在前、自有字段在后）在存储中的字节偏移（描述符 `offsets`）
        pub fn #offsets() -> &'static [u32] {
            const OFFSETS: &[u32] = &[#(::core::mem::offset_of!(#inner_ident, #names) as u32),*];
            OFFSETS
        }
    }]
}
