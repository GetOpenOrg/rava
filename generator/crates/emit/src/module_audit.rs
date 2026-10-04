//! 模块可读性审计（`[module-audit]`，T1 第 2 步 M1）。
//!
//! 每个发射类的结构化引用集（[`crate::imports::collect_referenced`]，即 `use` 行与正文名字的来源）逐项检查：
//! 被引用类的模块须在本类模块的可读范围内（本模块及其 requires 闭包，[`resolve::ModuleGraph::reads`]）。
//! 越界即反向 / 互不可达的模块边——按模块切 crate（M2）后它就是 crate 依赖环或缺依赖，终态为 0。
//! 本行同时给出发射类覆盖的模块数，供 M2 / M3 对账。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

use ty::ClassInfo;

use crate::ctx::EmitCtx;

/// 越界明细在审计行中最多列出的条数（`debug` 时全列）
const SHOWN: usize = 20;

#[derive(Debug, Default)]
pub struct ModuleAudit {
    /// 发射类 → 所属模块（None = 无名模块）
    classes: Mutex<BTreeMap<String, Option<String>>>,
    /// (引用方类, 被引用类)
    breaches: Mutex<BTreeSet<(String, String)>>,
}

impl ModuleAudit {
    /// 检查一个类的结构化引用集
    pub fn check(&self, ctx: &EmitCtx<'_>, ci: &ClassInfo, referenced: &BTreeSet<String>) {
        let g = ctx.modules();
        let here = g.module_of(ci.name());
        let bad: Vec<(String, String)> = referenced
            .iter()
            .filter(|r| !g.reads(here, g.module_of(r)))
            .map(|r| (ci.name().to_string(), r.clone()))
            .collect();
        lock(&self.classes).insert(ci.name().to_string(), here.map(str::to_string));
        if !bad.is_empty() {
            lock(&self.breaches).extend(bad);
        }
    }

    /// 越界条数
    pub fn breaches(&self) -> usize {
        lock(&self.breaches).len()
    }

    /// `[module-audit]` 行：模块数（无名模块不计）、越界条数；非零时附明细
    pub fn lines(&self, ctx: &EmitCtx<'_>, debug: bool) -> Vec<String> {
        let classes = lock(&self.classes);
        let modules: BTreeSet<&str> = classes.values().filter_map(Option::as_deref).collect();
        let breaches = lock(&self.breaches);
        let mut out = vec![format!("[module-audit] modules={} out_of_reads={}", modules.len(), breaches.len())];
        let g = ctx.modules();
        let shown = if debug { usize::MAX } else { SHOWN };
        out.extend(breaches.iter().take(shown).map(|(from, to)| {
            format!(
                "[module-audit] out-of-reads {from} ({}) -> {to} ({})",
                g.module_of(from).unwrap_or("-"),
                g.module_of(to).unwrap_or("-")
            )
        }));
        out
    }
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
