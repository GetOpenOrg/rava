//! 模块图：类 → 模块归属与模块 requires 图（T1 第 2 步 M1，方案 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md` §3.1）。
//!
//! 全部事实来自类路径上的档案本身，不含任何模块名 / 类名字面量：
//!
//! - **具名模块**：档案根的 `module-info.class`（jmod / 模块化 jar / 带描述符的类目录）；镜像改写类目录按其登记的模块；
//! - **自动模块**：无描述符的库 jar——清单 `Automatic-Module-Name`，缺省按 JPMS 的 jar 文件名推导规则命名；
//! - **无名模块**：用户类目录与其余无描述符档案（模块名为 None）。
//!
//! 类的归属：所在档案有模块名即取之；档案无模块（镜像独有类、VM 支持类目录）或类不在类路径上（lambda / 隐藏类 /
//! 代理类等运行期定义类，与宿主同包）时，按**包**归属——JPMS 中一个包只属于一个具名模块；仍无归属即无名模块。
//!
//! 模块可读性取 requires 的传递闭包（运行期 requires 与 `requires static` 都计入：代码可引用编译期依赖），
//! 是可读性的上界；生成代码的跨模块引用只允许落在其中（M2 起即 crate 依赖 ⊆ 该闭包）。
//! 无名模块与自动模块读全部模块；具名模块读不到无名模块。

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::classpath::{ClassPath, Origin};
use crate::hierarchy::package_of;

/// 模块图中的一个模块
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModuleNode {
    /// 自动模块（无描述符的库 jar）：读全部模块
    pub automatic: bool,
    /// 直接 requires（运行期 + static；按名序）
    pub requires: BTreeSet<String>,
    /// requires 传递闭包（不含自身；含类路径上不存在的模块名）
    pub upstream: BTreeSet<String>,
}

/// 模块事实（自有数据，可长期缓存）；查询经 [`ModuleFacts::graph`] 与类路径组成的视图
#[derive(Debug, Default)]
pub struct ModuleFacts {
    /// 档案下标 → 模块名（具名或自动；无名为 None）
    archive_module: Vec<Option<String>>,
    nodes: BTreeMap<String, ModuleNode>,
    /// 包 → 具名模块（多档案同包时取加入序最前者）
    package_owner: HashMap<String, (usize, String)>,
}

/// 模块图查询视图：模块事实 + 其来源类路径
#[derive(Clone, Copy)]
pub struct ModuleGraph<'a> {
    cp: &'a ClassPath,
    f: &'a ModuleFacts,
}

impl ModuleFacts {
    pub fn build(cp: &ClassPath) -> ModuleFacts {
        let views = cp.module_views();
        let paths = cp.archives();
        let mut archive_module = Vec::with_capacity(views.len());
        let mut nodes: BTreeMap<String, ModuleNode> = BTreeMap::new();
        for (idx, (view, (_, path))) in views.iter().zip(&paths).enumerate() {
            let name = if let Some(m) = cp.overlay_module(idx) {
                Some(m.to_string())
            } else if let Some(d) = &view.module {
                let node = nodes.entry(d.name.clone()).or_default();
                if node.requires.is_empty() && !node.automatic {
                    node.requires = d.requires.iter().chain(&d.requires_static).filter(|r| **r != d.name).cloned().collect();
                }
                Some(d.name.clone())
            } else if view.origin == Origin::Lib && path.is_file() {
                let manifest = cp.resource_in(idx, MANIFEST).unwrap_or_default();
                let name = manifest_attr(&String::from_utf8_lossy(&manifest), AUTOMATIC_NAME_ATTR)
                    .or_else(|| path.file_name().and_then(|f| f.to_str()).and_then(automatic_name_from_file));
                if let Some(n) = &name {
                    nodes.entry(n.clone()).or_insert_with(|| ModuleNode { automatic: true, ..Default::default() });
                }
                name
            } else {
                None
            };
            archive_module.push(name);
        }
        let mut package_owner: HashMap<String, (usize, String)> = HashMap::new();
        for (class, idx) in cp.indexed() {
            let Some(m) = archive_module.get(idx).and_then(Option::as_ref) else { continue };
            if nodes.get(m).is_some_and(|n| n.automatic) {
                continue;
            }
            let e = package_owner.entry(package_of(class).to_string()).or_insert_with(|| (idx, m.clone()));
            if idx < e.0 {
                *e = (idx, m.clone());
            }
        }
        close_upstream(&mut nodes);
        ModuleFacts { archive_module, nodes, package_owner }
    }

    /// 查询视图（`cp` 须是构建本事实的类路径）
    pub fn graph<'a>(&'a self, cp: &'a ClassPath) -> ModuleGraph<'a> {
        ModuleGraph { cp, f: self }
    }
}

impl<'a> ModuleGraph<'a> {

    /// 类所属模块（None = 无名模块）
    pub fn module_of(&self, class: &str) -> Option<&'a str> {
        if let Some(m) = self.cp.archive_of(class).and_then(|i| self.f.archive_module.get(i)).and_then(Option::as_deref) {
            return Some(m);
        }
        match self.cp.origin(class) {
            // 用户 / 库档案里无描述符的类：无名模块（不按包借用具名模块）
            Some(Origin::User | Origin::Lib) => None,
            _ => self.f.package_owner.get(package_of(class)).map(|(_, m)| m.as_str()),
        }
    }

    pub fn node(&self, module: &str) -> Option<&'a ModuleNode> {
        self.f.nodes.get(module)
    }

    /// 类路径上的全部模块（名序）
    pub fn nodes(&self) -> &'a BTreeMap<String, ModuleNode> {
        &self.f.nodes
    }

    /// `from` 模块的代码可否引用 `to` 模块的类（None = 无名模块）
    pub fn reads(&self, from: Option<&str>, to: Option<&str>) -> bool {
        let Some(f) = from else { return true };
        let Some(node) = self.f.nodes.get(f) else { return to == Some(f) };
        if node.automatic {
            return true;
        }
        match to {
            None => false,
            Some(t) => t == f || node.upstream.contains(t),
        }
    }

    /// 类 `from` 可否引用类 `to`
    pub fn class_reads(&self, from: &str, to: &str) -> bool {
        self.reads(self.module_of(from), self.module_of(to))
    }

    /// 给定模块集按 requires 的拓扑序（上游在前，同层按名序）；也是 VM 登记序
    pub fn topo<'s>(&self, set: impl IntoIterator<Item = &'s str>) -> Vec<String> {
        let deps: BTreeMap<String, BTreeSet<String>> = set
            .into_iter()
            .map(|m| (m.to_string(), self.f.nodes.get(m).map(|n| n.upstream.clone()).unwrap_or_default()))
            .collect();
        topo_order(&deps)
    }
}

/// 模块 → 上游集的拓扑序：上游在前，同层按名序；上游中不在键集里的名字忽略。
/// requires 无环（JPMS 保证）；万一有环，取名序最小者破环，保证终止
pub fn topo_order(deps: &BTreeMap<String, BTreeSet<String>>) -> Vec<String> {
    let mut pending: BTreeMap<&str, BTreeSet<&str>> = deps
        .iter()
        .map(|(m, d)| (m.as_str(), d.iter().map(String::as_str).filter(|x| deps.contains_key(*x) && *x != m).collect()))
        .collect();
    let mut out = Vec::with_capacity(deps.len());
    while !pending.is_empty() {
        let mut ready: Vec<&str> = pending.iter().filter(|(_, d)| d.is_empty()).map(|(m, _)| *m).collect();
        if ready.is_empty() {
            ready = pending.keys().take(1).copied().collect();
        }
        for m in &ready {
            pending.remove(m);
            out.push(m.to_string());
        }
        for d in pending.values_mut() {
            for m in &ready {
                d.remove(m);
            }
        }
    }
    out
}

const MANIFEST: &str = "META-INF/MANIFEST.MF";
const AUTOMATIC_NAME_ATTR: &str = "Automatic-Module-Name";

/// requires 传递闭包（迭代至不动点；不含自身）
fn close_upstream(nodes: &mut BTreeMap<String, ModuleNode>) {
    let direct: BTreeMap<String, BTreeSet<String>> = nodes.iter().map(|(k, n)| (k.clone(), n.requires.clone())).collect();
    for (name, node) in nodes.iter_mut() {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut stack: Vec<&String> = direct.get(name).into_iter().flatten().collect();
        while let Some(m) = stack.pop() {
            if m != name && seen.insert(m.clone()) {
                stack.extend(direct.get(m).into_iter().flatten());
            }
        }
        node.upstream = seen;
    }
}

/// jar 清单主段的属性值（续行以单个空格起首）
fn manifest_attr(text: &str, key: &str) -> Option<String> {
    let mut lines: Vec<String> = Vec::new();
    for ln in text.lines() {
        let ln = ln.trim_end_matches('\r');
        if ln.is_empty() {
            break;
        }
        match (ln.strip_prefix(' '), lines.last_mut()) {
            (Some(cont), Some(last)) => last.push_str(cont),
            _ => lines.push(ln.to_string()),
        }
    }
    lines.iter().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        (k.trim().eq_ignore_ascii_case(key)).then(|| v.trim().to_string()).filter(|v| !v.is_empty())
    })
}

/// JPMS 自动模块名推导（`ModuleFinder.of`）：去 `.jar`；自首个 `-<数字>`（其后为 `.` 或结尾）起截去版本；
/// 非字母数字换 `.`，连续 `.` 合一，去首尾 `.`。结果为空时 None
pub fn automatic_name_from_file(file: &str) -> Option<String> {
    let stem = file.strip_suffix(".jar").unwrap_or(file);
    let b = stem.as_bytes();
    let mut cut = b.len();
    for i in 0..b.len() {
        if b[i] != b'-' {
            continue;
        }
        let digits = b[i + 1..].iter().take_while(|c| c.is_ascii_digit()).count();
        let after = i + 1 + digits;
        if digits > 0 && (after == b.len() || b[after] == b'.') {
            cut = i;
            break;
        }
    }
    let mut out = String::new();
    for c in stem[..cut].chars() {
        let c = if c.is_ascii_alphanumeric() { c } else { '.' };
        if !(c == '.' && (out.is_empty() || out.ends_with('.'))) {
            out.push(c);
        }
    }
    while out.ends_with('.') {
        out.pop();
    }
    (!out.is_empty()).then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automatic_names_follow_jpms_rules() {
        assert_eq!(automatic_name_from_file("commons-io-2.11.0.jar").as_deref(), Some("commons.io"));
        assert_eq!(automatic_name_from_file("foo-bar-1.jar").as_deref(), Some("foo.bar"));
        assert_eq!(automatic_name_from_file("foo-1x.jar").as_deref(), Some("foo.1x"));
        assert_eq!(automatic_name_from_file("__a..b_.jar").as_deref(), Some("a.b"));
        assert_eq!(automatic_name_from_file("-1.jar"), None);
    }

    #[test]
    fn manifest_main_section_with_continuation() {
        let mf = "Manifest-Version: 1.0\r\nAutomatic-Module-Name: org.ex\r\n ample.lib\r\n\r\nName: x\r\nAutomatic-Module-Name: no\r\n";
        assert_eq!(manifest_attr(mf, AUTOMATIC_NAME_ATTR).as_deref(), Some("org.example.lib"));
        assert_eq!(manifest_attr("Name: x\n", AUTOMATIC_NAME_ATTR), None);
    }

    #[test]
    fn upstream_is_transitive_and_topo_puts_upstream_first() {
        let mut nodes: BTreeMap<String, ModuleNode> = BTreeMap::new();
        let mut add = |n: &str, r: &[&str]| {
            nodes.insert(n.into(), ModuleNode { requires: r.iter().map(|s| s.to_string()).collect(), ..Default::default() });
        };
        add("a", &[]);
        add("b", &["a"]);
        add("c", &["b"]);
        add("d", &["a"]);
        close_upstream(&mut nodes);
        assert_eq!(nodes["c"].upstream, ["a", "b"].iter().map(|s| s.to_string()).collect());
        let cp = ClassPath::new();
        let f = ModuleFacts { archive_module: Vec::new(), nodes, package_owner: HashMap::new() };
        let g = f.graph(&cp);
        assert_eq!(g.topo(["d", "c", "b", "a"]), vec!["a", "b", "d", "c"]);
        assert!(g.reads(Some("c"), Some("a")) && !g.reads(Some("a"), Some("c")) && !g.reads(Some("b"), Some("d")));
        assert!(g.reads(None, Some("c")) && !g.reads(Some("a"), None) && g.reads(Some("a"), Some("a")));
    }

    /// 真 JDK：jmod 类按描述符归属；镜像独有类 / VM 支持类按包归属到具名模块；requires 闭包无环且含上游
    #[test]
    fn real_jdk_module_graph() {
        let Some(home) = crate::jdk::find_major(21) else { return };
        let mut cp = ClassPath::new();
        cp.add_jdk(&home).unwrap();
        let support = std::env::var_os("CARGO_MANIFEST_DIR").map(std::path::PathBuf::from).unwrap().join("../../../runtime/java_support");
        for d in crate::image::image_class_dirs(&home, &support) {
            cp.add(Origin::Image, &d).unwrap();
        }
        let f = ModuleFacts::build(&cp);
        let g = f.graph(&cp);
        let unowned: Vec<&str> = cp.indexed().filter(|(n, _)| g.module_of(n).is_none()).map(|(n, _)| n).take(5).collect();
        assert!(unowned.is_empty(), "无归属的 JDK / 镜像类：{unowned:?}");
        // 每个模块的 upstream 不含自身，且被 requires 的模块都先于它出现在拓扑序中
        let order = g.topo(g.nodes().keys().map(String::as_str));
        let pos: HashMap<&str, usize> = order.iter().enumerate().map(|(i, m)| (m.as_str(), i)).collect();
        for (m, n) in g.nodes() {
            assert!(!n.upstream.contains(m), "{m} 自环");
            for r in n.upstream.iter().filter(|r| pos.contains_key(r.as_str())) {
                assert!(pos[r.as_str()] < pos[m.as_str()], "{r} 应先于 {m}");
            }
        }
        // 根模块（无 requires 的具名模块）被其余全部 JDK 模块读到
        let roots: Vec<&String> = g.nodes().iter().filter(|(_, n)| n.requires.is_empty()).map(|(m, _)| m).collect();
        assert_eq!(roots.len(), 1, "{roots:?}");
        assert!(g.nodes().keys().all(|m| g.reads(Some(m), Some(roots[0]))));
    }
}
