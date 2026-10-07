//! JCA 提供者序求值（seeds.toml `[jca.order]`，计划 docs/plans/2026-10-06-c1d-jca-provider-order.md）。
//!
//! 装载器调用点（清单 `loader`）先扣住不执行；工作队列在不动点上排空时判定：从扣住的调用点所在方法沿调用方
//! 上溯（内部类内逐层，视图类方法改从生产者上溯），到达的外部入口按调用点实参求出请求（类型 / 算法 / provider
//! 名），每个请求的按序游走深度都小于提供者表首个非内建表项、且提供者表不可改写时继续扣住；任一前提推不出即
//! 永久放行（重跑扣住的调用点）。
//!
//! 顺序无关：扣住期间的分析是「装载器不可达」系统的最小不动点；判定只在全部其他放行都已完成的不动点上做，
//! 是该不动点的函数；放行只一次、只增不减。

use super::*;
use crate::seeds::jca_order::{self as order, OrderCfg, Walk};

#[derive(Default)]
pub(super) struct JcaOrder {
    /// 清单与参考 JDK 求出的静态事实；None = 未配置 / 前提不成立（不扣住）
    facts: Option<Facts>,
    /// 扣住的装载器调用点
    held: BTreeSet<(usize, u32)>,
    /// 放行成因（放行后不再扣住）
    released: Option<String>,
    /// 经非字节码调用点（反射 / 方法句柄 / lambda / 根 / 手写）进入的受监视成员
    rooted: BTreeSet<MemberRef>,
    /// 最近一次判定扣住时各入口请求的最大游走深度（诊断）
    depths: BTreeMap<String, usize>,
}

struct Facts {
    jdk: u32,
    table: Vec<String>,
    /// 首个非内建表项序号
    bound: usize,
    loader: MemberRef,
    interior: Vec<String>,
    /// 视图类 → 生产者
    views: BTreeMap<String, Vec<MemberRef>>,
    entries: Vec<(MemberRef, Walk)>,
    mutators: Vec<(MemberRef, Option<usize>, Vec<String>, Vec<String>)>,
    total_types: Vec<String>,
    override_prop: String,
    /// 内建 provider 必定注册的服务（首次判定时按字节码求出）
    sure: Option<BTreeMap<String, BTreeSet<(String, String)>>>,
}

fn member(s: &str) -> Option<MemberRef> {
    super::seeds::parse_member(s)
}

impl JcaOrder {
    pub(super) fn new(man: &Manifest, cp: &ClassPath) -> Self {
        let facts = man.seeds.jca.order.as_ref().and_then(|c| Facts::build(c, man, cp.release()));
        JcaOrder { facts, ..Default::default() }
    }

    fn interior(&self, owner: &str) -> bool {
        self.facts.as_ref().is_some_and(|f| f.interior.iter().any(|p| owner == p || owner.strip_prefix(p.as_str()).is_some_and(|r| r.starts_with('$'))))
    }

    /// 受监视成员：内部类方法、入口、改写入口
    fn watched(&self, key: &MemberRef) -> bool {
        let Some(f) = &self.facts else { return false };
        self.interior(&key.owner) || f.entries.iter().any(|(e, _)| e == key) || f.mutators.iter().any(|(m, ..)| m == key)
    }
}

impl Facts {
    fn build(c: &OrderCfg, man: &Manifest, jdk: u32) -> Option<Facts> {
        let path = man.runtime_dir.join(c.table.replace("{jdk}", &jdk.to_string()));
        let text = std::fs::read_to_string(path).ok()?;
        // 偏好序在启动表里已设：ProviderList.getService 先走偏好表，序求值不适用
        if c.preferred_key.is_empty() || order::has_key(&text, &c.preferred_key) {
            return None;
        }
        let table = order::parse_table(&text, &c.table_key);
        let builtin = c.builtin.get(&jdk)?;
        Some(Facts {
            jdk,
            bound: order::first_loaded(&table, builtin),
            table,
            loader: member(&c.loader)?,
            interior: c.interior.clone(),
            views: c.views.get(&jdk).into_iter().flatten().map(|v| (v.class.clone(), v.producers.iter().filter_map(|p| member(p)).collect())).collect(),
            entries: c.entries.iter().filter_map(|e| Some((member(&e.member)?, e.walk.clone()))).collect(),
            mutators: c
                .mutators
                .iter()
                .filter_map(|m| {
                    let (pre, eq) = (vec![c.table_key.clone()], vec![c.preferred_key.clone()]);
                    Some((member(&m.member)?, m.key, pre, eq))
                })
                .collect(),
            total_types: c.total_types.iter().map(|t| t.to_uppercase()).collect(),
            override_prop: c.override_prop.clone(),
            sure: None,
        })
    }
}

impl Engine<'_> {
    /// 装载器调用点闸门：扣住返回 true（调用点本轮不处理，放行时重跑）
    pub(super) fn jca_order_hold(&mut self, m: usize, off: u32, mref: &MemberRef) -> bool {
        let j = &mut self.jorder;
        match &j.facts {
            Some(f) if j.released.is_none() && *mref == f.loader => {
                j.held.insert((m, off));
                true
            }
            _ => false,
        }
    }

    /// 方法入口：受监视成员经非字节码调用点进入即记为有根（其调用方不可见）
    pub(super) fn jca_order_entry(&mut self, key: &MemberRef, via: &Via) {
        if self.jorder.facts.is_none() || self.jorder.released.is_some() {
            return;
        }
        let tracked = matches!(via.kind, "invoke" | "dispatch" | "concrete") && matches!(via.from, From::Method(c) if self.methods[c].kind == Kind::Bytecode);
        if !tracked && self.jorder.watched(key) && !self.jorder.rooted.contains(key) {
            self.jorder.rooted.insert(key.clone());
        }
    }

    /// 诊断：summary.jca_order
    pub fn jca_order_report(&self) -> serde_json::Value {
        let j = &self.jorder;
        let Some(f) = &j.facts else { return serde_json::Value::Null };
        let held: Vec<String> = j.held.iter().map(|(m, off)| format!("{}@{off}", self.methods[*m].key)).collect();
        serde_json::json!({
            "jdk": f.jdk, "bound": f.bound, "table": f.table,
            "held": if j.released.is_some() { vec![] } else { held },
            "released": j.released, "depths": j.depths,
        })
    }

    /// 不动点上的判定：前提不成立即放行扣住的调用点（返回 true = 有新工作）
    pub(super) fn jca_order_release(&mut self) -> bool {
        if self.jorder.released.is_some() || self.jorder.held.is_empty() {
            return false;
        }
        match self.jca_order_check() {
            Ok(depths) => {
                self.jorder.depths = depths;
                false
            }
            Err(why) => {
                self.jorder.released = Some(why);
                self.jorder.depths.clear();
                for w in std::mem::take(&mut self.jorder.held) {
                    self.push_site(w, site_prof::TRIG_RELEASE, None);
                }
                true
            }
        }
    }

    fn jca_order_check(&mut self) -> Result<BTreeMap<String, usize>, String> {
        let f = self.jorder.facts.as_ref().ok_or("未配置")?;
        let (override_prop, mutators) = (f.override_prop.clone(), f.mutators.clone());
        {
            let u = self.ctx.punstable.borrow();
            if u.all || u.keys.contains(&override_prop) {
                return Err(format!("系统属性 {override_prop} 可被改写（{}）", u.cause.clone().unwrap_or_default()));
            }
        }
        for (mm, key, pre, eq) in &mutators {
            let nodes = self.jca_nodes_of(mm);
            if nodes.is_empty() {
                continue;
            }
            let Some(ki) = key else { return Err(format!("改写入口 {mm} 可达")) };
            for n in nodes {
                for (c, off, args) in self.jca_sites(n)? {
                    let prev = self.cur_site.replace((c, off));
                    let keys = args.get(*ki).map_or(keyed::Keys::Any, |v| self.names_of(c, v));
                    self.cur_site = prev;
                    let hit = match &keys {
                        keyed::Keys::Any => true,
                        keyed::Keys::Set(s) => s.iter().any(|k| pre.iter().any(|p| k.starts_with(p.as_str())) || eq.iter().any(|q| q.as_str() == &**k)),
                    };
                    if hit {
                        return Err(format!("改写入口 {mm} 的键可能命中提供者表（{}@{off}）", self.methods[c].key));
                    }
                }
            }
        }
        let entries = self.jca_order_walk()?;
        self.jca_order_sure();
        let mut depths: BTreeMap<String, usize> = BTreeMap::new();
        for (ei, n) in entries {
            let (e, walk) = self.jorder.facts.as_ref().map(|f| f.entries[ei].clone()).ok_or("未配置")?;
            let d = match &walk {
                Walk::Literal(name) => self.jca_depth_name(name).ok_or_else(|| format!("{e}：provider {name} 的序号不小于首个非内建表项"))?,
                Walk::Name(i) => {
                    let mut d = 0;
                    for (c, off, args) in self.jca_sites(n)? {
                        for name in self.jca_names(c, off, args.get(*i), &e)? {
                            d = d.max(self.jca_depth_name(&name).ok_or_else(|| format!("{e}：provider {name} 的序号不小于首个非内建表项"))?);
                        }
                    }
                    d
                }
                Walk::First { ty, algo, failover } => {
                    let mut d = 0;
                    for (c, off, args) in self.jca_sites(n)? {
                        let tys = self.jca_names(c, off, args.get(*ty), &e)?;
                        let algos = self.jca_names(c, off, args.get(*algo), &e)?;
                        let f = self.jorder.facts.as_ref().ok_or("未配置")?;
                        for t in &tys {
                            if *failover && !f.total_types.contains(&t.to_uppercase()) {
                                return Err(format!("{e}：服务类型 {t} 的首个服务可能构造失败（失败转移走全表）"));
                            }
                            for a in &algos {
                                let sure = f.sure.as_ref().ok_or("未配置")?;
                                let x = order::first_depth(&f.table, f.bound, sure, t, a)
                                    .ok_or_else(|| format!("{e}：{t}.{a} 不能由首个非内建表项之前的 provider 必定提供（{}@{off}）", self.methods[c].key))?;
                                d = d.max(x);
                            }
                        }
                    }
                    d
                }
            };
            let k = e.to_string();
            let cur = depths.get(&k).copied().unwrap_or(0);
            depths.insert(k, cur.max(d));
        }
        Ok(depths)
    }

    /// 从扣住的调用点沿调用方上溯：返回到达的 (入口序号, 入口方法节点)；越出内部类即 Err
    fn jca_order_walk(&self) -> Result<BTreeSet<(usize, usize)>, String> {
        let j = &self.jorder;
        let f = j.facts.as_ref().ok_or("未配置")?;
        let mut seen: BTreeSet<usize> = BTreeSet::new();
        let mut work: VecDeque<usize> = j.held.iter().map(|(m, _)| *m).collect();
        let mut entries = BTreeSet::new();
        while let Some(n) = work.pop_front() {
            if !seen.insert(n) {
                continue;
            }
            let key = &self.methods[n].key;
            if let Some(ei) = f.entries.iter().position(|(e, _)| e == key) {
                entries.insert((ei, n));
                continue;
            }
            if let Some(ps) = f.views.get(&key.owner) {
                for p in ps {
                    work.extend(self.jca_nodes_of(p));
                }
                continue;
            }
            if !j.interior(&key.owner) {
                return Err(format!("游走经非内部方法 {key} 到达"));
            }
            if j.rooted.contains(key) || self.methods[n].kind != Kind::Bytecode {
                return Err(format!("内部方法 {key} 有非字节码入口"));
            }
            let cs = self.callers.get(&n).filter(|s| !s.is_empty()).ok_or_else(|| format!("内部方法 {key} 无可见调用方"))?;
            work.extend(cs.iter().copied());
        }
        Ok(entries)
    }

    /// 成员的全部方法节点（各克隆上下文）
    fn jca_nodes_of(&self, key: &MemberRef) -> Vec<usize> {
        if !self.mbase.contains_key(key) {
            return vec![];
        }
        self.methods.iter().enumerate().filter(|(_, (k, _))| k.0 == *key).map(|(i, _)| i).collect()
    }

    /// 方法节点 n 的全部字节码调用点与实参（含接收者）；有非字节码入口 / 调用方不可见即 Err
    fn jca_sites(&self, n: usize) -> Result<Vec<(usize, u32, Vec<V>)>, String> {
        let key = &self.methods[n].key;
        if self.jorder.rooted.contains(key) {
            return Err(format!("{key} 有非字节码入口"));
        }
        let mut out = vec![];
        for &c in self.callers.get(&n).into_iter().flatten() {
            if self.methods[c].kind != Kind::Bytecode {
                return Err(format!("{key} 的调用方 {} 非字节码", self.methods[c].key));
            }
            let Some(a) = self.methods[c].analysis.clone() else { continue };
            if a.conservative {
                return Err(format!("{key} 的调用方 {} 按保守分析", self.methods[c].key));
            }
            for (&(_, off), ts) in self.dispatch.range((c, 0)..=(c, u32::MAX)) {
                if !ts.contains(&n) {
                    continue;
                }
                if let Some(Event::Invoke { args, .. }) = class_lookup::event_at(&a, off, class_lookup::is_invoke) {
                    out.push((c, off, args.clone()));
                }
            }
        }
        Ok(out)
    }

    fn jca_names(&mut self, c: usize, off: u32, v: Option<&V>, e: &MemberRef) -> Result<BTreeSet<String>, String> {
        let prev = self.cur_site.replace((c, off));
        let keys = v.map_or(keyed::Keys::Any, |v| self.names_of(c, v));
        self.cur_site = prev;
        match keys {
            keyed::Keys::Any => Err(format!("{e} 的实参名字推不出（{}@{off}）", self.methods[c].key)),
            keyed::Keys::Set(s) => Ok(s.iter().map(|x| x.to_string()).collect()),
        }
    }

    fn jca_depth_name(&self, name: &str) -> Option<usize> {
        let f = self.jorder.facts.as_ref()?;
        order::index_of(&f.table, name).filter(|&i| i < f.bound)
    }

    fn jca_order_sure(&mut self) {
        let Some(f) = self.jorder.facts.as_mut() else { return };
        if f.sure.is_none() {
            let c = self.man.seeds.jca.order.as_ref().cloned().unwrap_or_default();
            f.sure = Some(order::sure_services(&c, &self.man.seeds.jca.alias_sources, self.cp));
        }
    }
}
