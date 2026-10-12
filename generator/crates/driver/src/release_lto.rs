//! release 档 LTO 形态选择（fat / thin）与工作区 Cargo.toml 的就地改写。
//!
//! 背景：`[profile.release]` 自 build-memsafe（batch-1011h）起为内存受限缺省 thin LTO（用户 10-06 定，
//! 16 GB 机器 cgroup 上限内全部标杆用例可构建），大机器打包档 `release-max` 保留 fat LTO。代价是跨 crate
//! 调用链（如 Unsafe 直接内存的逐层包装）在 thin LTO 下不再整链内联，热路径回退约 15%（kr1 DirectMemBench
//! 普通段 64→74 ms，二分结论见 docs/reports/2026-10-11-code-review-triage.md 五节）。
//!
//! 本模块在 `--release` 编译前按内存余量就地决定工作区 `[profile.release]` 的 lto 形态：
//! - `--release-lto fat|thin`（`rava build` / `rava compile`）：显式钉死（复现 / 二分用）；
//! - 缺省 auto：fat LTO 的全程序链接峰值估计（[`crate::mem_budget::Model::of(ReleaseMax)`]）不超过
//!   探测可用内存的 [`FAT_SHARE_PCT`]%（fat 链接是单进程峰值，作业数退半救不了，留出一半余量给
//!   页缓存、链接器与估计误差）才取 fat，否则保持 thin——10-06 的内存约束在内存不足的机器上原样成立；
//! - 实测 OOM 时降级 thin 重编（估计是上包络，兜底）。
//!
//! 只改 lto 一行：产物仍落 `target/release/`（调用方按档位目录找二进制），`--release-max` /
//! `--release-small` 显式档不受影响。选择结果打进 `build_status.json` 的 `release_lto` 字段。

use std::path::Path;

use crate::cargo::BuildProfile;
use crate::mem_budget::{crate_stats, Model};

/// `--release-lto` 的合法取值（缺省 auto）
pub const FLAG_VALUES: [&str; 3] = ["fat", "thin", "auto"];

/// auto 判 fat 的可用内存份额上限（百分比）：fat 链接估计 ≤ 可用 × 本值才取 fat
pub const FAT_SHARE_PCT: u64 = 50;

/// release 档的 LTO 形态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseLto {
    Fat,
    Thin,
}

impl ReleaseLto {
    fn name(self) -> &'static str {
        match self {
            ReleaseLto::Fat => "fat",
            ReleaseLto::Thin => "thin",
        }
    }
}

/// 决策结果：形态与理由（日志与 build_status.json）。`pinned` 为环境变量显式指定（OOM 不降级）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub mode: ReleaseLto,
    pub reason: String,
    pub pinned: bool,
}

/// 纯函数决策（便于单测）。`fat_est_mb` / `avail_mb` 缺失（探测不到）时 auto 取 thin（保守）。
pub fn decide(env_val: Option<&str>, fat_est_mb: Option<u64>, avail_mb: Option<u64>) -> Decision {
    let unknown = |what: &str| format!("auto：{what}不可得，保守取 thin");
    match env_val.unwrap_or("auto").trim().to_ascii_lowercase().as_str() {
        "fat" => Decision { mode: ReleaseLto::Fat, reason: "--release-lto fat（显式指定）".into(), pinned: true },
        "thin" => Decision { mode: ReleaseLto::Thin, reason: "--release-lto thin（显式指定）".into(), pinned: true },
        "auto" | "" => {
            let (Some(est), Some(avail)) = (fat_est_mb, avail_mb) else {
                return Decision { mode: ReleaseLto::Thin, reason: unknown(if fat_est_mb.is_none() { "工作区体量" } else { "可用内存" }), pinned: false };
            };
            let cap = avail * FAT_SHARE_PCT / 100;
            if est <= cap {
                Decision { mode: ReleaseLto::Fat, reason: format!("auto：fat 链接估 {est} MB ≤ 可用 {avail} MB × {FAT_SHARE_PCT}% = {cap} MB"), pinned: false }
            } else {
                Decision { mode: ReleaseLto::Thin, reason: format!("auto：fat 链接估 {est} MB > 可用 {avail} MB × {FAT_SHARE_PCT}% = {cap} MB，保持 thin"), pinned: false }
            }
        }
        bad => Decision { mode: ReleaseLto::Thin, reason: format!("--release-lto {bad} 无法识别（取值 fat|thin|auto），保守取 thin"), pinned: false },
    }
}

/// `--release` 档的完整决策：环境变量 + 工作区 fat 链接估计（按 [`Model::of(ReleaseMax)`]）+ 可用内存
pub fn decide_for_scratch(env_val: Option<&str>, scratch: &Path, avail_mb: Option<u64>) -> Decision {
    let total: u64 = crate_stats(scratch).iter().map(|s| s.total_bytes()).sum();
    decide(env_val, Model::of(BuildProfile::ReleaseMax).link_est(total), avail_mb)
}

/// 就地把工作区 Cargo.toml 的 `[profile.release]` lto 行改为目标形态。
/// 返回是否发生了改写（幂等：已是目标形态返回 false）。
/// 找不到 `[profile.release]` 段或段内没有可识别的 lto 行时报错（发射层格式变了应同步这里）。
pub fn patch_workspace(out: &Path, mode: ReleaseLto) -> Result<bool, String> {
    let path = out.join("Cargo.toml");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}：{e}", path.display()))?;
    let Some(start) = text.find("[profile.release]") else {
        return Err(format!("{}：无 [profile.release] 段", path.display()));
    };
    let body_start = start + "[profile.release]".len();
    let body_end = text[body_start..].find("\n[").map(|i| body_start + i + 1).unwrap_or(text.len());
    let section = &text[body_start..body_end];
    let thin_line = "lto       = \"thin\"";
    let fat_line = "lto       = true";
    let (from, to) = match mode {
        ReleaseLto::Fat => (thin_line, fat_line),
        ReleaseLto::Thin => (fat_line, thin_line),
    };
    if section.contains(to) {
        return Ok(false);
    }
    let Some(pos) = section.find(from) else {
        return Err(format!("{}：[profile.release] 内无 `{}` 行", path.display(), from));
    };
    let abs = body_start + pos;
    let patched = format!("{}{}{}", &text[..abs], to, &text[abs + from.len()..]);
    std::fs::write(&path, patched).map_err(|e| format!("{}：{e}", path.display()))?;
    Ok(true)
}

/// 决策的日志行
pub fn log_line(d: &Decision) -> String {
    format!("[lto] release 档 {} LTO：{}", d.mode.name(), d.reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_pins() {
        assert_eq!(decide(Some("fat"), None, None).mode, ReleaseLto::Fat);
        assert_eq!(decide(Some("thin"), Some(1), Some(1 << 40)).mode, ReleaseLto::Thin);
        assert_eq!(decide(Some("FAT"), None, None).mode, ReleaseLto::Fat, "大小写不敏感");
        assert_eq!(decide(Some(" fat "), None, None).mode, ReleaseLto::Fat, "首尾空白 tolerated");
        let bad = decide(Some("fatl"), None, None);
        assert_eq!((bad.mode, bad.reason.contains("fatl")), (ReleaseLto::Thin, true));
    }

    #[test]
    fn auto_needs_both_and_respects_share() {
        assert_eq!(decide(None, None, Some(1000)).mode, ReleaseLto::Thin, "无估计取 thin");
        assert_eq!(decide(None, Some(1000), None).mode, ReleaseLto::Thin, "无探测取 thin");
        assert_eq!(decide(None, Some(400), Some(1000)).mode, ReleaseLto::Fat, "400 ≤ 500");
        assert_eq!(decide(None, Some(501), Some(1000)).mode, ReleaseLto::Thin, "501 > 500");
        assert_eq!(decide(Some("auto"), Some(500), Some(1000)).mode, ReleaseLto::Fat, "恰在界上取 fat");
    }

    #[test]
    fn patches_only_release_section() {
        let tmp = std::env::temp_dir().join("rava_release_lto_test");
        std::fs::create_dir_all(&tmp).unwrap();
        let emit_shape = "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n\n\
             [profile.release]\nopt-level = 3\nlto       = \"thin\"\ncodegen-units = 1\n\n\
             [profile.release-max]\ninherits = \"release\"\nlto = true\ncodegen-units = 1\n";
        std::fs::write(tmp.join("Cargo.toml"), emit_shape).unwrap();
        assert!(patch_workspace(&tmp, ReleaseLto::Fat).unwrap(), "首次改写");
        let after = std::fs::read_to_string(tmp.join("Cargo.toml")).unwrap();
        assert!(after.contains("lto       = true\ncodegen-units = 1\n\n[profile.release-max]"), "只改 release 段");
        assert!(after.contains("[profile.release-max]\ninherits = \"release\"\nlto = true"), "release-max 段不动");
        assert!(!patch_workspace(&tmp, ReleaseLto::Fat).unwrap(), "幂等");
        assert!(patch_workspace(&tmp, ReleaseLto::Thin).unwrap(), "可降回 thin");
        assert!(std::fs::read_to_string(tmp.join("Cargo.toml")).unwrap().starts_with("[workspace]"));
        std::fs::remove_dir_all(&tmp).unwrap();
    }

    #[test]
    fn patch_reports_missing_shapes() {
        let tmp = std::env::temp_dir().join("rava_release_lto_test2");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(tmp.join("Cargo.toml"), "[workspace]\n").unwrap();
        assert!(patch_workspace(&tmp, ReleaseLto::Fat).is_err());
        std::fs::remove_dir_all(&tmp).unwrap();
    }
}
