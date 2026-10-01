//! Java 类型包装：wrapper struct（vtable + any 两指针 + JVM null 标志）、
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
use super::storage_hooks::hook_ident;

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
            // 部件对实现层 crate（存储层 impl 按部件构造本类视图）可见
            #[doc(hidden)]
            pub vtable: __Shared<dyn #vtable_trait_ident>,
            #[doc(hidden)]
            pub any: __AnyRef,
            /// JVM null 标志：Default::default() = true（null），构造后调用 _init_not_null() = false
            pub _jvm_null: bool,
            #phantom_field
        }
    };

    // 跨 crate 子类（用户类继承 JDK 类）的向上转换 / 运行时类视图需要按部件重建祖先 wrapper；
    // 字段保持 crate 私有，经此构造入口完成。
    let wrapper_from_parts = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            #[doc(hidden)]
            pub fn __from_parts(
                vtable: __Shared<dyn #vtable_trait_ident>,
                any: __AnyRef,
                is_null: bool,
            ) -> Self {
                #struct_ident { vtable, any, _jvm_null: is_null, #phantom_init }
            }

            /// invokevirtual 在 Object 接收者上的类 vtable 分派入口（§6 步骤 4）：
            /// 运行时类是本类或其子类 → `Some(本实例化视图)`。vtable / 存储部件取自
            /// 原对象（与 `From<Object>` 擦除路径同源——共享存储与对象标识，子类
            /// vtable 经 supertrait 上转）；其余（闭包、无运行时类值）→ `None`，
            /// 调用方回落闭包 SAM 分支。对任意类型实参成立（Java 泛型运行时擦除）。
            #[doc(hidden)]
            pub fn __virtual_view(obj: &Object) -> ::std::option::Option<Self> {
                let mut __vt: ::std::option::Option<
                    __Shared<dyn #vtable_trait_ident>> = ::std::option::Option::None;
                ObjectVTable::__erased_vtable(__Shared::clone(&obj.0), &mut __vt);
                let __vt = __vt?;
                let mut __store: ::std::option::Option<
                    __AnyRef> = ::std::option::Option::None;
                ObjectVTable::__erased_inner(__Shared::clone(&obj.0), &mut __store);
                Some(#struct_ident {
                    vtable: __vt,
                    any: __store?,
                    _jvm_null: false,
                    #phantom_init
                })
            }
        }
    };

    let alloc = hook_ident(ctx, "alloc");
    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                let (vtable, any) = #alloc();
                #struct_ident {
                    vtable,
                    any,
                    _jvm_null: true,
                    #phantom_init
                }
            }
        }
    };

    let wrapper_clone = quote! {
        impl #impl_g ::std::clone::Clone for #struct_ident #ty_g #where_c {
            fn clone(&self) -> Self {
                #struct_ident {
                    vtable: __Shared::clone(&self.vtable),
                    any: __Shared::clone(&self.any),
                    _jvm_null: self._jvm_null,
                    #phantom_init
                }
            }
        }
    };

    let wrapper_partialeq = quote! {
        impl #impl_g ::std::cmp::PartialEq for #struct_ident #ty_g #where_c {
            fn eq(&self, other: &Self) -> bool {
                match (self._jvm_null, other._jvm_null) {
                    (true, true) => true,
                    (false, false) => self.vtable.__identity() == other.vtable.__identity(),
                    _ => false,
                }
            }
        }
    };

    let wrapper_debug = quote! {
        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_ident), ObjectVTable::__obj_str(&*self.vtable))
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
