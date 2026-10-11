//! 内存感知的 cargo 并行作业数（计划 docs/plans/2026-10-11-build-memsafe.md）。
//!
//! 终态规则（每次 `rava compile` / `rava build` 编译段执行；实测 OOM 时的退半重试见 [`crate::compile_cmd`]）：
//! 1. 可用内存 A：命令行 `--build-mem-mb` 显式给定，否则 [`crate::mem_probe::probe`]（min(系统可用, 各级 cgroup 余量)）。
//! 2. 预算 B = A × [`BUDGET_PCT`]%（留 15% 余量给页缓存、cargo、链接器与估计误差）。
//! 3. 逐 crate 估计单作业编译时的 rustc 峰值 E(c)（[`Model::crate_est`]，系数按档位取服务器逐 crate 实测的上包络）：
//!    代码量 c（MB，不含引导映像数据）走两段折线 min(a₁ + b₁·c, a₂ + b₂·c)——小 crate 每 MB 贵、
//!    大声明层每 MB 便宜；引导映像这类纯静态数据另按每 MB 系数计；超长函数（≥ [`LONG_FN_LINES`] 行，
//!    如资源束 `getContents` 的大数组字面量）在 codegen-units 1 的档位上另有超线性开销，按其总行数计。
//!    E 按单作业实测标定：并行时同一 rustc 多出的 codegen 线程也占作业槽，每槽的增量小于一个 crate 的 E。
//! 4. 可执行文件所在 crate 只在全部库 crate 编译完成后才启动（bin 须链接全部 rlib），不与它们并发：
//!    最坏峰值 = max(该 crate 估计（LTO 档位含全程序链接，[`Model::link_est`]）, 最大的 J 个库 crate 的 E 之和)。
//!    J = 使后者 ≤ B 的最大值，取值区间 [1, 上限]；上限 = 调用方 `CARGO_BUILD_JOBS`（若设）、否则自工作区
//!    向上最近的 cargo 配置 `[build] jobs`（多槽跑批按槽写入）、否则 CPU 核数——rava 总是显式设置
//!    `CARGO_BUILD_JOBS`，它优先于 cargo 配置，故配置值须在此并入上限，不能被覆盖掉。
//! 5. 单个 crate（含全程序链接）的估计已超出 B 时 J = 1，并提示该档位超出本机内存——
//!    作业数救不了单进程峰值，应换更省内存的档位（缺省 release 即为此而定）或更大的机器。
//! 6. 探测不到内存时不按内存收紧（J = 上限）。

use std::path::Path;

use serde_json::{json, Value};

use crate::cargo::BuildProfile;
use crate::mem_probe::MemProbe;

/// 预算占可用内存的百分比
pub const BUDGET_PCT: u64 = 85;

/// 超长函数的行数门槛（规则第 3 条）
pub const LONG_FN_LINES: u64 = 1000;

/// 一个工作区 crate 的体量
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateStat {
    pub name: String,
    /// 代码字节（`src/**.rs`，不含引导映像模块）
    pub code_bytes: u64,
    /// 引导映像模块字节（纯静态数据）
    pub data_bytes: u64,
    /// ≥ [`LONG_FN_LINES`] 行的函数的总行数
    pub long_fn_lines: u64,
}

impl CrateStat {
    pub fn total_bytes(&self) -> u64 {
        self.code_bytes + self.data_bytes
    }
}

/// rustc 峰值估计系数（MB；斜率为每 MB 源码）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Model {
    /// 代码折线第一段（小 crate）：基数、每 MB
    pub small: (f64, f64),
    /// 代码折线第二段（大 crate）：基数、每 MB
    pub large: (f64, f64),
    /// 引导映像数据：每 MB
    pub per_data_mb: f64,
    /// 超长函数：每千行
    pub per_long_kline: f64,
    /// 全程序链接（LTO 档位的可执行文件 crate）：基数；0 表示该档位无全程序链接期优化
    pub link_base_mb: f64,
    /// 全程序链接：每 MB 全工作区源码
    pub link_per_total_mb: f64,
}

impl Model {
    /// 各档位系数（服务器单作业逐 crate 实测上包络，数据见计划文档 §三）
    pub fn of(p: BuildProfile) -> Model {
        let dev = Model {
            small: (520.0, 300.0),
            large: (1350.0, 170.0),
            per_data_mb: 120.0,
            per_long_kline: 0.0,
            link_base_mb: 0.0,
            link_per_total_mb: 0.0,
        };
        let release = Model {
            small: (750.0, 460.0),
            large: (2000.0, 190.0),
            per_data_mb: 130.0,
            per_long_kline: 55.0,
            link_base_mb: 2300.0,
            link_per_total_mb: 40.0,
        };
        match p {
            BuildProfile::Dev | BuildProfile::DevOpt => dev,
            BuildProfile::Release => release,
            // fat LTO 全程序单模块：B3 实测大闭包 14.5–15.7 GB（binary-size §五）
            BuildProfile::ReleaseMax | BuildProfile::ReleaseSmall => Model { link_base_mb: 2500.0, link_per_total_mb: 150.0, ..release },
        }
    }

    pub fn crate_est(&self, s: &CrateStat) -> u64 {
        let c = mb(s.code_bytes);
        let code = (self.small.0 + self.small.1 * c).min(self.large.0 + self.large.1 * c);
        (code + self.per_data_mb * mb(s.data_bytes) + self.per_long_kline * s.long_fn_lines as f64 / 1000.0) as u64
    }

    /// 全程序链接估计（无全程序链接期优化的档位为 None）
    pub fn link_est(&self, total_bytes: u64) -> Option<u64> {
        (self.link_base_mb > 0.0).then(|| (self.link_base_mb + self.link_per_total_mb * mb(total_bytes)) as u64)
    }
}

fn mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

/// 一行是否为函数定义的开头（`[pub[(..)]] [const|async|unsafe|extern ".."]* fn `）
fn fn_start(line: &str) -> bool {
    let mut t = line.trim_start();
    if let Some(r) = t.strip_prefix("pub") {
        t = r.trim_start();
        if let Some(r) = t.strip_prefix('(') {
            t = r.split_once(')').map_or("", |x| x.1).trim_start();
        }
    }
    loop {
        let before = t;
        for q in ["const ", "async ", "unsafe "] {
            t = t.strip_prefix(q).unwrap_or(t).trim_start();
        }
        if let Some(r) = t.strip_prefix("extern ") {
            t = r.trim_start();
            if let Some(r) = t.strip_prefix('"') {
                t = r.split_once('"').map_or("", |x| x.1).trim_start();
            }
        }
        if t == before {
            break;
        }
    }
    t.starts_with("fn ")
}

/// 一个源文件中 ≥ [`LONG_FN_LINES`] 行的函数总行数（函数跨度 = 本函数头到下一个函数头 / 文件尾）
pub fn long_fn_lines(text: &str) -> u64 {
    let span = |from: Option<u64>, to: u64| from.map_or(0, |f| to - f);
    let long = |l: u64| if l >= LONG_FN_LINES { l } else { 0 };
    let (mut total, mut last, mut n) = (0, None, 0);
    for line in text.lines() {
        if fn_start(line) {
            total += long(span(last, n));
            last = Some(n);
        }
        n += 1;
    }
    total + long(span(last, n))
}

/// 工作区各 crate 的体量（成员目录 = 含 Cargo.toml 与 src/ 的子目录），按总字节降序
pub fn crate_stats(scratch: &Path) -> Vec<CrateStat> {
    let data_file = format!("{}.rs", emit::project::boot_image::MODULE);
    fn walk(d: &Path, data_file: &str, acc: &mut CrateStat) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let Ok(ft) = e.file_type() else { continue };
            let p = e.path();
            if ft.is_dir() {
                walk(&p, data_file, acc);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let len = e.metadata().map(|m| m.len()).unwrap_or(0);
                if e.file_name().to_str() == Some(data_file) {
                    acc.data_bytes += len;
                } else {
                    acc.code_bytes += len;
                    acc.long_fn_lines += std::fs::read_to_string(&p).map(|t| long_fn_lines(&t)).unwrap_or(0);
                }
            }
        }
    }
    let mut v: Vec<CrateStat> = std::fs::read_dir(scratch)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().join("Cargo.toml").is_file() && e.path().join("src").is_dir())
        .map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let mut s = CrateStat { name, code_bytes: 0, data_bytes: 0, long_fn_lines: 0 };
            walk(&e.path().join("src"), &data_file, &mut s);
            s
        })
        .collect();
    v.sort_by(|a, b| b.total_bytes().cmp(&a.total_bytes()).then_with(|| a.name.cmp(&b.name)));
    v
}

/// 作业数决定
#[derive(Debug, Clone, PartialEq)]
pub struct MemPlan {
    pub jobs: u32,
    /// 作业数上限（调用方 CARGO_BUILD_JOBS、cargo 配置 `[build] jobs` 或 CPU 核数）
    pub cap: u32,
    pub avail_mb: Option<u64>,
    pub source: Option<String>,
    pub budget_mb: Option<u64>,
    /// 最大单 crate 估计（crate, MB；可执行文件 crate 含全程序链接）
    pub top: Option<(String, u64)>,
    /// 全程序链接估计（MB）
    pub link_mb: Option<u64>,
    /// 最大的 J 个库 crate 估计之和（MB）
    pub lib_sum_mb: u64,
    /// 单进程估计已超预算
    pub over: bool,
    /// 实测 OOM 后退半重试用过的作业数（依次）
    pub oom_retries: Vec<u32>,
}

impl MemPlan {
    pub fn to_json(&self) -> Value {
        json!({
            "jobs": self.jobs, "cap": self.cap, "avail_mb": self.avail_mb, "source": self.source,
            "budget_mb": self.budget_mb, "top_crate": self.top.as_ref().map(|t| &t.0),
            "top_est_mb": self.top.as_ref().map(|t| t.1), "link_est_mb": self.link_mb,
            "lib_sum_est_mb": self.lib_sum_mb, "over_budget": self.over, "oom_retries": self.oom_retries,
        })
    }

    pub fn summary(&self) -> String {
        match (self.avail_mb, self.budget_mb) {
            (Some(a), Some(b)) => format!(
                "可用 {a} MB（{}）× {BUDGET_PCT}% = 预算 {b} MB；最大 crate {} 估 {} MB{}；最大 {} 个库 crate 估计和 {} MB；作业数 {} / 上限 {}",
                self.source.as_deref().unwrap_or("?"),
                self.top.as_ref().map_or("-", |t| t.0.as_str()),
                self.top.as_ref().map_or(0, |t| t.1),
                self.link_mb.map(|l| format!("（全程序链接估 {l} MB）")).unwrap_or_default(),
                self.jobs,
                self.lib_sum_mb,
                self.jobs,
                self.cap
            ),
            _ => format!("未探测到可用内存：作业数 {} / 上限 {}", self.jobs, self.cap),
        }
    }

    /// 实测 OOM 后退半（规则见 [`crate::compile_cmd`]）：已是 1 作业时返回 false
    pub fn halve(&mut self) -> bool {
        if self.jobs <= 1 {
            return false;
        }
        self.jobs /= 2;
        self.oom_retries.push(self.jobs);
        true
    }
}

/// 规则第 4–6 条（纯函数）：`stats` 为各 crate 体量（任意序），`bin_crate` 为可执行文件所在 crate
pub fn plan(stats: &[CrateStat], bin_crate: &str, model: Model, probe: Option<MemProbe>, cap: u32) -> MemPlan {
    let cap = cap.max(1);
    let total: u64 = stats.iter().map(CrateStat::total_bytes).sum();
    let link_mb = model.link_est(total);
    // 可执行文件 crate 单独运行：全程序链接在它的 rustc 内进行
    let bin_est = stats.iter().find(|s| s.name == bin_crate).map(|s| model.crate_est(s).max(link_mb.unwrap_or(0)));
    let mut libs: Vec<(String, u64)> =
        stats.iter().filter(|s| s.name != bin_crate).map(|s| (s.name.clone(), model.crate_est(s))).collect();
    libs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top = libs.first().cloned().into_iter().chain(bin_est.map(|e| (bin_crate.to_string(), e))).max_by_key(|t| t.1);
    let lib_sum = |j: usize| libs.iter().take(j).map(|l| l.1).sum::<u64>();
    let Some(p) = probe else {
        let lib_sum_mb = lib_sum(cap as usize);
        return MemPlan { jobs: cap, cap, avail_mb: None, source: None, budget_mb: None, top, link_mb, lib_sum_mb, over: false, oom_retries: vec![] };
    };
    let budget = p.avail_mb * BUDGET_PCT / 100;
    // 库 crate 少于上限且全部放得下时 J = 上限：多出的作业槽只给依赖（体量远小于工作区 crate）
    let jobs = (1..=cap).take_while(|j| *j == 1 || lib_sum(*j as usize) <= budget).last().unwrap_or(1);
    let over = top.as_ref().is_some_and(|t| t.1 > budget);
    MemPlan {
        jobs,
        cap,
        avail_mb: Some(p.avail_mb),
        source: Some(p.source),
        budget_mb: Some(budget),
        top,
        link_mb,
        lib_sum_mb: lib_sum(jobs as usize),
        over,
        oom_retries: vec![],
    }
}

/// 规则全程：探测可用内存、扫描工作区 crate 体量、按档位系数定作业数。
/// 可执行文件所在 crate 为发射层的用户 crate
/// `given_mb`：`--build-mem-mb` 显式给定的可用内存（代替探测）
pub fn decide(scratch: &Path, profile: BuildProfile, given_mb: Option<u64>) -> MemPlan {
    let probe = match given_mb {
        Some(mb) => Some(MemProbe { avail_mb: mb, source: "--build-mem-mb".into() }),
        None => crate::mem_probe::probe(),
    };
    plan(&crate_stats(scratch), emit::ctx::USER_CRATE, Model::of(profile), probe, job_cap(scratch))
}

/// 作业数上限：调用方 CARGO_BUILD_JOBS（正整数）、否则自 `scratch` 向上最近的 cargo 配置 `[build] jobs`、否则 CPU 核数
pub fn job_cap(scratch: &Path) -> u32 {
    std::env::var("CARGO_BUILD_JOBS")
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|j| *j > 0)
        .or_else(|| config_jobs(scratch))
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get() as u32))
}

/// 自 `dir` 向上逐级找 `.cargo/config.toml`（或无扩展名的 `.cargo/config`），取首个给出正整数 `[build] jobs` 的值
/// （cargo 配置合并时内层优先；负数 jobs 表示「核数减 N」，不作上限）
fn config_jobs(dir: &Path) -> Option<u32> {
    let abs = std::path::absolute(dir).ok()?;
    abs.ancestors().find_map(|d| {
        ["config.toml", "config"].iter().find_map(|f| {
            let t = std::fs::read_to_string(d.join(".cargo").join(f)).ok()?;
            let v: toml::Value = toml::from_str(&t).ok()?;
            v.get("build")?.get("jobs")?.as_integer().filter(|j| *j > 0).map(|j| j as u32)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: u64 = 1024 * 1024;

    fn probe(mb: u64) -> Option<MemProbe> {
        Some(MemProbe { avail_mb: mb, source: "test".into() })
    }

    fn model() -> Model {
        Model {
            small: (500.0, 100.0),
            large: (500.0, 100.0),
            per_data_mb: 10.0,
            per_long_kline: 100.0,
            link_base_mb: 1000.0,
            link_per_total_mb: 10.0,
        }
    }

    fn st(name: &str, code_mb: u64) -> CrateStat {
        CrateStat { name: name.into(), code_bytes: code_mb * M, data_bytes: 0, long_fn_lines: 0 }
    }

    #[test]
    fn jobs_fit_largest_lib_crates_in_budget() {
        // 库：decl 3500，body 1500 ×3；user 不与库并发：max(500, 链接 1000+10*60=1600)
        let stats = vec![st("decl", 30), st("b1", 10), st("b2", 10), st("b3", 10), st("user", 0)];
        // 预算 7000×85% = 5950：3500+1500 = 5000 ≤ 5950，再加 1500 = 6500 > 5950
        let p = plan(&stats, "user", model(), probe(7_000), 8);
        assert_eq!((p.jobs, p.budget_mb, p.link_mb, p.lib_sum_mb, p.over), (2, Some(5950), Some(1600), 5000, false));
        assert_eq!(p.top, Some(("decl".into(), 3500)));
        // 调用方上限更小时取上限
        assert_eq!(plan(&stats, "user", model(), probe(7_000), 1).jobs, 1);
        // 全部库放得下：J = 上限
        assert_eq!(plan(&stats, "user", model(), probe(10_000), 8).jobs, 8);
        // 预算不足最大 crate：1 作业并标超预算
        let p = plan(&stats, "user", model(), probe(4000), 8);
        assert_eq!((p.jobs, p.over), (1, true));
        // 全程序链接超预算同样标出
        let p = plan(&[st("a", 1), st("user", 0)], "user", Model { link_per_total_mb: 1000.0, ..model() }, probe(1500), 8);
        assert_eq!((p.top.map(|t| t.0), p.over), (Some("user".into()), true));
        // 探测不到内存：不收紧
        assert_eq!(plan(&stats, "user", model(), None, 6).jobs, 6);
    }

    #[test]
    fn estimate_terms() {
        let m = Model { large: (2000.0, 10.0), ..model() };
        // 折线取小：1 MB → min(600, 2010)；100 MB → min(10500, 3000)
        assert_eq!(m.crate_est(&st("a", 1)), 600);
        assert_eq!(m.crate_est(&st("a", 100)), 3000);
        let s = CrateStat { name: "a".into(), code_bytes: 0, data_bytes: 50 * M, long_fn_lines: 2500 };
        assert_eq!(m.crate_est(&s), 500 + 500 + 250);
    }

    #[test]
    fn long_functions_counted() {
        let body = "    let x = 1;\n".repeat(1200);
        let text = format!("pub(crate) unsafe fn a() {{\n{body}}}\n    fn b() {{}}\npub extern \"C\" fn c() {{\n}}\n");
        assert_eq!(long_fn_lines(&text), 1202);
        assert_eq!(long_fn_lines("fn a() {}\nfn b() {}\n"), 0);
        assert!(fn_start("    pub const fn x()") && !fn_start("    let f = fnx;") && !fn_start("// fn x"));
    }

    #[test]
    fn oom_halving_stops_at_one() {
        let mut p = plan(&[st("a", 1)], "user", model(), None, 5);
        assert!(p.halve() && p.halve());
        assert!(!p.halve());
        assert_eq!((p.jobs, p.oom_retries.clone()), (1, vec![2, 1]));
    }

    #[test]
    fn cargo_config_jobs_found_upwards() {
        let root = std::env::temp_dir().join(format!("rava_cfg_jobs_{}", std::process::id()));
        let scratch = root.join("slot").join("scratch");
        std::fs::create_dir_all(&scratch).unwrap();
        std::fs::create_dir_all(root.join(".cargo")).unwrap();
        std::fs::write(root.join(".cargo").join("config.toml"), "[build]\njobs = 3\n").unwrap();
        assert_eq!(config_jobs(&scratch), Some(3));
        // 内层配置优先；负数不作上限，继续向外找
        std::fs::create_dir_all(root.join("slot").join(".cargo")).unwrap();
        std::fs::write(root.join("slot").join(".cargo").join("config.toml"), "[build]\njobs = -1\n").unwrap();
        assert_eq!(config_jobs(&scratch), Some(3));
        std::fs::write(root.join("slot").join(".cargo").join("config.toml"), "[build]\njobs = 2\n").unwrap();
        assert_eq!(config_jobs(&scratch), Some(2));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn link_only_for_lto_profiles() {
        assert!(Model::of(BuildProfile::Dev).link_est(100 * M).is_none());
        assert!(Model::of(BuildProfile::Release).link_est(100 * M).is_some());
    }
}
