//! `rava build` / `rava emit`：P0 转译外壳（计划 docs/plans/2026-09-20-rust-generator-rewrite.md P0 / P5a）。
//!
//! build：javac → 闭包分析（同 `rava closure`）→ 闭包事实进程内直传 →
//! [`EmitInput`](input::EmitInput) → overlay → 发射层写 scratch → cargo 编译（[`crate::cargo`]）→ 运行，
//! 到 `--stop-after` 为止；阶段与失败现场写 `<scratch>/build_status.json`（[`crate::status`]）。
//! `--closure-json` 时另把 closure.json 落 `<scratch>/closure_input/`，并校验由它解析的事实与直传的一致。
//! emit：从既有 closure.json + 用户类目录重建输入后同样发射（不编译运行）。
//!
//! 方法体由 [`MethodBodies`]（`method` crate）生成。

use std::path::{Path, PathBuf};
use std::process::Command;

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use emit::audit::{append_raw_sites, audit_lines, AuditInputs};
use emit::ctx::{EmitCtx, EmitOptions, EmitShared};
use emit::method_bodies::{BodyAudit, MethodBodies};
use emit::perf::{report_lines, Perf};
use emit::precheck::DEFAULT_LIMIT;
use emit::project::{prepare_scratch, write_project, ProjectReport};
use input::{BuildInput, ClosureFacts, LibCrate, RuntimeManifest};
use resolve::{ClassPath, Hierarchy, Origin};
use ty::short_names::ShortNames;

use crate::api_roots::api_roots;
use crate::build_libs::{self, Libs};
use crate::build_opts::{BuildOpts, Mode, Stage, CLOSURE_INPUT_DIR};
use crate::cargo;
use crate::compile_cmd::{compile_stage, CompileArgs};
use crate::status::{BuildStatus, EmitSummary, STATUS_FILE};
use crate::closure_cmd::{find_runtime_dir, seed_roots, MAIN};
use crate::Args;

use resolve::jdk::release_major as jdk_major;

pub(crate) fn abs(p: &Path) -> PathBuf {
    std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf())
}

/// 仓库根：`<repo>/runtime/java_runtime` 上两级
pub(crate) fn repo_root(rt: &Path) -> PathBuf {
    rt.parent().and_then(Path::parent).map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn remove_dir(d: &Path) -> Result<(), String> {
    if d.is_dir() {
        std::fs::remove_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    Ok(())
}

/// javac 编译到独占目录（参数：`-g`，JDK ≥ 14 时 `--enable-preview --release N`；
/// jar 输入模式下全部 jar 上 `-cp`）
pub(crate) fn javac(home: &Path, java_files: &[PathBuf], jars: &[PathBuf], out: &Path) -> Result<(), String> {
    remove_dir(out)?;
    std::fs::create_dir_all(out).map_err(|e| format!("{}：{e}", out.display()))?;
    let mut cmd = Command::new(home.join("bin/javac"));
    cmd.arg("-g");
    if let Some(m) = jdk_major(home).filter(|m| *m >= 14) {
        cmd.args(["--enable-preview", "--release", &m.to_string()]);
    }
    if !jars.is_empty() {
        let cp: Vec<String> = jars.iter().map(|j| j.display().to_string()).collect();
        cmd.arg("-cp").arg(cp.join(":"));
    }
    let st = cmd.arg("-d").arg(out).args(java_files).status().map_err(|e| format!("javac：{e}"))?;
    if !st.success() {
        return Err(format!("javac 编译失败：{}", java_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(" ")));
    }
    Ok(())
}

/// 用户类目录 → 依赖库 jar → JDK → 镜像独有 / VM 支持类（同名先加入者优先，与 `rava closure` 一致）
pub(crate) fn class_path(user_dir: &Path, jars: &[PathBuf], home: &Path, images: &[PathBuf]) -> Result<ClassPath, String> {
    let mut cp = ClassPath::new();
    cp.add(Origin::User, user_dir).map_err(|e| format!("{}：{e}", user_dir.display()))?;
    for j in jars {
        cp.add(Origin::Lib, j).map_err(|e| format!("{}：{e}", j.display()))?;
    }
    cp.add_jdk(home).map_err(|e| e.to_string())?;
    for d in images {
        cp.add(Origin::Image, d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    Ok(cp)
}

/// `--image` 缺省：由 JDK 镜像与 `runtime/java_support` 派生（[`resolve::image`]）；显式给出则原样使用
pub(crate) fn image_dirs(o: &BuildOpts, home: &Path, rt: &Path) -> Vec<PathBuf> {
    if !o.images.is_empty() {
        return o.images.clone();
    }
    resolve::image::image_class_dirs(home, &support_root(rt))
}

/// VM 支持类源码根：与手写运行时同级的 `java_support/`
pub fn support_root(rt: &Path) -> PathBuf {
    rt.parent().unwrap_or(rt).join("java_support")
}

fn simple(n: &str) -> &str {
    n.rsplit('/').next().unwrap_or(n)
}

fn has_main(cp: &ClassPath, n: &str) -> bool {
    cp.get(n).is_some_and(|c| c.method(MAIN.0, MAIN.1).is_some_and(|m| m.is_static()))
}

/// 用户类序：逐源文件取同名类，再取 SourceFile 同源的其余类（按 `<简单名>.class`
/// 文件名序）；无源文件对应的类殿后。入口类（`--main` 或首个带 static main 的类）移到最前
pub fn user_order(cp: &ClassPath, java_files: &[PathBuf], main: Option<&str>) -> Result<Vec<String>, String> {
    let names = cp.names_of(Origin::User);
    let file_key = |n: &String| format!("{}.class", simple(n));
    let src_of = |n: &str| cp.get(n).and_then(|c| c.source_file.clone());
    let mut out: Vec<String> = Vec::new();
    for jf in java_files {
        let file = jf.file_name().and_then(|s| s.to_str()).unwrap_or_default();
        let stem = jf.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        let of_file: Vec<&String> = names.iter().filter(|n| src_of(n).as_deref() == Some(file) && !out.contains(n)).collect();
        if let Some(n) = of_file.iter().find(|n| simple(n) == stem) {
            out.push((*n).clone());
        }
        let mut rest: Vec<&String> = of_file.into_iter().filter(|n| !out.contains(n)).collect();
        rest.sort_by_key(|n| file_key(n));
        out.extend(rest.into_iter().cloned());
    }
    let mut left: Vec<&String> = names.iter().filter(|n| !out.contains(n)).collect();
    left.sort_by_key(|n| file_key(n));
    out.extend(left.into_iter().cloned());
    let main = match main {
        Some(m) => m.to_string(),
        None => out.iter().find(|n| has_main(cp, n)).cloned().ok_or("用户类中没有 static main(String[])")?,
    };
    let i = out.iter().position(|n| *n == main).ok_or_else(|| format!("入口类不在用户类中：{main}"))?;
    let m = out.remove(i);
    out.insert(0, m);
    Ok(out)
}

/// 闭包分析（同 `rava closure`；lib 种子类同 `--seed-class`；`--closure-cache` 时先查跨运行结果缓存，命中即由缓存的
/// closure.json 值经 [`ClosureFacts::from_json`] 得事实）。`--closure-json` 时向 `json_path` 写出 closure.json；
/// 冷算且持有产物值时校验两条路径（进程内 [`ClosureFacts::from_closure`] 与 closure.json 经 [`ClosureFacts::from_json`]）
/// 产出的事实逐字节一致（`Debug` 文本）；否则删除该处上轮遗留的 closure.json，免得与本轮不符。
/// `--trace-class` 打印 provenance 链（`[why]`），`--debug` 列未解析调用。
///
/// 事实交给 `then`（发射）在作用域线程里执行，本线程同时析构闭包结构（数百万个小分配，
/// Digester 约 60 ms、DeepCopy 数百 ms）：闭包借用非 `Sync` 的手写层，只能在创建它的线程析构，
/// 所以挪走的是发射。发射线程栈同发射工作线程（方法体生成有深递归）
#[allow(clippy::too_many_arguments)]
pub(crate) fn analyze<R: Send>(
    cp: &ClassPath,
    rt: &Path,
    main: &str,
    o: &BuildOpts,
    seed_classes: &[String],
    json_path: &Path,
    perf: &mut Perf,
    then: impl FnOnce(&ClosureFacts, &mut Perf) -> Result<R, String> + Send,
) -> Result<R, String> {
    let man = Manifest::load(rt)?;
    let hw = Handwritten::new(rt);
    let h = Hierarchy::new(cp);
    let roots: Vec<&String> = o.roots.iter().collect();
    let seeds: Vec<&String> = seed_classes.iter().collect();
    let mut seed_members = seed_roots(cp, &roots, &seeds)?;
    if !o.api_packages.is_empty() {
        let (api, n_cls) = api_roots(cp, &o.api_packages, o.api_recursive);
        println!(
            "[api] {}（{}子包）→ {n_cls} 个 public 类，{} 个入口方法",
            o.api_packages.join(", "),
            if o.api_recursive { "含" } else { "不含" },
            api.len()
        );
        seed_members.extend(api);
    }
    let input = closure::Input {
        cp,
        runtime_dir: rt,
        roots: vec![MemberRef { owner: main.to_string(), name: MAIN.0.into(), desc: MAIN.1.into() }],
        seed_roots: seed_members,
        locales: o.locales.clone(),
        diag: crate::closure_cmd::diag_opts(&o.cuts, &o.cut_files, o.dump_edges.clone())?,
        cold_cut: false,
        flow_batch: None,
    };
    // 跨运行闭包缓存缺省落仓库 build/closure_cache（键覆盖分析器、JDK、手写层、用户类与全部分析参数）
    let dir = o.closure_cache.clone().unwrap_or_else(|| repo_root(rt).join("build").join("closure_cache"));
    let cache = crate::closure_run::CacheOpts { dir: Some(dir), max_mb: o.closure_cache_max_mb };
    let out = crate::closure_run::analyze(&cache, &input, &h, &man, &hw, o.trace_class.is_some(), o.closure_json);
    if let (Some(t), Some(c)) = (&o.trace_class, &out.closure) {
        for line in c.why(&t.replace('.', "/")).into_iter().chain([String::new()]) {
            println!("{}", if line.is_empty() { line } else { format!("      [why] {line}") });
        }
    }
    perf.mark("closure");
    let p = json_path;
    let facts = match (&out.closure, &out.json) {
        (Some(c), _) => ClosureFacts::from_closure(c),
        (None, Some(v)) => ClosureFacts::from_json(v).map_err(|e| format!("闭包缓存条目：{e}"))?,
        (None, None) => return Err("闭包产物缺失".into()),
    };
    perf.mark("closure_facts");
    // 冷算且产物值在手（启用缓存或 --closure-json）：校验 closure.json 路径与进程内直传的事实一致——
    // 缓存命中走的正是 from_json，这一校验保证命中与冷算交给发射的事实相同
    if let (Some(_), Some(v)) = (&out.closure, &out.json) {
        let parsed = ClosureFacts::from_json(v).map_err(|e| format!("closure.json：{e}"))?;
        if format!("{parsed:#?}") != format!("{facts:#?}") {
            return Err("由 closure.json 解析的闭包事实与进程内直传的不一致".into());
        }
        perf.mark("closure_json");
    }
    if let (true, Some(v)) = (o.closure_json, &out.json) {
        let s = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
        std::fs::write(p, s).map_err(|e| format!("{}：{e}", p.display()))?;
    } else if p.exists() {
        std::fs::remove_file(p).map_err(|e| format!("{}：{e}", p.display()))?;
    }
    if o.debug {
        for u in &facts.unresolved {
            println!("[closure] unresolved: {u}");
        }
    }
    std::thread::scope(|s| {
        let t = std::thread::Builder::new()
            .stack_size(emit::par::WORKER_STACK)
            .spawn_scoped(s, || then(&facts, perf))
            .map_err(|e| format!("创建发射线程：{e}"))?;
        drop(out);
        match t.join() {
            Ok(r) => r,
            Err(p) => std::panic::resume_unwind(p),
        }
    })
}

/// 一次发射所需的全部输入
pub(crate) struct EmitJob<'a> {
    pub cp: &'a ClassPath,
    pub facts: &'a ClosureFacts,
    pub rt: &'a Path,
    pub user: &'a [String],
    pub java_files: Vec<PathBuf>,
    pub home: &'a Path,
    pub out: &'a Path,
    pub libs: &'a [LibCrate],
    pub o: &'a BuildOpts,
}

/// `--perf` 报告的 Top-N 条数
const PERF_TOP: usize = 15;

/// 闭包事实 → [`input::EmitInput`] → 短名表 → [`EmitCtx`]，交给 `f`（落盘发射与 `rava audit` 的内存发射共用）
pub(crate) fn with_emit_ctx<R>(
    j: &EmitJob<'_>,
    perf: &mut Perf,
    f: impl FnOnce(&EmitCtx<'_>, &ShortNames, &mut Perf) -> Result<R, String>,
) -> Result<R, String> {
    let manifest = RuntimeManifest::load(j.rt).map_err(|e| e.to_string())?;
    let runtime_src = j.rt.join("src");
    let inp = BuildInput { cp: j.cp, facts: j.facts, manifest: &manifest, user_classes: j.user, libs: j.libs, runtime_src: &runtime_src, jobs: j.o.emit_jobs }
        .build()
        .map_err(|e| format!("构建发射层输入：{e}"))?;
    for &(name, d) in &inp.timings {
        perf.mark_sub(name, d);
    }
    perf.mark("input");
    let names = ShortNames::build(&inp.registry);
    let opts = EmitOptions {
        strict: j.o.strict,
        jdk_major: jdk_major(j.home),
        java_files: j.java_files.clone(),
        batch: j.o.batch,
        debug: j.o.debug,
        jobs: j.o.emit_jobs,
    };
    let shared = EmitShared::new(&inp, &names, &manifest, j.cp, j.rt, opts).map_err(|e| e.to_string())?;
    perf.mark("names+ctx");
    f(&shared.view(), &names, perf)
}

/// EmitInput → overlay → 写 scratch → 预检 →（非 `--full-precheck`）审计行；另返回逐方法耗时（`--perf`）
fn emit_scratch(j: &EmitJob<'_>, perf: &mut Perf) -> Result<(ProjectReport, Vec<(String, std::time::Duration)>), String> {
    with_emit_ctx(j, perf, |ctx, names, perf| write_scratch(j, ctx, names, perf))
}

fn write_scratch(
    j: &EmitJob<'_>,
    ctx: &EmitCtx<'_>,
    names: &ShortNames,
    perf: &mut Perf,
) -> Result<(ProjectReport, Vec<(String, std::time::Duration)>), String> {
    prepare_scratch(j.out, j.rt, &ctx.macros_crate, false).map_err(|e| format!("overlay：{e}"))?;
    perf.mark("overlay");
    ir::raw_audit::reset();
    if j.o.raw_sites.is_some() {
        ir::raw_audit::enable_sites();
    }
    let bodies = MethodBodies::new(ctx);
    perf.mark("body_facts");
    let mut r = write_project(ctx, j.out, &bodies).map_err(|e| format!("发射：{e}"))?;
    perf.absorb(std::mem::take(&mut r.perf));
    let limit = if j.o.full_precheck { usize::MAX } else { DEFAULT_LIMIT };
    for line in r.precheck.lines(limit) {
        println!("{line}");
    }
    if !j.o.full_precheck {
        let crates: Vec<String> =
            std::iter::once("java_runtime".to_string()).chain(j.libs.iter().map(|l| l.name.clone())).chain(["user".to_string()]).collect();
        let body = BodyAudit::from_log(&r.body_log);
        let a = AuditInputs {
            body: &body,
            hw: &r.hw_audit,
            fallback: &ctx.fallback,
            out: j.out,
            crates: &crates,
            prelude_disambiguated: names.prelude_disambiguated(),
            debug: j.o.debug,
        };
        for line in audit_lines(&a) {
            println!("{line}");
        }
    }
    if let Some(p) = &j.o.raw_sites {
        append_raw_sites(p).map_err(|e| format!("{}：{e}", p.display()))?;
    }
    perf.mark("audit");
    let timings = std::mem::take(&mut r.body_log.timings);
    Ok((r, timings))
}

fn report(r: &ProjectReport, out: &Path) {
    println!("[emit] {} JDK 类 + {} 用户类 → {}（bin {}）", r.jdk_classes, r.user_classes, out.display(), r.bin_name);
}

/// `--perf` 报告；已发射时（`jdk_classes` 为 Some）补一行重型判定（与编译阶段 `Heavy::decide` 同一函数）
fn print_perf(on: bool, perf: &Perf, methods: &[(String, std::time::Duration)], jdk_classes: Option<usize>) {
    if !on {
        return;
    }
    for l in report_lines(perf, methods, PERF_TOP) {
        println!("{l}");
    }
    if let Some(n) = jdk_classes {
        let h = cargo::Heavy::decide(n);
        let verdict = h.jobs.map_or_else(|| "不干预作业数".to_string(), |j| format!("强制 {j} 作业"));
        println!("[perf] 重型判定：{} {n} 类（阈值 {}）→ {verdict}", cargo::PEAK_CRATE, cargo::HEAVY_CLASSES);
    }
}

pub fn run_build(args: &Args) -> Result<(), String> {
    let o = BuildOpts::parse(Mode::Build, &args.rest)?;
    let rt = abs(&find_runtime_dir(o.runtime.clone())?);
    let repo = repo_root(&rt);
    let out = abs(&o.scratch_dir(Mode::Build, &repo)?);
    if o.clean {
        remove_dir(&out)?;
    }
    // 上一轮的状态 / 产物清单不得冒充本轮
    for f in [STATUS_FILE, cargo::ARTIFACTS_FILE] {
        let p = out.join(f);
        if p.exists() {
            std::fs::remove_file(&p).map_err(|e| format!("{}：{e}", p.display()))?;
        }
    }
    let mut st = BuildStatus::default();
    let r = build_stages(&o, &rt, &repo, &out, &mut st);
    st.write(&out, r.as_ref().err())?;
    r
}

/// build 各阶段（javac → 闭包 → 发射 → 编译 → 运行），到 `--stop-after` 为止；进度与失败现场记入 `st`
fn build_stages(o: &BuildOpts, rt: &Path, repo: &Path, out: &Path, st: &mut BuildStatus) -> Result<(), String> {
    let mut perf = Perf::new();
    st.stage = Stage::Javac;
    let jdk = resolve::jdk::choose(o.jdk, o.java_home.as_deref(), Some(repo))?;
    println!("{}", jdk.describe());
    st.jdk = Some(jdk.clone());
    let home = jdk.home;
    let cin = out.join(CLOSURE_INPUT_DIR);
    let classes = cin.join("classes");
    let Libs { crates, seed_classes, jars } = build_libs::load(&o.libs)?;
    javac(&home, &o.inputs, &jars, &classes)?;
    perf.mark("javac");
    if o.stop_after == Stage::Javac {
        return Ok(());
    }
    st.stage = Stage::Closure;
    let cp = class_path(&classes, &jars, &home, &image_dirs(o, &home, rt))?;
    let user = user_order(&cp, o.java_files(Mode::Build), o.main.as_deref())?;
    perf.mark("classpath");
    let java_files: Vec<PathBuf> = o.java_files(Mode::Build).iter().map(|p| abs(p)).collect();
    let emit_too = o.stop_after >= Stage::Emit;
    let emitted = analyze(&cp, rt, &user[0], o, &seed_classes, &cin.join("closure.json"), &mut perf, |facts, perf| {
        if !emit_too {
            return Ok(None);
        }
        let job = EmitJob { cp: &cp, facts, rt, user: &user, java_files, home: &home, out, libs: &crates, o };
        emit_scratch(&job, perf).map(Some)
    })?;
    let Some((r, timings)) = emitted else {
        print_perf(o.perf, &perf, &[], None);
        return Ok(());
    };
    st.stage = Stage::Emit;
    print_perf(o.perf, &perf, &timings, Some(r.jdk_classes));
    let emit = EmitSummary { bin: r.bin_name.clone(), jdk_classes: r.jdk_classes, precheck: r.precheck.clone() };
    st.emit = Some(emit.clone());
    if o.full_precheck {
        return Ok(());
    }
    report(&r, out);
    if o.stop_after == Stage::Emit {
        return Ok(());
    }
    let c = CompileArgs {
        release: o.release,
        target_dir: o.target_dir.clone(),
        build_timeout: o.build_timeout,
        keep_artifacts: o.keep_artifacts,
    };
    let exe = compile_stage(out, repo, &emit, &c, st)?;
    if o.stop_after == Stage::Compile {
        return Ok(());
    }
    st.stage = Stage::Run;
    let ran = cargo::run(&exe);
    if !o.keep_artifacts {
        crate::artifacts::prune_scratch(out, false)?;
    }
    ran
}

pub fn run_emit(args: &Args) -> Result<(), String> {
    let o = BuildOpts::parse(Mode::Emit, &args.rest)?;
    let mut perf = Perf::new();
    let rt = abs(&find_runtime_dir(o.runtime.clone())?);
    let home = resolve::jdk::choose(o.jdk, o.java_home.as_deref(), Some(&repo_root(&rt)))?.home;
    let out = abs(&o.scratch_dir(Mode::Emit, &repo_root(&rt))?);
    let cj = abs(&o.inputs[0]);
    let classes = abs(&o.emit_classes_dir());
    let text = std::fs::read_to_string(&cj).map_err(|e| format!("{}：{e}", cj.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{}：{e}", cj.display()))?;
    let facts = ClosureFacts::from_json(&v).map_err(|e| format!("{}：{e}", cj.display()))?;
    perf.mark("closure_json_load");
    if o.clean {
        if cj.starts_with(&out) || classes.starts_with(&out) {
            return Err("closure.json / 用户类目录位于 scratch 内：--clean 会删除输入".into());
        }
        remove_dir(&out)?;
    }
    let cp = class_path(&classes, &[], &home, &image_dirs(&o, &home, &rt))?;
    let user = user_order(&cp, o.java_files(Mode::Emit), None)?;
    perf.mark("classpath");
    let java_files = o.java_files(Mode::Emit).iter().map(|p| abs(p)).collect();
    let job = EmitJob { cp: &cp, facts: &facts, rt: &rt, user: &user, java_files, home: &home, out: &out, libs: &[], o: &o };
    let (r, timings) = emit_scratch(&job, &mut perf)?;
    if !o.full_precheck {
        report(&r, &out);
    }
    print_perf(o.perf, &perf, &timings, Some(r.jdk_classes));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(p: &Path, s: &str) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, s).unwrap();
    }

    /// build 的 scratch 次序：`--clean` 先整体删除，再 javac 进 closure_input/，overlay 不得触及它；
    /// overlay 复制手写真源、改写宏依赖路径，剪除 runtime/ 已删除的无生成标记文件
    #[test]
    fn overlay_keeps_closure_input_and_applies_runtime() {
        let root = std::env::temp_dir().join(format!("rava-driver-overlay-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let rt = root.join("runtime/java_runtime");
        put(&rt.join("src/lib.rs"), "pub mod java;\n");
        put(&rt.join("build.rs"), "fn main() {}\n");
        put(&rt.join("Cargo.toml"), "[package]\nversion = \"0.1.0\"\n[dependencies]\nrava_macros = { path = \"../rava_macros\" }\n");
        put(&root.join("runtime/java_meta/Cargo.toml"), "[package]\nversion = \"0.1.0\"\n");
        let out = root.join("scratch");
        put(&out.join("stale.txt"), "x");
        remove_dir(&out).unwrap();
        assert!(!out.exists());
        put(&out.join(CLOSURE_INPUT_DIR).join("classes/A.class"), "cafebabe");
        let macros = root.join("runtime/rava_macros");
        prepare_scratch(&out, &rt, &macros, false).unwrap();
        assert!(out.join(CLOSURE_INPUT_DIR).join("classes/A.class").is_file());
        assert!(!out.join("java_runtime/src/lib.rs").exists(), "lib.rs 由 mod 树阶段写出");
        let cargo = std::fs::read_to_string(out.join("java_runtime/Cargo.toml")).unwrap();
        assert!(cargo.contains(&format!("path = \"{}\"", macros.display())));
        assert!(!cargo.contains("version = \"0.1.0\""));
        assert!(out.join("java_runtime/src/java/mod.rs").is_file());
        assert!(out.join("java_meta/Cargo.toml").is_file());
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn repo_root_and_jdk_major() {
        assert_eq!(repo_root(Path::new("/r/runtime/java_runtime")), PathBuf::from("/r"));
        let d = std::env::temp_dir().join(format!("rava-driver-release-{}", std::process::id()));
        put(&d.join("release"), "IMPLEMENTOR=\"x\"\nJAVA_VERSION=\"21.0.2\"\n");
        assert_eq!(jdk_major(&d), Some(21));
        std::fs::remove_dir_all(&d).unwrap();
    }
}
