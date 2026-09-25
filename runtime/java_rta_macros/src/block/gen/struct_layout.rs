//! Java 字段布局：`__inner` 存储 struct（平铺字段）+ `impl ObjectVTable for __inner`。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Ident;

use super::super::util::{is_basic, type_is_int, type_is_long};
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
        quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<Object>>>> };
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
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 自有字段
    for (name, ty) in ctx.fields.iter() {
        let cell_ty = if ctx.is_erased(name) {
            erased_ref_cell.clone()
        } else if is_basic(ty) {
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 对象标识单元：wrapper 钩子按值克隆 __inner 重建视图时随之共享，Default（新对象）各自新建
    inner_field_tokens.push(quote! { pub(crate) __identity: ::std::rc::Rc<()> });

    let inner_struct = quote! {
        #[doc(hidden)]
        #[derive(Clone, Default, PartialEq, Debug)]
        pub(crate) struct #inner_ident {
            #(#inner_field_tokens,)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 3. impl ObjectVTable for __inner
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name = &ctx.meta.binary_name;
    let check_types: Vec<String> = if ctx.meta.all_supertypes.is_empty() {
        vec![binary_name.clone()]
    } else {
        ctx.meta.all_supertypes.clone()
    };
    let patterns = check_types.iter().map(|s| quote! { #s });

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
    let interface_query: TokenStream2 = if iface_vtable_idents.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __interface(self: ::std::rc::Rc<Self>, slot: &mut dyn ::std::any::Any) {
                #(
                    if let Some(s) = slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn #iface_vtable_idents>>>() {
                        *s = Some(self);
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
    // __inner`），按调用方 slot 的（擦除）类 vtable 类型把自身填入——运行时类自身槽
    // 直取 self；祖先类槽经 supertrait 上转。与上方 `__interface` 的接口查询同型，
    // 意义在于**按运行时类**应答（wrapper 侧的同名方法按静态类生成臂，祖先视图包装
    // 的「降回中间类」查询由此承接——中间型 catch/checkcast 的擦除重建路径）。
    let ancestor_vtable_idents: Vec<Ident> = ctx.meta.all_superclasses.iter()
        .filter(|anc| !anc.is_empty())
        .map(|anc| format_ident!("{}__VTable", anc))
        .collect();
    let erased_vtable_query: TokenStream2 = quote! {
        fn __erased_vtable(self: ::std::rc::Rc<Self>, slot: &mut dyn ::std::any::Any) {
            if let ::std::option::Option::Some(s) =
                slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn #vtable_trait_ident>>>()
            {
                *s = ::std::option::Option::Some(
                    ::std::rc::Rc::clone(&self)
                        as ::std::rc::Rc<dyn #vtable_trait_ident>);
                return;
            }
            #(
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<::std::rc::Rc<dyn #ancestor_vtable_idents>>>()
                {
                    *s = ::std::option::Option::Some(
                        ::std::rc::Rc::clone(&self)
                            as ::std::rc::Rc<dyn #ancestor_vtable_idents>);
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
                    ::std::rc::Rc::clone(&self.#name)),
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
                    ::std::rc::Rc::clone(&self.#name)),
            }
        })
        .collect();
    let inner_long_cell_query: TokenStream2 = if inner_long_cell_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_long_cell(
                &self,
                field: &str,
            ) -> ::std::option::Option<::std::rc::Rc<::std::cell::Cell<i64>>> {
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
            ) -> ::std::option::Option<::std::rc::Rc<::std::cell::Cell<i32>>> {
                match field {
                    #(#inner_int_cell_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };

    // Unsafe/VarHandle 实例字段引用原子协议（运行时类应答）：引用字段（非基本，
    // 含擦除字段——擦除载体本就是 `Rc<RefCell<Option<Box<Object>>>>`）按字段名
    // 直答读/写。与 int/long cell 的关键差异：引用载体类型随声明类型异构
    // （`Box<Object>` / `Box<Completion>`），无统一 cell 类型可导出 → 读值在臂内
    // `Object::from` 上转、写值 `<T as From<Object>>::from` 还原（边界转换与
    // 字段访问器协议一致，null 双形态经 unwrap_or_default/default 归一）。
    // wrapper 侧同名方法按静态类生成臂（downcast 自身 inner，直接 cell 访问）
    // ——静态基类视图（如 Completion 视图承载 UniApply inner）的请求臂不可达，
    // 由 wrapper 未命中后经 vtable 委托到本覆盖应答（与 `__unsafe_long_cell`
    // 的「inner 覆盖 + wrapper 委托」同型）。字段不在名单 → 不生成方法，落
    // object.rs 的 trait 默认（读 None / 写 false，调用方归 stub）。
    let inner_ref_get_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .filter(|(name, ty)| ctx.is_erased(name) || !ctx.inherited_is_basic(name, ty))
        .map(|(name, _)| {
            let field_str = name.to_string();
            if ctx.is_erased(name) {
                quote! {
                    #field_str => ::std::option::Option::Some(
                        ::std::option::Option::unwrap_or_default(
                            self.#name.borrow().as_deref().map(::std::clone::Clone::clone))),
                }
            } else {
                quote! {
                    #field_str => ::std::option::Option::Some(
                        ::std::option::Option::unwrap_or_default(
                            self.#name.borrow().as_deref()
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
                        #field_str => ::std::option::Option::Some(
                            ::std::option::Option::unwrap_or_default(
                                self.#name.borrow().as_deref().map(::std::clone::Clone::clone))),
                    }
                } else {
                    quote! {
                        #field_str => ::std::option::Option::Some(
                            ::std::option::Option::unwrap_or_default(
                                self.#name.borrow().as_deref()
                                    .map(|__b| Object::from(::std::clone::Clone::clone(__b))))),
                    }
                }
            }))
        .collect();
    let inner_ref_set_arms: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .filter(|(name, ty)| ctx.is_erased(name) || !ctx.inherited_is_basic(name, ty))
        .map(|(name, ty)| {
            let field_str = name.to_string();
            if ctx.is_erased(name) {
                quote! {
                    #field_str => {
                        *self.#name.borrow_mut() =
                            ::std::option::Option::Some(::std::boxed::Box::new(v));
                        true
                    }
                }
            } else {
                quote! {
                    #field_str => {
                        *self.#name.borrow_mut() = ::std::option::Option::Some(
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
                        #field_str => {
                            *self.#name.borrow_mut() =
                                ::std::option::Option::Some(::std::boxed::Box::new(v));
                            true
                        }
                    }
                } else {
                    quote! {
                        #field_str => {
                            *self.#name.borrow_mut() = ::std::option::Option::Some(
                                ::std::boxed::Box::new(
                                    <#ty as ::std::convert::From<Object>>::from(v)));
                            true
                        }
                    }
                }
            }))
        .collect();
    let inner_ref_get_query: TokenStream2 = if inner_ref_get_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_ref_get(&self, field: &str) -> ::std::option::Option<Object> {
                match field {
                    #(#inner_ref_get_arms)*
                    _ => ::std::option::Option::None,
                }
            }
        }
    };
    let inner_ref_set_query: TokenStream2 = if inner_ref_set_arms.is_empty() {
        quote! {}
    } else {
        quote! {
            fn __unsafe_ref_set(&self, field: &str, v: Object) -> bool {
                match field {
                    #(#inner_ref_set_arms)*
                    _ => false,
                }
            }
        }
    };

    // Object.clone 的运行时类浅拷贝（C-1）：新 inner（新标识单元），每个字段新建存储
    // 单元、值按 Java 语义拷贝（基本类型 Cell 拷贝值；引用 / 擦除 RefCell 拷贝引用——
    // Box<T> 的 Clone 即 wrapper/Object 的引用克隆），再经本类 __as_Self 钩子包成运行时
    // 类 wrapper。inner 即运行时类（vtable 方法体里的 `this`），类自带 clone 体内的
    // super.clone() 经此得到运行时类副本（子类字段 / 类名完整保留）。
    let shallow_copy_inits: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
        .map(|(name, ty)| (name, !ctx.is_erased(name) && ctx.inherited_is_basic(name, ty)))
        .chain(ctx.fields.iter().map(|(name, ty)| (name, !ctx.is_erased(name) && is_basic(ty))))
        .map(|(name, basic)| if basic {
            quote! { #name: ::std::rc::Rc::new(::std::cell::Cell::new(self.#name.get())), }
        } else {
            quote! { #name: ::std::rc::Rc::new(::std::cell::RefCell::new(self.#name.borrow().clone())), }
        })
        .collect();
    let as_self_hook = &ctx.as_self_hook;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let inner_shallow_copy: TokenStream2 = quote! {
        fn __shallow_copy(&self) -> ::std::option::Option<Object> {
            let __c = #inner_ident {
                #(#shallow_copy_inits)*
                __identity: ::std::rc::Rc::new(()),
            };
            ::std::option::Option::Some(Object::from(#vtable_trait_ident::#as_self_hook(&__c)))
        }
    };

    // A-1 存储层擦除后，inner 的 ObjectVTable impl 只承载「按擦除类」的判定与桥接：
    // 视图重建（__view_as / __view_into）、逐字段浅拷贝（__shallow_copy）与擦除存储
    // 导出（__erased_state，已删除）都移到 wrapper 侧——Object 直接持有 wrapper
    // （blanket From<T: ObjectVTable>），只有 wrapper 的 impl 知道类型实参；
    // inner 的 Rc 可经 __erased_inner 取回（From<Object> 的擦除路径据此重建任意实例化视图）。
    let obj_vtable_for_inner = if !binary_name.is_empty() {
        quote! {
            impl ObjectVTable for #inner_ident {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    matches!(type_id, #(#patterns)|*)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                fn __class_name(&self) -> &'static str { #binary_name }
                fn __identity(&self) -> *const () {
                    ::std::rc::Rc::as_ptr(&self.__identity) as *const ()
                }
                #hash_code_inner_bridge
                #equals_inner_bridge
                #interface_query
                #erased_vtable_query
                #inner_long_cell_query
                #inner_int_cell_query
                #inner_ref_get_query
                #inner_ref_set_query
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
