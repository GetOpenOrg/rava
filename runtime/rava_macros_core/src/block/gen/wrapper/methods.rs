//! §7 wrapper impl 块：字段访问器委托、虚方法委托、继承成员转发、构造器 / 非虚方法、
//! static 字段存储与类初始化状态机。

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::super::super::class_init;
use super::super::super::erasure::{
    erased_call_args, erased_call_args_with, erased_call_ret_conv, erased_call_ret_conv_with,
    erasure_set_of, expand_non_virtual_fn, prepare_non_virtual_body,
};
use super::super::super::moved::{has_body, is_safe};
use super::super::super::parse::FnItem;
use super::super::super::parse::split_type_name_args;
use super::super::super::rewrite::{
    rewrite_base_calls_for_wrapper, rewrite_block, rewrite_virtual_calls_for_wrapper,
};
use super::super::super::util::{attr_str, strip_meta_attrs};
use super::super::context::GenContext;
use super::body_fns::{functionize, functionize_moved};

/// 返回 (wrapper impl 段, 方法体函数化后的模块级体函数)
pub(super) fn generate(ctx: &GenContext) -> syn::Result<(TokenStream2, Vec<TokenStream2>)> {
    let struct_ident = &ctx.struct_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let binary_name = &ctx.meta.binary_name;

    // ══════════════════════════════════════════════════════════════════════════
    // 7. Impl block on wrapper（字段访问器委托 + 虚方法委托 + 构造器双入口）
    // ══════════════════════════════════════════════════════════════════════════

    let mut wrapper_methods: Vec<TokenStream2> = Vec::new();
    // 方法体函数化后的模块级体函数（wrapper impl 块之后输出）
    let mut body_fns: Vec<TokenStream2> = Vec::new();

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
        // 方法体落在隐藏的 `__impl_<method>`（不分派）；公开的同名方法统一经 vtable 分派，
        // 子类覆盖版本对「父类型 wrapper 上的调用」同样生效。
        if has_body(f) && !is_safe(ctx, f) {
            impl_method(ctx, f, &keep_attrs, &mut wrapper_methods, &mut body_fns)?;
        }

        // vtable 方法签名已 Object 化（A-1）→ 类型化 wrapper 方法与擦除分派之间在
        // 此边界转换（形参装箱 / 返回值还原）
        let conv_args = erased_call_args(sig, &ctx.type_param_names);
        let call = quote! {
            #vtable_trait_ident::#mname(&*self.vtable, #(#conv_args),*)
        };
        let dispatch = erased_call_ret_conv(sig, &ctx.type_param_names, call);
        let null_check = class_init::entry_checks(sig);
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
            if has_body(f) && !is_safe(ctx, f) {
                impl_method(ctx, f, &keep_attrs, &mut wrapper_methods, &mut body_fns)?;
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
            let null_check = class_init::entry_checks(sig);
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
                    // 不占槽的类祖先方法（C3 按分派裁剪）：上转到声明者 wrapper 直接调用；
                    // 声明者视角的方法名（重载改名）与本类视角不同时同样取 `target`
                    let target = attr_str(&f.attrs, "target")
                        .map(|t| Ident::new(&t, proc_macro2::Span::call_site()))
                        .unwrap_or_else(|| mname.clone());
                    quote! {
                        <#owner_ty as ::std::convert::From<Self>>::from(::std::clone::Clone::clone(self))
                            .#target(#(#param_names),*)
                    }
                }
            }
        };
        let null_check = class_init::entry_checks(sig);
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { #null_check #body }
        });
    }

    // Constructor / NonVirtual 方法（保持原 body，走 Rewriter）
    for f in &ctx.non_virtual {
        if f.moved.is_some() {
            // 已下沉：外壳只取签名，空接收者检查同 prepare_non_virtual_body
            let keep_attrs = strip_meta_attrs(&f.attrs);
            let vis = &f.vis;
            let fz = functionize_moved(ctx, &quote! { #(#keep_attrs)* }, &quote! { #vis }, &f.sig,
                                       &f.sig.ident, &class_init::entry_checks(&f.sig))?;
            wrapper_methods.push(fz.shell);
            body_fns.push(fz.body_fn);
            continue;
        }
        let fz = prepare_non_virtual_body(f, &ctx.basic_names, &ctx.ref_names).and_then(|(null_check, b)| {
            let keep_attrs = strip_meta_attrs(&f.attrs);
            let vis = &f.vis;
            functionize(ctx, &quote! { #(#keep_attrs)* }, &quote! { #vis }, &f.sig, &f.sig.ident,
                        &b.stmts, &null_check)
        });
        match fz {
            Some(fz) => {
                wrapper_methods.push(fz.shell);
                body_fns.push(fz.body_fn);
            }
            None => wrapper_methods.push(
                expand_non_virtual_fn(f, &ctx.meta.binary_name, &ctx.basic_names, &ctx.ref_names)),
        }
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

    Ok((wrapper_impl, body_fns))
}

/// 需要 wrapper 上下文的虚方法体：落在隐藏的 `__impl_<method>`（函数化为体函数 + 外壳，
/// 不适用时内联）。已下沉的体只生成外壳（标注保证有体时函数化成立）。
fn impl_method(
    ctx: &GenContext,
    f: &FnItem,
    keep_attrs: &[&syn::Attribute],
    wrapper_methods: &mut Vec<TokenStream2>,
    body_fns: &mut Vec<TokenStream2>,
) -> syn::Result<()> {
    let sig = &f.sig;
    let mname = &sig.ident;
    let mut impl_sig = sig.clone();
    impl_sig.ident = format_ident!("__impl_{}", mname);
    let attrs = quote! { #(#keep_attrs)* #[doc(hidden)] };
    let Some(block) = &f.block else {
        let fz = functionize_moved(ctx, &attrs, &quote! { pub }, &impl_sig, mname, &quote! {})?;
        wrapper_methods.push(fz.shell);
        body_fns.push(fz.body_fn);
        return Ok(());
    };
    let mut b = block.clone();
    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
    rewrite_base_calls_for_wrapper(&mut b);
    rewrite_virtual_calls_for_wrapper(&mut b, &ctx.own_method_names, &ctx.vdispatch);
    match functionize(ctx, &attrs, &quote! { pub }, &impl_sig, mname, &b.stmts, &quote! {}) {
        Some(fz) => {
            wrapper_methods.push(fz.shell);
            body_fns.push(fz.body_fn);
        }
        None => wrapper_methods.push(quote! { #attrs pub #impl_sig #b }),
    }
    Ok(())
}
