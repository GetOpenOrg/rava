//! 根模块声明层分段的落盘（D8）：[`super::decl_segments`] 的分段结果 → 各段 crate。
//!
//! - 底段沿用声明层 crate（`<根>_decl`：overlay、build.rs、手写文件、strict.txt 不动）；上层段
//!   `<根>_decl_<j>` 只放分到该段的生成类文件（类文本不改写，只换落盘目录）。
//! - **镜像链**：段 j 只依赖段 j−1。lib.rs `pub use <段 j−1>::*;` 并声明本段的顶层包；本段有类的每个包
//!   目录写 mod.rs：前一视图有该包时 `pub use <段 j−1>::<包>::*;`，本段子包 `pub mod`，本段类
//!   `pub mod x; pub use x::*;`。于是各段内 `crate::java::…` / `crate::prelude` 照常解析，末段即完整视图。
//! - 消费者（门面、实现层、非根模块 crate、lib crate）以 Cargo 改名依赖末段，源码文本不变。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use ty::ident::is_rust_keyword;

use super::decl_segments::{plan, DeclGraph, DeclNode, SegmentBudget, DECL_SEGMENT_BYTES, UPPER_SEGMENT_PEAK};
use super::fs::{walk, Writer};
use super::module_side::{lib_manifest, path_dep};
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;
use crate::error::{io_err, Result};

/// 上层段 lib.rs 属性行（与非根模块 crate 同口径）
const SEGMENT_ALLOW: &str = "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, \
                             non_camel_case_types, non_upper_case_globals, static_mut_refs, ambiguous_glob_reexports, \
                             unused_comparisons)]";

/// 上层段 lib.rs 中与类无关的固定行（属性、注释、`pub use <前段>::*;`）的体量上界（字节）
const SEGMENT_FIXED_BYTES: usize = 1024;

/// 上层段 crate 名相对底段名的最长后缀（`_<j>`，j ≤ 99999）
const SEGMENT_SUFFIX_BYTES: usize = 6;

/// 声明层分段：`names[0]` 为底段，其后为上层段（链序）
#[derive(Debug, Clone)]
pub struct DeclSegs {
    names: Vec<String>,
    /// 各上层段的类文件（相对 src 的路径）
    files: Vec<Vec<PathBuf>>,
}

impl DeclSegs {
    /// 不分段（只有底段）
    pub fn single(decl: &str) -> DeclSegs {
        DeclSegs { names: vec![decl.to_string()], files: Vec::new() }
    }

    /// 末段（完整视图）crate 名
    pub fn top(&self) -> &str {
        self.names.last().map(String::as_str).unwrap_or_default()
    }

    /// 上层段 crate 名（链序）
    pub fn uppers(&self) -> impl Iterator<Item = &str> {
        self.names[1..].iter().map(String::as_str)
    }

    /// 上层段源码树
    pub fn upper_srcs(&self, out_dir: &Path) -> Vec<PathBuf> {
        self.uppers().map(|n| out_dir.join(n).join("src")).collect()
    }

    /// 各上层段 (crate 名, 类数)
    pub fn upper_sizes(&self) -> impl Iterator<Item = (&str, usize)> {
        self.uppers().zip(self.files.iter().map(Vec::len))
    }
}

/// 类文件相对路径 → Rust 包路径（`java::lang::ref`，不带 `r#`）
fn pkg_of(rel: &Path) -> String {
    rel.parent()
        .map(|p| p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("::"))
        .unwrap_or_default()
}

/// 手写真源全部 .rs：(相对路径, 文本)
fn handwritten_files(runtime_src: &Path) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for (dir, _, files) in walk(runtime_src) {
        for f in files.iter().filter(|f| f.ends_with(".rs")) {
            let p = dir.join(f);
            if let (Ok(rel), Ok(t)) = (p.strip_prefix(runtime_src), std::fs::read_to_string(&p)) {
                out.push((rel.to_path_buf(), t));
            }
        }
    }
    out
}

/// 共置手写伴生文件（`x_impl.rs` / `x_ext.rs`）的宿主候选相对路径（`x.rs` / `x_t.rs`）
fn companion_hosts(rel: &Path) -> Vec<PathBuf> {
    let Some(name) = rel.file_name().and_then(|n| n.to_str()) else { return Vec::new() };
    let Some(base) = name.strip_suffix("_impl.rs").or_else(|| name.strip_suffix("_ext.rs")) else { return Vec::new() };
    [format!("{base}.rs"), format!("{base}_t.rs")].into_iter().map(|f| rel.with_file_name(f)).collect()
}

/// 规划分段并把上层段类的落盘路径改到 `<out>/<段>/src/…`（类文本不变）。须在拆层之后、落盘之前调用
pub fn segment(ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, decl_src: &Path, out_dir: &Path) -> DeclSegs {
    let budget = SegmentBudget { single: DECL_SEGMENT_BYTES, fixed: SEGMENT_FIXED_BYTES, model: UPPER_SEGMENT_PEAK };
    segment_with_budget(ctx, ems, decl_src, out_dir, &budget)
}

/// 类落在上层段时为其写出的 mod 行体量上界：所在包 mod.rs 的 `pub mod x; pub use x::*;`，外加按「本类独占该包」
/// 计的包头（`#![allow(..)]`、`pub use <前段>::<包>::*;`）与各级祖先目录的 `pub mod <子包>;`（逐类全额计入，只会偏大）
fn mod_overhead(rel: &Path, prev_len: usize) -> usize {
    let comp_len = |s: &str| s.len() + 2; // 可能的 `r#`
    let stem = rel.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let class_lines = "pub mod ;\n".len() + "pub use ::*;\n".len() + 2 * comp_len(&stem);
    let comps: Vec<String> =
        rel.parent().map(|p| p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect()).unwrap_or_default();
    let pkg_path: usize = comps.iter().map(|c| comp_len(c) + 2).sum();
    let pkg_head = "#![allow(ambiguous_glob_reexports)]\n".len() + "pub use ::::*;\n".len() + prev_len + pkg_path;
    let ancestors: usize = comps.iter().map(|c| "pub mod ;\n".len() + comp_len(c)).sum();
    class_lines + pkg_head + ancestors
}

fn segment_with_budget(
    ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, decl_src: &Path, out_dir: &Path, budget: &SegmentBudget,
) -> DeclSegs {
    let crates = ctx.crates();
    let decl = crates.decl();
    // 节点：根模块落在声明层源码树下的类（含手写类）
    let members: Vec<(usize, PathBuf)> = ems
        .values()
        .enumerate()
        .filter(|(_, em)| em.crate_name == crates.root())
        .filter_map(|(i, em)| em.path.strip_prefix(decl_src).ok().map(|r| (i, r.to_path_buf())))
        .collect();
    // 权重 = 落盘字节（类文本原样写出）+ 落在上层段时的 mod 行上界；手写类随 overlay 计入 base
    let prev_len = decl.len() + SEGMENT_SUFFIX_BYTES;
    let weights: Vec<usize> = members
        .iter()
        .map(|(i, rel)| if ems[*i].handwritten { 0 } else { ems[*i].text.len() + mod_overhead(rel, prev_len) })
        .collect();
    let hw = handwritten_files(&ctx.runtime_src());
    // 底段 crate 含全部手写 overlay（保守：手写真源全部计入），参与不分段判定并计入上段的上游体量
    let base: usize = hw.iter().map(|(_, t)| t.len()).sum();
    if base + weights.iter().sum::<usize>() + budget.fixed <= budget.single {
        return DeclSegs::single(&decl);
    }
    let hosts: BTreeSet<PathBuf> = hw.iter().flat_map(|(rel, _)| companion_hosts(rel)).collect();
    let nodes: Vec<DeclNode<'_>> = members
        .iter()
        .map(|(i, rel)| {
            let em = &ems[*i];
            DeclNode {
                key: &em.binary_name,
                pkg: pkg_of(rel),
                ident: ctx.declared(&em.binary_name),
                text: if em.handwritten { "" } else { &em.text },
            }
        })
        .collect();
    let pinned: Vec<usize> = members
        .iter()
        .enumerate()
        .filter(|(_, (i, rel))| ems[*i].handwritten || hosts.contains(rel))
        .map(|(n, _)| n)
        .collect();
    let texts: Vec<&str> = hw.iter().map(|(_, t)| t.as_str()).collect();
    let g = DeclGraph::build(&nodes, &texts, &pinned);
    let segs = plan(&g, &weights, base, budget);
    drop(nodes);
    if segs.len() <= 1 {
        return DeclSegs::single(&decl);
    }
    let mut out = DeclSegs::single(&decl);
    for (j, seg) in segs.iter().enumerate().skip(1) {
        let name = crates.decl_segment(j);
        let src = out_dir.join(&name).join("src");
        let mut files = Vec::with_capacity(seg.len());
        for &n in seg {
            let (i, rel) = &members[n];
            ems[*i].path = src.join(rel);
            files.push(rel.clone());
        }
        out.names.push(name);
        out.files.push(files);
    }
    out
}

/// 路径段（关键字加 `r#`）
fn seg_ident(name: &str) -> String {
    if is_rust_keyword(name) {
        format!("r#{name}")
    } else {
        name.to_string()
    }
}

/// 相对目录 → `a::b::r#ref`
fn rust_path(rel: &Path) -> String {
    rel.components().map(|c| seg_ident(&c.as_os_str().to_string_lossy())).collect::<Vec<_>>().join("::")
}

/// 源码树下声明了 mod.rs 的包目录（相对路径）
fn view_of(src: &Path) -> BTreeSet<PathBuf> {
    walk(src)
        .into_iter()
        .filter(|(d, _, files)| d != src && files.iter().any(|f| f == "mod.rs"))
        .filter_map(|(d, _, _)| d.strip_prefix(src).ok().map(Path::to_path_buf))
        .collect()
}

impl DeclSegs {
    /// 上层段落盘：各包 mod.rs、lib.rs、Cargo.toml；清除段内本轮未写的文件与多余的旧段目录。
    /// 须在类文件与底段 mod 树写出之后调用
    pub fn write_crates(&self, ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, decl_src: &Path) -> Result<()> {
        remove_extra_segments(out_dir, &self.names[0], self.names.len() - 1)?;
        let mut view = view_of(decl_src);
        for (j, files) in self.files.iter().enumerate() {
            let (prev, name) = (&self.names[j], &self.names[j + 1]);
            let dir = out_dir.join(name);
            let src = dir.join("src");
            // 本段的包目录 → (子包, 类模块)
            let mut pkgs: BTreeMap<PathBuf, (BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
            for rel in files {
                let parent = rel.parent().unwrap_or(Path::new("")).to_path_buf();
                let stem = rel.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                pkgs.entry(parent.clone()).or_default().1.insert(stem);
                let mut d = parent;
                while let (Some(p), Some(n)) = (d.parent().map(Path::to_path_buf), d.file_name()) {
                    let n = n.to_string_lossy().into_owned();
                    pkgs.entry(p.clone()).or_default().0.insert(n);
                    d = p;
                }
            }
            let mut lib_rs = vec![
                SEGMENT_ALLOW.to_string(),
                format!("// 声明层第 {} 段：前段视图全部公开项 + 本段的类（本段顶层包模块优先）", j + 1),
                format!("pub use {prev}::*;"),
            ];
            for (pkg, (subs, classes)) in &pkgs {
                if pkg.as_os_str().is_empty() {
                    lib_rs.extend(subs.iter().map(|s| format!("pub mod {};", seg_ident(s))));
                    for c in classes {
                        lib_rs.push(format!("pub mod {};", seg_ident(c)));
                        lib_rs.push(format!("pub use {}::*;", seg_ident(c)));
                    }
                    continue;
                }
                let mut lines = vec!["#![allow(ambiguous_glob_reexports)]".to_string()];
                if view.contains(pkg) {
                    lines.push(format!("pub use {prev}::{}::*;", rust_path(pkg)));
                }
                lines.extend(subs.iter().map(|s| format!("pub mod {};", seg_ident(s))));
                for c in classes {
                    lines.push(format!("pub mod {};", seg_ident(c)));
                    lines.push(format!("pub use {}::*;", seg_ident(c)));
                }
                w.write(&src.join(pkg).join("mod.rs"), &(lines.join("\n") + "\n"))?;
            }
            w.write(&src.join("lib.rs"), &(lib_rs.join("\n") + "\n"))?;
            let deps = [path_dep(prev, prev), format!("rava_macros     = {{ path = \"{}\" }}", ctx.macros_crate.display())];
            w.write(&dir.join("Cargo.toml"), &lib_manifest(&dir, name, &deps))?;
            sweep_unwritten(&src, w)?;
            view.extend(pkgs.into_keys().filter(|p| !p.as_os_str().is_empty()));
        }
        Ok(())
    }
}

/// 段内本轮未写入的文件删除（段目录全为生成物），随后自底向上删空目录
fn sweep_unwritten(src: &Path, w: &Writer) -> Result<()> {
    let listing = walk(src);
    for (dir, _, files) in &listing {
        for f in files {
            let p = dir.join(f);
            if !w.written_this_run(&p) {
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
    }
    for (dir, _, _) in listing.iter().rev() {
        if dir != src && std::fs::read_dir(dir).is_ok_and(|mut rd| rd.next().is_none()) {
            std::fs::remove_dir(dir).map_err(|e| io_err(&dir.display().to_string(), e))?;
        }
    }
    Ok(())
}

/// 复用 scratch：本轮段数之外的旧段目录（`<底段>_<j>`，j > 段数）整个删除（全为生成物）
fn remove_extra_segments(out_dir: &Path, decl: &str, uppers: usize) -> Result<()> {
    let Ok(rd) = std::fs::read_dir(out_dir) else { return Ok(()) };
    let prefix = format!("{decl}_");
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(j) = name.strip_prefix(&prefix).and_then(|s| s.parse::<usize>().ok()) else { continue };
        if j > uppers && e.path().is_dir() {
            std::fs::remove_dir_all(e.path()).map_err(|err| io_err(&e.path().display().to_string(), err))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn companion_host_candidates() {
        let hosts = companion_hosts(Path::new("java/lang/string_impl.rs"));
        assert_eq!(hosts, vec![PathBuf::from("java/lang/string.rs"), PathBuf::from("java/lang/string_t.rs")]);
        assert!(companion_hosts(Path::new("java/lang/string.rs")).is_empty());
    }

    #[test]
    fn paths_escape_keywords() {
        assert_eq!(pkg_of(Path::new("java/lang/ref/cleaner.rs")), "java::lang::ref");
        assert_eq!(rust_path(Path::new("java/lang/ref")), "java::lang::r#ref");
    }

    #[test]
    fn mod_overhead_bounds_written_lines() {
        let rel = Path::new("java/lang/ref/cleaner.rs");
        let prev = "java_base_decl_12";
        // 本类独占新包时上层段为它写出的全部行
        let written = [
            "#![allow(ambiguous_glob_reexports)]\n".to_string(),
            format!("pub use {prev}::java::lang::r#ref::*;\n"),
            "pub mod cleaner;\npub use cleaner::*;\n".to_string(),
            "pub mod java;\npub mod lang;\npub mod r#ref;\n".to_string(),
        ];
        let actual: usize = written.iter().map(String::len).sum();
        assert!(mod_overhead(rel, "java_base_decl".len() + SEGMENT_SUFFIX_BYTES) >= actual);
    }

    #[test]
    fn single_has_no_uppers() {
        let s = DeclSegs::single("rt_decl");
        assert_eq!(s.top(), "rt_decl");
        assert_eq!(s.uppers().count(), 0);
    }
}
