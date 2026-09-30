//! runtime/ 手写层 overlay 进 scratch（`scripts/main.py prepare_scratch` 的移植）。
//!
//! - `runtime/java_runtime/src/**` → `<scratch>/java_runtime/src/**`（内容相同跳过，保留 mtime）；
//! - 根 `lib.rs` 不在此复制：由 [`super::mod_tree::complete_lib_rs`] 写出（手写真源 + 顶层包补全）；
//! - scratch 中 runtime/ 已删除的手写文件在 mod 树阶段清扫（[`super::mod_tree`] `sweep_stale`：
//!   须在本轮写出之后判定，否则本轮生成的无标记文件会被先删后写）；
//! - 构建脚本（crate 根的 `build.rs` 及其子文件 `build_*.rs`）原样复制；`Cargo.toml` 宏依赖改绝对路径、包版本唯一化；
//! - `java/ jdk/ sun/` 顶层目录兜底占位 mod.rs。

use std::path::Path;

use super::fs::walk;
use crate::error::{io_err, Result};
use crate::text::scratch_pkg_version;

const EMPTY_PKG_MOD: &str = "// 空包模块（overlay）：本包无生成类时 lib.rs 的\n\
// `pub mod` 声明仍需可解析；有生成类时被 codegen 覆写。\n";

fn copy_if_changed(src: &Path, dst: &Path) -> Result<()> {
    let data = std::fs::read(src).map_err(|e| io_err(&src.display().to_string(), e))?;
    if std::fs::read(dst).is_ok_and(|old| old == data) {
        return Ok(());
    }
    if let Some(d) = dst.parent() {
        std::fs::create_dir_all(d).map_err(|e| io_err(&d.display().to_string(), e))?;
    }
    std::fs::write(dst, data).map_err(|e| io_err(&dst.display().to_string(), e))
}

fn write_if_changed(path: &Path, content: &str) -> Result<()> {
    if std::fs::read(path).is_ok_and(|old| old == content.as_bytes()) {
        return Ok(());
    }
    std::fs::write(path, content).map_err(|e| io_err(&path.display().to_string(), e))
}

/// overlay 规则：`clean` 时先清空 scratch
pub fn prepare_scratch(out_dir: &Path, runtime_dir: &Path, macros_crate: &Path, clean: bool) -> Result<()> {
    if clean && out_dir.is_dir() {
        std::fs::remove_dir_all(out_dir).map_err(|e| io_err(&out_dir.display().to_string(), e))?;
    }
    let rt_src = runtime_dir.join("src");
    let dst_src = out_dir.join("java_runtime").join("src");
    for (dir, _, files) in walk(&rt_src) {
        let rel = dir.strip_prefix(&rt_src).unwrap_or(Path::new(""));
        let dst_dir = dst_src.join(rel);
        std::fs::create_dir_all(&dst_dir).map_err(|e| io_err(&dst_dir.display().to_string(), e))?;
        for f in files {
            // crate 根 lib.rs 由 mod 树阶段按「手写真源 + 顶层包补全」整体写出（内容不变不重写）
            if rel.as_os_str().is_empty() && f == "lib.rs" {
                continue;
            }
            copy_if_changed(&dir.join(&f), &dst_dir.join(&f))?;
        }
    }
    let jrt = out_dir.join("java_runtime");
    // 构建脚本：crate 根的全部 .rs（build.rs 及其 `mod` 子文件，如 build_closure.rs）
    let root_rs = std::fs::read_dir(runtime_dir).map_err(|e| io_err(&runtime_dir.display().to_string(), e))?;
    let mut scripts: Vec<std::path::PathBuf> = root_rs
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "rs"))
        .collect();
    scripts.sort();
    for p in scripts {
        let name = p.file_name().unwrap_or_default();
        copy_if_changed(&p, &jrt.join(name))?;
    }
    let cargo_src = runtime_dir.join("Cargo.toml");
    let cargo = std::fs::read_to_string(&cargo_src).map_err(|e| io_err(&cargo_src.display().to_string(), e))?;
    let cargo = cargo
        .replace("path = \"../rava_macros\"", &format!("path = \"{}\"", macros_crate.display()))
        .replace("version = \"0.1.0\"", &format!("version = \"{}\"", scratch_pkg_version(out_dir)));
    write_if_changed(&jrt.join("Cargo.toml"), &cargo)?;
    for pkg in ["java", "jdk", "sun"] {
        let d = dst_src.join(pkg);
        std::fs::create_dir_all(&d).map_err(|e| io_err(&d.display().to_string(), e))?;
        let m = d.join("mod.rs");
        if !m.exists() {
            std::fs::write(&m, EMPTY_PKG_MOD).map_err(|e| io_err(&m.display().to_string(), e))?;
        }
    }
    Ok(())
}
