//! 方法体函数化（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.3 / §7.5 S3）：
//! wrapper 固有方法的体移入模块级非泛型自由函数 `__jbm_<类>__<方法>`，wrapper 方法变为
//! 转发外壳。拆层后体函数随存储层进实现 crate，声明层外壳改经 extern 声明调用。
//!
//! 等价性：体函数的语句就是原方法体（宏的各项改写之后），只做两处纯改名——
//! - 接收者 `&self` → 形参 `this: &X`：体首句 `let this = self;` 去掉（形参即 `this`），
//!   其余 `self` 记号（非路径前缀 `self::`）改名为 `this`；
//! - `Self` → `X`（非泛型类，二者是同一类型）。
//! 外壳按原实参顺序转发，返回值原样交回；入口检查（空接收者 / 栈界）在体函数首句；
//! 求值顺序与副作用不变。
//!
//! 适用范围：非泛型类、方法自身无泛型形参、接收者为 `&self` 或无接收者、形参均为标识符
//! 模式。泛型类的体随 S6 擦除核心进实现层。体内含嵌套 `impl` 项（其中 `Self` 另有所指）
//! 或在 `&self` 方法里另有名为 `this` 的绑定时，改名不再保义，保留内联。

use proc_macro2::{Group, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::Ident;

use super::super::context::GenContext;

/// 体函数名：`__jbm_<类>__<方法>`（与存储钩子 `__jb_<类>__<用途>` 分开命名空间）
pub(super) fn body_fn_ident(ctx: &GenContext, method: &Ident) -> Ident {
    format_ident!("__jbm_{}__{}", ctx.struct_ident, method)
}

fn this_ident() -> Ident {
    format_ident!("this")
}

/// 函数化结果：模块级体函数 + wrapper 内的转发外壳
pub(super) struct Functionized {
    pub(super) body_fn: TokenStream2,
    pub(super) shell: TokenStream2,
}

/// 去全部空白的文本（编译器记号流的文本化在 `;` 前不留空格，与 proc_macro2 回退实现不同）
fn flat(ts: &TokenStream2) -> String {
    ts.to_string().split_whitespace().collect()
}

fn mentions_ident(ts: &TokenStream2, name: &str) -> bool {
    ts.clone().into_iter().any(|tt| match tt {
        TokenTree::Ident(i) => i == name,
        TokenTree::Group(g) => mentions_ident(&g.stream(), name),
        _ => false,
    })
}

/// 记号级改名：`from` 标识符 → `to`。`skip_path_prefix` 时跳过后随 `::` 的出现
/// （`self::x` 是模块路径，不是接收者）。递归进入分组（含宏调用的记号流）。
fn rename_ident(ts: TokenStream2, from: &str, to: &Ident, skip_path_prefix: bool) -> TokenStream2 {
    let tts: Vec<TokenTree> = ts.into_iter().collect();
    let mut out: Vec<TokenTree> = Vec::with_capacity(tts.len());
    for (i, tt) in tts.iter().enumerate() {
        match tt {
            TokenTree::Ident(id) if id == from => {
                let path_prefix = skip_path_prefix && matches!(
                    (tts.get(i + 1), tts.get(i + 2)),
                    (Some(TokenTree::Punct(a)), Some(TokenTree::Punct(b)))
                        if a.as_char() == ':' && a.spacing() == Spacing::Joint && b.as_char() == ':');
                if path_prefix {
                    out.push(tt.clone());
                } else {
                    let mut r = to.clone();
                    r.set_span(id.span());
                    out.push(TokenTree::Ident(r));
                }
            }
            TokenTree::Group(g) => {
                let mut ng = Group::new(g.delimiter(), rename_ident(g.stream(), from, to, skip_path_prefix));
                ng.set_span(g.span());
                out.push(TokenTree::Group(ng));
            }
            other => out.push(other.clone()),
        }
    }
    out.into_iter().collect()
}

/// 签名形态：(有 `&self` 接收者, 非 self 形参名)；不适用函数化时 None
fn shape(ctx: &GenContext, sig: &syn::Signature) -> Option<(bool, Vec<Ident>)> {
    if ctx.class_is_generic || !sig.generics.params.is_empty() || sig.generics.where_clause.is_some() {
        return None;
    }
    let mut has_recv = false;
    let mut idents: Vec<Ident> = Vec::new();
    for a in sig.inputs.iter() {
        match a {
            syn::FnArg::Receiver(r) => {
                if r.reference.is_none() || r.mutability.is_some() || r.colon_token.is_some() {
                    return None;
                }
                if r.reference.as_ref().is_some_and(|(_, lt)| lt.is_some()) {
                    return None;
                }
                has_recv = true;
            }
            syn::FnArg::Typed(pt) => {
                let syn::Pat::Ident(pi) = &*pt.pat else { return None };
                if pi.by_ref.is_some() || pi.subpat.is_some() {
                    return None;
                }
                idents.push(pi.ident.clone());
            }
        }
    }
    Some((has_recv, idents))
}

/// 体语句去掉首句 `let this = self;`（有接收者时）后的记号流；改名不保义时 None
fn body_tokens(has_recv: bool, stmts: &[syn::Stmt]) -> Option<TokenStream2> {
    let mut stmts: Vec<&syn::Stmt> = stmts.iter().collect();
    if has_recv {
        if stmts.first().is_some_and(|s| flat(&quote!(#s)) == "letthis=self;") {
            stmts.remove(0);
        }
    }
    let body: TokenStream2 = quote! { #(#stmts)* };
    if mentions_ident(&body, "impl") || (has_recv && mentions_ident(&body, "this")
        && mentions_ident(&body, "self")) {
        return None;
    }
    Some(body)
}

/// 已完成宏改写的方法能否函数化（与 [`functionize`] 同一判据；剥体标注用）
pub(crate) fn functionize_applicable(ctx: &GenContext, sig: &syn::Signature, stmts: &[syn::Stmt]) -> bool {
    shape(ctx, sig).is_some_and(|(has_recv, _)| body_tokens(has_recv, stmts).is_some())
}

/// 把一个已完成宏改写的 wrapper 方法（签名 + 体语句）拆成体函数 + 外壳；不适用时 None。
/// `fn_name` 是 wrapper 上的方法名（外壳用），`body_name` 是体函数名后缀（方法名）。
/// `prologue` 是入口检查语句（空接收者 / 栈界），落在体函数首句。
pub(super) fn functionize(
    ctx: &GenContext,
    attrs: &TokenStream2,
    vis: &TokenStream2,
    sig: &syn::Signature,
    body_name: &Ident,
    stmts: &[syn::Stmt],
    prologue: &TokenStream2,
) -> Option<Functionized> {
    let (has_recv, idents) = shape(ctx, sig)?;
    let body = body_tokens(has_recv, stmts)?;
    let body = if has_recv { rename_ident(body, "self", &this_ident(), true) } else { body };
    let body = rename_ident(body, "Self", &ctx.struct_ident, false);
    Some(build(ctx, attrs, vis, sig, body_name, body, prologue, has_recv, &idents))
}

/// 已下沉方法（声明层，体不在本文本）的函数化：体函数只用于拆层取外壳（体为空），
/// 外壳与有体时逐记号相同——二者都只取签名。标注保证有体时函数化成立。
pub(super) fn functionize_moved(
    ctx: &GenContext,
    attrs: &TokenStream2,
    vis: &TokenStream2,
    sig: &syn::Signature,
    body_name: &Ident,
    prologue: &TokenStream2,
) -> syn::Result<Functionized> {
    let (has_recv, idents) = shape(ctx, sig).ok_or_else(|| syn::Error::new_spanned(
        &sig.ident, "rava_moved：方法签名不适用函数化，体不能下沉"))?;
    Ok(build(ctx, attrs, vis, sig, body_name, TokenStream2::new(), prologue, has_recv, &idents))
}

#[allow(clippy::too_many_arguments)]
fn build(
    ctx: &GenContext,
    attrs: &TokenStream2,
    vis: &TokenStream2,
    sig: &syn::Signature,
    body_name: &Ident,
    body: TokenStream2,
    prologue: &TokenStream2,
    has_recv: bool,
    idents: &[Ident],
) -> Functionized {
    let struct_ident = &ctx.struct_ident;

    // 体函数签名：接收者 → `this: &X`；其余形参（含 mut）与返回类型原样，`Self` → `X`
    let fn_ident = body_fn_ident(ctx, body_name);
    let params: Vec<TokenStream2> = sig.inputs.iter().map(|a| match a {
        syn::FnArg::Receiver(_) => quote! { this: &#struct_ident },
        syn::FnArg::Typed(pt) => quote! { #pt },
    }).collect();
    let output = &sig.output;
    let head = rename_ident(quote! { (#(#params),*) #output }, "Self", struct_ident, false);
    // 入口检查（空接收者 / 栈界）随体进体函数首句：体函数的唯一调用方是外壳，外壳在检查与
    // 转发之间无其他求值，次序不变；外壳因此只剩一次调用（无 `?` 展开），声明层每个外壳的
    // MIR 与借用检查量随之减小，检查的展开落在实现 crate
    let prologue = if has_recv { rename_ident(prologue.clone(), "self", &this_ident(), true) } else { prologue.clone() };
    let body_fn = quote! {
        #[allow(non_snake_case, unused_mut, unused_variables)]
        pub fn #fn_ident #head {
            #prologue
            #body
        }
    };

    // 外壳：形参去 mut，按原顺序转发
    let mut shell_sig = sig.clone();
    for a in shell_sig.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = a {
            if let syn::Pat::Ident(pi) = &mut *pt.pat {
                pi.mutability = None;
            }
        }
    }
    let recv_arg = if has_recv { quote! { self, } } else { quote! {} };
    let shell = quote! {
        #attrs
        #vis #shell_sig { #fn_ident(#recv_arg #(#idents),*) }
    };
    Functionized { body_fn, shell }
}
