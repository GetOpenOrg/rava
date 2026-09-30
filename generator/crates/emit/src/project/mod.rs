//! 工程发射（← `emitter/project_writer.write_cargo_project`）。
//!
//! 流程：布局 → 逐类文本（JDK 闭包序，再用户类序）→ 第二阶段收尾（继承补声明、SAM 合成、
//! 反射分派；步骤 (d)）→ 落盘 → 包 mod 树 → lib.rs 补全 → 用户子包 mod.rs → main.rs →
//! Cargo.toml / strict.txt / jdk_feature.txt。

pub mod entry;
pub mod fs;
pub mod layout;
pub mod mod_tree;
pub mod overlay;
#[cfg(test)]
mod tests;

use std::path::Path;

use indexmap::IndexMap;

pub use overlay::prepare_scratch;

use crate::body::MethodBodyEmitter;
use crate::class_writer::{gen_class_rs, ClassSite};
use crate::ctx::{EmitCtx, HwAudit, ProjectState};
use crate::emission::ClassEmission;
use crate::error::{EmitError, Result};
use crate::imports::collect_referenced;
use crate::perf::Perf;
use fs::Writer;
use layout::{JdkLayout, UserLayout};

/// 发射结果摘要
#[derive(Debug, Default)]
pub struct ProjectReport {
    pub jdk_classes: usize,
    pub user_classes: usize,
    /// 入口类 bin 名（`cargo run --bin`）
    pub bin_name: String,
    /// 类发射记录（发射序）
    pub emissions: Vec<ClassEmission>,
    /// FS-H0 手写审计（发射序）
    pub hw_audit: Vec<(HwAudit, String)>,
    /// 分阶段耗时与逐类耗时
    pub perf: Perf,
}

/// 逐类生成文本（尚未落盘）
fn emit_classes(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    w: &Writer,
    jdk: &JdkLayout,
    user: &UserLayout,
    perf: &mut Perf,
) -> Result<IndexMap<String, ClassEmission>> {
    let mut ems = IndexMap::new();
    for (c, path) in &jdk.files {
        let Some(ci) = ctx.class(c) else { continue };
        let t0 = std::time::Instant::now();
        let ct = gen_class_rs(ctx, state, bodies, ci, &ClassSite { jdk, user_sibling_imports: None })?;
        let em = ClassEmission {
            binary_name: c.clone(),
            crate_prefix: "crate".into(),
            path: path.clone(),
            handwritten: w.is_handwritten(path),
            crate_name: "java_runtime".into(),
            text: ct.text,
            methods: ct.methods,
        };
        perf.classes.push((c.clone(), t0.elapsed()));
        ems.insert(c.clone(), em);
    }
    for (c, e) in &user.entries {
        let Some(ci) = ctx.class(c) else { continue };
        let t0 = std::time::Instant::now();
        // 兄弟类导入按未过滤引用集（生成集过滤只作用于 JDK 导入）
        let referenced = collect_referenced(ctx, ci, None);
        let site = ClassSite { jdk, user_sibling_imports: Some(user.sibling_imports(ctx, c, &referenced)) };
        let ct = gen_class_rs(ctx, state, bodies, ci, &site)?;
        let em = ClassEmission {
            binary_name: c.clone(),
            crate_prefix: "java_runtime".into(),
            path: e.path.clone(),
            handwritten: false,
            crate_name: "user".into(),
            text: ct.text,
            methods: ct.methods,
        };
        perf.classes.push((c.clone(), t0.elapsed()));
        ems.insert(c.clone(), em);
    }
    Ok(ems)
}

/// 发射完整 scratch workspace（overlay 需先完成：mod 树按磁盘实际内容重建）
pub fn write_project(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &mut dyn MethodBodyEmitter) -> Result<ProjectReport> {
    if !ctx.input.lib_crates.is_empty() {
        return Err(EmitError::Unported("lib crate 模式（jar 输入）未移植".into()));
    }
    let jrt_src = out_dir.join("java_runtime").join("src");
    let user_src = out_dir.join("user").join("src");
    let runtime_src = ctx.runtime_src();
    let mut perf = Perf::new();
    let mut w = Writer::new(out_dir, &runtime_src);
    let jdk = JdkLayout::build(ctx, &jrt_src);
    let user = UserLayout::build(ctx, &user_src);
    perf.mark("layout");
    let mut state = ProjectState::default();
    let mut ems = emit_classes(ctx, &mut state, bodies, &w, &jdk, &user, &mut perf)?;
    state.check_lambda_ledger()?;
    perf.mark("classes");
    let disp = crate::phase2::finish(ctx, &mut state, &mut ems)?;
    perf.mark("phase2");
    for em in ems.values() {
        w.write(&em.path, &em.text)?;
    }
    entry::write_module_resources(ctx, &mut w, &jrt_src)?;
    perf.mark("write");
    mod_tree::write_mod_tree(&jrt_src, Some(&ctx.runtime_dir), &mut w)?;
    mod_tree::complete_lib_rs(&jrt_src, &runtime_src, &mut w)?;
    entry::write_user_mods(&mut w, &user_src, &user)?;
    let bin = entry::write_main(ctx, &mut w, &user_src, &user, &jdk, &disp)?;
    entry::write_cargo_files(ctx, &mut w, out_dir, &bin)?;
    perf.mark("mod_tree+entry");
    Ok(ProjectReport {
        jdk_classes: jdk.files.len(),
        user_classes: user.entries.len(),
        bin_name: bin,
        emissions: ems.into_values().collect(),
        hw_audit: std::mem::take(&mut state.hw_audit),
        perf,
    })
}
