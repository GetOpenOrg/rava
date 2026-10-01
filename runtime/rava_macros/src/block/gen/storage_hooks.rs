//! 存储钩子（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.3.1）：声明层对存储层
//! `X__inner` 的全部依赖只经本模块的三个非泛型自由函数——分配默认存储、取按名字段协议的
//! 存储视图、按精确存储类型还原部件。钩子体就是原先内联在调用点的代码，调用点语义、
//! 求值顺序与失败回落顺序不变；拆层后钩子定义随存储层进实现 crate，声明层只留调用。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::context::GenContext;

/// 钩子名：`__jb_<类>__<用途>`（`alloc` / `cells` / `from_any`）
pub(crate) fn hook_ident(ctx: &GenContext, what: &str) -> Ident {
    format_ident!("__jb_{}__{}", ctx.struct_ident, what)
}

/// 三个钩子各为一个模块级自由函数（拆层时逐个拆为导出定义 + 外壳）
pub(crate) fn generate(ctx: &GenContext) -> Vec<TokenStream2> {
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let alloc = hook_ident(ctx, "alloc");
    let cells = hook_ident(ctx, "cells");
    let from_any = hook_ident(ctx, "from_any");
    vec![quote! {
        /// 分配一个默认存储，返回 (vtable, 存储) 部件（wrapper 的 Default 与浅拷贝共用）
        #[doc(hidden)]
        pub fn #alloc() -> (__Shared<dyn #vtable_trait_ident>, __AnyRef) {
            let __rc = __Shared::new(<#inner_ident as ::std::default::Default>::default());
            (__Shared::clone(&__rc) as __Shared<dyn #vtable_trait_ident>, __rc as __AnyRef)
        }
    }, quote! {
        /// 存储恰是本类存储时，返回其 ObjectVTable 视图（按名字段协议的静态类应答方）
        #[doc(hidden)]
        pub fn #cells(any: &__AnyRef) -> ::std::option::Option<&dyn ObjectVTable> {
            ::std::option::Option::map(
                any.downcast_ref::<#inner_ident>(),
                |__i| __i as &dyn ObjectVTable)
        }
    }, quote! {
        /// 存储恰是本类存储时还原 (vtable, 存储) 部件；否则原样交还
        #[doc(hidden)]
        pub fn #from_any(
            any: __AnyRef,
        ) -> ::std::result::Result<(__Shared<dyn #vtable_trait_ident>, __AnyRef), __AnyRef> {
            match any.downcast::<#inner_ident>() {
                ::std::result::Result::Ok(__rc) => ::std::result::Result::Ok(
                    (__Shared::clone(&__rc) as __Shared<dyn #vtable_trait_ident>, __rc as __AnyRef)),
                ::std::result::Result::Err(__other) => ::std::result::Result::Err(__other),
            }
        }
    }]
}
