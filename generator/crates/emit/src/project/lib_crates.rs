//! lib crate 发射（jar 输入模式 `--lib`；← `project_writer` 的 lib crate 段）。
//!
//! 每个 lib crate 一个 `crate-type = ["lib"]` 的独立 crate：类文件 + lib.rs 顶层包模块树 +
//! Java 可见性映射 + 按目标 crate 定向的引用。依赖方向 = 声明序（后声明者 path 依赖先声明者），
//! user crate 依赖全部 lib crate。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use super::fs::Writer;
use super::mod_tree::write_mod_tree;
use crate::class_writer::LibSite;
use crate::ctx::EmitCtx;
use crate::error::Result;
use crate::imports::CrateRoute;
use super::module_side::{lib_manifest, path_dep, root_decl_dep};
use crate::text::safe_pkg_part;

/// lib crate 的 lib.rs 属性行
const LIB_ALLOW: &str = "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, \
                         non_camel_case_types, non_upper_case_globals, static_mut_refs, ambiguous_glob_reexports, unused_comparisons)]";

/// 全部 lib crate 的布局与定向表
#[derive(Debug, Default)]
pub struct LibPlan {
    /// lib crate（声明序）→ 发射类集合
    pub routes: Vec<(String, BTreeSet<String>)>,
    /// 可导入的生成集：JDK 生成类 ∪ 全部 lib 类
    pub generated: BTreeSet<String>,
    /// lib crate（声明序）→ 类 → 文件路径（按类名排序）
    pub files: Vec<(String, IndexMap<String, PathBuf>)>,
}

/// 类文件路径：`<src>/<包…>/<模块名>.rs`（模块名见 `module_names`：与子包 / 包内类型名冲突时加 `_t`）
fn lib_files(src: &Path, classes: &[String]) -> IndexMap<String, PathBuf> {
    let modules = crate::module_names::lib_modules(classes);
    let mut sorted: Vec<&String> = classes.iter().collect();
    sorted.sort();
    let mut out = IndexMap::new();
    for c in sorted {
        let pkg = c.rsplit_once('/').map_or("", |(p, _)| p);
        let parent = pkg.split('/').filter(|p| !p.is_empty()).fold(src.to_path_buf(), |d, p| d.join(p));
        out.insert(c.clone(), parent.join(format!("{}.rs", modules[c.as_str()])));
    }
    out
}

impl LibPlan {
    pub fn build(ctx: &EmitCtx<'_>, out_dir: &Path, jdk_generated: &BTreeSet<String>) -> LibPlan {
        let mut plan = LibPlan { generated: jdk_generated.clone(), ..LibPlan::default() };
        for (name, classes) in &ctx.input.lib_crates {
            let set: BTreeSet<String> = classes.iter().cloned().collect();
            plan.generated.extend(set.iter().cloned());
            plan.routes.push((name.clone(), set));
            plan.files.push((name.clone(), lib_files(&out_dir.join(name).join("src"), classes)));
        }
        plan
    }

    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    /// lib 模式下类所在 crate 的定向参数（`current`：lib 名；None = user crate）；非 lib 模式 None
    pub fn site<'l>(&'l self, current: Option<&'l str>) -> Option<LibSite<'l>> {
        (!self.is_empty()).then(|| LibSite { route: CrateRoute { libs: &self.routes, current }, generated: &self.generated })
    }

    /// lib crate 名（声明序）
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.routes.iter().map(|(n, _)| n.as_str())
    }

    /// 类文件落盘后：各 lib crate 的包 mod 树、lib.rs、Cargo.toml
    pub fn write_crates(&self, ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, top: &str) -> Result<()> {
        for (i, (name, files)) in self.files.iter().enumerate() {
            let dir = out_dir.join(name);
            let src = dir.join("src");
            write_mod_tree(&src, None, crate::par::resolve_jobs(ctx.opts.jobs), w)?;
            let tops: BTreeSet<&str> = files.keys().filter_map(|c| c.split_once('/').map(|(t, _)| t)).collect();
            let mut lib_rs = vec![LIB_ALLOW.to_string()];
            lib_rs.extend(tops.iter().map(|t| format!("pub mod {};", safe_pkg_part(t))));
            w.write(&src.join("lib.rs"), &(lib_rs.join("\n") + "\n"))?;
            // JDK 依赖：根（以根名引入声明层）+ 全部非根模块 crate（库的 JDK 引用面不按模块细分）
            let crates = ctx.crates();
            let mut deps = vec![root_decl_dep(crates, top)];
            deps.extend(crates.others().iter().map(|c| path_dep(&c.name, &c.name)));
            deps.push(format!("rava_macros     = {{ path = \"{}\" }}", ctx.macros_crate.display()));
            deps.extend(self.routes[..i].iter().map(|(prev, _)| dep_line(prev)));
            w.write(&dir.join("Cargo.toml"), &lib_manifest(&dir, name, &deps))?;
        }
        Ok(())
    }
}

/// `{name:<15} = { path = "../name" }`
pub fn dep_line(name: &str) -> String {
    format!("{name:<15} = {{ path = \"../{name}\" }}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_file_layout_sorts_and_suffixes() {
        let src = Path::new("/s");
        let classes = vec!["org/b/Z".to_string(), "org/a/Core".to_string(), "org/a/core/X".to_string()];
        let f = lib_files(src, &classes);
        let got: Vec<(&str, &Path)> = f.iter().map(|(c, p)| (c.as_str(), p.as_path())).collect();
        assert_eq!(
            got,
            vec![
                ("org/a/Core", Path::new("/s/org/a/core_t.rs")),
                ("org/a/core/X", Path::new("/s/org/a/core/x.rs")),
                ("org/b/Z", Path::new("/s/org/b/z.rs")),
            ]
        );
        assert_eq!(dep_line("hamcrest"), "hamcrest        = { path = \"../hamcrest\" }");
    }
}
