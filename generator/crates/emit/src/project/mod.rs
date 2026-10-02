//! 工程发射（← `emitter/project_writer.write_cargo_project`）。
//!
//! 流程：布局 → 逐类文本（JDK 闭包序，再用户类序）→ 第二阶段收尾（继承补声明、SAM 合成、
//! 反射分派；步骤 (d)）→ 落盘 → 包 mod 树 → lib.rs 补全 → 用户子包 mod.rs → main.rs →
//! Cargo.toml / strict.txt / jdk_feature.txt。

pub mod entry;
pub mod fs;
pub mod layers;
pub mod layout;
pub mod lib_crates;
pub mod mod_tree;
pub mod overlay;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use indexmap::IndexMap;
use ty::{ClassInfo, NameScope};

pub use overlay::prepare_scratch;

use crate::body::MethodBodyEmitter;
use crate::class_writer::{class_prep, class_text, ClassSite, INHERITED_IMPORTS_SLOT};
use crate::ctx::{EmitCtx, HwAudit, ProjectState};
use crate::emission::ClassEmission;
use crate::error::Result;
use crate::imports::import_lines;
use crate::perf::{CrateStat, Perf};
use crate::precheck::Precheck;
use fs::Writer;
use layout::{JdkLayout, UserLayout};
use lib_crates::LibPlan;

/// 发射结果摘要
#[derive(Debug, Default)]
pub struct ProjectReport {
    pub jdk_classes: usize,
    pub user_classes: usize,
    /// 入口类 bin 名（`cargo run --bin`）
    pub bin_name: String,
    /// 类发射记录（发射序）
    pub emissions: Vec<ClassEmission>,
    /// 编译前缺口预检（拆层前的完整方法体文本上扫描）
    pub precheck: Precheck,
    /// FS-H0 手写审计（发射序）
    pub hw_audit: Vec<(HwAudit, String)>,
    /// 方法体生成日志（发射序）
    pub body_log: crate::body::BodyLog,
    /// 分阶段耗时与逐类耗时
    pub perf: Perf,
}

/// 三侧布局
struct Layouts<'l> {
    jdk: &'l JdkLayout,
    libs: &'l LibPlan,
    user: &'l UserLayout,
}

/// 待发射类所属 crate
#[derive(Clone, Copy)]
enum JobCrate<'c> {
    Jdk,
    Lib(&'c str),
    User,
}

/// 一个待发射类
struct ClassJob<'c> {
    binary: &'c str,
    ci: &'c ClassInfo,
    path: PathBuf,
    krate: JobCrate<'c>,
}

/// 类所在 crate 的发射参数
fn site_for<'l>(lay: &Layouts<'l>, krate: JobCrate<'l>) -> ClassSite<'l> {
    let jdk = lay.jdk;
    match krate {
        JobCrate::Jdk => ClassSite { jdk, user: None, lib: None },
        JobCrate::Lib(lib) => ClassSite { jdk, user: None, lib: lay.libs.site(Some(lib)) },
        JobCrate::User => ClassSite { jdk, user: Some(lay.user), lib: lay.libs.site(None) },
    }
}

/// 发射记录所属 crate
fn crate_of<'l>(lay: &Layouts<'l>, em: &ClassEmission) -> JobCrate<'l> {
    match em.crate_name.as_str() {
        "java_runtime" => JobCrate::Jdk,
        "user" => JobCrate::User,
        n => lay.libs.names().find(|l| *l == n).map_or(JobCrate::Jdk, JobCrate::Lib),
    }
}

/// 各文件导入块：文件作用域的认领记录（第一、二阶段全部文本生成完毕后）→ 导入插入位
fn fill_imports(ctx: &EmitCtx<'_>, lay: &Layouts<'_>, ems: &mut IndexMap<String, ClassEmission>) {
    let all: Vec<&ClassEmission> = ems.values().collect();
    let texts = crate::par::par_map(crate::par::resolve_jobs(ctx.opts.jobs), &all, |em| {
        let site = site_for(lay, crate_of(lay, em));
        let block: String = import_lines(ctx, &site.import_site(), &em.scope).into_iter().map(|l| l + "\n").collect();
        crate::phase2::fill_slot(&em.text, INHERITED_IMPORTS_SLOT, &block)
    });
    for (em, t) in ems.values_mut().zip(texts) {
        em.text = t;
    }
}

/// 逐类生成文本（尚未落盘），发射序：JDK（闭包序）→ lib crate（声明序，类名序）→ 用户类。两段：
/// 1. 并行：引用集预认领 / 手写覆盖副本（每类只写自己的作用域）；
/// 2. 并行：类体（方法体生成占绝大部分耗时），每类账本写入独立增量，按发射序并入 `state`。
///
/// 各类只读共享上下文（缓存为纯函数记忆化），故输出与串行发射逐字节一致
fn emit_classes<'l>(
    ctx: &'l EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    w: &Writer,
    lay: &'l Layouts<'l>,
    perf: &mut Perf,
) -> Result<IndexMap<String, ClassEmission>> {
    let (jdk, user) = (lay.jdk, lay.user);
    let mut jobs: Vec<ClassJob<'l>> = Vec::new();
    for (c, path) in &jdk.files {
        if let Some(ci) = ctx.class(c) {
            jobs.push(ClassJob { binary: c, ci, path: path.clone(), krate: JobCrate::Jdk });
        }
    }
    for (lib, files) in &lay.libs.files {
        for (c, path) in files {
            if let Some(ci) = ctx.class(c) {
                jobs.push(ClassJob { binary: c, ci, path: path.clone(), krate: JobCrate::Lib(lib) });
            }
        }
    }
    for (c, e) in &user.entries {
        if let Some(ci) = ctx.class(c) {
            jobs.push(ClassJob { binary: c, ci, path: e.path.clone(), krate: JobCrate::User });
        }
    }
    let threads = crate::par::resolve_jobs(ctx.opts.jobs);
    // 每文件一个名字作用域：本类先认领自己的名字，前置阶段预认领结构化引用集，其后文本渲染时认领
    let scopes: Vec<Arc<NameScope>> = jobs.iter().map(|j| Arc::new(NameScope::new(j.binary, ctx.ty.global_names()))).collect();
    let staged: Vec<(&ClassJob<'l>, &Arc<NameScope>)> = jobs.iter().zip(&scopes).collect();
    let preps = crate::par::par_map(threads, &staged, |(j, scope)| {
        let t = std::time::Instant::now();
        let site = site_for(lay, j.krate);
        let prep = class_prep(&ctx.scoped(scope), j.ci, &site);
        (site, prep, t.elapsed())
    });
    perf.mark("classes.prep");
    let preps: Vec<_> = preps.into_iter().map(|(s, p, t)| p.map(|p| (s, p, t))).collect::<Result<_>>()?;
    let work: Vec<_> = staged.into_iter().zip(preps).collect();
    let texts = crate::par::par_map(threads, &work, |((j, scope), (site, prep, _))| {
        let t = std::time::Instant::now();
        let mut delta = ProjectState::default();
        let ct = class_text(&ctx.scoped(scope), &mut delta, bodies, j.ci, site, prep);
        (ct, delta, t.elapsed())
    });
    let mut ems = IndexMap::new();
    for (((j, scope), (_, _, t_prep)), (ct, delta, t_text)) in work.iter().zip(texts) {
        let ct = ct?;
        state.merge(delta);
        let (crate_prefix, crate_name, handwritten) = match j.krate {
            JobCrate::Jdk => ("crate", "java_runtime", w.is_handwritten(&j.path)),
            JobCrate::Lib(lib) => ("java_runtime", lib, false),
            JobCrate::User => ("java_runtime", "user", false),
        };
        let em = ClassEmission {
            binary_name: j.binary.to_string(),
            crate_prefix: crate_prefix.into(),
            path: j.path.clone(),
            handwritten,
            crate_name: crate_name.into(),
            text: ct.text,
            methods: ct.methods,
            scope: Arc::clone(scope),
        };
        perf.classes.push((j.binary.to_string(), *t_prep + t_text));
        ems.insert(j.binary.to_string(), em);
    }
    Ok(ems)
}

/// 第二阶段（接口实现 / 继承成员 / SAM 对象 / 反射分派）只作用于有布局的类：L1 不透明类
/// 无成员可补，先摘出、完成后按原发射序放回；最后由各文件作用域记录填导入块
fn finish_phase2(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    ems: &mut IndexMap<String, ClassEmission>,
    lay: &Layouts<'_>,
    perf: &mut Perf,
) -> Result<crate::project::entry::DispatchReg> {
    let order: Vec<String> = ems.keys().cloned().collect();
    let mut opaque: IndexMap<String, ClassEmission> = IndexMap::new();
    for k in order.iter().filter(|k| ctx.is_opaque(k)) {
        if let Some(e) = ems.shift_remove(k) {
            opaque.insert(k.clone(), e);
        }
    }
    let disp = crate::phase2::finish(ctx, state, ems, perf)?;
    let mut rest = std::mem::take(ems);
    for k in order {
        if let Some(e) = opaque.shift_remove(&k).or_else(|| rest.shift_remove(&k)) {
            ems.insert(k, e);
        }
    }
    ems.extend(rest);
    fill_imports(ctx, lay, ems);
    perf.mark("phase2.imports");
    Ok(disp)
}

/// 拆层后各 crate 规模：java_runtime（类数取 JDK 布局，与重型判定同源）→ lib（名字序）→
/// 实现层 java_body_k → user
fn crate_stats(ems: &IndexMap<String, ClassEmission>, body: &layers::BodyPlan, jdk_classes: usize) -> Vec<CrateStat> {
    let mut by_crate: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for em in ems.values() {
        let e = by_crate.entry(em.crate_name.as_str()).or_default();
        e.0 += 1;
        if !em.handwritten {
            e.1 += em.text.len();
        }
    }
    let stat = |name: &str, classes: usize, bytes: usize| CrateStat { name: name.to_string(), classes, bytes };
    let jrt = by_crate.remove("java_runtime").unwrap_or_default();
    let user = by_crate.remove("user");
    let mut out = vec![stat("java_runtime", jdk_classes, jrt.1)];
    out.extend(by_crate.iter().map(|(n, (c, b))| stat(n, *c, *b)));
    out.extend(body.crates.iter().map(|c| stat(&c.name, c.files.len(), c.files.values().map(String::len).sum())));
    out.extend(user.map(|(c, b)| stat("user", c, b)));
    out
}

/// 只发射、不落盘的缺口预检（`rava audit` 用）：布局 → 逐类发射 → 第二阶段收尾 → [`Precheck`]。
/// `out_dir` 只用于推导目标路径（判定手写真源同路径覆盖），不创建、不写入
pub fn scan_gaps(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &dyn MethodBodyEmitter) -> Result<Precheck> {
    let jrt_src = out_dir.join("java_runtime").join("src");
    let w = Writer::new(out_dir, &ctx.runtime_src());
    let jdk = JdkLayout::build(ctx, &jrt_src);
    let user = UserLayout::build(ctx, &out_dir.join("user").join("src"));
    let libs = LibPlan::build(ctx, out_dir, &jdk.generated);
    let mut perf = Perf::new();
    let mut state = ProjectState::default();
    let lay = Layouts { jdk: &jdk, libs: &libs, user: &user };
    let mut ems = emit_classes(ctx, &mut state, bodies, &w, &lay, &mut perf)?;
    state.check_lambda_ledger()?;
    finish_phase2(ctx, &mut state, &mut ems, &lay, &mut perf)?;
    Ok(Precheck::scan(ems.values(), &ctx.input.precheck_visited))
}

/// 发射完整 scratch workspace（overlay 需先完成：mod 树按磁盘实际内容重建）
pub fn write_project(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &dyn MethodBodyEmitter) -> Result<ProjectReport> {
    let jrt_src = out_dir.join("java_runtime").join("src");
    let user_src = out_dir.join("user").join("src");
    let runtime_src = ctx.runtime_src();
    let mut perf = Perf::new();
    let mut w = Writer::new(out_dir, &runtime_src);
    let jdk = JdkLayout::build(ctx, &jrt_src);
    let user = UserLayout::build(ctx, &user_src);
    let libs = LibPlan::build(ctx, out_dir, &jdk.generated);
    perf.mark("layout");
    let mut state = ProjectState::default();
    let lay = Layouts { jdk: &jdk, libs: &libs, user: &user };
    let mut ems = emit_classes(ctx, &mut state, bodies, &w, &lay, &mut perf)?;
    state.check_lambda_ledger()?;
    perf.mark("classes");
    let disp = finish_phase2(ctx, &mut state, &mut ems, &lay, &mut perf)?;
    let precheck = Precheck::scan(ems.values(), &ctx.input.precheck_visited);
    // S4 物理拆层：JDK 生成类分声明层（原位）与实现层（java_body_k）
    let body_plan = layers::split(ctx, &mut ems, &jrt_src)?;
    perf.crates = crate_stats(&ems, &body_plan, jdk.files.len());
    perf.mark("layers");
    let files: Vec<(&Path, &str)> = ems.values().map(|em| (em.path.as_path(), em.text.as_str())).collect();
    w.write_all(crate::par::resolve_jobs(ctx.opts.jobs), &files)?;
    entry::write_module_resources(ctx, &mut w, &jrt_src)?;
    perf.mark("write");
    mod_tree::write_mod_tree(&jrt_src, Some(&ctx.runtime_dir), crate::par::resolve_jobs(ctx.opts.jobs), &mut w)?;
    perf.mark("mod_tree");
    libs.write_crates(ctx, &mut w, out_dir)?;
    body_plan.write_crates(ctx, &mut w, out_dir)?;
    let body_names: Vec<&str> = body_plan.names().collect();
    mod_tree::complete_lib_rs(&jrt_src, &runtime_src, &mut w)?;
    entry::write_user_mods(&mut w, &user_src, &user)?;
    let bin = entry::write_main(ctx, &mut w, &user_src, &user, &jdk, &disp, &body_names)?;
    mod_tree::sweep_user_crate(&user_src, &user.mod_tree, &w, crate::par::resolve_jobs(ctx.opts.jobs))?;
    let lib_names: Vec<&str> = libs.names().collect();
    entry::write_cargo_files(ctx, &mut w, out_dir, &bin, &lib_names, &body_names)?;
    perf.mark("entry");
    Ok(ProjectReport {
        jdk_classes: jdk.files.len(),
        user_classes: user.entries.len(),
        bin_name: bin,
        emissions: ems.into_values().collect(),
        precheck,
        hw_audit: std::mem::take(&mut state.hw_audit),
        body_log: std::mem::take(&mut state.body_log),
        perf,
    })
}
