//! 引擎：类登记、实例化与类初始化、容器 / 工厂判定。

use super::*;

impl<'a> Engine<'a> {
    pub(super) fn id(&mut self, name: &str) -> u32 {
        if let Some(i) = self.ids.get(name) {
            return *i;
        }
        let rc: Rc<str> = Rc::from(name);
        let i = self.names.len() as u32;
        self.names.push(rc.clone());
        self.ids.insert(rc, i);
        i
    }

    pub fn name(&self, id: u32) -> &str {
        &self.names[id as usize]
    }

    /// 类型 x 是否 f 的子类型（合成 lambda 类按其 SAM 接口判定）
    pub(super) fn sub(&mut self, x: u32, f: u32) -> bool {
        if x == f {
            return true;
        }
        if let Some(r) = self.sub_cache.get(&(x, f)) {
            return *r;
        }
        let fname = self.names[f as usize].clone();
        let r = if let Some(&t) = self.arrays.get(&x).or_else(|| self.objs.get(&x)) {
            self.sub(t, f)
        } else if self.mirrors.contains_key(&x) {
            let c = self.id(CLASS);
            self.sub(c, f)
        } else if let Some(l) = self.lambdas.get(&x) {
            &*fname == OBJECT || self.h.is_subtype(&l.iface, &fname)
        } else if let Some(r) = self.hwobj_sub(x, &fname) {
            r
        } else {
            let xname = self.names[x as usize].clone();
            self.h.is_subtype(&xname, &fname)
        };
        self.sub_cache.insert((x, f), r);
        r
    }

    pub(super) fn is_iface(&self, id: u32) -> bool {
        !self.lambdas.contains_key(&id) && self.h.is_interface(&self.names[id as usize])
    }

    /// open(o) 按过滤类型 t 收窄的结果（None = 空），按 (o, t) 缓存
    fn open_narrow(&mut self, o: u32, t: u32) -> Option<u32> {
        if let Some(&r) = self.narrow_cache.get(&(o, t)) {
            return (r != u32::MAX).then_some(r);
        }
        let r = if self.sub(o, t) {
            Some(o)
        } else if self.sub(t, o) {
            Some(t)
        } else if !self.is_iface(o) && self.is_iface(t) {
            // 类 × 接口：交集是「o 的子类中实现 t 者」。保留 open(o)——展开时按接收者类型再求交；
            // final 类没有子类，不实现 t 即为空
            (!self.h.class(&self.names[o as usize]).is_some_and(|c| c.access & 0x0010 != 0)).then_some(o)
        } else if self.is_iface(o) {
            Some(t)
        } else {
            None
        };
        self.narrow_cache.insert((o, t), r.unwrap_or(u32::MAX));
        r
    }

    /// 类型集按过滤类型收窄
    pub(super) fn filter(&mut self, s: &TypeSet, t: u32) -> TypeSet {
        if self.names[t as usize].as_ref() == OBJECT {
            return s.clone();
        }
        let mut out = TypeSet::default();
        let ti = t as usize;
        if self.sub_rows.len() <= ti {
            self.sub_rows.resize_with(ti + 1, Vec::new);
        }
        let mut row = std::mem::take(&mut self.sub_rows[ti]);
        let mut kept: Vec<u32> = Vec::new();
        for (k, x) in s.classes.iter().enumerate() {
            let i = x as usize;
            let v = match row.get(i) {
                Some(&v) if v != 0 => v,
                _ => {
                    let v = if self.sub(x, t) { 2 } else { 1 };
                    if row.len() <= i {
                        row.resize(i + 1, 0);
                    }
                    row[i] = v;
                    v
                }
            };
            if v == 2 {
                // 输入有序，输出按序追加（首个命中时按剩余输入一次预留）
                if kept.capacity() == 0 {
                    kept.reserve(s.classes.len() - k);
                }
                kept.push(x);
            }
        }
        out.classes = IdSet::from_sorted(kept);
        self.sub_rows[ti] = row;
        for o in &s.open {
            if let Some(r) = self.open_narrow(o, t) {
                out.open.insert(r);
            }
        }
        out
    }

    /// G 中 ⊂ t 的成员
    pub(super) fn g_of(&mut self, t: u32) -> Rc<[u32]> {
        if !self.g_sub.contains_key(&t) {
            let g: Vec<u32> = self.g.iter().copied().collect();
            let v: Vec<u32> = g.into_iter().filter(|&x| self.sub(x, t)).collect();
            self.g_sub.insert(t, v);
        }
        self.g_sub[&t].as_slice().into()
    }

    /// 类型集里 ⊂ owner 的具体接收者（open 按 G 展开；展开过的方法 m 在 G 增长时重处理）
    /// 接收者集合（升序）：精确部分 ⊂ owner 者，open 部分按 G 展开
    pub(super) fn receivers(&mut self, m: usize, s: &TypeSet, owner: u32) -> Vec<u32> {
        let mut exact = Vec::new();
        for x in &s.classes {
            if self.sub(x, owner) {
                exact.push(x);
            }
        }
        if s.open.is_empty() {
            return exact;
        }
        let mut out = IdSet::from_sorted(exact);
        {
            for o in s.open.iter() {
                match (self.cur_call, self.cur_site) {
                    (Some(c), _) => {
                        self.open_calls.entry((o, owner)).or_default().insert(c);
                    }
                    (None, Some(w)) => {
                        self.open_sites.entry((o, owner)).or_default().insert(w);
                    }
                    (None, None) => {
                        self.open_methods.entry((o, owner)).or_default().insert(m);
                    }
                }
                for &x in self.g_of(o).iter() {
                    if out.contains(&x) || !self.sub(x, owner) {
                        continue;
                    }
                    // open 值只能是逃逸对象：数组分配点须已逃逸（非逃逸数组只经精确流转可达）
                    if self.arrays.contains_key(&x) && !self.escaped.contains(&x) {
                        continue;
                    }
                    out.insert(x);
                }
            }
        }
        out.iter().collect()
    }

    // ── 类登记 ──────────────────────────────────────────────────────────────

    pub(super) fn domain(&self, cls: &str) -> Domain {
        self.ctx.domain(cls)
    }

    /// 登记类（及其超类型，作为类型层级）；返回类文件
    pub(super) fn touch(&mut self, cls: &str, level: Level, via: Via) -> Option<std::sync::Arc<ClassFile>> {
        let cls = cls.trim_start_matches('[');
        let cls = cls.strip_prefix('L').and_then(|c| c.strip_suffix(';')).unwrap_or(cls);
        if cls.len() == 1 && "BCDFIJSZV".contains(cls) {
            return None;
        }
        let Some(cf) = self.h.class(cls) else {
            self.missing.entry(cls.to_string()).or_insert(via);
            return None;
        };
        if cut::edges_on() {
            let from = self.via_node(&via);
            cut::edge(&from, &format!("C:{cls}"));
        }
        let domain = self.domain(cls);
        let fresh = !self.classes.contains_key(cls);
        let node = self.classes.entry(cls.to_string()).or_insert_with(|| ClassNode {
            domain,
            level,
            via: via.clone(),
            level_via: BTreeMap::new(),
        });
        if level > node.level {
            node.level = level;
        }
        node.level_via.entry(level).or_insert(via);
        if fresh {
            for up in cf.super_name.iter().chain(cf.interfaces.iter()).cloned().collect::<Vec<_>>() {
                self.touch(&up, Level::Type, Via::class("supertype", cls));
            }
            self.touch_hw_types(cls);
        }
        Some(cf)
    }

    /// 类入生成范围即编译其手写文件：文件里出现的每个类型路径与共置手写模块要求对应类存在（L1）
    fn touch_hw_types(&mut self, cls: &str) {
        let hw = self.hw.class(cls);
        for t in &hw.type_refs {
            let found = match t.0.last().and_then(|l| MODULE_SUFFIXES.iter().find_map(|x| l.strip_suffix(x))) {
                Some(snake) => self.class_of_module(cls, &t.0, snake).into_iter().collect::<Vec<_>>(),
                None => self.resolve_hw_type(cls, t),
            };
            for c in found {
                self.touch(&c, Level::Type, Via::class("hw-type", cls));
            }
        }
    }

    /// 类型路径 → 存在的类；按路径找不到时去掉类型前的模块段重试（`module_t::Module` 这类改名的类文件模块）
    fn resolve_hw_type(&self, host: &str, t: &TypeRef) -> Vec<String> {
        let hit = |t: &TypeRef| self.hw.resolve_type(host, t).into_iter().find(|c| self.cp.contains(c));
        if let Some(c) = hit(t) {
            return vec![c];
        }
        let n = t.0.len();
        if n >= 2 && !matches!(t.0[n - 2].as_str(), "super" | "crate" | "self") {
            let mut s = t.0.clone();
            s.remove(n - 2);
            return hit(&TypeRef(s)).into_iter().collect();
        }
        vec![]
    }

    /// 共置手写模块路径（`super::x_impl` / `crate::a::b::x_impl`）→ 其宿主类
    pub(super) fn class_of_module(&mut self, host: &str, segs: &[String], snake: &str) -> Option<String> {
        let host_pkg = host.rsplit_once('/').map_or("", |(p, _)| p);
        let dirs: Vec<&str> = segs[..segs.len() - 1].iter().map(String::as_str).filter(|s| *s != "self").collect();
        let supers = dirs.iter().take_while(|s| **s == "super").count();
        let pkg = match dirs.first() {
            Some(&"super") => {
                let up: Vec<&str> = host_pkg.split('/').collect();
                let mut p = up[..up.len().saturating_sub(supers - 1)].to_vec();
                p.extend(&dirs[supers..]);
                p.join("/")
            }
            None => host_pkg.to_string(),
            Some(&"crate") => dirs[1..].join("/"),
            _ => dirs.join("/"),
        };
        if self.snake_index.is_none() {
            let mut idx = HashMap::default();
            for o in [Origin::Jdk, Origin::Lib, Origin::Image] {
                for n in self.cp.names_of(o) {
                    let (p, simple) = n.rsplit_once('/').unwrap_or(("", n.as_str()));
                    idx.entry(format!("{p}/{}", to_snake(simple))).or_insert_with(|| n.clone());
                }
            }
            self.snake_index = Some(idx);
        }
        self.snake_index.as_ref().unwrap().get(&format!("{pkg}/{snake}")).cloned()
    }

    /// 截断体：边界类里有字节码、未经手写提供的方法，发射层翻译其方法体但不展开被调方（规模截断的
    /// 过渡语义）。体内引用的类按类型级入闭包，使翻译体里的类型与字段访问器可解析；被调方不入链
    pub(super) fn touch_truncated_body(&mut self, m: usize, cf: &ClassFile, key: &MemberRef, via: &Via) {
        if self.methods[m].kind != Kind::Handwritten("boundary")
            || self.man.is_intrinsic(&key.to_string())
            || self.ctx.provided(cf, &key.name, &key.desc)
        {
            return;
        }
        let Some(code) = cf.method(&key.name, &key.desc).filter(|x| !x.is_native()).and_then(|x| x.code.as_ref()) else { return };
        let mut descs: Vec<String> = Vec::new();
        let mut classes: Vec<String> = code.exception_table.iter().filter_map(|e| e.catch_type.clone()).collect();
        for i in &code.insns {
            match &i.operand {
                classfile::Operand::Field(r) | classfile::Operand::Method(r, _) => {
                    classes.push(r.owner.clone());
                    descs.push(r.desc.clone());
                }
                classfile::Operand::Class(c) | classfile::Operand::Ldc(classfile::Const::Class(c)) | classfile::Operand::MultiANewArray(c, _) => {
                    if c.starts_with('[') {
                        descs.push(c.clone());
                    } else {
                        classes.push(c.clone());
                    }
                }
                _ => {}
            }
        }
        for c in classes {
            if !c.starts_with('[') {
                self.touch(&c, Level::Type, via.clone());
            }
        }
        for d in descs {
            self.touch_desc(&d, via);
        }
    }

    pub(super) fn touch_desc(&mut self, desc: &str, via: &Via) {
        for c in class_refs(desc) {
            self.touch(&c, Level::Type, via.clone());
        }
    }

    // ── 实例化 / 初始化 ────────────────────────────────────────────────────

    /// 数组分配点：独立的抽象对象（元素节点 `E(id)`），类型为数组类型
    pub(super) fn array_site(&mut self, m: usize, off: u32, t: &str, empty: bool, via: Via) -> u32 {
        let name = format!("{t}@{m}:{off}");
        if let Some(&id) = self.ids.get(name.as_str()) {
            if !empty {
                self.array_sized(id);
            }
            return id;
        }
        self.touch(t, Level::Type, via);
        let tid = self.id(t);
        let id = self.id(&name);
        self.arrays.insert(id, tid);
        if empty {
            self.empty_arrays.insert(id, HashMap::default());
        }
        // 多维数组的内层数组来自同一条指令：各维是同一分配点下按分量类型区分的数组分配点
        if let Some(c) = absint::component(t).filter(|c| c.starts_with('[')) {
            let inner = self.array_site(m, off, &c, false, Via::class("multianewarray", t));
            for p in PARITIES {
                self.add_to(Node::E(id, p), &TypeSet::exact(inner));
            }
        }
        if self.g.insert(id) {
            self.on_g_grow(id);
        }
        id
    }

    /// 清单声明元素类型的手写返回数组（`[facts.array_returns]`）：返回值取该方法的一个数组分配点，
    /// 元素为所列类型的 open（VM 写入的对象来自非建模代码），替代 open(返回类型) 的「元素为任意分量子类型」
    pub(super) fn array_return(&mut self, m: usize, rt: u32, elems: &[String]) {
        let t = self.names[rt as usize].to_string();
        let via = Via::method("array-return", m, None);
        let id = self.array_site(m, ARRAY_RET, &t, false, via.clone());
        for e in elems {
            let v = self.name_at(e, &via);
            for p in PARITIES {
                self.add_to(Node::E(id, p), &TypeSet::open(v));
            }
        }
        self.add_to(Node::R(m), &TypeSet::exact(id));
    }

    /// 清单中的类型名（binary name 或数组描述符）→ 类型 id，并登记为类型层级
    fn name_at(&mut self, t: &str, via: &Via) -> u32 {
        if !t.starts_with('[') {
            self.touch(t, Level::Type, via.clone());
        }
        self.id(t)
    }

    /// 数组分配点出现非 0 长度：不再按空数组处理，补回暂存的元素值
    pub(super) fn array_sized(&mut self, id: u32) {
        if let Some(held) = self.empty_arrays.remove(&id) {
            let mut v: Vec<(Node, TypeSet)> = held.into_iter().collect();
            v.sort_by_key(|(n, _)| format!("{n:?}"));
            for (n, s) in v {
                self.add_to(n, &s);
            }
        }
    }

    pub(super) fn instantiate(&mut self, cls: &str, via: Via) {
        if cut::edges_on() {
            let from = self.via_node(&via);
            cut::edge_plain(&from, &format!("A:{cls}"));
        }
        let id = self.id(cls);
        if !cls.starts_with('[') && self.touch(cls, Level::Alloc, via).is_none() {
            return;
        }
        if self.g.insert(id) {
            self.on_g_grow(id);
        }
    }

    /// 容器形态类（按字节码判定，不列类名）：翻译域的类（含超类）持有实例字段，其泛型签名引用类型变量、
    /// 或（泛型类链中）擦除为 `Object[]`，或声明类型本身是容器形态类（内部类的 `this$0`、`HashSet.map` 等）
    pub(super) fn container(&mut self, cls: &str) -> bool {
        let id = self.id(cls);
        if let Some(&c) = self.containers.get(&id) {
            return c;
        }
        // 递归保护：成环部分取最小不动点
        self.containers.insert(id, false);
        let r = matches!(self.domain(cls), Domain::Translate | Domain::User) && self.container_shape(cls);
        self.containers.insert(id, r);
        r
    }

    pub(super) fn container_shape(&mut self, cls: &str) -> bool {
        let obj_arr = format!("[L{OBJECT};");
        let mut chain = Vec::new();
        let mut cur = self.h.class(cls);
        while let Some(cf) = cur {
            cur = cf.super_name.as_deref().and_then(|s| self.h.class(s));
            chain.push(cf);
        }
        let inst = |cf: &std::sync::Arc<ClassFile>| cf.fields.iter().filter(|f| !f.is_static()).cloned().collect::<Vec<_>>();
        // 先看自身形态，再按字段声明类型递归（成环时结果与查询顺序无关）。
        // 字段签名直接引用类型变量即元素存储；擦除为 `Object[]` 的字段、容器形态的字段类型只在泛型类链
        // （类签名引用类型变量，含外部类的）里才算——非泛型类的 `Object[]` 是异构记录（表达式节点的实参表），
        // 非泛型类持有的容器是固定元素类型的缓存（`SoftReference<MethodHandle>`），按对象分开不带来元素类型精度
        let generic = chain.iter().any(|cf| cf.signature.as_deref().is_some_and(has_type_var));
        if chain
            .iter()
            .any(|cf| inst(cf).iter().any(|f| generic && f.desc == obj_arr || f.signature.as_deref().is_some_and(has_type_var)))
        {
            return true;
        }
        if !generic {
            return false;
        }
        // 泛型类链中持有函数式接口字段的对象（流水线的 sink 链、捕获 lambda / 局部值的匿名类）：
        // 字段值决定其方法把元素交给谁，按对象分开才能让各条流水线的元素互不汇合
        for cf in &chain {
            for f in inst(cf) {
                let Some(c) = f.desc.strip_prefix('L').and_then(|d| d.strip_suffix(';')) else { continue };
                if if self.h.is_interface(c) { self.functional(c) } else { self.container(c) } {
                    return true;
                }
            }
        }
        false
    }

    /// 函数式接口（JLS §9.8）：自身与超接口合计恰有一个抽象方法（不计 `Object` 的公开方法；被默认方法覆盖的不计）
    pub(super) fn functional(&self, iface: &str) -> bool {
        let Some(cf) = self.h.class(iface) else { return false };
        let mut all = self.h.all_superinterfaces(&cf);
        all.push(cf);
        let obj = self.h.class(OBJECT);
        let mut abs = HashSet::<(&str, &str)>::default();
        let mut dflt = HashSet::<(&str, &str)>::default();
        for i in &all {
            for m in i.methods.iter().filter(|m| !m.is_static() && !m.is_private()) {
                if obj.as_ref().is_some_and(|o| o.method(&m.name, &m.desc).is_some_and(|om| om.access & 0x0001 != 0)) {
                    continue;
                }
                if m.is_abstract() { abs.insert((&m.name, &m.desc)) } else { dflt.insert((&m.name, &m.desc)) };
            }
        }
        abs.iter().filter(|k| !dflt.contains(*k)).count() == 1
    }

    /// 方法 m 偏移 off 处分配的容器抽象对象：分配点 + 堆上下文（分配方法的接收者对象的分配点链，截断到 HEAP_DEPTH）
    pub(super) fn obj_at(&mut self, m: usize, off: u32, cls: &str) -> u32 {
        let b = self.mbase[&self.methods[m].key];
        let mut chain = format!("@{b}:{off}");
        let ctx = self.methods[m].ctx;
        // 递归结构（同类对象在自身方法里分配同类，如链表节点 / 表达式树）：堆上下文不再延长，
        // 否则分配点两两组合成 O(站点²) 个抽象对象而不带来任何分派精度
        let recursive = ctx != NOCTX && self.objs.get(&ctx).is_some_and(|&t| &*self.names[t as usize] == cls);
        if ctx != NOCTX && !recursive {
            for seg in self.obj_chain.get(&ctx).map_or("", |c| &**c).split('#').filter(|g| !g.is_empty()).take(HEAP_DEPTH - 1) {
                chain.push('#');
                chain.push_str(seg);
            }
        }
        let name = format!("{cls}{chain}");
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        let tid = self.id(cls);
        let id = self.id(&name);
        self.objs.insert(id, tid);
        self.obj_chain.insert(id, Rc::from(chain));
        id
    }

    /// 调用点上下文：以调用点命名的堆上下文（不是对象，不进入值集），克隆体内的容器分配以它为链首
    pub(super) fn site_ctx(&mut self, m: usize, off: u32) -> u32 {
        let chain = format!("@{}:{off}", self.mbase[&self.methods[m].key]);
        if let Some(&id) = self.ids.get(chain.as_str()) {
            return id;
        }
        let id = self.id(&chain);
        self.obj_chain.insert(id, Rc::from(chain));
        id
    }

    /// 新鲜工厂：有引用形参的静态字节码方法，返回值来自本方法分配的容器对象 / 引用数组，或来自另一个新鲜工厂
    pub(super) fn fresh_factory(&mut self, key: &MemberRef) -> bool {
        if let Some(&r) = self.factories.get(key) {
            return r;
        }
        // 递归保护：成环部分取最小不动点
        self.factories.insert(key.clone(), false);
        let r = self.fresh_factory_uncached(key);
        self.factories.insert(key.clone(), r);
        r
    }

    pub(super) fn fresh_factory_uncached(&mut self, key: &MemberRef) -> bool {
        let Some(cf) = self.h.class(&key.owner) else { return false };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return false };
        if !meth.is_static() || self.kind_of(&cf, meth) != Kind::Bytecode {
            return false;
        }
        if !parse_method(&key.desc).is_some_and(|md| md.params.iter().any(|p| p.is_reference())) {
            return false;
        }
        let Some(code) = meth.code.as_ref() else { return false };
        let live = |_: &str| true;
        let a = self.ctx.aux_analyze(&key.owner, &key.desc, true, code, &Facts { ctx: &self.ctx, live: &live, m: None, params: vec![] });
        if a.conservative {
            return false;
        }
        let sites: BTreeSet<u32> = a
            .events
            .iter()
            .filter_map(|(_, e)| match e {
                Event::Return(v @ V::Ref { .. }) => Some(v.srcs()),
                _ => None,
            })
            .flat_map(|ss| ss.iter().filter_map(|s| if let &Src::Site(o) = s { Some(o) } else { None }).collect::<Vec<_>>())
            .collect();
        for (off, e) in &a.events {
            if !sites.contains(off) {
                continue;
            }
            let fresh = match e {
                Event::New(c) => self.container(c),
                Event::NewArray(t, _) => t.starts_with("[L") || t.starts_with("[["),
                Event::Invoke { opcode: classfile::insn::op::INVOKESTATIC, mref, iface, .. } => {
                    match self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) {
                        Some(site) => {
                            let (o, n, d) = site.key();
                            self.fresh_factory(&MemberRef { owner: o, name: n, desc: d })
                        }
                        None => false,
                    }
                }
                _ => false,
            };
            if fresh {
                return true;
            }
        }
        false
    }
}
