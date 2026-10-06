//! 常量查询：按对象的实例字段值（计划 c1d §30.9 的 P1 + P5）。
//!
//! 全局值集 `fvals` 按字段键汇合全部对象的写入。本模块在它旁边按抽象对象（容器分配点）分开记录：
//! - **写入**：字节码 `putfield` 的接收者值集里的每个抽象对象，各自并入 `ovals[(对象, 字段)]`；
//!   接收者值集里的其余值（open / 非抽象对象的类 / 无接收者）并入 `owild[字段]`。
//!   基本类型字段不拆接收者，物化快照（`concrete/apply.rs`）的写入也一律并入 `owild`。
//!   写站点按接收者增量重跑（`bytecode.rs::field` 的 `recv_delta`），后到的抽象对象同样补记。
//!   两张表都从初值起算，写入初值不算变化。
//! - **读取**：`getfield` 的接收者只来自形参 i，且该形参值集非空、全由抽象对象组成时，答复
//!   「初值 ⊔ owild ⊔ 各对象值」，否则退回全局值集。初值总并入，构造器确定初始化另做（见 §30.9 P2）。
//! - **依赖**：每次按对象读都记成 `(形参, 字段, 答复)`（形参无对象集时答复 None，同样记）。
//!   抽象解释是 Oracle 答复的确定函数，所以以下三处都按「答复是否变化」判定，不按「输入是否变化」：
//!   - 按对象值变化（`odeps` / `owdeps` 登记的读者）：用当前形参对象集复核，答复有变才重分析；
//!   - 形参值集增长（`obj_watch`）：同样复核，答复不变就只补登新对象的读者；
//!   - 共享摘要（`share.rs`）：摘要的每条记录在新上下文的形参对象集下答复都相同，才复用。
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

/// 一次按对象字段读：(形参序号, 字段键, 按对象的答复)。答复 None = 不按对象折叠，读取退回全局值集
pub(super) type ObjQuery = (u16, MemberRef, Option<V>);

/// 被分析方法各形参的抽象对象集（全由抽象对象组成的非空值集；其余为 None）与分析期间的按对象读
#[derive(Default)]
pub(super) struct ObjParams {
    pub(super) sets: Vec<Option<Rc<[u32]>>>,
    pub(super) queries: RefCell<Vec<ObjQuery>>,
}

impl Ctx<'_> {
    /// 可按对象折叠的实例字段：非 static、非 VM 注入字面量、无字节码外的写入来源（反序列化除外，见模块注释）
    pub(super) fn obj_foldable(&self, f: &MemberRef) -> Option<Rc<FieldInfo>> {
        let fi = self.field_info(f)?;
        let open = fi.open || self.field_offset_under(&fi, self.fopen_all.get(), false) || self.hw_written(&fi);
        (fi.access & acc::STATIC == 0 && self.man.injected_literal(&fi.key.owner, &fi.key.name).is_none() && !open).then_some(fi)
    }

    /// 抽象对象集 objs 上实例字段 key 的值：初值 ⊔ 通配值 ⊔ 各对象值
    pub(super) fn obj_field_value(&self, key: &MemberRef, objs: &[u32]) -> Option<V> {
        let mut pv = default_pv(&key.desc);
        if let Some(w) = self.owild.borrow().get(key) {
            pv = PV::join(Some(&pv), w);
        }
        let ov = self.ovals.borrow();
        for &o in objs {
            if let Some(x) = ov.get(&(o, key.clone())) {
                pv = PV::join(Some(&pv), x);
            }
        }
        pv.value()
    }
}

impl Facts<'_, '_> {
    /// 按对象的字段读：接收者只来自形参 i。形参无对象集时同样登记查询（答复 None），共享判定据此比对（见 `share.rs`）
    pub(super) fn obj_field(&self, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        let m = self.m?;
        let [crate::absint::Src::Param(i)] = &*recv?.srcs() else { return None };
        let fi = self.ctx.obj_foldable(f)?;
        // 全局读者登记：开放判定变化（只增不减）令其失效
        self.ctx.dep(m, Dep::Field(fi.key.clone()));
        let r = self.objs.sets.get(*i as usize).and_then(Option::as_ref).and_then(|objs| self.ctx.obj_field_value(&fi.key, objs));
        self.objs.queries.borrow_mut().push((*i, fi.key.clone(), r.clone()));
        r
    }
}

impl Engine<'_> {
    /// 方法 m 各形参的抽象对象集（只看声明类型为容器形态类的引用形参）
    pub(super) fn param_obj_sets(&mut self, m: usize) -> Vec<Option<Rc<[u32]>>> {
        let pts = self.methods[m].ptypes.clone();
        let mut out = Vec::with_capacity(pts.len());
        for (i, pt) in pts.iter().enumerate() {
            let set = match pt {
                Some(t) if !self.arrays.contains_key(t) && !self.names[*t as usize].starts_with('[') => {
                    let name = self.names[*t as usize].clone();
                    if self.container(&name) {
                        let s = self.set_of(Node::P(m, i as u16));
                        let all_objs = s.open.is_empty() && !s.classes.is_empty() && s.classes.iter().all(|x| self.objs.contains_key(&x) && !self.mirrors.contains_key(&x));
                        all_objs.then(|| s.classes.iter().collect::<Vec<u32>>().into())
                    } else {
                        None
                    }
                }
                _ => None,
            };
            out.push(set);
        }
        out
    }

    /// 字节码写站点：值并入接收者值集里各抽象对象的字段值；其余接收者并入 `owild`
    pub(super) fn obj_field_put(&mut self, key: &MemberRef, objs: &[u32], other: bool, v: &PV) {
        // 只复核读过变化对象的方法（按对象登记于 `odeps`）；通配值变化复核 `owdeps` 登记的全部读者
        let mut readers: BTreeSet<usize> = BTreeSet::new();
        for &o in objs {
            let k = (o, key.clone());
            let changed = {
                let mut ov = self.ctx.ovals.borrow_mut();
                // 读取总并入初值：从初值起算，写入初值不算变化
                let cur = ov.get(&k).cloned().unwrap_or_else(|| default_pv(&key.desc));
                let new = PV::join(Some(&cur), v);
                let changed = cur != new;
                if changed {
                    ov.insert(k.clone(), new);
                }
                changed
            };
            if changed {
                readers.extend(self.ctx.odeps.borrow().get(&k).into_iter().flatten().copied());
            }
        }
        self.obj_readers_recheck(readers);
        if other {
            self.wild_put(key, v);
        }
    }

    /// 接收者不按对象分开的写入（物化快照等）：并入 `owild`
    pub(super) fn wild_put(&mut self, key: &MemberRef, v: &PV) {
        if self.wild_join(key, v) {
            let deps = self.ctx.owdeps.borrow().get(key).cloned().unwrap_or_default();
            self.obj_readers_recheck(deps);
        }
    }

    fn wild_join(&mut self, key: &MemberRef, v: &PV) -> bool {
        let mut w = self.ctx.owild.borrow_mut();
        let cur = w.get(key).cloned().unwrap_or_else(|| default_pv(&key.desc));
        let new = PV::join(Some(&cur), v);
        if cur == new {
            return false;
        }
        w.insert(key.clone(), new);
        true
    }

    /// 方法 m 的按对象读（按 m 自己的形参对象集）：登记逐对象读者，形参值集增长时重分析
    pub(super) fn obj_queries_bind(&mut self, m: usize, sets: &[Option<Rc<[u32]>>], queries: &Rc<[ObjQuery]>) {
        if queries.is_empty() {
            self.obj_queries.remove(&m);
            return;
        }
        self.obj_queries.insert(m, queries.clone());
        for (i, key, _) in queries.iter() {
            // 无对象集的形参同样盯住：值集由空变为全抽象对象时答复可能变窄（复核见 `obj_grown`）
            self.obj_watch.entry(Node::P(m, *i)).or_default().insert(m);
            self.graph.mark_hooked(Node::P(m, *i));
            let Some(objs) = sets.get(*i as usize).and_then(Option::as_ref) else { continue };
            for &o in objs.iter() {
                self.ctx.odeps.borrow_mut().entry((o, key.clone())).or_default().insert(m);
            }
            self.ctx.owdeps.borrow_mut().entry(key.clone()).or_default().insert(m);
        }
    }

    /// 摘要的按对象读在形参对象集 sets 下答复不变（共享前提，见 `share.rs`）
    pub(super) fn obj_queries_same(&self, sets: &[Option<Rc<[u32]>>], queries: &[ObjQuery]) -> bool {
        queries.iter().all(|(i, key, r)| {
            let now = self.ctx.obj_foldable(key).and_then(|_| sets.get(*i as usize).and_then(Option::as_ref).and_then(|objs| self.ctx.obj_field_value(key, objs)));
            now == *r
        })
    }

    /// 按对象值变化：读者的按对象读按当前值复核，答复有变的才重分析（分析是 Oracle 答复的确定函数）
    fn obj_readers_recheck(&mut self, readers: BTreeSet<usize>) {
        for m in readers {
            let Some(qs) = self.obj_queries.get(&m).cloned() else { continue };
            let sets = self.param_obj_sets(m);
            if !self.obj_queries_same(&sets, &qs) {
                self.invalidate(m, Why::FieldPut);
            }
        }
    }

    /// 节点 n 的值集增长：按对象读过它的方法重分析
    pub(super) fn obj_grown(&mut self, n: Node) {
        let Some(ms) = self.obj_watch.remove(&n) else { return };
        for m in ms {
            // 按新对象集复核：答复都不变则只补登新对象的读者，否则重分析
            let Some(qs) = self.obj_queries.get(&m).cloned() else { continue };
            let sets = self.param_obj_sets(m);
            if self.obj_queries_same(&sets, &qs) {
                self.obj_queries_bind(m, &sets, &qs);
            } else {
                self.invalidate(m, Why::Mirror);
            }
        }
    }
}
