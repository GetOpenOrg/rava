//! `rava build` / `rava emit`：P0 转译外壳（计划 docs/plans/2026-09-20-rust-generator-rewrite.md P0 / P5a）。
//!
//! build：javac → 闭包分析（同 `rava closure`，closure.json 落 `<scratch>/closure_input/`）→
//! [`EmitInput`](input::EmitInput) → overlay → 发射层写 scratch →（缺省）`cargo run`。
//! emit：从既有 closure.json + 用户类目录重建输入后同样发射（不编译运行）。
//!
//! 方法体由 [`MethodBodies`]（`method` crate）生成。

use std::path::{Path, PathBuf};
use std::process::Command;

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use emit::audit::audit_lines;
use emit::method_bodies::MethodBodies;
use emit::ctx::{EmitCtx, EmitOptions};
use emit::project::{prepare_scratch, write_project, ProjectReport};
use input::{BuildInput, ClosureFacts, RuntimeManifest};
use resolve::{ClassPath, Hierarchy, Origin};
use ty::short_names::ShortNames;

use crate::build_opts::{BuildOpts, Mode, CLOSURE_INPUT_DIR};
use crate::closure_cmd::{find_runtime_dir, seed_roots, MAIN};
use crate::Args;

fn java_home(o: &BuildOpts) -> Result<PathBuf, String> {
    if let Some(h) = &o.java_home {
        return Ok(h.clone());
    }
    resolve::jdk::find_java_home(o.jdk).ok_or_else(|| "找不到含 jmods/ 的 JDK".to_string())
}

/// `<java_home>/release` 的 JAVA_VERSION 主版本
fn jdk_major(home: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(home.join("release")).ok()?;
    let v = text.lines().find_map(|l| l.strip_prefix("JAVA_VERSION="))?.trim_matches('"');
    v.split('.').next()?.parse().ok()
}

fn abs(p: &Path) -> PathBuf {
    std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf())
}

/// 仓库根：`<repo>/runtime/java_runtime` 上两级
fn repo_root(rt: &Path) -> PathBuf {
    rt.parent().and_then(Path::parent).map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn remove_dir(d: &Path) -> Result<(), String> {
    if d.is_dir() {
        std::fs::remove_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    Ok(())
}

/// javac 编译到独占目录（与 main.py 同参：`-g`，JDK ≥ 14 时 `--enable-preview --release N`）
fn javac(home: &Path, java_files: &[PathBuf], out: &Path) -> Result<(), String> {
    remove_dir(out)?;
    std::fs::create_dir_all(out).map_err(|e| format!("{}：{e}", out.display()))?;
    let mut cmd = Command::new(home.join("bin/javac"));
    cmd.arg("-g");
    if let Some(m) = jdk_major(home).filter(|m| *m >= 14) {
        cmd.args(["--enable-preview", "--release", &m.to_string()]);
    }
    let st = cmd.arg("-d").arg(out).args(java_files).status().map_err(|e| format!("javac：{e}"))?;
    if !st.success() {
        return Err(format!("javac 编译失败：{}", java_files.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(" ")));
    }
    Ok(())
}

/// 用户类目录 → JDK → 镜像独有 / VM 支持类（同名先加入者优先，与 `rava closure` 一致）
fn class_path(user_dir: &Path, home: &Path, images: &[PathBuf]) -> Result<ClassPath, String> {
    let mut cp = ClassPath::new();
    cp.add(Origin::User, user_dir).map_err(|e| format!("{}：{e}", user_dir.display()))?;
    cp.add_jdk(home).map_err(|e| e.to_string())?;
    for d in images {
        cp.add(Origin::Image, d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    Ok(cp)
}

fn simple(n: &str) -> &str {
    n.rsplit('/').next().unwrap_or(n)
}

fn has_main(cp: &ClassPath, n: &str) -> bool {
    cp.get(n).is_some_and(|c| c.method(MAIN.0, MAIN.1).is_some_and(|m| m.is_static()))
}

/// 用户类序（与 main.py 同）：逐源文件取同名类，再取 SourceFile 同源的其余类（按 `<简单名>.class`
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

/// 闭包分析（同 `rava closure`）；closure.json 写入 `json_path`
fn analyze(cp: &ClassPath, rt: &Path, main: &str, o: &BuildOpts, json_path: &Path) -> Result<ClosureFacts, String> {
    let man = Manifest::load(rt)?;
    let hw = Handwritten::new(rt);
    let h = Hierarchy::new(cp);
    let roots: Vec<&String> = o.roots.iter().collect();
    let input = closure::Input {
        cp,
        runtime_dir: rt,
        roots: vec![MemberRef { owner: main.to_string(), name: MAIN.0.into(), desc: MAIN.1.into() }],
        seed_roots: seed_roots(cp, &roots, &[])?,
        locales: o.locales.clone(),
    };
    let c = closure::analyze(&input, &h, &man, &hw);
    for e in hw.errors.borrow().iter() {
        eprintln!("[closure] 手写文件解析失败：{e}");
    }
    let s = serde_json::to_string_pretty(&c.to_json()).map_err(|e| e.to_string())?;
    std::fs::write(json_path, s).map_err(|e| format!("{}：{e}", json_path.display()))?;
    Ok(ClosureFacts::from_closure(&c))
}

/// 一次发射所需的全部输入
struct EmitJob<'a> {
    cp: &'a ClassPath,
    facts: &'a ClosureFacts,
    rt: &'a Path,
    user: &'a [String],
    java_files: Vec<PathBuf>,
    home: &'a Path,
    out: &'a Path,
    strict: bool,
}

/// EmitInput → overlay → 写 scratch
fn emit_scratch(j: &EmitJob<'_>) -> Result<ProjectReport, String> {
    let manifest = RuntimeManifest::load(j.rt).map_err(|e| e.to_string())?;
    let runtime_src = j.rt.join("src");
    let inp = BuildInput { cp: j.cp, facts: j.facts, manifest: &manifest, user_classes: j.user, libs: &[], runtime_src: &runtime_src }
        .build()
        .map_err(|e| format!("构建发射层输入：{e}"))?;
    let names = ShortNames::build(&inp.registry);
    let opts = EmitOptions { strict: j.strict, jdk_major: jdk_major(j.home), java_files: j.java_files.clone() };
    let ctx = EmitCtx::new(&inp, &names, &manifest, j.cp, j.rt, opts).map_err(|e| e.to_string())?;
    prepare_scratch(j.out, j.rt, &ctx.macros_crate, false).map_err(|e| format!("overlay：{e}"))?;
    let mut bodies = MethodBodies::new(&ctx);
    let r = write_project(&ctx, j.out, &mut bodies).map_err(|e| format!("发射：{e}"))?;
    for line in audit_lines(&bodies.audit, &r.hw_audit) {
        println!("{line}");
    }
    Ok(r)
}

fn report(r: &ProjectReport, out: &Path) {
    println!("[emit] {} JDK 类 + {} 用户类 → {}（bin {}）", r.jdk_classes, r.user_classes, out.display(), r.bin_name);
}

/// 与 main.py 同一 cargo 流程：共享 `build/target`、关闭增量
fn cargo_run(out: &Path, bin: &str, repo: &Path) -> Result<(), String> {
    println!("\n[run] cargo run --bin {bin}");
    let st = Command::new("cargo")
        .args(["run", "--bin", bin])
        .current_dir(out)
        .env("CARGO_TARGET_DIR", repo.join("build").join("target"))
        .env("CARGO_INCREMENTAL", "0")
        .status()
        .map_err(|e| format!("cargo：{e}"))?;
    if !st.success() {
        return Err(format!("cargo run 失败（{st}）"));
    }
    Ok(())
}

pub fn run_build(args: &Args) -> Result<(), String> {
    let o = BuildOpts::parse(Mode::Build, &args.rest)?;
    let home = java_home(&o)?;
    let rt = abs(&find_runtime_dir(o.runtime.clone())?);
    let repo = repo_root(&rt);
    let out = abs(&o.scratch_dir(Mode::Build, &repo)?);
    if o.clean {
        remove_dir(&out)?;
    }
    let cin = out.join(CLOSURE_INPUT_DIR);
    let classes = cin.join("classes");
    javac(&home, &o.inputs, &classes)?;
    let cp = class_path(&classes, &home, &o.images)?;
    let user = user_order(&cp, o.java_files(Mode::Build), o.main.as_deref())?;
    let facts = analyze(&cp, &rt, &user[0], &o, &cin.join("closure.json"))?;
    let java_files = o.java_files(Mode::Build).iter().map(|p| abs(p)).collect();
    let job = EmitJob { cp: &cp, facts: &facts, rt: &rt, user: &user, java_files, home: &home, out: &out, strict: o.strict };
    let r = emit_scratch(&job)?;
    report(&r, &out);
    if o.no_run {
        return Ok(());
    }
    cargo_run(&out, &r.bin_name, &repo)
}

pub fn run_emit(args: &Args) -> Result<(), String> {
    let o = BuildOpts::parse(Mode::Emit, &args.rest)?;
    let home = java_home(&o)?;
    let rt = abs(&find_runtime_dir(o.runtime.clone())?);
    let out = abs(&o.scratch_dir(Mode::Emit, &repo_root(&rt))?);
    let cj = abs(&o.inputs[0]);
    let classes = abs(&o.emit_classes_dir());
    let text = std::fs::read_to_string(&cj).map_err(|e| format!("{}：{e}", cj.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{}：{e}", cj.display()))?;
    let facts = ClosureFacts::from_json(&v).map_err(|e| format!("{}：{e}", cj.display()))?;
    if o.clean {
        if cj.starts_with(&out) || classes.starts_with(&out) {
            return Err("closure.json / 用户类目录位于 scratch 内：--clean 会删除输入".into());
        }
        remove_dir(&out)?;
    }
    let cp = class_path(&classes, &home, &o.images)?;
    let user = user_order(&cp, o.java_files(Mode::Emit), None)?;
    let java_files = o.java_files(Mode::Emit).iter().map(|p| abs(p)).collect();
    let job = EmitJob { cp: &cp, facts: &facts, rt: &rt, user: &user, java_files, home: &home, out: &out, strict: o.strict };
    let r = emit_scratch(&job)?;
    report(&r, &out);
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
        let out = root.join("scratch");
        put(&out.join("stale.txt"), "x");
        remove_dir(&out).unwrap();
        assert!(!out.exists());
        put(&out.join(CLOSURE_INPUT_DIR).join("classes/A.class"), "cafebabe");
        put(&out.join("java_runtime/src/gone_impl.rs"), "impl X {}\n");
        let macros = root.join("runtime/rava_macros");
        prepare_scratch(&out, &rt, &macros, false).unwrap();
        assert!(out.join(CLOSURE_INPUT_DIR).join("classes/A.class").is_file());
        assert_eq!(std::fs::read_to_string(out.join("java_runtime/src/lib.rs")).unwrap(), "pub mod java;\n");
        assert!(!out.join("java_runtime/src/gone_impl.rs").exists());
        let cargo = std::fs::read_to_string(out.join("java_runtime/Cargo.toml")).unwrap();
        assert!(cargo.contains(&format!("path = \"{}\"", macros.display())));
        assert!(!cargo.contains("version = \"0.1.0\""));
        assert!(out.join("java_runtime/src/java/mod.rs").is_file());
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
