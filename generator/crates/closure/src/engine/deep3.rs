//! 引擎：小分配 soup 逐表深字节归因（`--lambda-prof` 的 census deep3 行，只读诊断；性能计划 §4.17）。
//!
//! §4.16 的 census deep/deep2 已归因 ~7.6 GB，活跃字节另有 ~6 GB 住在 64–1024 B 尺寸段的小分配
//! （事件 args 内层 `Vec<V>`、pstr / keyed、MemberRef 串、输出 B 树的 String 键等未逐项枚举的表）。
//! 本模块逐表走**真实深堆**（capacity 口径，与计数分配器同口径）：
//! - `Rc` 内容（`Rc<str>` / `Rc<[i32]>` / `Srcs` / `Rc<Obj>` / `Rc<Analysis>` / `Rc<Cfg>` / `Rc<[V]>` …）
//!   按**指针去重**——同一分配只记入首个到达的表，跨表共享（名字串 / 共享摘要）不会重复计数；
//! - `String` 记 capacity+1、`Vec` 记 capacity×元素宽、`HashMap`/`IndexMap` 记 capacity×槽宽×9/7、
//!   `BTreeMap`/`BTreeSet` 记 n×(键值宽+8)（B 树节点摊还）；
//! - 各表合计与计数分配器的活跃字节对账，残差即排空间隙的工作队列瞬态与本轮仍未枚举的结构。
//!
//! 输出：`census deep3` 一行（≥8 MiB 的表，按字节降序）+ `census deep3b` 一行（小表逐项与合计、去重指针数），
//! 随 `--lambda-prof` 周期打印，增长曲线即逐 dump 串起。

use super::*;
use crate::absint::Srcs;

/// 深字节累加器：`Rc` 指针去重（同一分配只记入首个到达的表）
pub(super) struct Deep {
    seen: HashSet<(usize, u8)>,
}

fn s_cap(s: &str) -> usize {
    s.len() + 1
}
/// B 树节点摊还：每元素键值宽 + 8（子指针 / 计数摊还，归因量级即可）
const BT: usize = 8;

impl Deep {
    pub(super) fn new() -> Deep {
        Deep { seen: HashSet::default() }
    }
    fn once(&mut self, p: usize, k: u8) -> bool {
        self.seen.insert((p, k))
    }
    /// `Rc<str>`：两个引用计数 + 内容 + NUL
    pub(super) fn rc_str(&mut self, s: &Rc<str>) -> usize {
        self.once(Rc::as_ptr(s) as *const u8 as usize, 0).then(|| 16 + s.len() + 1).unwrap_or(0)
    }
    pub(super) fn ints(&mut self, s: &Rc<[i32]>) -> usize {
        self.once(s.as_ptr() as usize, 1).then(|| 16 + s.len() * 4).unwrap_or(0)
    }
    pub(super) fn srcs(&mut self, s: &Srcs) -> usize {
        self.once(s.as_ptr() as usize, 2).then(|| 16 + s.len() * std::mem::size_of::<Src>()).unwrap_or(0)
    }
    fn cfg(&mut self, c: &Rc<absint::cfg::Cfg>) -> usize {
        if !self.once(Rc::as_ptr(c) as *const absint::cfg::Cfg as usize, 3) {
            return 0;
        }
        16 + c.prof_bytes()
    }
    fn vs(&mut self, s: &Rc<[V]>) -> usize {
        if !self.once(s.as_ptr() as usize, 4) {
            return 0;
        }
        16 + s.len() * std::mem::size_of::<V>() + s.iter().map(|v| self.v(v)).sum::<usize>()
    }
    fn obj(&mut self, o: &Rc<Obj>) -> usize {
        if !self.once(Rc::as_ptr(o) as *const Obj as usize, 5) {
            return 0;
        }
        let inner = match &**o {
            Obj::Fields(fs) | Obj::Image(_, fs) => {
                fs.capacity() * std::mem::size_of::<(MemberRef, V)>()
                    + fs.iter().map(|(k, v)| self.member(k) + self.v(v)).sum::<usize>()
            }
            Obj::Elems(es) => self.vs(es),
            Obj::Accepted(s) => self.rc_str(s),
            _ => 0,
        };
        16 + std::mem::size_of::<Obj>() + inner
    }
    pub(super) fn member(&mut self, m: &MemberRef) -> usize {
        s_cap(&m.owner) + s_cap(&m.name) + s_cap(&m.desc)
    }
    /// 抽象值的深内容（枚举本体由所在容器计入）
    pub(super) fn v(&mut self, v: &V) -> usize {
        match v {
            V::Ints(xs) => self.ints(xs),
            V::Ref { ty: Some(t), src, obj, .. } => self.rc_str(t) + self.srcs(src) + obj.as_ref().map_or(0, |o| self.obj(o)),
            V::Ref { src, .. } => self.srcs(src),
            V::Str(s, src) => self.rc_str(s) + self.srcs(src),
            V::Class(c, _) => self.rc_str(c),
            V::Offset(m) => 16 + self.member(m),
            _ => 0,
        }
    }
    fn vs_vec(&mut self, xs: &Vec<V>) -> usize {
        xs.capacity() * std::mem::size_of::<V>() + xs.iter().map(|v| self.v(v)).sum::<usize>()
    }
    fn pv(&mut self, p: &PV) -> usize {
        match p {
            PV::Const(v) => self.v(v),
            PV::Top => 0,
        }
    }
    fn const_(&mut self, c: &Const) -> usize {
        match c {
            Const::String(s) | Const::Class(s) | Const::MethodType(s) => s_cap(s),
            Const::StringUtf16(xs) => xs.capacity() * 2,
            Const::MethodHandle(h) => 16 + self.member(&h.member),
            Const::Dynamic(_, a, b) => s_cap(a) + s_cap(b),
            _ => 0,
        }
    }
    fn event(&mut self, e: &Event) -> usize {
        match e {
            Event::Invoke { mref, args, .. } => self.member(mref) + self.vs_vec(args),
            Event::Indy { name, desc, args, .. } => s_cap(name) + s_cap(desc) + self.vs_vec(args),
            Event::New(t) | Event::NewArray(t, _) => s_cap(t),
            Event::Field { mref, recv, value, .. } => {
                self.member(mref) + recv.as_ref().map_or(0, |v| self.v(v)) + value.as_ref().map_or(0, |v| self.v(v))
            }
            Event::Ldc(c) => self.const_(c),
            Event::CheckCast(t, v) | Event::InstanceOf(t, v) => s_cap(t) + v.as_ref().map_or(0, |v| self.v(v)),
            Event::NotInstance(t, v) | Event::MirrorSub(t, v) => s_cap(t) + self.v(v),
            Event::KeyTest { kc, input, name, .. } => s_cap(kc) + self.v(input) + self.v(name),
            Event::ArrayLoad { array, index } => self.v(array) + self.v(index),
            Event::ArrayStore { array, index, value } => self.v(array) + self.v(index) + self.v(value),
            Event::Throw(v) | Event::Return(v) => self.v(v),
            Event::Catch(t) => t.as_ref().map_or(0, |x| s_cap(x)),
            Event::Const { value, .. } => self.v(value),
        }
    }
    /// 一份分析（`Rc` 指针去重：analysis / applied / 共享摘要只计一次）
    fn analysis(&mut self, a: &Rc<Analysis>) -> usize {
        if !self.once(Rc::as_ptr(a) as *const Analysis as usize, 6) {
            return 0;
        }
        let ev = a.events.capacity() * (4 + std::mem::size_of::<Event>())
            + a.events.iter().map(|(_, e)| self.event(e)).sum::<usize>();
        16 + std::mem::size_of::<Analysis>() + ev
            + a.reachable.capacity()
            + a.pending_types.capacity() * 24
            + a.pending_types.iter().map(|x| s_cap(x)).sum::<usize>()
            + a.mirror_assumed.capacity() * (2 + 24)
            + a.mirror_assumed.iter().map(|(_, s)| s_cap(s)).sum::<usize>()
            + self.cfg(&a.cfg)
    }
    fn via(&mut self, v: &Via) -> usize {
        match &v.from {
            From::Root(s) | From::Class(s) => s_cap(s),
            _ => 0,
        }
    }
    fn feed(&mut self, f: &Feed) -> usize {
        match f {
            Feed::N(_) => 0,
            Feed::S(s) => s.classes.heap_bytes() + s.open.heap_bytes(),
        }
    }
    fn args(&mut self, a: &Args) -> usize {
        a.capacity() * std::mem::size_of::<Option<Vec<Feed>>>()
            + a.iter()
                .flatten()
                .map(|xs| xs.capacity() * std::mem::size_of::<Feed>() + xs.iter().map(|f| self.feed(f)).sum::<usize>())
                .sum::<usize>()
    }
    fn typeset(&mut self, s: &TypeSet) -> usize {
        s.classes.heap_bytes() + s.open.heap_bytes()
    }
    /// `Rc<[u32]>`（增长枢纽来源 / 汇集对象集等；指针去重）
    pub(super) fn u32s(&mut self, s: &Rc<[u32]>) -> usize {
        self.once(s.as_ptr() as usize, 7).then(|| 16 + s.len() * 4).unwrap_or(0)
    }
    pub(super) fn strings(&mut self, v: &Rc<[String]>) -> usize {
        if !self.once(v.as_ptr() as usize, 9) {
            return 0;
        }
        16 + v.len() * 24 + v.iter().map(|x| s_cap(x)).sum::<usize>()
    }
    fn feeds(&mut self, v: &Rc<[Feed]>) -> usize {
        if !self.once(v.as_ptr() as usize, 10) {
            return 0;
        }
        16 + v.len() * std::mem::size_of::<Feed>() + v.iter().map(|f| self.feed(f)).sum::<usize>()
    }
    fn hw_writes(&mut self, v: &Rc<[Option<HwWrite>]>) -> usize {
        if !self.once(v.as_ptr() as usize, 11) {
            return 0;
        }
        16 + v.len() * std::mem::size_of::<Option<HwWrite>>()
            + v.iter().flatten().map(|w| w.values.capacity() * 8 + w.elements.capacity() * 8).sum::<usize>()
    }
}

impl Engine<'_> {
    /// 逐表深字节归因：`census deep3`（大表，按字节降序）/ `census deep3b`（小表逐项与合计）。
    /// 字节口径见模块说明
    pub(super) fn deep3_dump(&self, tag: &str) {
        let mib = |b: usize| b as f64 / 1048576.0;
        let mut d = Deep::new();
        let mut rows: Vec<(&'static str, usize, u64)> = Vec::new();
        let mut small: Vec<(&'static str, usize)> = Vec::new();
        macro_rules! take {
            ($name:expr, $b:expr, $n:expr) => {{
                let b: usize = $b;
                if b >= 8 << 20 {
                    rows.push(($name, b, $n));
                } else {
                    small.push(($name, b));
                }
            }};
        }

        // ── methods 列：MNode + 键串 + 溯源串 + 摘要（analysis/applied 指针去重，共享摘要在此落名）
        {
            let mut b = self.methods.capacity() * std::mem::size_of::<((MemberRef, u32), MNode)>() * 9 / 7;
            let mut anev = 0u64;
            for (_, m) in &self.methods {
                b += d.member(&m.key) + d.via(&m.via);
                b += m.hw_fns.capacity() * 24 + m.hw_fns.iter().map(|x| s_cap(x)).sum::<usize>();
                b += m.returned.as_ref().map_or(0, |r| r.as_ref().map_or(0, |v| v.capacity() * 2));
                if let Some(a) = &m.analysis {
                    anev += a.events.len() as u64;
                    b += d.analysis(a);
                }
                if let Some(a) = &m.applied {
                    anev += a.events.len() as u64;
                    b += d.analysis(a);
                }
            }
            take!("methods+an", b, anev);
        }
        // ── 共享摘要表（Dep 键与参数镜像串；分析本体已在 methods 行按指针去重）
        {
            let mut b = self.shared.capacity() * std::mem::size_of::<(MemberRef, Vec<share::Shared>)>() * 9 / 7;
            for (k, v) in &self.shared {
                b += d.member(k) + v.capacity() * std::mem::size_of::<share::Shared>();
                for s in v {
                    b += s.prof_bytes(&mut d);
                }
            }
            take!("shared", b, self.shared.len() as u64);
        }
        // ── 名字表
        {
            let mut b = self.names.capacity() * 16;
            for n in &self.names {
                b += d.rc_str(n);
            }
            b += self.ids.capacity() * 24 * 9 / 7;
            take!("names", b, self.names.len() as u64);
        }
        // ── 抽象对象 / 分配点链 / 镜像
        {
            let mut b = self.objs.capacity() * 12 * 9 / 7 + self.obj_chain.capacity() * 24 * 9 / 7;
            for v in self.obj_chain.values() {
                b += d.rc_str(v);
            }
            b += self.seg_cls.capacity() * 24 * 9 / 7;
            for k in self.seg_cls.keys() {
                b += d.rc_str(k);
            }
            b += self.site_variants.capacity() * 24 * 9 / 7;
            for k in self.site_variants.keys() {
                b += d.rc_str(k);
            }
            b += self.obj_fields.values().map(|v| v.capacity() * 10).sum::<usize>()
                + self.escaped.capacity() * 8
                + self.arrays.capacity() * 12 * 9 / 7
                + self.ctx_heap.capacity() * 12 * 9 / 7
                + self.containers.capacity() * 9
                + self.empty_arrays.values().map(|m| m.values().map(|t| d.typeset(t)).sum::<usize>()).sum::<usize>()
                + self.mirrors.capacity() * 12 * 9 / 7
                + self.mirror_of.capacity() * 4;
            take!("objs+chains", b, self.objs.len() as u64);
        }
        // ── MemberRef 族：mref_keys / 字段表 / 各按成员键的映射
        {
            let mut b = self.mref_keys.capacity() * std::mem::size_of::<(MemberRef, (Rc<str>, bool))>() * 9 / 7;
            for (k, v) in &self.mref_keys {
                b += d.member(k) + d.rc_str(&v.0);
            }
            b += self.fields.capacity() * std::mem::size_of::<(MemberRef, Option<u32>)>() * 9 / 7
                + self.fields.keys().map(|k| d.member(k)).sum::<usize>()
                + self.mbase.capacity() * std::mem::size_of::<(MemberRef, usize)>() * 9 / 7
                + self.mbase.keys().map(|k| d.member(k)).sum::<usize>()
                + self.mem_decl.capacity() * 12 * 9 / 7
                + self.factories.capacity() * std::mem::size_of::<(MemberRef, bool)>() * 9 / 7
                + self.factories.keys().map(|k| d.member(k)).sum::<usize>()
                + self.lookup_hosts.capacity() * std::mem::size_of::<MemberRef>() * 9 / 7
                + self.lookup_hosts.iter().map(|k| d.member(k)).sum::<usize>();
            take!("mrefs", b, self.mref_keys.len() as u64);
        }
        // ── 输出面 String 键 B 树族
        {
            let set_s = |s: &BTreeSet<String>| s.len() * (24 + BT) + s.iter().map(|x| s_cap(x)).sum::<usize>();
            let b = set_s(&self.unresolved)
                + set_s(&self.refs)
                + set_s(&self.sigpoly_sites)
                + set_s(&self.reflect_gaps)
                + set_s(&self.serial_allocs)
                + set_s(&self.field_enum_gaps)
                + set_s(&self.hw_written_names)
                + set_s(&self.reflect_field_names)
                + self.indy_models.iter().map(|(k, v)| 24 + BT + s_cap(k) + s_cap(&v.0)).sum::<usize>()
                + self.static_mh_fields.iter().map(|(a, c)| (24 + s_cap(a) + s_cap(c)) * 2).sum::<usize>()
                + self.reflect_fields.iter().map(|(a, c)| (24 + s_cap(a) + s_cap(c)) * 2).sum::<usize>()
                + self.hw_written.iter().map(|m| std::mem::size_of::<MemberRef>() + BT + d.member(m)).sum::<usize>()
                + self.reflect_members.iter().map(|(_, m)| std::mem::size_of::<(Members, MemberRef)>() + BT + d.member(m)).sum::<usize>()
                + self.inited.iter().map(|(k, v)| 24 + s_cap(k) + d.via(v)).sum::<usize>()
                + self.missing.iter().map(|(k, v)| 24 + s_cap(k) + d.via(v)).sum::<usize>()
                + self.hw_read_names.iter().map(|(k, v)| 24 + BT + s_cap(k) + v.len() * std::mem::size_of::<Node>()).sum::<usize>()
                + self.hw_copy_names.values().map(|v| v.len() * 10).sum::<usize>()
                + self.fopen_name_via.iter().map(|(k, _)| 24 + BT + s_cap(k)).sum::<usize>()
                + self.reflect_names.values().map(|t| t.iter().map(|(s, _)| 24 + BT + s_cap(s)).sum::<usize>()).sum::<usize>();
            take!("outstr", b, self.refs.len() as u64);
        }
        // ── pstr（形参字符串常量集）/ keyed（按键查找闸门）
        take!("pstr", self.pstr.deep_bytes(), 0);
        take!("keyed", self.keyed.deep_bytes(&mut d), self.keyed.gates.len() as u64);
        // ── lambdas
        {
            let mut b = self.lambdas.capacity() * std::mem::size_of::<(u32, Lambda)>() * 9 / 7;
            for l in self.lambdas.values() {
                b += s_cap(&l.iface)
                    + s_cap(&l.sam)
                    + l.markers.capacity() * 24
                    + l.markers.iter().map(|x| s_cap(x)).sum::<usize>()
                    + d.member(&l.imh.member)
                    + d.args(&l.cap)
                    + l.vals.as_ref().map_or(0, |v| d.vs(v))
                    + l.adapt.capacity() * std::mem::size_of::<lambda_adapt::Conv>();
            }
            take!("lambdas", b, self.lambdas.len() as u64);
        }
        // ── 常量格 / 实参值的深内容（列宽已在 deep/deep2 计，这里补 V 深堆）
        {
            let mut b = 0usize;
            for v in self.pvals.values() {
                b += v.capacity() * std::mem::size_of::<PV>() + v.iter().map(|p| d.pv(p)).sum::<usize>();
            }
            for p in self.nret.values().chain(self.sret.values()) {
                b += d.pv(p);
            }
            for h in &self.hubs {
                if let Some(v) = &h.vals {
                    b += v.capacity() * std::mem::size_of::<PV>() + v.iter().map(|p| d.pv(p)).sum::<usize>();
                }
            }
            for c in &self.lcalls {
                for a in &c.call.1 {
                    if let Some(xs) = a {
                        b += xs.capacity() * std::mem::size_of::<Feed>() + xs.iter().map(|f| d.feed(f)).sum::<usize>();
                    }
                }
            }
            take!("pvals_deep", b, 0);
        }
        // ── 派发与枢纽挂点表
        {
            let b = self.dispatch.iter().map(|(_, v)| v.len() * (8 + BT)).sum::<usize>()
                + self.dispatch.len() * std::mem::size_of::<((usize, u32), BTreeSet<usize>)>()
                + self.hub_sites.iter().map(|(_, v)| v.len() * (4 + BT)).sum::<usize>()
                + self.hub_sites.len() * std::mem::size_of::<((usize, u32), BTreeSet<u32>)>()
                + self.open_methods.values().map(|v| v.len() * (8 + BT)).sum::<usize>()
                + self.open_sites.values().map(|v| v.len() * (16 + BT)).sum::<usize>()
                + self.g.len() * (4 + BT)
                + self.g_sub.capacity() * 24 * 9 / 7
                + self.g_sub.values().map(|v| v.capacity() * 4).sum::<usize>()
                + self.gathers.capacity() * std::mem::size_of::<(gather::Slot, u32, bool)>()
                + self.gather_ids.capacity() * 40 * 9 / 7
                + self.gather_ids.keys().map(|k| d.u32s(&k.2)).sum::<usize>()
                + self.callers.capacity() * std::mem::size_of::<(usize, BTreeSet<usize>)>() * 9 / 7
                + self.callers.values().map(|v| v.len() * (8 + BT)).sum::<usize>()
                + self.mflows.values().map(|v| v.capacity() * std::mem::size_of::<(Node, MirrorOp)>()).sum::<usize>()
                + self.mflows.capacity() * std::mem::size_of::<(Node, Vec<(Node, MirrorOp)>)>() * 9 / 7
                + self.mflow_seen.capacity() * std::mem::size_of::<(Node, Node, MirrorOp)>() * 9 / 7
                + self.mirror_open.values().map(|v| v.capacity() * 12).sum::<usize>()
                + self.mirror_open_seen.capacity() * 24 * 9 / 7
                + self.open_inj.iter().map(|(_, v)| v.len() * (4 + BT)).sum::<usize>()
                + self.array_of.capacity() * std::mem::size_of::<((usize, u32), ArrayOfSite)>() * 9 / 7
                + self.array_of.values().map(|s| s.mirrors.len() * (4 + BT) + s.dsts.capacity() * std::mem::size_of::<Node>()).sum::<usize>()
                + self.gather_last.values().flat_map(|m| m.values()).map(|v| d.u32s(&v.1)).sum::<usize>()
                + self.hw_gather_last.values().map(|v| d.u32s(&v.1)).sum::<usize>()
                + self.hw_rgather_last.values().map(|v| d.u32s(&v.1)).sum::<usize>()
                + self.hw_wgather_last.values().map(|v| d.u32s(&v.1)).sum::<usize>();
            take!("dispatch2", b, self.dispatch.len() as u64);
        }
        // ── 站点去重 / 反射调用族 / 按名查找族
        {
            let mut b = self.recv_sites.capacity() * 12 * 9 / 7
                + self.direct_virtual_sites.capacity() * 12 * 9 / 7
                + self.ptaint.capacity() * 16 * 9 / 7
                + self.htaint.capacity() * 12 * 9 / 7
                + self.lookup_top.capacity() * 12 * 9 / 7
                + self.lookup_unsure.capacity() * 12 * 9 / 7
                + self.lookup_pending.len() * (12 + BT)
                + self.lookup_released.capacity() * 12 * 9 / 7
                + self.lookup_trial.capacity() * 12 * 9 / 7
                + self.class_patterns.values().map(|p| p.capacity() * 24 + p.iter().map(|x| x.capacity() * std::mem::size_of::<class_lookup::Part>()).sum::<usize>()).sum::<usize>()
                + self.cp_pattern_hits.iter().map(|(k, v)| 24 + BT + s_cap(k) + d.strings(v)).sum::<usize>()
                + self.xreaders.values().map(|v| v.len() * (4 + BT)).sum::<usize>()
                + self.rdirect.capacity() * 48 * 9 / 7
                + self.rdirect.values().map(|m| d.member(m)).sum::<usize>()
                + self.rcall_conv.capacity() * 28 * 9 / 7
                + self.rcall_conv_pending.iter().map(|(k, v)| 24 + s_cap(k) + v.capacity() * std::mem::size_of::<(usize, u16, V)>() + v.iter().map(|x| d.v(&x.2)).sum::<usize>()).sum::<usize>()
                + self.rmark_all.values().map(|v| v.len() * (24 + BT) + v.iter().map(|x| s_cap(x)).sum::<usize>()).sum::<usize>()
                + self.rmark_readers.values().map(|v| v.len() * (12 + BT)).sum::<usize>()
                + self.ctor_lookups.iter().map(|(_, (n, vs))| 24 + BT + s_cap(n) + vs.capacity() * std::mem::size_of::<V>() + vs.iter().map(|x| d.v(x)).sum::<usize>()).sum::<usize>()
               
                + self.ctor_reqs.iter().map(|(_, v)| v.as_ref().map_or(0, |s| s.len() * (16 + BT) + s.iter().map(|x| d.rc_str(x)).sum::<usize>())).sum::<usize>()
                + self.rcall_members.capacity() * std::mem::size_of::<RcallMember>() * 9 / 7;
            for x in &self.rcall_members {
                b += d.member(&x.key) + x.done.capacity() * 8 + x.targets.capacity() * 8 + x.wait.capacity() * 4;
            }
            take!("sites_rcall", b, 0);
        }
        // ── 手写调用点 / 字段句柄 / 枚举常量族
        {
            let b = self.hw_sites.capacity() * 20
                + self.hw_site_ids.capacity() * 24 * 9 / 7
                + self.hw_reads.capacity() * 28 * 9 / 7
                + self.hw_offsets.capacity() * 24 * 9 / 7
                + self.hw_writes.capacity() * 40 * 9 / 7
                + self.hw_writes.values().map(|w| d.hw_writes(w)).sum::<usize>()
                + self.self_fields.values().map(|v| v.capacity() * std::mem::size_of::<(usize, u32, bool, Rc<[Feed]>, Node)>() + v.iter().map(|x| d.feeds(&x.3)).sum::<usize>()).sum::<usize>()
                + self.ref_fields.values().map(|v| 16 + v.len() * 16).sum::<usize>()
                + self.offset_waits.values().map(|v| v.capacity() * 20).sum::<usize>()
                + self.offset_read_waits.values().map(|v| v.capacity() * 20).sum::<usize>()
                + self.fa_sites.capacity() * 40 * 9 / 7
                + self.fh_marks.capacity() * 24 * 9 / 7
                + self.fh_named.capacity() * 32 * 9 / 7
                + self.enum_consts.capacity() * 20 * 9 / 7
                + self.enum_vals.capacity() * 48 * 9 / 7
                + self.enum_vals.keys().map(|k| d.rc_str(&k.1)).sum::<usize>()
                + self.field_strs.capacity() * 48 * 9 / 7
                + self.field_strs.keys().map(|k| d.member(k)).sum::<usize>()
                + self.field_strs.values().map(|v| v.as_ref().map_or(0, |s| s.len() * (16 + BT) + s.iter().map(|x| d.rc_str(x)).sum::<usize>())).sum::<usize>()
                + self.name_reads.values().map(|v| v.capacity() * 36 + v.iter().map(|(s, _, _)| s_cap(s)).sum::<usize>()).sum::<usize>()
                + self.hw_static_reads.capacity() * 32 * 9 / 7;
            take!("hw2", b, 0);
        }
        // ── 类表（ClassNode + 溯源串 + 层级溯源 B 树）
        {
            let mut b = self.classes.capacity() * std::mem::size_of::<(String, ClassNode)>() * 9 / 7;
            for (k, c) in &self.classes {
                b += s_cap(k)
                    + d.via(&c.via)
                    + c.level_via.len() * (std::mem::size_of::<(Level, Via)>() + BT)
                    + c.level_via.values().map(|v| d.via(v)).sum::<usize>();
            }
            take!("classes", b, self.classes.len() as u64);
        }
        // ── 工作队列与去重挡（瞬态，但对账需要）
        {
            let b = self.mwork.capacity() * 8
                + self.in_mwork.capacity() * 8 * 9 / 7
                + self.swork.capacity() * 8
                + self.in_swork.capacity() * 12 * 9 / 7
                + self.cwork.capacity() * 4
                + self.in_cwork.capacity() * 4 * 9 / 7
                + self.fwork.capacity() * 4
                + self.hw_self_copies.capacity() * 8 * 9 / 7
                + self.lambda_stack.capacity() * std::mem::size_of::<LambdaCall>();
            take!("work", b, self.fwork.len() as u64);
        }

        rows.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let mut line = String::new();
        for (name, b, n) in &rows {
            line.push_str(&format!(" {}={:.0}MiB/{}", name, mib(*b), n));
        }
        let big_sum: usize = rows.iter().map(|x| x.1).sum();
        eprintln!("{tag} census deep3{line} big_sum={:.0}MiB", mib(big_sum));
        let small_total: usize = small.iter().map(|x| x.1).sum();
        let mut line2 = String::new();
        for (name, b) in &small {
            line2.push_str(&format!(" {}={:.0}MiB", name, mib(*b)));
        }
        eprintln!("{tag} census deep3b{line2} small_sum={:.0}MiB uniq_ptrs={}", mib(small_total), d.seen.len());
    }
}
