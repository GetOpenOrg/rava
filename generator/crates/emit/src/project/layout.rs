//! 文件布局（`write_cargo_project` 的路径 / 包表 / mod 树计算段）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::ctx::EmitCtx;
use crate::text::{pkg_from_java, safe_pkg_part, to_snake};

/// 不加入全局跨包 glob 导入的 JDK 包前缀（内部实现类，避免命名冲突）
const SKIP_GLOBAL_IMPORT_PREFIX: &str = "jdk/";

/// java_runtime 侧布局
#[derive(Debug, Default)]
pub struct JdkLayout {
    /// 类 → 文件路径（闭包序）
    pub files: IndexMap<String, PathBuf>,
    /// 跨包 glob 导入的 crate 包路径（`java::util` 形态，排序）
    pub pkg_paths: Vec<String>,
    /// 跳过全局导入但已生成的类完整路径（`jdk::internal::misc::X`）
    pub skipped_classes: BTreeSet<String>,
    /// 简单名 → 同名类所在包（`/` 分隔；≥2 个时入表）。
    /// Python 为 `list(set)`（顺序不定），此处排序
    pub conflict_map: BTreeMap<String, Vec<String>>,
    /// 本轮生成的 JDK 类
    pub generated: BTreeSet<String>,
}

fn pkg_parts(bin: &str) -> Vec<&str> {
    let mut v: Vec<&str> = bin.split('/').collect();
    v.pop();
    v
}

fn rust_pkg(parts: &[&str]) -> String {
    parts.iter().map(|p| safe_pkg_part(p)).collect::<Vec<_>>().join("::")
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
        let mut pkg_set = BTreeSet::new();
        let mut sn_to_pkgs: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for c in classes {
            let parts = pkg_parts(c);
            if !c.starts_with(SKIP_GLOBAL_IMPORT_PREFIX) && !parts.is_empty() {
                pkg_set.insert(rust_pkg(&parts));
            }
            if !parts.is_empty() {
                let simple = c.rsplit('/').next().unwrap_or(c);
                sn_to_pkgs.entry(simple.to_string()).or_default().push(parts.join("/"));
                if c.starts_with(SKIP_GLOBAL_IMPORT_PREFIX) {
                    lay.skipped_classes.insert(format!("{}::{}", rust_pkg(&parts), ctx.short(c)));
                }
            }
        }
        lay.pkg_paths = pkg_set.iter().cloned().collect();
        for (sn, pkgs) in sn_to_pkgs {
            let in_scope: BTreeSet<String> = pkgs
                .into_iter()
                .filter(|p| pkg_set.contains(&rust_pkg(&p.split('/').collect::<Vec<_>>())))
                .collect();
            if in_scope.len() >= 2 {
                lay.conflict_map.insert(sn, in_scope.into_iter().collect());
            }
        }
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
            // Python 以完整 binary name 取 snake（含包时路径形态异常，见 GOLDEN_DIFF）；此处取简单名
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
            lay.reexport.entry(parent).or_default().insert((mod_name.clone(), ctx.short(c)));
            lay.entries.insert(c.clone(), UserEntry { path, pkg_parts, mod_name });
        }
        lay
    }

    /// 同 crate 兄弟类导入（`use crate::[pkg::]mod::Type;`）：`referenced` 为字节码引用集
    /// （import_gen `collect_referenced(ci, None)`），另经 InnerClasses 补内部类 / 外部类 / 同级内部类
    pub fn sibling_imports(&self, ctx: &EmitCtx<'_>, cls: &str, referenced: &BTreeSet<String>) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        let mut add = |name: &str| {
            let Some(e) = self.entries.get(name) else { return };
            if name == cls {
                return;
            }
            let mut path = e.pkg_parts.clone();
            path.push(e.mod_name.clone());
            let line = format!("use crate::{}::{};", path.join("::"), ctx.short(name));
            if seen.insert(line.clone()) {
                out.push(line);
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
