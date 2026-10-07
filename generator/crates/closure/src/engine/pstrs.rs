//! 引擎：形参字符串常量集——按名查找的名字来自本方法形参时，取流到该形参的全部字符串常量。
//!
//! 单调且与处理顺序无关：
//! - 集合只并不减，不走形参常量（`pvals`）的汇合格。汇合格先到的常量会被后到的调用点抬为 Top，
//!   若按汇合结果取常量，结果就取决于调用点接入的先后；
//! - 实参是字符串常量或含字面量来源的合流值：各字面量并入被调形参槽；实参来自调用方形参（透传，含合流）：登记子集边「调用方形参槽 → 被调形参槽」，
//!   常量集沿边传递（调用方形参上已有与日后新增的常量都会到达）；
//! - 派发枢纽同样有形参槽：调用点实参并入枢纽槽，枢纽槽流向父枢纽槽与各目标的形参槽；
//! - 方法形参槽增长时，读过该槽的按名查找站点入站点队列重跑（不在接边中途重入）。
//!
//! 按名取类另需形参上的**全部**名字（不止字面量）：
//! - String 形参槽另记非常量实参的调用点（调用方, 偏移, 实参序号），求值时在调用方帧里按拼接段求出名字；
//! - 有实参值未知的调用边（非字节码调用方、无调用点记录即被分析的入口、实参未知的枢纽接入）的方法 / 枢纽，
//!   其形参槽推不出（与 `pvals` 的 Top 同口径，恒不撤）；
//! - 读者沿子集边逆向遍历全部上游槽：任一推不出即整体推不出；字面量取起点槽（子集边已传递）；
//!   上游槽变化（新常量、新流入边、新非常量实参、推不出）、非常量实参所在调用方重分析时读者重跑；
//! - 递归传参成环（槽在求值栈上）或上游槽过多时推不出。
//!
//! String 字段同样有槽（`PSlot::F`，按字段、不分接收者）：字节码写入的字面量并入、写入本方法形参时登记子集边
//! 「形参槽 → 字段槽」，其余写入（拼接、调用结果等）使字段槽推不出；字段可经字节码外途径写入（`field_open`）时
//! 读者不取槽。读取 String 字段的名字段由此取得全部写入名字（如按类型名查找服务时，类型名存于列表对象的字段）。

use super::class_lookup::{event_at, expand, is_invoke, Gap, Part, MAX_NAMES};
use super::name_eval::Frame;
use super::sealed::{flatten, is_field};
use super::*;

/// 字符串常量集的槽：方法形参（方法，形参槽）/ 枢纽形参（枢纽，形参序号，不含接收者）/ String 字段（字段节点序号）
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(super) enum PSlot {
    M(usize, usize),
    H(u32, usize),
    F(usize),
}

#[derive(Default)]
pub(super) struct PStrs {
    /// 字面量序号集（`absint::lit`；传递只做位运算）
    sets: HashMap<PSlot, IdSet>,
    succ: HashMap<PSlot, BTreeSet<PSlot>>,
    /// 读过方法形参槽的按名查找站点（偏移）
    sites: HashMap<(usize, usize), BTreeSet<u32>>,
    /// 子集边的逆：槽 → 流入它的槽
    pred: HashMap<PSlot, BTreeSet<PSlot>>,
    /// 流入 String 形参槽的非常量实参：(调用方, 调用偏移, 实参序号（不含接收者）)
    inputs: HashMap<PSlot, BTreeSet<(usize, u32, usize)>>,
    /// 有实参值未知的调用边的方法 / 枢纽：形参槽推不出
    top_m: HashSet<usize>,
    top_h: HashSet<u32>,
    /// 经字节码调用点以外的入口接入的方法（枢纽、引导阶段、反射调用、无调用点记录）：字段配对看不到这些入口
    offsite: HashSet<usize>,
    /// 有非常量写入的 String 字段：字段槽推不出
    top_f: HashSet<usize>,
    /// 读过槽（按名取类遍历到的上游槽）的站点
    demand: HashMap<PSlot, BTreeSet<(usize, u32)>>,
    /// 求值读过其调用点实参的调用方 → 读者站点
    xdemand: HashMap<usize, BTreeSet<(usize, u32)>>,
    /// 正在求值的起点槽
    active: Vec<PSlot>,
}

/// 读者遍历的上游槽数上限：超出按推不出处理
const MAX_SLOTS: usize = 256;
/// 形参名字求值的嵌套层数上限（调用方实参又来自其形参……）
const MAX_NEST: usize = 4;

/// 实参值是否不全由字面量与形参透传构成（需在调用方帧里求值）
fn computed(v: &V) -> bool {
    match v {
        V::Str(..) | V::Null => false,
        V::Ref { src, .. } => src.is_empty() || src.iter().any(|s| !matches!(s, Src::Param(_) | Src::Str(_))),
        _ => true,
    }
}

impl<'a> Engine<'a> {
    /// 方法 m 形参槽 i 上的字符串常量（按字符串序）；登记站点 (m, off) 为读者
    pub(super) fn pstr_read(&mut self, m: usize, i: usize, off: u32) -> Vec<Rc<str>> {
        self.pstr.sites.entry((m, i)).or_default().insert(off);
        let mut out: Vec<Rc<str>> = self.pstr.sets.get(&PSlot::M(m, i)).into_iter().flatten().map(crate::absint::lit_str).collect();
        out.sort_unstable();
        out
    }

    /// 读过方法形参槽的按名查找站点（形参槽被污染时重跑，engine/field_names.rs）
    pub(super) fn pstr_readers(&self, m: usize, i: usize) -> Vec<u32> {
        self.pstr.sites.get(&(m, i)).into_iter().flatten().copied().collect()
    }

    /// 方法形参槽 (m, i) 的子集边后继中的方法形参槽（污染沿边传播，engine/field_names.rs）
    pub(super) fn pstr_succ_methods(&self, m: usize, i: usize) -> Vec<(usize, usize)> {
        self.pstr
            .succ
            .get(&PSlot::M(m, i))
            .into_iter()
            .flatten()
            .filter_map(|s| match *s {
                PSlot::M(t, j) => Some((t, j)),
                PSlot::H(..) | PSlot::F(_) => None,
            })
            .collect()
    }

    /// 常量并入槽 at，沿子集边传递（只传新增部分）
    fn pstr_add(&mut self, at: PSlot, strs: IdSet) {
        let mut work = vec![(at, strs)];
        while let Some((s, xs)) = work.pop() {
            let set = self.pstr.sets.entry(s).or_default();
            let new = xs.minus(set);
            if new.is_empty() {
                continue;
            }
            set.union_with(&new);
            self.pstr_wake(s);
            if let PSlot::M(t, i) = s {
                let offs: Vec<u32> = self.pstr.sites.get(&(t, i)).into_iter().flatten().copied().collect();
                for off in offs {
                    self.push_site((t, off), site_prof::TRIG_PSTR, None);
                }
            }
            for &n in self.pstr.succ.get(&s).into_iter().flatten() {
                work.push((n, new.clone()));
            }
        }
    }

    /// 子集边 from → to：from 已有的常量立即传递
    pub(super) fn pstr_edge(&mut self, from: PSlot, to: PSlot) {
        if from == to || !self.pstr.succ.entry(from).or_default().insert(to) {
            return;
        }
        self.pstr.pred.entry(to).or_default().insert(from);
        self.pstr_wake(to);
        let xs = self.pstr.sets.get(&from).cloned().unwrap_or_default();
        if !xs.is_empty() {
            self.pstr_add(to, xs);
        }
    }

    /// 调用方 m 偏移 off 的调用点实参值 vals（不含接收者）流入槽 to(j)；string(j) = 槽 j 是 String 形参
    pub(super) fn pstr_site(&mut self, m: usize, off: u32, vals: &[V], to: impl Fn(usize) -> PSlot, string: impl Fn(usize) -> bool) {
        for (j, v) in vals.iter().enumerate() {
            if string(j) && computed(v) && self.pstr.inputs.entry(to(j)).or_default().insert((m, off, j)) {
                self.pstr_wake(to(j));
            }
            let lits = v.lit_ids();
            if !lits.is_empty() {
                self.pstr_add(to(j), lits.into_iter().collect());
            }
            if let V::Ref { src, .. } = v {
                for s in src.iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), to(j));
                    }
                }
            }
        }
    }

    /// 读过槽 s 的按名取类站点入站点队列重跑
    fn pstr_wake(&mut self, s: PSlot) {
        let Some(ws) = self.pstr.demand.get(&s) else { return };
        for w in ws.iter().copied().collect::<Vec<_>>() {
            self.push_site(w, site_prof::TRIG_PSTR_WAKE, None);
        }
    }

    /// 方法 t 有实参值未知的调用边（或无调用点记录即被分析）：形参槽推不出
    pub(super) fn pstr_top_m(&mut self, t: usize) {
        if self.pstr.top_m.insert(t) {
            for i in 0..self.methods[t].ptypes.len() {
                self.pstr_wake(PSlot::M(t, i));
            }
        }
    }

    /// 方法 t 经字节码调用点以外的入口接入：包装配对（`lookup_pair.rs`）只在字节码调用点上配对，这类方法形参上的
    /// 名字按形参常量集与污染口径取（engine/field_names.rs）；首次登记时重跑读过其形参槽的站点
    pub(super) fn pstr_offsite(&mut self, t: usize) {
        if self.pstr.offsite.insert(t) {
            for i in 0..self.methods[t].ptypes.len() {
                for off in self.pstr_readers(t, i) {
                    self.push_site((t, off), site_prof::TRIG_TAINT, None);
                }
            }
        }
    }

    pub(super) fn pstr_is_offsite(&self, t: usize) -> bool {
        self.pstr.offsite.contains(&t)
    }

    /// 枢纽 h 有实参值未知的调用点接入：形参槽推不出
    pub(super) fn pstr_top_h(&mut self, h: u32) {
        if self.pstr.top_h.insert(h) {
            for j in 0..self.hubs[h as usize].ptypes.len() {
                self.pstr_wake(PSlot::H(h, j));
            }
        }
    }

    /// 调用方 m 重分析：读过其调用点实参的读者站点重跑
    pub(super) fn pstr_reanalyzed(&mut self, m: usize) {
        let Some(ws) = self.pstr.xdemand.get(&m) else { return };
        for w in ws.iter().copied().collect::<Vec<_>>() {
            self.push_site(w, site_prof::TRIG_PSTR_REANALYZED, None);
        }
    }

    /// 方法 m 中字节码写入 String 字段（字段节点 fi）的值 v（None = 值未知）并入字段槽
    pub(super) fn pstr_field_put(&mut self, m: usize, fi: usize, v: Option<&V>) {
        let slot = PSlot::F(fi);
        match v {
            Some(V::Null) => {}
            Some(v) if !computed(v) => {
                let lits = v.lit_ids();
                if !lits.is_empty() {
                    self.pstr_add(slot, lits.into_iter().collect());
                }
                for s in v.srcs().iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), slot);
                    }
                }
            }
            _ => {
                if self.pstr.top_f.insert(fi) {
                    self.pstr_wake(slot);
                }
            }
        }
    }

    /// 方法 m 的 String 形参槽 i 上的名字与是否推得出（当前站点为读者）；None = 非 String 形参 / 不在站点内
    pub(super) fn param_names(&mut self, m: usize, i: usize, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let string = self.id(STRING);
        if self.methods[m].ptypes.get(i).copied().flatten() != Some(string) {
            return None;
        }
        self.slot_names(PSlot::M(m, i), depth)
    }

    /// String 字段（字段节点 fi）各字节码写入的名字与是否推得出（当前站点为读者）
    pub(super) fn field_names(&mut self, fi: usize, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        self.slot_names(PSlot::F(fi), depth)
    }

    /// 名字段读自 String 字段（方法 m 中偏移 o 的 getfield / getstatic）时该字段各字节码写入的全部名字。
    /// static final 字段由常量求值给出，不经字段槽；字段可经字节码外途径写入时推不出——仍给出字节码写入的名字，
    /// 只标记推不出（字段转为不折叠时 m 失效重分析）
    pub(super) fn read_field_names(&mut self, m: usize, a: &Analysis, o: u32, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let Some(Event::Field { opcode, mref, .. }) = event_at(a, o, is_field) else { return None };
        if !matches!(*opcode, classfile::op::GETFIELD | classfile::op::GETSTATIC) || mref.desc != format!("L{STRING};") {
            return None;
        }
        let fi = self.ctx.field_info(mref)?;
        if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
            return None;
        }
        self.ctx.dep(m, Dep::Field(fi.key.clone()));
        let open = self.ctx.field_open(&fi);
        let n = match self.fields.get_index_of(&fi.key) {
            Some(n) => n,
            None if open => return self.cur_site.map(|_| (BTreeSet::new(), false)),
            None => self.field_node(fi.key.clone()),
        };
        self.field_names(n, depth).map(|(names, complete)| (names, complete && !open))
    }

    /// 槽 start 上的全部名字：沿子集边逆向遍历上游槽，字面量取起点槽，非常量实参在调用方帧里求值
    fn slot_names(&mut self, start: PSlot, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let (reader, inputs, complete) = self.slot_upstream(start)?;
        let mut out: BTreeSet<Rc<str>> = self.pstr.sets.get(&start).into_iter().flatten().map(crate::absint::lit_str).collect();
        self.pstr.active.push(start);
        let evaluated = self.param_inputs(reader, &inputs, depth, &mut out);
        self.pstr.active.pop();
        Some((out, complete && evaluated))
    }

    /// 方法 m 的 String 形参槽 i 上的全部候选模式（推不出的段记为任意串，见 `class_lookup.rs::Gap::Class`；
    /// 当前站点为读者）；None = 槽推不出或模式超出上限
    pub(super) fn param_patterns(&mut self, m: usize, i: usize) -> Option<Vec<Vec<Part>>> {
        let string = self.id(STRING);
        if self.methods[m].ptypes.get(i).copied().flatten() != Some(string) {
            return None;
        }
        let start = PSlot::M(m, i);
        let (reader, inputs, complete) = self.slot_upstream(start)?;
        if !complete {
            return None;
        }
        let mut out: Vec<Vec<Part>> = self.pstr.sets.get(&start).into_iter().flatten().map(|l| vec![Part::Lit(crate::absint::lit_str(l))]).collect();
        self.pstr.active.push(start);
        let r = self.input_patterns(reader, &inputs, &mut out);
        self.pstr.active.pop();
        r.map(|_| out)
    }

    /// 槽 start 的读者登记与上游遍历：返回读者站点、全部上游槽的非常量实参与槽是否推得出（任一上游槽推不出、
    /// 成环或超限即推不出）。推不出只作标记、不截断遍历：名字是状态的单调函数——上游槽转为推不出后，
    /// 其更上游的字面量（已沿子集边到达起点槽）与非常量实参照样计入，结果只并不减，与求值先后无关。
    /// 当前不在站点内（无读者可登记）为 None
    fn slot_upstream(&mut self, start: PSlot) -> Option<((usize, u32), BTreeSet<(usize, u32, usize)>, bool)> {
        let reader = self.cur_site?;
        let mut inputs: BTreeSet<(usize, u32, usize)> = BTreeSet::new();
        if self.pstr.active.contains(&start) || self.pstr.active.len() >= MAX_NEST {
            if !self.pstr.active.contains(&start) {
                stats::cap_hit(stats::CAP_NEST);
            }
            return Some((reader, inputs, false));
        }
        let mut complete = true;
        let mut seen: HashSet<PSlot> = HashSet::default();
        let mut stack = vec![start];
        while let Some(s) = stack.pop() {
            if !seen.insert(s) {
                continue;
            }
            if seen.len() > MAX_SLOTS {
                stats::cap_hit(stats::CAP_SLOTS);
                // 超限：遍历到的子集取决于图的形状，非常量实参一概不取（起点槽字面量已含全部上游字面量）
                inputs.clear();
                complete = false;
                break;
            }
            self.pstr.demand.entry(s).or_default().insert(reader);
            let top = match s {
                PSlot::M(t, _) => self.pstr.top_m.contains(&t),
                PSlot::H(h, _) => self.pstr.top_h.contains(&h),
                PSlot::F(fi) => self.pstr.top_f.contains(&fi),
            };
            complete &= !top;
            inputs.extend(self.pstr.inputs.get(&s).into_iter().flatten().copied());
            stack.extend(self.pstr.pred.get(&s).into_iter().flatten().copied());
        }
        Some((reader, inputs, complete))
    }

    /// 非常量实参所在调用点的实参值与调用方帧数据；调用方待重分析 / 调用点已不可达为 Ok(None)，保守分析为 Err
    fn input_value(&mut self, reader: (usize, u32), (cm, off, j): (usize, u32, usize)) -> Result<Option<(Rc<Analysis>, V)>, ()> {
        self.pstr.xdemand.entry(cm).or_default().insert(reader);
        let Some(ca) = self.methods[cm].analysis.clone() else { return Ok(None) };
        if ca.conservative {
            return Err(());
        }
        let Some(Event::Invoke { opcode, args, .. }) = event_at(&ca, off, is_invoke) else { return Ok(None) };
        let v = args.get(usize::from(*opcode != classfile::op::INVOKESTATIC) + j).ok_or(())?.clone();
        Ok(Some((ca, v)))
    }

    /// 非常量实参在各自调用方帧里按 `Gap::Class` 拆段、展开成候选模式并入 out
    fn input_patterns(&mut self, reader: (usize, u32), inputs: &BTreeSet<(usize, u32, usize)>, out: &mut Vec<Vec<Part>>) -> Option<()> {
        for &inp in inputs {
            let Some((ca, v)) = self.input_value(reader, inp).ok()? else { continue };
            let owner = self.methods[inp.0].key.owner.clone();
            let f = Frame { m: Some(inp.0), a: &ca, owner: &owner, up: None };
            let parts = self.name_parts(&f, &v, Gap::Class, 0)?;
            out.extend(expand(&parts)?);
            if out.len() > MAX_NAMES {
                stats::cap_hit(stats::CAP_PNAMES);
                return None;
            }
        }
        Some(())
    }

    /// 非常量实参在各自调用方帧里求出的名字并入 out；返回是否全部求出（求不出的实参只作标记，其余照常并入）
    fn param_inputs(&mut self, reader: (usize, u32), inputs: &BTreeSet<(usize, u32, usize)>, depth: u8, out: &mut BTreeSet<Rc<str>>) -> bool {
        let mut complete = true;
        for &(cm, off, j) in inputs {
            // 调用方正待重分析（重分析后重跑）/ 调用点在当前分析里已不可达（不再流入）：跳过
            let (ca, v) = match self.input_value(reader, (cm, off, j)) {
                Ok(Some(x)) => x,
                Ok(None) => continue,
                Err(()) => {
                    complete = false;
                    continue;
                }
            };
            let owner = self.methods[cm].key.owner.clone();
            let f = Frame { m: Some(cm), a: &ca, owner: &owner, up: None };
            // 内部求值（Gap::Fail）的推不出只记在引擎级标志上：在这里收进本槽的「是否推得出」，不外泄给外层求值——
            // 外层按自己的 gap 处理推不出的槽（按名取类为「已知名字 | 任意串」，按生成范围内的类名匹配），
            // 而不是因上游某个调用方推不出把整个站点记为推不出
            let saved = (std::mem::take(&mut self.lookup_incomplete), std::mem::take(&mut self.lookup_partial));
            let r = self.name_parts(&f, &v, Gap::Fail, depth).as_deref().and_then(flatten);
            let inner = std::mem::replace(&mut self.lookup_incomplete, saved.0) | std::mem::replace(&mut self.lookup_partial, saved.1);
            match r {
                Some(names) => {
                    out.extend(names);
                    complete &= !inner;
                }
                None => complete = false,
            }
        }
        if out.len() > MAX_NAMES {
            stats::cap_hit(stats::CAP_PNAMES);
            return false;
        }
        complete
    }
}
