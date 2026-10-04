//! vtable trait 声明（字段访问器抽象方法 + 虚方法缺省实现）。

use super::*;

/// §1 VTable trait —— 非泛型（A-1 去形参：与接口载体 `I__VTable` 对齐）。
pub(crate) fn vtable_trait(ctx: &GenContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let as_self_hook = &ctx.as_self_hook;
    let erased_ty_args = &ctx.erased_ty_args;
    let dyn_view_trait = format_ident!("{}__AsVTable", struct_ident);
    let dyn_view_hook = format_ident!("__dyn_{}", struct_ident);

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
        if has_body(f) {
            if is_safe(ctx, f) {
                // vtable-safe 方法体（仅非泛型类）→ default 委托 base 函数
                // （体可直接在 vtable 视图上运行）：本类 vtable 的 `&dyn` 视图经
                // `__dyn_<X>` 取得（缺省方法的 Self 可能非 Sized，不能直接 unsize）
                vtable_default_methods.push(quote! {
                    #erased_default_sig {
                        #fn_base_name(#dyn_view_trait::#dyn_view_hook(self), #(#param_names_for_default),*)
                    }
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
                #erased_default_sig { __stub(#msg) }
            });
        }
    }

    // 本类 vtable 的 `&dyn` 视图（仅 Safe 缺省方法需要）：独立 trait + 覆盖全部 Sized 实现类型的
    // 一揽子 impl，作为本类 vtable trait 的 supertrait——生成的与手写的实现类型都自动具备，
    // `dyn` 对象经 supertrait 槽位取得同一视图。
    let needs_dyn_view = ctx.vtable_defines.iter().any(|f| has_body(f) && is_safe(ctx, f));
    let (dyn_view_decl, dyn_view_bound) = if needs_dyn_view {
        (quote! {
            #[allow(non_camel_case_types)]
            pub trait #dyn_view_trait {
                fn #dyn_view_hook(&self) -> &dyn #vtable_trait_ident;
            }
            impl<__T: #vtable_trait_ident> #dyn_view_trait for __T {
                #[inline]
                fn #dyn_view_hook(&self) -> &dyn #vtable_trait_ident { self }
            }
        }, quote! { + #dyn_view_trait })
    } else {
        (quote! {}, quote! {})
    };

    let vtable_trait = quote! {
        #dyn_view_decl
        #[allow(non_camel_case_types)]
        pub trait #vtable_trait_ident: #vtable_supertrait #dyn_view_bound {
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args;
            #(#vtable_abstract_methods)*
            #(#vtable_default_methods)*
        }
    };

    vtable_trait
}
