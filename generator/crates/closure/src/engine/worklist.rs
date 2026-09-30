//! 引擎：主循环、失效与重分析（分析缓存、站点去重记录）。

use super::*;

impl<'a> Engine<'a> {
    // ── 主循环 ──────────────────────────────────────────────────────────────

    /// 入口形参来自 VM / 手写层：按声明类型 open
    pub(super) fn open_params(&mut self, t: usize) {
        let pts = self.methods[t].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            if let Some(pt) = pt {
                self.add_to(Node::P(t, i as u16), &TypeSet::open(*pt));
            }
        }
    }

    pub fn root(&mut self, key: MemberRef, kind: &'static str) {
        let via = Via::root(kind, &key.to_string());
        self.init(&key.owner.clone(), via.clone());
        let m = self.method(key, via);
        self.open_params(m);
        self.returns_to_vm(m);
    }

    /// VM / 手写层回调的方法：返回值交给非建模代码
    pub(super) fn returns_to_vm(&mut self, t: usize) {
        if let Some(rt) = self.methods[t].rtype {
            self.flow(Node::R(t), Node::Esc, rt);
        }
    }

    pub fn root_upcall(&mut self, u: &Upcall, kind: &'static str) {
        match u {
            Upcall::Method(k) => {
                let via = Via::root(kind, &k.to_string());
                if k.name == "<init>" {
                    self.instantiate(&k.owner, via.clone());
                }
                self.init(&k.owner, via.clone());
                if let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, false) {
                    let (o, n, d) = site.key();
                    let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                    self.open_params(t);
                    self.returns_to_vm(t);
                } else {
                    self.unresolved.insert(k.to_string());
                }
            }
            Upcall::Field(f) => self.init(&f.owner, Via::root(kind, &f.to_string())),
        }
    }

    /// 外部种子方法（缺口扫描的 JDK 入口 / lib 公开 API 面）：等价于「某个用户程序调用了它」，构造器同时实例化
    pub fn root_seed(&mut self, key: MemberRef, kind: &'static str) {
        if key.name == "<init>" {
            self.instantiate(&key.owner.clone(), Via::root(kind, &key.to_string()));
        }
        self.root(key, kind);
    }

    pub fn root_init(&mut self, cls: &str, kind: &'static str) {
        self.init(cls, Via::root(kind, cls));
    }

    pub fn run(&mut self) {
        // 写入未知数组的元素：数组可能由非建模代码持有
        let obj = self.id(OBJECT);
        self.flow(Node::Array, Node::Esc, obj);
        self.ctx.stats.borrow_mut().mark_rss("setup");
        loop {
            self.stat_enter(Phase::Flows);
            self.drain_flows();
            self.stat_leave();
            if let Some((k, e, s)) = self.rpending.pop() {
                self.stat_enter(Phase::Enumerate);
                self.enumerate(k, e, &s);
                self.stat_leave();
                continue;
            }
            // 先处理方法（图扩张），读者站点最后重跑：集合增长在两次重跑之间尽量合并
            let Some(m) = self.mwork.pop_front() else {
                if let Some((m, off)) = self.swork.pop_front() {
                    self.in_swork.remove(&(m, off));
                    self.stat_enter(Phase::Sites);
                    self.ctx.stats.borrow_mut().site_reruns += 1;
                    self.rerun_site(m, off);
                    self.stat_leave();
                    continue;
                }
                if let Some(c) = self.cwork.pop_front() {
                    self.in_cwork.remove(&c);
                    self.stat_enter(Phase::Lcalls);
                    self.ctx.stats.borrow_mut().lcall_reruns += 1;
                    self.rerun_lcall(c);
                    self.stat_leave();
                    continue;
                }
                // 工作队列排空：清单种子按当前可达集补种，补入的新工作继续传播
                self.stat_enter(Phase::Seeds);
                let seeded = self.seed_round();
                self.stat_leave();
                if seeded {
                    continue;
                }
                // 乐观阶段收敛：仍「尚无返回」的被调方法确实不返回。关掉乐观假设，把得到过该答复的
                // 方法按值未知重算——导出的不可达代码只从跳转 / switch / return / athrow 之后开始
                if !self.ctx.optimistic.replace(false) {
                    self.ctx.stats.borrow_mut().mark_rss("final");
                    break;
                }
                let never: Vec<usize> = std::mem::take(&mut *self.ctx.never.borrow_mut()).into_iter().collect();
                self.ctx.stats.borrow_mut().mark_rss("optimistic");
                for m in never {
                    self.invalidate(m, Why::Never);
                }
                continue;
            };
            self.in_mwork.remove(&m);
            self.stat_enter(Phase::Process);
            self.process(m);
            self.stat_leave();
        }
    }

    /// 方法的分析结果失效：重分析，调用方重处理
    pub(super) fn invalidate(&mut self, m: usize, why: Why) {
        let had = self.methods[m].analysis.take().is_some();
        self.ctx.stats.borrow_mut().invalidated(m, why, had);
        if !had && self.in_mwork.contains(&m) {
            return;
        }
        // 调用方对被调方分析的依赖只有透传摘要（返回常量经 rdeps、「尚无返回」经 never 各自失效），
        // 重分析后摘要变化才重处理调用方（见 `analysis`）
        self.push_m(m);
    }

    pub(super) fn invalidate_all(&mut self, ms: Option<BTreeSet<usize>>, why: Why) {
        for m in ms.unwrap_or_default() {
            self.invalidate(m, why);
        }
    }

    /// 字段写入值并入值集；变化时读者失效
    pub(super) fn field_put(&mut self, key: &MemberRef, v: PV) {
        let cur = self.ctx.fvals.borrow().get(key).cloned().unwrap_or_else(|| default_pv(&key.desc));
        let new = PV::join(Some(&cur), &v);
        if new == cur {
            return;
        }
        self.ctx.fvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.fdeps.borrow().get(key).cloned();
        self.invalidate_all(deps, Why::FieldPut);
    }

    /// 字段的写入来源超出字节码：不折叠，读者失效
    pub(super) fn open_field(&mut self, key: MemberRef) {
        if self.ctx.fopen.borrow_mut().insert(key.clone()) {
            self.ctx.cevals.borrow_mut().clear();
            let deps = self.ctx.fdeps.borrow().get(&key).cloned();
            self.invalidate_all(deps, Why::FieldOpen);
            self.open_static(&key);
        }
    }

    pub(super) fn open_field_name(&mut self, name: &str) {
        if self.ctx.fopen_names.borrow_mut().insert(name.to_string()) {
            self.ctx.cevals.borrow_mut().clear();
            let deps: BTreeSet<usize> =
                self.ctx.fdeps.borrow().iter().filter(|(k, _)| k.name == name).flat_map(|(_, v)| v.iter().copied()).collect();
            self.invalidate_all(Some(deps), Why::FieldOpenName);
            // 已登记的同名字段；之后登记的由 field_node 按 fopen_names 接入
            let hits: Vec<MemberRef> = self.fields.keys().filter(|k| k.name == name).cloned().collect();
            for k in hits {
                self.open_static(&k);
            }
        }
    }

    /// 全局开关（反射枚举 / 反序列化）打开：所有读过字段的方法失效
    pub(super) fn open_fields_all(&mut self) {
        self.ctx.cevals.borrow_mut().clear();
        let deps: BTreeSet<usize> = self.ctx.fdeps.borrow().values().flat_map(|v| v.iter().copied()).collect();
        self.invalidate_all(Some(deps), Why::FieldsAll);
    }

    pub(super) fn process(&mut self, m: usize) {
        match self.methods[m].kind {
            Kind::Bytecode => self.process_bytecode(m),
            Kind::Handwritten(HWOBJ_KIND) => self.process_hwobj_method(m),
            Kind::Handwritten(_) => self.process_handwritten(m),
            Kind::Abstract | Kind::Missing => {}
        }
    }

    pub(super) fn analysis(&mut self, m: usize) -> Option<Rc<Analysis>> {
        if let Some(a) = &self.methods[m].analysis {
            return Some(a.clone());
        }
        let key = self.methods[m].key.clone();
        let cf = self.h.class(&key.owner)?;
        let meth = cf.method(&key.name, &key.desc)?;
        let code = meth.code.as_ref()?;
        // catch 类型存活：G 中有其子类型（预先计算，避免 Oracle 借用引擎）
        let mut live_cache: HashMap<String, bool> = HashMap::default();
        for h in &code.exception_table {
            if let Some(ct) = &h.catch_type {
                let tid = self.id(ct);
                let live = !self.g_of(tid).is_empty();
                live_cache.insert(ct.clone(), live);
            }
        }
        let live = |t: &str| live_cache.get(t).copied().unwrap_or(true);
        // 尚无调用点记录的入口（根 / 手写 / VM）：形参值未知，并固定为 Top 保证单调
        let n = self.methods[m].ptypes.len();
        let pv = self.pvals.entry(m).or_insert_with(|| vec![PV::Top; n]);
        let params: Vec<Option<V>> = pv.iter().map(PV::value).collect();
        let facts = Facts { ctx: &self.ctx, live: &live, m: Some(m), params };
        self.stat_enter(Phase::Analyze);
        let a = Rc::new(absint::analyze(&key.owner, &key.desc, meth.is_static(), code, &facts));
        self.stat_leave();
        let unchanged = self.methods[m].applied.as_ref().is_some_and(|o| o.events == a.events);
        self.ctx.stats.borrow_mut().analyzed(m, unchanged);
        // 透传摘要变化：调用方按新摘要重接调用边
        let returned = a.returned_params();
        if self.methods[m].returned.replace(returned.clone()).is_some_and(|old| old != returned) {
            for c in self.callers.get(&m).cloned().unwrap_or_default() {
                self.ctx.stats.borrow_mut().reapply += 1;
                self.methods[c].applied = None;
                self.push_m(c);
            }
        }
        if !a.pending_catch.is_empty() {
            self.pending_catch.insert(m, a.pending_catch.clone());
        }
        self.methods[m].analysis = Some(a.clone());
        Some(a)
    }

    /// 方法在这些偏移处的站点去重记录作废（重分析后事件变化）
    pub(super) fn reset_offsets(&mut self, m: usize, offs: &HashSet<u32>) {
        if let Some(d) = self.dispatched.get_mut(&m) {
            d.retain(|k| !offs.contains(&k.0));
        }
        if let Some(d) = self.hub_linked.get_mut(&m) {
            d.retain(|k| !offs.contains(&k.0));
        }
        if let Some(d) = self.recv_done.get_mut(&m) {
            d.retain(|k| !offs.contains(&k.0));
        }
        if let Some(d) = self.lambda_done.get_mut(&m) {
            for off in offs {
                for (_, id) in d.remove(off).unwrap_or_default() {
                    self.lcalls[id as usize].live = false;
                }
            }
        }
    }

    /// 方法的站点去重记录作废（首次处理 / 被调方摘要变化后站点须完整重接）
    pub(super) fn reset_sites(&mut self, m: usize) {
        self.dispatched.remove(&m);
        self.hub_linked.remove(&m);
        self.recv_done.remove(&m);
        for (_, at) in self.lambda_done.remove(&m).unwrap_or_default() {
            for (_, id) in at {
                self.lcalls[id as usize].live = false;
            }
        }
    }
}
