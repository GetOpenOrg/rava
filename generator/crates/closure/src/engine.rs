//! 闭包引擎：XTA 类型集传播 + 虚分派 + 类初始化触发 + 手写节点 + 溯源（计划 §3.1 / §3.3 / §3.5）。
//!
//! 节点：方法（字节码 / 手写 / 抽象）、字段、全局数组元素。每个节点持有类型集
//! `TypeSet { classes, open }`：`classes` 是确定流入的已实例化类型；`open(T)` 表示
//! 「任意已实例化的 T 子类型」——手写返回值、手写可写字段、异常处理器入口等无法在字节码层
//! 追踪的来源一律按 open 处理（安全回退）。流边按形参 / 返回 / 字段声明类型过滤。
//!
//! 虚调用的接收者 = 调用方类型集里 ⊂ 引用类的类型；open 部分按全局已实例化集 G 展开。
//! G 增长时，持有 open 的方法与等待 catch 类型的方法重新处理——单调不动点。

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use classfile::descriptor::{class_refs, parse_field, parse_method, FieldType};
use classfile::{acc, ClassFile, Const, MemberRef, MethodHandle};
use indexmap::IndexMap;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::absint::{self, Analysis, Event, Oracle, V};
use crate::handwritten::{Handwritten, TypeRef, Upcall};
use crate::manifest::{Domain, Fact, IndyKind, Manifest};

const OBJECT: &str = "java/lang/Object";
const THROWABLE: &str = "java/lang/Throwable";
const TO_STRING: (&str, &str) = ("toString", "()Ljava/lang/String;");

// ── 溯源 ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum From {
    Root(String),
    Method(usize),
    Class(String),
}

#[derive(Debug, Clone)]
pub struct Via {
    pub kind: &'static str,
    pub from: From,
    pub off: Option<u32>,
}

impl Via {
    fn root(kind: &'static str, what: &str) -> Via {
        Via { kind, from: From::Root(what.to_string()), off: None }
    }
    fn method(kind: &'static str, m: usize, off: Option<u32>) -> Via {
        Via { kind, from: From::Method(m), off }
    }
    fn class(kind: &'static str, c: &str) -> Via {
        Via { kind, from: From::Class(c.to_string()), off: None }
    }
}

/// 类在闭包中的层级（取最高）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// 仅作类型出现（签名 / checkcast / catch / 超类型）
    Type,
    /// 类初始化被触发
    Init,
    /// 被实例化
    Alloc,
    /// 有可达的字节码方法体
    Code,
}

pub struct ClassNode {
    pub domain: Domain,
    pub level: Level,
    pub via: Via,
    /// 各层级首次到达的溯源
    pub level_via: BTreeMap<Level, Via>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bytecode,
    /// 手写承载（native / 边界类 / VM 内建 / 共置手写体提供）
    Handwritten(&'static str),
    Abstract,
    /// 类或方法不存在
    Missing,
}

// ── 类型集 ──────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Debug)]
pub struct TypeSet {
    pub classes: BTreeSet<u32>,
    pub open: BTreeSet<u32>,
}

impl TypeSet {
    fn add_all(&mut self, o: &TypeSet) -> bool {
        let n = self.classes.len() + self.open.len();
        self.classes.extend(o.classes.iter().copied());
        self.open.extend(o.open.iter().copied());
        n != self.classes.len() + self.open.len()
    }
    fn is_empty(&self) -> bool {
        self.classes.is_empty() && self.open.is_empty()
    }
}

pub struct MNode {
    pub key: MemberRef,
    pub kind: Kind,
    pub via: Via,
    pub set: TypeSet,
    analysis: Option<Rc<Analysis>>,
    /// 手写体命中的 fn 名（溯源）
    pub hw_fns: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Node {
    M(usize),
    F(usize),
    Array,
}

struct Lambda {
    iface: String,
    sam: String,
    imh: MethodHandle,
}

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

struct Ctx<'a> {
    h: &'a Hierarchy<'a>,
    man: &'a Manifest,
    /// static final 字段常量缓存（None = 非常量）
    consts: std::cell::RefCell<HashMap<MemberRef, Option<V>>>,
    in_progress: std::cell::RefCell<HashSet<String>>,
}

struct Facts<'c, 'a> {
    ctx: &'c Ctx<'a>,
    live: &'c dyn Fn(&str) -> bool,
}

fn const_value(c: &Const) -> Option<V> {
    match c {
        Const::Int(v) => Some(V::Int(*v)),
        Const::Long(v) => Some(V::Long(*v)),
        Const::String(s) => Some(V::Str(Rc::from(s.as_str()))),
        _ => None,
    }
}

impl Ctx<'_> {
    fn static_const(&self, f: &MemberRef) -> Option<V> {
        let site = self.h.resolve_field(&f.owner, &f.name, &f.desc)?;
        let fd = site.field();
        if fd.access & acc::STATIC == 0 || fd.access & acc::FINAL == 0 {
            return None;
        }
        if let Some(c) = &fd.constant_value {
            return const_value(c);
        }
        let key = MemberRef { owner: site.class.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
        if let Some(v) = self.consts.borrow().get(&key) {
            return v.clone();
        }
        // `<clinit>` 唯一一次常量赋值（递归保护：分析中的类不再展开）
        let cls = site.class.clone();
        if !self.in_progress.borrow_mut().insert(cls.name.clone()) {
            return None;
        }
        let v = cls.method("<clinit>", "()V").and_then(|m| m.code.as_ref()).and_then(|code| {
            let live = |_: &str| true;
            let a = absint::analyze(&cls.name, "()V", true, code, &Facts { ctx: self, live: &live });
            let puts: Vec<&Option<V>> = a
                .events
                .iter()
                .filter_map(|(_, e)| match e {
                    Event::Field { opcode: classfile::op::PUTSTATIC, mref, value }
                        if mref.name == key.name && mref.desc == key.desc && mref.owner == key.owner =>
                    {
                        Some(value)
                    }
                    _ => None,
                })
                .collect();
            match puts.as_slice() {
                [Some(v @ (V::Int(_) | V::Long(_) | V::Str(_) | V::Null))] => Some(v.clone()),
                _ => None,
            }
        });
        self.in_progress.borrow_mut().remove(&cls.name);
        self.consts.borrow_mut().insert(key, v.clone());
        v
    }
}

impl Oracle for Facts<'_, '_> {
    fn invoke_result(&self, m: &MemberRef, args: &[V]) -> Option<V> {
        let k = m.to_string();
        if let Some(f) = self.ctx.man.return_fact(&k) {
            return Some(match f {
                Fact::Null => V::Null,
                Fact::Int(i) => V::Int(*i),
            });
        }
        if self.ctx.man.is_null_to_false(&k) && args.iter().any(|a| *a == V::Null) {
            return Some(V::Int(0));
        }
        None
    }
    fn static_field(&self, f: &MemberRef) -> Option<V> {
        self.ctx.static_const(f)
    }
    fn catch_live(&self, ty: &str) -> bool {
        (self.live)(ty)
    }
}

// ── 引擎 ────────────────────────────────────────────────────────────────────

pub struct Engine<'a> {
    ctx: Ctx<'a>,
    h: &'a Hierarchy<'a>,
    cp: &'a ClassPath,
    man: &'a Manifest,
    hw: &'a Handwritten,

    names: Vec<Rc<str>>,
    ids: HashMap<Rc<str>, u32>,
    sub_cache: HashMap<(u32, u32), bool>,

    pub classes: IndexMap<String, ClassNode>,
    pub missing: BTreeMap<String, Via>,
    pub methods: IndexMap<MemberRef, MNode>,
    fields: IndexMap<MemberRef, TypeSet>,
    arrays: TypeSet,

    /// G：全局已实例化（类型 id）
    g: BTreeSet<u32>,
    lambdas: HashMap<u32, Lambda>,
    pub inited: IndexMap<String, Via>,

    flows: HashMap<Node, Vec<(Node, Rc<[u32]>)>>,
    flow_seen: HashSet<(Node, Node, Rc<[u32]>)>,
    /// 调用点分派结果：(方法, 偏移) → 目标方法
    pub dispatch: BTreeMap<(usize, u32), BTreeSet<usize>>,
    /// 死分支（方法 → 剪掉的目标偏移）
    pub dead: BTreeMap<usize, Vec<(u32, u32)>>,
    pub unresolved: BTreeSet<String>,

    mwork: VecDeque<usize>,
    in_mwork: HashSet<usize>,
    fwork: VecDeque<Node>,
    in_fwork: HashSet<Node>,
    open_methods: BTreeSet<usize>,
    pending_catch: BTreeMap<usize, Vec<String>>,
}

impl<'a> Engine<'a> {
    pub fn new(h: &'a Hierarchy<'a>, cp: &'a ClassPath, man: &'a Manifest, hw: &'a Handwritten) -> Self {
        Engine {
            ctx: Ctx {
                h,
                man,
                consts: Default::default(),
                in_progress: Default::default(),
            },
            h,
            cp,
            man,
            hw,
            names: Vec::new(),
            ids: HashMap::new(),
            sub_cache: HashMap::new(),
            classes: IndexMap::new(),
            missing: BTreeMap::new(),
            methods: IndexMap::new(),
            fields: IndexMap::new(),
            arrays: TypeSet::default(),
            g: BTreeSet::new(),
            lambdas: HashMap::new(),
            inited: IndexMap::new(),
            flows: HashMap::new(),
            flow_seen: HashSet::new(),
            dispatch: BTreeMap::new(),
            dead: BTreeMap::new(),
            unresolved: BTreeSet::new(),
            mwork: VecDeque::new(),
            in_mwork: HashSet::new(),
            fwork: VecDeque::new(),
            in_fwork: HashSet::new(),
            open_methods: BTreeSet::new(),
            pending_catch: BTreeMap::new(),
        }
    }

    fn id(&mut self, name: &str) -> u32 {
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
    fn sub(&mut self, x: u32, f: u32) -> bool {
        if x == f {
            return true;
        }
        if let Some(r) = self.sub_cache.get(&(x, f)) {
            return *r;
        }
        let fname = self.names[f as usize].clone();
        let r = if let Some(l) = self.lambdas.get(&x) {
            &*fname == OBJECT || self.h.is_subtype(&l.iface, &fname)
        } else {
            let xname = self.names[x as usize].clone();
            self.h.is_subtype(&xname, &fname)
        };
        self.sub_cache.insert((x, f), r);
        r
    }

    fn is_iface(&self, id: u32) -> bool {
        !self.lambdas.contains_key(&id) && self.h.is_interface(&self.names[id as usize])
    }

    /// 类型集按过滤类型（任一）收窄
    fn filter(&mut self, s: &TypeSet, f: &[u32]) -> TypeSet {
        let mut out = TypeSet::default();
        for &x in &s.classes {
            if f.iter().any(|&t| self.sub(x, t)) {
                out.classes.insert(x);
            }
        }
        for &o in &s.open {
            for &t in f {
                if self.sub(o, t) {
                    out.open.insert(o);
                } else if self.sub(t, o) || self.is_iface(o) || self.is_iface(t) {
                    out.open.insert(t);
                }
            }
        }
        out
    }

    /// 类型集里 ⊂ owner 的具体接收者（open 按 G 展开）
    fn receivers(&mut self, s: &TypeSet, owner: u32) -> BTreeSet<u32> {
        let mut out = BTreeSet::new();
        for &x in &s.classes {
            if self.sub(x, owner) {
                out.insert(x);
            }
        }
        if !s.open.is_empty() {
            let g: Vec<u32> = self.g.iter().copied().collect();
            for x in g {
                if out.contains(&x) || !self.sub(x, owner) {
                    continue;
                }
                let opens: Vec<u32> = s.open.iter().copied().collect();
                if opens.iter().any(|&o| self.sub(x, o)) {
                    out.insert(x);
                }
            }
        }
        out
    }

    // ── 类登记 ──────────────────────────────────────────────────────────────

    fn domain(&self, cls: &str) -> Domain {
        self.man.domain(cls, self.cp.origin(cls) == Some(Origin::User))
    }

    /// 登记类（及其超类型，作为类型层级）；返回类文件
    fn touch(&mut self, cls: &str, level: Level, via: Via) -> Option<Rc<ClassFile>> {
        let cls = cls.trim_start_matches('[');
        let cls = cls.strip_prefix('L').and_then(|c| c.strip_suffix(';')).unwrap_or(cls);
        if cls.len() == 1 && "BCDFIJSZV".contains(cls) {
            return None;
        }
        let Some(cf) = self.h.class(cls) else {
            self.missing.entry(cls.to_string()).or_insert(via);
            return None;
        };
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
        }
        Some(cf)
    }

    fn touch_desc(&mut self, desc: &str, via: &Via) {
        for c in class_refs(desc) {
            self.touch(&c, Level::Type, via.clone());
        }
    }

    // ── 实例化 / 初始化 ────────────────────────────────────────────────────

    fn instantiate(&mut self, cls: &str, via: Via, into: Option<usize>) {
        let id = self.id(cls);
        if !cls.starts_with('[') && self.touch(cls, Level::Alloc, via.clone()).is_none() {
            return;
        }
        if let Some(m) = into {
            let mut s = TypeSet::default();
            s.classes.insert(id);
            self.add_to(Node::M(m), &s);
        }
        if self.g.insert(id) {
            self.on_g_grow(id);
        }
    }

    fn on_g_grow(&mut self, id: u32) {
        let open: Vec<usize> = self.open_methods.iter().copied().collect();
        for m in open {
            self.push_m(m);
        }
        let pend: Vec<(usize, Vec<String>)> = self.pending_catch.iter().map(|(k, v)| (*k, v.clone())).collect();
        for (m, tys) in pend {
            let hit = tys.iter().any(|t| {
                let tid = self.id(t);
                self.sub(id, tid)
            });
            if hit {
                self.pending_catch.remove(&m);
                self.methods[m].analysis = None;
                self.push_m(m);
            }
        }
    }

    /// JVMS §5.5 初始化：超类链、声明非抽象实例方法的超接口、`<clinit>`
    pub fn init(&mut self, cls: &str, via: Via) {
        if cls.starts_with('[') || self.inited.contains_key(cls) {
            return;
        }
        let Some(cf) = self.touch(cls, Level::Init, via.clone()) else { return };
        self.inited.insert(cls.to_string(), via);
        if !cf.is_interface() {
            if let Some(s) = &cf.super_name {
                self.init(s, Via::class("super-init", cls));
            }
            for i in self.h.all_superinterfaces(&cf) {
                if i.methods.iter().any(|m| !m.is_static() && !m.is_abstract() && !m.is_private()) {
                    self.init(&i.name, Via::class("iface-init", cls));
                }
            }
        }
        if cf.method("<clinit>", "()V").is_some() {
            let k = MemberRef { owner: cls.to_string(), name: "<clinit>".into(), desc: "()V".into() };
            self.method(k, Via::class("clinit", cls));
        }
    }

    // ── 方法节点 ────────────────────────────────────────────────────────────

    fn kind_of(&self, cf: &ClassFile, m: &classfile::Method) -> Kind {
        let member = format!("{}.{}:{}", cf.name, m.name, m.desc);
        match self.domain(&cf.name) {
            Domain::Boundary => return Kind::Handwritten("boundary"),
            Domain::Root => return Kind::Handwritten("root"),
            _ => {}
        }
        if m.is_native() {
            return Kind::Handwritten("native");
        }
        if self.man.is_intrinsic(&member) {
            return Kind::Handwritten("intrinsic");
        }
        if m.name != "<clinit>" && self.provided(cf, &m.name, &m.desc) {
            return Kind::Handwritten("provides");
        }
        if m.code.is_none() {
            return Kind::Abstract;
        }
        Kind::Bytecode
    }

    /// 共置手写体按精确 Rust 名提供该成员（与发射侧 `_nf_covered` 同口径：mangle 名，或类内无重载时的裸名）
    fn provided(&self, cf: &ClassFile, name: &str, desc: &str) -> bool {
        let hw = self.hw.class(&cf.name);
        if hw.fns.is_empty() {
            return false;
        }
        let (rust, mangled) = self.rust_names(cf, name, desc);
        hw.fns.iter().any(|(f, i)| {
            let f = f.strip_prefix("__impl_").unwrap_or(f);
            i.is_pub && (f == mangled || rust.as_deref() == Some(f))
        })
    }

    /// (类内无重载时的裸名, mangle 名)
    fn rust_names(&self, cf: &ClassFile, name: &str, desc: &str) -> (Option<String>, String) {
        let base = if name == "<init>" { "new" } else { name };
        let suffix = self.hw.descriptor_suffix(desc);
        let mangled = if suffix.is_empty() { base.to_string() } else { format!("{base}_{suffix}") };
        let overloaded = cf.methods.iter().filter(|m| m.name == name).count() > 1;
        (if overloaded { None } else { Some(base.to_string()) }, mangled)
    }

    /// 方法节点（按声明类 + 名字 + 描述符）；首次登记入队
    fn method(&mut self, key: MemberRef, via: Via) -> usize {
        if let Some(i) = self.methods.get_index_of(&key) {
            return i;
        }
        let (kind, cf) = match self.h.class(&key.owner) {
            Some(cf) => match cf.method(&key.name, &key.desc) {
                Some(m) => (self.kind_of(&cf, m), Some(cf.clone())),
                None => (Kind::Missing, None),
            },
            None => (Kind::Missing, None),
        };
        let idx = self.methods.len();
        self.methods.insert(
            key.clone(),
            MNode { key: key.clone(), kind, via: via.clone(), set: TypeSet::default(), analysis: None, hw_fns: vec![] },
        );
        let lvl = if kind == Kind::Bytecode { Level::Code } else { Level::Type };
        self.touch(&key.owner, lvl, Via::method("member", idx, None));
        if cf.is_some() {
            let v = Via::method("signature", idx, None);
            self.touch_desc(&key.desc, &v);
        } else {
            self.unresolved.insert(key.to_string());
        }
        self.push_m(idx);
        idx
    }

    fn push_m(&mut self, m: usize) {
        if self.in_mwork.insert(m) {
            self.mwork.push_back(m);
        }
    }

    // ── 类型流 ──────────────────────────────────────────────────────────────

    fn set_of(&self, n: Node) -> &TypeSet {
        match n {
            Node::M(i) => &self.methods[i].set,
            Node::F(i) => &self.fields[i],
            Node::Array => &self.arrays,
        }
    }

    fn add_to(&mut self, n: Node, s: &TypeSet) {
        if s.is_empty() {
            return;
        }
        let changed = match n {
            Node::M(i) => self.methods[i].set.add_all(s),
            Node::F(i) => self.fields[i].add_all(s),
            Node::Array => self.arrays.add_all(s),
        };
        if changed {
            if let Node::M(i) = n {
                if !self.methods[i].set.open.is_empty() {
                    self.open_methods.insert(i);
                }
                self.push_m(i);
            }
            if self.in_fwork.insert(n) {
                self.fwork.push_back(n);
            }
        }
    }

    /// 流边 src → dst（按 filter 收窄）；立即按当前集合推一次
    fn flow(&mut self, src: Node, dst: Node, filter: Vec<u32>) {
        if filter.is_empty() {
            return;
        }
        let f: Rc<[u32]> = Rc::from(filter);
        if !self.flow_seen.insert((src, dst, f.clone())) {
            return;
        }
        self.flows.entry(src).or_default().push((dst, f.clone()));
        let s = self.set_of(src).clone();
        let out = self.filter(&s, &f);
        self.add_to(dst, &out);
    }

    fn drain_flows(&mut self) {
        while let Some(n) = self.fwork.pop_front() {
            self.in_fwork.remove(&n);
            let s = self.set_of(n).clone();
            let edges = self.flows.get(&n).cloned().unwrap_or_default();
            for (dst, f) in edges {
                let out = self.filter(&s, &f);
                self.add_to(dst, &out);
            }
        }
    }

    fn ref_ids(&mut self, types: &[FieldType]) -> Vec<u32> {
        types
            .iter()
            .filter(|t| t.is_reference())
            .map(|t| match t {
                FieldType::Object(c) => self.id(c),
                other => {
                    let d = other.descriptor();
                    self.id(&d)
                }
            })
            .collect()
    }

    fn field_node(&mut self, key: MemberRef) -> usize {
        if let Some(i) = self.fields.get_index_of(&key) {
            return i;
        }
        self.fields.insert(key, TypeSet::default());
        self.fields.len() - 1
    }

    // ── 主循环 ──────────────────────────────────────────────────────────────

    pub fn root(&mut self, key: MemberRef, kind: &'static str) {
        let via = Via::root(kind, &key.to_string());
        self.init(&key.owner.clone(), via.clone());
        let m = self.method(key.clone(), via);
        // 入口形参（String[] args 等）来自 VM：按声明类型 open
        if let Some(md) = parse_method(&key.desc) {
            let ids = self.ref_ids(&md.params);
            let s = TypeSet { classes: BTreeSet::new(), open: ids.into_iter().collect() };
            self.add_to(Node::M(m), &s);
        }
    }

    pub fn root_upcall(&mut self, u: &Upcall, kind: &'static str) {
        match u {
            Upcall::Method(k) => {
                let via = Via::root(kind, &k.to_string());
                if k.name == "<init>" {
                    self.instantiate(&k.owner, via.clone(), None);
                }
                self.init(&k.owner, via.clone());
                if let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, false) {
                    let (o, n, d) = site.key();
                    self.method(MemberRef { owner: o, name: n, desc: d }, via);
                } else {
                    self.unresolved.insert(k.to_string());
                }
            }
            Upcall::Field(f) => self.init(&f.owner, Via::root(kind, &f.to_string())),
        }
    }

    pub fn root_init(&mut self, cls: &str, kind: &'static str) {
        self.init(cls, Via::root(kind, cls));
    }

    pub fn run(&mut self) {
        loop {
            self.drain_flows();
            let Some(m) = self.mwork.pop_front() else { break };
            self.in_mwork.remove(&m);
            self.process(m);
        }
    }

    fn process(&mut self, m: usize) {
        match self.methods[m].kind {
            Kind::Bytecode => self.process_bytecode(m),
            Kind::Handwritten(_) => self.process_handwritten(m),
            Kind::Abstract | Kind::Missing => {}
        }
    }

    fn analysis(&mut self, m: usize) -> Option<Rc<Analysis>> {
        if let Some(a) = &self.methods[m].analysis {
            return Some(a.clone());
        }
        let key = self.methods[m].key.clone();
        let cf = self.h.class(&key.owner)?;
        let meth = cf.method(&key.name, &key.desc)?;
        let code = meth.code.as_ref()?;
        let g: Vec<u32> = self.g.iter().copied().collect();
        // catch 类型存活：G 中有其子类型（预先计算，避免 Oracle 借用引擎）
        let mut live_cache: HashMap<String, bool> = HashMap::new();
        for h in &code.exception_table {
            if let Some(ct) = &h.catch_type {
                let tid = self.id(ct);
                let live = g.iter().any(|&x| self.sub(x, tid));
                live_cache.insert(ct.clone(), live);
            }
        }
        let live = |t: &str| live_cache.get(t).copied().unwrap_or(true);
        let a = Rc::new(absint::analyze(&key.owner, &key.desc, meth.is_static(), code, &Facts { ctx: &self.ctx, live: &live }));
        if !a.pending_catch.is_empty() {
            self.pending_catch.insert(m, a.pending_catch.clone());
        }
        self.methods[m].analysis = Some(a.clone());
        Some(a)
    }

    fn process_bytecode(&mut self, m: usize) {
        let Some(a) = self.analysis(m) else { return };
        let owner = self.methods[m].key.owner.clone();
        let cf = self.h.class(&owner);
        let mut dead = Vec::new();
        for (off, e) in a.events.iter() {
            let off = *off;
            let via = |k: &'static str| Via::method(k, m, Some(off));
            match e {
                Event::New(c) => {
                    self.instantiate(c, via("new"), Some(m));
                    self.init(c, via("new"));
                }
                Event::NewArray(t) => {
                    self.touch(t, Level::Type, via("newarray"));
                    self.instantiate(t, via("newarray"), Some(m));
                }
                Event::Ldc(c) => self.ldc(m, off, c),
                Event::CheckCast(c) => {
                    self.touch(c, Level::Type, via("checkcast"));
                }
                Event::InstanceOf(c) => {
                    self.touch(c, Level::Type, via("instanceof"));
                }
                Event::Catch(ct) => {
                    let t = ct.clone().unwrap_or_else(|| THROWABLE.to_string());
                    self.touch(&t, Level::Type, via("catch"));
                    let id = self.id(&t);
                    let s = TypeSet { classes: BTreeSet::new(), open: [id].into() };
                    self.add_to(Node::M(m), &s);
                }
                Event::Field { opcode, mref, .. } => self.field(m, off, *opcode, mref),
                Event::Invoke { opcode, mref, iface, args } => self.invoke(m, off, *opcode, mref, *iface, args),
                Event::Indy { bsm, name, desc, args } => {
                    if let Some(cf) = &cf {
                        self.indy(m, off, cf, *bsm, name, desc, args);
                    }
                }
                Event::ArrayLoad { array } => {
                    let comp = array.static_type().and_then(absint::component);
                    let t = comp.as_deref().unwrap_or(OBJECT).to_string();
                    let tid = self.id(&t);
                    self.flow(Node::Array, Node::M(m), vec![tid]);
                    // open 数组（手写层产出）的元素同样 open
                    let opens: Vec<u32> = self.methods[m].set.open.iter().copied().collect();
                    let mut add = TypeSet::default();
                    for o in opens {
                        if let Some(c) = absint::component(&self.names[o as usize].clone()) {
                            let cid = self.id(&c);
                            if self.sub(cid, tid) {
                                add.open.insert(cid);
                            } else if self.sub(tid, cid) {
                                add.open.insert(tid);
                            }
                        }
                    }
                    self.add_to(Node::M(m), &add);
                }
                Event::ArrayStore { array, .. } => {
                    let comp = array.static_type().and_then(absint::component);
                    let tid = self.id(comp.as_deref().unwrap_or(OBJECT));
                    self.flow(Node::M(m), Node::Array, vec![tid]);
                }
                Event::Throw(_) => {}
                Event::DeadBranch(t) => dead.push((off, *t)),
            }
        }
        if !dead.is_empty() {
            self.dead.insert(m, dead);
        }
    }

    fn ldc(&mut self, m: usize, off: u32, c: &Const) {
        let via = Via::method("ldc", m, Some(off));
        match c {
            Const::String(_) => {
                self.instantiate("java/lang/String", via, Some(m));
            }
            Const::Class(n) => {
                self.touch(n, Level::Type, via.clone());
                self.instantiate("java/lang/Class", via, Some(m));
            }
            Const::MethodType(d) => self.touch_desc(d, &via),
            Const::MethodHandle(mh) => {
                let mh = mh.clone();
                self.invoke_mh(m, off, &mh, None);
            }
            Const::Dynamic(bsm, _, d) => {
                self.touch_desc(d, &via);
                let owner = self.methods[m].key.owner.clone();
                if let Some(cf) = self.h.class(&owner) {
                    if let Some(b) = cf.bootstrap_methods.get(*bsm as usize) {
                        let h = b.handle.clone();
                        self.invoke_mh(m, off, &h, None);
                        for a in b.args.clone() {
                            if let Const::MethodHandle(x) = a {
                                self.invoke_mh(m, off, &x, None);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn field(&mut self, m: usize, off: u32, opcode: u8, f: &MemberRef) {
        use classfile::op;
        let via = Via::method("field", m, Some(off));
        let Some(site) = self.h.resolve_field(&f.owner, &f.name, &f.desc) else {
            self.touch(&f.owner, Level::Type, via);
            self.unresolved.insert(f.to_string());
            return;
        };
        let decl = site.class.name.clone();
        self.touch(&f.owner, Level::Type, via.clone());
        self.touch_desc(&f.desc, &via);
        if opcode == op::GETSTATIC || opcode == op::PUTSTATIC {
            self.init(&decl, via.clone());
        }
        let Some(ft) = parse_field(&f.desc) else { return };
        if !ft.is_reference() {
            // 手写字段访问器仍需沿其回调入链
            self.field_handwritten(m, &decl, &f.name, &via, None);
            return;
        }
        let tid = self.ref_ids(std::slice::from_ref(&ft))[0];
        let key = MemberRef { owner: decl.clone(), name: f.name.clone(), desc: f.desc.clone() };
        let fi = self.field_node(key);
        if opcode == op::PUTSTATIC || opcode == op::PUTFIELD {
            self.flow(Node::M(m), Node::F(fi), vec![tid]);
        } else {
            self.flow(Node::F(fi), Node::M(m), vec![tid]);
            self.field_handwritten(m, &decl, &f.name, &via, Some((fi, tid)));
        }
    }

    /// 字段读：声明类是边界类或有共置手写文件 → 手写层可能写入，按 open 处理；
    /// 手写静态访问器（如标准流）声明的回调入链
    fn field_handwritten(&mut self, m: usize, decl: &str, name: &str, via: &Via, node: Option<(usize, u32)>) {
        let hwc = self.hw.class(decl);
        let boundary = matches!(self.domain(decl), Domain::Boundary | Domain::Root);
        if let Some((fi, tid)) = node {
            if boundary || !hwc.fns.is_empty() {
                let s = TypeSet { classes: BTreeSet::new(), open: [tid].into() };
                self.add_to(Node::F(fi), &s);
            }
        }
        if hwc.fns.is_empty() {
            return;
        }
        let mh = self.hw.member(decl, name);
        if mh.fns.is_empty() {
            return;
        }
        self.apply_hw(m, decl, &mh.upcalls, &mh.allocs, &mh.ctors, via);
    }

    fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, _args: &[V]) {
        use classfile::op;
        let via = Via::method("invoke", m, Some(off));
        self.touch(&mref.owner, Level::Type, via.clone());
        let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, iface) else {
            self.unresolved.insert(mref.to_string());
            return;
        };
        let (o, n, d) = site.key();
        let resolved = MemberRef { owner: o, name: n, desc: d };
        let md = parse_method(&mref.desc);
        let params = md.as_ref().map(|d| self.ref_ids(&d.params)).unwrap_or_default();
        let ret = md.as_ref().and_then(|d| d.ret.as_ref()).map(|r| self.ref_ids(std::slice::from_ref(r))).unwrap_or_default();
        match opcode {
            op::INVOKESTATIC => {
                self.init(&resolved.owner, via.clone());
                let t = self.method(resolved, via);
                self.edge(m, off, t, &params, &ret, None);
            }
            op::INVOKESPECIAL => {
                let t = self.method(resolved, via);
                self.edge(m, off, t, &params, &ret, None);
            }
            _ => {
                let rm = site.method();
                if rm.is_private() || rm.is_static() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface() {
                    // 非虚：直接到已解析方法（仍需接收者存在才有意义，此处不收窄）
                    let t = self.method(resolved, via);
                    self.edge(m, off, t, &params, &ret, None);
                    return;
                }
                let owner = self.id(&mref.owner);
                let s = self.methods[m].set.clone();
                let recv = self.receivers(&s, owner);
                for r in recv {
                    self.dispatch_one(m, off, r, &site, &params, &ret);
                }
            }
        }
    }

    /// 接收者 r 上分派已解析方法
    fn dispatch_one(&mut self, m: usize, off: u32, r: u32, site: &resolve::MethodSite, params: &[u32], ret: &[u32]) {
        let via = Via::method("dispatch", m, Some(off));
        if let Some(l) = self.lambdas.get(&r) {
            let rm = site.method();
            if rm.name == l.sam {
                let imh = l.imh.clone();
                self.invoke_mh(m, off, &imh, Some(params));
                return;
            }
            let iface = l.iface.clone();
            if let Some(sel) = self.h.select(&iface, site) {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, params, ret, None);
            }
            return;
        }
        let rname = self.names[r as usize].to_string();
        match self.h.select(&rname, site) {
            Some(sel) => {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, params, ret, Some(r));
            }
            None => {
                self.unresolved.insert(format!("select {rname} {}", site.method().name));
            }
        }
    }

    /// 调用边：实参流入（形参类型过滤）、返回值流回；接收者单独注入
    fn edge(&mut self, m: usize, off: u32, t: usize, params: &[u32], ret: &[u32], recv: Option<u32>) {
        self.dispatch.entry((m, off)).or_default().insert(t);
        self.flow(Node::M(m), Node::M(t), params.to_vec());
        self.flow(Node::M(t), Node::M(m), ret.to_vec());
        if let Some(r) = recv {
            let s = TypeSet { classes: [r].into(), open: BTreeSet::new() };
            self.add_to(Node::M(t), &s);
        }
    }

    /// 方法句柄调用（ldc MH / lambda 实现 / 引导方法静态实参）
    fn invoke_mh(&mut self, m: usize, off: u32, mh: &MethodHandle, sam_params: Option<&[u32]>) {
        let k = &mh.member;
        let via = Via::method("method-handle", m, Some(off));
        self.touch(&k.owner, Level::Type, via.clone());
        match mh.kind {
            // getField / getStatic / putField / putStatic
            1..=4 => {
                let opc = [0, classfile::op::GETFIELD, classfile::op::GETSTATIC, classfile::op::PUTFIELD, classfile::op::PUTSTATIC]
                    [mh.kind as usize];
                self.field(m, off, opc, k);
            }
            _ => {
                let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, mh.interface) else {
                    self.unresolved.insert(k.to_string());
                    return;
                };
                let (o, n, d) = site.key();
                let resolved = MemberRef { owner: o, name: n, desc: d };
                let md = parse_method(&k.desc);
                let mut params = md.as_ref().map(|d| self.ref_ids(&d.params)).unwrap_or_default();
                if let Some(sp) = sam_params {
                    params.extend_from_slice(sp);
                }
                let ret = md.as_ref().and_then(|d| d.ret.as_ref()).map(|r| self.ref_ids(std::slice::from_ref(r))).unwrap_or_default();
                match mh.kind {
                    6 => {
                        self.init(&resolved.owner, via.clone());
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, &params, &ret, None);
                    }
                    7 => {
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, &params, &ret, None);
                    }
                    8 => {
                        self.instantiate(&k.owner, via.clone(), Some(m));
                        self.init(&k.owner, via.clone());
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, &params, &ret, None);
                    }
                    _ => {
                        // 虚句柄：接收者 = 调用方类型集 ∪ open(声明类)（绑定 / 非绑定接收者都可能来自调用方外）
                        let owner = self.id(&k.owner);
                        let mut s = self.methods[m].set.clone();
                        s.open.insert(owner);
                        let recv = self.receivers(&s, owner);
                        for r in recv {
                            self.dispatch_one(m, off, r, &site, &params, &ret);
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn indy(&mut self, m: usize, off: u32, cf: &ClassFile, bsm: u16, name: &str, desc: &str, args: &[V]) {
        let via = Via::method("indy", m, Some(off));
        self.touch_desc(desc, &via);
        let Some(b) = cf.bootstrap_methods.get(bsm as usize).cloned() else { return };
        let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
        match self.man.indy_kind(&bkey) {
            Some(IndyKind::Lambda) => {
                let (Some(Const::MethodHandle(imh)), Some(ret)) =
                    (b.args.get(1), parse_method(desc).and_then(|d| d.ret).and_then(|r| r.class_ref().map(String::from)))
                else {
                    return;
                };
                let lname = format!("{}$$Lambda@{}:{}", cf.name, m, off);
                let lid = self.id(&lname);
                self.lambdas.insert(lid, Lambda { iface: ret.clone(), sam: name.to_string(), imh: imh.clone() });
                self.touch(&ret, Level::Alloc, via.clone());
                for a in &b.args {
                    if let Const::MethodType(d) = a {
                        self.touch_desc(d, &via);
                    }
                }
                let s = TypeSet { classes: [lid].into(), open: BTreeSet::new() };
                self.add_to(Node::M(m), &s);
                if self.g.insert(lid) {
                    self.on_g_grow(lid);
                }
                // 捕获实参流入实现方法（实现方法随 SAM 调用分派入链；静态 / 构造实现在此即入链，捕获值随之流入）
                if matches!(imh.kind, 6 | 7 | 8) {
                    let imh = imh.clone();
                    self.invoke_mh(m, off, &imh, None);
                }
            }
            Some(IndyKind::Concat) => {
                self.instantiate("java/lang/String", via.clone(), Some(m));
                let obj = self.id(OBJECT);
                let Some(site) = self.h.resolve_method(OBJECT, TO_STRING.0, TO_STRING.1, false) else { return };
                let s = self.methods[m].set.clone();
                for a in args {
                    if !matches!(a, V::Ref { .. } | V::Str(_) | V::Class(_)) {
                        continue;
                    }
                    let ty = a.static_type().map(|t| t.to_string());
                    let tid = ty.map(|t| self.id(&t)).unwrap_or(obj);
                    let mut s2 = self.filter(&s, &[tid]);
                    if matches!(a, V::Ref { .. }) && s2.is_empty() {
                        // 实参类型集为空：值来自调用方外（形参未流入），保持安全
                        s2.open.insert(tid);
                    }
                    let recv = self.receivers(&s2, tid);
                    for r in recv {
                        self.dispatch_one(m, off, r, &site, &[], &[]);
                    }
                }
            }
            Some(IndyKind::Native) => {
                for a in &b.args {
                    match a {
                        Const::MethodHandle(x) => {
                            let x = x.clone();
                            self.invoke_mh(m, off, &x, None);
                        }
                        Const::Class(c) => {
                            self.touch(c, Level::Type, via.clone());
                        }
                        _ => {}
                    }
                }
            }
            None => {
                let h = b.handle.clone();
                self.invoke_mh(m, off, &h, None);
                for a in &b.args {
                    if let Const::MethodHandle(x) = a {
                        let x = x.clone();
                        self.invoke_mh(m, off, &x, None);
                    }
                }
            }
        }
    }

    // ── 手写节点 ────────────────────────────────────────────────────────────

    fn process_handwritten(&mut self, m: usize) {
        let key = self.methods[m].key.clone();
        let via = Via::method("handwritten", m, None);
        // 手写返回值：open(返回类型)
        if let Some(r) = parse_method(&key.desc).and_then(|d| d.ret) {
            if r.is_reference() {
                let ids = self.ref_ids(std::slice::from_ref(&r));
                let s = TypeSet { classes: BTreeSet::new(), open: ids.into_iter().collect() };
                self.add_to(Node::M(m), &s);
            }
        }
        if key.name == "<clinit>" {
            return;
        }
        let Some(cf) = self.h.class(&key.owner) else { return };
        let mh = self.hw_member(&cf, &key.name, &key.desc);
        if self.methods[m].hw_fns.is_empty() {
            self.methods[m].hw_fns = mh.fns.clone();
        }
        let owner = key.owner.clone();
        self.apply_hw(m, &owner, &mh.upcalls, &mh.allocs, &mh.ctors, &via);
    }

    /// 精确匹配（mangle 名 / 无重载裸名）优先，否则按名字前缀（安全过近似）
    fn hw_member(&self, cf: &ClassFile, name: &str, desc: &str) -> crate::handwritten::MemberHw {
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
        }
        out.fns = exact;
        out
    }

    fn resolve_tref(&self, host: &str, t: &TypeRef) -> Option<String> {
        self.hw.resolve_type(host, t).into_iter().find(|c| self.cp.contains(c))
    }

    fn apply_hw(&mut self, m: usize, host: &str, upcalls: &[Upcall], allocs: &BTreeSet<TypeRef>, ctors: &BTreeSet<(TypeRef, String)>, via: &Via) {
        for t in allocs {
            if let Some(c) = self.resolve_tref(host, t) {
                self.instantiate(&c, via.clone(), Some(m));
                self.init(&c, via.clone());
            }
        }
        for (t, ctor) in ctors {
            let Some(c) = self.resolve_tref(host, t) else { continue };
            let Some(cf) = self.h.class(&c) else { continue };
            let inits: Vec<String> = cf
                .methods
                .iter()
                .filter(|x| x.name == "<init>" && self.rust_names(&cf, "<init>", &x.desc).1 == *ctor)
                .map(|x| x.desc.clone())
                .collect();
            if inits.is_empty() {
                continue;
            }
            self.instantiate(&c, via.clone(), Some(m));
            self.init(&c, via.clone());
            for d in inits {
                let t = self.method(MemberRef { owner: c.clone(), name: "<init>".into(), desc: d }, via.clone());
                self.edge(m, 0, t, &[], &[], None);
            }
        }
        for u in upcalls {
            match u {
                Upcall::Field(f) => {
                    let f = f.clone();
                    self.field(m, 0, classfile::op::GETSTATIC, &f);
                }
                Upcall::Method(k) => {
                    let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, self.h.is_interface(&k.owner)) else {
                        self.unresolved.insert(k.to_string());
                        continue;
                    };
                    let md = parse_method(&k.desc);
                    let params = md.as_ref().map(|d| self.ref_ids(&d.params)).unwrap_or_default();
                    let ret = md.as_ref().and_then(|d| d.ret.as_ref()).map(|r| self.ref_ids(std::slice::from_ref(r))).unwrap_or_default();
                    let (o, n, d) = site.key();
                    let resolved = MemberRef { owner: o, name: n, desc: d };
                    let rm = site.method();
                    if k.name == "<init>" {
                        self.instantiate(&k.owner, via.clone(), Some(m));
                        self.init(&k.owner, via.clone());
                        let t = self.method(resolved, via.clone());
                        self.edge(m, 0, t, &params, &ret, None);
                    } else if rm.is_static() || rm.is_private() {
                        self.init(&resolved.owner, via.clone());
                        let t = self.method(resolved, via.clone());
                        self.edge(m, 0, t, &params, &ret, None);
                    } else {
                        // JNI Call<T>Method：按接收者实际类型分派；接收者来自手写层 → open(引用类)
                        let owner = self.id(&k.owner);
                        let mut s = self.methods[m].set.clone();
                        s.open.insert(owner);
                        let recv = self.receivers(&s, owner);
                        for r in recv {
                            self.dispatch_one(m, 0, r, &site, &params, &ret);
                        }
                    }
                }
            }
        }
    }

    // ── 结果查询 ────────────────────────────────────────────────────────────

    pub fn instantiated(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .g
            .iter()
            .filter(|i| !self.lambdas.contains_key(i))
            .map(|i| self.names[*i as usize].to_string())
            .filter(|n| !n.starts_with('['))
            .collect();
        v.sort();
        v
    }

    pub fn lambda_count(&self) -> usize {
        self.lambdas.len()
    }

    pub fn method_label(&self, i: usize) -> String {
        self.methods[i].key.to_string()
    }
}
