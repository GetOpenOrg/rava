//! runtime/ 下的兄弟 crate 落入 scratch 的 `vendor/`：产物 workspace 自包含、可整体搬移。
//!
//! - 落入范围：`runtime/java_runtime/Cargo.toml` 中 `path = "../<名>"` 依赖的传递闭包（宏 crate、协程原语
//!   及其路径依赖），由清单内容决定，生成器不列 crate 名；
//! - scratch 内一律相对路径引用（`../vendor/<名>`），不再回指 rava 仓库的绝对路径；
//! - 共享 target 缓存：各 scratch 中 vendor 相对 workspace 根的位置相同，包版本取 `0.0.<内容摘要>`
//!   （同内容同版本，跨 scratch / 跨检出共用产物；内容不同即不同产物，不会误用陈旧产物），
//!   落盘文件的 mtime 取真源 mtime（新 scratch 的副本不比既有产物新，cargo 指纹判新鲜）；
//! - 与真源同步：vendor 下真源已无的文件与已不在闭包中的 crate 删除；`target/` 与 `Cargo.lock` 不落入。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::archive_side::{is_version_line, without_version, Fnv};
use super::fs::walk;
use crate::error::{io_err, EmitError, Result};

/// scratch 根下的 vendor 目录名
pub const VENDOR_DIR: &str = "vendor";

/// 宏 crate 名（生成文件以 `rava_macros::java_class!` 等引用）
pub const MACROS_CRATE: &str = "rava_macros";

/// scratch 内 crate（`<scratch>/<crate>/Cargo.toml`）对 vendor crate 的依赖行
pub fn vendored_dep(name: &str) -> String {
    format!("{name:<15} = {{ path = \"../{VENDOR_DIR}/{name}\" }}")
}

/// 宏 crate 依赖行
pub fn macros_dep() -> String {
    vendored_dep(MACROS_CRATE)
}

/// Cargo.toml 中 `path = "../<名>"` 形式的兄弟依赖名（`[dependencies]` / `[build-dependencies]`）
pub fn sibling_deps(cargo_toml: &str, file: &Path) -> Result<Vec<String>> {
    let t: toml::Table = cargo_toml.parse().map_err(|e| EmitError::Input(format!("{}：{e}", file.display())))?;
    let mut out = Vec::new();
    for sec in ["dependencies", "build-dependencies"] {
        let Some(deps) = t.get(sec).and_then(toml::Value::as_table) else { continue };
        for v in deps.values() {
            let Some(p) = v.get("path").and_then(toml::Value::as_str) else { continue };
            if let Some(name) = p.strip_prefix("../").filter(|n| !n.is_empty() && !n.contains(['/', '\\']) && *n != "..") {
                out.push(name.to_string());
            }
        }
    }
    Ok(out)
}

/// 把 `java_runtime` Cargo.toml 中兄弟依赖的相对路径改指 vendor
pub fn redirect_siblings(cargo_toml: &str, names: &[String]) -> String {
    names.iter().fold(cargo_toml.to_string(), |acc, n| {
        acc.replace(&format!("path = \"../{n}\""), &format!("path = \"../{VENDOR_DIR}/{n}\""))
    })
}

/// crate 目录下需落入的文件（相对路径，`/` 分隔，升序）：跳过 `target/` 与 `Cargo.lock`
fn crate_files(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for (d, _, files) in walk(dir) {
        let rel = d.strip_prefix(dir).unwrap_or(Path::new(""));
        if rel.components().next().is_some_and(|c| c.as_os_str() == "target") {
            continue;
        }
        for f in files {
            if rel.as_os_str().is_empty() && f == "Cargo.lock" {
                continue;
            }
            out.push(rel.join(&f).to_string_lossy().replace('\\', "/"));
        }
    }
    out.sort();
    out
}

/// 写入并把 mtime 置为真源 mtime（内容相同不重写）
fn put_with_mtime(src: &Path, dst: &Path, data: &[u8]) -> Result<()> {
    if std::fs::read(dst).is_ok_and(|old| old == data) {
        return Ok(());
    }
    if let Some(d) = dst.parent() {
        std::fs::create_dir_all(d).map_err(|e| io_err(&d.display().to_string(), e))?;
    }
    std::fs::write(dst, data).map_err(|e| io_err(&dst.display().to_string(), e))?;
    let mtime = std::fs::metadata(src).and_then(|m| m.modified()).map_err(|e| io_err(&src.display().to_string(), e))?;
    let f = std::fs::File::options().write(true).open(dst).map_err(|e| io_err(&dst.display().to_string(), e))?;
    f.set_modified(mtime).map_err(|e| io_err(&dst.display().to_string(), e))
}

/// 删除 `root` 下不在 `keep`（相对路径）中的文件，其后空目录自底向上移除
fn prune(root: &Path, keep: &BTreeSet<String>) -> Result<()> {
    let mut dirs = Vec::new();
    for (d, _, files) in walk(root) {
        let rel = d.strip_prefix(root).unwrap_or(Path::new("")).to_path_buf();
        for f in files {
            let r = rel.join(&f).to_string_lossy().replace('\\', "/");
            if !keep.contains(&r) {
                let p = d.join(&f);
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
        dirs.push(d);
    }
    for d in dirs.into_iter().rev() {
        if d != root {
            let _ = std::fs::remove_dir(&d); // 非空即保留
        }
    }
    Ok(())
}

/// 把 `runtime_dir`（`runtime/java_runtime`）的兄弟依赖闭包落入 `<out_dir>/vendor/`；返回落入的 crate 名（升序）
pub fn vendor_siblings(out_dir: &Path, runtime_dir: &Path) -> Result<Vec<String>> {
    let root = runtime_dir.parent().unwrap_or(Path::new(""));
    let cargo = runtime_dir.join("Cargo.toml");
    let text = std::fs::read_to_string(&cargo).map_err(|e| io_err(&cargo.display().to_string(), e))?;
    let mut queue = sibling_deps(&text, &cargo)?;
    let mut crates: BTreeMap<String, (PathBuf, Vec<String>)> = BTreeMap::new();
    while let Some(name) = queue.pop() {
        if crates.contains_key(&name) {
            continue;
        }
        let dir = root.join(&name);
        let manifest = dir.join("Cargo.toml");
        let text = std::fs::read_to_string(&manifest).map_err(|e| io_err(&manifest.display().to_string(), e))?;
        queue.extend(sibling_deps(&text, &manifest)?);
        let files = crate_files(&dir);
        crates.insert(name, (dir, files));
    }
    let vendor = out_dir.join(VENDOR_DIR);
    for (name, (dir, files)) in &crates {
        let mut h = Fnv::new();
        let mut contents = Vec::with_capacity(files.len());
        for rel in files {
            let p = dir.join(rel);
            let bytes = std::fs::read(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            h.add(rel.as_bytes());
            if rel == "Cargo.toml" {
                h.add(without_version(&String::from_utf8_lossy(&bytes)).as_bytes());
            } else {
                h.add(&bytes);
            }
            contents.push(bytes);
        }
        let version = format!("version = \"0.0.{}\"", h.0);
        let dst_root = vendor.join(name);
        for (rel, bytes) in files.iter().zip(contents) {
            let data = if rel == "Cargo.toml" {
                let text = String::from_utf8_lossy(&bytes);
                let mut stamped = false;
                let lines: Vec<String> = text
                    .lines()
                    .map(|l| {
                        if !stamped && is_version_line(l) {
                            stamped = true;
                            version.clone()
                        } else {
                            l.to_string()
                        }
                    })
                    .collect();
                (lines.join("\n") + "\n").into_bytes()
            } else {
                bytes
            };
            put_with_mtime(&dir.join(rel), &dst_root.join(rel), &data)?;
        }
        prune(&dst_root, &files.iter().cloned().collect())?;
    }
    if let Ok(rd) = std::fs::read_dir(&vendor) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().into_owned();
            if !crates.contains_key(&n) {
                let p = e.path();
                let r = if p.is_dir() { std::fs::remove_dir_all(&p) } else { std::fs::remove_file(&p) };
                r.map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
    }
    Ok(crates.into_keys().collect())
}
