//! 变量分析与提升（← `method/vars.py`）：JVM 局部变量槽是函数级作用域，Rust 是块级——
//! 块内首次声明、块外被读取的变量提升到块前（无初值前置声明 `let mut x: T;`，块内降为赋值）。
//!
//! - [`analyze_mutation`]：被赋值的 let 标 `mut`
//! - [`hoist_loop_vars`]：loop 内声明、loop 外读取
//! - [`hoist_if_vars`]：if / else / match / try 块内声明、块外（或 else 兄弟块）读取；每次提升一层
//! - [`promote_undeclared_assigns`]：作用域外的赋值升为 let
//!
//! 变量身份以 LocalVariableTable 区间为证据（[`same_jvm_var`]）。

mod if_emit;
mod if_hoist;
mod loop_hoist;
mod refs;
mod slot_type;

use std::collections::{BTreeMap, BTreeSet};

use instr::InstrEnv;
use ir::{AssignStmt, Expr, LetStmt, Stmt, Type, UpcastWrap, VarOrigin};
use sim::{safe_name, SlotDecl};

use crate::entry::{Entry, Item};
use crate::text;

pub use if_hoist::hoist_if_vars;
pub use loop_hoist::hoist_loop_vars;

/// 变量提升 pass 的方法级只读输入
pub struct VarsCtx<'a> {
    pub env: &'a InstrEnv<'a>,
    /// 形参与 `this`：签名已声明，不参与提升
    pub predeclared: &'a BTreeSet<String>,
    /// LVT 声明表（slot → 区间，按起点升序）
    pub slot_decls: &'a BTreeMap<u16, Vec<SlotDecl>>,
    /// LocalVariableTable 里出现过的名字（其余是 javac 合成的无名槽）
    pub lvt_names: BTreeSet<String>,
}

impl<'a> VarsCtx<'a> {
    pub fn new(
        env: &'a InstrEnv<'a>,
        predeclared: &'a BTreeSet<String>,
        slot_decls: &'a BTreeMap<u16, Vec<SlotDecl>>,
    ) -> VarsCtx<'a> {
        let lvt_names = slot_decls.values().flatten().map(|d| safe_name(&d.name)).collect();
        VarsCtx { env, predeclared, slot_decls, lvt_names }
    }
}

/// 具名 let（`let _ = e;` 在 Python 侧是原文语句 RawStmt，变量分析不视为声明）
fn let_of(e: &Entry) -> Option<&LetStmt> {
    match e.as_stmt() {
        Some(Stmt::Let(l)) if !l.name.is_discard() => Some(l),
        _ => None,
    }
}

fn let_of_mut(e: &mut Entry) -> Option<&mut LetStmt> {
    match &mut e.item {
        Item::Stmt(s) => match &mut **s {
            Stmt::Let(l) if !l.name.is_discard() => Some(l),
            _ => None,
        },
        _ => None,
    }
}

/// 赋值目标为变量 `name` 的赋值语句
fn assign_to<'e>(e: &'e Entry, name: &str) -> Option<&'e AssignStmt> {
    match e.as_stmt() {
        Some(Stmt::Assign(a)) if matches!(&a.target, Expr::Var(v) if v.as_str() == name) => Some(a),
        _ => None,
    }
}

/// 文本行的前导空白
fn leading_ws(s: &str) -> &str {
    &s[..s.len() - s.trim_start().len()]
}

/// 类型文本中是否有独立的 `_`（待推断实参）
fn has_infer_placeholder(s: &str) -> bool {
    let b = s.as_bytes();
    let w = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    (0..b.len()).any(|i| b[i] == b'_' && (i == 0 || !w(b[i - 1])) && b.get(i + 1).is_none_or(|&c| !w(c)))
}

/// 前置声明的类型标注：声明自带标注优先；省略标注的声明取其模拟类型
/// （含待推断实参 `_` 的类型不能作标注）（← `_hoisted_let_type`）
fn hoisted_let_type(l: &LetStmt) -> Option<Type> {
    if let Some(t) = &l.ty {
        return Some(t.clone());
    }
    match &l.origin.value_ty {
        Some(t) if !has_infer_placeholder(&ir::render::render_type(t)) => Some(t.clone()),
        _ => None,
    }
}

/// 条目的「声明 / 降级赋值」类型（let → 提升标注类型；对 name 的赋值 → 值类型）
fn store_type(e: &Entry, name: &str) -> Option<Option<Type>> {
    if let Some(l) = let_of(e).filter(|l| l.name.as_str() == name) {
        return Some(hoisted_let_type(l));
    }
    assign_to(e, name).map(|a| a.origin.value_ty.clone())
}

/// 条目的值（let / 赋值）
fn store_value(e: &Entry) -> Option<&Expr> {
    match e.as_stmt() {
        Some(Stmt::Let(l)) if !l.name.is_discard() => l.value.as_ref(),
        Some(Stmt::Assign(a)) => Some(&a.value),
        _ => None,
    }
}

/// 占位初值（无值的前置声明同视为无值可对齐）（← `_is_default_value`）
fn is_default_value(v: Option<&Expr>) -> bool {
    match v {
        None => true,
        Some(Expr::Raw(r)) => r.as_str() == "Default::default()",
        Some(_) => false,
    }
}

/// 提升后原位置的 `let x: T = v;` 降为 `x = v;`；无初值的前置声明标记删除（← `_demote_let`）
fn demote_let(entries: &mut [Entry], k: usize) {
    let Some(l) = let_of(&entries[k]) else { return };
    entries[k].item = match &l.value {
        None => Item::Removed,
        Some(v) => Item::Stmt(Box::new(Stmt::Assign(AssignStmt {
            target: Expr::Var(l.name.clone()),
            value: v.clone(),
            origin: VarOrigin { value_ty: hoisted_let_type(l), slot: l.origin.slot, bind_off: l.origin.bind_off },
        }))),
    };
    if entries[k].item == Item::Removed {
        entries[k].indent.clear();
    }
}

/// 待降级声明的值侧对齐到提升声明类型（子类型 → `.into()`）；同时补值类型（← `_align_store_value`）
fn align_store_value(l: &mut LetStmt, hoisted: &Type, later_s: &str, hoisted_s: &str) {
    if is_default_value(l.value.as_ref()) {
        l.origin.value_ty = Some(hoisted.clone());
        return;
    }
    if later_s != hoisted_s {
        if let Some(v) = l.value.take() {
            l.value = Some(Expr::Upcast { expr: Box::new(v), wrap: UpcastWrap::Auto });
        }
    }
    l.origin.value_ty = Some(hoisted.clone());
}

/// 条目的变量身份（名字、槽位、store 偏移）
fn var_identity(e: &Entry) -> Option<(&str, Option<u16>, Option<u32>)> {
    match e.as_stmt()? {
        Stmt::Let(l) if !l.name.is_discard() => Some((l.name.as_str(), l.origin.slot, l.origin.bind_off)),
        Stmt::Assign(a) => {
            let name = match &a.target {
                Expr::Var(v) => v.as_str(),
                _ => return None,
            };
            Some((name, a.origin.slot, a.origin.bind_off))
        }
        _ => None,
    }
}

/// LVT 中覆盖 store 偏移 off 的**本名**区间（含初始化 store 落在区间起点前 1~4 字节）
fn lvt_covering_entry<'d>(decls: &'d [SlotDecl], off: u32, name: &str) -> Option<&'d SlotDecl> {
    decls
        .iter()
        .find(|d| safe_name(&d.name) == name && ((d.start <= off && off < d.end) || (off < d.start && d.start <= off + 4)))
}

/// LVT 区间驱动的变量身份：同名两语句是否同一 JVM 变量（None = 无身份证据）（← `_same_jvm_var`）
fn same_jvm_var(cx: &VarsCtx, a: &Entry, b: &Entry) -> Option<bool> {
    let (name, sa, oa) = var_identity(a)?;
    let (_, sb, ob) = var_identity(b)?;
    let (sa, sb, oa, ob) = (sa?, sb?, oa?, ob?);
    if sa != sb {
        return Some(false);
    }
    let decls = cx.slot_decls.get(&sa).filter(|d| !d.is_empty())?;
    let ea = lvt_covering_entry(decls, oa, name)?;
    let eb = lvt_covering_entry(decls, ob, name)?;
    let r = |d: &SlotDecl| d.ty.as_ref().map(|t| text::ty(cx.env, t));
    if r(ea) != r(eb) {
        return Some(false);
    }
    let (lo, hi) = if oa <= ob { (oa, ob) } else { (ob, oa) };
    if decls.iter().any(|d| safe_name(&d.name) != name && d.start < hi && d.end > lo) {
        // 两存点间该槽被异名区间换主：另一个变量
        return Some(false);
    }
    Some(true)
}

/// 被赋值的 let 标 `mut`（← `_analyze_mutation`；entries 只含直线语句）
pub fn analyze_mutation(entries: &mut [Entry]) {
    let assigned: BTreeSet<String> = entries
        .iter()
        .filter_map(|e| match e.as_stmt() {
            Some(Stmt::Assign(AssignStmt { target: Expr::Var(v), .. })) => Some(v.as_str().to_string()),
            _ => None,
        })
        .collect();
    for e in entries.iter_mut() {
        if let Some(l) = let_of_mut(e) {
            if assigned.contains(l.name.as_str()) {
                l.mutable = true;
            }
        }
    }
}

/// 当前词法作用域无对应 let 的赋值升为 `let mut`（类型由 Rust 推断）（← `_promote_undeclared_assigns`）
pub fn promote_undeclared_assigns(entries: &mut [Entry], predeclared: &BTreeSet<String>) {
    let mut declared: BTreeMap<String, i32> = BTreeMap::new();
    let mut nesting = 0i32;
    for e in entries.iter_mut() {
        if e.is_text() {
            nesting += e.delta();
            if e.delta() < 0 {
                declared.retain(|_, d| *d <= nesting);
            }
            // catch 绑定在 catch 体内已声明：体内对它的重新赋值是赋值而非新 let
            if let Some(bind) = e.catch_binding() {
                declared.insert(bind.to_string(), nesting);
            }
            continue;
        }
        let promoted = match e.as_stmt() {
            Some(Stmt::Let(l)) if !l.name.is_discard() => {
                declared.insert(l.name.as_str().to_string(), nesting);
                None
            }
            Some(Stmt::Assign(a)) => match &a.target {
                Expr::Var(v) if !predeclared.contains(v.as_str()) && !declared.contains_key(v.as_str()) => {
                    Some(LetStmt {
                        name: v.clone(),
                        ty: None,
                        mutable: true,
                        value: Some(a.value.clone()),
                        origin: VarOrigin { value_ty: None, slot: a.origin.slot, bind_off: a.origin.bind_off },
                    })
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(l) = promoted {
            declared.insert(l.name.as_str().to_string(), nesting);
            e.item = Item::Stmt(Box::new(Stmt::Let(l)));
        }
    }
}

/// 每条目开始处的嵌套深度
fn entry_nesting(entries: &[Entry]) -> Vec<i32> {
    let mut out = Vec::with_capacity(entries.len());
    let mut cur = 0;
    for e in entries {
        out.push(cur);
        cur += e.delta();
    }
    out
}

/// 倒序插入（同位置多条：后登记者在前，同 Python 稳定排序 + 逐条 insert）
fn apply_insertions(entries: &mut Vec<Entry>, mut insertions: Vec<(usize, Entry)>) {
    insertions.sort_by_key(|(k, _)| std::cmp::Reverse(*k));
    for (k, e) in insertions {
        entries.insert(k, e);
    }
    crate::entry::drop_removed(entries);
}

#[cfg(test)]
mod tests {
    use super::has_infer_placeholder;

    #[test]
    fn infer_placeholder() {
        assert!(has_infer_placeholder("_"));
        assert!(has_infer_placeholder("HashMap<_, V>"));
        assert!(!has_infer_placeholder("__Shared<i32>"));
        assert!(!has_infer_placeholder("a_b"));
    }
}
