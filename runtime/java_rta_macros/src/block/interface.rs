//! 接口展开：与 Java 接口同名的载体类型、接口的擦除 vtable，以及实现类的桥接调用。
//!
//! - `expand_interface`：`Iface__VTable`（擦除签名的 itable）+ 载体 `Iface<E>`（接口引用 +
//!   静态成员命名空间）。
//! - `expand_interface_impl`：`impl Iface for Class` 条目 → `impl Iface__VTable for Class__inner`。
//! - `erased_impl_call` / `erased_wrapper_call`：擦除签名条目体经擦除实例化 wrapper
//!   （`X<Object, ..>`）执行 `__impl_<m>` 的边界转换与 wrapper 重建。

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{GenericParam, Ident, Type};

use super::class_init;
use super::erasure::{
    erase_signature, erase_type, expand_non_virtual_fn, flat_type_tokens,
    mentions_any, objectize_type, param_idents, result_inner_ty, same_type_tokens,
    without_param_mut,
};
use super::parse::{self, ClassMeta, FnItem, InterfaceImpl};
use super::util::{attr_str, strip_meta_attrs};



fn is_instance_decl(f: &FnItem) -> bool {
    matches!(f.sig.inputs.first(), Some(syn::FnArg::Receiver(_)))
}

/// 接口展开：与 Java 接口同名的载体类型 + 接口的擦除 vtable。
///
/// - `Iface__VTable`：接口实例方法的**擦除签名** trait（无类型参数，对象安全）——
///   等价 JVM 的 itable：运行时只认擦除后的接口，不认类型实参。实现类的 `java_class!`
///   块按 `impl Iface for Class` 为其 `__inner` 生成实现。
/// - 载体 `Iface<E>`：持有一个 `Object` 接口引用（与 `Object` 双向互转 +
///   `Deref<Target = Object>`）。实例方法保持 Java 的泛型签名（`next() -> Result<E>`），
///   内部经 `ObjectVTable::__interface` 取得对象的 `Iface__VTable` 视图后分派，
///   类型变量位置的实参 / 返回值在 `Object` 与 `E` 之间转换（等价 javac 插入的 checkcast）。
///   函数式接口的 lambda 实例是 A-5 合成对象（`Iface__Lambda`，sam_objects 生成），
///   实现 `Iface__VTable` 并经 `__interface` 应答——SAM 与 default 方法都走 vtable 分派。
/// - 命名空间语义：接口的 static 方法 / static 字段访问器落在载体的 inherent impl 上，
///   调用点与 Java 同构（`Map::copyOf(m)`）。
///
/// 类型别名（`type Iface = Object`）无法承担命名空间语义：别名上的关联函数解析到 `Object`，
/// 且别名不能携带未使用的类型参数。
pub(crate) fn expand_interface(
    meta: &ClassMeta,
    struct_ident: &Ident,
    gen: &syn::Generics,
    fns: &[FnItem],
    statics: &[parse::StaticItem],
) -> TokenStream2 {
    let (impl_g, ty_g, where_c) = gen.split_for_impl();
    let type_params: Vec<&Ident> = gen.params.iter()
        .filter_map(|p| if let GenericParam::Type(tp) = p { Some(&tp.ident) } else { None })
        .collect();
    let type_param_names: HashSet<String> = type_params.iter().map(|i| i.to_string()).collect();
    let no_fields: HashSet<String> = HashSet::new();
    let static_members: Vec<TokenStream2> = fns.iter()
        .filter(|f| !is_instance_decl(f))
        .map(|f| expand_non_virtual_fn(f, &meta.binary_name, &no_fields, &no_fields))
        .collect();
    let binary_name = &meta.binary_name;
    let vtable_ident = format_ident!("{}__VTable", struct_ident);

    let mut vtable_methods: Vec<TokenStream2> = Vec::new();
    let mut carrier_methods: Vec<TokenStream2> = Vec::new();

    // 继承成员（超接口声明、本接口未重声明）：向上转型为声明接口的载体后调用，
    // 分派仍经声明接口的 vtable —— 每个 `Iface__VTable` 只含本接口自己声明的方法。
    for f in fns.iter().filter(|f| is_instance_decl(f)) {
        let Some(owner) = attr_str(&f.attrs, "inherited_from") else { continue };
        let owner_ty = match syn::parse_str::<Type>(&owner) {
            Ok(t) => t,
            Err(e) => return e.to_compile_error(),
        };
        let sig = without_param_mut(&f.sig);
        let mname = &sig.ident;
        let args = param_idents(&sig);
        let keep_attrs = strip_meta_attrs(&f.attrs);
        carrier_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            pub #sig {
                <#owner_ty as ::std::convert::From<Object>>::from(::std::clone::Clone::clone(&self.__ref))
                    .#mname(#(#args),*)
            }
        });
    }

    let instance_decls: Vec<&FnItem> = fns.iter()
        .filter(|f| is_instance_decl(f) && attr_str(&f.attrs, "inherited_from").is_none())
        .collect();

    for f in &instance_decls {
        let erased = erase_signature(&f.sig, &type_param_names);
        let mname = f.sig.ident.clone();
        let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
        let stub_msg = format!("stub: {}.{}:{}", binary_name, mname, desc);
        vtable_methods.push(quote! {
            #[allow(unused_variables)]
            #erased { panic!(#stub_msg) }
        });

        let args = param_idents(&f.sig);
        // A-5：default 方法体提为载体固有方法 `__default_<m>`——合成对象
        // （`Iface__Lambda`）的 vtable default 条目经它执行默认体而不重入
        // 载体分派（载体 → vtable → 条目 → 载体 …… 会环）；载体的最终回退
        // 同样调用它（单一默认体，两处消费）。JVM lambda 类继承接口 default
        // 的语义由此承载。
        let default_method: Option<proc_macro2::TokenStream> = f.block.as_ref().map(|block| {
            let mut d_sig = without_param_mut(&f.sig);
            d_sig.ident = format_ident!("__default_{}", f.sig.ident);
            quote! {
                #[doc(hidden)]
                pub #d_sig #block
            }
        });
        let default_fallback = if default_method.is_some() {
            let d_ident = format_ident!("__default_{}", f.sig.ident);
            let d_args = param_idents(&f.sig);
            quote! { return self.#d_ident(#(#d_args),*); }
        } else {
            quote! {}
        };
        let default_method = default_method.unwrap_or_default();
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let sig = without_param_mut(&f.sig);
        let missing_msg = format!("AbstractMethodError: {}.{}:{}", binary_name, mname, desc);
        // default 方法体（codegen 以载体为接收者翻译一份落到接口块内）：载体分派的
        // 最终回退 —— vtable 未命中（lambda / 闭包接收者不实现 `Iface__VTable`）
        // 且非 SAM 直调时执行 default 体，对应 JVM 对函数式接口实例调用 default
        // 方法的语义（类覆盖 / 实现类展开体仍经 vtable 优先分派）。
        carrier_methods.push(quote! {
            #default_method
            #(#keep_attrs)*
            pub #sig {
                let mut __vt: ::std::option::Option<__Shared<dyn #vtable_ident>> = None;
                ObjectVTable::__interface(__Shared::clone(&self.__ref.0), &mut __vt);
                if let Some(__vt) = __vt {
                    return Ok(::std::convert::From::from(
                        <dyn #vtable_ident>::#mname(&*__vt #(, ::std::convert::Into::into(#args))*)?));
                }
                #default_fallback
                panic!("{} (receiver: {})", #missing_msg, ObjectVTable::__obj_str(&*self.__ref.0))
            }
        });
    }

    let impl_methods: HashSet<String> = meta.impl_methods.iter().cloned().collect();
    let (static_storage, static_accessors) =
        class_init::expand_statics(struct_ident, statics, &impl_methods);
    let has_clinit = fns.iter().any(|f| f.sig.ident == class_init::CLINIT_FN);
    // 接口初始化不触发父接口初始化（JVMS §5.5）；接口无实例形态，不登记常量目录
    let (init_state, class_init_fn) =
        class_init::expand_class_init(struct_ident, binary_name, None, &[], has_clinit, quote! {});

    quote! {
        #[allow(non_camel_case_types)]
        pub trait #vtable_ident: 'static + __ThreadSafe {
            #(#vtable_methods)*
        }

        #(#static_storage)*
        #init_state

        #[derive(::core::clone::Clone, ::core::default::Default)]
        pub struct #struct_ident #impl_g #where_c {
            __ref: Object,
            __phantom: ( #( ::std::marker::PhantomData<fn() -> #type_params>, )* ),
        }

        impl #impl_g From<Object> for #struct_ident #ty_g #where_c {
            fn from(obj: Object) -> Self {
                Self { __ref: obj, __phantom: ::std::default::Default::default() }
            }
        }

        impl #impl_g From<#struct_ident #ty_g> for Object #where_c {
            fn from(iface: #struct_ident #ty_g) -> Object { iface.__ref }
        }

        // 载体进入类型位置（A-4 批次 3+：形参 / 返回 / 局部 / 字段）后，字段存储层
        // `__inner` 的 `derive(PartialEq, Debug)` 要求载体满足同一约束——与类 Wrapper
        // 同形：身份相等（同一底层对象 = 同一接口视图；null 单例共享指针恒等），
        // Debug 经运行时 toString 桥接。
        impl #impl_g ::std::cmp::PartialEq for #struct_ident #ty_g #where_c {
            fn eq(&self, other: &Self) -> bool {
                self.__ref.0.__identity() == other.__ref.0.__identity()
            }
        }

        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_ident), ObjectVTable::__obj_str(&*self.__ref.0))
            }
        }

        impl #impl_g ::std::ops::Deref for #struct_ident #ty_g #where_c {
            type Target = Object;
            fn deref(&self) -> &Object { &self.__ref }
        }

        impl #impl_g #struct_ident #ty_g #where_c {
            pub const BINARY_NAME: &'static str = #binary_name;

            /// null 探测与类 Wrapper 的固有方法同形（类型位置载体化后，null 检查
            /// 发射面 `x.is_jvm_null()` 对载体与 wrapper 统一）：载体 null 即其底层
            /// Object 引用是 null 单例。
            pub fn is_jvm_null(&self) -> bool {
                ObjectVTable::is_jvm_null(&*self.__ref.0)
            }

            #(#static_members)*
            #(#static_accessors)*
            #class_init_fn

            #(#carrier_methods)*
        }
    }
}

/// `impl Iface for Class { 擦除签名声明 }` → `impl Iface__VTable for Class__inner`。
///
/// 每个方法重建本类 wrapper 后调用其同名（或 `target` 指定的）成员：调用经 wrapper 的
/// vtable 委托保持多态；擦除签名与成员真实签名之间的转换（`Object` ↔ 类型变量 / 具体类）
/// 由 `From` / `Into` 按成员签名推断——等价 javac 桥接方法里的 checkcast 与隐式向上转型。
///
/// __inner 非泛型（A-1）：本 impl 不再携带类的类型形参（擦除签名不含它们，携带反而使
/// E0207/E0283 —— 形参无约束、`__interface` 填充时实例化不可定）。wrapper 以全 `Object`
/// 实参构造：分派经 vtable 是擦除的，任一实例化的行为参数化一致（Object 满足全部形参 bound）。
pub(crate) fn expand_interface_impl(
    ii: &InterfaceImpl,
    struct_ident: &Ident,
    inner_ident: &Ident,
    vtable_trait_ident: &Ident,
    erased_ty_args: &TokenStream2,
    phantom_init: &TokenStream2,
) -> TokenStream2 {
    let iface_vtable = format_ident!("{}__VTable", ii.iface);
    let erased_vt: TokenStream2 = quote! { dyn #vtable_trait_ident };
    let methods: Vec<TokenStream2> = ii.fns.iter().map(|f| {
        let sig = without_param_mut(&f.sig);
        let args = param_idents(&sig);
        let target = attr_str(&f.attrs, "target")
            .map(|t| Ident::new(&t, proc_macro2::Span::call_site()))
            .unwrap_or_else(|| sig.ident.clone());
        // 成员返回同一泛型类的另一实例化（`Optional<Double>` 之于接口的 `Optional<? extends
        // ConstantDesc>`）：Java 侧靠擦除直接通过，Rust 侧经 Object 边界按接口声明类型取回
        let convert = if attr_str(&f.attrs, "result").as_deref() == Some("checkcast") {
            quote! { Ok(::std::convert::From::from(Object::from(__result))) }
        } else {
            quote! { Ok(::std::convert::Into::into(__result)) }
        };
        quote! {
            #sig {
                let __rc = __Shared::new(::std::clone::Clone::clone(self));
                let __wrapper: #struct_ident #erased_ty_args = #struct_ident {
                    vtable: __rc.clone() as __Shared<#erased_vt>,
                    any: __rc as __AnyRef,
                    _jvm_null: false,
                    #phantom_init
                };
                let __result = __wrapper.#target(#(::std::convert::From::from(#args)),*)?;
                #convert
            }
        }
    }).collect();
    quote! {
        impl #iface_vtable for #inner_ident {
            #(#methods)*
        }
    }
}

/// 擦除签名的 vtable 条目（trait default 体 / impl 条目体）经擦除实例化 wrapper
/// （`X<Object, ...>`）执行 `__impl_<m>`。条目签名是擦除形态（提及类型形参处一律
/// `Object`），`__impl_<m>` 在擦除 wrapper 上的形态是 objectize(签名)——两者的差异
/// 只出现在「嵌套提及」（`JArray<E>` / `Foo<E>`）：实参经 `From<Object>` 还原、返回值
/// 经 blanket `From<T: ObjectVTable> for Object` 装箱。
pub(crate) fn erased_impl_call(
    sig: &syn::Signature,
    impl_name: &Ident,
    type_param_names: &HashSet<String>,
    erasure: &HashSet<String>,
) -> TokenStream2 {
    let conv_args: Vec<TokenStream2> = sig.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => {
            let ident = match &*pt.pat {
                syn::Pat::Ident(pi) => pi.ident.clone(),
                _ => return None,
            };
            let mentions = mentions_any(&pt.ty, type_param_names);
            let hits = !erasure.is_empty()
                && erasure.contains(&flat_type_tokens(&pt.ty));
            if mentions || hits {
                let obj_ty = if mentions {
                    objectize_type(&pt.ty, type_param_names)
                } else {
                    (*pt.ty).clone()
                };
                // 命中擦除名集的位置，条目签名一侧已是 Object（erase_signature_with）
                let erased_ty: Type = if hits && !mentions {
                    syn::parse_quote!(Object)
                } else {
                    erase_type(&pt.ty, type_param_names)
                };
                if same_type_tokens(&obj_ty, &erased_ty) {
                    Some(quote! { #ident })
                } else {
                    Some(quote! { <#obj_ty as ::std::convert::From<Object>>::from(#ident) })
                }
            } else {
                Some(quote! { #ident })
            }
        }
        _ => None,
    }).collect();
    let mut call = quote! { __w.#impl_name(#(#conv_args),*) };
    if let syn::ReturnType::Type(_, ty) = &sig.output {
        if let Some(inner) = result_inner_ty(ty) {
            let mentions = mentions_any(inner, type_param_names);
            let hits = !erasure.is_empty()
                && erasure.contains(&flat_type_tokens(inner));
            if mentions || hits {
                let obj_inner = if mentions {
                    objectize_type(inner, type_param_names)
                } else {
                    inner.clone()
                };
                let erased_inner: Type = if hits && !mentions {
                    syn::parse_quote!(Object)
                } else {
                    erase_type(inner, type_param_names)
                };
                if !same_type_tokens(&obj_inner, &erased_inner) {
                    call = quote! {
                        #call.map(|__v| <#erased_inner as ::std::convert::From<#obj_inner>>::from(__v))
                    };
                }
            }
        }
    }
    call
}

/// impl `X__VTable for X__inner` 条目（签名已擦除）的体：从 `self`（&X__inner）构造
/// 擦除实例化 wrapper 后执行 `__impl_<m>`（边界转换见 erased_impl_call）。
pub(crate) fn erased_wrapper_call(
    sig: &syn::Signature,
    impl_name: &Ident,
    struct_ident: &Ident,
    vtable_trait_ident: &Ident,
    erased_ty_args: &TokenStream2,
    phantom_init: &TokenStream2,
    type_param_names: &HashSet<String>,
    erasure: &HashSet<String>,
) -> TokenStream2 {
    let call = erased_impl_call(sig, impl_name, type_param_names, erasure);
    quote! {
        let __rc = __Shared::new(::std::clone::Clone::clone(self));
        let __w: #struct_ident #erased_ty_args = #struct_ident {
            vtable: __rc.clone() as __Shared<dyn #vtable_trait_ident>,
            any: __rc as __AnyRef,
            _jvm_null: false,
            #phantom_init
        };
        #call
    }
}
