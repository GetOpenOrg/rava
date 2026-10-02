//! 引擎：手写方法的内存效果——调用点级数组 / 字段写入与读取建模、签名多态写入与静态字段按名打开。

use super::*;

/// 按偏移读写的目标字段口径
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Gate {
    /// 普通 Unsafe / VarHandle 调用点：偏移可经字节码外途径取得的字段（[`Engine::offset_exposed`]）
    Offset,
    /// 方法句柄解释器调用点（`[facts.handle_interpreters]`）：只作用于 DMH 所指字段——偏移可经字段句柄 /
    /// MemberName 取得的字段，不含数组元素与只经反序列化放开的字段
    Handle,
}

impl<'a> Engine<'a> {
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
        self.note_self_copies(s, base, &ws);
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

    /// 元素来源与写入目标是同一个入口形参值的写入（`System.arraycopy(es, i + 1, es, i, n)` 等
    /// 数组内搬移）：运行期两者是同一数组，写入不改变其元素集。只认未经合流的入口形参值——
    /// 形参在方法执行期间恒指同一对象；调用点 / 字段读取来源在循环里可能是不同对象，不认
    fn note_self_copies(&mut self, s: u32, base: usize, ws: &[Option<HwWrite>]) {
        let Some(vals) = self.call_vals.clone() else { return };
        let entry = |k: usize| match k.checked_sub(base).and_then(|k| vals.get(k)) {
            Some(V::Ref { src, .. }) if src.len() == 1 && matches!(src[0], Src::Param(_)) => Some(src[0]),
            _ => None,
        };
        for (j, w) in ws.iter().enumerate() {
            let Some(w) = w else { continue };
            for &i in &w.elements {
                if i != j && entry(i).is_some() && entry(i) == entry(j) {
                    self.hw_self_copies.insert((s, i as u16, j as u16));
                }
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
        let gate = self.site_gate(s);
        if !delta.open.is_empty() {
            self.add_to(res, &TypeSet::open(rt));
        }
        for x in delta.classes.iter() {
            if self.arrays.contains_key(&x) {
                // DMH 字段访问器的基址是对象 / 类镜像，不读数组元素
                if gate == Gate::Handle {
                    continue;
                }
                for p in PARITIES {
                    self.flow(Node::E(x, p), res, rt);
                }
                continue;
            }
            // 类镜像是静态字段基址（staticFieldBase）：读所指类的静态引用字段；所指未知按 open
            // 类镜像：同时是静态字段基址（staticFieldBase），读所指类的静态引用字段（所指未知按 open）与 Class 的实例字段
            let class = self.id(CLASS);
            if let Some(&c) = self.mirrors.get(&x) {
                for (fi, _) in self.static_ref_fields(c) {
                    self.offset_read(Node::F(fi), res, fi, rt, gate);
                }
            } else if x == class {
                self.add_to(res, &TypeSet::open(rt));
            }
            let (cls, obj) = match (self.objs.get(&x), self.mirrors.contains_key(&x)) {
                (Some(&c), _) => (c, true),
                (None, true) => (class, false),
                (None, false) => (x, false),
            };
            for (fi, tid) in self.ref_fields(cls).iter().copied() {
                let n = if obj { self.obj_field(x, fi, tid) } else { Node::F(fi) };
                self.offset_read(n, res, fi, rt, gate);
            }
        }
    }

    /// 类 c 自身声明的静态引用字段节点（静态字段基址 = 声明类的类镜像）
    fn static_ref_fields(&mut self, c: u32) -> Vec<(usize, u32)> {
        let Some(cf) = self.h.class(&self.names[c as usize].clone()) else { return vec![] };
        let mut out = Vec::new();
        for f in cf.fields.iter().filter(|f| f.is_static()) {
            let Some(tid) = parse_field(&f.desc).and_then(|t| self.ptype(&t)) else { continue };
            let fi = self.field_node(MemberRef { owner: cf.name.clone(), name: f.name.clone(), desc: f.desc.clone() });
            out.push((fi, tid));
        }
        out
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
    /// 其余取得引用元素数组视图的手写体保守处理（基本元素视图不改写引用元素，见 `MemberHw::array_access`）：每个非接收者引用形参都可被写入，来源为其它形参的值、
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
                let access = self.hw_body(t).is_some_and(|(_, mh)| mh.array_access);
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
                if w.as_ref().is_some_and(|w| w.elements.contains(&(i as usize))) && !self.hw_self_copies.contains(&(s, i, j as u16)) {
                    for p in PARITIES {
                        self.flow(Node::E(y, p), Node::W(s, j as u16), obj);
                    }
                }
            }
            // 写入目标：DMH 字段访问器经解释器的字段写入只落在对象 / 类镜像所指字段上，不写数组元素
            match ws.get(i as usize) {
                Some(Some(w)) if !(w.fields && self.site_gate(s) == Gate::Handle) => {}
                _ => continue,
            }
            let t = self.arrays[&y];
            let Some(c) = absint::component(&self.names[t as usize].clone()).filter(|c| c.len() > 1) else { continue };
            let cid = self.id(&c);
            for p in PARITIES {
                self.flow(Node::W(s, i), Node::E(y, p), cid);
            }
        }
    }

    /// 手写调用点 s 的读写口径：调用方是清单声明的方法句柄解释器时按 DMH 所指字段
    fn site_gate(&self, s: u32) -> Gate {
        let (m, _, _) = self.hw_sites[s as usize];
        if self.man.is_handle_interpreter(&self.methods[m].key) { Gate::Handle } else { Gate::Offset }
    }

    fn is_poly(&self, t: usize) -> bool {
        let key = &self.methods[t].key;
        self.h.class(&key.owner).and_then(|cf| cf.method(&key.name, &key.desc).map(resolve::is_signature_polymorphic)).unwrap_or(false)
    }

    /// 签名多态写入调用点 / 所指未知的静态字段基址：写入值接到按名打开的静态引用字段
    fn poly_write(&mut self, wn: Node) {
        if self.poly_writes.contains(&wn) {
            return;
        }
        self.poly_writes.push(wn);
        for (fi, tid) in self.open_statics.clone() {
            self.flow(wn, Node::U(fi), tid);
        }
    }

    /// 以类 c 的镜像为静态字段基址的按偏移写入：静态字段偏移只能经按名取到的字段句柄 / MemberName 得到
    /// （`staticFieldOffset(Field)` 等），写入目标限于 c 上按名打开的静态引用字段；之后打开的由 `open_static` 接入
    fn mirror_write(&mut self, c: u32, wn: Node) {
        let ws = self.mirror_writes.entry(c).or_default();
        if ws.contains(&wn) {
            return;
        }
        ws.push(wn);
        for (fi, tid) in self.static_ref_fields(c) {
            if self.open_statics.iter().any(|&(f, _)| f == fi) {
                self.flow(wn, Node::U(fi), tid);
            }
        }
    }

    /// 字段按名打开：静态引用字段接收签名多态写入调用点、以及以所属类镜像为基址的按偏移写入的写入值
    pub(super) fn open_static(&mut self, key: &MemberRef) {
        let is_static = self.h.class(&key.owner).and_then(|cf| cf.field(&key.name, &key.desc).map(|f| f.is_static())).unwrap_or(false);
        let Some(tid) = parse_field(&key.desc).and_then(|t| self.ptype(&t)).filter(|_| is_static) else { return };
        let fi = self.field_node(key.clone());
        if self.open_statics.iter().any(|&(f, _)| f == fi) {
            return;
        }
        self.open_statics.push((fi, tid));
        let owner = self.id(&key.owner);
        let mws = self.mirror_writes.get(&owner).cloned().unwrap_or_default();
        for wn in self.poly_writes.iter().copied().chain(mws).collect::<Vec<_>>() {
            self.flow(wn, Node::U(fi), tid);
        }
    }

    /// 实例字段 fi 可经偏移写入：实例字段偏移只能经按名取到的字段句柄 / MemberName（含方法句柄的字段访问器成员）、
    /// 字段枚举或反序列化取得，这些来源同时决定字段不折叠（`field_open`）——两者是同一集合。
    /// 方法句柄解释器（[`Gate::Handle`]）只读写 DMH 所指字段：DMH 字段访问器只能由按名查找 / 字段枚举取得的
    /// 字段构造（[`Self::handle_field`]、全部放开）。字节码外写入（手写层写入、VM 状态、边界类）使字段不折叠，
    /// 但不产生字段句柄；只经反序列化放开的字段也不算（反序列化经 FieldReflector 自身的 Unsafe 调用点读写）。
    /// 所属类推不出的字段按可写
    fn offset_exposed(&self, fi: usize, gate: Gate) -> bool {
        let Some((key, _)) = self.fields.get_index(fi) else { return true };
        let Some(info) = self.ctx.field_info(key) else { return true };
        match gate {
            Gate::Offset => self.ctx.field_open_under(&info, self.ctx.fopen_all.get(), self.ctx.deser.get()),
            Gate::Handle => {
                self.ctx.fopen_all.get() || self.handle_fields.contains(key) || self.handle_names.contains(&key.name)
            }
        }
    }

    /// 按名查找 / 字段枚举取得字段 key 的字段句柄：字段不折叠，解释器口径的挂起读写随之接上
    pub(super) fn handle_field(&mut self, key: MemberRef) {
        let fresh = self.handle_fields.insert(key.clone());
        self.open_field(key.clone());
        if fresh {
            self.offset_fields_opened(Some((&key.name, Some(&key))));
        }
    }

    /// 按名查找的目标类推不出：任意类的同名字段都可取得字段句柄
    pub(super) fn handle_field_name(&mut self, name: &str) {
        let fresh = self.handle_names.insert(name.to_string());
        self.open_field_name(name);
        if fresh {
            self.offset_fields_opened(Some((name, None)));
        }
    }

    /// 字段 fi 的偏移可经字节码外的途径取得、从而可按偏移读取：可按偏移写入的字段（[`Self::offset_exposed`]），
    /// 加上已被字段枚举取到、但因字段句柄写入口未达而未放开的字段（枚举结果足以算偏移，读取不需要写入口）
    fn offset_readable(&self, fi: usize, gate: Gate) -> bool {
        if self.offset_exposed(fi, gate) || self.fenum_pending.contains(&None) {
            return true;
        }
        let Some((key, _)) = self.fields.get_index(fi) else { return true };
        self.fenum_pending.iter().flatten().any(|c| c == &key.owner || self.h.is_subtype(c, &key.owner))
    }

    /// 字段节点 n 经偏移读入结果节点 res：偏移尚不可得时挂起，可得时由 [`Self::offset_fields_opened`] 接上。
    /// 与写入对称：按偏移读取只能落在偏移已取得的字段上，不是「读任意对象的任意字段」
    fn offset_read(&mut self, n: Node, res: Node, fi: usize, tid: u32, gate: Gate) {
        if self.offset_readable(fi, gate) {
            self.flow(n, res, tid);
            return;
        }
        let ws = self.offset_read_waits.entry(fi).or_default();
        if !ws.contains(&(n, res, tid, gate)) {
            ws.push((n, res, tid, gate));
        }
    }

    /// 写入值 wn 经偏移写到对象字段节点 n：字段未开放时挂起，开放时由 [`Self::offset_fields_opened`] 接上
    fn offset_write(&mut self, wn: Node, n: Node, fi: usize, tid: u32, gate: Gate) {
        if self.offset_exposed(fi, gate) {
            self.flow(wn, n, tid);
            return;
        }
        let ws = self.offset_waits.entry(fi).or_default();
        if !ws.contains(&(wn, n, tid, gate)) {
            ws.push((wn, n, tid, gate));
        }
    }

    /// 字段开放后：接上目标字段已开放的挂起偏移写入。only = 本次开放所及的字段（单个字段键 / 同名），
    /// None = 全局开关打开，逐个复查
    pub(super) fn offset_fields_opened(&mut self, only: Option<(&str, Option<&MemberRef>)>) {
        self.offset_reads_ready();
        if self.offset_waits.is_empty() {
            return;
        }
        let ready: Vec<usize> = match only {
            Some((_, Some(key))) => self.fields.get_index_of(key).filter(|fi| self.offset_waits.contains_key(fi)).into_iter().collect(),
            Some((name, None)) => {
                self.offset_waits.keys().copied().filter(|&fi| self.fields.get_index(fi).is_some_and(|(k, _)| k.name == name)).collect()
            }
            None => self.offset_waits.keys().copied().collect(),
        };
        // 普通口径是解释器口径的超集：普通口径未开放的字段两种挂起都不接
        let ready: Vec<usize> = ready.into_iter().filter(|&fi| self.offset_exposed(fi, Gate::Offset)).collect();
        for fi in ready {
            let handle = self.offset_exposed(fi, Gate::Handle);
            let ws = self.offset_waits.remove(&fi).unwrap_or_default();
            let (go, stay): (Vec<_>, Vec<_>) = ws.into_iter().partition(|w| w.3 == Gate::Offset || handle);
            if !stay.is_empty() {
                self.offset_waits.insert(fi, stay);
            }
            for (wn, n, tid, _) in go {
                self.flow(wn, n, tid);
            }
        }
    }

    /// 字段开放 / 字段枚举挂起后：接上偏移已可得的挂起读取
    pub(super) fn offset_reads_ready(&mut self) {
        if self.offset_read_waits.is_empty() {
            return;
        }
        let ready: Vec<usize> = self.offset_read_waits.keys().copied().filter(|&fi| self.offset_readable(fi, Gate::Offset)).collect();
        for fi in ready {
            let handle = self.offset_readable(fi, Gate::Handle);
            let ws = self.offset_read_waits.remove(&fi).unwrap_or_default();
            let (go, stay): (Vec<_>, Vec<_>) = ws.into_iter().partition(|w| w.3 == Gate::Offset || handle);
            if !stay.is_empty() {
                self.offset_read_waits.insert(fi, stay);
            }
            for (n, res, tid, _) in go {
                self.flow(n, res, tid);
            }
        }
    }

    /// 调用点 s 的写入目标实参 i 新增对象：写入值接到对象的引用实例字段（与 `memory_read` 对称）。
    /// 抽象对象按对象分量接入；非抽象类 / open 目标接未知接收者写入节点，open 目标的子类字段不可枚举，
    /// 写入值随之逃逸。目标字段限于可经偏移写入的字段（[`Self::offset_exposed`]），不是「任意对象写任意字段」。类镜像是静态字段基址（staticFieldBase 返回声明类的类镜像）：写入所指类的静态引用字段；
    /// 所指未知的镜像、以及可能是类镜像的 open 目标（Class 的超类型）写入按名打开的静态引用字段——
    /// 静态字段偏移只能经按名取到的字段句柄 / MemberName 得到，按名打开已覆盖全部可写目标
    pub(super) fn hw_site_fields(&mut self, s: u32, i: u16, delta: &TypeSet) {
        let (_, _, t) = self.hw_sites[s as usize];
        if !self.hw_writes(t).get(i as usize).is_some_and(|w| w.as_ref().is_some_and(|w| w.fields)) {
            return;
        }
        let wn = Node::W(s, i);
        let gate = self.site_gate(s);
        let obj = self.id(OBJECT);
        let class = self.id(CLASS);
        let xs: Vec<u32> = delta.classes.iter().filter(|x| !self.arrays.contains_key(x)).collect();
        for x in xs {
            if let Some(&c) = self.mirrors.get(&x) {
                self.mirror_write(c, wn);
            } else if x == class {
                self.poly_write(wn);
            }
            let (cls, is_obj) = match (self.objs.get(&x), self.mirrors.contains_key(&x)) {
                (Some(&c), _) => (c, true),
                (None, true) => (class, false),
                (None, false) => (x, false),
            };
            for (fi, tid) in self.ref_fields(cls).iter().copied() {
                let n = if is_obj { self.obj_field(x, fi, tid) } else { Node::U(fi) };
                self.offset_write(wn, n, fi, tid, gate);
            }
        }
        let os: Vec<u32> = delta.open.iter().collect();
        for o in os {
            for (fi, tid) in self.ref_fields(o).iter().copied() {
                self.offset_write(wn, Node::U(fi), fi, tid, gate);
            }
            if self.sub(class, o) {
                self.poly_write(wn);
            }
            self.flow(wn, Node::Esc, obj);
        }
    }
}
