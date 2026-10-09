//! 引擎：反射调用实参池的去冗余与按接收者派发（`reflect_call.rs` 的池增长后半段）。
//!
//! 两类判定都依赖「谁先到达」：池值是否被池中 open 涵盖（进不进去冗余视图 [`Node::RN`]）、可覆写成员的接收者是否被
//! 退回的 VM 枢纽涵盖（要不要逐个派发）。涵盖条件随分析只增不减，判定若在到达时做，结果取决于求值次序（散列种子）。
//! 所以此刻已涵盖的永久略去，尚未涵盖的挂起，到工作队列排空（单调部分的不动点）时由 [`Engine::rcall_release`] 定夺。

use super::reflect_call::{rc_bit, CHANNELS, VmBind};
use super::*;

impl Engine<'_> {
    /// 实参池新增值：经该通道调用的实例成员按新增接收者派发；去冗余视图按 [`Self::rcall_absorb`] 取值
    pub(super) fn rcall_pool_grown(&mut self, ch: u8, delta: &TypeSet) {
        let lean = self.rcall_absorb(ch, delta);
        if !lean.is_empty() {
            self.add_to(Node::RN(ch), &lean);
        }
        for i in 0..self.rcall_members.len() {
            if self.rcall_members[i].mask & rc_bit(ch) != 0 {
                self.rcall_dispatch(i, delta);
            }
        }
    }

    /// 值 x 已被通道 ch 池中的 open 涵盖：x 不是 lambda / 手写实现对象，抽象对象 / 数组分配点须已逃逸（未逃逸的只经
    /// 字节码可见引用读写，open 视图碰不到它），且属于池中某 open 类型。三个条件随分析只增不减，一旦成立永远成立
    pub(super) fn rcall_covered(&mut self, x: u32, opens: &[u32]) -> bool {
        let synthetic = self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x);
        let hidden = (self.objs.contains_key(&x) || self.arrays.contains_key(&x)) && !self.escaped.contains(&x);
        !synthetic && !hidden && opens.iter().any(|&o| self.sub(x, o))
    }

    fn rcall_opens(&self, ch: u8) -> Vec<u32> {
        self.graph.get(&Node::RP(ch)).map(|s| s.open.iter().collect()).unwrap_or_default()
    }

    /// 池增量 delta 中立即进去冗余视图的部分：open 与 lambda / 手写实现对象（永不被涵盖）。其余值此刻已被涵盖的
    /// 永久略去；尚未被涵盖的挂起，到工作队列排空时由 [`Self::rcall_release`] 定夺。
    ///
    /// 「x 逐个列出」与「x 被 open 涵盖」的下游效果并不等价（未收窄的按偏移写入对逐个列出的对象写其全部引用字段，
    /// 对 open 目标只写 open 类型自身的字段），所以 x 是否列出不能取决于 x 与涵盖它的 open 谁先入池、x 何时逃逸：
    /// 判定只在不动点上做，与 `class_lookup.rs::lookup_release` 同一口径
    fn rcall_absorb(&mut self, ch: u8, delta: &TypeSet) -> TypeSet {
        let opens = self.rcall_opens(ch);
        let mut keep = IdSet::default();
        let mut wait = Vec::new();
        for x in delta.classes.iter() {
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                keep.insert(x);
            } else if self.rcall_covered(x, &opens) {
                self.rcall_stats.absorbed += 1;
            } else {
                wait.push(x);
            }
        }
        self.rcall_rn_pending[ch as usize].extend(wait);
        TypeSet { classes: keep, open: delta.open.clone() }
    }

    /// 工作队列排空：各通道挂起的池值按此刻的池与逃逸集判定——已被涵盖的略去，其余并入去冗余视图。
    /// 排空时的状态是单调部分的不动点，与求值先后无关；放行后的新增值照常挂起、下一次排空再判
    pub(super) fn rcall_release(&mut self) -> bool {
        let mut any = self.rcall_release_recvs();
        for ch in CHANNELS {
            let wait = std::mem::take(&mut self.rcall_rn_pending[ch as usize]);
            if wait.is_empty() {
                continue;
            }
            let opens = self.rcall_opens(ch);
            let mut keep = IdSet::default();
            for x in wait.iter() {
                if self.rcall_covered(x, &opens) {
                    self.rcall_stats.absorbed += 1;
                } else {
                    keep.insert(x);
                }
            }
            if !keep.is_empty() {
                self.rcall_stats.released += keep.len();
                self.add_to(Node::RN(ch), &TypeSet { classes: keep, open: IdSet::default() });
                any = true;
            }
        }
        if any {
            self.rcall_stats.release_rounds += 1;
        }
        any
    }

    /// 成员 i 按接收者值集 s 派发（只处理新的接收者值）。
    ///
    /// 可覆写成员退回 VM 枢纽（池中有所指未知的接收者）后，枢纽按声明类成员集展开（同样按接收者克隆上下文、
    /// P0 = exact、形参接池），已涵盖逐接收者派发，只剩枢纽不展开的未逃逸数组分配点要逐个派发。退回与逃逸都随分析
    /// 只增不减，接收者在退回前到达就逐个派发、退回后到达就略去，会使派发集合取决于到达先后：所以可覆写成员的
    /// 接收者（此刻尚未被枢纽涵盖的）挂起，到工作队列排空时由 [`Self::rcall_release`] 按当时的退回状态定夺
    pub(super) fn rcall_dispatch(&mut self, i: usize, s: &TypeSet) {
        let (key, iface, virt) = {
            let m = &self.rcall_members[i];
            (m.key.clone(), m.iface, m.virt)
        };
        if self.rcall_members[i].fallback && !s.classes.iter().any(|x| self.rcall_hub_misses(x)) {
            return;
        }
        // 静态成员没有接收者：池值只作实参（`rcall_bind`）
        if self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(|mm| mm.is_static())).unwrap_or(false) {
            return;
        }
        let owner = self.id(&key.owner);
        let via = Via::class("reflect", &key.owner);
        let opens: Vec<u32> = s.open.iter().filter(|&o| self.sub(o, owner) || self.sub(owner, o)).collect();
        // 所指未知的接收者退回 VM 枢纽：按 G 中 ⊂ 声明类的成员逐个进其接收者上下文（不可覆写成员的目标固定为
        // 成员本身）。不以 open 接收者进方法本体——本体按 open 汇合全部逃逸对象的字段，写回时流向每个逃逸对象
        if !opens.is_empty() && !std::mem::replace(&mut self.rcall_members[i].fallback, true) {
            self.rcall_stats.hub_fallbacks += 1;
            if !virt {
                self.rcall_stats.open_recvs += 1;
            }
            self.vm_dispatch(&key, iface, virt, via.clone(), VmBind::Rcall(i));
        }
        let fallback = self.rcall_members[i].fallback;
        let xs: Vec<u32> = s.classes.iter().filter(|&x| !fallback || self.rcall_hub_misses(x)).collect();
        let mut wait = false;
        for x in xs {
            if !self.sub(x, owner) || !self.rcall_members[i].done.insert(x) {
                continue;
            }
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                self.rcall_stats.synthetic_recvs += 1;
                continue;
            }
            if !self.rcall_hub_misses(x) {
                // 枢纽可能涵盖：挂起（`done` 已记，排空时取出判定）
                self.rcall_members[i].wait.push(x);
                wait = true;
                continue;
            }
            self.rcall_dispatch_recv(i, x);
        }
        if wait && !std::mem::replace(&mut self.rcall_members[i].waiting, true) {
            self.rcall_wait_members.push(i);
        }
    }

    /// 枢纽展开不到的接收者：未逃逸的数组分配点（逐个派发的唯一来源）
    fn rcall_hub_misses(&self, x: u32) -> bool {
        self.arrays.contains_key(&x) && !self.escaped.contains(&x)
    }

    /// 成员 i 对接收者 x 逐个派发：按接收者选中实现，进按接收者克隆的上下文，P0 = exact(x)，形参接实参池
    fn rcall_dispatch_recv(&mut self, i: usize, x: u32) {
        let (key, iface, virt, mask) = {
            let m = &self.rcall_members[i];
            (m.key.clone(), m.iface, m.virt, m.mask)
        };
        let k = if virt {
            let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else {
                self.unresolved.insert(key.to_string());
                return;
            };
            let rt = self.ty(x);
            let rname = self.names[rt as usize].to_string();
            let Some(sel) = self.h.select(&rname, &site) else {
                self.unresolved.insert(format!("select {rname} {}", key.name));
                return;
            };
            let (o, n, d) = sel.key();
            MemberRef { owner: o, name: n, desc: d }
        } else {
            key.clone()
        };
        self.rcall_stats.dispatched += 1;
        let via = Via::class("reflect", &key.owner);
        let cx = self.recv_ctx(x);
        let t = self.method_ctx(k, cx, via);
        if virt {
            self.vm_targets.insert(t);
        }
        self.add_to(Node::P(t, 0), &TypeSet::exact(x));
        self.rcall_members[i].targets.push(t);
        self.rcall_bind(t, mask);
    }

    /// 排空时：挂起的接收者按成员此刻的退回状态定夺——已退回的由枢纽涵盖，其余逐个派发
    fn rcall_release_recvs(&mut self) -> bool {
        let mut ms = std::mem::take(&mut self.rcall_wait_members);
        ms.sort_unstable();
        let mut any = false;
        for i in ms {
            self.rcall_members[i].waiting = false;
            let xs = std::mem::take(&mut self.rcall_members[i].wait);
            if self.rcall_members[i].fallback {
                continue;
            }
            for x in xs {
                self.rcall_dispatch_recv(i, x);
                any = true;
            }
        }
        any
    }
}
