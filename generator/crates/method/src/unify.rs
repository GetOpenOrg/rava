//! 块级值合并与条件构造（← `method/unify.py`）。
//!
//! - [`jump_condition`]：跳转指令 → 条件（弹操作数、按类型强转比较数）
//! - [`unify_pair`] / [`unify_values`] / [`arm_value`]：汇合点各前驱栈值的类型统一与合并
//! - [`inline_temps`]：条件块内单用临时量内联进条件
//! - [`CondValues`]：由条件跳转物化出的布尔值（Python `CondExpr`）的条件侧表

use std::collections::BTreeMap;

use cfg::opcodes::{is_two_operand_branch, IFEQ, IFNE, IF_ACMPEQ, IF_ACMPNE};
use cfg::Cond;
use classfile::insn::op;
use instr::hierarchy::{common_ref_type, common_ref_type_widening};
use instr::InstrEnv;
use ir::{Expr, Stmt};
use sim::{StackEntry, StackSim, ValueId};
use ty::{Prim, RsType};

use crate::coerce_text::{acmp_operand, icmp_operand, to_object};
use crate::cond_text::{atom, cmp_op, map_atoms, neg_cmp_op, render_cond};
use crate::error::MethodResult;
use crate::text;

/// 物化布尔值的条件侧表：栈值身份 → (物化表达式, 条件)。
/// 表达式未被改写（未经 dup 物化为临时量）时才视为同一 `CondExpr`
#[derive(Debug, Clone, Default)]
pub struct CondValues(BTreeMap<ValueId, (Expr, Cond)>);

impl CondValues {
    pub fn insert(&mut self, id: ValueId, e: Expr, c: Cond) {
        self.0.insert(id, (e, c));
    }

    pub fn get(&self, entry: &StackEntry) -> Option<&Cond> {
        self.0.get(&entry.id).filter(|(e, _)| *e == entry.expr).map(|(_, c)| c)
    }
}

// ── 条件构造 ─────────────────────────────────────────────────────────────

const PRIM_TEXTS: [&str; 13] = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize", "()"];

/// 类型变量形态的短名（`T` / `E1`）：≤2 字符、大写开头、去尾部数字后全字母
fn short_tvar(ty: &str) -> bool {
    !ty.is_empty()
        && ty.chars().count() <= 2
        && ty.starts_with(|c: char| c.is_uppercase())
        && {
            let s = ty.trim_end_matches(|c: char| c.is_ascii_digit());
            !s.is_empty() && s.chars().all(char::is_alphabetic)
        }
}

/// 类型是否以 `.is_jvm_null()` 判空（生成类、JArray）；其余走 `_is_jnull()`
fn uses_jvm_null_method(ty: &str, tparams: &[String]) -> bool {
    if matches!(ty, "Object" | "()" | "") || PRIM_TEXTS.contains(&ty) || tparams.iter().any(|p| p == ty) {
        return false;
    }
    if ["Rc<", "__Shared<", "Vec<", "Box<", "std::"].iter().any(|p| ty.starts_with(p)) {
        return false;
    }
    !short_tvar(ty)
}

fn is_type_var(ty: &str, tparams: &[String]) -> bool {
    tparams.iter().any(|p| p == ty) || short_tvar(ty)
}

/// 弹出条件跳转的操作数，返回「跳转成立」的条件
pub fn jump_condition(env: &InstrEnv, sim: &mut StackSim, conds: &CondValues, opc: u8) -> MethodResult<Cond> {
    if is_two_operand_branch(opc) {
        let b = sim.pop()?;
        let a = sim.pop()?;
        let (a_t, b_t) = (text::expr(env, &a.expr), text::expr(env, &b.expr));
        let (a_s, b_s) = if opc == IF_ACMPEQ || opc == IF_ACMPNE {
            (acmp_operand(env, &a_t, &a.ty), acmp_operand(env, &b_t, &b.ty))
        } else {
            (icmp_operand(&a_t, &a.ty), icmp_operand(&b_t, &b.ty))
        };
        return Ok(atom(&cmp_op(opc, &a_s, &b_s)?, Some(&neg_cmp_op(opc, &a_s, &b_s)?)));
    }
    let a = sim.pop()?;
    if (opc == IFEQ || opc == IFNE) && a.ty == RsType::Prim(Prim::Bool) {
        let base = match conds.get(&a) {
            Some(c) => c.clone(),
            None => atom(&text::expr(env, &a.expr), None),
        };
        return Ok(if opc == IFNE { base } else { base.negate() });
    }
    let a_s = text::expr(env, &a.expr);
    if opc == op::IFNULL || opc == op::IFNONNULL {
        let ty = text::ty(env, &a.ty);
        let is_null = if uses_jvm_null_method(&ty, &env.tparams) {
            atom(&format!("{a_s}.is_jvm_null()"), Some(&format!("!{a_s}.is_jvm_null()")))
        } else if is_type_var(&ty, &env.tparams) {
            // 类型变量实例化为任意引用载体，经 Into<Object> 统一判空
            atom(&format!("_is_jnull_ref(&{a_s})"), Some(&format!("!_is_jnull_ref(&{a_s})")))
        } else {
            atom(&format!("_is_jnull(&{a_s})"), Some(&format!("!_is_jnull(&{a_s})")))
        };
        return Ok(if opc == op::IFNULL { is_null } else { is_null.negate() });
    }
    Ok(atom(&cmp_op(opc, &a_s, "")?, Some(&neg_cmp_op(opc, &a_s, "")?)))
}

// ── 分支臂取值与类型统一 ─────────────────────────────────────────────────

const INT_TYPES: [&str; 8] = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"];
const JVM_INT_FAMILY: [&str; 4] = ["i8", "i16", "u16", "i32"];
const NULL_EXPRS: [&str; 2] = ["Object::default()", "Clone::clone(&Object::default())"];

fn is_int(t: &str) -> bool {
    INT_TYPES.contains(&t)
}

fn is_scalar(t: &str) -> bool {
    is_int(t) || matches!(t, "bool" | "f32" | "f64")
}

/// 留在栈上的值 → 值位置表达式文本（局部变量包 Clone::clone 保活；this → `Clone::clone(this)`）
pub fn arm_value(env: &InstrEnv, e: &StackEntry) -> MethodResult<String> {
    if crate::node::var_name(&e.expr) == Some("this") {
        return Ok("Clone::clone(this)".to_string());
    }
    Ok(text::expr(env, &sim::exprs::clone_moved_var(e.expr.clone(), &e.ty)?))
}

fn from_common(common: &str, v: &str) -> String {
    format!("<{common} as ::std::convert::From<_>>::from({v})")
}

fn from_object(target: &str, v: &str) -> String {
    format!("<{target} as ::std::convert::From<Object>>::from(Object::from({v}))")
}

/// 同基泛型的擦除实例化 `Base<Object, ..>`（实参个数取两侧较大者）
fn erased_instance(t: &RsType, arity: usize) -> RsType {
    match t {
        RsType::Array(_) => RsType::array(RsType::Object),
        RsType::Class { binary, .. } => RsType::class(binary.clone(), RsType::objects(arity)),
        other => other.clone(),
    }
}

/// 两个汇合值统一到同一 Rust 类型：返回 (tv, ev, ty)
pub fn unify_pair(env: &InstrEnv, tv: String, ty: &RsType, ev: String, ety: &RsType) -> (String, String, RsType) {
    let (ts, es) = (text::ty(env, ty), text::ty(env, ety));
    let (mut tv, mut ev, mut out) = (tv, ev, ty.clone());
    if ts == es {
        return (tv, ev, out);
    }
    let tparams = &env.tparams;
    let is_tparam = |s: &str| tparams.iter().any(|p| p == s);
    let same_base = sim::erased_base(ty, env) == sim::erased_base(ety, env) && ts.contains('<') && es.contains('<');
    if ts == "bool" && is_int(&es) {
        // bool 侧只可能来自 0/1 菱形折叠，以 int 为准，bool 经 as 还原 0/1
        tv = format!("({tv} as {es})");
        out = ety.clone();
    } else if es == "bool" && is_int(&ts) {
        ev = format!("({ev} as {ts})");
    } else if NULL_EXPRS.contains(&ev.as_str()) && !is_scalar(&ts) {
        ev = "Default::default()".to_string();
    } else if NULL_EXPRS.contains(&tv.as_str()) && !is_scalar(&es) {
        tv = "Default::default()".to_string();
        out = ety.clone();
    } else if es == "Object" && !is_scalar(&ts) && ts != "Object" && !is_tparam(&ts) {
        // 具体类型臂与擦除 Object 臂汇合：合并点按擦除 Object 落定，具体臂上转
        tv = to_object(env, &tv, ty, false);
        out = RsType::Object;
    } else if ts == "Object" && !is_scalar(&es) && es != "Object" && !is_tparam(&es) {
        ev = to_object(env, &ev, ety, false);
    } else if JVM_INT_FAMILY.contains(&ts.as_str()) && JVM_INT_FAMILY.contains(&es.as_str()) {
        // 两臂同属 JVM 计算类型 int：拓宽到 i32
        if ts != "i32" {
            tv = format!("({tv} as i32)");
        }
        if es != "i32" {
            ev = format!("({ev} as i32)");
        }
        out = RsType::Prim(Prim::I32);
    } else if is_scalar(&ts) || is_scalar(&es) {
        ev = format!("({ev} as {ts})");
    } else if let Some(common) = common_ref_type(&env.ctx, ty, ety) {
        let cs = text::ty(env, &common);
        if ts != cs {
            tv = from_common(&cs, &tv);
        }
        if es != cs {
            ev = from_common(&cs, &ev);
        }
        out = common;
    } else if let Some(common) = common_ref_type_widening(&env.ctx, ty, ety) {
        // 泛型父子类臂（类型实参一致）：合并点取父类
        let cs = text::ty(env, &common);
        if ts != cs {
            tv = from_common(&cs, &tv);
        }
        if es != cs {
            ev = from_common(&cs, &ev);
        }
        out = common;
    } else if same_base && ts.contains("Object") && tparams.iter().any(|t| es.contains(t.as_str())) {
        // 擦除 Object 实例化臂 vs 具体泛型臂：合并点取具体臂类型
        tv = from_object(&es, &tv);
        out = ety.clone();
    } else if same_base && es.contains("Object") && !ts.contains("Object") {
        ev = from_object(&ts, &ev);
    } else if same_base {
        // 同一泛型类的不同实例化：合并点取擦除实例化
        let arity = ty.type_args().len().max(ety.type_args().len());
        let tgt_t = erased_instance(ty, arity);
        let tgt = text::ty(env, &tgt_t);
        if ts != tgt {
            tv = from_object(&tgt, &tv);
        }
        if es != tgt {
            ev = from_object(&tgt, &ev);
        }
        out = tgt_t;
    } else if !is_scalar(&ts) && !is_scalar(&es) {
        // 无公共父类的引用类型：合并点为根类，两臂各自上转
        if ts != "Object" {
            tv = to_object(env, &tv, ty, false);
        }
        if es != "Object" {
            ev = to_object(env, &ev, ety, false);
        }
        out = RsType::Object;
    }
    (tv, ev, out)
}

const HOLE: &str = "\u{0}";

/// 渲染文本 `^(\w+)<(_(?:, _)*)>$` → (基名, 占位个数)
fn infer_holes(ts: &str) -> Option<(String, usize)> {
    let (base, rest) = ts.split_once('<')?;
    let inner = rest.strip_suffix('>')?;
    if base.is_empty() || !base.chars().all(text::is_word_char) {
        return None;
    }
    let parts: Vec<&str> = inner.split(", ").collect();
    parts.iter().all(|p| *p == "_").then(|| (base.to_string(), parts.len()))
}

/// N 个汇合值统一类型：返回 (各值文本, 类型)
pub fn unify_values(env: &InstrEnv, entries: &[StackEntry]) -> MethodResult<(Vec<String>, RsType)> {
    let mut values = vec![arm_value(env, &entries[0])?];
    let mut ty = entries[0].ty.clone();
    for e in &entries[1..] {
        let (hole, ev, nt) = unify_pair(env, HOLE.to_string(), &ty, arm_value(env, e)?, &e.ty);
        ty = nt;
        if hole != HOLE {
            // null 臂可直接成为任何引用类型，不套转换
            values = values
                .into_iter()
                .map(|v| if v == "Default::default()" { v } else { hole.replace(HOLE, &v) })
                .collect();
        }
        values.push(ev);
    }
    // 汇合值是菱形构造结果 `X<_>`：类型实参按擦除形态 Object 落定
    if let Some((base, n)) = infer_holes(&text::ty(env, &ty)) {
        let holes = vec!["_"; n].join(", ");
        let erased = vec!["Object"; n].join(", ");
        values = values
            .into_iter()
            .map(|v| {
                v.replace(&format!("{base}::<{holes}>"), &format!("{base}::<{erased}>"))
                    .replace(&format!("{base}<{holes}>"), &format!("{base}<{erased}>"))
            })
            .collect();
        ty = erased_instance(&ty, n);
    }
    Ok((values, ty))
}

/// 两个栈条目是否为同一值（身份相同，或渲染文本与类型文本均相同）
pub fn same_entry(env: &InstrEnv, a: &StackEntry, b: &StackEntry) -> bool {
    if crate::node::same_value(a, b) {
        return true;
    }
    text::expr(env, &a.expr) == text::expr(env, &b.expr) && text::ty(env, &a.ty) == text::ty(env, &b.ty)
}

pub fn same_stack(env: &InstrEnv, a: &[StackEntry], b: &[StackEntry]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| same_entry(env, x, y))
}

// ── 临时变量回填：`let _tN = call?; if _tN {` → 条件内联 ─────────────────

/// 临时变量声明（`^let (mut )?(_[A-Za-z]\w*?\d+)(?:: (.+?))? = (.*);$`）的解析结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempLet {
    pub name: String,
    pub mutable: bool,
    pub ty: Option<String>,
    pub value: String,
}

/// `_[A-Za-z]\w*?\d+`：下划线 + 字母开头、以数字结尾的单词
pub(crate) fn is_temp_name(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 3
        && b[0] == b'_'
        && b[1].is_ascii_alphabetic()
        && b[b.len() - 1].is_ascii_digit()
        && s.chars().all(text::is_word_char)
}

/// 文本形态的临时变量声明解析（与 Python 正则逐字同口径）
pub fn parse_temp_let_text(s: &str) -> Option<TempLet> {
    let s = s.trim();
    let rest = s.strip_prefix("let ")?.strip_suffix(';')?;
    let (mutable, rest) = match rest.strip_prefix("mut ") {
        Some(r) => (true, r),
        None => (false, rest),
    };
    // 名字：最长单词前缀中满足 `_[A-Za-z]\w*?\d+` 且其后紧跟 `: ` 或 ` = ` 的最短前缀
    let word_end = rest.find(|c: char| !text::is_word_char(c)).unwrap_or(rest.len());
    for end in 3..=word_end {
        if !rest.is_char_boundary(end) {
            continue;
        }
        let name = &rest[..end];
        if !is_temp_name(name) {
            continue;
        }
        let tail = &rest[end..];
        if let Some(t) = tail.strip_prefix(": ") {
            // 类型取最短的 `.+?` 使其后为 ` = `
            if let Some(p) = t.find(" = ").filter(|&p| p > 0) {
                return Some(TempLet {
                    name: name.to_string(),
                    mutable,
                    ty: Some(t[..p].to_string()),
                    value: t[p + 3..].to_string(),
                });
            }
        }
        if let Some(v) = tail.strip_prefix(" = ") {
            return Some(TempLet { name: name.to_string(), mutable, ty: None, value: v.to_string() });
        }
    }
    None
}

/// 语句形态：`let` 语句按渲染文本、Raw 按原文识别；其余 None
pub fn parse_temp_let(env: &InstrEnv, s: &Stmt) -> Option<TempLet> {
    match s {
        Stmt::Let(_) => parse_temp_let_text(&text::stmt(env, s)),
        Stmt::Raw(r) => parse_temp_let_text(&r.0),
        _ => None,
    }
}

const NO_INLINE_MARKERS: [&str; 4] = ["Default::default()", ".into()", "panic!", "\n"];

fn needs_paren(t: &str) -> bool {
    let mut depth: i32 = 0;
    for ch in t.chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if depth == 0 && " <>=|&+-*/%!".contains(c) => return true,
            _ => {}
        }
    }
    false
}

fn word_count(name: &str, t: &str) -> usize {
    text::word_positions_no_dot(t, name).len()
}

fn paren_val(v: &str) -> String {
    if needs_paren(v) {
        format!("({v})")
    } else {
        v.to_string()
    }
}

/// stmts 全部是只用一次的临时变量声明时，把它们回填进条件并返回新条件；否则 None。
/// 保持求值顺序：临时变量在条件文本中的出现顺序必须与声明顺序一致。
pub fn inline_temps(env: &InstrEnv, stmts: &[Stmt], cond: &Cond, exit_stack: &[StackEntry]) -> Option<Cond> {
    if stmts.is_empty() {
        return Some(cond.clone());
    }
    let mut temps: Vec<(String, String)> = Vec::new();
    for s in stmts {
        let t = parse_temp_let(env, s)?;
        if t.mutable || NO_INLINE_MARKERS.iter().any(|m| t.value.contains(m)) {
            return None;
        }
        temps.push((t.name, t.value));
    }
    let stack_text = exit_stack.iter().map(|e| text::expr(env, &e.expr)).collect::<Vec<_>>().join(" ");
    let mut resolved: Vec<(String, String)> = Vec::new();
    let mut used_in_value: Vec<String> = Vec::new();
    for (name, value) in &temps {
        let mut value = value.clone();
        for (prev, pv) in resolved.clone() {
            let n = word_count(&prev, &value);
            if n == 0 {
                continue;
            }
            if n != 1 || used_in_value.contains(&prev) {
                return None;
            }
            value = text::replace_word_no_dot(&value, &prev, &paren_val(&pv));
            used_in_value.push(prev);
        }
        resolved.push((name.clone(), value));
    }
    let top: Vec<&String> = temps.iter().map(|(n, _)| n).filter(|n| !used_in_value.contains(n)).collect();
    let ct = render_cond(cond);
    let mut positions = Vec::new();
    for name in &top {
        if word_count(name, &ct) != 1 || word_count(name, &stack_text) != 0 {
            return None;
        }
        positions.push(text::word_positions_no_dot(&ct, name)[0]);
    }
    for name in &used_in_value {
        if word_count(name, &ct) != 0 || word_count(name, &stack_text) != 0 {
            return None;
        }
    }
    if positions.windows(2).any(|w| w[0] > w[1]) {
        return None;
    }
    let lookup = |n: &str| resolved.iter().find(|(k, _)| k == n).map(|(_, v)| v.clone()).unwrap_or_default();
    let reps: Vec<(String, String)> = top.iter().map(|n| ((*n).clone(), paren_val(&lookup(n)))).collect();
    Some(map_atoms(cond, &mut |s: &str| {
        let mut out = s.to_string();
        for (n, r) in &reps {
            out = text::replace_word_no_dot(&out, n, r);
        }
        out
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temp_let() {
        let t = parse_temp_let_text("let _t3 = a.b()?;").unwrap();
        assert_eq!((t.name.as_str(), t.mutable, t.ty, t.value.as_str()), ("_t3", false, None, "a.b()?"));
        let t = parse_temp_let_text("let mut _merged12: HashMap<K, V> = x = y;").unwrap();
        assert_eq!(t.ty.as_deref(), Some("HashMap<K, V>"));
        assert_eq!(t.value, "x = y");
        assert!(parse_temp_let_text("let x1 = 2;").is_none());
        assert!(parse_temp_let_text("let _t = 2;").is_none());
        let t = parse_temp_let_text("let _a1b2 = 2;").unwrap();
        assert_eq!(t.name, "_a1b2");
    }

    #[test]
    fn tvar_shapes() {
        assert!(short_tvar("T"));
        assert!(short_tvar("E1"));
        assert!(!short_tvar("Ab1"));
        assert!(!short_tvar("t"));
        assert!(uses_jvm_null_method("HashMap", &[]));
        assert!(!uses_jvm_null_method("K", &[]));
    }
}
