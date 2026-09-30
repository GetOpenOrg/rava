//! 第二阶段收尾（← `project_writer` 690–705 段）：全部类文本生成后，按定义侧记录补齐
//! 插入位，再统一落盘。
//!
//! 顺序与 Python 一致：
//! 1. [`iface_impls::resolve_interface_impls`]：具体类的接口 vtable impl（`IMPLS_SLOT`）、
//!    协变 upcast（`UPCASTS_SLOT`），需要的继承成员登记到需求账本；
//! 2. [`iface_impls::resolve_interface_inherited_members`]：接口接收者的超接口成员；
//! 3. [`inherited::resolve_inherited_members`]：类接收者的继承成员（桥接优先）+ use 行；
//! 4. [`sam_objects::synthesize`]：函数式接口合成对象（接口文件尾段）；
//! 5. [`dispatch::synthesize`]：L3 反射分派 / 字段闭包（类文件尾段）与 main 登记行。

pub mod bridge;
pub mod dispatch;
pub mod iface_impls;
pub mod inherited;
pub mod sam_objects;
pub mod sig;
pub mod uses;

use std::collections::BTreeSet;

use indexmap::IndexMap;
use ty::ClassInfo;

use crate::ctx::{EmitCtx, ProjectState};
use crate::emission::ClassEmission;
use crate::error::Result;
use crate::perf::Perf;
use crate::project::entry::DispatchReg;

/// 发射记录（binary name → 记录；发射序）
pub type Emissions = IndexMap<String, ClassEmission>;

/// 超类链每个祖先的类型实参（渲染文本；直接父类在前）
pub fn anc_args(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<(String, Vec<String>)> {
    let names = ctx.ty.names;
    ctx.ty
        .ancestor_type_args(ci, None)
        .into_iter()
        .map(|(b, a)| (b, a.iter().map(|t| t.render(names)).collect()))
        .collect()
}

/// 类的有效类型形参
pub fn class_params(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    ctx.ty.effective_class_type_params(ci).to_vec()
}

/// 整行插入位（`^[ \t]*SLOT\n`，全部出现）替换为 `repl`
pub fn fill_slot(text: &str, slot: &str, repl: &str) -> String {
    let mut out = String::with_capacity(text.len() + repl.len());
    for line in text.split_inclusive('\n') {
        if line.strip_suffix('\n').is_some_and(|l| l.trim_start_matches([' ', '\t']) == slot) {
            out.push_str(repl);
        } else {
            out.push_str(line);
        }
    }
    out
}

/// 在首个整行插入位之前插入 `ins`（插入位保留）
pub fn insert_before_slot(text: &str, slot: &str, ins: &str) -> String {
    let mut out = String::with_capacity(text.len() + ins.len());
    let mut done = false;
    for line in text.split_inclusive('\n') {
        if !done && line.strip_suffix('\n').is_some_and(|l| l.trim_start_matches([' ', '\t']) == slot) {
            out.push_str(ins);
            done = true;
        }
        out.push_str(line);
    }
    out
}

/// 继承成员需求按接收者分组（接收者首次登记序；组内按 (名, 参数描述符) 排序）
pub fn requests_by_recv(state: &ProjectState) -> Vec<(String, BTreeSet<(String, String)>)> {
    let mut groups: IndexMap<String, BTreeSet<(String, String)>> = IndexMap::new();
    for (r, n, d) in &state.inherited_requests {
        if r.is_empty() || n.is_empty() {
            continue;
        }
        groups.entry(r.clone()).or_default().insert((n.clone(), d.clone()));
    }
    groups.into_iter().collect()
}

/// 共置手写文件提供的方法名
pub fn provided_methods(ctx: &EmitCtx<'_>, bin: &str) -> BTreeSet<String> {
    ctx.input.handwritten.get(bin).map(|h| h.methods.iter().cloned().collect()).unwrap_or_default()
}

/// 第二阶段全序（Python 同序）：接口实现 → 接口接收者继承成员 → 类接收者继承成员 →
/// SAM 合成对象 → 反射闭包；返回 main 反射登记行
pub fn finish(ctx: &EmitCtx<'_>, state: &mut ProjectState, ems: &mut Emissions, perf: &mut Perf) -> Result<DispatchReg> {
    ctx.sam().check_sites(&state.sam_sites)?;
    iface_impls::resolve_interface_impls(ctx, state, ems);
    perf.mark("phase2.impls");
    iface_impls::resolve_interface_inherited_members(ctx, state, ems);
    perf.mark("phase2.iface_inherited");
    inherited::resolve_inherited_members(ctx, state, ems);
    perf.mark("phase2.inherited");
    sam_objects::synthesize(ctx, ems)?;
    perf.mark("phase2.sam");
    let reg = dispatch::synthesize(ctx, ems);
    perf.mark("phase2.dispatch");
    Ok(reg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_fill() {
        let t = "a\n    //@@S@@\nb\n";
        assert_eq!(fill_slot(t, "//@@S@@", "X\n"), "a\nX\nb\n");
        assert_eq!(fill_slot(t, "//@@S@@", ""), "a\nb\n");
        assert_eq!(insert_before_slot(t, "//@@S@@", "u\n"), "a\nu\n    //@@S@@\nb\n");
    }
}
