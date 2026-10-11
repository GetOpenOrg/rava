//! VTableBodyKind 分类：判断方法体是否可安全放入 vtable 上下文。
//!
//! 唯一判定来源：[`BodyFacts`] 按语法树（`syn::visit`）收集方法体里与 vtable 上下文相关的三类
//! 事实，`classify_vtable_body`（宏展开 / 生成器剥体计划）与 `define_base_has_body`（base 函数是否
//! 保留真实体）都基于它判定，不再各自做文本子串匹配。

use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{Block, Expr, Token};

/// 判断方法体是否可以安全放入 vtable 上下文（__inner impl）。
/// vtable 上下文中 self 是 &dyn Trait，禁止出现：
///   - Self:: 引用
///   - Clone::clone(this)（可能依赖 wrapper 类型的 Clone impl）
///   - this.non_accessor_method()（调用 wrapper 上的方法，__inner 无法访问）
#[derive(Debug)]
pub(crate) enum VTableBodyKind {
    Safe,         // 直接放入 vtable impl
    NeedsWrapper, // 需要 wrapper 重建（含 Clone::clone，this 需要是 wrapper）
    Skip,         // 跳过（含 Self::，wrapper 重建也无法解决）
}

/// 方法体中与 vtable 上下文相关的语法事实。
#[derive(Debug, Default)]
pub(crate) struct BodyFacts {
    /// 出现以 `Self` 开头的多段路径（`Self::x`、`Self::T::y`；单独的 `Self` / `<Self as T>::x` 不算）
    pub(crate) self_path: bool,
    /// 出现 `…Clone::clone(this)`（实参恰为裸 `this`）
    pub(crate) clone_this: bool,
    /// 以裸 `this` 为接收者的方法调用名（`this.m(..)`、`this.m::<T>(..)`），按出现顺序
    pub(crate) this_calls: Vec<String>,
}

impl BodyFacts {
    pub(crate) fn of(block: &Block) -> Self {
        let mut f = BodyFacts::default();
        f.visit_block(block);
        f
    }

    /// 是否有 `this.m(..)` 调用落在 wrapper 上：方法名不以 `__` 开头（访问器 / 内部钩子除外）
    /// 且不被 `in_vtable` 接受（vtable 上可直接调用的方法）。
    pub(crate) fn calls_wrapper_method(&self, in_vtable: impl Fn(&str) -> bool) -> bool {
        self.this_calls.iter().any(|m| !m.starts_with("__") && !in_vtable(m))
    }

    /// 宏调用：实参能按逗号分隔表达式解析（`format!` / `vec!` / `panic!` 等）时按语法树访问；
    /// 否则（如 `matches!` 的模式实参）在记号树上识别同样三种形态。
    fn visit_macro_tokens(&mut self, tokens: &TokenStream2) {
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        match syn::parse::Parser::parse2(parser, tokens.clone()) {
            Ok(args) => args.iter().for_each(|e| self.visit_expr(e)),
            Err(_) => self.scan_tokens(tokens),
        }
    }

    fn scan_tokens(&mut self, tokens: &TokenStream2) {
        let tts: Vec<TokenTree> = tokens.clone().into_iter().collect();
        let ident = |i: usize, name: &str| matches!(tts.get(i), Some(TokenTree::Ident(x)) if x == name);
        let punct = |i: usize, c: char| matches!(tts.get(i), Some(TokenTree::Punct(p)) if p.as_char() == c);
        let path_sep = |i: usize| punct(i, ':') && punct(i + 1, ':');
        let paren = |i: usize| match tts.get(i) {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => Some(g.stream()),
            _ => None,
        };
        for (i, tt) in tts.iter().enumerate() {
            if let TokenTree::Group(g) = tt {
                self.visit_macro_tokens(&g.stream());
                continue;
            }
            let after_dot = i > 0 && punct(i - 1, '.');
            if ident(i, "Self") && path_sep(i + 1) && !(i >= 2 && path_sep(i - 2)) {
                self.self_path = true;
            }
            if ident(i, "Clone") && path_sep(i + 1) && ident(i + 3, "clone") {
                if let Some(args) = paren(i + 4) {
                    let args: Vec<TokenTree> = args.into_iter().collect();
                    if matches!(args.as_slice(), [TokenTree::Ident(x)] if x == "this") {
                        self.clone_this = true;
                    }
                }
            }
            if ident(i, "this") && !after_dot && punct(i + 1, '.') {
                if let Some(TokenTree::Ident(m)) = tts.get(i + 2) {
                    let turbofish = path_sep(i + 3) && punct(i + 5, '<');
                    if paren(i + 3).is_some() || turbofish {
                        self.this_calls.push(m.to_string());
                    }
                }
            }
        }
    }
}

fn is_bare_this(e: &Expr) -> bool {
    matches!(e, Expr::Path(p) if p.qself.is_none() && p.path.is_ident("this"))
}

impl<'ast> Visit<'ast> for BodyFacts {
    fn visit_path(&mut self, p: &'ast syn::Path) {
        if p.leading_colon.is_none() && p.segments.len() >= 2 && p.segments[0].ident == "Self" {
            self.self_path = true;
        }
        visit::visit_path(self, p);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let Expr::Path(f) = &*c.func {
            let segs = &f.path.segments;
            let n = segs.len();
            let is_clone = n >= 2 && segs[n - 2].ident == "Clone" && segs[n - 1].ident == "clone";
            if is_clone && c.args.len() == 1 && is_bare_this(&c.args[0]) {
                self.clone_this = true;
            }
        }
        visit::visit_expr_call(self, c);
    }

    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        if is_bare_this(&m.receiver) {
            self.this_calls.push(m.method.to_string());
        }
        visit::visit_expr_method_call(self, m);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        visit::visit_macro(self, m);
        self.visit_macro_tokens(&m.tokens);
    }
}

pub(crate) fn classify_vtable_body(block: &Block) -> VTableBodyKind {
    let facts = BodyFacts::of(block);
    if facts.self_path {
        VTableBodyKind::Skip
    } else if facts.clone_this || facts.calls_wrapper_method(|_| false) {
        // this.method() 调用 non-__ 方法：可能是 native 方法（仅在 wrapper 上有实现）
        VTableBodyKind::NeedsWrapper
    } else {
        VTableBodyKind::Safe
    }
}

/// A-1 存储层擦除后的门控分类：泛型类（带类型形参）的方法体不走 vtable 直连（Safe）
/// 路径 —— `impl<P..> X__VTable<P..> for X__inner` 覆盖全部实例化，inner 上下文对
/// `this.__get_x()` / `this.vtable_method()` 的解析无法定实例化（E0283）。统一改走
/// wrapper 钩子（NeedsWrapper）：wrapper 的 vtable 槽位类型已知，签名不歧义。
pub(crate) fn vtable_body_kind_gated(block: &Block, class_is_generic: bool) -> VTableBodyKind {
    match classify_vtable_body(block) {
        VTableBodyKind::Safe if class_is_generic => VTableBodyKind::NeedsWrapper,
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(b: Block) -> BodyFacts {
        BodyFacts::of(&b)
    }

    #[test]
    fn self_path_only_multi_segment() {
        assert!(facts(syn::parse_quote!({ Self::f(1) })).self_path);
        assert!(facts(syn::parse_quote!({ let x: Self::T = y; })).self_path);
        assert!(!facts(syn::parse_quote!({ let s: Self = Self { a: 1 }; })).self_path);
        assert!(!facts(syn::parse_quote!({ <Self as Tr>::f(1) })).self_path);
        // 标识符后缀 / 字符串字面量里的 `Self::` 不算
        assert!(!facts(syn::parse_quote!({ MySelf::f(); panic!("Self::x this.f()") })).self_path);
    }

    #[test]
    fn clone_this_requires_bare_this() {
        assert!(facts(syn::parse_quote!({ Ok(Clone::clone(this)) })).clone_this);
        assert!(facts(syn::parse_quote!({ ::std::clone::Clone::clone(this) })).clone_this);
        assert!(!facts(syn::parse_quote!({ Clone::clone(&this) })).clone_this);
        assert!(!facts(syn::parse_quote!({ Clone::clone(this.x) })).clone_this);
    }

    #[test]
    fn this_calls_only_bare_receiver() {
        let f = facts(syn::parse_quote!({
            this.a(1)?; this.__get_x(); this.b::<T>(); this.f.c(); __this.d(); (this).e(); x.this.g();
        }));
        assert_eq!(f.this_calls, ["a", "__get_x", "b"]);
        assert!(f.calls_wrapper_method(|_| false));
        assert!(!f.calls_wrapper_method(|m| m == "a" || m == "b"));
    }

    #[test]
    fn macro_arguments_are_visited() {
        // 可解析为表达式的宏实参走语法树
        let f = facts(syn::parse_quote!({ let v = vec![this.a()?, Self::K]; format!("{}", this.b()?) }));
        assert!(f.self_path);
        assert_eq!(f.this_calls, ["a", "b"]);
        // 模式实参不可解析：记号树兜底
        let f = facts(syn::parse_quote!({ matches!(this.a(), Some(_) if Clone::clone(this).ok()) }));
        assert!(f.clone_this);
        assert_eq!(f.this_calls, ["a"]);
    }

    #[test]
    fn classification() {
        assert!(matches!(classify_vtable_body(&syn::parse_quote!({ Self::f() })), VTableBodyKind::Skip));
        assert!(matches!(classify_vtable_body(&syn::parse_quote!({ this.m() })), VTableBodyKind::NeedsWrapper));
        assert!(matches!(classify_vtable_body(&syn::parse_quote!({ this.__get_x() })), VTableBodyKind::Safe));
    }
}
