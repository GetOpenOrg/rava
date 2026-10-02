//! 引擎：反射调用——按通道的实参池与按接收者派发。
//!
//! 反射方法成员（按名查找 / 枚举入链的方法）只经反射调用执行。调用入口分两条通道，各有实参池 [`Node::RP`]：
//! - 反射对象通道：清单 `method_invokers` 里声明了反射对象形参（Object / Object[] 以外的引用形参，即 Method）的
//!   入口（`invoke0`）。调用点上声明为 Object 的实参（接收者）并入池，声明为 Object[] 的实参数组经 [`Node::RA`]
//!   把元素并入池；
//! - 方法句柄通道：签名多态入口（`method_invokers` 与 `[facts.handle_interpreters]` 的成员）。调用点上的接收者
//!   （句柄本身）与全部实参、解释器手写体的值池（LambdaForm 常量、绑定值、字段 / 内存读取，同手写回调的实参来源）、
//!   经本通道调用的成员的返回值（具名函数的结果是后续具名函数的实参）并入池。
//!
//! 成员经哪条通道调用：枚举得到的方法、按名查找返回反射对象的（`getMethod` / `getDeclaredMethod`）走反射对象通道；
//! 返回其他类型的（方法句柄 / MemberName）走方法句柄通道；反射对象可转成方法句柄（清单 `method_to_handle` 可达）时
//! 反射对象通道的成员另走方法句柄通道。成员的形参按声明类型接其各通道的实参池（形参常量未知）。
//!
//! 实例成员按池中接收者逐个派发（与字节码调用同口径）：可覆写的按接收者选中实现，不可覆写的（私有 / final，如序列化
//! 回调 `writeObject`）就是成员本身；容器对象各进按接收者克隆的上下文，同一成员被多个容器对象反射调用时各对象的
//! 字段 / 数组互不汇合。池中所指未知（open）的接收者：可覆写的退回 VM 枢纽（[`HubSet::Vm`]，目标形参同样接实参池），
//! 不可覆写的进成员本体。lambda / 手写实现对象不是字节码类，反射调用不在字节码层选中实现（同 VM 枢纽）。
//!
//! 去冗余：成员形参接 [`Node::RN`]（池中 open 已涵盖、只能经未知接收者视图读写的值不逐个列出）；可覆写成员退回 VM
//! 枢纽后，枢纽按声明类成员集展开已覆盖逐接收者派发，不再重复派发。池收窄后两者都不触发，结果即逐值派发。

use super::*;

/// 反射调用通道：反射对象 / 方法句柄
pub(super) const RC_OBJ: u8 = 0;
pub(super) const RC_HANDLE: u8 = 1;
const CHANNELS: [u8; 2] = [RC_OBJ, RC_HANDLE];

/// 通道位
pub(super) const fn rc_bit(c: u8) -> u8 {
    1 << c
}

pub(super) fn channel_name(c: u8) -> &'static str {
    if c == RC_OBJ {
        "反射对象"
    } else {
        "方法句柄"
    }
}

/// 反射节点增长钩子（`enum_recv` 登记）：成员枚举的接收者 / 反射调用实参池 / 实参数组
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum RHook {
    Enum(Members),
    Pool,
    Array,
}

/// VM 反射虚调用枢纽的目标形参接法
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum VmBind {
    /// 按声明类型 open（同 VM 入口）
    Open,
    /// 接反射方法成员（序号）各通道的实参池
    Rcall(usize),
}

/// 反射方法成员
pub(super) struct RcallMember {
    pub(super) key: MemberRef,
    iface: bool,
    /// 可覆写：按接收者选中实现；否则实现就是成员本身
    virt: bool,
    /// 经哪些通道调用（通道位集）
    mask: u8,
    /// 已派发的接收者值；已接入的目标（通道扩大时补接）
    done: HashSet<u32>,
    targets: Vec<usize>,
    /// 池中有所指未知的接收者、已退回 VM 枢纽
    fallback: bool,
}

/// 反射调用统计（closure.json `summary.rcall`）
#[derive(Default)]
pub(super) struct RcallStats {
    /// 按池中接收者派发的（成员, 接收者值）数
    pub(super) dispatched: usize,
    /// 退回 VM 枢纽的可覆写成员数 / 进成员本体的所指未知接收者（不可覆写成员）次数
    pub(super) hub_fallbacks: usize,
    pub(super) open_recvs: usize,
    /// 跳过的 lambda / 手写实现对象接收者
    pub(super) synthetic_recvs: usize,
}

impl<'a> Engine<'a> {
    /// 反射对象类型：方法反射调用入口描述符里 Object 以外的引用形参类型（按 Method 调用的入口的 Method）
    fn rcall_obj_type(&self, c: &str) -> bool {
        c != OBJECT
            && self
                .man
                .method_invoker_keys()
                .filter_map(|k| k.split_once(':').and_then(|(_, d)| parse_method(d)))
                .any(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(x) if x == c)))
    }

    /// 描述符 desc 有反射对象形参
    fn rcall_obj_param(&self, desc: &str) -> bool {
        parse_method(desc).is_some_and(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(c) if self.rcall_obj_type(c))))
    }

    /// 按名查找方法（描述符 desc）的结果经哪条通道调用：返回反射对象类型的走反射对象通道，其余走方法句柄通道
    pub(super) fn rcall_lookup_channel(&self, desc: &str) -> u8 {
        let ret = parse_method(desc).and_then(|md| md.ret);
        match ret {
            Some(FieldType::Object(c)) if self.rcall_obj_type(&c) => RC_OBJ,
            _ => RC_HANDLE,
        }
    }

    /// 方法 t 是反射调用入口时的通道
    fn rcall_entry(&mut self, t: usize) -> Option<u8> {
        if let Some(&c) = self.rcall_entries.get(&t) {
            return c;
        }
        let key = self.methods[t].key.clone();
        let c = if self.man.member_invoker(&key.to_string()).contains(&Members::Methods) {
            Some(if self.rcall_obj_param(&key.desc) { RC_OBJ } else { RC_HANDLE })
        } else if self.man.is_handle_interpreter(&key) {
            Some(RC_HANDLE)
        } else {
            None
        };
        self.rcall_entries.insert(t, c);
        c
    }

    /// 反射对象转方法句柄（清单 `method_to_handle`）的来源追踪，在每个字节码调用点 m@off 上：
    /// - 被调方法是转换入口、或其反射对象形参（含接收者）已登记为转换形参：该实参的各来源——本方法内按名查找的结果
    ///   （常量名 + 已知类镜像）→ 该名字另经方法句柄通道调用；本方法形参 → 登记为转换形参，回溯其各调用点；
    ///   其余来源（字段、其他调用的返回值等）推不出所指方法 → 反射对象通道的全部成员另经方法句柄通道调用；
    /// - 被调方法的反射对象形参尚未登记：记下实参来源，登记时补处理
    pub(super) fn rcall_conversion(&mut self, m: usize, off: u32, mref: &MemberRef, opcode: u8, args: &[V]) {
        if self.rcall_m2h {
            return;
        }
        let Some(md) = parse_method(&mref.desc) else { return };
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let mut idx: Vec<u16> = md
            .params
            .iter()
            .enumerate()
            .filter(|(_, p)| matches!(p, FieldType::Object(c) if self.rcall_obj_type(c)))
            .map(|(j, _)| (base + j) as u16)
            .collect();
        if base == 1 && self.rcall_obj_type(&mref.owner) {
            idx.push(0);
        }
        if idx.is_empty() {
            return;
        }
        // 按解析到的声明方法登记（与形参来源所在方法的键同口径）
        let iface = opcode == classfile::op::INVOKEINTERFACE;
        let k = match self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, iface) {
            Some(site) => {
                let (o, n, d) = site.key();
                MemberRef { owner: o, name: n, desc: d }.to_string()
            }
            None => self.mref_key(mref).to_string(),
        };
        let conv = self.man.is_method_to_handle(&k);
        for i in idx {
            let Some(v) = args.get(i as usize) else { continue };
            if conv || self.rcall_conv.contains(&(k.clone(), i)) {
                self.rcall_conv_arg(m, v);
            } else if self.rcall_conv_seen.insert((m, off, i)) {
                self.rcall_conv_pending.entry(k.clone()).or_default().push((m, i, v.clone()));
            }
            if self.rcall_m2h {
                return;
            }
        }
    }

    /// 方法 m 里转成方法句柄的反射对象值 v：按来源定转换的方法（见 [`Self::rcall_conversion`]）
    fn rcall_conv_arg(&mut self, m: usize, v: &V) {
        let srcs = match v {
            V::Null => return,
            V::Ref { .. } => v.srcs(),
            _ => Rc::from([].as_slice()),
        };
        if srcs.is_empty() {
            self.rcall_global_m2h();
            return;
        }
        for s in srcs.iter() {
            match *s {
                Src::Param(j) => {
                    let key = self.methods[m].key.to_string();
                    if self.rcall_conv.insert((key.clone(), j)) {
                        let ps: Vec<(usize, u16, V)> = self.rcall_conv_pending.get(&key).map(|v| v.iter().filter(|e| e.1 == j).cloned().collect()).unwrap_or_default();
                        for (cm, _, cv) in ps {
                            self.rcall_conv_arg(cm, &cv);
                        }
                    }
                }
                Src::Site(o) => {
                    if !self.rcall_conv_lookup(m, o) {
                        self.rcall_global_m2h();
                    }
                }
                _ => self.rcall_global_m2h(),
            }
            if self.rcall_m2h {
                return;
            }
        }
    }

    /// 方法 m 偏移 o 处是返回反射对象的按名查找、名字为常量、查找类已知：该名字另经方法句柄通道调用
    fn rcall_conv_lookup(&mut self, m: usize, o: u32) -> bool {
        let Some(a) = self.analysis(m) else { return false };
        let Some(Event::Invoke { opcode, mref, args, .. }) = class_lookup::event_at(&a, o, class_lookup::is_invoke) else { return false };
        let lk = self.mref_key(mref);
        if !self.man.is_method_lookup(&lk) || self.rcall_lookup_channel(&mref.desc) != RC_OBJ {
            return false;
        }
        let Some(md) = parse_method(&mref.desc) else { return false };
        let base = usize::from(*opcode != classfile::op::INVOKESTATIC);
        let mut name = None;
        let mut classes = BTreeSet::new();
        let mut unknown = false;
        for (j, p) in md.params.iter().enumerate() {
            match p {
                FieldType::Object(c) if c == STRING => match args.get(base + j) {
                    Some(V::Str(s)) => name = Some(s.clone()),
                    _ => return false,
                },
                FieldType::Object(c) if c == CLASS => unknown |= args.get(base + j).is_none_or(|v| self.class_values(m, v, &mut classes)),
                _ => {}
            }
        }
        if base == 1 && mref.owner == CLASS {
            unknown |= args.first().is_none_or(|v| self.class_values(m, v, &mut classes));
        }
        let Some(name) = name else { return false };
        if unknown || classes.is_empty() {
            return false;
        }
        for c in classes {
            self.reflect_name(&c, &name, RC_HANDLE);
        }
        true
    }

    /// 转成方法句柄的反射对象推不出所指方法：反射对象通道的全部成员另经方法句柄通道调用
    fn rcall_global_m2h(&mut self) {
        if std::mem::replace(&mut self.rcall_m2h, true) {
            return;
        }
        self.rcall_conv_pending.clear();
        for i in 0..self.rcall_members.len() {
            if self.rcall_members[i].mask & rc_bit(RC_OBJ) != 0 {
                self.rcall_widen(i, rc_bit(RC_HANDLE));
            }
        }
    }

    /// 调用边 m@off → t（手写）：t 是反射调用入口时，接收者 / 实参接入其通道的实参池
    pub(super) fn rcall_site(&mut self, m: usize, off: u32, t: usize, recv: Option<&[Feed]>, a: &[Option<Vec<Feed>>]) {
        let Some(ch) = self.rcall_entry(t) else { return };
        self.rcall_sites.insert((m, off, t));
        let obj = self.id(OBJECT);
        let objs = self.id(&format!("[L{OBJECT};"));
        let pool = Node::RP(ch);
        self.rcall_hooks(ch);
        if self.methods[t].kind != Kind::Bytecode && ch == RC_HANDLE && self.rcall_pooled.insert(t) {
            // 句柄解释器的值池：解释执行时交给具名函数的值（同手写回调的实参来源）
            self.flow(Node::S(t, POOL), pool, obj);
        }
        let key = &self.methods[t].key;
        let poly = self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(resolve::is_signature_polymorphic)).unwrap_or(false);
        if poly {
            // 签名多态：实参按调用点描述符排布，句柄本身与全部引用实参都是解释执行的输入
            for fs in recv.into_iter().chain(a.iter().flatten().map(Vec::as_slice)) {
                self.feed(fs, pool, obj);
            }
            return;
        }
        let base = usize::from(!self.methods[t].is_static);
        let ptypes = self.methods[t].ptypes.clone();
        for (j, f) in a.iter().enumerate() {
            let (Some(fs), Some(Some(pt))) = (f, ptypes.get(base + j)) else { continue };
            if *pt == obj {
                self.feed(fs, pool, obj);
            } else if *pt == objs {
                self.feed(fs, Node::RA(ch), objs);
            }
        }
    }

    /// 通道 ch 的实参池 / 实参数组节点登记增长钩子（一次）
    fn rcall_hooks(&mut self, ch: u8) {
        for (n, k) in [(Node::RP(ch), RHook::Pool), (Node::RA(ch), RHook::Array)] {
            if self.enum_recv.insert(n, (k, ch as usize)).is_none() {
                let s = self.set_of(n);
                if !s.is_empty() {
                    self.rpending.push((k, ch as usize, s));
                }
            }
        }
    }

    /// 实参数组新增值：数组的元素（分配点元素节点；open 数组按元素类型 open）并入该通道的实参池
    pub(super) fn rcall_array_grown(&mut self, ch: u8, delta: &TypeSet) {
        let pool = Node::RP(ch);
        let obj = self.id(OBJECT);
        let xs: Vec<u32> = delta.classes.iter().filter(|x| self.arrays.contains_key(x)).collect();
        for x in xs {
            for p in PARITIES {
                self.flow(Node::E(x, p), pool, obj);
            }
        }
        let mut comps = TypeSet::default();
        for o in delta.open.iter() {
            let name = self.names[o as usize].clone();
            if let Some(c) = absint::component(&name) {
                comps.open.insert(self.id(&c));
            }
        }
        if !comps.is_empty() {
            self.add_to(pool, &comps);
        }
    }

    /// 实参池新增值：经该通道调用的实例成员按新增接收者派发
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

    /// 池增量 delta 去掉池中 open 已涵盖的值：x 属于池中某 open 类型，且 x 不是 lambda / 手写实现对象，抽象对象 / 数组
    /// 分配点须已逃逸（未逃逸的只经字节码可见引用读写，open 视图碰不到它）。之后才逃逸的值已逐个列出，不受影响
    fn rcall_absorb(&mut self, ch: u8, delta: &TypeSet) -> TypeSet {
        let opens: Vec<u32> = self.graph.get(&Node::RP(ch)).map(|s| s.open.iter().collect()).unwrap_or_default();
        if opens.is_empty() || delta.classes.is_empty() {
            return delta.clone();
        }
        let mut keep = IdSet::default();
        for x in delta.classes.iter() {
            let synthetic = self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x);
            let hidden = (self.objs.contains_key(&x) || self.arrays.contains_key(&x)) && !self.escaped.contains(&x);
            if synthetic || hidden || !opens.iter().any(|&o| self.sub(x, o)) {
                keep.insert(x);
            }
        }
        TypeSet { classes: keep, open: delta.open.clone() }
    }

    /// 反射方法成员 key 经通道位集 mask 入链（重复入链时并入新通道）
    pub(super) fn rcall_member(&mut self, key: MemberRef, iface: bool, virt: bool, mask: u8) {
        let mask = if self.rcall_m2h && mask & rc_bit(RC_OBJ) != 0 { mask | rc_bit(RC_HANDLE) } else { mask };
        let i = match self.rcall_ix.get(&key) {
            Some(&i) => i,
            None => {
                let i = self.rcall_members.len();
                self.rcall_ix.insert(key.clone(), i);
                self.rcall_members.push(RcallMember { key, iface, virt, mask: 0, done: HashSet::default(), targets: vec![], fallback: false });
                i
            }
        };
        self.rcall_widen(i, mask);
    }

    /// 成员 i 的通道并入 bits：成员本体（静态 / 声明键）与已接目标补接新通道的实参池，按新通道池中现有接收者派发
    fn rcall_widen(&mut self, i: usize, bits: u8) {
        let new = bits & !self.rcall_members[i].mask;
        if new == 0 {
            return;
        }
        self.rcall_members[i].mask |= new;
        let mask = self.rcall_members[i].mask;
        let key = self.rcall_members[i].key.clone();
        let via = Via::class("reflect", &key.owner);
        // 成员本体（反射分派表按声明键引用）：实例成员的接收者只经派发进入
        let t = self.method(key, via);
        self.rcall_bind(t, mask);
        for j in 0..self.rcall_members[i].targets.len() {
            let t = self.rcall_members[i].targets[j];
            self.rcall_bind(t, mask);
        }
        if self.methods[t].is_static {
            return;
        }
        for ch in CHANNELS {
            if new & rc_bit(ch) != 0 {
                self.rcall_hooks(ch);
                let s = self.set_of(Node::RP(ch));
                self.rcall_dispatch(i, &s);
            }
        }
    }

    /// 目标 t 的形参（不含接收者）接 mask 各通道的实参池（形参常量为未知）；经方法句柄通道调用的返回值并入其池
    fn rcall_bind(&mut self, t: usize, mask: u8) {
        let old = self.rcall_bound.get(&t).copied().unwrap_or(0);
        let new = mask & !old;
        if new == 0 {
            return;
        }
        self.rcall_bound.insert(t, old | new);
        let base = usize::from(!self.methods[t].is_static);
        let pts = self.methods[t].ptypes.clone();
        if old == 0 {
            self.bind_pvs(t, base, pts.len(), None);
        }
        let obj = self.id(OBJECT);
        // 内存效果由清单声明的成员（[facts.array_writes] / [facts.memory_reads]）经方法句柄通道调用时，其读写由解释器
        // 手写体调用点按「DMH 所指字段」建模（hw_mem Gate::Handle）；再把值池接到其形参，会让本体的按偏移读写臂
        // 把值池泛写进值池里的每个对象，故方法句柄通道不接这类成员的形参与返回
        let ks = self.methods[t].key.to_string();
        let effect = self.man.array_writes(&ks).is_some() || self.man.memory_read(&ks).is_some();
        for ch in CHANNELS {
            if new & rc_bit(ch) == 0 || (ch == RC_HANDLE && effect) {
                continue;
            }
            for (i, pt) in pts.iter().enumerate().skip(base) {
                if let Some(pt) = pt {
                    self.flow(Node::RN(ch), Node::P(t, i as u16), *pt);
                }
            }
            if ch == RC_HANDLE && self.methods[t].rtype.is_some() {
                self.flow(Node::R(t), Node::RP(ch), obj);
            }
        }
    }

    /// 成员 i 按接收者值集 s 派发（只处理新的接收者值）
    fn rcall_dispatch(&mut self, i: usize, s: &TypeSet) {
        let (key, iface, virt, mask) = {
            let m = &self.rcall_members[i];
            (m.key.clone(), m.iface, m.virt, m.mask)
        };
        // 已退回 VM 枢纽：枢纽按成员集展开到声明类的全部成员（同样按接收者克隆上下文、P0 = exact、形参接池），
        // 逐接收者派发只剩枢纽不展开的未逃逸数组分配点；open 已由枢纽承接
        let covered = virt && self.rcall_members[i].fallback;
        if covered && !s.classes.iter().any(|x| self.arrays.contains_key(&x) && !self.escaped.contains(&x)) {
            return;
        }
        let owner = self.id(&key.owner);
        let via = Via::class("reflect", &key.owner);
        let opens: Vec<u32> = s.open.iter().filter(|&o| self.sub(o, owner) || self.sub(owner, o)).collect();
        if !opens.is_empty() {
            if virt {
                if !std::mem::replace(&mut self.rcall_members[i].fallback, true) {
                    self.rcall_stats.hub_fallbacks += 1;
                    self.vm_dispatch(&key, iface, via.clone(), VmBind::Rcall(i));
                }
            } else {
                let o = TypeSet { classes: IdSet::default(), open: IdSet::from_sorted(opens) };
                let o = self.filter(&o, owner);
                if !o.is_empty() {
                    self.rcall_stats.open_recvs += 1;
                    let t = self.method(key.clone(), via.clone());
                    self.add_to(Node::P(t, 0), &o);
                }
            }
        }
        let site = if virt { self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) } else { None };
        if virt && site.is_none() {
            self.unresolved.insert(key.to_string());
            return;
        }
        let covered = virt && self.rcall_members[i].fallback;
        let xs: Vec<u32> = s.classes.iter().filter(|x| !covered || (self.arrays.contains_key(x) && !self.escaped.contains(x))).collect();
        for x in xs {
            if !self.sub(x, owner) || !self.rcall_members[i].done.insert(x) {
                continue;
            }
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                self.rcall_stats.synthetic_recvs += 1;
                continue;
            }
            let k = match &site {
                Some(site) => {
                    let rt = self.ty(x);
                    let rname = self.names[rt as usize].to_string();
                    let Some(sel) = self.h.select(&rname, site) else {
                        self.unresolved.insert(format!("select {rname} {}", key.name));
                        continue;
                    };
                    let (o, n, d) = sel.key();
                    MemberRef { owner: o, name: n, desc: d }
                }
                None => key.clone(),
            };
            self.rcall_stats.dispatched += 1;
            let cx = self.recv_ctx(x);
            let t = self.method_ctx(k, cx, via.clone());
            if virt {
                self.vm_targets.insert(t);
            }
            self.add_to(Node::P(t, 0), &TypeSet::exact(x));
            self.rcall_members[i].targets.push(t);
            self.rcall_bind(t, mask);
        }
    }

    /// VM 枢纽 h 的目标接法并入 bind；已选中的目标补接
    pub(super) fn vm_hub_bind(&mut self, h: u32, bind: VmBind) {
        let fresh = match bind {
            VmBind::Open => self.vm_open_hubs.insert(h),
            VmBind::Rcall(i) => self.vm_rcall_hubs.insert(h, i) != Some(i),
        };
        if !fresh {
            return;
        }
        for t in self.vm_hub_targets.get(&h).cloned().unwrap_or_default() {
            self.vm_bind_target(t, bind);
        }
    }

    /// VM 枢纽 h 选中目标 t：按枢纽的各接法接形参
    pub(super) fn vm_hub_target(&mut self, h: u32, t: usize) {
        self.vm_hub_targets.entry(h).or_default().push(t);
        if self.vm_open_hubs.contains(&h) {
            self.vm_bind_target(t, VmBind::Open);
        }
        if let Some(&i) = self.vm_rcall_hubs.get(&h) {
            self.vm_bind_target(t, VmBind::Rcall(i));
        }
    }

    fn vm_bind_target(&mut self, t: usize, bind: VmBind) {
        match bind {
            VmBind::Open => self.open_params(t),
            VmBind::Rcall(i) => {
                self.rcall_members[i].targets.push(t);
                let mask = self.rcall_members[i].mask;
                self.rcall_bind(t, mask);
            }
        }
    }

    /// 反射调用统计
    pub fn rcall_summary(&self) -> serde_json::Value {
        let size = |n: Node| self.graph.get(&n).map_or((0, 0), |s| (s.classes.len(), s.open.len()));
        let chans: Vec<serde_json::Value> = CHANNELS
            .iter()
            .map(|&c| {
                let (p, po) = size(Node::RP(c));
                let members = self.rcall_members.iter().filter(|m| m.mask & rc_bit(c) != 0).count();
                serde_json::json!({"channel": channel_name(c), "members": members, "pool": p, "pool_open": po,
                    "targets": self.rcall_bound.values().filter(|&&b| b & rc_bit(c) != 0).count()})
            })
            .collect();
        serde_json::json!({
            "members": self.rcall_members.len(),
            "sites": self.rcall_sites.len(),
            "method_to_handle": self.rcall_m2h,
            "channels": chans,
            "dispatched": self.rcall_stats.dispatched,
            "hub_fallbacks": self.rcall_stats.hub_fallbacks,
            "open_recvs": self.rcall_stats.open_recvs,
            "synthetic_recvs": self.rcall_stats.synthetic_recvs,
        })
    }
}
