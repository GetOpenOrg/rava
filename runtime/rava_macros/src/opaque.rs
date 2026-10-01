//! `java_class_opaque!`：L1（名字级）类型的不透明声明（C3 第 6 项）。
//!
//! 分析器判为 L1 的类 / 接口只出现在签名、checkcast / instanceof、catch 中，从未实例化且无
//! 已实例化子类型——值只可能是 null。发射形态只保留类型身份：同名载体（一个 `Object` 引用），
//! 无字段、无方法、无 vtable trait、无类初始化。
//!
//! ```ignore
//! rava_macros::java_class_opaque! {
//!     #[binary_name = "java/util/RandomAccess"]
//!     pub struct X<E>: Parent<_>, Iface<_, _>;
//! }
//! ```
//!
//! - `From<Object>`：null 得本类型的 null；非 null 按 checkcast 语义核对运行时类（是本类型的
//!   子类型则保留同一引用，否则 ClassCastException）。
//! - 冒号后为全部传递超类型（Object 除外），`_` 标出该超类型的类型实参个数：逐个展开
//!   `From<X<..>> for Anc<..>`（对祖先任意实参成立，经 Object 边界转换，保持对象身份）。
//! - 与接口载体同形的 `PartialEq`（身份）/ `Debug` / `Deref<Target = Object>` / `is_jvm_null`。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{GenericParam, Generics, Ident, LitStr, Token, Type, Visibility};

struct OpaqueInput {
    binary_name: LitStr,
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    supers: Vec<Type>,
}

impl Parse for OpaqueInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut binary_name = None;
        for attr in input.call(syn::Attribute::parse_outer)? {
            if attr.path().is_ident("binary_name") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
                        binary_name = Some(s.clone());
                    }
                }
            }
        }
        let vis: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let ident: Ident = input.parse()?;
        let generics: Generics = input.parse()?;
        let mut supers = Vec::new();
        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let list = Punctuated::<Type, Token![,]>::parse_separated_nonempty(input)?;
            supers.extend(list);
        }
        input.parse::<Token![;]>()?;
        let binary_name = binary_name.ok_or_else(|| input.error("java_class_opaque! 缺 #[binary_name = \"...\"]"))?;
        Ok(OpaqueInput { binary_name, vis, ident, generics, supers })
    }
}

/// 超类型 `Anc<_, _>` → (`Anc`, 实参个数)
fn super_arity(t: &Type) -> syn::Result<(syn::Path, usize)> {
    let Type::Path(tp) = t else { return Err(syn::Error::new_spanned(t, "java_class_opaque! 超类型须为路径")) };
    let mut path = tp.path.clone();
    let last = path.segments.last_mut().ok_or_else(|| syn::Error::new_spanned(t, "空路径"))?;
    let n = match &last.arguments {
        syn::PathArguments::AngleBracketed(a) => a.args.len(),
        _ => 0,
    };
    last.arguments = syn::PathArguments::None;
    Ok((path, n))
}

pub(crate) fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<OpaqueInput>(input).and_then(|p| expand_inner(&p)) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(p: &OpaqueInput) -> syn::Result<TokenStream2> {
    let OpaqueInput { binary_name, vis, ident, .. } = p;
    let gen = crate::block::augment_generic_bounds(&p.generics);
    let (impl_g, ty_g, where_c) = gen.split_for_impl();
    let type_params: Vec<&Ident> = gen
        .params
        .iter()
        .filter_map(|g| if let GenericParam::Type(tp) = g { Some(&tp.ident) } else { None })
        .collect();

    let mut upcasts = Vec::new();
    for s in &p.supers {
        let (path, arity) = super_arity(s)?;
        let mut g = gen.clone();
        let anc: Vec<Ident> = (0..arity).map(|i| format_ident!("__Anc{}", i)).collect();
        for a in &anc {
            g.params.push(GenericParam::Type(syn::parse_quote! { #a }));
        }
        let g = crate::block::augment_generic_bounds(&g);
        let (u_impl_g, _, u_where_c) = g.split_for_impl();
        let target = if anc.is_empty() { quote! { #path } } else { quote! { #path<#(#anc),*> } };
        upcasts.push(quote! {
            impl #u_impl_g ::std::convert::From<#ident #ty_g> for #target #u_where_c {
                fn from(v: #ident #ty_g) -> Self {
                    <#target as ::std::convert::From<Object>>::from(v.__ref)
                }
            }
        });
    }

    Ok(quote! {
        #[allow(non_camel_case_types)]
        #vis struct #ident #impl_g #where_c {
            __ref: Object,
            __phantom: ( #( ::std::marker::PhantomData<fn() -> #type_params>, )* ),
        }

        impl #impl_g ::std::clone::Clone for #ident #ty_g #where_c {
            fn clone(&self) -> Self {
                Self { __ref: ::std::clone::Clone::clone(&self.__ref), __phantom: ::std::default::Default::default() }
            }
        }

        impl #impl_g ::std::default::Default for #ident #ty_g #where_c {
            fn default() -> Self {
                Self { __ref: ::std::default::Default::default(), __phantom: ::std::default::Default::default() }
            }
        }

        impl #impl_g ::std::convert::From<Object> for #ident #ty_g #where_c {
            // checkcast 语义（JVMS §6.5）：null 通过；非 null 须为本类型的子类型
            fn from(obj: Object) -> Self {
                if !obj.0.is_jvm_null() && !obj.0.is_instance_of(#binary_name) {
                    return obj.checkcast::<Self>(#binary_name);
                }
                Self { __ref: obj, __phantom: ::std::default::Default::default() }
            }
        }

        impl #impl_g ::std::convert::From<#ident #ty_g> for Object #where_c {
            fn from(v: #ident #ty_g) -> Object { v.__ref }
        }

        impl #impl_g ::std::cmp::PartialEq for #ident #ty_g #where_c {
            fn eq(&self, other: &Self) -> bool {
                self.__ref.0.__identity() == other.__ref.0.__identity()
            }
        }

        impl #impl_g ::std::fmt::Debug for #ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#ident), ObjectVTable::__obj_str(&*self.__ref.0))
            }
        }

        impl #impl_g ::std::ops::Deref for #ident #ty_g #where_c {
            type Target = Object;
            fn deref(&self) -> &Object { &self.__ref }
        }

        impl #impl_g #ident #ty_g #where_c {
            pub const BINARY_NAME: &'static str = #binary_name;

            pub fn is_jvm_null(&self) -> bool {
                ObjectVTable::is_jvm_null(&*self.__ref.0)
            }
        }

        #(#upcasts)*
    })
}
