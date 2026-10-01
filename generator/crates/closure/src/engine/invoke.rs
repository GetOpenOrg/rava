//! 引擎：调用——反射写入、静态 / 虚调用接边、形参绑定。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        self.refs.insert(mref.to_string());
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
        let k = mref.to_string();
        if self.man.is_method_lookup(&k) {
            let mut names: BTreeSet<Rc<str>> = BTreeSet::new();
            // 拼接出的名字按目标类逐个解析（只保留该类上声明的方法）；目标类另取 Class 实参值集里类镜像所指的类
            // （如取自 static final Class 字段）。形参透传的名字不与镜像类相乘：其类同样来自形参，交叉组合会失真
            let mut per_class: Vec<(String, Rc<str>)> = vec![];
            let mut targets: Option<Vec<String>> = None;
            for a in args {
                match a {
                    V::Str(name) => {
                        names.insert(name.clone());
                    }
                    V::Ref { .. } => {
                        // 合流前的各字面量（如按条件二选一的名字）与形参上流入的字符串常量
                        names.extend(a.lits());
                        names.extend(self.param_strs(m, off, a));
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
            for name in &names {
                for c in &classes {
                    self.reflect_name(c, name);
                }
            }
            for (c, name) in &per_class {
                self.reflect_name(c, name);
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
        if self.man.is_field_enumerator(&k) {
            let cls = match args.first() {
                Some(V::Class(c, _)) => Some(c.to_string()),
                _ => None,
            };
            self.enumerate_fields(cls);
        }
        if self.man.is_deserializer(&k) && !self.ctx.deser.replace(true) {
            self.open_fields_all();
        }
    }

    /// 字段枚举（cls = 接收者类字面量，None = 推不出）：句柄写入口可达时放开，否则挂起到写入口可达
    fn enumerate_fields(&mut self, cls: Option<String>) {
        if !self.fwriter_live {
            self.fenum_pending.insert(cls);
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
                    self.open_fields_all();
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
        self.touch(&mref.owner, Level::Type, via.clone());
        let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, iface) else {
            self.unresolved.insert(mref.to_string());
            return;
        };
        let (o, n, d) = site.key();
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
                // 静态调用继承调用方的克隆上下文（容器方法里的静态辅助方法随容器对象分开）
                self.init(&resolved.owner, via.clone());
                // 只有返回引用的辅助方法随上下文克隆（返回值按容器对象分开）；返回基本类型 / void 的静态方法克隆收益可忽略，按本体共享
                // 上下文无关的调用方调用新鲜工厂（返回本方法新分配的容器 / 引用数组）：按调用点克隆，
                // 否则各调用点的实参元素经同一个返回对象汇合（`Arrays.copyOf` 的副本数组）
                // 分派转发方法（形参流到分派接收者）按调用点克隆，优先于以上规则（边界计划 §6.2 G1）
                let caller_ctx = self.methods[m].ctx;
                let ctx = match caller_ctx {
                    _ if !md.ret.as_ref().is_some_and(|r| r.is_reference()) => NOCTX,
                    NOCTX if self.fresh_factory(&resolved) => self.site_ctx(m, off),
                    c => c,
                };
                // 选择子形参上传常量（或调用方已在上下文中）：按调用点克隆，分支按形参常量剪枝（`selector.rs`）
                let ctx = self.selector_ctx(m, off, &resolved, pargs).unwrap_or(ctx);
                // 按名取类：名字能由常量拼出时结果只含所指类的镜像，不再接被调方法返回的所指未知的 Class
                let (named, top) = if self.man.names.is_class_lookup(&mref.to_string()) { self.class_lookup(m, off, args) } else { (vec![], true) };
                let t = self.callee(m, off, resolved, ctx, via);
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
                    // 非虚：直接到已解析方法，接收者值流入 this
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
                    let parent = self.hub_last.get(&(m, off)).copied();
                    let h = self.hub(mref, iface, owner, HubSet::Exact(recv), parent, &site, &md, via.clone());
                    self.hub_last.insert((m, off), h);
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
                let t = self.method_ctx(MemberRef { owner: o, name: n, desc: d }, self.ctx_of(r), via);
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
        let s = self.filter(&s, owner);
        let mut rest = TypeSet { classes: IdSet::default(), open: s.open.clone() };
        // 字节码调用点自身的接收者（非 lambda 转接）：重跑时只接新增对象
        let dedup = site && self.methods[m].kind == Kind::Bytecode;
        for x in &s.classes {
            if self.objs.contains_key(&x) {
                if dedup && !self.recv_done.entry(m).or_default().insert((off, x)) {
                    continue;
                }
                let t = self.method_ctx(key.clone(), x, via.clone());
                self.edge(m, off, t, Recv::Exact(x), a, ret, res);
            } else {
                rest.classes.insert(x);
            }
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
        let mut recv_fs: Option<Vec<Feed>> = None;
        if !is_static {
            match (recv, ptypes.first().copied().flatten()) {
                (Recv::Exact(r), _) => {
                    self.add_to(Node::P(t, 0), &TypeSet::exact(r));
                    recv_fs = Some(vec![Feed::S(TypeSet::exact(r))]);
                }
                (Recv::Feeds(fs), Some(pt)) => {
                    self.feed(&fs, Node::P(t, 0), pt);
                    recv_fs = Some(fs);
                }
                _ => {}
            }
        }
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(base + j)) {
                self.feed(fs, Node::P(t, (base + j) as u16), *pt);
            }
        }
        if matches!(self.methods[t].kind, Kind::Handwritten(_)) {
            self.hw_site(m, off, t, recv_fs.as_deref(), a);
        }
        if let (Some(rt), Some(res)) = (ret, res) {
            let model = self.methods[t].ret_model;
            if model == RetModel::Mirror {
                // 类镜像：结果 = 本调用点接收者各值的 Class 对象（逐调用点）
                for f in recv_fs.iter().flatten() {
                    match f {
                        Feed::N(n) => self.mflow(*n, res),
                        Feed::S(s) => {
                            let k = self.mirror_set(s);
                            self.add_to(res, &k);
                        }
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
    fn param_strs(&mut self, m: usize, off: u32, v: &V) -> Vec<Rc<str>> {
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
