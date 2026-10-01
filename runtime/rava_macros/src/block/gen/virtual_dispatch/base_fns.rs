//! `ClassName__method_base` 自由函数（invokespecial super 调用的精确目标）。

use super::*;

/// §11 自由函数 ClassName__methodName_base（供 invokespecial super() 调用）。
pub(crate) fn base_fns(ctx: &GenContext) -> TokenStream2 {
    let vtable_trait_ident = &ctx.vtable_trait_ident;

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
                        __stub(#msg)
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


    quote! { #(#base_fns)* }
}
