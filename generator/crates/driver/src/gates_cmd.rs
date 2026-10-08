//! `rava closure <输入> --gates`：门自动排名（引擎侧见 `closure/src/engine/gates.rs`）。
//!
//! 1. 本进程跑一次基线分析并留存触发边，在引擎存活时完成全部模型计算（候选、单切、贪心、组合、类别），
//!    产出不借用引擎的计划后立即释放引擎；
//! 2. 计划给出的实测清单（贪心前缀 / 组合 / 单切，预算 `--gates-verify`）以本二进制的子进程重跑：
//!    输入为已编译的类目录 + `--main`，原样带上影响闭包的选项，追加 `--cut-file` 与 `--gates-child`（只写紧凑结果）；
//!    子进程与手工 `--cut-file` 运行逐字节同口径（不复用引导映像、不读写缓存）；
//! 3. 并行度：`--gates-jobs`（0 = 自动）与 CPU 数、内存预算 `--gates-mem-mb`（缺省 12288）按基线峰值估算的
//!    同时可容纳子进程数三者取小；单个子进程超过 `--gates-timeout` 秒（缺省 1800）即终止、记失败。
//!
//! 输出：`--gates-out` 机读 JSON；`-o` 的 closure.json 另并入 `gates` 键；`--gates-md` 人读排名表（同时打到标准输出）。

use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use closure::engine::gates::{GateParams, RealRun, VerifyReq};
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use resolve::Hierarchy;
use serde_json::{json, Value};

use crate::Args;

/// 子进程不带的选项（带值）：输出 / 诊断 / 缓存 / 门排名自身
const CHILD_DROP_VALUE: &[&str] = &[
    "-o", "--why", "--flows", "--report", "--boot-report", "--dump-edges", "--closure-cache", "--closure-cache-max-mb", "--main",
    "--gates-top", "--gates-pool", "--gates-verify", "--gates-jobs", "--gates-mem-mb", "--gates-timeout", "--gates-out", "--gates-md",
    "--gates-child",
];
/// 子进程不带的开关
const CHILD_DROP_FLAG: &[&str] = &["--gates", "--site-prof"];

/// 门排名的带值选项
pub(crate) const VALUE_OPTS: &[&str] =
    &["--gates-top", "--gates-pool", "--gates-verify", "--gates-jobs", "--gates-mem-mb", "--gates-timeout", "--gates-out", "--gates-md", "--gates-child"];

fn num(args: &Args, k: &str, default: u64) -> Result<u64, String> {
    args.opt(k).map_or(Ok(default), |v| v.parse::<u64>().map_err(|_| format!("{k} 需为非负整数：{v}")))
}

/// 子进程参数：输入换成已编译类目录，去掉输出 / 诊断 / 缓存 / 门排名选项，补 `--main`
fn child_args(args: &Args, classes: &Path, main: &str) -> Vec<String> {
    let mut out = vec![classes.display().to_string(), "--main".into(), main.to_string()];
    let mut it = args.rest.iter().skip(1);
    while let Some(a) = it.next() {
        if CHILD_DROP_VALUE.contains(&a.as_str()) {
            it.next();
        } else if !CHILD_DROP_FLAG.contains(&a.as_str()) {
            out.push(a.clone());
        }
    }
    out
}

/// `--gates-child <out.json>`：分析一次，写紧凑结果（类表、方法数、耗时、峰值内存）
pub(crate) fn child(input: &closure::Input, h: &Hierarchy, man: &Manifest, hw: &Handwritten, out: &str) -> Result<(), String> {
    let c = closure::analyze(input, h, man, hw).map_err(|f| f.to_string())?;
    let mut classes: Vec<&String> = c.engine.classes.keys().collect();
    classes.sort_unstable();
    let v = json!({
        "classes": classes,
        "methods": c.engine.method_count(),
        "elapsed_ms": c.elapsed_ms as u64,
        "peak_mem_mb": closure::engine::peak_mem_mb(),
    });
    std::fs::write(out, serde_json::to_vec(&v).map_err(|e| e.to_string())?).map_err(|e| format!("{out}：{e}"))
}

fn read_child(p: &Path) -> Result<RealRun, String> {
    let v: Value = serde_json::from_slice(&std::fs::read(p).map_err(|e| format!("{}：{e}", p.display()))?).map_err(|e| e.to_string())?;
    let classes = v["classes"].as_array().ok_or("子进程结果缺 classes")?.iter().filter_map(|x| x.as_str().map(String::from)).collect();
    Ok(RealRun {
        classes,
        methods: v["methods"].as_u64().unwrap_or(0) as usize,
        elapsed_ms: v["elapsed_ms"].as_u64().unwrap_or(0),
        peak_mem_mb: v["peak_mem_mb"].as_u64().unwrap_or(0),
    })
}

fn log_tail(p: &Path) -> String {
    let s = std::fs::read_to_string(p).unwrap_or_default();
    let lines: Vec<&str> = s.lines().rev().take(5).collect();
    lines.into_iter().rev().collect::<Vec<_>>().join(" | ")
}

struct Running {
    idx: usize,
    child: Child,
    start: Instant,
}

/// 并行跑实测清单（至多 `jobs` 个子进程同时运行）
fn run_children(reqs: &[VerifyReq], base: &[String], dir: &Path, jobs: usize, timeout: Duration) -> Vec<Result<RealRun, String>> {
    let exe = std::env::current_exe().ok();
    let mut results: Vec<Option<Result<RealRun, String>>> = vec![None; reqs.len()];
    let mut running: Vec<Running> = Vec::new();
    let mut next = 0;
    let path = |i: usize, ext: &str| dir.join(format!("verify-{i}.{ext}"));
    while next < reqs.len() || !running.is_empty() {
        while running.len() < jobs && next < reqs.len() {
            let i = next;
            next += 1;
            let spawn = (|| -> Result<Child, String> {
                let exe = exe.as_ref().ok_or("取不到本二进制路径")?;
                let cuts = path(i, "cuts");
                std::fs::write(&cuts, reqs[i].cuts.join("\n") + "\n").map_err(|e| e.to_string())?;
                let log = std::fs::File::create(path(i, "log")).map_err(|e| e.to_string())?;
                Command::new(exe)
                    .arg("closure")
                    .args(base)
                    .arg("--cut-file")
                    .arg(&cuts)
                    .arg("--gates-child")
                    .arg(path(i, "json"))
                    .stdout(Stdio::null())
                    .stderr(log)
                    .spawn()
                    .map_err(|e| format!("启动子进程：{e}"))
            })();
            match spawn {
                Ok(child) => running.push(Running { idx: i, child, start: Instant::now() }),
                Err(e) => results[i] = Some(Err(e)),
            }
        }
        std::thread::sleep(Duration::from_millis(200));
        running.retain_mut(|r| {
            let done = match r.child.try_wait() {
                Ok(Some(st)) if st.success() => Some(read_child(&path(r.idx, "json"))),
                Ok(Some(st)) => Some(Err(format!("子进程失败（{st}）：{}", log_tail(&path(r.idx, "log"))))),
                Ok(None) if r.start.elapsed() > timeout => {
                    let _ = r.child.kill();
                    let _ = r.child.wait();
                    Some(Err(format!("超时（{} 秒）", timeout.as_secs())))
                }
                Ok(None) => None,
                Err(e) => Some(Err(e.to_string())),
            };
            match done {
                Some(res) => {
                    let label = reqs[r.idx].cuts.first().cloned().unwrap_or_default();
                    match &res {
                        Ok(x) => eprintln!("[gates] 实测 {}/{}：{} 类（{} ms，{} MB）{label}", r.idx + 1, reqs.len(), x.classes.len(), x.elapsed_ms, x.peak_mem_mb),
                        Err(e) => eprintln!("[gates] 实测 {}/{} 失败：{e}", r.idx + 1, reqs.len()),
                    }
                    results[r.idx] = Some(res);
                    false
                }
                None => true,
            }
        });
    }
    results.into_iter().map(|r| r.unwrap_or_else(|| Err("未运行".into()))).collect()
}

/// 同时可容纳的子进程数：显式 / CPU 数 / 内存预算按基线峰值估算，三者取小
fn effective_jobs(explicit: u64, budget_mb: u64, base_peak_mb: u64) -> usize {
    let cpus = std::thread::available_parallelism().map_or(1, |n| n.get());
    let want = if explicit == 0 { cpus } else { explicit as usize };
    // 父进程释放引擎后约留基线峰值的一半（类路径缓存、计划）；子进程峰值按基线峰值计
    let per = base_peak_mb.max(64);
    let by_mem = (budget_mb.saturating_sub(per / 2) / per).max(1) as usize;
    want.min(cpus).min(by_mem).max(1)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    args: &Args,
    mut input: closure::Input,
    h: &Hierarchy,
    man: &Manifest,
    hw: &Handwritten,
    classes: &Path,
    main: &str,
) -> Result<(), String> {
    input.diag.keep_edges = true;
    let params = GateParams {
        top: num(args, "--gates-top", 20)? as usize,
        pool: num(args, "--gates-pool", 0)? as usize,
        ..GateParams::default()
    };
    let budget = num(args, "--gates-verify", 12)? as usize;
    let t0 = Instant::now();
    let none = crate::closure_run::CacheOpts { dir: None, max_mb: None };
    let out = crate::closure_run::analyze(&none, &input, h, man, hw, true, args.opt("-o").is_some()).map_err(|f| f.to_string())?;
    let mut c = out.closure.ok_or("闭包引擎缺失")?;
    let edges = c.edges.take().ok_or("触发边未记录")?;
    let base_ms = c.elapsed_ms;
    let plan = c.engine.gate_plan(edges, params);
    drop(c);
    let base_peak = closure::engine::peak_mem_mb();
    eprintln!(
        "[gates] 基线 {} 类 / {} 方法（{base_ms} ms，峰值 {base_peak} MB）；模型 {} 类 / {} 方法；候选池 {}，排名 {} 门，贪心 {} 步，组合 {} 组（{} ms）",
        plan.base.classes,
        plan.base.methods,
        plan.base.model_classes,
        plan.base.model_methods,
        plan.base.pool,
        plan.gates.len(),
        plan.greedy.len(),
        plan.groups.len(),
        plan.base.model_ms
    );

    let reqs = plan.verify(budget);
    let jobs = effective_jobs(num(args, "--gates-jobs", 0)?, num(args, "--gates-mem-mb", 12288)?, base_peak);
    let timeout = Duration::from_secs(num(args, "--gates-timeout", 1800)?);
    let dir = std::env::temp_dir().join(format!("rava-gates-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}：{e}", dir.display()))?;
    let tv = Instant::now();
    if !reqs.is_empty() {
        eprintln!("[gates] 实测重跑 {} 次，并行 {jobs}", reqs.len());
    }
    let runs = run_children(&reqs, &child_args(args, classes, main), &dir, jobs, timeout);
    let _ = std::fs::remove_dir_all(&dir);
    let meta = json!({
        "main": main,
        "base_elapsed_ms": base_ms as u64,
        "base_peak_mem_mb": base_peak,
        "verify_jobs": jobs,
        "verify_elapsed_ms": tv.elapsed().as_millis() as u64,
        "total_elapsed_ms": t0.elapsed().as_millis() as u64,
        "mem_budget_mb": num(args, "--gates-mem-mb", 12288)?,
    });
    let (gj, md) = plan.finish(&reqs, &runs, meta);
    if let Some(p) = args.opt("--gates-out") {
        write(&p, &serde_json::to_string_pretty(&gj).map_err(|e| e.to_string())?)?;
    }
    if let Some(p) = args.opt("-o") {
        let mut v = out.json.ok_or("闭包产物缺失")?;
        v["gates"] = gj;
        write(&p, &serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?)?;
    }
    if let Some(p) = args.opt("--gates-md") {
        write(&p, &md)?;
    }
    println!("{md}");
    Ok(())
}

fn write(p: &str, s: &str) -> Result<(), String> {
    if let Some(d) = Path::new(p).parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    std::fs::write(p, s).map_err(|e| format!("{p}：{e}"))
}

/// 已编译类目录的绝对路径（子进程 cwd 相同，绝对化只为日志可读）
pub(crate) fn abs(p: &Path) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_args_drop_outputs_and_keep_inputs() {
        let a = Args {
            rest: ["T.java", "--jdk", "21", "-o", "x.json", "--gates", "--gates-top", "5", "--image", "/i", "--cut", "a.b:()V", "--main", "T", "--cold-cut"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        };
        let v = child_args(&a, Path::new("/tmp/cls"), "p/T");
        assert_eq!(v, ["/tmp/cls", "--main", "p/T", "--jdk", "21", "--image", "/i", "--cut", "a.b:()V", "--cold-cut"]);
    }

    #[test]
    fn jobs_bounded_by_memory() {
        assert_eq!(effective_jobs(8, 12288, 3400).min(3), effective_jobs(8, 12288, 3400));
        assert_eq!(effective_jobs(1, 12288, 100), 1);
        assert!(effective_jobs(0, 100, 5000) >= 1);
    }
}
