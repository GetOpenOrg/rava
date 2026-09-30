//! 包 mod 树落盘（`project_writer._write_jdk_mod_tree` / `_complete_jrt_lib_rs` 的移植）。
//!
//! 类文件全部落盘后调用：mod.rs 如实声明磁盘上的全部模块（手写 overlay + 本轮生成 +
//! 同 scratch 上轮幸存）。本轮未写入的带生成标记文件先清除；不在本轮 mod 树中的陈旧包目录
//! 删除生成 mod.rs 与空目录（复用 scratch 场景）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use closure::handwritten::HwTypeRefs;
use ty::ident::is_rust_keyword;

use super::fs::{has_marker, walk, Writer};
use crate::error::{io_err, Result};

/// 共置手写文件的编译期模块依赖判定：依赖取自手写文件自身的路径引用
/// （`closure::handwritten` 已解析的 use 表 / 类型路径），引用的模块文件在 scratch 中缺席
/// （语料条件生成类本轮未生成）则该 companion 不声明——等价于该 impl 尚不存在，
/// 其服务的 native 方法回落 panic 存根，而不是让无法解析的 import 拖垮整个 crate。
pub struct CompanionDeps {
    hw: HwTypeRefs,
    /// scratch 的 crate src 根（`crate::` 起点）
    src_root: PathBuf,
}

/// 路径段去 `r#` 前缀
fn seg_name(s: &str) -> &str {
    s.strip_prefix("r#").unwrap_or(s)
}

impl CompanionDeps {
    pub fn new(runtime_dir: &Path, src_root: &Path) -> CompanionDeps {
        CompanionDeps { hw: HwTypeRefs::new(runtime_dir), src_root: src_root.to_path_buf() }
    }

    /// 一条路径引用（`super::…` / `crate::…`）指向的模块文件是否在场。
    /// 逐段下行：目录 → 进入；`<seg>.rs` / `<seg>_t.rs` → 在场；crate 根层的名字属 lib.rs
    /// 基础设施、大写段是 glob 再导出的类型 → 不再判定；包目录下缺席的小写段 → 缺席
    fn path_present(&self, dir: &Path, segs: &[String]) -> bool {
        let (mut cur, rest) = match segs.first().map(String::as_str) {
            Some("crate") => (self.src_root.clone(), &segs[1..]),
            Some("super") => {
                let mut d = dir.to_path_buf();
                let mut i = 1;
                while segs.get(i).is_some_and(|s| s == "super") {
                    d = d.parent().map_or(d.clone(), Path::to_path_buf);
                    i += 1;
                }
                (d, &segs[i..])
            }
            _ => return true,
        };
        for seg in rest {
            let seg = seg_name(seg);
            if cur.join(seg).is_dir() {
                cur = cur.join(seg);
                continue;
            }
            if cur.join(format!("{seg}.rs")).is_file() || cur.join(format!("{seg}_t.rs")).is_file() {
                return true;
            }
            if cur == self.src_root || !seg.starts_with(|c: char| c.is_ascii_lowercase()) {
                return true;
            }
            return false;
        }
        true
    }

    /// `dir/<base>_impl.rs`（或 `_ext.rs`）的全部路径引用在场
    fn satisfied(&self, dir: &Path, base: &str) -> bool {
        let Ok(rel) = dir.strip_prefix(&self.src_root) else { return true };
        let rel = rel.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/");
        let cls = if rel.is_empty() { base.to_string() } else { format!("{rel}/{base}") };
        self.hw.class(&cls).iter().all(|t| self.path_present(dir, &t.0))
    }
}

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

/// 一次递归列举（[`walk`] 形态）；清扫后原地剔除已删文件，供后续各步复用
type Listing = Vec<(PathBuf, Vec<String>, Vec<String>)>;

/// 已读过的生成标记（路径 → [`has_marker`]），免得同一文件读两遍
type Markers = BTreeMap<PathBuf, Option<bool>>;

fn is_module_file(f: &str) -> bool {
    f.ends_with(".rs") && f != "lib.rs" && f != "mod.rs"
}

/// 本轮未写入的陈旧 .rs（lib.rs / mod.rs 除外）清除：带生成标记的上轮生成文件；
/// `handwritten_src` 给出时（java_runtime）还有手写真源已删除的无标记文件。
/// 在本轮写出之后判定，本轮生成的无标记文件（如模块资源表）不会被先删后写。
/// 标记按 `jobs` 并行读取，删除按列举序串行（出错报列举序第一个）；删除的文件从 `listing` 剔除
fn sweep_stale(listing: &mut Listing, src_root: &Path, handwritten_src: Option<&Path>, writer: &Writer, jobs: usize) -> Result<Markers> {
    let cands: Vec<PathBuf> = listing
        .iter()
        .flat_map(|(dir, _, files)| files.iter().filter(|f| is_module_file(f)).map(move |f| dir.join(f)))
        .filter(|p| !writer.written_this_run(p))
        .collect();
    let marks = crate::par::par_map(jobs, &cands, |p| has_marker(p));
    let markers: Markers = cands.into_iter().zip(marks).collect();
    for (dir, _, files) in listing.iter_mut() {
        let rel = dir.strip_prefix(src_root).unwrap_or(Path::new("")).to_path_buf();
        let mut removed = Vec::new();
        for f in files.iter() {
            let p = dir.join(f);
            let Some(&mark) = markers.get(&p) else { continue };
            let stale = match mark {
                Some(true) => true,
                Some(false) => handwritten_src.is_some_and(|hw| !hw.join(&rel).join(f).exists()),
                None => false,
            };
            if stale {
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
                removed.push(f.clone());
            }
        }
        files.retain(|f| !removed.contains(f));
    }
    Ok(markers)
}

/// 从磁盘收集 mod 树：目录 → 子模块名（文件 stem 与子目录名）
fn scan_tree(listing: &Listing, src_root: &Path, handwritten_src: Option<&Path>, markers: &Markers) -> BTreeMap<PathBuf, BTreeSet<String>> {
    let mut tree: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let mut skip_under: Vec<PathBuf> = Vec::new();
    for (dir, _, files) in listing {
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
            if !is_module_file(f) {
                continue;
            }
            // 共置手写文件由 companion 声明；含生成标记的是碰巧以 Impl/Ext 结尾的生成类
            if f.ends_with("_impl.rs") || f.ends_with("_ext.rs") {
                let p = dir.join(f);
                let mark = markers.get(&p).copied().unwrap_or_else(|| has_marker(&p));
                if mark != Some(true) {
                    continue;
                }
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

/// 陈旧包目录清除（`project_writer._write_jdk_mod_tree` 同款）：[`sweep_stale`] 只删类文件，包内类
/// 全部离开闭包后，上轮生成的 mod.rs 仍声明已删除模块（E0583），且目录与同名类文件并存
/// （E0761，`java/lang/module/` 与 `java/lang/module.rs`）。不在本轮 mod 树中的子包目录删除其
/// mod.rs 与空目录（目录内只剩本轮无宿主的共置手写时同样无可声明模块，保留文件）。
/// 保留：手写模块目录（runtime/ 真源提供 mod.rs）整棵子树；java_runtime 手写 lib.rs 直接声明的
/// 顶层目录（其余顶层包由 [`complete_lib_rs`] 按磁盘补声明，目录删除即不再声明）；lib crate
/// （`handwritten_src` 为 None）的全部顶层目录
fn prune_stale_pkg_dirs(
    listing: &Listing,
    src_root: &Path,
    handwritten_src: Option<&Path>,
    tree: &BTreeMap<PathBuf, BTreeSet<String>>,
) -> Result<()> {
    let lib_declared: BTreeSet<String> = handwritten_src
        .and_then(|hw| std::fs::read_to_string(hw.join("lib.rs")).ok())
        .map(|text| {
            let re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+(?:r#)?(\w+)\s*;").expect("静态正则");
            re.captures_iter(&text).filter_map(|c| c.get(1)).map(|m| m.as_str().to_string()).collect()
        })
        .unwrap_or_default();
    let handwritten_mod_dir = |d: &Path| {
        let Some(hw) = handwritten_src else { return false };
        let rel = d.strip_prefix(src_root).unwrap_or(Path::new(""));
        rel.ancestors().any(|a| !a.as_os_str().is_empty() && hw.join(a).join("mod.rs").is_file())
    };
    // 自底向上：子目录先于父目录处理，删空的子目录让父目录也可能变空
    for (dir, _, files) in listing.iter().rev() {
        if dir == src_root || tree.contains_key(dir) || handwritten_mod_dir(dir) {
            continue;
        }
        if dir.parent() == Some(src_root) {
            let name = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            if handwritten_src.is_none() || lib_declared.contains(&name) {
                continue;
            }
        }
        remove_stale_dir(dir, files)?;
    }
    Ok(())
}

/// 陈旧包目录：删除其生成 mod.rs，目录变空则删除目录
fn remove_stale_dir(dir: &Path, files: &[String]) -> Result<()> {
    if files.iter().any(|f| f == "mod.rs") {
        let m = dir.join("mod.rs");
        std::fs::remove_file(&m).map_err(|e| io_err(&m.display().to_string(), e))?;
    }
    if std::fs::read_dir(dir).is_ok_and(|mut rd| rd.next().is_none()) {
        std::fs::remove_dir(dir).map_err(|e| io_err(&dir.display().to_string(), e))?;
    }
    Ok(())
}

/// user crate 的陈旧清扫（复用 scratch）：与 java_runtime 同一机制——本轮未写入的带生成标记
/// .rs 清除（user crate 无手写真源），不在本轮 mod 树 `dirs` 中的包目录删除 mod.rs 与空目录。
/// 顶层包目录也在其列：user crate 的顶层模块由本轮写出的 main.rs 声明，不在本轮树中即不再声明。
/// 须在本轮 user 类文件、mod.rs、main.rs 全部写出之后调用
pub fn sweep_user_crate(src_root: &Path, dirs: &BTreeMap<PathBuf, BTreeSet<String>>, writer: &Writer, jobs: usize) -> Result<()> {
    let mut listing = walk(src_root);
    sweep_stale(&mut listing, src_root, None, writer, jobs)?;
    for (dir, _, files) in listing.iter().rev() {
        if dir != src_root && !dirs.contains_key(dir) {
            remove_stale_dir(dir, files)?;
        }
    }
    Ok(())
}

/// 目录的共置手写声明：(补充 pub 声明的宿主, `mod X_impl;` 行)
fn companions(
    dir: &Path,
    files: &[String],
    children: &BTreeSet<String>,
    deps: Option<&CompanionDeps>,
) -> (BTreeSet<String>, Vec<String>) {
    let mut extra_pub = BTreeSet::new();
    let mut mods = Vec::new();
    for f in files {
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
        if deps.is_some_and(|d| !d.satisfied(dir, base)) {
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
/// `runtime_dir`：手写真源 `runtime/java_runtime`（java_runtime 时给出，lib crate 为 None）。
///
/// 目录只列举一次：清扫只删文件（从列举中剔除），扫描不改盘，陈旧目录清除只动不在 mod 树中的
/// 目录，所以三步与共置手写判定看到的列举与各自重新遍历相同。共置手写取列举中的文件名：
/// 包目录名是 Java 包名段或手写模块名，不含 `.`，不会被当成 `_impl.rs` / `_ext.rs`。
/// 各目录的 mod.rs 只取决于该目录的列举与手写依赖判定（写 mod.rs 不改变任何目录的
/// `_impl` / `_ext` / 类文件在场情况），按 `jobs` 并行生成与落盘，写出顺序同目录序
pub fn write_mod_tree(src_root: &Path, runtime_dir: Option<&Path>, jobs: usize, writer: &mut Writer) -> Result<()> {
    let handwritten_src = runtime_dir.map(|r| r.join("src"));
    let mut listing = walk(src_root);
    let markers = sweep_stale(&mut listing, src_root, handwritten_src.as_deref(), writer, jobs)?;
    if !src_root.is_dir() {
        return Ok(());
    }
    let tree = scan_tree(&listing, src_root, handwritten_src.as_deref(), &markers);
    prune_stale_pkg_dirs(&listing, src_root, handwritten_src.as_deref(), &tree)?;
    let files_of: BTreeMap<&Path, &[String]> = listing.iter().map(|(d, _, f)| (d.as_path(), f.as_slice())).collect();
    let deps = runtime_dir.map(|r| CompanionDeps::new(r, src_root));
    let dirs: Vec<(&PathBuf, &BTreeSet<String>)> = tree.iter().filter(|(d, _)| d.as_path() != src_root).collect();
    let texts = crate::par::par_map(jobs, &dirs, |&(dir, children)| {
        let files = files_of.get(dir.as_path()).copied().unwrap_or(&[]);
        mod_rs_text(&tree, dir, files, children, deps.as_ref())
    });
    let paths: Vec<PathBuf> = dirs.iter().map(|(d, _)| d.join("mod.rs")).collect();
    let files: Vec<(&Path, &str)> = paths.iter().map(PathBuf::as_path).zip(texts.iter().map(String::as_str)).collect();
    writer.write_all(jobs, &files)
}

/// 一个包目录的 mod.rs 内容
fn mod_rs_text(
    tree: &BTreeMap<PathBuf, BTreeSet<String>>,
    dir: &Path,
    files: &[String],
    children: &BTreeSet<String>,
    deps: Option<&CompanionDeps>,
) -> String {
    let mut lines = vec!["#![allow(ambiguous_glob_reexports)]".to_string()];
    for c in children {
        lines.push(mod_decl(c));
        // 子包只声明 pub mod，不 glob 再导出（Java 包无嵌套可见性）
        if tree.contains_key(&dir.join(c)) {
            continue;
        }
        lines.push(use_decl(c));
    }
    let (extra_pub, comp) = companions(dir, files, children, deps);
    for b in &extra_pub {
        lines.push(mod_decl(b));
        lines.push(use_decl(b));
    }
    lines.extend(comp);
    lines.join("\n") + "\n"
}

fn is_identifier(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c == '_' || c.is_alphabetic()) && cs.all(|c| c == '_' || c.is_alphanumeric())
}

/// scratch `java_runtime/src/lib.rs` 写出：手写真源 `runtime_src/lib.rs` + 顶层模块补全
/// （jar 输入模式的新顶层包根）。整体经 [`Writer`] 写出：内容不变不重写（保留 mtime）
pub fn complete_lib_rs(src_root: &Path, runtime_src: &Path, writer: &mut Writer) -> Result<()> {
    let Ok(text) = std::fs::read_to_string(runtime_src.join("lib.rs")) else { return Ok(()) };
    let re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+(r#\s*)?(\w+)\s*;").expect("静态正则");
    let declared: BTreeSet<&str> = re.captures_iter(&text).filter_map(|c| c.get(2)).map(|m| m.as_str()).collect();
    let mut names: Vec<String> = std::fs::read_dir(src_root)
        .map(|rd| rd.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect())
        .unwrap_or_default();
    names.sort();
    let missing: Vec<String> = names
        .iter()
        .filter(|n| !declared.contains(n.as_str()) && is_identifier(n) && src_root.join(n).is_dir())
        .filter(|n| src_root.join(n).join("mod.rs").is_file())
        .map(|n| mod_decl(n))
        .collect();
    let out = if missing.is_empty() {
        text
    } else {
        format!("{text}\n// jar 输入模式：新顶层包根（按磁盘实际目录补全，非手写清单成员）\n{}\n", missing.join("\n"))
    };
    writer.write(&src_root.join("lib.rs"), &out)
}
