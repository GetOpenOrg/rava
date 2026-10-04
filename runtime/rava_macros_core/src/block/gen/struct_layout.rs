//! Java 字段布局：`__inner` 存储 struct（平铺字段）+ `impl ObjectVTable for __inner`。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::super::util::{is_basic, type_is_int, type_is_bool, type_is_long, type_is_word, type_is_dword};
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

    let inner_struct = quote! {
        #[derive(::core::clone::Clone, ::core::default::Default, ::core::cmp::PartialEq, ::core::fmt::Debug)]
        pub(crate) struct #inner_ident {
            #(#inner_field_tokens,)*
        }
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

    // Unsafe 实例字段原子协议（运行时类应答）：inner 平铺持有全部继承字段且存储
    // 即共享单元（Rc<Cell<i64/i32>>），按字段名直答。wrapper 侧的同名方法按静态
    // 类生成臂（downcast 自身 inner）——静态基类视图（如 AQS 视图承载
    // CountDownLatch$Sync inner）的请求臂不可达，由 wrapper 未命中后经 vtable
    // 委托到本覆盖应答（与 `__erased_vtable` 的「inner 覆盖 + wrapper 委托」
    // 同型）。臂只对非擦除的裸 i64/i32 平铺字段生成；字段不在名单 → 不生成方法，
    // 落 object.rs 的 trait 默认 None（调用方归 stub）。
    let inner_long_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .chain(ctx.fields.iter())
        .filter(|(name, ty)| !ctx.is_erased(name) && type_is_long(ty))
        .map(|(name, _)| {
            let field_str = name.to_string();
            quote! {
                #field_str => ::std::option::Option::Some(
                    __Shared::clone(&self.#name)),
            }
        })
        .collect();
    let inner_int_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .chain(ctx.fields.iter())
        .filter(|(name, ty)| !ctx.is_erased(name) && type_is_int(ty))
        .map(|(name, _)| {
            let field_str = name.to_string();
            quote! {
                #field_str => ::std::option::Option::Some(
                    __Shared::clone(&self.#name)),
            }
        })
        .collect();
    let inner_bool_cell_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .chain(ctx.fields.iter())
        .filter(|(name, ty)| !ctx.is_erased(name) && type_is_bool(ty))
        .map(|(name, _)| {
            let field_str = name.to_string();
            quote! {
                #field_str => ::std::option::Option::Some(
                    __Shared::clone(&self.#name)),
            }
        })
        .collect();
    let inner_bool_cell_query: TokenStream2 = if inner_bool_cell_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_bool_cell(
                &self,
                field: &str,
            ) -> ::std::option::Option<__Shared<__PrimCell<bool>>> {
                match field {
                    #(#inner_bool_cell_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };
    // Unsafe 字 / 双字视图（`__unsafe_word`：int、float 与子字字段；`__unsafe_dword`：long 与
    // double 字段）：字段名 → 单元的视图读-改-写（`__PrimCell::__word_update` / `__dword_update`
    // 按单元类型实例化）。
    let view_query = |method: &str, update: &str, w_ty: TokenStream2, pred: fn(&Type) -> bool| -> TokenStream2 {
        let method = format_ident!("{}", method);
        let update = format_ident!("{}", update);
        let arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .filter(|(name, ty)| !ctx.is_erased(name) && pred(ty))
            .map(|(name, _)| {
                let field_str = name.to_string();
                quote! { #field_str => ::std::option::Option::Some(self.#name.#update(op)), }
            })
            .collect();
        if arms.is_empty() {
            return quote! {};
        }
        quote! {
            fn #method(
                &self,
                field: &str,
                op: &mut dyn FnMut(#w_ty) -> ::std::option::Option<#w_ty>,
            ) -> ::std::option::Option<#w_ty> {
                match field {
                    #(#arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };
    let inner_word_query = view_query("__unsafe_word", "__word_update", quote! { i32 }, type_is_word);
    let inner_dword_query = view_query("__unsafe_dword", "__dword_update", quote! { i64 }, type_is_dword);
    let inner_long_cell_query: TokenStream2 = if inner_long_cell_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_long_cell(
                &self,
                field: &str,
            ) -> ::std::option::Option<__Shared<__PrimCell<i64>>> {
                match field {
                    #(#inner_long_cell_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };
    let inner_int_cell_query: TokenStream2 = if inner_int_cell_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_int_cell(
                &self,
                field: &str,
            ) -> ::std::option::Option<__Shared<__PrimCell<i32>>> {
                match field {
                    #(#inner_int_cell_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };

    // Unsafe/VarHandle 实例字段引用原子协议（运行时类应答）：引用字段（非基本，
    // 含擦除字段——擦除载体本就是 `Rc<RefCell<Option<Box<Object>>>>`）按字段名
    // 分派到槽，槽上的读 / 写 / 读-改-写由 runtime 的 `__ref_slot_access::<T>` 承担
    // （按载体类型实例化、跨类共享；边界转换与字段访问器协议一致）。本类只生成
    // 「字段名 → 槽」的单一 match。wrapper 侧同名方法先问静态类 inner、未命中委托
    // vtable 对象——静态基类视图（如 Completion 视图承载 UniApply inner）由此落到
    // 本覆盖应答（与 `__unsafe_long_cell` 的「inner 覆盖 + wrapper 委托」同型）。
    // 字段不在名单 → 不生成方法，落 object.rs 的 trait 默认 None（调用方归 stub）。
    let inner_ref_access_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .filter(|(name, ty)| ctx.is_erased(name) || !ctx.inherited_is_basic(name, ty))
        .chain(ctx.fields.iter()
            .filter(|(name, ty)| ctx.is_erased(name) || !is_basic(ty)))
        .map(|(name, _)| {
            let field_str = name.to_string();
            quote! { #field_str => __ref_slot_access(&*self.#name, op), }
        })
        .collect();
    let inner_ref_access_query: TokenStream2 = if inner_ref_access_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_ref_access(
                &self, field: &str, op: &mut __RefAccess<'_>,
            ) -> ::std::option::Option<Object> {
                match field {
                    #(#inner_ref_access_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };

    // Unsafe 实例字段偏移的 Java 字段身份 → 按名协议的 Rust 字段名（只列二者不同的平铺字段；
    // 未列出的字段两名相同，trait 默认 None 由调用方取 Java 名）
    let inner_field_slot_query: TokenStream2 = if ctx.meta.field_slots.is_empty() {
        quote! {}
    } else {
        let arms = ctx.meta.field_slots.iter().map(|(decl, java, rust)| {
            quote! { (#decl, #java) => ::std::option::Option::Some(#rust), }
        });
        quote! {
            fn __field_slot(&self, decl: &str, name: &str) -> ::std::option::Option<&'static str> {
                match (decl, name) {
                    #(#arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };

    // Object.clone 的运行时类浅拷贝（C-1）：新 inner（新标识单元），每个字段新建存储
    // 单元、值按 Java 语义拷贝（基本类型 Cell 拷贝值；引用 / 擦除 RefCell 拷贝引用——
    // Box<T> 的 Clone 即 wrapper/Object 的引用克隆），新存储直接装入 Object（S7-2b）。
    // inner 即运行时类（vtable 方法体里的 `this`），类自带 clone 体内的
    // super.clone() 经此得到运行时类副本（子类字段 / 类名完整保留）。
    let shallow_copy_inits: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .map(|(name, ty)| (name, !ctx.is_erased(name) && ctx.inherited_is_basic(name, ty)))
        .chain(ctx.fields.iter().map(|(name, ty)| (name, !ctx.is_erased(name) && is_basic(ty))))
        .map(|(name, basic)| if basic {
            quote! { #name: __Shared::new(__PrimCell::new(self.#name.get())), }
        } else {
            quote! { #name: __Shared::new(__RefSlot::new(self.#name.borrow().clone())), }
        })
        .collect();
    let as_self_hook = &ctx.as_self_hook;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let inner_shallow_copy: TokenStream2 = quote! {
        fn __shallow_copy(&self) -> ::std::option::Option<Object> {
            let __c = #inner_ident {
                #(#shallow_copy_inits)*
                __identity: __Shared::new(()),
            };
            ::std::option::Option::Some(Object::__from_shared(__Shared::new(__c)))
        }
    };

    // inner 即运行时类对象：Object 直接持有它（S7-2b），身份 / 类名 / instanceof / 桥接 /
    // 浅拷贝 / 按名字段协议都由本 impl 应答；From<Object> 的擦除路径经 `__erased_vtable`
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
                #inner_long_cell_query
                #inner_int_cell_query
                #inner_bool_cell_query
                #inner_word_query
                #inner_dword_query
                #inner_ref_access_query
                #inner_field_slot_query
                #to_string_inner_bridge
                #inner_shallow_copy
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
