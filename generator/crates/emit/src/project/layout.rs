//! 文件布局（`write_cargo_project` 的路径 / 包表 / mod 树计算段）。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::ctx::EmitCtx;

/// JDK 侧布局（各模块 crate）
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
    /// 各 JDK 类落到其模块 crate 的源码树（[`ModuleCrates::src_dir`]）下按包路径的文件
    pub fn build(ctx: &EmitCtx<'_>, out_dir: &Path) -> JdkLayout {
        let classes = &ctx.input.jdk_classes;
        let crates = ctx.crates();
        let mut lay = JdkLayout::default();
        for c in classes {
            // 模块名见 `module_names`：与子包 / 包内类型名 / 共置手写路径冲突时加 `_t`
            let parent = pkg_parts(c).iter().fold(crates.src_dir(out_dir, crates.crate_of(c)), |d, p| d.join(p));
            lay.files.insert(c.clone(), parent.join(format!("{}.rs", ctx.module_of(c))));
            lay.generated.insert(c.clone());
        }
        lay
    }
}

impl JdkLayout {
    /// 落入手写私有辅助目录（`closure::handwritten::layout`）的类：辅助目录不是包，Java 包段与宿主文件名
    /// 撞名即约定被破坏（手写文件名须改），返回首个冲突的类与目录
    pub fn helper_clash(&self, ctx: &EmitCtx<'_>, out_dir: &Path) -> Option<(String, PathBuf)> {
        let crates = ctx.crates();
        let runtime_src = ctx.runtime_src();
        self.files.iter().find_map(|(c, path)| {
            let rel = path.parent()?.strip_prefix(crates.src_dir(out_dir, crates.crate_of(c))).ok()?;
            let dir = closure::handwritten::layout::helper_root(&runtime_src, &runtime_src.join(rel))?;
            Some((c.clone(), dir))
        })
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
        let mut lay = UserLayout::default();
        let modules = ctx.class_modules();
        for c in &ctx.input.user_classes {
            let Some(pkg_parts) = modules.user_pkg(c).map(<[String]>::to_vec) else { continue };
            // 模块名见 `module_names`（包段取源文件 package 声明）
            let mod_name = ctx.module_of(c);
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
