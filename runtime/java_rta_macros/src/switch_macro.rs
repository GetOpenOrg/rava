//! `java_switch!` 块级宏 — Java switch 语义到 Rust 的转译。
//!
//! # 三种展开模式
//!
//! 1. **整数/enum switch（expression body）** — `1 => expr` 形式，直接转为 Rust `match`
//! 2. **整数/enum switch（block body，可含 fallthrough）** — `1 => { ... }` 形式，
//!    展开为 if-chain + `__fall` 标志；codegen 向不含 fallthrough 的 block 末尾插入
//!    `__fall = false;` 作为 `break` 等价物
//! 3. **String switch** — `s: String;` 类型注释触发，展开为 `.equals()` 链
//!
//! # 宏语法
//!
//! ```text
//! java_switch! { <scrutinee>[: Type];
//!     <pat1> [| <pat2>] => <expr_or_block>,
//!     ...
//!     _ => <expr_or_block>,
//! }
//! ```
//!
//! arm 之间逗号可选（block body 后通常省略逗号）。

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    Block, Expr, Ident, Lit, Token,
};

// ══════════════════════════════════════════════════════════════════════════════
// 输入 AST
// ══════════════════════════════════════════════════════════════════════════════

/// arm body 类型
enum ArmBody {
    /// 表达式 body（无 fallthrough，情形 1）
    Expr(Expr),
    /// 块 body（可含 fallthrough，情形 2/3）
    Block(Block),
}

/// 单个 switch arm
struct SwitchArm {
    /// or-patterns（每项为已含负号的 token 流）
    pats: Vec<TokenStream2>,
    /// 是否为 wildcard（`_`）arm
    is_wildcard: bool,
    body: ArmBody,
}

/// `java_switch!` 的完整输入
struct SwitchInput {
    scrutinee: Expr,
    /// 是否为 String switch（带 `: String` 类型注释）
    is_string: bool,
    arms: Vec<SwitchArm>,
}

impl Parse for SwitchInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // ── scrutinee 解析 ────────────────────────────────────────────────────
        //
        // 支持两种格式：
        //   普通：`expr;`
        //   String switch：`ident: String;`（或其他类型，但仅 String 特殊处理）
        //
        // 探测方式：peek(Ident) && peek2(:)，若成立则 fork 尝试 `ident: Type;`。
        // 若 fork 成功则 advance_to，否则回退到普通表达式解析。

        let scrutinee: Expr;
        let is_string: bool;

        if input.peek(Ident) && input.peek2(Token![:]) {
            // 尝试 `ident: Type ;` 格式
            let fork = input.fork();
            let id: Ident = fork.parse()?;
            let _colon: Token![:] = fork.parse()?;
            match fork.parse::<syn::Type>() {
                Ok(ty) if fork.peek(Token![;]) => {
                    // 确认是类型注释格式
                    let _: Token![;] = fork.parse()?;
                    let ty_str = quote!(#ty).to_string().replace(' ', "");
                    is_string = ty_str == "String";
                    input.advance_to(&fork);
                    // scrutinee 是 ident 本身（变量名）
                    scrutinee = syn::parse_str::<Expr>(&id.to_string())?;
                }
                _ => {
                    // 解析类型失败或后面没有 `;`，回退到普通表达式
                    scrutinee = input.parse()?;
                    let _: Token![;] = input.parse()?;
                    is_string = false;
                }
            }
        } else {
            // 普通表达式 scrutinee
            scrutinee = input.parse()?;
            let _: Token![;] = input.parse()?;
            is_string = false;
        }

        // ── arms 解析 ──────────────────────────────────────────────────────────

        let mut arms: Vec<SwitchArm> = Vec::new();

        while !input.is_empty() {
            // 解析 or-patterns（`|` 分隔）
            let mut pats: Vec<TokenStream2> = Vec::new();
            let mut is_wildcard = false;

            loop {
                let (pat_ts, pat_wild) = parse_single_pat(input)?;
                if pat_wild {
                    is_wildcard = true;
                }
                pats.push(pat_ts);

                // 遇到 `|` 则继续解析下一个 pattern
                if input.peek(Token![|]) {
                    input.parse::<Token![|]>()?;
                } else {
                    break;
                }
            }

            // `=>`
            input.parse::<Token![=>]>()?;

            // body：块或表达式
            let body = if input.peek(syn::token::Brace) {
                ArmBody::Block(input.parse()?)
            } else {
                // parse_without_eager_brace 防止贪婪吃掉后续块
                ArmBody::Expr(Expr::parse_without_eager_brace(input)?)
            };

            arms.push(SwitchArm { pats, is_wildcard, body });

            // 可选分隔逗号（expression body 后通常有逗号，block body 后通常省略）
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(SwitchInput { scrutinee, is_string, arms })
    }
}

/// 解析单个 pattern token：`_`、字面量、负整数。
/// 返回 `(token_stream, is_wildcard)`。
fn parse_single_pat(input: ParseStream) -> syn::Result<(TokenStream2, bool)> {
    if input.peek(Token![_]) {
        input.parse::<Token![_]>()?;
        Ok((quote! { _ }, true))
    } else if input.peek(Token![-]) {
        // 负整数字面量（如 `-1`）
        input.parse::<Token![-]>()?;
        let lit: Lit = input.parse()?;
        Ok((quote! { -#lit }, false))
    } else {
        let lit: Lit = input.parse()?;
        Ok((quote! { #lit }, false))
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 展开入口
// ══════════════════════════════════════════════════════════════════════════════

pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<SwitchInput>(input) {
        Ok(si) => expand_inner(si),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(si: SwitchInput) -> TokenStream2 {
    let SwitchInput { scrutinee, is_string, arms } = si;

    if is_string {
        // 情形 3：String switch → equals 链
        return expand_string_switch(scrutinee, arms);
    }

    // 根据 body 类型决定策略
    let all_expr_body = arms.iter().all(|a| matches!(a.body, ArmBody::Expr(_)));

    if all_expr_body {
        // 情形 1：直接转为 Rust match
        expand_direct_match(scrutinee, arms)
    } else {
        // 情形 2：if-chain（支持 fallthrough）
        expand_fallthrough(scrutinee, arms)
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 情形 1：直接 Rust match（所有 body 均为表达式）
// ══════════════════════════════════════════════════════════════════════════════

fn expand_direct_match(scrutinee: Expr, arms: Vec<SwitchArm>) -> TokenStream2 {
    let arms_ts: Vec<TokenStream2> = arms
        .iter()
        .map(|arm| {
            // or-pattern：`pat1 | pat2 | ...`
            let pat = pats_to_token(&arm.pats);
            let body = match &arm.body {
                ArmBody::Expr(e) => quote! { #e },
                ArmBody::Block(b) => quote! { #b },
            };
            quote! { #pat => #body }
        })
        .collect();

    quote! {
        match #scrutinee {
            #(#arms_ts,)*
        }
    }
}

/// 将 or-patterns token 列表拼成 `p1 | p2 | ...`。
fn pats_to_token(pats: &[TokenStream2]) -> TokenStream2 {
    quote! { #(#pats)|* }
}

// ══════════════════════════════════════════════════════════════════════════════
// 情形 2：fallthrough if-chain（任一 arm 含 block body）
//
// 展开结构：
//   let __sw   = scrutinee;
//   let mut __fall    = false;   // fallthrough 标志
//   let mut __matched = false;   // 是否有 arm 匹配过
//
//   if __sw == N || __fall { __fall = true; __matched = true; <block>; }
//   ...
//   if !__matched || __fall { <default>; }   // wildcard arm
//
// codegen 向「含 break」的 block 末尾插入 `__fall = false;`（break 等价物）。
// ══════════════════════════════════════════════════════════════════════════════

fn expand_fallthrough(scrutinee: Expr, arms: Vec<SwitchArm>) -> TokenStream2 {
    let sw = Ident::new("__sw", Span::call_site());
    let fall = Ident::new("__fall", Span::call_site());
    let matched = Ident::new("__matched", Span::call_site());

    let mut stmts: Vec<TokenStream2> = Vec::new();

    for arm in &arms {
        let body_stmts = inline_body(&arm.body);

        if arm.is_wildcard {
            // default arm：未匹配或 fallthrough 时执行
            stmts.push(quote! {
                if !#matched || #fall {
                    #body_stmts
                }
            });
        } else {
            // 普通 arm：值匹配 或 fallthrough 时执行
            let conds: Vec<TokenStream2> = arm
                .pats
                .iter()
                .map(|p| quote! { #sw == #p })
                .collect();

            stmts.push(quote! {
                if #(#conds)||* || #fall {
                    #fall = true;
                    #matched = true;
                    #body_stmts
                }
            });
        }
    }

    quote! {
        {
            let #sw = #scrutinee;
            let mut #fall = false;
            let mut #matched = false;
            #(#stmts)*
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 情形 3：String switch → equals 链
//
// Java String switch 语义：先 hashCode 再 equals。
// 此处简化为直接 equals 链；hashCode 预筛选作为后续优化留 TODO。
//
// 展开结构：
//   { let __s = &scrutinee;
//     if __s.equals("a") { ... }
//     else if __s.equals("b") { ... }
//     else { /* default */ }
//   }
// ══════════════════════════════════════════════════════════════════════════════

fn expand_string_switch(scrutinee: Expr, arms: Vec<SwitchArm>) -> TokenStream2 {
    let s = Ident::new("__s", Span::call_site());

    // 分离 wildcard arm 与 regular arms
    let (regular, default): (Vec<&SwitchArm>, Vec<&SwitchArm>) =
        arms.iter().partition(|a| !a.is_wildcard);

    // default else 分支
    let default_else: TokenStream2 = default
        .first()
        .map(|d| {
            let body = inline_body(&d.body);
            quote! { else { #body } }
        })
        .unwrap_or_default();

    // 无 regular arms：直接输出 default
    if regular.is_empty() {
        let body = default
            .first()
            .map(|d| inline_body(&d.body))
            .unwrap_or_default();
        return quote! { { #body } };
    }

    // 构建 if / else-if 链
    let mut chain: Vec<TokenStream2> = Vec::new();
    for (i, arm) in regular.iter().enumerate() {
        let conds: Vec<TokenStream2> = arm
            .pats
            .iter()
            .map(|p| quote! { #s.equals(#p) })
            .collect();
        let cond = quote! { #(#conds)||* };
        let body = inline_body(&arm.body);

        if i == 0 {
            chain.push(quote! { if #cond { #body } });
        } else {
            chain.push(quote! { else if #cond { #body } });
        }
    }

    quote! {
        {
            let #s = &#scrutinee;
            #(#chain)*
            #default_else
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 辅助工具
// ══════════════════════════════════════════════════════════════════════════════

/// 将 arm body 内联为语句序列 token 流（去掉最外层 `{ }` 包裹）。
fn inline_body(body: &ArmBody) -> TokenStream2 {
    match body {
        ArmBody::Expr(e) => quote! { #e },
        ArmBody::Block(b) => {
            let stmts = &b.stmts;
            quote! { #(#stmts)* }
        }
    }
}
