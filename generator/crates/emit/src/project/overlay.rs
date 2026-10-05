//! runtime/ 手写层 overlay 进 scratch。
//!
//! - `runtime/java_runtime/src/**` → 各模块 crate 的 `src/**`（按包所属模块，根模块 → 声明层 crate；见 [`JdkDirs`]；
//!   内容相同跳过，保留 mtime）；
//! - 根 `lib.rs` 不在此复制：由 [`super::mod_tree::complete_lib_rs`] 写出（手写真源 + 顶层包补全）；
//! - 与 runtime/ 同步（runtime/ 是唯一真源）：本轮落盘的手写文件清单记入 scratch 根的 [`LEDGER`]；
//!   上轮清单中本轮不再落盘的路径（runtime/ 中删除 / 改名、目录改定向到别的 crate）若无生成标记即删除，
//!   删后空目录自底向上移除——覆盖 `.rs` 之外的资源文件与手写 `mod.rs`；有生成标记的文件从不删除。
//!   mod 树阶段另有兜底清扫（[`super::mod_tree`] `sweep_stale`：无清单的旧 scratch 中 runtime/ 已无的
//!   无标记 `.rs`）；
//! - `build.rs` 原样复制进声明层；`Cargo.toml` 包名改为声明层 crate 名，兄弟 crate（`rava_macros` / `rava_coro`）依赖改绝对路径、包版本唯一化；
//! - `java/ jdk/ sun/` 顶层目录兜底占位 mod.rs；
//! - `runtime/java_meta/`（反射元数据表 crate，全部手写、无生成文件）整体镜像到
//!   `<scratch>/java_meta/`：包版本唯一化、运行时依赖改指声明层，scratch 中真源已无的文件删除。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use closure::handwritten::layout::helper_root;

use super::fs::{has_marker, walk};
use crate::ctx::EmitShared;
use crate::error::{io_err, Result};
use crate::module_crates::crate_name;
use crate::text::scratch_pkg_version;

/// overlay 清单文件（scratch 根下）：上轮落盘的手写文件路径（相对 scratch 根，`/` 分隔，每行一条）
pub const LEDGER: &str = ".rava_overlay";

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

/// 手写真源落入 scratch 的目录表：手写文件按所在包的模块落到对应模块 crate 的源码树
/// （根模块 → 声明层 crate），本程序没有的模块的手写文件不落盘
#[derive(Debug, Clone)]
pub struct JdkDirs {
    /// 根模块声明层 crate 目录名（crate 根手写基础设施、`build.rs`、`Cargo.toml` 所在）
    pub decl: String,
    /// 手写目录（相对 `src`，`/` 分隔）→ 落入的 crate 目录名（None = 本程序无其模块，跳过）
    dirs: BTreeMap<String, Option<String>>,
}

impl JdkDirs {
    /// 全部落入一个 crate（无模块图）
    pub fn single(decl: &str) -> JdkDirs {
        JdkDirs { decl: decl.to_string(), dirs: BTreeMap::new() }
    }

    /// 显式目录表（单元测试：手写目录改定向到别的 crate）
    #[cfg(test)]
    pub(super) fn with_dirs(decl: &str, dirs: &[(&str, Option<&str>)]) -> JdkDirs {
        let dirs = dirs.iter().map(|(d, t)| (d.to_string(), t.map(str::to_string))).collect();
        JdkDirs { decl: decl.to_string(), dirs }
    }

    /// 由模块 crate 表与模块图：各手写目录按包归属定向
    pub fn of(ctx: &EmitShared<'_>) -> JdkDirs {
        let crates = ctx.crates();
        let g = ctx.modules();
        let decl = crates.decl();
        let rt_src = ctx.runtime_src();
        let mut dirs = BTreeMap::new();
        for (dir, _, _) in walk(&rt_src) {
            let rel_of = |d: &Path| d.strip_prefix(&rt_src).unwrap_or(Path::new("")).to_string_lossy().replace('\\', "/");
            let rel = rel_of(&dir);
            // 私有辅助目录不是包：随宿主文件所在的包定向
            let pkg = match helper_root(&rt_src, &dir) {
                Some(h) => rel_of(h.parent().unwrap_or(&rt_src)),
                None => rel.clone(),
            };
            let target = match crates.crate_of_package(&pkg) {
                Some(c) => Some(crates.dir_of(c)),
                None => match g.package_module(&pkg).map(crate_name) {
                    Some(c) if c != crates.root() => crates.contains(&c).then(|| crates.dir_of(&c)),
                    _ => Some(decl.clone()),
                },
            };
            dirs.insert(rel, target);
        }
        JdkDirs { decl, dirs }
    }

    /// 手写目录（相对 `src`）落入的 crate 目录名；None = 跳过
    fn target(&self, rel: &str) -> Option<&str> {
        match self.dirs.get(rel) {
            Some(t) => t.as_deref(),
            None => Some(&self.decl),
        }
    }
}

/// overlay 规则：`clean` 时先清空 scratch
pub fn prepare_scratch(out_dir: &Path, runtime_dir: &Path, macros_crate: &Path, dirs: &JdkDirs, clean: bool) -> Result<()> {
    if clean && out_dir.is_dir() {
        std::fs::remove_dir_all(out_dir).map_err(|e| io_err(&out_dir.display().to_string(), e))?;
    }
    let rt_src = runtime_dir.join("src");
    let decl = out_dir.join(&dirs.decl);
    let decl_src = decl.join("src");
    let mut placed = BTreeSet::new();
    for (dir, _, files) in walk(&rt_src) {
        let rel = dir.strip_prefix(&rt_src).unwrap_or(Path::new(""));
        let Some(target) = dirs.target(&rel.to_string_lossy().replace('\\', "/")) else { continue };
        let dst_dir = out_dir.join(target).join("src").join(rel);
        std::fs::create_dir_all(&dst_dir).map_err(|e| io_err(&dst_dir.display().to_string(), e))?;
        for f in files {
            // crate 根 lib.rs 由 mod 树阶段按「手写真源 + 顶层包补全」整体写出（内容不变不重写）
            if rel.as_os_str().is_empty() && f == "lib.rs" {
                continue;
            }
            copy_if_changed(&dir.join(&f), &dst_dir.join(&f))?;
            placed.insert(Path::new(target).join("src").join(rel).join(&f).to_string_lossy().replace('\\', "/"));
        }
    }
    sync_ledger(out_dir, &placed)?;
    copy_if_changed(&runtime_dir.join("build.rs"), &decl.join("build.rs"))?;
    let cargo_src = runtime_dir.join("Cargo.toml");
    let cargo = std::fs::read_to_string(&cargo_src).map_err(|e| io_err(&cargo_src.display().to_string(), e))?;
    // runtime/ 下的兄弟 crate（rava_macros、rava_coro）不复制进 scratch，以绝对路径依赖（共享 target 缓存命中）
    let siblings = macros_crate.parent().unwrap_or(Path::new(""));
    let cargo = rename_package(&cargo, &dirs.decl)
        .replace("path = \"../rava_macros\"", &format!("path = \"{}\"", macros_crate.display()))
        .replace("path = \"../rava_coro\"", &format!("path = \"{}\"", siblings.join("rava_coro").display()))
        .replace("version = \"0.1.0\"", &format!("version = \"{}\"", scratch_pkg_version(out_dir)));
    write_if_changed(&decl.join("Cargo.toml"), &cargo)?;
    for pkg in ["java", "jdk", "sun"] {
        let d = decl_src.join(pkg);
        std::fs::create_dir_all(&d).map_err(|e| io_err(&d.display().to_string(), e))?;
        let m = d.join("mod.rs");
        if !m.exists() {
            std::fs::write(&m, EMPTY_PKG_MOD).map_err(|e| io_err(&m.display().to_string(), e))?;
        }
    }
    let meta_src = runtime_dir.parent().unwrap_or(Path::new("")).join("java_meta");
    mirror_meta_crate(&meta_src, &out_dir.join("java_meta"), &scratch_pkg_version(out_dir), &dirs.decl)
}

/// 按 overlay 清单与 runtime/ 同步：上轮清单中本轮未落盘、且无生成标记的文件删除（其后空目录自底向上
/// 移除，止于 scratch 根），再写本轮清单。清单条目须是 scratch 内的相对路径（含 `..` / 绝对路径的行忽略）
fn sync_ledger(out_dir: &Path, placed: &BTreeSet<String>) -> Result<()> {
    let ledger = out_dir.join(LEDGER);
    let previous = std::fs::read_to_string(&ledger).unwrap_or_default();
    let mut emptied: Vec<PathBuf> = Vec::new();
    for rel in previous.lines().map(str::trim).filter(|l| !l.is_empty() && !placed.contains(*l)) {
        let rel_path = Path::new(rel);
        if !rel_path.components().all(|c| matches!(c, Component::Normal(_))) {
            continue;
        }
        let p = out_dir.join(rel_path);
        // 不可读（已不存在）或带生成标记（同路径已由生成器接管）→ 不动
        if p.is_file() && has_marker(&p) == Some(false) {
            std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            emptied.extend(p.parent().map(Path::to_path_buf));
        }
    }
    for start in emptied {
        let mut d = start;
        while d != out_dir && d.starts_with(out_dir) {
            let empty = std::fs::read_dir(&d).map(|mut it| it.next().is_none()).unwrap_or(false);
            if !empty {
                break;
            }
            std::fs::remove_dir(&d).map_err(|e| io_err(&d.display().to_string(), e))?;
            d.pop();
        }
    }
    let mut text = String::new();
    for rel in placed {
        text.push_str(rel);
        text.push('\n');
    }
    std::fs::create_dir_all(out_dir).map_err(|e| io_err(&out_dir.display().to_string(), e))?;
    write_if_changed(&ledger, &text)
}

/// 手写真源 `Cargo.toml` 的 `[package]` / `[lib]` 名改为声明层 crate 名
fn rename_package(cargo: &str, name: &str) -> String {
    let mut section = "";
    let mut out = String::with_capacity(cargo.len());
    for line in cargo.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t;
        }
        if matches!(section, "[package]" | "[lib]") && t.starts_with("name") && t[4..].trim_start().starts_with('=') {
            out.push_str(&format!("name = \"{name}\""));
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// java_meta 的 `Cargo.toml`：包版本唯一化；对运行时的依赖（真源写作 `path = "../<真源 crate>"`）改指
/// 声明层 crate（保留依赖名：本 crate 源码不随声明层 crate 名变化）
fn meta_cargo(text: &str, version: &str, decl: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let line = line.replace("version = \"0.1.0\"", &format!("version = \"{version}\""));
        match line.split_once(" = { path = \"../") {
            Some((dep, _)) if !line.trim_start().starts_with('#') => {
                out.push_str(&format!("{dep} = {{ package = \"{decl}\", path = \"../{decl}\" }}"));
            }
            _ => out.push_str(&line),
        }
        out.push('\n');
    }
    out
}

/// `runtime/java_meta` → `<scratch>/java_meta`：逐文件复制（内容相同跳过），`Cargo.toml` 包版本
/// 唯一化；目标侧真源已无的文件删除（本 crate 无生成文件，镜像即真源）
fn mirror_meta_crate(src: &Path, dst: &Path, version: &str, decl: &str) -> Result<()> {
    if !src.join("Cargo.toml").is_file() {
        return Err(io_err(
            &src.display().to_string(),
            std::io::Error::new(std::io::ErrorKind::NotFound, "java_meta crate 缺失"),
        ));
    }
    let mut kept = std::collections::BTreeSet::new();
    for (dir, _, files) in walk(src) {
        let rel = dir.strip_prefix(src).unwrap_or(Path::new(""));
        for f in files {
            let (from, to) = (dir.join(&f), dst.join(rel).join(&f));
            if rel.as_os_str().is_empty() && f == "Cargo.toml" {
                let text = std::fs::read_to_string(&from).map_err(|e| io_err(&from.display().to_string(), e))?;
                std::fs::create_dir_all(dst).map_err(|e| io_err(&dst.display().to_string(), e))?;
                write_if_changed(&to, &meta_cargo(&text, version, decl))?;
            } else {
                copy_if_changed(&from, &to)?;
            }
            kept.insert(to);
        }
    }
    for (dir, _, files) in walk(dst) {
        for f in files {
            let p = dir.join(&f);
            if !kept.contains(&p) {
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
    }
    // 真源已无的目录：删文件后剩下的空目录自底向上移除
    let mut dirs: Vec<_> = walk(dst).into_iter().map(|(d, _, _)| d).filter(|d| d != dst).collect();
    dirs.sort_by_key(|d| std::cmp::Reverse(d.components().count()));
    for d in dirs {
        let empty = std::fs::read_dir(&d).map(|mut it| it.next().is_none()).unwrap_or(false);
        if empty {
            std::fs::remove_dir(&d).map_err(|e| io_err(&d.display().to_string(), e))?;
        }
    }
    Ok(())
}
