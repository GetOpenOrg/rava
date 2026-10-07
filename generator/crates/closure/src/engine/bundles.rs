//! 引擎：按名装载的资源束（seeds.toml `[bundles]`，结构判定见 `seeds/bundles.rs`）。
//!
//! 新可达方法扫描一次：登记 `lookups` 成员的调用点，收集束形 ldc 字面量。每个补种轮在调用点所在方法的
//! 当前分析里重求基名实参（与 JCA 请求点同一套键值求值）。名字推不出（如基名经 lambda 捕获实参传入）时，
//! 回退到调用链上的束形字面量：原生程序里能装载的束，名字只能来自闭包内的代码；回退名只在类路径上真有
//! 对应的束类 / 属性文件时入选。入选基名按 locale 父链展开：束类按 locale 束的方式补种（反射构造），
//! 属性文件进 `named_resources`（生成器嵌入模块资源表，运行时由模块资源查询读入）。集合只增不减（外层不动点）。

use super::*;
use crate::seeds::{bundles, locale};
use classfile::{Const, Operand};

#[derive(Default)]
pub struct BundleState {
    scanned: HashSet<usize>,
    /// 按名装载调用点：(方法, 偏移, 基名形参序号)
    sites: Vec<(usize, u32, usize)>,
    /// 基名推不出的调用点
    unknown: BTreeSet<(usize, u32)>,
    /// 调用链上的束形字面量 → 首见方法
    literals: BTreeMap<String, usize>,
    /// locale 父链后缀（首次需要时求）
    suffixes: Option<Vec<String>>,
    /// 已处理的基名
    bases: BTreeSet<String>,
    /// 输出：入选的束类
    pub classes: BTreeSet<String>,
    /// 入选的属性文件束数（资源路径并入 `named_resources`）
    props: usize,
}

impl<'a> Engine<'a> {
    /// 一轮资源束补种
    pub(super) fn seed_bundles(&mut self) {
        if self.man.seeds.bundles.lookups.is_empty() && self.man.seeds.resource_lookups.lookups.is_empty() {
            return;
        }
        self.named_scan();
        if self.seeds.bundles.sites.is_empty() {
            return;
        }
        // 基名 → 引出它的调用点（Via）
        let mut want: BTreeMap<String, (usize, Option<u32>)> = BTreeMap::new();
        for (m, off, j) in self.seeds.bundles.sites.clone() {
            if self.seeds.bundles.unknown.contains(&(m, off)) {
                continue;
            }
            let Some(a) = self.methods[m].analysis.clone() else { continue };
            let Some(Event::Invoke { opcode, args, .. }) = class_lookup::event_at(&a, off, class_lookup::is_invoke) else { continue };
            let skip = usize::from(*opcode != classfile::op::INVOKESTATIC);
            let Some(v) = args.get(skip + j).cloned() else { continue };
            let prev = self.cur_site.replace((m, off));
            // 槽值推不全（如未定字段 / 未知调用的结果）时 names_of 只给出已知部分：已知名字照收，
            // 站点同时记为推不出，由字面量兜底覆盖其余基名
            let saved = (std::mem::take(&mut self.lookup_partial), std::mem::take(&mut self.lookup_incomplete));
            let keys = self.names_of(m, &v);
            let partial = std::mem::replace(&mut self.lookup_partial, saved.0);
            let incomplete = std::mem::replace(&mut self.lookup_incomplete, saved.1);
            self.cur_site = prev;
            match keys {
                keyed::Keys::Any => {
                    self.seeds.bundles.unknown.insert((m, off));
                }
                keyed::Keys::Set(names) => {
                    if partial || incomplete {
                        self.seeds.bundles.unknown.insert((m, off));
                    }
                    for n in names {
                        want.entry(n.to_string()).or_insert((m, Some(off)));
                    }
                }
            }
        }
        if !self.seeds.bundles.unknown.is_empty() {
            for (s, &m) in &self.seeds.bundles.literals {
                want.entry(s.clone()).or_insert((m, None));
            }
        }
        want.retain(|b, _| !self.seeds.bundles.bases.contains(b));
        if want.is_empty() {
            return;
        }
        let suffixes = self.bundle_suffixes();
        let root = self.man.seeds.bundles.root.clone();
        let (c0, r0) = (self.seeds.bundles.classes.len(), self.seeds.bundles.props);
        let named: Vec<String> = want.iter().filter(|(_, (_, off))| off.is_some()).map(|(b, _)| b.clone()).collect();
        let fallback = want.len() - named.len();
        for (base, (m, off)) in want {
            self.seeds.bundles.bases.insert(base.clone());
            let (classes, props) = bundles::candidates(&base, &suffixes);
            for c in classes {
                if self.seeds.bundles.classes.contains(&c) || !self.cp.contains(&c) || !self.h.is_subtype(&c, &root) {
                    continue;
                }
                let Some(cf) = self.cp.get(&c) else { continue };
                if cf.is_abstract() || cf.method("<init>", "()V").is_none() {
                    continue;
                }
                // 束类由 ResourceBundle 按类名反射构造（Class.forName + newInstance）：
                // 无参构造器入链并登记反射分派面；内容方法经虚分派随实例化可达
                let via = Via::method("bundle", m, off);
                self.instantiate(&c, via.clone());
                self.init(&c, via);
                self.seed_method(MemberRef { owner: c.clone(), name: "<init>".into(), desc: "()V".into() }, "bundle");
                self.seeds.reflect_names.entry(c.clone()).or_default().insert("<init>".into());
                self.seeds.bundles.classes.insert(c);
            }
            for p in props {
                if !self.seeds.named_resources.contains(&p) && self.cp.resource(&p).is_some() {
                    self.seeds.named_resources.insert(p);
                    self.seeds.bundles.props += 1;
                }
            }
        }
        let b = &self.seeds.bundles;
        eprintln!(
            "[closure] 资源束：{} 个调用点（{} 个基名推不出）；基名 {:?} + 字面量回退 {} 个 → 束类 +{}、属性文件 +{}（累计 {} / {}）",
            b.sites.len(),
            b.unknown.len(),
            named,
            fallback,
            b.classes.len() - c0,
            b.props - r0,
            b.classes.len(),
            b.props
        );
    }

    /// 新可达方法：登记按名装载调用点、束形字面量与按名读取的资源调用点（`res_lookups.rs`，按声明处的成员键）
    fn named_scan(&mut self) {
        for i in 0..self.methods.len() {
            if !self.seeds.bundles.scanned.insert(i) {
                continue;
            }
            let key = &self.methods[i].key;
            let Some(cf) = self.cp.get(&key.owner) else { continue };
            let Some(code) = cf.method(&key.name, &key.desc).and_then(|m| m.code.as_ref()) else { continue };
            for x in &code.insns {
                match &x.operand {
                    Operand::Method(r, iface) => {
                        let k = format!("{}.{}:{}", r.owner, r.name, r.desc);
                        if let Some(&j) = self.man.seeds.bundles.lookups.get(&k) {
                            self.seeds.bundles.sites.push((i, x.offset, j));
                        }
                        if self.man.seeds.resource_lookups.names.contains(r.name.as_str()) {
                            if let Some(site) = self.h.resolve_method(&r.owner, &r.name, &r.desc, *iface) {
                                let (o, n, d) = site.key();
                                if let Some(&l) = self.man.seeds.resource_lookups.lookups.get(&format!("{o}.{n}:{d}")) {
                                    self.seeds.res_lookups.sites.push((i, x.offset, l));
                                }
                            }
                        }
                    }
                    Operand::Ldc(Const::String(s)) if bundles::bundle_shaped(s) => {
                        self.seeds.bundles.literals.entry(s.clone()).or_insert(i);
                    }
                    _ => {}
                }
            }
        }
    }

    /// 入选 locale 的父链后缀（与 locale 种子同一口径：ROOT + en + 用户代码可见 locale + `--locale`）
    fn bundle_suffixes(&mut self) -> Vec<String> {
        if let Some(s) = &self.seeds.bundles.suffixes {
            return s.clone();
        }
        let locs = locale::collect(&self.man.seeds.locale, self.cp, &self.user_classes(), &self.seeds.locales);
        let mut out: Vec<String> = Vec::new();
        for l in &locs {
            for s in locale::parent_chain(l) {
                if !out.contains(&s) {
                    out.push(s);
                }
            }
        }
        self.seeds.bundles.suffixes = Some(out.clone());
        out
    }
}
