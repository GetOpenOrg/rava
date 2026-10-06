//! 物理拆层（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5 S4）。
//!
//! 第二阶段收尾之后、落盘之前，把 JDK 生成类（非手写、非接口）的 `java_class!` 块一分为二：
//! - 声明层：原文件原位（`<根>_decl/src/…`），块首加 `#[rava_layer = "decl"]`；
//! - 实现层：同一块文本加 `#[rava_layer = "body"]`，连同原文件头（allow 属性与 use 列表）
//!   写进实现 crate `<根>_body_k/src/body/<同相对路径>`，另加一行 glob 导入声明层的本类模块。
//!   块后的 `iface_upcasts!`、反射字段闭包等属声明层，不进实现层。
//!
//! 实现 crate 划分：类按 binary name 排序，按块文本字节贪心装箱（单箱上限
//! [`BODY_CRATE_BYTES`]），划分只依赖闭包本身，同一闭包恒得同一划分（编译缓存可复用）。
//! 实现 crate 根 `use <根>_decl::*;`（私有 glob）：类文件头的 `crate::java::…` /
//! `crate::prelude` 路径经它解析到声明层；本 crate 的类模块挂在 `body` 子树下且 mod.rs 只声明
//! 不再导出，不遮蔽 `crate::java`。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use rava_macros_core::plan::{decl_elisions, elide};
use ty::ident::is_rust_keyword;

use super::fs::{has_marker, walk, Writer};
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;
use crate::error::{io_err, EmitError, Result};
use super::module_side::{lib_manifest, path_dep};

/// 单个实现 crate 的块文本字节上限（§7.6：单个实现 crate rustc 峰值 ≤ 1.5 GB）
pub const BODY_CRATE_BYTES: usize = 5 << 20;

/// 实现 crate 至少分两箱：声明层编完元数据后实现层并行编译（`CARGO_BUILD_JOBS=2` 下两箱同时跑），
/// 小闭包（HelloWorld 一箱 3.7 MB）的实现层墙钟减半
pub const MIN_BODY_CRATES: usize = 2;

const BLOCK_OPEN: &str = "rava_macros::java_class! {\n";

/// 实现 crate 的 lib.rs（`decl`：根模块声明层 crate 名）
fn body_lib(decl: &str) -> String {
    format!(
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, \
         non_camel_case_types, non_upper_case_globals, static_mut_refs, unused_comparisons)]\n\
         // 声明层全部公开项（`crate::java::…` / `crate::prelude` 经此解析到声明层）\n\
         use {decl}::*;\n\
         pub mod body;\n"
    )
}

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

/// 一个类文本拆成 (声明层文本, 实现层文本)；块不存在时 Ok(None)。
/// 声明层剥去下沉方法体（判定与宏同一份代码，`rava_macros_core::plan`）。
pub fn split_text(text: &str, decl_crate: &str, module: &str) -> std::result::Result<Option<(String, String)>, String> {
    let Some(open) = text.find(BLOCK_OPEN) else { return Ok(None) };
    let start = open + BLOCK_OPEN.len();
    // 块以第 0 列的 `}` 行结束（块内各项至少缩进 4 列）
    let Some(end) = text[start..].find("\n}\n") else { return Ok(None) };
    let inner_end = start + end + 1;
    let close = start + end + 3;
    let plan = decl_elisions(&text[start..inner_end])?;
    let decl_block = elide(&text[start..inner_end], &plan);
    let decl = format!(
        "{}{BLOCK_OPEN}    #[rava_layer = \"decl\"]\n{}{}",
        prune_uses(&text[..open], &[&decl_block, &text[inner_end..]]),
        decl_block,
        &text[inner_end..]
    );
    let body = format!(
        "{}use {decl_crate}::{module}::*;\n\n{BLOCK_OPEN}    #[rava_layer = \"body\"]\n{}",
        &text[..open],
        &text[start..close]
    );
    Ok(Some((decl, body)))
}

/// 声明层文件头只留被本文件其余文本引用的单名 `use crate::…::名;`（拆 crate §7.5.4 #3）。
///
/// 剥体后，只被方法体引用的类型导入在声明层不再有引用者，却仍要逐条进名称解析；实现层文件头保留全部导入。
/// 引用判定按词：本文件其余文本（块、块后宏，含属性字符串，`$` 视同 `_`）出现该名，或该名形如
/// `<词>__…`（宏按类名派生的 vtable trait / base 函数等）且 `<词>` 出现，即保留；`__` 开头、glob、
/// 花括号与路径重导出一律保留。只会多留、不会误删：宏派生的名字都由块内出现的类名拼出。
fn prune_uses(header: &str, rest: &[&str]) -> String {
    let mut words: BTreeSet<String> = BTreeSet::new();
    let single = |line: &str| -> Option<String> {
        let path = line.strip_prefix("use crate::")?.strip_suffix(';')?;
        let name = path.rsplit("::").next()?;
        (path.contains("::") && !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
            .then(|| name.to_string())
    };
    let mut add_words = |s: &str| {
        for w in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$')).filter(|w| !w.is_empty()) {
            words.insert(w.replace('$', "_"));
        }
    };
    for line in header.lines().filter(|l| single(l.trim_end()).is_none()) {
        add_words(line);
    }
    rest.iter().for_each(|s| add_words(s));
    let keep = |name: &str| {
        name.starts_with("__") || words.contains(name) || name.split_once("__").is_some_and(|(stem, _)| words.contains(stem))
    };
    let mut out = String::with_capacity(header.len());
    for line in header.split_inclusive('\n') {
        if single(line.trim_end()).is_some_and(|n| !keep(&n)) {
            continue;
        }
        out.push_str(line);
    }
    out
}

/// 改写 `ems` 中可拆类的文本为声明层，返回实现层装箱结果
pub fn split(ctx: &EmitCtx<'_>, ems: &mut IndexMap<String, ClassEmission>, decl_src: &Path) -> Result<BodyPlan> {
    let crates = ctx.crates();
    let decl = crates.decl();
    // 可拆类：(ems 下标, 相对路径)
    let mut cands: Vec<(usize, PathBuf)> = Vec::new();
    for (i, em) in ems.values().enumerate() {
        if em.crate_name != crates.root() || em.handwritten {
            continue;
        }
        // 接口与不透明（L1）类不拆：后者只有类型身份（`java_class_opaque!`，无方法体 / 存储层），整类留声明层
        if ctx.class(&em.binary_name).is_none_or(|ci| ci.is_interface()) || ctx.is_opaque(&em.binary_name) {
            continue;
        }
        let Ok(rel) = em.path.strip_prefix(decl_src) else { continue };
        cands.push((i, rel.to_path_buf()));
    }
    // 剥体计划要解析整块（与宏同一解析器），按类并行
    let split: Vec<std::result::Result<Option<(String, String)>, String>> = {
        let ems_ref = &*ems;
        crate::par::par_map(crate::par::resolve_jobs(ctx.opts.jobs), &cands, |(i, rel)| {
            let em = &ems_ref[*i];
            split_text(&em.text, &decl, &module_path(rel)).map_err(|e| format!("{}：剥体计划失败：{e}", em.binary_name))
        })
    };
    let mut bodies: Vec<(String, PathBuf, String)> = Vec::new();
    for ((i, rel), r) in cands.into_iter().zip(split) {
        let Some((decl, body)) = r.map_err(EmitError::Input)? else { continue };
        let em = &mut ems[i];
        em.text = decl;
        bodies.push((em.binary_name.clone(), rel, body));
    }
    bodies.sort_by(|a, b| a.0.cmp(&b.0));
    let sizes: Vec<usize> = bodies.iter().map(|b| b.2.len()).collect();
    let bins = pack(&sizes, BODY_CRATE_BYTES);
    let mut plan = BodyPlan::default();
    for ((_, rel, body), bin) in bodies.into_iter().zip(bins) {
        while plan.crates.len() <= bin {
            let name = crates.body(plan.crates.len() + 1);
            plan.crates.push(BodyCrate { name, files: BTreeMap::new() });
        }
        plan.crates[bin].files.insert(rel, body);
    }
    Ok(plan)
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
    // 公开：构建期引导映像（根门面 crate）按路径引用实现层的 `X__inner`
    if is_rust_keyword(name) { format!("pub mod r#{name};") } else { format!("pub mod {name};") }
}

impl BodyPlan {
    /// 各实现 crate 落盘：类文件、`body` 模块树（只声明不导出）、lib.rs、Cargo.toml；
    /// 清除本轮未写入的带生成标记的陈旧类文件（复用 scratch）
    /// 实现层各类文件的 (落盘路径, 文本)
    pub fn files(&self, out_dir: &Path) -> Vec<(PathBuf, &str)> {
        let mut out = Vec::new();
        for c in &self.crates {
            let body_root = out_dir.join(&c.name).join("src").join("body");
            out.extend(c.files.iter().map(|(rel, t)| (body_root.join(rel), t.as_str())));
        }
        out
    }

    /// `top`：声明层末段 crate（以声明层名改名引入，类文件头 `use <声明层>::…` 不变）
    pub fn write_crates(&self, ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, top: &str) -> Result<()> {
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
            let decl = ctx.crates().decl();
            w.write(&src.join("lib.rs"), &body_lib(&decl))?;
            let deps = [path_dep(&decl, top), format!("rava_macros     = {{ path = \"{}\" }}", ctx.macros_crate.display())];
            w.write(&dir.join("Cargo.toml"), &lib_manifest(&dir, &c.name, &deps))?;
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
        let (decl, body) = split_text(text, "rt_decl", "a::b").unwrap().unwrap();
        assert_eq!(
            decl,
            "#![allow(x)]\nuse crate::prelude::*;\n\nrava_macros::java_class! {\n    #[rava_layer = \"decl\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n// tail\nrava_macros::iface_upcasts! { impl B => C }\n"
        );
        assert_eq!(
            body,
            "#![allow(x)]\nuse crate::prelude::*;\n\nuse rt_decl::a::b::*;\n\nrava_macros::java_class! {\n    #[rava_layer = \"body\"]\n    #[binary_name = \"a/B\"]\n    pub struct B {}\n}\n"
        );
        assert_eq!(module_path(Path::new("java/lang/ref/reference.rs")), "java::lang::r#ref::reference");
    }

    #[test]
    fn decl_uses_pruned_to_referenced() {
        let header = "#![allow(x)]\nuse crate::prelude::*;\nuse crate::a::Used;\nuse crate::a::OnlyInBody;\n\
                      use crate::a::Anc__VTable;\nuse crate::a::Outer_Inner;\nuse crate::a::__helper;\nuse crate::a::{X, Y};\n\n";
        let block = "    #[superclass = \"Anc\"]\n    #[nest_members = \"a/Outer$Inner\"]\n    pub struct B {}\n    \
                     impl B {\n        pub fn f(&self, u: Used) -> Result<()>;\n    }\n";
        assert_eq!(
            prune_uses(header, &[block, "}\n"]),
            "#![allow(x)]\nuse crate::prelude::*;\nuse crate::a::Used;\nuse crate::a::Anc__VTable;\n\
             use crate::a::Outer_Inner;\nuse crate::a::__helper;\nuse crate::a::{X, Y};\n\n"
        );
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
