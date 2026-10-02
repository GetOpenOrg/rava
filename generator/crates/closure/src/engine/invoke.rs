//! 引擎：调用——反射写入、静态 / 虚调用接边、形参绑定。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        self.note_ref(mref);
        self.reflective_writes(m, off, mref, opcode, args);
        self.service_lookup(m, off, opcode, mref, args);
        self.class_init_site(m, off, opcode, mref, args);
        let pargs = if opcode == classfile::op::INVOKESTATIC { args } else { args.get(1..).unwrap_or(&[]) };
        self.call_vals = Some(Rc::from(pargs));
        self.invoke_inner(m, off, opcode, mref, iface, args);
        self.call_vals = None;
    }

    /// 反射式字段写入（按字节码形状）：
    /// - 同一调用里有字符串常量，且形参含 Class 或接收者是 Class：点名字段不折叠
    ///   （所属类取 Class 常量实参 / 接收者，取不到时同名字段全部不折叠）；
    /// - 清单 `[facts.field_writes] enumerators`（返回字段句柄数组）：句柄写入口（`handle_writers`）也可达时
    ///   接收者类的全部字段不折叠，推不出时全部字段；
    /// - 清单 `[facts.reflect] method_lookups`：字符串常量登记为 Class 常量所指类的方法点名；名字是本方法形参时
    ///   取各调用点在该形参上的字符串常量（如按名构造 MemberName 的辅助方法），常量集增长时本站点重跑；
    /// - 清单 `deserializers` 可达：非 static、非 transient 字段全部不折叠
    pub(super) fn reflective_writes(&mut self, m: usize, off: u32, mref: &MemberRef, opcode: u8, args: &[V]) {
        self.rcall_conversion(m, off, mref, opcode, args);
        let class_param = parse_method(&mref.desc)
            .is_some_and(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(c) if c == CLASS)));
        let class_recv = opcode != classfile::op::INVOKESTATIC && mref.owner == CLASS;
        let classes: Vec<String> = args
            .iter()
            .filter_map(|a| match a {
                V::Class(c, _) => Some(c.to_string()),
                _ => None,
            })
            .collect();
        let k = self.mref_key(mref);
        if self.man.is_method_lookup(&k) {
            // 查找结果经哪条反射调用通道调用（按查找结果的类型，见 `reflect_call.rs`）
            let ch = self.rcall_lookup_channel(&mref.desc);
            let mut names: BTreeSet<Rc<str>> = BTreeSet::new();
            // 本调用点上的字面量名（常量实参 / 合流前的各字面量）；形参透传来的名字不在此列
            let mut site_names: BTreeSet<Rc<str>> = BTreeSet::new();
            // 拼接出的名字按目标类逐个解析（只保留该类上声明的方法）；目标类另取 Class 实参值集里类镜像所指的类
            // （如取自 static final Class 字段）。形参透传的名字不与镜像类相乘：其类同样来自形参，交叉组合会失真
            let mut per_class: Vec<(String, Rc<str>)> = vec![];
            let mut targets: Option<Vec<String>> = None;
            for a in args {
                match a {
                    V::Str(name) => {
                        names.insert(name.clone());
                        site_names.insert(name.clone());
                    }
                    V::Ref { .. } => {
                        // 合流前的各字面量（如按条件二选一的名字）与形参上流入的字符串常量
                        let lits = a.lits();
                        site_names.extend(lits.iter().cloned());
                        names.extend(lits);
                        names.extend(self.param_strs(m, off, a));
                        names.extend(self.field_strs(m, a));
                        let Some(parts) = self.method_name_parts(m, a) else { continue };
                        if targets.is_none() {
                            let mut ts = classes.clone();
                            for c in self.class_arg_mirrors(m, mref, opcode, args) {
                                if !ts.contains(&c) {
                                    ts.push(c);
                                }
                            }
                            targets = Some(ts);
                        }
                        for c in targets.iter().flatten() {
                            for n in self.declared_matching(c, &parts) {
                                per_class.push((c.clone(), n));
                            }
                        }
                    }
                    _ => {}
                }
            }
            // 常量名的查找目标：Class 常量实参；本调用点的字面量名另对 Class 接收者值集里类镜像所指的类点名
            // （如 `this.getMethod("values")`：接收者是流到该方法的类镜像）。
            // 形参透传的名字不与接收者镜像相乘：名字与接收者各自来自全部调用点，交叉组合会把任意镜像类上的
            // 同名方法拉进反射面（如序列化辅助方法按形参取名、按形参取类）；拼段名同理只按常量类 / Class 形参定目标
            for name in &names {
                for c in &classes {
                    self.reflect_name(c, name, ch);
                }
            }
            if class_recv && !site_names.is_empty() {
                for c in args.first().map(|r| self.recv_mirrors(m, r)).unwrap_or_default() {
                    if classes.contains(&c) {
                        continue;
                    }
                    for name in &site_names {
                        self.reflect_name(&c, name, ch);
                    }
                }
            } else if class_recv && !names.is_empty() && classes.is_empty() {
                // 名字只经形参流入、接收者非常量：查找目标推不出，记为反射缺口
                self.reflect_gaps.insert(format!("{} <- recv(param-name)", self.methods[m].key));
            }
            for (c, name) in &per_class {
                self.reflect_name(c, name, ch);
            }
        }
        if class_param || class_recv {
            for name in args.iter().flat_map(V::lits) {
                let name = &name;
                let mut hit = false;
                for c in &classes {
                    if let Some((decl, desc)) = self.field_by_name(c, name) {
                        self.open_field(MemberRef { owner: decl, name: name.to_string(), desc });
                        hit = true;
                    }
                }
                if !hit {
                    self.open_field_name(name);
                }
            }
        }
        if (class_param || class_recv) && !self.man.is_method_lookup(&k) {
            self.field_lookup(m, off, mref, opcode, args, &classes, class_recv);
        }
        if self.man.is_field_enumerator(&k) {
            // 接收者 Class 值集里的类镜像逐类放开（值集增长时本站点重跑）；含所指未知的 Class 时全部放开，记为缺口
            let mut cs = BTreeSet::new();
            let unknown = args.first().is_none_or(|v| self.class_values(m, v, &mut cs));
            for c in cs {
                self.enumerate_fields(Some(c));
            }
            if unknown {
                self.field_enum_gaps.insert(format!("{}@{off}", self.methods[m].key));
                self.enumerate_fields(None);
            }
        }
        if self.man.is_deserializer(&k) && !self.ctx.deser.replace(true) {
            self.open_fields_all(self.ctx.fopen_all.get(), false);
        }
    }

    /// 字段枚举（cls = 接收者 Class 值所指的类，None = 推不出）：句柄写入口可达时放开，否则挂起到写入口可达
    fn enumerate_fields(&mut self, cls: Option<String>) {
        if !self.fwriter_live {
            if self.fenum_pending.insert(cls) {
                self.offset_reads_ready();
            }
            return;
        }
        match cls {
            Some(c) => {
                let mut cur = Some(c);
                while let Some(cls) = cur {
                    let Some(cf) = self.h.class(&cls) else { break };
                    for f in &cf.fields {
                        self.open_field(MemberRef { owner: cls.clone(), name: f.name.clone(), desc: f.desc.clone() });
                    }
                    cur = cf.super_name.clone();
                }
            }
            None => {
                if !self.ctx.fopen_all.replace(true) {
                    self.open_fields_all(false, self.ctx.deser.get());
                }
            }
        }
    }

    /// 调用边到达字段句柄写入口：调用者是句柄桥（取得的句柄只经 Field.set* 的访问器使用，
    /// 写入由 Field.set* 计入）时不算；每条边都判（首个调用者是桥不代表后续调用者也是）
    pub(super) fn handle_writer_edge(&mut self, key: &MemberRef, via: &Via) {
        if !self.man.is_field_handle_writer(key) {
            return;
        }
        if let From::Method(c) = via.from {
            if self.man.is_field_handle_bridge(&self.methods[c].key.to_string()) {
                return;
            }
        }
        self.field_writer_live();
    }

    /// 按字段句柄写字段的入口可达：挂起的字段枚举生效
    pub(super) fn field_writer_live(&mut self) {
        if std::mem::replace(&mut self.fwriter_live, true) {
            return;
        }
        for cls in std::mem::take(&mut self.fenum_pending) {
            self.enumerate_fields(cls);
        }
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
        let res = Some(Node::S(m, off));
        let recv_feeds = |e: &mut Self| match recv_v {
            Some(v) => e.feeds(m, v, owner),
            None => vec![Feed::S(TypeSet::open(owner))],
        };
        match opcode {
            op::INVOKESTATIC => {
                self.init(&resolved.owner, via.clone());
                // 克隆上下文的选择见 `ctxsel.rs`
                let ret_ref = md.ret.as_ref().is_some_and(|r| r.is_reference());
                let ctx = self.static_ctx(m, off, &resolved, Call::Invoke { ret_ref, args: pargs });
                // 按名取类：名字能由常量拼出时结果只含所指类的镜像，不再接被调方法返回的所指未知的 Class
                let (named, top) = if self.man.names.is_class_lookup(&self.mref_key(mref)) { self.class_lookup(m, off, args) } else { (vec![], true) };
                let t = self.method_ctx(resolved, ctx, via);
                self.edge(m, off, t, Recv::None, &a, ret, if top { res } else { None });
                for c in named {
                    self.named_class(m, off, &c);
                }
            }
            op::INVOKESPECIAL => {
                let r = recv_feeds(self);
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
                    self.edge_recv(m, off, resolved, via, r, &a, ret, res, true);
                    return;
                }
                let r = recv_feeds(self);
                let s = self.value_set(&r);
                // 精确接收者：少量时逐个派发，否则经集合枢纽；open 部分经 open 枢纽
                let exact = TypeSet { classes: s.classes, open: IdSet::default() };
                let recv: Vec<u32> = self.receivers(m, &exact, owner);
                if recv.len() < HUB_MIN {
                    for r in recv {
                        self.dispatch_one(m, off, r, &site, &a, ret, res, NOCTX);
                    }
                } else {
                    // 同一调用点、同一接收者集合即同一枢纽键（成员与接口标志由该偏移的指令决定）
                    let last = self.hub_last.get(&(m, off)).cloned();
                    let h = match last {
                        Some((h, rs)) if *rs == recv[..] => h,
                        last => {
                            let rs: Rc<[u32]> = recv.into();
                            let h = self.hub(mref, iface, owner, HubSet::Exact(rs.clone()), last.map(|x| x.0), &site, &md, via.clone());
                            self.hub_last.insert((m, off), (h, rs));
                            h
                        }
                    };
                    self.link_hub(h, m, off, &a, res);
                }
                for o in s.open.iter() {
                    let h = self.hub(mref, iface, owner, HubSet::Open(o), None, &site, &md, via.clone());
                    self.link_hub(h, m, off, &a, res);
                }
            }
        }
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
                let cx = self.recv_ctx(r);
                let t = cut::with_ctx(None, Some(format!("A:{rname}")), || self.method_ctx(MemberRef { owner: o, name: n, desc: d }, cx, via));
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
        // 接收者按被调方法声明类收窄（checkcast 不改变值来源，来源节点可能更宽）
        let owner = self.id(&key.owner);
        let s = self.value_set(&fs);
        // 字节码调用点自身的接收者（非 lambda 转接）：重跑时只接新增的值（同一分析结果下实参来源、常量与
        // 被调方摘要不变，调用边的其余部分已接上；接收者各部分的效果按值累加）
        let dedup = site && self.methods[m].kind == Kind::Bytecode;
        let s = if dedup { self.recv_delta(m, off, s) } else { s };
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
        for x in objs {
            let t = self.method_ctx(key.clone(), self.recv_ctx(x), via.clone());
            self.edge(m, off, t, Recv::Exact(x), a, ret, res);
        }
        if !rest.is_empty() {
            let t = self.method(key, via);
            self.edge(m, off, t, Recv::Feeds(vec![Feed::S(rest)]), a, ret, res);
        }
    }

    /// 调用边：接收者注入 this、实参按位置流入形参（被调声明类型过滤）、返回值流回结果节点
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge(&mut self, m: usize, off: u32, t: usize, recv: Recv, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        self.dispatch.entry((m, off)).or_default().insert(t);
        self.callers.entry(t).or_default().insert(m);
        let is_static = self.methods[t].is_static;
        let ptypes = self.methods[t].ptypes.clone();
        let base = usize::from(!is_static);
        self.bind_params(t, base, ptypes.len());
        if let Some(cv) = self.call_vals.clone() {
            self.pstr_site(m, &cv, |j| pstrs::PSlot::M(t, base + j));
        }
        let recv_fs = self.edge_this(t, recv);
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(base + j)) {
                self.feed(fs, Node::P(t, (base + j) as u16), *pt);
            }
        }
        self.edge_ret(m, off, t, recv_fs, a, ret, res);
    }

    /// 已对 (m, off, t) 以同一实参 a（同一实参值）完整接边后，再派发新接收者 r：
    /// 调用关系、形参常量、字符串常量与实参边都已接上且不随接收者变化，只接接收者及依赖接收者的结果部分
    #[allow(clippy::too_many_arguments)]
    pub(super) fn edge_more(&mut self, m: usize, off: u32, t: usize, r: u32, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        let recv_fs = self.edge_this(t, Recv::Exact(r));
        self.edge_ret(m, off, t, recv_fs, a, ret, res);
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
            _ => None,
        }
    }

    /// 手写调用点与按调用点建模的返回值
    #[allow(clippy::too_many_arguments)]
    fn edge_ret(&mut self, m: usize, off: u32, t: usize, recv_fs: Option<Vec<Feed>>, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        let is_static = self.methods[t].is_static;
        let base = usize::from(!is_static);
        if matches!(self.methods[t].kind, Kind::Handwritten(_)) {
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
                        Feed::S(s) => self.mirror_into(op, s, res),
                    }
                }
            } else if model == RetModel::Receiver {
                // 浅拷贝：返回值 = 本调用点的接收者类型集（数组共享元素节点；逐调用点，不经被调方形参汇合）
                if let Some(fs) = &recv_fs {
                    self.feed(fs, res, rt);
                }
            } else if let RetModel::Read(src) = model {
                let i = src + usize::from(!is_static);
                let fs = if !is_static && i == 0 { recv_fs.clone() } else { a.get(src).cloned().flatten() };
                if let Some(fs) = fs {
                    self.hw_read_site(m, off, t, i as u16, &fs, res, rt);
                }
            } else if let Some(ps) = self.passthrough(t) {
                // 透传方法：结果 = 本调用点对应实参（逐调用点，不经 R 汇合）
                for i in ps {
                    let fs = if !is_static && i == 0 { recv_fs.clone() } else { a.get(i as usize - base).cloned().flatten() };
                    if let Some(fs) = fs {
                        self.feed(&fs, res, rt);
                    }
                }
            } else {
                self.flow(Node::R(t), res, rt);
            }
        }
    }

    /// 形参常量：并入本调用点的实参值（非字节码调用点 = Top）；变化时被调方法失效
    pub(super) fn bind_params(&mut self, t: usize, base: usize, n: usize) {
        let vals: Option<Vec<PV>> = self.call_vals.as_ref().map(|vs| vs.iter().map(PV::of).collect());
        self.bind_pvs(t, base, n, vals.as_deref());
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
