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
use rava_macros_core::plan::{decl_plan, elide};
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
    /// 类 → 其实现层模块的绝对路径（`::<实现 crate>::body::<模块>`；存储类型 `X__inner` 在此）
    pub homes: BTreeMap<String, String>,
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
/// 方法 trait 匿名导入（`use crate::…::X__Methods as _;`，§9.9 K3）在声明层文件头的保留策略
#[derive(Debug, Default)]
pub enum AnonKeep {
    /// 只按词剪枝（纯外壳文件：无留体 fn）
    #[default]
    Word,
    /// 全保留（留体 fn 缺 java_method 属性、无法按名差分）
    All,
    /// 词干出现或名字在集合中才保留（留体 fn 的精确需求集，名字 = `<声明名>__Methods`）
    Set(BTreeSet<String>),
}

/// 一个类文本拆成 (声明层文本, 实现层文本)；块不存在时 Ok(None)。
/// 声明层剥去下沉方法体（判定与宏同一份代码，`rava_macros_core::plan`）。
/// `anon_of`：由剥体计划给出的留体 fn（[`StayBodies`]）构造匿名导入保留策略
pub fn split_text(
    text: &str,
    decl_crate: &str,
    module: &str,
    anon_of: impl FnOnce(&rava_macros_core::plan::StayBodies) -> AnonKeep,
) -> std::result::Result<Option<(String, String)>, String> {
    let Some(open) = text.find(BLOCK_OPEN) else { return Ok(None) };
    let start = open + BLOCK_OPEN.len();
    // 块以第 0 列的 `}` 行结束（块内各项至少缩进 4 列）
    let Some(end) = text[start..].find("\n}\n") else { return Ok(None) };
    let inner_end = start + end + 1;
    let close = start + end + 3;
    let (plan, stay) = decl_plan(&text[start..inner_end])?;
    let anon = anon_of(&stay);
    let decl_block = elide(&text[start..inner_end], &plan);
    let decl = format!(
        "{}{BLOCK_OPEN}    #[rava_layer = \"decl\"]\n{}{}",
        prune_uses(&text[..open], &[&decl_block, &text[inner_end..]], &anon),
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
/// 方法 trait 匿名导入（`use crate::…::X__Methods as _;`，§9.9 K3）同单名规则：词干 `X` 出现才保留。
/// 纯外壳文件里，声明层的跨类方法调用都出自宏生成的外壳（继承转发 / 反射分派尾巴等），其所指类的
/// 二进制名或 Rust 名必在属性串、签名或尾巴文本中出现，按词剪枝不漏。**例外**：留在声明层的方法体
/// （wrapper 上下文体不可下沉）按推断类型解析方法调用，接收者类名可以不在文本中出现——这类体的
/// 导入需求按 [`AnonKeep::Set`] 差分保留（`ProjectState::method_traits`），缺属性时退全保留。
fn prune_uses(header: &str, rest: &[&str], anon: &AnonKeep) -> String {
    let mut words: BTreeSet<String> = BTreeSet::new();
    let single = |line: &str| -> Option<String> {
        let path = line.strip_prefix("use crate::")?.strip_suffix(';')?;
        let a = path.strip_suffix(" as _");
        // 方法 trait 匿名导入 `use crate::…::X__Methods as _;`：名取 `X__Methods`；
        // 全保留策略下不作单名处理（不参与剪枝，也不贡献词）
        if a.is_some() && matches!(anon, AnonKeep::All) {
            return None;
        }
        let path = a.unwrap_or(path);
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
        words.contains(name)
            || name.split_once("__").is_some_and(|(stem, _)| words.contains(stem))
            || name.starts_with("__")
            || matches!(anon, AnonKeep::Set(s) if s.contains(name))
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

/// 改写 `ems` 中可拆类的文本为声明层，返回实现层装箱结果。
/// `method_traits`：方法体所需的方法 trait 账本（`(类, 方法名, 描述符) → 目标类`），留体 fn 的
/// 匿名导入按它精确保留
pub fn split(
    ctx: &EmitCtx<'_>,
    ems: &mut IndexMap<String, ClassEmission>,
    decl_src: &Path,
    method_traits: &BTreeMap<(String, String, String), BTreeSet<String>>,
) -> Result<BodyPlan> {
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
            let bin = em.binary_name.clone();
            let ledger = method_traits;
            split_text(&em.text, &decl, &module_path(rel), |stay| match stay {
                rava_macros_core::plan::StayBodies::None => AnonKeep::Word,
                rava_macros_core::plan::StayBodies::Unknown => AnonKeep::All,
                rava_macros_core::plan::StayBodies::Fns(keys) => AnonKeep::Set(
                    keys.iter()
                        .filter_map(|k| ledger.get(&(bin.clone(), k.0.clone(), k.1.clone())))
                        .flatten()
                        .map(|b| format!("{}__Methods", ctx.declared(b)))
                        .collect(),
                ),
            })
            .map_err(|e| format!("{}：剥体计划失败：{e}", em.binary_name))
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
    for ((cls, rel, body), bin) in bodies.into_iter().zip(bins) {
        while plan.crates.len() <= bin {
            let name = crates.body(plan.crates.len() + 1);
            plan.crates.push(BodyCrate { name, files: BTreeMap::new() });
        }
        plan.homes.insert(cls, format!("::{}::body::{}", plan.crates[bin].name, module_path(&rel)));
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
        let (decl, body) = split_text(text, "rt_decl", "a::b", |_| AnonKeep::Word).unwrap().unwrap();
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
            prune_uses(header, &[block, "}\n"], &AnonKeep::Word),
            "#![allow(x)]\nuse crate::prelude::*;\nuse crate::a::Used;\nuse crate::a::Anc__VTable;\n\
             use crate::a::Outer_Inner;\nuse crate::a::__helper;\nuse crate::a::{X, Y};\n\n"
        );
    }

    /// 方法 trait 匿名导入按词剪枝（§9.9 K3）：声明层文本出现词干才保留，只被方法体使用的剪去
    #[test]
    fn method_trait_imports_pruned_by_stem_word() {
        let header = "#![allow(x)]\nuse crate::prelude::*;\n\
                      use crate::a::Anc__Methods as _;\nuse crate::a::OnlyInBody__Methods as _;\n\
                      use crate::a::Map_Entry__Methods as _;\nuse crate::a::{X, Y};\n\n";
        // 词干来源：超类属性串（Anc）、反射分派尾巴（`Map_Entry::new(..)`，Map$Entry 的 Rust 名）
        let block = "    #[superclass = \"a/Anc\"]\n    pub struct B {}\n";
        let tail = "// tail\nimpl B { pub fn __reflect_dispatch(_: &str) { let _ = Map_Entry::new(); } }\n";
        let pruned = prune_uses(header, &[block, tail], &AnonKeep::Word);
        assert!(pruned.contains("use crate::a::Anc__Methods as _;"), "{pruned}");
        assert!(pruned.contains("use crate::a::Map_Entry__Methods as _;"), "{pruned}");
        assert!(!pruned.contains("OnlyInBody"), "{pruned}");
        // 词干不出现在声明层任何位置（`$` 词干按 `_` 折算）时剪去
        let header2 = "#![allow(x)]\nuse crate::prelude::*;\nuse crate::a::Other__Methods as _;\nuse crate::a::Other;\n\n";
        let pruned2 = prune_uses(header2, &[block, tail], &AnonKeep::Word);
        assert!(!pruned2.contains("Other"), "{pruned2}");
        // 留有方法体的文件：方法调用按推断类型解析，匿名导入全部保留
        let pruned3 = prune_uses(header2, &[block, tail], &AnonKeep::All);
        // 留体 fn 的差分需求集：名字在集合中即保留（词干可不出现在文本里）
        let set = AnonKeep::Set(["Other__Methods".to_string()].into_iter().collect());
        let pruned4 = prune_uses(header2, &[block, tail], &set);
        assert!(pruned4.contains("use crate::a::Other__Methods as _;"), "{pruned4}");
        assert!(!pruned4.contains("use crate::a::Other;"), "{pruned4}");
        assert!(pruned3.contains("use crate::a::Other__Methods as _;"), "{pruned3}");
        // 类型导入仍按词剪枝（推断不需要类型名）
        assert!(!pruned3.contains("use crate::a::Other;"), "{pruned3}");
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
