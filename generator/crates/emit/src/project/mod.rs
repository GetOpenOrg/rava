//! 工程发射（← `emitter/project_writer.write_cargo_project`）。
//!
//! 流程：布局 → 逐类文本（JDK 闭包序，再用户类序）→ 第二阶段收尾（继承补声明、SAM 合成、
//! 反射分派；步骤 (d)）→ 落盘 → 包 mod 树 → lib.rs 补全 → 用户子包 mod.rs → main.rs →
//! Cargo.toml / strict.txt / jdk_feature.txt。

mod archive_side;
pub mod meta_sides;
pub mod entry;
pub mod fs;
pub mod layers;
pub mod line_tables;
pub mod layout;
pub mod lib_crates;
pub mod mod_tree;
pub mod module_side;
pub mod overlay;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use indexmap::IndexMap;
use ty::{ClassInfo, NameScope};

pub use overlay::{prepare_scratch, JdkDirs};

use crate::body::MethodBodyEmitter;
use crate::class_writer::{class_prep, class_text, ClassSite, INHERITED_IMPORTS_SLOT};
use crate::ctx::{EmitCtx, HwAudit, ProjectState, USER_CRATE};
use crate::module_crates::ModuleCrates;
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
    /// 可读层禁用形态计数（拆层前的类发射文本，每个方法体计一次）
    pub readability: crate::audit::ReadabilityCounts,
    /// 分阶段耗时与逐类耗时
    pub perf: Perf,
}

/// 三侧布局
struct Layouts<'l> {
    jdk: &'l JdkLayout,
    libs: &'l LibPlan,
    user: &'l UserLayout,
    /// JDK 模块 crate 表
    crates: &'l ModuleCrates,
}

/// 待发射类所属 crate
#[derive(Clone, Copy)]
enum JobCrate<'c> {
    /// JDK 模块 crate（crate 名）
    Jdk(&'c str),
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
    let (jdk, crates) = (lay.jdk, lay.crates);
    match krate {
        JobCrate::Jdk(here) => ClassSite { jdk, user: None, lib: None, here, crates },
        JobCrate::Lib(lib) => ClassSite { jdk, user: None, lib: lay.libs.site(Some(lib)), here: lib, crates },
        JobCrate::User => ClassSite { jdk, user: Some(lay.user), lib: lay.libs.site(None), here: USER_CRATE, crates },
    }
}

/// 发射记录所属 crate
fn crate_of<'l>(lay: &Layouts<'l>, em: &ClassEmission) -> JobCrate<'l> {
    let n = em.crate_name.as_str();
    if n == USER_CRATE {
        return JobCrate::User;
    }
    if let Some(l) = lay.libs.names().find(|l| *l == n) {
        return JobCrate::Lib(l);
    }
    JobCrate::Jdk(lay.crates.all().iter().find(|c| c.name == n).map_or(lay.crates.root(), |c| c.name.as_str()))
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
            jobs.push(ClassJob { binary: c, ci, path: path.clone(), krate: JobCrate::Jdk(lay.crates.crate_of(c)) });
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
    let mut held = Vec::new();
    for (((j, scope), (_, _, t_prep)), (ct, mut delta, t_text)) in work.iter().zip(texts) {
        let ct = ct?;
        if matches!(j.krate, JobCrate::User) {
            archive_side::split_user_delta(ctx, &mut delta, &mut held);
        }
        state.merge(delta);
        let root = lay.crates.root();
        let (crate_prefix, crate_name, handwritten) = match j.krate {
            JobCrate::Jdk(here) => ("crate", here, w.is_handwritten(&j.path)),
            JobCrate::Lib(lib) => (root, lib, false),
            JobCrate::User => (root, USER_CRATE, false),
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
    archive_side::check_leaks(state, held);
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

/// 拆层后各 crate 规模：根声明层（类数取根模块布局，与重型判定同源）→ 其余模块 crate 与 lib（名字序）→
/// 实现层 `<根>_body_k` → user
fn crate_stats(
    ems: &IndexMap<String, ClassEmission>, body: &layers::BodyPlan, crates: &ModuleCrates, root_classes: usize,
) -> Vec<CrateStat> {
    let mut by_crate: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for em in ems.values() {
        let e = by_crate.entry(em.crate_name.as_str()).or_default();
        e.0 += 1;
        if !em.handwritten {
            e.1 += em.text.len();
        }
    }
    let stat = |name: &str, classes: usize, bytes: usize| CrateStat { name: name.to_string(), classes, bytes };
    let decl = by_crate.remove(crates.root()).unwrap_or_default();
    let user = by_crate.remove(USER_CRATE);
    let mut out = vec![stat(&crates.decl(), root_classes, decl.1)];
    out.extend(by_crate.iter().map(|(n, (c, b))| stat(n, *c, *b)));
    out.extend(body.crates.iter().map(|c| stat(&c.name, c.files.len(), c.files.values().map(String::len).sum())));
    out.extend(user.map(|(c, b)| stat(USER_CRATE, c, b)));
    out
}

/// 只发射、不落盘的缺口预检（`rava audit` 用）：布局 → 逐类发射 → 第二阶段收尾 → [`Precheck`]。
/// `out_dir` 只用于推导目标路径（判定手写真源同路径覆盖），不创建、不写入
pub fn scan_gaps(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &dyn MethodBodyEmitter) -> Result<Precheck> {
    let crates = ctx.crates();
    let w = Writer::new(&crates.src_dirs(out_dir), &ctx.runtime_src());
    let jdk = JdkLayout::build(ctx, out_dir);
    let user = UserLayout::build(ctx, &out_dir.join("user").join("src"));
    let libs = LibPlan::build(ctx, out_dir, &jdk.generated);
    let mut perf = Perf::new();
    let mut state = ProjectState::default();
    archive_side::seed_requests(ctx, &mut state);
    let lay = Layouts { jdk: &jdk, libs: &libs, user: &user, crates };
    let mut ems = emit_classes(ctx, &mut state, bodies, &w, &lay, &mut perf)?;
    state.check_lambda_ledger()?;
    finish_phase2(ctx, &mut state, &mut ems, &lay, &mut perf)?;
    Ok(Precheck::scan(ems.values(), &ctx.input.precheck_visited))
}

/// 发射完整 scratch workspace（overlay 需先完成：mod 树按磁盘实际内容重建）
pub fn write_project(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &dyn MethodBodyEmitter) -> Result<ProjectReport> {
    let crates = ctx.crates();
    let jdk_srcs = crates.src_dirs(out_dir);
    let decl_src = crates.src_dir(out_dir, crates.root());
    let user_src = out_dir.join(USER_CRATE).join("src");
    let runtime_src = ctx.runtime_src();
    let mut perf = Perf::new();
    let mut w = Writer::new(&jdk_srcs, &runtime_src);
    let jdk = JdkLayout::build(ctx, out_dir);
    let user = UserLayout::build(ctx, &user_src);
    let libs = LibPlan::build(ctx, out_dir, &jdk.generated);
    perf.mark("layout");
    let mut state = ProjectState::default();
    archive_side::seed_requests(ctx, &mut state);
    let lay = Layouts { jdk: &jdk, libs: &libs, user: &user, crates };
    let mut ems = emit_classes(ctx, &mut state, bodies, &w, &lay, &mut perf)?;
    state.check_lambda_ledger()?;
    perf.mark("classes");
    let disp = finish_phase2(ctx, &mut state, &mut ems, &lay, &mut perf)?;
    // 模块可读性越界 = 模块 crate 依赖环 / 缺依赖（按模块切 crate 后不可编译）：生成期即报错
    if ctx.module_audit.breaches() > 0 {
        let detail = ctx.module_audit.lines(ctx, true).join("\n");
        return Err(crate::error::EmitError::Assert(format!("模块可读性越界，模块 crate 依赖不成立：\n{detail}")));
    }
    let precheck = Precheck::scan(ems.values(), &ctx.input.precheck_visited);
    let readability = crate::audit::readability_counts(ems.values().map(|em| em.text.as_str()));
    // S4 物理拆层：根模块生成类分声明层（原位）与实现层（`<根>_body_k`）；其余模块 crate 不拆
    let body_plan = layers::split(ctx, &mut ems, &decl_src)?;
    let root_classes = jdk.files.keys().filter(|c| crates.crate_of(c) == crates.root()).count();
    perf.crates = crate_stats(&ems, &body_plan, crates, root_classes);
    perf.mark("layers");
    let files: Vec<(&Path, &str)> = ems.values().map(|em| (em.path.as_path(), em.text.as_str())).collect();
    w.write_all(crate::par::resolve_jobs(ctx.opts.jobs), &files)?;
    entry::write_module_resources(ctx, &mut w, &decl_src)?;
    entry::write_closure_tables(ctx, &mut w, out_dir)?;
    perf.mark("write");
    for src in &jdk_srcs {
        mod_tree::write_mod_tree(src, Some(&ctx.runtime_dir), crate::par::resolve_jobs(ctx.opts.jobs), &mut w)?;
    }
    perf.mark("mod_tree");
    module_side::write_module_crates(ctx, &mut w, out_dir)?;
    libs.write_crates(ctx, &mut w, out_dir)?;
    body_plan.write_crates(ctx, &mut w, out_dir)?;
    // FS-E1：落盘文本的 Java 行表（拆层后各文件的最终行号）
    let body_files = body_plan.files(out_dir);
    let mut final_files = files;
    final_files.extend(body_files.iter().map(|(p, t)| (p.as_path(), *t)));
    let lnt = |class: &str, name: &str, desc: &str| {
        let i = ctx.class(class)?.methods().iter().position(|m| m.name == name && m.desc == desc)?;
        ctx.extras(class).methods.get(i).map(|m| m.line_numbers.clone())
    };
    let user_lines = line_tables::write(&mut w, out_dir, &final_files, &lnt, &root_line_registration(ctx, &decl_src))?;
    meta_sides::write_user(ctx, &mut w, &user_src, &final_files, &user_lines)?;
    let body_names: Vec<&str> = body_plan.names().collect();
    mod_tree::complete_lib_rs(&decl_src, &runtime_src, &mut w)?;
    module_side::write_facade(&mut w, out_dir, crates, &body_names)?;
    entry::write_user_mods(&mut w, &user_src, &user)?;
    let bin = entry::write_main(ctx, &mut w, &user_src, &user, &jdk, &ems, &disp)?;
    mod_tree::sweep_user_crate(&user_src, &user.mod_tree, &w, crate::par::resolve_jobs(ctx.opts.jobs))?;
    let lib_names: Vec<&str> = libs.names().collect();
    let lib_srcs: Vec<PathBuf> = lib_names.iter().map(|n| out_dir.join(n).join("src")).collect();
    let archive_roots: Vec<&Path> = jdk_srcs.iter().chain(lib_srcs.iter()).map(PathBuf::as_path).collect();
    meta_sides::write_archive(ctx, &mut w, out_dir, &archive_roots)?;
    entry::write_cargo_files(ctx, &mut w, out_dir, &bin, &lib_names, &body_names)?;
    if ctx.opts.archive {
        let decl = crates.decl();
        let stamped: Vec<&str> = std::iter::once(decl.as_str())
            .chain(crates.all().iter().map(|c| c.name.as_str()))
            .chain([entry::META_CRATE])
            .chain(body_names.iter().copied())
            .chain(lib_names.iter().copied())
            .collect();
        let included = [meta_sides::META_TABLES, entry::CLOSURE_TABLES, line_tables::LINE_TABLES_PATH];
        archive_side::stamp_versions(&mut w, out_dir, &stamped, &included)?;
    }
    perf.mark("entry");
    state.hw_audit.extend(crate::audit::handwritten_vtable_impls(&runtime_src).into_iter().map(|m| (HwAudit::VtableImpl, m)));
    Ok(ProjectReport {
        jdk_classes: jdk.files.len(),
        user_classes: user.entries.len(),
        bin_name: bin,
        emissions: ems.into_values().collect(),
        precheck,
        hw_audit: std::mem::take(&mut state.hw_audit),
        body_log: std::mem::take(&mut state.body_log),
        readability,
        perf,
    })
}

/// 手写根类的行表登记（根类无生成文件）：根类字节码的方法按调用侧根类命名规则对应 overlay
/// 落盘的根类手写文件中的 fn（根类重载取描述符后缀名，前提是该名在手写 API 名面中；
/// 规则与登记形态见 `line_tables::handwritten::root_methods`）
fn root_line_registration(ctx: &EmitCtx<'_>, decl_src: &Path) -> Vec<(PathBuf, Vec<line_tables::handwritten::HwMethod>)> {
    let root = ty::consts::OBJECT;
    let Some(cf) = ctx.cp.get(root) else { return Vec::new() };
    let source = cf.source_file.clone().unwrap_or_default();
    let rust_name = |name: &str, desc: &str| {
        let mangled = ty::type_map::mangle_name(&ctx.manifest.ty, name, desc);
        let chosen = if mangled != name && ctx.root_api().contains(&mangled) { mangled } else { name.to_string() };
        ty::ident::safe_ident(&chosen)
    };
    // 可覆盖 = 实例、非 final、非 private（虚派发目标可被子类替换）
    let overridable = |a: u16| a & (classfile::acc::STATIC | classfile::acc::FINAL | classfile::acc::PRIVATE) == 0;
    let methods = cf.methods.iter().map(|m| (m.name.as_str(), m.desc.as_str(), m.is_native(), overridable(m.access)));
    let hws = line_tables::handwritten::root_methods(root, &source, &ctx.short(root), methods, &rust_name);
    crate::ctx::EmitShared::root_files(decl_src).into_iter().map(|p| (p, hws.clone())).collect()
}
