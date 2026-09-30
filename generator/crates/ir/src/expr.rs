//! 表达式节点（← `rs_ir.py` 表达式段 21 个节点）。

use crate::{Ident, Lit, Path, PathSegment, Stmt, Type};

/// 二元运算符（优先级见 `render::expr::precedence`，与 Python `_PREC` 一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    BitOr,
    BitXor,
    BitAnd,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl BinOp {
    pub fn as_str(self) -> &'static str {
        match self {
            BinOp::Or => "||",
            BinOp::And => "&&",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::BitOr => "|",
            BinOp::BitXor => "^",
            BinOp::BitAnd => "&",
            BinOp::Shl => "<<",
            BinOp::Shr => ">>",
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
        }
    }

    pub fn from_symbol(s: &str) -> Option<BinOp> {
        const ALL: [BinOp; 18] = [
            BinOp::Or,
            BinOp::And,
            BinOp::Eq,
            BinOp::Ne,
            BinOp::Lt,
            BinOp::Gt,
            BinOp::Le,
            BinOp::Ge,
            BinOp::BitOr,
            BinOp::BitXor,
            BinOp::BitAnd,
            BinOp::Shl,
            BinOp::Shr,
            BinOp::Add,
            BinOp::Sub,
            BinOp::Mul,
            BinOp::Div,
            BinOp::Rem,
        ];
        ALL.into_iter().find(|op| op.as_str() == s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg,
    Not,
}

impl UnOp {
    pub fn as_str(self) -> &'static str {
        match self {
            UnOp::Neg => "-",
            UnOp::Not => "!",
        }
    }
}

/// 被调函数路径（← `Call.func: str`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FnPath {
    /// `a::B::<T>::f`（段上的泛型以 turbofish 渲染）
    Path(Path),
    /// `<SelfTy as Trait>::rest`（如 `<X<A> as ::std::convert::From<_>>::from`）；
    /// `trait_` 为 None 时渲染 `<SelfTy>::rest`
    Qualified { self_ty: Box<Type>, trait_: Option<Path>, rest: Vec<PathSegment> },
}

/// 宏调用 `name!(args..)`（Python 实参是原始字符串，这里为表达式；
/// 格式串等记号用 [`Lit::Str`]）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroCall {
    pub name: Ident,
    pub args: Vec<Expr>,
}

/// 内联块表达式 `{ stmts; tail }`。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
    pub tail: Option<Box<Expr>>,
}

/// `if` 表达式（返回值）。条件为字面量 `false` 时渲染器只输出 else 块（或 `{}`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr {
    pub cond: Box<Expr>,
    pub then: BlockExpr,
    pub else_: Option<BlockExpr>,
}

/// `getstatic`：`Short<turbofish..>::field()?`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaticFieldRef {
    /// 声明类 JVM binary 名（如 `java/lang/System`），渲染经 [`crate::ShortNames`]
    pub class: String,
    pub field: Ident,
    /// 字段类型（不渲染，供下游类型推断）
    pub ty: Type,
    /// 类的泛型实参（消歧 E0283，渲染为 `::<A, B>`），空则省略
    pub turbofish: Vec<Type>,
}

/// 类祖先按值上转（R-2′）的包装方式——形态统一为后缀 `.into()`，目标类型由左值 /
/// 形参 / 返回位给定（宏 `From<Self> for Ancestor`）。合并 Python `UpcastExpr.wrap`
/// （auto / owned）与 `upcast_expr(src, wrap)` 字符串函数的 none / clone / paren。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpcastWrap {
    /// 源已是可后缀的表达式（调用链 / 已 clone 的实参）：`src.into()`
    Bare,
    /// 源是位置（变量 / 字段路径）：`Clone::clone(&src).into()` 保所有权（E0382）
    Clone,
    /// 恒加括号：`(src).into()`
    Paren,
    /// 原子表达式直接后缀，否则加括号（[`crate::Renderer::is_atomic`] 判定）
    Auto,
    /// 源是 [`Expr::Var`] → 同 `Clone`；否则同 `Paren`
    Owned,
}

/// checkcast 语义（← `CastExpr.checked` / `interface_target` 两个 bool + `binary_name`）。
/// 非法组合（unchecked 却带运行时判定名）在类型上不可表达。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CastMode {
    /// 静态合法的视图转换（跨实例化擦除路径，A-1）：
    /// `<Target as ::std::convert::From<Object>>::from(src)`
    Unchecked,
    /// checkcast（运行时可失败，S-1）：`src.try_cast::<Target>("binary")?`；
    /// 目标为 `JArray<E>` 时按元素类型 `src.try_cast_array::<E>("binary")?`
    Checked { binary: String },
    /// checkcast 到接口（目标擦除记录为 Object，A-4 批次 1）：`src.try_cast_iface("binary")?`
    CheckedInterface { binary: String },
}

/// checkcast / 跨实例化转换（A-3 定型）。src 统一为 `Clone::clone(&expr)`
/// （`this` 为 `Clone::clone(this)`），`box_first` 时再包 `Object::from(..)`。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub expr: Box<Expr>,
    pub target: Type,
    pub mode: CastMode,
    pub box_first: bool,
}

/// 文本逃生舱（← `RawExpr` / `RawStmt` / `RsRawItem`）。raw-audit 计数对象，终态 0；
/// 渲染器原样输出，任何判定不得解析其内容（原子性判定除外，见 `render::atomic`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Raw(pub String);

/// 表达式节点。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Lit(Lit),
    Var(Ident),
    Binary { op: BinOp, lhs: Box<Expr>, rhs: Box<Expr> },
    Unary { op: UnOp, expr: Box<Expr> },
    /// 自由函数调用 `path(args)`
    Call { func: FnPath, args: Vec<Expr> },
    /// `recv.method::<turbofish>(args)`
    MethodCall { recv: Box<Expr>, method: Ident, turbofish: Vec<Type>, args: Vec<Expr> },
    Field { recv: Box<Expr>, name: Ident },
    Index { recv: Box<Expr>, index: Box<Expr> },
    /// 数值 `as` 转换；`outer_paren` 为真渲染 `(x as T)`，否则 `x as T`（由外层定界）
    Cast { expr: Box<Expr>, ty: Type, outer_paren: bool },
    Ref { expr: Box<Expr>, mutable: bool },
    Deref(Box<Expr>),
    Block(BlockExpr),
    If(IfExpr),
    Macro(MacroCall),
    Raw(Raw),
    /// JVM `new`（等待 `<init>` 确定构造参数）：`Short::new()`，类名为 JVM binary 名
    NewPending { class: String },
    StaticField(StaticFieldRef),
    Upcast { expr: Box<Expr>, wrap: UpcastWrap },
    CheckCast(CastExpr),
    /// instanceof 的运行时判定：`(expr).is_instance_of("binary")`
    InstanceOf { expr: Box<Expr>, binary: String },
    /// `?` 错误传播
    Try(Box<Expr>),
    /// 显式括号 `(inner)`
    Paren(Box<Expr>),
}

impl Expr {
    pub fn var(name: Ident) -> Expr {
        Expr::Var(name)
    }

    pub fn call(func: Path, args: Vec<Expr>) -> Expr {
        Expr::Call { func: FnPath::Path(func), args }
    }

    pub fn method(recv: Expr, method: Ident, args: Vec<Expr>) -> Expr {
        Expr::MethodCall { recv: Box::new(recv), method, turbofish: Vec::new(), args }
    }

    pub fn binary(op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }

    pub fn try_(inner: Expr) -> Expr {
        Expr::Try(Box::new(inner))
    }

    pub fn paren(inner: Expr) -> Expr {
        Expr::Paren(Box::new(inner))
    }

    pub fn reference(expr: Expr) -> Expr {
        Expr::Ref { expr: Box::new(expr), mutable: false }
    }

    pub fn upcast(expr: Expr, wrap: UpcastWrap) -> Expr {
        Expr::Upcast { expr: Box::new(expr), wrap }
    }

    /// 是否为名为 `name` 的变量（`this` 等特殊接收者的结构判定）。
    pub fn is_var_named(&self, name: &str) -> bool {
        matches!(self, Expr::Var(v) if v.as_str() == name)
    }
}
