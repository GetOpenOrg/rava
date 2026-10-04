//! Java 类型转换：BINARY_NAME 常量、`From<Object>`（downcast / 擦除重建）、
//! `Into<Object>`、`From<Child> for Ancestor`（vtable trait upcasting，含多级跳跃）。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::super::erasure::type_args_arity;
use super::context::GenContext;
use super::storage_hooks::hook_ident;

/// 返回 (声明层转换项, 存储类型 `X__inner` 上的 BINARY_NAME 常量——随存储层进实现层)
pub(crate) fn generate(ctx: &GenContext) -> (TokenStream2, TokenStream2) {
    let struct_ident = &ctx.struct_ident;
    let inner_ident = &ctx.inner_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let binary_name = &ctx.meta.binary_name;
    let from_any = hook_ident(ctx, "from_any");

    // ══════════════════════════════════════════════════════════════════════════
    // 8. BINARY_NAME 常量
    // ══════════════════════════════════════════════════════════════════════════

    let (binary_name_impl, inner_binary_name): (TokenStream2, TokenStream2) = if !binary_name.is_empty() {
        (quote! {
            impl #impl_g #struct_ident #ty_g #where_c {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        }, quote! {
            // vtable 上下文（impl XxxVTable for __inner）中的方法体里 Self = __inner，
            // Self::BINARY_NAME 必须同样可解析（__inner 非泛型，A-1 存储层擦除）
            impl #inner_ident {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        })
    } else {
        (quote! {}, quote! {})
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 9. From<Object> for ClassName
    // ══════════════════════════════════════════════════════════════════════════

    let obj = quote! { Object };
    // A-1 存储层擦除：__inner 非泛型 → 同一泛型类的所有类型实例化共享同一存储形态。
    // `From<Object> for X<A>` 对任意 A 成立：
    //   1. 快路径：运行时类与目标实例化同族同参（含子类按超类实参映射的视图）→ slot 命中；
    //   2. 擦除路径：运行时类是本类**或其子类**（Java 泛型运行时本就擦除）→ 经
    //      `__erased_vtable` + `__erased_inner` 取回 (vtable, 存储) 部件，重建本实例化
    //      视图（共享存储与对象标识）。子类值经超类 vtable supertrait 上转，任意实例化
    //      均可重建（`Enum::<Object>::from(枚举常量)` 即此形态）；
    //   3. 其余（运行时类不是本类族）→ checkcast 的 ClassCastException（既有语义）。
    // 判定逻辑与具体类无关，全在 runtime 的 `__class_from_object`；按类只转交本类的部件构造入口
    // 与存储钩子（拆 crate §7.5.4 #2）。
    let from_object_impl = quote! {
        impl #impl_g From<#obj> for #struct_ident #ty_g #where_c {
            fn from(obj: #obj) -> Self {
                __class_from_object(obj, #binary_name, Self::__from_parts, #from_any)
            }
        }
    };

    // R-1: binary_name 为空的类需要手动 Into<Object>
    let into_object_impl: TokenStream2 = if binary_name.is_empty() {
        quote! {
            impl #impl_g Into<#obj> for #struct_ident #ty_g #where_c {
                fn into(self) -> #obj { #obj::from_any(self) }
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 10. From<ClassName> for each ancestor（vtable trait upcasting，含多级跳跃）
    // ══════════════════════════════════════════════════════════════════════════

    let from_child_for_parent: TokenStream2 = if ctx.meta.superclass.is_some() {
        // 为每个祖先（排除 Object 和自身）生成 From<Self> for Ancestor
        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = ctx.meta.all_superclasses.clone();
        let impls: Vec<TokenStream2> = ancestors.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let anc_vtable = format_ident!("{}__VTable", anc_name);
            let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            if atag.is_empty() {
                // 非泛型祖先：目标无类型实参
                return quote! {
                    impl #impl_g From<#struct_ident #ty_g> for #anc_ident #where_c {
                        fn from(child: #struct_ident #ty_g) -> #anc_ident {
                            #anc_ident::__from_parts(
                                child.vtable as __Shared<dyn #anc_vtable>,
                                child.any,
                                child._jvm_null,
                            )
                        }
                    }
                };
            }
            // 泛型祖先：擦除实例化视图（A-1 γ'）——对祖先的**任意**类型实参成立
            // （Java 泛型运行时擦除，vtable 非泛型后 upcast 不再依赖实参一致；
            // `CountedCompleter<Object>: From<Sorter<T>>` 这类跨实例化 upcast）。
            // 祖先形参以带宏标准 bound 的新形参承载（宏为所有类的形参注入同一组
            // Clone/Default/'static/From<Object>/Into<Object>，祖先 wrapper 的 impl
            // 上下文恰要求这组 bound）。
            let arity = type_args_arity(&atag);
            let mut gamma_gen = ctx.gen.clone();
            for i in 0..arity {
                let pid = format_ident!("__Anc{}", i);
                let p: syn::TypeParam = syn::parse_quote! {
                    #pid : Clone + Default + 'static
                        + ::std::convert::From<Object> + ::std::convert::Into<Object>
                        + __ThreadSafe
                };
                gamma_gen.params.push(syn::GenericParam::Type(p));
            }
            let (gamma_impl_g, _, gamma_where_c) = gamma_gen.split_for_impl();
            let anc_params: Vec<Ident> = (0..arity)
                .map(|i| format_ident!("__Anc{}", i))
                .collect();
            let anc_ty_args = quote! { <#(#anc_params),*> };
            quote! {
                impl #gamma_impl_g
                    From<#struct_ident #ty_g> for #anc_ident #anc_ty_args #gamma_where_c
                {
                    fn from(child: #struct_ident #ty_g) -> #anc_ident #anc_ty_args {
                        #anc_ident::#anc_ty_args::__from_parts(
                            child.vtable as __Shared<dyn #anc_vtable>,
                            child.any,
                            child._jvm_null,
                        )
                    }
                }
            }
        }).collect();
        quote! { #(#impls)* }
    } else {
        quote! {}
    };

    (quote! {
        #binary_name_impl
        #from_object_impl
        #into_object_impl
        #from_child_for_parent
    }, inner_binary_name)
}
