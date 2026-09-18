//! `java_class!` 块级宏实现 — vtable 两指针架构。
//!
//! 展开产物：
//!   - `ClassName__VTable` trait（虚方法分派接口，含 default impl）
//!   - `ClassName__inner` 存储 struct（平铺字段，无 `_super` 嵌套）
//!   - `impl AncestorVTable for ClassName__inner`（字段访问器 + 覆盖方法）
//!   - `pub struct ClassName { vtable: Rc<dyn ClassName__VTable>, any: Rc<dyn Any> }`
//!   - `impl ObjectVTable for ClassName`（委托到 vtable，R-1 blanket 需要）
//!   - 字段访问器委托 + 虚方法委托 + 构造器（on wrapper）
//!   - `ClassName__methodName_base` 自由函数（super() 调用路由）
//!   - `From<ClassName> for DirectParent`（vtable trait upcasting）
//!   - `From<Object> for ClassName`（downcast 路径）
//!
//! ## virtual_in 属性
//!
//! codegen 在 `#[java_method(virtual_in = "RustClassName")]` 中携带：
//!   - 等于当前类名 → VirtualDefine（新虚方法，进 trait default impl）
//!   - 不等于当前类名 → VirtualOverride（覆盖祖先，进 `impl AncestorVTable for __inner`）
//!   - 缺失 → Constructor（`new`/`new_*` 前缀）或 NonVirtual

mod class_init;
mod classify;
mod generic_sig;
mod parse;
mod rewrite;
mod util;

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{GenericParam, Ident, Type};

use classify::{classify_vtable_body, is_vtable_safe_body, VTableBodyKind};
use generic_sig::rebuild_sig_with_generics;
use parse::{split_type_name_args, ClassInput, ClassMeta, FnItem, InterfaceImpl};
use rewrite::{
    replace_clone_this_in_ok, rewrite_base_calls_for_wrapper, rewrite_block,
    rewrite_virtual_calls_for_wrapper, rewrite_vtable_calls_ufcs_for_base,
};
use util::{attr_str, classify_method, is_basic, strip_meta_attrs, MethodKind};


pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<ClassInput>(input) {
        Ok(v) => expand_inner(v),
        Err(e) => e.to_compile_error(),
    }
}

/// Constructor / NonVirtual / static 方法：保持原 body（走 Rewriter）；无 body → panic stub。
/// 类的 wrapper impl 与接口载体的 impl 共用。
fn expand_non_virtual_fn(
    f: &FnItem,
    binary_name: &str,
    basic_names: &HashSet<String>,
    ref_names: &HashSet<String>,
) -> TokenStream2 {
    let keep_attrs = strip_meta_attrs(&f.attrs);
    let vis = &f.vis;
    let sig = &f.sig;
    match &f.block {
        Some(block) => {
            let mut b = block.clone();
            // static 方法 / 构造器入口是类初始化触发点（JVMS §5.5：invokestatic / new）
            if class_init::is_init_trigger(sig) {
                class_init::inject_init_trigger(&mut b);
            }
            let null_check = class_init::null_receiver_check(sig);
            rewrite_block(&mut b, basic_names, ref_names);
            // 构造器 / 非虚方法同样运行在 wrapper 上下文（this: Wrapper 或 &Wrapper）：
            // super.method() 的 __base(this, ...) 需经 vtable 取得 &__BT: AncestorVTable
            rewrite_base_calls_for_wrapper(&mut b);
            let stmts = &b.stmts;
            quote! {
                #(#keep_attrs)*
                #vis #sig { #null_check #(#stmts)* }
            }
        }
        None => {
            let mname = sig.ident.to_string();
            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
            let msg = if desc.is_empty() {
                format!("stub: {}.{}", binary_name, mname)
            } else {
                format!("stub: {}.{}:{}", binary_name, mname, desc)
            };
            quote! {
                #(#keep_attrs)*
                #vis #sig { panic!(#msg) }
            }
        }
    }
}

/// 方法声明的非 self 形参名（`a: T` → `a`）。
fn param_idents(sig: &syn::Signature) -> Vec<Ident> {
    sig.inputs.iter().filter_map(|arg| match arg {
        syn::FnArg::Typed(pt) => match &*pt.pat {
            syn::Pat::Ident(pi) => Some(pi.ident.clone()),
            _ => None,
        },
        syn::FnArg::Receiver(_) => None,
    }).collect()
}

/// 去掉形参上的 `mut`（声明 → 转发方法签名）。
fn without_param_mut(sig: &syn::Signature) -> syn::Signature {
    let mut out = sig.clone();
    for arg in out.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = arg {
            if let syn::Pat::Ident(pi) = &mut *pt.pat {
                pi.mutability = None;
            }
        }
    }
    out
}

fn tt_mentions(tt: &proc_macro2::TokenTree, names: &HashSet<String>) -> bool {
    match tt {
        proc_macro2::TokenTree::Ident(i) => names.contains(&i.to_string()),
        proc_macro2::TokenTree::Group(g) => g.stream().into_iter().any(|t| tt_mentions(&t, names)),
        _ => false,
    }
}

fn mentions_any(ty: &Type, names: &HashSet<String>) -> bool {
    quote!(#ty).into_iter().any(|tt| tt_mentions(&tt, names))
}

/// 类型擦除（JVM 语义）：提及接口类型变量的类型位置在运行时一律是 `Object` 引用；
/// 其余类型（基本类型、非泛型类、已实参化的类）保持不变。`Result<T>` 只擦除 `T`。
fn erase_type(ty: &Type, type_params: &HashSet<String>) -> Type {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Result" {
                if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                        let erased = erase_type(inner, type_params);
                        return syn::parse_quote!(Result<#erased>);
                    }
                }
            }
        }
    }
    if mentions_any(ty, type_params) {
        syn::parse_quote!(Object)
    } else {
        ty.clone()
    }
}

/// 接口实例方法声明 → 擦除签名（`Iface__VTable` 的方法签名）。
fn erase_signature(sig: &syn::Signature, type_params: &HashSet<String>) -> syn::Signature {
    let mut out = without_param_mut(sig);
    for arg in out.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = arg {
            let erased = erase_type(&pt.ty, type_params);
            *pt.ty = erased;
        }
    }
    if let syn::ReturnType::Type(_, ty) = &mut out.output {
        let erased = erase_type(ty, type_params);
        **ty = erased;
    }
    out
}

fn is_instance_decl(f: &FnItem) -> bool {
    matches!(f.sig.inputs.first(), Some(syn::FnArg::Receiver(_)))
}

fn is_abstract_decl(f: &FnItem) -> bool {
    attr_str(&f.attrs, "modifiers").map_or(false, |m| m.split(' ').any(|w| w == "abstract"))
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
///   函数式接口的 lambda 对象（`Rc<dyn Fn(擦除形参) -> Result<擦除返回>>`）由其唯一抽象方法直接调用。
/// - 命名空间语义：接口的 static 方法 / static 字段访问器落在载体的 inherent impl 上，
///   调用点与 Java 同构（`Map::copyOf(m)`）。
///
/// 类型别名（`type Iface = Object`）无法承担命名空间语义：别名上的关联函数解析到 `Object`，
/// 且别名不能携带未使用的类型参数。
fn expand_interface(
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
    let abstract_count = instance_decls.iter().filter(|f| is_abstract_decl(f)).count();

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
        // 函数式接口的唯一抽象方法：lambda 对象即该方法的实现
        let lambda_call = if is_abstract_decl(f) && abstract_count == 1 {
            let erased_param_tys: Vec<Type> = erased.inputs.iter().filter_map(|a| match a {
                syn::FnArg::Typed(pt) => Some((*pt.ty).clone()),
                _ => None,
            }).collect();
            let erased_ret = match &erased.output {
                syn::ReturnType::Type(_, ty) => quote! { #ty },
                syn::ReturnType::Default => quote! { () },
            };
            quote! {
                if let Some(__f) = self.__ref.0.as_any()
                    .downcast_ref::<::std::rc::Rc<dyn Fn(#(#erased_param_tys),*) -> #erased_ret>>()
                {
                    return Ok(::std::convert::From::from(
                        (__f)(#(::std::convert::Into::into(#args)),*)?));
                }
            }
        } else {
            quote! {}
        };
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let sig = without_param_mut(&f.sig);
        let missing_msg = format!("AbstractMethodError: {}.{}:{}", binary_name, mname, desc);
        carrier_methods.push(quote! {
            #(#keep_attrs)*
            pub #sig {
                let mut __vt: ::std::option::Option<::std::rc::Rc<dyn #vtable_ident>> = None;
                ObjectVTable::__interface(::std::rc::Rc::clone(&self.__ref.0), &mut __vt);
                if let Some(__vt) = __vt {
                    return Ok(::std::convert::From::from(
                        <dyn #vtable_ident>::#mname(&*__vt #(, ::std::convert::Into::into(#args))*)?));
                }
                #lambda_call
                panic!("{} (receiver: {})", #missing_msg, ObjectVTable::__obj_str(&*self.__ref.0))
            }
        });
    }

    let impl_methods: HashSet<String> = meta.impl_methods.iter().cloned().collect();
    let (static_storage, static_accessors) =
        class_init::expand_statics(struct_ident, statics, &impl_methods);
    let has_clinit = fns.iter().any(|f| f.sig.ident == class_init::CLINIT_FN);
    // 接口初始化不触发父接口初始化（JVMS §5.5）
    let (init_state, class_init_fn) =
        class_init::expand_class_init(struct_ident, binary_name, None, has_clinit);

    quote! {
        #[allow(non_camel_case_types)]
        pub trait #vtable_ident: 'static {
            #(#vtable_methods)*
        }

        #(#static_storage)*
        #init_state

        #[derive(Clone, Default)]
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

        impl #impl_g ::std::ops::Deref for #struct_ident #ty_g #where_c {
            type Target = Object;
            fn deref(&self) -> &Object { &self.__ref }
        }

        impl #impl_g #struct_ident #ty_g #where_c {
            pub const BINARY_NAME: &'static str = #binary_name;

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
fn expand_interface_impl(
    ii: &InterfaceImpl,
    struct_ident: &Ident,
    inner_ident: &Ident,
    vtable_trait_ident: &Ident,
    gen: &syn::Generics,
) -> TokenStream2 {
    let (impl_g, ty_g, where_c) = gen.split_for_impl();
    let iface_vtable = format_ident!("{}__VTable", ii.iface);
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
                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                let __wrapper = #struct_ident {
                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                };
                let __result = __wrapper.#target(#(::std::convert::From::from(#args)),*)?;
                #convert
            }
        }
    }).collect();
    quote! {
        impl #impl_g #iface_vtable for #inner_ident #ty_g #where_c {
            #(#methods)*
        }
    }
}

fn expand_inner(input: ClassInput) -> TokenStream2 {
    let ClassInput { attrs, struct_ident, generics, fields, fns, iface_impls, statics } = input;

    let meta = match ClassMeta::from_attrs(&attrs) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error(),
    };

    // ── 泛型参数补齐 Clone + Default + 'static + From<Object> + Into<Object> ──────────────────────────────
    let mut gen = generics.clone();
    for param in &mut gen.params {
        if let GenericParam::Type(tp) = param {
            let mut has_clone = false;
            let mut has_default = false;
            let mut has_static = false;
            let mut has_from_object = false;
            let mut has_into_object = false;
            for b in &tp.bounds {
                match b {
                    syn::TypeParamBound::Trait(t) => {
                        if let Some(s) = t.path.segments.last() {
                            if s.ident == "Clone" {
                                has_clone = true;
                            } else if s.ident == "Default" {
                                has_default = true;
                            } else if s.ident == "From" {
                                has_from_object = true;
                            } else if s.ident == "Into" {
                                has_into_object = true;
                            }
                        }
                    }
                    syn::TypeParamBound::Lifetime(l) => {
                        if l.ident == "static" {
                            has_static = true;
                        }
                    }
                    _ => {}
                }
            }
            if !has_clone {
                tp.bounds.push(syn::parse_quote!(Clone));
            }
            if !has_default {
                tp.bounds.push(syn::parse_quote!(Default));
            }
            if !has_static {
                tp.bounds.push(syn::parse_quote!('static));
            }
            // Java 类型实参恒为引用类型：与 Object 双向可转（装箱 / checkcast）。
            // 擦除后的接口 vtable、桥接方法在类型变量位置依赖这组转换。
            if !has_from_object {
                tp.bounds.push(syn::parse_quote!(::std::convert::From<Object>));
            }
            if !has_into_object {
                tp.bounds.push(syn::parse_quote!(::std::convert::Into<Object>));
            }
        }
    }
    let (impl_g, ty_g, where_c) = gen.split_for_impl();

    // ── 接口：同名载体类型（接口引用 + 静态成员）────────────────────────────
    if meta.is_interface {
        return expand_interface(&meta, &struct_ident, &gen, &fns, &statics);
    }

    let self_name = struct_ident.to_string();
    let inner_ident = format_ident!("{}__inner", struct_ident);
    let vtable_trait_ident = format_ident!("{}__VTable", struct_ident);
    // wrapper 重建钩子：`Class__VTable::__as_Class(&self) -> Class`。
    // 每个（子）类的 __inner 为全部祖先 VTable 实现该钩子，使祖先方法体能在
    // 「祖先 wrapper 包裹实际 __inner」的上下文中执行（trait default / super 调用）。
    let as_self_hook = format_ident!("__as_{}", struct_ident);

    // 提取 superclass 的泛型参数（如 Enum<Object> → <Object>），供 vtable 继承使用
    // 注意：AngleBracketedGenericArguments 自带 <> 括号，quote! { #ab } 即为 <Object>
    let superclass_vtable_args: TokenStream2 = meta.superclass.as_ref().map_or(
        quote! {},
        |sup_ty| {
            if let syn::Type::Path(tp) = sup_ty {
                if let Some(seg) = tp.path.segments.last() {
                    if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                        if !ab.args.is_empty() {
                            return quote! { #ab };
                        }
                    }
                }
            }
            quote! {}
        },
    );

    // ── 字段集合 ─────────────────────────────────────────────────────────────
    let mut basic_names: HashSet<String> = HashSet::new();
    let mut ref_names: HashSet<String> = HashSet::new();
    // 继承字段的存储形态由声明它的祖先决定（见 superclass_reference_fields）
    let inherited_is_basic = |name: &syn::Ident, ty: &syn::Type| -> bool {
        is_basic(ty) && !meta.superclass_reference_fields.contains(&name.to_string())
    };
    for (name, ty) in fields.iter() {
        if is_basic(ty) {
            basic_names.insert(name.to_string());
        } else {
            ref_names.insert(name.to_string());
        }
    }
    for (name, ty) in meta.superclass_fields.iter() {
        if inherited_is_basic(name, ty) {
            basic_names.insert(name.to_string());
        } else {
            ref_names.insert(name.to_string());
        }
    }

    // ── 方法分类 ─────────────────────────────────────────────────────────────
    // vtable_defines:  VirtualDefine 方法
    // vtable_overrides: vtable_class → Vec<method>
    // non_virtual:     Constructor / NonVirtual 方法
    let mut vtable_defines: Vec<&FnItem> = Vec::new();
    let mut vtable_overrides: HashMap<String, Vec<&FnItem>> = HashMap::new();
    let mut non_virtual: Vec<&FnItem> = Vec::new();
    // inherited:       继承成员声明（祖先声明、本类未覆盖）→ wrapper 上的转发方法
    let mut inherited: Vec<(&FnItem, String, Option<String>, bool)> = Vec::new();

    for f in &fns {
        match classify_method(&f.attrs, &f.sig, &self_name) {
            MethodKind::Inherited { owner, vtable_owner, interface_owner } =>
                inherited.push((f, owner, vtable_owner, interface_owner)),
            MethodKind::VirtualDefine => vtable_defines.push(f),
            MethodKind::VirtualOverride { vtable_class } => {
                vtable_overrides.entry(vtable_class).or_default().push(f);
            }
            MethodKind::Constructor | MethodKind::NonVirtual => non_virtual.push(f),
        }
    }

    // ── PhantomData 检测 ─────────────────────────────────────────────────────
    let mut used_words: HashSet<String> = HashSet::new();
    let mut type_texts: Vec<String> = Vec::new();
    for (_, ty) in fields.iter().chain(meta.superclass_fields.iter()) {
        type_texts.push(quote!(#ty).to_string());
    }
    for text in &type_texts {
        let mut cur = String::new();
        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                cur.push(ch);
            } else if !cur.is_empty() {
                used_words.insert(std::mem::take(&mut cur));
            }
        }
        if !cur.is_empty() {
            used_words.insert(cur);
        }
    }
    let mut phantom_fields: Vec<TokenStream2> = Vec::new();
    for param in &gen.params {
        if let GenericParam::Type(tp) = param {
            if !used_words.contains(&tp.ident.to_string()) {
                let p = &tp.ident;
                phantom_fields.push(quote! { #p });
            }
        }
    }

    // 类级类型参数名（用于 generic_signature 泛型重建）
    let class_type_params: Vec<String> = gen.params.iter().filter_map(|p| {
        if let GenericParam::Type(tp) = p { Some(tp.ident.to_string()) } else { None }
    }).collect();

    // ══════════════════════════════════════════════════════════════════════════
    // 1. VTable trait
    // ══════════════════════════════════════════════════════════════════════════

    // supertrait：有父类 → 父类 __VTable（携带泛型参数）；无父类 → ObjectVTable
    let vtable_supertrait: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        // 提取超类的基础类型名（去掉泛型参数），如 Enum<Object> → Enum
        let base_name = if let Type::Path(tp) = sup_ty {
            tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
        } else {
            quote!(#sup_ty).to_string()
        };
        let sup_vtable = format_ident!("{}__VTable", base_name);
        quote! { #sup_vtable #superclass_vtable_args }
    } else {
        quote! { ObjectVTable }
    };

    // 字段 accessor 抽象方法（只有 own fields，不含继承字段）
    let mut vtable_abstract_methods: Vec<TokenStream2> = Vec::new();
    for (name, ty) in &fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> #ty;
                fn #set(&self, v: #ty);
            });
        } else {
            let borm = format_ident!("__borrow_mut_{}", name);
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> #ty;
                fn #set(&self, v: #ty);
                fn #borm(&self) -> ::std::cell::RefMut<'_, #ty>;
            });
        }
    }

    // VirtualDefine 的 default impl：
    // - 有方法体 → default 委托 ClassName__method_base 自由函数（base 函数含实际体或 stub）
    //   subclass 若未 override，走 base 函数路径（Safe → 实际实现；NeedsWrapper/Skip → stub panic）
    // - 无方法体（abstract）→ default 生成 stub（子类必须覆盖，未覆盖则运行时命中）
    // 注：调用 base 函数时用 turbofish ::<ClassTypeParams..., Self> 避免 E0282 类型推断失败
    let class_ty_idents: Vec<syn::Ident> = gen.params.iter()
        .filter_map(|p| if let syn::GenericParam::Type(tp) = p { Some(tp.ident.clone()) } else { None })
        .collect();
    let mut vtable_default_methods: Vec<TokenStream2> = Vec::new();
    for f in &vtable_defines {
        let mname = &f.sig.ident;
        let fn_base_name = format_ident!("{}__{}_base", self_name, mname);
        let non_self_params_for_default: Vec<_> = f.sig.inputs.iter()
            .filter(|a| matches!(a, syn::FnArg::Typed(_)))
            .collect();
        let param_names_for_default: Vec<syn::Ident> = non_self_params_for_default.iter()
            .filter_map(|a| {
                if let syn::FnArg::Typed(pt) = a {
                    if let syn::Pat::Ident(pi) = &*pt.pat { Some(pi.ident.clone()) }
                    else { None }
                } else { None }
            })
            .collect();
        let mname_str = f.sig.ident.to_string();
        let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
        let binary = &meta.binary_name;
        // generic_signature 重建：将 Object 参数/返回替换为类型变量（K, V 等）
        let effective_sig = attr_str(&f.attrs, "generic_signature")
            .and_then(|gs| rebuild_sig_with_generics(&f.sig, &gs, &class_type_params))
            .unwrap_or_else(|| f.sig.clone());
        if let Some(block) = &f.block {
            if matches!(classify_vtable_body(block), VTableBodyKind::Safe) {
                // vtable-safe 方法体 → default 委托 base 函数（体可直接在 &Self 上运行）
                // turbofish 传类型参数（避免 E0282）：<ClassTypeParams..., Self>
                vtable_default_methods.push(quote! {
                    #effective_sig { #fn_base_name::<#(#class_ty_idents,)* Self>(self, #(#param_names_for_default),*) }
                });
            } else {
                // 方法体需要 wrapper 上下文（this 传参 / 非虚方法调用 / Self::）→
                // 经钩子重建声明类 wrapper，执行 wrapper 上的方法体 `__impl_<method>`
                let impl_name = format_ident!("__impl_{}", mname);
                vtable_default_methods.push(quote! {
                    #effective_sig {
                        let __w = self.#as_self_hook();
                        __w.#impl_name(#(#param_names_for_default),*)
                    }
                });
            }
        } else if attr_str(&f.attrs, "body").as_deref() == Some("handwritten") {
            // 方法体由共置 `_impl.rs` 手写为 wrapper 上的 `__impl_<method>` → 经钩子重建 wrapper 后执行
            let impl_name = format_ident!("__impl_{}", mname);
            vtable_default_methods.push(quote! {
                #effective_sig {
                    let __w = self.#as_self_hook();
                    __w.#impl_name(#(#param_names_for_default),*)
                }
            });
        } else {
            // 无方法体（abstract）→ stub，子类必须覆盖
            let msg = format!("stub: {}.{}:{}", binary, mname_str, desc);
            vtable_default_methods.push(quote! {
                #effective_sig { panic!(#msg) }
            });
        }
    }

    let vtable_trait = quote! {
        #[allow(non_camel_case_types)]
        pub trait #vtable_trait_ident #impl_g #where_c: #vtable_supertrait {
            #[doc(hidden)]
            fn #as_self_hook(&self) -> #struct_ident #ty_g;
            #(#vtable_abstract_methods)*
            #(#vtable_default_methods)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 2. Inner struct（平铺字段：superclass_fields + own fields）
    // ══════════════════════════════════════════════════════════════════════════

    let mut inner_field_tokens: Vec<TokenStream2> = Vec::new();

    // 继承字段（平铺，不再有 _super）
    // 用 Rc<Cell<T>> / Rc<RefCell<...>> 而非裸 Cell/RefCell，保证 NeedsWrapper clone
    // 时共享同一个 Cell，mutations 对原始 inner struct 可见（否则 clone 是值拷贝，
    // __set_xxx 修改的是孤立副本，调用方看不到变化）。
    for (name, ty) in &meta.superclass_fields {
        let cell_ty = if inherited_is_basic(name, ty) {
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 自有字段
    for (name, ty) in &fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 对象标识单元：wrapper 钩子按值克隆 __inner 重建视图时随之共享，Default（新对象）各自新建
    inner_field_tokens.push(quote! { pub(crate) __identity: ::std::rc::Rc<()> });

    if !phantom_fields.is_empty() {
        inner_field_tokens.push(quote! {
            pub(crate) __phantom: ( #( ::std::marker::PhantomData<fn() -> #phantom_fields>, )* )
        });
    }

    let inner_struct = quote! {
        #[doc(hidden)]
        #[derive(Clone, Default, PartialEq, Debug)]
        pub(crate) struct #inner_ident #impl_g #where_c {
            #(#inner_field_tokens,)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 3. impl ObjectVTable for __inner
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name = &meta.binary_name;
    let check_types: Vec<String> = if meta.all_supertypes.is_empty() {
        vec![binary_name.clone()]
    } else {
        meta.all_supertypes.clone()
    };
    let patterns = check_types.iter().map(|s| quote! { #s });

    // 继承链上有翻译出（或手写体）的 hashCode()I / equals(Object)Z 时，根类的对应入口桥接到
    // 该虚方法（经所属 vtable 分派，子类覆盖自动生效）——与 toString 同一机制
    let hash_code_inner_bridge: proc_macro2::TokenStream = match &meta.hash_code_vtable {
        Some(owner) => {
            let owner_vtable = format_ident!("{}__VTable", owner);
            quote! {
                fn hashCode(&self) -> i32 {
                    match #owner_vtable::hashCode(self) {
                        Ok(h) => h,
                        Err(e) => panic!("hashCode 抛出异常: {:?}", e),
                    }
                }
            }
        }
        None => quote! {},
    };
    let equals_inner_bridge: proc_macro2::TokenStream = match &meta.equals_vtable {
        Some(owner) => {
            let owner_vtable = format_ident!("{}__VTable", owner);
            quote! {
                fn equals(&self, other: Object) -> Result<bool> {
                    #owner_vtable::equals(self, other)
                }
            }
        }
        None => quote! {},
    };

    // 接口视图查询：按调用方 slot 的（擦除）接口类型把自身填入
    let iface_vtable_idents: Vec<Ident> = iface_impls.iter()
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
    let to_string_inner_bridge: proc_macro2::TokenStream = match &meta.to_string_vtable {
        Some(owner) => {
            let owner_vtable = format_ident!("{}__VTable", owner);
            quote! {
                fn __obj_str(&self) -> ::std::string::String {
                    match #owner_vtable::toString(self) {
                        Ok(s) => ::std::string::ToString::to_string(&s),
                        Err(e) => panic!("toString 抛出异常: {:?}", e),
                    }
                }
            }
        }
        None => quote! {},
    };

    // 运行时类视图：按运行时类（__inner）重建本类 / 任一祖先类型的 wrapper。
    // 异常对象以静态类型（如 Throwable）抛出后，catch 需要按运行时类还原为 catch 类型。
    let ancestor_views: Vec<TokenStream2> = meta.all_superclasses.iter().map(|anc_name| {
        let anc_ident = format_ident!("{}", anc_name);
        let anc_vtable = format_ident!("{}__VTable", anc_name);
        let atag = meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
        quote! {
            if type_id == <#anc_ident #atag>::BINARY_NAME {
                let view: #anc_ident #atag = #anc_ident::__from_parts(
                    ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #anc_vtable #atag>,
                    rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    false,
                );
                return ::std::option::Option::Some(::std::boxed::Box::new(view));
            }
        }
    }).collect();

    // 同一组视图的类型驱动形式（`Object::downcast::<T>()`）：slot 为 `Option<祖先 wrapper>` 时写入
    let ancestor_slot_views: Vec<TokenStream2> = meta.all_superclasses.iter().map(|anc_name| {
        let anc_ident = format_ident!("{}", anc_name);
        let anc_vtable = format_ident!("{}__VTable", anc_name);
        let atag = meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
        quote! {
            if let Some(s) = slot.downcast_mut::<::std::option::Option<#anc_ident #atag>>() {
                *s = ::std::option::Option::Some(#anc_ident::__from_parts(
                    ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #anc_vtable #atag>,
                    rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    false,
                ));
                return true;
            }
        }
    }).collect();

    // Object.clone() 的逐字段浅拷贝：每个字段新建存储单元，值按 Java 语义拷贝
    // （基本类型拷贝值，引用类型拷贝引用）。
    let copy_field_inits: Vec<TokenStream2> = meta.superclass_fields.iter()
        .map(|(n, t)| (n, inherited_is_basic(n, t)))
        .chain(fields.iter().map(|(n, t)| (n, is_basic(t))))
        .map(|(name, basic)| {
            if basic {
                quote! { #name: ::std::rc::Rc::new(::std::cell::Cell::new(self.#name.get())) }
            } else {
                quote! { #name: ::std::rc::Rc::new(::std::cell::RefCell::new(self.#name.borrow().clone())) }
            }
        })
        .collect();

    // 不可变状态的泛型类：导出擦除后的字段值（连同对象标识单元），供另一类型实例化重建视图
    let erased_state_fn: TokenStream2 = if meta.immutable_state {
        let erased_values: Vec<TokenStream2> = meta.superclass_fields.iter()
            .map(|(n, t)| (n, inherited_is_basic(n, t)))
            .chain(fields.iter().map(|(n, t)| (n, is_basic(t))))
            .map(|(name, basic)| {
                if basic {
                    quote! { ::std::convert::Into::<Object>::into(self.#name.get()) }
                } else {
                    quote! {
                        ::std::convert::Into::<Object>::into(
                            self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default())
                    }
                }
            })
            .collect();
        quote! {
            fn __erased_state(&self) -> ::std::option::Option<(::std::rc::Rc<()>, ::std::vec::Vec<Object>)> {
                ::std::option::Option::Some((
                    ::std::rc::Rc::clone(&self.__identity),
                    vec![#(#erased_values),*],
                ))
            }
        }
    } else {
        quote! {}
    };

    let obj_vtable_for_inner = if !binary_name.is_empty() {
        quote! {
            impl #impl_g ObjectVTable for #inner_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    matches!(type_id, #(#patterns)|*)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                fn __class_name(&self) -> &'static str { #binary_name }
                fn __identity(&self) -> *const () {
                    ::std::rc::Rc::as_ptr(&self.__identity) as *const ()
                }
                fn __view_as(
                    &self,
                    any: ::std::rc::Rc<dyn ::std::any::Any>,
                    type_id: &str,
                ) -> ::std::option::Option<::std::boxed::Box<dyn ::std::any::Any>> {
                    let rc = any.downcast::<#inner_ident #ty_g>().ok()?;
                    if type_id == #binary_name {
                        let view: #struct_ident #ty_g = #struct_ident {
                            vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                            any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                            _jvm_null: false,
                        };
                        return ::std::option::Option::Some(::std::boxed::Box::new(view));
                    }
                    #(#ancestor_views)*
                    ::std::option::Option::None
                }
                fn __view_into(
                    &self,
                    any: ::std::rc::Rc<dyn ::std::any::Any>,
                    slot: &mut dyn ::std::any::Any,
                ) -> bool {
                    let rc = match any.downcast::<#inner_ident #ty_g>() {
                        Ok(rc) => rc,
                        Err(_) => return false,
                    };
                    if let Some(s) = slot.downcast_mut::<::std::option::Option<#struct_ident #ty_g>>() {
                        *s = ::std::option::Option::Some(#struct_ident {
                            vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                            any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                            _jvm_null: false,
                        });
                        return true;
                    }
                    #(#ancestor_slot_views)*
                    false
                }
                fn __shallow_copy(&self) -> ::std::option::Option<Object> {
                    let rc = ::std::rc::Rc::new(#inner_ident {
                        #(#copy_field_inits,)*
                        ..::std::default::Default::default()
                    });
                    let copy: #struct_ident #ty_g = #struct_ident {
                        vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                        any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                        _jvm_null: false,
                    };
                    ::std::option::Option::Some(Object::from(copy))
                }
                #hash_code_inner_bridge
                #equals_inner_bridge
                #erased_state_fn
                #interface_query
                #to_string_inner_bridge
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 4. impl vtable traits for __inner
    // ══════════════════════════════════════════════════════════════════════════
    //
    // 策略：
    // a) 无父类：impl Self__VTable for __inner（own field accessors）
    // b) 有父类：
    //    - 找出"顶层祖先" vtable（all_supertypes 中第一个非 Object 非 self 用户类）
    //      → 放 superclass_fields 的所有 accessor + VirtualOverride 方法
    //    - 其余祖先 vtable：空 impl（满足 trait 层次要求）
    //    - Self__VTable：impl（own field accessors）
    //
    // 注意：对无父类类，own_fields accessor 放 Self__VTable impl；
    //       对有父类类，顶层祖先的 vtable impl 接管 ALL 继承字段 accessor。

    let mut vtable_impls: Vec<TokenStream2> = Vec::new();

    // 当前类自有方法名（VirtualDefine + VirtualOverride + NonVirtual）
    // 在 NeedsWrapper 路径中，不在此集合的 this.method() 调用均为继承虚方法，需通过 vtable 访问。
    // 共置 `_impl.rs` 的手写方法同样是 wrapper 上的自有 inherent 方法（impl_methods）。
    let own_method_names: HashSet<String> = fns.iter()
        .map(|f| f.sig.ident.to_string())
        .chain(meta.impl_methods.iter().cloned())
        .collect();

    if meta.superclass.is_none() {
        // ── 无父类：impl Self__VTable for __inner ────────────────────────────
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &fields {
            let get = format_ident!("__get_{}", name);
            let set = format_ident!("__set_{}", name);
            if is_basic(ty) {
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty { self.#name.get() }
                    fn #set(&self, v: #ty) { self.#name.set(v); }
                });
            } else {
                let borm = format_ident!("__borrow_mut_{}", name);
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty {
                        self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                    }
                    fn #set(&self, v: #ty) {
                        *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                    }
                    fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                        ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                            opt.as_deref_mut().expect("field not initialized")
                        })
                    }
                });
            }
        }
        // VirtualDefine 方法体：Safe → 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &vtable_defines {
            if let Some(block) = &f.block {
                if matches!(classify_vtable_body(block), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &basic_names, &ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #ty_g {
                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                }
            }
        });

        vtable_impls.push(quote! {
            impl #impl_g #vtable_trait_ident #ty_g for #inner_ident #ty_g #where_c {
                #(#own_accessor_impls)*
            }
        });
    } else {
        // ── 有父类：顶层祖先 vtable impl + 其余空 impl + Self__VTable impl ──

        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = meta.all_superclasses.clone();

        // 构建字段名 → (Ident, Type) 的查找表（superclass_fields 平铺列表）
        let sc_fields_map: HashMap<String, (Ident, Type)> = meta
            .superclass_fields
            .iter()
            .map(|(n, t)| (n.to_string(), (n.clone(), t.clone())))
            .collect();

        for anc_name in ancestors.iter() {
            let anc_vtable_ident = format_ident!("{}__VTable", anc_name);

            let mut items: Vec<TokenStream2> = Vec::new();

            // 将此祖先自己声明的字段 accessor 放进此 vtable impl
            let anc_own_field_names: Vec<String> =
                if let Some(names) = meta.ancestor_fields_layout.get(anc_name) {
                    names.clone()
                } else if meta.ancestor_fields_layout.is_empty() {
                    // 兜底：无 ancestor_fields_layout 时（旧属性），所有字段放最深祖先（首位）
                    if anc_name == ancestors.first().map(|s| s.as_str()).unwrap_or("") {
                        meta.superclass_fields.iter().map(|(n, _)| n.to_string()).collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

            for field_name in &anc_own_field_names {
                if let Some((name, ty)) = sc_fields_map.get(field_name) {
                    let get = format_ident!("__get_{}", name);
                    let set = format_ident!("__set_{}", name);
                    if inherited_is_basic(name, ty) {
                        items.push(quote! {
                            fn #get(&self) -> #ty { self.#name.get() }
                            fn #set(&self, v: #ty) { self.#name.set(v); }
                        });
                    } else {
                        let borm = format_ident!("__borrow_mut_{}", name);
                        items.push(quote! {
                            fn #get(&self) -> #ty {
                                self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                            }
                            fn #set(&self, v: #ty) {
                                *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                            }
                            fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                                ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                                    opt.as_deref_mut().expect("field not initialized")
                                })
                            }
                        });
                    }
                }
            }

            // VirtualOverride 方法：virtual_in == anc_name
            if let Some(override_fns) = vtable_overrides.get(anc_name) {
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &basic_names, &ref_names);
                            if is_vtable_safe_body(block) {
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #sig #b
                                });
                            } else {
                                // 方法体需要 wrapper 上下文（this 传参 / 非虚方法调用 / Self::）：
                                // 方法体只落在 wrapper 的 `__impl_<method>` 上（见 wrapper 方法生成），
                                // 此处经钩子重建本类 wrapper 后执行——与 VirtualDefine 同一路径，
                                // super 调用的 base 函数也复用它（不分派，精确命中本类实现）。
                                let impl_name = format_ident!("__impl_{}", sig.ident);
                                let args = param_idents(sig);
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #sig {
                                        let __w = <Self as #vtable_trait_ident #ty_g>::#as_self_hook(self);
                                        __w.#impl_name(#(#args),*)
                                    }
                                });
                            }
                        }
                        None if attr_str(&f.attrs, "body").as_deref() == Some("handwritten") => {
                            let impl_name = format_ident!("__impl_{}", sig.ident);
                            let args = param_idents(sig);
                            items.push(quote! {
                                #(#keep_attrs)*
                                #sig {
                                    let __w = <Self as #vtable_trait_ident #ty_g>::#as_self_hook(self);
                                    __w.#impl_name(#(#args),*)
                                }
                            });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                            let bin = &meta.binary_name;
                            let msg = format!("stub: {}.{}:{}", bin, mname, desc);
                            items.push(quote! {
                                #sig { panic!(#msg) }
                            });
                        }
                    }
                }
            }

            // 祖先 vtable 的类型实参：逐个祖先取 all_superclasses 中携带的实参
            // 例：ReferencePipeline<P_IN, P_OUT> 实现 AbstractPipeline__VTable<P_IN, P_OUT, Object>
            //     与 PipelineHelper__VTable<P_OUT>（元数、顺序各不相同）
            let anc_vtable_args: TokenStream2 =
                meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();

            // 祖先 wrapper 重建钩子（经 __from_parts：祖先可能在另一个 crate，字段不可见）
            let anc_ident = format_ident!("{}", anc_name);
            let anc_hook = format_ident!("__as_{}", anc_name);
            items.push(quote! {
                fn #anc_hook(&self) -> #anc_ident #anc_vtable_args {
                    let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                    #anc_ident::__from_parts(
                        __rc.clone() as ::std::rc::Rc<dyn #anc_vtable_ident #anc_vtable_args>,
                        __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                        false,
                    )
                }
            });

            vtable_impls.push(quote! {
                impl #impl_g #anc_vtable_ident #anc_vtable_args for #inner_ident #ty_g #where_c {
                    #(#items)*
                }
            });
        }

        // Self__VTable impl（own fields 的 accessor）
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &fields {
            let get = format_ident!("__get_{}", name);
            let set = format_ident!("__set_{}", name);
            if is_basic(ty) {
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty { self.#name.get() }
                    fn #set(&self, v: #ty) { self.#name.set(v); }
                });
            } else {
                let borm = format_ident!("__borrow_mut_{}", name);
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty {
                        self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                    }
                    fn #set(&self, v: #ty) {
                        *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                    }
                    fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                        ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                            opt.as_deref_mut().expect("field not initialized")
                        })
                    }
                });
            }
        }
        // VirtualDefine 方法体：Safe → 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &vtable_defines {
            if let Some(block) = &f.block {
                if matches!(classify_vtable_body(block), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &basic_names, &ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #ty_g {
                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                }
            }
        });

        vtable_impls.push(quote! {
            impl #impl_g #vtable_trait_ident #ty_g for #inner_ident #ty_g #where_c {
                #(#own_accessor_impls)*
            }
        });

        // 处理未被 ancestors 列表覆盖的 VirtualOverride（vtable_class 不在祖先中）
        // 这种情况理论上不应出现，但做兜底生成
        for (vtable_class, override_fns) in &vtable_overrides {
            if !ancestors.contains(vtable_class) {
                let anc_vtable_ident = format_ident!("{}__VTable", vtable_class);
                let mut items: Vec<TokenStream2> = Vec::new();
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &basic_names, &ref_names);
                            items.push(quote! { #(#keep_attrs)* #sig #b });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let msg = format!("stub: {}.{}", meta.binary_name, mname);
                            items.push(quote! { #sig { panic!(#msg) } });
                        }
                    }
                }
                vtable_impls.push(quote! {
                    impl #impl_g #anc_vtable_ident #ty_g for #inner_ident #ty_g #where_c {
                        #(#items)*
                    }
                });
            }
        }
    }

    // ══════════════════════════════════════════════════════════════════════════
    // 5. Wrapper struct + Default + Clone
    // ══════════════════════════════════════════════════════════════════════════

    let wrapper_struct = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident #impl_g #where_c {
            pub(crate) vtable: ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
            pub(crate) any: ::std::rc::Rc<dyn ::std::any::Any>,
            /// JVM null 标志：Default::default() = true（null），构造后调用 _init_not_null() = false
            pub _jvm_null: bool,
        }
    };

    // 跨 crate 子类（用户类继承 JDK 类）的向上转换 / 运行时类视图需要按部件重建祖先 wrapper；
    // 字段保持 crate 私有，经此构造入口完成。
    let wrapper_from_parts = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            #[doc(hidden)]
            pub fn __from_parts(
                vtable: ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                any: ::std::rc::Rc<dyn ::std::any::Any>,
                is_null: bool,
            ) -> Self {
                #struct_ident { vtable, any, _jvm_null: is_null }
            }
        }
    };

    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                let rc = ::std::rc::Rc::new(<#inner_ident #ty_g as ::std::default::Default>::default());
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: true,
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
        let to_string_fwd: TokenStream2 = if meta.to_string_vtable.is_some() {
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
                fn __erased_state(&self) -> ::std::option::Option<(::std::rc::Rc<()>, ::std::vec::Vec<Object>)> {
                    self.vtable.__erased_state()
                }
                fn __view_as(
                    &self,
                    _any: ::std::rc::Rc<dyn ::std::any::Any>,
                    type_id: &str,
                ) -> ::std::option::Option<::std::boxed::Box<dyn ::std::any::Any>> {
                    self.vtable.__view_as(::std::rc::Rc::clone(&self.any), type_id)
                }
                fn __view_into(
                    &self,
                    _any: ::std::rc::Rc<dyn ::std::any::Any>,
                    slot: &mut dyn ::std::any::Any,
                ) -> bool {
                    self.vtable.__view_into(::std::rc::Rc::clone(&self.any), slot)
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

    // 字段访问器委托（own fields）
    for (name, ty) in &fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        }
    }

    // 字段访问器委托（superclass_fields，继承字段）
    for (name, ty) in &meta.superclass_fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if inherited_is_basic(name, ty) {
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        }
    }

    // VirtualDefine 方法：wrapper 统一委托到 vtable 以保证多态正确性。
    // VirtualDefine wrapper：
    // - 有方法体且体不是 vtable-safe（含 Clone::clone(this)/Self:: 等 wrapper 专属操作）→
    //   直接在 wrapper 上下文执行方法体（__inner 无法持有 Rc，无法重建 wrapper 返回值）
    // - 其余情况（无方法体/vtable-safe 方法体）→ 通过 vtable dispatch，保证子类覆盖生效
    for f in &vtable_defines {
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
            if !matches!(classify_vtable_body(block), VTableBodyKind::Safe) {
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                rewrite_base_calls_for_wrapper(&mut b);
                rewrite_virtual_calls_for_wrapper(&mut b, &own_method_names);
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

        let param_names: Vec<_> = sig.inputs.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(pt) = arg {
                if let syn::Pat::Ident(pi) = &*pt.pat {
                    return Some(pi.ident.clone());
                }
            }
            None
        }).collect();
        let null_check = class_init::null_receiver_check(sig);
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { #null_check #vtable_trait_ident::#mname(&*self.vtable, #(#param_names),*) }
        });
    }

    // VirtualOverride 委托（wrapper 也需要暴露同名方法，转发到 vtable）
    let mut seen_delegators: HashSet<String> = vtable_defines
        .iter()
        .map(|f| f.sig.ident.to_string())
        .collect();
    for (vtable_class, override_fns) in &vtable_overrides {
        for f in override_fns {
            let mname_str = f.sig.ident.to_string();
            if seen_delegators.contains(&mname_str) {
                continue;
            }
            seen_delegators.insert(mname_str);
            let sig = &f.sig;
            let mname = &sig.ident;
            let param_names: Vec<_> = sig.inputs.iter().filter_map(|arg| {
                if let syn::FnArg::Typed(pt) = arg {
                    if let syn::Pat::Ident(pi) = &*pt.pat {
                        return Some(pi.ident.clone());
                    }
                }
                None
            }).collect();
            let keep_attrs = strip_meta_attrs(&f.attrs);
            let vis = &f.vis;
            // 需要 wrapper 上下文的覆盖体：方法体落在隐藏的 `__impl_<method>`（不分派），
            // vtable impl 与 super 调用的 base 函数都经钩子重建 wrapper 后执行它。
            if let Some(block) = &f.block {
                if !is_vtable_safe_body(block) {
                    let mut b = block.clone();
                    rewrite_block(&mut b, &basic_names, &ref_names);
                    rewrite_base_calls_for_wrapper(&mut b);
                    rewrite_virtual_calls_for_wrapper(&mut b, &own_method_names);
                    let mut impl_sig = sig.clone();
                    impl_sig.ident = format_ident!("__impl_{}", mname);
                    wrapper_methods.push(quote! {
                        #(#keep_attrs)*
                        #[doc(hidden)]
                        pub #impl_sig #b
                    });
                }
            }
            // UFCS：用 vtable_class__VTable 消歧义（VirtualOverride 同名方法冲突）
            let anc_vtable = format_ident!("{}__VTable", vtable_class);
            let null_check = class_init::null_receiver_check(sig);
            wrapper_methods.push(quote! {
                #(#keep_attrs)*
                #[inline]
                #vis #sig { #null_check #anc_vtable::#mname(&*self.vtable, #(#param_names),*) }
            });
        }
    }

    // 继承成员：wrapper 上的同名转发方法（调用点写 `obj.method(args)`，与 Java 一致）。
    // 虚方法经「本类 VTable → 声明该方法的祖先 VTable」的完全限定 UFCS 分派：
    // 既保持多态，又消除同名方法多 supertrait 来源的歧义（E0034）。
    for (f, owner, vtable_owner, interface_owner) in &inherited {
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
                    Err(e) => return e.to_compile_error(),
                };
                let (vo_name, vo_args) = split_type_name_args(&vo_ty);
                let vo_trait = format_ident!("{}__VTable", vo_name);
                quote! {
                    <dyn #vtable_trait_ident #ty_g as #vo_trait #vo_args>::#mname(&*self.vtable, #(#param_names),*)
                }
            }
            None => {
                let owner_ty = match syn::parse_str::<Type>(owner) {
                    Ok(t) => t,
                    Err(e) => return e.to_compile_error(),
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
    for f in &non_virtual {
        wrapper_methods.push(expand_non_virtual_fn(f, &meta.binary_name, &basic_names, &ref_names));
    }

    // __new_with_super（有父类时生成）
    let new_with_super: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        // 从 parent 的字段访问器拉取 superclass_fields 的值，初始化 __inner
        let mut field_inits: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &meta.superclass_fields {
            let get = format_ident!("__get_{}", name);
            if inherited_is_basic(name, ty) {
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
        quote! {
            #[doc(hidden)]
            pub fn __new_with_super(parent: #sup_ty) -> Self {
                let inner = #inner_ident {
                    #(#field_inits)*
                    ..::std::default::Default::default()
                };
                let rc = ::std::rc::Rc::new(inner);
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                }
            }
        }
    } else {
        quote! {}
    };

    // static 字段存储 + 访问器、类初始化状态机（JVMS §5.5）
    let impl_method_set: HashSet<String> = meta.impl_methods.iter().cloned().collect();
    let (static_storage, static_accessors) =
        class_init::expand_statics(&struct_ident, &statics, &impl_method_set);
    let has_clinit = fns.iter().any(|f| f.sig.ident == class_init::CLINIT_FN);
    let (init_state, class_init_fn) = class_init::expand_class_init(
        &struct_ident, &meta.binary_name, meta.superclass.as_ref(), has_clinit);

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

    // ══════════════════════════════════════════════════════════════════════════
    // 8. BINARY_NAME 常量
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name_impl: TokenStream2 = if !binary_name.is_empty() {
        quote! {
            impl #impl_g #struct_ident #ty_g #where_c {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
            // vtable 上下文（impl XxxVTable for __inner）中的方法体里 Self = __inner，
            // Self::BINARY_NAME 必须同样可解析
            impl #impl_g #inner_ident #ty_g #where_c {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 9. From<Object> for ClassName
    // ══════════════════════════════════════════════════════════════════════════

    let obj = quote! { Object };
    // 同一泛型类的另一类型实例化（Java 的 unchecked cast：`(Optional<T>) EMPTY`）：Rust 侧两个
    // 单态化互不相同。状态不可变的类按擦除字段值重建本实例化的存储并共享对象标识单元，
    // 与原对象不可区分（字段构造后不变，`==` 按标识单元判定）。
    let reinstantiate: TokenStream2 = if meta.immutable_state {
        let rebuilt_fields: Vec<TokenStream2> = meta.superclass_fields.iter()
            .map(|(n, t)| (n, t, inherited_is_basic(n, t)))
            .chain(fields.iter().map(|(n, t)| (n, t, is_basic(t))))
            .map(|(name, ty, basic)| {
                if basic {
                    quote! {
                        #name: ::std::rc::Rc::new(::std::cell::Cell::new(
                            <#ty as ::std::convert::From<Object>>::from(__values.next().expect("erased state"))))
                    }
                } else {
                    quote! {
                        #name: ::std::rc::Rc::new(::std::cell::RefCell::new(::std::option::Option::Some(
                            ::std::boxed::Box::new(
                                <#ty as ::std::convert::From<Object>>::from(__values.next().expect("erased state"))))))
                    }
                }
            })
            .collect();
        quote! {
            if let ::std::option::Option::Some(same) = obj.try_checkcast::<Self>() {
                return same;
            }
            if obj.0.__class_name() == #binary_name {
                if let ::std::option::Option::Some((__id, __fields)) = obj.0.__erased_state() {
                    let mut __values = __fields.into_iter();
                    let rc = ::std::rc::Rc::new(#inner_ident {
                        #(#rebuilt_fields,)*
                        __identity: __id,
                        ..::std::default::Default::default()
                    });
                    return #struct_ident {
                        vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                        any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                        _jvm_null: false,
                    };
                }
            }
        }
    } else {
        quote! {}
    };
    let from_object_impl = quote! {
        impl #impl_g From<#obj> for #struct_ident #ty_g #where_c {
            // Java checkcast 语义：运行时类是本类或其子类均成立（子类对象按运行时类重建本类视图）
            // null 通过任何 checkcast（JVMS §6.5 checkcast），得到本类的 null 引用
            fn from(obj: #obj) -> Self {
                if obj.0.is_jvm_null() { return Self::default(); }
                #reinstantiate
                obj.checkcast::<Self>(#binary_name)
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

    let from_child_for_parent: TokenStream2 = if meta.superclass.is_some() {
        // 为每个祖先（排除 Object 和自身）生成 From<Self> for Ancestor
        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = meta.all_superclasses.clone();
        let impls: Vec<TokenStream2> = ancestors.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let anc_vtable = format_ident!("{}__VTable", anc_name);
            let atag = meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            quote! {
                impl #impl_g From<#struct_ident #ty_g> for #anc_ident #atag #where_c {
                    fn from(child: #struct_ident #ty_g) -> #anc_ident #atag {
                        // 结构体字面量不能用 Type<E> {...} 语法（被解析为比较链），
                        // 省略泛型参数由返回类型推导
                        #anc_ident::__from_parts(
                            child.vtable as ::std::rc::Rc<dyn #anc_vtable #atag>,
                            child.any,
                            child._jvm_null,
                        )
                    }
                }
            }
        }).collect();
        quote! { #(#impls)* }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 11. 自由函数 ClassName__methodName_base（供 invokespecial super() 调用）
    // ══════════════════════════════════════════════════════════════════════════

    let mut base_fns: Vec<TokenStream2> = Vec::new();

    // base 函数用的合并泛型：类泛型 + __BT: ?Sized（无 vtable trait 约束，因为全是 panic stub）
    // ?Sized 允许传入 &dyn VTable（fat pointer），不要求 __BT 实现 Sized
    let mut base_gen = gen.clone();
    base_gen.params.push(syn::parse_quote!(__BT: ?Sized));
    let (base_impl_g, _, _) = base_gen.split_for_impl();

    // VirtualDefine 方法生成 base 函数
    // 策略（两阶段）：
    //   1. 先对 body 做 replace_clone_this_in_ok（Ok(Clone::clone(this)) → Ok(Default::default())）
    //   2. 检查替换后 body 在 this: &__BT 上下文是否安全：
    //      - 仍有 bare Clone::clone(this)（传参用途）→ 不安全 → panic stub
    //      - 仍有 this.method() 且 method 不在 vtable_define_names（NonVirtual/native）→ 不安全 → panic stub
    //      - 否则（VirtualDefine vtable 方法调用 + accessor 调用）→ 安全，使用实际 body
    // 注：VirtualDefine 方法（appendNull / ensureCapacityInternal 等）在 &__BT: VTable 上下文可直接调用

    // 本类 VirtualDefine 方法名集合，供下面 base 函数安全判断
    let vtable_define_names: HashSet<String> = vtable_defines.iter()
        .map(|f| f.sig.ident.to_string())
        .collect();

    for f in &vtable_defines {
        if let Some(block) = &f.block {
            let sig = &f.sig;
            let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
            let non_self_params: Vec<_> = sig.inputs.iter()
                .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                .collect();
            let ret = &sig.output;
            let mname_str = sig.ident.to_string();
            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
            let binary = &meta.binary_name;

            let mut b = block.clone();
            rewrite_block(&mut b, &basic_names, &ref_names);
            // 去掉首句 "let this = self;"（base 函数参数直接就是 this）
            if let Some(first) = b.stmts.first() {
                let fs = quote!(#first).to_string();
                if fs.contains("this") && fs.contains("self") {
                    b.stmts.remove(0);
                }
            }
            // 替换 Ok(Clone::clone(this)) → Ok(Default::default())
            replace_clone_this_in_ok(&mut b);

            // 判断替换后 body 是否对 &__BT 上下文安全
            let bs = quote!(#b).to_string();
            // 折叠空白（proc_macro2 在 proc macro 上下文中保留原始换行/缩进）
            let bs_flat: String = bs.split_whitespace().collect::<Vec<_>>().join(" ");
            let has_bare_clone_this = bs_flat.contains("Clone :: clone (this)")
                || bs_flat.contains("Clone :: clone(this)")
                || bs_flat.contains("Clone::clone (this)")
                || bs_flat.contains("Clone::clone(this)");
            // 检查 this.method() 中是否有非 vtable / 非 accessor 方法
            let has_non_vtable_call = {
                let mut found = false;
                let parts: Vec<&str> = bs_flat.split("this .").chain(bs_flat.split("this.")).skip(1).collect();
                for part in parts {
                    let trimmed = part.trim_start();
                    let mname: String = trimmed.chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !mname.is_empty() && !mname.starts_with("__") {
                        let rest = &trimmed[mname.len()..];
                        if rest.trim_start().starts_with('(') {
                            // 只有不在 vtable_define_names 里的才是 non-vtable 调用
                            if !vtable_define_names.contains(&mname) {
                                found = true;
                                break;
                            }
                        }
                    }
                }
                found
            };

            let has_self_ref = bs_flat.contains("Self ::") || bs_flat.contains("Self::");
            if !matches!(classify_vtable_body(block), VTableBodyKind::Safe) {
                // 方法体需要 wrapper 上下文 → 经钩子重建本类 wrapper，执行 `__impl_<method>`
                let _ = (&binary, &mname_str, &desc, has_bare_clone_this, has_non_vtable_call, has_self_ref);
                let impl_name = format_ident!("__impl_{}", sig.ident);
                let base_param_names: Vec<syn::Ident> = non_self_params.iter()
                    .filter_map(|a| {
                        if let syn::FnArg::Typed(pt) = a {
                            if let syn::Pat::Ident(pi) = &*pt.pat { Some(pi.ident.clone()) } else { None }
                        } else { None }
                    })
                    .collect();
                let mut body_gen = gen.clone();
                body_gen.params.push(syn::parse_quote!(__BT));
                body_gen.make_where_clause().predicates.push(
                    syn::parse_quote!(__BT: #vtable_trait_ident #ty_g + ?Sized)
                );
                if let Some(method_where) = &sig.generics.where_clause {
                    body_gen.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
                }
                let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #body_impl_g (this: &__BT #(, #non_self_params)*) #ret #body_where_c {
                        let __w = #vtable_trait_ident::#as_self_hook(this);
                        __w.#impl_name(#(#base_param_names),*)
                    }
                });
            } else if has_bare_clone_this || has_non_vtable_call || has_self_ref {
                // vtable-safe 分类下不会出现；保留 stub 以精确报告
                let msg = format!("stub: super {}.{}:{}", binary, mname_str, desc);
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #base_impl_g (this: &__BT #(, #non_self_params)*) #ret {
                        panic!(#msg)
                    }
                });
            } else {
                // body 安全（只有 vtable 方法调用 + accessor），可在 &(impl VTable + ?Sized) 运行
                // 将 this.vtable_method(args) 改为 VTable::vtable_method(this, args) UFCS，
                // 避免 __BT: VTableA + VTableB 时同名方法产生 E0034 歧义。
                rewrite_vtable_calls_ufcs_for_base(&mut b, &vtable_define_names, &vtable_trait_ident);
                let mut body_gen = gen.clone();
                body_gen.params.push(syn::parse_quote!(__BT));
                body_gen.make_where_clause().predicates.push(
                    syn::parse_quote!(__BT: #vtable_trait_ident #ty_g + ?Sized)
                );
                // 方法自身的 where 子句（类型变量上界约束等）：方法体依赖它，base 函数同样声明
                if let Some(method_where) = &sig.generics.where_clause {
                    body_gen.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
                }
                let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #body_impl_g (this: &__BT #(, #non_self_params)*) #ret #body_where_c {
                        #b
                    }
                });
            }
        }
    }

    // VirtualOverride 方法生成 base 函数（`super.m()` 的精确目标，不分派）。
    // 约束统一为本类 VTable：`__BT: Self__VTable<..>`——调用方一定是本类的子类（或本类自身），
    // 其 vtable 经 supertrait 链满足该约束；本类及全部祖先的字段访问器都在约束可见范围内
    // （祖先 VTable 约束只能看到该祖先的字段，读不到中间层 / 本类字段）。
    //   - vtable-safe 方法体（只有字段访问器）→ 直接在 `&__BT` 上执行
    //   - 其余 → 经钩子重建本类 wrapper，执行 wrapper 上的 `__impl_<method>`
    let mut seen_override_bases: HashSet<String> = HashSet::new();
    for (_vtable_class, override_fns) in &vtable_overrides {
        for f in override_fns {
            let Some(block) = &f.block else { continue };
            let sig = &f.sig;
            if vtable_define_names.contains(&sig.ident.to_string())
                || !seen_override_bases.insert(sig.ident.to_string())
            {
                continue;
            }
            let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
            let non_self_params: Vec<_> = sig.inputs.iter()
                .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                .collect();
            let ret = &sig.output;

            let mut body_gen = gen.clone();
            body_gen.params.push(syn::parse_quote!(__BT));
            body_gen.make_where_clause().predicates.push(
                syn::parse_quote!(__BT: #vtable_trait_ident #ty_g + ?Sized)
            );
            // 方法自身的 where 子句（类型变量上界约束等）：方法体依赖它，base 函数同样声明
            if let Some(method_where) = &sig.generics.where_clause {
                body_gen.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
            }
            let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();

            let body: TokenStream2 = if is_vtable_safe_body(block) {
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                // 去掉首句 "let this = self;"（base 函数参数直接就是 this）
                if let Some(first) = b.stmts.first() {
                    let fs = quote!(#first).to_string();
                    if fs.contains("this") && fs.contains("self") {
                        b.stmts.remove(0);
                    }
                }
                let stmts = &b.stmts;
                quote! { #(#stmts)* }
            } else {
                let impl_name = format_ident!("__impl_{}", sig.ident);
                let args = param_idents(sig);
                quote! {
                    let __w = #vtable_trait_ident::#as_self_hook(this);
                    __w.#impl_name(#(#args),*)
                }
            };
            base_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case, unused_variables)]
                pub fn #fn_name #body_impl_g (this: &__BT #(, #non_self_params)*) #ret #body_where_c {
                    #body
                }
            });
        }
    }

    // ══════════════════════════════════════════════════════════════════════════
    // 最终组合
    // ══════════════════════════════════════════════════════════════════════════

    // 实现的接口：impl Iface__VTable for __inner（擦除签名 → 本类成员的桥接）
    let interface_impls: Vec<TokenStream2> = iface_impls.iter()
        .map(|ii| expand_interface_impl(ii, &struct_ident, &inner_ident, &vtable_trait_ident, &gen))
        .collect();

    quote! {
        #vtable_trait
        #inner_struct
        #obj_vtable_for_inner
        #(#interface_impls)*
        #(#vtable_impls)*
        #wrapper_struct
        #wrapper_from_parts
        #wrapper_default
        #wrapper_clone
        #wrapper_partialeq
        #wrapper_debug
        #obj_vtable_for_wrapper
        #wrapper_impl
        #binary_name_impl
        #from_object_impl
        #into_object_impl
        #from_child_for_parent
        #(#base_fns)*
    }
}
