//! 包 mod 树落盘（`project_writer._write_jdk_mod_tree` / `_complete_jrt_lib_rs` 的移植）。
//!
//! 类文件全部落盘后调用：mod.rs 如实声明磁盘上的全部模块（手写 overlay + 本轮生成 +
//! 同 scratch 上轮幸存）。本轮未写入的带生成标记文件先清除。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use ty::ident::is_rust_keyword;

use super::fs::{has_marker, walk, Writer};
use crate::error::{io_err, Result};

/// 共置手写文件的编译期硬依赖（同目录 sibling 缺席则该 impl 不声明）。
/// 过渡：Python `_IMPL_FILE_DEPS` 的逐项移植；终态应迁入 runtime 清单（生成器不写文件名特判）。
const IMPL_FILE_DEPS: &[(&str, &[&str])] = &[("method_handle_natives_impl.rs", &["member_name.rs", "method_type.rs"])];

fn mod_decl(name: &str) -> String {
    if is_rust_keyword(name) {
        format!("pub mod r#{name};")
    } else {
        format!("pub mod {name};")
    }
}

fn use_decl(name: &str) -> String {
    if is_rust_keyword(name) {
        format!("pub use r#{name}::*;")
    } else {
        format!("pub use {name}::*;")
    }
}

/// 本轮未写入、带生成标记的 .rs（lib.rs / mod.rs 除外）清除
fn sweep_stale(src_root: &Path, writer: &Writer) -> Result<()> {
    for (dir, _, files) in walk(src_root) {
        for f in files {
            if !f.ends_with(".rs") || f == "lib.rs" || f == "mod.rs" {
                continue;
            }
            let p = dir.join(&f);
            if writer.written_this_run(&p) {
                continue;
            }
            if has_marker(&p) == Some(true) {
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
    }
    Ok(())
}

/// 从磁盘收集 mod 树：目录 → 子模块名（文件 stem 与子目录名）
fn scan_tree(src_root: &Path, handwritten_src: Option<&Path>) -> BTreeMap<PathBuf, BTreeSet<String>> {
    let mut tree: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let mut skip_under: Vec<PathBuf> = Vec::new();
    for (dir, _, files) in walk(src_root) {
        if skip_under.iter().any(|s| dir.starts_with(s)) {
            continue;
        }
        let rel = dir.strip_prefix(src_root).unwrap_or(Path::new(""));
        // 手写模块目录（runtime/ 真源提供 mod.rs）：模块结构由手写 mod.rs 自行声明
        if let Some(hw) = handwritten_src {
            if !rel.as_os_str().is_empty() && hw.join(rel).join("mod.rs").is_file() {
                skip_under.push(dir.clone());
                continue;
            }
        }
        for f in files {
            if !f.ends_with(".rs") || f == "lib.rs" || f == "mod.rs" {
                continue;
            }
            // 共置手写文件由 companion 声明；含生成标记的是碰巧以 Impl/Ext 结尾的生成类
            if (f.ends_with("_impl.rs") || f.ends_with("_ext.rs")) && has_marker(&dir.join(&f)) != Some(true) {
                continue;
            }
            tree.entry(dir.clone()).or_default().insert(f[..f.len() - 3].to_string());
        }
    }
    // 自底向上传播目录
    loop {
        let mut added = false;
        let dirs: Vec<PathBuf> = tree.keys().cloned().collect();
        for d in dirs {
            if d == src_root {
                continue;
            }
            let (Some(par), Some(name)) = (d.parent(), d.file_name()) else { continue };
            let name = name.to_string_lossy().into_owned();
            let set = tree.entry(par.to_path_buf()).or_default();
            if set.insert(name) {
                added = true;
            }
        }
        if !added {
            break;
        }
    }
    tree
}

/// 目录的共置手写声明：(补充 pub 声明的宿主, `mod X_impl;` 行)
fn companions(dir: &Path, children: &BTreeSet<String>) -> (BTreeSet<String>, Vec<String>) {
    let mut extra_pub = BTreeSet::new();
    let mut mods = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else { return (extra_pub, mods) };
    let mut names: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    for f in names {
        let base = if let Some(b) = f.strip_suffix("_impl.rs") {
            b
        } else if let Some(b) = f.strip_suffix("_ext.rs") {
            b
        } else {
            continue;
        };
        let stem = &f[..f.len() - 3];
        if children.contains(stem) {
            continue;
        }
        if !(dir.join(format!("{base}.rs")).exists() || dir.join(format!("{base}_t.rs")).exists()) {
            continue;
        }
        let deps = IMPL_FILE_DEPS.iter().find(|(k, _)| *k == f).map_or(&[][..], |(_, d)| *d);
        if !deps.iter().all(|d| dir.join(d).exists()) {
            continue;
        }
        mods.push(format!("mod {stem};"));
        if !children.contains(base) {
            extra_pub.insert(base.to_string());
        }
    }
    (extra_pub, mods)
}

/// 重建 `src_root` 下各包的 mod.rs（根目录自身除外：lib.rs 手写）。
/// `handwritten_src`：手写真源 src（java_runtime 时给出，lib crate 为 None）
pub fn write_mod_tree(src_root: &Path, handwritten_src: Option<&Path>, writer: &mut Writer) -> Result<()> {
    sweep_stale(src_root, writer)?;
    if !src_root.is_dir() {
        return Ok(());
    }
    let tree = scan_tree(src_root, handwritten_src);
    for (dir, children) in &tree {
        if dir == src_root {
            continue;
        }
        let mut lines = vec!["#![allow(ambiguous_glob_reexports)]".to_string()];
        for c in children {
            lines.push(mod_decl(c));
            // 子包只声明 pub mod，不 glob 再导出（Java 包无嵌套可见性）
            if tree.contains_key(&dir.join(c)) {
                continue;
            }
            lines.push(use_decl(c));
        }
        let (extra_pub, comp) = companions(dir, children);
        for b in &extra_pub {
            lines.push(mod_decl(b));
            lines.push(use_decl(b));
        }
        lines.extend(comp);
        writer.write(&dir.join("mod.rs"), &(lines.join("\n") + "\n"))?;
    }
    Ok(())
}

fn is_identifier(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c == '_' || c.is_alphabetic()) && cs.all(|c| c == '_' || c.is_alphanumeric())
}

/// scratch `java_runtime/src/lib.rs` 顶层模块补全（jar 输入模式的新顶层包根）
pub fn complete_lib_rs(src_root: &Path) -> Result<()> {
    let lib_rs = src_root.join("lib.rs");
    let Ok(text) = std::fs::read_to_string(&lib_rs) else { return Ok(()) };
    let re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+(r#\s*)?(\w+)\s*;").expect("静态正则");
    let declared: BTreeSet<&str> = re.captures_iter(&text).filter_map(|c| c.get(2)).map(|m| m.as_str()).collect();
    let Ok(rd) = std::fs::read_dir(src_root) else { return Ok(()) };
    let mut names: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    let missing: Vec<String> = names
        .iter()
        .filter(|n| !declared.contains(n.as_str()) && is_identifier(n) && src_root.join(n).is_dir())
        .filter(|n| src_root.join(n).join("mod.rs").is_file())
        .map(|n| mod_decl(n))
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    let add = format!(
        "\n// jar 输入模式：新顶层包根（按磁盘实际目录补全，非手写清单成员）\n{}\n",
        missing.join("\n")
    );
    let out = text + &add;
    std::fs::write(&lib_rs, out).map_err(|e| io_err(&lib_rs.display().to_string(), e))
}
