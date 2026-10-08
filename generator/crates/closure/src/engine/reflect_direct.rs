//! 引擎：直连反射调用（清单 `[facts.reflect.direct_invokers]`，见 `manifest/direct.rs`）。
//!
//! 反射调用入口（Method.invoke）的体对任意反射对象判 CS、取访问器、两路调用访问器，分支取舍取决于运行期的
//! 反射对象，经它的每个调用点都把注解解析与 CS 访问器链带进闭包。查找结果建模为携带所指方法的标记
//! （`method_marks.rs`）后，调用点满足下列条件时，反射对象只可能是标记所指的非 CS 方法之一，原入口的执行与清单
//! 登记的特化入口（helper）逐句一致：
//! 1. 接收者（反射对象）值集全是标记（无 open、无普通反射对象）——一旦混入普通对象即回退；
//! 2. 各标记所指方法全非 @CallerSensitive。缺口标记（查找类所指未知）不产生目标、不致回退：缺口类上的成员本就不经
//!    查找点进入调用链，原入口与 helper 在缺口上同样落到调用链外（helper 的 native 与原入口的 native 访问器共用
//!    `reflect_dispatch::native_invoke`：非 CS 方法的实参检查、空接收者 NPE、拆箱 / 装箱、按接收者虚派发、
//!    InvocationTargetException 包装逐句一致；CS 方法只在 JDK 内且名字可见，缺口不改变此前提）。
//!
//! 满足时调用点接到 helper（静态调用，按 @CallerSensitive 声明压栈调用方所在类）与各目标：静态目标直接接边，实例目标
//! 按 invoke 的接收者实参值集逐类型选实现、各实现以接收者来源为 this 接边（同字节码虚调用的目标选择，见 `direct_virtual`）；目标形参（不含接收者）取 invoke 实参数组的元素
//! （按下标奇偶槽；open 数组取元素类型 open，同本地访问器的实参池 `reflect_call.rs`）；引用返回值流入调用点结果，
//! 基本类型返回值经装箱类 `valueOf` 流入（同本地访问器）；
//! 目标入反射分派面（`reflect_members`，helper 的 native 按声明键分派），不再接原入口——Method.invoke 体内的
//! CS 判定、注解解析、访问器链不经本调用点入链。条件随值集增长可由真变假（混入普通对象、标记所指出现 CS 方法）：
//! 一旦不满足，调用点记入 `rdirect_fallback`，此后恒按原入口接边（已接的直连边保留，过近似），导出时不改写。
//! 导出见 `report.rs`（folds `direct_calls`：全部克隆在该点都按直连处理时才改写）。

use super::*;
use crate::manifest::DirectInvoker;

impl<'a> Engine<'a> {
    /// 字节码调用点按直连处理：true = 已接 helper 与各目标，调用方不再按原入口接边
    pub(super) fn reflect_direct(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) -> bool {
        if opcode != classfile::op::INVOKEVIRTUAL || self.man.direct_invokers.is_empty() || self.rdirect_fallback.contains(&(m, off)) {
            return false;
        }
        let key = self.mref_key(mref);
        let man = self.man;
        let Some(spec) = man.direct_invokers.get(&key) else { return false };
        match self.direct_targets(m, off, spec, args) {
            Some(ts) => {
                self.direct_edges(m, off, spec, args, ts);
                true
            }
            None => {
                self.rdirect_fallback.insert((m, off));
                false
            }
        }
    }

    /// 直连条件成立时的目标集（接收者值集尚空 = 空集）；None = 不满足
    fn direct_targets(&mut self, m: usize, off: u32, spec: &DirectInvoker, args: &[V]) -> Option<Vec<MemberRef>> {
        let tid = self.id(&spec.recv);
        let fs = self.feeds(m, args.first()?, tid);
        let s = self.value_set(&fs);
        if !s.open.is_empty() {
            return None;
        }
        let mut out = BTreeSet::new();
        for x in s.classes.iter() {
            out.extend(self.mark_targets(x, (m, off))?);
        }
        for k in &out {
            let cf = self.h.class(&k.owner)?;
            let x = cf.method(&k.name, &k.desc)?;
            if x.annotations.iter().any(|a| self.man.is_caller_sensitive_annotation(&a.type_desc)) {
                return None;
            }
        }
        Some(out.into_iter().collect())
    }

    /// invoke 实参数组值 v 的元素来源（按下标奇偶槽）：数组分配点的元素节点；open 数组取元素类型 open
    fn direct_arg_feeds(&mut self, m: usize, v: Option<&V>) -> [Vec<Feed>; 2] {
        let mut out: [Vec<Feed>; 2] = Default::default();
        let Some(v) = v else { return out };
        let tid = self.id(&format!("[L{OBJECT};"));
        let fs = self.feeds(m, v, tid);
        let s = self.value_set(&fs);
        for x in s.classes.iter() {
            if self.arrays.contains_key(&x) {
                for p in PARITIES {
                    out[p as usize].push(Feed::N(Node::E(x, p)));
                }
            }
        }
        for o in s.open.iter() {
            let name = self.names[o as usize].clone();
            if let Some(c) = absint::component(&name) {
                let cid = self.id(&c);
                for f in out.iter_mut() {
                    f.push(Feed::S(TypeSet::open(cid)));
                }
            }
        }
        out
    }

    /// 直连接边：调用点 → helper（实参原样，结果不取 helper 返回值）；调用点 → 各目标（结果 = 目标返回值）
    fn direct_edges(&mut self, m: usize, off: u32, spec: &DirectInvoker, args: &[V], ts: Vec<MemberRef>) {
        let helper = spec.helper.clone();
        self.rdirect.insert((m, off), helper.clone());
        self.note_ref(&helper);
        let via = Via::method("invoke", m, Some(off));
        self.touch(&helper.owner, Level::Layout, via.clone());
        self.init(&helper.owner, via.clone());
        let Some(hmd) = parse_method(&helper.desc) else { return };
        let mut a: Args = Vec::with_capacity(hmd.params.len());
        for (p, v) in hmd.params.iter().zip(args.iter()) {
            let f = self.ptype(p).map(|t| self.feeds(m, v, t));
            a.push(f);
        }
        let hret = hmd.ret.as_ref().and_then(|r| self.ptype(r));
        // helper 标注 @CallerSensitive：生成器在改写后的调用点压入调用方所在类，分析同口径
        let wrapped = self.ref_caller_sensitive(&helper);
        let outer = std::mem::replace(&mut self.cs.site_wrapped, wrapped);
        let vals = self.call_vals.replace(Rc::from(args));
        let t = self.method(helper, via.clone());
        self.edge(m, off, t, Recv::None, &a, hret, None);
        self.cs.site_wrapped = false;
        // 目标的实参取自实参数组元素，形参常量按未知绑定
        self.call_vals = None;
        let el = self.direct_arg_feeds(m, args.get(2));
        let obj = self.id(OBJECT);
        let res = Node::S(m, off);
        for key in ts {
            self.init(&key.owner, via.clone());
            self.reflect_members.insert((Members::Methods, key.clone()));
            let ret = key.desc.rsplit_once(')').map_or(b'V', |(_, r)| r.as_bytes()[0]);
            let ret_ref = matches!(ret, b'L' | b'[');
            // 基本类型返回值由本地访问器装箱（`装箱类.valueOf`），装箱结果流入调用点结果
            if !ret_ref && ret != b'V' {
                self.box_edge(m, off, ret, &via, None, Some(res));
            }
            let rt = ret_ref.then_some(obj);
            let rn = ret_ref.then_some(res);
            let Some(cf) = self.h.class(&key.owner) else { continue };
            let Some(md) = parse_method(&key.desc) else { continue };
            let a: Args = md.params.iter().enumerate().map(|(i, p)| self.ptype(p).map(|_| el[i % 2].clone())).collect();
            let is_static = cf.method(&key.name, &key.desc).is_some_and(|x| x.is_static());
            if is_static {
                let t = self.method(key, via.clone());
                self.edge(m, off, t, Recv::None, &a, rt, rn);
            } else {
                // 实例目标：按 invoke 的接收者实参（声明类过滤）虚派发，同字节码 invokevirtual / invokeinterface
                let Some(recv) = args.get(1) else { continue };
                self.direct_virtual(m, off, &key, cf.is_interface(), recv, &a, rt, rn, &via);
            }
        }
        self.call_vals = vals;
        self.cs.site_wrapped = outer;
    }

    /// 实例目标 key 在调用点 (m, off) 上按接收者值 recv 派发（同字节码 invokevirtual / invokeinterface 的目标选择）：
    /// - 非虚目标（private / final 方法、final 类上的方法；同 `invoke.rs` 的非虚判定）：直接接已解析方法，接收者来源
    ///   （精确与 open）按声明类过滤流入 this，不建枢纽——序列化回调 `writeObject` / `readObject` 等私有方法即此类；
    /// - 精确接收者按类型选出实现（逐类型一次，按调用点与目标记在 `rdirect_sel`），各实现以接收者来源为 this 直接接边
    ///   （实现的 this 形参按声明类过滤；不按接收者对象克隆）。直连调用点的接收者是被序列化 / 反射调用的任意对象，
    ///   值集随程序增长、目标又按标记成百，按实现接边的边数是「目标的实现数」，与接收者增长次数无关；
    /// - open 部分经 open 枢纽，按目标声明类 C 归一（`direct_open_roots`）：值集含 C 的超类型 open 时只接 open(C)
    ///   一个枢纽（G(o) ∩ C = G(C)），否则只接与 C 相关的 open 并剔除被超类型涵盖者（`open_roots`，同字节码虚调用）。
    ///   枢纽键是（目标, open(C)），与字节码虚调用及其他直连点共用；单点枢纽数以目标数为界，与 open 类型数无关；
    /// - lambda 与手写层对象接收者（派发规则特殊）经精确集合枢纽（按目标链上次枢纽为父）。
    /// 已接的实现与 open 按（调用点, 目标）记在 `rdirect_done`（与 `hub_linked` 同口径清空），值集增长时只增量接入。
    #[allow(clippy::too_many_arguments)]
    fn direct_virtual(&mut self, m: usize, off: u32, key: &MemberRef, iface: bool, recv: &V, a: &Args, ret: Option<u32>, res: Option<Node>, via: &Via) {
        let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else {
            self.unresolved.insert(key.to_string());
            return;
        };
        let Some(md) = parse_method(&key.desc) else { return };
        let owner = self.id(&key.owner);
        let fs = self.feeds(m, recv, owner);
        let s = self.value_set(&fs);
        let rm = site.method();
        let nonvirt = rm.is_private() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface();
        let exact = TypeSet { classes: s.classes.clone(), open: IdSet::default() };
        let rs = self.receivers(m, &exact, owner);
        let lk = (m, off, key.clone());
        let dk = (off, key.clone());
        let mut done = self.rdirect_done.get_mut(&m).and_then(|d| d.remove(&dk)).unwrap_or_default();
        let (special, plain): (Vec<u32>, Vec<u32>) = rs.into_iter().partition(|r| !nonvirt && (self.lambdas.contains_key(r) || self.hwobjs.contains_key(r)));
        let impls: Vec<MemberRef> = if nonvirt {
            if plain.is_empty() && s.open.is_empty() {
                Vec::new()
            } else {
                let (o, n, d) = site.key();
                vec![MemberRef { owner: o, name: n, desc: d }]
            }
        } else {
            let mut sel = self.rdirect_sel.remove(&lk).unwrap_or_default();
            for r in plain {
                let t = self.ty(r);
                if !sel.0.insert(t) {
                    continue;
                }
                let rname = self.names[t as usize].to_string();
                match self.h.select(&rname, &site) {
                    Some(x) => {
                        let (o, n, d) = x.key();
                        sel.1.insert(MemberRef { owner: o, name: n, desc: d });
                    }
                    None => {
                        self.unresolved.insert(format!("select {rname} {}", site.method().name));
                    }
                }
            }
            let impls = sel.1.iter().cloned().collect();
            self.rdirect_sel.insert(lk.clone(), sel);
            impls
        };
        for k in impls {
            if done.impls.insert(k.clone()) {
                let t = self.method(k, via.clone());
                self.edge(m, off, t, Recv::Feeds(fs.clone()), a, ret, res);
            }
        }
        if !special.is_empty() {
            let rs: Rc<[u32]> = special.into();
            let h = match self.rdirect_last.get(&lk).cloned() {
                Some((h, prev)) if *prev == rs[..] => h,
                last => {
                    let h = self.hub(key, iface, owner, HubSet::Exact(rs.clone()), last.map(|x| x.0), &site, &md, via.clone());
                    self.rdirect_last.insert(lk, (h, rs));
                    h
                }
            };
            self.link_hub(h, m, off, &Vec::new(), res);
            self.direct_hub_args(h, a);
        }
        if !nonvirt {
            let mut grew = false;
            for o in s.open.iter() {
                grew |= done.opens.insert(o);
            }
            if grew {
                let opens: Vec<u32> = done.opens.iter().copied().collect();
                for o in self.direct_open_roots(owner, &opens) {
                    if done.roots.insert(o) {
                        let h = self.hub(key, iface, owner, HubSet::Open(o), None, &site, &md, via.clone());
                        self.link_hub(h, m, off, &Vec::new(), res);
                        self.direct_hub_args(h, a);
                    }
                }
            }
        }
        self.rdirect_done.entry(m).or_default().insert(dk, done);
    }

    /// 直连实例目标（声明类 c）的接收者 open 类型 opens 中各自建枢纽者：有 c 的超类型时即 c 本身（其接收者
    /// G(o) ∩ c = G(c)，其余 open 的贡献都含于其中）；否则只取可能与 c 有公共子类型者（o ⊂ c，或一方是接口；
    /// 两个互不继承的类无公共子类型），再剔除被另一 open 超类型涵盖者
    fn direct_open_roots(&mut self, c: u32, opens: &[u32]) -> Vec<u32> {
        if opens.iter().any(|&o| self.sub(c, o)) {
            return vec![c];
        }
        let c_iface = self.is_iface(c);
        let rel: Vec<u32> = opens.iter().copied().filter(|&o| c_iface || self.is_iface(o) || self.sub(o, c)).collect();
        self.open_roots(&rel)
    }

    /// 实参 a 接到枢纽 h 的形参（不含接收者）
    fn direct_hub_args(&mut self, h: u32, a: &Args) {
        let ptypes = self.hubs[h as usize].ptypes.clone();
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(j)) {
                self.feed(fs, Node::HP(h, j as u16), *pt);
            }
        }
    }

    /// 方法 m 偏移 pc 的调用点按直连处理（从未回退）时的特化入口
    pub(super) fn direct_call_of(&self, m: usize, pc: u32) -> Option<&MemberRef> {
        if self.rdirect_fallback.contains(&(m, pc)) {
            return None;
        }
        self.rdirect.get(&(m, pc))
    }
}

/// 直连调用点上一个实例目标已接的部分（`direct_virtual`）
#[derive(Default)]
pub(super) struct DirectDone {
    /// 已以接收者来源为 this 接边的实现
    impls: BTreeSet<MemberRef>,
    /// 已见的接收者 open 类型
    opens: BTreeSet<u32>,
    /// 已接入枢纽的 open 类型
    roots: BTreeSet<u32>,
}
