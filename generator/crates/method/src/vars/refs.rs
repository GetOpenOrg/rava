//! 变量引用检测（← `vars._ir_names` / `_refs`）：语句按 IR 收集变量名（字段名 / 方法名 /
//! 类型 / 字符串字面量不计，Raw 文本与宏实参仍按标识符扫描）；结构行与文本行按词边界扫描。

use std::cell::OnceCell;
use std::collections::BTreeSet;

use instr::InstrEnv;
use ir::{ArmBody, BlockExpr, ElseBranch, Expr, Stmt};

use crate::entry::{Entry, Item};
use crate::text;

fn scan(text: &str, out: &mut BTreeSet<String>) {
    out.extend(text::identifiers(text).into_iter().map(str::to_string));
}

fn scan_no_str(text: &str, out: &mut BTreeSet<String>) {
    scan(&text::strip_str_lits(text), out);
}

fn block_names(env: &InstrEnv, b: &BlockExpr, out: &mut BTreeSet<String>) {
    for s in &b.stmts {
        stmt_names(env, s, out);
    }
    if let Some(t) = &b.tail {
        expr_names(env, t, out);
    }
}

pub fn expr_names(env: &InstrEnv, e: &Expr, out: &mut BTreeSet<String>) {
    match e {
        Expr::Var(v) => {
            out.insert(v.as_str().to_string());
        }
        Expr::Raw(r) => scan(&r.0, out),
        // Python `Lit.value` 是渲染文本：去掉字符串字面量后扫描
        Expr::Lit(_) => scan_no_str(&text::expr(env, e), out),
        // Python 宏实参是原始文本
        Expr::Macro(m) => {
            for a in &m.args {
                scan_no_str(&text::expr(env, a), out);
            }
        }
        // Python `Call.func` 是路径文本（含 turbofish）
        Expr::Call { func, args } => {
            let head = text::expr(env, &Expr::Call { func: func.clone(), args: Vec::new() });
            scan(head.strip_suffix("()").unwrap_or(&head), out);
            for a in args {
                expr_names(env, a, out);
            }
        }
        Expr::MethodCall { recv, args, .. } => {
            expr_names(env, recv, out);
            for a in args {
                expr_names(env, a, out);
            }
        }
        Expr::Field { recv, .. } => expr_names(env, recv, out),
        Expr::Binary { lhs, rhs, .. } => {
            expr_names(env, lhs, out);
            expr_names(env, rhs, out);
        }
        Expr::Index { recv, index } => {
            expr_names(env, recv, out);
            expr_names(env, index, out);
        }
        Expr::Unary { expr, .. }
        | Expr::Cast { expr, .. }
        | Expr::Ref { expr, .. }
        | Expr::Upcast { expr, .. }
        | Expr::InstanceOf { expr, .. } => expr_names(env, expr, out),
        Expr::Deref(x) | Expr::Try(x) | Expr::Paren(x) => expr_names(env, x, out),
        Expr::CheckCast(c) => expr_names(env, &c.expr, out),
        Expr::Block(b) => block_names(env, b, out),
        Expr::If(i) => {
            expr_names(env, &i.cond, out);
            block_names(env, &i.then, out);
            if let Some(e) = &i.else_ {
                block_names(env, e, out);
            }
        }
        Expr::NewPending { .. } | Expr::StaticField(_) => {}
    }
}

fn stmts_names(env: &InstrEnv, ss: &[Stmt], out: &mut BTreeSet<String>) {
    for s in ss {
        stmt_names(env, s, out);
    }
}

pub fn stmt_names(env: &InstrEnv, s: &Stmt, out: &mut BTreeSet<String>) {
    match s {
        // `let _ = e;`：Python 侧为原文语句，按渲染文本扫描标识符
        Stmt::Let(l) if l.name.is_discard() => scan(&text::stmt(env, s), out),
        Stmt::Let(l) => {
            out.insert(l.name.as_str().to_string());
            if let Some(v) = &l.value {
                expr_names(env, v, out);
            }
        }
        Stmt::Assign(a) => {
            expr_names(env, &a.target, out);
            expr_names(env, &a.value, out);
        }
        Stmt::Expr(e) | Stmt::Return(Some(e)) => expr_names(env, e, out),
        Stmt::Return(None) | Stmt::Break(_) | Stmt::Continue(_) => {}
        Stmt::Raw(r) => scan(&r.0, out),
        Stmt::Loop(l) => stmts_names(env, &l.body, out),
        Stmt::While(w) => {
            expr_names(env, &w.cond, out);
            stmts_names(env, &w.body, out);
        }
        Stmt::If(i) => {
            expr_names(env, &i.cond, out);
            stmts_names(env, &i.then, out);
            match &i.else_ {
                ElseBranch::None => {}
                ElseBranch::Block(b) => stmts_names(env, b, out),
                ElseBranch::If(x) => stmt_names(env, &Stmt::If((**x).clone()), out),
            }
        }
        Stmt::LabeledBlock { body, .. } => stmts_names(env, body, out),
        Stmt::Match(m) => {
            expr_names(env, &m.scrutinee, out);
            for arm in &m.arms {
                match &arm.body {
                    ArmBody::Block(b) => stmts_names(env, b, out),
                    ArmBody::Expr(e) => expr_names(env, e, out),
                }
            }
        }
        Stmt::JavaTry(t) => {
            stmts_names(env, &t.body, out);
            for c in &t.catches {
                stmts_names(env, &c.body, out);
            }
        }
    }
}

/// `\blet\s+(?:mut\s+)?NAME\b`
fn has_let_decl(text: &str, name: &str) -> bool {
    text::word_positions(text, "let").into_iter().any(|p| {
        let rest = &text[p + 3..];
        let r = rest.trim_start();
        if r.len() == rest.len() {
            return false;
        }
        let r = match r.strip_prefix("mut") {
            Some(x) if x.starts_with(char::is_whitespace) => x.trim_start(),
            _ => r,
        };
        r.strip_prefix(name).is_some_and(|x| !x.starts_with(text::is_word_char))
    })
}

/// 条目是否引用 name、是否为 name 的 let 声明。`rendered` 为本轮开始时的条目文本
/// （已删除条目按其原文本扫描，同 Python `(None)` 条目走文本分支）
pub fn refs(env: &InstrEnv, e: &Entry, rendered: &str, name: &str) -> (bool, bool) {
    match &e.item {
        Item::Stmt(s) => {
            let mut names = BTreeSet::new();
            stmt_names(env, s, &mut names);
            let is_let = matches!(&**s, Stmt::Let(l) if !l.name.is_discard() && l.name.as_str() == name);
            (names.contains(name), is_let)
        }
        Item::Line(_) | Item::Struct { .. } | Item::Removed => {
            let hit = text::has_word(rendered, name);
            (hit, hit && has_let_decl(rendered, name))
        }
    }
}

/// 条目不变期间（一轮扫描的只读阶段）的引用名缓存：语句条目的变量名集按条目惰性求一次，
/// 结果与逐次调用 [`refs`] 相同。条目被改写后不得继续使用
pub struct RefCache {
    names: Vec<OnceCell<BTreeSet<String>>>,
}

impl RefCache {
    pub fn new(len: usize) -> RefCache {
        RefCache { names: (0..len).map(|_| OnceCell::new()).collect() }
    }

    /// 同 [`refs`]`(env, &entries[k], rendered, name)`
    pub fn refs(&self, env: &InstrEnv, entries: &[Entry], k: usize, rendered: &str, name: &str) -> (bool, bool) {
        match &entries[k].item {
            Item::Stmt(s) => {
                let names = self.names[k].get_or_init(|| {
                    let mut out = BTreeSet::new();
                    stmt_names(env, s, &mut out);
                    out
                });
                let is_let = matches!(&**s, Stmt::Let(l) if !l.name.is_discard() && l.name.as_str() == name);
                (names.contains(name), is_let)
            }
            _ => refs(env, &entries[k], rendered, name),
        }
    }
}

/// 条目文本：文本行原样，语句渲染（← Pass 2 `rendered`）
pub fn render_entries(env: &InstrEnv, entries: &[Entry]) -> Vec<String> {
    entries
        .iter()
        .map(|e| match &e.item {
            Item::Stmt(s) => text::stmt(env, s),
            Item::Line(t) | Item::Struct { text: t, .. } => t.clone(),
            Item::Removed => String::new(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::has_let_decl;

    #[test]
    fn let_decl() {
        assert!(has_let_decl("    let mut x: i32;", "x"));
        assert!(has_let_decl("let x = 1;", "x"));
        assert!(!has_let_decl("let xy = 1;", "x"));
        assert!(!has_let_decl("letx = 1;", "x"));
        assert!(!has_let_decl("let mutx = 1;", "x"));
        assert!(has_let_decl("let mutx = 1;", "mutx"));
    }
}
