use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam};

/// `#[java_rta_macros::java_class(binary_name = "...", ...)]`
///
/// 为 Java 翻译类自动生成：
///   - `impl Into<Object>`：封装为 Object（JVM upcasting）
///   - `impl From<Object>`：从 Object 中取出（JVM checkcast/downcasting）
///   - `impl Debug`：基础调试输出（类型名）
///
/// 适用于 JDK 字节码翻译生成的 struct 以及用户自定义的 Java 风格 struct。
#[proc_macro_attribute]
pub fn java_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    // 为每个类型参数补全 Clone + 'static bound（若尚未存在）
    let mut gen = input.generics.clone();
    for param in &mut gen.params {
        if let GenericParam::Type(tp) = param {
            let has_clone = tp.bounds.iter().any(|b| {
                if let syn::TypeParamBound::Trait(t) = b {
                    t.path.segments.last().map_or(false, |s| s.ident == "Clone")
                } else {
                    false
                }
            });
            let has_static = tp.bounds.iter().any(|b| {
                matches!(b, syn::TypeParamBound::Lifetime(l) if l.ident == "static")
            });
            if !has_clone {
                tp.bounds.push(syn::parse_quote!(Clone));
            }
            if !has_static {
                tp.bounds.push(syn::parse_quote!('static));
            }
        }
    }

    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let obj = quote! { ::java_runtime::java::lang::Object };

    let into_impl: TokenStream2 = quote! {
        impl #impl_generics Into<#obj> for #name #ty_generics #where_clause {
            fn into(self) -> #obj { #obj::from_any(self) }
        }
    };

    let from_impl: TokenStream2 = quote! {
        impl #impl_generics From<#obj> for #name #ty_generics #where_clause {
            fn from(obj: #obj) -> Self { obj.downcast::<Self>() }
        }
    };

    let debug_impl: TokenStream2 = quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", ::std::any::type_name::<Self>())
            }
        }
    };

    let expanded = quote! {
        #input
        #into_impl
        #from_impl
        #debug_impl
    };

    expanded.into()
}
