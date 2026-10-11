//! Java 类型包装：wrapper struct（`__r` = 对象句柄，S7-2 / §9.9 K1）、
//! Default/Clone/PartialEq/Debug、`From<Wrapper> for Object`（S7-2b：Object 直接持有存储）、
//! wrapper impl 块（字段访问器委托 + 虚方法委托 + 构造器 new/__init_on 双入口）。
//!
//! - 本文件：§5 wrapper struct 与基础 trait impl，并按固定顺序拼装各段
//! - `into_object`：§6 `From<Wrapper> for Object`
//! - `methods`：§7 wrapper impl 块（访问器 / 虚方法委托 / 继承转发 / 构造器 / 静态与类初始化）
//! - `body_fns`：方法体函数化（体移入模块级 `__jbm_<类>__<方法>`，wrapper 方法为外壳）

mod body_fns;
mod methods;
mod into_object;

pub(crate) use body_fns::functionize_applicable;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::context::GenContext;

/// §5-§7 Wrapper struct + Default/Clone/PartialEq/Debug + `From<Wrapper> for Object`
/// + wrapper impl 块；第二项为方法体函数（`__jbm_`，拆层时进实现层）。
pub(crate) fn generate(ctx: &GenContext) -> syn::Result<(TokenStream2, Vec<TokenStream2>)> {
    let struct_ident = &ctx.struct_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let phantom_field = &ctx.phantom_field;
    let phantom_init = &ctx.phantom_init;
    // 本类在描述符 display 表中的深度（与 `class_desc` 同口径：祖先数，不含 Object）
    let depth = ctx.meta.all_superclasses.len() as u16;

    // ══════════════════════════════════════════════════════════════════════════
    // 5. Wrapper struct + Default + Clone
    // ══════════════════════════════════════════════════════════════════════════

    let wrapper_struct = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident #impl_g #where_c {
            // 对象句柄（S7-2；null 即空句柄，不分配）。不含本类视图指针（S7 计划 §9.9 K1）：
            // wrapper 与签名无关，分派经 `__vt()` 按本类深度现取视图。
            // 对实现层 / 用户 crate 可见（跨 crate 子类的上转与分派经它）
            pub __r: __Handle,
            #phantom_field
        }
    };

    // 跨 crate 子类（用户类继承 JDK 类）的向上转换 / 运行时类视图按引用重建祖先 wrapper，
    // 经此构造入口完成。
    let wrapper_from_parts = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            pub fn __from_parts(__r: __Handle) -> Self {
                #struct_ident { __r, #phantom_init }
            }

            // 构建期引导映像的对象引用（常量求值，映像模块的静态初值）
            #[doc(hidden)]
            pub const fn __from_image(__r: __Handle) -> Self {
                #struct_ident { __r, #phantom_init }
            }

            // 虚分派 / 字段访问入口（S7 计划 §3.1 取法 A）：按本类 display 深度向运行时类
            // 存储取本类视图（一次间接调用 + 一次槽类型比较）
            #[doc(hidden)]
            #[inline]
            pub fn __vt(&self) -> &dyn #vtable_trait_ident {
                self.__r.view::<dyn #vtable_trait_ident>(#depth)
            }

            // invokevirtual 在 Object 接收者上的类 vtable 分派入口（§6 步骤 4）：
            // 运行时类是本类或其子类 → `Some(本实例化视图)`。句柄取自原对象（与
            // `From<Object>` 擦除路径同源——共享存储与对象标识，子类 vtable 经 supertrait
            // 上转）；其余（闭包、无运行时类值）→ `None`，调用方回落闭包 SAM 分支。
            // 对任意类型实参成立（Java 泛型运行时擦除）。
            pub fn __virtual_view(obj: &Object) -> ::std::option::Option<Self> {
                __erased_view(obj, Self::__DESC, Self::__from_parts)
            }

            // Java null 判定（S7-2b：wrapper 不再实现 ObjectVTable，判空是固有方法；
            // 与接口载体的同名固有方法同形）
            #[inline]
            pub fn is_jvm_null(&self) -> bool { self.__r.is_none() }

            // getfield / putfield 接收者判空（`__NonNull` 对 wrapper 的固有同名入口）
            #[inline]
            pub fn __nn(&self) -> Result<&Self> {
                if self.__r.is_none() { Err(JvmError::null_pointer()) } else { Ok(self) }
            }
        }
    };

    // null 引用不分配存储（S7-2）：构造器经 `_init_not_null` 才分配
    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                #struct_ident { __r: __Handle::NULL, #phantom_init }
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
                __Handle::same(&self.__r, &other.__r)
            }
        }
    };

    let wrapper_debug = quote! {
        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_ident), self.__r.obj_str())
            }
        }
    };

    let into_object = into_object::generate(ctx);
    let (wrapper_impl, body_fns) = methods::generate(ctx)?;

    Ok((quote! {
        #wrapper_struct
        #wrapper_from_parts
        #wrapper_default
        #wrapper_clone
        #wrapper_partialeq
        #wrapper_debug
        #into_object
        #wrapper_impl
    }, body_fns))
}
