//! `java_enum!` 块级宏实现 — Java 枚举类型展开。
//!
//! 展开产物：
//!   - `#[derive(Debug, Clone, PartialEq)] pub enum Name { V1, V2, ... }`
//!   - `impl Name { ordinal(), name(), values(), valueOf(), 用户定义方法 }`
//!   - `impl ObjectVTable for Name { hashCode, toString, is_instance_of, as_any }`
//!
//! 输入语法（非标准 Rust，分号分隔变体与方法）：
//! ```text
//! java_enum! {
//!     #[binary_name = "Foo$Bar"]
//!     pub enum Direction {
//!         NORTH, SOUTH, EAST, WEST;
//!         pub fn opposite(&self) -> Result<Direction> { ... }
//!     }
//! }
//! ```

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Block, Expr, Ident, Lit, Signature, Token, Visibility,
};

// ══════════════════════════════════════════════════════════════════════════════
// 输入数据结构
// ══════════════════════════════════════════════════════════════════════════════

struct FnItem {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    block: Option<Block>,
}

struct JavaEnumInput {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    /// 变体名列表，顺序即 ordinal 顺序
    variants: Vec<Ident>,
    /// 分号后的方法列表
    methods: Vec<FnItem>,
}

impl Parse for JavaEnumInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 读取外部属性（#[binary_name = "..."] 等）
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let _: Token![enum] = input.parse()?;
        let name: Ident = input.parse()?;

        // 花括号体
        let content;
        syn::braced!(content in input);

        // 解析变体（逗号分隔的 Ident，直到 `;`）
        let mut variants: Vec<Ident> = Vec::new();
        while !content.is_empty() && !content.peek(Token![;]) {
            // 忽略变体上的属性（一般不会有，但语法允许）
            let _ = content.call(Attribute::parse_outer)?;
            let variant_name: Ident = content.parse()?;
            variants.push(variant_name);
            // 末尾逗号可选
            if content.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
            }
        }

        // 消费分隔符 `;`
        let mut methods: Vec<FnItem> = Vec::new();
        if !content.is_empty() && content.peek(Token![;]) {
            let _: Token![;] = content.parse()?;
            methods = parse_enum_methods(&content)?;
        }

        Ok(JavaEnumInput { attrs, vis, name, variants, methods })
    }
}

/// 解析 `; ... }` 之间的方法列表（标准 Rust fn 项）。
fn parse_enum_methods(input: ParseStream) -> syn::Result<Vec<FnItem>> {
    let mut out = Vec::new();
    while !input.is_empty() {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let block = if input.peek(Token![;]) {
            let _: Token![;] = input.parse()?;
            None
        } else {
            Some(input.parse::<Block>()?)
        };
        out.push(FnItem { attrs, vis, sig, block });
    }
    Ok(out)
}

// ══════════════════════════════════════════════════════════════════════════════
// 属性工具
// ══════════════════════════════════════════════════════════════════════════════

/// 从 `#[name = "value"]` 形式属性读取字符串字面量。
fn attr_lit_str(attrs: &[Attribute], name: &str) -> Option<String> {
    for a in attrs {
        if a.path().is_ident(name) {
            if let syn::Meta::NameValue(nv) = &a.meta {
                if let Expr::Lit(el) = &nv.value {
                    if let Lit::Str(s) = &el.lit {
                        return Some(s.value());
                    }
                }
            }
        }
    }
    None
}

/// 剔除宏专用属性，保留 Rust 侧有意义的属性（如 `#[allow(...)]`）。
const META_ATTRS: &[&str] = &[
    "binary_name", "descriptor", "generic_signature", "native", "readonly",
    "field_sig", "java_method", "java_native", "jvm_native",
];

fn strip_meta_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs.iter().filter(|a| {
        let p = a.path();
        !META_ATTRS.iter().any(|n| p.is_ident(n))
    }).collect()
}

// ══════════════════════════════════════════════════════════════════════════════
// 展开入口
// ══════════════════════════════════════════════════════════════════════════════

pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<JavaEnumInput>(input) {
        Ok(v) => expand_inner(v),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(input: JavaEnumInput) -> TokenStream2 {
    let JavaEnumInput { attrs, vis, name, variants, methods } = input;

    // binary_name 用于 is_instance_of 和 stub 消息
    let binary_name = attr_lit_str(&attrs, "binary_name")
        .unwrap_or_else(|| name.to_string());

    // ── 1. enum 定义 ─────────────────────────────────────────────────────────

    let enum_def = quote! {
        #[derive(Debug, Clone, PartialEq)]
        #vis enum #name {
            #( #variants, )*
        }
    };

    // ── 2. ordinal() — 返回枚举常量的整数序号（从 0 开始）────────────────────

    let ordinal_arms: Vec<TokenStream2> = variants.iter().enumerate().map(|(i, v)| {
        let idx = i as i32;
        quote! { #name::#v => #idx, }
    }).collect();

    let ordinal_fn = quote! {
        /// 返回枚举常量的整数序号（对应 Java `Enum.ordinal()`）。
        pub fn ordinal(&self) -> i32 {
            match self {
                #( #ordinal_arms )*
            }
        }
    };

    // ── 3. name() — 返回枚举常量名字符串 ─────────────────────────────────────

    let name_arms: Vec<TokenStream2> = variants.iter().map(|v| {
        let s = v.to_string();
        quote! { #name::#v => #s.to_owned(), }
    }).collect();

    let name_fn = quote! {
        /// 返回枚举常量名（对应 Java `Enum.name()`）。
        pub fn name(&self) -> ::std::string::String {
            match self {
                #( #name_arms )*
            }
        }
    };

    // ── 4. values() — 返回所有枚举常量的 Vec ─────────────────────────────────

    let values_fn = quote! {
        /// 返回所有枚举常量（对应 Java `Enum.values()`）。
        pub fn values() -> ::std::vec::Vec<#name> {
            vec![ #( #name::#variants, )* ]
        }
    };

    // ── 5. valueOf(name) — 按名称查找枚举常量 ────────────────────────────────

    let value_of_arms: Vec<TokenStream2> = variants.iter().map(|v| {
        let s = v.to_string();
        quote! { #s => ::std::option::Option::Some(#name::#v), }
    }).collect();

    let value_of_fn = quote! {
        /// 按名称查找枚举常量（对应 Java `Enum.valueOf()`）。
        pub fn valueOf(name: &str) -> ::std::option::Option<#name> {
            match name {
                #( #value_of_arms )*
                _ => ::std::option::Option::None,
            }
        }
    };

    // ── 6. 用户定义方法 ───────────────────────────────────────────────────────

    let user_methods: Vec<TokenStream2> = methods.iter().map(|f| {
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let vis = &f.vis;
        let sig = &f.sig;
        match &f.block {
            Some(block) => quote! {
                #( #keep_attrs )*
                #vis #sig #block
            },
            None => {
                // 无方法体 → stub panic
                let mname = sig.ident.to_string();
                let msg = format!("stub: {}.{}", binary_name, mname);
                quote! {
                    #( #keep_attrs )*
                    #vis #sig { panic!(#msg) }
                }
            }
        }
    }).collect();

    // ── 7. impl 块（ordinal + name + values + valueOf + 用户方法）────────────

    let impl_block = quote! {
        impl #name {
            #ordinal_fn
            #name_fn
            #values_fn
            #value_of_fn
            #( #user_methods )*
        }
    };

    // ── 8. impl ObjectVTable for EnumName ─────────────────────────────────────

    let obj_vtable_impl = quote! {
        impl ObjectVTable for #name {
            fn hashCode(&self) -> i32 { self.ordinal() }
            fn toString(&self) -> ::std::string::String { format!("{:?}", self) }
            fn is_instance_of(&self, type_id: &str) -> bool {
                type_id == #binary_name
            }
            fn as_any(&self) -> &dyn ::std::any::Any { self }
        }
    };

    // ── 最终组合 ──────────────────────────────────────────────────────────────

    quote! {
        #enum_def
        #impl_block
        #obj_vtable_impl
    }
}
