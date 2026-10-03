//! loop 内声明、loop 外读取的变量提升到 loop 前（← `_hoist_loop_vars`）。

use std::collections::{BTreeMap, BTreeSet};

use ir::{LetStmt, Stmt, VarOrigin};

use super::refs::{render_entries, RefCache};
use super::if_emit::align_later;
use super::slot_type::{all_alignable, merged_slot_type, widen_into_merged};
use super::{
    apply_insertions, demote_let, entry_nesting, hoisted_let_type, leading_ws, let_of, read_is_other_var, same_jvm_var, VarsCtx,
};
use crate::entry::Entry;
use crate::error::MethodResult;

pub fn hoist_loop_vars(cx: &VarsCtx, entries: &mut Vec<Entry>) -> MethodResult<()> {
    // Pass 1：嵌套块内首次声明的变量及其位置
    let mut nesting = 0;
    let mut declared_at: BTreeMap<String, (usize, i32)> = BTreeMap::new();
    let mut loop_heads: Vec<usize> = Vec::new();
    for (k, e) in entries.iter().enumerate() {
        if e.is_text() {
            if e.is_loop_head() {
                loop_heads.push(k);
            }
            nesting += e.delta();
        } else if let Some(l) = let_of(e) {
            let name = l.name.as_str();
            if nesting > 0 && !declared_at.contains_key(name) && !cx.predeclared.contains(name) {
                declared_at.insert(name.to_string(), (k, nesting));
            }
        }
    }
    if declared_at.is_empty() || loop_heads.is_empty() {
        return Ok(());
    }
    // Pass 2：作用域关闭后被引用（首个命中是另一 let 声明＝槽复用，不算）
    let rendered = render_entries(cx.env, entries);
    let depth = entry_nesting(entries);
    let cache = RefCache::new(entries.len());
    let mut to_hoist: BTreeSet<&str> = BTreeSet::new();
    for (name, &(decl_k, decl_nesting)) in &declared_at {
        let Some(close) = (decl_k + 1..entries.len()).find(|&k2| depth[k2] < decl_nesting) else {
            continue;
        };
        for k2 in close..entries.len() {
            let (hit, is_let) = cache.refs(cx.env, entries, k2, &rendered[k2], name);
            if hit {
                // 读取点属于另一个 JVM 变量（LVT 区间证据，如循环内 return 分支里同槽复用的新变量）：不提升
                if !is_let && !read_is_other_var(cx, &entries[decl_k], &entries[k2], name) {
                    to_hoist.insert(name);
                }
                break;
            }
        }
    }
    // Pass 3：loop 前插入无初值前置声明，loop 内声明降为赋值（按名序）
    let mut insertions = Vec::new();
    for name in to_hoist {
        let (decl_k, _) = declared_at[name];
        let Some(&loop_k) = loop_heads.iter().rev().find(|&&lk| lk < decl_k) else {
            continue;
        };
        let indent = match entries[loop_k].text() {
            Some(t) => leading_ws(t).to_string(),
            None => entries[loop_k].indent.clone(),
        };
        let first = entries[decl_k].clone();
        let inner = let_of(&first).expect("declared_at 只登记 let");
        // 循环体内同一 JVM 变量的其余 let（如 if / else 两臂各自的首存）一并降级，
        // 否则后续 if 提升会为它们另立同名声明，遮蔽本声明
        // loop_k 是 decl_k 之前最近的循环头；该循环在 decl_k 前已结束时只降级触发声明本身
        let loop_end = (loop_k + 1..entries.len())
            .find(|&k| depth[k] <= depth[loop_k])
            .unwrap_or(entries.len())
            .max(decl_k + 1);
        let mut ty = hoisted_let_type(inner);
        if !all_alignable(cx, entries, name, decl_k - 1, loop_end, ty.as_ref()) {
            if let Ok(Some(m)) = merged_slot_type(cx, entries, name, decl_k - 1, loop_end) {
                widen_into_merged(cx, entries, name, decl_k - 1, loop_end, &m)?;
                ty = Some(m);
            }
        }
        let hoisted = LetStmt {
            name: inner.name.clone(),
            ty: ty.clone(),
            mutable: true,
            value: None,
            origin: VarOrigin { value_ty: None, slot: inner.origin.slot, bind_off: inner.origin.bind_off },
        };
        insertions.push((loop_k, Entry::stmt(&indent, Stmt::Let(hoisted))));
        for k in decl_k..loop_end {
            let same = k == decl_k
                || (let_of(&entries[k]).is_some_and(|l| l.name.as_str() == name)
                    && same_jvm_var(cx, &first, &entries[k]) != Some(false));
            if !same {
                continue;
            }
            if let Some(h) = &ty {
                align_later(cx, entries, k, h);
            }
            demote_let(entries, k);
        }
    }
    apply_insertions(entries, insertions);
    Ok(())
}
