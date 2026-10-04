//! Java 类型包装：wrapper struct（类型化引用 `__r` = 句柄 + 本类视图指针，S7-2）、
//! Default/Clone/PartialEq/Debug、impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）、
//! wrapper impl 块（字段访问器委托 + 虚方法委托 + 构造器 new/__init_on 双入口）。
//!
//! - 本文件：§5 wrapper struct 与基础 trait impl，并按固定顺序拼装各段
//! - `object_vtable`：§6 impl ObjectVTable for Wrapper
//! - `methods`：§7 wrapper impl 块（访问器 / 虚方法委托 / 继承转发 / 构造器 / 静态与类初始化）
//! - `body_fns`：方法体函数化（体移入模块级 `__jbm_<类>__<方法>`，wrapper 方法为外壳）

mod body_fns;
mod methods;
mod object_vtable;

pub(crate) use body_fns::functionize_applicable;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::context::GenContext;

/// §5-§7 Wrapper struct + Default/Clone/PartialEq/Debug + impl ObjectVTable for Wrapper
/// + wrapper impl 块；第二项为方法体函数（`__jbm_`，拆层时进实现层）。
pub(crate) fn generate(ctx: &GenContext) -> syn::Result<(TokenStream2, Vec<TokenStream2>)> {
    let struct_ident = &ctx.struct_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let phantom_field = &ctx.phantom_field;
    let phantom_init = &ctx.phantom_init;

    // ══════════════════════════════════════════════════════════════════════════
    // 5. Wrapper struct + Default + Clone
    // ══════════════════════════════════════════════════════════════════════════

    let wrapper_struct = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident #impl_g #where_c {
            // 类型化引用（S7-2）：对象句柄（null 即空句柄，不分配）+ 本类视图指针。
            // 对实现层 / 用户 crate 可见（跨 crate 子类的上转与分派经它）
            pub __r: __Ref<dyn #vtable_trait_ident>,
            #phantom_field
        }
    };

    // 跨 crate 子类（用户类继承 JDK 类）的向上转换 / 运行时类视图按引用重建祖先 wrapper，
    // 经此构造入口完成。
    let wrapper_from_parts = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            pub fn __from_parts(__r: __Ref<dyn #vtable_trait_ident>) -> Self {
                #struct_ident { __r, #phantom_init }
            }

            // invokevirtual 在 Object 接收者上的类 vtable 分派入口（§6 步骤 4）：
            // 运行时类是本类或其子类 → `Some(本实例化视图)`。句柄取自原对象（与
            // `From<Object>` 擦除路径同源——共享存储与对象标识，子类 vtable 经 supertrait
            // 上转）；其余（闭包、无运行时类值）→ `None`，调用方回落闭包 SAM 分支。
            // 对任意类型实参成立（Java 泛型运行时擦除）。
            pub fn __virtual_view(obj: &Object) -> ::std::option::Option<Self> {
                __erased_view(obj, Self::__from_parts)
            }
        }
    };

    // null 引用不分配存储（S7-2）：构造器经 `_init_not_null` 才分配
    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                #struct_ident { __r: __Ref::NULL, #phantom_init }
            }
        }
    };

    let wrapper_clone = quote! {
        impl #impl_g ::std::clone::Clone for #struct_ident #ty_g #where_c {
            fn clone(&self) -> Self {
                #struct_ident { __r: ::std::clone::Clone::clone(&self.__r), #phantom_init }
            }
        }
    };

    let wrapper_partialeq = quote! {
        impl #impl_g ::std::cmp::PartialEq for #struct_ident #ty_g #where_c {
            fn eq(&self, other: &Self) -> bool {
                __Handle::same(self.__r.handle(), other.__r.handle())
            }
        }
    };

    let wrapper_debug = quote! {
        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_ident), self.__r.handle().obj_str())
            }
        }
    };

    let obj_vtable_for_wrapper = object_vtable::generate(ctx);
    let (wrapper_impl, body_fns) = methods::generate(ctx)?;

    Ok((quote! {
        #wrapper_struct
        #wrapper_from_parts
        #wrapper_default
        #wrapper_clone
        #wrapper_partialeq
        #wrapper_debug
        #obj_vtable_for_wrapper
        #wrapper_impl
    }, body_fns))
}
