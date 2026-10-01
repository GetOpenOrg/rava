//! 物理拆层（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S4）。
//!
//! 第二阶段收尾之后、落盘之前，把 JDK 生成类（非手写、非接口）的 `java_class!` 块一分为二：
//! - 声明层：原文件原位（`java_runtime/src/…`），块首加 `#[rava_layer = "decl"]`；
//! - 实现层：同一块文本加 `#[rava_layer = "body"]`，连同原文件头（allow 属性与 use 列表）
//!   写进实现 crate `java_body_k/src/body/<同相对路径>`，另加一行 glob 导入声明层的本类模块。
//!   块后的 `iface_upcasts!`、`implref`、反射字段闭包等属声明层，不进实现层。
//!
//! 实现 crate 划分：类按 binary name 排序，按块文本字节贪心装箱（单箱上限
//! [`BODY_CRATE_BYTES`]），划分只依赖闭包本身，同一闭包恒得同一划分（编译缓存可复用）。
//! 实现 crate 根 `use java_runtime::*;`（私有 glob）：类文件头的 `crate::java::…` /
//! `crate::prelude` 路径经它解析到声明层；本 crate 的类模块挂在 `body` 子树下且 mod.rs 只声明
//! 不再导出，不遮蔽 `crate::java`。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use ty::ident::is_rust_keyword;

use super::fs::{has_marker, walk, Writer};
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;
use crate::error::{io_err, Result};
use crate::text::scratch_pkg_version;

/// 实现 crate 名前缀（`java_meta` 构建脚本扫描兄弟 crate 时据此排除实现层副本）
pub const BODY_CRATE_PREFIX: &str = "java_body_";

/// 单个实现 crate 的块文本字节上限（§7.6：单个实现 crate rustc 峰值 ≤ 1.5 GB）
pub const BODY_CRATE_BYTES: usize = 5 << 20;

/// 实现 crate 至少分两箱：声明层编完元数据后实现层并行编译（`CARGO_BUILD_JOBS=2` 下两箱同时跑），
/// 小闭包（HelloWorld 一箱 3.7 MB）的实现层墙钟减半
pub const MIN_BODY_CRATES: usize = 2;

const BLOCK_OPEN: &str = "rava_macros::java_class! {\n";

/// 实现 crate 的 lib.rs
const BODY_LIB: &str = "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, \
                        non_camel_case_types, non_upper_case_globals, static_mut_refs, unused_comparisons)]\n\
                        // 声明层全部公开项（`crate::java::…` / `crate::prelude` 经此解析到 java_runtime）\n\
                        use java_runtime::*;\n\
                        mod body;\n";

/// 一个实现 crate：名字 + 类文件（相对 `src/body` 的路径 → 文本）
#[derive(Debug, Default)]
pub struct BodyCrate {
    pub name: String,
    pub files: BTreeMap<PathBuf, String>,
}

/// 拆层结果（实现 crate 按名字序）
#[derive(Debug, Default)]
pub struct BodyPlan {
    pub crates: Vec<BodyCrate>,
}

impl BodyPlan {
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.crates.iter().map(|c| c.name.as_str())
    }
}

/// 类文件相对路径 → 模块路径（`java/lang/ref/reference.rs` → `java::lang::r#ref::reference`）
fn module_path(rel: &Path) -> String {
    let segs: Vec<String> = rel
        .with_extension("")
        .components()
        .map(|c| {
            let s = c.as_os_str().to_string_lossy().into_owned();
            if is_rust_keyword(&s) { format!("r#{s}") } else { s }
        })
        .collect();
    segs.join("::")
}

/// 一个类文本拆成 (声明层文本, 实现层文本)；块不存在时 None
pub fn split_text(text: &str, module: &str) -> Option<(String, String)> {
    let open = text.find(BLOCK_OPEN)?;
    let start = open + BLOCK_OPEN.len();
    // 块以第 0 列的 `}` 行结束（块内各项至少缩进 4 列）
    let close = start + text[start..].find("\n}\n")? + 3;
    let decl = format!("{}{BLOCK_OPEN}    #[rava_layer = \"decl\"]\n{}", &text[..open], &text[start..]);
    let body = format!(
        "{}use java_runtime::{module}::*;\n\n{BLOCK_OPEN}    #[rava_layer = \"body\"]\n{}",
        &text[..open],
        &text[start..close]
    );
    Some((decl, body))
}

/// 改写 `ems` 中可拆类的文本为声明层，返回实现层装箱结果
pub fn split(ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, jrt_src: &Path) -> BodyPlan {
    let mut bodies: Vec<(String, PathBuf, String)> = Vec::new();
    for em in ems.values_mut() {
        if em.crate_name != "java_runtime" || em.handwritten {
            continue;
        }
        if ctx.class(&em.binary_name).is_none_or(|ci| ci.is_interface()) {
            continue;
        }
        let Ok(rel) = em.path.strip_prefix(jrt_src) else { continue };
        let rel = rel.to_path_buf();
        let Some((decl, body)) = split_text(&em.text, &module_path(&rel)) else { continue };
        em.text = decl;
        bodies.push((em.binary_name.clone(), rel, body));
    }
    bodies.sort_by(|a, b| a.0.cmp(&b.0));
    let sizes: Vec<usize> = bodies.iter().map(|b| b.2.len()).collect();
    let bins = pack(&sizes, BODY_CRATE_BYTES);
    let mut plan = BodyPlan::default();
    for ((_, rel, body), bin) in bodies.into_iter().zip(bins) {
        while plan.crates.len() <= bin {
            let name = format!("{BODY_CRATE_PREFIX}{}", plan.crates.len() + 1);
            plan.crates.push(BodyCrate { name, files: BTreeMap::new() });
        }
        plan.crates[bin].files.insert(rel, body);
    }
    plan
}

/// 均衡装箱：箱数 k = max(⌈总字节 / 上限⌉, [`MIN_BODY_CRATES`])（不超过项数），按序把每项分到其字节中点落入的 1/k 区间，
/// 各箱约为总量 / k（≤ 上限，偏差不超过单项大小），不出现贪心装箱尾部的小箱；
/// 结果只依赖项序与大小（确定性）。返回每项的箱号（单调不减、无空箱）。
fn pack(sizes: &[usize], cap: usize) -> Vec<usize> {
    let total: usize = sizes.iter().sum();
    let k = total.div_ceil(cap.max(1)).max(MIN_BODY_CRATES).min(sizes.len().max(1));
    let mut prefix = 0usize;
    let mut out = Vec::with_capacity(sizes.len());
    let mut last = 0usize;
    for &n in sizes {
        let mid = prefix + n / 2;
        prefix += n;
        // 箱号单调不减，且相邻项最多跳一箱（大项跨越多个区间时不留空箱）
        let bin = ((mid as u128 * k as u128 / total.max(1) as u128) as usize).min(k - 1);
        let bin = if out.is_empty() { 0 } else { bin.clamp(last, last + 1) };
        out.push(bin);
        last = bin;
    }
    out
}

/// 模块声明行（关键字名加 `r#`）
fn mod_line(name: &str) -> String {
    if is_rust_keyword(name) { format!("mod r#{name};") } else { format!("mod {name};") }
}

impl BodyPlan {
    /// 各实现 crate 落盘：类文件、`body` 模块树（只声明不导出）、lib.rs、Cargo.toml；
    /// 清除本轮未写入的带生成标记的陈旧类文件（复用 scratch）
    pub fn write_crates(&self, ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path) -> Result<()> {
        for c in &self.crates {
            let dir = out_dir.join(&c.name);
            let src = dir.join("src");
            let body_root = src.join("body");
            let files: Vec<(PathBuf, &str)> = c.files.iter().map(|(rel, t)| (body_root.join(rel), t.as_str())).collect();
            let refs: Vec<(&Path, &str)> = files.iter().map(|(p, t)| (p.as_path(), *t)).collect();
            w.write_all(crate::par::resolve_jobs(ctx.opts.jobs), &refs)?;
            let mut tree: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
            for rel in c.files.keys() {
                let mut parent = body_root.clone();
                for comp in rel.components() {
                    let name = comp.as_os_str().to_string_lossy().into_owned();
                    let name = name.strip_suffix(".rs").map(str::to_string).unwrap_or(name);
                    tree.entry(parent.clone()).or_default().insert(name.clone());
                    parent = parent.join(&name);
                }
            }
            sweep_stale(&body_root, w)?;
            for (d, children) in &tree {
                let text: Vec<String> = children.iter().map(|m| mod_line(m)).collect();
                w.write(&d.join("mod.rs"), &(text.join("\n") + "\n"))?;
            }
            w.write(&src.join("lib.rs"), BODY_LIB)?;
            let l = [
                "[package]".to_string(),
                format!("name = \"{}\"", c.name),
                format!("version = \"{}\"", scratch_pkg_version(&dir)),
                "edition = \"2021\"".into(),
                String::new(),
                "[lib]".into(),
                format!("name = \"{}\"", c.name),
                "path = \"src/lib.rs\"".into(),
                "crate-type = [\"lib\"]".into(),
                String::new(),
                "[dependencies]".into(),
                "java_runtime    = { path = \"../java_runtime\" }".into(),
                format!("rava_macros = {{ path = \"{}\" }}", ctx.macros_crate.display()),
                String::new(),
            ];
            let mut l = l.to_vec();
            l.extend(super::entry::lints_section());
            w.write(&dir.join("Cargo.toml"), &l.join("\n"))?;
        }
        Ok(())
    }
}

/// 本轮未写入的带生成标记类文件删除（其模块不再声明，留着只占磁盘）
fn sweep_stale(body_root: &Path, w: &Writer) -> Result<()> {
    for (dir, _, files) in walk(body_root) {
        for f in files {
            let p = dir.join(&f);
            if f == "mod.rs" || !f.ends_with(".rs") || w.written_this_run(&p) {
                continue;
            }
            if has_marker(&p) == Some(true) {
                std::fs::remove_file(&p).map_err(|e| io_err(&p.display().to_string(), e))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_text_separates_layers() {
        let text = "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n// tail\nrava_macros::iface_upcasts! { impl B => C }\n";
        let (decl, body) = split_text(text, "a::b").unwrap();
        assert_eq!(
            decl,
            "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    #[rava_layer = \"decl\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n// tail\nrava_macros::iface_upcasts! { impl B => C }\n"
        );
        assert_eq!(
            body,
            "#![allow(x)]\nuse crate::prelude::*;\n\nuse java_runtime::a::b::*;\n\nrava_macros::java_class! {\n    #[rava_layer = \"body\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n"
        );
        assert_eq!(module_path(Path::new("java/lang/ref/reference.rs")), "java::lang::r#ref::reference");
    }

    #[test]
    fn pack_balances_bins() {
        // 总 25、上限 10 → 3 箱，各约 8
        let bins = pack(&[3, 3, 3, 3, 3, 3, 3, 4], 10);
        assert_eq!(bins, vec![0, 0, 0, 1, 1, 1, 2, 2]);
        assert_eq!(pack(&[5], 10), vec![0]);
        // 总量不足一箱也分两箱
        assert_eq!(pack(&[2, 2, 2, 2], 10), vec![0, 0, 1, 1]);
        assert_eq!(pack(&[], 10), Vec::<usize>::new());
        // 单项超上限：箱号不跳空
        assert_eq!(pack(&[1, 30, 1], 10), vec![0, 1, 2]);
    }
}
