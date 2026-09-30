//! 同名异型槽的汇合类型（← `_merged_slot_type` / `_all_alignable` / `_widen_into_merged`）：
//! 同一 slot 在兄弟分支存入不同引用类型时，JVM 校验器在合并点取公共祖先；无公共类祖先时
//! 回退根类，存入侧装箱上转。

use ir::anchors::OBJECT;
use ir::render::render_type;
use ir::{Expr, LetStmt, Raw, Stmt, Type, UpcastWrap};
use ty::RsType;

use super::{is_default_value, store_type, store_value, VarsCtx};
use crate::coerce_text::to_object;
use crate::entry::{Entry, Item};
use crate::text;
use crate::error::MethodResult;
use crate::types::{forms_alignable, from_rust_text, ir_type_of};

const PRIMITIVE_RUST_TYPES: [&str; 13] =
    ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize", "()"];

/// (start, end) 内同名声明 / 降级赋值的引用类型不一致时的汇合类型；否则 None
pub(super) fn merged_slot_type(
    cx: &VarsCtx,
    entries: &[Entry],
    name: &str,
    start: usize,
    end: usize,
) -> MethodResult<Option<Type>> {
    let mut seen: Vec<String> = Vec::new();
    for e in &entries[start + 1..end] {
        let Some(Some(t)) = store_type(e, name) else { continue };
        let r = render_type(&t);
        if PRIMITIVE_RUST_TYPES.contains(&r.as_str()) {
            return Ok(None);
        }
        if !seen.contains(&r) {
            seen.push(r);
        }
    }
    if seen.len() < 2 {
        return Ok(None);
    }
    let root = || ir_type_of(OBJECT).map(Some);
    let ctx = &cx.env.ctx;
    let mut common = seen[0].clone();
    for other in &seen[1..] {
        let (Some(a), Some(b)) = (from_rust_text(cx.env, &common), from_rust_text(cx.env, other)) else {
            return root();
        };
        match instr::hierarchy::common_ref_type_widening(ctx, &a, &b) {
            Some(c) => common = text::ty(cx.env, &c),
            None => return root(),
        }
    }
    ir_type_of(&common).map(Some)
}

/// span 内同名 let / 赋值的类型是否都能对齐到提升类型（同型或可对齐）
pub(super) fn all_alignable(
    cx: &VarsCtx,
    entries: &[Entry],
    name: &str,
    start: usize,
    end: usize,
    hoisted: Option<&Type>,
) -> bool {
    let Some(h) = hoisted else { return true };
    let hs = render_type(h);
    entries[start + 1..end].iter().all(|e| {
        let Some(Some(t)) = store_type(e, name) else { return true };
        if is_default_value(store_value(e)) {
            return true;
        }
        let ts = render_type(&t);
        ts == hs || forms_alignable(cx.env, Some(&ts), Some(&hs))
    })
}

/// 汇合类型为公共类祖先：各次存入按 `.into()` 上转；为根类：装箱上转
pub(super) fn widen_into_merged(cx: &VarsCtx, entries: &mut [Entry], name: &str, start: usize, end: usize, merged: &Type) {
    let merged_s = render_type(merged);
    let is_root = merged_s == OBJECT;
    for e in &mut entries[start + 1..end] {
        let Some(t) = store_type(e, name) else { continue };
        let convert = match t {
            Some(t) if !is_default_value(store_value(e)) => {
                let rendered = render_type(&t);
                (rendered != merged_s).then_some(rendered)
            }
            _ => None,
        };
        let Item::Stmt(s) = &mut e.item else { continue };
        match &mut **s {
            Stmt::Let(l) => retarget_let(l, convert.as_deref(), merged, |v, r| widen_value(cx, v, r, is_root)),
            Stmt::Assign(a) => {
                if let Some(rendered) = convert {
                    let v = std::mem::replace(&mut a.value, Expr::Raw(Raw(String::new())));
                    a.value = widen_value(cx, v, &rendered, is_root);
                }
                a.origin.value_ty = Some(merged.clone());
            }
            _ => {}
        }
    }
}

/// 同名 let 改指汇合类型：值类型与汇合类型不同（`convert` = 原类型文本）时按 `widen` 上转初值；
/// 已是汇合类型的 let 保留原初值——随后降级为赋值时它就是这次存储本身
fn retarget_let(l: &mut LetStmt, convert: Option<&str>, merged: &Type, widen: impl FnOnce(Expr, &str) -> Expr) {
    if let Some(rendered) = convert {
        if let Some(v) = l.value.take() {
            l.value = Some(widen(v, rendered));
        }
    }
    if l.ty.is_some() {
        l.ty = Some(merged.clone());
    }
    l.origin.value_ty = Some(merged.clone());
}

/// 存入值上转到汇合类型：根类装箱，公共祖先 `.into()`（目标由汇合后的声明类型给出）
fn widen_value(cx: &VarsCtx, v: Expr, rendered: &str, is_root: bool) -> Expr {
    if is_root {
        let rt = from_rust_text(cx.env, rendered).unwrap_or(RsType::Object);
        Expr::Raw(Raw(to_object(cx.env, &text::expr(cx.env, &v), &rt, false)))
    } else {
        Expr::Upcast { expr: Box::new(v), wrap: UpcastWrap::Auto }
    }
}

#[cfg(test)]
mod tests {
    use super::retarget_let;
    use crate::types::ir_type_of;
    use ir::anchors::OBJECT;
    use ir::{Expr, Ident, LetStmt, Raw, VarOrigin};

    fn let_of(ty: &str, value: &str) -> LetStmt {
        LetStmt {
            name: Ident::new("local_1").unwrap(),
            ty: Some(ir_type_of(ty).unwrap()),
            mutable: true,
            value: Some(Expr::Var(Ident::new(value).unwrap())),
            origin: VarOrigin::default(),
        }
    }

    /// 已是汇合类型的 let 保留初值（回归：DeepCopy `ObjectInputStream.readObject0` 的
    /// try/finally 返回值暂存 `local_5` 在该臂丢失存储，E0381）
    #[test]
    fn same_type_let_keeps_value() {
        let merged = ir_type_of(OBJECT).unwrap();
        let mut l = let_of(OBJECT, "_t1");
        retarget_let(&mut l, None, &merged, |_, _| panic!("同型不应上转"));
        assert_eq!(l.value, Some(Expr::Var(Ident::new("_t1").unwrap())));
        assert_eq!(l.ty, Some(merged.clone()));
        assert_eq!(l.origin.value_ty, Some(merged));
    }

    #[test]
    fn different_type_let_widens_value() {
        let merged = ir_type_of(OBJECT).unwrap();
        let mut l = let_of("Foo", "_t2");
        retarget_let(&mut l, Some("Foo"), &merged, |v, r| {
            assert_eq!(r, "Foo");
            assert_eq!(v, Expr::Var(Ident::new("_t2").unwrap()));
            Expr::Raw(Raw("boxed".into()))
        });
        assert_eq!(l.value, Some(Expr::Raw(Raw("boxed".into()))));
        assert_eq!(l.ty, Some(merged));
    }
}
