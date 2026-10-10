//! 引擎：方法上下文克隆的分布剖析（随 `--lambda-prof` 周期打印，只读诊断，不影响分析结果）。
//!
//! 打印四类数据（stderr，`[lambda-prof …]` 前缀）：
//! - `ctxkind`：方法节点按上下文种类（本体 / 抽象对象 / 调用点 / 形参常量 / 档位）的计数，及各种类的不同上下文数；
//! - `ctxhist`：每成员克隆数的分布（≤ 8 / 64 / 256 / 1024 / 更多的成员数与节点数）；
//! - `ctxmerge`：克隆最多的成员，其上下文按「分配点」「类」归并后剩几个（归并预算的效果上界）；
//! - `objsite` / `objcls`：抽象对象按分配点 / 类的变体数前列（堆上下文展开）。

use super::*;

const TOP: usize = 25;

/// 上下文种类
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
enum CtxKind {
    Body,
    Obj,
    Site,
    Const,
    Level,
    Other,
}

impl Engine<'_> {
    fn ctx_kind(&self, c: u32) -> CtxKind {
        if c == NOCTX {
            CtxKind::Body
        } else if self.level_ctxs.contains_key(&c) {
            CtxKind::Level
        } else if self.ctx_heap.contains_key(&c) {
            CtxKind::Const
        } else if self.objs.contains_key(&c) {
            CtxKind::Obj
        } else if self.obj_chain.contains_key(&c) {
            CtxKind::Site
        } else {
            CtxKind::Other
        }
    }

    /// 上下文的首段（分配点 / 调用点；形参常量上下文取其堆上下文）
    fn ctx_site(&self, c: u32) -> Option<&str> {
        let h = self.ctx_heap.get(&c).copied().unwrap_or(c);
        self.obj_chain.get(&h).and_then(|ch| ch.split('#').next())
    }

    /// 分配点段 `@b:off` 的可读标签
    fn seg_label(&self, seg: &str) -> String {
        let parse = || {
            let s = seg.strip_prefix('@')?;
            let (b, off) = s.split_once(':')?;
            let b: usize = b.parse().ok()?;
            Some(format!("{}@{off}", self.method_label(b)))
        };
        parse().unwrap_or_else(|| seg.to_string())
    }

    pub(super) fn ctx_prof_dump(&self, tag: &str) {
        let mut kinds: HashMap<CtxKind, (u64, HashSet<u32>)> = HashMap::default();
        let mut per_m: HashMap<usize, Vec<u32>> = HashMap::default();
        for i in 0..self.methods.len() {
            let c = self.methods[i].ctx;
            let e = kinds.entry(self.ctx_kind(c)).or_default();
            e.0 += 1;
            e.1.insert(c);
            if c != NOCTX {
                let b = self.mbase.get(&self.methods[i].key).copied().unwrap_or(i);
                per_m.entry(b).or_default().push(c);
            }
        }
        let mut ks: Vec<_> = kinds.iter().map(|(k, (n, s))| (*k, *n, s.len())).collect();
        ks.sort();
        eprintln!("{tag} ctxfree hits={} members={}", self.ctx_free_hits, self.ctx_free_memo.values().filter(|&&f| f).count());
        eprintln!("{tag} ctxkind objs={} {}", self.objs.len(), ks.iter().map(|(k, n, d)| format!("{k:?}={n}/{d}")).collect::<Vec<_>>().join(" "));
        let bounds = [8usize, 64, 256, 1024, usize::MAX];
        let mut hist = [(0u64, 0u64); 5];
        for cs in per_m.values() {
            let i = bounds.iter().position(|&b| cs.len() <= b).unwrap_or(4);
            hist[i].0 += 1;
            hist[i].1 += cs.len() as u64;
        }
        eprintln!("{tag} ctxhist (members/nodes) <=8:{:?} <=64:{:?} <=256:{:?} <=1024:{:?} >1024:{:?}", hist[0], hist[1], hist[2], hist[3], hist[4]);
        let mut ms: Vec<(&usize, &Vec<u32>)> = per_m.iter().collect();
        ms.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then(a.0.cmp(b.0)));
        for (&b, cs) in ms.into_iter().take(TOP) {
            let mut kc: HashMap<CtxKind, u32> = HashMap::default();
            let mut sites: HashSet<&str> = HashSet::default();
            let mut classes: HashSet<u32> = HashSet::default();
            let mut typed: HashSet<(u32, &str)> = HashSet::default();
            for &c in cs {
                *kc.entry(self.ctx_kind(c)).or_default() += 1;
                if let Some(s) = self.ctx_site(c) {
                    sites.insert(s);
                }
                if let Some(&t) = self.objs.get(&c) {
                    classes.insert(t);
                    let tail = self.obj_chain.get(&c).and_then(|ch| ch.split_once('#').map(|x| x.1)).unwrap_or("");
                    typed.insert((t, tail));
                }
            }
            let mut kv: Vec<_> = kc.into_iter().collect();
            kv.sort();
            eprintln!("{tag} ctxmerge n={} sites={} classes={} typed={} kinds={kv:?} {}", cs.len(), sites.len(), classes.len(), typed.len(), self.method_label(b));
        }
        let mut by_site: HashMap<&str, u64> = HashMap::default();
        let mut by_cls: HashMap<u32, u64> = HashMap::default();
        for (o, &t) in &self.objs {
            *by_cls.entry(t).or_default() += 1;
            if let Some(s) = self.obj_chain.get(o).and_then(|ch| ch.split('#').next()) {
                *by_site.entry(s).or_default() += 1;
            }
        }
        let mut ss: Vec<_> = by_site.into_iter().collect();
        ss.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        eprintln!("{tag} objsites={}", ss.len());
        for (s, n) in ss.into_iter().take(TOP) {
            let cls = self.seg_cls.get(s).map_or("?", |&t| &*self.names[t as usize]);
            eprintln!("{tag} objsite n={n} {cls} {}", self.seg_label(s));
        }
        let mut cs: Vec<_> = by_cls.into_iter().collect();
        cs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        for (t, n) in cs.into_iter().take(TOP) {
            eprintln!("{tag} objcls n={n} {}", self.names[t as usize]);
        }
    }
}
