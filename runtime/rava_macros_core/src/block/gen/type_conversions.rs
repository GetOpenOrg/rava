//! Java 类型转换：BINARY_NAME 常量、`From<Object>`（downcast / 擦除重建）、
//! `Into<Object>`、`From<Child> for Ancestor`（vtable trait upcasting，含多级跳跃）。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::super::erasure::type_args_arity;
use super::super::util::is_basic;
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

    // ══════════════════════════════════════════════════════════════════════════
    // 8. BINARY_NAME 常量
    // ══════════════════════════════════════════════════════════════════════════

    let class_desc = class_desc(ctx);
    let (binary_name_impl, inner_binary_name): (TokenStream2, TokenStream2) = if !binary_name.is_empty() {
        (quote! {
            #class_desc
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
    //   2. 擦除路径：运行时类是本类**或其子类**（Java 泛型运行时本就擦除）→ 句柄不变，
    //      经句柄所持存储的 `__erased_vtable` 取本类视图指针，重建本实例化视图（共享存储
    //      与对象标识，S7-2）。子类值经超类 vtable supertrait 上转，任意实例化
    //      均可重建（`Enum::<Object>::from(枚举常量)` 即此形态）；
    //   3. 其余（运行时类不是本类族）→ checkcast 的 ClassCastException（既有语义）。
    // 判定逻辑与具体类无关，全在 runtime 的 `__class_from_object`（读描述符）；按类只转交本类描述符
    // 与部件构造入口（拆 crate §7.5.4 #2）。
    let from_object_impl = quote! {
        impl #impl_g From<#obj> for #struct_ident #ty_g #where_c {
            fn from(obj: #obj) -> Self {
                __class_from_object(obj, Self::__DESC, Self::__from_parts)
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
                                child.__r.upcast(|__v| __v as &dyn #anc_vtable))
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
                            child.__r.upcast(|__v| __v as &dyn #anc_vtable))
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

/// 类静态描述符（S7-0，计划 §3.2）：模块级 `static X__DESC` + wrapper 固有常量 `X::__DESC`。
///
/// 祖先描述符经祖先 wrapper 的固有常量取得（`<Anc<Object, ..>>::__DESC`）：类型路径无需额外
/// `use`，祖先形参取 Object（宏为所有形参注入的标准 bound 对 Object 恒成立）。固有常量而非
/// trait：不引入 Java 命名空间之外的公开 trait（命名原则），名字带 `__` 前缀避开 Java 静态字段。
/// 描述符必须是 `static`（地址即类标识，`is_subclass_of` 按地址比较）；const 内联会产生多份副本。
fn class_desc(ctx: &GenContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let binary_name = &ctx.meta.binary_name;
    let desc_ident = format_ident!("{}__DESC", struct_ident);
    let ancestors: Vec<TokenStream2> = ctx.meta.all_superclasses.iter().map(|anc_name| {
        let anc_ident = format_ident!("{}", anc_name);
        let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
        let arity = if atag.is_empty() { 0 } else { type_args_arity(&atag) };
        if arity == 0 {
            quote! { <#anc_ident>::__DESC }
        } else {
            let objs = vec![quote! { Object }; arity];
            quote! { <#anc_ident<#(#objs),*>>::__DESC }
        }
    }).collect();
    let depth = ancestors.len() as u16;
    let mut supertypes: Vec<String> = if ctx.meta.all_supertypes.is_empty() {
        vec![binary_name.clone()]
    } else {
        ctx.meta.all_supertypes.clone()
    };
    supertypes.sort();
    supertypes.dedup();
    let fields = field_descs(ctx);
    let field_base = ctx.meta.superclass_fields.len() as u16;
    let alloc = hook_ident(ctx, "alloc");
    let offsets = hook_ident(ctx, "offsets");
    quote! {
        #[doc(hidden)]
        pub static #desc_ident: __ClassDesc = __ClassDesc {
            binary_name: #binary_name,
            depth: #depth,
            display: &[#(#ancestors,)* &#desc_ident],
            supertypes: &[#(#supertypes),*],
            fields: &[#(#fields,)*],
            field_base: #field_base,
            alloc: || #alloc().into_object(&#desc_ident),
            offsets: || #offsets(),
            typed_null: {
                static __TYPED_NULL: __TypedNull = __TypedNull::new(#binary_name, Some(&#desc_ident));
                &__TYPED_NULL
            },
        };
        impl #impl_g #struct_ident #ty_g #where_c {
            #[doc(hidden)]
            pub const __DESC: &'static __ClassDesc = &#desc_ident;
        }
    }
}

/// 本类自有实例字段的描述（S7-3，与存储布局同序）：Java 名取 `field_slots` 中本类的映射（关键字
/// 后缀 / `$` 替换 / 遮蔽后缀），缺省与 Rust 名相同；种类按存储形态——擦除字段与引用字段
/// 带按载体类型实例化的单元协议（擦除载体为 Object），基本字段按 Rust 类型细分。
fn field_descs(ctx: &GenContext) -> Vec<TokenStream2> {
    let binary_name = &ctx.meta.binary_name;
    ctx.fields.iter().map(|(name, ty)| {
        let rust = name.to_string();
        let java = ctx.meta.field_slots.iter()
            .find(|(decl, _, r)| decl == binary_name && *r == rust)
            .map_or(rust.as_str(), |(_, java, _)| java.as_str());
        // 引用字段的单元协议按 volatile 修饰取序（`of_ref_volatile`：Unsafe / VarHandle 按名读写
        // 与字段访问器同为 SeqCst 族，无 GC 文档第四节小步 B）
        let of_ref = if ctx.is_volatile(name) { format_ident!("of_ref_volatile") } else { format_ident!("of_ref") };
        if ctx.is_erased(name) {
            return quote! { __FieldDesc::#of_ref::<Object>(#java, #rust) };
        }
        if !is_basic(ty) {
            return quote! { __FieldDesc::#of_ref::<#ty>(#java, #rust) };
        }
        let k = match quote!(#ty).to_string().as_str() {
            "bool" => "Bool",
            "i8" => "Byte",
            "i16" => "Short",
            "u16" => "Char",
            "i32" => "Int",
            "f32" => "Float",
            "i64" => "Long",
            "f64" => "Double",
            _ => "Prim",
        };
        let k = format_ident!("{}", k);
        quote! { __FieldDesc::of_prim(#java, #rust, __FieldKind::#k) }
    }).collect()
}
