use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, GenericParam, LitStr, LitBool};

mod block;

/// `java_class! { ... }` — 块级宏（方案 2026-09-16 的 `java_class!` 块级宏统一方案）。
///
/// 同时持有 struct 字段定义与 impl 块方法体，在一个宏调用里完成全部展开：
///   - Inner struct（flat layout）+ newtype 包装
///   - 字段访问器（基本类型 Cell / 引用类型 RefCell，borrow 窗口最小化）
///   - 方法体 token 重写（self.field → 访问器调用）
///   - JavaObject / ObjectVTable / Upcast / native 存根生成
#[proc_macro]
pub fn java_class(input: TokenStream) -> TokenStream {
    block::expand(input.into()).into()
}

/// 标记该方法实现了 Java 字节码中的 `ACC_NATIVE` 方法。
#[proc_macro_attribute]
pub fn jvm_native(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// 标记该方法属于内部边界类（`jdk/internal/`、`sun/`），BFS 截断后整体手写。
#[proc_macro_attribute]
pub fn jvm_boundary(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// 标记该方法是 Rust 侧人机工程学扩展，Java 规范中不存在。
#[proc_macro_attribute]
pub fn jvm_ext(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// `#[java_rta_macros::java_method(name = "...", descriptor = "...", ...)]`
///
/// 携带 Java 字节码方法元数据（名称、描述符、访问标志等），供宏和工具链读取。
/// 当前行为：透传（identity passthrough），未来扩展 SAM 检测、泛型签名校验等。
#[proc_macro_attribute]
pub fn java_method(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// `#[java_rta_macros::java_native(name = "...", descriptor = "...", ...)]`
///
/// 携带 `ACC_NATIVE` 方法的 Java 字节码元数据。当前行为：透传。
#[proc_macro_attribute]
pub fn java_native(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// `#[java_rta_macros::java_class_attr(binary_name = "...", all_supertypes = "...", ...)]`
///
/// 迁移期保留的旧属性宏（方案 §13）。块级宏 `java_class!` 稳定后废弃。
/// 为 Java 翻译类自动生成：
///   - `pub const BINARY_NAME: &'static str`    — 类的 JVM 二进制名（Arch-6）
///   - `pub fn __is_instance_of_fn`             — instanceof 辅助函数（Arch-2）
///   - `impl Into<Object>`                      — 封装为 Object，携带类型标识
///   - `impl From<Object>`                      — 从 Object 中取出（checkcast）
///   - `impl Debug`                             — 基础调试输出
#[proc_macro_attribute]
pub fn java_class_attr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    // ── 解析属性参数 ──────────────────────────────────────────────────────────
    let mut binary_name = String::new();
    let mut all_supertypes: Vec<String> = Vec::new();
    let mut is_interface = false;
    // codegen 为生成类设置；handwritten java_runtime 类缺省 false（不转发）
    let mut has_to_string_method = false;
    let mut has_hash_code_method = false;

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("binary_name") {
            binary_name = meta.value()?.parse::<LitStr>()?.value();
        } else if meta.path.is_ident("all_supertypes") {
            let val = meta.value()?.parse::<LitStr>()?.value();
            if !val.is_empty() {
                all_supertypes = val.split(';')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_owned())
                    .collect();
            }
        } else if meta.path.is_ident("is_interface") {
            is_interface = meta.value()?.parse::<LitBool>()?.value();
        } else if meta.path.is_ident("has_to_string_method") {
            has_to_string_method = meta.value()?.parse::<LitBool>()?.value();
        } else if meta.path.is_ident("has_hash_code_method") {
            has_hash_code_method = meta.value()?.parse::<LitBool>()?.value();
        } else {
            // 跳过其他已知键（super_class, interfaces, access, modifiers 等）
            if meta.input.peek(syn::Token![=]) {
                let value = meta.value()?;
                let _: syn::Expr = value.parse()?;
            }
        }
        Ok(())
    });
    parse_macro_input!(attr with parser);

    // ── 接口：生成 type alias `pub type <Name> = Object;`，不生成 struct ───────
    // Arch-1：接口变量在运行时就是 Object，方法调用通过 downcast 到具体类型派发。
    if is_interface {
        let expanded = quote! {
            pub type #name = Object;
        };
        return expanded.into();
    }

    // ── 泛型参数：补充 Clone + 'static bound ──────────────────────────────────
    let mut gen = input.generics.clone();
    for param in &mut gen.params {
        if let GenericParam::Type(tp) = param {
            let has_clone = tp.bounds.iter().any(|b| {
                if let syn::TypeParamBound::Trait(t) = b {
                    t.path.segments.last().map_or(false, |s| s.ident == "Clone")
                } else { false }
            });
            let has_static = tp.bounds.iter().any(|b| {
                matches!(b, syn::TypeParamBound::Lifetime(l) if l.ident == "static")
            });
            if !has_clone { tp.bounds.push(syn::parse_quote!(Clone)); }
            if !has_static { tp.bounds.push(syn::parse_quote!('static)); }
        }
    }

    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    // 每个生成文件都已通过 `use crate::java::lang::*` 或
    // `use java_runtime::java::lang::*` 将 Object 引入作用域。
    let obj = quote! { Object };

    // ── BINARY_NAME 常量 ──────────────────────────────────────────────────────
    let binary_name_impl: TokenStream2 = if !binary_name.is_empty() {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        }
    } else {
        quote! {}
    };


    // ── ObjectVTable impl（Arch-4：虚方法派发；Arch-2：instanceof）───────────────
    let vtable_impl: TokenStream2 = if !is_interface && !binary_name.is_empty() {
        let check_types: Vec<&str> = if !all_supertypes.is_empty() {
            all_supertypes.iter().map(|s| s.as_str()).collect()
        } else {
            vec![binary_name.as_str()]
        };
        let patterns = check_types.iter().map(|s| quote! { #s });
        let to_string_fwd: TokenStream2 = if has_to_string_method {
            quote! {
                // Java toString() 返回 Result<lang::String>，需通过 Display 转为 std::string::String
                fn toString(&self) -> ::std::string::String {
                    Self::toString(self)
                        .map(|s| ::std::format!("{}", s))
                        .unwrap_or_else(|_| ::std::any::type_name::<Self>().to_owned())
                }
            }
        } else { quote! {} };
        let hash_code_fwd: TokenStream2 = if has_hash_code_method {
            quote! {
                fn hashCode(&self) -> i32 {
                    Self::hashCode(self).unwrap_or(0)
                }
            }
        } else { quote! {} };
        quote! {
            impl #impl_generics ObjectVTable for #name #ty_generics #where_clause {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    matches!(type_id, #(#patterns)|*)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    } else {
        quote! {}
    };

    // ── Into<Object> ──────────────────────────────────────────────────────────
    // 具体类（已有 ObjectVTable impl）：直接通过 Rc::new(self) 存储，支持 instanceof/toString 派发
    // 接口或无 binary_name：fallback 到 from_any（JvmRef 包装）
    let into_impl: TokenStream2 = if !is_interface && !binary_name.is_empty() {
        quote! {
            impl #impl_generics Into<#obj> for #name #ty_generics #where_clause {
                fn into(self) -> #obj { #obj(::std::rc::Rc::new(self)) }
            }
        }
    } else {
        quote! {
            impl #impl_generics Into<#obj> for #name #ty_generics #where_clause {
                fn into(self) -> #obj { #obj::from_any(self) }
            }
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
        #binary_name_impl
        #vtable_impl
        #into_impl
        #from_impl
        #debug_impl
    };

    expanded.into()
}
