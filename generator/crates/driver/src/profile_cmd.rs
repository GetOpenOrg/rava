//! `rava profile`：多根开放世界分析，产出档案闭包与档案键（T1 档案化，计划
//! `docs/plans/2026-10-01-cross-test-compile-reuse.md` §5.3 第 1 步；并集语义见 [`closure::profile`]）。
//!
//! ```text
//! rava profile [<A.java | 类目录>]… [--entries <清单>] [--closure <closure.json>]…
//!              [--deps deps.lock.toml] [--jdk N | --java-home P] [--runtime R] [--image D]… [-o profile.json] [--entry-out DIR]
//!              [--closure-cache D] [--closure-cache-max-mb N] [--flow-batch N] [--hash-seed N]
//! rava profile --covers <profile.json> <closure.json>… [--jdk N | --java-home P] [--runtime R] [--image D]…
//! ```
//!
//! 入口三种来源，可混用：
//! - 位置参数：每个 `.java` 文件或类目录各成一个入口（名 = 文件主名 / 目录名）；
//! - `--entries <清单>`：每行一个入口，`#` 起注释；行内为一个或多个 `.java` / 类目录，另可带
//!   `--name N`、`--main 类`、`--cp 锁条目名[,…]`、`--root 类.方法:描述符`、`--seed-class 类`、`--locale L`；
//!   相对路径按清单所在目录解析；
//! - `--closure <closure.json>`：已算好的单例闭包（分发流程中各测试的产物，名 = 文件主名去掉 `.closure`）。
//!
//! 每个入口在独立的用户命名空间里分析（各自 javac、类路径、引擎；可经 `--closure-cache` 命中），入口名须唯一。
//! `--entry-out DIR` 写出各入口的单例闭包 `<名>.closure.json`（第 1b 步按它生成用户 crate）。
//! `--covers`：检查档案是否覆盖给定单例闭包（§4.1 子集复用：并入后档案内容不变），逐个打印 covered / not-covered，
//! 有未覆盖者退出码非零。

use std::path::{Path, PathBuf};
use std::time::Instant;

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use closure::profile::{self, EntryClosure, KeyInputs};
use resolve::{ClassPath, Hierarchy, Origin};
use serde_json::Value;

use crate::build_libs::LibEntry;
use crate::closure_cmd::{seed_roots, MAIN};
use crate::deps_lock::DepsLock;
use crate::Args;

/// 生成器源码树摘要（`build.rs`）
const GENERATOR_DIGEST: &str = env!("RAVA_GENERATOR_DIGEST");

const VALUE_OPTS: &[&str] = &[
    "--deps", "--entries", "--launch", "--closure", "--jdk", "--java-home", "--runtime", "--image", "-o", "--entry-out", "--closure-cache",
    "--closure-cache-max-mb", "--flow-batch", "--hash-seed", "--covers",
];

/// 一个待分析的入口
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct EntrySpec {
    name: String,
    inputs: Vec<PathBuf>,
    main: Option<String>,
    /// 入口类路径：依赖锁条目名（实际顺序取锁序）
    cp: Vec<String>,
    roots: Vec<String>,
    seed_classes: Vec<String>,
    locales: Vec<String>,
}

impl EntrySpec {
    fn of_input(p: &Path) -> EntrySpec {
        EntrySpec { name: default_name(p), inputs: vec![p.to_path_buf()], ..EntrySpec::default() }
    }

    /// 入口输入摘要：源文件 / 类目录 / 依赖库内容 + 选项。`jars`：该入口类路径上的锁 jar（锁序）
    fn digest(&self, jars: &[PathBuf]) -> Result<String, String> {
        let mut parts: Vec<(String, PathBuf)> = self.inputs.iter().map(|p| (format!("input {}", base(p)), p.clone())).collect();
        for j in jars {
            parts.push((format!("cp {}", base(j)), j.clone()));
        }
        let opts = format!("main={:?} roots={:?} seeds={:?} locales={:?}", self.main, self.roots, self.seed_classes, self.locales);
        parts.push((opts, PathBuf::new()));
        profile::files_digest(&parts)
    }
}

fn base(p: &Path) -> String {
    p.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default()
}

fn default_name(p: &Path) -> String {
    if p.is_dir() {
        base(p)
    } else {
        p.file_stem().map(|f| f.to_string_lossy().to_string()).unwrap_or_default()
    }
}

/// 入口清单解析（相对路径按 `dir` 解析）
fn parse_entries(text: &str, dir: &Path) -> Result<Vec<EntrySpec>, String> {
    let mut out = Vec::new();
    for (ln, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut e = EntrySpec::default();
        let mut name = None;
        let mut it = line.split_whitespace();
        while let Some(t) = it.next() {
            let mut val = || it.next().map(String::from).ok_or_else(|| format!("入口清单第 {} 行：{t} 缺少值", ln + 1));
            match t {
                "--name" => name = Some(val()?),
                "--main" => e.main = Some(val()?.replace('.', "/")),
                "--cp" => {
                    let raw = val()?;
                    e.cp.extend(raw.split(',').map(str::trim).filter(|n| !n.is_empty()).map(String::from));
                }
                "--root" => e.roots.push(val()?),
                "--seed-class" => e.seed_classes.push(val()?),
                "--locale" => e.locales.push(val()?),
                t if t.starts_with('-') => return Err(format!("入口清单第 {} 行：未知选项 {t}", ln + 1)),
                t => e.inputs.push(dir.join(t)),
            }
        }
        let first = e.inputs.first().ok_or_else(|| format!("入口清单第 {} 行没有 .java / 类目录", ln + 1))?;
        e.name = name.unwrap_or_else(|| default_name(first));
        out.push(e);
    }
    Ok(out)
}

fn multi<'a>(args: &'a Args, flag: &str) -> Vec<&'a String> {
    args.rest.iter().zip(args.rest.iter().skip(1)).filter(|(a, _)| *a == flag).map(|(_, v)| v).collect()
}

fn positional(args: &Args) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    let mut it = args.rest.iter();
    while let Some(a) = it.next() {
        if VALUE_OPTS.contains(&a.as_str()) {
            it.next().ok_or_else(|| format!("{a} 缺少值"))?;
        } else if a.starts_with('-') {
            return Err(format!("未知参数：{a:?}"));
        } else {
            out.push(PathBuf::from(a));
        }
    }
    Ok(out)
}

/// 共享环境：JDK、runtime、镜像目录
struct Env {
    home: PathBuf,
    /// 依赖锁（`--deps`；库输入唯一来源）
    deps: Option<DepsLock>,
    /// 启动选项（`--launch`；记入 profile.entries[].launch，本步入库传递）
    launch: Option<String>,
    rt: PathBuf,
    images: Vec<PathBuf>,
    cache: crate::closure_run::CacheOpts,
    flow_batch: Option<usize>,
    work: PathBuf,
}

impl Env {
    /// 入口类路径条目（锁序）：`--cp` 条目名 → 锁条目 + 元数据
    fn lib_entries(&self, names: &[String]) -> Result<Vec<LibEntry>, String> {
        let Some(lock) = &self.deps else { return Ok(Vec::new()) };
        Ok(lock.select(names)?
            .iter()
            .map(|j| LibEntry {
                path: j.path.clone(),
                meta: resolve::classpath::LibMeta {
                    coordinate: j.coordinate.clone(),
                    module: j.module.clone(),
                    sha256: Some(j.sha256.clone()),
                },
            })
            .collect())
    }

    /// 非用户类路径（JDK + 镜像 + 给定依赖库）：异常表查询与归档摘要。
    /// 与 [`crate::build_cmd::class_path`] 同样做 JDK 包遮蔽与模块图硬校验
    fn jdk_path(&self, jars: &[PathBuf]) -> Result<ClassPath, String> {
        let release = resolve::jdk::major_of(&self.home).ok_or(format!("{}：无法识别 JDK 主版本", self.home.display()))?;
        let mut cp = ClassPath::new(release);
        for j in jars {
            cp.add(Origin::Lib, j).map_err(|e| format!("{}：{e}", j.display()))?;
        }
        cp.add_jdk(&self.home).map_err(|e| e.to_string())?;
        for d in &self.images {
            cp.add(Origin::Image, d).map_err(|e| format!("{}：{e}", d.display()))?;
        }
        cp.shadow_jdk_owned_packages();
        resolve::modules::check(&cp).map_err(|e| format!("[modules] {e}"))?;
        Ok(cp)
    }

    /// 一个入口的单例闭包（独立的 javac 输出、类路径、引擎）
    fn analyze(&self, e: &EntrySpec) -> Result<Value, String> {
        let libs = self.lib_entries(&e.cp)?;
        let jars: Vec<PathBuf> = libs.iter().map(|l| l.path.clone()).collect();
        let java: Vec<PathBuf> = e.inputs.iter().filter(|p| p.is_file()).cloned().collect();
        let dirs: Vec<&PathBuf> = e.inputs.iter().filter(|p| p.is_dir()).collect();
        let classes = match (java.is_empty(), dirs.as_slice()) {
            (true, [d]) => (*d).clone(),
            (false, []) => {
                let out = self.work.join(&e.name);
                crate::build_cmd::javac(&self.home, &java, &jars, &out)?;
                out
            }
            _ => return Err(format!("入口 {}：输入须为若干 .java 文件或恰一个类目录", e.name)),
        };
        let cp = crate::build_cmd::class_path(&classes, &libs, &self.home, &self.images, None)?;
        let main = crate::build_cmd::user_order(&cp, &java, e.main.as_deref())?.remove(0);
        let man = Manifest::load(&self.rt)?;
        let hw = Handwritten::new(&self.rt);
        let h = Hierarchy::new(&cp);
        let seeds: Vec<&String> = e.seed_classes.iter().collect();
        let input = closure::Input {
            cp: &cp,
            runtime_dir: &self.rt,
            roots: vec![MemberRef { owner: main, name: MAIN.0.into(), desc: MAIN.1.into() }],
            seed_roots: seed_roots(&cp, &e.roots.iter().collect::<Vec<_>>(), &seeds)?,
            locales: e.locales.clone(),
            diag: closure::engine::Diag::default(),
            cold_cut: false,
            flow_batch: self.flow_batch,
        };
        let out = crate::closure_run::analyze(&self.cache, &input, &h, &man, &hw, false, true).map_err(|f| format!("入口 {}：{f}", e.name))?;
        let v = out.json.ok_or_else(|| format!("入口 {}：闭包产物缺失", e.name))?;
        if !java.is_empty() {
            let _ = std::fs::remove_dir_all(&classes);
        }
        Ok(v)
    }
}

/// 异常处理器起点（`类.方法:描述符`）
fn handler_pcs(cp: &ClassPath, id: &str) -> Option<Vec<u32>> {
    let (owner, rest) = id.split_once('.')?;
    let (name, desc) = rest.split_once(':')?;
    let cf = cp.get(owner)?;
    let code = cf.method(name, desc)?.code.as_ref()?;
    Some(code.exception_table.iter().map(|e| e.handler as u32).collect())
}

fn read_json(p: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(p).map_err(|e| format!("{}：{e}", p.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}：{e}", p.display()))
}

fn closure_name(p: &Path) -> String {
    let stem = p.file_stem().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
    stem.strip_suffix(".closure").map(String::from).unwrap_or(stem)
}

fn write_json(p: &Path, v: &Value) -> Result<(), String> {
    let s = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
    std::fs::write(p, s).map_err(|e| format!("{}：{e}", p.display()))
}

fn env_of(args: &Args) -> Result<Env, String> {
    let num = |k: &str| args.opt(k).map(|v| v.parse::<u64>().map_err(|_| format!("{k} 需为非负整数：{v}"))).transpose();
    if let Some(s) = num("--hash-seed")? {
        closure::engine::set_hash_seed(s);
    }
    let home = crate::java_home(args)?;
    let rt = std::path::absolute(crate::closure_cmd::find_runtime_dir(args.opt("--runtime").map(PathBuf::from))?).map_err(|e| e.to_string())?;
    let images: Vec<PathBuf> = multi(args, "--image").into_iter().map(PathBuf::from).collect();
    let images = if images.is_empty() { resolve::image::image_class_dirs(&home, &crate::build_cmd::support_root(&rt)) } else { images };
    let cache = crate::closure_run::CacheOpts { dir: args.opt("--closure-cache").map(PathBuf::from), max_mb: num("--closure-cache-max-mb")? };
    let work = std::env::temp_dir().join(format!("rava-profile-{}", std::process::id()));
    let deps = args.opt("--deps").map(|d| DepsLock::load(Path::new(&d))).transpose()?;
    let launch = args.opt("--launch");
    Ok(Env { home, deps, launch, rt, images, cache, flow_batch: num("--flow-batch")?.map(|n| n as usize), work })
}

pub fn run(args: &Args) -> Result<(), String> {
    let pos = positional(args)?;
    let env = env_of(args)?;
    if let Some(p) = args.opt("--covers") {
        return covers(&env, Path::new(&p), &pos);
    }
    let t0 = Instant::now();
    let mut specs: Vec<EntrySpec> = pos.iter().map(|p| EntrySpec::of_input(p)).collect();
    for f in multi(args, "--entries") {
        let f = Path::new(f);
        let text = std::fs::read_to_string(f).map_err(|e| format!("{}：{e}", f.display()))?;
        specs.extend(parse_entries(&text, f.parent().unwrap_or(Path::new(".")))?);
    }
    let closure_files: Vec<PathBuf> = multi(args, "--closure").into_iter().map(PathBuf::from).collect();
    if specs.is_empty() && closure_files.is_empty() {
        return Err("没有入口（.java / 类目录 / --entries / --closure）".into());
    }
    let entry_out = args.opt("--entry-out").map(PathBuf::from);
    if let Some(d) = &entry_out {
        std::fs::create_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    let mut entries: Vec<EntryClosure> = Vec::new();
    let mut digests: Vec<(String, String)> = Vec::new();
    let mut lib_jars: Vec<PathBuf> = Vec::new();
    for s in &specs {
        let t = Instant::now();
        let v = env.analyze(s)?;
        eprintln!("[profile] {}：{} ms", s.name, t.elapsed().as_millis());
        if let Some(d) = &entry_out {
            write_json(&d.join(format!("{}.closure.json", s.name)), &v)?;
        }
        let entry_jars: Vec<PathBuf> = env.lib_entries(&s.cp)?.iter().map(|l| l.path.clone()).collect();
        digests.push((s.name.clone(), s.digest(&entry_jars)?));
        lib_jars.extend(entry_jars);
        entries.push(EntryClosure { name: s.name.clone(), closure: v, cp: s.cp.clone(), launch: env.launch.clone() });
    }
    let _ = std::fs::remove_dir_all(&env.work);
    for p in &closure_files {
        let v = read_json(p)?;
        digests.push((closure_name(p), profile::files_digest(&[("closure".into(), p.clone())])?));
        entries.push(EntryClosure { name: closure_name(p), closure: v, cp: Vec::new(), launch: None });
    }
    lib_jars.sort();
    lib_jars.dedup();
    let cp = env.jdk_path(&lib_jars)?;
    let archives: Vec<(Origin, PathBuf)> = cp.archives();
    let deps_lock = match args.opt("--deps") {
        Some(d) => profile::files_digest(&[("deps.lock.toml".into(), PathBuf::from(d))])?,
        None => String::new(),
    };
    let inputs = KeyInputs {
        generator: GENERATOR_DIGEST.into(),
        runtime: profile::runtime_digest(env.rt.parent().unwrap_or(&env.rt))?,
        jdk_major: resolve::jdk::release_major(&env.home).ok_or_else(|| format!("读不出 JDK 主版本：{}/release", env.home.display()))?,
        archives: profile::archives_digest(&archives)?,
        entries: profile::entries_digest(&digests),
        deps_lock,
    };
    let facts = resolve::ModuleFacts::build(&cp);
    let mut v = profile::build(&entries, &|id: &str| handler_pcs(&cp, id), &inputs, Some((&facts, &cp)))?;
    let elapsed = t0.elapsed().as_millis() as u64;
    let peak = closure::engine::peak_mem_mb();
    v["profile"]["elapsed_ms"] = elapsed.into();
    v["profile"]["peak_mem_mb"] = peak.into();
    if let Some(o) = args.opt("-o") {
        write_json(Path::new(&o), &v)?;
    }
    let s = &v["summary"];
    println!(
        "[profile] 入口 {} 个；类 {}，方法 {}，折叠方法 {}；键 {}；内容摘要 {}；{} ms，峰值 {} MB",
        entries.len(),
        s["classes"],
        s["methods"],
        s["fold_methods"],
        v["profile"]["key"].as_str().unwrap_or_default(),
        v["profile"]["content_digest"].as_str().unwrap_or_default(),
        elapsed,
        peak
    );
    Ok(())
}

/// `--covers <profile.json> <closure.json>…`
fn covers(env: &Env, profile_path: &Path, closures: &[PathBuf]) -> Result<(), String> {
    if closures.is_empty() {
        return Err("--covers 需要至少一个 closure.json".into());
    }
    let p = read_json(profile_path)?;
    let cp = env.jdk_path(&[])?;
    let mut all = true;
    for c in closures {
        let e = EntryClosure { name: closure_name(c), closure: read_json(c)?, cp: Vec::new(), launch: None };
        let ok = profile::covers(&p, &e, &|id: &str| handler_pcs(&cp, id))?;
        all &= ok;
        println!("{} {}", if ok { "covered" } else { "not-covered" }, c.display());
    }
    if all {
        Ok(())
    } else {
        Err("档案未覆盖全部入口".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_file_lines() {
        let text = "# 注释\nA.java\n\n sub/B.java sub/BHelper.java --name B2 --main p.B --locale zh-CN  # 行尾注释\nclasses/ --root p.C.run:()V --cp junit,hamcrest\n";
        let es = parse_entries(text, Path::new("/d")).unwrap();
        assert_eq!(es.len(), 3);
        assert_eq!(es[0], EntrySpec { name: "A".into(), inputs: vec!["/d/A.java".into()], ..EntrySpec::default() });
        assert_eq!(es[1].name, "B2");
        assert_eq!(es[1].inputs, [PathBuf::from("/d/sub/B.java"), PathBuf::from("/d/sub/BHelper.java")]);
        assert_eq!(es[1].main.as_deref(), Some("p/B"));
        assert_eq!(es[1].locales, ["zh-CN"]);
        assert_eq!(es[2].roots, ["p.C.run:()V"]);
        assert_eq!(es[2].cp, ["junit", "hamcrest"]);
        assert!(parse_entries("--name X\n", Path::new("/d")).is_err());
        assert!(parse_entries("A.java --bogus\n", Path::new("/d")).is_err());
    }

    #[test]
    fn closure_names() {
        assert_eq!(closure_name(Path::new("/x/TestA.closure.json")), "TestA");
        assert_eq!(closure_name(Path::new("/x/TestB.json")), "TestB");
    }
}
