//! 条件的文本层（← `cfg/conditions.py` 的 `atom` / `cmp_op` / `render_cond`）。
//!
//! 条件代数沿用 [`cfg::Cond`]；方法体生成里的原子一律是 [`Expr::Raw`]，承载与 Python
//! 逐字相同的文本（`(a==0)` 无空格等），渲染经 [`render_cond`] 按 Python 口径拼接。

use cfg::opcodes::{self as opc, is_two_operand_branch};
use cfg::Cond;
use classfile::insn::op;
use ir::{Expr, Raw};

use crate::error::{cfg_err, MethodResult};

pub fn raw(text: impl Into<String>) -> Expr {
    Expr::Raw(Raw(text.into()))
}

/// 原子文本（Raw 之外的形态按渲染器口径取文本不在此层出现）
pub fn atom_text(e: &Expr) -> &str {
    match e {
        Expr::Raw(r) => &r.0,
        _ => "",
    }
}

/// 文本在前缀 `!` 之下是否无需括号（`_is_simple_operand`）
pub fn is_simple_operand(text: &str) -> bool {
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

fn balanced(text: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in text.chars() {
        if ch == '(' {
            depth += 1;
        } else if ch == ')' {
            depth -= 1;
            if depth < 0 {
                return false;
            }
        }
    }
    depth == 0
}

/// 原子条件（`atom(pos, neg)`）：`true` / `false` 折叠为常量；`neg` 缺省按文本推导
pub fn atom(pos: &str, neg: Option<&str>) -> Cond {
    let pos = pos.trim();
    if pos == "true" {
        return Cond::Const(true);
    }
    if pos == "false" {
        return Cond::Const(false);
    }
    let neg = match neg {
        Some(n) => n.to_string(),
        None => {
            if let Some(rest) = pos.strip_prefix('!').filter(|r| is_simple_operand(r)) {
                rest.to_string()
            } else if let Some(inner) =
                pos.strip_prefix("!(").and_then(|r| r.strip_suffix(')')).filter(|i| balanced(i))
            {
                inner.to_string()
            } else if is_simple_operand(pos) {
                format!("!{pos}")
            } else {
                format!("!({pos})")
            }
        }
    };
    Cond::Atom { pos: raw(pos), neg: raw(neg.trim()) }
}

/// 条件 → Rust 布尔表达式文本（`render_cond`）
pub fn render_cond(c: &Cond) -> String {
    render_nested(c, false)
}

fn render_nested(c: &Cond, nested: bool) -> String {
    let (sep, items) = match c {
        Cond::Const(v) => return if *v { "true" } else { "false" }.to_string(),
        Cond::Atom { pos, .. } => return atom_text(pos).to_string(),
        Cond::And(items) => (" && ", items),
        Cond::Or(items) => (" || ", items),
    };
    let text = items.iter().map(|i| render_nested(i, true)).collect::<Vec<_>>().join(sep);
    if nested {
        format!("({text})")
    } else {
        text
    }
}

/// 对全部原子文本（pos / neg）应用 f（`map_atoms`；常量不重判）
pub fn map_atoms(c: &Cond, f: &mut dyn FnMut(&str) -> String) -> Cond {
    match c {
        Cond::Atom { pos, neg } => Cond::Atom { pos: raw(f(atom_text(pos))), neg: raw(f(atom_text(neg))) },
        Cond::And(items) => Cond::And(items.iter().map(|i| map_atoms(i, f)).collect()),
        Cond::Or(items) => Cond::Or(items.iter().map(|i| map_atoms(i, f)).collect()),
        Cond::Const(v) => Cond::Const(*v),
    }
}

/// 顶层含比较运算符的操作数加括号（`_paren_if_cmp`）
fn paren_if_cmp(expr: &str) -> String {
    if expr.starts_with('(') && expr.ends_with(')') {
        return expr.to_string();
    }
    let b = expr.as_bytes();
    let mut depth: i32 = 0;
    for i in 0..b.len() {
        match b[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            c if depth == 0 => {
                let two = &b[i..(i + 2).min(b.len())];
                if matches!(two, b"!=" | b"==" | b"<=" | b">=") {
                    return format!("({expr})");
                }
                if (c == b'<' || c == b'>') && two != b"->" && two != b"=>" {
                    return format!("({expr})");
                }
            }
            _ => {}
        }
    }
    expr.to_string()
}

/// JVM 比较跳转 → 跳转成立的条件文本（`cmp_op`）；单操作数指令忽略 `b`
pub fn cmp_op(code: u8, a: &str, b: &str) -> MethodResult<String> {
    if is_two_operand_branch(code) {
        let sym = match code {
            opc::IF_ICMPEQ | opc::IF_ACMPEQ => "==",
            opc::IF_ICMPNE | opc::IF_ACMPNE => "!=",
            opc::IF_ICMPLT => "<",
            opc::IF_ICMPGE => ">=",
            opc::IF_ICMPLE => "<=",
            _ => ">",
        };
        return Ok(format!("{} {sym} {}", paren_if_cmp(a), paren_if_cmp(b)));
    }
    Ok(match code {
        opc::IFEQ => format!("({a}==0)"),
        opc::IFNE => format!("({a}!=0)"),
        opc::IFLT => format!("({a}<0)"),
        opc::IFGE => format!("({a}>=0)"),
        opc::IFLE => format!("({a}<=0)"),
        opc::IFGT => format!("({a}>0)"),
        op::IFNULL => format!("_is_jnull(&{a})"),
        op::IFNONNULL => format!("!_is_jnull(&{a})"),
        _ => return cfg_err(format!("非条件跳转指令: {}", classfile::insn::opcode_name(code))),
    })
}

/// fall-through 条件文本（`neg_cmp_op`）
pub fn neg_cmp_op(code: u8, a: &str, b: &str) -> MethodResult<String> {
    match cfg::negate_branch(code) {
        Some(n) => cmp_op(n, a, b),
        None => cfg_err(format!("非条件跳转指令: {}", classfile::insn::opcode_name(code))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atom_negation_forms() {
        let neg = |p: &str| match atom(p, None) {
            Cond::Atom { neg, .. } => atom_text(&neg).to_string(),
            other => format!("{other:?}"),
        };
        assert_eq!(neg("!x"), "x");
        // 与 Python 同口径：`(a && b)` 顶层无运算符，按简单操作数剥 `!`
        assert_eq!(neg("!(a && b)"), "(a && b)");
        assert_eq!(neg("!(a) && b"), "!(!(a) && b)");
        assert_eq!(neg("a.b()"), "!a.b()");
        assert_eq!(neg("a == b"), "!(a == b)");
        assert!(matches!(atom(" true ", None), Cond::Const(true)));
    }

    #[test]
    fn cmp_text() {
        assert_eq!(cmp_op(opc::IFEQ, "x", "").unwrap(), "(x==0)");
        assert_eq!(cmp_op(opc::IF_ICMPLT, "a < b", "c").unwrap(), "(a < b) < c");
        assert_eq!(cmp_op(opc::IF_ICMPEQ, "f() -> x", "c").unwrap(), "(f() -> x) == c");
        assert_eq!(cmp_op(opc::IF_ICMPEQ, "|x| => y", "c").unwrap(), "(|x| => y) == c");
        assert_eq!(neg_cmp_op(op::IFNULL, "v", "").unwrap(), "!_is_jnull(&v)");
        let c = Cond::and(atom("a", None), Cond::or(atom("b", None), atom("c", None)));
        assert_eq!(render_cond(&c), "a && (b || c)");
    }
}
