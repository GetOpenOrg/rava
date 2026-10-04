//! 引擎：字节码方法处理——absint 事件、返回值、常量、字段访问。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn process_bytecode(&mut self, m: usize) {
        let Some(a) = self.analysis(m) else { return };
        // 同一次装入：`applied` 就是当前这次装入的分析（共享摘要下 `Rc` 身份不足以判定）
        let seq = self.methods[m].aseq;
        let same = self.methods[m].applied.is_some() && self.methods[m].applied_seq == seq;
        if !same {
            self.sysprops_scan(m, &a);
            self.vm_rules_scan(m, &a);
            // 按名取类读过本方法调用点实参的读者（形参名字）重跑
            self.pstr_reanalyzed(m);
        }
        let owner = self.methods[m].key.owner.clone();
        let cf = self.h.class(&owner);
        self.returns(m, &a);
        self.methods[m].applied_seq = seq;
        let old = match self.methods[m].applied.replace(a.clone()) {
            // 首次 / 被调方摘要变化：站点按新摘要完整重接
            None => {
                self.reset_sites(m);
                None
            }
            // 同一分析重处理（open 展开的 G 增长）：站点去重记录仍成立
            Some(_) if same => {
                self.ctx.stats.borrow_mut().reprocess += 1;
                None
            }
            Some(o) => Some(o),
        };
        let Some(old) = old else {
            for (off, e) in a.events.iter() {
                self.event(m, *off, e, &cf);
            }
            return;
        };
        // 重分析：同一偏移处与已执行分析相同的事件效果已生效（其后的增长由读者登记 / 流边驱动），
        // 且站点去重记录对之仍成立；只执行变化了的偏移，并清掉这些偏移的去重记录
        fn at(x: &Analysis, off: u32) -> &[(u32, Event)] {
            let lo = x.events.partition_point(|e| e.0 < off);
            let hi = x.events.partition_point(|e| e.0 <= off);
            &x.events[lo..hi]
        }
        let same_at = |off: u32| at(&a, off) == at(&old, off);
        let mut offs: Vec<u32> = a.events.iter().map(|e| e.0).collect();
        offs.dedup();
        let mut changed: HashSet<u32> = offs.into_iter().filter(|&o| !same_at(o)).collect();
        // 跨偏移读者（按名查找）的求值依赖本方法其它偏移的事件与辅助方法读过的字段：重分析即一并重跑
        // （事件全同也要重跑——失效可能来自辅助方法独立分析读过的字段转为不折叠）
        if let Some(xs) = self.xreaders.get(&m) {
            changed.extend(xs.iter().copied());
        }
        if changed.is_empty() {
            return;
        }
        self.reset_offsets(m, &changed);
        for (off, e) in a.events.iter() {
            if changed.contains(off) {
                self.event(m, *off, e, &cf);
            }
        }
    }

    /// 读者站点的类型集增长：只重跑该偏移处的事件（分析已失效时方法整体在队列里）
    pub(super) fn rerun_site(&mut self, m: usize, off: u32) {
        let Some(a) = self.methods[m].analysis.clone() else {
            self.push_m(m);
            return;
        };
        if self.in_mwork.contains(&m) {
            return;
        }
        let cf = self.h.class(&self.methods[m].key.owner);
        let lo = a.events.partition_point(|e| e.0 < off);
        for (o, e) in a.events[lo..].iter().take_while(|e| e.0 == off) {
            let t0 = std::time::Instant::now();
            self.event(m, *o, e, &cf);
            let c = &mut self.ctx.stats.borrow_mut().rerun_by_event[super::stats::rerun_kind(e)];
            c[0] += 1;
            c[1] += t0.elapsed().as_nanos() as u64;
        }
    }

    pub(super) fn event(&mut self, m: usize, off: u32, e: &Event, cf: &Option<std::sync::Arc<ClassFile>>) {
        if self.cuts.active() {
            let c = &self.cuts;
            // 方法级切除也要挡住读者站点重跑（不经 process）
            let k = self.methods[m].key.to_string();
            if c.method(&k) || c.site(&k, off) {
                return;
            }
        }
        let prev = self.cur_site.replace((m, off));
        self.event_inner(m, off, e, cf);
        self.cur_site = prev;
    }

    pub(super) fn event_inner(&mut self, m: usize, off: u32, e: &Event, cf: &Option<std::sync::Arc<ClassFile>>) {
        let obj = self.id(OBJECT);
        {
            let via = |k: &'static str| Via::method(k, m, Some(off));
            match e {
                Event::New(c) => {
                    self.instantiate(c, via("new"));
                    self.init(c, via("new"));
                    // 容器形态类按分配点（+ 堆上下文）成为抽象对象；G 里记类型本身（open 展开用）
                    let id = if self.container(c) { self.obj_at(m, off, c) } else { self.id(c) };
                    self.add_to(Node::S(m, off), &TypeSet::exact(id));
                }
                Event::NewArray(t, empty) => {
                    let id = self.array_site(m, off, t, *empty, via("newarray"));
                    self.add_to(Node::S(m, off), &TypeSet::exact(id));
                }
                Event::Ldc(c) => self.ldc(m, off, c),
                Event::CheckCast(c, v) => {
                    self.touch(c, Level::Type, via("checkcast"));
                    // 转换结果是独立来源：只收输入中 ⊂ 目标类型的部分（转换失败的值到不了后继）
                    if let Some(v) = v {
                        let cid = self.id(c);
                        let fs = self.feeds(m, v, cid);
                        self.feed(&fs, Node::S(m, off), cid);
                    }
                }
                Event::InstanceOf(c, v) => {
                    self.touch(c, Level::Type, via("instanceof"));
                    // 判定成立一侧的收窄值：输入中 ⊂ 目标类型的部分
                    if let Some(v) = v {
                        let cid = self.id(c);
                        let fs = self.feeds(m, v, cid);
                        self.feed(&fs, Node::S(m, off), cid);
                    }
                }
                Event::NotInstance(c, v) => {
                    // 判定不成立一侧的收窄值：输入中 ⊄ 目标类型的部分（null 不入类型集）
                    let cid = self.id(c);
                    let obj = self.id(OBJECT);
                    let fs = self.feeds(m, v, obj);
                    self.feed(&fs, Node::S(m, off), NOT_SUB | cid);
                }
                Event::Catch(ct) => {
                    let t = ct.clone().unwrap_or_else(|| THROWABLE.to_string());
                    self.touch(&t, Level::Type, via("catch"));
                    let id = self.id(&t);
                    self.add_to(Node::S(m, CATCH | off), &TypeSet::open(id));
                }
                Event::Field { opcode, mref, recv, value } => {
                    self.field(m, off, *opcode, mref, recv.as_ref(), value.as_ref(), Node::S(m, off))
                }
                Event::Invoke { opcode, mref, iface, args } => self.invoke(m, off, *opcode, mref, *iface, args),
                Event::Indy { bsm, name, desc, args } => {
                    if let Some(cf) = &cf {
                        self.indy(m, off, cf, *bsm, name, desc, args);
                    }
                }
                Event::ArrayLoad { array, index } => {
                    let aty = array.static_type().map(str::to_string);
                    let comp = aty.as_deref().and_then(absint::component);
                    let tid = self.id(comp.as_deref().unwrap_or(OBJECT));
                    let aid = aty.map(|t| self.id(&t)).unwrap_or(obj);
                    let fs = self.feeds(m, array, aid);
                    let s = self.value_set(&fs);
                    let mut add = TypeSet::default();
                    let xs: Vec<u32> = s.classes.iter().filter(|x| self.arrays.contains_key(x)).collect();
                    if xs.len() < s.classes.len() {
                        add.open.insert(tid);
                    }
                    // 分配点多时经汇集节点汇集（与逐分配点接边同集合，见 `gather.rs`）
                    if !xs.is_empty() {
                        for p in slots(index) {
                            self.gather_elems(m, off, p, tid, &xs, Node::S(m, off));
                        }
                    }
                    // open 数组（手写层 / VM 产出）的元素同样 open
                    for o in &s.open {
                        if let Some(c) = absint::component(&self.names[o as usize].clone()) {
                            let cid = self.id(&c);
                            if self.sub(cid, tid) {
                                add.open.insert(cid);
                            } else if self.sub(tid, cid) {
                                add.open.insert(tid);
                            }
                        } else if o == obj {
                            add.open.insert(tid);
                        }
                    }
                    self.add_to(Node::S(m, off), &add);
                }
                Event::ArrayStore { array, index, value } => {
                    let aty = array.static_type().map(str::to_string);
                    let comp = aty.as_deref().and_then(absint::component);
                    let tid = self.id(comp.as_deref().unwrap_or(OBJECT));
                    let aid = aty.map(|t| self.id(&t)).unwrap_or(obj);
                    let fs = self.feeds(m, value, tid);
                    let afs = self.feeds(m, array, aid);
                    let s = self.value_set(&afs);
                    for x in &s.classes {
                        // 按分配点的实际分量类型收窄（静态类型可能更宽，如经 Object[] 视角写入 Class[]）
                        let Some(&at) = self.arrays.get(&x) else { continue };
                        let Some(c) = absint::component(&self.names[at as usize].clone()).filter(|c| c.len() > 1) else { continue };
                        let cx = self.id(&c);
                        for p in slots(index) {
                            self.feed(&fs, Node::E(x, p), cx);
                        }
                    }
                    // 数组值未知（保守分析）：可能是任一分配点数组。open 数组来自手写层 / VM，
                    // 其读取已按 open(分量) 处理，写入无需回灌分配点
                    if matches!(array, V::Top) {
                        self.feed(&fs, Node::Array, tid);
                    }
                }
                Event::Return(v) => {
                    if let Some(rt) = self.methods[m].rtype {
                        let fs = self.feeds(m, v, rt);
                        self.feed(&fs, Node::R(m), rt);
                    }
                }
                Event::Throw(_) | Event::Const { .. } => {}
            }
        }
    }

    /// 返回常量 = 各返回路径值的并；变化时查询过它的调用方失效
    pub(super) fn returns(&mut self, m: usize, a: &Analysis) {
        let mut r: Option<PV> = None;
        for (_, e) in &a.events {
            if let Event::Return(v) = e {
                r = Some(PV::join(r.as_ref(), &PV::of_ret(v)));
            }
        }
        let Some(r) = r else { return };
        self.sysprops_rval(m, a, &r);
        let key = self.methods[m].key.clone();
        let cur = self.ctx.rvals.borrow().get(&key).cloned();
        let new = PV::join(cur.as_ref(), &r);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.ctx.rvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.rdeps.borrow().get(&key).cloned();
        self.invalidate_all(deps, Why::RetConst);
    }

    pub(super) fn ldc(&mut self, m: usize, off: u32, c: &Const) {
        let via = Via::method("ldc", m, Some(off));
        match c {
            Const::String(_) | Const::StringUtf16(_) => {
                self.instantiate(STRING, via);
            }
            Const::Class(n) => {
                self.touch(n, Level::Type, via.clone());
                self.instantiate(CLASS, via);
                let k = self.mirror(n);
                self.add_to(Node::S(m, off), &TypeSet::exact(k));
            }
            Const::MethodType(d) => self.touch_desc(d, &via),
            Const::MethodHandle(mh) => {
                let mh = mh.clone();
                self.invoke_mh(m, off, &mh);
            }
            Const::Dynamic(bsm, _, d) => {
                self.touch_desc(d, &via);
                // 动态常量的值由引导方法产出：按声明类型 open
                if let Some(t) = parse_field(d).and_then(|ft| self.ptype(&ft)) {
                    self.add_to(Node::S(m, off), &TypeSet::open(t));
                }
                let owner = self.methods[m].key.owner.clone();
                if let Some(cf) = self.h.class(&owner) {
                    if let Some(b) = cf.bootstrap_methods.get(*bsm as usize) {
                        let h = b.handle.clone();
                        self.invoke_mh(m, off, &h);
                        for a in b.args.clone() {
                            if let Const::MethodHandle(x) = a {
                                self.invoke_mh(m, off, &x);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// 字段访问：写入值来源 → 字段节点；读 → 结果节点 `res`。实例字段按接收者拆分：
    /// 抽象对象（容器分配点）走各自的字段节点，其余接收者（open / 非抽象对象 / 未知）写 `U`、读 `F`
    #[allow(clippy::too_many_arguments)]
    pub(super) fn field(&mut self, m: usize, off: u32, opcode: u8, f: &MemberRef, recv: Option<&V>, value: Option<&V>, res: Node) {
        use classfile::op;
        // 字节码站点重跑（接收者集合增长）：与值无关的部分（登记类 / 值集 / 手写访问器）与未知接收者视图
        // 各只接一次，只处理新增抽象对象（`recv_done` 以哨兵登记，同一分析结果下成立）
        let fresh = self.methods[m].kind == Kind::Bytecode && res == Node::S(m, off);
        let first = !fresh || self.recv_mark(m, off, FIELD_STATIC);
        let via = Via::method("field", m, Some(off));
        let Some(site) = self.h.resolve_field(&f.owner, &f.name, &f.desc) else {
            self.touch(&f.owner, Level::Type, via);
            self.unresolved.insert(f.to_string());
            return;
        };
        let decl = site.class.name.clone();
        if first {
            // 字段读写按属主的字段访问器发射（静态字段经属主类名访问）：属主至少 L2
            self.touch(&f.owner, Level::Layout, via.clone());
            self.touch_desc(&f.desc, &via);
            self.nest_access(m, off, &decl, site.field().access & acc::PRIVATE != 0);
            if opcode == op::GETSTATIC || opcode == op::PUTSTATIC {
                self.init(&decl, via.clone());
            }
            // 接收者钩子（`receiver = true`）在有接收者值集时按值集判定（见下）；静态钩子与其余访问点
            // 无条件接入——静态钩子（如 initPhase3 段）不看接收者，实例字段读写同样先执行它
            let recv_hook = instance_op(opcode)
                && recv.is_some()
                && self.recv_hook_field(&decl, f);
            if !recv_hook {
                self.field_hook(&decl, f, opcode, &via, res);
            }
        }
        if first && (opcode == op::PUTSTATIC || opcode == op::PUTFIELD) {
            let fd = site.field();
            // static final 由 `<clinit>` 常量求值（Ctx::static_const），其余字段并入值集
            if fd.access & acc::STATIC == 0 || fd.access & acc::FINAL == 0 {
                let key = MemberRef { owner: decl.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
                self.field_put(&key, value.map_or(PV::Top, PV::of));
                self.field_strs_put(&key, value);
                if key.desc == format!("L{};", absint::STRING) {
                    let fi = self.field_node(key);
                    self.pstr_field_put(m, fi, value);
                }
            }
        }
        let Some(ft) = parse_field(&f.desc) else { return };
        let Some(tid) = self.ptype(&ft) else {
            // 手写字段访问器仍需沿其回调入链；基本类型字段无值集，接收者钩子在此无条件接入
            if first {
                if instance_op(opcode) && recv.is_some() && self.recv_hook_field(&decl, f) {
                    self.field_hook(&decl, f, opcode, &via, res);
                }
                self.field_handwritten(&decl, &f.name, &f.desc, &via, None);
            }
            return;
        };
        let key = MemberRef { owner: decl.clone(), name: f.name.clone(), desc: f.desc.clone() };
        let fi = self.field_node(key);
        let instance = opcode == op::GETFIELD || opcode == op::PUTFIELD;
        let (objs, other) = match recv.filter(|_| instance) {
            Some(v) => {
                let oid = self.id(&f.owner);
                let fs = self.feeds(m, v, oid);
                let s = self.value_set(&fs);
                // 字节码站点重跑：只看新增的接收者值（过滤结果按值确定，已看过的值已接上）
                let s = if fresh { self.recv_delta(m, off, s) } else { s };
                let s = self.filter(&s, oid);
                if self.recv_hook_needed(&decl, f, &s) {
                    self.field_hook(&decl, f, opcode, &via, res);
                }
                let objs: Vec<u32> = s.classes.iter().filter(|x| self.objs.contains_key(x)).collect();
                (objs.clone(), !s.open.is_empty() || s.classes.len() > objs.len())
            }
            None => (vec![], true),
        };
        let other = other && (!fresh || self.recv_mark(m, off, FIELD_OTHER));
        let nodes: Vec<Node> = objs.iter().map(|&o| self.obj_field(o, fi, tid)).collect();
        if opcode == op::PUTSTATIC || opcode == op::PUTFIELD {
            let fs = match value {
                Some(v) => self.feeds(m, v, tid),
                None => vec![Feed::S(TypeSet::open(tid))],
            };
            if fresh {
                // 字节码写站点：抽象对象多时经汇集节点分发（与逐对象接边同集合，见 `gather.rs`）
                if !objs.is_empty() {
                    self.gather_write(m, off, fi, tid, &objs, &fs);
                }
            } else {
                for n in nodes {
                    self.feed(&fs, n, tid);
                }
            }
            if other {
                self.feed(&fs, Node::U(fi), tid);
            }
            // 边界类字段 / 有手写访问器的字段：写入值由手写层读出
            if first && (matches!(self.domain(&decl), Domain::Boundary | Domain::Root) || !self.hw.member(&decl, &f.name).fns.is_empty()) {
                self.feed(&fs, Node::Esc, tid);
            }
        } else if fresh {
            // 字节码读站点：抽象对象多时经汇集节点汇集（与逐对象接边同集合，见 `gather.rs`）
            if !objs.is_empty() {
                self.gather_read(m, off, fi, tid, &objs, res);
            }
            if other {
                self.flow(Node::F(fi), res, tid);
            }
            if first {
                self.field_handwritten(&decl, &f.name, &f.desc, &via, Some((fi, tid)));
            }
        } else {
            for n in nodes {
                self.flow(n, res, tid);
            }
            if other {
                self.flow(Node::F(fi), res, tid);
            }
            if first {
                self.field_handwritten(&decl, &f.name, &f.desc, &via, Some((fi, tid)));
            }
        }
    }

    /// 字段读：声明类是边界类（struct 与字段整体手写）或字段有手写访问器 → 按 open 处理（公开 API 类的
    /// 手写写入经 `__set_` 在 [`Self::hw_fields`] 精确接入）；手写访问器声明的回调入链
    pub(super) fn field_handwritten(&mut self, decl: &str, name: &str, fdesc: &str, via: &Via, node: Option<(usize, u32)>) {
        let boundary = matches!(self.domain(decl), Domain::Boundary | Domain::Root);
        let mh = self.hw.member(decl, name);
        if let Some((fi, tid)) = node {
            // 边界类字段，或值由手写访问器提供（如标准流 `System::out()`）：按 open 处理。清单字段钩子
            // （`[vm_state.field_hooks]`，如 Class.classLoader 的定义加载器）的 VM 写入即钩子本身，读站点已
            // 接钩子值池（`field_hook`），不再按 open 处理
            let hooked = self.man.vm_state.field_hook(decl, name, fdesc).is_some();
            if (boundary && !hooked) || !mh.fns.is_empty() {
                self.add_to(Node::U(fi), &TypeSet::open(tid));
            }
        }
        if mh.fns.is_empty() {
            return;
        }
        // 访问器体按独立伪方法节点建模（回调实参取访问器自己的值池，不取读取方的）
        self.hwfield_method(decl, name, fdesc, via.clone());
    }
}


impl Engine<'_> {
    /// 站点 (m, off) 登记已接上的接收者对象（或哨兵）x；首次登记返回 true
    /// 批量登记升序的对象 xs，返回其中新登记的（升序）；一趟归并，免逐个插入的搬移
    pub(super) fn recv_mark_all(&mut self, m: usize, off: u32, xs: &[u32]) -> Vec<u32> {
        let v = self.recv_done.entry(m).or_default().entry(off).or_default();
        let mut new = Vec::new();
        let mut i = 0;
        for &x in xs {
            while i < v.len() && v[i] < x {
                i += 1;
            }
            if i == v.len() || v[i] != x {
                new.push(x);
            }
        }
        if !new.is_empty() {
            let old = std::mem::take(v);
            let mut out = Vec::with_capacity(old.len() + new.len());
            let (mut a, mut b) = (old.iter().peekable(), new.iter().peekable());
            while let (Some(&&p), Some(&&q)) = (a.peek(), b.peek()) {
                if p < q {
                    out.push(p);
                    a.next();
                } else {
                    out.push(q);
                    b.next();
                }
            }
            out.extend(a);
            out.extend(b);
            *v = out;
        }
        new
    }

    /// 站点 (m, off) 的接收者值集 s 中尚未接过的部分（精确值与 open 值分别登记，open 值以 `OPEN_MARK` 区分）
    pub(super) fn recv_delta(&mut self, m: usize, off: u32, s: TypeSet) -> TypeSet {
        let cs: Vec<u32> = s.classes.iter().collect();
        let classes = IdSet::from_sorted(self.recv_mark_all(m, off, &cs));
        let open = if s.open.is_empty() {
            s.open
        } else {
            let os: Vec<u32> = s.open.iter().map(|o| o | OPEN_MARK).collect();
            IdSet::from_sorted(self.recv_mark_all(m, off, &os).into_iter().map(|o| o & !OPEN_MARK).collect())
        };
        TypeSet { classes, open }
    }

    pub(super) fn recv_mark(&mut self, m: usize, off: u32, x: u32) -> bool {
        let v = self.recv_done.entry(m).or_default().entry(off).or_default();
        match v.binary_search(&x) {
            Ok(_) => false,
            Err(i) => {
                v.insert(i, x);
                true
            }
        }
    }
}

fn instance_op(opcode: u8) -> bool {
    opcode == classfile::op::GETFIELD || opcode == classfile::op::PUTFIELD
}
