//! 引擎：清单种子（seeds.toml 注解 / locale / JCA，与镜像独有 / VM 支持类）。
//!
//! 这些类 / 成员经类名反射或 VM 装载进入运行期，没有静态调用边：按「触发成员可达 + 结构证据」
//! 在工作队列排空时补种，补入的新工作继续传播，直到不再有新种子（外层不动点）。

use std::collections::{HashMap as StdMap, HashSet as StdSet};

use super::*;
use crate::seeds::jca::{self, Service};
use crate::seeds::{annotation, locale};

#[derive(Default)]
pub struct SeedState {
    /// `--locale` 显式给出的标签
    pub locales: Vec<String>,
    anno_done: bool,
    locale_bases: BTreeSet<String>,
    jca_services: Option<BTreeSet<Service>>,
    jca_algos: StdSet<String>,
    jca_aliases: StdMap<String, StdSet<String>>,
    jca_scanned: HashSet<usize>,
    image_done: BTreeSet<String>,
    family_done: BTreeSet<String>,

    /// 输出：纯数据资源束（发射层 register_data_bundles）
    pub data_bundles: BTreeSet<String>,
    /// 输出：注解枚举元素类型（类初始化钩子）
    pub annotation_enums: BTreeSet<String>,
    /// 输出：入选的 JCA 服务
    pub jca: BTreeSet<Service>,
    /// 输出：按名登记的反射分派面（类 → 成员名）
    pub reflect_names: BTreeMap<String, BTreeSet<String>>,
    /// 输出：全量反射面（字段 + 方法）的类
    pub reflect_all: BTreeSet<String>,
}

impl<'a> Engine<'a> {
    /// 可达成员（`类.成员`，不分描述符）
    fn reached_members(&self) -> HashSet<String> {
        self.methods.values().map(|m| format!("{}.{}", m.key.owner, m.key.name)).collect()
    }

    fn user_classes(&self) -> Vec<Rc<ClassFile>> {
        self.cp.names_of(Origin::User).iter().filter_map(|n| self.cp.get(n)).collect()
    }

    /// 种子方法：形参来自非建模代码（反射 / VM），返回值交回非建模代码
    fn seed_method(&mut self, key: MemberRef, kind: &'static str) {
        let via = Via::root(kind, &key.to_string());
        let t = self.method(key, via);
        self.open_params(t);
        self.returns_to_vm(t);
    }

    /// 一轮补种；有新增返回 true
    pub(super) fn seed_round(&mut self) -> bool {
        let before = self.methods.len() + self.g.len() + self.inited.len();
        let reached = self.reached_members();
        self.seed_annotations(&reached);
        self.seed_locale(&reached);
        self.seed_jca(&reached);
        self.seed_image();
        self.methods.len() + self.g.len() + self.inited.len() != before
    }

    fn seed_annotations(&mut self, reached: &HashSet<String>) {
        let cfg = &self.man.seeds.annotation;
        if self.seeds.anno_done || !cfg.triggers.iter().any(|t| reached.contains(t)) {
            return;
        }
        self.seeds.anno_done = true;
        for s in cfg.seeds.clone() {
            if let Some(k) = parse_member(&s) {
                self.seed_method(k, "annotation");
            }
        }
        let found = annotation::collect(self.cp, &self.user_classes());
        for a in &found.annos {
            let Some(cf) = self.touch(a, Level::Type, Via::root("annotation", a)) else { continue };
            for m in cf.methods.iter().filter(|m| !m.name.starts_with('<')) {
                self.seed_method(MemberRef { owner: a.clone(), name: m.name.clone(), desc: m.desc.clone() }, "annotation");
            }
        }
        for e in &found.enums {
            self.init(e, Via::root("annotation-enum", e));
            self.seeds.annotation_enums.insert(e.clone());
        }
        for t in &found.types {
            self.touch(t, Level::Type, Via::root("annotation-class", t));
        }
    }

    fn seed_locale(&mut self, reached: &HashSet<String>) {
        let cfg = &self.man.seeds.locale;
        let bases: Vec<String> = cfg.triggered_bases(|m| reached.contains(m)).into_iter().filter(|b| !self.seeds.locale_bases.contains(b)).collect();
        if bases.is_empty() {
            return;
        }
        self.seeds.locale_bases.extend(bases.iter().cloned());
        let locs = locale::collect(cfg, self.cp, &self.user_classes(), &self.seeds.locales);
        let names = locale::bundle_classes(&locs, &bases, self.cp, &self.man.seeds.carriers);
        eprintln!("[closure] locale 种子：{} 个 locale → {} 个资源束（{}）", locs.len(), names.len(), bases.join(", "));
        for b in names {
            let Some(cf) = self.cp.get(&b) else { continue };
            self.instantiate(&b, Via::root("locale", &b));
            self.init(&b, Via::root("locale", &b));
            self.seed_method(MemberRef { owner: b.clone(), name: "<init>".into(), desc: "()V".into() }, "locale");
            if let Some((n, d)) = self.man.seeds.carriers.carrier_of(self.cp, &cf) {
                self.seed_method(MemberRef { owner: b.clone(), name: n, desc: d }, "locale");
            }
            self.seeds.data_bundles.insert(b);
        }
    }

    fn seed_jca(&mut self, reached: &HashSet<String>) {
        let cfg = &self.man.seeds.jca;
        if !cfg.triggers.iter().any(|t| reached.contains(t)) {
            return;
        }
        if self.seeds.jca_services.is_none() {
            self.seeds.jca_services = Some(jca::extract_services(cfg, self.cp));
            self.seeds.jca_algos = jca::user_algorithms(&self.user_classes());
            self.seeds.jca_aliases = jca::alias_groups(cfg, self.cp);
        }
        let services = self.seeds.jca_services.take().unwrap_or_default();
        let types: StdSet<&str> = services.iter().map(|s| s.ty.as_str()).collect();
        // 可达 JDK 方法里 engine getInstance 调用的字符串实参（JDK 自身按名取服务）
        for i in 0..self.methods.len() {
            if !self.seeds.jca_scanned.insert(i) {
                continue;
            }
            let key = &self.methods[i].key;
            if self.cp.origin(&key.owner) == Some(Origin::User) {
                continue;
            }
            let Some(cf) = self.cp.get(&key.owner) else { continue };
            if let Some(code) = cf.method(&key.name, &key.desc).and_then(|m| m.code.as_ref()) {
                jca::engine_call_strings(&code.insns, &types, &mut self.seeds.jca_algos);
            }
        }
        let forced: StdSet<(String, String)> = cfg.defaults.iter().filter(|(t, _, _)| reached.contains(t)).map(|(_, t, a)| (t.clone(), a.clone())).collect();
        let live: StdSet<String> = self.methods.values().map(|m| m.key.owner.rsplit('/').next().unwrap_or(&m.key.owner).to_string()).collect();
        let picked: Vec<Service> = jca::select(&services, &self.seeds.jca_algos, &live, &self.seeds.jca_aliases, &forced)
            .into_iter()
            .filter(|s| !self.seeds.jca.contains(*s))
            .cloned()
            .collect();
        self.seeds.jca_services = Some(services);
        for s in picked {
            // 实现类由 Provider$Service.newInstance 反射构造：构造器形参由服务类型决定，全部构造器入链
            if let Some(cf) = self.cp.get(&s.imp) {
                self.instantiate(&s.imp, Via::root("jca", &s.imp));
                self.init(&s.imp, Via::root("jca", &s.imp));
                for m in cf.methods.iter().filter(|m| m.is_init()) {
                    self.seed_method(MemberRef { owner: s.imp.clone(), name: m.name.clone(), desc: m.desc.clone() }, "jca");
                }
                self.seeds.reflect_names.entry(s.imp.clone()).or_default().insert("<init>".into());
            }
            // provider 对象由手写边界按需构造（ProviderConfig 对内建 provider 直接 new）
            if let Some(p) = self.man.seeds.jca.provider_class(&s.provider).map(String::from) {
                if self.cp.contains(&p) {
                    self.instantiate(&p, Via::root("jca-provider", &p));
                    self.init(&p, Via::root("jca-provider", &p));
                    self.seed_method(MemberRef { owner: p, name: "<init>".into(), desc: "()V".into() }, "jca-provider");
                }
            }
            self.seeds.jca.insert(s);
        }
    }

    /// 镜像独有类（jlink 预生成）/ VM 支持类：父类在闭包内的非接口类整体入闭包（实例化 + 全部方法 +
    /// 全量反射面）；与之同直接父类的闭包类（物种族）同取全量反射面与全部方法
    fn seed_image(&mut self) {
        let obj = OBJECT;
        let mut supers: BTreeSet<String> = BTreeSet::new();
        for x in self.cp.names_of(Origin::Image) {
            let Some(cf) = self.cp.get(&x) else { continue };
            let Some(sup) = cf.super_name.clone() else { continue };
            if self.seeds.image_done.contains(&x) {
                supers.insert(sup);
                continue;
            }
            if cf.is_interface() || sup == obj || !self.classes.contains_key(&sup) {
                continue;
            }
            self.seeds.image_done.insert(x.clone());
            supers.insert(sup);
            self.instantiate(&x, Via::root("image", &x));
            self.init(&x, Via::root("image", &x));
            self.seed_all_members(&x, &cf, "image");
        }
        let family: Vec<String> = self
            .classes
            .keys()
            .filter(|c| !self.seeds.image_done.contains(*c) && !self.seeds.family_done.contains(*c))
            .filter(|c| self.cp.get(c).is_some_and(|cf| !cf.is_interface() && cf.super_name.as_ref().is_some_and(|s| supers.contains(s))))
            .cloned()
            .collect();
        for c in family {
            self.seeds.family_done.insert(c.clone());
            if let Some(cf) = self.cp.get(&c) {
                self.seed_all_members(&c, &cf, "species-family");
            }
        }
    }

    fn seed_all_members(&mut self, cls: &str, cf: &ClassFile, kind: &'static str) {
        self.seeds.reflect_all.insert(cls.to_string());
        for m in &cf.methods {
            self.seeds.reflect_names.entry(cls.to_string()).or_default().insert(m.name.clone());
            self.seed_method(MemberRef { owner: cls.to_string(), name: m.name.clone(), desc: m.desc.clone() }, kind);
        }
    }
}

/// `类.方法:描述符` → MemberRef
pub(super) fn parse_member(s: &str) -> Option<MemberRef> {
    let (head, desc) = s.split_once(':')?;
    let (owner, name) = head.rsplit_once('.')?;
    Some(MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() })
}
