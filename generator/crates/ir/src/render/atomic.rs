//! 表达式原子性（← `is_atomic_rs(expr_str)`）：决定 `.into()` 等后缀前是否加括号。
//!
//! Python 对渲染后的文本扫描：顶层（括号深度 0）只允许字母数字与 `_ . : ?`。
//! 这里按节点结构给出同一判定——括号 / 花括号内的子节点不影响结果，只看顶层
//! 会出现的记号；文本载体（Raw、字面量记号、短名）才做字符扫描。

use super::lit::write_lit;
use super::Renderer;
use crate::{CastMode, Expr, FnPath, UpcastWrap};

impl Renderer<'_> {
    /// 渲染结果是否原子（调用链 / 路径），与 Python `is_atomic_rs(render_expr(e))` 同判定。
    pub fn is_atomic(&self, e: &Expr) -> bool {
        match e {
            Expr::Lit(l) => {
                let mut s = String::new();
                write_lit(self, &mut s, l);
                scan_atomic(&s)
            }
            Expr::Var(_) | Expr::Paren(_) | Expr::Block(_) | Expr::InstanceOf { .. } => true,
            Expr::Binary { .. } | Expr::Unary { .. } | Expr::Ref { .. } | Expr::Deref(_) => false,
            Expr::Macro(_) => false,
            Expr::Raw(r) => scan_atomic(r.as_str()),
            Expr::Call { func, .. } => match func {
                // turbofish 的 `<` 在顶层 → 非原子；限定路径以 `<` 开头 → 非原子
                FnPath::Path(p) => p.segments.iter().all(|s| s.generics.is_empty()),
                FnPath::Qualified { .. } => false,
            },
            Expr::MethodCall { recv, turbofish, .. } => turbofish.is_empty() && self.is_atomic(recv),
            Expr::Field { recv, .. } | Expr::Index { recv, .. } => self.is_atomic(recv),
            Expr::Cast { outer_paren, .. } => *outer_paren,
            // 条件恒 false 时只渲染块（`{..}` / `{}`），否则 `if ..` 含空格
            Expr::If(i) => matches!(*i.cond, Expr::Lit(crate::Lit::Bool(false))),
            Expr::NewPending { class } => scan_atomic(&self.short(class)),
            Expr::StaticField(sf) => sf.turbofish.is_empty() && scan_atomic(&self.short(&sf.class)),
            Expr::Upcast { expr, wrap } => match wrap {
                UpcastWrap::Bare => self.is_atomic(expr),
                _ => true,
            },
            Expr::CheckCast(c) => match &c.mode {
                CastMode::CheckedInterface { .. } => true,
                // `try_cast::<T>` / `try_cast_array::<E>` 的 turbofish；`<T as From<Object>>` 以 `<` 开头
                CastMode::Checked { .. } | CastMode::Unchecked => false,
            },
            Expr::Try(inner) => self.is_atomic(inner),
        }
    }
}

/// 文本载体的原子性扫描（Python `is_atomic_rs` 原样）：去首尾空白后，括号深度 0 处
/// 只允许字母数字与 `_ . : ?`。只用于 Raw / 字面量记号 / 短名等叶子文本。
pub(crate) fn scan_atomic(text: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in text.trim().chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if depth == 0 && !(c.is_alphanumeric() || matches!(c, '_' | '.' | ':' | '?')) => return false,
            _ => {}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::scan_atomic;

    /// ← test_upcast_expr.py::test_is_atomic
    #[test]
    fn scan_matches_python() {
        assert!(scan_atomic("a.b::c()?"));
        assert!(scan_atomic("f(a + b)"));
        assert!(!scan_atomic("a + b"));
        assert!(!scan_atomic("-x"));
        assert!(scan_atomic("  x  "));
        assert!(!scan_atomic("if c { a } else { b }"));
        assert!(scan_atomic("(a)"));
    }
}
