//! 闭包分析的跨运行结果缓存（计划 2026-09-30-closure-analyzer-performance.md P6、§4.8）。
//!
//! 缓存的是整份分析产物（`Closure::to_json()` + 分析期诊断行），键是分析的全部输入的指纹：
//!
//! - 格式：条目格式版本 [`store::FORMAT`]、closure.json 折叠点版本 [`crate::FOLDS_VERSION`]；
//! - 分析器：当前可执行文件的内容（任何代码改动都换键）；
//! - 类路径：全部档案（用户类 / 依赖 jar / jmod / 镜像与 VM 支持类目录）按加入序的来源角色、路径与内容；
//! - 手写层：`runtime_dir` 下全部文件（清单 TOML、手写源、缩写表）的相对路径与内容；
//! - 分析参数：[`Input`] 的每个字段（解构穷举——新增字段不进键即编译失败）、内部表哈希初值、
//!   命令行追加的放行项（`Manifest::cli_overrides`）。
//!
//! 分析器不读环境变量；JDK 位置等环境项都经命令行化为上面的档案路径。命中时产物按构造与冷算逐字节相同
//! （`summary.elapsed_ms` / `summary.perf` 两个计时字段除外，见 [`mark_hit`]）。

pub mod hash;
pub mod store;
#[cfg(test)]
mod tests;

use std::path::Path;

use serde_json::{json, Value};

use crate::manifest::Manifest;
use crate::Input;
use hash::Fp;
pub use store::{Entry, Load, Store};

/// 缺省总量上限（MB，`--closure-cache-max-mb`）
pub const DEFAULT_MAX_MB: u64 = 4096;

/// 分析输入的指纹（32 位十六进制）
pub fn key(input: &Input<'_>, man: &Manifest) -> Result<String, String> {
    let exe = std::env::current_exe().map_err(|e| format!("定位分析器可执行文件：{e}"))?;
    let exe = std::fs::read(&exe).map_err(|e| format!("{}：{e}", exe.display()))?;
    key_with(&exe, input, man.cli_overrides())
}

/// [`key`] 的主体：分析器身份与命令行放行项由调用方给出（单测用）
pub fn key_with(analyzer: &[u8], input: &Input<'_>, overrides: (&[String], &[String])) -> Result<String, String> {
    let Input { cp, runtime_dir, roots, seed_roots, locales, cold_cut, flow_batch } = input;
    let mut f = Fp::default();
    f.field("format", &store::FORMAT.to_le_bytes());
    f.field("folds_version", &crate::FOLDS_VERSION.to_le_bytes());
    f.field("analyzer", analyzer);
    for (origin, path) in cp.archives() {
        f.field("archive", format!("{origin:?} {}", path.display()).as_bytes());
        tree(&mut f, &path)?;
    }
    f.field("runtime", runtime_dir.display().to_string().as_bytes());
    tree(&mut f, runtime_dir)?;
    for r in roots {
        f.field("root", r.to_string().as_bytes());
    }
    for r in seed_roots {
        f.field("seed_root", r.to_string().as_bytes());
    }
    for l in locales {
        f.field("locale", l.as_bytes());
    }
    f.field("cold_cut", &[*cold_cut as u8]);
    let batch = flow_batch.filter(|&n| n > 0).unwrap_or(crate::engine::FLOW_BATCH);
    f.field("flow_batch", &(batch as u64).to_le_bytes());
    f.field("hash_seed", &crate::engine::hash_seed().to_le_bytes());
    let (release, dropped) = overrides;
    for r in release {
        f.field("release", r.as_bytes());
    }
    for r in dropped {
        f.field("release_bytecode", r.as_bytes());
    }
    Ok(f.hex())
}

/// 文件或目录树（相对路径排序）的内容并入指纹
fn tree(f: &mut Fp, root: &Path) -> Result<(), String> {
    let err = |p: &Path, e: std::io::Error| format!("{}：{e}", p.display());
    if root.is_file() {
        f.field("file", &std::fs::read(root).map_err(|e| err(root, e))?);
        return Ok(());
    }
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).map_err(|e| err(&d, e))? {
            let p = e.map_err(|e| err(&d, e))?.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                files.push(p);
            }
        }
    }
    files.sort();
    for p in files {
        let rel = p.strip_prefix(root).unwrap_or(&p);
        f.field("path", rel.to_string_lossy().as_bytes());
        f.field("data", &std::fs::read(&p).map_err(|e| err(&p, e))?);
    }
    Ok(())
}

/// 命中产物换上本次的计时字段：`summary.elapsed_ms` 为本次取用耗时，`summary.perf` 只记缓存取用
pub fn mark_hit(closure: &mut Value, elapsed_ms: u128, key_ms: u128) {
    if let Some(s) = closure.get_mut("summary").and_then(Value::as_object_mut) {
        s.insert("elapsed_ms".into(), json!(elapsed_ms));
        s.insert(
            "perf".into(),
            json!({"cache": {"hit": true, "key_ms": key_ms}, "peak_mem_mb": crate::engine::peak_mem_mb()}),
        );
    }
}
