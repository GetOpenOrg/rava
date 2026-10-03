//! `ClassName__method_base` 自由函数（invokespecial super 调用的精确目标）。
//!
//! 接收者统一为 `this: &dyn X__VTable`（不再按调用方类型单态化）：base 体只经本类 vtable
//! trait 及其 supertrait 链访问 `this`，`&dyn` 上调的是同一组 trait 方法、命中同一实现。
//! 调用方传 `&X__inner`（unsize）、`&dyn Sub__VTable`（trait upcasting）或 `&*w.vtable`。

use super::*;

/// vtable-safe 方法体的 base 形态：`rewrite_block` 后去掉首句 `let this = self;`
/// （base 的形参就是 `this`）。只剥这一确切语句，其余语句原样保留。
pub(super) fn base_body(ctx: &GenContext, block: &syn::Block) -> syn::Block {
    let mut b = block.clone();
    ctx.rewrite_vtable_body(&mut b);
    if let Some(first) = b.stmts.first() {
        // 去全部空白后比较（编译器记号流的文本化在 `;` 前不留空格，与 proc_macro2 回退实现不同）
        let fs: String = quote!(#first).to_string().split_whitespace().collect();
        if fs == "letthis=self;" {
            b.stmts.remove(0);
        }
    }
    b
}

/// VirtualDefine 的 vtable-safe 方法体在 base 函数里是否保留真实体（否则为精确报告的
/// stub；分类为 Safe 时不会出现，判定与 base 函数生成同一谓词）。
pub(crate) fn define_base_has_body(ctx: &GenContext, f: &FnItem) -> bool {
    if f.moved.is_some() {
        // 已下沉：结论即标注（标注由本函数对完整体求得，见 moved::moved_fact）
        return f.moved == Some(super::super::super::moved::Moved::Safe);
    }
    let Some(block) = &f.block else { return false };
    if !matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
        return false;
    }
    let mut b = base_body(ctx, block);
    replace_clone_this_in_ok(&mut b);
    let bs_flat: String = quote!(#b).to_string().split_whitespace().collect::<Vec<_>>().join(" ");
    let has_bare_clone_this = bs_flat.contains("Clone :: clone (this)")
        || bs_flat.contains("Clone :: clone(this)")
        || bs_flat.contains("Clone::clone (this)")
        || bs_flat.contains("Clone::clone(this)");
    // this.method() 中是否有非 vtable / 非 accessor 方法
    let parts: Vec<&str> = bs_flat.split("this .").chain(bs_flat.split("this.")).skip(1).collect();
    let has_non_vtable_call = parts.iter().any(|part| {
        let trimmed = part.trim_start();
        let mname: String = trimmed.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        !mname.is_empty()
            && !mname.starts_with("__")
            && trimmed[mname.len()..].trim_start().starts_with('(')
            && !ctx.vtable_define_names.contains(&mname)
    });
    let has_self_ref = bs_flat.contains("Self ::") || bs_flat.contains("Self::");
    !(has_bare_clone_this || has_non_vtable_call || has_self_ref)
}

/// 生成 base 函数的 VirtualOverride 条目（按 vtable_overrides 有序表遍历，同名只取首个，
/// 与 VirtualDefine 同名者让位）。inner_impls 据此判断覆盖体能否改调 base 函数。
pub(super) fn override_base_owners<'c>(ctx: &'c GenContext) -> Vec<&'c FnItem> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for override_fns in ctx.vtable_overrides.values() {
        for f in override_fns {
            if !has_body(f) {
                continue;
            }
            let name = f.sig.ident.to_string();
            if ctx.vtable_define_names.contains(&name) || !seen.insert(name) {
                continue;
            }
            out.push(*f);
        }
    }
    out
}

/// base 函数名
pub(super) fn base_fn_ident(ctx: &GenContext, f: &FnItem) -> Ident {
    format_ident!("{}__{}_base", ctx.self_name, f.sig.ident)
}

/// 一个 base 函数项；`decl_only` = 带类 / 方法泛型或体为存根（拆层时留在声明层直接定义，
/// 其余拆入实现层）
pub(crate) struct BaseFn {
    pub(crate) item: TokenStream2,
    pub(crate) decl_only: bool,
}

/// §11 自由函数 ClassName__methodName_base（供 invokespecial super() 调用）。
pub(crate) fn base_fns(ctx: &GenContext) -> Vec<BaseFn> {
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let mut base_fns: Vec<BaseFn> = Vec::new();

    // 泛型 = 类泛型 + 方法自身 where 子句（方法体依赖类型变量上界约束）
    let generics_of = |sig: &syn::Signature| {
        let mut g = ctx.gen.clone();
        if let Some(method_where) = &sig.generics.where_clause {
            g.make_where_clause().predicates.extend(method_where.predicates.iter().cloned());
        }
        g
    };
    let emit = |sig: &syn::Signature, fn_name: &Ident, body: TokenStream2, stub: bool| -> BaseFn {
        let non_self_params: Vec<_> = sig.inputs.iter()
            .filter(|a| matches!(a, syn::FnArg::Typed(_)))
            .collect();
        let ret = &sig.output;
        let g = generics_of(sig);
        let generic = !g.params.is_empty()
            || g.where_clause.as_ref().is_some_and(|w| !w.predicates.is_empty());
        let (impl_g, _, where_c) = g.split_for_impl();
        let item = quote! {
            #[doc(hidden)]
            #[allow(non_snake_case, unused_variables)]
            pub fn #fn_name #impl_g (this: &dyn #vtable_trait_ident #(, #non_self_params)*) #ret #where_c {
                #body
            }
        };
        BaseFn { item, decl_only: generic || stub }
    };

    // VirtualDefine：
    //   - 需要 wrapper 上下文 → 经钩子重建本类擦除实例化 wrapper，执行 `__impl_<method>`
    //     （签名保持类型化，钩子返回擦除实例化 → 形参 / 返回值在边界转换，A-1 去形参）
    //   - vtable-safe → 真实体（只有字段访问器 / vtable 方法调用；UFCS 消同名歧义 E0034）
    //   - 其余（Safe 分类下不会出现）→ stub 精确报告
    for f in &ctx.vtable_defines {
        if !has_body(f) {
            continue;
        }
        let sig = &f.sig;
        let fn_name = base_fn_ident(ctx, f);
        let mut stub = false;
        let body = if !is_safe(ctx, f) {
            erased_hook_call(
                sig, &format_ident!("__impl_{}", sig.ident), &ctx.vtable_trait_ident,
                &ctx.as_self_hook, &ctx.type_param_names)
        } else if define_base_has_body(ctx, f) {
            match &f.block {
                Some(block) => {
                    let mut b = base_body(ctx, block);
                    replace_clone_this_in_ok(&mut b);
                    rewrite_vtable_calls_ufcs_for_base(&mut b, &ctx.vtable_define_names, &ctx.vtable_trait_ident);
                    let stmts = &b.stmts;
                    quote! { #(#stmts)* }
                }
                // 已下沉：声明层只取外壳（拆层只用签名），体在实现层
                None => quote! {},
            }
        } else {
            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
            let msg = format!("stub: super {}.{}:{}", ctx.meta.binary_name, sig.ident, desc);
            stub = true;
            quote! { __stub(#msg) }
        };
        base_fns.push(emit(sig, &fn_name, body, stub));
    }

    // VirtualOverride（`super.m()` 的精确目标，不分派）：
    //   - vtable-safe 方法体（只有字段访问器）→ 直接在 `this` 上执行
    //   - 其余 → 经钩子重建本类 wrapper，执行 wrapper 上的 `__impl_<method>`
    for f in override_base_owners(ctx) {
        let sig = &f.sig;
        let body: TokenStream2 = if is_safe(ctx, f) {
            match &f.block {
                Some(block) => {
                    let b = base_body(ctx, block);
                    let stmts = &b.stmts;
                    quote! { #(#stmts)* }
                }
                // 已下沉：声明层只取外壳
                None => quote! {},
            }
        } else {
            erased_hook_call(
                sig, &format_ident!("__impl_{}", sig.ident),
                &ctx.vtable_trait_ident, &ctx.as_self_hook, &ctx.type_param_names)
        };
        base_fns.push(emit(sig, &base_fn_ident(ctx, f), body, false));
    }

    base_fns
}

/// 外壳调用：`X__m_base(self, a, b, ..)`，并把签名形参的 `mut` 去掉（体已搬进 base 函数，
/// 外壳只转发）。形参出现非标识符模式时返回 None（调用方保留内联体）。
pub(super) fn base_shell_call(
    ctx: &GenContext, f: &FnItem, sig: &mut syn::Signature,
) -> Option<TokenStream2> {
    let mut idents: Vec<Ident> = Vec::new();
    for a in sig.inputs.iter() {
        if let syn::FnArg::Typed(pt) = a {
            let syn::Pat::Ident(pi) = &*pt.pat else { return None };
            idents.push(pi.ident.clone());
        }
    }
    for a in sig.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = a {
            if let syn::Pat::Ident(pi) = &mut *pt.pat {
                pi.mutability = None;
            }
        }
    }
    let base = base_fn_ident(ctx, f);
    Some(quote! { #base(self #(, #idents)*) })
}
