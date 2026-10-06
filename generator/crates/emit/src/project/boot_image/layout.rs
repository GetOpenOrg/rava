//! 映像对象的存储槽布局：与宏展开的 `X__inner` 逐字段一致（`java_class!` 的 struct 布局，
//! 见 rava_macros_core `block/gen/struct_layout.rs`）——继承字段在前（最深祖先先），本类字段在后；
//! 擦除字段存 `Object`，基本类型字段为 `__PrimCell`，其余引用字段为 `__RefField<Option<T>>`。

use std::collections::BTreeSet;

use ty::ident::safe_ident;
use ty::ClassInfo;

use crate::class_writer::fields::{flatten_super_fields, resolve_field_rust, PRIMITIVE_RUST_TYPES};
use crate::ctx::EmitCtx;
use crate::text::contains_word;

/// 引用槽的静态类型（决定常量引用的构造形态）
#[derive(Clone, Debug, PartialEq)]
pub enum SlotTy {
    /// `Object`（含擦除槽）
    Object,
    /// 类 wrapper（binary name）
    Class(String),
    /// 接口载体（binary name）
    Iface(String),
    /// 数组：槽的 Rust 类型文本（去空白，与映像数组的短形态比对）
    Array(String),
    /// 无法在常量中构造的类型：启动时经 `From<Object>` 回填
    Other,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    /// 基本类型槽（描述符首字符）
    Prim(u8),
    Ref(SlotTy),
}

#[derive(Clone, Debug)]
pub struct Slot {
    /// 存储字段的 Rust 名
    pub rust: String,
    /// 声明类与 Java 字段名（映像的字段键）
    pub decl: String,
    pub java: String,
    pub desc: String,
    pub cell: Cell,
}

/// 类型文本的头部标识符（`a::b::X<..>` → `X`）
fn head(view: &str) -> &str {
    let h = view.split('<').next().unwrap_or(view).trim();
    h.rsplit("::").next().unwrap_or(h)
}

/// 去空白的类型文本（与映像数组的短形态比对）
pub fn squash(t: &str) -> String {
    t.chars().filter(|c| !c.is_whitespace()).collect()
}

/// 引用槽的静态类型：按描述符所指的类与渲染出的 Rust 类型头部一致性判定
fn slot_ty(ctx: &EmitCtx<'_>, desc: &str, view: &str) -> SlotTy {
    let h = head(view);
    if h == "Object" {
        return SlotTy::Object;
    }
    if desc.starts_with('[') {
        return if h == "JArray" { SlotTy::Array(squash(view)) } else { SlotTy::Other };
    }
    let Some(cls) = desc.strip_prefix('L').and_then(|d| d.strip_suffix(';')) else { return SlotTy::Other };
    match ctx.class(cls) {
        Some(ci) if ctx.declared(cls) == h => {
            if ci.is_interface() {
                SlotTy::Iface(cls.to_string())
            } else {
                SlotTy::Class(cls.to_string())
            }
        }
        _ => SlotTy::Other,
    }
}

fn cell(ctx: &EmitCtx<'_>, desc: &str, view: &str, erased: bool, prim: bool) -> Cell {
    if erased {
        Cell::Ref(SlotTy::Object)
    } else if prim {
        Cell::Prim(desc.as_bytes()[0])
    } else {
        Cell::Ref(slot_ty(ctx, desc, view))
    }
}

/// 类的实例存储布局（与 `X__inner` 字段次序一致）
pub fn instance_layout(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<Slot> {
    let tps = ctx.ty.effective_class_type_params(ci).to_vec();
    let mentions = |view: &str| tps.iter().any(|p| contains_word(view, p));
    let sup = flatten_super_fields(ctx, ci);
    let mut out = Vec::new();
    for ((name, view), (decl, java, desc)) in sup.fields.iter().zip(&sup.origin) {
        let erased = sup.erased.contains(name) || mentions(view);
        let prim = PRIMITIVE_RUST_TYPES.contains(&view.as_str()) && !sup.reference.contains(name);
        out.push(Slot {
            rust: name.clone(),
            decl: decl.clone(),
            java: java.clone(),
            desc: desc.clone(),
            cell: cell(ctx, desc, view, erased, prim),
        });
    }
    let super_names: BTreeSet<&str> = sup.fields.iter().map(|(n, _)| n.as_str()).collect();
    let mut own = Vec::new();
    for f in ci.fields().iter().filter(|f| !f.is_static()) {
        let name = ctx.ty.instance_field_rust_name(ci.name(), &safe_ident(&f.name));
        if super_names.contains(name.as_str()) {
            continue;
        }
        let view = resolve_field_rust(ctx, f, &tps).render(&ctx.ty);
        let prim = PRIMITIVE_RUST_TYPES.contains(&view.as_str());
        own.push(Slot {
            rust: name,
            decl: ci.name().to_string(),
            java: f.name.clone(),
            desc: f.desc.clone(),
            cell: cell(ctx, &f.desc, &view, mentions(&view), prim),
        });
    }
    out.extend(own);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heads_and_squash() {
        assert_eq!(head("a::b::X<K, V>"), "X");
        assert_eq!(head("JArray<i8>"), "JArray");
        assert_eq!(squash("JArray<X<Object, Object>>"), "JArray<X<Object,Object>>");
    }
}
