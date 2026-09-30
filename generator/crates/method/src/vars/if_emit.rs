//! if 提升的顶层声明处理与提升声明登记（← `_hoist_if_outer` / `_hoist_if_emit`）。

use ir::render::render_type;
use ir::{LetStmt, Stmt, Type, VarOrigin};

use super::if_hoist::{Candidate, Frame};
use super::slot_type::{all_alignable, merged_slot_type, widen_into_merged};
use super::{
    align_store_value, demote_let, hoisted_let_type, is_default_value, leading_ws, let_of, let_of_mut, same_jvm_var,
    VarsCtx,
};
use crate::entry::Entry;
use crate::error::MethodResult;
use crate::types::{forms_alignable, ir_type_of};

fn entry_hoisted_type(e: &Entry) -> Option<Type> {
    let_of(e).and_then(hoisted_let_type)
}

/// 条目 k 是名为 name 的 let
fn is_let_of(entries: &[Entry], k: usize, name: &str) -> bool {
    let_of(&entries[k]).is_some_and(|l| l.name.as_str() == name)
}

/// 待降级 let 的值侧对齐到 hoisted（形态不同、可对齐、非占位值时）
fn align_later(cx: &VarsCtx, entries: &mut [Entry], k: usize, hoisted: &Type) {
    let hoisted_s = render_type(hoisted);
    let Some(l) = let_of_mut(&mut entries[k]) else { return };
    let Some(later_s) = hoisted_let_type(l).map(|t| render_type(&t)) else { return };
    if later_s != hoisted_s
        && forms_alignable(cx.env, Some(&later_s), Some(&hoisted_s))
        && !is_default_value(l.value.as_ref())
    {
        align_store_value(l, hoisted, &later_s, &hoisted_s);
    }
}

/// Pass 4b：函数体顶层已有同名声明。已可见（顶层声明不晚于引用 / 无块外引用）→ 同一 JVM
/// 变量的分段 let 降级并回顶层绑定，返回 true 跳过；晚于引用 → 顶层声明一并降级（类型
/// 不可对齐＝同槽两个 JVM 变量时保留），返回 false 继续提升
pub(super) fn outer(cx: &VarsCtx, entries: &mut [Entry], f: &Frame, c: &Candidate) -> MethodResult<bool> {
    let Some(&outer_k) = f.outer_first_k.get(&c.name) else {
        return Ok(false);
    };
    let name = c.name.as_str();
    let outer_item = entries[outer_k].clone();
    if c.ref_idx.is_none_or(|r| outer_k <= r) {
        for &(dk, _) in &c.decl_list {
            if dk <= outer_k || !is_let_of(entries, dk, name) {
                continue;
            }
            if same_jvm_var(cx, &outer_item, &entries[dk]) != Some(true) {
                continue;
            }
            if let Some(outer_ty) = entry_hoisted_type(&outer_item) {
                align_later(cx, entries, dk, &outer_ty);
            }
            demote_let(entries, dk);
        }
        return Ok(true);
    }
    let mut outer_ty_s = None;
    let mut split = false;
    if let_of(&outer_item).is_some() {
        outer_ty_s = entry_hoisted_type(&outer_item).map(|t| render_type(&t));
        split = c.ty_str.is_some() && !forms_alignable(cx.env, outer_ty_s.as_deref(), c.ty_str.as_deref());
    }
    if !split {
        for ck in outer_k..entries.len() {
            if !is_let_of(entries, ck, name) {
                continue;
            }
            if let (Some(os), Some(ts)) = (&outer_ty_s, &c.ty_str) {
                let l = let_of_mut(&mut entries[ck]).expect("已判定为 let");
                if os != ts && !is_default_value(l.value.as_ref()) {
                    align_store_value(l, &ir_type_of(ts)?, os, ts);
                }
            }
            demote_let(entries, ck);
        }
    }
    Ok(false)
}

/// Pass 4c：引用可见性上移、登记提升声明插入点，块内同名 let 值侧对齐后降级
pub(super) fn emit(
    cx: &VarsCtx,
    entries: &mut [Entry],
    f: &Frame,
    c: &Candidate,
    mut block_k: usize,
) -> MethodResult<(usize, Entry)> {
    let name = c.name.as_str();
    // 声明插在 block_k 之前（同层）：引用须在该层可见，否则继续上移
    if let Some(r) = c.ref_idx {
        let ref_nesting = f.depth[r];
        while ref_nesting < f.depth[block_k] {
            match f.parent(block_k, f.depth[block_k]) {
                Some(p) => block_k = p,
                None => break,
            }
        }
    }
    let indent = match entries[block_k].text() {
        Some(t) => leading_ws(t).to_string(),
        None => entries[block_k].indent.clone(),
    };
    let first_decl_k = c.found_decl.0;
    let mut hoisted_type = entry_hoisted_type(&entries[first_decl_k]);
    // 提升点所辖语句（block_k 开启的整条 if/else、match、loop）的范围
    let span_end = (block_k + 1..entries.len()).find(|&k2| f.depth[k2] <= f.depth[block_k]).unwrap_or(entries.len());
    // 同名异型：公共祖先 widening，无公共类祖先回退根类装箱；LVT 具名变量仅在存在不可对齐形态时合并
    let merged = if !cx.lvt_names.contains(name)
        || !all_alignable(cx, entries, name, block_k, span_end, hoisted_type.as_ref())
    {
        merged_slot_type(cx, entries, name, block_k, span_end)?
    } else {
        None
    };
    if let Some(m) = merged {
        widen_into_merged(cx, entries, name, block_k, span_end, &m);
        hoisted_type = Some(m);
    }
    // 提升声明继承触发声明的 JVM 身份（槽位 / store 偏移）
    let first_let = entries[first_decl_k].clone();
    let (slot, bind_off) = match first_let.as_stmt() {
        Some(Stmt::Let(l)) => (l.origin.slot, l.origin.bind_off),
        Some(Stmt::Assign(a)) => (a.origin.slot, a.origin.bind_off),
        _ => (None, None),
    };
    let decl = LetStmt {
        name: ir::Ident::new(name).map_err(|e| crate::error::MethodError::Unported(format!("变量名 {name}：{e:?}")))?,
        ty: hoisted_type.clone(),
        mutable: true,
        value: None,
        origin: VarOrigin { value_ty: None, slot, bind_off },
    };
    for &(dk, _) in &c.decl_list {
        if !(block_k < dk && dk < span_end) || !is_let_of(entries, dk, name) {
            continue;
        }
        // LVT 证据判定为另一个 JVM 变量（槽复用换主）：保留其自己的 let
        if same_jvm_var(cx, &first_let, &entries[dk]) == Some(false) {
            continue;
        }
        if let Some(h) = &hoisted_type {
            align_later(cx, entries, dk, h);
        }
        demote_let(entries, dk);
    }
    Ok((block_k, Entry::stmt(&indent, Stmt::Let(decl))))
}
