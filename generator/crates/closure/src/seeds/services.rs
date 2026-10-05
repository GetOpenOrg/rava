//! 服务目录事实（seeds.toml `[services]`）：`ServiceLoader` 按服务目录反射构造 provider，无静态调用边。
//!
//! 目录与 JVM 默认启动（类路径应用、无 `--add-modules` / `--limit-modules`）同构：
//! - 模块 provider：引导层（`ModuleBootstrap` 的 resolveAndBind）里各模块描述符的 `provides`。
//!   根集 = 系统镜像中至少有一个无限定 exports、且未标 `DO_NOT_RESOLVE_BY_DEFAULT` 的模块；
//!   沿运行期 requires（不含 `requires static`）闭包；绑定：已解析模块 `uses` 的服务，其 provider
//!   所在的可观察模块并入（连同其 requires 闭包），到不动点。
//! - 类路径 provider：用户 / 库档案的 `META-INF/services/<服务>`（类路径上的模块化 jar 按无名模块处理，
//!   其 module-info 不参与）。
//!
//! 入选：清单 `lookups` 成员（ServiceLoader 构造）可达，其服务 Class 实参值集里的类镜像即被查找的服务。

use std::collections::{BTreeMap, BTreeSet, HashMap};

use classfile::module::ModuleDecl;
use resolve::{ArchiveView, Origin};

#[derive(Debug, Default)]
pub struct ServicesCfg {
    /// 服务查找入口 `类.方法:描述符` → 服务 Class 形参序号（不含接收者，0 起）
    pub lookups: HashMap<String, usize>,
    /// 模块服务目录的装填入口（生成器在引导期按 `services` 事实调用）：有模块 provider 入选时作根
    pub population: Vec<String>,
}

impl ServicesCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let Some(sec) = sec else { return Self::default() };
        let lookups = sec
            .get("lookups")
            .and_then(|v| v.as_table())
            .map(|t| t.iter().filter_map(|(k, v)| Some((k.clone(), usize::try_from(v.as_integer()?).ok()?))).collect())
            .unwrap_or_default();
        let population = sec
            .get("population")
            .and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
            .unwrap_or_default();
        ServicesCfg { lookups, population }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Provider {
    /// 所在模块（None = 类路径）
    pub module: Option<String>,
    pub class: String,
}

/// 服务（内部名）→ provider（模块 provider 在前，按可观察模块序与声明序；类路径 provider 按档案序）
#[derive(Debug, Default)]
pub struct Catalog {
    pub by_service: BTreeMap<String, Vec<Provider>>,
}

impl Catalog {
    pub fn get(&self, service: &str) -> &[Provider] {
        self.by_service.get(service).map_or(&[], Vec::as_slice)
    }
}

/// JVM 默认启动的引导层模块集
pub fn boot_layer(observable: &[&ModuleDecl]) -> BTreeSet<String> {
    let by_name: HashMap<&str, &ModuleDecl> = observable.iter().rev().map(|m| (m.name.as_str(), *m)).collect();
    let mut resolved: BTreeSet<String> = BTreeSet::new();
    let mut work: Vec<&str> = observable.iter().filter(|m| m.exports_api && !m.do_not_resolve_by_default).map(|m| m.name.as_str()).collect();
    loop {
        while let Some(n) = work.pop() {
            let Some(m) = by_name.get(n) else { continue };
            if resolved.insert(n.to_string()) {
                work.extend(m.requires.iter().map(String::as_str));
            }
        }
        let uses: BTreeSet<&str> = resolved.iter().filter_map(|n| by_name.get(n.as_str())).flat_map(|m| m.uses.iter().map(String::as_str)).collect();
        work = observable
            .iter()
            .filter(|m| !resolved.contains(&m.name) && m.provides.iter().any(|(s, _)| uses.contains(s.as_str())))
            .map(|m| m.name.as_str())
            .collect();
        if work.is_empty() {
            return resolved;
        }
    }
}

/// 类路径服务配置文件的 provider 类名（UTF-8；`#` 起为注释；去重保序），转内部名
pub fn parse_service_file(bytes: &[u8]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in String::from_utf8_lossy(bytes).lines() {
        let name = line.split('#').next().unwrap_or("").trim();
        if name.is_empty() {
            continue;
        }
        let n = name.replace('.', "/");
        if !out.contains(&n) {
            out.push(n);
        }
    }
    out
}

pub fn catalog(views: &[ArchiveView]) -> Catalog {
    let mods: Vec<&ModuleDecl> = views.iter().filter(|v| v.origin == Origin::Jdk).filter_map(|v| v.module.as_ref()).collect();
    let layer = boot_layer(&mods);
    let mut by_service: BTreeMap<String, Vec<Provider>> = BTreeMap::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for m in &mods {
        if !layer.contains(&m.name) || !seen.insert(m.name.as_str()) {
            continue;
        }
        for (svc, with) in &m.provides {
            let v = by_service.entry(svc.clone()).or_default();
            v.extend(with.iter().map(|c| Provider { module: Some(m.name.clone()), class: c.clone() }));
        }
    }
    for v in views.iter().filter(|v| matches!(v.origin, Origin::User | Origin::Lib)) {
        for (svc, bytes) in &v.services {
            let list = by_service.entry(svc.replace('.', "/")).or_default();
            for c in parse_service_file(bytes) {
                let p = Provider { module: None, class: c };
                if !list.contains(&p) {
                    list.push(p);
                }
            }
        }
    }
    Catalog { by_service }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(name: &str, requires: &[&str], api: bool, uses: &[&str], provides: &[(&str, &str)]) -> ModuleDecl {
        ModuleDecl {
            name: name.into(),
            requires: requires.iter().map(|s| s.to_string()).collect(),
            requires_static: Vec::new(),
            exports_api: api,
            exports: Vec::new(),
            opens: Vec::new(),
            uses: uses.iter().map(|s| s.to_string()).collect(),
            provides: provides.iter().map(|(s, p)| (s.to_string(), vec![p.to_string()])).collect(),
            do_not_resolve_by_default: false,
        }
    }

    /// 根集 + requires 闭包 + uses 绑定；无人 uses 的 provider 模块、无 API 的模块不入层
    #[test]
    fn boot_layer_binds_used_services() {
        let base = m("base", &[], true, &["s/Cs"], &[]);
        let charsets = m("charsets", &["base"], false, &[], &[("s/Cs", "x/Ext")]);
        let dep = m("dep", &[], false, &[], &[]);
        let tool = m("tool", &["dep"], false, &[], &[("s/Unused", "y/P")]);
        let mut inc = m("inc", &[], true, &[], &[]);
        inc.do_not_resolve_by_default = true;
        let helper = m("helper", &["dep"], false, &[], &[("s/Cs", "z/P")]);
        let l = boot_layer(&[&base, &charsets, &dep, &tool, &inc, &helper]);
        let got: Vec<&str> = l.iter().map(String::as_str).collect();
        assert_eq!(got, vec!["base", "charsets", "dep", "helper"]);
    }

    #[test]
    fn service_file_lines() {
        let b = b"# comment\n a.b.C \n\na.b.C\nd.E # tail\n";
        assert_eq!(parse_service_file(b), vec!["a/b/C".to_string(), "d/E".to_string()]);
    }
}
