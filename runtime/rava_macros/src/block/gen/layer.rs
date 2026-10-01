//! 物理拆层（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S4）：同一个
//! `java_class!` 块按 `#[rava_layer = "decl" | "body"]` 只展开其中一层。
//!
//! - 缺省（无属性）= 完整展开（用户 crate / 库 crate / Python 生成器）；
//! - `decl`：声明层（vtable trait、wrapper 及其固有方法外壳、静态存储与类初始化、类型转换），
//!   落 `java_runtime`；
//! - `body`：存储层与方法体（`X__inner` 及其全部 vtable / 接口 impl、存储钩子、`__jbm_` 体
//!   函数、非泛型 `_base` 函数），落实现 crate `java_body_k`，经 `use java_runtime::*` 依赖声明层。
//!
//! 声明层调用存储层的非泛型自由函数（钩子 / 体函数 / base 函数）时，同名外壳经
//! `extern "Rust"` 声明按链接符号转调实现层的 `#[export_name]` 定义：外壳与定义的形参、
//! 返回类型取自同一函数项（同一份块文本的同一展开），符号名含类二进制名、函数名与签名
//! 指纹，签名不一致即链接失败而不会静默错配。外壳 `#[inline]`，调用语义只多一次转发。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// 块的展开层
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Layer {
    #[default]
    Full,
    Decl,
    Body,
}

impl Layer {
    pub(crate) fn parse(s: &str) -> Option<Layer> {
        match s {
            "full" => Some(Layer::Full),
            "decl" => Some(Layer::Decl),
            "body" => Some(Layer::Body),
            _ => None,
        }
    }
}

/// FNV-1a 64：符号指纹（确定性、无依赖）
fn fnv1a(parts: &[&str]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for p in parts {
        for b in p.bytes().chain(std::iter::once(0u8)) {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    }
    h
}

/// 链接符号：`__rava_<二进制名转义>__<函数名>_<指纹>`
fn link_symbol(binary_name: &str, fn_name: &str, sig_text: &str) -> String {
    let esc: String = binary_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("__rava_{esc}__{fn_name}_{:016x}", fnv1a(&[binary_name, fn_name, sig_text]))
}

/// 拆分后的一个自由函数：实现层定义 + 声明层外壳
pub(crate) struct SplitFn {
    pub(crate) body: TokenStream2,
    pub(crate) shell: TokenStream2,
}

/// 把一个非泛型模块级自由函数拆成实现层定义（加 `#[export_name]`）与声明层外壳。
/// 调用方保证函数无泛型形参、无 where 子句、签名不含 `Self`。
pub(crate) fn split_free_fn(binary_name: &str, item: &TokenStream2) -> syn::Result<SplitFn> {
    let f: syn::ItemFn = syn::parse2(item.clone())?;
    if !f.sig.generics.params.is_empty()
        || f.sig.generics.where_clause.as_ref().is_some_and(|w| !w.predicates.is_empty())
    {
        return Err(syn::Error::new_spanned(&f.sig, "rava_layer：泛型自由函数不可拆层"));
    }
    let name = &f.sig.ident;
    // 外壳 / extern 声明的形参：标识符模式去 `mut`，其余模式按位置命名
    let mut params: Vec<TokenStream2> = Vec::new();
    let mut args: Vec<syn::Ident> = Vec::new();
    for (i, a) in f.sig.inputs.iter().enumerate() {
        let syn::FnArg::Typed(pt) = a else {
            return Err(syn::Error::new_spanned(a, "rava_layer：自由函数不应有接收者"));
        };
        let id = match &*pt.pat {
            syn::Pat::Ident(pi) => pi.ident.clone(),
            _ => format_ident!("__a{}", i),
        };
        let ty = &pt.ty;
        params.push(quote! { #id: #ty });
        args.push(id);
    }
    let output = &f.sig.output;
    let sig_text = quote! { (#(#params),*) #output }.to_string();
    let sym = link_symbol(binary_name, &name.to_string(), &sig_text);

    let attrs = &f.attrs;
    let vis = &f.vis;
    let sig = &f.sig;
    let block = &f.block;
    let body = quote! {
        #(#attrs)*
        #[export_name = #sym]
        #vis #sig #block
    };
    let shell = quote! {
        #(#attrs)*
        #[inline]
        #vis fn #name(#(#params),*) #output {
            extern "Rust" {
                #[link_name = #sym]
                fn #name(#(#params),*) #output;
            }
            unsafe { #name(#(#args),*) }
        }
    };
    Ok(SplitFn { body, shell })
}

/// 按层取一组可拆的自由函数：完整 = 原样；声明层 = 外壳；实现层 = 导出定义
pub(crate) fn place_fns(
    layer: Layer, binary_name: &str, items: &[TokenStream2],
) -> syn::Result<TokenStream2> {
    match layer {
        Layer::Full => Ok(quote! { #(#items)* }),
        Layer::Decl | Layer::Body => {
            let mut out = Vec::with_capacity(items.len());
            for it in items {
                let s = split_free_fn(binary_name, it)?;
                out.push(if layer == Layer::Decl { s.shell } else { s.body });
            }
            Ok(quote! { #(#out)* })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_keeps_signature_and_symbol_in_sync() {
        let item = quote! {
            #[doc(hidden)]
            pub fn __jbm_A__m(this: &A, mut x: i32) -> Result<i32> { x += 1; Ok(x) }
        };
        let s = split_free_fn("p/A", &item).unwrap();
        let body = s.body.to_string();
        let shell = s.shell.to_string();
        let sym = body.split("export_name = ").nth(1).unwrap().split(']').next().unwrap().trim();
        assert!(shell.contains(&format!("link_name = {sym}")));
        assert!(sym.starts_with("\"__rava_p_A____jbm_A__m_"));
        assert!(shell.contains("unsafe { __jbm_A__m (this , x) }"));
        assert!(!shell.contains("mut x"));
        assert!(body.contains("mut x"));
    }
}
