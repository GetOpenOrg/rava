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

use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use ty::ClassInfo;

pub use overlay::prepare_scratch;

use crate::body::MethodBodyEmitter;
use crate::class_writer::{class_cross_imports, class_prep, class_text, ClassSite};
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
    /// 方法体生成日志（发射序）
    pub body_log: crate::body::BodyLog,
    /// 分阶段耗时与逐类耗时
    pub perf: Perf,
}

/// 一个待发射类
struct ClassJob<'c> {
    binary: &'c str,
    ci: &'c ClassInfo,
    path: PathBuf,
    user: bool,
}

/// 逐类生成文本（尚未落盘）。三段：
/// 1. 并行：引用集 / 手写覆盖副本 / 跨类导入规划（只读）；
/// 2. 串行（发射序）：跨类导入短名裁决——`seen_simples` 首个引入者胜出，结果依赖发射序（只做查表）；
/// 3. 并行：类体（方法体生成占绝大部分耗时），每类账本写入独立增量，按发射序并入 `state`。
///
/// 除 2 外各类只读共享上下文（缓存为纯函数记忆化），故输出与串行发射逐字节一致
fn emit_classes(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    w: &Writer,
    jdk: &JdkLayout,
    user: &UserLayout,
    perf: &mut Perf,
) -> Result<IndexMap<String, ClassEmission>> {
    let mut jobs: Vec<ClassJob<'_>> = Vec::new();
    for (c, path) in &jdk.files {
        if let Some(ci) = ctx.class(c) {
            jobs.push(ClassJob { binary: c, ci, path: path.clone(), user: false });
        }
    }
    for (c, e) in &user.entries {
        if let Some(ci) = ctx.class(c) {
            jobs.push(ClassJob { binary: c, ci, path: e.path.clone(), user: true });
        }
    }
    let threads = crate::par::resolve_jobs(ctx.opts.jobs);
    let site_of = |j: &ClassJob<'_>| {
        // 用户类兄弟导入按未过滤引用集（生成集过滤只作用于 JDK 导入）
        let sib = j.user.then(|| user.sibling_imports(ctx, j.binary, &collect_referenced(ctx, j.ci, None)));
        ClassSite { jdk, user_sibling_imports: sib }
    };
    let preps = crate::par::par_map(threads, &jobs, |j| {
        let t = std::time::Instant::now();
        let site = site_of(j);
        let prep = class_prep(ctx, j.ci, &site);
        (site, prep, t.elapsed())
    });
    perf.mark("classes.prep");
    let preps: Vec<_> = preps.into_iter().map(|(s, p, t)| p.map(|p| (s, p, t))).collect::<Result<_>>()?;
    let cross: Vec<_> = preps.iter().map(|(_, prep, _)| class_cross_imports(state, prep)).collect();
    perf.mark("classes.imports");
    let work: Vec<_> = jobs.iter().zip(preps).zip(cross).collect();
    let texts = crate::par::par_map(threads, &work, |((j, (site, prep, _)), imports)| {
        let t = std::time::Instant::now();
        let mut delta = ProjectState::default();
        let ct = class_text(ctx, &mut delta, bodies, j.ci, site, prep, imports.clone());
        (ct, delta, t.elapsed())
    });
    let mut ems = IndexMap::new();
    for (((j, (_, _, t_prep)), _), (ct, delta, t_text)) in work.iter().zip(texts) {
        let ct = ct?;
        state.merge(delta);
        let em = ClassEmission {
            binary_name: j.binary.to_string(),
            crate_prefix: if j.user { "java_runtime" } else { "crate" }.into(),
            path: j.path.clone(),
            handwritten: !j.user && w.is_handwritten(&j.path),
            crate_name: if j.user { "user" } else { "java_runtime" }.into(),
            text: ct.text,
            methods: ct.methods,
        };
        perf.classes.push((j.binary.to_string(), *t_prep + t_text));
        ems.insert(j.binary.to_string(), em);
    }
    Ok(ems)
}

/// 发射完整 scratch workspace（overlay 需先完成：mod 树按磁盘实际内容重建）
pub fn write_project(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &dyn MethodBodyEmitter) -> Result<ProjectReport> {
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
    let disp = crate::phase2::finish(ctx, &mut state, &mut ems, &mut perf)?;
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
        body_log: std::mem::take(&mut state.body_log),
        perf,
    })
}
