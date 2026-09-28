//! `iface_upcasts!`：类实例 / 子接口载体 → 擦除接口载体视图的协变 upcast（A-4）。
//!
//! 生成侧只写裸类型参数名（T-1：bounds 进宏）：
//! ```ignore
//! rava_macros::iface_upcasts! { impl<E> ArrayList<E> => List<Object>, Collection<Object> }
//! ```
//! 宏为每个类型参数补齐与 `java_class!` 同一来源的约束（`augment_generic_bounds`），
//! 并逐目标展开 `impl From<Src> for Iface<Object..>`——经 Object 边界解包再包装（同一 __ref，
//! 保持对象身份）。实现体用显式 UFCS：接口载体可能自带 Java `static from(..)` 工厂方法，
//! `Iface::from(..)` 路径解析会被固有方法遮蔽。

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Generics, Token, Type};

struct UpcastInput {
    generics: Generics,
    source: Type,
    targets: Punctuated<Type, Token![,]>,
}

impl Parse for UpcastInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![impl]>()?;
        let generics: Generics = input.parse()?;
        let source: Type = input.parse()?;
        input.parse::<Token![=>]>()?;
        let targets = Punctuated::<Type, Token![,]>::parse_terminated(input)?;
        Ok(UpcastInput { generics, source, targets })
    }
}

pub(crate) fn expand(input: TokenStream2) -> TokenStream2 {
    let parsed: UpcastInput = match syn::parse2(input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error(),
    };
    let generics = crate::block::augment_generic_bounds(&parsed.generics);
    let (impl_g, _, where_c) = generics.split_for_impl();
    let src = &parsed.source;
    let impls = parsed.targets.iter().map(|tgt| {
        quote! {
            impl #impl_g From<#src> for #tgt #where_c {
                fn from(v: #src) -> Self {
                    <#tgt as ::std::convert::From<Object>>::from(
                        <Object as ::std::convert::From<#src>>::from(v))
                }
            }
        }
    });
    quote! { #(#impls)* }
}
