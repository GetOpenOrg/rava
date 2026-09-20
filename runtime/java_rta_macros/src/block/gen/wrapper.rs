//! Java 类型包装：wrapper struct（vtable + any 两指针 + JVM null 标志）、
//! Default/Clone/PartialEq/Debug、impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）、
//! wrapper impl 块（字段访问器委托 + 虚方法委托 + 构造器 + __new_with_super）。

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
use super::super::util::{attr_str, strip_meta_attrs};
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
            pub(crate) vtable: ::std::rc::Rc<dyn #vtable_trait_ident>,
            pub(crate) any: ::std::rc::Rc<dyn ::std::any::Any>,
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
                vtable: ::std::rc::Rc<dyn #vtable_trait_ident>,
                any: ::std::rc::Rc<dyn ::std::any::Any>,
                is_null: bool,
            ) -> Self {
                #struct_ident { vtable, any, _jvm_null: is_null, #phantom_init }
            }
        }
    };

    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                let rc = ::std::rc::Rc::new(<#inner_ident as ::std::default::Default>::default());
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
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
                    vtable: ::std::rc::Rc::clone(&self.vtable),
                    any: ::std::rc::Rc::clone(&self.any),
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
        let to_string_fwd: TokenStream2 = if ctx.meta.to_string_vtable.is_some() {
            quote! {
                fn __obj_str(&self) -> ::std::string::String {
                    ObjectVTable::__obj_str(&*self.vtable)
                }
            }
        } else {
            quote! {}
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
        quote! {
            impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    self.vtable.is_instance_of(type_id)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                fn is_jvm_null(&self) -> bool { self._jvm_null }
                fn __interface(self: ::std::rc::Rc<Self>, slot: &mut dyn ::std::any::Any) {
                    ObjectVTable::__interface(::std::rc::Rc::clone(&self.vtable), slot)
                }
                fn __class_name(&self) -> &'static str { self.vtable.__class_name() }
                fn __identity(&self) -> *const () { self.vtable.__identity() }
                /// 擦除存储导出（A-1）：wrapper 持有的非泛型 `Rc<X__inner>`。
                /// `From<Object> for X<A>` 的擦除路径据此对任意类型实参重建视图。
                fn __erased_inner(self: ::std::rc::Rc<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn ::std::any::Any>>>()
                    {
                        *s = ::std::option::Option::Some(::std::rc::Rc::clone(&self.any));
                    }
                }
                /// 擦除 vtable 导出（A-1 部件形态）：按调用方 slot 的（擦除）类 vtable
                /// 类型把自身 vtable 填入——自身槽位直取；祖先类槽位经 supertrait 上转
                /// （类 vtable trait 非泛型，与类型实参无关）。与 `__erased_inner` 配对，
                /// 供 `From<Object> for X<A>` 重建「运行时类是本类或其子类」的任意实例化视图。
                fn __erased_vtable(self: ::std::rc::Rc<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn #vtable_trait_ident>>>()
                    {
                        *s = ::std::option::Option::Some(::std::rc::Rc::clone(&self.vtable));
                        return;
                    }
                    #(
                        if let ::std::option::Option::Some(s) =
                            slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn #ancestor_vtable_idents>>>()
                        {
                            *s = ::std::option::Option::Some(
                                ::std::rc::Rc::clone(&self.vtable)
                                    as ::std::rc::Rc<dyn #ancestor_vtable_idents>);
                            return;
                        }
                    )*
                }
                fn __view_as(
                    &self,
                    _any: ::std::rc::Rc<dyn ::std::any::Any>,
                    type_id: &str,
                ) -> ::std::option::Option<::std::boxed::Box<dyn ::std::any::Any>> {
                    #(#view_as_arms)*
                    ::std::option::Option::None
                }
                fn __view_into(
                    &self,
                    _any: ::std::rc::Rc<dyn ::std::any::Any>,
                    slot: &mut dyn ::std::any::Any,
                ) -> bool {
                    #(#view_into_arms)*
                    false
                }
                fn __shallow_copy(&self) -> ::std::option::Option<Object> {
                    let __rc = ::std::rc::Rc::new(<#inner_ident as ::std::default::Default>::default());
                    // 类型标注：vtable 去形参后字面量的字段不再提及本类形参——全部字段
                    // 为具体类型的类（E 无从钉住）会触发 E0283；以 Self 钉住
                    let __copy: Self = #struct_ident {
                        vtable: ::std::rc::Rc::clone(&__rc) as ::std::rc::Rc<dyn #vtable_trait_ident>,
                        any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                        _jvm_null: false,
                        #phantom_init
                    };
                    #(#copy_stmts)*
                    ::std::option::Option::Some(Object::from(__copy))
                }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 7. Impl block on wrapper（字段访问器委托 + 虚方法委托 + 构造器 + __new_with_super）
    // ══════════════════════════════════════════════════════════════════════════

    let mut wrapper_methods: Vec<TokenStream2> = Vec::new();

    // _init_not_null：构造器完成后调用，将 _jvm_null 标志清零
    wrapper_methods.push(quote! {
        #[doc(hidden)] #[inline]
        pub fn _init_not_null(&mut self) { self._jvm_null = false; }
    });

    // 字段访问器委托（own + 继承字段）。vtable 访问器签名已 Object 化（A-1 去形参）：
    // 擦除字段的类型化转换（From<Object> / Into<Object>）发生在 wrapper 委托边界 ——
    // 等价 javac 在字段访问处插入的 checkcast。
    let wrapper_delegate_items = |name: &syn::Ident, ty: &Type| -> TokenStream2 {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if ctx.is_erased(name) {
            quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty {
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
                pub fn #get(&self) -> #ty { self.vtable.#get() }
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
            // 方法签名已 Object 化 → 边界转换同 VirtualDefine 委托（含 vtable_erasure 名集）
            let anc_vtable = format_ident!("{}__VTable", vtable_class);
            let ov_erasure = erasure_set_of(f, &ctx.type_param_names);
            let conv_args = erased_call_args_with(sig, &ctx.type_param_names, &ov_erasure);
            let call = quote! {
                #anc_vtable::#mname(&*self.vtable, #(#conv_args),*)
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
                // vtable 去形参（A-1）：两个 trait 均非泛型；被调方法签名已 Object 化 →
                // 形参 / 返回值在边界转换（类型化 wrapper 方法 ↔ 擦除 vtable 分派），
                // owner 类型形参位置（vtable_erasure 名集）一并装箱 / 还原
                let erasure = erasure_set_of(f, &ctx.type_param_names);
                let conv_args = erased_call_args_with(sig, &ctx.type_param_names, &erasure);
                let call = quote! {
                    <dyn #vtable_trait_ident as #vo_trait>::#mname(&*self.vtable, #(#conv_args),*)
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

    // __new_with_super（有父类时生成）
    let new_with_super: TokenStream2 = if let Some(sup_ty) = &ctx.meta.superclass {
        // 从 parent 的字段访问器拉取 superclass_fields 的值，初始化 __inner。
        // 擦除字段以 Object 存储：访问器值经 Into<Object> 装箱写入。
        let mut field_inits: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &ctx.meta.superclass_fields {
            let get = format_ident!("__get_{}", name);
            if ctx.is_erased(name) {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(
                            ::std::convert::Into::<Object>::into(parent.#get())))
                    )),
                });
            } else if ctx.inherited_is_basic(name, ty) {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::Cell::new(parent.#get())),
                });
            } else {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(parent.#get()))
                    )),
                });
            }
        }
        // 本类自有字段从旧 this 保留（G-11）：javac 21 对内部类构造器的
        // `putfield this$N` 先于 `invokespecial super.<init>`，重建若按
        // Default::default() 会把已赋字段抹掉 → 后续解引用 NPE（TestVar 的
        // TreeMap$EntrySet 实证）。存储形态与本类声明一致。
        for (name, _ty) in ctx.fields {
            let get = format_ident!("__get_{}", name);
            if ctx.is_erased(name) {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(
                            ::std::convert::Into::<Object>::into(old.#get())))
                    )),
                });
            } else if ctx.basic_names.contains(&name.to_string()) {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::Cell::new(old.#get())),
                });
            } else {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(old.#get()))
                    )),
                });
            }
        }
        quote! {
            #[doc(hidden)]
            #[allow(unused_variables)]
            pub fn __new_with_super(parent: #sup_ty, old: Self) -> Self {
                let _ = &old;
                let inner = #inner_ident {
                    #(#field_inits)*
                    ..::std::default::Default::default()
                };
                let rc = ::std::rc::Rc::new(inner);
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                    #phantom_init
                }
            }
        }
    } else {
        quote! {}
    };

    // static 字段存储 + 访问器、类初始化状态机（JVMS §5.5）
    let impl_method_set: HashSet<String> = ctx.meta.impl_methods.iter().cloned().collect();
    let (static_storage, static_accessors) =
        class_init::expand_statics(&ctx.struct_ident, &ctx.statics, &impl_method_set);
    let has_clinit = ctx.fns.iter().any(|f| f.sig.ident == class_init::CLINIT_FN);
    // 自身类型 static 字段（枚举常量形态）→ 初始化完成后登记常量目录
    let constant_register = class_init::constant_directory_registration(
        &ctx.struct_ident, binary_name, &ctx.statics);
    let (init_state, class_init_fn) = class_init::expand_class_init(
        &ctx.struct_ident, &ctx.meta.binary_name, ctx.meta.superclass.as_ref(), has_clinit,
        constant_register);

    let wrapper_impl = quote! {
        #(#static_storage)*
        #init_state

        impl #impl_g #struct_ident #ty_g #where_c {
            #(#wrapper_methods)*
            #(#static_accessors)*
            #class_init_fn
            #new_with_super
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
