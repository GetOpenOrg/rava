//! `rava audit`：编译前缺口审计（调用链可达、但生成体落为 native 存根 / 方法体存根的成员）。
//!
//! - `rava audit api <包>… [--recursive]`：指定包的 public 类的 public / protected 方法为入口，一次闭包 + 发射，
//!   报告 `docs/reports/gap-scan-api-<包>.md`；
//! - `rava audit corpus [--filter S]… [-j N]`：e2e 语料逐例，按触达测试数汇总，报告 `docs/reports/gap-scan-corpus-<过滤>.md`；
//! - `rava audit native [--filter S]… [-j N]`：同一逐例扫描，只取闭包种类为 `handwritten:native` /
//!   `handwritten:boundary` 的缺口（无手写体），报告 `docs/reports/native-gap-scan.md`；
//! - `rava audit boot [<A.java>] [closure 选项]…`：构建期引导映像审计（缺省入口 e2e HelloWorld），报告
//!   `docs/reports/boot-image-jdk<N>-<平台>.md`（`--boot-report` 可改），求值失败即命令失败；
//! - `rava audit test <A.java> [build 选项]…`：单例扫描（corpus / native 的子进程），输出一行 `[gaps] <json>`。
//!
//! 缺口归类即发射层预检（[`Precheck`]）：发射只在内存进行（[`scan_gaps`]），不写 scratch；javac 产物落
//! `<仓库>/build/audit/` 下的临时目录，用完即删。口径是调用链可达（过近似），不等于运行期必然执行。

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use emit::method_bodies::MethodBodies;
use emit::perf::Perf;
use emit::precheck::Precheck;
use emit::project::scan_gaps;
use input::facts::MethodKind;
use serde_json::{json, Value};

use crate::api_roots::api_roots;
use crate::audit_report::{native_report, write_gap_report, GapHits, GAP_KINDS};
use crate::build_cmd::{abs, analyze, class_path, image_dirs, javac, repo_root, user_order, with_emit_ctx, EmitJob};
use crate::build_libs::{self, Libs};
use crate::build_opts::{BuildOpts, Mode};
use crate::closure_cmd::find_runtime_dir;
use crate::Args;

/// 单例扫描结果行前缀（子进程 stdout）
const GAPS_TAG: &str = "[gaps] ";
/// 单例扫描超时（含 javac / 闭包 / 内存发射）
const TEST_TIMEOUT: Duration = Duration::from_secs(1800);
/// 语料扫描缺省并行子进程数
const DEFAULT_JOBS: usize = 2;
/// api 模式的合成入口类
const ENTRY_CLASS: &str = "GapScanEntry";

/// 一次扫描的缺口：预检两类 + 缺口成员的闭包种类
#[derive(Debug, Default)]
pub struct Gaps {
    pub precheck: Precheck,
    pub kinds: BTreeMap<String, String>,
}

impl Gaps {
    fn to_json(&self) -> Value {
        json!({ "native-missing": self.precheck.native_missing, "boundary-stub": self.precheck.boundary_stub, "kinds": self.kinds })
    }

    fn from_json(v: &Value) -> Gaps {
        let list = |k: &str| -> Vec<String> {
            v[k].as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from).collect()
        };
        let kinds = v["kinds"].as_object().into_iter().flatten().map(|(k, s)| (k.clone(), s.as_str().unwrap_or_default().to_string())).collect();
        Gaps { precheck: Precheck { native_missing: list("native-missing"), boundary_stub: list("boundary-stub") }, kinds }
    }

    fn of_kind(&self, kind: &str) -> &[String] {
        if kind == GAP_KINDS[0] { &self.precheck.native_missing } else { &self.precheck.boundary_stub }
    }
}

fn kind_name(k: &MethodKind) -> String {
    match k {
        MethodKind::Bytecode => "bytecode".into(),
        MethodKind::Handwritten(w) => format!("handwritten:{w}"),
        MethodKind::Abstract => "abstract".into(),
        MethodKind::Missing => "missing".into(),
    }
}

/// `<仓库>/build/audit/<tag>-<pid>`：本次扫描独占的临时目录（javac 产物；内存发射的虚拟 scratch 路径）
fn temp_dir(repo: &Path, tag: &str) -> PathBuf {
    repo.join("build").join("audit").join(format!("{tag}-{}", std::process::id()))
}

/// javac → 闭包 → 内存发射 → 预检；`o.inputs` 为源文件
fn scan(o: &BuildOpts, tag: &str) -> Result<Gaps, String> {
    let rt = abs(&find_runtime_dir(o.runtime.clone())?);
    let repo = repo_root(&rt);
    let home = resolve::jdk::choose(o.jdk, o.java_home.as_deref(), Some(&repo))?.home;
    let tmp = temp_dir(&repo, tag);
    let r = scan_in(o, &rt, &home, &tmp);
    if tmp.is_dir() {
        std::fs::remove_dir_all(&tmp).map_err(|e| format!("{}：{e}", tmp.display()))?;
    }
    r
}

fn scan_in(o: &BuildOpts, rt: &Path, home: &Path, tmp: &Path) -> Result<Gaps, String> {
    let classes = tmp.join("classes");
    let libs_sel = build_libs::select_entries(o)?;
    let jars: Vec<PathBuf> = libs_sel.iter().map(|e| e.path.clone()).collect();
    javac(home, &o.inputs, &jars, &classes)?;
    let cp = class_path(&classes, &libs_sel, home, &image_dirs(o, home, rt), None)?;
    let Libs { crates, .. } = build_libs::from_lock(&libs_sel, &cp, &resolve::ModuleFacts::build(&cp))?;
    let seed_classes = o.seed_classes.clone();
    let user = user_order(&cp, &o.inputs, o.main.as_deref())?;
    let java_files: Vec<PathBuf> = o.inputs.iter().map(|p| abs(p)).collect();
    let virtual_out = tmp.join("scratch");
    let mut perf = Perf::new();
    analyze(&cp, rt, &user[0], o, &seed_classes, &tmp.join("closure.json"), &mut perf, |facts, perf| {
        let job = EmitJob { cp: &cp, facts, rt, user: &user, java_files, home, out: &virtual_out, libs: &crates, o };
        let precheck = with_emit_ctx(&job, perf, |ctx, _, _| {
            scan_gaps(ctx, &virtual_out, &MethodBodies::new(ctx)).map_err(|e| format!("发射：{e}"))
        })?;
        let gap: BTreeSet<&String> = precheck.native_missing.iter().chain(&precheck.boundary_stub).collect();
        let kinds = facts
            .methods
            .iter()
            .filter_map(|m| {
                let id = m.id.to_string();
                gap.contains(&id).then(|| (id, kind_name(&m.kind)))
            })
            .collect();
        Ok(Gaps { precheck, kinds })
    })
}

/// 子命令参数：`--filter S…`（到下一个选项为止）、`-j N`、`--recursive`、其余原样转交 build 选项
#[derive(Default)]
struct AuditArgs {
    positional: Vec<String>,
    filters: Vec<String>,
    jobs: usize,
    recursive: bool,
    /// 转交单例扫描的 build 选项（`--jdk` / `--java-home` / `--runtime` / `--closure-cache` …）
    pass: Vec<String>,
}

fn parse_audit(rest: &[String]) -> Result<AuditArgs, String> {
    let mut a = AuditArgs { jobs: DEFAULT_JOBS, ..AuditArgs::default() };
    let mut it = rest.iter().peekable();
    while let Some(x) = it.next() {
        match x.as_str() {
            "--filter" => {
                while let Some(v) = it.next_if(|v| !v.starts_with('-')) {
                    a.filters.push(v.clone());
                }
            }
            "-j" | "--jobs" => {
                let v = it.next().ok_or("-j 缺少取值")?;
                a.jobs = v.parse().ok().filter(|n| *n > 0).ok_or_else(|| format!("-j 需为正整数：{v}"))?;
            }
            "--recursive" => a.recursive = true,
            s if s.starts_with('-') => {
                a.pass.push(x.clone());
                if let Some(v) = it.next_if(|v| !v.starts_with('-')) {
                    a.pass.push(v.clone());
                }
            }
            _ => a.positional.push(x.clone()),
        }
    }
    Ok(a)
}

/// 转交选项中的 `--runtime`（缺省自动查找）
fn runtime_of(pass: &[String]) -> Result<PathBuf, String> {
    let given = pass.iter().position(|p| p == "--runtime").and_then(|i| pass.get(i + 1)).map(PathBuf::from);
    Ok(abs(&find_runtime_dir(given)?))
}

/// 未显式给出时补上跨运行闭包缓存（与 build 同一目录）
fn with_cache(mut pass: Vec<String>, repo: &Path) -> Vec<String> {
    if !pass.iter().any(|p| p == "--closure-cache") {
        pass.extend(["--closure-cache".to_string(), repo.join("build").join("closure_cache").display().to_string()]);
    }
    pass
}

pub fn run(args: &Args) -> Result<(), String> {
    let Some((mode, rest)) = args.rest.split_first() else {
        return Err("用法：rava audit api|corpus|native|boot|test …".into());
    };
    match mode.as_str() {
        "test" => run_test(rest),
        "boot" => run_boot(rest),
        "api" => run_api(&parse_audit(rest)?),
        "corpus" | "native" => run_corpus(mode == "native", &parse_audit(rest)?),
        _ => Err(format!("未知审计模式：{mode}（api / corpus / native / boot / test）")),
    }
}

/// 引导映像审计：转 `rava closure <入口> --boot-report <报告>`
fn run_boot(rest: &[String]) -> Result<(), String> {
    let mut argv: Vec<String> = rest.to_vec();
    let probe = Args { rest: argv.clone() };
    let rt = runtime_of(&argv)?;
    let repo = repo_root(&rt);
    if argv.first().is_none_or(|a| a.starts_with('-')) {
        argv.insert(0, repo.join("tests/e2e/01_basics/HelloWorld.java").display().to_string());
    }
    if probe.opt("--boot-report").is_none() {
        let jdk = crate::choose_jdk(&probe)?.major.map_or("x".to_string(), |m| m.to_string());
        let out = repo.join("docs/reports").join(format!("boot-image-jdk{jdk}-{}.md", std::env::consts::OS));
        argv.extend(["--boot-report".to_string(), out.display().to_string()]);
    }
    crate::closure_cmd::run(&Args { rest: argv })
}

/// 单例：`[gaps] <json>` 一行（其余输出为闭包诊断）
fn run_test(rest: &[String]) -> Result<(), String> {
    let o = BuildOpts::parse(Mode::Build, rest)?;
    let tag = o.inputs.first().and_then(|p| p.file_stem()).map_or("test".into(), |s| s.to_string_lossy().to_string());
    let g = scan(&o, &tag)?;
    println!("{GAPS_TAG}{}", g.to_json());
    Ok(())
}

fn run_api(a: &AuditArgs) -> Result<(), String> {
    if a.positional.is_empty() {
        return Err("rava audit api 需要至少一个包（斜线形态，如 java/util）".into());
    }
    let rt = runtime_of(&a.pass)?;
    let repo = repo_root(&rt);
    let entry_dir = temp_dir(&repo, "api-entry");
    std::fs::create_dir_all(&entry_dir).map_err(|e| format!("{}：{e}", entry_dir.display()))?;
    let entry = entry_dir.join(format!("{ENTRY_CLASS}.java"));
    std::fs::write(&entry, format!("public class {ENTRY_CLASS} {{ public static void main(String[] a) {{}} }}\n"))
        .map_err(|e| format!("{}：{e}", entry.display()))?;
    let mut argv = vec![entry.display().to_string()];
    for p in &a.positional {
        argv.extend(["--api-package".to_string(), p.clone()]);
    }
    if a.recursive {
        argv.push("--api-recursive".into());
    }
    argv.extend(with_cache(a.pass.clone(), &repo));
    let t0 = Instant::now();
    let r = BuildOpts::parse(Mode::Build, &argv).and_then(|o| {
        let n = api_counts(&o, &rt, &entry_dir)?;
        scan(&o, "api").map(|g| (g, n))
    });
    std::fs::remove_dir_all(&entry_dir).map_err(|e| format!("{}：{e}", entry_dir.display()))?;
    let (g, (n_cls, n_seeds)) = r?;
    let mut hits = GapHits::default();
    for kind in GAP_KINDS {
        for m in g.of_kind(kind) {
            hits.add(kind, m, "api");
        }
    }
    let meta = vec![
        format!(
            "入口包：{}（{}子包），{n_cls} 个 public 类 / {n_seeds} 个 public·protected 方法",
            a.positional.join(", "),
            if a.recursive { "含" } else { "不含" }
        ),
        format!("BFS 耗时 {:.1} 分钟", t0.elapsed().as_secs_f64() / 60.0),
        format!("native-missing {} 个，boundary-stub {} 个", g.precheck.native_missing.len(), g.precheck.boundary_stub.len()),
    ];
    let suffix: Vec<String> = a.positional.iter().map(|p| p.replace('/', ".")).collect();
    let path = repo.join("docs/reports").join(format!("gap-scan-api-{}.md", suffix.join("-")));
    write_gap_report(&path, "编译前缺口扫描（API 模式）", &meta, &hits, None)?;
    println!("报告 → {}", path.strip_prefix(&repo).unwrap_or(&path).display());
    Ok(())
}

/// api 模式入口规模（public 类数 / 入口方法数），口径同 build 的 `[api]` 行
fn api_counts(o: &BuildOpts, rt: &Path, entry_dir: &Path) -> Result<(usize, usize), String> {
    let home = resolve::jdk::choose(o.jdk, o.java_home.as_deref(), Some(&repo_root(rt)))?.home;
    let cp = class_path(entry_dir, &[], &home, &image_dirs(o, &home, rt), None)?;
    let (roots, n_cls) = api_roots(&cp, &o.api_packages, o.api_recursive);
    Ok((n_cls, roots.len()))
}

/// e2e 语料：`tests/e2e/<类别>/<Test>.java`，按测试名子串过滤
fn corpus_files(repo: &Path, filters: &[String]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for cat in std::fs::read_dir(repo.join("tests/e2e")).into_iter().flatten().flatten() {
        for f in std::fs::read_dir(cat.path()).into_iter().flatten().flatten() {
            let p = f.path();
            let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            if p.extension().is_some_and(|e| e == "java") && (filters.is_empty() || filters.iter().any(|s| stem.contains(s))) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// 子进程跑单例扫描：(缺口, 失败说明)
fn scan_child(exe: &Path, file: &Path, pass: &[String]) -> (Gaps, Option<String>) {
    let spawned = Command::new(exe).args(["audit", "test"]).arg(file).args(pass).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
    let mut child = match spawned {
        Ok(c) => c,
        Err(e) => return (Gaps::default(), Some(format!("启动失败：{e}"))),
    };
    let (mut so, mut se) = (child.stdout.take().expect("piped"), child.stderr.take().expect("piped"));
    let out_t = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = so.read_to_string(&mut s);
        s
    });
    let err_t = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = se.read_to_string(&mut s);
        s
    });
    let t0 = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break Some(st),
            Ok(None) if t0.elapsed() >= TEST_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(200)),
            Err(_) => break None,
        }
    };
    let (stdout, stderr) = (out_t.join().unwrap_or_default(), err_t.join().unwrap_or_default());
    let gaps = stdout.lines().find_map(|l| l.strip_prefix(GAPS_TAG)).and_then(|j| serde_json::from_str::<Value>(j).ok());
    match (status, gaps) {
        (None, _) => (Gaps::default(), Some(format!("超时（{} s）", TEST_TIMEOUT.as_secs()))),
        (Some(st), Some(v)) if st.success() => (Gaps::from_json(&v), None),
        (Some(st), _) => {
            let last = stderr.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("?").to_string();
            (Gaps::default(), Some(format!("{st}：{last}")))
        }
    }
}

fn run_corpus(native: bool, a: &AuditArgs) -> Result<(), String> {
    let rt = runtime_of(&a.pass)?;
    let repo = repo_root(&rt);
    let files = corpus_files(&repo, &a.filters);
    let exe = std::env::current_exe().map_err(|e| format!("rava 可执行文件：{e}"))?;
    let pass = with_cache(a.pass.clone(), &repo);
    println!("语料模式：{} 个测试，并行 {}", files.len(), a.jobs);
    let t0 = Instant::now();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let results: Mutex<Vec<(String, Gaps, Option<String>)>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..a.jobs.min(files.len().max(1)) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                let Some(f) = files.get(i) else { break };
                let name = f.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                let (g, err) = scan_child(&exe, f, &pass);
                let k = done.fetch_add(1, Ordering::SeqCst) + 1;
                println!(
                    "[{k:>4}/{}] {name:<40} native-missing={} boundary-stub={}{}",
                    files.len(),
                    g.precheck.native_missing.len(),
                    g.precheck.boundary_stub.len(),
                    if err.is_some() { "  (失败)" } else { "" }
                );
                results.lock().expect("results").push((name, g, err));
            });
        }
    });
    let mut results = results.into_inner().expect("results");
    results.sort_by(|x, y| x.0.cmp(&y.0));
    let errors: Vec<String> = results.iter().filter_map(|(n, _, e)| e.as_ref().map(|e| format!("{n}: {e}"))).collect();
    if native {
        let (path, summary) = native_report(&repo, files.len(), &results)?;
        println!("{summary}\n报告 → {}", path.strip_prefix(&repo).unwrap_or(&path).display());
    } else {
        let mut hits = GapHits::default();
        for (name, g, _) in &results {
            for kind in GAP_KINDS {
                for m in g.of_kind(kind) {
                    hits.add(kind, m, name);
                }
            }
        }
        let meta = vec![
            format!("测试数 {}，耗时 {:.1} 分钟，并行 {}", files.len(), t0.elapsed().as_secs_f64() / 60.0, a.jobs),
            format!("native-missing 去重 {} 个，boundary-stub 去重 {} 个", hits.len(GAP_KINDS[0]), hits.len(GAP_KINDS[1])),
            format!(
                "BFS 失败 {} 个{}",
                errors.len(),
                if errors.is_empty() { String::new() } else { format!("：{}", errors.iter().take(10).cloned().collect::<Vec<_>>().join("; ")) }
            ),
        ];
        let suffix = if a.filters.is_empty() { "all".to_string() } else { a.filters.join("-") };
        let path = repo.join("docs/reports").join(format!("gap-scan-corpus-{suffix}.md"));
        write_gap_report(&path, "编译前缺口扫描（语料模式）", &meta, &hits, Some(files.len()))?;
        println!("报告 → {}", path.strip_prefix(&repo).unwrap_or(&path).display());
    }
    if !errors.is_empty() {
        return Err(format!("{} 个测试扫描失败：{}", errors.len(), errors.iter().take(5).cloned().collect::<Vec<_>>().join("; ")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_args() {
        let v: Vec<String> = ["--filter", "Cal", "Date", "-j", "3", "--jdk", "21", "--recursive", "java/util"].iter().map(|s| s.to_string()).collect();
        let a = parse_audit(&v).unwrap();
        assert_eq!(a.filters, ["Cal", "Date"]);
        assert_eq!((a.jobs, a.recursive), (3, true));
        assert_eq!(a.pass, ["--jdk", "21"]);
        assert_eq!(a.positional, ["java/util"]);
        assert!(parse_audit(&["-j".to_string(), "0".to_string()]).is_err());
    }

    #[test]
    fn gaps_json_round_trip() {
        let g = Gaps {
            precheck: Precheck { native_missing: vec!["a/B.c:()V".into()], boundary_stub: vec!["a/B.d:()V".into()] },
            kinds: [("a/B.c:()V".to_string(), "handwritten:native".to_string())].into_iter().collect(),
        };
        let back = Gaps::from_json(&g.to_json());
        assert_eq!((back.precheck, back.kinds), (g.precheck, g.kinds));
    }
}
