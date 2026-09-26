//! Java 类型包装：wrapper struct（vtable + any 两指针 + JVM null 标志）、
//! Default/Clone/PartialEq/Debug、impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）、
//! wrapper impl 块（字段访问器委托 + 虚方法委托 + 构造器 new/__init_on 双入口）。

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::super::class_init;
use super::super::erasure::{
    erased_call_args, erased_call_args_with, erased_call_ret_conv, erased_call_ret_conv_with,
    erasure_set_of, expand_non_virtual_fn,
};
use super::super::classify::{vtable_body_kind_gated, VTableBodyKind};
use super::super::parse::split_type_name_args;
use super::super::rewrite::{
    rewrite_base_calls_for_wrapper, rewrite_block, rewrite_virtual_calls_for_wrapper,
};
use super::super::util::{attr_str, is_basic, strip_meta_attrs, type_is_int, type_is_bool, type_is_long};
use super::context::GenContext;

/// §5-§7 Wrapper struct + Default/Clone/PartialEq/Debug + impl ObjectVTable for Wrapper
/// + wrapper impl 块。
pub(crate) fn generate(ctx: &GenContext) -> syn::Result<TokenStream2> {
    let struct_ident = &ctx.struct_ident;
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let phantom_field = &ctx.phantom_field;
    let phantom_init = &ctx.phantom_init;
    let binary_name = &ctx.meta.binary_name;

    // ══════════════════════════════════════════════════════════════════════════
    // 5. Wrapper struct + Default + Clone
    // ══════════════════════════════════════════════════════════════════════════

    let wrapper_struct = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident #impl_g #where_c {
            pub(crate) vtable: __Shared<dyn #vtable_trait_ident>,
            pub(crate) any: __Shared<dyn ::std::any::Any>,
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
                any: __Shared<dyn ::std::any::Any>,
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
                    __Shared<dyn ::std::any::Any>> = ::std::option::Option::None;
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

    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                let rc = __Shared::new(<#inner_ident as ::std::default::Default>::default());
                #struct_ident {
                    vtable: __Shared::clone(&rc) as __Shared<dyn #vtable_trait_ident>,
                    any: rc as __Shared<dyn ::std::any::Any>,
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

    // ══════════════════════════════════════════════════════════════════════════
    // 6. impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）
    // ══════════════════════════════════════════════════════════════════════════

    let obj_vtable_for_wrapper = if !binary_name.is_empty() {
        // toString 在 Java 恒为虚方法：wrapper 一律把字符串化经 vtable 分派到运行时类
        // （祖先视图（From<Child> for Ancestor）的 wrapper 由此获得多态 toString——
        // 如 Number 视图转发 Integer 的 toString；未覆盖类落到 __inner 的默认
        // type_name，与既有输出一致。hashCode/equals 同此形态，本就无条件转发）。
        let to_string_fwd: TokenStream2 = quote! {
            fn __obj_str(&self) -> ::std::string::String {
                ObjectVTable::__obj_str(&*self.vtable)
            }
        };
        let hash_code_fwd: TokenStream2 = quote! {
            fn hashCode(&self) -> i32 { ObjectVTable::hashCode(&*self.vtable) }
            fn equals(&self, other: Object) -> Result<bool> {
                ObjectVTable::equals(&*self.vtable, other)
            }
        };
        // 擦除 vtable 导出的祖先槽位（类祖先的 vtable trait 名；vtable 非泛型，
        // 超类链是其 supertrait —— 子类 vtable 可直接上转）
        let ancestor_vtable_idents: Vec<syn::Ident> = ctx.meta.all_superclasses.iter()
            .map(|anc_name| format_ident!("{}__VTable", anc_name))
            .collect();
        // 运行时类视图（A-1 后落在 wrapper 侧：Object 直接持有 wrapper，类型实参只在
        // wrapper 的 impl 上下文可见）：按 binary name 重建本类 / 任一祖先类型的 wrapper。
        // 祖先视图 = 宏生成的 From<Self> for Ancestor（vtable trait upcasting，保持运行时类）。
        // 异常对象以静态类型（如 Throwable）抛出后，catch 需要按运行时类还原为 catch 类型。
        let view_as_arms: Vec<TokenStream2> = std::iter::once(quote! {
            if type_id == #binary_name {
                return ::std::option::Option::Some(
                    ::std::boxed::Box::new(::std::clone::Clone::clone(self)));
            }
        }).chain(ctx.meta.all_superclasses.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            quote! {
                if type_id == <#anc_ident #atag>::BINARY_NAME {
                    let view: #anc_ident #atag = <#anc_ident #atag as ::std::convert::From<Self>>::from(
                        ::std::clone::Clone::clone(self));
                    return ::std::option::Option::Some(::std::boxed::Box::new(view));
                }
            }
        })).collect();
        // 同一组视图的类型驱动形式（`Object::downcast::<T>()`）：slot 为 `Option<本类/祖先 wrapper>` 时写入
        let view_into_arms: Vec<TokenStream2> = std::iter::once(quote! {
            if let ::std::option::Option::Some(s) =
                slot.downcast_mut::<::std::option::Option<Self>>()
            {
                *s = ::std::option::Option::Some(::std::clone::Clone::clone(self));
                return true;
            }
        }).chain(ctx.meta.all_superclasses.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            quote! {
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<#anc_ident #atag>>()
                {
                    *s = ::std::option::Option::Some(
                        <#anc_ident #atag as ::std::convert::From<Self>>::from(
                            ::std::clone::Clone::clone(self)));
                    return true;
                }
            }
        })).chain(ctx.meta.iface_carrier_views.iter().filter_map(|iface_ty| {
            // A-4 批次 6：接口载体臂——JLS 4.10.3 的子类型关系含接口（数组协变与
            // try_checkcast::<载体> 按此判定），祖先臂只覆盖父类链。成员清单由
            // codegen 过滤并给出擦除载体形态（非泛型裸短名 / 泛型 `I<Object, ..>`，
            // 闭包外接口无生成载体类型，不在此列）；填充走 From<Object> 的载体包装
            // （非受检视图——对象身份保持，分派经运行时类 itable）。
            // UFCS 必须显式：接口载体可能自带 Java `static from(..)` 工厂方法，
            // `Iface::from(..)` 路径解析会被固有方法遮蔽（同 interface_gen upcast）。
            let ty = syn::parse_str::<Type>(iface_ty).ok()?;
            Some(quote! {
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<#ty>>()
                {
                    *s = ::std::option::Option::Some(
                        <#ty as ::std::convert::From<Object>>::from(
                            <Object as ::std::convert::From<Self>>::from(
                                ::std::clone::Clone::clone(self))));
                    return true;
                }
            })
        })).collect();
        // Object.clone() 的逐字段浅拷贝：新对象（新标识单元），每个字段新建存储单元，
        // 值按 Java 语义拷贝（基本类型拷贝值，引用类型拷贝引用）。经 wrapper 访问器
        // 完成拷贝（擦除字段的转换在访问器边界发生，与直连字段拷贝等价）。
        let copy_stmts: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .map(|(name, _)| {
                let get = format_ident!("__get_{}", name);
                let set = format_ident!("__set_{}", name);
                quote! { __copy.#set(self.#get()); }
            })
            .collect();
        // Unsafe 实例字段 long 原子协议（ObjectVTable::__unsafe_long_cell）：平铺字段
        // （继承 + 自有）里非擦除 `long` 字段的共享存储单元臂。unsafe__impl 经
        // offset→字段名反查后按名分派；返回的 Rc<Cell<i64>> 与全部 wrapper 视图共享，
        // Unsafe 写入对直接字段读取（__get_xxx）可见（与 JVM 字段内存语义一致）。
        let long_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .filter(|(name, ty)| !ctx.is_erased(name) && type_is_long(ty))
            .map(|(name, _)| {
                let field_str = name.to_string();
                quote! {
                    (::std::option::Option::Some(i), #field_str) =>
                        ::std::option::Option::Some(__Shared::clone(&i.#name)),
                }
            })
            .collect();
        // 同一协议的 int 镜像（`__unsafe_int_cell`）：平铺的非擦除 `int` 字段。
        let int_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .filter(|(name, ty)| !ctx.is_erased(name) && type_is_int(ty))
            .map(|(name, _)| {
                let field_str = name.to_string();
                quote! {
                    (::std::option::Option::Some(i), #field_str) =>
                        ::std::option::Option::Some(__Shared::clone(&i.#name)),
                }
            })
            .collect();
        // 同一协议的 boolean 镜像（`__unsafe_bool_cell`）：平铺的非擦除 `boolean` 字段
        // （VarHandle 字节数组视图的 `be` 字节序位等只读形态消费）。
        let bool_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .filter(|(name, ty)| !ctx.is_erased(name) && type_is_bool(ty))
            .map(|(name, _)| {
                let field_str = name.to_string();
                quote! {
                    (::std::option::Option::Some(i), #field_str) =>
                        ::std::option::Option::Some(__Shared::clone(&i.#name)),
                }
            })
            .collect();
        // Unsafe/VarHandle 实例字段引用原子协议（`__unsafe_ref_get`/`__unsafe_ref_set`）：
        // 平铺字段里非基本的引用字段（含擦除——载体即 `RefCell<Option<Box<Object>>>`）。
        // 臂内直接 cell 访问（i: &__inner），边界转换与 inner 侧同式；静态类臂未命中
        // （any 是运行时子类 inner / 字段不在本类名单）→ 委托 vtable 对象的同名
        // 覆盖应答（与 long/int cell 的委托同型）。
        let ref_get_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .filter(|(name, ty)| ctx.is_erased(name) || !ctx.inherited_is_basic(name, ty))
            .map(|(name, _)| {
                let field_str = name.to_string();
                if ctx.is_erased(name) {
                    quote! {
                        (::std::option::Option::Some(i), #field_str) =>
                            ::std::option::Option::Some(
                                ::std::option::Option::unwrap_or_default(
                                    i.#name.borrow().as_deref().map(::std::clone::Clone::clone))),
                    }
                } else {
                    quote! {
                        (::std::option::Option::Some(i), #field_str) =>
                            ::std::option::Option::Some(
                                ::std::option::Option::unwrap_or_default(
                                    i.#name.borrow().as_deref()
                                        .map(|__b| Object::from(::std::clone::Clone::clone(__b))))),
                    }
                }
            })
            .chain(ctx.fields.iter()
                .filter(|(name, ty)| ctx.is_erased(name) || !is_basic(ty))
                .map(|(name, _)| {
                    let field_str = name.to_string();
                    if ctx.is_erased(name) {
                        quote! {
                            (::std::option::Option::Some(i), #field_str) =>
                                ::std::option::Option::Some(
                                    ::std::option::Option::unwrap_or_default(
                                        i.#name.borrow().as_deref()
                                            .map(::std::clone::Clone::clone))),
                        }
                    } else {
                        quote! {
                            (::std::option::Option::Some(i), #field_str) =>
                                ::std::option::Option::Some(
                                    ::std::option::Option::unwrap_or_default(
                                        i.#name.borrow().as_deref()
                                            .map(|__b| Object::from(
                                                ::std::clone::Clone::clone(__b))))),
                        }
                    }
                }))
            .collect();
        let ref_set_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .filter(|(name, ty)| ctx.is_erased(name) || !ctx.inherited_is_basic(name, ty))
            .map(|(name, ty)| {
                let field_str = name.to_string();
                if ctx.is_erased(name) {
                    quote! {
                        (::std::option::Option::Some(i), #field_str) => {
                            *i.#name.borrow_mut() =
                                ::std::option::Option::Some(::std::boxed::Box::new(v));
                            true
                        }
                    }
                } else {
                    quote! {
                        (::std::option::Option::Some(i), #field_str) => {
                            *i.#name.borrow_mut() = ::std::option::Option::Some(
                                ::std::boxed::Box::new(
                                    <#ty as ::std::convert::From<Object>>::from(v)));
                            true
                        }
                    }
                }
            })
            .chain(ctx.fields.iter()
                .filter(|(name, ty)| ctx.is_erased(name) || !is_basic(ty))
                .map(|(name, ty)| {
                    let field_str = name.to_string();
                    if ctx.is_erased(name) {
                        quote! {
                            (::std::option::Option::Some(i), #field_str) => {
                                *i.#name.borrow_mut() =
                                    ::std::option::Option::Some(
                                        ::std::boxed::Box::new(v));
                                true
                            }
                        }
                    } else {
                        quote! {
                            (::std::option::Option::Some(i), #field_str) => {
                                *i.#name.borrow_mut() = ::std::option::Option::Some(
                                    ::std::boxed::Box::new(
                                        <#ty as ::std::convert::From<Object>>::from(v)));
                                true
                            }
                        }
                    }
                }))
            .collect();
        quote! {
            impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    self.vtable.is_instance_of(type_id)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                fn is_jvm_null(&self) -> bool { self._jvm_null }
                fn __interface(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    ObjectVTable::__interface(__Shared::clone(&self.vtable), slot)
                }
                fn __class_name(&self) -> &'static str { self.vtable.__class_name() }
                fn __identity(&self) -> *const () { self.vtable.__identity() }
                /// 擦除存储导出（A-1）：wrapper 持有的非泛型 `Rc<X__inner>`。
                /// `From<Object> for X<A>` 的擦除路径据此对任意类型实参重建视图。
                fn __erased_inner(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<__Shared<dyn ::std::any::Any>>>()
                    {
                        *s = ::std::option::Option::Some(__Shared::clone(&self.any));
                    }
                }
                /// 擦除 vtable 导出（A-1 部件形态）：按调用方 slot 的（擦除）类 vtable
                /// 类型把自身 vtable 填入——自身槽位直取；祖先类槽位经 supertrait 上转
                /// （类 vtable trait 非泛型，与类型实参无关）。与 `__erased_inner` 配对，
                /// 供 `From<Object> for X<A>` 重建「运行时类是本类或其子类」的任意实例化视图。
                fn __erased_vtable(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<__Shared<dyn #vtable_trait_ident>>>()
                    {
                        *s = ::std::option::Option::Some(__Shared::clone(&self.vtable));
                        return;
                    }
                    #(
                        if let ::std::option::Option::Some(s) =
                            slot.downcast_mut::<::std::option::Option<__Shared<dyn #ancestor_vtable_idents>>>()
                        {
                            *s = ::std::option::Option::Some(
                                __Shared::clone(&self.vtable)
                                    as __Shared<dyn #ancestor_vtable_idents>);
                            return;
                        }
                    )*
                    // 静态类臂未命中 → 委托 vtable 对象（= 运行时类 inner）的擦除查询：
                    // 祖先视图包装（如 Throwable 视图承载 IllegalStateException inner）对
                    // 「运行时类自身/其祖先」槽位的请求由此应答——中间型（Throwable <
                    // catch T < 运行时 R）catch_as 的擦除重建 Path A。vtable trait 链根部
                    // 超 trait 即 ObjectVTable，上转恒可到达 inner 侧的覆盖。
                    ObjectVTable::__erased_vtable(
                        __Shared::clone(&self.vtable)
                            as __Shared<dyn ObjectVTable>,
                        slot,
                    );
                }
                fn __view_as(
                    &self,
                    _any: __Shared<dyn ::std::any::Any>,
                    type_id: &str,
                ) -> ::std::option::Option<::std::boxed::Box<dyn ::std::any::Any>> {
                    #(#view_as_arms)*
                    ::std::option::Option::None
                }
                fn __view_into(
                    &self,
                    _any: __Shared<dyn ::std::any::Any>,
                    slot: &mut dyn ::std::any::Any,
                ) -> bool {
                    #(#view_into_arms)*
                    false
                }
                fn __shallow_copy(&self) -> ::std::option::Option<Object> {
                    // 运行时类优先（C-1）：vtable 即运行时类 inner，其浅拷贝保留子类字段与
                    // 类名（静态基类视图 Point 承载 Deep 对象时得 Deep 副本）；未应答时
                    // 回退按本（静态）类逐字段拷贝
                    if let ::std::option::Option::Some(__o) = ObjectVTable::__shallow_copy(&*self.vtable) {
                        return ::std::option::Option::Some(__o);
                    }
                    let __rc = __Shared::new(<#inner_ident as ::std::default::Default>::default());
                    // 类型标注：vtable 去形参后字面量的字段不再提及本类形参——全部字段
                    // 为具体类型的类（E 无从钉住）会触发 E0283；以 Self 钉住
                    let __copy: Self = #struct_ident {
                        vtable: __Shared::clone(&__rc) as __Shared<dyn #vtable_trait_ident>,
                        any: __rc as __Shared<dyn ::std::any::Any>,
                        _jvm_null: false,
                        #phantom_init
                    };
                    #(#copy_stmts)*
                    ::std::option::Option::Some(Object::from(__copy))
                }
                /// Unsafe 实例字段 long 原子协议：按字段名取共享存储单元（ObjectVTable
                /// 侧默认 None，见 object.rs）。臂覆盖平铺的非擦除 long 字段；静态类臂
                /// 未命中（any 是运行时子类 inner——静态基类视图，如 AQS 视图承载
                /// CountDownLatch$Sync；或字段不在本类名单）→ 委托 vtable 对象（=
                /// 运行时类 inner）的同名覆盖应答（vtable trait 链根部超 trait 即
                /// ObjectVTable，上转分派；inner 平铺持有全部继承字段，直接可答——
                /// 与 `__erased_vtable` 的委托同型）。
                fn __unsafe_long_cell(
                    &self,
                    field: &str,
                ) -> ::std::option::Option<__Shared<__PrimCell<i64>>> {
                    match (self.any.downcast_ref::<#inner_ident>(), field) {
                        #(#long_cell_arms)*
                        _ => ObjectVTable::__unsafe_long_cell(&*self.vtable, field),
                    }
                }
                /// Unsafe 实例字段 int 原子协议：`__unsafe_long_cell` 的 int 镜像
                /// （平铺的非擦除 int 字段臂 + 未命中委托 vtable 对象）。
                fn __unsafe_int_cell(
                    &self,
                    field: &str,
                ) -> ::std::option::Option<__Shared<__PrimCell<i32>>> {
                    match (self.any.downcast_ref::<#inner_ident>(), field) {
                        #(#int_cell_arms)*
                        _ => ObjectVTable::__unsafe_int_cell(&*self.vtable, field),
                    }
                }
                /// 实例字段 boolean 按名协议：`__unsafe_int_cell` 的 boolean 镜像。
                fn __unsafe_bool_cell(
                    &self,
                    field: &str,
                ) -> ::std::option::Option<__Shared<__PrimCell<bool>>> {
                    match (self.any.downcast_ref::<#inner_ident>(), field) {
                        #(#bool_cell_arms)*
                        _ => ObjectVTable::__unsafe_bool_cell(&*self.vtable, field),
                    }
                }
                /// Unsafe/VarHandle 实例字段引用原子协议（读形态）：平铺的引用字段
                /// 臂（含擦除——载体即 `RefCell<Option<Box<Object>>>`）+ 未命中委托
                /// vtable 对象（运行时类 inner 平铺持有全部继承字段，直接可答——
                /// 静态基类视图由此承接，与 long/int cell 的委托同型）。
                fn __unsafe_ref_get(
                    &self,
                    field: &str,
                ) -> ::std::option::Option<Object> {
                    match (self.any.downcast_ref::<#inner_ident>(), field) {
                        #(#ref_get_arms)*
                        _ => ObjectVTable::__unsafe_ref_get(&*self.vtable, field),
                    }
                }
                /// Unsafe/VarHandle 实例字段引用原子协议（写形态）：
                /// `__unsafe_ref_get` 的镜像（命中写入 true + 未命中委托）。
                fn __unsafe_ref_set(&self, field: &str, v: Object) -> bool {
                    match (self.any.downcast_ref::<#inner_ident>(), field) {
                        #(#ref_set_arms)*
                        _ => ObjectVTable::__unsafe_ref_set(&*self.vtable, field, v),
                    }
                }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 7. Impl block on wrapper（字段访问器委托 + 虚方法委托 + 构造器双入口）
    // ══════════════════════════════════════════════════════════════════════════

    let mut wrapper_methods: Vec<TokenStream2> = Vec::new();

    // _init_not_null：构造器完成后调用，将 _jvm_null 标志清零
    wrapper_methods.push(quote! {
        #[doc(hidden)] #[inline]
        pub fn _init_not_null(&mut self) { self._jvm_null = false; }
    });

    // 字段访问器委托（own + 继承字段）。读取处是 GIL 安全点（`gil::safepoint`：自旋等待
    // 他线程写入的循环借此让出）。vtable 访问器签名已 Object 化（A-1 去形参）：
    // 擦除字段的类型化转换（From<Object> / Into<Object>）发生在 wrapper 委托边界 ——
    // 等价 javac 在字段访问处插入的 checkcast。
    let wrapper_delegate_items = |name: &syn::Ident, ty: &Type| -> TokenStream2 {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if ctx.is_erased(name) {
            quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty {
                    __safepoint();
                    <#ty as ::std::convert::From<Object>>::from(self.vtable.#get())
                }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) {
                    self.vtable.#set(::std::convert::Into::<Object>::into(v));
                }
            }
        } else {
            quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { __safepoint(); self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            }
        }
    };
    for (name, ty) in ctx.fields.iter().chain(ctx.meta.superclass_fields.iter()) {
        wrapper_methods.push(wrapper_delegate_items(name, ty));
    }

    // VirtualDefine 方法：wrapper 统一委托到 vtable 以保证多态正确性。
    // VirtualDefine wrapper：
    // - 有方法体且体不是 vtable-safe（含 Clone::clone(this)/Self:: 等 wrapper 专属操作）→
    //   直接在 wrapper 上下文执行方法体（__inner 无法持有 Rc，无法重建 wrapper 返回值）
    // - 其余情况（无方法体/vtable-safe 方法体）→ 通过 vtable dispatch，保证子类覆盖生效
    for f in &ctx.vtable_defines {
        let sig = &f.sig;
        let mname = &sig.ident;
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let vis = &f.vis;

        // NeedsWrapper 方法体（含 Clone::clone(this) 或 this.method() 调用）：
        // 直接放进 wrapper impl（this: &Wrapper）以保证 this 类型正确。
        // 同时：
        // 1. 将 __base(this, ...) 改为 __base(&*this.vtable, ...)
        // 2. 将 this.method(args) 改为 (&*this.vtable).method(args)，
        //    通过 vtable supertrait 链访问继承但未显式覆盖的虚方法。
        if let Some(block) = &f.block {
            if !matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                let mut b = block.clone();
                rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                rewrite_base_calls_for_wrapper(&mut b);
                rewrite_virtual_calls_for_wrapper(&mut b, &ctx.own_method_names, &ctx.vdispatch);
                // 方法体落在隐藏的 `__impl_<method>`（不分派）；公开的同名方法统一经 vtable 分派，
                // 子类覆盖版本对「父类型 wrapper 上的调用」同样生效。
                let mut impl_sig = sig.clone();
                impl_sig.ident = format_ident!("__impl_{}", mname);
                wrapper_methods.push(quote! {
                    #(#keep_attrs)*
                    #[doc(hidden)]
                    pub #impl_sig #b
                });
            }
        }

        // vtable 方法签名已 Object 化（A-1）→ 类型化 wrapper 方法与擦除分派之间在
        // 此边界转换（形参装箱 / 返回值还原）
        let conv_args = erased_call_args(sig, &ctx.type_param_names);
        let call = quote! {
            #vtable_trait_ident::#mname(&*self.vtable, #(#conv_args),*)
        };
        let dispatch = erased_call_ret_conv(sig, &ctx.type_param_names, call);
        let null_check = class_init::null_receiver_check(sig);
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { #null_check #dispatch }
        });
    }

    // VirtualOverride 委托（wrapper 也需要暴露同名方法，转发到 vtable）
    let mut seen_delegators: HashSet<String> = ctx.vtable_defines
        .iter()
        .map(|f| f.sig.ident.to_string())
        .collect();
    for (vtable_class, override_fns) in &ctx.vtable_overrides {
        for f in override_fns {
            let mname_str = f.sig.ident.to_string();
            if seen_delegators.contains(&mname_str) {
                continue;
            }
            seen_delegators.insert(mname_str);
            let sig = &f.sig;
            let mname = &sig.ident;
            let keep_attrs = strip_meta_attrs(&f.attrs);
            let vis = &f.vis;
            // 需要 wrapper 上下文的覆盖体：方法体落在隐藏的 `__impl_<method>`（不分派），
            // vtable impl 与 super 调用的 base 函数都经钩子重建 wrapper 后执行它。
            if let Some(block) = &f.block {
                if !matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                    let mut b = block.clone();
                    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                    rewrite_base_calls_for_wrapper(&mut b);
                    rewrite_virtual_calls_for_wrapper(&mut b, &ctx.own_method_names, &ctx.vdispatch);
                    let mut impl_sig = sig.clone();
                    impl_sig.ident = format_ident!("__impl_{}", mname);
                    wrapper_methods.push(quote! {
                        #(#keep_attrs)*
                        #[doc(hidden)]
                        pub #impl_sig #b
                    });
                }
            }
            // UFCS：用 vtable_class__VTable 消歧义（VirtualOverride 同名方法冲突）；
            // 方法签名已 Object 化 → 边界转换同 VirtualDefine 委托（含 vtable_erasure 名集）。
            // K-6 槽位名解耦：wrapper 名（本类重载态）≠ trait 槽位名（声明者态）时
            // UFCS 目标取 vtable_name 属性给出的 trait 成员名
            let anc_vtable = format_ident!("{}__VTable", vtable_class);
            let slot_name = attr_str(&f.attrs, "vtable_name")
                .map(|t| Ident::new(&t, proc_macro2::Span::call_site()))
                .unwrap_or_else(|| mname.clone());
            let ov_erasure = erasure_set_of(f, &ctx.type_param_names);
            let conv_args = erased_call_args_with(sig, &ctx.type_param_names, &ov_erasure);
            let call = quote! {
                #anc_vtable::#slot_name(&*self.vtable, #(#conv_args),*)
            };
            let dispatch = erased_call_ret_conv_with(sig, &ctx.type_param_names, &ov_erasure, call);
            let null_check = class_init::null_receiver_check(sig);
            wrapper_methods.push(quote! {
                #(#keep_attrs)*
                #[inline]
                #vis #sig { #null_check #dispatch }
            });
        }
    }

    // 继承成员：wrapper 上的同名转发方法（调用点写 `obj.method(args)`，与 Java 一致）。
    // 虚方法经「本类 VTable → 声明该方法的祖先 VTable」的完全限定 UFCS 分派：
    // 既保持多态，又消除同名方法多 supertrait 来源的歧义（E0034）。
    for (f, owner, vtable_owner, interface_owner) in &ctx.inherited {
        let sig = &f.sig;
        let mname = &sig.ident;
        let vis = &f.vis;
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let param_names: Vec<_> = sig.inputs.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(pt) = arg {
                if let syn::Pat::Ident(pi) = &*pt.pat {
                    return Some(pi.ident.clone());
                }
            }
            None
        }).collect();
        let body: TokenStream2 = match vtable_owner {
            Some(vo) => {
                let vo_ty = match syn::parse_str::<Type>(vo) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let (vo_name, _vo_args) = split_type_name_args(&vo_ty);
                let vo_trait = format_ident!("{}__VTable", vo_name);
                // K-6：跨分支重载发散时成员名（接收者态）≠ 槽位名（声明者态）→
                // UFCS 目标取 vtable_name 属性给出的 trait 成员名
                let slot_name = attr_str(&f.attrs, "vtable_name")
                    .map(|t| Ident::new(&t, proc_macro2::Span::call_site()))
                    .unwrap_or_else(|| mname.clone());
                // vtable 去形参（A-1）：两个 trait 均非泛型；被调方法签名已 Object 化 →
                // 形参 / 返回值在边界转换（类型化 wrapper 方法 ↔ 擦除 vtable 分派），
                // owner 类型形参位置（vtable_erasure 名集）一并装箱 / 还原
                let erasure = erasure_set_of(f, &ctx.type_param_names);
                let conv_args = erased_call_args_with(sig, &ctx.type_param_names, &erasure);
                let call = quote! {
                    <dyn #vtable_trait_ident as #vo_trait>::#slot_name(&*self.vtable, #(#conv_args),*)
                };
                erased_call_ret_conv_with(sig, &ctx.type_param_names, &erasure, call)
            }
            None => {
                let owner_ty = match syn::parse_str::<Type>(owner) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                if *interface_owner {
                    // 接口载体持有对象引用（保留运行时类）→ 接口 vtable 分派到具体实现。
                    // 接口侧的方法名（重载改名按接口自身判定）可能与本类视角下的名字不同 → `target`
                    let target = attr_str(&f.attrs, "target")
                        .map(|t| Ident::new(&t, proc_macro2::Span::call_site()))
                        .unwrap_or_else(|| mname.clone());
                    quote! {
                        <#owner_ty as ::std::convert::From<Object>>::from(
                            <Object as ::std::convert::From<Self>>::from(::std::clone::Clone::clone(self)))
                            .#target(#(#param_names),*)
                    }
                } else {
                    quote! {
                        <#owner_ty as ::std::convert::From<Self>>::from(::std::clone::Clone::clone(self))
                            .#mname(#(#param_names),*)
                    }
                }
            }
        };
        let null_check = class_init::null_receiver_check(sig);
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { #null_check #body }
        });
    }

    // Constructor / NonVirtual 方法（保持原 body，走 Rewriter）
    for f in &ctx.non_virtual {
        wrapper_methods.push(expand_non_virtual_fn(f, &ctx.meta.binary_name, &ctx.basic_names, &ctx.ref_names));
    }

    // K-5 构造器链身份：不再有 __new_with_super。对象身份在最外层 `new` 的具体类
    // 构造器内经 `Self::default()` 建立一次；`super(...)` 由 Python 侧发射为
    // `Parent::__init_on(<Parent as From<Self>>::from(Clone::clone(&this)), args)`
    // ——this 以父类视图（vtable 上转 + any 共享部件）传入父类构造器体，putfield
    // 经访问器落在唯一身份的 inner 上，`this.m()` 虚分派命中最终子类的 override
    //（JVM 单一对象模型）。super 前已赋字段（G-11 的 this$0 场景）天然保留，
    // G-11 的重建保留逻辑随重建一起消亡。

    // static 字段存储 + 访问器、类初始化状态机（JVMS §5.5）
    let impl_method_set: HashSet<String> = ctx.meta.impl_methods.iter().cloned().collect();
    let (static_storage, static_accessors) =
        class_init::expand_statics(&ctx.struct_ident, &ctx.statics, &impl_method_set);
    let has_clinit = ctx.fns.iter().any(|f| f.sig.ident == class_init::CLINIT_FN);
    // 自身类型 static 字段（枚举常量形态）→ 初始化完成后登记常量目录
    let constant_register = class_init::constant_directory_registration(
        &ctx.struct_ident, binary_name, &ctx.statics);
    let (init_state, class_init_fn) = class_init::expand_class_init(
        &ctx.struct_ident, &ctx.meta.binary_name, ctx.meta.superclass.as_ref(),
        &ctx.meta.init_interfaces, has_clinit,
        constant_register);

    let wrapper_impl = quote! {
        #(#static_storage)*
        #init_state

        impl #impl_g #struct_ident #ty_g #where_c {
            #(#wrapper_methods)*
            #(#static_accessors)*
            #class_init_fn
        }
    };


    Ok(quote! {
        #wrapper_struct
        #wrapper_from_parts
        #wrapper_default
        #wrapper_clone
        #wrapper_partialeq
        #wrapper_debug
        #obj_vtable_for_wrapper
        #wrapper_impl
    })
}
