//! 语句渲染（← `render_stmt` 与 `method/emit.py` 的结构行形态）。
//!
//! 叶子语句只在首行前加缩进（多行 Raw 的后续行原样，与 Python 一致）；
//! 控制流语句的块头 / 块尾 / 衔接行与 `method/emit.py` 产出的结构行逐字同形，
//! 子语句缩进一级（match 臂体、try / catch 体缩进两级）。

use super::{pad, Renderer};
use crate::{ArmBody, CatchClause, ElseBranch, IfStmt, Label, MatchStmt, Pattern, Stmt, TryStmt};

impl Renderer<'_> {
    pub(crate) fn write_stmt(&self, out: &mut String, s: &Stmt, indent: usize) {
        pad(out, indent);
        match s {
            Stmt::Let(l) => {
                out.push_str(if l.mutable { "let mut " } else { "let " });
                out.push_str(l.name.as_str());
                if let Some(t) = &l.ty {
                    out.push_str(": ");
                    super::ty::write_type(out, t);
                }
                if let Some(v) = &l.value {
                    out.push_str(" = ");
                    self.write_expr(out, v);
                }
                out.push(';');
            }
            Stmt::Assign(a) => {
                self.write_expr(out, &a.target);
                out.push_str(" = ");
                self.write_expr(out, &a.value);
                out.push(';');
            }
            Stmt::Expr(e) => {
                self.write_expr(out, e);
                out.push(';');
            }
            Stmt::Return(v) => match v {
                Some(e) => {
                    out.push_str("return ");
                    self.write_expr(out, e);
                    out.push(';');
                }
                None => out.push_str("return;"),
            },
            Stmt::Break(l) => write_jump(out, "break", l.as_ref()),
            Stmt::Continue(l) => write_jump(out, "continue", l.as_ref()),
            Stmt::Loop(l) => {
                write_label(out, l.label.as_ref());
                out.push_str("loop {");
                self.write_body_close(out, &l.body, indent);
            }
            Stmt::While(w) => {
                write_label(out, w.label.as_ref());
                out.push_str("while ");
                self.write_expr(out, &w.cond);
                out.push_str(" {");
                self.write_body_close(out, &w.body, indent);
            }
            Stmt::If(i) => self.write_if(out, i, indent),
            Stmt::LabeledBlock { label, body } => {
                out.push_str(&label.to_string());
                out.push_str(": {");
                self.write_body_close(out, body, indent);
            }
            Stmt::Match(m) => self.write_match(out, m, indent),
            Stmt::JavaTry(t) => self.write_try(out, t, indent),
            Stmt::Raw(r) => out.push_str(r.as_str()),
        }
    }

    /// 子语句（缩进 `indent`）逐行输出，每行前换行。
    fn write_body(&self, out: &mut String, body: &[Stmt], indent: usize) {
        for s in body {
            out.push('\n');
            self.write_stmt(out, s, indent);
        }
    }

    /// 子语句（缩进 indent+1）+ 换行 + `indent` 级的 `}`。
    fn write_body_close(&self, out: &mut String, body: &[Stmt], indent: usize) {
        self.write_body(out, body, indent + 1);
        out.push('\n');
        pad(out, indent);
        out.push('}');
    }

    /// `if c {` .. `} else if c2 {` .. `} else {` .. `}`（调用方已输出首行缩进）。
    fn write_if(&self, out: &mut String, i: &IfStmt, indent: usize) {
        out.push_str("if ");
        self.write_expr(out, &i.cond);
        out.push_str(" {");
        self.write_body(out, &i.then, indent + 1);
        out.push('\n');
        pad(out, indent);
        match &i.else_ {
            ElseBranch::None => out.push('}'),
            ElseBranch::Block(b) => {
                out.push_str("} else {");
                self.write_body_close(out, b, indent);
            }
            ElseBranch::If(next) => {
                out.push_str("} else ");
                self.write_if(out, next, indent);
            }
        }
    }

    fn write_match(&self, out: &mut String, m: &MatchStmt, indent: usize) {
        out.push_str("match ");
        self.write_expr(out, &m.scrutinee);
        out.push_str(" {");
        for arm in &m.arms {
            out.push('\n');
            pad(out, indent + 1);
            write_pattern(self, out, &arm.pattern);
            out.push_str(" => ");
            match &arm.body {
                ArmBody::Block(b) => {
                    out.push('{');
                    self.write_body_close(out, b, indent + 1);
                }
                ArmBody::Expr(e) => {
                    self.write_expr(out, e);
                    out.push(',');
                }
            }
        }
        out.push('\n');
        pad(out, indent);
        out.push('}');
    }

    fn write_try(&self, out: &mut String, t: &TryStmt, indent: usize) {
        out.push_str("java_try! {\n");
        pad(out, indent + 1);
        out.push_str("try {");
        self.write_body(out, &t.body, indent + 2);
        for c in &t.catches {
            out.push('\n');
            pad(out, indent + 1);
            out.push_str("} ");
            self.write_catch_head(out, c);
            out.push_str(" {");
            self.write_body(out, &c.body, indent + 2);
        }
        out.push('\n');
        pad(out, indent + 1);
        out.push_str("}\n");
        pad(out, indent);
        out.push('}');
    }

    /// `catch (e)` / `catch (e: A)` / `catch (e: A | B as Lub)`（← `try_catch.catch_head`）。
    fn write_catch_head(&self, out: &mut String, c: &CatchClause) {
        out.push_str("catch (");
        out.push_str(c.bind.as_str());
        if !c.types.is_empty() {
            out.push_str(": ");
            for (i, t) in c.types.iter().enumerate() {
                if i > 0 {
                    out.push_str(" | ");
                }
                out.push_str(&self.short(t));
            }
            if let Some(lub) = &c.lub {
                out.push_str(" as ");
                super::ty::write_type(out, lub);
            }
        }
        out.push(')');
    }
}

fn write_jump(out: &mut String, keyword: &str, label: Option<&Label>) {
    out.push_str(keyword);
    if let Some(l) = label {
        out.push(' ');
        out.push_str(&l.to_string());
    }
    out.push(';');
}

fn write_label(out: &mut String, label: Option<&Label>) {
    if let Some(l) = label {
        out.push_str(&l.to_string());
        out.push_str(": ");
    }
}

fn write_pattern(rd: &Renderer<'_>, out: &mut String, p: &Pattern) {
    match p {
        Pattern::Wildcard => out.push('_'),
        Pattern::Alts(lits) => {
            for (i, l) in lits.iter().enumerate() {
                if i > 0 {
                    out.push_str(" | ");
                }
                super::lit::write_lit(rd, out, l);
            }
        }
    }
}
