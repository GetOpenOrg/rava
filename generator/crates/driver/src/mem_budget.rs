//! 内存感知的 cargo 并行作业数（二进制体积 B4，docs/plans/2026-10-04-binary-size.md §六）。
//!
//! 终态规则（每次 `rava compile` / `rava build` 编译段执行一次，不是失败后的重试）：
//! 1. 可用内存 A：[`crate::mem_probe::probe`]（`RAVA_BUILD_MEM_MB` > min(系统可用, 各级 cgroup 余量)）。
//! 2. 预算 B = A × [`BUDGET_PCT`]%（留 15% 余量给页缓存、cargo、链接器与估计误差）。
//! 3. 逐 crate 估计 rustc 峰值 E(c) = 基数 + 斜率 × 该 crate 源码 MB（`src/**.rs` 字节；系数按档位，见
//!    [`Model::of`]，取自服务器逐 crate 实测的上包络）。可执行文件所在的 user crate 在 LTO 档位下
//!    承担全程序链接期优化，另按全工作区源码总量估计（[`Model::link_est`]）。
//! 4. 作业数 J = 满足「最大的 J 个 E 之和 ≤ B」的最大 J（并行时最坏是最大的几个 crate 同时编译），
//!    取值区间 [1, 上限]，上限 = 调用方 `CARGO_BUILD_JOBS`（若设）否则 CPU 核数。
//! 5. 单个 crate（含全程序链接）的估计已超出 B 时 J = 1，并提示该档位超出本机内存——
//!    作业数救不了单进程峰值，应换更省内存的档位（缺省 release 即为此而定）或更大的机器。
//! 6. 探测不到内存时不按内存收紧（J = 上限）。

use std::path::Path;

use serde_json::{json, Value};

use crate::cargo::BuildProfile;
use crate::mem_probe::MemProbe;

/// 预算占可用内存的百分比
pub const BUDGET_PCT: u64 = 85;

/// rustc 峰值估计系数（MB；斜率为每 MB 源码）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Model {
    /// 单 crate：基数
    pub base_mb: f64,
    /// 单 crate：每 MB 源码
    pub per_src_mb: f64,
    /// 全程序链接（LTO 档位的可执行文件 crate）：基数；0 表示该档位无全程序链接期优化
    pub link_base_mb: f64,
    /// 全程序链接：每 MB 全工作区源码
    pub link_per_total_mb: f64,
}

impl Model {
    /// 各档位系数（服务器实测上包络，见计划文档 §六）
    pub fn of(p: BuildProfile) -> Model {
        match p {
            BuildProfile::Dev | BuildProfile::DevOpt => {
                Model { base_mb: 400.0, per_src_mb: 75.0, link_base_mb: 0.0, link_per_total_mb: 0.0 }
            }
            BuildProfile::Release => Model { base_mb: 500.0, per_src_mb: 90.0, link_base_mb: 1500.0, link_per_total_mb: 15.0 },
            BuildProfile::ReleaseMax | BuildProfile::ReleaseSmall => {
                Model { base_mb: 500.0, per_src_mb: 90.0, link_base_mb: 1500.0, link_per_total_mb: 60.0 }
            }
        }
    }

    pub fn crate_est(&self, src_bytes: u64) -> u64 {
        (self.base_mb + self.per_src_mb * mb(src_bytes)) as u64
    }

    /// 全程序链接估计（无全程序链接期优化的档位为 None）
    pub fn link_est(&self, total_bytes: u64) -> Option<u64> {
        (self.link_base_mb > 0.0).then(|| (self.link_base_mb + self.link_per_total_mb * mb(total_bytes)) as u64)
    }
}

fn mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

/// 工作区各 crate 的源码字节（成员目录 = 含 Cargo.toml 与 src/ 的子目录），按字节降序
pub fn crate_sizes(scratch: &Path) -> Vec<(String, u64)> {
    fn walk(d: &Path, acc: &mut u64) {
        let Ok(rd) = std::fs::read_dir(d) else { return };
        for e in rd.flatten() {
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_dir() {
                walk(&e.path(), acc);
            } else if e.path().extension().is_some_and(|x| x == "rs") {
                *acc += e.metadata().map(|m| m.len()).unwrap_or(0);
            }
        }
    }
    let mut v: Vec<(String, u64)> = std::fs::read_dir(scratch)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().join("Cargo.toml").is_file() && e.path().join("src").is_dir())
        .map(|e| {
            let mut n = 0;
            walk(&e.path().join("src"), &mut n);
            (e.file_name().to_string_lossy().into_owned(), n)
        })
        .collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    v
}

/// 作业数决定
#[derive(Debug, Clone, PartialEq)]
pub struct MemPlan {
    pub jobs: u32,
    /// 作业数上限（调用方 CARGO_BUILD_JOBS 或 CPU 核数）
    pub cap: u32,
    pub avail_mb: Option<u64>,
    pub source: Option<String>,
    pub budget_mb: Option<u64>,
    /// 最大单 crate 估计（crate, MB）
    pub top: Option<(String, u64)>,
    /// 全程序链接估计（MB）
    pub link_mb: Option<u64>,
    /// 单进程估计已超预算
    pub over: bool,
}

impl MemPlan {
    pub fn to_json(&self) -> Value {
        json!({
            "jobs": self.jobs, "cap": self.cap, "avail_mb": self.avail_mb, "source": self.source,
            "budget_mb": self.budget_mb, "top_crate": self.top.as_ref().map(|t| &t.0),
            "top_est_mb": self.top.as_ref().map(|t| t.1), "link_est_mb": self.link_mb, "over_budget": self.over,
        })
    }

    pub fn summary(&self) -> String {
        match (self.avail_mb, self.budget_mb) {
            (Some(a), Some(b)) => format!(
                "可用 {a} MB（{}）× {BUDGET_PCT}% = 预算 {b} MB；最大 crate {} 估 {} MB{}；作业数 {} / 上限 {}",
                self.source.as_deref().unwrap_or("?"),
                self.top.as_ref().map_or("-", |t| t.0.as_str()),
                self.top.as_ref().map_or(0, |t| t.1),
                self.link_mb.map(|l| format!("，全程序链接估 {l} MB")).unwrap_or_default(),
                self.jobs,
                self.cap
            ),
            _ => format!("未探测到可用内存：作业数 {} / 上限 {}", self.jobs, self.cap),
        }
    }
}

/// 规则第 4–6 条（纯函数）：`sizes` 为各 crate 源码字节（任意序），`bin_crate` 为可执行文件所在 crate
pub fn plan(sizes: &[(String, u64)], bin_crate: &str, model: Model, probe: Option<MemProbe>, cap: u32) -> MemPlan {
    let cap = cap.max(1);
    let total: u64 = sizes.iter().map(|s| s.1).sum();
    let link_mb = model.link_est(total);
    let mut ests: Vec<(String, u64)> = sizes
        .iter()
        .map(|(n, b)| {
            let e = model.crate_est(*b);
            // 全程序链接在可执行文件所在 crate 的 rustc 内进行
            (n.clone(), if n == bin_crate { e.max(link_mb.unwrap_or(0)) } else { e })
        })
        .collect();
    ests.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let top = ests.first().cloned();
    let Some(p) = probe else {
        return MemPlan { jobs: cap, cap, avail_mb: None, source: None, budget_mb: None, top, link_mb, over: false };
    };
    let budget = p.avail_mb * BUDGET_PCT / 100;
    let mut jobs = 1;
    let mut sum = 0;
    for (i, (_, e)) in ests.iter().enumerate().take(cap as usize) {
        sum += e;
        if sum > budget {
            break;
        }
        jobs = i as u32 + 1;
    }
    // 工作区 crate 少于上限时，多出的作业槽只给依赖（体量远小于工作区 crate），不受限
    if ests.len() < cap as usize && sum <= budget {
        jobs = cap;
    }
    let over = top.as_ref().is_some_and(|t| t.1 > budget);
    MemPlan { jobs, cap, avail_mb: Some(p.avail_mb), source: Some(p.source), budget_mb: Some(budget), top, link_mb, over }
}

/// 规则全程：探测可用内存、扫描工作区 crate 体量、按档位系数定作业数。
/// 可执行文件所在 crate 为发射层的用户 crate
pub fn decide(scratch: &Path, profile: BuildProfile) -> MemPlan {
    plan(&crate_sizes(scratch), emit::ctx::USER_CRATE, Model::of(profile), crate::mem_probe::probe(), job_cap())
}

/// 作业数上限：调用方 CARGO_BUILD_JOBS（正整数）否则 CPU 核数
pub fn job_cap() -> u32 {
    std::env::var("CARGO_BUILD_JOBS")
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|j| *j > 0)
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, |n| n.get() as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: u64 = 1024 * 1024;

    fn probe(mb: u64) -> Option<MemProbe> {
        Some(MemProbe { avail_mb: mb, source: "test".into() })
    }

    fn model() -> Model {
        Model { base_mb: 500.0, per_src_mb: 100.0, link_base_mb: 1000.0, link_per_total_mb: 10.0 }
    }

    #[test]
    fn jobs_fit_largest_crates_in_budget() {
        // 估计：decl 500+100*30=3500，body 500+100*10=1500 ×3，user 500+0 → max(link 1000+10*60=1600)
        let sizes: Vec<(String, u64)> = vec![
            ("decl".into(), 30 * M),
            ("b1".into(), 10 * M),
            ("b2".into(), 10 * M),
            ("b3".into(), 10 * M),
            ("user".into(), 0),
        ];
        // 预算 10000×85% = 8500：3500+1600+1500 = 6600 ≤ 8500，再加 1500 = 8100，再加 1500 = 9600 > 8500
        let p = plan(&sizes, "user", model(), probe(10_000), 8);
        assert_eq!((p.jobs, p.budget_mb, p.link_mb, p.over), (4, Some(8500), Some(1600), false));
        assert_eq!(p.top, Some(("decl".into(), 3500)));
        // 调用方上限更小时取上限
        assert_eq!(plan(&sizes, "user", model(), probe(10_000), 2).jobs, 2);
        // 预算不足最大 crate：1 作业并标超预算
        let p = plan(&sizes, "user", model(), probe(4000), 8);
        assert_eq!((p.jobs, p.over), (1, true));
        // 探测不到内存：不收紧
        assert_eq!(plan(&sizes, "user", model(), None, 6).jobs, 6);
    }

    #[test]
    fn few_crates_leave_cap_for_dependencies() {
        let sizes = vec![("decl".into(), M), ("user".into(), 0)];
        assert_eq!(plan(&sizes, "user", model(), probe(64_000), 8).jobs, 8);
    }

    #[test]
    fn link_only_for_lto_profiles() {
        assert!(Model::of(BuildProfile::Dev).link_est(100 * M).is_none());
        assert!(Model::of(BuildProfile::Release).link_est(100 * M).is_some());
    }
}
