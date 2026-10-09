//! 引擎：调用——静态 / 虚调用接边、形参绑定（反射式写入见 `reflect_writes.rs`）。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        if self.jca_order_hold(m, off, mref) {
            return;
        }
        self.prof_seg(site_prof::SEG_PRE);
        self.note_ref(mref);
        self.reflective_writes(m, off, mref, opcode, args);
        self.service_lookup(m, off, opcode, mref, args);
        self.prop_key_site(m, off, opcode, mref, iface, args);
        // 直连反射调用（`reflect_direct.rs`）：调用点已接特化入口与各目标，不再按原入口接边
        if self.reflect_direct(m, off, opcode, mref, args) {
            return;
        }
        let pargs = if opcode == classfile::op::INVOKESTATIC { args } else { args.get(1..).unwrap_or(&[]) };
        self.call_vals = Some(Rc::from(pargs));
        let lambda_cap = self.lambda_cap.take();
        let wrapped = self.ref_caller_sensitive(mref);
        let outer = std::mem::replace(&mut self.cs.site_wrapped, wrapped);
        let lambda = self.cs.lambda_site.take();
        self.prof_seg(site_prof::SEG_ARGS);
        self.invoke_inner(m, off, opcode, mref, iface, args);
        self.prof_seg(site_prof::SEG_POST);
        self.lookup_wrap_call(m, off, args);
        self.cs.lambda_site = lambda;
        self.cs.site_wrapped = outer;
        self.call_vals = None;
        self.lambda_cap = lambda_cap;
    }

    pub(super) fn invoke_inner(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        use classfile::op;
        let via = Via::method("invoke", m, Some(off));
        // 静态 / 特殊调用按属主类发射（`X::m(..)` / 固有方法），属主至少 L2；
        // 虚 / 接口调用的属主可停在 L1：其值只可能是 null，调用点导出为 null_recv（`fold.rs`）
        let lvl = if opcode == op::INVOKESTATIC || opcode == op::INVOKESPECIAL { Level::Layout } else { Level::Type };
        self.touch(&mref.owner, lvl, via.clone());
        let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, iface) else {
            self.unresolved.insert(mref.to_string());
            return;
        };
        if resolve::is_signature_polymorphic(site.method()) {
            self.sigpoly_sites.insert(format!("{}@{off}", self.methods[m].key));
        }
        let (o, n, d) = site.key();
        self.nest_access(m, off, &o, site.method().is_private());
        let resolved = MemberRef { owner: o, name: n, desc: d };
        let Some(md) = parse_method(&mref.desc) else { return };
        let owner = self.id(&mref.owner);
        let is_static = opcode == op::INVOKESTATIC;
        let recv_v = if is_static { None } else { args.first() };
        let pargs = if is_static { args } else { args.get(1..).unwrap_or(&[]) };
        let mut a: Args = Vec::with_capacity(md.params.len());
        for (p, v) in md.params.iter().zip(pargs.iter()) {
            let f = self.ptype(p).map(|t| self.feeds(m, v, t));
            a.push(f);
        }
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        // 反射对象查找 / 复制入口的结果按标记建模（`method_marks.rs`）时不接被调方返回值；
        // 按键查找入口的结果先经闸门（`keyed.rs`）
        let res = if self.method_marks_site(m, off, opcode, mref, args) { None } else { Some(self.keyed_res(m, off, &resolved, pargs)) };
        let recv_feeds = |e: &mut Self| match recv_v {
            Some(v) => e.feeds(m, v, owner),
            None => vec![Feed::S(TypeSet::open(owner))],
        };
        match opcode {
            op::INVOKESTATIC => {
                self.prof_path(site_prof::PATH_STATIC);
                self.prof_role(&[], &a);
                self.prof_seg(site_prof::SEG_STATIC);
                self.init(&resolved.owner, via.clone());
                if self.concrete_call(m, off, &resolved, &md, None, pargs) {
                    return;
                }
                // 克隆上下文的选择见 `ctxsel.rs`
                let heap = md.ret.iter().chain(&md.params).any(|r| r.is_reference());
                let ctx = self.static_ctx(m, off, &resolved, Call::Invoke { heap, args: pargs });
                // 按名取类：名字能由常量拼出时结果只含所指类的镜像，不再接被调方法返回的所指未知的 Class
                // 按名加载（class_loads）同样解析，只取镜像不初始化
                let key = self.mref_key(mref);
                let load = self.man.names.is_class_load(&key);
                let (named, top) = if load || self.man.names.is_class_lookup(&key) { self.class_lookup(m, off, args) } else { (vec![], true) };
                let t = self.method_ctx(resolved, ctx, via);
                self.site_linked(m, off, t);
                self.edge(m, off, t, Recv::None, &a, ret, if top { res } else { None });
                for c in named {
                    self.named_class(m, off, &c, !load);
                }
            }
            op::INVOKESPECIAL => {
                if resolved.name == "<init>" {
                    self.keyed_ctor(m, &mref.owner, &mref.desc, recv_v, pargs);
                }
                let r = recv_feeds(self);
                self.prof_path(site_prof::PATH_SPECIAL);
                self.prof_role(&r, &a);
                self.prof_seg(site_prof::SEG_EDGE_RECV);
                self.edge_recv(m, off, resolved, via, r, &a, ret, res, true);
            }
            _ => {
                let rm = site.method();
                // 数组类型上的调用（`arr.clone()` 等）同样非虚：数组没有覆盖方法，目标恒为已解析的继承方法。
                // 若经枢纽派发，手写层 / VM 产出的 open 数组没有分配点可展开，结果（clone 的副本）会丢失
                if rm.is_private() || rm.is_static() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface() || is_array_type(&mref.owner) {
                    // 非虚：直接到已解析方法，接收者值流入 this。非 private 的目标在生成代码里仍经槽调用，计入 `dispatched`
                    if !rm.is_private() && !rm.is_static() {
                        self.direct_virtual_sites.insert((m, off));
                    }
                    let r = recv_feeds(self);
                    self.prof_path(site_prof::PATH_NONVIRT);
                    self.prof_role(&r, &a);
                    self.prof_seg(site_prof::SEG_EDGE_RECV);
                    if !rm.is_static() && self.man.concrete.entries.contains(&*self.mref_key(&resolved)) {
                        let s = self.value_set(&r);
                        if self.concrete_call(m, off, &resolved, &md, Some(&s), pargs) {
                            return;
                        }
                    }
                    let nv = (!rm.is_private() && !rm.is_static() && !is_array_type(&mref.owner)).then_some((&site, &md));
                    self.edge_recv_hub(m, off, resolved, via, r, &a, ret, res, nv);
                    return;
                }
                let r = recv_feeds(self);
                self.prof_role(&r, &a);
                self.prof_seg(site_prof::SEG_RECV);
                // 接收者值集未变（`recv_fp.rs`）时沿用上次的接收者判定与 open 枢纽
                let fp = (self.methods[m].kind == Kind::Bytecode).then(|| self.feeds_version(&r));
                let cached = fp.and_then(|v| self.recv_fp_hit(m, off, v)).flatten();
                let mut seen = None;
                let (recv, opens): (Rc<[u32]>, Option<IdSet>) = match &cached {
                    Some((recv, _)) => {
                        self.prof_path(site_prof::PATH_VFP_HIT);
                        (recv.clone(), None)
                    }
                    None => {
                        // 值集确有增长：精确部分只取新增（`recv_fp.rs::value_since`）
                        let since = match fp {
                            Some(v) => self.value_since(m, off, &r, v.0, true),
                            None => recv_fp::Since { s: self.value_set(&r), prev: None, seen: None },
                        };
                        seen = since.seen;
                        // 精确接收者：少量时逐个派发，否则经集合枢纽；open 部分经 open 枢纽
                        let exact = TypeSet { classes: since.s.classes, open: IdSet::default() };
                        let fresh = self.receivers(m, &exact, owner);
                        let old = since.prev.as_ref().map_or(0, |p| p.len());
                        let recv: Rc<[u32]> = match since.prev {
                            Some(prev) => merge_sorted(&prev, &fresh).into(),
                            None => fresh.into(),
                        };
                        self.prof_vmiss(recv.len(), recv.len() - old);
                        (recv, Some(since.s.open))
                    }
                };
                if recv.len() < HUB_MIN {
                    self.prof_path(site_prof::PATH_VSMALL);
                    self.prof_seg(site_prof::SEG_DISPATCH);
                    for &r in recv.iter() {
                        self.dispatch_one(m, off, r, &site, &a, ret, res, NOCTX);
                    }
                } else {
                    // 同一调用点、同一接收者集合即同一枢纽键（成员与接口标志由该偏移的指令决定）
                    self.prof_seg(site_prof::SEG_HUB);
                    let last = self.hub_last.get(&(m, off)).cloned();
                    let h = match last {
                        Some((h, rs)) if *rs == recv[..] => {
                            self.prof_path(site_prof::PATH_VHUB_SAME);
                            h
                        }
                        last => {
                            self.prof_path(site_prof::PATH_VHUB_NEW);
                            let rs = recv.clone();
                            let h = self.hub(mref, iface, owner, HubSet::Exact(rs.clone()), last.map(|x| x.0), &site, &md, via.clone());
                            self.hub_last.insert((m, off), (h, rs));
                            h
                        }
                    };
                    self.prof_seg(site_prof::SEG_LINK);
                    self.link_hub(h, m, off, &a, res);
                }
                self.prof_seg(site_prof::SEG_LINK);
                if let Some((_, hubs)) = cached {
                    for &h in hubs.iter() {
                        self.link_hub(h, m, off, &a, res);
                    }
                    return;
                }
                let opens: Vec<u32> = opens.unwrap_or_default().iter().collect();
                let mut hubs = Vec::new();
                for o in self.open_roots(&opens) {
                    let h = self.hub(mref, iface, owner, HubSet::Open(o), None, &site, &md, via.clone());
                    self.link_hub(h, m, off, &a, res);
                    hubs.push(h);
                }
                if let Some(v) = fp {
                    self.recv_fp_store(m, off, v, Some((recv, hubs.into())), seen);
                }
            }
        }
    }

    /// 值集的 open 类型中各自建枢纽者：open(o) 的接收者（G 中 ⊂ o 者）被同一值集里另一 open 超类型的枢纽涵盖时
    /// 只接后者，目标与结果相同
    pub(super) fn open_roots(&mut self, opens: &[u32]) -> Vec<u32> {
        let mut out = Vec::with_capacity(opens.len());
        for &o in opens {
            if !opens.iter().any(|&p| p != o && self.sub(o, p)) {
                out.push(o);
            }
        }
        out
    }

    /// 接收者 r 上分派已解析方法
    #[allow(clippy::too_many_arguments)]
    pub(super) fn dispatch_one(&mut self, m: usize, off: u32, r: u32, site: &resolve::MethodSite, a: &Args, ret: Option<u32>, res: Option<Node>, via_lambda: u32) {
        self.recv_sites.insert((m, off));
        // lambda 接收者不在此去重：同一调用的去重与增量由 `invoke_lambda` 负责
        let lambda = self.lambdas.contains_key(&r);
        if !lambda && self.methods[m].kind == Kind::Bytecode && !self.dispatched.entry(m).or_default().insert((off, r, via_lambda)) {
            return;
        }
        let via = Via::method("dispatch", m, Some(off));
        if let Some(l) = self.lambdas.get(&r) {
            if site.method().name == l.sam {
                // SAM 实参经捕获拼接后才到实现方法：形参值未知
                let vals = self.call_vals.take();
                self.invoke_lambda(m, off, r, a, ret, res);
                self.call_vals = vals;
                return;
            }
            // 非 SAM 方法（default / Object 方法）按 lambda 类实现的接口选择：函数式接口在前，其后为 altMetafactory 附加接口
            let ifaces: Vec<String> = std::iter::once(&l.iface).chain(&l.markers).cloned().collect();
            if let Some(sel) = ifaces.iter().find_map(|i| self.h.select(i, site)) {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            return;
        }
        if self.hwobjs.contains_key(&r) {
            if let Some(t) = self.hwobj_target(r, site, via) {
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            return;
        }
        let rt = self.ty(r);
        let rname = self.names[rt as usize].to_string();
        match self.h.select(&rname, site) {
            Some(sel) => {
                let (o, n, d) = sel.key();
                let key = MemberRef { owner: o, name: n, desc: d };
                let base = match self.recv_ctx(r) {
                    NOCTX => self.relay_ctx(m, &key),
                    c => c,
                };
                let cx = self.recv_call_ctx(m, off, &key, base);
                let t = cut::with_ctx(None, Some(format!("A:{rname}")), || self.method_ctx(key, cx, via));
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            None => {
                self.unresolved.insert(format!("select {rname} {}", site.method().name));
            }
        }
    }

    /// 非虚调用（special / private / final）：接收者里的抽象对象各进其克隆，其余接收者进方法本体。
    /// 按接收者当前值拆分（接收者节点属调用方，增长时调用方重处理）
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge_recv(&mut self, m: usize, off: u32, key: MemberRef, via: Via, fs: Vec<Feed>, a: &Args, ret: Option<u32>, res: Option<Node>, site: bool) {
        self.edge_recv_in(m, off, key, via, fs, a, ret, res, site, None);
    }

    /// 字节码调用点上的 final 实例方法 / final 类方法（经槽调用的非虚目标，`nv` 为其调用点解析与描述符）。
    /// 抽象对象接收者达 `HUB_MIN` 时与虚调用同样经精确集合枢纽：各对象仍进各自的接收者上下文克隆
    /// （`hub_recv` 与逐个接边同口径取 `recv_ctx`），调用点只接一次枢纽。递归的 final 方法（树节点查找 / 插入）
    /// 的每个接收者克隆里同一调用点都会见到同一批对象，逐个接边是 克隆数 × 对象数 条边，经枢纽是二者之和。
    /// 调用点按选择子常量克隆（`recv_call_clones`）时仍逐个接边，克隆口径不变
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge_recv_hub(&mut self, m: usize, off: u32, key: MemberRef, via: Via, fs: Vec<Feed>, a: &Args, ret: Option<u32>, res: Option<Node>, nv: Option<(&resolve::MethodSite, &classfile::descriptor::MethodDesc)>) {
        self.edge_recv_in(m, off, key, via, fs, a, ret, res, true, nv);
    }

    #[allow(clippy::too_many_arguments)]
    fn edge_recv_in(&mut self, m: usize, off: u32, key: MemberRef, via: Via, fs: Vec<Feed>, a: &Args, ret: Option<u32>, res: Option<Node>, site: bool, nv: Option<(&resolve::MethodSite, &classfile::descriptor::MethodDesc)>) {
        // 接收者按被调方法声明类收窄（checkcast 不改变值来源，来源节点可能更宽）
        let owner = self.id(&key.owner);
        // 字节码调用点自身的接收者（非 lambda 转接）：重跑时只接新增的值（同一分析结果下实参来源、常量与
        // 被调方摘要不变，调用边的其余部分已接上；接收者各部分的效果按值累加）；值集未变时增量为空（`recv_fp.rs`）
        let dedup = site && self.methods[m].kind == Kind::Bytecode;
        let fp = dedup.then(|| self.feeds_version(&fs));
        if fp.is_some_and(|v| self.recv_fp_hit(m, off, v).is_some()) {
            return;
        }
        let s = if let Some(v) = fp {
            // 值集确有增长：只取新增（已见部分都已登记进 `recv_done`，见 `recv_fp.rs::value_since`）
            let since = self.value_since(m, off, &fs, v.0, false);
            self.recv_fp_store(m, off, v, None, since.seen);
            self.recv_delta(m, off, since.s)
        } else {
            self.value_set(&fs)
        };
        let s = self.filter(&s, owner);
        let mut rest = TypeSet { classes: IdSet::default(), open: s.open.clone() };
        let mut objs: Vec<u32> = Vec::new();
        let mut cls: Vec<u32> = Vec::new();
        for x in &s.classes {
            if self.objs.contains_key(&x) {
                objs.push(x);
            } else {
                cls.push(x);
            }
        }
        rest.classes = IdSet::from_sorted(cls);
        // 字节码调用点自身在被调方选择子形参上传常量时按调用点克隆（`ctxsel.rs`）
        let cx = |e: &mut Self, base: u32| if site { e.recv_call_ctx(m, off, &key, base) } else { base };
        if let Some((msite, md)) = nv.filter(|_| dedup && !objs.is_empty() && !self.recv_call_clones(m, &key)) {
            // 本调用点已接的全部抽象对象接收者（`recv_done` 登记了全集，按属主收窄）
            let done: Vec<u32> = self.recv_done.get(&m).and_then(|d| d.get(&off)).map(|v| v.iter().copied().filter(|x| self.objs.contains_key(x)).collect()).unwrap_or_default();
            let all: Vec<u32> = done.into_iter().filter(|&x| self.sub(x, owner)).collect();
            if all.len() >= HUB_MIN {
                let last = self.hub_last.get(&(m, off)).cloned();
                let h = match last {
                    Some((h, rs)) if *rs == all[..] => h,
                    last => {
                        let rs: Rc<[u32]> = all.into();
                        let h = self.hub(&key, false, owner, HubSet::Exact(rs.clone()), last.map(|x| x.0), msite, md, via.clone());
                        self.hub_last.insert((m, off), (h, rs));
                        h
                    }
                };
                self.link_hub(h, m, off, a, res);
                objs.clear();
            }
        }
        for x in objs {
            let base = self.recv_ctx(x);
            let c = cx(self, base);
            let t = self.method_ctx(key.clone(), c, via.clone());
            self.edge(m, off, t, Recv::Exact(x), a, ret, res);
        }
        if !rest.is_empty() {
            // 非对象接收者：内存访问中继方法继承调用方上下文（`relay.rs`），其余进本体
            let base = self.relay_ctx(m, &key);
            let c = cx(self, base);
            let t = self.method_ctx(key, c, via);
            // 物化后的值集不再带来源节点：来源的接口界随接收者传下（`getClass` 结果只展开同时 ⊂ 该接口的子类型）
            let recv = match self.feeds_bound(&fs) {
                Some(b) => Recv::Bounded(rest, b),
                None => Recv::Feeds(vec![Feed::S(rest)]),
            };
            self.edge(m, off, t, recv, a, ret, res);
        }
    }

    /// 调用边：接收者注入 this、实参按位置流入形参（被调声明类型过滤）、返回值流回结果节点
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge(&mut self, m: usize, off: u32, t: usize, recv: Recv, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        if self.dispatch.entry((m, off)).or_default().insert(t) {
            self.ctx.stats.borrow_mut().sprof.dispatch_new += 1;
        }
        self.callers.entry(t).or_default().insert(m);
        self.caller_edge(m, t);
        let is_static = self.methods[t].is_static;
        let ptypes = self.methods[t].ptypes.clone();
        let base = usize::from(!is_static);
        // lambda 接边：捕获值与 SAM 实参按实现方法形参对齐（`lambda_vals.rs`）；接边期间的嵌套调用不继承
        if let Some(c) = self.lambda_cap.take() {
            self.bind_lambda_params(m, off, t, base, &ptypes, &c);
            self.lambda_cap = Some(c);
        } else {
            self.bind_params(m, t, base, ptypes.len());
            if let Some(cv) = self.call_vals.clone() {
                let string = self.id(STRING);
                self.pstr_site(m, off, &cv, |j| Some(pstrs::PSlot::M(t, base + j)), |j| ptypes.get(base + j).copied().flatten() == Some(string));
            }
        }
        // 按字段句柄存取、接收者为来源标记（或由标记给出身份的句柄对象）：按调用点建模，对象实参与写入值
        // 不流入被调方形参（`field_access.rs`）
        let fa = self.field_access_recv(t, &recv).filter(|k| !matches!(k, field_access::FaRecv::Bytecode));
        let bound = match &recv {
            Recv::Bounded(_, b) => Some(*b),
            _ => None,
        };
        let recv_fs = self.edge_this(t, recv);
        if let Some(k) = fa {
            self.field_access_site(m, off, t, k, a, res.filter(|_| ret.is_some()));
            return;
        }
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(base + j)) {
                self.feed(fs, Node::P(t, (base + j) as u16), *pt);
            }
        }
        self.edge_ret(m, off, t, recv_fs, bound, a, ret, res);
    }

    /// 已对 (m, off, t) 以同一实参 a（同一实参值）完整接边后，再派发新接收者 r：
    /// 调用关系、形参常量、字符串常量与实参边都已接上且不随接收者变化，只接接收者及依赖接收者的结果部分
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge_more(&mut self, m: usize, off: u32, t: usize, r: u32, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        let recv_fs = self.edge_this(t, Recv::Exact(r));
        self.edge_ret(m, off, t, recv_fs, None, a, ret, res);
    }

    /// 接收者流入被调方 this，返回本调用点的接收者来源
    fn edge_this(&mut self, t: usize, recv: Recv) -> Option<Vec<Feed>> {
        if self.methods[t].is_static {
            return None;
        }
        match (recv, self.methods[t].ptypes.first().copied().flatten()) {
            (Recv::Exact(r), _) => {
                self.add_to(Node::P(t, 0), &TypeSet::exact(r));
                Some(vec![Feed::S(TypeSet::exact(r))])
            }
            (Recv::Feeds(fs), Some(pt)) => {
                self.feed(&fs, Node::P(t, 0), pt);
                Some(fs)
            }
            (Recv::Bounded(s, _), Some(pt)) => {
                let fs = vec![Feed::S(s)];
                self.feed(&fs, Node::P(t, 0), pt);
                Some(fs)
            }
            _ => None,
        }
    }

    /// 接收者来源 fs 的共同接口界：含 open 的来源全是同一接口类型判定站点（`open_bounds`）时为该接口，否则无界
    fn feeds_bound(&mut self, fs: &[Feed]) -> Option<u32> {
        let mut bound = None;
        for f in fs {
            let b = match f {
                Feed::S(s) if s.open.is_empty() => continue,
                Feed::S(_) => return None,
                Feed::N(n) => match self.open_bounds.get(n).copied() {
                    Some(b) => b,
                    None if self.set_of(*n).open.is_empty() => continue,
                    None => return None,
                },
            };
            if bound.is_some_and(|x| x != b) {
                return None;
            }
            bound = Some(b);
        }
        bound
    }

    /// 手写调用点与按调用点建模的返回值；bound 为物化接收者的接口界（[`Recv::Bounded`]）
    #[allow(clippy::too_many_arguments)]
    fn edge_ret(&mut self, m: usize, off: u32, t: usize, recv_fs: Option<Vec<Feed>>, bound: Option<u32>, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        let is_static = self.methods[t].is_static;
        let base = usize::from(!is_static);
        // 调用方的内存效果已按清单逐调用点建模（`[facts.array_writes]` / `[facts.memory_reads]`）时，其手写体对内存访问
        // 成员的上调是同一语义的实现（VarHandle.set → Unsafe.putReference、putReferenceVolatile → putReference 等），
        // 不再以调用方值池为实参另建一份汇合的读写——否则偏移与对象跨调用点相乘
        let subsumed = self.declares_memory(m) && self.declares_memory(t);
        if matches!(self.methods[t].kind, Kind::Handwritten(_)) && !subsumed {
            self.hw_site(m, off, t, recv_fs.as_deref(), a);
            self.rcall_site(m, off, t, recv_fs.as_deref(), a);
        }
        if let (Some(rt), Some(res)) = (ret, res) {
            let model = self.methods[t].ret_model;
            if let Some(op) = model.mirror_op() {
                // 类镜像：结果 = 本调用点接收者各值的 Class 对象 / 各镜像所指类的超类镜像（逐调用点）
                for f in recv_fs.iter().flatten() {
                    match f {
                        Feed::N(n) => self.mflow(*n, res, op),
                        Feed::S(s) => self.mirror_into(op, s, res, bound),
                    }
                }
            } else if model == RetModel::Receiver {
                // 浅拷贝：返回值 = 本调用点的接收者类型集（数组共享元素节点；逐调用点，不经被调方形参汇合）
                if let Some(fs) = &recv_fs {
                    self.feed(fs, res, rt);
                }
            } else if model == RetModel::Caller {
                self.caller_ret(m, t, res, rt);
            } else if let RetModel::NewArray(i) = model {
                // 新数组：结果 = 本调用点元素类型实参各类镜像所指类型的数组分配点（逐调用点；元素只来自其后的写入）
                let op = MirrorOp::ArrayOf(m as u32, off);
                for f in a.get(i).cloned().flatten().iter().flatten() {
                    match f {
                        Feed::N(n) => self.mflow(*n, res, op),
                        Feed::S(s) => self.mirror_into(op, s, res, None),
                    }
                }
            } else if model == RetModel::StaticBase {
                // 静态字段基址：结果 = 本调用点字段句柄实参各值所指字段声明类的类镜像（逐调用点；不经返回值汇合的 open 基址）
                for f in a.first().cloned().flatten().iter().flatten() {
                    match f {
                        Feed::N(n) => self.mflow(*n, res, MirrorOp::Holder),
                        Feed::S(s) => self.mirror_into(MirrorOp::Holder, s, res, None),
                    }
                }
            } else if let RetModel::Read(src) = model {
                let i = src + usize::from(!is_static);
                let fs = if subsumed {
                    None
                } else if !is_static && i == 0 {
                    recv_fs.clone()
                } else {
                    a.get(src).cloned().flatten()
                };
                // 签名多态读取（VarHandle get 族）：静态字段句柄无 holder 坐标，另读按名打开的静态字段
                if self.is_poly(t) && !subsumed {
                    self.poly_read(res, rt);
                }
                if let Some(fs) = fs {
                    self.hw_read_site(m, off, t, i as u16, &fs, res, rt);
                }
            } else if let Some(ps) = self.passthrough(t) {
                // 透传方法：结果 = 本调用点对应实参（逐调用点，不经 R 汇合）。实参按被调形参的声明类型收窄，与经
                // P → R 的汇合路径同一口径：摘要随分析推进由透传转为汇合时，已接的透传边被 R 涵盖，结果与处理次序无关
                for i in ps {
                    let fs = if !is_static && i == 0 { recv_fs.clone() } else { a.get(i as usize - base).cloned().flatten() };
                    let Some(fs) = fs else { continue };
                    let f = match self.methods[t].ptypes.get(i as usize).copied().flatten() {
                        Some(pt) if self.sub(pt, rt) => pt,
                        _ => rt,
                    };
                    self.feed(&fs, res, f);
                }
            } else {
                self.flow(Node::R(t), res, rt);
            }
        }
    }

    /// 形参常量：并入本调用点的实参值（非字节码调用点 = Top）；变化时被调方法失效
    pub(super) fn bind_params(&mut self, m: usize, t: usize, base: usize, n: usize) {
        let cv = self.call_vals.clone();
        let vals: Option<Vec<PV>> = cv.as_ref().map(|vs| vs.iter().map(PV::of_ret).collect());
        match &cv {
            Some(vs) => {
                self.taint_site(m, t, base, n, vs);
                self.join_pvs(t, base, n, vals.as_deref());
            }
            None => self.bind_pvs(t, base, n, None),
        }
    }

    /// 值来自本方法形参时，各调用点在该形参上的字符串常量；登记 (m, off) 为读者
    pub(super) fn param_strs(&mut self, m: usize, off: u32, v: &V) -> Vec<Rc<str>> {
        let mut out = Vec::new();
        for s in v.srcs().iter() {
            let Src::Param(i) = s else { continue };
            out.extend(self.pstr_read(m, *i as usize, off));
        }
        out
    }

    /// 形参常量并入（vals 不含接收者；None = 实参值未知）
    pub(super) fn bind_pvs(&mut self, t: usize, base: usize, n: usize, vals: Option<&[PV]>) {
        self.pstr_offsite(t);
        if vals.is_none() {
            self.pstr_top_m(t);
        }
        self.taint_params(t, base, n, vals);
        self.join_pvs(t, base, n, vals);
    }

    pub(super) fn join_pvs(&mut self, t: usize, base: usize, n: usize, vals: Option<&[PV]>) {
        let cur = self.pvals.get(&t).cloned();
        let new: Vec<PV> = (0..n)
            .map(|i| {
                let v = match (vals, i.checked_sub(base)) {
                    (Some(vs), Some(j)) => vs.get(j).cloned().unwrap_or(PV::Top),
                    _ => PV::Top,
                };
                PV::join(cur.as_ref().and_then(|c| c.get(i)), &v)
            })
            .collect();
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.pvals.insert(t, new);
        if cur.is_some() {
            self.invalidate(t, Why::ParamConst);
        }
    }
}

/// 调用描述中的属主是数组类型（`[` 开头的描述符形式）
/// 两个不相交的升序序列归并为升序
fn merge_sorted(a: &[u32], b: &[u32]) -> Vec<u32> {
    if b.is_empty() {
        return a.to_vec();
    }
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

fn is_array_type(owner: &str) -> bool {
    owner.starts_with('[')
}

#[cfg(test)]
mod tests {
    use super::is_array_type;

    #[test]
    fn array_owners_are_recognized_by_descriptor_form() {
        assert!(is_array_type("[Lp/C;"));
        assert!(is_array_type("[[I"));
        assert!(!is_array_type("p/C"));
    }
}
