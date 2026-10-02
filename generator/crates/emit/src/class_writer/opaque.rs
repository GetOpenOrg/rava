//! L1（名字级）类的不透明形态（C3 第 6 项）：`java_class_opaque!` 一行声明。
//!
//! 分析器判为 L1 的类只作类型出现、值只可能是 null（其全部非 L1 子类型为空，见分析器
//! `levels.rs`）：不发字段、方法、vtable trait、类初始化，只保留类型身份——签名 / checkcast /
//! instanceof 照常引用；向全部传递超类型的 upcast 由宏按 Object 边界展开。
//! 类级元数据（超类型、修饰符、内部类 / 外围方法、record 等）照发：类镜像经 java_meta 表读取。

use std::collections::BTreeSet;

use ty::ClassInfo;

use crate::ctx::{EmitCtx, ProjectState};
use crate::lang;

use super::{struct_name, ClassSite, ClassText, FILE_ALLOW};

/// 全部传递超类型（超类链与超接口闭包，Object 除外；类在前、接口按发现序）
pub fn opaque_supers(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let reg = ctx.ty.reg;
    let mut out: Vec<String> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut work: Vec<String> = Vec::new();
    let push = |n: &str, out: &mut Vec<String>, seen: &mut BTreeSet<String>, work: &mut Vec<String>| {
        if !n.is_empty() && n != lang::OBJECT && seen.insert(n.to_string()) {
            out.push(n.to_string());
            work.push(n.to_string());
        }
    };
    push(ci.super_class(), &mut out, &mut seen, &mut work);
    for i in ci.interfaces() {
        push(i, &mut out, &mut seen, &mut work);
    }
    while let Some(n) = work.pop() {
        let Some(c) = reg.get(&n) else { continue };
        push(c.super_class(), &mut out, &mut seen, &mut work);
        for i in c.interfaces() {
            push(i, &mut out, &mut seen, &mut work);
        }
    }
    out
}

/// 不透明类文件文本：文件头 + 导入块插入位 + `java_class_opaque!` + implref 稳定别名
pub fn opaque_text(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
) -> ClassText {
    state.generated_classes.insert(ci.name().to_string());
    let sname = struct_name(ctx, ci);
    let tps = ctx.ty.effective_class_type_params(ci);
    let generics = if tps.is_empty() { String::new() } else { format!("<{}>", tps.join(", ")) };
    let supers: Vec<String> = opaque_supers(ctx, ci)
        .iter()
        .filter_map(|s| {
            let sc = ctx.ty.reg.get(s)?;
            let n = ctx.ty.effective_class_type_params(sc).len();
            let args = if n == 0 { String::new() } else { format!("<{}>", vec!["_"; n].join(", ")) };
            Some(format!("{}{args}", ctx.short(s)))
        })
        .collect();
    let bound = if supers.is_empty() { String::new() } else { format!(": {}", supers.join(", ")) };
    let mut parts: Vec<String> = vec![FILE_ALLOW.to_string(), format!("use {}::prelude::*;", site.prefix())];
    parts.push(super::INHERITED_IMPORTS_SLOT.to_string());
    parts.push(String::new());
    parts.push("rava_macros::java_class_opaque! {".into());
    parts.extend(super::head::opaque_metadata_lines(ctx, ci).into_iter().map(|l| format!("    {l}")));
    parts.push(format!("    pub struct {sname}{generics}{bound};"));
    parts.push("}".into());
    parts.push(String::new());
    let java_simple = ci.name().rsplit('/').next().unwrap_or(ci.name()).replace('$', "_");
    parts.push("#[doc(hidden)]".into());
    parts.push(format!("pub mod implref {{ pub use super::{sname} as {java_simple}; }}"));
    parts.push(String::new());
    ClassText { text: parts.join("\n"), methods: Vec::new() }
}
