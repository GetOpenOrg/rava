//! 引擎：方法摘要按入口抽象状态共享（上下文共享，计划 2026-09-30-closure-analyzer-performance.md P3）。
//!
//! 克隆上下文（`MNode.ctx`）按调用点 / 接收者对象区分节点身份，但方法体的抽象解释只取决于：
//! 方法本身（字节码）、入口形参常量（`pvals`）、Class 形参的类镜像集（`param_mirror_sets`），以及
//! 经 `Facts` 读到的全局事实。全局事实的每次读取都按被分析上下文登记依赖（字段读者 `fdeps`、
//! 返回常量读者 `rdeps`、「尚无返回」答复 `never`、系统属性读者 `pdeps`），事实变化即令登记者失效。
//!
//! 因此：入口状态相同、且摘要仍被某个上下文有效持有（依赖都未触发）时，新上下文的分析结果必与之相同。
//! 此时直接复用摘要（`Rc` 共享），并把分析期间登记的依赖原样重放到新上下文——之后任何一条依赖触发，
//! 全部持有者一并失效，与各自独立分析时的失效集合相同。
//!
//! 收尾阶段（`NoReturn::closing`）的「尚无返回」答复随被调方法的建节点 / 分析进度变化而不经失效，
//! 答复有时效性：收尾阶段既不复用也不登记。

use std::rc::Weak;

use super::*;

/// 分析期间登记的一条依赖（按被分析上下文）
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum Dep {
    /// 字段读者
    Field(MemberRef),
    /// 返回常量读者
    Ret(MemberRef),
    /// 按「尚无返回」答复过
    Never,
    /// 系统属性 / 标签对象读者
    Props,
    /// 记忆条目（编号见 `memo.rs`）的取用者
    Memo(u32),
}

/// 一份可共享的摘要
pub(super) struct Shared {
    params: Vec<Option<V>>,
    mirrors: Vec<Option<BTreeSet<Rc<str>>>>,
    a: Weak<Analysis>,
    /// 装入过该摘要的上下文（其中仍持有者使之有效）
    holders: Vec<usize>,
    deps: Rc<[Dep]>,
}

impl Ctx<'_> {
    /// 上下文 m 登记依赖 d；分析进行中（`dep_log` 打开）同时记入日志
    pub(super) fn dep(&self, m: usize, d: Dep) {
        match &d {
            // 已登记的键不再克隆（逐调用点 / 逐字段读都会走到这里）
            Dep::Field(k) => {
                let mut ds = self.fdeps.borrow_mut();
                match ds.get_mut(k) {
                    Some(s) => {
                        s.insert(m);
                    }
                    None => {
                        ds.entry(k.clone()).or_default().insert(m);
                    }
                }
            }
            Dep::Ret(t) => {
                let mut ds = self.rdeps.borrow_mut();
                match ds.get_mut(t) {
                    Some(s) => {
                        s.insert(m);
                    }
                    None => {
                        ds.entry(t.clone()).or_default().insert(m);
                    }
                }
            }
            Dep::Never => {
                self.never.borrow_mut().insert(m);
            }
            Dep::Props => {
                self.pdeps.borrow_mut().insert(m);
            }
            Dep::Memo(i) => {
                self.mdeps.borrow_mut().entry(*i).or_default().insert(m);
            }
        }
        if let Some(log) = self.dep_log.borrow_mut().as_mut() {
            log.push(d);
        }
    }
}

impl<'a> Engine<'a> {
    /// 可复用的摘要：同一方法、入口状态相同、仍有上下文有效持有。顺带清掉失效的记录与持有者
    pub(super) fn shared_analysis(
        &mut self,
        key: &MemberRef,
        params: &[Option<V>],
        mirrors: &[Option<BTreeSet<Rc<str>>>],
    ) -> Option<(Rc<Analysis>, Rc<[Dep]>)> {
        let methods = &self.methods;
        let es = self.shared.get_mut(key)?;
        let held = |h: usize, p: *const Analysis| methods[h].analysis.as_ref().is_some_and(|x| Rc::as_ptr(x) == p);
        for e in es.iter_mut() {
            let p = e.a.as_ptr();
            e.holders.retain(|&h| held(h, p));
        }
        es.retain(|e| !e.holders.is_empty());
        let e = es.iter().find(|e| e.params == params && e.mirrors == mirrors)?;
        Some((e.a.upgrade()?, e.deps.clone()))
    }

    /// 新上下文装入共享摘要：重放依赖、记为持有者
    pub(super) fn share_join(&mut self, m: usize, a: &Rc<Analysis>, deps: &[Dep]) {
        for d in deps {
            self.ctx.dep(m, d.clone());
        }
        let key = &self.methods[m].key;
        if let Some(e) = self.shared.get_mut(key).and_then(|es| es.iter_mut().find(|e| e.a.as_ptr() == Rc::as_ptr(a))) {
            e.holders.push(m);
        }
        self.ctx.stats.borrow_mut().shared += 1;
    }

    /// 登记新算出的摘要（依赖日志去重）
    pub(super) fn share_record(
        &mut self,
        m: usize,
        params: Vec<Option<V>>,
        mirrors: Vec<Option<BTreeSet<Rc<str>>>>,
        a: &Rc<Analysis>,
        mut deps: Vec<Dep>,
    ) {
        deps.sort_unstable();
        deps.dedup();
        let key = self.methods[m].key.clone();
        let e = Shared { params, mirrors, a: Rc::downgrade(a), holders: vec![m], deps: deps.into() };
        self.shared.entry(key).or_default().push(e);
    }
}
