//! 构建可用内存探测（内存感知作业数的输入，规则见 [`crate::mem_budget`]）。
//!
//! 可用内存 = min(系统可用, 本进程所在各级 cgroup 的余量)：
//! - 显式给定：环境变量 `RAVA_BUILD_MEM_MB`（MB，整数）优先于探测；
//! - Linux：`/proc/meminfo` 的 `MemAvailable`；cgroup v2 自本进程 cgroup 逐级上行到根，每级有
//!   `memory.max` 上限时取 上限 − (memory.current − inactive_file)（不活跃文件页可回收，不算占用）；
//!   cgroup v1 取 memory 层级的 limit_in_bytes − (usage_in_bytes − total_inactive_file)；
//! - macOS：`vm_stat` 的 free + inactive + speculative + purgeable 页 × 页大小；
//! - 都探测不到时返回 None（调用方不按内存收紧作业数）。

use std::path::{Path, PathBuf};
use std::process::Command;

/// 显式给定可用内存（MB）的环境变量
pub const MEM_ENV: &str = "RAVA_BUILD_MEM_MB";

/// 探测结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemProbe {
    /// 可用内存（MB）
    pub avail_mb: u64,
    /// 取值来源（日志与 build_status.json）
    pub source: String,
}

const MB: u64 = 1024 * 1024;

pub fn probe() -> Option<MemProbe> {
    if let Some(v) = std::env::var(MEM_ENV).ok().and_then(|v| v.trim().parse::<u64>().ok()) {
        return Some(MemProbe { avail_mb: v, source: MEM_ENV.to_string() });
    }
    let mut best: Option<MemProbe> = None;
    let mut take = |mb: u64, source: String| {
        if best.as_ref().is_none_or(|b| mb < b.avail_mb) {
            best = Some(MemProbe { avail_mb: mb, source });
        }
    };
    if let Some(kb) = std::fs::read_to_string("/proc/meminfo").ok().as_deref().and_then(|t| meminfo_available_kb(t)) {
        take(kb / 1024, "MemAvailable".into());
    }
    if let Some((mb, at)) = cgroup_v2_headroom() {
        take(mb, format!("cgroup v2 {at}"));
    }
    if let Some(mb) = cgroup_v1_headroom() {
        take(mb, "cgroup v1".into());
    }
    if cfg!(target_os = "macos") {
        if let Some(mb) = Command::new("vm_stat").output().ok().and_then(|o| vm_stat_available(&String::from_utf8_lossy(&o.stdout))) {
            take(mb / MB, "vm_stat".into());
        }
    }
    best
}

/// `/proc/meminfo` → MemAvailable（kB）
fn meminfo_available_kb(text: &str) -> Option<u64> {
    text.lines().find_map(|l| l.strip_prefix("MemAvailable:")).and_then(|v| v.split_whitespace().next()?.parse().ok())
}

/// `memory.stat` 中某键的值
fn stat_value(text: &str, key: &str) -> Option<u64> {
    text.lines().find_map(|l| {
        let mut it = l.split_whitespace();
        (it.next() == Some(key)).then(|| it.next()?.parse().ok()).flatten()
    })
}

fn read_u64(p: &Path) -> Option<u64> {
    std::fs::read_to_string(p).ok()?.trim().parse().ok()
}

/// 一级 cgroup 的余量（字节）：上限 − (当前 − 不活跃文件页)；无上限（"max"）为 None
fn level_headroom(max: Option<u64>, current: u64, inactive_file: u64) -> Option<u64> {
    Some(max?.saturating_sub(current.saturating_sub(inactive_file)))
}

/// cgroup v2：自本进程 cgroup 逐级上行，取各级余量最小者（MB, 所在层级）
fn cgroup_v2_headroom() -> Option<(u64, String)> {
    let text = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let rel = text.lines().find_map(|l| l.strip_prefix("0::"))?.trim_start_matches('/').to_string();
    let root = PathBuf::from("/sys/fs/cgroup");
    let mut dir = root.join(&rel);
    let mut best: Option<(u64, String)> = None;
    loop {
        let max = read_u64(&dir.join("memory.max"));
        if let (Some(cur), true) = (read_u64(&dir.join("memory.current")), max.is_some()) {
            let inactive = std::fs::read_to_string(dir.join("memory.stat")).ok().and_then(|s| stat_value(&s, "inactive_file")).unwrap_or(0);
            if let Some(h) = level_headroom(max, cur, inactive) {
                if best.as_ref().is_none_or(|b| h / MB < b.0) {
                    let at = dir.strip_prefix(&root).map(|p| format!("/{}", p.display())).unwrap_or_default();
                    best = Some((h / MB, at));
                }
            }
        }
        if dir == root || !dir.pop() || !dir.starts_with(&root) {
            break;
        }
    }
    best
}

/// cgroup v1：memory 层级中本进程 cgroup 的余量（MB）；上限为「无限」（≥ 2^60）视为无
fn cgroup_v1_headroom() -> Option<u64> {
    let text = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let rel = text.lines().find_map(|l| {
        let mut it = l.splitn(3, ':');
        let (_, ctrl, path) = (it.next()?, it.next()?, it.next()?);
        ctrl.split(',').any(|c| c == "memory").then(|| path.trim_start_matches('/').to_string())
    })?;
    let dir = Path::new("/sys/fs/cgroup/memory").join(rel);
    let limit = read_u64(&dir.join("memory.limit_in_bytes")).filter(|l| *l < (1 << 60))?;
    let usage = read_u64(&dir.join("memory.usage_in_bytes"))?;
    let inactive = std::fs::read_to_string(dir.join("memory.stat")).ok().and_then(|s| stat_value(&s, "total_inactive_file")).unwrap_or(0);
    level_headroom(Some(limit), usage, inactive).map(|b| b / MB)
}

/// `vm_stat` 输出 → 可用字节（free + inactive + speculative + purgeable）
fn vm_stat_available(text: &str) -> Option<u64> {
    let page: u64 = text.lines().next()?.split("page size of ").nth(1)?.split_whitespace().next()?.parse().ok()?;
    let pages = |key: &str| -> u64 {
        text.lines()
            .find_map(|l| l.strip_prefix(key))
            .and_then(|v| v.trim().trim_end_matches('.').parse().ok())
            .unwrap_or(0)
    };
    let total = pages("Pages free:") + pages("Pages inactive:") + pages("Pages speculative:") + pages("Pages purgeable:");
    (total > 0).then_some(total * page)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_meminfo_and_stat() {
        let mi = "MemTotal:       16303420 kB\nMemFree:          812344 kB\nMemAvailable:   12176400 kB\n";
        assert_eq!(meminfo_available_kb(mi), Some(12_176_400));
        let st = "anon 1000\nfile 5000\nactive_file 3000\ninactive_file 2000\n";
        assert_eq!(stat_value(st, "inactive_file"), Some(2000));
        assert_eq!(stat_value(st, "anon_thp"), None);
    }

    #[test]
    fn headroom_discounts_inactive_file() {
        assert_eq!(level_headroom(Some(12 * MB), 5 * MB, 2 * MB), Some(9 * MB));
        assert_eq!(level_headroom(Some(4 * MB), 9 * MB, 0), Some(0), "超限按 0");
        assert_eq!(level_headroom(None, 1, 0), None, "无上限");
    }

    #[test]
    fn parses_vm_stat() {
        let t = "Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:                               10000.\n\
                 Pages active:                            500000.\nPages inactive:                           20000.\n\
                 Pages speculative:                         3000.\nPages purgeable:                          1000.\n";
        assert_eq!(vm_stat_available(t), Some(34_000 * 16384));
    }
}
