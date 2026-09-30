//! Python IR（dump_ir.py 的 dataclass JSON，标签字段 `_t`）→ Rust IR。
//!
//! 文本载体经 [`super::parse`] 结构化；解析失败或结构化后渲染与原文不一致的，
//! 整体退回 [`Raw`] 并按类别计数（[`Fallbacks`]）。未知节点直接报错。

use super::parse::{parse_expr, parse_fn_path, parse_lit, parse_turbofish, parse_type};
use ir::{
    AssignStmt, BinOp, CastExpr, CastMode, Expr, Ident, LetStmt, Lit, Raw, Renderer, StaticFieldRef, Stmt,
    Type, UpcastWrap, VarOrigin,
};
use serde_json::Value;
use std::collections::BTreeMap;

pub type Res<T> = Result<T, String>;

/// 退回 Raw 的次数（类别 → 次数）与结构化成功次数。
#[derive(Default, Debug)]
pub struct Fallbacks {
    pub raw: BTreeMap<&'static str, usize>,
    pub structured: BTreeMap<&'static str, usize>,
    /// 退回 Raw 的原文（类别, 文本），供差异清单登记
    pub raw_samples: Vec<(&'static str, String)>,
}

impl Fallbacks {
    fn hit(&mut self, cat: &'static str, ok: bool) {
        *if ok { &mut self.structured } else { &mut self.raw }.entry(cat).or_default() += 1;
    }
}

pub struct Conv<'a> {
    pub rd: Renderer<'a>,
    pub fb: Fallbacks,
}

fn tag(v: &Value) -> Res<&str> {
    v.get("_t").and_then(Value::as_str).ok_or_else(|| format!("缺 _t：{v}"))
}

fn s<'v>(v: &'v Value, k: &str) -> Res<&'v str> {
    v.get(k).and_then(Value::as_str).ok_or_else(|| format!("缺字符串字段 {k}：{v}"))
}

fn b(v: &Value, k: &str) -> bool {
    v.get(k).and_then(Value::as_bool).unwrap_or(false)
}

fn ident(t: &str) -> Res<Ident> {
    Ident::new(t).map_err(|e| e.to_string())
}

fn raw(t: &str) -> Expr {
    Expr::Raw(Raw(t.to_string()))
}

fn opt<'v>(v: &'v Value, k: &str) -> Option<&'v Value> {
    v.get(k).filter(|x| !x.is_null())
}

impl Conv<'_> {
    pub fn ty(&mut self, v: &Value) -> Res<Type> {
        match tag(v)? {
            "RsPrimitive" | "RsNamed" => {
                let mut name = s(v, "name")?.to_string();
                let path = v.get("path").and_then(Value::as_str).unwrap_or("");
                if !path.is_empty() {
                    name = format!("{path}::{name}");
                }
                self.ty_text(&name)
            }
            "RsInfer" => Ok(Type::Infer),
            t => Err(format!("未支持的类型节点 {t}")),
        }
    }

    /// 类型文本（`RsNamed.name`、`CastExpr.target`）：严格解析，失败即报错（类型无 Raw）。
    pub fn ty_text(&mut self, text: &str) -> Res<Type> {
        let t = parse_type(text).ok_or_else(|| format!("类型解析失败：{text}"))?;
        let back = self.rd.ty(&t);
        if back != text {
            return Err(format!("类型往返不一致：{text} → {back}"));
        }
        Ok(t)
    }

    /// 表达式文本：解析 + 往返校验，失败退回 Raw。
    pub fn expr_text(&mut self, cat: &'static str, text: &str) -> Expr {
        let e = parse_expr(text).filter(|e| self.rd.expr(e) == text);
        self.fb.hit(cat, e.is_some());
        if e.is_none() {
            self.fb.raw_samples.push((cat, text.to_string()));
        }
        e.unwrap_or_else(|| raw(text))
    }

    fn exprs(&mut self, v: &Value, k: &str) -> Res<Vec<Expr>> {
        v.get(k).and_then(Value::as_array).ok_or_else(|| format!("缺列表 {k}"))?.iter().map(|a| self.expr(a)).collect()
    }

    fn boxed(&mut self, v: &Value, k: &str) -> Res<Box<Expr>> {
        Ok(Box::new(self.expr(v.get(k).ok_or_else(|| format!("缺 {k}"))?)?))
    }

    pub fn expr(&mut self, v: &Value) -> Res<Expr> {
        Ok(match tag(v)? {
            "Var" => {
                let name = s(v, "name")?;
                match Ident::new(name) {
                    Ok(i) => Expr::Var(i),
                    Err(_) => self.expr_text("Var(表达式文本)", name),
                }
            }
            "Lit" => {
                let text = s(v, "value")?;
                match parse_lit(text).filter(|l| self.rd.expr(&Expr::Lit(l.clone())) == text) {
                    Some(l) => {
                        self.fb.hit("Lit(字面量)", true);
                        Expr::Lit(l)
                    }
                    None => self.expr_text("Lit(表达式文本)", text),
                }
            }
            "Call" => {
                let func = s(v, "func")?;
                let args = self.exprs(v, "args")?;
                let fp = parse_fn_path(func);
                self.fb.hit("Call.func", fp.is_some());
                match fp {
                    Some(func) => Expr::Call { func, args },
                    None => {
                        self.fb.raw_samples.push(("Call.func", func.to_string()));
                        let parts: Vec<String> = args.iter().map(|a| self.rd.expr(a)).collect();
                        raw(&format!("{func}({})", parts.join(", ")))
                    }
                }
            }
            "MethodCall" => Expr::MethodCall {
                recv: self.boxed(v, "recv")?,
                method: ident(s(v, "method")?)?,
                turbofish: Vec::new(),
                args: self.exprs(v, "args")?,
            },
            "TryExpr" => Expr::Try(self.boxed(v, "inner")?),
            "Paren" => Expr::Paren(self.boxed(v, "inner")?),
            "RefExpr" => Expr::Ref { expr: self.boxed(v, "expr")?, mutable: b(v, "mutable") },
            "RawExpr" | "CondExpr" => raw(s(v, "code")?),
            "BinOp" => {
                let op = s(v, "op")?;
                Expr::Binary {
                    op: BinOp::from_symbol(op).ok_or_else(|| format!("未知运算符 {op}"))?,
                    lhs: self.boxed(v, "left")?,
                    rhs: self.boxed(v, "right")?,
                }
            }
            "Cast" => Expr::Cast {
                expr: self.boxed(v, "expr")?,
                ty: self.ty(v.get("ty").ok_or("Cast 缺 ty")?)?,
                outer_paren: b(v, "outer"),
            },
            "StaticFieldRef" => {
                let tf = s(v, "turbofish")?;
                Expr::StaticField(StaticFieldRef {
                    class: s(v, "class_name")?.to_string(),
                    field: ident(s(v, "field_name")?)?,
                    ty: match opt(v, "ty") {
                        Some(t) => self.ty(t)?,
                        None => Type::Infer,
                    },
                    turbofish: parse_turbofish(tf).ok_or_else(|| format!("turbofish 解析失败：{tf}"))?,
                })
            }
            "NewPendingExpr" => Expr::NewPending { class: s(v, "class_name")?.to_string() },
            "InstanceOfExpr" => {
                Expr::InstanceOf { expr: self.boxed(v, "expr")?, binary: s(v, "binary_name")?.to_string() }
            }
            "UpcastExpr" => {
                let wrap = match s(v, "wrap")? {
                    "owned" => UpcastWrap::Owned,
                    "auto" => UpcastWrap::Auto,
                    w => return Err(format!("未知 UpcastExpr.wrap {w}")),
                };
                Expr::Upcast { expr: self.boxed(v, "expr")?, wrap }
            }
            "CastExpr" => Expr::CheckCast(self.cast(v)?),
            t => return Err(format!("未支持的表达式节点 {t}")),
        })
    }

    pub fn cast(&mut self, v: &Value) -> Res<CastExpr> {
        let binary = s(v, "binary_name")?.to_string();
        let mode = match (b(v, "checked"), b(v, "interface_target")) {
            (false, _) => CastMode::Unchecked,
            (true, true) => CastMode::CheckedInterface { binary },
            (true, false) => CastMode::Checked { binary },
        };
        Ok(CastExpr {
            expr: self.boxed(v, "expr")?,
            target: self.ty_text(s(v, "target")?)?,
            mode,
            box_first: b(v, "box_first"),
        })
    }

    fn origin(&mut self, v: &Value) -> Res<VarOrigin> {
        Ok(VarOrigin {
            value_ty: opt(v, "value_ty").map(|t| self.ty(t)).transpose()?,
            slot: opt(v, "slot").and_then(Value::as_u64).map(|x| x as u16),
            bind_off: opt(v, "bind_off").and_then(Value::as_u64).map(|x| x as u32),
        })
    }

    pub fn stmt(&mut self, v: &Value) -> Res<Stmt> {
        Ok(match tag(v)? {
            "LetStmt" => Stmt::Let(LetStmt {
                name: ident(s(v, "name")?)?,
                ty: opt(v, "ty").map(|t| self.ty(t)).transpose()?,
                mutable: b(v, "mutable"),
                value: opt(v, "value").map(|e| self.expr(e)).transpose()?,
                origin: self.origin(v)?,
            }),
            "AssignStmt" => Stmt::Assign(AssignStmt {
                target: self.expr(v.get("target").ok_or("缺 target")?)?,
                value: self.expr(v.get("value").ok_or("缺 value")?)?,
                origin: self.origin(v)?,
            }),
            "ExprStmt" => Stmt::Expr(self.expr(v.get("expr").ok_or("缺 expr")?)?),
            "ReturnStmt" => Stmt::Return(opt(v, "value").map(|e| self.expr(e)).transpose()?),
            "BreakStmt" => Stmt::Break(None),
            "ContinueStmt" => Stmt::Continue(None),
            "RawStmt" => Stmt::Raw(Raw(s(v, "code")?.to_string())),
            t => return Err(format!("未支持的语句节点 {t}")),
        })
    }

    /// `upcast_expr(src: str, wrap)` 的字符串输入：解析 + 往返，失败以 Raw 为源。
    pub fn upcast(&mut self, src: &str, wrap: &str) -> Res<Expr> {
        let wrap = match wrap {
            "none" => UpcastWrap::Bare,
            "clone" => UpcastWrap::Clone,
            "paren" => UpcastWrap::Paren,
            "auto" => UpcastWrap::Auto,
            w => return Err(format!("未知 wrap {w}")),
        };
        let e = self.expr_text("upcast_expr(字符串源)", src);
        Ok(Expr::upcast(e, wrap))
    }
}

/// 递归收集表达式树中所有子表达式（原子性一致性校验用）。
pub fn walk_exprs<'e>(e: &'e Expr, out: &mut Vec<&'e Expr>) {
    out.push(e);
    match e {
        Expr::Binary { lhs, rhs, .. } => {
            walk_exprs(lhs, out);
            walk_exprs(rhs, out);
        }
        Expr::Call { args, .. } | Expr::Macro(ir::MacroCall { args, .. }) => args.iter().for_each(|a| walk_exprs(a, out)),
        Expr::MethodCall { recv, args, .. } => {
            walk_exprs(recv, out);
            args.iter().for_each(|a| walk_exprs(a, out));
        }
        Expr::Unary { expr, .. }
        | Expr::Field { recv: expr, .. }
        | Expr::Cast { expr, .. }
        | Expr::Ref { expr, .. }
        | Expr::Deref(expr)
        | Expr::Upcast { expr, .. }
        | Expr::InstanceOf { expr, .. }
        | Expr::Try(expr)
        | Expr::Paren(expr) => walk_exprs(expr, out),
        Expr::Index { recv, index } => {
            walk_exprs(recv, out);
            walk_exprs(index, out);
        }
        Expr::CheckCast(c) => walk_exprs(&c.expr, out),
        Expr::Lit(Lit::Null) | Expr::Lit(_) | Expr::Var(_) | Expr::Raw(_) => {}
        Expr::Block(_) | Expr::If(_) | Expr::NewPending { .. } | Expr::StaticField(_) => {}
    }
}
