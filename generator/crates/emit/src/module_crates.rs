//! 按 jmod 模块切分的 JDK crate 表（T1 第 2 步 M2，方案 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md` §2.6 / §4.8）。
//!
//! 全部事实来自模块图（[`resolve::ModuleGraph`]），不含模块名字面量：
//!
//! - 一个模块一个 crate，crate 名 = 模块名 `.` → `_`；
//! - **根模块** = 根类（`Object`）所在模块：运行时基础设施（prelude / error / meta / sync_model …）与手写根类
//!   都在其中；物理上拆为声明层 `<根>_decl` + 实现层 `<根>_body_k`，由门面 crate `<根>` 再导出；
//! - 无模块归属的 JDK 类（VM 支持类目录中包也无具名模块者）归根模块；
//! - crate 序 = 模块拓扑序（上游在前，根恒在首位）；crate 依赖 = requires 闭包 ∩ 本程序模块集（非根 crate 恒依赖根）。
//!
//! 引用路径的首段（[`ModuleCrates::head`]）：同 crate 写 `crate`，跨 crate 写目标 crate 名。基础设施
//! 路径在 JDK crate 内一律写 `crate::…`——非根 crate 的 lib.rs 私有 glob 导入根门面，基础设施名不与
//! Java 顶层包名相撞，故经 glob 解析到根；Java 类路径则必须 crate 正确（非根 crate 有自己的顶层包模块）。

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use resolve::hierarchy::package_of;
use resolve::ModuleGraph;

/// 无模块图（单元测试的裸类路径）时的单 crate 名
pub const SINGLE_CRATE: &str = "java_runtime";
/// 根模块声明层 crate 名后缀
const DECL_SUFFIX: &str = "_decl";
/// 根模块实现层 crate 名中缀（`<根>_body_<k>`）
const BODY_INFIX: &str = "_body_";

/// 一个模块 crate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleCrate {
    /// crate 名（模块名 `.` → `_`）
    pub name: String,
    /// 模块名（无模块图时 None）
    pub module: Option<String>,
    /// 依赖的模块 crate（拓扑序）
    pub deps: Vec<String>,
}

/// 本程序的 JDK 模块 crate 表
#[derive(Debug, Clone)]
pub struct ModuleCrates {
    /// 拓扑序；`[0]` = 根
    crates: Vec<ModuleCrate>,
    /// 类 → crate 下标（本程序 JDK 类）
    class_crate: HashMap<String, usize>,
    /// 包 → crate 下标（运行期定义类与宿主同包：lambda / 隐藏类 / 代理类）
    pkg_crate: HashMap<String, usize>,
}

/// 模块名 → crate 名
pub fn crate_name(module: &str) -> String {
    module.replace(['.', '-'], "_")
}

impl ModuleCrates {
    /// 单 crate 表（无模块图）
    pub fn single(name: &str) -> ModuleCrates {
        ModuleCrates {
            crates: vec![ModuleCrate { name: name.to_string(), module: None, deps: Vec::new() }],
            class_crate: HashMap::new(),
            pkg_crate: HashMap::new(),
        }
    }

    /// 由模块图与本程序的 JDK 类集构建；`root_class` 为根类（其模块即根模块）
    pub fn build<'c>(g: &ModuleGraph<'_>, root_class: &str, classes: impl IntoIterator<Item = &'c str>) -> ModuleCrates {
        let classes: Vec<&str> = classes.into_iter().collect();
        let Some(root) = g.module_of(root_class) else { return ModuleCrates::single(SINGLE_CRATE) };
        let mut set: BTreeSet<&str> = classes.iter().filter_map(|c| g.module_of(c)).collect();
        set.insert(root);
        let mut order: Vec<String> = g.topo(set.iter().copied());
        order.retain(|m| m != root);
        order.insert(0, root.to_string());
        let pos: HashMap<&str, usize> = order.iter().enumerate().map(|(i, m)| (m.as_str(), i)).collect();
        let crates = order
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let mut deps: Vec<usize> = g
                    .node(m)
                    .map(|n| n.upstream.iter().filter_map(|u| pos.get(u.as_str()).copied()).collect())
                    .unwrap_or_default();
                if i > 0 && !deps.contains(&0) {
                    deps.push(0);
                }
                deps.retain(|d| *d != i);
                deps.sort_unstable();
                ModuleCrate { name: crate_name(m), module: Some(m.clone()), deps: deps.into_iter().map(|d| crate_name(&order[d])).collect() }
            })
            .collect();
        let mut class_crate = HashMap::new();
        let mut pkg_crate = HashMap::new();
        for c in classes {
            let idx = g.module_of(c).and_then(|m| pos.get(m).copied()).unwrap_or(0);
            class_crate.insert(c.to_string(), idx);
            pkg_crate.entry(package_of(c).to_string()).or_insert(idx);
        }
        ModuleCrates { crates, class_crate, pkg_crate }
    }

    /// 全部模块 crate（拓扑序，根在首位）
    pub fn all(&self) -> &[ModuleCrate] {
        &self.crates
    }

    /// 根以外的模块 crate（单 crate，Full 模式）
    pub fn others(&self) -> &[ModuleCrate] {
        &self.crates[1..]
    }

    /// 根门面 crate 名（基础设施与根模块类的跨 crate 路径首段）
    pub fn root(&self) -> &str {
        &self.crates[0].name
    }

    /// 根模块声明层 crate 名
    pub fn decl(&self) -> String {
        format!("{}{DECL_SUFFIX}", self.root())
    }

    /// 根模块第 j 个上层声明段 crate 名（`<根>_decl_<j>`，j ≥ 1；底段即 [`Self::decl`]）
    pub fn decl_segment(&self, j: usize) -> String {
        format!("{}_{j}", self.decl())
    }

    /// 根模块第 k 个实现层 crate 名
    pub fn body(&self, k: usize) -> String {
        format!("{}{BODY_INFIX}{k}", self.root())
    }

    /// crate 的源码目录名：根 → 声明层 crate，其余 → crate 本身
    pub fn dir_of(&self, name: &str) -> String {
        if name == self.root() {
            self.decl()
        } else {
            name.to_string()
        }
    }

    /// crate 的源码树 `<out>/<目录>/src`
    pub fn src_dir(&self, out_dir: &Path, name: &str) -> PathBuf {
        out_dir.join(self.dir_of(name)).join("src")
    }

    /// 全部 JDK 源码树（根声明层在首位）
    pub fn src_dirs(&self, out_dir: &Path) -> Vec<PathBuf> {
        self.crates.iter().map(|c| self.src_dir(out_dir, &c.name)).collect()
    }

    /// 是否为模块 crate 名（含根门面）
    pub fn contains(&self, name: &str) -> bool {
        self.crates.iter().any(|c| c.name == name)
    }

    /// JDK 类所在 crate（本程序 JDK 类按其模块；运行期定义类按包；其余归根）
    pub fn crate_of(&self, class: &str) -> &str {
        let idx = self
            .class_crate
            .get(class)
            .or_else(|| self.pkg_crate.get(package_of(class)))
            .copied()
            .unwrap_or(0);
        &self.crates[idx].name
    }

    /// 包（`/` 分隔）所在 crate：本程序有该包的类 → 其 crate；否则 None
    pub fn crate_of_package(&self, pkg: &str) -> Option<&str> {
        self.pkg_crate.get(pkg).map(|i| self.crates[*i].name.as_str())
    }

    /// crate `here` 中引用 crate `target` 的路径首段
    pub fn head<'s>(here: &str, target: &'s str) -> &'s str {
        if here == target {
            "crate"
        } else {
            target
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_name_from_module() {
        assert_eq!(crate_name("a.b"), "a_b");
        assert_eq!(crate_name("x"), "x");
    }

    #[test]
    fn single_table() {
        let t = ModuleCrates::single("rt");
        assert_eq!(t.root(), "rt");
        assert_eq!(t.decl(), "rt_decl");
        assert_eq!(t.body(2), "rt_body_2");
        assert_eq!(t.decl_segment(1), "rt_decl_1");
        assert_eq!(t.crate_of("p/A"), "rt");
        assert!(t.others().is_empty());
        assert_eq!(ModuleCrates::head("rt", "rt"), "crate");
        assert_eq!(ModuleCrates::head("m", "rt"), "rt");
    }
}
