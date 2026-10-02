//! 文件导入块：由文件名字作用域的认领记录生成。
//!
//! 文件里出现的每个类名都在本文件作用域认领过（结构化引用集预认领 + 渲染时认领），
//! 认领记录即导入来源：类名导入（binary 序）→ 派生名导入（`__VTable` / `__m_base`，
//! binary 序）。路径按目标类所在位置定向：同 crate 用户类走其模块，其余生成类走包路径
//! （crate 前缀按归属定向，lib crate 只导入依赖方向允许的 crate）；不在生成集的类不导入。

use std::collections::BTreeSet;

use ty::NameScope;

use super::cross::{rust_pkg_of, CrossInput};
use crate::ctx::EmitCtx;
use crate::project::layout::UserLayout;

/// prelude 同名 newtype：不 use 导入（调用方走全路径）
const PRELUDE_NEWTYPE_NAMES: [&str; 1] = ["JArray"];

/// 文件所在 crate 的导入定向参数
pub struct ImportSite<'s> {
    pub cross: CrossInput<'s>,
    /// user crate 布局（本文件属 user crate 时）
    pub user: Option<&'s UserLayout>,
}

/// 类在定义处的 Rust 名（导入路径末段）
fn declared(ctx: &EmitCtx<'_>, binary: &str) -> String {
    ctx.ty.global_names().short(binary).into_owned()
}

/// 继承链顶（无父类的根类）：`__base` 自由函数恒可导入
fn is_chain_root(ctx: &EmitCtx<'_>, binary: &str) -> bool {
    ctx.ty.reg.get(binary).is_some_and(|c| c.super_class().is_empty())
}

/// 类定义所在模块路径（不含末段名）；不可导入 → None。`derived`：派生名（根类的 `__base`
/// 自由函数不受生成集约束）
fn module_path(ctx: &EmitCtx<'_>, site: &ImportSite<'_>, binary: &str, derived: bool) -> Option<String> {
    if let Some(e) = site.user.and_then(|u| u.entries.get(binary)) {
        let mut segs = vec!["crate".to_string()];
        segs.extend(e.pkg_parts.iter().cloned());
        segs.push(e.mod_name.clone());
        return Some(segs.join("::"));
    }
    let inp = &site.cross;
    let generated = inp.generated?;
    if !generated.contains(binary) && !(derived && is_chain_root(ctx, binary)) {
        return None;
    }
    let pkg = rust_pkg_of(binary)?;
    let target = inp.prefix_of(binary);
    if inp.route.is_some_and(|r| !r.reachable(target)) {
        return None;
    }
    Some(format!("{target}::{pkg}"))
}

/// `use <模块>::<定义名>[ as <本地名>];`
fn use_line(module: &str, declared: &str, local: &str) -> String {
    if declared == local {
        format!("use {module}::{declared};")
    } else {
        format!("use {module}::{declared} as {local};")
    }
}

/// 作用域记录 → 导入行（类名导入 → 派生名导入；同一行只出一次）
pub fn import_lines(ctx: &EmitCtx<'_>, site: &ImportSite<'_>, scope: &NameScope) -> Vec<String> {
    let st = scope.snapshot();
    let self_bin = scope.self_bin();
    let self_name = st.name_of(self_bin).unwrap_or_default();
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for (b, local) in st.claims() {
        if b == self_bin || local == self_name || PRELUDE_NEWTYPE_NAMES.contains(&local) {
            continue;
        }
        let Some(m) = module_path(ctx, site, b, false) else { continue };
        let line = use_line(&m, &declared(ctx, b), local);
        if seen.insert(line.clone()) {
            out.push(line);
        }
    }
    for (b, suffix) in st.derived() {
        if b == self_bin {
            continue;
        }
        let Some(m) = module_path(ctx, site, b, true) else { continue };
        let local = st.name_of(b).unwrap_or_default();
        let line = use_line(&m, &format!("{}{suffix}", declared(ctx, b)), &format!("{local}{suffix}"));
        if seen.insert(line.clone()) {
            out.push(line);
        }
    }
    out
}
