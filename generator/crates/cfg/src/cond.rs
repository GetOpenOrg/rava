//! 条件代数（← `conditions.py`）：JVM 条件跳转 → 可取反 / 可短路合并的布尔条件。
//!
//! [`Cond`] 有四种形态：
//! - `Atom`：原子条件，同时携带「成立」与「不成立」两个表达式（取反不产生 `!(..)` 包裹）
//! - `And` / `Or`：全部 / 任一成立
//! - `Const`：编译期常量（静态 instanceof 等）
//!
//! 取反走德摩根律，始终保持可读形态：`!(a || b)` → `!a && !b`。
//! 与 Python 的差异：原子是 [`ir::Expr`] 而非文本；`render_cond` 变为 [`Cond::to_expr`]
//! （嵌套复合条件以 [`Expr::Paren`] 显式定界，渲染文本与 Python 一致）；
//! `cmp_op` 产出结构化比较（`(a==0)` 渲染为 `(a == 0)`，见 `GOLDEN_DIFF.md`）。

use ir::{BinOp, Expr, Ident, Lit, Path, UnOp};

use crate::opcodes::*;
use crate::CfgError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cond {
    Atom { pos: Expr, neg: Expr },
    And(Vec<Cond>),
    Or(Vec<Cond>),
    Const(bool),
}

/// 文本在前缀 `!` 之下是否无需括号（Raw 逃生舱的原子性判定，与 Python 同口径）。
fn raw_is_simple(text: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if depth == 0 && " <>=|&+-*/%".contains(c) => return false,
            _ => {}
        }
    }
    true
}

/// 表达式在前缀 `!` 之下是否需要括号。
fn needs_paren_under_not(e: &Expr) -> bool {
    match e {
        Expr::Binary { .. } | Expr::Ref { .. } | Expr::Deref(_) => true,
        Expr::Unary { op: UnOp::Neg, .. } => true,
        Expr::Cast { outer_paren, .. } => !outer_paren,
        Expr::Raw(r) => !raw_is_simple(r.as_str().trim()),
        _ => false,
    }
}

fn not(e: Expr) -> Expr {
    let inner = if needs_paren_under_not(&e) { Expr::paren(e) } else { e };
    Expr::Unary { op: UnOp::Not, expr: Box::new(inner) }
}

impl Cond {
    /// 原子条件；不成立形态自动推导（`!x` ↔ `x`，`!(x)` ↔ `x`，其余加 `!`）。
    /// 字面量 `true` / `false` 折叠为常量。
    pub fn atom(pos: Expr) -> Cond {
        if let Expr::Lit(Lit::Bool(b)) = pos {
            return Cond::Const(b);
        }
        let neg = match &pos {
            Expr::Unary { op: UnOp::Not, expr } => match expr.as_ref() {
                Expr::Paren(inner) => (**inner).clone(),
                other => other.clone(),
            },
            _ => not(pos.clone()),
        };
        Cond::Atom { pos, neg }
    }

    /// 显式给出成立 / 不成立两种形态的原子条件。
    pub fn atom_pair(pos: Expr, neg: Expr) -> Cond {
        if let Expr::Lit(Lit::Bool(b)) = pos {
            return Cond::Const(b);
        }
        Cond::Atom { pos, neg }
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Cond::Const(_))
    }

    pub fn negate(&self) -> Cond {
        match self {
            Cond::Const(v) => Cond::Const(!v),
            Cond::Atom { pos, neg } => Cond::Atom { pos: neg.clone(), neg: pos.clone() },
            Cond::And(items) => Cond::Or(items.iter().map(Cond::negate).collect()),
            Cond::Or(items) => Cond::And(items.iter().map(Cond::negate).collect()),
        }
    }

    fn combine(is_or: bool, a: Cond, b: Cond) -> Cond {
        // or：true 吸收；and：false 吸收
        let absorbing = is_or;
        if let Cond::Const(v) = a {
            // 左侧常量：吸收值 → 整体为常量（右侧不求值，与短路语义一致）；否则结果即右侧
            return if v == absorbing { a } else { b };
        }
        if matches!(b, Cond::Const(v) if v != absorbing) {
            return a;
        }
        // 右侧为吸收常量时左侧仍需求值（可能含调用），保留为普通合取/析取项
        let mut items = Vec::new();
        for c in [a, b] {
            match (is_or, c) {
                (true, Cond::Or(xs)) | (false, Cond::And(xs)) => items.extend(xs),
                (_, other) => items.push(other),
            }
        }
        if is_or {
            Cond::Or(items)
        } else {
            Cond::And(items)
        }
    }

    pub fn and(a: Cond, b: Cond) -> Cond {
        Cond::combine(false, a, b)
    }

    pub fn or(a: Cond, b: Cond) -> Cond {
        Cond::combine(true, a, b)
    }

    /// 渲染用布尔表达式（← `render_cond`）：复合条件内的复合子项以括号定界。
    pub fn to_expr(&self) -> Expr {
        self.to_expr_nested(false)
    }

    fn to_expr_nested(&self, nested: bool) -> Expr {
        let (op, items) = match self {
            Cond::Const(v) => return Expr::Lit(Lit::Bool(*v)),
            Cond::Atom { pos, .. } => return pos.clone(),
            Cond::And(items) => (BinOp::And, items),
            Cond::Or(items) => (BinOp::Or, items),
        };
        let mut it = items.iter().map(|c| c.to_expr_nested(true));
        let first = it.next().unwrap_or(Expr::Lit(Lit::Bool(op == BinOp::And)));
        let chain = it.fold(first, |acc, x| Expr::binary(op, acc, x));
        if nested {
            Expr::paren(chain)
        } else {
            chain
        }
    }

    /// 对所有原子（pos / neg）应用 f，返回新条件。
    pub fn map_atoms(&self, f: &mut dyn FnMut(&Expr) -> Expr) -> Cond {
        match self {
            Cond::Atom { pos, neg } => Cond::Atom { pos: f(pos), neg: f(neg) },
            Cond::And(items) => Cond::And(items.iter().map(|c| c.map_atoms(f)).collect()),
            Cond::Or(items) => Cond::Or(items.iter().map(|c| c.map_atoms(f)).collect()),
            Cond::Const(v) => Cond::Const(*v),
        }
    }

    /// 遍历所有原子的两种形态。
    pub fn for_each_atom(&self, f: &mut dyn FnMut(&Expr)) {
        match self {
            Cond::Atom { pos, neg } => {
                f(pos);
                f(neg);
            }
            Cond::And(items) | Cond::Or(items) => items.iter().for_each(|c| c.for_each_atom(f)),
            Cond::Const(_) => {}
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// JVM 比较指令 → 条件表达式
// ─────────────────────────────────────────────────────────────────────────────

fn is_cmp(op: BinOp) -> bool {
    matches!(op, BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge)
}

/// Raw 文本顶层是否含比较运算符（Python `_paren_if_cmp` 的文本口径）。
fn raw_has_top_cmp(text: &str) -> bool {
    if text.starts_with('(') && text.ends_with(')') {
        return false;
    }
    let b = text.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..b.len() {
        match b[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            c if depth == 0 => {
                let next = b.get(i + 1).copied();
                if next == Some(b'=') && matches!(c, b'!' | b'=' | b'<' | b'>') {
                    return true;
                }
                // `->` / `=>` 的 `>` 前一字节是 `-` / `=`：Python 口径按「从当前字节起两字节」判定，
                // 当前字节为 `<` / `>` 时两字节串不可能是 `->` / `=>`
                if c == b'<' || c == b'>' {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// 顶层是比较的操作数加括号，防止 Rust 链式比较错误（E0308）。
fn paren_if_cmp(e: Expr) -> Expr {
    let needs = match &e {
        Expr::Binary { op, .. } => is_cmp(*op),
        Expr::Raw(r) => raw_has_top_cmp(r.as_str()),
        _ => false,
    };
    if needs {
        Expr::paren(e)
    } else {
        e
    }
}

fn zero() -> Expr {
    Expr::Lit(Lit::Int { value: 0, ty: None })
}

fn is_jnull(a: Expr) -> Result<Expr, CfgError> {
    let name = Ident::new("_is_jnull").map_err(|e| CfgError::new(e.to_string()))?;
    Ok(Expr::call(Path::from_idents([name]), vec![Expr::reference(a)]))
}

/// JVM 比较跳转 → 跳转成立的条件表达式（← `cmp_op`）。单操作数指令忽略 `b`。
pub fn cmp_op(opc: u8, a: Expr, b: Expr) -> Result<Expr, CfgError> {
    let two = |op: BinOp| Expr::binary(op, paren_if_cmp(a.clone()), paren_if_cmp(b.clone()));
    let one = |op: BinOp| Expr::paren(Expr::binary(op, a.clone(), zero()));
    Ok(match opc {
        IF_ICMPEQ | IF_ACMPEQ => two(BinOp::Eq),
        IF_ICMPNE | IF_ACMPNE => two(BinOp::Ne),
        IF_ICMPLT => two(BinOp::Lt),
        IF_ICMPGE => two(BinOp::Ge),
        IF_ICMPLE => two(BinOp::Le),
        IF_ICMPGT => two(BinOp::Gt),
        IFEQ => one(BinOp::Eq),
        IFNE => one(BinOp::Ne),
        IFLT => one(BinOp::Lt),
        IFGE => one(BinOp::Ge),
        IFLE => one(BinOp::Le),
        IFGT => one(BinOp::Gt),
        classfile::op::IFNULL => is_jnull(a)?,
        classfile::op::IFNONNULL => Expr::Unary { op: UnOp::Not, expr: Box::new(is_jnull(a)?) },
        _ => return Err(CfgError::new(format!("非条件跳转指令: {}", classfile::insn::opcode_name(opc)))),
    })
}

/// 条件跳转指令的取反指令（`ifeq` ↔ `ifne` 等）。
pub fn negate_branch(opc: u8) -> Option<u8> {
    Some(match opc {
        IFEQ => IFNE,
        IFNE => IFEQ,
        IFLT => IFGE,
        IFGE => IFLT,
        IFLE => IFGT,
        IFGT => IFLE,
        IF_ICMPEQ => IF_ICMPNE,
        IF_ICMPNE => IF_ICMPEQ,
        IF_ICMPLT => IF_ICMPGE,
        IF_ICMPGE => IF_ICMPLT,
        IF_ICMPLE => IF_ICMPGT,
        IF_ICMPGT => IF_ICMPLE,
        IF_ACMPEQ => IF_ACMPNE,
        IF_ACMPNE => IF_ACMPEQ,
        classfile::op::IFNULL => classfile::op::IFNONNULL,
        classfile::op::IFNONNULL => classfile::op::IFNULL,
        _ => return None,
    })
}

/// fall-through 条件（跳转条件的否定，← `neg_cmp_op`）。
pub fn neg_cmp_op(opc: u8, a: Expr, b: Expr) -> Result<Expr, CfgError> {
    let n = negate_branch(opc)
        .ok_or_else(|| CfgError::new(format!("非条件跳转指令: {}", classfile::insn::opcode_name(opc))))?;
    cmp_op(n, a, b)
}

/// 跳转指令 → 原子条件（成立 = 跳转，不成立 = 落入）。
pub fn branch_atom(opc: u8, a: Expr, b: Expr) -> Result<Cond, CfgError> {
    Ok(Cond::atom_pair(cmp_op(opc, a.clone(), b.clone())?, neg_cmp_op(opc, a, b)?))
}
