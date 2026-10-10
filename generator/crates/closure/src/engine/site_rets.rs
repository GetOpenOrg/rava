//! 引擎：静态调用点按克隆节点取返回值。
//!
//! 静态调用的目标按上下文克隆（转发方法的调用点克隆、工厂方法的调用点克隆、堆上下文继承，见 `ctxsel.rs`）时，
//! 调用点的结果取所接克隆节点各次分析返回值之并（`sret`），不再取按成员汇合的返回常量格：后者在全部调用点上
//! 汇合，同一转发方法（`doPrivileged` 之类）在不同调用点返回不同常量时只剩 Top。
//!
//! - **答复表**：分析方法 m 之前按调用点算好（`site_table`），随入口状态参与摘要共享的比对（`share.rs`）。
//!   调用点已接边时取所接节点；尚未接边时按与实参无关的克隆判定预判（选择子形参克隆依赖实参值，按可能克隆预判）。
//!   目标节点不克隆（`NOCTX`）的调用点不进表，仍走按成员的返回常量格。
//! - **乐观**：克隆节点尚无返回值（未建节点 / 未分析 / 尚未返回）时按「尚无返回」答复（调用之后不可达），
//!   与按成员的「尚无返回」同一套收尾（`noreturn.rs`）：收尾阶段已分析且无返回路径的节点改走按成员的格。
//! - **失效**：节点返回值变化时读者重分析；调用点所接节点与答复表里的不同时（首次接边 / 档位改接）调用方重分析。
//!
//! 正确性：节点返回值是该节点全部分析返回值之并，节点的形参常量是其全部调用点实参之并，故本调用点的实际返回值
//! 必在其中；乐观答复只在「尚无返回」时给出，返回值到达即失效重算，与按成员格的乐观口径相同。

use super::*;

/// 答复表中一个调用点的答复
#[derive(Clone, Debug, PartialEq)]
pub(super) enum SiteAns {
    /// 克隆节点尚无返回值
    Never,
    /// 克隆节点返回值之并（常量）
    Val(V),
}

/// 一次分析所用的答复表（偏移升序）
pub(super) type SiteTable = Rc<[(u32, SiteAns)]>;

impl Engine<'_> {
    /// 方法 m 的静态调用点答复表，并登记 m 为所接克隆节点返回值的读者
    pub(super) fn site_table(&mut self, m: usize, code: &classfile::Code) -> SiteTable {
        if self.methods[m].kind != Kind::Bytecode {
            return Rc::from([]);
        }
        let mut out = Vec::new();
        for ins in &code.insns {
            if ins.opcode != classfile::op::INVOKESTATIC {
                continue;
            }
            let classfile::Operand::Method(mref, iface) = &ins.operand else { continue };
            if let Some(a) = self.site_answer(m, ins.offset, mref, *iface) {
                out.push((ins.offset, a));
            }
        }
        let t = Rc::from(out);
        self.site_used.insert(m, Rc::clone(&t));
        t
    }

    /// 调用点 (m, off) 的答复；None = 不克隆（走按成员的返回常量格）
    fn site_answer(&mut self, m: usize, off: u32, mref: &MemberRef, iface: bool) -> Option<SiteAns> {
        let node = match self.site_nodes.get(&(m, off)) {
            Some(&t) => Some(t),
            None => {
                self.site_predict(m, mref, iface)?;
                None
            }
        };
        if let Some(t) = node {
            if self.methods[t].ctx == NOCTX {
                return None;
            }
            self.sret_readers.entry(t).or_default().insert(m);
            if let Some(PV::Const(v)) = self.sret.get(&t) {
                return Some(SiteAns::Val(v.clone()));
            }
            if self.sret.contains_key(&t) {
                return None;
            }
        }
        // 尚无返回值：乐观阶段按「尚无返回」；收尾阶段只对未建 / 未分析 / 等待中的节点
        let nr = self.ctx.noreturn.borrow();
        if !nr.closing() {
            return Some(SiteAns::Never);
        }
        if nr.settled() {
            return None;
        }
        let pending = node.is_none_or(|t| self.methods[t].aseq == 0 || nr.answer_never(&self.methods[t].key));
        pending.then_some(SiteAns::Never)
    }

    /// 尚未接边的调用点：与实参无关的克隆判定（`ctxsel.rs::static_ctx`）；Some = 会克隆或可能克隆。
    /// 有选择子形参的目标是否克隆取决于调用点字面常量与调用方上下文（`selector.rs`），此处不细分、按可能克隆预判
    /// （乐观答复「尚无返回」）：接边后所接节点不克隆时
    /// `site_linked` 比对答复不同、调用方重分析改取按成员的格。若按不克隆预判，首次分析取到按成员汇合的 Top，
    /// 并入克隆节点只增不减的返回值之后不再收回
    fn site_predict(&mut self, m: usize, mref: &MemberRef, iface: bool) -> Option<()> {
        let site = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, iface)?;
        let (o, n, d) = site.key();
        let key = MemberRef { owner: o, name: n, desc: d };
        if self.kind_of(&site.class, site.method()) != Kind::Bytecode {
            return None;
        }
        if self.ctx.selector_slots(&key) != 0 {
            return Some(());
        }
        if self.man.concrete.entries.contains(&*self.mref_key(&key)) {
            return None;
        }
        let md = parse_method(&key.desc)?;
        let heap = md.ret.iter().chain(&md.params).any(|r| r.is_reference());
        let caller = self.methods[m].ctx;
        let clone = match caller {
            _ if !heap => false,
            NOCTX => self.fresh_factory(&key),
            _ => true,
        };
        (clone || self.forwarder(&key)).then_some(())
    }

    /// 调用点 (m, off) 接到节点 t：与 m 上次分析所用答复不同时 m 重分析
    pub(super) fn site_linked(&mut self, m: usize, off: u32, t: usize) {
        if self.site_nodes.insert((m, off), t) == Some(t) {
            return;
        }
        let Some(used) = self.site_used.get(&m).cloned() else { return };
        let Some(cf) = self.h.class(&self.methods[m].key.owner) else { return };
        let Some(code) = cf.method(&self.methods[m].key.name, &self.methods[m].key.desc).and_then(|x| x.code.as_ref()) else { return };
        let Some(ins) = code.insns.iter().find(|x| x.offset == off) else { return };
        let classfile::Operand::Method(mref, iface) = &ins.operand else { return };
        let (mref, iface) = (mref.clone(), *iface);
        let now = self.site_answer(m, off, &mref, iface);
        let was = used.iter().find(|(o, _)| *o == off).map(|(_, a)| a.clone());
        if now != was {
            self.invalidate(m, Why::RetConst);
        }
    }

    /// 克隆的静态方法节点一次分析的返回值 r：并入 `sret[m]`，变化时读者重分析
    pub(super) fn site_ret_note(&mut self, m: usize, r: &PV) {
        if !self.methods[m].is_static || self.methods[m].ctx == NOCTX {
            return;
        }
        let cur = self.sret.get(&m).cloned();
        let new = PV::join(cur.as_ref(), r);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.sret.insert(m, new);
        let readers = self.sret_readers.get(&m).cloned();
        self.invalidate_all(readers, Why::RetConst);
    }
}
