//! 引擎：手写节点——手写方法的调用点、字段、返回与回调建模。

use super::*;

impl<'a> Engine<'a> {
    // ── 手写节点 ────────────────────────────────────────────────────────────

    /// 手写方法的数组写入（arraycopy、Unsafe 引用写入等）按调用点建模：写入目标是本调用点实参里的数组，
    /// 写入值按 `hw_writes` 给出的来源（实参值 / 实参数组的元素 / 手写体产出）逐调用点接入。
    /// 不经被调方法的形参汇合：arraycopy 等被全程序共享，汇合会把所有数组的元素并成同一个集合
    pub(super) fn hw_site(&mut self, m: usize, off: u32, t: usize, recv: Option<&[Feed]>, a: &[Option<Vec<Feed>>]) {
        let ws = self.hw_writes(t);
        if ws.iter().all(Option::is_none) || self.hw_site_ids.contains_key(&(m, off, t)) {
            return;
        }
        let s = self.hw_site_id(m, off, t);
        let base = usize::from(!self.methods[t].is_static);
        let obj = self.id(OBJECT);
        // 签名多态：实参按调用点描述符排布（VM 打包进 Object[]），引用实参一律按 Object 接入
        let poly = self.is_poly(t);
        let ptypes = if poly {
            let mut p = self.methods[t].ptypes[..base].to_vec();
            p.extend(a.iter().map(|f| f.as_ref().map(|_| obj)));
            p
        } else {
            self.methods[t].ptypes.clone()
        };
        let feeds: Vec<Option<&[Feed]>> = (0..ptypes.len()).map(|i| if i < base { recv } else { a.get(i - base).and_then(|f| f.as_deref()) }).collect();
        let mut watched: BTreeSet<usize> = BTreeSet::new();
        for (j, w) in ws.iter().enumerate() {
            let Some(w) = w else { continue };
            let wn = Node::W(s, j as u16);
            if w.produced {
                self.flow(Node::S(t, PROD), wn, obj);
            }
            let last = (w.last && ptypes.len() > base).then(|| ptypes.len() - 1);
            if last.is_some() && poly {
                self.poly_write(wn);
            }
            for &i in w.values.iter().chain(last.iter()) {
                if let (Some(Some(pi)), Some(Some(fs))) = (ptypes.get(i), feeds.get(i)) {
                    self.feed(fs, wn, *pi);
                }
            }
            watched.insert(j);
            watched.extend(w.elements.iter().copied());
        }
        // 实参节点最后接入：新增数组经 add_to 钩子接上元素读写
        for i in watched {
            if let (Some(Some(pi)), Some(Some(fs))) = (ptypes.get(i), feeds.get(i)) {
                self.feed(fs, Node::A(s, i as u16), *pi);
            }
        }
    }

    pub(super) fn hw_site_id(&mut self, m: usize, off: u32, t: usize) -> u32 {
        if let Some(&s) = self.hw_site_ids.get(&(m, off, t)) {
            return s;
        }
        let s = self.hw_sites.len() as u32;
        self.hw_site_ids.insert((m, off, t), s);
        self.hw_sites.push((m, off, t));
        s
    }

    /// 读内存的手写调用点：结果 = 本调用点源实参所指各对象的元素 / 引用字段（逐调用点，不经 R 汇合）
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hw_read_site(&mut self, m: usize, off: u32, t: usize, i: u16, fs: &[Feed], res: Node, rt: u32) {
        let s = self.hw_site_id(m, off, t);
        if self.hw_reads.insert(s, (i, res, rt)).is_some() {
            return;
        }
        let pt = self.methods[t].ptypes.get(i as usize).copied().flatten().unwrap_or_else(|| self.id(OBJECT));
        let a = Node::A(s, i);
        let cur = self.set_of(a);
        if !cur.is_empty() {
            self.memory_read(s, &cur);
        }
        self.feed(fs, a, pt);
    }

    pub(super) fn memory_read(&mut self, s: u32, delta: &TypeSet) {
        let (_, res, rt) = self.hw_reads[&s];
        if !delta.open.is_empty() {
            self.add_to(res, &TypeSet::open(rt));
        }
        for &x in delta.classes.iter() {
            if self.arrays.contains_key(&x) {
                for p in PARITIES {
                    self.flow(Node::E(x, p), res, rt);
                }
                continue;
            }
            let (cls, obj) = match self.objs.get(&x) {
                Some(&c) => (c, true),
                None => (x, false),
            };
            for (fi, tid) in self.ref_fields(cls).iter().copied() {
                let n = if obj { self.obj_field(x, fi, tid) } else { Node::F(fi) };
                self.flow(n, res, rt);
            }
        }
    }

    pub(super) fn ref_fields(&mut self, cls: u32) -> Rc<[(usize, u32)]> {
        if let Some(r) = self.ref_fields.get(&cls) {
            return r.clone();
        }
        let mut out = Vec::new();
        let mut cur = self.h.class(&self.names[cls as usize].clone());
        while let Some(cf) = cur {
            for f in cf.fields.iter().filter(|f| !f.is_static()) {
                let Some(tid) = parse_field(&f.desc).and_then(|t| self.ptype(&t)) else { continue };
                let fi = self.field_node(MemberRef { owner: cf.name.clone(), name: f.name.clone(), desc: f.desc.clone() });
                out.push((fi, tid));
            }
            cur = cf.super_name.as_deref().and_then(|s| self.h.class(s));
        }
        let r: Rc<[(usize, u32)]> = out.into();
        self.ref_fields.insert(cls, r.clone());
        r
    }

    /// 手写方法按形参（含接收者序号）的数组写入来源。`[facts.array_writes]` 声明的按声明；
    /// 其余取得数组视图的手写体保守处理：每个非接收者引用形参都可被写入，来源为其它形参的值、
    /// 全部实参数组的元素与手写体产出。数组只有 Object 的方法，没有一个改写元素，接收者不是写入目标
    pub(super) fn hw_writes(&mut self, t: usize) -> Rc<[Option<HwWrite>]> {
        if let Some(w) = self.hw_writes.get(&t) {
            return w.clone();
        }
        let base = usize::from(!self.methods[t].is_static);
        let key = self.methods[t].key.clone();
        let ptypes = self.methods[t].ptypes.clone();
        let refs: Vec<usize> = (0..ptypes.len()).filter(|&i| ptypes[i].is_some()).collect();
        let w: Rc<[Option<HwWrite>]> = match self.man.array_writes(&key.to_string()) {
            Some(d) => (0..ptypes.len())
                .map(|j| {
                    (d.dst.map(|x| x + base) == Some(j)).then(|| HwWrite {
                        values: d.values.iter().map(|x| x + base).collect(),
                        elements: d.elements.iter().map(|x| x + base).collect(),
                        produced: d.produced,
                        fields: d.fields,
                        last: d.last,
                    })
                })
                .collect(),
            None => {
                let access = self.h.class(&key.owner).is_some_and(|cf| self.hw_member(&cf, &key.name, &key.desc).array_access);
                (0..ptypes.len())
                    .map(|j| {
                        (access && j >= base && ptypes[j].is_some()).then(|| HwWrite {
                            values: refs.iter().copied().filter(|&i| i != j).collect(),
                            elements: refs.clone(),
                            produced: true,
                            fields: false,
                            last: false,
                        })
                    })
                    .collect()
            }
        };
        self.hw_writes.insert(t, w.clone());
        w
    }

    /// 调用点 s 的第 i 个实参新增数组 ys：元素来源含 i 的写入目标接上其元素；i 是写入目标则接收写入
    pub(super) fn hw_site_arrays(&mut self, s: u32, i: u16, ys: &[u32]) {
        let (_, _, t) = self.hw_sites[s as usize];
        let ws = self.hw_writes(t);
        let obj = self.id(OBJECT);
        for &y in ys {
            for (j, w) in ws.iter().enumerate() {
                if w.as_ref().is_some_and(|w| w.elements.contains(&(i as usize))) {
                    for p in PARITIES {
                        self.flow(Node::E(y, p), Node::W(s, j as u16), obj);
                    }
                }
            }
            if ws.get(i as usize).is_none_or(Option::is_none) {
                continue;
            }
            let t = self.arrays[&y];
            let Some(c) = absint::component(&self.names[t as usize].clone()).filter(|c| c.len() > 1) else { continue };
            let cid = self.id(&c);
            for p in PARITIES {
                self.flow(Node::W(s, i), Node::E(y, p), cid);
            }
        }
    }

    fn is_poly(&self, t: usize) -> bool {
        let key = &self.methods[t].key;
        self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(resolve::is_signature_polymorphic)).unwrap_or(false)
    }

    /// 签名多态写入调用点：写入值接到按名打开的静态引用字段（静态字段句柄没有 holder 坐标）
    fn poly_write(&mut self, wn: Node) {
        self.poly_writes.push(wn);
        for (fi, tid) in self.open_statics.clone() {
            self.flow(wn, Node::U(fi), tid);
        }
    }

    /// 字段按名打开：静态引用字段接收签名多态写入调用点的写入值
    pub(super) fn open_static(&mut self, key: &MemberRef) {
        let is_static = self.h.class(&key.owner).and_then(|cf| cf.field(&key.name, &key.desc).map(|f| f.is_static())).unwrap_or(false);
        let Some(tid) = parse_field(&key.desc).and_then(|t| self.ptype(&t)).filter(|_| is_static) else { return };
        let fi = self.field_node(key.clone());
        if self.open_statics.iter().any(|&(f, _)| f == fi) {
            return;
        }
        self.open_statics.push((fi, tid));
        for wn in self.poly_writes.clone() {
            self.flow(wn, Node::U(fi), tid);
        }
    }

    /// 调用点 s 的写入目标实参 i 新增对象：写入值接到对象的引用实例字段（与 `memory_read` 对称）。
    /// 抽象对象按对象分量接入；非抽象类 / open 目标接未知接收者写入节点，open 目标的子类字段不可枚举，
    /// 写入值随之逃逸
    pub(super) fn hw_site_fields(&mut self, s: u32, i: u16, delta: &TypeSet) {
        let (_, _, t) = self.hw_sites[s as usize];
        if !self.hw_writes(t).get(i as usize).is_some_and(|w| w.as_ref().is_some_and(|w| w.fields)) {
            return;
        }
        let wn = Node::W(s, i);
        let obj = self.id(OBJECT);
        let xs: Vec<u32> = delta.classes.iter().copied().filter(|x| !self.arrays.contains_key(x)).collect();
        for x in xs {
            let (cls, is_obj) = match self.objs.get(&x) {
                Some(&c) => (c, true),
                None => (x, false),
            };
            for (fi, tid) in self.ref_fields(cls).iter().copied() {
                let n = if is_obj { self.obj_field(x, fi, tid) } else { Node::U(fi) };
                self.flow(wn, n, tid);
            }
        }
        let os: Vec<u32> = delta.open.iter().copied().collect();
        for o in os {
            for (fi, tid) in self.ref_fields(o).iter().copied() {
                self.flow(wn, Node::U(fi), tid);
            }
            self.flow(wn, Node::Esc, obj);
        }
    }

    pub(super) fn process_handwritten(&mut self, m: usize) {
        let key = self.methods[m].key.clone();
        let via = Via::method("handwritten", m, None);
        // 手写返回值：open(返回类型)
        let reads = self.man.memory_read(&key.to_string()).is_some();
        if let Some(rt) = self.methods[m].rtype {
            if !self.man.returns_receiver(&key.to_string()) && !reads {
                self.add_to(Node::R(m), &TypeSet::open(rt));
            }
        }
        if key.name == "<clinit>" {
            return;
        }
        let ks = key.to_string();
        if let Some(k) = self.man.member_enumerator(&ks) {
            let n = Node::P(m, 0);
            if self.enum_recv.insert(n, (k, m)).is_none() {
                let s = self.set_of(n);
                self.rpending.push((k, m, s));
            }
        }
        for &k in self.man.member_invoker(&ks) {
            if self.invokable.insert(k) {
                let es: Vec<(Members, u32)> = self.enumerated.iter().filter(|e| Self::invoked_by(e.0) == k).copied().collect();
                for (a, c) in es {
                    self.expose(a, c);
                }
                if k == Members::Methods {
                    let mut named: Vec<u32> = self.reflect_names.keys().copied().collect();
                    named.sort_unstable();
                    for c in named {
                        self.expose(Members::Methods, c);
                    }
                }
            }
        }
        // 形参与手写体产出汇入值池（回调实参取自值池）
        let pts = self.methods[m].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            if let Some(pt) = pt {
                self.flow(Node::P(m, i as u16), Node::S(m, POOL), *pt);
            }
        }
        let obj = self.id(OBJECT);
        self.flow(Node::S(m, PROD), Node::S(m, POOL), obj);
        let Some(cf) = self.h.class(&key.owner) else { return };
        self.touch_truncated_body(m, &cf, &key, &via);
        let mh = self.hw_member(&cf, &key.name, &key.desc);
        // 返回值已精确建模（内存读取 / 接收者浅拷贝 / 类镜像）时不经 open 返回值交出
        let modeled = reads || self.man.returns_receiver(&ks) || self.man.returns_mirror(&ks);
        let rt = self.methods[m].rtype.filter(|_| !modeled);
        let is_static = self.methods[m].is_static;
        for t in self.hw_exports(&key.owner, &mh, rt, is_static) {
            self.flow(Node::S(m, POOL), Node::Esc, t);
        }
        if self.methods[m].hw_fns.is_empty() {
            self.methods[m].hw_fns = mh.fns.clone();
        }
        let owner = key.owner.clone();
        self.apply_hw(m, &owner, &mh, &via);
    }

    /// 手写体把值池里的值交回建模代码的类型：open 返回值（返回类型）、手写体按名访问字段的对象（字段声明类，
    /// 读出的字段值经未知接收者视图取得）、接收者推不出的回调（回调声明类，按 open 分派到非抽象接收者）。
    /// 值池中只有这些类型的抽象对象会以 open / 非抽象接收者的身份重新出现
    pub(super) fn hw_exports(&mut self, host: &str, mh: &MemberHw, rt: Option<u32>, is_static: bool) -> BTreeSet<u32> {
        let mut out: BTreeSet<u32> = rt.into_iter().collect();
        for fa in &mh.fields {
            // 接收者自身字段按接收者对象逐个接入（self_field_objs），接收者无须逃逸
            if fa.on_self && !is_static && fa.recv.as_ref().and_then(|r| self.stype_class(host, r)).and_then(|c| self.field_by_name(&c, &fa.field)).is_some() {
                continue;
            }
            let decl = fa.recv.as_ref().and_then(|r| self.stype_class(host, r)).and_then(|c| self.field_by_name(&c, &fa.field));
            match decl {
                Some((d, _)) => {
                    out.insert(self.id(&d));
                }
                None => {
                    let owners: Vec<String> = self.fields.keys().filter(|k| k.name == fa.field).map(|k| k.owner.clone()).collect();
                    for o in owners {
                        out.insert(self.id(&o));
                    }
                }
            }
        }
        for u in &self.hw_upcalls(host, mh) {
            if let Upcall::Method(u) = u {
                if u.name != "<init>" {
                    out.insert(self.id(&u.owner));
                }
            }
        }
        out
    }

    /// 精确匹配（mangle 名 / 无重载裸名）优先，否则按名字前缀（安全过近似）
    pub(super) fn hw_member(&self, cf: &ClassFile, name: &str, desc: &str) -> crate::handwritten::MemberHw {
        let all = self.hw.member(&cf.name, name);
        let (rust, mangled) = self.rust_names(cf, name, desc);
        let exact: Vec<String> = all
            .fns
            .iter()
            .filter(|f| {
                let f = f.strip_prefix("__impl_").unwrap_or(f);
                f == mangled || rust.as_deref() == Some(f)
            })
            .cloned()
            .collect();
        if exact.is_empty() || exact.len() == all.fns.len() {
            return all;
        }
        let hwc = self.hw.class(&cf.name);
        let mut out = crate::handwritten::MemberHw::default();
        for f in &exact {
            let i = &hwc.fns[f];
            out.provided |= i.is_pub;
            out.upcalls.extend(i.upcalls.iter().cloned());
            out.allocs.extend(i.allocs.iter().cloned());
            out.ctors.extend(i.ctors.iter().cloned());
            out.calls.extend(i.calls.iter().cloned());
            out.opaque.extend(i.opaque.iter().cloned());
            out.fields.extend(i.fields.iter().cloned());
            out.array_access |= i.array_access;
        }
        out.fns = exact;
        out
    }

    pub(super) fn resolve_tref(&self, host: &str, t: &TypeRef) -> Option<String> {
        self.hw.resolve_type(host, t).into_iter().find(|c| self.cp.contains(c))
    }

    /// 手写体调用点推断出的具体类型（须是可实例化的类）
    pub(super) fn hw_type(&mut self, host: &str, t: &Option<TypeRef>) -> Option<u32> {
        let c = self.resolve_tref(host, t.as_ref()?)?;
        let cf = self.h.class(&c)?;
        if cf.is_interface() || cf.access & acc::ABSTRACT != 0 {
            return None;
        }
        Some(self.id(&c))
    }

    /// 回调 / 构造在手写体里的调用点：同名（Rust 名规则）、实参个数一致；
    /// 同名标识符出现在宏内（syn 不展开）或找不到调用点 → None（退回值池）
    pub(super) fn hw_sites<'b>(mh: &'b MemberHw, matches: impl Fn(&TypedCall) -> bool, java_name: &str, nargs: usize) -> Option<Vec<&'b TypedCall>> {
        if mh.opaque.iter().any(|i| member_matches(i, java_name)) {
            return None;
        }
        let v: Vec<&TypedCall> = mh.calls.iter().filter(|c| c.args.len() == nargs && matches(c)).collect();
        (!v.is_empty()).then_some(v)
    }

    /// 实参来源：各调用点该位置都推断出具体类型 → 精确类型集；否则值池
    pub(super) fn hw_args(&mut self, m: usize, host: &str, desc: &str, sites: &Option<Vec<&TypedCall>>) -> Args {
        let pool = Node::S(m, POOL);
        let Some(md) = parse_method(desc) else { return vec![] };
        let mut out = Vec::with_capacity(md.params.len());
        for (j, p) in md.params.iter().enumerate() {
            if self.ptype(p).is_none() {
                out.push(None);
                continue;
            }
            let mut set = TypeSet::default();
            let mut ok = sites.is_some();
            for c in sites.iter().flatten() {
                match self.hw_type(host, &c.args[j]) {
                    Some(id) => {
                        set.classes.insert(id);
                    }
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            out.push(Some(if ok { vec![Feed::S(set)] } else { vec![Feed::N(pool)] }));
        }
        out
    }

    /// 类（含超类 / 超接口）中按名字找字段 → (声明类, 描述符)
    pub(super) fn field_by_name(&self, cls: &str, name: &str) -> Option<(String, String)> {
        let c = self.h.class(cls)?;
        if let Some(f) = c.fields.iter().find(|f| f.name == name) {
            return Some((c.name.clone(), f.desc.clone()));
        }
        c.super_name
            .iter()
            .chain(c.interfaces.iter())
            .find_map(|s| self.field_by_name(s, name))
    }

    /// 手写体接收者静态类型 → 类名（引用类型；推不出为 None）
    pub(super) fn stype_class(&self, host: &str, s: &SType) -> Option<String> {
        let of_desc = |d: &str| match parse_field(d)? {
            FieldType::Object(c) => Some(c),
            _ => None,
        };
        match s {
            SType::Named(t) => self.resolve_tref(host, t),
            SType::Field(b, f) => {
                let c = self.stype_class(host, b)?;
                of_desc(&self.field_by_name(&c, f)?.1)
            }
            SType::Ret(t, m) => self.rust_method_ret(&self.resolve_tref(host, t)?, m),
            SType::Call(b, m) => self.rust_method_ret(&self.stype_class(host, b)?, m),
        }
    }

    /// 手写体字段访问器：写入值接进字段节点并登记「有手写写入」；读出值汇入值池。
    /// 接收者推不出 → 所有同名字段按 open 处理（安全回退）
    pub(super) fn hw_fields(&mut self, m: usize, host: &str, fields: &[FieldAccess]) {
        let pool = Node::S(m, POOL);
        let prod = Node::S(m, PROD);
        for fa in fields {
            let site = fa.recv.as_ref().and_then(|r| self.stype_class(host, r)).and_then(|c| self.field_by_name(&c, &fa.field));
            let Some((decl, desc)) = site else {
                if fa.write {
                    self.open_field_name(&fa.field);
                }
                let fresh = if fa.write {
                    self.hw_written_names.insert(fa.field.clone())
                } else {
                    self.hw_read_names.entry(fa.field.clone()).or_default().insert(prod)
                };
                if !fresh {
                    continue;
                }
                let hit: Vec<(usize, String)> = self
                    .fields
                    .keys()
                    .enumerate()
                    .filter(|(_, k)| k.name == fa.field)
                    .map(|(i, k)| (i, k.desc.clone()))
                    .collect();
                for (i, d) in hit {
                    let Some(tid) = parse_field(&d).and_then(|t| self.ptype(&t)) else { continue };
                    if fa.write {
                        // 写入值取自值池、只以 open 出现在读者处：须逃逸
                        self.add_to(Node::U(i), &TypeSet::open(tid));
                        self.flow(pool, Node::Esc, tid);
                    } else {
                        self.flow(Node::F(i), prod, tid);
                    }
                }
                continue;
            };
            let key = MemberRef { owner: decl, name: fa.field.clone(), desc };
            let tid = parse_field(&key.desc).and_then(|t| self.ptype(&t));
            if fa.write {
                self.hw_written.insert(key.clone());
                self.open_field(key.clone());
            }
            let Some(tid) = tid else { continue };
            let fi = self.field_node(key);
            if fa.on_self && !self.methods[m].is_static {
                let fs: Rc<[Feed]> = match (fa.write, self.hw_type(host, &fa.value)) {
                    (false, _) => Rc::from([]),
                    (true, Some(id)) => Rc::from([Feed::S(TypeSet::exact(id))]),
                    (true, None) => Rc::from([Feed::N(pool)]),
                };
                let recv = Node::P(m, 0);
                self.self_fields.entry(recv).or_default().push((fi, tid, fa.write, fs, prod));
                let cur = self.sets.get(&recv).cloned().unwrap_or_default();
                self.self_field_objs(recv, &cur);
                continue;
            }
            if fa.write {
                let fs = match self.hw_type(host, &fa.value) {
                    Some(id) => vec![Feed::S(TypeSet::exact(id))],
                    None => vec![Feed::N(pool)],
                };
                self.feed(&fs, Node::U(fi), tid);
            } else {
                self.flow(Node::F(fi), prod, tid);
            }
        }
    }

    /// 手写体访问接收者自身字段：抽象对象接其字段节点，非抽象接收者（类本身 / open）经未知接收者视图
    pub(super) fn self_field_objs(&mut self, recv: Node, delta: &TypeSet) {
        let Some(acc) = self.self_fields.get(&recv).cloned() else { return };
        let raw = !delta.open.is_empty() || delta.classes.iter().any(|x| !self.objs.contains_key(x));
        let objs: Vec<u32> = delta.classes.iter().copied().filter(|x| self.objs.contains_key(x)).collect();
        for (fi, tid, write, fs, prod) in acc.iter() {
            let (fi, tid) = (*fi, *tid);
            for &o in &objs {
                let n = self.obj_field(o, fi, tid);
                if *write { self.feed(fs, n, tid) } else { self.flow(n, *prod, tid) }
            }
            if raw {
                if *write { self.feed(fs, Node::U(fi), tid) } else { self.flow(Node::F(fi), *prod, tid) }
            }
        }
    }

    /// 手写体效果：分配 / 构造 / 回调。实参按手写体调用点的语法推断精确接入，推断不出的经方法 m 的值池流转
    /// 手写体第 k 个分配点的值：容器类取抽象对象（伪偏移自 u32::MAX 递减，不与字节码偏移相撞），其余取类本身。
    /// 手写分配若只取类本身，其字段写入落到 U(f) 并流向该类全部对象，污染所有同类容器
    pub(super) fn hw_obj(&mut self, m: usize, k: &mut u32, cls: &str) -> u32 {
        *k += 1;
        if self.container(cls) { self.obj_at(m, u32::MAX - *k, cls) } else { self.id(cls) }
    }

    pub(super) fn apply_hw(&mut self, m: usize, host: &str, mh: &MemberHw, via: &Via) {
        let prod = Node::S(m, PROD);
        self.hw_fields(m, host, &mh.fields);
        let mut k = 0u32;
        // 手写体新建的对象（按类）：新建局部变量上的回调以它们为接收者
        let mut made: HashMap<String, Vec<u32>> = HashMap::default();
        for t in &mh.allocs {
            if let Some(c) = self.resolve_tref(host, t) {
                self.instantiate(&c, via.clone());
                self.init(&c, via.clone());
                let id = self.hw_obj(m, &mut k, &c);
                self.add_to(prod, &TypeSet::exact(id));
                made.entry(c).or_default().push(id);
            }
        }
        for (t, ctor) in &mh.ctors {
            let Some(c) = self.resolve_tref(host, t) else { continue };
            let Some(cf) = self.h.class(&c) else { continue };
            // 无重载构造器的 Rust 名是裸 `new`，重载的取 mangle 名；都不中（缩写后缀等）→ 全部构造器（安全过近似）
            let all: Vec<&classfile::Method> = cf.methods.iter().filter(|x| x.is_init()).collect();
            let exact: Vec<String> = all
                .iter()
                .filter(|x| {
                    let (plain, mangled) = self.rust_names(&cf, "<init>", &x.desc);
                    plain.as_deref() == Some(ctor.as_str()) || mangled == *ctor
                })
                .map(|x| x.desc.clone())
                .collect();
            let inits = if exact.is_empty() { all.iter().map(|x| x.desc.clone()).collect() } else { exact };
            if inits.is_empty() {
                continue;
            }
            self.instantiate(&c, via.clone());
            self.init(&c, via.clone());
            let id = self.hw_obj(m, &mut k, &c);
            self.add_to(prod, &TypeSet::exact(id));
            made.entry(c.clone()).or_default().push(id);
            for d in inits {
                let n = parse_method(&d).map_or(0, |md| md.params.len());
                let sites = Self::hw_sites(mh, |x| x.name == *ctor && x.path_ty.as_ref() == Some(t), "<init>", n);
                let a = self.hw_args(m, host, &d, &sites);
                let t = self.method(MemberRef { owner: c.clone(), name: "<init>".into(), desc: d }, via.clone());
                self.edge(m, 0, t, Recv::Exact(id), &a, None, None);
            }
        }
        for u in &self.hw_upcalls(host, mh) {
            match u {
                Upcall::Field(f) => {
                    let f = f.clone();
                    self.field(m, 0, classfile::op::GETSTATIC, &f, None, None, prod);
                }
                Upcall::Method(u) => {
                    let Some(site) = self.h.resolve_method(&u.owner, &u.name, &u.desc, self.h.is_interface(&u.owner)) else {
                        self.unresolved.insert(u.to_string());
                        continue;
                    };
                    let n = parse_method(&u.desc).map_or(0, |md| md.params.len());
                    let sites = Self::hw_sites(mh, |x| member_matches(&x.name, &u.name), &u.name, n);
                    let a = self.hw_args(m, host, &u.desc, &sites);
                    let ret = parse_method(&u.desc).and_then(|d| d.ret).and_then(|r| self.ptype(&r));
                    let (o, nm, d) = site.key();
                    let resolved = MemberRef { owner: o, name: nm, desc: d };
                    let rm = site.method();
                    let owner = self.id(&u.owner);
                    if u.name == "<init>" {
                        self.instantiate(&u.owner, via.clone());
                        self.init(&u.owner, via.clone());
                        let obj = self.hw_obj(m, &mut k, &u.owner);
                        self.add_to(prod, &TypeSet::exact(obj));
                        let t = self.method(resolved, via.clone());
                        self.edge(m, 0, t, Recv::Exact(obj), &a, None, None);
                        continue;
                    }
                    // 接收者：各方法调用点都推断出具体类型 → 精确；否则来自手写层的 open(引用类)
                    let mut recv = TypeSet::default();
                    let mut typed = sites.as_ref().is_some_and(|v| v.iter().all(|c| c.recv.is_some()));
                    for c in sites.iter().flatten() {
                        // 新建局部变量上的回调：接收者就是本方法手写体新建的该类对象
                        if let Some(ids) = c.fresh.as_ref().and_then(|t| self.resolve_tref(host, t)).and_then(|t| made.get(&t)) {
                            recv.classes.extend(ids.iter().copied());
                            continue;
                        }
                        match c.recv.as_ref().and_then(|r| self.hw_type(host, r)) {
                            Some(id) => {
                                recv.classes.insert(id);
                            }
                            None => typed = false,
                        }
                    }
                    if !typed {
                        recv = TypeSet::open(owner);
                    }
                    if rm.is_static() || rm.is_private() {
                        self.init(&resolved.owner, via.clone());
                        let t = self.method(resolved, via.clone());
                        self.edge(m, 0, t, Recv::Feeds(vec![Feed::S(recv)]), &a, ret, Some(prod));
                    } else {
                        // JNI Call<T>Method：按接收者实际类型分派
                        let rs = self.receivers(m, &recv, owner);
                        for r in rs {
                            self.dispatch_one(m, 0, r, &site, &a, ret, Some(prod), NOCTX);
                        }
                    }
                }
            }
        }
    }
}
