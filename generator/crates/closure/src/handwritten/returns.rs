//! 手写 fn 返回值来源的语法判定（计划 c1d §30 B3）。
//!
//! 手写体就是该成员运行期的真实语义，返回值的来源可以从函数体直接读出，无须在清单里另行声明。
//! 判定只认两种返回点，其余一律记为未知（引擎退回 open(返回类型)，即原行为）：
//! - **null**：`Ok(Default::default())` / `Ok(T::default())`（单独的 `T::default()` 是 Java null）；
//! - **Java 静态方法的返回**：路径调用 `T::m(…)` 直接作尾表达式（其 `Result` 原样交出），或 `Ok(T::m(…)?)`。
//!   路径能否解析成 Java 静态方法由引擎判定，解析不出同样按未知处理。
//!
//! 返回点是 fn 体的尾表达式（穿过块 / `if` / `match` 的各分支）与全部 `return` 表达式。闭包、async 块与
//! 嵌套 item 里的 `return` 不是本 fn 的返回点，跳过；`?` 只在出错时提前返回，交出的是异常而非返回值。
//! 不展开宏：宏内出现 `return` 时整体记为未知；发散宏（`panic!` 等）作尾表达式时不产生返回值。

use syn::visit::Visit;

use super::syntax::expr_path_segs;
use super::TypeRef;

/// 发散宏：作尾表达式时不产生返回值
const DIVERGING_MACROS: &[&str] = &["panic", "unreachable", "unimplemented", "todo"];

/// 返回值经由的 Java 静态方法调用：类型路径（未展开 use）、Rust 方法名、实参个数
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RetCall {
    pub path_ty: TypeRef,
    pub name: String,
    pub nargs: usize,
}

/// 手写 fn 返回值的来源
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum HwRet {
    /// 尚未并入任何 fn（合并的单位元）
    #[default]
    Unset,
    /// 推不出：引擎按 open(返回类型) 处理
    Unknown,
    /// 各返回点只有 null 与这些调用的返回值（空表 = 恒返回 null）
    Known(Vec<RetCall>),
}

impl HwRet {
    /// 两个同名 fn（或一个成员匹配到的多个 fn）的返回来源之并
    pub fn join(&mut self, o: &HwRet) {
        *self = match (std::mem::take(self), o) {
            (HwRet::Unset, x) => x.clone(),
            (x, HwRet::Unset) => x,
            (HwRet::Known(mut a), HwRet::Known(b)) => {
                for c in b {
                    if !a.contains(c) {
                        a.push(c.clone());
                    }
                }
                HwRet::Known(a)
            }
            _ => HwRet::Unknown,
        };
    }

    /// 已推出的返回来源（未并入 / 未知为 None）
    pub fn known(&self) -> Option<&[RetCall]> {
        match self {
            HwRet::Known(v) => Some(v),
            _ => None,
        }
    }
}

/// fn 体的返回来源
pub(super) fn fn_returns(block: &syn::Block) -> HwRet {
    let mut acc = Acc { calls: Vec::new(), unknown: false };
    let mut rv = Returns { acc: &mut acc };
    rv.visit_block(block);
    acc.block_tail(block);
    if acc.unknown {
        HwRet::Unknown
    } else {
        HwRet::Known(acc.calls)
    }
}

struct Acc {
    calls: Vec<RetCall>,
    unknown: bool,
}

impl Acc {
    fn block_tail(&mut self, b: &syn::Block) {
        match b.stmts.last() {
            Some(syn::Stmt::Expr(e, None)) => self.tail(e),
            // 以宏语句收尾（无分号的宏调用即尾表达式）
            Some(syn::Stmt::Macro(m)) if m.semi_token.is_none() => self.mac(&m.mac),
            // 无尾表达式：只经 `return` 返回（或返回 `()`，无引用值）
            _ => {}
        }
    }

    fn mac(&mut self, m: &syn::Macro) {
        if !m.path.segments.last().is_some_and(|s| DIVERGING_MACROS.contains(&s.ident.to_string().as_str())) {
            self.unknown = true;
        }
    }

    /// 返回点表达式（尾表达式或 `return` 的操作数）
    fn tail(&mut self, e: &syn::Expr) {
        match e {
            syn::Expr::Paren(p) => self.tail(&p.expr),
            syn::Expr::Group(g) => self.tail(&g.expr),
            syn::Expr::Block(b) => self.block_tail(&b.block),
            syn::Expr::Unsafe(b) => self.block_tail(&b.block),
            syn::Expr::If(i) => {
                self.block_tail(&i.then_branch);
                match &i.else_branch {
                    Some((_, e)) => self.tail(e),
                    None => self.unknown = true,
                }
            }
            syn::Expr::Match(m) => {
                for a in &m.arms {
                    self.tail(&a.body);
                }
            }
            // `return` 由遍历另行收集
            syn::Expr::Return(_) => {}
            syn::Expr::Macro(m) => self.mac(&m.mac),
            syn::Expr::Call(c) => match ctor_name(c).as_deref() {
                Some("Ok") if c.args.len() == 1 => self.value(&c.args[0], false),
                // 异常路径，不是返回值
                Some("Err") => {}
                _ => self.value(e, true),
            },
            _ => self.unknown = true,
        }
    }

    /// `Ok(v)` 的 v（`direct` = 尾表达式本身即可失败调用，其 `Result` 原样交出）
    fn value(&mut self, v: &syn::Expr, direct: bool) {
        let call = match (v, direct) {
            (syn::Expr::Call(c), true) => Some(c),
            (syn::Expr::Try(t), false) => match &*t.expr {
                syn::Expr::Call(c) => Some(c),
                _ => None,
            },
            (syn::Expr::Call(c), false) if c.args.is_empty() && path_of(c).is_some_and(|s| s.last().is_some_and(|l| l == "default")) => {
                // Java null
                return;
            }
            _ => None,
        };
        match call.and_then(|c| path_of(c).map(|s| (s, c.args.len()))) {
            Some((segs, nargs)) if segs.len() >= 2 => {
                let (name, ty) = segs.split_last().expect("至少两段");
                self.calls.push(RetCall { path_ty: TypeRef(ty.to_vec()), name: name.clone(), nargs });
            }
            _ => self.unknown = true,
        }
    }
}

fn path_of(c: &syn::ExprCall) -> Option<Vec<String>> {
    match &*c.func {
        syn::Expr::Path(p) => Some(expr_path_segs(p)),
        _ => None,
    }
}

fn ctor_name(c: &syn::ExprCall) -> Option<String> {
    path_of(c).filter(|s| s.len() == 1).map(|s| s[0].clone())
}

/// 收集本 fn 的 `return` 返回点（跳过闭包 / async 块 / 嵌套 item；宏内出现 `return` 即未知）
struct Returns<'a> {
    acc: &'a mut Acc,
}

impl<'ast> Visit<'ast> for Returns<'_> {
    fn visit_expr_return(&mut self, r: &'ast syn::ExprReturn) {
        match &r.expr {
            Some(e) => {
                self.acc.tail(e);
                self.visit_expr(e);
            }
            None => {}
        }
    }
    fn visit_expr_closure(&mut self, _: &'ast syn::ExprClosure) {}
    fn visit_expr_async(&mut self, _: &'ast syn::ExprAsync) {}
    fn visit_item(&mut self, _: &'ast syn::Item) {}
    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        if has_return(m.tokens.clone()) {
            self.acc.unknown = true;
        }
    }
}

fn has_return(ts: proc_macro2::TokenStream) -> bool {
    ts.into_iter().any(|t| match t {
        proc_macro2::TokenTree::Ident(i) => i == "return",
        proc_macro2::TokenTree::Group(g) => has_return(g.stream()),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(src: &str) -> HwRet {
        let f: syn::ItemFn = syn::parse_str(src).expect("fn");
        fn_returns(&f.block)
    }

    fn call(ty: &[&str], name: &str, nargs: usize) -> RetCall {
        RetCall { path_ty: TypeRef(ty.iter().map(|s| s.to_string()).collect()), name: name.into(), nargs }
    }

    #[test]
    fn null_and_static_calls() {
        assert_eq!(ret("fn f(n: String) -> Result<R> { let _ = n; Ok(Default::default()) }"), HwRet::Known(vec![]));
        assert_eq!(ret("fn f() -> Result<R> { Ok(R::default()) }"), HwRet::Known(vec![]));
        assert_eq!(ret("fn f() -> Result<E> { crate::p::q::Z::empty() }"), HwRet::Known(vec![call(&["crate", "p", "q", "Z"], "empty", 0)]));
        assert_eq!(ret("fn f(x: i32) -> Result<E> { if x > 0 { return Ok(A::make(x)?); } match x { 0 => Ok(Default::default()), _ => Err(e()) } }"), HwRet::Known(vec![call(&["A"], "make", 1)]));
        assert_eq!(ret("fn f() -> Result<E> { panic!(\"x\") }"), HwRet::Known(vec![]));
    }

    #[test]
    fn unknown_sources() {
        // 局部变量、本文件自由 fn、方法调用、宏内 return、无 else 的 if
        assert_eq!(ret("fn f() -> Result<E> { let x = A::make()?; Ok(x) }"), HwRet::Unknown);
        assert_eq!(ret("fn f() -> Result<E> { helper() }"), HwRet::Unknown);
        assert_eq!(ret("fn f(&self) -> Result<E> { self.g() }"), HwRet::Unknown);
        assert_eq!(ret("fn f() -> Result<E> { m!(return Ok(y)); Ok(Default::default()) }"), HwRet::Unknown);
        assert_eq!(ret("fn f() -> Result<E> { java_try!(x) }"), HwRet::Unknown);
        // 闭包里的 return 不是本 fn 的返回点
        assert_eq!(ret("fn f() -> Result<E> { let g = || { return 1; }; Ok(Default::default()) }"), HwRet::Known(vec![]));
    }

    #[test]
    fn join() {
        let mut a = HwRet::Unset;
        a.join(&HwRet::Known(vec![]));
        assert_eq!(a, HwRet::Known(vec![]));
        a.join(&HwRet::Known(vec![call(&["A"], "m", 0)]));
        assert_eq!(a.known().map(<[_]>::len), Some(1));
        a.join(&HwRet::Unknown);
        assert_eq!(a, HwRet::Unknown);
    }
}
