//! `rava closure` / `rava build` 共用的闭包分析一步：查跨运行缓存（`closure::cache`），未命中则冷算并写回。
//!
//! 命中与冷算对外行为相同：分析期诊断行按原顺序打到标准错误，产物 `closure.json` 值逐字节相同
//! （计时字段 `summary.elapsed_ms` / `summary.perf` 除外）。需要引擎本体的诊断（provenance 链、报告、流查询）
//! 由调用方以 `need_engine` 声明，此时不读缓存（冷算结果仍写回）。

use std::path::PathBuf;
use std::time::Instant;

use closure::cache::{self, Entry, Load, Store};
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use closure::{Closure, Input};
use resolve::Hierarchy;
use serde_json::Value;

/// 缓存设置（命令行 `--closure-cache DIR` / `--closure-cache-max-mb N`）
pub struct CacheOpts {
    pub dir: Option<PathBuf>,
    pub max_mb: Option<u64>,
}

/// 分析结果：命中只有产物值；冷算另有引擎
pub struct Outcome<'a> {
    pub closure: Option<Closure<'a>>,
    /// `Closure::to_json()`（命中、启用缓存或 `want_json` 时有）
    pub json: Option<Value>,
}

/// 查缓存或冷算。`need_engine`：调用方要用引擎本体（不读缓存）；`want_json`：调用方要产物值
pub fn analyze<'a>(
    opts: &CacheOpts,
    input: &Input<'a>,
    h: &'a Hierarchy<'a>,
    man: &'a Manifest,
    hw: &'a Handwritten,
    need_engine: bool,
    want_json: bool,
) -> Outcome<'a> {
    let t0 = Instant::now();
    let slot = opts.dir.as_ref().and_then(|d| {
        let max = opts.max_mb.unwrap_or(cache::DEFAULT_MAX_MB).saturating_mul(1 << 20);
        let r = Store::open(d, max).and_then(|s| cache::key(input, man).map(|k| (s, k)));
        r.map_err(|e| eprintln!("[closure-cache] 停用：{e}")).ok()
    });
    let key_ms = t0.elapsed().as_millis();
    // 触发边转储是冷算的副产物：要转储时不读缓存
    let need_engine = need_engine || input.diag.dump_edges.is_some() || input.diag.site_prof;
    if let (Some((s, k)), false) = (&slot, need_engine) {
        match s.load(k) {
            Load::Hit(Entry { diag, closure: mut v }) => {
                for d in &diag {
                    eprintln!("{d}");
                }
                cache::mark_hit(&mut v, t0.elapsed().as_millis(), key_ms);
                eprintln!("[closure-cache] 命中 {k}（{} ms）", t0.elapsed().as_millis());
                return Outcome { closure: None, json: Some(v) };
            }
            Load::Corrupt(why) => eprintln!("[closure-cache] 条目损坏已删除，重算：{why}"),
            Load::Miss => {}
        }
    }
    let c = closure::analyze(input, h, man, hw);
    let mut diag: Vec<String> = hw.errors.borrow().iter().map(|e| format!("[closure] 手写文件解析失败：{e}")).collect();
    diag.extend(input.cp.failures().into_iter().map(|(n, e)| format!("[closure] 类解析失败：{n}：{e}")));
    for d in &diag {
        eprintln!("{d}");
    }
    let json = (slot.is_some() || want_json).then(|| c.to_json());
    if let (Some((s, k)), Some(v)) = (&slot, &json) {
        if let Err(e) = s.save_parts(k, &diag, v) {
            eprintln!("[closure-cache] 写入失败：{e}");
        }
    }
    Outcome { closure: Some(c), json }
}
