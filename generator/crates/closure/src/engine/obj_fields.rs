//! 常量查询：按对象的实例字段值（计划 c1d §30.9 的 P1 + P5；站点来源 P4）。
//!
//! 全局值集 `fvals` 按字段键汇合全部对象的写入。本模块在它旁边按抽象对象（容器分配点）分开记录：
//! - **写入**：字节码 `putfield` 的接收者值集里的每个抽象对象，各自并入 `ovals[(对象, 字段)]`；
//!   接收者值集里的其余值（open / 非抽象对象的类 / 无接收者）并入 `owild[字段]`；容器形态类的非抽象对象实例
//!   （反序列化等不经分配点建出、以类 id 出现的对象）与抽象对象不相交，不并入（`untracked_instance`）。
//!   写入值取返回常量格（[`PV::of_ret`]：确定非空的引用记为「非空引用」，`path == null` 一类判定据此可折）。
//!   基本类型字段不拆接收者，物化快照（`concrete/apply.rs`）的写入也一律并入 `owild`。
//!   写站点按接收者增量重跑（`bytecode.rs::field` 的 `recv_delta`），后到的抽象对象同样补记。
//!   两张表都从 ⊥ 起算（不含初值）。
//! - **读取**：`getfield` 的接收者只来自一个形参 / 站点（[`Recv`]），且其值集全由抽象对象组成时，答复
//!   「owild ⊔ 各对象值 ⊔ 未确定初始化对象的初值」，否则退回全局值集。确定初始化（字节码 `new` 分配、
//!   各构造器完成前该字段必然已写且未在写入前交出 / 读取，见 `ctor_init.rs`）的对象不并入初值。
//! - **⊥**（值集为空，或各对象尚无写入）：定论阶段之前答复 [`ObjAns::Never`]——读取之后暂不可达，读者记入
//!   `never`，排空时重算（同 `noreturn.rs`）；定论阶段退回全局值集。⊥ 若在收尾阶段就退回全局值集，随后到达的
//!   对象 / 写入会把答复收窄，而按宽答复建立的边与登记不可撤回，结果依赖处理顺序（散列种子）。
//!   这样答复在定论之前只升不降（Never → 值 → 更大的值 / 退回），导出前全部 Never 都已按定论重算。
//! - **接收者来源**（[`Recv`]）：形参（声明类型为容器形态类），或站点（P4）——结果声明类型为容器形态类的字段读 / 调用
//!   （`obj_site_cands`），对象集取流图站点节点 `S(m, 偏移)` 的值集。站点节点是派发所用的同一值集，覆盖该站点在
//!   方法节点 m 各次执行中产生的全部对象。
//! - **依赖**：每次按对象读都记成 `(来源, 字段, 答复)`（来源无对象集时答复 None，同样记）。
//!   抽象解释是 Oracle 答复的确定函数，所以以下三处都按「答复是否变化」判定，不按「输入是否变化」：
//!   - 按对象值变化（`odeps` / `owdeps` 登记的读者）：记入待复核（原因 = 该字段），
//!   - 来源值集增长（`obj_watch`）：记入待复核（原因 = 增长）；
//!     待复核在流传播排空后 / 主循环每轮开头统一处理（`obj_flush`）：只复核输入可能变了的查询，答复有变才重分析，
//!     不变就只补登新对象的读者。零碎增量合并成一次复核，复核只看当时的终态；
//!   - 共享摘要（`share.rs`）：摘要的每条记录在新上下文的来源对象集下答复都相同，才复用。
//!   开放判定（只增不减）仍经全局读者 `fdeps` 失效。
//!
//! 健全性：
//! - 每次字节码写入要么记到接收者值集里的每个抽象对象，要么记入 `owild`，读取并入两者，
//!   所以覆盖了该对象上的全部字节码写入。未知接收者（open）的写入在流图里流向所有已逃逸对象，这里由 `owild` 覆盖。
//! - 字节码外的写入一律不按对象折叠：手写体写入（`hw_written`），偏移可得（反射 / VarHandle / Unsafe
//!   按名或枚举取得，`fopen*`），以及 VM 字段钩子（`FieldInfo::open`）。
//! - 反序列化不看（P5）：反序列化只写它自己分配的对象，序列化分配的对象是类 id 而不是抽象对象（`serial_alloc.rs`），
//!   不会出现在全由抽象对象组成的值集里。论证同 `construct.rs::object_field`。

use super::*;

/// 按对象查询的接收者来源：只来自一个形参，或只来自一个站点（产生该值的指令偏移，值集为流图站点节点 `S(m, 偏移)`）
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Recv {
    Param(u16),
    Site(u32),
}

impl Recv {
    pub(super) fn of(v: Option<&V>) -> Option<Recv> {
        match &*v?.srcs() {
            [crate::absint::Src::Param(i)] => Some(Recv::Param(*i)),
            [crate::absint::Src::Site(o)] => Some(Recv::Site(*o)),
            _ => None,
        }
    }

    /// 方法 m 中承载该来源值集的流图节点
    pub(super) fn node(self, m: usize) -> Node {
        match self {
            Recv::Param(i) => Node::P(m, i),
            Recv::Site(o) => Node::S(m, o),
        }
    }
}

/// 按对象查询的答复
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ObjAns<T> {
    /// 各对象的值之并
    Value(T),
    /// ⊥（对象集为空，或各对象都尚无值）：定论阶段之前按「读取 / 调用之后暂不可达」答复（乐观，同 `noreturn.rs`）
    Never,
}

/// 一次按对象查询。答复 None = 不按对象折叠，退回全局值集 / 返回常量
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ObjQuery {
    /// 实例字段读：(接收者来源, 字段键, 答复)
    Field(Recv, MemberRef, Option<ObjAns<V>>),
    /// 实例方法返回值（见 `obj_rets.rs`）：(接收者来源, 调用点, 答复)
    Ret(Recv, Rc<super::obj_rets::RetSite>, Option<super::obj_rets::RetAns>),
}

impl ObjQuery {
    fn recv(&self) -> Recv {
        let (ObjQuery::Field(r, ..) | ObjQuery::Ret(r, ..)) = self;
        *r
    }
}

/// 方法节点的接收者对象集：形参与站点各取全由抽象对象组成的值集（可为空集；含其他值 None）。站点只取结果声明类型为容器形态类的字段读 / 调用（`obj_site_cands`）
#[derive(Clone, Default, PartialEq)]
pub(super) struct ObjSets {
    pub(super) params: Vec<Option<Rc<[u32]>>>,
    pub(super) sites: HashMap<u32, Option<Rc<[u32]>>>,
}

impl ObjSets {
    pub(super) fn get(&self, r: Recv) -> Option<&Rc<[u32]>> {
        match r {
            Recv::Param(i) => self.params.get(i as usize).and_then(Option::as_ref),
            Recv::Site(o) => self.sites.get(&o).and_then(Option::as_ref),
        }
    }
}

/// 按对象读待复核的原因（`obj_flush` 据此只复核受影响的查询）
#[derive(Default)]
pub(super) struct ObjDirty {
    /// 全部查询（构造器摘要作废）
    pub(super) all: bool,
    /// 来源值集增长
    pub(super) grown: bool,
    /// 按对象值变化的字段
    pub(super) fields: BTreeSet<MemberRef>,
    /// 按对象返回值变化的方法
    pub(super) rets: BTreeSet<MemberRef>,
}

#[derive(Clone, Copy)]
pub(super) enum ObjCause<'a> {
    Field(&'a MemberRef),
    Ret(&'a MemberRef),
    All,
}

/// 方法已登记的按对象读与登记时的来源对象集（再次登记同一组查询时只补登新增对象）
pub(super) struct ObjBound {
    pub(super) queries: Rc<[ObjQuery]>,
    pub(super) sets: ObjSets,
}

/// 被分析方法的接收者对象集与分析期间的按对象读
#[derive(Default)]
pub(super) struct ObjParams {
    pub(super) sets: ObjSets,
    pub(super) queries: RefCell<Vec<ObjQuery>>,
}

impl Ctx<'_> {
    /// 按对象字段读的答复：值集为 Top 时不折；⊥ 见 [`ObjAns::Never`]（定论阶段退回全局值集）
    pub(super) fn obj_field_answer(&self, sets: &ObjSets, rc: Recv, key: &MemberRef) -> Option<ObjAns<V>> {
        let objs = sets.get(rc)?;
        match self.obj_field_value(key, objs) {
            Some(p) => p.value().map(ObjAns::Value),
            None => self.bottom_answer(),
        }
    }

    /// 按对象查询的 ⊥ 答复：定论阶段之前按不可达（答复随对象集 / 对象值只升不降），定论阶段退回全局值
    pub(super) fn bottom_answer<T>(&self) -> Option<ObjAns<T>> {
        self.noreturn.borrow().bottom_never().then_some(ObjAns::Never)
    }

    /// 可按对象折叠的实例字段：非 static、非 VM 注入字面量、无字节码外的写入来源（反序列化除外，见模块注释）
    pub(super) fn obj_foldable(&self, f: &MemberRef) -> Option<Rc<FieldInfo>> {
        let fi = self.field_info(f)?;
        let open = fi.open || self.field_offset_under(&fi, self.fopen_all.get(), false) || self.hw_written(&fi);
        (fi.access & acc::STATIC == 0 && self.man.injected_literal(&fi.key.owner, &fi.key.name).is_none() && !open).then_some(fi)
    }

    /// 抽象对象集 objs 上实例字段 key 的值：通配值 ⊔ 各对象值 ⊔ 未确定初始化对象的初值；⊥ → None
    pub(super) fn obj_field_value(&self, key: &MemberRef, objs: &[u32]) -> Option<PV> {
        let mut pv: Option<PV> = self.owild.borrow().get(key).cloned();
        let ov = self.ovals.borrow();
        let ov = ov.get(key);
        let mut dflt = false;
        for &o in objs {
            if !dflt && self.obj_definite(o).binary_search(key).is_err() {
                dflt = true;
                pv = Some(PV::join(pv.as_ref(), &default_pv(&key.desc)));
            }
            if let Some(x) = ov.and_then(|ov| ov.get(&o)) {
                pv = Some(PV::join(pv.as_ref(), x));
            }
        }
        pv
    }
}

impl Facts<'_, '_> {
    /// 按对象的字段读：接收者只来自一个形参 / 站点。无对象集时同样登记查询（答复 None），共享判定据此比对（见 `share.rs`）
    pub(super) fn obj_field(&self, f: &MemberRef, recv: Option<&V>) -> Option<ObjAns<V>> {
        let m = self.m?;
        let rc = Recv::of(recv)?;
        let fi = self.ctx.obj_foldable(f)?;
        // 全局读者登记：开放判定变化（只增不减）令其失效
        self.ctx.dep(m, Dep::Field(fi.key.clone()));
        let r = self.ctx.obj_field_answer(&self.objs.sets, rc, &fi.key);
        self.objs.queries.borrow_mut().push(ObjQuery::Field(rc, fi.key.clone(), r.clone()));
        r
    }
}

impl Engine<'_> {
    /// 方法 m 的接收者对象集：形参只看声明类型为容器形态类的引用形参，站点见 `obj_site_cands`
    pub(super) fn obj_sets(&mut self, m: usize) -> ObjSets {
        let params = (0..self.methods[m].ptypes.len()).map(|i| self.obj_param_set(m, i as u16)).collect();
        let mut sites = HashMap::default();
        for &off in self.obj_site_cands(m).iter() {
            sites.insert(off, self.node_objs(Node::S(m, off)));
        }
        ObjSets { params, sites }
    }

    /// 形参 i 的对象集：只看声明类型为容器形态类的引用形参
    fn obj_param_set(&mut self, m: usize, i: u16) -> Option<Rc<[u32]>> {
        let t = (*self.methods[m].ptypes.get(i as usize)?)?;
        if self.arrays.contains_key(&t) || self.names[t as usize].starts_with('[') {
            return None;
        }
        let name = self.names[t as usize].clone();
        if self.container(&name) {
            self.node_objs(Node::P(m, i))
        } else {
            None
        }
    }

    /// 节点值集全由抽象对象组成时的对象集（可为空）；含 open / 非抽象对象的类 / 镜像时 None。顺带记下对象的类
    fn node_objs(&self, n: Node) -> Option<Rc<[u32]>> {
        let Some(s) = self.graph.get(&n) else { return Some(Rc::from([])) };
        let all_objs = s.open.is_empty() && s.classes.iter().all(|x| self.objs.contains_key(&x) && !self.mirrors.contains_key(&x));
        if !all_objs {
            return None;
        }
        // 对象的类（按对象选择调用目标用，见 `obj_rets.rs`）
        let mut oc = self.ctx.oclass.borrow_mut();
        for x in s.classes.iter() {
            oc.entry(x).or_insert_with(|| self.names[self.objs[&x] as usize].clone());
        }
        Some(s.classes.iter().collect::<Vec<u32>>().into())
    }

    /// 方法 m 中可作按对象接收者来源的站点：结果声明类型为容器形态类的实例 / 静态字段读与调用（按方法键缓存）
    fn obj_site_cands(&mut self, m: usize) -> Rc<[u32]> {
        let key = self.methods[m].key.clone();
        if let Some(c) = self.site_cands.get(&key) {
            return c.clone();
        }
        let mut out: Vec<u32> = Vec::new();
        let code = self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).and_then(|mm| mm.code.clone()));
        for x in code.iter().flat_map(|c| c.insns.iter()) {
            let ty = match (x.opcode, &x.operand) {
                (classfile::op::GETFIELD | classfile::op::GETSTATIC, classfile::Operand::Field(f)) => f.desc.as_str(),
                (classfile::op::INVOKEVIRTUAL | classfile::op::INVOKESPECIAL | classfile::op::INVOKESTATIC | classfile::op::INVOKEINTERFACE, classfile::Operand::Method(r, _)) => {
                    r.desc.rsplit(')').next().unwrap_or("")
                }
                _ => continue,
            };
            let Some(cls) = ty.strip_prefix('L').and_then(|t| t.strip_suffix(';')) else { continue };
            if self.container(cls) {
                out.push(x.offset);
            }
        }
        let out: Rc<[u32]> = out.into();
        self.site_cands.insert(key, out.clone());
        out
    }

    /// 值集里的类 id x 是容器形态类的非抽象对象实例：容器形态类经字节码 `new`、lambda、手写体与引导映像建出的对象一律是
    /// 抽象对象（`obj_at` / `image_obj_site`），具体求值的结果引用容器形态对象时整体回退（`concrete.rs`），所以值集里
    /// 以类 id 出现的该类实例只来自不经分配点的途径（反序列化的 `<alloc>`、反射 / Unsafe 分配），与任何抽象对象都不是
    /// 同一个运行期对象。其上的字节码写入只需对按类 id 的读取可见（全局值集 `fvals` 照常并入），不并入 `owild`——
    /// 否则 `readObject` → 解析器写回的字段（URI 的 `authority` 等）令全部抽象对象的按对象读退回 Top（计划 c1d §31.3 ④-3）
    pub(super) fn untracked_instance(&mut self, x: u32) -> bool {
        if self.objs.contains_key(&x) || self.arrays.contains_key(&x) || self.mirrors.contains_key(&x) || self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
            return false;
        }
        let name = self.names[x as usize].clone();
        !name.starts_with('[') && self.container(&name)
    }

    /// 字节码写站点：值并入接收者值集里各抽象对象的字段值；其余接收者并入 `owild`
    pub(super) fn obj_field_put(&mut self, key: &MemberRef, objs: &[u32], other: bool, v: &PV) {
        // 只复核读过变化对象的方法（按对象登记于 `odeps`）；通配值变化复核 `owdeps` 登记的全部读者
        let mut readers: BTreeSet<usize> = BTreeSet::new();
        if !objs.is_empty() {
            let mut ovals = self.ctx.ovals.borrow_mut();
            let ov = ovals.entry(key.clone()).or_default();
            let deps = self.ctx.odeps.borrow();
            let deps = deps.get(key);
            for &o in objs {
                // 从 ⊥ 起算：初值由读取侧按确定初始化并入
                let cur = ov.get(&o);
                let new = PV::join(cur, v);
                if cur != Some(&new) {
                    ov.insert(o, new);
                    readers.extend(deps.and_then(|d| d.get(&o)).into_iter().flatten().copied());
                }
            }
        }
        self.obj_readers_recheck(readers, ObjCause::Field(key));
        if other {
            self.wild_put(key, v);
        }
    }

    /// 接收者不按对象分开的写入（物化快照等）：并入 `owild`
    pub(super) fn wild_put(&mut self, key: &MemberRef, v: &PV) {
        if self.wild_join(key, v) {
            let deps = self.ctx.owdeps.borrow().get(key).cloned().unwrap_or_default();
            self.obj_readers_recheck(deps, ObjCause::Field(key));
        }
    }

    fn wild_join(&mut self, key: &MemberRef, v: &PV) -> bool {
        let mut w = self.ctx.owild.borrow_mut();
        let cur = w.get(key).cloned();
        let new = PV::join(cur.as_ref(), v);
        if cur.as_ref() == Some(&new) {
            return false;
        }
        w.insert(key.clone(), new);
        true
    }

    /// 方法 m 的按对象读（按 m 自己的形参对象集）：登记逐对象读者，形参值集增长时重分析。
    /// 同一组查询再次登记（复核答复不变）时只补登新增对象
    pub(super) fn obj_queries_bind(&mut self, m: usize, sets: &ObjSets, queries: &Rc<[ObjQuery]>) {
        if queries.is_empty() {
            self.obj_queries.remove(&m);
            return;
        }
        let prev = self.obj_queries.insert(m, ObjBound { queries: queries.clone(), sets: sets.clone() });
        let prev = prev.filter(|p| Rc::ptr_eq(&p.queries, queries));
        let mut watched: Vec<Recv> = Vec::new();
        for q in queries.iter() {
            let rc = q.recv();
            if !watched.contains(&rc) {
                watched.push(rc);
                let n = rc.node(m);
                // 无对象集的来源同样盯住：值集由空变为全抽象对象时答复可能变窄（复核见 `obj_grown`）
                self.obj_watch.entry(n).or_default().insert(m);
                self.graph.mark_hooked(n);
            }
            let Some(objs) = sets.get(rc) else { continue };
            let old = prev.as_ref().and_then(|p| p.sets.get(rc));
            let fresh = |o: &u32| old.is_none_or(|old| old.binary_search(o).is_err());
            match q {
                ObjQuery::Field(_, key, _) => {
                    let mut od = self.ctx.odeps.borrow_mut();
                    let od = od.entry(key.clone()).or_default();
                    for o in objs.iter().filter(|o| fresh(o)) {
                        od.entry(*o).or_default().insert(m);
                    }
                    if prev.is_none() {
                        self.ctx.owdeps.borrow_mut().entry(key.clone()).or_default().insert(m);
                    }
                }
                // 各对象所执行的方法分别登记（选择只看对象的类，不随分析变化）
                ObjQuery::Ret(_, s, _) => {
                    for &o in objs.iter().filter(|o| fresh(o)) {
                        let Some(t) = self.ctx.obj_ret_key(s, o) else { continue };
                        self.ctx.ordeps.borrow_mut().entry((*t).clone()).or_default().entry(o).or_default().insert(m);
                        let mut wd = self.ctx.orwdeps.borrow_mut();
                        match wd.get_mut(&*t) {
                            Some(d) => {
                                d.insert(m);
                            }
                            None => {
                                wd.insert((*t).clone(), BTreeSet::from([m]));
                            }
                        }
                    }
                }
            }
        }
    }

    /// 摘要的按对象读在形参对象集 sets 下答复不变（共享前提，见 `share.rs`）
    pub(super) fn obj_queries_same(&self, sets: &ObjSets, queries: &[ObjQuery]) -> bool {
        queries.iter().all(|q| self.obj_query_same(sets, q))
    }

    fn obj_query_same(&self, sets: &ObjSets, q: &ObjQuery) -> bool {
        match q {
            ObjQuery::Field(rc, key, r) => self.ctx.obj_foldable(key).and_then(|_| self.ctx.obj_field_answer(sets, *rc, key)) == *r,
            ObjQuery::Ret(rc, s, r) => self.ctx.obj_ret_answer(sets, *rc, s) == *r,
        }
    }

    /// 按对象值变化：读者记入待复核（流传播排空后一次复核，见 [`Self::obj_flush`]）
    pub(super) fn obj_readers_recheck(&mut self, readers: BTreeSet<usize>, cause: ObjCause<'_>) {
        for m in readers {
            let d = self.obj_dirty.entry(m).or_default();
            match cause {
                ObjCause::Field(k) => {
                    if !d.fields.contains(k) {
                        d.fields.insert(k.clone());
                    }
                }
                ObjCause::Ret(t) => {
                    if !d.rets.contains(t) {
                        d.rets.insert(t.clone());
                    }
                }
                ObjCause::All => d.all = true,
            }
        }
    }

    /// 节点 n 的值集增长：按对象读过它的方法记入待复核
    pub(super) fn obj_grown(&mut self, n: Node) {
        if let Some(ms) = self.obj_watch.remove(&n) {
            for m in ms {
                self.obj_dirty.entry(m).or_default().grown = true;
            }
        }
    }

    /// 待复核的方法按当前来源对象集与按对象值复核答复：都不变则补登新对象的读者与来源盯防，否则重分析。
    /// 只复核输入可能变了的查询（来源对象集变化，或所读字段 / 同名同描述符方法的按对象值变化；构造器摘要作废时全部），
    /// 其余查询的答复是同一组输入的确定函数。分析是 Oracle 答复的确定函数，所以按答复判定；同一批增量
    /// （一次流排空、一次方法处理）合并成一次复核，复核只看终态，与增量到达的先后无关
    pub(super) fn obj_flush(&mut self) {
        while !self.obj_dirty.is_empty() {
            let dirty = std::mem::take(&mut self.obj_dirty);
            for (m, d) in dirty {
                let Some(b) = self.obj_queries.get(&m) else { continue };
                let (qs, old) = (b.queries.clone(), b.sets.clone());
                let reset = d.grown || d.all;
                let sets = if reset { self.obj_sets_of(m, &qs) } else { old.clone() };
                let same = qs.iter().all(|q| {
                    let rc = q.recv();
                    let need = d.all
                        || sets.get(rc) != old.get(rc)
                        || match q {
                            ObjQuery::Field(_, k, _) => d.fields.contains(k),
                            ObjQuery::Ret(_, s, _) => d.rets.iter().any(|t| t.name == s.m.name && t.desc == s.m.desc),
                        };
                    !need || self.obj_query_same(&sets, q)
                });
                if !same {
                    self.invalidate(m, Why::Mirror);
                } else if reset {
                    self.obj_queries_bind(m, &sets, &qs);
                }
            }
        }
    }

    /// 只含查询所用来源的对象集（复核用）
    fn obj_sets_of(&mut self, m: usize, qs: &[ObjQuery]) -> ObjSets {
        let mut sets = ObjSets { params: vec![None; self.methods[m].ptypes.len()], sites: HashMap::default() };
        for q in qs {
            let rc = q.recv();
            match rc {
                Recv::Param(i) if sets.params.get(i as usize).is_some_and(Option::is_none) => {
                    sets.params[i as usize] = self.obj_param_set(m, i);
                }
                Recv::Site(o) if !sets.sites.contains_key(&o) => {
                    // 与 `obj_sets` 同口径：非候选站点无对象集
                    let v = if self.obj_site_cands(m).contains(&o) { self.node_objs(Node::S(m, o)) } else { None };
                    sets.sites.insert(o, v);
                }
                _ => {}
            }
        }
        sets
    }
}
