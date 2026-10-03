//! 引擎：服务目录事实（seeds.toml `[services]`，目录构造见 `seeds/services.rs`）。
//!
//! `lookups` 成员的调用点上，服务 Class 实参值集里的类镜像即被查找的服务（值集增长时站点重跑）；
//! 值集含所指未知的 Class（open / 非镜像值）时，按闭包内的服务处理（计入 `services_unknown`）：原生程序里
//! 只有闭包内的类有类镜像，所指未知的 Class 只能是其中之一；此后有目录服务类入闭包，站点重跑补选。
//! 入选服务的 provider 按 JVM `ServiceLoader.loadProvider` 的构造途径入链：命名模块里声明了
//! `public static provider()` 的取该方法，否则取公开无参构造器（实例化 + 类初始化）。
//! 有模块 provider 入选时，清单 `population`（引导期装填模块服务目录的 JDK 方法）作根。

use super::*;
use crate::seeds::services::{self, Catalog, Provider};

#[derive(Default)]
pub struct ServiceState {
    population_done: bool,
    /// 输出：被查找的服务 → 入选 provider（无 provider 的服务同样记录）
    pub selected: BTreeMap<String, Vec<Provider>>,
    /// 输出：出现过所指未知的服务 Class 实参（按闭包内的服务处理）
    pub unknown: bool,
    /// 服务 Class 实参所指未知的查找站点：目录服务类入闭包时重跑
    unknown_sites: BTreeSet<(usize, u32)>,
}

impl Ctx<'_> {
    /// 服务目录（引导层模块 provides + 类路径 META-INF/services），首次使用时构造
    pub(super) fn service_catalog(&self) -> Rc<Catalog> {
        self.catalog.get_or_init(|| Rc::new(services::catalog(&self.cp.module_views()))).clone()
    }
}

impl<'a> Engine<'a> {
    fn service_catalog(&mut self) -> Rc<Catalog> {
        self.ctx.service_catalog()
    }

    /// 调用点（方法 m、偏移 off）若是服务查找入口，按服务 Class 实参补种 provider
    pub(super) fn service_lookup(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) {
        let k = self.mref_key(mref);
        let Some(&j) = self.man.seeds.services.lookups.get(&*k) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let Some(a) = args.get(skip + j) else { return };
        let (known, unknown) = match a {
            V::Class(c, _) => (vec![c.to_string()], false),
            V::Null => (vec![], false),
            V::Ref { .. } => {
                let class = self.id(CLASS);
                let fs = self.feeds(m, a, class);
                let s = self.value_set(&fs);
                let mut known = vec![];
                let mut unknown = !s.open.is_empty();
                for x in s.classes.iter() {
                    match self.mirrors.get(&x) {
                        Some(&c) => known.push(self.names[c as usize].to_string()),
                        None => unknown = true,
                    }
                }
                (known, unknown)
            }
            _ => (vec![], true),
        };
        let catalog = self.service_catalog();
        let mut picked: Vec<String> = known;
        if unknown {
            self.seeds.services.unknown = true;
            self.seeds.services.unknown_sites.insert((m, off));
            picked.extend(catalog.by_service.keys().filter(|s| self.classes.contains_key(s.as_str())).cloned());
        }
        for svc in picked {
            if self.seeds.services.selected.contains_key(&svc) {
                continue;
            }
            let providers = catalog.get(&svc).to_vec();
            for p in &providers {
                self.seed_provider(p, Via::method("service-provider", m, Some(off)));
            }
            if providers.iter().any(|p| p.module.is_some()) && !self.seeds.services.population_done {
                self.seeds.services.population_done = true;
                for k in self.man.seeds.services.population.clone() {
                    let Some(key) = super::seeds::parse_member(&k) else { continue };
                    let via = Via::method("service-catalog", m, Some(off));
                    self.init(&key.owner.clone(), via.clone());
                    let t = self.method(key, via);
                    self.open_params(t);
                    self.returns_to_vm(t);
                }
            }
            self.seeds.services.selected.insert(svc, providers);
        }
    }

    /// 类入闭包：它是目录里的服务且有所指未知的查找站点时，站点重跑（补选该服务）
    pub(super) fn service_class_entered(&mut self, cls: &str) {
        if self.seeds.services.unknown_sites.is_empty() || self.seeds.services.selected.contains_key(cls) {
            return;
        }
        if !self.service_catalog().by_service.contains_key(cls) {
            return;
        }
        for w in self.seeds.services.unknown_sites.clone() {
            if self.in_swork.insert(w) {
                self.swork.push_back(w);
            }
        }
    }

    fn seed_provider(&mut self, p: &Provider, via: Via) {
        let Some(cf) = self.cp.get(&p.class) else {
            self.missing.entry(p.class.clone()).or_insert(via);
            return;
        };
        let factory = p.module.as_ref().and(cf.methods.iter().find(|x| {
            x.name == "provider" && x.desc.starts_with("()") && x.is_static() && x.access & classfile::acc::PUBLIC != 0
        }));
        let (name, desc) = match factory {
            Some(f) => (f.name.clone(), f.desc.clone()),
            None => {
                if cf.method("<init>", "()V").is_none() {
                    return;
                }
                self.instantiate(&p.class, via.clone());
                ("<init>".to_string(), "()V".to_string())
            }
        };
        self.init(&p.class, via.clone());
        self.seeds.reflect_names.entry(p.class.clone()).or_default().insert(name.clone());
        let t = self.method(MemberRef { owner: p.class.clone(), name, desc }, via);
        self.open_params(t);
        self.returns_to_vm(t);
    }
}
