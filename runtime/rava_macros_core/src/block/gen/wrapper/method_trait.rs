//! 每类方法 trait `X__Methods`（S7 计划 §9.9 K3）：wrapper 固有 impl 的可读层成员
//! （Java 方法外壳、继承转发、构造器、静态方法、字段访问器、`__class_init`、`_init_not_null`）
//! 移入与 wrapper 同类型形参的 trait，由 wrapper 实现。
//!
//! trait 方法签名与原固有方法逐一相同（只去可见性与声明处模式），trait 形参与 wrapper
//! 形参一一对应（`impl<G> X__Methods<G> for X<G>`）→ 静态调用点 `X::m(..)` 的类型形参推断
//! 与固有 impl 一致。调用方按需导入 `use …::X__Methods as _;`（生成器按字节码指令的 owner 登记）。
//!
//! 留在固有 impl 的成员：
//! - `const fn`（trait 方法不可为 const）；
//! - 与 wrapper 已实现的 std prelude trait（及其 blanket impl）同名的方法（[`STD_COLLISIONS`]）：
//!   trait 方法与 `Clone::clone` / `From::from` 等同名即 E0034 歧义，固有方法则优先于 trait 方法；
//! - 关联常量（`__STATICS` 等）。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FnArg, ImplItem, ImplItemFn, Pat, TraitItemFn, Visibility};

use super::super::context::GenContext;

/// wrapper 实现的 std prelude trait（Clone / PartialEq / Default / From，及 blanket 的
/// Into / TryFrom / TryInto / ToOwned）的方法名：同名 Java 方法留在固有 impl
pub(crate) const STD_COLLISIONS: [&str; 11] = [
    "clone", "clone_from", "clone_into", "to_owned", "eq", "ne", "default", "from", "into", "try_from",
    "try_into",
];

/// 方法 trait 名（宏生成，不出现在可读层方法体文本中）
pub(crate) fn trait_ident(ctx: &GenContext) -> syn::Ident {
    format_ident!("{}__Methods", ctx.struct_ident)
}

/// 该固有成员是否移入方法 trait
fn moves_to_trait(f: &ImplItemFn) -> bool {
    f.sig.constness.is_none() && !STD_COLLISIONS.contains(&f.sig.ident.to_string().as_str())
}

/// trait 中的方法声明：签名同实现（形参模式去 `mut` / 非标识符模式改 `_`，无体声明不接受模式），
/// 属性去 `#[inline]`（只对有体函数有意义）
fn declaration(f: &ImplItemFn) -> TraitItemFn {
    let mut sig = f.sig.clone();
    for arg in sig.inputs.iter_mut() {
        match arg {
            FnArg::Receiver(r) => {
                if r.reference.is_none() {
                    r.mutability = None;
                }
            }
            FnArg::Typed(pt) => {
                let plain = match &*pt.pat {
                    Pat::Ident(pi) if pi.subpat.is_none() => {
                        let mut pi = pi.clone();
                        pi.mutability = None;
                        pi.by_ref = None;
                        Pat::Ident(pi)
                    }
                    _ => syn::parse_quote!(_),
                };
                *pt.pat = plain;
            }
        }
    }
    let attrs = f.attrs.iter().filter(|a| !a.path().is_ident("inline")).cloned().collect();
    TraitItemFn { attrs, sig, default: None, semi_token: Some(Default::default()) }
}

/// wrapper 成员 → (方法 trait 声明 + wrapper 的 trait 实现, 留在固有 impl 的成员)
pub(super) fn split(ctx: &GenContext, members: TokenStream2) -> syn::Result<TokenStream2> {
    let struct_ident = &ctx.struct_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let trait_ident = trait_ident(ctx);
    let all: syn::ItemImpl = syn::parse2(quote! {
        impl #impl_g #struct_ident #ty_g #where_c { #members }
    })?;

    let mut decls: Vec<TraitItemFn> = Vec::new();
    let mut impls: Vec<ImplItemFn> = Vec::new();
    let mut inherent: Vec<ImplItem> = Vec::new();
    for item in all.items {
        match item {
            ImplItem::Fn(f) if moves_to_trait(&f) => {
                decls.push(declaration(&f));
                let mut f = f;
                f.vis = Visibility::Inherited;
                impls.push(f);
            }
            other => inherent.push(other),
        }
    }

    let inherent_impl = if inherent.is_empty() {
        quote! {}
    } else {
        quote! { impl #impl_g #struct_ident #ty_g #where_c { #(#inherent)* } }
    };
    Ok(quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types, non_snake_case)]
        pub trait #trait_ident #impl_g : ::core::marker::Sized #where_c {
            #(#decls)*
        }
        #[allow(non_snake_case)]
        impl #impl_g #trait_ident #ty_g for #struct_ident #ty_g #where_c {
            #(#impls)*
        }
        #inherent_impl
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl_of(src: &str) -> String {
        let f: ImplItemFn = syn::parse_str(src).unwrap();
        let d = declaration(&f);
        quote!(#d).to_string()
    }

    #[test]
    fn declaration_strips_patterns_and_inline() {
        let d = decl_of("#[inline] #[doc(hidden)] pub fn m(mut self, mut x: i32, (a, b): (u8, u8)) -> i32 { x }");
        assert!(!d.contains("inline"), "{d}");
        assert!(d.contains("doc (hidden)"), "{d}");
        assert!(d.contains("fn m (self , x : i32 , _ : (u8 , u8)) -> i32 ;"), "{d}");
        let d = decl_of("pub fn n(&mut self) {}");
        assert!(d.contains("fn n (& mut self) ;"), "{d}");
    }

    #[test]
    fn collisions_and_const_fns_stay_inherent() {
        let keep: ImplItemFn = syn::parse_str("pub fn clone(&self) -> Result<Object> { todo() }").unwrap();
        assert!(!moves_to_trait(&keep));
        let keep: ImplItemFn = syn::parse_str("pub const fn k() -> u8 { 0 }").unwrap();
        assert!(!moves_to_trait(&keep));
        let mv: ImplItemFn = syn::parse_str("pub fn size(&self) -> Result<i32> { Ok(0) }").unwrap();
        assert!(moves_to_trait(&mv));
    }

    const GENERIC_CLASS: &str = r#"
    #[binary_name = "a/L"]
    pub struct L<E> { pub n: i32 }
    impl<E> L<E> {
        pub static COUNT: i32;
        #[java_method(name = "<init>", descriptor = "()V", access = "public")]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this._init_not_null();
            Self::__init_on(this)
        }
        #[doc(hidden)]
        pub fn __init_on(mut this: Self) -> Result<Self> { Ok(this) }
        #[java_method(name = "make", descriptor = "(I)La/L;", access = "public", modifiers = "static")]
        pub fn make(mut k: i32) -> Result<L<Object>> { k += 1; Ok(L::<Object>::default()) }
        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public")]
        pub fn clone(&self) -> Result<Object> { Ok(Object::default()) }
    }
"#;

    fn expanded() -> String {
        use std::str::FromStr;
        let out = super::super::super::super::expand(TokenStream2::from_str(GENERIC_CLASS).unwrap()).to_string();
        assert!(!out.contains("compile_error"), "{out}");
        out
    }

    /// 方法 trait 形参与 wrapper 形参一一对应、实现头 `impl<E> L__Methods<E> for L<E>`：
    /// 静态调用点 `L::new()` / `L::make(..)` 的 `E` 由 Self 类型唯一确定，推断与固有 impl 一致
    #[test]
    fn trait_mirrors_wrapper_generics() {
        let out = expanded();
        assert!(out.contains("pub trait L__Methods < E > : :: core :: marker :: Sized"), "{out}");
        assert!(out.contains("impl < E > L__Methods < E > for L < E >"), "{out}");
        assert!(out.contains("fn new () -> Result < Self > ;"), "{out}");
        assert!(out.contains("fn make (k : i32) -> Result < L < Object > > ;"), "{out}");
        for m in ["fn _init_not_null (& mut self) ;", "fn __get_n (& self) -> i32 ;", "fn COUNT () -> Result < i32 > ;",
                  "fn __class_init () -> Result < () > ;"] {
            assert!(out.contains(m), "{m}: {out}");
        }
    }

    /// 与 std prelude trait 同名的 Java 方法（`clone`）留在固有 impl（trait 内同名即 E0034）
    #[test]
    fn std_named_method_stays_inherent() {
        let out = expanded();
        let tr = out.split("pub trait L__Methods").nth(1).unwrap().split("impl <").next().unwrap();
        assert!(!tr.contains("fn clone"), "{tr}");
        assert!(out.contains("pub fn clone (& self) -> Result < Object >"), "{out}");
    }
}
