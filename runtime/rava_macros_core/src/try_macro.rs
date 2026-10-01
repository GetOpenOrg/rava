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
//!     'java_try_end_N: {
//!         let __thrown: JvmError = 'java_try_N: { BODY'; break 'java_try_end_N; };
//!         if __thrown.is_instance_of(<T>::BINARY_NAME) { let e: T = ...; HANDLER }
//!         else { return Err(__thrown); }
//!     }
//! }
//! ```
//! 其中 BODY' 是把 `expr?` 改写为 `break 'java_try_N <异常>` 后的 try 体。
//! 正常完成与各处理器是控制流图上**各自独立的路径**，在 try 语句之后才汇合——
//! 每条路径上完成的局部变量初始化对 rustc 的定值分析可见（Java 的 definite assignment
//! 同样逐路径成立）。把结果先归并成一个值再 `match` 的形式会丢掉这一信息。
//!
//! try 体 / 处理器里指向 try 之外循环的 `break` / `continue` 改写为带该循环标签的形式
//! （`java_class!` 展开方法体时已知外围循环；循环没有标签时为其补一个）。
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

/// try 语句所在位置的外围循环（break / continue 的目标）。
#[derive(Clone)]
enum LoopTarget {
    /// 外围循环及其标签
    Label(Lifetime),
    /// 外围循环未知（`java_try!` 作为独立宏展开）：经流程码转发
    Unknown,
}

/// try 体 / 处理器改写器：把异常出口改为跳到本层标签，把指向外围循环的
/// `break` / `continue` 改为不经过本层标签块的形式。
struct ExitRewriter {
    label: Lifetime,
    end_label: Lifetime,
    /// 是否改写异常出口（try 体：是；处理器：否——处理器里的异常向外传播）
    rewrite_throws: bool,
    target: LoopTarget,
    loop_depth: usize,
    uses_loop_label: bool,
    uses_break: bool,
    uses_continue: bool,
}

impl ExitRewriter {
    fn macro_name(mac: &syn::Macro) -> Option<std::string::String> {
        mac.path.segments.last().map(|s| s.ident.to_string())
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

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        // 先改写子表达式，再处理本节点
        visit_mut::visit_expr_mut(self, expr);
        let label = &self.label;
        let end_label = &self.end_label;
        match expr {
            Expr::Try(t) if self.rewrite_throws => {
                let inner = &t.expr;
                *expr = syn::parse_quote! {
                    match #inner {
                        ::core::result::Result::Ok(__v) => __v,
                        ::core::result::Result::Err(__e) => break #label From::from(__e),
                    }
                };
            }
            Expr::Return(r) if self.rewrite_throws => {
                let thrown = match r.expr.as_deref() {
                    Some(Expr::Call(c)) if c.args.len() == 1 => match &*c.func {
                        Expr::Path(p)
                            if p.path.segments.last().map_or(false, |s| s.ident == "Err") =>
                        {
                            c.args.first().cloned()
                        }
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(value) = thrown {
                    *expr = syn::parse_quote! { break #label #value };
                }
            }
            Expr::Break(b) if b.label.is_none() && self.loop_depth == 0 => {
                match &self.target {
                    LoopTarget::Label(l) => {
                        self.uses_loop_label = true;
                        *expr = syn::parse_quote! { break #l };
                    }
                    LoopTarget::Unknown => {
                        self.uses_break = true;
                        *expr = syn::parse_quote! { break #end_label 1u8 };
                    }
                }
            }
            Expr::Continue(c) if c.label.is_none() && self.loop_depth == 0 => {
                match &self.target {
                    LoopTarget::Label(l) => {
                        self.uses_loop_label = true;
                        *expr = syn::parse_quote! { continue #l };
                    }
                    LoopTarget::Unknown => {
                        self.uses_continue = true;
                        *expr = syn::parse_quote! { break #end_label 2u8 };
                    }
                }
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

/// 扫描循环体：是否存在绑定到**本层**循环的无标签 `break`。
/// 有 → 本循环可正常落出；没有 → `loop` 的 Rust 类型是 `!`（每个出口都是
/// return / continue / 指向外层标签的 break），try 体以它结尾时不落出。
/// 嵌套循环的无标签 break 绑定到嵌套循环自身，不计；闭包的控制流独立，不下钻。
struct BreakScan {
    depth: usize,
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for BreakScan {
    fn visit_expr_break(&mut self, b: &'ast syn::ExprBreak) {
        if b.label.is_none() && self.depth == 0 {
            self.found = true;
        }
        syn::visit::visit_expr_break(self, b);
    }
    fn visit_expr_loop(&mut self, l: &'ast syn::ExprLoop) {
        self.depth += 1;
        syn::visit::visit_expr_loop(self, l);
        self.depth -= 1;
    }
    fn visit_expr_while(&mut self, w: &'ast syn::ExprWhile) {
        self.depth += 1;
        syn::visit::visit_expr_while(self, w);
        self.depth -= 1;
    }
    fn visit_expr_for_loop(&mut self, f: &'ast syn::ExprForLoop) {
        self.depth += 1;
        syn::visit::visit_expr_for_loop(self, f);
        self.depth -= 1;
    }
    fn visit_expr_closure(&mut self, _c: &'ast syn::ExprClosure) {
        // 闭包有自己的返回通道，其 break 与本层循环无关
    }
}

fn expr_never_falls_through(e: &Expr) -> bool {
    match e {
        Expr::Return(_) | Expr::Continue(_) => true,
        Expr::Break(_) => true,
        Expr::Loop(l) => {
            // `loop` 含绑定本层的 break 才会落出；否则类型为 `!`（每条出口都是
            // return / continue / 跨层 break，生成代码的典型形态：
            // loop { if c { return Ok(v); } ...步进... }）
            let mut scan = BreakScan { depth: 0, found: false };
            syn::visit::Visit::visit_block(&mut scan, &l.body);
            !scan.found
        }
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

/// 展开一个 try 语句。try 体与处理器里的嵌套 `java_try!` 必须已经展开（见 `TryExpander`）。
/// 返回展开结果，以及是否引用了外围循环的标签。
fn expand_input(input: TryInput, target: LoopTarget) -> (TokenStream2, bool) {
    let n = LABEL_COUNTER.fetch_add(1, Ordering::Relaxed);
    let label = Lifetime::new(&format!("'java_try_{n}"), Span::call_site());
    let end_label = Lifetime::new(&format!("'java_try_end_{n}"), Span::call_site());
    let mut rewriter = ExitRewriter {
        label: label.clone(),
        end_label: end_label.clone(),
        rewrite_throws: true,
        target,
        loop_depth: 0,
        uses_loop_label: false,
        uses_break: false,
        uses_continue: false,
    };
    let mut body = input.body;
    let falls_through = !never_falls_through(&body.stmts);
    rewriter.visit_block_mut(&mut body);
    let body_stmts = &body.stmts;

    // catch 子句按声明顺序匹配（与异常表顺序一致）
    rewriter.rewrite_throws = false;
    let mut chain = quote! { { return Err(__thrown); } };
    for mut clause in input.catches.into_iter().rev() {
        rewriter.visit_block_mut(&mut clause.body);
        let binding = &clause.binding;
        let matched = &clause.matched;
        let stmts = &clause.body.stmts;
        let condition = if matched.is_empty() {
            quote! { true }
        } else {
            quote! { #( __thrown.is_instance_of(<#matched>::BINARY_NAME) )||* }
        };
        let bind_value = match &clause.declared {
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

    let expanded = if rewriter.uses_break || rewriter.uses_continue {
        // 外围循环未知：正常完成 / 处理器完成 / break / continue 经流程码区分
        let break_arm = if rewriter.uses_break { quote! { 1u8 => break, } } else { quote! {} };
        let continue_arm =
            if rewriter.uses_continue { quote! { 2u8 => continue, } } else { quote! {} };
        quote! {
            {
                #[allow(unreachable_code, unused_labels)]
                let __java_try_flow: u8 = #end_label: {
                    let __thrown: JvmError = #label: {
                        #(#body_stmts)*
                        break #end_label 0u8;
                    };
                    #chain
                    0u8
                };
                match __java_try_flow {
                    #break_arm
                    #continue_arm
                    _ => {}
                }
            }
        }
    } else if falls_through {
        quote! {
            {
                #end_label: {
                    #[allow(unreachable_code, unused_labels)]
                    let __thrown: JvmError = #label: {
                        #(#body_stmts)*
                        break #end_label;
                    };
                    #chain
                }
            }
        }
    } else {
        // try 体从不正常完成：整个 try 语句的类型由处理器决定（处理器也都不落出时为 `!`）
        quote! {
            {
                #[allow(unreachable_code, unused_labels)]
                let __thrown: JvmError = #label: {
                    #(#body_stmts)*
                };
                #chain
            }
        }
    };
    (expanded, rewriter.uses_loop_label)
}

/// 自内向外展开 `java_try!`，并维护外围循环栈。
struct TryExpander {
    /// 每层循环：(标签, 标签是否由本展开器补上)；None = 尚无标签
    loops: Vec<Option<(Lifetime, bool)>>,
    error: Option<syn::Error>,
}

impl TryExpander {
    fn expanded(&mut self, mac: &syn::Macro) -> Option<Expr> {
        if ExitRewriter::macro_name(mac).as_deref() != Some("java_try") {
            return None;
        }
        let mut input = match syn::parse2::<TryInput>(mac.tokens.clone()) {
            Ok(i) => i,
            Err(e) => { self.error = Some(e); return None; }
        };
        // 内层 try 先自洽展开，再由本层改写其残余出口（catch 体、未匹配重抛）
        self.visit_block_mut(&mut input.body);
        for clause in input.catches.iter_mut() {
            self.visit_block_mut(&mut clause.body);
        }
        let (target, fresh) = match self.loops.last() {
            Some(Some((l, _))) => (LoopTarget::Label(l.clone()), false),
            Some(None) => {
                let n = LABEL_COUNTER.fetch_add(1, Ordering::Relaxed);
                let l = Lifetime::new(&format!("'java_loop_{n}"), Span::call_site());
                (LoopTarget::Label(l), true)
            }
            None => (LoopTarget::Unknown, false),
        };
        let fresh_label = match (&target, fresh) {
            (LoopTarget::Label(l), true) => Some(l.clone()),
            _ => None,
        };
        let (tokens, uses_loop_label) = expand_input(input, target);
        if uses_loop_label {
            if let (Some(l), Some(slot)) = (fresh_label, self.loops.last_mut()) {
                *slot = Some((l, true));
            }
        }
        match syn::parse2::<Expr>(tokens) {
            Ok(e) => Some(e),
            Err(e) => { self.error = Some(e); None }
        }
    }

    fn enter_loop(&mut self, label: &Option<syn::Label>) {
        self.loops.push(label.as_ref().map(|l| (l.name.clone(), false)));
    }

    fn leave_loop(&mut self, label: &mut Option<syn::Label>) {
        if let Some(Some((l, true))) = self.loops.pop() {
            *label = Some(syn::Label { name: l, colon_token: Default::default() });
        }
    }
}

impl VisitMut for TryExpander {
    fn visit_expr_closure_mut(&mut self, c: &mut syn::ExprClosure) {
        // 闭包体是独立的控制流：break / continue 不能跨出闭包
        let saved = std::mem::take(&mut self.loops);
        visit_mut::visit_expr_closure_mut(self, c);
        self.loops = saved;
    }

    fn visit_expr_loop_mut(&mut self, l: &mut syn::ExprLoop) {
        self.enter_loop(&l.label);
        self.visit_block_mut(&mut l.body);
        self.leave_loop(&mut l.label);
    }

    fn visit_expr_while_mut(&mut self, l: &mut syn::ExprWhile) {
        self.visit_expr_mut(&mut l.cond);
        self.enter_loop(&l.label);
        self.visit_block_mut(&mut l.body);
        self.leave_loop(&mut l.label);
    }

    fn visit_expr_for_loop_mut(&mut self, l: &mut syn::ExprForLoop) {
        self.visit_expr_mut(&mut l.expr);
        self.enter_loop(&l.label);
        self.visit_block_mut(&mut l.body);
        self.leave_loop(&mut l.label);
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Macro(sm) = stmt {
            if let Some(e) = self.expanded(&sm.mac) {
                *stmt = Stmt::Expr(e, None);
                return;
            }
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Macro(em) = expr {
            if let Some(e) = self.expanded(&em.mac) {
                *expr = e;
                return;
            }
        }
        visit_mut::visit_expr_mut(self, expr);
    }
}

/// 独立的 `java_try! { .. }` 宏调用：外围循环不可见。
pub fn expand(input: TokenStream2) -> TokenStream2 {
    let mac: syn::Macro = syn::parse_quote! { java_try! { #input } };
    let mut expander = TryExpander { loops: Vec::new(), error: None };
    let expanded = expander.expanded(&mac);
    match (expander.error, expanded) {
        (Some(e), _) => e.to_compile_error(),
        (None, Some(e)) => quote! { #e },
        (None, None) => quote! {},
    }
}

/// 就地展开方法体里的全部 `java_try!`（供 `java_class!` 在改写方法体之前调用）。
/// `java_class!` 的方法体改写基于语法树，宏调用的内容对它不可见；
/// 先展开，try / catch 体里的字段访问、继承方法调用才会与体外代码得到同样的改写。
pub fn expand_in_block(block: &mut Block) {
    let mut expander = TryExpander { loops: Vec::new(), error: None };
    expander.visit_block_mut(block);
}
