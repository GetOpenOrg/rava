//! 类模块名（类文件 stem）分配：翻译对照文档「类模块名与类型名分离」一节的实现。
//!
//! 包模块的 mod.rs 以 `pub mod <m>; pub use <m>::*;` 声明并再导出每个类文件。Rust 的模块与
//! 类型同处类型命名空间，显式 `mod` 条目遮蔽 glob 再导出的同名类型：Java 类名本身全小写
//! （`p/foo`）时 snake 名与定义名同为 `foo`，`p::foo` 解析成模块，类型位置报 E0573、
//! `foo::new` / `foo::FIELD` 报 E0425。
//!
//! 规则（同一包目录内，按 binary 字典序逐个分配，结果只由该目录的类集决定）：
//! 类模块名 = snake(简单名)；落在保留名中时追加 `_t`，直到空闲。保留名 =
//! - 同目录子包名（目录与 `<m>.rs` 并存即 E0761）；
//! - 本包全部类的定义名（类型名，glob 再导出进同一命名空间）；
//! - 本包已分配给其他类的模块名（`FooBar` 与 `foo_bar` 同 snake）；
//! - 调用方的让出谓词（与共置手写 `x_impl.rs` / `x_ext.rs` 同路径，见 `project::layout`）。
//!
//! 驼峰类名的 snake 名必含小写化，与含大写的定义名天然不同，规则对它们是恒等的。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::ctx::EmitShared;
use crate::text::{pkg_from_java, to_snake};

/// 本次发射全部类的模块位置：JDK 类（按模块 crate）、lib 类（按 lib crate）、用户类（按源码 package）
#[derive(Debug, Default)]
pub struct ClassModules {
    /// binary → 模块名
    module: BTreeMap<String, String>,
    /// 用户类 → 包段（取 .java 源文件的 package 声明）
    user_pkg: BTreeMap<String, Vec<String>>,
}

impl ClassModules {
    pub fn build(ctx: &EmitShared<'_>) -> ClassModules {
        let mut out = ClassModules::default();
        // JDK：各模块 crate 一棵源码树
        let runtime_src = ctx.runtime_src();
        let crates = ctx.crates();
        let mut by_crate: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for c in &ctx.input.jdk_classes {
            by_crate.entry(crates.crate_of(c)).or_default().push(c);
        }
        for classes in by_crate.values() {
            let m = assign_tree(classes.iter().map(|c| (*c, bin_pkg_parts(c))), |pkg, m| companion_clash(&runtime_src, pkg, m));
            out.module.extend(m);
        }
        // lib crate：各自一棵源码树
        for (_, classes) in &ctx.input.lib_crates {
            out.module.extend(lib_modules(classes));
        }
        // 用户类：user crate 一棵源码树，包段取源文件的 package 声明
        let by_source: BTreeMap<String, String> = ctx
            .opts
            .java_files
            .iter()
            .filter_map(|f| Some((f.file_name()?.to_string_lossy().into_owned(), pkg_from_java(f))))
            .collect();
        for c in &ctx.input.user_classes {
            let Some(ci) = ctx.class(c) else { continue };
            let pkg = ci.class_file().source_file.as_ref().and_then(|s| by_source.get(s)).cloned().unwrap_or_default();
            let parts: Vec<String> = if pkg.is_empty() { Vec::new() } else { pkg.split('.').map(str::to_string).collect() };
            out.user_pkg.insert(c.clone(), parts);
        }
        out.module.extend(assign_tree(out.user_pkg.iter().map(|(c, p)| (c.as_str(), p.clone())), |_, _| false));
        out
    }

    /// 类的模块名（本次发射的类；其余 None）
    pub fn module_of(&self, binary: &str) -> Option<&str> {
        self.module.get(binary).map(String::as_str)
    }

    /// 用户类的包段（源文件 package 声明）
    pub fn user_pkg(&self, binary: &str) -> Option<&[String]> {
        self.user_pkg.get(binary).map(Vec::as_slice)
    }
}

/// lib crate 源码树的类模块名（包段取 binary）
pub fn lib_modules(classes: &[String]) -> BTreeMap<String, String> {
    assign_tree(classes.iter().map(|c| (c.as_str(), bin_pkg_parts(c))), |_, _| false)
}

/// 类名以 Impl / Ext 结尾时，snake 名与同包类 X 的共置手写 `x_impl.rs` / `x_ext.rs` 同名
/// （`Inet6AddressImpl` ↔ `Inet6Address` 的 native 手写 `inet6_address_impl.rs`）：手写真源同路径
/// 已有文件即为共置手写，生成类让出该路径，否则生成文件被当作手写而不落盘
fn companion_clash(runtime_src: &Path, parts: &[String], stem: &str) -> bool {
    (stem.ends_with("_impl") || stem.ends_with("_ext"))
        && parts.iter().fold(runtime_src.to_path_buf(), |d, p| d.join(p)).join(format!("{stem}.rs")).is_file()
}

/// 一个包目录内的类模块名：`classes` 为该目录的类（binary），`subpkgs` 为同目录子包名，
/// `yields(m)` 为真时让出 `m`。返回 binary → 模块名
pub fn assign<'a>(classes: impl IntoIterator<Item = &'a str>, subpkgs: &BTreeSet<String>, yields: impl Fn(&str) -> bool) -> BTreeMap<String, String> {
    let mut bins: Vec<&str> = classes.into_iter().collect();
    bins.sort_unstable();
    bins.dedup();
    let types: BTreeSet<String> = bins.iter().map(|b| ty::short_names::declared(b)).collect();
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut out = BTreeMap::new();
    for b in bins {
        let simple = b.rsplit('/').next().unwrap_or(b);
        let mut m = to_snake(simple);
        while subpkgs.contains(&m) || types.contains(&m) || taken.contains(&m) || yields(&m) {
            m.push_str("_t");
        }
        taken.insert(m.clone());
        out.insert(b.to_string(), m);
    }
    out
}

/// 整棵源码树的类模块名：`classes` 为 (binary, 包段)，按包段分目录逐个 [`assign`]；
/// 子包名取同树各类包段的下一级。`yields(包段, m)` 同 [`assign`]
pub fn assign_tree<'a>(classes: impl IntoIterator<Item = (&'a str, Vec<String>)>, yields: impl Fn(&[String], &str) -> bool) -> BTreeMap<String, String> {
    let mut groups: BTreeMap<Vec<String>, Vec<&str>> = BTreeMap::new();
    let mut subs: BTreeMap<Vec<String>, BTreeSet<String>> = BTreeMap::new();
    for (b, parts) in classes {
        for i in 0..parts.len() {
            subs.entry(parts[..i].to_vec()).or_default().insert(parts[i].clone());
        }
        groups.entry(parts).or_default().push(b);
    }
    let empty = BTreeSet::new();
    let mut out = BTreeMap::new();
    for (pkg, bins) in groups {
        out.extend(assign(bins, subs.get(&pkg).unwrap_or(&empty), |m| yields(&pkg, m)));
    }
    out
}

/// binary 的包段（按 `/` 切分去末段）
pub fn bin_pkg_parts(binary: &str) -> Vec<String> {
    let mut v: Vec<String> = binary.split('/').map(str::to_string).collect();
    v.pop();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(classes: &[&str]) -> BTreeMap<String, String> {
        assign_tree(classes.iter().map(|c| (*c, bin_pkg_parts(c))), |_, _| false)
    }

    #[test]
    fn camel_case_names_unchanged() {
        let m = tree(&["p/HashMap", "p/URLDecoder", "p/Map$Entry", "p/Type"]);
        assert_eq!(m["p/HashMap"], "hash_map");
        assert_eq!(m["p/URLDecoder"], "url_decoder");
        assert_eq!(m["p/Map$Entry"], "map_entry");
        assert_eq!(m["p/Type"], "type_");
    }

    #[test]
    fn lowercase_class_yields_type_name() {
        // 全小写类名：snake 名即定义名，模块让出类型名
        let m = tree(&["p/lr_parser", "p/sym", "p/Symbol", "p/virtual_parse_stack"]);
        assert_eq!(m["p/lr_parser"], "lr_parser_t");
        assert_eq!(m["p/sym"], "sym_t");
        assert_eq!(m["p/Symbol"], "symbol");
        assert_eq!(m["p/virtual_parse_stack"], "virtual_parse_stack_t");
    }

    #[test]
    fn other_class_type_name_is_reserved() {
        // `Foo_bar` 的 snake 名 `foo_bar` 是同包类 `foo_bar` 的类型名
        let m = tree(&["p/Foo_bar", "p/foo_bar"]);
        assert_eq!(m["p/Foo_bar"], "foo_bar_t");
        assert_eq!(m["p/foo_bar"], "foo_bar_t_t");
    }

    #[test]
    fn same_snake_and_suffix_chain_are_injective() {
        let m = tree(&["p/FooBar", "p/foo_bar", "p/foo_bar_t"]);
        assert_eq!(m["p/FooBar"], "foo_bar_t_t");
        assert_eq!(m["p/foo_bar"], "foo_bar_t_t_t");
        assert_eq!(m["p/foo_bar_t"], "foo_bar_t_t_t_t");
        let set: BTreeSet<&String> = m.values().collect();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn subpackage_and_yield() {
        let m = tree(&["org/a/Core", "org/a/core/X"]);
        assert_eq!(m["org/a/Core"], "core_t");
        assert_eq!(m["org/a/core/X"], "x");
        let y = assign_tree([("q/FooImpl", bin_pkg_parts("q/FooImpl"))], |pkg, m| pkg == ["q".to_string()] && m == "foo_impl");
        assert_eq!(y["q/FooImpl"], "foo_impl_t");
    }

    #[test]
    fn prelude_named_type_does_not_reserve_snake() {
        // 定义名取限定名（`p_Option`），snake 名 `option` 不受影响
        let m = tree(&["p/Option"]);
        assert_eq!(m["p/Option"], "option");
    }
}
