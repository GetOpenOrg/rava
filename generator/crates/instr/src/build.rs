//! 表达式 / 语句构造的薄封装（Python 各处 f-string 拼接形态的结构化等价物）。
//!
//! 只出现 Rust 语言 / std 名与运行时锚点（[`ir::anchors`]），不出现 JDK 类名。

use ir::{Expr, FnPath, Ident, LetStmt, Lit, Path, PathSegment, Stmt, Type};
use ty::RsType;

use crate::env::InstrEnv;
use crate::error::InstrResult;

pub fn id(s: &str) -> InstrResult<Ident> {
    Ok(Ident::new(s)?)
}

pub fn var(s: &str) -> InstrResult<Expr> {
    Ok(Expr::Var(id(s)?))
}

pub fn seg(name: &str) -> InstrResult<PathSegment> {
    Ok(PathSegment::new(id(name)?))
}

pub fn seg_g(name: &str, generics: Vec<Type>) -> InstrResult<PathSegment> {
    Ok(PathSegment::with_generics(id(name)?, generics))
}

/// 多段路径（各段无泛型）
pub fn path(segs: &[&str]) -> InstrResult<Path> {
    Ok(Path::new(segs.iter().map(|s| seg(s)).collect::<InstrResult<Vec<_>>>()?))
}

/// `a::b::c(args)`
pub fn call(segs: &[&str], args: Vec<Expr>) -> InstrResult<Expr> {
    Ok(Expr::call(path(segs)?, args))
}

/// 路径调用（段已构造）
pub fn call_path(p: Path, args: Vec<Expr>) -> Expr {
    Expr::Call { func: FnPath::Path(p), args }
}

/// `recv.name(args)`
pub fn mcall(recv: Expr, name: &str, args: Vec<Expr>) -> InstrResult<Expr> {
    Ok(Expr::method(recv, id(name)?, args))
}

/// `recv.name::<T..>(args)`
pub fn mcall_tf(recv: Expr, name: &str, turbofish: Vec<Type>, args: Vec<Expr>) -> InstrResult<Expr> {
    Ok(Expr::MethodCall { recv: Box::new(recv), method: id(name)?, turbofish, args })
}

/// `e?`
pub fn try_(e: Expr) -> Expr {
    Expr::try_(e)
}

/// `(e)`
pub fn paren(e: Expr) -> Expr {
    Expr::paren(e)
}

/// `(e as T)`（outer_paren = true）/ `e as T`
pub fn cast(e: Expr, t: Type, outer_paren: bool) -> Expr {
    Expr::Cast { expr: Box::new(e), ty: t, outer_paren }
}

pub fn i32_lit(v: i64) -> Expr {
    Expr::Lit(Lit::int(v.into(), ir::IntTy::I32))
}

/// Python 字符串管线的 `this` 叶子：以 Raw 承载（渲染器对 `Var(this)` 的克隆形态不同，
/// 字符串管线历来发射 `Clone::clone(&this)`）
pub fn str_leaf(e: Expr) -> Expr {
    if e.is_var_named("this") {
        Expr::raw("this".to_string())
    } else {
        e
    }
}

/// `let name: T = value;`
pub fn let_typed(name: Ident, ty: Option<Type>, value: Expr) -> Stmt {
    Stmt::Let(LetStmt::new(name, ty, Some(value)))
}

/// `let mut name: T = value;`
pub fn let_mut(name: Ident, ty: Option<Type>, value: Expr) -> Stmt {
    let mut l = LetStmt::new(name, ty, Some(value));
    l.mutable = true;
    Stmt::Let(l)
}

/// `let _ = value;`
pub fn let_discard(value: Expr) -> Stmt {
    Stmt::Let(LetStmt::new(Ident::discard(), None, Some(value)))
}

/// `expr;`
pub fn expr_stmt(e: Expr) -> Stmt {
    Stmt::Expr(e)
}

/// RsType → ir::Type
pub fn ir_ty(env: &InstrEnv, t: &RsType) -> InstrResult<Type> {
    Ok(sim::to_ir_type(t, env)?)
}

/// 类型文本（Python `render_type` 口径）——只用于判定
pub fn ty_text(env: &InstrEnv, t: &RsType) -> String {
    sim::type_text(t, env)
}

/// 表达式文本（Python `render_expr` 口径）——只用于判定
pub fn text(env: &InstrEnv, e: &Expr) -> String {
    ir::Renderer::new(&sim::TyNames(env)).expr(e)
}
