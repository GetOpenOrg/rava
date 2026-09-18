//! `java_try! { try { ... } catch (e: T) { ... } }` — 封装 Java try/catch 语义。
//!
//! ## 可读层形式
//!
//! ```ignore
//! java_try! {
//!     try {
//!         System::out()?.println_i(Self::divide(10, 0)?)?;
//!     } catch (e: ArithmeticException) {
//!         System::out()?.println(e.getMessage()?)?;
//!     } catch (e: IllegalStateException | IllegalArgumentException as RuntimeException) {
//!         ...
//!     }
//! }
//! ```
//!
//! ## 语义（JVMS §2.10 异常表匹配）
//!
//! - try 体内任何异常完成（`expr?` 与 `return Err(x)`，即 athrow）都不离开方法，
//!   而是转入按声明顺序匹配的 catch 子句；匹配依据是异常对象的**运行时类**
//!   （含子类），由 `JvmError::is_instance_of(<T>::BINARY_NAME)` 判定。
//! - 无子句匹配时异常原样继续向外传播（外层 `java_try!` 或方法调用者）。
//! - 不受本层异常表覆盖的代码（javac 内联的 finally 副本、try 语句之后的代码）由生成器
//!   放在 `java_try!` 之外，try 体内的每条语句都受本层保护。
//! - try 体中的 `return Ok(v)`、`break`、`continue` 保持 Java 语义
//!   （后两者经流程码转发到 try 之外的循环）。
//!
//! ## 展开形式
//!
//! ```ignore
//! {
//!     let __java_try_r: Result<u8, JvmError> = 'java_try_N: { BODY'; Ok(0) };
//!     match __java_try_r {
//!         Ok(1) => break, Ok(2) => continue, Ok(_) => {}
//!         Err(__thrown) => {
//!             if __thrown.is_instance_of(<T>::BINARY_NAME) { let e: T = ...; HANDLER }
//!             else { return Err(__thrown); }
//!         }
//!     }
//! }
//! ```
//! 其中 BODY' 是把 `expr?` 改写为 `break 'java_try_N Err(..)` 后的 try 体。
//! 闭包体不改写（闭包内的 `?` 属于闭包自身的返回）。

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::sync::atomic::{AtomicUsize, Ordering};
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::{self, VisitMut};
use syn::{Block, Expr, Ident, Lifetime, Stmt, Token, Type};

static LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct CatchClause {
    binding: Ident,
    /// 异常表中的 catch_type 列表（multi-catch 有多个）
    matched: Vec<Type>,
    /// 绑定变量的声明类型（multi-catch 时为公共上界）；None = catch-any，绑定为 Throwable
    declared: Option<Type>,
    body: Block,
}

struct TryInput {
    body: Block,
    catches: Vec<CatchClause>,
}

impl Parse for TryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![try]>()?;
        let body: Block = input.parse()?;
        let mut catches = Vec::new();
        while !input.is_empty() {
            let kw: Ident = input.parse()?;
            if kw != "catch" {
                return Err(syn::Error::new(kw.span(), "expected `catch`"));
            }
            let head;
            syn::parenthesized!(head in input);
            let binding: Ident = head.parse()?;
            if head.is_empty() {
                // `catch (t)`：catch-any（异常表 catch_type = 0，finally 的兜底处理器）
                let body: Block = input.parse()?;
                catches.push(CatchClause {
                    binding,
                    matched: Vec::new(),
                    declared: None,
                    body,
                });
                continue;
            }
            head.parse::<Token![:]>()?;
            let mut matched = vec![parse_catch_type(&head)?];
            while head.peek(Token![|]) {
                head.parse::<Token![|]>()?;
                matched.push(parse_catch_type(&head)?);
            }
            let declared = if head.peek(Token![as]) {
                head.parse::<Token![as]>()?;
                Some(head.parse::<Type>()?)
            } else {
                Some(matched[0].clone())
            };
            let body: Block = input.parse()?;
            catches.push(CatchClause { binding, matched, declared, body });
        }
        if catches.is_empty() {
            return Err(input.error("java_try! requires at least one catch clause"));
        }
        Ok(TryInput { body, catches })
    }
}

/// catch 类型只能是路径类型；用 `Type::without_plus` 避免把 `A | B` 之外的记号吞掉。
fn parse_catch_type(input: ParseStream) -> syn::Result<Type> {
    let path: syn::TypePath = input.parse()?;
    Ok(Type::Path(path))
}

/// try 体改写器：把异常出口改为跳到本层标签。
struct ExitRewriter {
    label: Lifetime,
    loop_depth: usize,
    uses_break: bool,
    uses_continue: bool,
    error: Option<syn::Error>,
}

impl ExitRewriter {
    fn macro_name(mac: &syn::Macro) -> Option<std::string::String> {
        mac.path.segments.last().map(|s| s.ident.to_string())
    }

    /// 处理 try 体里的嵌套宏调用；返回替换后的表达式（None 表示不处理）。
    fn rewrite_macro(&mut self, mac: &syn::Macro) -> Option<Expr> {
        match Self::macro_name(mac).as_deref() {
            Some("java_try") => {
                // 内层 try 先自洽展开，再由本层改写其残余出口（catch 体、未匹配重抛）
                let inner: TryInput = match syn::parse2(mac.tokens.clone()) {
                    Ok(i) => i,
                    Err(e) => { self.error = Some(e); return None; }
                };
                let expanded = expand_input(inner);
                match syn::parse2::<Expr>(expanded) {
                    Ok(mut e) => { self.visit_expr_mut(&mut e); Some(e) }
                    Err(e) => { self.error = Some(e); None }
                }
            }
            _ => None,
        }
    }
}

impl VisitMut for ExitRewriter {
    fn visit_expr_closure_mut(&mut self, _c: &mut syn::ExprClosure) {
        // 闭包体有自己的返回通道，不改写
    }

    fn visit_item_mut(&mut self, _i: &mut syn::Item) {
        // 嵌套 item（fn 等）不属于本 try 体的控制流
    }

    fn visit_expr_loop_mut(&mut self, l: &mut syn::ExprLoop) {
        self.loop_depth += 1;
        visit_mut::visit_expr_loop_mut(self, l);
        self.loop_depth -= 1;
    }

    fn visit_expr_while_mut(&mut self, l: &mut syn::ExprWhile) {
        self.visit_expr_mut(&mut l.cond);
        self.loop_depth += 1;
        self.visit_block_mut(&mut l.body);
        self.loop_depth -= 1;
    }

    fn visit_expr_for_loop_mut(&mut self, l: &mut syn::ExprForLoop) {
        self.visit_expr_mut(&mut l.expr);
        self.loop_depth += 1;
        self.visit_block_mut(&mut l.body);
        self.loop_depth -= 1;
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Macro(sm) = stmt {
            if let Some(e) = self.rewrite_macro(&sm.mac) {
                *stmt = Stmt::Expr(e, Some(Default::default()));
                return;
            }
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Macro(em) = expr {
            if let Some(e) = self.rewrite_macro(&em.mac) {
                *expr = e;
                return;
            }
        }
        // 先改写子表达式，再处理本节点
        visit_mut::visit_expr_mut(self, expr);
        let label = &self.label;
        match expr {
            Expr::Try(t) => {
                let inner = &t.expr;
                *expr = syn::parse_quote! {
                    match #inner {
                        ::core::result::Result::Ok(__v) => __v,
                        ::core::result::Result::Err(__e) =>
                            break #label Err(From::from(__e)),
                    }
                };
            }
            Expr::Return(r) => {
                let is_err = match r.expr.as_deref() {
                    Some(Expr::Call(c)) => match &*c.func {
                        Expr::Path(p) => p.path.segments.last().map_or(false, |s| s.ident == "Err"),
                        _ => false,
                    },
                    _ => false,
                };
                if is_err {
                    let value = r.expr.as_ref().unwrap();
                    *expr = syn::parse_quote! { break #label #value };
                }
            }
            Expr::Break(b) if b.label.is_none() && self.loop_depth == 0 => {
                self.uses_break = true;
                *expr = syn::parse_quote! { break #label Ok(1u8) };
            }
            Expr::Continue(c) if c.label.is_none() && self.loop_depth == 0 => {
                self.uses_continue = true;
                *expr = syn::parse_quote! { break #label Ok(2u8) };
            }
            _ => {}
        }
    }
}

/// 语句序列是否必然异常/跳转完成（不会"落出"末尾）。
/// try 体从不正常完成时，正常完成分支不可达；据此让整个 try 语句的类型为 `!`，
/// 使其可以出现在方法体末尾（Java 中这类方法同样没有后续 return）。
fn never_falls_through(stmts: &[Stmt]) -> bool {
    match stmts.last() {
        Some(Stmt::Expr(e, _)) => expr_never_falls_through(e),
        Some(Stmt::Macro(sm)) => macro_never_falls_through(&sm.mac),
        _ => false,
    }
}

fn macro_never_falls_through(mac: &syn::Macro) -> bool {
    match ExitRewriter::macro_name(mac).as_deref() {
        Some("java_try") => syn::parse2::<TryInput>(mac.tokens.clone()).map_or(false, |t| {
            never_falls_through(&t.body.stmts)
                && t.catches.iter().all(|c| never_falls_through(&c.body.stmts))
        }),
        _ => false,
    }
}

fn expr_never_falls_through(e: &Expr) -> bool {
    match e {
        Expr::Return(_) | Expr::Continue(_) => true,
        Expr::Break(_) => true,
        Expr::Block(b) if b.label.is_none() => never_falls_through(&b.block.stmts),
        Expr::If(i) => match &i.else_branch {
            Some((_, els)) => {
                never_falls_through(&i.then_branch.stmts) && expr_never_falls_through(els)
            }
            None => false,
        },
        // switch 的每个臂都以 return / throw 结束（match 总是穷尽的，含 `_` 臂）
        Expr::Match(m) => m.arms.iter().all(|arm| expr_never_falls_through(&arm.body)),
        Expr::Macro(m) => macro_never_falls_through(&m.mac),
        _ => false,
    }
}

fn expand_input(input: TryInput) -> TokenStream2 {
    let n = LABEL_COUNTER.fetch_add(1, Ordering::Relaxed);
    let label = Lifetime::new(&format!("'java_try_{n}"), Span::call_site());
    let mut rewriter = ExitRewriter {
        label: label.clone(),
        loop_depth: 0,
        uses_break: false,
        uses_continue: false,
        error: None,
    };
    let mut body = input.body;
    let normal_arm = if never_falls_through(&body.stmts) {
        quote! { ::core::result::Result::Ok(_) => unreachable!(), }
    } else {
        quote! { ::core::result::Result::Ok(_) => {} }
    };
    rewriter.visit_block_mut(&mut body);
    if let Some(e) = rewriter.error {
        return e.to_compile_error();
    }
    let body_stmts = &body.stmts;

    let break_arm = if rewriter.uses_break {
        quote! { ::core::result::Result::Ok(1u8) => break, }
    } else {
        quote! {}
    };
    let continue_arm = if rewriter.uses_continue {
        quote! { ::core::result::Result::Ok(2u8) => continue, }
    } else {
        quote! {}
    };

    // catch 子句按声明顺序匹配（与异常表顺序一致）
    let mut chain = quote! { { return Err(__thrown); } };
    for clause in input.catches.iter().rev() {
        let binding = &clause.binding;
        let declared = &clause.declared;
        let matched = &clause.matched;
        let stmts = &clause.body.stmts;
        let condition = if matched.is_empty() {
            quote! { true }
        } else {
            quote! { #( __thrown.is_instance_of(<#matched>::BINARY_NAME) )||* }
        };
        let bind_value = match declared {
            Some(ty) => quote! { let mut #binding: #ty = __thrown.catch_as::<#ty>(<#ty>::BINARY_NAME); },
            None => quote! { let mut #binding = __thrown.catch_any(); },
        };
        chain = quote! {
            if #condition {
                #[allow(unused_mut, unused_variables)]
                #bind_value
                #(#stmts)*
            } else #chain
        };
    }

    quote! {
        {
            #[allow(unreachable_code)]
            let __java_try_r: ::core::result::Result<u8, JvmError> = #label: {
                #(#body_stmts)*
                ::core::result::Result::Ok(0u8)
            };
            match __java_try_r {
                #break_arm
                #continue_arm
                #normal_arm
                ::core::result::Result::Err(__thrown) => { #chain }
            }
        }
    }
}

pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<TryInput>(input) {
        Ok(i) => expand_input(i),
        Err(e) => e.to_compile_error(),
    }
}

/// 就地展开方法体里的全部 `java_try!`（供 `java_class!` 在改写方法体之前调用）。
/// `java_class!` 的方法体改写基于语法树，宏调用的内容对它不可见；
/// 先展开，try / catch 体里的字段访问、继承方法调用才会与体外代码得到同样的改写。
pub fn expand_in_block(block: &mut Block) {
    struct TryExpander;
    impl TryExpander {
        fn expanded(mac: &syn::Macro) -> Option<Expr> {
            if ExitRewriter::macro_name(mac).as_deref() != Some("java_try") {
                return None;
            }
            let input = syn::parse2::<TryInput>(mac.tokens.clone()).ok()?;
            syn::parse2::<Expr>(expand_input(input)).ok()
        }
    }
    impl VisitMut for TryExpander {
        fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
            if let Stmt::Macro(sm) = stmt {
                if let Some(e) = Self::expanded(&sm.mac) {
                    *stmt = Stmt::Expr(e, None);
                }
            }
            visit_mut::visit_stmt_mut(self, stmt);
        }
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            if let Expr::Macro(em) = expr {
                if let Some(e) = Self::expanded(&em.mac) {
                    *expr = e;
                }
            }
            visit_mut::visit_expr_mut(self, expr);
        }
    }
    TryExpander.visit_block_mut(block);
}
