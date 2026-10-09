//! 引擎：主循环、失效与重分析（分析缓存、站点去重记录）。

use super::*;

/// 缺省流传播批量（`Engine::flow_batch`）
pub const FLOW_BATCH: usize = 64;

impl<'a> Engine<'a> {
    // ── 主循环 ──────────────────────────────────────────────────────────────

    /// 入口形参来自 VM / 手写层：按声明类型 open，形参常量并入 Top。
    /// 常量必须在入口处显式并入：只靠「分析时尚无调用点记录即 Top」的兜底，先经字节码调用点（常量实参）
    /// 入链、后经反射 / VM 入口的方法会保留先到的常量，结果随传播顺序变化
    pub(super) fn open_params(&mut self, t: usize) {
        let n = self.methods[t].ptypes.len();
        self.bind_pvs(t, 0, n, None);
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

    /// 外部种子方法（缺口扫描的 JDK 入口 / lib 公开 API 面）：等价于「某个用户程序调用了它」，构造器同时实例化
    pub fn root_seed(&mut self, key: MemberRef, kind: &'static str) {
        if key.name == "<init>" {
            self.instantiate(&key.owner.clone(), Via::root(kind, &key.to_string()));
        }
        self.root(key, kind);
    }

    pub fn run(&mut self) {
        // 写入未知数组的元素：数组可能由非建模代码持有
        let obj = self.id(OBJECT);
        self.flow(Node::Array, Node::Esc, obj);
        self.ctx.stats.borrow_mut().mark_rss("setup");
        let mut batch = 0usize;
        loop {
            self.pkey_flush();
            if !self.obj_dirty.is_empty() {
                self.stat_enter(Phase::Flows);
                self.obj_flush();
                self.stat_leave();
            }
            // 流传播按批：连续处理若干方法 / 站点后再排空，各处的零碎增量在源头汇齐后一次推下去。
            // 不动点单调，先处理的单元读到的是较小的集合，增长后经读者登记重跑——终态集合与逐个排空相同
            let idle = self.mwork.is_empty() && self.swork.is_empty() && self.cwork.is_empty();
            if idle || batch >= self.flow_batch {
                batch = 0;
                self.stat_enter(Phase::Flows);
                self.drain_flows();
                self.obj_flush();
                self.stat_leave();
            }
            batch += 1;
            if let Some((k, e, s)) = self.rpending.pop() {
                self.stat_enter(Phase::Enumerate);
                match k {
                    RHook::Enum(k) => self.enumerate(k, e, &s),
                    // 反射调用实参池 / 实参数组增长（e = 通道），见 `reflect_call.rs`
                    RHook::Pool => self.rcall_pool_grown(e as u8, &s),
                    RHook::Array => self.rcall_array_grown(e as u8, &s),
                }
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
                // 反射调用实参池的去冗余视图在不动点上判定涵盖（`reflect_call.rs::rcall_release`）
                if self.rcall_release() {
                    continue;
                }
                // 直连反射调用点的精确接收者在不动点上判定涵盖（`reflect_direct.rs::rdirect_release`）
                if self.rdirect_release() {
                    continue;
                }
                // 工作队列排空：清单种子按当前可达集补种，补入的新工作继续传播
                self.stat_enter(Phase::Seeds);
                let seeded = self.seed_round();
                self.stat_leave();
                if seeded {
                    continue;
                }
                // 名字齐全的按名取类站点在不动点上放行（`class_lookup.rs::lookup_release`）
                if self.lookup_release() {
                    continue;
                }
                // 反射数组分配调用点在不动点上放行逐类型分配点（`reflect.rs::array_of_release`）
                if self.array_of_release() {
                    continue;
                }
                // 收尾：得到过「不返回」答复的方法按值未知重算（定论判定见 `noreturn.rs`）
                if self.nr_drain() {
                    continue;
                }
                // JCA 提供者序：其余放行都完成后判定装载器调用点能否继续扣住（`jca_order.rs`）
                if self.jca_order_release() {
                    continue;
                }
                self.promote_layout();
                self.ctx.stats.borrow_mut().mark_rss("final");
                break;
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
        if had {
            self.nr_dropped(m);
        }
        self.ctx.stats.borrow_mut().invalidated(m, why, had);
        if !had && self.in_mwork.contains(&m) {
            return;
        }
        // 调用方对被调方分析的依赖只有透传摘要（返回常量经 rdeps、「尚无返回」经 never 各自失效），
        // 重分析后摘要变化才重处理调用方（见 `analysis`）
        self.push_m(m);
    }

    pub(super) fn invalidate_all(&mut self, ms: Option<BTreeSet<usize>>, why: Why) {
        self.obj_defs_dropped();
        for m in ms.unwrap_or_default() {
            self.invalidate(m, why);
        }
    }

    /// 字段写入值并入值集；变化时读者失效。
    ///
    /// 静态字段的缺省值总可观察（初始化前 / 初始化中读）；实例字段的缺省值只在有对象处于该状态时并入：
    /// 抽象分配（`alloc_defaults`）或物化快照中未写的字段。值集为空（尚无对象）时读者按缺省值读
    pub(super) fn field_put(&mut self, key: &MemberRef, v: PV) {
        let cur = self.ctx.fvals.borrow().get(key).cloned().or_else(|| self.is_static_key(key).then(|| self.ctx.static_initial(key)));
        let new = PV::join(cur.as_ref(), &v);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.ctx.fvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.fdeps.borrow().get(key).cloned();
        self.invalidate_all(deps, Why::FieldPut);
    }

    fn is_static_key(&self, key: &MemberRef) -> bool {
        self.h.class(&key.owner).and_then(|c| c.field(&key.name, &key.desc).map(|f| f.is_static())).unwrap_or(true)
    }

    /// 类首次抽象分配：本类及超类的实例字段缺省值并入值集
    pub(super) fn alloc_defaults(&mut self, cls: &str) {
        let id = self.id(cls);
        if cls.starts_with('[') || !self.dflt_alloc.insert(id) {
            return;
        }
        let mut keys = Vec::new();
        let mut cur = self.h.class(cls);
        while let Some(cf) = cur {
            keys.extend(cf.fields.iter().filter(|f| !f.is_static()).map(|f| MemberRef { owner: cf.name.clone(), name: f.name.clone(), desc: f.desc.clone() }));
            cur = cf.super_name.as_deref().and_then(|s| self.h.class(s));
        }
        for k in keys {
            let d = default_pv(&k.desc);
            self.field_put(&k, d);
        }
    }

    /// 字段的写入来源超出字节码：不折叠，读者失效
    pub(super) fn open_field(&mut self, key: MemberRef) {
        if self.ctx.fopen.borrow_mut().insert(key.clone()) {
            let mut deps = self.ctx.ceval_drop_field(&key);
            deps.extend(self.ctx.fdeps.borrow().get(&key).into_iter().flatten().copied());
            self.invalidate_all(Some(deps), Why::FieldOpen);
            self.open_static(&key);
            self.offset_fields_opened(Some((&key.name, Some(&key))));
        }
    }

    pub(super) fn open_field_name(&mut self, name: &str) {
        if self.ctx.fopen_names.borrow_mut().insert(name.to_string()) {
            self.fopen_name_via.insert(name.to_string(), self.cur_site);
            let mut deps = self.ctx.ceval_drop_name(name);
            deps.extend(self.ctx.fdeps.borrow().iter().filter(|(k, _)| k.name == name).flat_map(|(_, v)| v.iter().copied()));
            self.invalidate_all(Some(deps), Why::FieldOpenName);
            // 已登记的同名字段；之后登记的由 field_node 按 fopen_names 接入
            let hits: Vec<MemberRef> = self.fields.keys().filter(|k| k.name == name).cloned().collect();
            for k in hits {
                self.open_static(&k);
            }
            self.offset_fields_opened(Some((name, None)));
        }
    }

    /// 全局开关（反射枚举 / 反序列化）打开（was_all / was_deser = 打开前的开关）：读过因此转为不折叠的
    /// 字段的方法失效（此前已不折叠的字段读答复不变）
    pub(super) fn open_fields_all(&mut self, was_all: bool, was_deser: bool) {
        let ctx = &self.ctx;
        let mut changed: HashMap<MemberRef, bool> = HashMap::default();
        let mut newly = |k: &MemberRef| match changed.get(k) {
            Some(&c) => c,
            None => {
                let c = ctx.field_info(k).is_none_or(|fi| ctx.field_open(&fi) && !ctx.field_open_under(&fi, was_all, was_deser));
                changed.insert(k.clone(), c);
                c
            }
        };
        let mut deps = ctx.ceval_drop(|inp| inp.reads.iter().any(&mut newly));
        let keys: Vec<MemberRef> = ctx.fdeps.borrow().keys().cloned().collect();
        for k in keys.iter().filter(|k| newly(k)) {
            deps.extend(ctx.fdeps.borrow().get(k).into_iter().flatten().copied());
        }
        self.invalidate_all(Some(deps), Why::FieldsAll);
        self.offset_fields_opened(None);
    }

    pub(super) fn process(&mut self, m: usize) {
        if self.cuts.active() && self.cuts.method(&self.methods[m].key.to_string()) {
            return;
        }
        match self.methods[m].kind {
            Kind::Bytecode if self.is_concrete(m) => self.process_concrete(m),
            Kind::Bytecode => self.process_bytecode(m),
            Kind::Handwritten(HWOBJ_KIND) => self.process_hwobj_method(m),
            Kind::Handwritten(HWFIELD_KIND) => self.process_hwfield_method(m),
            Kind::Handwritten(VMHOOK_KIND) => self.process_vm_hook(m),
            Kind::Handwritten(RTFN_KIND) => self.process_rt_fn(m),
            Kind::Handwritten(_) => self.process_handwritten(m),
            Kind::Abstract | Kind::Missing => {}
        }
    }

    /// 站点求值（按名查找的名字、键集、字段 / 返回串）所用的分析：其事件正在 / 已经执行的那次分析（`applied`），
    /// 未执行过时取当前分析。方法失效后、重分析前，站点仍可因接收者 / 键集增长按 `applied` 的事件重跑：
    /// 此时实参来自 `applied`，名字也须按它求值——取当前分析（已失效为空）会把这次求值当成「推不出」，
    /// 放宽结果（任意键 / 开放查找）不可撤回，结果依赖失效与重跑的先后
    pub(super) fn site_analysis(&self, m: usize) -> Option<Rc<Analysis>> {
        self.methods[m].applied.clone().or_else(|| self.methods[m].analysis.clone())
    }

    pub(super) fn analysis(&mut self, m: usize) -> Option<Rc<Analysis>> {
        if self.is_concrete(m) {
            return None;
        }
        if let Some(a) = &self.methods[m].analysis {
            return Some(a.clone());
        }
        let key = self.methods[m].key.clone();
        let cf = self.h.class(&key.owner)?;
        let meth = cf.method(&key.name, &key.desc)?;
        let code = meth.code.as_ref()?;
        // catch / instanceof 目标类型存活：G 中有其子类型（预先计算，避免 Oracle 借用引擎）
        let mut live_cache: HashMap<String, bool> = HashMap::default();
        let inst_types = code.insns.iter().filter(|x| x.opcode == classfile::op::INSTANCEOF).filter_map(|x| match &x.operand {
            classfile::Operand::Class(c) => Some(c),
            _ => None,
        });
        for ct in code.exception_table.iter().filter_map(|h| h.catch_type.as_ref()).chain(inst_types) {
            if !live_cache.contains_key(ct) {
                let tid = self.id(ct);
                let live = !self.g_of(tid).is_empty();
                live_cache.insert(ct.clone(), live);
            }
        }
        let live = |t: &str| live_cache.get(t).copied().unwrap_or(true);
        self.image_settle_reads(code);
        // 尚无调用点记录即被分析：只剩无形参值可言的入口（<clinit>、序列化分配的无参构造器、按上下文克隆的
        // lambda 实现 / 具体求值节点，其各入口一律不带实参值），形参值未知，固定为 Top
        let n = self.methods[m].ptypes.len();
        if !self.pvals.contains_key(&m) {
            self.pstr_top_m(m);
            self.taint_params(m, 0, n, None);
        }
        let pv = self.pvals.entry(m).or_insert_with(|| vec![PV::Top; n]);
        let params: Vec<Option<V>> = pv.iter().map(PV::value).collect();
        let mirrors = self.param_mirror_sets(m);
        let pobjs = self.obj_sets(m);
        let callers = self.caller_mirrors(m);
        let sites = self.site_table(m, code);
        self.stat_enter(Phase::Analyze);
        self.nr_begin(m);
        // 入口状态相同的有效摘要：直接共享并重放其依赖（收尾阶段不共享，见 `share.rs`）
        let closing = self.ctx.noreturn.borrow().closing();
        // 档位上下文（`levels_boot.rs`）的克隆按档位折叠引导查询：不与本体共享摘要
        let level = self.level_of(m);
        let reuse = if closing || level.is_some() { None } else { self.shared_analysis(&key, &params, &mirrors, &pobjs, &sites) };
        let (a, queries) = if let Some((a, deps, queries)) = reuse {
            self.share_join(m, &a, &deps);
            (a, queries)
        } else {
            let entry = (!closing && level.is_none()).then(|| (params.clone(), mirrors.clone()));
            *self.ctx.dep_log.borrow_mut() = entry.is_some().then(Vec::new);
            let objs = super::obj_fields::ObjParams { sets: pobjs.clone(), queries: Default::default() };
            let facts = Facts { ctx: &self.ctx, live: &live, m: Some(m), params, mirrors, level, objs, callers, caller_sites: Default::default(), sites: sites.clone() };
            let mut a = absint::analyze(&key.owner, &key.desc, meth.is_static(), code, &facts);
            let queries: Rc<[super::obj_fields::ObjQuery]> = facts.objs.queries.take().into();
            let deps = self.ctx.dep_log.borrow_mut().take();
            if self.cold_cut {
                let cold = crate::cold::doomed(code);
                let hot: HashSet<u32> = code.insns.iter().zip(&cold).filter(|(_, c)| !**c).map(|(x, _)| x.offset).collect();
                a.events.retain(|(off, _)| hot.contains(off));
            }
            // 摘要常驻到下次失效（读者重跑 / 重分析按偏移比对都要用），收掉构建期的余量
            a.events.shrink_to_fit();
            let a = Rc::new(a);
            if let (Some((params, mirrors)), Some(deps)) = (entry, deps) {
                self.share_record(m, (params, mirrors, queries.clone()), sites, &a, deps);
            }
            (a, queries)
        };
        self.obj_queries_bind(m, &pobjs, &queries);
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
        if !a.pending_types.is_empty() {
            self.pending_types.insert(m, a.pending_types.clone());
        }
        for (i, c) in &a.mirror_assumed {
            let cid = self.id(c);
            self.mirror_watch.entry(Node::P(m, *i)).or_default().insert((m, cid));
            self.graph.mark_hooked(Node::P(m, *i));
        }
        if a.site_mirror_assumed {
            self.caller_watch(m);
        }
        for &i in &a.mirror_field_assumed {
            self.mirror_watch.entry(Node::P(m, i)).or_default().insert((m, HOOK_FIELD));
            self.graph.mark_hooked(Node::P(m, i));
        }
        self.methods[m].analysis = Some(a.clone());
        self.methods[m].aseq = self.methods[m].aseq.wrapping_add(1);
        self.nr_end(m);
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
        if let Some(d) = self.hub_lsent.get_mut(&m) {
            d.retain(|k| !offs.contains(&k.0));
        }
        if let Some(d) = self.rdirect_done.get_mut(&m) {
            d.retain(|k, _| !offs.contains(&k.0));
        }
        if let Some(d) = self.recv_done.get_mut(&m) {
            d.retain(|o, _| !offs.contains(o));
        }
        if let Some(d) = self.recv_fp.get_mut(&m) {
            d.retain(|o, _| !offs.contains(o));
        }
        if let Some(d) = self.gather_last.get_mut(&m) {
            d.retain(|k, _| !offs.contains(&k.0));
        }
        if let Some(d) = self.refl_seen.get_mut(&m) {
            d.retain(|o, _| !offs.contains(o));
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
        self.hub_lsent.remove(&m);
        self.rdirect_done.remove(&m);
        self.recv_done.remove(&m);
        self.recv_fp.remove(&m);
        self.gather_last.remove(&m);
        self.refl_seen.remove(&m);
        for (_, at) in self.lambda_done.remove(&m).unwrap_or_default() {
            for (_, id) in at {
                self.lcalls[id as usize].live = false;
            }
        }
    }
}
