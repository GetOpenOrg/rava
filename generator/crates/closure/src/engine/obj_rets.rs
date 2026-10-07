//! 常量查询：按接收者对象的返回值（计划 c1d §30.9 的 P3）。
//!
//! 返回常量格 `rvals` 按成员键汇合全部方法节点（含按接收者克隆的节点）的返回值。本模块在它旁边按抽象对象分开记录：
//! - **归属**：方法节点 m（实例方法）每次分析的返回值并入 `nret[m]`，再并入接收者形参节点 `P(m,0)` 值集里每个
//!   抽象对象 o 的 `orvals[(o, 成员)]`；`P(m,0)` 增长时把 `nret[m]` 补归属到新对象（`oret_watch`）。
//!   具体求值结果的返回值不按对象归属，并入 `orwild[成员]`。
//! - **读取**：调用点的接收者只来自一个形参 / 站点、其值集全由抽象对象组成，且各对象都选得出有字节码的目标 t 时，
//!   答复「各对象的 orwild[t] ⊔ orvals[(o, t)]」之并；⊥（值集为空或都尚无归属）与按成员的「尚无返回」同一口径
//!   （`noreturn.rs::answer_never`）：收尾阶段之前按不返回；收尾阶段只在某个目标 t 仍可按不返回答复
//!   （尚无节点 / 未分析 / 等待中）时按不返回，否则退回 `rvals`。按对象答复因此不比按成员答复更早放弃「不返回」，
//!   乐观不返回的定论（`settled`）不受按对象查询影响。
//! - **依赖**：查询记成 `ObjQuery::Ret`，与按对象字段读同一套复核框架（`obj_fields.rs`）：
//!   `ordeps` / `orwdeps` 登记读者，归属变化时按答复复核（原因 = 方法键，复核同名同描述符的调用点），来源值集增长经
//!   `obj_watch` 复核，共享摘要按答复比对。目标选择按（调用点, 对象）缓存（`oret_sel`）。
//!
//! 健全性：接收者为 o 的调用执行目标 t 时，派发把 o 送进所连目标节点的 `P(·,0)`，该节点的分析覆盖这次执行
//! （入口状态含本调用点的实参），其返回值归属到 o。所以 `orvals[(o, t)]` 覆盖接收者为 o 的全部返回。
//! 节点按旧对象集得出的返回值先归属到新对象、随后因答复变化重分析时，新返回值继续并入（只增不减），终态健全。

use super::obj_fields::{ObjAns, ObjCause, ObjQuery, ObjSets, Recv};
use super::*;
use crate::engine::sets::TypeSet;

/// 按对象取返回值的调用点：(调用指令, 符号引用, 接口调用)
#[derive(Clone, Debug, PartialEq)]
pub(super) struct RetSite {
    pub(super) opcode: u8,
    pub(super) m: MemberRef,
    pub(super) iface: bool,
}

/// 按对象查询的答复：各对象所执行方法的返回值之并，或 ⊥ 时的不返回
pub(super) type RetAns = ObjAns<PV>;

impl Ctx<'_> {
    /// 接收者为抽象对象 o 时调用点 s 执行的方法：精确目标，或按 o 的类选择（JVMS §5.4.6）；
    /// 只取有字节码的方法（其余的返回不按对象归属）
    pub(super) fn obj_ret_key(&self, s: &RetSite, o: u32) -> Option<Rc<MemberRef>> {
        let sk = (s.opcode, s.iface, s.m.clone());
        if let Some(t) = self.oret_sel.borrow().get(&sk).and_then(|c| c.get(&o)) {
            return t.clone();
        }
        let t = self.obj_ret_select(s, o).map(Rc::new);
        self.oret_sel.borrow_mut().entry(sk).or_default().insert(o, t.clone());
        t
    }

    fn obj_ret_select(&self, s: &RetSite, o: u32) -> Option<MemberRef> {
        let t = match self.exact_target(s.opcode, &s.m, s.iface) {
            Some((_, t)) => t,
            None => {
                let site = self.h.resolve_method(&s.m.owner, &s.m.name, &s.m.desc, s.iface)?;
                let c = self.oclass.borrow().get(&o)?.clone();
                let (owner, name, desc) = self.h.select(&c, &site)?.key();
                MemberRef { owner, name, desc }
            }
        };
        let cf = self.h.class(&t.owner)?;
        let mm = cf.method(&t.name, &t.desc)?;
        (mm.code.is_some() && self.kind_of(&cf, mm) == Kind::Bytecode).then_some(t)
    }

    /// 接收者来源 rc 上调用点 s 的答复：各对象所执行方法 t 的（通配值 ⊔ 对象值）之并；
    /// 有对象选不出字节码目标 → None（退回按成员的返回常量）；⊥（对象集为空，或各对象尚无归属）见模块注释
    pub(super) fn obj_ret_answer(&self, sets: &ObjSets, rc: Recv, s: &RetSite) -> Option<RetAns> {
        let objs = sets.get(rc)?;
        let mut pv: Option<PV> = None;
        let mut ts: Vec<Rc<MemberRef>> = Vec::new();
        let orvals = self.orvals.borrow();
        let orwild = self.orwild.borrow();
        for &o in objs.iter() {
            let t = self.obj_ret_key(s, o)?;
            if let Some(x) = orvals.get(&*t).and_then(|ov| ov.get(&o)) {
                pv = Some(PV::join_ret(pv.as_ref(), x));
            }
            // 通配值按目标只并一次
            if !ts.iter().any(|u| Rc::ptr_eq(u, &t) || **u == *t) {
                if let Some(w) = orwild.get(&*t) {
                    pv = Some(PV::join_ret(pv.as_ref(), w));
                }
                ts.push(t);
            }
        }
        match pv {
            Some(p) => Some(ObjAns::Value(p)),
            None => {
                let nr = self.noreturn.borrow();
                (!nr.closing() || ts.iter().any(|t| nr.answer_never(t))).then_some(ObjAns::Never)
            }
        }
    }
}

impl Facts<'_, '_> {
    /// 按接收者对象的返回值：接收者只来自形参 i 的实例调用。
    /// 形参无对象集时同样登记查询（答复 None），共享判定据此比对
    pub(super) fn obj_ret(&self, opcode: u8, t: &MemberRef, iface: bool, args: &[V]) -> Option<RetAns> {
        self.m?;
        if opcode == classfile::op::INVOKESTATIC || t.desc.ends_with(")V") {
            return None;
        }
        let rc = Recv::of(args.first())?;
        let s = Rc::new(RetSite { opcode, m: t.clone(), iface });
        let r = self.ctx.obj_ret_answer(&self.objs.sets, rc, &s);
        self.objs.queries.borrow_mut().push(ObjQuery::Ret(rc, s, r.clone()));
        if r.is_some() {
            self.ctx.stats.borrow_mut().oret_hits += 1;
        }
        r
    }
}

impl Engine<'_> {
    /// 方法节点 m 一次分析的返回值 r：并入 `nret[m]`，变化时归属到接收者值集里的抽象对象
    pub(super) fn obj_ret_note(&mut self, m: usize, r: &PV) {
        if self.methods[m].is_static {
            return;
        }
        let cur = self.nret.get(&m).cloned();
        let new = PV::join_ret(cur.as_ref(), r);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.nret.insert(m, new.clone());
        let n = Node::P(m, 0);
        if self.oret_watch.insert(n) {
            self.graph.mark_hooked(n);
        }
        let objs: Vec<u32> = self.set_of(n).classes.iter().filter(|x| self.objs.contains_key(x)).collect();
        self.oret_attr(m, &objs, &new);
    }

    /// 接收者形参节点增长：节点返回值补归属到新对象
    pub(super) fn oret_grown(&mut self, n: Node, delta: &TypeSet) {
        let Node::P(m, 0) = n else { return };
        let Some(v) = self.nret.get(&m).cloned() else { return };
        let objs: Vec<u32> = delta.classes.iter().filter(|x| self.objs.contains_key(x)).collect();
        self.oret_attr(m, &objs, &v);
    }

    fn oret_attr(&mut self, m: usize, objs: &[u32], v: &PV) {
        let key = self.methods[m].key.clone();
        let mut readers: BTreeSet<usize> = BTreeSet::new();
        if !objs.is_empty() {
            let mut orvals = self.ctx.orvals.borrow_mut();
            let ov = orvals.entry(key.clone()).or_default();
            let deps = self.ctx.ordeps.borrow();
            let deps = deps.get(&key);
            for &o in objs {
                let cur = ov.get(&o);
                let new = PV::join_ret(cur, v);
                if cur != Some(&new) {
                    ov.insert(o, new);
                    readers.extend(deps.and_then(|d| d.get(&o)).into_iter().flatten().copied());
                }
            }
        }
        self.obj_readers_recheck(readers, ObjCause::Ret(&key));
    }

    /// 不按对象归属的返回值（具体求值结果）：并入 `orwild`，变化时复核全部按对象读者
    pub(super) fn oret_wild(&mut self, key: &MemberRef, v: &PV) {
        let changed = {
            let mut w = self.ctx.orwild.borrow_mut();
            let cur = w.get(key).cloned();
            let new = PV::join_ret(cur.as_ref(), v);
            let changed = cur.as_ref() != Some(&new);
            if changed {
                w.insert(key.clone(), new);
            }
            changed
        };
        if changed {
            let deps = self.ctx.orwdeps.borrow().get(key).cloned().unwrap_or_default();
            self.obj_readers_recheck(deps, ObjCause::Ret(key));
        }
    }
}
