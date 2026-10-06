//! 引擎：内存访问中继方法的上下文（计划 c1d §30 B2 / B6：容器元素出口按对象）。
//!
//! 内存访问中继方法：字节码方法，其引用形参（直接或经下游中继方法）作为实参流到清单声明的手写内存访问成员的
//! 内存槽——`[facts.memory_reads]` 的 `src`（读其元素 / 引用字段）、`[facts.array_writes]` 的 `dst` / `values` /
//! `elements`（写入目标与写入值）。典型形态是 Unsafe 访问序变体（acquire / release / opaque 读写按字节码翻译，
//! 经 native 读写）：接收者是单例、不是抽象对象，按接收者选上下文时进方法本体，手写调用点在本体里只有一个——
//! 全部调用方的数组 / 对象在这一个调用点上汇合，容器 A 的元素读出容器 B 的元素（`ConcurrentHashMap.tabAt` 读出
//! 全部 map 的节点，节点值再经未知接收者视图读出全部 map 的值）。
//!
//! 策略：调用方已在某个上下文（容器对象 / 调用点）中、接收者不是抽象对象时，中继方法继承调用方上下文——
//! 与静态辅助方法（`ctxsel.rs` 规则 1，`tabAt` / `casTabAt` 自身）同一口径，手写调用点随之按上下文分开。
//! 调用方上下文无关时仍进本体（不新增上下文）。判定只看字节码结构与清单的内存槽声明，不按类名。
//!
//! 判定是字节码调用图上的最小不动点（单调：被调方槽多则调用方槽只多不少），按自 key 可达的未定成员集一次求解后
//! 整体缓存——唯一解，与查询顺序、成环形态无关（D1）；每个成员的字节码只分析一次。
//!
//! 健全性：克隆只把一个方法节点拆成按上下文的若干节点，每个克隆的形参取自其调用方的实参，全部克隆之并等于本体
//! 原来的值集；手写调用点的读写按克隆内的实参接入，读出的仍是该实参所指对象的元素 / 字段全集——只细分，不丢值。

use super::*;

type SlotMask = u64;

fn bit(i: usize) -> SlotMask {
    if i < 64 {
        1 << i
    } else {
        0
    }
}

/// 一条调用边的被调方：字节码方法（按不动点求槽）或手写内存访问成员（槽由清单给定）
#[derive(Clone)]
enum Target {
    Code(MemberRef),
    Mem(SlotMask),
}

/// 一条调用边：被调方 + 各实参槽携带的本方法形参槽（只记携带形参的实参）
type Edge = (Target, Vec<(usize, SlotMask)>);

/// 中继判定缓存：已定成员的槽（含 this 位），与各成员的调用边（纯字节码事实）
#[derive(Default)]
pub(super) struct RelayCache {
    done: HashMap<MemberRef, SlotMask>,
    edges: HashMap<MemberRef, Rc<Vec<Edge>>>,
}

impl<'a> Engine<'a> {
    /// 非对象接收者调用实例方法 key 时的上下文：调用方在上下文中、key 是内存访问中继方法则继承之，否则本体
    pub(super) fn relay_ctx(&mut self, m: usize, key: &MemberRef) -> u32 {
        let caller = self.methods[m].ctx;
        // 具体求值上下文不外传（其方法按轨迹处理，不是堆上下文）
        if caller == NOCTX || self.methods[m].kind != Kind::Bytecode || self.is_concrete(m) {
            return NOCTX;
        }
        if self.relay_slots(key) != 0 {
            caller
        } else {
            NOCTX
        }
    }

    /// 流到内存槽的形参槽（实例方法 0 = this）；只经接收者（单例）流到内存槽的不算中继——接收者不是被读写的对象
    pub(super) fn relay_slots(&mut self, key: &MemberRef) -> SlotMask {
        let raw = self.relay_raw(key);
        match self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(|m| m.is_static())) {
            Some(false) => raw & !1,
            _ => raw,
        }
    }

    /// 含 this 位的槽：自 key 可达的未定成员集上求最小不动点，整体入缓存
    fn relay_raw(&mut self, key: &MemberRef) -> SlotMask {
        if let Some(&r) = self.relays.done.get(key) {
            return r;
        }
        // 可达未定成员集（按发现序；不动点与序无关）
        let mut set: Vec<MemberRef> = vec![key.clone()];
        let mut seen: HashSet<MemberRef> = HashSet::default();
        seen.insert(key.clone());
        let mut i = 0;
        while i < set.len() {
            let es = self.relay_edges(&set[i].clone());
            for (t, _) in es.iter() {
                if let Target::Code(c) = t {
                    if !self.relays.done.contains_key(c) && seen.insert(c.clone()) {
                        set.push(c.clone());
                    }
                }
            }
            i += 1;
        }
        let mut cur: HashMap<MemberRef, SlotMask> = set.iter().map(|k| (k.clone(), 0)).collect();
        loop {
            let mut changed = false;
            for k in &set {
                let es = self.relay_edges(k);
                let mut out = cur[k];
                for (t, args) in es.iter() {
                    let cm = match t {
                        Target::Mem(s) => *s,
                        Target::Code(c) => self.relays.done.get(c).or_else(|| cur.get(c)).copied().unwrap_or(0),
                    };
                    for &(j, pm) in args {
                        if cm & bit(j) != 0 {
                            out |= pm;
                        }
                    }
                }
                if out != cur[k] {
                    cur.insert(k.clone(), out);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let r = cur[key];
        self.relays.done.extend(cur);
        r
    }

    /// 方法 key 的调用边（缓存）：非字节码 / 无引用形参 / 保守分析的方法无边
    fn relay_edges(&mut self, key: &MemberRef) -> Rc<Vec<Edge>> {
        if let Some(e) = self.relays.edges.get(key) {
            return e.clone();
        }
        let e = Rc::new(self.relay_edges_uncached(key));
        self.relays.edges.insert(key.clone(), e.clone());
        e
    }

    /// 被调方 callee（已解析）：字节码方法进不动点，手写内存访问成员按清单声明的内存槽（按接收者偏移）
    fn relay_target(&self, callee: MemberRef) -> Option<Target> {
        let cf = self.h.class(&callee.owner)?;
        let meth = cf.method(&callee.name, &callee.desc)?;
        let base = usize::from(!meth.is_static());
        if self.kind_of(&cf, meth) == Kind::Bytecode {
            return Some(Target::Code(callee));
        }
        let k = callee.to_string();
        let mut s = 0;
        if let Some(i) = self.man.memory_read(&k) {
            s |= bit(i + base);
        }
        if let Some(w) = self.man.array_writes(&k) {
            for i in w.dst.iter().chain(&w.values).chain(&w.elements) {
                s |= bit(i + base);
            }
        }
        (s != 0).then_some(Target::Mem(s))
    }

    fn relay_edges_uncached(&self, key: &MemberRef) -> Vec<Edge> {
        let Some(cf) = self.h.class(&key.owner) else { return vec![] };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return vec![] };
        if self.kind_of(&cf, meth) != Kind::Bytecode {
            return vec![];
        }
        let Some(md) = parse_method(&key.desc) else { return vec![] };
        if !md.params.iter().any(|p| p.is_reference()) {
            return vec![];
        }
        let Some(code) = meth.code.as_ref() else { return vec![] };
        let live = |_: &str| true;
        let a = self.ctx.aux_analyze(&key.owner, &key.desc, meth.is_static(), code, &Facts { ctx: &self.ctx, live: &live, m: None, params: vec![], mirrors: vec![] });
        if a.conservative {
            return vec![];
        }
        let mut out = Vec::new();
        for (_, e) in &a.events {
            let Event::Invoke { mref, iface, args, .. } = e else { continue };
            // 只记携带本方法形参的实参
            let mut carried: Vec<(usize, SlotMask)> = Vec::new();
            for (j, v) in args.iter().enumerate() {
                if let V::Ref { src, .. } = v {
                    let pm = src.iter().fold(0, |acc, x| if let Src::Param(i) = *x { acc | bit(i as usize) } else { acc });
                    if pm != 0 {
                        carried.push((j, pm));
                    }
                }
            }
            if carried.is_empty() {
                continue;
            }
            let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) else { continue };
            let (o, n, d) = site.key();
            let callee = MemberRef { owner: o, name: n, desc: d };
            if let Some(t) = self.relay_target(callee) {
                out.push((t, carried));
            }
        }
        out
    }
}
