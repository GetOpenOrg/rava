//! 语句节点（← `rs_ir.py` 语句段；`BlockStmt` / `StructLine` 由结构化控制流语句取代）。
//!
//! Python 的方法体结构块是「文本结构行（带花括号深度增量 delta 与角色 tag）+ 子语句序列」
//! 的交替列表，变量提升等 pass 按行文本与 delta 工作；这里控制流直接是树：
//! 标签、条件、臂、catch 子句都是字段，缩进由渲染器按嵌套深度产生，
//! 不再需要 delta / tag / flatten。

use crate::{Expr, Ident, Label, Lit, Raw, Type};

/// 绑定的变量身份与模拟类型（不渲染）：变量提升 pass 的证据源。
/// `slot` / `bind_off` 为 JVM 局部变量槽与 store 的字节码偏移（LVT 区间判定）；
/// `value_ty` 为值的模拟类型（`ty` 省略交给 Rust 推断时，前置声明仍需类型标注）。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct VarOrigin {
    pub value_ty: Option<Type>,
    pub slot: Option<u16>,
    pub bind_off: Option<u32>,
}

/// `let [mut] name[: ty][ = value];`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt {
    pub name: Ident,
    pub ty: Option<Type>,
    pub mutable: bool,
    pub value: Option<Expr>,
    pub origin: VarOrigin,
}

impl LetStmt {
    pub fn new(name: Ident, ty: Option<Type>, value: Option<Expr>) -> LetStmt {
        LetStmt { name, ty, mutable: false, value, origin: VarOrigin::default() }
    }
}

/// `target = value;`（由声明降级而来的赋值保留 origin）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssignStmt {
    pub target: Expr,
    pub value: Expr,
    pub origin: VarOrigin,
}

/// `[label: ]loop { body }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopStmt {
    pub label: Option<Label>,
    pub body: Vec<Stmt>,
}

/// `[label: ]while cond { body }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileStmt {
    pub label: Option<Label>,
    pub cond: Expr,
    pub body: Vec<Stmt>,
}

/// if 语句的 else 分支：无 / `else { .. }` / `else if ..`（链式）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElseBranch {
    None,
    Block(Vec<Stmt>),
    If(Box<IfStmt>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Vec<Stmt>,
    pub else_: ElseBranch,
}

/// match 臂模式：`_` 或 `a | b | c`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Wildcard,
    Alts(Vec<Lit>),
}

/// match 臂体：块 `pat => { .. }` 或单表达式 `pat => expr,`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArmBody {
    Block(Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: ArmBody,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchStmt {
    pub scrutinee: Expr,
    pub arms: Vec<MatchArm>,
}

/// `java_try!` 的 catch 子句：`catch (bind)` / `catch (bind: A | B as Lub)`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatchClause {
    pub bind: Ident,
    /// 捕获的异常类（JVM binary 名，渲染经 ShortNames）；空 = catch-any
    pub types: Vec<String>,
    /// 绑定变量的静态类型（多类型捕获的 LUB，或与唯一捕获类型不同时）；None 则省略 `as ..`
    pub lub: Option<Type>,
    pub body: Vec<Stmt>,
}

/// `java_try! { try { body } catch (..) { .. } }`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryStmt {
    pub body: Vec<Stmt>,
    pub catches: Vec<CatchClause>,
}

/// 语句节点。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    Expr(Expr),
    Return(Option<Expr>),
    Break(Option<Label>),
    Continue(Option<Label>),
    Loop(LoopStmt),
    While(WhileStmt),
    If(IfStmt),
    /// 带标签块 `'bN: { body }`
    LabeledBlock { label: Label, body: Vec<Stmt> },
    Match(MatchStmt),
    JavaTry(TryStmt),
    Raw(Raw),
}

impl Stmt {
    /// 文本逃生舱语句（raw-audit `raw_stmt` 计数，位点取调用者）
    #[track_caller]
    pub fn raw(text: impl Into<String>) -> Stmt {
        crate::raw_audit::record(crate::raw_audit::RawKind::Stmt, std::panic::Location::caller());
        Stmt::Raw(Raw::from_text(text.into()))
    }
}
