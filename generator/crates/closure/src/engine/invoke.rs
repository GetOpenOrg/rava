//! 引擎：调用——反射写入、静态 / 虚调用接边、形参绑定。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        self.note_ref(mref);
        self.reflective_writes(m, off, mref, opcode, args);
        self.service_lookup(m, off, opcode, mref, args);
        self.prop_key_site(m, off, opcode, mref, iface, args);
        let pargs = if opcode == classfile::op::INVOKESTATIC { args } else { args.get(1..).unwrap_or(&[]) };
        self.call_vals = Some(Rc::from(pargs));
        let wrapped = self.ref_caller_sensitive(mref);
        let outer = std::mem::replace(&mut self.cs.site_wrapped, wrapped);
        let lambda = self.cs.lambda_site.take();
        self.invoke_inner(m, off, opcode, mref, iface, args);
        self.lookup_wrap_call(m, off, args);
        self.cs.lambda_site = lambda;
        self.cs.site_wrapped = outer;
        self.call_vals = None;
    }

    /// 反射式字段写入（按字节码形状）：
    /// - 同一调用里有字符串常量，且形参含 Class 或接收者是 Class：点名字段不折叠
    ///   （所属类取 Class 常量实参 / 接收者，取不到时同名字段全部不折叠）；
    /// - 清单 `[facts.field_writes] enumerators`（返回字段句柄数组）：句柄写入口（`handle_writers`）也可达时
    ///   接收者类的全部字段不折叠，推不出时全部字段（调用方是清单 `serial_enumerators` 时为可序列化字段）；
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
        if self.man.is_constructor_lookup(&k) {
            self.constructor_lookup(m, off, &k, mref, opcode, args);
        }
        if let Some(idx) = self.man.serial_allocator(&k) {
            self.serial_alloc_site(m, off, &k, opcode, args, idx);
        }
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
                    V::Str(name, _) if !a.derived_str() => {
                        names.insert(name.clone());
                        site_names.insert(name.clone());
                    }
                    // 常量格给出的名字（形参 / 字段 / 返回常量）按其来源处理，同常量格推不出时：
                    // 不算本点字面量，不与接收者镜像相乘（中间态常量与终态给出同样的点名，见 `V::Str`）
                    V::Ref { .. } | V::Str(..) => {
                        // 合流前的各字面量（如按条件二选一的名字）与形参上流入的字符串常量
                        site_names.extend(a.site_lits());
                        names.extend(a.lits());
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
            }
            // 查找类与名字都来自本方法形参：登记为包装方法，各调用点按本点实参配对点名（`lookup_pair.rs`）
            let wrapped = class_recv && classes.is_empty() && self.lookup_wraps(m, mref, opcode, args, ch);
            if class_recv && !wrapped && !names.is_empty() && classes.is_empty() && site_names.is_empty() {
                // 名字只经形参流入、接收者非常量：查找目标推不出，记为反射缺口
                self.reflect_gaps.insert(format!("{} <- recv(param-name)", self.methods[m].key));
            }
            // 本调用点的字面量名另对 Class 形参值集里类镜像所指的类点名（如 `findStatic(invokerClass, "invoke_V", …)`：
            // 类取自字段 / 类定义点返回的镜像）。与接收者镜像同一口径：只乘本调用点字面量，不乘形参透传的名字
            if class_param && !site_names.is_empty() {
                for c in self.class_arg_mirrors(m, mref, opcode, args) {
                    if classes.contains(&c) {
                        continue;
                    }
                    for name in &site_names {
                        self.reflect_name(&c, name, ch);
                    }
                }
            }
            for (c, name) in &per_class {
                self.reflect_name(c, name, ch);
            }
        }
        if class_param || class_recv {
            // 按名放开字段：字面量与常量格给出的名字，另取按来源给出的名字（形参上各调用点的字符串常量、字段写入的
            // 字面量集）。形参常量窗口内的文本是终态形参字符串集的子集，后者在形参抬为 Top 后仍给出同样的名字
            // 类值（Class 接收者 / Class 形参）与名字都来自本方法形参时登记字段配对，形参名字不在此汇合放开，
            // 由各调用点按本点实参配对点名（`lookup_pair.rs`）
            let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
            let mut cpos: Vec<usize> = if class_recv { vec![0] } else { vec![] };
            // 名字位：声明为 String 的形参。Object 等其他引用形参不是字段名（如 `compareComparables(Class, Object, Object)`
            // 的键），不取名字、不登记配对——否则映射键上流入的全部字符串常量都会按名放开字段
            let mut spos: Vec<usize> = vec![];
            if let Some(md) = parse_method(&mref.desc) {
                cpos.extend(md.params.iter().enumerate().filter(|(_, p)| matches!(p, FieldType::Object(c) if c == CLASS)).map(|(i, _)| i + skip));
                spos.extend(md.params.iter().enumerate().filter(|(_, p)| matches!(p, FieldType::Object(c) if c == STRING)).map(|(i, _)| i + skip));
            }
            let mut fnames: BTreeSet<Rc<str>> = BTreeSet::new();
            for (i, a) in args.iter().enumerate() {
                if !spos.contains(&i) {
                    continue;
                }
                fnames.extend(a.lits());
                if matches!(a, V::Ref { .. }) || a.derived_str() {
                    let mut paired = false;
                    for &c in &cpos {
                        paired |= self.lookup_wrap_site(m, &args[c], a, &[], 0, true);
                    }
                    if !paired {
                        fnames.extend(self.param_strs(m, off, a));
                    }
                    fnames.extend(self.field_strs(m, a));
                }
            }
            for name in &fnames {
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
        self.field_name_site(m, off, &k, opcode, args);
        self.handle_writer_site(m, mref, opcode, args);
        self.mirror_init_site(m, off, mref, &k, opcode, args);
        if (class_param || class_recv) && !self.man.is_method_lookup(&k) {
            self.field_lookup(m, off, mref, opcode, args, &classes, class_recv);
        }
        if self.man.is_field_enumerator(&k) {
            // 接收者 Class 值集里的类镜像逐类放开（值集增长时本站点重跑）；含所指未知的 Class 时全部放开，记为缺口
            // 放开与登记都幂等：只取值集中尚未处理的部分（站点重跑由值集增长驱动）
            let mut cs = BTreeSet::new();
            let unknown = match args.first() {
                None => true,
                Some(v) => {
                    let mut seen = self.refl_seen.entry(m).or_default().remove(&off).unwrap_or_default();
                    let u = self.class_values_new(m, v, &mut seen.fenum, &mut cs);
                    self.refl_seen.entry(m).or_default().insert(off, seen);
                    u
                }
            };
            // 序列化口径的调用方只用可序列化字段：已知的类与推不出的接收者都按可序列化字段放开。
            // 它们不取静态字段的值（computeDefaultSUID 只读名字与修饰符，默认序列化字段滤掉 static），
            // 枚举本身不初始化类（Class.getDeclaredFields 不触发 `<clinit>`），所以不按静态字段句柄初始化声明类；
            // computeDefaultSUID 对可序列化类的初始化由 hasStaticInitializer（class_initializers）建模
            let serial = self.man.is_serial_enumerator(&self.methods[m].key.to_string());
            if !serial {
                self.enumerated_static_owners(m, off, &cs);
            }
            let mut scopes: Vec<field_handles::EnumScope> = cs.into_iter().map(|c| (serial, Some(c))).collect();
            if unknown {
                scopes.push((serial, None));
                if !serial {
                    self.field_enum_gaps.insert(format!("{}@{off}", self.methods[m].key));
                }
            }
            for (_, c) in &scopes {
                if serial {
                    self.enumerate_serial_fields(c.clone());
                } else {
                    self.enumerate_fields(c.clone());
                }
            }
            // 结果句柄带各口径的来源标记：流到句柄写入口时才放开（`field_handles.rs`）
            self.mark_enumeration(m, off, &mref.desc, &scopes);
        }
        if self.man.is_deserializer(&k) && !self.ctx.deser.replace(true) {
            self.open_fields_all(self.ctx.fopen_all.get(), false);
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
        // 按键查找入口的结果先经闸门（`keyed.rs`）
        let res = Some(self.keyed_res(m, off, &resolved, pargs));
        let recv_feeds = |e: &mut Self| match recv_v {
            Some(v) => e.feeds(m, v, owner),
            None => vec![Feed::S(TypeSet::open(owner))],
        };
        match opcode {
            op::INVOKESTATIC => {
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
                    if !rm.is_static() && self.man.concrete.entries.contains(&*self.mref_key(&resolved)) {
                        let s = self.value_set(&r);
                        if self.concrete_call(m, off, &resolved, &md, Some(&s), pargs) {
                            return;
                        }
                    }
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
                // open(o) 的接收者（G 中 ⊂ o 者）被同一值集里另一 open 超类型的枢纽涵盖：只接后者，目标与结果相同
                let opens: Vec<u32> = s.open.iter().collect();
                for &o in &opens {
                    if opens.iter().any(|&p| p != o && self.sub(o, p)) {
                        continue;
                    }
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
        self.caller_edge(m, t);
        let is_static = self.methods[t].is_static;
        let ptypes = self.methods[t].ptypes.clone();
        let base = usize::from(!is_static);
        self.bind_params(m, t, base, ptypes.len());
        if let Some(cv) = self.call_vals.clone() {
            let string = self.id(STRING);
            self.pstr_site(m, off, &cv, |j| pstrs::PSlot::M(t, base + j), |j| ptypes.get(base + j).copied().flatten() == Some(string));
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
        // 调用方的内存效果已按清单逐调用点建模（`[facts.array_writes]` / `[facts.memory_reads]`）时，其手写体对内存访问
        // 成员的上调是同一语义的实现（VarHandle.set → Unsafe.putReference、putReferenceOpaque → putReference 等），
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
                        Feed::S(s) => self.mirror_into(op, s, res),
                    }
                }
            } else if model == RetModel::Receiver {
                // 浅拷贝：返回值 = 本调用点的接收者类型集（数组共享元素节点；逐调用点，不经被调方形参汇合）
                if let Some(fs) = &recv_fs {
                    self.feed(fs, res, rt);
                }
            } else if model == RetModel::Caller {
                self.caller_ret(m, t, res, rt);
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
    pub(super) fn bind_params(&mut self, m: usize, t: usize, base: usize, n: usize) {
        let cv = self.call_vals.clone();
        let vals: Option<Vec<PV>> = cv.as_ref().map(|vs| vs.iter().map(PV::of).collect());
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
        if vals.is_none() {
            self.pstr_top_m(t);
        }
        self.taint_params(t, base, n, vals);
        self.join_pvs(t, base, n, vals);
    }

    fn join_pvs(&mut self, t: usize, base: usize, n: usize, vals: Option<&[PV]>) {
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
