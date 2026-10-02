//! 文件布局（`write_cargo_project` 的路径 / 包表 / mod 树计算段）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::ctx::EmitCtx;
use crate::text::{pkg_from_java, to_snake};


/// java_runtime 侧布局
#[derive(Debug, Default)]
pub struct JdkLayout {
    /// 类 → 文件路径（闭包序）
    pub files: IndexMap<String, PathBuf>,
    /// 本轮生成的 JDK 类
    pub generated: BTreeSet<String>,
}

fn pkg_parts(bin: &str) -> Vec<&str> {
    let mut v: Vec<&str> = bin.split('/').collect();
    v.pop();
    v
}

impl JdkLayout {
    pub fn build(ctx: &EmitCtx<'_>, jrt_src: &Path) -> JdkLayout {
        let classes = &ctx.input.jdk_classes;
        // 父目录 → 子包目录名（类名与子包同名 → `_t` 后缀）
        let mut pkg_dirs: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
        for c in classes {
            let mut parent = jrt_src.to_path_buf();
            for p in pkg_parts(c) {
                pkg_dirs.entry(parent.clone()).or_default().insert(p.to_string());
                parent = parent.join(p);
            }
        }
        let mut lay = JdkLayout::default();
        for c in classes {
            let parts = pkg_parts(c);
            let simple = c.rsplit('/').next().unwrap_or(c);
            let parent = parts.iter().fold(jrt_src.to_path_buf(), |d, p| d.join(p));
            let mut m = to_snake(simple);
            if pkg_dirs.get(&parent).is_some_and(|s| s.contains(&m)) {
                m.push_str("_t");
            }
            lay.files.insert(c.clone(), parent.join(format!("{m}.rs")));
            lay.generated.insert(c.clone());
        }
        lay
    }
}

/// 用户类位置
#[derive(Debug, Clone)]
pub struct UserEntry {
    pub path: PathBuf,
    pub pkg_parts: Vec<String>,
    pub mod_name: String,
}

/// user crate 侧布局
#[derive(Debug, Default)]
pub struct UserLayout {
    /// 类 → 位置（用户类序）
    pub entries: IndexMap<String, UserEntry>,
    /// 目录 → 子模块
    pub mod_tree: BTreeMap<PathBuf, BTreeSet<String>>,
    /// 目录 → (模块名, 类型名) 再导出
    pub reexport: BTreeMap<PathBuf, BTreeSet<(String, String)>>,
}

impl UserLayout {
    pub fn build(ctx: &EmitCtx<'_>, user_src: &Path) -> UserLayout {
        let by_source: BTreeMap<String, String> = ctx
            .opts
            .java_files
            .iter()
            .filter_map(|f| Some((f.file_name()?.to_string_lossy().into_owned(), pkg_from_java(f))))
            .collect();
        let mut lay = UserLayout::default();
        for c in &ctx.input.user_classes {
            let Some(ci) = ctx.class(c) else { continue };
            let pkg = ci
                .class_file()
                .source_file
                .as_ref()
                .and_then(|s| by_source.get(s))
                .cloned()
                .unwrap_or_default();
            let pkg_parts: Vec<String> = if pkg.is_empty() { Vec::new() } else { pkg.split('.').map(str::to_string).collect() };
            // 模块名取简单名的 snake（不取完整 binary name：含包时路径形态异常）
            let simple = c.rsplit('/').next().unwrap_or(c);
            let mod_name = to_snake(simple);
            let dir = pkg_parts.iter().fold(user_src.to_path_buf(), |d, p| d.join(p));
            let path = dir.join(format!("{mod_name}.rs"));
            let mut parent = user_src.to_path_buf();
            for p in &pkg_parts {
                lay.mod_tree.entry(parent.clone()).or_default().insert(p.clone());
                parent = parent.join(p);
            }
            lay.mod_tree.entry(parent.clone()).or_default().insert(mod_name.clone());
            lay.reexport.entry(parent).or_default().insert((mod_name.clone(), ctx.declared(c)));
            lay.entries.insert(c.clone(), UserEntry { path, pkg_parts, mod_name });
        }
        lay
    }

    /// 同 crate 兄弟类（结构化引用集的 user crate 部分）：`referenced` 为字节码引用集
    /// （`collect_referenced(ci, None)`），另经 InnerClasses 补内部类 / 外部类 / 同级内部类
    pub fn sibling_set(&self, ctx: &EmitCtx<'_>, cls: &str, referenced: &BTreeSet<String>) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        let mut add = |name: &str| {
            if name != cls && self.entries.contains_key(name) {
                out.insert(name.to_string());
            }
        };
        for r in referenced {
            add(r);
        }
        let Some(ci) = ctx.class(cls) else { return out };
        for ic in ci.inner_classes() {
            add(&ic.inner);
        }
        let outer = ci.inner_classes().iter().find(|ic| ic.inner == cls).and_then(|ic| ic.outer.clone());
        if let Some(outer) = outer.filter(|o| self.entries.contains_key(o)) {
            add(&outer);
            if let Some(oci) = ctx.class(&outer) {
                for ic in oci.inner_classes() {
                    add(&ic.inner);
                }
            }
        }
        out
    }
}
