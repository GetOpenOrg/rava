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
mod erasure;
mod gen;
mod generic_sig;
mod interface;
mod parse;
mod rewrite;
mod util;

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use classify::{vtable_body_kind_gated, VTableBodyKind};
use erasure::{
    erase_item_signature_with, erase_signature, erase_signature_with,
    erased_args_of_same_arity, erased_call_args, erased_call_args_with, erased_call_ret_conv,
    erased_call_ret_conv_with, erased_hook_call, erasure_set_of, expand_non_virtual_fn,
    forward_conv_spec,
};
use gen::GenContext;
use interface::{erased_impl_call, erased_wrapper_call, expand_interface, expand_interface_impl};
use generic_sig::rebuild_sig_with_generics;
use parse::{split_type_name_args, ClassInput, ClassMeta};
use rewrite::{
    replace_clone_this_in_ok, rewrite_base_calls_for_wrapper, rewrite_block,
    rewrite_dropped_params_in_inherited_body, rewrite_virtual_calls_for_wrapper,
    rewrite_vtable_calls_ufcs_for_base,
};
use util::{attr_str, is_basic, strip_meta_attrs};


pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<ClassInput>(input) {
        Ok(v) => expand_inner(v),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(input: ClassInput) -> TokenStream2 {
    match expand_class(&input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

fn expand_class(input: &ClassInput) -> syn::Result<TokenStream2> {
    let meta = ClassMeta::from_attrs(&input.attrs)?;

    // ── 泛型参数补齐 Clone + Default + 'static + From<Object> + Into<Object> ──────────────────────────────
    let gen = gen::context::augment_generic_bounds(&input.generics);

    // ── 接口：同名载体类型（接口引用 + 静态成员）────────────────────────────
    if meta.is_interface {
        return Ok(expand_interface(&meta, &input.struct_ident, &gen, &input.fns, &input.statics));
    }

    let ctx = GenContext::build(input, meta, gen);
    let binary_name = &ctx.meta.binary_name;
    // quote! 插值位置不能写 `#ctx.field`（会被解析为插值 ctx + 字面量 .field），
    // 预绑定为本名（与拆分前的局部变量同名，引用形态等价）
    let struct_ident = &ctx.struct_ident;
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let as_self_hook = &ctx.as_self_hook;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let erased_ty_args = &ctx.erased_ty_args;
    let phantom_field = &ctx.phantom_field;
    let phantom_init = &ctx.phantom_init;

    // ══════════════════════════════════════════════════════════════════════════
    // 1. VTable trait —— 非泛型（A-1 去形参：与接口载体 `I__VTable` 对齐）
    // ══════════════════════════════════════════════════════════════════════════

    // supertrait：有父类 → 父类 __VTable（同为非泛型）；无父类 → ObjectVTable
    let vtable_supertrait: TokenStream2 = if let Some(sup_ty) = &ctx.meta.superclass {
        // 提取超类的基础类型名（去掉泛型参数），如 Enum<Object> → Enum
        let base_name = if let Type::Path(tp) = sup_ty {
            tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
        } else {
            quote!(#sup_ty).to_string()
        };
        let sup_vtable = format_ident!("{}__VTable", base_name);
        quote! { #sup_vtable }
    } else {
        quote! { ObjectVTable }
    };

    // 字段 accessor 抽象方法（只有 own ctx.fields，不含继承字段）。
    // 擦除字段（声明类型提及类型形参，__inner 中以 Object 存储）：签名同步 Object 化 ——
    // impl 直连存储（Object 进 Object 出），类型化转换移到 wrapper 委托（β' 边界）。
    let mut vtable_abstract_methods: Vec<TokenStream2> = Vec::new();
    for (name, ty) in ctx.fields.iter() {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if ctx.is_erased(name) {
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> Object;
                fn #set(&self, v: Object);
            });
        } else {
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> #ty;
                fn #set(&self, v: #ty);
            });
        }
    }

    // VirtualDefine 的 default impl（A-1 去形参：签名 Object 化）：
    // - 有方法体 → default 委托 ClassName__method_base 自由函数（base 函数含实际体或
    //   钩子桥接；Safe 路径仅非泛型类，turbofish ::<Self>）
    // - 无方法体（abstract）→ default 生成 stub（子类必须覆盖，未覆盖则运行时命中）
    let mut vtable_default_methods: Vec<TokenStream2> = Vec::new();
    for f in &ctx.vtable_defines {
        let mname = &f.sig.ident;
        let fn_base_name = format_ident!("{}__{}_base", ctx.self_name, mname);
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
        let binary = &ctx.meta.binary_name;
        // generic_signature 重建：将 Object 参数/返回替换为类型变量（K, V 等），
        // 再整体 Object 化为擦除 vtable 签名（提及类型形参的位置 → Object）
        let effective_sig = attr_str(&f.attrs, "generic_signature")
            .and_then(|gs| rebuild_sig_with_generics(&f.sig, &gs, &ctx.class_type_params))
            .unwrap_or_else(|| f.sig.clone());
        let erased_default_sig = erase_signature(&effective_sig, &ctx.type_param_names);
        if let Some(block) = &f.block {
            if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                // vtable-safe 方法体（仅非泛型类）→ default 委托 base 函数
                // （体可直接在 &Self 上运行）；turbofish ::<Self>
                vtable_default_methods.push(quote! {
                    #erased_default_sig { #fn_base_name::<Self>(self, #(#param_names_for_default),*) }
                });
            } else {
                // 方法体需要 wrapper 上下文（this 传参 / 非虚方法调用 / Self::）→
                // 经钩子重建声明类的擦除实例化 wrapper，执行 wrapper 方法体 `__impl_<method>`
                // （签名已擦除，边界转换见 erased_impl_call）
                let impl_name = format_ident!("__impl_{}", mname);
                let call = erased_impl_call(
                    &effective_sig, &impl_name, &ctx.type_param_names, &HashSet::new());
                vtable_default_methods.push(quote! {
                    #erased_default_sig {
                        let __w = self.#as_self_hook();
                        #call
                    }
                });
            }
        } else if attr_str(&f.attrs, "body").as_deref() == Some("handwritten") {
            // 方法体由共置 `_impl.rs` 手写为 wrapper 上的 `__impl_<method>` → 经钩子重建 wrapper 后执行
            let impl_name = format_ident!("__impl_{}", mname);
            let call = erased_impl_call(
                &effective_sig, &impl_name, &ctx.type_param_names, &HashSet::new());
            vtable_default_methods.push(quote! {
                #erased_default_sig {
                    let __w = self.#as_self_hook();
                    #call
                }
            });
        } else {
            // 无方法体（abstract）→ stub，子类必须覆盖
            let msg = format!("stub: {}.{}:{}", binary, mname_str, desc);
            vtable_default_methods.push(quote! {
                #erased_default_sig { panic!(#msg) }
            });
        }
    }

    let vtable_trait = quote! {
        #[allow(non_camel_case_types)]
        pub trait #vtable_trait_ident: #vtable_supertrait {
            #[doc(hidden)]
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args;
            #(#vtable_abstract_methods)*
            #(#vtable_default_methods)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 2. Inner struct + 3. impl ObjectVTable for __inner → gen/struct_layout.rs
    // ══════════════════════════════════════════════════════════════════════════
    let layout = gen::struct_layout::generate(&ctx);

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

    // 字段访问器 impl 体（三个 vtable impl 生成点共用）。
    // 擦除字段（__inner 中以 Object 存储）：签名 Object 化（与 trait 声明一致），
    // impl 直连存储（Object 进 Object 出）；类型化转换移到 wrapper 委托（β' 边界）。
    let accessor_impl_items = |name: &syn::Ident, ty: &Type, basic: bool| -> TokenStream2 {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if ctx.is_erased(name) {
            quote! {
                fn #get(&self) -> Object {
                    self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                }
                fn #set(&self, v: Object) {
                    *self.#name.borrow_mut() = ::std::option::Option::Some(
                        ::std::boxed::Box::new(v));
                }
            }
        } else if basic {
            quote! {
                fn #get(&self) -> #ty { self.#name.get() }
                fn #set(&self, v: #ty) { self.#name.set(v); }
            }
        } else {
            quote! {
                fn #get(&self) -> #ty {
                    self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                }
                fn #set(&self, v: #ty) {
                    *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                }
            }
        }
    };

    if ctx.meta.superclass.is_none() {
        // ── 无父类：impl Self__VTable for __inner（非泛型，A-1 去形参）──────────
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in ctx.fields.iter() {
            own_accessor_impls.push(accessor_impl_items(name, ty, is_basic(ty)));
        }
        // VirtualDefine 方法体：Safe（仅非泛型类）→ 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &ctx.vtable_defines {
            if let Some(block) = &f.block {
                if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args {
                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident>,
                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                    #phantom_init
                }
            }
        });

        vtable_impls.push(quote! {
            impl #vtable_trait_ident for #inner_ident {
                #(#own_accessor_impls)*
            }
        });
    } else {
        // ── 有父类：顶层祖先 vtable impl + 其余空 impl + Self__VTable impl ──

        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = ctx.meta.all_superclasses.clone();

        // 构建字段名 → (Ident, Type) 的查找表（superclass_fields 平铺列表）
        let sc_fields_map: HashMap<String, (Ident, Type)> = ctx.meta
            .superclass_fields
            .iter()
            .map(|(n, t)| (n.to_string(), (n.clone(), t.clone())))
            .collect();

        for anc_name in ancestors.iter() {
            let anc_vtable_ident = format_ident!("{}__VTable", anc_name);

            // 祖先 vtable 的类型实参（本类视角）：仅用于推导祖先的形参元数 ——
            // vtable 去形参（A-1）后 impl 头与钩子返回类型均按「该元数的全 Object
            // 擦除实例化」生成（祖先 trait 声明的钩子返回 `Anc<Object, ..>`）。
            let anc_vtable_args: TokenStream2 =
                ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            let anc_erased_args = erased_args_of_same_arity(&anc_vtable_args);

            let mut items: Vec<TokenStream2> = Vec::new();

            // 将此祖先自己声明的字段 accessor 放进此 vtable impl
            let anc_own_field_names: Vec<String> =
                if let Some(names) = ctx.meta.ancestor_fields_layout.get(anc_name) {
                    names.clone()
                } else if ctx.meta.ancestor_fields_layout.is_empty() {
                    // 兜底：无 ancestor_fields_layout 时（旧属性），所有字段放最深祖先（首位）
                    if anc_name == ancestors.first().map(|s| s.as_str()).unwrap_or("") {
                        ctx.meta.superclass_fields.iter().map(|(n, _)| n.to_string()).collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

            for field_name in &anc_own_field_names {
                if let Some((name, ty)) = sc_fields_map.get(field_name) {
                    items.push(accessor_impl_items(name, ty, ctx.inherited_is_basic(name, ty)));
                }
            }

            // VirtualOverride 方法：virtual_in == anc_name（签名随 trait 整体 Object 化）
            if let Some(override_fns) = ctx.vtable_overrides.get(anc_name) {
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    // 覆盖条目可能直挂方法体（Safe）→ 保留形参 mut（体可能赋值）；
                    // 声明祖先的类型形参位置（vtable_erasure 名集）一并擦除
                    let ov_erasure = erasure_set_of(f, &ctx.type_param_names);
                    let erased_item_sig = erase_item_signature_with(
                        sig, &ctx.type_param_names, &ov_erasure);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                            if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #erased_item_sig #b
                                });
                            } else {
                                // 方法体需要 wrapper 上下文（this 传参 / 非虚方法调用 / Self::）：
                                // 方法体只落在 wrapper 的 `__impl_<method>` 上（见 wrapper 方法生成），
                                // 此处经擦除实例化的 wrapper 执行——与 VirtualDefine 同一路径，
                                // super 调用的 base 函数也复用它（不分派，精确命中本类实现）。
                                // impl 签名已擦除：形参 / 返回值在边界与擦除 wrapper 的
                                // `__impl_<method>`（类型实参全 Object）做 Object ↔ 类型化转换。
                                let impl_name = format_ident!("__impl_{}", sig.ident);
                                let wrapper_call = erased_wrapper_call(
                                    sig, &impl_name, &ctx.struct_ident, &ctx.vtable_trait_ident,
                                    &ctx.erased_ty_args, &ctx.phantom_init, &ctx.type_param_names,
                                    &ov_erasure,
                                );
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #erased_item_sig { #wrapper_call }
                                });
                            }
                        }
                        None if attr_str(&f.attrs, "body").as_deref() == Some("handwritten") => {
                            let impl_name = format_ident!("__impl_{}", sig.ident);
                            let wrapper_call = erased_wrapper_call(
                                sig, &impl_name, &ctx.struct_ident, &ctx.vtable_trait_ident,
                                &ctx.erased_ty_args, &ctx.phantom_init, &ctx.type_param_names,
                                &ov_erasure,
                            );
                            items.push(quote! {
                                #(#keep_attrs)*
                                #erased_item_sig { #wrapper_call }
                            });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                            let bin = &ctx.meta.binary_name;
                            let msg = format!("stub: {}.{}:{}", bin, mname, desc);
                            items.push(quote! {
                                #erased_item_sig { panic!(#msg) }
                            });
                        }
                    }
                }
            }

            // 继承成员填槽（S-16）：带转发体的继承声明把体放进本类对 vtable_owner 的
            // vtable impl。vtable_owner 声明为 abstract（实现位于中间祖先）时，trait
            // default 是 stub panic——不填槽则子类对象经 vtable 虚分派命中空洞声明。
            // 转发体由 emitter 生成（Owner__m_base / __as_Owner 钩子，super 调用同源）。
            for (f, _owner, vtable_owner, _interface_owner) in &ctx.inherited {
                let Some(block) = &f.block else { continue };
                let Some(vo) = vtable_owner else { continue };
                let vo_ty = match syn::parse_str::<Type>(vo) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let (vo_name, _) = split_type_name_args(&vo_ty);
                if vo_name != *anc_name {
                    continue;
                }
                let sig = &f.sig;
                let erasure = erasure_set_of(f, &ctx.type_param_names);
                let erased_item_sig = erase_signature_with(sig, &ctx.type_param_names, &erasure);
                let conv = forward_conv_spec(sig, &erased_item_sig, &ctx.type_param_names, &erasure);
                let mut b = block.clone();
                // 槽位上下文不携带本类类型形参（vtable 去形参）→ 转发体的 base 调用
                // turbofish 里被删形参 / 擦除形态取 Object（base 函数体是参数化的，
                // 行为参数化一致）；`X__VTable<..>` 路径的实参整体剥除。
                let dropped: HashSet<String> = if ctx.class_is_generic {
                    ctx.gen.type_params().map(|tp| tp.ident.to_string()).collect()
                } else {
                    HashSet::new()
                };
                rewrite_dropped_params_in_inherited_body(&mut b, &dropped, &erasure, &conv);
                items.push(quote! { #erased_item_sig #b });
            }

            // 祖先 wrapper 重建钩子（经 __from_parts：祖先可能在另一个 crate，字段不可见）。
            // 返回祖先的擦除实例化（与祖先 trait 声明的钩子签名一致，A-1 去形参）。
            // turbofish：vtable 非泛型后 __from_parts 的祖先形参不再经实参类型钉住
            // （非泛型祖先无实参，返回类型可钉住，直接调用）。
            let anc_ident = format_ident!("{}", anc_name);
            let anc_hook = format_ident!("__as_{}", anc_name);
            let anc_from_parts: TokenStream2 = if anc_erased_args.is_empty() {
                quote! { #anc_ident::__from_parts }
            } else {
                quote! { #anc_ident::#anc_erased_args::__from_parts }
            };
            items.push(quote! {
                fn #anc_hook(&self) -> #anc_ident #anc_erased_args {
                    let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                    #anc_from_parts(
                        __rc.clone() as ::std::rc::Rc<dyn #anc_vtable_ident>,
                        __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                        false,
                    )
                }
            });

            vtable_impls.push(quote! {
                impl #anc_vtable_ident for #inner_ident {
                    #(#items)*
                }
            });
        }

        // Self__VTable impl（own ctx.fields 的 accessor）
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in ctx.fields.iter() {
            own_accessor_impls.push(accessor_impl_items(name, ty, is_basic(ty)));
        }
        // VirtualDefine 方法体：Safe → 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &ctx.vtable_defines {
            if let Some(block) = &f.block {
                if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args {
                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident>,
                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                    _jvm_null: false,
                    #phantom_init
                }
            }
        });

        vtable_impls.push(quote! {
            impl #vtable_trait_ident for #inner_ident {
                #(#own_accessor_impls)*
            }
        });

        // 处理未被 ancestors 列表覆盖的 VirtualOverride（vtable_class 不在祖先中）
        // 这种情况理论上不应出现，但做兜底生成
        for (vtable_class, override_fns) in &ctx.vtable_overrides {
            if !ancestors.contains(vtable_class) {
                let anc_vtable_ident = format_ident!("{}__VTable", vtable_class);
                let mut items: Vec<TokenStream2> = Vec::new();
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let erased_item_sig = erase_signature(sig, &ctx.type_param_names);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                            items.push(quote! { #(#keep_attrs)* #erased_item_sig #b });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let msg = format!("stub: {}.{}", ctx.meta.binary_name, mname);
                            items.push(quote! { #erased_item_sig { panic!(#msg) } });
                        }
                    }
                }
                vtable_impls.push(quote! {
                    impl #anc_vtable_ident for #inner_ident {
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
        quote! {
            #[doc(hidden)]
            pub fn __new_with_super(parent: #sup_ty) -> Self {
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

    // ══════════════════════════════════════════════════════════════════════════
    // 8-10. BINARY_NAME / From<Object> / Into<Object> / From<Child> for Ancestor
    //       → gen/type_conversions.rs
    // ══════════════════════════════════════════════════════════════════════════
    let conversions = gen::type_conversions::generate(&ctx);

    // ══════════════════════════════════════════════════════════════════════════
    // 11. 自由函数 ClassName__methodName_base（供 invokespecial super() 调用）
    // ══════════════════════════════════════════════════════════════════════════

    let mut base_fns: Vec<TokenStream2> = Vec::new();

    // base 函数用的合并泛型：类泛型 + __BT: ?Sized（无 vtable trait 约束，因为全是 panic stub）
    // ?Sized 允许传入 &dyn VTable（fat pointer），不要求 __BT 实现 Sized
    let mut base_gen = ctx.gen.clone();
    base_gen.params.push(syn::parse_quote!(__BT: ?Sized));
    let (base_impl_g, _, _) = base_gen.split_for_impl();

    // VirtualDefine 方法生成 base 函数
    // 策略（两阶段）：
    //   1. 先对 body 做 replace_clone_this_in_ok（Ok(Clone::clone(this)) → Ok(Default::default())）
    //   2. 检查替换后 body 在 this: &__BT 上下文是否安全：
    //      - 仍有 bare Clone::clone(this)（传参用途）→ 不安全 → panic stub
    //      - 仍有 this.method() 且 method 不在 ctx.vtable_define_names（NonVirtual/native）→ 不安全 → panic stub
    //      - 否则（VirtualDefine vtable 方法调用 + accessor 调用）→ 安全，使用实际 body
    // 注：VirtualDefine 方法（appendNull / ensureCapacityInternal 等）在 &__BT: VTable 上下文可直接调用

    for f in &ctx.vtable_defines {
        if let Some(block) = &f.block {
            let sig = &f.sig;
            let fn_name = format_ident!("{}__{}_base", ctx.self_name, sig.ident);
            let non_self_params: Vec<_> = sig.inputs.iter()
                .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                .collect();
            let ret = &sig.output;
            let mname_str = sig.ident.to_string();
            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
            let binary = &ctx.meta.binary_name;

            let mut b = block.clone();
            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
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
                            // 只有不在 ctx.vtable_define_names 里的才是 non-vtable 调用
                            if !ctx.vtable_define_names.contains(&mname) {
                                found = true;
                                break;
                            }
                        }
                    }
                }
                found
            };

            let has_self_ref = bs_flat.contains("Self ::") || bs_flat.contains("Self::");
            if !matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                // 方法体需要 wrapper 上下文 → 经钩子重建本类擦除实例化 wrapper，执行
                // `__impl_<method>`。base 函数签名保持类型化（Python 调用点的 turbofish
                // 不变）；钩子返回擦除实例化 → 形参 / 返回值在边界转换（A-1 去形参）。
                let _ = (&binary, &mname_str, &desc, has_bare_clone_this, has_non_vtable_call, has_self_ref);
                let impl_name = format_ident!("__impl_{}", sig.ident);
                let body = erased_hook_call(
                    sig, &impl_name, &ctx.vtable_trait_ident, &ctx.as_self_hook, &ctx.type_param_names);
                let mut body_gen = ctx.gen.clone();
                body_gen.params.push(syn::parse_quote!(__BT));
                body_gen.make_where_clause().predicates.push(
                    syn::parse_quote!(__BT: #vtable_trait_ident + ?Sized)
                );
                if let Some(method_where) = &sig.generics.where_clause {
                    body_gen.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
                }
                let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #body_impl_g (this: &__BT #(, #non_self_params)*) #ret #body_where_c {
                        #body
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
                rewrite_vtable_calls_ufcs_for_base(&mut b, &ctx.vtable_define_names, &ctx.vtable_trait_ident);
                let mut body_gen = ctx.gen.clone();
                body_gen.params.push(syn::parse_quote!(__BT));
                body_gen.make_where_clause().predicates.push(
                    syn::parse_quote!(__BT: #vtable_trait_ident + ?Sized)
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
    for (_vtable_class, override_fns) in &ctx.vtable_overrides {
        for f in override_fns {
            let Some(block) = &f.block else { continue };
            let sig = &f.sig;
            if ctx.vtable_define_names.contains(&sig.ident.to_string())
                || !seen_override_bases.insert(sig.ident.to_string())
            {
                continue;
            }
            let fn_name = format_ident!("{}__{}_base", ctx.self_name, sig.ident);
            let non_self_params: Vec<_> = sig.inputs.iter()
                .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                .collect();
            let ret = &sig.output;

            let mut body_gen = ctx.gen.clone();
            body_gen.params.push(syn::parse_quote!(__BT));
            body_gen.make_where_clause().predicates.push(
                syn::parse_quote!(__BT: #vtable_trait_ident + ?Sized)
            );
            // 方法自身的 where 子句（类型变量上界约束等）：方法体依赖它，base 函数同样声明
            if let Some(method_where) = &sig.generics.where_clause {
                body_gen.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
            }
            let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();

            let body: TokenStream2 = if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                let mut b = block.clone();
                rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
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
                // 方法体需要 wrapper 上下文 → 经钩子重建本类擦除实例化 wrapper，执行
                // `__impl_<method>`（签名保持类型化，边界转换同 VirtualDefine base 函数）
                erased_hook_call(
                    sig, &format_ident!("__impl_{}", sig.ident),
                    &ctx.vtable_trait_ident, &ctx.as_self_hook, &ctx.type_param_names)
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
    let interface_impls: Vec<TokenStream2> = ctx.iface_impls.iter()
        .map(|ii| expand_interface_impl(ii, &ctx.struct_ident, &ctx.inner_ident, &ctx.vtable_trait_ident,
                                        &ctx.erased_ty_args, &ctx.phantom_init))
        .collect();

    Ok(quote! {
        #vtable_trait
        #layout
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
        #conversions
        #(#base_fns)*
    })
}
