//! 引擎：派发枢纽的建立分布剖析（随 `--lambda-prof` 周期打印，只读诊断，不影响分析结果）。
//!
//! 打印三类数据（stderr，`[lambda-prof …]` 前缀）：
//! - `hubkind`：枢纽按种类（open / 精确集合 / 增长 / VM）的个数、接入点数、展开接收者数、精确集合元素总数、lambda 表长总数；
//! - `hubm`：按调用成员归并的枢纽数前列（附各种类个数、不同档位上下文数、接入点 / 接收者总数）；
//! - `hubsite`：按建立枢纽的调用点（不分克隆上下文）归并的枢纽数前列，及精确集合枢纽父链深度分布。

use super::*;

const TOP: usize = 20;

#[derive(Default)]
struct Agg {
    n: u64,
    open: u64,
    exact: u64,
    grow: u64,
    vm: u64,
    links: u64,
    recvs: u64,
    set: u64,
    lambdas: u64,
    special: u64,
}

impl Agg {
    fn add(&mut self, kind: usize, h: &Hub) {
        self.n += 1;
        match kind {
            0 => self.open += 1,
            1 => self.exact += 1,
            2 => self.grow += 1,
            _ => self.vm += 1,
        }
        self.links += h.links.len() as u64;
        self.recvs += h.recvs.len() as u64;
        self.set += h.set.as_ref().map_or(0, |s| s.len() as u64);
        self.lambdas += h.lambdas.len() as u64;
        self.special += h.special.values().map(|v| v.len() as u64).sum::<u64>();
    }

    fn line(&self) -> String {
        format!(
            "n={} open={} exact={} grow={} vm={} links={} recvs={} set={} lambdas={} special={}",
            self.n, self.open, self.exact, self.grow, self.vm, self.links, self.recvs, self.set, self.lambdas, self.special
        )
    }
}

impl Engine<'_> {
    fn hub_kind(&self, h: u32) -> usize {
        let hub = &self.hubs[h as usize];
        if self.vm_hubs.contains(&h) {
            3
        } else if hub.open.is_some() {
            0
        } else if hub.set.is_some() {
            1
        } else {
            2
        }
    }

    pub(super) fn hub_prof_dump(&self, tag: &str) {
        let mut all = Agg::default();
        let mut by_m: HashMap<String, (Agg, HashSet<u32>)> = HashMap::default();
        let mut by_site: HashMap<String, Agg> = HashMap::default();
        let mut depth: BTreeMap<u32, u64> = BTreeMap::new();
        for h in 0..self.hubs.len() as u32 {
            let hub = &self.hubs[h as usize];
            let k = self.hub_kind(h);
            all.add(k, hub);
            let (o, nm, d) = hub.site.key();
            let e = by_m.entry(format!("{o}.{nm}{d}")).or_default();
            e.0.add(k, hub);
            e.1.insert(self.via_level(&hub.via));
            let site = match &hub.via.from {
                From::Method(m) => format!("{} {}@{}", hub.via.kind, self.method_label(*m), hub.via.off.unwrap_or(0)),
                other => format!("{} {other:?}", hub.via.kind),
            };
            by_site.entry(site).or_default().add(k, hub);
            if k == 1 {
                let mut n = 0;
                let mut p = hub.parent;
                while let Some(x) = p {
                    n += 1;
                    p = self.hubs[x as usize].parent;
                }
                *depth.entry(n.min(64)).or_default() += 1;
            }
        }
        eprintln!("{tag} hubkind {}", all.line());
        let mut ms: Vec<(String, (Agg, HashSet<u32>))> = by_m.into_iter().collect();
        ms.sort_by(|a, b| b.1 .0.n.cmp(&a.1 .0.n).then(a.0.cmp(&b.0)));
        for (l, (a, lcs)) in ms.into_iter().take(TOP) {
            eprintln!("{tag} hubm lc={} {} {l}", lcs.len(), a.line());
        }
        let mut ss: Vec<(String, Agg)> = by_site.into_iter().collect();
        ss.sort_by(|a, b| b.1.n.cmp(&a.1.n).then(a.0.cmp(&b.0)));
        for (l, a) in ss.into_iter().take(TOP) {
            eprintln!("{tag} hubsite {} {l}", a.line());
        }
        eprintln!("{tag} hubdepth {depth:?}");
    }
}
