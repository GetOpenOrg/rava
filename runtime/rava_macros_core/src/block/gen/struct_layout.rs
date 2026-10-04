//! Java 字段布局：`__inner` 存储 struct（平铺字段）+ `impl ObjectVTable for __inner`。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::super::util::is_basic;
use super::context::GenContext;

/// Inner struct（平铺字段：superclass_fields + own fields，非泛型——A-1 存储层擦除）
/// 与 impl ObjectVTable for __inner（按擦除类的判定 + toString/hashCode/equals 桥接）。
pub(crate) fn generate(ctx: &GenContext) -> TokenStream2 {
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;

    // ══════════════════════════════════════════════════════════════════════════
    // 2. Inner struct（平铺字段：superclass_fields + own ctx.fields）—— 非泛型（A-1 存储层擦除）
    // ══════════════════════════════════════════════════════════════════════════

    let erased_ref_cell: TokenStream2 =
        quote! { __Shared<__RefSlot<::std::option::Option<::std::boxed::Box<Object>>>> };
    let mut inner_field_tokens: Vec<TokenStream2> = Vec::new();

    // 继承字段（平铺，不再有 _super）
    // 用 Rc<Cell<T>> / Rc<RefCell<...>> 而非裸 Cell/RefCell，保证 NeedsWrapper clone
    // 时共享同一个 Cell，mutations 对原始 inner struct 可见（否则 clone 是值拷贝，
    // __set_xxx 修改的是孤立副本，调用方看不到变化）。
    // 擦除字段以 Object 存储（声明类型提及类型形参；JVM 字段存储按描述符擦除）。
    for (name, ty) in &ctx.meta.superclass_fields {
        let cell_ty = if ctx.is_erased(name) {
            erased_ref_cell.clone()
        } else if ctx.inherited_is_basic(name, ty) {
            quote! { __Shared<__PrimCell<#ty>> }
        } else {
            quote! { __Shared<__RefSlot<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 自有字段
    for (name, ty) in ctx.fields.iter() {
        let cell_ty = if ctx.is_erased(name) {
            erased_ref_cell.clone()
        } else if is_basic(ty) {
            quote! { __Shared<__PrimCell<#ty>> }
        } else {
            quote! { __Shared<__RefSlot<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 对象标识单元：wrapper 钩子按值克隆 __inner 重建视图时随之共享，Default（新对象）各自新建
    inner_field_tokens.push(quote! { pub(crate) __identity: __Shared<()> });

    // `repr(C)` + 每个字段一个 `__Shared` 细指针：第 i 个字段位于基址 + i 个指针宽，描述符的
    // `fields` / `field_base` 据此定位字段（S7-3，`field_desc`）；断言守护该布局
    let field_count = ctx.meta.superclass_fields.len() + ctx.fields.len();
    let inner_struct = quote! {
        #[derive(::core::clone::Clone, ::core::default::Default, ::core::cmp::PartialEq, ::core::fmt::Debug)]
        #[repr(C)]
        pub(crate) struct #inner_ident {
            #(#inner_field_tokens,)*
        }
        const _: () = ::core::assert!(::core::mem::offset_of!(#inner_ident, __identity)
            == #field_count * ::core::mem::size_of::<usize>());
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 3. impl ObjectVTable for __inner
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name = &ctx.meta.binary_name;
    // 运行时类的静态描述符（S7-0）：wrapper 擦除实例化上的固有常量（声明层发射）
    let desc_query: TokenStream2 = if binary_name.is_empty() {
        quote! {}
    } else {
        let struct_ident = &ctx.struct_ident;
        let erased = &ctx.erased_ty_args;
        quote! {
            fn __desc(&self) -> ::std::option::Option<&'static __ClassDesc> {
                ::std::option::Option::Some(<#struct_ident #erased>::__DESC)
            }
        }
    };

    // 继承链上有翻译出（或手写体）的 hashCode()I / equals(Object)Z 时，根类的对应入口桥接到
    // 该虚方法（经所属 vtable 分派，子类覆盖自动生效）——与 toString 同一机制。
    // __inner 非泛型后桥接调用需显式给出祖先 vtable 的类型实参（本类形参全取 Object，
    // 恒被 `impl<P..> Anc__VTable<args(P..)> for X__inner>` 覆盖）。表达式位置的 trait
    // 路径必须用 turbofish（`Anc::<Object>::m`）；`<Anc<Object>>::m` 要求内部是类型。
    // 桥接路径：`Owner__VTable::<Object..>` —— 显式实参消解「impl<P..> VTable<P..> for
    // __inner 覆盖全部实例化」带来的 UFCS 推断歧义（E0283）。owner 是本类自身时取全
    // Object（ancestor_type_args 只含祖先）；owner 是祖先时把本类形参替换为 Object。
    let bridge_path = |owner: &str| -> TokenStream2 {
        let owner_vtable = format_ident!("{}__VTable", owner);
        quote! { #owner_vtable }
    };
    let hash_code_inner_bridge: proc_macro2::TokenStream = match &ctx.meta.hash_code_vtable {
        Some(owner) => {
            let path = bridge_path(owner);
            quote! {
                fn hashCode(&self) -> i32 {
                    match #path::hashCode(self) {
                        Ok(h) => h,
                        Err(e) => panic!("hashCode 抛出异常: {:?}", e),
                    }
                }
            }
        }
        None => quote! {},
    };
    let equals_inner_bridge: proc_macro2::TokenStream = match &ctx.meta.equals_vtable {
        Some(owner) => {
            let path = bridge_path(owner);
            quote! {
                fn equals(&self, other: Object) -> Result<bool> {
                    #path::equals(self, other)
                }
            }
        }
        None => quote! {},
    };

    // 接口视图查询：按调用方 slot 的（擦除）接口类型把自身填入
    let iface_vtable_idents: Vec<Ident> = ctx.iface_impls.iter()
        .map(|ii| format_ident!("{}__VTable", ii.iface))
        .collect();
    // 接口视图指针填入（S7-2c，与下方 `__erased_vtable` 同形）：调用方（`__IfaceRef::new`）
    // 以持有本存储的 Object 为句柄，指针不持有——不再按接口单态化 `__Shared<dyn I__VTable>`。
    let interface_query: TokenStream2 = if iface_vtable_idents.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __interface(&self, slot: &mut dyn ::std::any::Any) {
                #(
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<::std::ptr::NonNull<dyn #iface_vtable_idents>>>()
                    {
                        *s = ::std::option::Option::Some(
                            ::std::ptr::NonNull::from(self as &dyn #iface_vtable_idents));
                        return;
                    }
                )*
            }
        }
    };

    // 继承链上有翻译出的 toString() 时，根类的字符串化入口桥接到该虚方法（经所属 vtable 分派）
    let to_string_inner_bridge: proc_macro2::TokenStream = match &ctx.meta.to_string_vtable {
        Some(owner) => {
            let path = bridge_path(owner);
            quote! {
                fn __to_string(&self) -> Result<::std::string::String> {
                    #path::toString(self).map(|s| ::std::string::ToString::to_string(&s))
                }
                fn __obj_str(&self) -> ::std::string::String {
                    match #path::toString(self) {
                        Ok(s) => ::std::string::ToString::to_string(&s),
                        Err(e) => panic!("toString 抛出异常: {:?}", e),
                    }
                }
            }
        }
        None => quote! {},
    };

    // 擦除 vtable 查询（运行时类覆盖）：inner 即 vtable 对象（`impl *__VTable for
    // __inner`），按调用方 slot 的（擦除）类 vtable 类型把自身的视图指针填入（调用方与
    // 持有本存储的句柄合成 `__Ref`，S7-2）——运行时类自身槽直取 self；祖先类槽经
    // supertrait 上转。与上方 `__interface` 的接口查询同形，
    // 意义在于**按运行时类**应答（wrapper 侧的同名方法按静态类生成臂，祖先视图包装
    // 的「降回中间类」查询由此承接——中间型 catch/checkcast 的擦除重建路径）。
    let ancestor_vtable_idents: Vec<Ident> = ctx.meta.all_superclasses.iter()
        .filter(|anc| !anc.is_empty())
        .map(|anc| format_ident!("{}__VTable", anc))
        .collect();
    let erased_vtable_query: TokenStream2 = quote! {
        fn __erased_vtable(&self, slot: &mut dyn ::std::any::Any) {
            if let ::std::option::Option::Some(s) =
                slot.downcast_mut::<::std::option::Option<::std::ptr::NonNull<dyn #vtable_trait_ident>>>()
            {
                *s = ::std::option::Option::Some(
                    ::std::ptr::NonNull::from(self as &dyn #vtable_trait_ident));
                return;
            }
            #(
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<::std::ptr::NonNull<dyn #ancestor_vtable_idents>>>()
                {
                    *s = ::std::option::Option::Some(
                        ::std::ptr::NonNull::from(self as &dyn #ancestor_vtable_idents));
                    return;
                }
            )*
        }
    };

    let as_self_hook = &ctx.as_self_hook;
    let vtable_trait_ident = &ctx.vtable_trait_ident;

    // inner 即运行时类对象：Object 直接持有它（S7-2b），身份 / 类名 / instanceof / 桥接由本
    // impl 应答（浅拷贝 / 按名字段协议读描述符的 `fields`，S7-3）；From<Object> 的擦除路径经 `__erased_vtable`
    // 重建任意实例化视图（S7-2）。
    // 代理载体（FS-R R4a）：手写层提供 `__vm_proxy_invoke` / `__vm_proxy_implements`
    // 的类——instanceof 另按实例的接口列表应答，接口载体分派回退经其转发。
    let is_proxy_carrier = ctx.meta.impl_methods.iter().any(|m| m == "__vm_proxy_invoke");
    // instanceof / 类名读本类描述符（ObjectVTable 缺省实现，S7-1）；只有代理载体覆盖
    // instanceof：描述符名单之外，再问代理实例运行期实现的接口
    let proxy_instance_check = if is_proxy_carrier {
        quote! {
            fn is_instance_of(&self, type_id: &str) -> bool {
                ::std::option::Option::is_some_and(ObjectVTable::__desc(self), |d| d.is_subtype_name(type_id))
                    || #vtable_trait_ident::#as_self_hook(self).__vm_proxy_implements(type_id)
            }
        }
    } else {
        quote! {}
    };
    let proxy_invoke_hook = if is_proxy_carrier {
        quote! {
            fn __proxy_invoke(&self, iface: &str, name: &str, desc: &str, args: ::std::vec::Vec<Object>)
                -> ::std::option::Option<Result<Object>> {
                ::std::option::Option::Some(#vtable_trait_ident::#as_self_hook(self).__vm_proxy_invoke(iface, name, desc, args))
            }
        }
    } else {
        quote! {}
    };
    let obj_vtable_for_inner = if !binary_name.is_empty() {
        quote! {
            impl ObjectVTable for #inner_ident {
                #proxy_instance_check
                #proxy_invoke_hook
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                #desc_query
                fn __identity(&self) -> *const () {
                    __Shared::as_ptr(&self.__identity) as *const ()
                }
                #hash_code_inner_bridge
                #equals_inner_bridge
                #interface_query
                #erased_vtable_query
                #to_string_inner_bridge
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #inner_struct
        #obj_vtable_for_inner
    }
}
