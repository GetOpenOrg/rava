//! GenContext：`expand_inner` 各生成阶段共享的上下文（唯一的跨阶段数据传递机制）。
//!
//! 字段表以 2026-09-18 审计修订版为准；A-1（vtable 去形参 / 存储层擦除）合入后新增
//! `as_self_hook` / `type_param_names` / `class_is_generic` / `erased_ty_args` /
//! `erased_own` / `erased_super` / `phantom_field` / `phantom_init` / `inherited` /
//! `vdispatch` 等字段（差异见拆分报告）。

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{GenericParam, Ident, Type};

use super::super::erasure::{erasure_set_of, flat_type_tokens, mentions_any, result_inner_ty};
use super::super::parse::{ClassInput, ClassMeta, FnItem, InterfaceImpl, StaticItem};
use super::super::rewrite::VDispatchSig;
use super::super::util::{classify_method, is_basic, MethodKind};

/// 泛型参数补齐 Clone + Default + 'static + From<Object> + Into<Object>
/// （接口路径与类路径共用；Java 类型实参恒为引用类型：与 Object 双向可转）。
pub(crate) fn augment_generic_bounds(generics: &syn::Generics) -> syn::Generics {
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
    gen
}

/// 继承字段的存储形态由声明它的祖先决定（见 superclass_reference_fields）。
fn inherited_is_basic(meta: &ClassMeta, name: &syn::Ident, ty: &syn::Type) -> bool {
    is_basic(ty) && !meta.superclass_reference_fields.contains(&name.to_string())
}

pub(crate) struct GenContext<'a> {
    // ── 标识符 ────────────────────────────────────────────────
    pub(crate) struct_ident: &'a Ident,
    /// struct_ident.to_string()，base fn 命名用
    pub(crate) self_name: String,
    pub(crate) inner_ident: Ident,
    pub(crate) vtable_trait_ident: Ident,
    /// wrapper 重建钩子 `Class__VTable::__as_Class(&self) -> Class<Object, ..>`
    pub(crate) as_self_hook: Ident,

    // ── 泛型（完整对象 + split 便捷引用）────────────────────────
    pub(crate) gen: syn::Generics,
    pub(crate) impl_g: TokenStream2,
    pub(crate) ty_g: TokenStream2,
    pub(crate) where_c: TokenStream2,
    /// 类型参数名，§1 generic_sig 重建与擦除判定用
    pub(crate) class_type_params: Vec<String>,
    pub(crate) type_param_names: HashSet<String>,
    pub(crate) class_is_generic: bool,
    /// 擦除实例化的类型实参（全 Object）：`X<Object, ..., Object>`
    pub(crate) erased_ty_args: TokenStream2,

    // ── 元数据 ────────────────────────────────────────────────
    pub(crate) meta: ClassMeta,

    // ── 字段 ──────────────────────────────────────────────────
    pub(crate) fields: &'a [(Ident, Type)],
    /// 值类型字段名（供 Rewriter 使用）
    pub(crate) basic_names: HashSet<String>,
    /// 引用类型字段名（供 Rewriter 使用）
    pub(crate) ref_names: HashSet<String>,
    /// 声明类型提及本类类型形参的自有字段（A-1 存储层擦除名单）
    pub(crate) erased_own: HashSet<String>,
    /// 声明类型提及本类类型形参的继承字段
    pub(crate) erased_super: HashSet<String>,
    /// wrapper 的 PhantomData 字段声明（泛型类）
    pub(crate) phantom_field: TokenStream2,
    /// wrapper 构造点的 __phantom 初始化 token
    pub(crate) phantom_init: TokenStream2,

    // ── 方法分类 ──────────────────────────────────────────────
    pub(crate) vtable_defines: Vec<&'a FnItem>,
    pub(crate) vtable_overrides: HashMap<String, Vec<&'a FnItem>>,
    /// Constructor + NonVirtual 合并（代码中不分离）
    pub(crate) non_virtual: Vec<&'a FnItem>,
    /// 继承成员声明（祖先声明、本类未覆盖）→ wrapper 上的转发方法
    pub(crate) inherited: Vec<(&'a FnItem, String, Option<String>, bool)>,

    // ── 方法名集合（两个不同语义，不可合并为一个）────────────
    /// 全部方法名，rewrite_virtual_calls_for_wrapper 用
    pub(crate) own_method_names: HashSet<String>,
    /// 仅 VirtualDefine 方法名，§11 base fn 安全检查用
    pub(crate) vtable_define_names: HashSet<String>,
    /// wrapper 方法体经 `this.vtable` 分派的继承虚方法边界规格（A-1 β'）
    pub(crate) vdispatch: HashMap<String, VDispatchSig>,

    // ── 其余输入 ──────────────────────────────────────────────
    pub(crate) fns: &'a [FnItem],
    pub(crate) iface_impls: &'a [InterfaceImpl],
    pub(crate) statics: &'a [StaticItem],
}

impl<'a> GenContext<'a> {
    /// 存储层擦除（A-1 方案 a）：声明类型提及本类类型形参的实例字段（自有 + 继承，
    /// 含声明方宏已 Object 化的 superclass_erased_fields）在 __inner 中以 Object 存储。
    pub(crate) fn is_erased(&self, name: &syn::Ident) -> bool {
        self.erased_own.contains(&name.to_string())
            || self.erased_super.contains(&name.to_string())
            || self.meta.superclass_erased_fields.contains(&name.to_string())
    }

    pub(crate) fn inherited_is_basic(&self, name: &syn::Ident, ty: &syn::Type) -> bool {
        inherited_is_basic(&self.meta, name, ty)
    }

    pub(crate) fn build(
        input: &'a ClassInput,
        meta: ClassMeta,
        gen: syn::Generics,
    ) -> GenContext<'a> {
        let struct_ident = &input.struct_ident;
        // split_for_impl 借用 gen，无法与 gen 同存于 struct —— 经 quote! 预渲染为
        // TokenStream2（token 等价；审计表的字段类型即 TokenStream2）
        let (impl_g, ty_g, where_c) = {
            let (a, b, c) = gen.split_for_impl();
            (quote! { #a }, quote! { #b }, quote! { #c })
        };
        let self_name = struct_ident.to_string();
        let inner_ident = format_ident!("{}__inner", struct_ident);
        let vtable_trait_ident = format_ident!("{}__VTable", struct_ident);
        // wrapper 重建钩子：`Class__VTable::__as_Class(&self) -> Class`。
        // 每个（子）类的 __inner 为全部祖先 VTable 实现该钩子，使祖先方法体能在
        // 「祖先 wrapper 包裹实际 __inner」的上下文中执行（trait default / super 调用）。
        let as_self_hook = format_ident!("__as_{}", struct_ident);

        // ── 字段集合 ─────────────────────────────────────────────────────────────
        let mut basic_names: HashSet<String> = HashSet::new();
        let mut ref_names: HashSet<String> = HashSet::new();
        for (name, ty) in input.fields.iter() {
            if is_basic(ty) {
                basic_names.insert(name.to_string());
            } else {
                ref_names.insert(name.to_string());
            }
        }
        for (name, ty) in meta.superclass_fields.iter() {
            if inherited_is_basic(&meta, name, ty) {
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

        for f in &input.fns {
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

        // ── 存储层擦除（A-1 方案 a）────────────────────────────────────────────
        // 声明类型提及本类类型形参的实例字段（自有 + 继承）在 __inner 中以 Object 存储：
        // JVM 字段存储本就按描述符擦除，__inner 非泛型 → Rc<X__inner> 的 TypeId 与
        // 类型实参无关。访问器签名保持类型化，边界经 From<Object> / Into<Object> 转换。
        // 类级类型参数名（用于 generic_signature 泛型重建与擦除判定）
        let class_type_params: Vec<String> = gen.params.iter().filter_map(|p| {
            if let GenericParam::Type(tp) = p { Some(tp.ident.to_string()) } else { None }
        }).collect();
        let type_param_names: HashSet<String> = class_type_params.iter().cloned().collect();
        let erased_own: HashSet<String> = input.fields.iter()
            .filter(|(_, ty)| mentions_any(ty, &type_param_names))
            .map(|(n, _)| n.to_string())
            .collect();
        let erased_super: HashSet<String> = meta.superclass_fields.iter()
            .filter(|(_, ty)| mentions_any(ty, &type_param_names))
            .map(|(n, _)| n.to_string())
            .collect();
        // 擦除实例化的类型实参（全 Object）：`X<Object, ..., Object>`。Object 满足全部形参
        // bound（宏补的 Clone/Default/'static/From<Object>/Into<Object> 与 Python 侧类上界的
        // `E: Into<B>` 对擦除实参均成立）——需要「不携带本类形参的 impl 上下文」构造 wrapper
        // 时使用（接口 vtable impl、删减形参后的祖先 vtable impl）。
        let erased_ty_args: TokenStream2 = {
            let n = gen.type_params().count();
            if n == 0 {
                quote! {}
            } else {
                let objs = vec![quote! { Object }; n];
                quote! { <#(#objs),*> }
            }
        };
        // 泛型类：方法体统一走 wrapper 钩子（见 classify::vtable_body_kind_gated 的说明）
        let class_is_generic = !class_type_params.is_empty();
        // wrapper 的类型形参不再出现在字段里（vtable 去形参后为非泛型 Rc<dyn X__VTable>）
        // → PhantomData 标记持有形参（与接口载体的 __phantom 同构），E0392 消除
        let phantom_field: TokenStream2 = if class_is_generic {
            let params: Vec<&Ident> = gen.params.iter()
                .filter_map(|p| if let GenericParam::Type(tp) = p { Some(&tp.ident) } else { None })
                .collect();
            quote! { __phantom: ( #( ::std::marker::PhantomData<fn() -> #params>, )* ) }
        } else {
            quote! {}
        };
        let phantom_init: TokenStream2 = if class_is_generic {
            quote! { __phantom: ::std::default::Default::default(), }
        } else {
            quote! {}
        };

        // ── 方法名集合（两个不同语义）────────────────────────────────────────
        // 当前类自有方法名（VirtualDefine + VirtualOverride + NonVirtual）
        // 在 NeedsWrapper 路径中，不在此集合的 this.method() 调用均为继承虚方法，需通过 vtable 访问。
        // 共置 `_impl.rs` 的手写方法同样是 wrapper 上的自有 inherent 方法（impl_methods）。
        let own_method_names: HashSet<String> = input.fns.iter()
            .map(|f| f.sig.ident.to_string())
            .chain(meta.impl_methods.iter().cloned())
            .collect();
        // 本类 VirtualDefine 方法名集合，base 函数安全判断用
        let vtable_define_names: HashSet<String> = vtable_defines.iter()
            .map(|f| f.sig.ident.to_string())
            .collect();
        // wrapper 方法体经 `this.vtable` 分派的继承虚方法边界规格（A-1 β'）：
        // vtable 方法签名已 Object 化 → 调用点按「提及本类形参 / 命中 vtable_erasure 名集」装箱，
        // 返回值 null 容忍还原
        let vdispatch: HashMap<String, VDispatchSig> = inherited.iter()
            .filter(|(.., vo, _)| vo.is_some())
            .map(|(f, ..)| {
                let erasure = erasure_set_of(f, &type_param_names);
                let box_args: Vec<bool> = f.sig.inputs.iter().filter_map(|a| match a {
                    syn::FnArg::Typed(pt) => Some(
                        mentions_any(&pt.ty, &type_param_names)
                        || (!erasure.is_empty()
                            && erasure.contains(&flat_type_tokens(&pt.ty)))),
                    _ => None,
                }).collect();
                let ret_conv: Option<Type> = match &f.sig.output {
                    syn::ReturnType::Type(_, ty) => result_inner_ty(ty).and_then(|inner| {
                        let hit = mentions_any(inner, &type_param_names)
                            || (!erasure.is_empty()
                                && erasure.contains(&flat_type_tokens(inner)));
                        if hit { Some(inner.clone()) } else { None }
                    }),
                    syn::ReturnType::Default => None,
                };
                (f.sig.ident.to_string(), VDispatchSig { box_args, ret_conv })
            })
            .collect();

        GenContext {
            struct_ident,
            self_name,
            inner_ident,
            vtable_trait_ident,
            as_self_hook,
            gen,
            impl_g,
            ty_g,
            where_c,
            class_type_params,
            type_param_names,
            class_is_generic,
            erased_ty_args,
            meta,
            fields: &input.fields,
            basic_names,
            ref_names,
            erased_own,
            erased_super,
            phantom_field,
            phantom_init,
            vtable_defines,
            vtable_overrides,
            non_virtual,
            inherited,
            own_method_names,
            vtable_define_names,
            vdispatch,
            fns: &input.fns,
            iface_impls: &input.iface_impls,
            statics: &input.statics,
        }
    }
}
