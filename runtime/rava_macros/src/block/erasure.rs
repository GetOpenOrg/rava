//! 类型擦除辅助（JVM 泛型擦除语义的签名 / token 转换）与非虚方法展开。
//!
//! A-1 存储层擦除下，vtable 方法签名按「提及本类类型形参 → Object」整体 Object 化。
//! 本模块集中提供：类型 / 签名的擦除与 objectize、擦除调用点的边界转换（装箱 / 还原）、
//! 以及 Constructor / NonVirtual / static 方法的原 body 展开。

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, Type};

use super::class_init;
use super::parse::FnItem;
use super::rewrite::{rewrite_base_calls_for_wrapper, rewrite_block, ForwardConvSpec};
use super::util::{attr_str, strip_meta_attrs};

/// Constructor / NonVirtual / static 方法：保持原 body（走 Rewriter）；无 body → panic stub。
/// 类的 wrapper impl 与接口载体的 impl 共用。
pub(crate) fn expand_non_virtual_fn(
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
pub(crate) fn param_idents(sig: &syn::Signature) -> Vec<Ident> {
    sig.inputs.iter().filter_map(|arg| match arg {
        syn::FnArg::Typed(pt) => match &*pt.pat {
            syn::Pat::Ident(pi) => Some(pi.ident.clone()),
            _ => None,
        },
        syn::FnArg::Receiver(_) => None,
    }).collect()
}

/// 去掉形参上的 `mut`（声明 → 转发方法签名）。
pub(crate) fn without_param_mut(sig: &syn::Signature) -> syn::Signature {
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

pub(crate) fn mentions_any(ty: &Type, names: &HashSet<String>) -> bool {
    quote!(#ty).into_iter().any(|tt| tt_mentions(&tt, names))
}

/// token 流中出现的本类类型形参标识符替换为 `Object`（保持类型结构）：
/// 描述「方法体落在擦除实例化 `X<Object, ..>` wrapper 上时，签名的实例化形态」
/// （`JArray<E>` → `JArray<Object>`，裸 `E` → `Object`）。
fn objectize_ts(ts: &TokenStream2, params: &HashSet<String>) -> TokenStream2 {
    let replaced: Vec<TokenStream2> = ts.clone().into_iter().map(|tt| match tt {
        proc_macro2::TokenTree::Ident(i) => {
            if params.contains(&i.to_string()) { quote! { Object } } else { quote! { #i } }
        }
        proc_macro2::TokenTree::Group(g) => {
            let inner = objectize_ts(&g.stream(), params);
            let rebuilt = proc_macro2::Group::new(g.delimiter(), inner);
            quote! { #rebuilt }
        }
        other => quote! { #other },
    }).collect();
    quote! { #(#replaced)* }
}

pub(crate) fn objectize_type(ty: &Type, params: &HashSet<String>) -> Type {
    let ts = objectize_ts(&quote!(#ty), params);
    syn::parse_quote!(#ts)
}

/// 两个类型的 token 文本是否一致（边界转换必要性的判据）。
pub(crate) fn same_type_tokens(a: &Type, b: &Type) -> bool {
    quote!(#a).to_string() == quote!(#b).to_string()
}

/// `Result<T>` 返回类型中的 `T`；非 `Result<..>` 返回返回 None。
/// 类型实参流（`<T, Object>`）的顶层元数；空实参流返回 0。
pub(crate) fn type_args_arity(args: &TokenStream2) -> usize {
    let mut ts: Vec<proc_macro2::TokenTree> = args.clone().into_iter().collect();
    if ts.is_empty() {
        return 0;
    }
    let starts_lt = matches!(ts.first(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '<');
    let ends_gt = matches!(ts.last(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '>');
    if starts_lt && ends_gt && ts.len() >= 2 {
        ts.drain(0..1);
        ts.pop();
    }
    let mut depth = 0usize;
    let mut n = 1usize;
    for tt in &ts {
        if let proc_macro2::TokenTree::Punct(p) = tt {
            match p.as_char() {
                '<' | '(' | '[' => depth += 1,
                '>' | ')' | ']' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => n += 1,
                _ => {}
            }
        }
    }
    n
}

/// 与给定类型实参流（`<T, Object>`）同元数的全 `Object` 实参流；空实参流返回空。
/// 用于从「本类视角的祖先实参元数」推导祖先的擦除实例化（祖先 trait 的钩子返回类型）。
pub(crate) fn erased_args_of_same_arity(args: &TokenStream2) -> TokenStream2 {
    let mut ts: Vec<proc_macro2::TokenTree> = args.clone().into_iter().collect();
    if ts.is_empty() {
        return quote! {};
    }
    // 剥除外层 <>（实参流自带的括号不计入深度）
    let starts_lt = matches!(ts.first(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '<');
    let ends_gt = matches!(ts.last(), Some(proc_macro2::TokenTree::Punct(p)) if p.as_char() == '>');
    if starts_lt && ends_gt && ts.len() >= 2 {
        ts.drain(0..1);
        ts.pop();
    }
    let mut depth = 0usize;
    let mut n = 1usize;
    for tt in &ts {
        if let proc_macro2::TokenTree::Punct(p) = tt {
            match p.as_char() {
                '<' | '(' | '[' => depth += 1,
                '>' | ')' | ']' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => n += 1,
                _ => {}
            }
        }
    }
    let objs = vec![quote! { Object }; n];
    quote! { <#(#objs),*> }
}

pub(crate) fn result_inner_ty(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Result" {
                if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                        return Some(inner);
                    }
                }
            }
        }
    }
    None
}

/// 类型化调用点 → 擦除 vtable 方法的实参转换：提及类型形参的形参经 `Into::<Object>`
/// 装箱（擦除签名在这些位置接收 `Object`），其余原样传递。
pub(crate) fn erased_call_args(
    sig: &syn::Signature,
    type_param_names: &HashSet<String>,
) -> Vec<TokenStream2> {
    erased_call_args_with(sig, type_param_names, &HashSet::new())
}

/// 同 erased_call_args，附加「扁平 token 全等命中即装箱」的扩展判据
/// （vtable_erasure 名集：owner 类型形参位置的代入形态）。
pub(crate) fn erased_call_args_with(
    sig: &syn::Signature,
    type_param_names: &HashSet<String>,
    exact: &HashSet<String>,
) -> Vec<TokenStream2> {
    sig.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => {
            let ident = match &*pt.pat {
                syn::Pat::Ident(pi) => pi.ident.clone(),
                _ => return None,
            };
            let hit = mentions_any(&pt.ty, type_param_names)
                || (!exact.is_empty() && exact.contains(&flat_type_tokens(&pt.ty)));
            if hit {
                Some(quote! { ::std::convert::Into::<Object>::into(#ident) })
            } else {
                Some(quote! { #ident })
            }
        }
        _ => None,
    }).collect()
}

/// 擦除 vtable 调用（返回 `Result<Object>`）→ 调用点签名返回 `Result<T>`（T 提及类型
/// 形参）的还原：`.map(<T as From<Object>>::from)`；其余情形原样返回调用表达式。
pub(crate) fn erased_call_ret_conv(
    sig: &syn::Signature,
    type_param_names: &HashSet<String>,
    call: TokenStream2,
) -> TokenStream2 {
    erased_call_ret_conv_with(sig, type_param_names, &HashSet::new(), call)
}

pub(crate) fn erased_call_ret_conv_with(
    sig: &syn::Signature,
    type_param_names: &HashSet<String>,
    exact: &HashSet<String>,
    call: TokenStream2,
) -> TokenStream2 {
    if let syn::ReturnType::Type(_, ty) = &sig.output {
        if let Some(inner) = result_inner_ty(ty) {
            let hit = mentions_any(inner, type_param_names)
                || (!exact.is_empty() && exact.contains(&flat_type_tokens(inner)));
            if hit {
                // null 容忍：null 经由擦除 vtable 往返后在值位置还原（Java 丢弃返回值 /
                // 引用位置 null）——primitive 位置取 Default（与类型化直连时代码的
                // Ok(Default::default()) 行为一致），避免拆箱 NPE（S-1 家族的值位形态）
                return quote! {
                    #call.map(|__v| if __v.0.is_jvm_null() {
                        ::std::default::Default::default()
                    } else {
                        <#inner as ::std::convert::From<Object>>::from(__v)
                    })
                };
            }
        }
    }
    call
}

/// 类型化签名上下文（base 函数体：类形参在作用域内）经 `__as_X` 钩子（返回擦除实例化
/// `X<Object, ..>`）执行 wrapper 的 `__impl_<m>`：形参 / 返回值在「类型化 ↔ objectize」
/// 边界转换——裸类型形参经 Into/From<Object>，嵌套提及（`JArray<E>`）经 Object 装拆箱
/// （元素类型跨实例化按精确类型取回，S-4 边界）。Python 调用点的 turbofish 不变。
pub(crate) fn erased_hook_call(
    sig: &syn::Signature,
    impl_name: &Ident,
    vtable_trait_ident: &Ident,
    as_self_hook: &Ident,
    type_param_names: &HashSet<String>,
) -> TokenStream2 {
    let conv_args: Vec<TokenStream2> = sig.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => {
            let ident = match &*pt.pat {
                syn::Pat::Ident(pi) => pi.ident.clone(),
                _ => return None,
            };
            if mentions_any(&pt.ty, type_param_names) {
                let obj_ty = objectize_type(&pt.ty, type_param_names);
                Some(quote! { <#obj_ty as ::std::convert::From<Object>>::from(
                    ::std::convert::Into::<Object>::into(#ident)) })
            } else {
                Some(quote! { #ident })
            }
        }
        _ => None,
    }).collect();
    let mut call = quote! { __w.#impl_name(#(#conv_args),*) };
    if let syn::ReturnType::Type(_, ty) = &sig.output {
        if let Some(inner) = result_inner_ty(ty) {
            if mentions_any(inner, type_param_names) {
                let obj_inner = objectize_type(inner, type_param_names);
                let obj_is_object = same_type_tokens(
                    &obj_inner,
                    &syn::parse_quote!(Object));
                if obj_is_object {
                    // 裸类型形参：vtable 位置是 Object，primitive 实参下 null 需取
                    // Default（与类型化直连代码行为一致），避免拆箱 NPE
                    call = quote! {
                        #call.map(|__v| if __v.0.is_jvm_null() {
                            ::std::default::Default::default()
                        } else {
                            <#inner as ::std::convert::From<Object>>::from(__v)
                        })
                    };
                } else {
                    // 嵌套提及：objectize 形态 → Object 装箱 → 还原（From 对 null 安全）
                    call = quote! {
                        #call.map(|__v| <#inner as ::std::convert::From<Object>>::from(
                            ::std::convert::Into::<Object>::into(__v)))
                    };
                }
            }
        }
    }
    quote! {
        let __w = #vtable_trait_ident::#as_self_hook(this);
        #call
    }
}

/// 类型擦除（JVM 语义）：提及接口类型变量的类型位置在运行时一律是 `Object` 引用；
/// 其余类型（基本类型、非泛型类、已实参化的类）保持不变。`Result<T>` 只擦除 `T`。
pub(crate) fn erase_type(ty: &Type, type_params: &HashSet<String>) -> Type {
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
pub(crate) fn erase_signature(sig: &syn::Signature, type_params: &HashSet<String>) -> syn::Signature {
    erase_signature_with(sig, type_params, &HashSet::new())
}

/// impl 条目的擦除签名：同 erase_signature_with，但保留形参的 `mut` ——
/// 带方法体的条目（Safe 覆盖体直挂）可能对形参赋值（E0384）。
pub(crate) fn erase_item_signature_with(
    sig: &syn::Signature,
    type_params: &HashSet<String>,
    exact: &HashSet<String>,
) -> syn::Signature {
    let mut out = erase_signature_with(sig, type_params, exact);
    for (a, b) in out.inputs.iter_mut().zip(sig.inputs.iter()) {
        if let (syn::FnArg::Typed(pt), syn::FnArg::Typed(orig)) = (a, b) {
            if let (syn::Pat::Ident(pi), syn::Pat::Ident(oi)) = (&mut *pt.pat, &*orig.pat) {
                pi.mutability = oi.mutability;
            }
        }
    }
    out
}

/// 擦除签名（扩展判据）：提及本类类型形参 → Object；类型扁平 token 命中
/// `vtable_erasure` 名集（owner 类型形参位置的代入形态）→ 同样 Object。
pub(crate) fn erase_signature_with(
    sig: &syn::Signature,
    type_params: &HashSet<String>,
    exact: &HashSet<String>,
) -> syn::Signature {
    let ty_erased = |ty: &Type| -> Type {
        if !exact.is_empty() && exact.contains(&flat_type_tokens(ty)) {
            return syn::parse_quote!(Object);
        }
        erase_type(ty, type_params)
    };
    let ret_erased = |ty: &Type| -> Type {
        if let Some(inner) = result_inner_ty(ty) {
            if !exact.is_empty() && exact.contains(&flat_type_tokens(inner)) {
                return syn::parse_quote!(Result<Object>);
            }
        }
        erase_type(ty, type_params)
    };
    let mut out = without_param_mut(sig);
    for arg in out.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = arg {
            *pt.ty = ty_erased(&pt.ty);
        }
    }
    if let syn::ReturnType::Type(_, ty) = &mut out.output {
        **ty = ret_erased(&ty);
    }
    out
}

/// 方法条目的擦除名集：本类类型形参 ∪ `vtable_erasure` 属性携带的「owner 类型形参
/// 位置在接收者视角下代入后的类型串」（声明方 vtable 已在这些位置 Object 化，A-1）。
pub(crate) fn erasure_set_of(f: &FnItem, type_param_names: &HashSet<String>) -> HashSet<String> {
    let mut set = type_param_names.clone();
    if let Some(e) = attr_str(&f.attrs, "vtable_erasure") {
        for t in e.split(';').filter(|x| !x.is_empty()) {
            // 与 flat_type_tokens 同一规范化（去全部空白）——Python 侧类型串带空格
            let flat: String = t.chars().filter(|c| !c.is_whitespace()).collect();
            set.insert(flat);
        }
    }
    set
}

/// 转发体（槽位条目 → base 函数 / 擦除 wrapper `__impl_<m>`）的边界转换规格：
/// 逐位置比较「擦除条目签名」与「objectize 后的原签名」，形态不同的位置需要显式
/// From<Object> / From<Obj> 转换（嵌套提及，如 `HashMap_Node<K, V>`）。
pub(crate) fn forward_conv_spec(
    orig: &syn::Signature,
    erased: &syn::Signature,
    type_param_names: &HashSet<String>,
    erasure: &HashSet<String>,
) -> ForwardConvSpec {
    let mut arg_convs: Vec<(String, Type)> = Vec::new();
    let orig_params: Vec<&Type> = orig.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => Some(&*pt.ty),
        _ => None,
    }).collect();
    let erased_params: Vec<&Type> = erased.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => Some(&*pt.ty),
        _ => None,
    }).collect();
    // 裸单标识符类型（`T` / `Status`）：base 调用的 turbofish 实参即此类型，
    // 被「被删形参 / 擦除名集」改写为 Object 后 base 形参已是 Object——无需显式转换；
    // 嵌套形态（`HashMap_Node<K, V>`）的 turbofish 只改写内部实参，base 形参保持
    // 包装形态 → 需 From<Object> 还原
    let is_bare_ident = |ty: &Type| -> bool {
        if let Type::Path(tp) = ty {
            tp.qself.is_none() && tp.path.segments.len() == 1 && tp.path.segments[0].arguments.is_none()
        } else {
            false
        }
    };
    for (i, (o, e)) in orig_params.iter().zip(erased_params.iter()).enumerate() {
        let mentions_own = mentions_any(o, type_param_names);
        let hits_erasure = !erasure.is_empty()
            && erasure.contains(&flat_type_tokens(o));
        if !(mentions_own || hits_erasure) {
            continue;
        }
        // 本类形参的裸位置由 turbofish 改写消解（base 形参已 Object）；
        // vtable_erasure 命中的位置（含裸形态）base 形参保持代入形态 → 显式转换
        if mentions_own && !hits_erasure && is_bare_ident(o) {
            continue;
        }
        let obj: Type = if mentions_any(o, type_param_names) {
            objectize_type(o, type_param_names)
        } else {
            (*o).clone()
        };
        if !same_type_tokens(&obj, e) {
            if let Some((name, _)) = orig.inputs.iter().filter_map(|a| match a {
                syn::FnArg::Typed(pt) => match &*pt.pat {
                    syn::Pat::Ident(pi) => Some((pi.ident.to_string(), ())),
                    _ => None,
                },
                _ => None,
            }).nth(i) {
                arg_convs.push((name, obj));
            }
        }
    }
    let mut ret_conv = None;
    if let (syn::ReturnType::Type(_, o), syn::ReturnType::Type(_, e)) = (&orig.output, &erased.output) {
        if let (Some(oi), Some(ei)) = (result_inner_ty(o), result_inner_ty(e)) {
            let mentions_own = mentions_any(oi, type_param_names);
            let hits_erasure = !erasure.is_empty()
                && erasure.contains(&flat_type_tokens(oi));
            if (mentions_own || hits_erasure) && !(mentions_own && !hits_erasure && is_bare_ident(oi)) {
                let obj_inner: Type = if mentions_any(oi, type_param_names) {
                    objectize_type(oi, type_param_names)
                } else {
                    (*oi).clone()
                };
                if !same_type_tokens(&obj_inner, ei) {
                    ret_conv = Some((ei.clone(), obj_inner));
                }
            }
        }
    }
    ForwardConvSpec { arg_convs, ret_conv }
}

/// 类型的扁平 token 文本（去空白）——与 Python 侧 vtable_erasure 条目全等匹配用。
pub(crate) fn flat_type_tokens(ty: &Type) -> String {
    quote!(#ty).to_string()
        .chars().filter(|c| !c.is_whitespace()).collect()
}
