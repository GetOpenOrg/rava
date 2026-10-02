//! 引擎：G 增长的连带重跑——派发缓存（g_sub）、枢纽增量展开、open 展开的方法与站点、等待异常类型的方法。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn on_g_grow(&mut self, id: u32) {
        let ts: Vec<u32> = self.g_sub.keys().copied().collect();
        for t in ts {
            if self.sub(id, t) {
                // 有序插入
                let v = self.g_sub.get_mut(&t).unwrap();
                if let Err(i) = v.binary_search(&id) {
                    v.insert(i, id);
                }
            }
        }
        self.hubs_grow(id);
        self.reopen(id);
        self.vm_hooks_on_alloc(id);
        let pend: Vec<(usize, Vec<String>)> = self.pending_types.iter().map(|(k, v)| (*k, v.clone())).collect();
        for (m, tys) in pend {
            let hit = tys.iter().any(|t| {
                let tid = self.id(t);
                self.sub(id, tid)
            });
            if hit {
                self.pending_types.remove(&m);
                let had = self.methods[m].analysis.take().is_some();
                if had {
                    self.nr_dropped(m);
                }
                self.ctx.stats.borrow_mut().invalidated(m, Why::Catch, had);
                self.push_m(m);
            }
        }
    }

    /// open 展开的取值面扩大（G 增长 / 数组逃逸）：x 落在其 open 类型与接收者上界之下的方法与站点重跑
    pub(super) fn reopen(&mut self, x: u32) {
        self.reopen_mirrors(x);
        let keys: Vec<(u32, u32)> =
            self.open_methods.keys().chain(self.open_sites.keys()).chain(self.open_calls.keys()).copied().collect();
        let hit: HashSet<(u32, u32)> = keys.into_iter().filter(|&(o, owner)| self.sub(x, o) && self.sub(x, owner)).collect();
        let mut open: BTreeSet<usize> = BTreeSet::new();
        let mut sites: BTreeSet<(usize, u32)> = BTreeSet::new();
        for (k, ms) in &self.open_methods {
            if hit.contains(k) {
                open.extend(ms.iter().copied());
            }
        }
        for (k, ws) in &self.open_sites {
            if hit.contains(k) {
                sites.extend(ws.iter().copied());
            }
        }
        let mut calls: BTreeSet<u32> = BTreeSet::new();
        for (k, cs) in &self.open_calls {
            if hit.contains(k) {
                calls.extend(cs.iter().copied());
            }
        }
        for c in calls {
            if self.in_cwork.insert(c) {
                self.cwork.push_back(c);
            }
        }
        for m in open {
            self.push_m(m);
        }
        for w in sites {
            if self.in_swork.insert(w) {
                self.swork.push_back(w);
            }
        }
    }

}
