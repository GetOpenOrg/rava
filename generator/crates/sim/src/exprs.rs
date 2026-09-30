//! 表达式构造与判定（← `stack._maybe_downcast` / `_clone_moved_var` / `_opaque_let_value` /
//! `is_clone_of_var` / `_is_trivial_expr` / `_materialized_let_type`，以及 Python 各阶段
//! 拼接的 RawExpr 文本的结构化等价物）。
//!
//! 本模块只出现 Rust 语言 / std 名（`Clone`、`Default`、`From`、`Into`、`std::convert`）
//! 与运行时锚点（[`ir::anchors`]），不出现 JDK 类名。

use crate::env::{ident, prim_ir, to_ir_type, SimEnv};
use crate::error::SimResult;
use crate::types::{is_object, is_scalar};
use ir::{BinOp, CastExpr, CastMode, Expr, FnPath, Lit, MacroCall, Path, PathSegment, Stmt, Type};
use ty::RsType;

const CLONE: &str = "Clone";
const DEFAULT: &str = "Default";
const FROM: &str = "From";
const INTO: &str = "Into";

fn seg(name: &str, generics: Vec<Type>) -> SimResult<PathSegment> {
    Ok(PathSegment::with_generics(ident(name)?, generics))
}

/// `A::b(args)` 形态的两段路径调用（段上泛型以 turbofish 渲染）。
fn call2(outer: &str, outer_generics: Vec<Type>, func: &str, args: Vec<Expr>) -> SimResult<Expr> {
    let path = Path::new(vec![seg(outer, outer_generics)?, seg(func, Vec::new())?]);
    Ok(Expr::Call { func: FnPath::Path(path), args })
}

fn named(name: &str, generics: Vec<Type>) -> SimResult<Type> {
    Ok(Type::named(ident(name)?, generics))
}

/// `Default::default()`
pub fn default_value() -> SimResult<Expr> {
    call2(DEFAULT, Vec::new(), "default", Vec::new())
}

/// `Clone::clone(&v)`
pub fn clone_ref(e: Expr) -> SimResult<Expr> {
    call2(CLONE, Vec::new(), "clone", vec![Expr::reference(e)])
}

/// `Clone::clone(v)`（`this` 是 `&Self`，直接克隆本体）
pub fn clone_plain(e: Expr) -> SimResult<Expr> {
    call2(CLONE, Vec::new(), "clone", vec![e])
}

/// `From::from(v)`
pub fn from_call(e: Expr) -> SimResult<Expr> {
    call2(FROM, Vec::new(), "from", vec![e])
}

/// `Object::from(v)`
pub fn object_from(e: Expr) -> SimResult<Expr> {
    call2(ir::anchors::OBJECT, Vec::new(), "from", vec![e])
}

/// `Into::<T>::into(v)`
pub fn into_call(target: Type, e: Expr) -> SimResult<Expr> {
    call2(INTO, vec![target], "into", vec![e])
}

/// `<Target as ::std::convert::From<Src>>::from(v)`
pub fn qualified_from(target: Type, src: Type, e: Expr) -> SimResult<Expr> {
    let trait_ = Path {
        global: true,
        segments: vec![seg("std", Vec::new())?, seg("convert", Vec::new())?, seg(FROM, vec![src])?],
    };
    Ok(Expr::Call {
        func: FnPath::Qualified { self_ty: Box::new(target), trait_: Some(trait_), rest: vec![seg("from", Vec::new())?] },
        args: vec![e],
    })
}

/// `Object` 类型节点
pub fn object_type() -> SimResult<Type> {
    named(ir::anchors::OBJECT, Vec::new())
}

/// 栈下溢占位值 `(panic!("stack underflow") as i32)`
pub fn underflow_value() -> SimResult<Expr> {
    let panic = Expr::Macro(MacroCall { name: ident("panic")?, args: vec![Expr::Lit(Lit::Str("stack underflow".into()))] });
    Ok(Expr::Cast { expr: Box::new(panic), ty: Type::I32, outer_paren: true })
}

/// JVM int 族值落到声明类型：`(v) as i32` / `((v) as i32 != 0i32)` / `(((v) as i32) as T)`
pub fn int_family_cast(e: Expr, target: ty::Prim) -> Expr {
    let as_i32 = |outer_paren| Expr::Cast { expr: Box::new(Expr::paren(e.clone())), ty: Type::I32, outer_paren };
    match target {
        ty::Prim::I32 => as_i32(false),
        ty::Prim::Bool => Expr::paren(Expr::binary(BinOp::Ne, as_i32(false), Expr::Lit(Lit::i32(0)))),
        other => {
            Expr::Cast { expr: Box::new(as_i32(true)), ty: Type::Prim(prim_ir(other)), outer_paren: true }
        }
    }
}

/// 非受检视图转换 `<Target as From<Object>>::from(Clone::clone(&v))`（A-3 `CastExpr`）
pub fn unchecked_cast(e: Expr, target: Type, box_first: bool) -> Expr {
    Expr::CheckCast(CastExpr { expr: Box::new(e), target, mode: CastMode::Unchecked, box_first })
}

// ── 判定 ─────────────────────────────────────────────────────────────────────

/// 两段路径调用 `A::b(..)` 的段名（无泛型）
fn call2_names(e: &Expr) -> Option<(&str, &str, &[Expr])> {
    match e {
        Expr::Call { func: FnPath::Path(p), args } if !p.global && p.segments.len() == 2 => {
            let (a, b) = (&p.segments[0], &p.segments[1]);
            (a.generics.is_empty() && b.generics.is_empty()).then(|| (a.ident.as_str(), b.ident.as_str(), args.as_slice()))
        }
        _ => None,
    }
}

fn raw_text(e: &Expr) -> Option<&str> {
    match e {
        Expr::Raw(r) => Some(r.as_str()),
        _ => None,
    }
}

/// `Default::default()`（结构节点或同文本 Raw）
pub fn is_default(e: &Expr) -> bool {
    matches!(call2_names(e), Some((DEFAULT, "default", []))) || raw_text(e) == Some("Default::default()")
}

/// 空引用字面量 `Object::default()`
pub fn is_null(e: &Expr) -> bool {
    matches!(e, Expr::Lit(Lit::Null))
}

/// 空引用（字面量或同文本 Raw）——Python `render_expr(e) == 'Object::default()'` 判定
fn renders_null(e: &Expr) -> bool {
    is_null(e) || raw_text(e) == Some("Object::default()")
}

/// `render_expr(e) in ('Default::default()', 'Object::default()')`
pub fn is_default_or_null(e: &Expr) -> bool {
    is_default(e) || renders_null(e)
}

/// `Clone::clone(&v)`（`_clone_moved_var` 的产物）
pub fn is_clone_of_var(e: &Expr) -> bool {
    match call2_names(e) {
        Some((CLONE, "clone", [Expr::Ref { expr, mutable: false }])) => matches!(**expr, Expr::Var(_)),
        _ => false,
    }
}

fn is_word(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c.is_alphabetic() || c == '_') && cs.all(|c| c.is_alphanumeric() || c == '_')
}

/// Python `_TRIVIAL_RAW_RE`（只作用于 Raw 文本）
fn trivial_raw(text: &str) -> bool {
    let t = text.trim();
    if is_word(t) || t == "Default::default()" {
        return true;
    }
    if let Some(inner) = t.strip_prefix("Clone::clone(").and_then(|r| r.strip_suffix(')')) {
        return is_word(inner.strip_prefix('&').unwrap_or(inner));
    }
    if let Some(head) = t.strip_suffix("::default()") {
        return is_word(head);
    }
    let digits = t.strip_prefix('-').unwrap_or(t);
    let mut cs = digits.chars();
    matches!(cs.next(), Some(c) if c.is_ascii_digit()) && cs.all(|c| c.is_alphanumeric() || c == '_' || c == '.')
}

/// 重复求值无副作用且无开销：变量、字面量、待定 new、`X::default()`、对变量的 Clone
pub fn is_trivial(e: &Expr) -> bool {
    match e {
        Expr::Var(_) | Expr::Lit(_) | Expr::NewPending { .. } => true,
        Expr::Raw(r) => trivial_raw(r.as_str()),
        Expr::Unary { op: ir::UnOp::Neg, expr } => matches!(**expr, Expr::Lit(_)),
        _ => {
            is_clone_of_var(e)
                || matches!(call2_names(e), Some((_, "default", [])))
                || matches!(call2_names(e), Some((CLONE, "clone", [Expr::Var(_)])))
        }
    }
}

/// Raw 文本含调用 / 宏（标识符、`>` 或 `!` 紧接 `(`），或 `?` 传播
fn raw_reads_state(t: &str) -> bool {
    let b = t.as_bytes();
    b.contains(&b'?') || b.windows(2).any(|w| w[1] == b'(' && (w[0].is_ascii_alphanumeric() || matches!(w[0], b'_' | b'>' | b'!')))
}

/// 求值结果依赖可变状态或带副作用：含调用、宏、字段 / 数组读取、静态字段、`?` 传播。
/// 栈上这类待求值条目在任何语句发射之前按栈序物化（JVM 已在语句之前求值它们）
pub fn reads_state(e: &Expr) -> bool {
    match e {
        Expr::Lit(Lit::JStringConcat { args, .. }) => args.iter().any(reads_state),
        _ if is_trivial(e) => false,
        Expr::Lit(_) | Expr::Var(_) | Expr::NewPending { .. } => false,
        Expr::Call { .. }
        | Expr::MethodCall { .. }
        | Expr::Macro(_)
        | Expr::StaticField(_)
        | Expr::Index { .. }
        | Expr::Field { .. }
        | Expr::Try(_)
        | Expr::Block(_)
        | Expr::If(_) => true,
        Expr::Raw(r) => raw_reads_state(r.as_str()),
        Expr::Binary { lhs, rhs, .. } => reads_state(lhs) || reads_state(rhs),
        Expr::Unary { expr, .. }
        | Expr::Cast { expr, .. }
        | Expr::Ref { expr, .. }
        | Expr::Deref(expr)
        | Expr::Paren(expr)
        | Expr::Upcast { expr, .. }
        | Expr::InstanceOf { expr, .. } => reads_state(expr),
        Expr::CheckCast(c) => reads_state(&c.expr),
    }
}

/// let 省略类型标注的值：类型自明的既有类型化节点之外的全部表达式
pub fn opaque_let_value(e: &Expr) -> bool {
    !matches!(
        e,
        Expr::Var(_)
            | Expr::Lit(_)
            | Expr::Cast { .. }
            | Expr::CheckCast(_)
            | Expr::Upcast { .. }
            | Expr::InstanceOf { .. }
            | Expr::StaticField(_)
            | Expr::NewPending { .. }
    )
}

fn ends_with_into(e: &Expr) -> bool {
    match e {
        Expr::Upcast { .. } => true,
        Expr::MethodCall { method, turbofish, args, .. } => method.as_str() == "into" && turbofish.is_empty() && args.is_empty(),
        Expr::Raw(r) => r.as_str().ends_with(".into()"),
        _ => false,
    }
}

fn stmt_contains_default(s: &Stmt) -> bool {
    match s {
        Stmt::Let(l) => l.value.as_ref().is_some_and(contains_default),
        Stmt::Assign(a) => contains_default(&a.target) || contains_default(&a.value),
        Stmt::Expr(e) | Stmt::Return(Some(e)) => contains_default(e),
        Stmt::Raw(r) => r.as_str().contains("Default::default()"),
        _ => false,
    }
}

/// 渲染文本含 `Default::default()`（结构遍历；内联块只看简单语句）
fn contains_default(e: &Expr) -> bool {
    if is_default(e) {
        return true;
    }
    let sub = |x: &Expr| contains_default(x);
    match e {
        Expr::Raw(r) => r.as_str().contains("Default::default()"),
        Expr::Binary { lhs, rhs, .. } => sub(lhs) || sub(rhs),
        Expr::Call { args, .. } | Expr::Macro(MacroCall { args, .. }) => args.iter().any(sub),
        Expr::MethodCall { recv, args, .. } => sub(recv) || args.iter().any(sub),
        Expr::Index { recv, index } => sub(recv) || sub(index),
        Expr::Unary { expr, .. }
        | Expr::Field { recv: expr, .. }
        | Expr::Cast { expr, .. }
        | Expr::Ref { expr, .. }
        | Expr::Deref(expr)
        | Expr::Upcast { expr, .. }
        | Expr::InstanceOf { expr, .. }
        | Expr::Try(expr)
        | Expr::Paren(expr) => sub(expr),
        Expr::CheckCast(c) => sub(&c.expr),
        Expr::Block(b) => b.stmts.iter().any(stmt_contains_default) || b.tail.as_deref().is_some_and(sub),
        Expr::If(i) => {
            sub(&i.cond)
                || i.then.stmts.iter().any(stmt_contains_default)
                || i.then.tail.as_deref().is_some_and(sub)
                || i.else_.as_ref().is_some_and(|b| b.stmts.iter().any(stmt_contains_default) || b.tail.as_deref().is_some_and(sub))
        }
        Expr::Lit(_) | Expr::Var(_) | Expr::NewPending { .. } | Expr::StaticField(_) => false,
    }
}

/// 物化临时变量是否需要类型注解：表达式以 `.into()` 结尾或含 `Default::default()`
pub fn materialized_needs_type(e: &Expr) -> bool {
    ends_with_into(e) || contains_default(e)
}

/// 局部变量出现在右值位置时包 `Clone::clone(&v)` 保活（Java 引用赋值无 move 语义）
pub fn clone_moved_var(e: Expr, ty: &RsType) -> SimResult<Expr> {
    if matches!(e, Expr::Var(_)) && !is_scalar(ty) {
        clone_ref(e)
    } else {
        Ok(e)
    }
}

/// 新鲜变量存入非 Object 引用类型：按目标类型还原视图（非受检 `CastExpr`）
pub fn maybe_downcast(e: Expr, ty: &RsType, env: &dyn SimEnv) -> SimResult<Expr> {
    if matches!(e, Expr::Var(_)) && !is_scalar(ty) && !is_object(ty) {
        Ok(unchecked_cast(e, to_ir_type(ty, env)?, false))
    } else {
        Ok(e)
    }
}
