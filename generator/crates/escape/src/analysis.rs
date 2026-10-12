//! 区域逃逸分析：全局不动点、抽象堆、调用与分派（无 GC 文档 §4.5）。
//!
//! 一个候选区域一次分析：区域代码以「区域前已有」的局部变量起步，沿调用链分析可达方法（上下文不敏感：
//! 形参 = 全部调用点实参之并，返回 = 全部 return 之并），反复全量重算直到抽象堆、形参、返回都不再变化。
//! 任何把区域对象交给区域外（旧对象字段 / 静态字段 / 不透明代码）的路径都拒绝该区域。

use std::collections::HashMap;

use classfile::op;
use classfile::{Insn, Method, Operand};
use input::EscapeEffect;
use ty::registry::ClassInfo;

use crate::val::{Frame, NodeId, Prim, Refs, Val, FRESH_UNKNOWN, OLD};
use crate::world::{method_kinds, Callee, Kind, LambdaSite, World};

/// 拒绝原因（`RAVA_ESCAPE_TRACE` 输出）
#[derive(Debug)]
pub struct Reject(pub String);

pub type R<T> = Result<T, Reject>;

pub fn reject<T>(why: String) -> R<T> {
    Err(Reject(why))
}

/// 抽象对象
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeKind {
    Old,
    FreshUnknown,
    /// 区域内经 `new` / 数组分配指令创建（字段全由分析可见的代码写入）
    Alloc(String),
    /// 区域内由运行时创建（拼接结果等）：按该类分派，字段内容未知
    Made(String),
    Lambda(LambdaSite),
    /// 登记了 `fresh` 效果的 native 新建的对象（不登记进区域、初始内容为空、运行时类未知）：键为被调方法
    Native(String),
}

struct Summary<'a> {
    ci: &'a ClassInfo,
    idx: usize,
    params: Vec<Val>,
    /// None = 尚未到达任何 return
    ret: Option<Val>,
    /// 首次请求该摘要的摘要（None = 区域代码）：拒绝时给出调用链
    parent: Option<usize>,
    /// 某个调用点的异常可能在区域动态范围内被捕获（调用点受处理器覆盖，或调用方本身如此）
    catchable: bool,
    /// 最近一次被请求 / 执行的轮次
    req: u32,
    ran: u32,
}

pub struct Analysis<'w, 'a> {
    pub(crate) w: &'w World<'a>,
    pub(crate) nodes: Vec<NodeKind>,
    ix: HashMap<NodeKind, NodeId>,
    /// 节点的未知内容（运行时创建 / 未知区域对象：任一字段可能指向旧对象或未知区域对象）
    base: Vec<Refs>,
    /// 字段敏感的内容：节点 → 字段槽 → 写入值之并
    fields: Vec<HashMap<Slot, Refs>>,
    slots: HashMap<String, Slot>,
    /// int 类字段常量：(字段名, 描述符) → 区域对象上写入值之并（初值 0）
    fconst: HashMap<(String, String), Prim>,
    /// athrow 抛出的值之并（处理器入口的异常值）
    pub(crate) thrown: Refs,
    sums: Vec<Summary<'a>>,
    /// (类, 方法下标, 上下文键) → 摘要；上下文键为实参抽象值的编码，同一方法超过 [`MAX_CTX`] 个上下文后
    /// 新调用并入合并上下文（键为 [`MERGED`]，形参取并）
    sum_ix: HashMap<(String, usize, Vec<u64>), usize>,
    ctx_count: HashMap<(String, usize), usize>,
    /// 当前轮次；摘要只在本轮被请求时执行（实参增长后旧上下文不再执行，其效果已在堆上）
    pass: u32,
    pub(crate) changed: bool,
    pub(crate) steps: u64,
    /// 浅模式：只求栈形（候选识别），调用不分析被调方法
    pub(crate) shallow: bool,
    /// 正在执行的摘要（None = 区域代码）
    cur: Option<usize>,
    /// 正在执行的代码抛出的异常可能在区域动态范围内被捕获（区域代码本身不受处理器覆盖：false）
    pub(crate) catch_ctx: bool,
    /// 当前指令处发起的调用：被调方的 `catchable`
    pub(crate) site_covered: bool,
    /// 在途的 lambda 实现句柄虚调（owner, name, desc）：call_virtual ↔ call_lambda 互递归的环判定
    lambda_stack: Vec<(String, String, String)>,
}

/// 单个候选区域的步数上限（超出即拒绝）
/// 字段槽：数组元素、lambda 捕获值为保留槽；`ALL` 只用于读（任一字段）
pub(crate) type Slot = u32;
pub(crate) const ELEM: Slot = 0;
pub(crate) const CAP: Slot = 1;
pub(crate) const ALL: Slot = Slot::MAX;

/// 每个方法的实参上下文上限
const MAX_CTX: usize = 16;
const MERGED: u64 = u64::MAX;

/// 实参抽象值的上下文键
fn ctx_key(args: &[Val]) -> Vec<u64> {
    let mut k = Vec::new();
    for a in args {
        match a {
            Val::Ref(r) => {
                k.push(1);
                let words = r.words();
                k.push(words.len() as u64);
                k.extend_from_slice(words);
            }
            Val::Int(Prim::Const(c)) => k.extend([2, *c as u32 as u64]),
            Val::Int(Prim::Bot) => k.push(3),
            Val::Int(Prim::Top) => k.push(4),
            Val::Wide => k.push(5),
            Val::Void => k.push(6),
            Val::Live => k.push(7),
        }
    }
    k
}

const STEP_BUDGET: u64 = 4_000_000;
const MAX_PASSES: u32 = 256;

impl<'w, 'a> Analysis<'w, 'a> {
    pub fn new(w: &'w World<'a>, shallow: bool) -> Self {
        let mut a = Analysis {
            w,
            nodes: Vec::new(),
            ix: HashMap::new(),
            base: Vec::new(),
            fields: Vec::new(),
            slots: HashMap::new(),
            fconst: HashMap::new(),
            thrown: Refs::default(),
            sums: Vec::new(),
            sum_ix: HashMap::new(),
            ctx_count: HashMap::new(),
            pass: 0,
            changed: false,
            steps: 0,
            shallow,
            cur: None,
            catch_ctx: false,
            site_covered: false,
            lambda_stack: Vec::new(),
        };
        a.slot("[]");
        a.slot("<cap>");
        let o = a.node(NodeKind::Old);
        let x = a.node(NodeKind::FreshUnknown);
        debug_assert!(o == OLD && x == FRESH_UNKNOWN);
        a
    }

    pub(crate) fn node(&mut self, k: NodeKind) -> NodeId {
        if let Some(&n) = self.ix.get(&k) {
            return n;
        }
        let n = self.nodes.len() as NodeId;
        let init = match &k {
            NodeKind::Old | NodeKind::Alloc(_) | NodeKind::Lambda(_) | NodeKind::Native(_) => Refs::default(),
            NodeKind::FreshUnknown | NodeKind::Made(_) => {
                let mut r = Refs::single(OLD);
                r.insert(FRESH_UNKNOWN);
                r
            }
        };
        self.nodes.push(k.clone());
        self.base.push(init);
        self.fields.push(HashMap::new());
        self.ix.insert(k, n);
        self.changed = true;
        n
    }

    pub(crate) fn tick(&mut self, n: u64) -> R<()> {
        self.steps += n;
        if self.steps > STEP_BUDGET {
            return reject("分析步数超出上限".into());
        }
        Ok(())
    }

    /// 区域代码（直线段）的全局不动点：`run_root` 每轮执行一次区域代码
    pub fn fixpoint(&mut self, mut run_root: impl FnMut(&mut Self) -> R<()>) -> R<()> {
        for pass in 1..=MAX_PASSES {
            self.pass = pass;
            self.changed = false;
            self.cur = None;
            self.catch_ctx = false;
            self.site_covered = false;
            run_root(self)?;
            // 本轮被请求的摘要逐个执行，直到没有请求了而未执行的
            loop {
                let mut any = false;
                let mut i = 0;
                while i < self.sums.len() {
                    if self.sums[i].req == pass && self.sums[i].ran != pass {
                        any = true;
                        self.sums[i].ran = pass;
                        self.cur = Some(i);
                        self.catch_ctx = self.sums[i].catchable;
                        if let Err(Reject(why)) = self.run_summary(i) {
                            return reject(format!("{why}；调用链：{}", self.chain(i)));
                        }
                    }
                    i += 1;
                }
                if !any {
                    break;
                }
            }
            if !self.changed {
                return Ok(());
            }
        }
        reject("不动点迭代轮数超出上限".into())
    }

    // ── 抽象堆 ────────────────────────────────────────────────────────────────

    /// 字段槽编号（按字段名；数组元素、lambda 捕获值各一个保留槽）
    pub(crate) fn slot(&mut self, name: &str) -> Slot {
        let n = self.slots.len() as Slot;
        *self.slots.entry(name.to_string()).or_insert(n)
    }

    /// 读引用字段 / 数组元素（`ALL` 为任一字段）
    pub(crate) fn load(&self, recv: &Refs, slot: Slot) -> Refs {
        let mut out = Refs::default();
        for n in recv.iter() {
            if n == OLD {
                out.insert(OLD);
                continue;
            }
            out.union_with(&self.base[n as usize]);
            let f = &self.fields[n as usize];
            if slot == ALL {
                for v in f.values() {
                    out.union_with(v);
                }
            } else if let Some(v) = f.get(&slot) {
                out.union_with(v);
            }
        }
        out
    }

    /// 写引用字段 / 数组元素
    pub(crate) fn store(&mut self, recv: &Refs, slot: Slot, val: &Refs, what: &dyn Fn() -> String) -> R<()> {
        debug_assert!(slot != ALL);
        if val.has_fresh() && recv.contains(OLD) {
            return reject(format!("区域对象存入旧对象：{}", what()));
        }
        if val.is_empty() {
            return Ok(());
        }
        for n in recv.iter().filter(|&n| n != OLD) {
            if self.fields[n as usize].entry(slot).or_default().union_with(val) {
                self.changed = true;
            }
        }
        Ok(())
    }

    /// 复制节点全部内容（数组 clone）
    pub(crate) fn copy_contents(&mut self, from: NodeId, to: NodeId) {
        let src = self.fields[from as usize].clone();
        let b = self.base[from as usize].clone();
        let dst = &mut self.fields[to as usize];
        for (k, v) in src {
            if dst.entry(k).or_default().union_with(&v) {
                self.changed = true;
            }
        }
        if self.base[to as usize].union_with(&b) {
            self.changed = true;
        }
    }

    /// 节点可直接到达的全部节点
    pub(crate) fn succ(&self, n: usize) -> Refs {
        let mut out = self.base[n].clone();
        for v in self.fields[n].values() {
            out.union_with(v);
        }
        out
    }

    pub(crate) fn get_int_field(&self, recv: &Refs, name: &str, desc: &str) -> Prim {
        let exact = recv.iter().all(|n| matches!(self.nodes[n as usize], NodeKind::Alloc(_) | NodeKind::Lambda(_)));
        if recv.is_empty() {
            return Prim::Bot;
        }
        if !exact {
            return Prim::Top;
        }
        self.fconst.get(&(name.to_string(), desc.to_string())).copied().unwrap_or(Prim::Const(0)).join(Prim::Const(0))
    }

    pub(crate) fn put_int_field(&mut self, recv: &Refs, name: &str, desc: &str, v: Prim) {
        if !recv.has_fresh() {
            return;
        }
        let e = self.fconst.entry((name.to_string(), desc.to_string())).or_insert(Prim::Bot);
        let n = e.join(v);
        if n != *e {
            *e = n;
            self.changed = true;
        }
    }

    pub(crate) fn throw(&mut self, v: &Refs) {
        if self.thrown.union_with(v) {
            self.changed = true;
        }
    }

    // ── 方法摘要 ──────────────────────────────────────────────────────────────

    /// 摘要 `i` 的首次请求链（自内向外，至多 16 层）
    fn chain(&self, mut i: usize) -> String {
        let mut out = Vec::new();
        loop {
            let s = &self.sums[i];
            let m = &s.ci.methods()[s.idx];
            out.push(format!("{}.{}{}", s.ci.name(), m.name, m.desc));
            match s.parent {
                Some(p) if out.len() < 16 => i = p,
                _ => break,
            }
        }
        out.join(" ← ")
    }

    fn run_summary(&mut self, i: usize) -> R<()> {
        let (ci, idx) = (self.sums[i].ci, self.sums[i].idx);
        let m = &ci.methods()[idx];
        let mut locals = Vec::new();
        let (kinds, _) = method_kinds(&m.desc);
        let params = self.sums[i].params.clone();
        let mut it = params.into_iter();
        if !m.is_static() {
            locals.push(it.next().unwrap_or(Val::null()));
        }
        for k in kinds {
            let v = it.next().unwrap_or(Val::Void);
            locals.push(v);
            if k == Kind::Wide {
                locals.push(Val::Void);
            }
        }
        let rets = crate::flow::run_method(self, ci, m, locals)?;
        if let Some(r) = rets {
            let s = &mut self.sums[i];
            match &mut s.ret {
                Some(old) => {
                    if old.join_in(&r) {
                        self.changed = true;
                    }
                }
                None => {
                    s.ret = Some(r);
                    self.changed = true;
                }
            }
        }
        Ok(())
    }

    /// 调用一个已解析的目标：返回 None 表示不正常返回
    pub(crate) fn call_target(&mut self, ci: &'a ClassInfo, idx: usize, args: Vec<Val>) -> R<Option<Val>> {
        let m: &'a Method = &ci.methods()[idx];
        let (_, rk) = method_kinds(&m.desc);
        let top = |k: Option<Kind>| match k {
            None => Val::Void,
            Some(Kind::Ref) => Val::old(),
            Some(Kind::Int) => Val::int_top(),
            Some(Kind::Wide) => Val::Wide,
        };
        let key = || format!("{}.{}:{}", ci.name(), m.name, m.desc);
        let fresh_arg = args.iter().any(|a| a.refs().has_fresh());
        if self.shallow {
            return Ok(Some(top(rk)));
        }
        match self.w.callee(ci, idx) {
            Callee::Stub | Callee::Abstract => Ok(None),
            Callee::Effect(EscapeEffect::Pure) => Ok(Some(top(rk))),
            Callee::Effect(EscapeEffect::ReturnsReceiver) => {
                Ok(Some(if rk.is_some() { args.first().cloned().unwrap_or(Val::null()) } else { Val::Void }))
            }
            Callee::Effect(EscapeEffect::Fresh) => {
                let n = self.node(NodeKind::Native(key()));
                Ok(Some(Val::Ref(Refs::single(n))))
            }
            Callee::Effect(EscapeEffect::ArrayCopy) => {
                let (Some(src), Some(dst)) = (args.first(), args.get(2)) else {
                    return reject(format!("array_copy 登记形参不符：{}", key()));
                };
                let v = self.load(&src.refs(), ELEM);
                self.store(&dst.refs(), ELEM, &v, &key)?;
                Ok(Some(top(rk)))
            }
            Callee::Native if fresh_arg => reject(format!("区域对象传入 native：{}", key())),
            Callee::Native => Ok(Some(top(rk))),
            Callee::Opaque => reject(format!("调用手写 / VM 内建方法：{}", key())),
            Callee::Code(ci, _) => Ok(self.call_summary(ci, idx, args)),
        }
    }

    fn call_summary(&mut self, ci: &'a ClassInfo, idx: usize, args: Vec<Val>) -> Option<Val> {
        let mk = (ci.name().to_string(), idx);
        let exact = (mk.0.clone(), idx, ctx_key(&args));
        let merged = (mk.0.clone(), idx, vec![MERGED]);
        let found = self.sum_ix.get(&exact).or_else(|| self.sum_ix.get(&merged)).copied();
        let mut changed = false;
        let i = match found {
            Some(i) => i,
            None => {
                changed = true;
                let n = self.ctx_count.entry(mk).or_default();
                let key = if *n >= MAX_CTX { merged } else { exact };
                *n += 1;
                let i = self.sums.len();
                self.sum_ix.insert(key, i);
                let params = args.clone();
                self.sums.push(Summary {
                    ci,
                    idx,
                    params,
                    ret: None,
                    parent: self.cur,
                    catchable: false,
                    req: 0,
                    ran: 0,
                });
                i
            }
        };
        let (pass, site_covered) = (self.pass, self.site_covered);
        let s = &mut self.sums[i];
        for (p, a) in s.params.iter_mut().zip(&args) {
            changed |= p.join_in(a);
        }
        if site_covered && !s.catchable {
            s.catchable = true;
            changed = true;
        }
        s.req = pass;
        if changed {
            self.changed = true;
        }
        s.ret.clone()
    }

    /// 按接收者节点逐一分派的虚调用；`args[0]` 为接收者
    pub(crate) fn call_virtual(&mut self, owner: &str, name: &str, desc: &str, args: Vec<Val>) -> R<Option<Val>> {
        let recv = args.first().map(|a| a.refs()).unwrap_or_default();
        let mut out: Option<Val> = None;
        let join = |out: &mut Option<Val>, r: Option<Val>| {
            if let Some(r) = r {
                match out {
                    Some(o) => {
                        o.join_in(&r);
                    }
                    None => *out = Some(r),
                }
            }
        };
        for n in recv.iter().collect::<Vec<_>>() {
            let mut a = args.clone();
            a[0] = Val::Ref(Refs::single(n));
            let kind = self.nodes[n as usize].clone();
            match kind {
                NodeKind::Old | NodeKind::FreshUnknown | NodeKind::Native(_) => {
                    let targets = self.w.cha(owner, name, desc);
                    self.tick(targets.len() as u64)?;
                    for (c, i) in targets.iter() {
                        let ci = self.w.class(c).expect("CHA 目标类");
                        let r = self.call_target(ci, *i, a.clone())?;
                        join(&mut out, r);
                    }
                    if n == OLD || n == FRESH_UNKNOWN {
                        for site in self.w.lambdas().to_vec() {
                            if site.sam == name && self.w.is_subtype(&site.iface, owner) {
                                let r = self.call_lambda(&site, None, desc, a[1..].to_vec())?;
                                join(&mut out, r);
                            }
                        }
                    }
                }
                NodeKind::Alloc(c) | NodeKind::Made(c) => {
                    if c.starts_with('[') && name == "clone" {
                        let fresh = self.node(NodeKind::Alloc(c.clone()));
                        self.copy_contents(n, fresh);
                        join(&mut out, Some(Val::Ref(Refs::single(fresh))));
                        continue;
                    }
                    for (ci, i) in self.w.resolve_virtual(&c, name, desc) {
                        let r = self.call_target(ci, i, a.clone())?;
                        join(&mut out, r);
                    }
                }
                NodeKind::Lambda(site) => {
                    if site.sam == name {
                        let r = self.call_lambda(&site, Some(n), desc, a[1..].to_vec())?;
                        join(&mut out, r);
                    } else {
                        for (ci, i) in self.w.resolve_virtual(&site.iface, name, desc) {
                            let r = self.call_target(ci, i, a.clone())?;
                            join(&mut out, r);
                        }
                    }
                }
            }
        }
        Ok(out)
    }

    /// lambda 实现句柄调用：实参 = 捕获值 ++ 调用实参（按实现描述符的槽类别转换，装箱 / 拆箱视为新分配 / ⊤）
    fn call_lambda(&mut self, site: &LambdaSite, node: Option<NodeId>, sam_desc: &str, call: Vec<Val>) -> R<Option<Val>> {
        if self.shallow {
            return Ok(Some(Val::old()));
        }
        let (ik, irk) = method_kinds(&site.desc);
        let (_, srk) = method_kinds(sam_desc);
        // 实现形参类别：实例方法句柄（虚 / 接口 / 特殊）首位是接收者
        let mut kinds = Vec::new();
        if matches!(site.kind, 5 | 7 | 9) {
            kinds.push(Kind::Ref);
        }
        kinds.extend(ik);
        let captured = kinds.len().saturating_sub(call.len());
        let cap = node.map(|n| self.load(&Refs::single(n), CAP)).unwrap_or_else(|| Refs::single(OLD));
        let mut args = Vec::new();
        for (j, k) in kinds.iter().enumerate() {
            let v = if j < captured {
                match k {
                    Kind::Ref => Val::Ref(cap.clone()),
                    Kind::Int => Val::int_top(),
                    Kind::Wide => Val::Wide,
                }
            } else {
                let a = call.get(j - captured).cloned().unwrap_or(Val::Void);
                self.coerce(a, Some(*k))
            };
            args.push(v);
        }
        let r = match site.kind {
            5 | 9 => {
                // 环判定：call_virtual（OLD / FRESH_UNKNOWN / Lambda 接收者上的同 SAM 站点遍历）
                // ↔ call_lambda（5|9 实现句柄虚调）是纯 Rust 互递归，不经 call_summary 的工作列表，
                // 两个 SAM 名与实现名互为对方的站点（或单站点 sam == 实现名，如 compare/compare、
                // apply/apply 的方法引用）在 OLD 接收者上即成无限环（JDK 25 档案实测 FunctionalCompositionTest
                // 转译期撑爆线程栈，128 MB RUST_MIN_STACK 仍溢出）。lambda 站点有限，在途集合出现重复键必为环；
                // 命中按未知被调保守返回（与 call_target 的 Callee::Native 同口径），不影响无环链的结果。
                let key = (site.owner.clone(), site.name.clone(), site.desc.clone());
                if self.lambda_stack.iter().any(|k| *k == key) {
                    let (_, rk) = method_kinds(&site.desc);
                    Some(match rk {
                        None => Val::Void,
                        Some(Kind::Ref) => Val::old(),
                        Some(Kind::Int) => Val::int_top(),
                        Some(Kind::Wide) => Val::Wide,
                    })
                } else {
                    self.lambda_stack.push(key);
                    let r = self.call_virtual(&site.owner, &site.name, &site.desc, args)?;
                    self.lambda_stack.pop();
                    r
                }
            }
            6 | 7 => match self.w.resolve_direct(&site.owner, &site.name, &site.desc) {
                Some((ci, i)) => self.call_target(ci, i, args)?,
                None => None,
            },
            8 => {
                let fresh = self.node(NodeKind::Alloc(site.owner.clone()));
                let mut a = vec![Val::Ref(Refs::single(fresh))];
                a.extend(args);
                match self.w.resolve_direct(&site.owner, &site.name, &site.desc) {
                    Some((ci, i)) => self.call_target(ci, i, a)?.map(|_| Val::Ref(Refs::single(fresh))),
                    None => None,
                }
            }
            k => return reject(format!("未知方法句柄类别 {k}：{}.{}", site.owner, site.name)),
        };
        let _ = irk;
        Ok(r.map(|v| self.coerce(v, srk)))
    }

    /// 槽类别转换：引用 → 基本类型为拆箱（⊤），基本类型 → 引用为装箱（运行时新分配）
    pub(crate) fn coerce(&mut self, v: Val, k: Option<Kind>) -> Val {
        match (k, &v) {
            (None, _) => Val::Void,
            (Some(Kind::Ref), Val::Ref(_) | Val::Live) => Val::Ref(v.refs()),
            (Some(Kind::Ref), _) => Val::Ref(Refs::single(FRESH_UNKNOWN)),
            (Some(Kind::Int), Val::Int(_)) => v,
            (Some(Kind::Int), _) => Val::int_top(),
            (Some(Kind::Wide), _) => Val::Wide,
        }
    }

    /// invoke 指令：弹出实参、分派、压入返回值；返回 false 表示不正常返回
    pub(crate) fn invoke(&mut self, insn: &Insn, f: &mut Frame) -> R<bool> {
        let Operand::Method(mref, _) = &insn.operand else { return reject("invoke 操作数".into()) };
        let (pk, rk) = method_kinds(&mref.desc);
        let mut args = Vec::with_capacity(pk.len() + 1);
        for _ in 0..pk.len() {
            args.push(crate::step::pop(f)?);
        }
        if insn.opcode != op::INVOKESTATIC {
            args.push(crate::step::pop(f)?);
        }
        args.reverse();
        let r = match insn.opcode {
            op::INVOKESTATIC | op::INVOKESPECIAL => {
                match self.w.resolve_direct(&mref.owner, &mref.name, &mref.desc) {
                    Some((ci, i)) => self.call_target(ci, i, args)?,
                    // 解析不到的目标（类不在类路径上）：保守视为不透明
                    None if args.iter().any(|a| a.refs().has_fresh()) && !self.shallow => {
                        return reject(format!("无法解析的调用：{mref}"))
                    }
                    None => Some(Val::old()),
                }
            }
            _ => self.call_virtual(&mref.owner, &mref.name, &mref.desc, args)?,
        };
        let Some(r) = r else { return Ok(false) };
        if rk.is_some() {
            let v = self.coerce(r, rk);
            f.stack.push(v);
        }
        Ok(true)
    }
}
