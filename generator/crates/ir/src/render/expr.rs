//! 表达式渲染（← `render_expr` / `_render_block_expr` / `_render_if_expr`），带优先级括号。

use super::lit::write_lit;
use super::ty::{write_generics, write_path, write_segments, write_type, PathCtx};
use super::{Renderer, INDENT};
use crate::{BinOp, BlockExpr, Expr, FnPath, IfExpr, Lit, MacroCall};

/// 运算符优先级（数字越大越紧；与 Python `_PREC` 一致）。
pub(crate) fn precedence(op: BinOp) -> u8 {
    match op {
        BinOp::Or => 1,
        BinOp::And => 2,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => 3,
        BinOp::BitOr => 4,
        BinOp::BitXor => 5,
        BinOp::BitAnd => 6,
        BinOp::Shl | BinOp::Shr => 7,
        BinOp::Add | BinOp::Sub => 8,
        BinOp::Mul | BinOp::Div | BinOp::Rem => 9,
    }
}

impl Renderer<'_> {
    pub(crate) fn write_expr(&self, out: &mut String, e: &Expr) {
        match e {
            Expr::Lit(l) => write_lit(self, out, l),
            Expr::Var(v) => out.push_str(v.as_str()),
            Expr::Binary { op, lhs, rhs } => {
                let prec = precedence(*op);
                self.write_operand(out, lhs, prec);
                out.push(' ');
                out.push_str(op.as_str());
                out.push(' ');
                // 右操作数 +1：同级右侧加括号，保持左结合
                self.write_operand(out, rhs, prec + 1);
            }
            Expr::Unary { op, expr } => {
                out.push_str(op.as_str());
                self.write_expr(out, expr);
            }
            Expr::Call { func, args } => {
                self.write_fn_path(out, func);
                self.write_args(out, args);
            }
            Expr::MethodCall { recv, method, turbofish, args } => {
                self.write_expr(out, recv);
                out.push('.');
                out.push_str(method.as_str());
                write_generics(out, turbofish, PathCtx::Expr);
                self.write_args(out, args);
            }
            Expr::Field { recv, name } => {
                self.write_expr(out, recv);
                out.push('.');
                out.push_str(name.as_str());
            }
            Expr::Index { recv, index } => {
                self.write_expr(out, recv);
                out.push('[');
                self.write_expr(out, index);
                out.push(']');
            }
            Expr::Cast { expr, ty, outer_paren } => {
                if *outer_paren {
                    out.push('(');
                }
                self.write_expr(out, expr);
                out.push_str(" as ");
                write_type(out, ty);
                if *outer_paren {
                    out.push(')');
                }
            }
            Expr::Ref { expr, mutable } => {
                out.push_str(if *mutable { "&mut " } else { "&" });
                self.write_expr(out, expr);
            }
            Expr::Deref(inner) => {
                out.push('*');
                self.write_expr(out, inner);
            }
            Expr::Block(b) => self.write_block_expr(out, b),
            Expr::If(i) => self.write_if_expr(out, i),
            Expr::Macro(m) => self.write_macro(out, m),
            Expr::Raw(r) => out.push_str(r.as_str()),
            Expr::NewPending { class } => {
                out.push_str(&self.short(class));
                out.push_str("::new()");
            }
            Expr::StaticField(sf) => {
                out.push_str(&self.short(&sf.class));
                write_generics(out, &sf.turbofish, PathCtx::Expr);
                out.push_str("::");
                out.push_str(sf.field.as_str());
                out.push_str("()?");
            }
            Expr::Upcast { expr, wrap } => self.write_upcast(out, expr, *wrap),
            Expr::CheckCast(c) => self.write_checkcast(out, c),
            Expr::InstanceOf { expr, binary } => {
                out.push('(');
                self.write_expr(out, expr);
                out.push_str(").is_instance_of(");
                super::lit::write_str_token(out, binary);
                out.push(')');
            }
            Expr::Try(inner) => {
                self.write_expr(out, inner);
                out.push('?');
            }
            Expr::Paren(inner) => {
                out.push('(');
                self.write_expr(out, inner);
                out.push(')');
            }
        }
    }

    /// 二元运算的操作数：子节点是更松的二元运算时加括号。
    fn write_operand(&self, out: &mut String, e: &Expr, parent_prec: u8) {
        let needs = matches!(e, Expr::Binary { op, .. } if precedence(*op) < parent_prec);
        if needs {
            out.push('(');
        }
        self.write_expr(out, e);
        if needs {
            out.push(')');
        }
    }

    pub(crate) fn write_args(&self, out: &mut String, args: &[Expr]) {
        out.push('(');
        for (i, a) in args.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            self.write_expr(out, a);
        }
        out.push(')');
    }

    fn write_fn_path(&self, out: &mut String, func: &FnPath) {
        match func {
            FnPath::Path(p) => write_path(out, p, PathCtx::Expr),
            FnPath::Qualified { self_ty, trait_, rest } => {
                out.push('<');
                write_type(out, self_ty);
                if let Some(t) = trait_ {
                    out.push_str(" as ");
                    write_path(out, t, PathCtx::Type);
                }
                out.push('>');
                if !rest.is_empty() {
                    out.push_str("::");
                    write_segments(out, rest, PathCtx::Expr);
                }
            }
        }
    }

    fn write_macro(&self, out: &mut String, m: &MacroCall) {
        out.push_str(m.name.as_str());
        out.push('!');
        self.write_args(out, &m.args);
    }

    /// `{\n    stmt\n    tail\n}`：块内语句按 0 级渲染后统一加一级缩进（多行语句的
    /// 后续行不再缩进，与 Python 一致）。
    fn write_block_expr(&self, out: &mut String, b: &BlockExpr) {
        out.push_str("{\n");
        for s in &b.stmts {
            out.push_str(INDENT);
            self.write_stmt(out, s, 0);
            out.push('\n');
        }
        if let Some(t) = &b.tail {
            out.push_str(INDENT);
            self.write_expr(out, t);
            out.push('\n');
        }
        out.push('}');
    }

    /// 条件为字面量 false：跳过 then 块只渲染 else 块（无 else 为 `{}`），
    /// 避免死代码中的类型擦除不一致（E0308）。
    fn write_if_expr(&self, out: &mut String, e: &IfExpr) {
        if matches!(*e.cond, Expr::Lit(Lit::Bool(false))) {
            match &e.else_ {
                Some(b) => self.write_block_expr(out, b),
                None => out.push_str("{}"),
            }
            return;
        }
        out.push_str("if ");
        self.write_expr(out, &e.cond);
        out.push(' ');
        self.write_block_expr(out, &e.then);
        if let Some(b) = &e.else_ {
            out.push_str(" else ");
            self.write_block_expr(out, b);
        }
    }
}
