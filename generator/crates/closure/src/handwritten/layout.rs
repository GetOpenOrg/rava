//! 手写层文件布局约定：宿主文件的私有辅助子模块目录（规范见 docs/reference/handwritten-boundary.md §五）。
//!
//! 手写文件 `<stem>.rs` 可拥有与之同名的私有辅助目录 `<stem>/`（Rust 非 mod.rs 子模块布局），宿主仅限：
//! - 共置手写文件（stem 以 `_impl` / `_ext` 结尾，[`MODULE_SUFFIXES`]）；
//! - crate 根下的基础设施模块文件（`lib.rs` 除外）。
//!
//! 辅助目录整棵子树是宿主的私有模块树，不是 Java 包：overlay 按宿主所在包定向、mod 树不为其生成 mod.rs、
//! 扫描器把子树文件并入宿主（同一「手写单元」）。宿主须在辅助目录名之外不与任何 Java 包段冲突：
//! JDK 包段不以 `_impl` / `_ext` 结尾，crate 根基础设施名不是 JDK 顶层包名（发射期另有冲突检查）。
//!
//! 书写约定（使合并文本与拆分前的单文件语义一致，扫描器以合并文本为准）：
//! - 宿主声明子模块恰为 `mod <name>;` 与 `use <name>::*;` 两行；
//! - 辅助文件以 `use super::*;` 取得宿主的名字空间，不写其他 `super::` / `self::` 相对路径；
//! - 辅助文件不写内层属性 `#![…]`（文件头说明用 `//!` 允许，合并时降为普通注释）；
//! - 辅助文件中供宿主调用的项用 `pub(super)`（不是 `pub`：`pub fn` 是手写方法的登记口径）。

use std::path::{Path, PathBuf};

use super::MODULE_SUFFIXES;

/// crate 根模块文件名（不作宿主）
const CRATE_ROOT_FILE: &str = "lib";

/// `dir`（`src` 之下）是否是某宿主文件的私有辅助目录本身（不看祖先）
pub fn is_helper_dir(src: &Path, dir: &Path) -> bool {
    let (Some(parent), Some(name)) = (dir.parent(), dir.file_name().and_then(|n| n.to_str())) else { return false };
    if dir == src || !dir.starts_with(src) {
        return false;
    }
    let host_ok = MODULE_SUFFIXES.iter().any(|s| name.ends_with(s)) || (parent == src && name != CRATE_ROOT_FILE);
    host_ok && dir.is_dir() && parent.join(format!("{name}.rs")).is_file()
}

/// `path`（文件或目录，`src` 之下）所在的最外层辅助目录（含 `path` 自身是辅助目录）；不在辅助目录内 → None
pub fn helper_root(src: &Path, path: &Path) -> Option<PathBuf> {
    let rel = path.strip_prefix(src).ok()?;
    let mut cur = src.to_path_buf();
    for c in rel.components() {
        cur.push(c);
        if is_helper_dir(src, &cur) {
            return Some(cur);
        }
    }
    None
}

/// 文件所属手写单元的宿主文件：辅助目录内的文件 → 最外层辅助目录的宿主 `<dir>.rs`；其余 → 自身
pub fn host_of(src: &Path, path: &Path) -> PathBuf {
    match helper_root(src, path) {
        Some(d) => d.with_extension("rs"),
        None => path.to_path_buf(),
    }
}

/// 宿主文件的手写单元：宿主自身 + 其辅助目录下全部 `.rs`（路径序）；宿主不合约定时只有自身
pub fn unit_files(src: &Path, host: &Path) -> Vec<PathBuf> {
    let mut out = vec![host.to_path_buf()];
    let dir = host.with_extension("");
    if is_helper_dir(src, &dir) {
        walk_rs(&dir, &mut out);
    }
    out
}

fn walk_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in &entries {
        if p.is_file() && p.extension().is_some_and(|x| x == "rs") {
            out.push(p.clone());
        }
    }
    for p in &entries {
        if p.is_dir() {
            walk_rs(p, out);
        }
    }
}

/// 文件的直接子模块名（同名目录 `<stem>/` 下的 `.rs` 文件 stem 与子目录名）
fn child_modules(file: &Path) -> Vec<String> {
    let dir = file.with_extension("");
    let Ok(rd) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut out: Vec<String> = rd
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.is_dir() {
                p.file_name().and_then(|n| n.to_str()).map(str::to_string)
            } else if p.extension().is_some_and(|x| x == "rs") {
                p.file_stem().and_then(|n| n.to_str()).map(str::to_string)
            } else {
                None
            }
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// 单元内一个文件的合并形态：剥去子模块声明行（`mod c;` / `use c::*;`），辅助文件另剥 `use super::*;`、
/// 文件头 `//!` 降为 `//`。被剥的行留空行（文件内行号不变）
fn merged_text(file: &Path, text: &str, is_host: bool) -> String {
    let children = child_modules(file);
    let mut out = String::with_capacity(text.len() + 1);
    for line in text.lines() {
        let t = line.trim();
        let strip = children.iter().any(|c| t == format!("mod {c};") || t == format!("use {c}::*;"))
            || (!is_host && t == "use super::*;");
        if strip {
            // 剥除
        } else if let (false, Some(rest)) = (is_host, line.strip_prefix("//!")) {
            out.push_str("//");
            out.push_str(rest);
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

/// 读宿主文件的手写单元合并文本：宿主 + 辅助文件（路径序）依次拼接，语义等同拆分前的单文件。
/// 宿主不可读 → Err；辅助文件不可读时跳过
pub fn read_unit(src: &Path, host: &Path) -> std::io::Result<String> {
    let files = unit_files(src, host);
    let head = std::fs::read_to_string(host)?;
    if files.len() == 1 {
        return Ok(head);
    }
    let mut out = merged_text(host, &head, true);
    for f in &files[1..] {
        if let Ok(t) = std::fs::read_to_string(f) {
            out.push_str(&merged_text(f, &t, false));
        }
    }
    Ok(out)
}

/// 辅助文件违反书写约定的行（`相对路径:行号: 内容`）：相对路径 `super::` / `self::`（`use super::*;` 除外）
/// 与内层属性 `#![`。合并文本以宿主为基准解析路径，这些写法会使分析与编译看到的名字不一致
pub fn helper_violations(src: &Path, file: &Path, text: &str) -> Vec<String> {
    let rel = file.strip_prefix(src).unwrap_or(file).to_string_lossy().replace('\\', "/");
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("//") || t == "use super::*;" {
            continue;
        }
        if t.starts_with("#![") || t.contains("super::") || t.contains("self::") {
            out.push(format!("{rel}:{}: {t}", i + 1));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(p: &Path, s: &str) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, s).unwrap();
    }

    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("rava_hw_layout_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn helper_dirs_by_convention() {
        let src = tmp("dirs");
        put(&src.join("lib.rs"), "");
        put(&src.join("array.rs"), "mod store;\nuse store::*;\n");
        put(&src.join("array/store.rs"), "use super::*;\n");
        put(&src.join("java/lang/class_impl.rs"), "");
        put(&src.join("java/lang/class_impl/reflect.rs"), "");
        put(&src.join("java/lang/class_impl/reflect/rows.rs"), "");
        put(&src.join("java/lang/object.rs"), "");
        put(&src.join("java/lang/object/x.rs"), "");
        put(&src.join("java/lang/invoke/x_impl.rs"), "");
        assert!(is_helper_dir(&src, &src.join("array")));
        assert!(is_helper_dir(&src, &src.join("java/lang/class_impl")));
        assert!(!is_helper_dir(&src, &src.join("java/lang/object")), "非共置手写宿主不带辅助目录");
        assert!(!is_helper_dir(&src, &src.join("java")), "Java 包目录");
        assert!(!is_helper_dir(&src, &src.join("java/lang/invoke")));
        let rows = src.join("java/lang/class_impl/reflect/rows.rs");
        assert_eq!(helper_root(&src, &rows), Some(src.join("java/lang/class_impl")));
        assert_eq!(host_of(&src, &rows), src.join("java/lang/class_impl.rs"));
        assert_eq!(host_of(&src, &src.join("java/lang/object.rs")), src.join("java/lang/object.rs"));
        assert_eq!(
            unit_files(&src, &src.join("java/lang/class_impl.rs")),
            vec![src.join("java/lang/class_impl.rs"), src.join("java/lang/class_impl/reflect.rs"), rows.clone()]
        );
        assert_eq!(unit_files(&src, &src.join("java/lang/object.rs")), vec![src.join("java/lang/object.rs")]);
        let _ = std::fs::remove_dir_all(&src);
    }

    #[test]
    fn unit_text_equals_single_file() {
        let src = tmp("text");
        put(&src.join("lib.rs"), "");
        put(&src.join("p/a_impl.rs"), "use super::*;\nmod rows;\nuse rows::*;\nimpl A { pub fn f() { g() } }\n");
        put(&src.join("p/a_impl/rows.rs"), "//! 行表\nuse super::*;\npub(super) fn g() {}\n");
        let text = read_unit(&src, &src.join("p/a_impl.rs")).unwrap();
        assert_eq!(text, "use super::*;\n\n\nimpl A { pub fn f() { g() } }\n// 行表\n\npub(super) fn g() {}\n");
        assert!(syn::parse_file(&text).is_ok());
        assert!(helper_violations(&src, &src.join("p/a_impl/rows.rs"), "use super::*;\nuse super::super::B;\n#![allow(x)]\n").len() == 2);
        let _ = std::fs::remove_dir_all(&src);
    }

    /// 手写真源的全部辅助文件遵守书写约定，且所在辅助目录的宿主确实声明了它（`mod <name>;`）
    #[test]
    fn runtime_helpers_follow_convention() {
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime/src");
        let mut files = Vec::new();
        walk_rs(&src, &mut files);
        assert!(files.len() > 10, "未找到手写层源文件：{}", src.display());
        let mut bad = Vec::new();
        for f in files.iter().filter(|f| helper_root(&src, f).is_some()) {
            let text = std::fs::read_to_string(f).unwrap();
            bad.extend(helper_violations(&src, f, &text));
            let (Some(parent), Some(stem)) = (f.parent(), f.file_stem().and_then(|s| s.to_str())) else { continue };
            let decl = std::fs::read_to_string(parent.with_extension("rs")).unwrap_or_default();
            if !decl.lines().any(|l| l.trim() == format!("mod {stem};")) {
                bad.push(format!("{}: 宿主未声明 `mod {stem};`", f.display()));
            }
        }
        assert!(bad.is_empty(), "辅助文件违反书写约定（见 layout.rs 模块注释）：\n{}", bad.join("\n"));
    }
}
