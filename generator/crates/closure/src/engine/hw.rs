//! 引擎：手写节点——手写方法的返回、导出、回调与分配效果建模。

use super::*;

/// 签名多态调用点的伴生 fn 后缀：清单登记需要调用点类型的签名多态成员 `m`，发射层改发
/// `recv.m__site("<调用点描述符>", 实参数组)`（`instr::sim::methods`），其体即该调用点的落地语义
const SIGPOLY_SITE_SUFFIX: &str = "__site";

impl<'a> Engine<'a> {
    // ── 手写节点 ────────────────────────────────────────────────────────────

    pub(super) fn process_handwritten(&mut self, m: usize) {
        let key = self.methods[m].key.clone();
        let via = Via::method("handwritten", m, None);
        // 手写返回值：open(返回类型)
        let reads = self.man.memory_read(&key.to_string()).is_some();
        let array_ret = self.man.array_return(&key.to_string()).map(<[String]>::to_vec);
        if let Some(rt) = self.methods[m].rtype {
            if let Some(es) = &array_ret {
                self.array_return(m, rt, es);
            } else if self.man.returns_primitive_class(&key.to_string()) {
                let k = self.primitive_mirror();
                self.add_to(Node::R(m), &TypeSet::exact(k));
            } else if let Some(cls) = self.man.defined_class(&key.to_string()).map(str::to_string) {
                let k = self.mirror(&cls);
                self.add_to(Node::R(m), &TypeSet::exact(k));
            } else if !self.man.returns_receiver(&key.to_string()) && !reads {
                self.add_to(Node::R(m), &TypeSet::open(rt));
            }
        }
        if key.name == "<clinit>" {
            return;
        }
        let ks = key.to_string();
        if let Some(k) = self.man.member_enumerator(&ks) {
            let n = Node::P(m, 0);
            if self.enum_recv.insert(n, (RHook::Enum(k), m)).is_none() {
                let s = self.set_of(n);
                self.rpending.push((RHook::Enum(k), m, s));
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
        if mh.fns.is_empty() {
            self.hw_base_fn(m, &cf, &key.name, &key.desc);
        }
        // 返回值已精确建模（内存读取 / 接收者浅拷贝 / 类镜像 / 超类 / 元素类型镜像）时不经 open 返回值交出
        let modeled = reads
            || array_ret.is_some()
            || self.man.returns_receiver(&ks)
            || self.man.returns_mirror(&ks)
            || self.man.returns_superclass(&ks)
            || self.man.returns_component_class(&ks)
            || self.man.returns_declaring_class(&ks)
            || self.man.returns_primitive_class(&ks)
            || self.man.defined_class(&ks).is_some();
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
                // 接收者全为自身接收者的回调按 P(m,0) 分派，接收者已在建模代码手里，无须逃逸
                if u.name != "<init>" && !Self::self_only(mh, &u.name, &u.desc, is_static) {
                    out.insert(self.id(&u.owner));
                }
            }
        }
        out
    }

    /// 回调的各方法调用点接收者都是本方法自身的接收者（`self` / `self.0`）：实际接收者就是 P(m,0) 中的对象
    pub(super) fn self_only(mh: &MemberHw, name: &str, desc: &str, is_static: bool) -> bool {
        if is_static {
            return false;
        }
        let n = parse_method(desc).map_or(0, |md| md.params.len());
        Self::hw_sites(mh, |x| member_matches(&x.name, name), name, n).is_some_and(|v| v.iter().all(|c| c.on_self && c.fresh.is_none() && c.recv.is_some()))
    }

    /// 精确匹配（mangle 名 / 无重载裸名）优先，否则按名字前缀（安全过近似）
    pub(super) fn hw_member(&self, cf: &ClassFile, name: &str, desc: &str) -> crate::handwritten::MemberHw {
        let all = self.hw.member(&cf.name, name);
        let (rust, mangled) = self.rust_names(cf, name, desc);
        // 签名多态成员（JVMS §2.9.3，结构判定）的调用点经 `__site` 伴生落地，伴生体与成员体同属本成员
        let sigpoly = cf.methods.iter().any(|m| m.name == name && m.desc == desc && resolve::is_signature_polymorphic(m));
        let exact: Vec<String> = all
            .fns
            .iter()
            .filter(|f| {
                let f = f.strip_prefix("__impl_").unwrap_or(f);
                let f = if sigpoly { f.strip_suffix(SIGPOLY_SITE_SUFFIX).unwrap_or(f) } else { f };
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
            out.absorb(f, &hwc.fns[f]);
        }
        out
    }

    /// 基本类型（含 void）的类镜像：一个 Class 类型的抽象对象，九个基本类型类合一。不登记为类镜像——所指不是
    /// 字节码类：类初始化无对象（跳过），超类为 null（[`Self::super_set`]），无 Java 字段（[`Self::class_values`]）
    pub(super) fn primitive_mirror(&mut self) -> u32 {
        if let Some(k) = self.prim_mirror {
            return k;
        }
        let class = self.id(CLASS);
        let k = self.id(&format!("{CLASS}#<primitive>"));
        self.objs.insert(k, class);
        self.prim_mirror = Some(k);
        k
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
        self.sysprops_hw(host, mh);
        self.hwobj_made(m, host, mh);
        self.hw_fields(m, host, &mh.fields);
        self.hw_fn_calls(m, host, mh);
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
                Upcall::Init(c) => self.init(c, via.clone()),
                Upcall::Field(f) => {
                    let f = f.clone();
                    if !self.hw_static_reads.insert((m, f.clone())) {
                        continue;
                    }
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
                    if Self::self_only(mh, &u.name, &u.desc, self.methods[m].is_static) {
                        // 自身接收者：读 P(m,0)（登记读者，接收者增长时重跑本方法）
                        let prev = (self.cur_site.replace((m, 0)), self.cur_call.take());
                        recv = self.value_set(&[Feed::N(Node::P(m, 0))]);
                        (self.cur_site, self.cur_call) = prev;
                    } else if !typed {
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
