//! 存储钩子（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.3.1）：声明层对存储层
//! `X__inner` 的依赖只经本模块的非泛型自由函数——分配默认存储。S7-2 起句柄统一，按名字段
//! 协议与擦除重建都经句柄所持存储应答，原 `cells` / `from_any` 两个钩子删除；拆层后钩子定义
//! 随存储层进实现 crate，声明层只留调用。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::context::GenContext;

/// 钩子名：`__jb_<类>__<用途>`（现只有 `alloc`）
pub(crate) fn hook_ident(ctx: &GenContext, what: &str) -> Ident {
    format_ident!("__jb_{}__{}", ctx.struct_ident, what)
}

/// 钩子为模块级自由函数（拆层时拆为导出定义 + 外壳）
pub(crate) fn generate(ctx: &GenContext) -> Vec<TokenStream2> {
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let alloc = hook_ident(ctx, "alloc");
    vec![quote! {
        /// 分配一个默认存储，返回其本类引用（构造器的 `_init_not_null` 调用；S7-2 起 null 不分配）
        pub fn #alloc() -> __Ref<dyn #vtable_trait_ident> {
            __Ref::new(
                __Shared::new(<#inner_ident as ::std::default::Default>::default()),
                |__i| __i as &dyn #vtable_trait_ident)
        }
    }]
}
