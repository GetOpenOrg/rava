//! 闭包引擎：值级类型流（VTA）+ 虚分派 + 类初始化触发 + 手写节点 + 溯源（计划 §3.1 / §3.3 / §3.5）。
//!
//! 类型节点：方法形参 `P(m, i)`（实例方法 0 = this）、返回值 `R(m)`、方法内产生值的站点
//! `S(m, off)`（new / 调用返回 / 字段读 / 数组读 / indy / catch）、字段、全局数组元素。
//! 每个节点持有类型集 `TypeSet { classes, open }`：`classes` 是确定流入的已实例化类型；
//! `open(T)` 表示「任意已实例化的 T 子类型」——手写返回值、手写可写字段、异常处理器入口等
//! 无法在字节码层追踪的来源一律按 open 处理（安全回退）。流边按形参 / 返回 / 字段声明类型过滤。
//!
//! absint 给每个引用值标出来源（形参 / 站点），引擎按来源把实参接到被调形参、把返回值接回
//! 调用站点——虚调用的接收者只取接收者值本身的类型集，被分派方法的 this 精确注入实际接收者。
//! 手写方法没有字节码可追踪：其形参、分配、回调返回汇入一个 pool 站点，回调实参取自 pool。
//! G（全局已实例化）增长时，按 open 展开过接收者的方法与等待 catch 类型的方法重新处理——单调不动点。

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::rc::Rc;

use classfile::descriptor::{class_refs, parse_field, parse_method, FieldType};
use classfile::{acc, ClassFile, Const, MemberRef, MethodHandle};
use indexmap::IndexMap;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::absint::{self, Analysis, Event, Oracle, Src, V};
use crate::handwritten::{member_matches, Handwritten, MemberHw, TypedCall, TypeRef, Upcall};
use crate::manifest::{Domain, Fact, IndyKind, Manifest};

const OBJECT: &str = "java/lang/Object";
const STRING: &str = "java/lang/String";
const CLASS: &str = "java/lang/Class";
const THROWABLE: &str = "java/lang/Throwable";
const TO_STRING: (&str, &str) = ("toString", "()Ljava/lang/String;");
/// 站点键：异常处理器入口
const CATCH: u32 = 1 << 31;
/// 站点键：手写方法的值池
const POOL: u32 = u32::MAX;
/// 站点键：手写方法的数组元素池（实参数组之间的元素互通，如 arraycopy）
const ELEM: u32 = u32::MAX - 1;
/// 数组元素节点的下标奇偶槽
const PARITIES: [u8; 2] = [0, 1];

/// 下标值可能落入的奇偶槽：奇偶已知 → 一个，否则两个
fn slots(index: &V) -> Vec<u8> {
    match index.parity() {
        Some(p) => vec![p as u8],
        None => PARITIES.to_vec(),
    }
}

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
    fn exact(id: u32) -> TypeSet {
        TypeSet { classes: [id].into(), open: BTreeSet::new() }
    }
    fn open(id: u32) -> TypeSet {
        TypeSet { classes: BTreeSet::new(), open: [id].into() }
    }
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
    pub is_static: bool,
    /// 形参类型（实例方法 0 = 声明类；基本类型为 None）
    ptypes: Vec<Option<u32>>,
    /// 返回类型（引用）
    rtype: Option<u32>,
    analysis: Option<Rc<Analysis>>,
    /// 手写体命中的 fn 名（溯源）
    pub hw_fns: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Node {
    /// 形参（方法, 序号）
    P(usize, u16),
    /// 返回值
    R(usize),
    /// 方法内站点（方法, 偏移 | CATCH | POOL）
    S(usize, u32),
    F(usize),
    /// 数组分配点的元素（分配点, 下标奇偶 0 偶 / 1 奇）：键值交错数组的键、值分开
    E(u32, u8),
    /// 写入未知数组（open / 保守分析）的元素：流入每个数组分配点
    Array,
}

impl Node {
    /// 节点变化时需要重新处理的方法
    fn owner(self) -> Option<usize> {
        match self {
            Node::P(m, _) | Node::S(m, _) => Some(m),
            _ => None,
        }
    }
}

/// 值的类型来源：节点，或直接给定的类型集（字面量 / 未知值的 open）
#[derive(Clone, Debug)]
enum Feed {
    N(Node),
    S(TypeSet),
}

/// 按声明形参位置的实参来源（基本类型为 None）
type Args = Vec<Option<Vec<Feed>>>;

/// 被调方法的接收者
enum Recv {
    None,
    Exact(u32),
    Feeds(Vec<Feed>),
}

#[derive(Clone)]
struct Lambda {
    iface: String,
    sam: String,
    imh: MethodHandle,
    /// 捕获实参来源（按 indy 描述符形参位置）
    cap: Args,
}

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

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
    fields: IndexMap<MemberRef, ()>,
    sets: HashMap<Node, TypeSet>,

    /// G：全局已实例化（类型 id）
    g: BTreeSet<u32>,
    lambdas: HashMap<u32, Lambda>,
    /// 数组分配点（抽象对象 id）→ 数组类型 id
    arrays: HashMap<u32, u32>,
    pub inited: IndexMap<String, Via>,

    flows: HashMap<Node, Vec<(Node, u32)>>,
    flow_seen: HashSet<(Node, Node, u32)>,
    /// 调用点分派结果：(方法, 偏移) → 目标方法
    pub dispatch: BTreeMap<(usize, u32), BTreeSet<usize>>,
    /// 死分支（方法 → 剪掉的目标偏移）
    pub dead: BTreeMap<usize, Vec<(u32, u32)>>,
    pub unresolved: BTreeSet<String>,

    mwork: VecDeque<usize>,
    in_mwork: HashSet<usize>,
    fwork: VecDeque<Node>,
    in_fwork: HashSet<Node>,
    /// 按 open 在 G 上展开过接收者的方法（G 增长时重处理）
    open_methods: BTreeSet<usize>,
    pending_catch: BTreeMap<usize, Vec<String>>,
    /// 进行中的 lambda 调用（lambda, 实参）：绑定方法引用的接收者可能是 lambda 自身，同一调用重入即成环
    lambda_stack: HashSet<(u32, String)>,
    /// 待沿流边推送的新增类型（差分传播）
    fdelta: HashMap<Node, TypeSet>,
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
            sets: HashMap::new(),
            g: BTreeSet::new(),
            lambdas: HashMap::new(),
            arrays: HashMap::new(),
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
            lambda_stack: HashSet::new(),
            fdelta: HashMap::new(),
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
        let r = if let Some(&t) = self.arrays.get(&x) {
            self.sub(t, f)
        } else if let Some(l) = self.lambdas.get(&x) {
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

    /// 类型集按过滤类型收窄
    fn filter(&mut self, s: &TypeSet, t: u32) -> TypeSet {
        let mut out = TypeSet::default();
        for &x in &s.classes {
            if self.sub(x, t) {
                out.classes.insert(x);
            }
        }
        for &o in &s.open {
            if self.sub(o, t) {
                out.open.insert(o);
            } else if self.sub(t, o) || self.is_iface(o) || self.is_iface(t) {
                out.open.insert(t);
            }
        }
        out
    }

    /// 类型集里 ⊂ owner 的具体接收者（open 按 G 展开；展开过的方法 m 在 G 增长时重处理）
    fn receivers(&mut self, m: usize, s: &TypeSet, owner: u32) -> BTreeSet<u32> {
        let mut out = BTreeSet::new();
        for &x in &s.classes {
            if self.sub(x, owner) {
                out.insert(x);
            }
        }
        if !s.open.is_empty() {
            self.open_methods.insert(m);
            let g: Vec<u32> = self.g.iter().copied().collect();
            let opens: Vec<u32> = s.open.iter().copied().collect();
            for x in g {
                if out.contains(&x) || !self.sub(x, owner) {
                    continue;
                }
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

    /// 数组分配点：独立的抽象对象（元素节点 `E(id)`），类型为数组类型
    fn array_site(&mut self, m: usize, off: u32, t: &str, via: Via) -> u32 {
        let name = format!("{t}@{m}:{off}");
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        self.touch(t, Level::Type, via);
        let tid = self.id(t);
        let id = self.id(&name);
        self.arrays.insert(id, tid);
        // 多维数组的内层数组来自同一条指令：按 open(分量类型) 处理
        if let Some(c) = absint::component(t) {
            if c.starts_with('[') {
                let cid = self.id(&c);
                for p in PARITIES {
                    self.add_to(Node::E(id, p), &TypeSet::open(cid));
                }
            }
        }
        let cid = absint::component(t).filter(|c| c.len() > 1).map(|c| self.id(&c));
        if let Some(cid) = cid {
            for p in PARITIES {
                self.flow(Node::Array, Node::E(id, p), cid);
            }
        }
        if self.g.insert(id) {
            self.on_g_grow(id);
        }
        id
    }

    fn instantiate(&mut self, cls: &str, via: Via) {
        let id = self.id(cls);
        if !cls.starts_with('[') && self.touch(cls, Level::Alloc, via).is_none() {
            return;
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

    fn ptype(&mut self, t: &FieldType) -> Option<u32> {
        match t {
            FieldType::Object(c) => Some(self.id(c)),
            other if other.is_reference() => {
                let d = other.descriptor();
                Some(self.id(&d))
            }
            _ => None,
        }
    }

    /// 方法节点（按声明类 + 名字 + 描述符）；首次登记入队
    fn method(&mut self, key: MemberRef, via: Via) -> usize {
        if let Some(i) = self.methods.get_index_of(&key) {
            return i;
        }
        let (kind, cf, is_static) = match self.h.class(&key.owner) {
            Some(cf) => match cf.method(&key.name, &key.desc) {
                Some(m) => (self.kind_of(&cf, m), Some(cf.clone()), m.is_static()),
                None => (Kind::Missing, None, false),
            },
            None => (Kind::Missing, None, false),
        };
        let mut ptypes = Vec::new();
        if !is_static {
            ptypes.push(Some(self.id(&key.owner)));
        }
        let md = parse_method(&key.desc);
        let mut rtype = None;
        if let Some(md) = &md {
            for p in &md.params {
                let t = self.ptype(p);
                ptypes.push(t);
            }
            rtype = md.ret.as_ref().and_then(|r| self.ptype(r));
        }
        let idx = self.methods.len();
        self.methods.insert(
            key.clone(),
            MNode { key: key.clone(), kind, via: via.clone(), is_static, ptypes, rtype, analysis: None, hw_fns: vec![] },
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

    fn set_of(&self, n: Node) -> TypeSet {
        self.sets.get(&n).cloned().unwrap_or_default()
    }

    fn add_to(&mut self, n: Node, s: &TypeSet) {
        if s.is_empty() {
            return;
        }
        let cur = self.sets.entry(n).or_default();
        let delta = TypeSet {
            classes: s.classes.difference(&cur.classes).copied().collect(),
            open: s.open.difference(&cur.open).copied().collect(),
        };
        if delta.is_empty() {
            return;
        }
        cur.add_all(&delta);
        if let Some(m) = n.owner() {
            self.push_m(m);
        }
        // 只沿流边推送新增部分（差分传播）
        self.fdelta.entry(n).or_default().add_all(&delta);
        if self.in_fwork.insert(n) {
            self.fwork.push_back(n);
        }
    }

    /// 流边 src → dst（按 filter 收窄）；立即按当前集合推一次
    fn flow(&mut self, src: Node, dst: Node, filter: u32) {
        if !self.flow_seen.insert((src, dst, filter)) {
            return;
        }
        self.flows.entry(src).or_default().push((dst, filter));
        let s = self.set_of(src);
        let out = self.filter(&s, filter);
        self.add_to(dst, &out);
    }

    fn drain_flows(&mut self) {
        while let Some(n) = self.fwork.pop_front() {
            self.in_fwork.remove(&n);
            let Some(s) = self.fdelta.remove(&n) else { continue };
            let edges = self.flows.get(&n).cloned().unwrap_or_default();
            for (dst, f) in edges {
                let out = self.filter(&s, f);
                self.add_to(dst, &out);
            }
        }
    }

    /// 方法 m 内抽象值 v 的类型来源；未知值按声明类型 open
    fn feeds(&mut self, m: usize, v: &V, decl: u32) -> Vec<Feed> {
        match v {
            V::Str(_) => vec![Feed::S(TypeSet::exact(self.id(STRING)))],
            V::Class(_) => vec![Feed::S(TypeSet::exact(self.id(CLASS)))],
            V::Top => vec![Feed::S(TypeSet::open(decl))],
            V::Ref { src, .. } => src
                .iter()
                .map(|s| match *s {
                    Src::Param(i) => Feed::N(Node::P(m, i)),
                    Src::Site(o) => Feed::N(Node::S(m, o)),
                    Src::Catch(o) => Feed::N(Node::S(m, CATCH | o)),
                    Src::Str => Feed::S(TypeSet::exact(self.id(STRING))),
                    Src::Class => Feed::S(TypeSet::exact(self.id(CLASS))),
                })
                .collect(),
            _ => vec![],
        }
    }

    fn feed(&mut self, fs: &[Feed], dst: Node, filter: u32) {
        for f in fs {
            match f {
                Feed::N(n) => self.flow(*n, dst, filter),
                Feed::S(s) => {
                    let out = self.filter(s, filter);
                    self.add_to(dst, &out);
                }
            }
        }
    }

    fn value_set(&self, fs: &[Feed]) -> TypeSet {
        let mut out = TypeSet::default();
        for f in fs {
            match f {
                Feed::N(n) => {
                    if let Some(s) = self.sets.get(n) {
                        out.add_all(s);
                    }
                }
                Feed::S(s) => {
                    out.add_all(s);
                }
            }
        }
        out
    }

    /// 形参类型列表 → 全部取自同一来源的实参
    fn args_from(&mut self, desc: &str, f: impl Fn(u32) -> Vec<Feed>) -> Args {
        let Some(md) = parse_method(desc) else { return vec![] };
        md.params.iter().map(|p| self.ptype(p).map(&f)).collect()
    }

    fn field_node(&mut self, key: MemberRef) -> usize {
        self.fields.insert_full(key, ()).0
    }

    // ── 主循环 ──────────────────────────────────────────────────────────────

    /// 入口形参来自 VM / 手写层：按声明类型 open
    fn open_params(&mut self, t: usize) {
        let pts = self.methods[t].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            if let Some(pt) = pt {
                self.add_to(Node::P(t, i as u16), &TypeSet::open(*pt));
            }
        }
    }

    pub fn root(&mut self, key: MemberRef, kind: &'static str) {
        let via = Via::root(kind, &key.to_string());
        self.init(&key.owner.clone(), via.clone());
        let m = self.method(key, via);
        self.open_params(m);
    }

    pub fn root_upcall(&mut self, u: &Upcall, kind: &'static str) {
        match u {
            Upcall::Method(k) => {
                let via = Via::root(kind, &k.to_string());
                if k.name == "<init>" {
                    self.instantiate(&k.owner, via.clone());
                }
                self.init(&k.owner, via.clone());
                if let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, false) {
                    let (o, n, d) = site.key();
                    let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                    self.open_params(t);
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
        let obj = self.id(OBJECT);
        let mut dead = Vec::new();
        for (off, e) in a.events.iter() {
            let off = *off;
            let via = |k: &'static str| Via::method(k, m, Some(off));
            match e {
                Event::New(c) => {
                    self.instantiate(c, via("new"));
                    self.init(c, via("new"));
                    let id = self.id(c);
                    self.add_to(Node::S(m, off), &TypeSet::exact(id));
                }
                Event::NewArray(t) => {
                    let id = self.array_site(m, off, t, via("newarray"));
                    self.add_to(Node::S(m, off), &TypeSet::exact(id));
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
                    self.add_to(Node::S(m, CATCH | off), &TypeSet::open(id));
                }
                Event::Field { opcode, mref, value } => self.field(m, off, *opcode, mref, value.as_ref(), Node::S(m, off)),
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
                    for &x in &s.classes {
                        if self.arrays.contains_key(&x) {
                            for p in slots(index) {
                                self.flow(Node::E(x, p), Node::S(m, off), tid);
                            }
                        } else {
                            add.open.insert(tid);
                        }
                    }
                    // open 数组（手写层 / VM 产出）的元素同样 open
                    for &o in &s.open {
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
                    for &x in &s.classes {
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
                self.instantiate(STRING, via);
            }
            Const::Class(n) => {
                self.touch(n, Level::Type, via.clone());
                self.instantiate(CLASS, via);
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

    /// 字段访问：写入值来源 → 字段节点；读 → 结果节点 `res`
    fn field(&mut self, m: usize, off: u32, opcode: u8, f: &MemberRef, value: Option<&V>, res: Node) {
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
        let Some(tid) = self.ptype(&ft) else {
            // 手写字段访问器仍需沿其回调入链
            self.field_handwritten(m, &decl, &f.name, &via, None);
            return;
        };
        let key = MemberRef { owner: decl.clone(), name: f.name.clone(), desc: f.desc.clone() };
        let fi = self.field_node(key);
        if opcode == op::PUTSTATIC || opcode == op::PUTFIELD {
            let fs = match value {
                Some(v) => self.feeds(m, v, tid),
                None => vec![Feed::S(TypeSet::open(tid))],
            };
            self.feed(&fs, Node::F(fi), tid);
        } else {
            self.flow(Node::F(fi), res, tid);
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
                self.add_to(Node::F(fi), &TypeSet::open(tid));
            }
        }
        if hwc.fns.is_empty() {
            return;
        }
        let mh = self.hw.member(decl, name);
        if mh.fns.is_empty() {
            return;
        }
        self.apply_hw(m, decl, &mh, via);
    }

    fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
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
                self.init(&resolved.owner, via.clone());
                let t = self.method(resolved, via);
                self.edge(m, off, t, Recv::None, &a, ret, res);
            }
            op::INVOKESPECIAL => {
                let t = self.method(resolved, via);
                let r = recv_feeds(self);
                self.edge(m, off, t, Recv::Feeds(r), &a, ret, res);
            }
            _ => {
                let rm = site.method();
                if rm.is_private() || rm.is_static() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface() {
                    // 非虚：直接到已解析方法，接收者值流入 this
                    let t = self.method(resolved, via);
                    let r = recv_feeds(self);
                    self.edge(m, off, t, Recv::Feeds(r), &a, ret, res);
                    return;
                }
                let r = recv_feeds(self);
                let s = self.value_set(&r);
                let recv = self.receivers(m, &s, owner);
                for r in recv {
                    self.dispatch_one(m, off, r, &site, &a, ret, res);
                }
            }
        }
    }

    /// 接收者 r 上分派已解析方法
    #[allow(clippy::too_many_arguments)]
    fn dispatch_one(&mut self, m: usize, off: u32, r: u32, site: &resolve::MethodSite, a: &Args, ret: Option<u32>, res: Option<Node>) {
        let via = Via::method("dispatch", m, Some(off));
        if let Some(l) = self.lambdas.get(&r) {
            if site.method().name == l.sam {
                self.invoke_lambda(m, off, r, a, ret, res);
                return;
            }
            let iface = l.iface.clone();
            if let Some(sel) = self.h.select(&iface, site) {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            return;
        }
        let rt = self.arrays.get(&r).copied().unwrap_or(r);
        let rname = self.names[rt as usize].to_string();
        match self.h.select(&rname, site) {
            Some(sel) => {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            None => {
                self.unresolved.insert(format!("select {rname} {}", site.method().name));
            }
        }
    }

    /// 调用边：接收者注入 this、实参按位置流入形参（被调声明类型过滤）、返回值流回结果节点
    #[allow(clippy::too_many_arguments)]
    fn edge(&mut self, m: usize, off: u32, t: usize, recv: Recv, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        self.dispatch.entry((m, off)).or_default().insert(t);
        let is_static = self.methods[t].is_static;
        let ptypes = self.methods[t].ptypes.clone();
        let base = usize::from(!is_static);
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
        if let (Some(rt), Some(res)) = (ret, res) {
            if self.man.returns_receiver(&self.methods[t].key.to_string()) {
                // 浅拷贝：返回值 = 接收者本身的类型集（数组共享元素节点）
                self.flow(Node::P(t, 0), res, rt);
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

    /// 字节码方法的返回值只来自形参时，返回这些形参序号
    fn passthrough(&mut self, t: usize) -> Option<Vec<u16>> {
        if self.methods[t].kind != Kind::Bytecode {
            return None;
        }
        self.analysis(t)?.returned_params()
    }

    /// lambda 的 SAM 调用：捕获实参 ++ SAM 实参 → 实现方法
    fn invoke_lambda(&mut self, m: usize, off: u32, lid: u32, a: &Args, ret: Option<u32>, res: Option<Node>) {
        // 同一 lambda 以相同实参重入：外层调用已接上全部流边
        let key = (lid, format!("{a:?}|{ret:?}|{res:?}"));
        if !self.lambda_stack.insert(key.clone()) {
            return;
        }
        self.invoke_lambda_body(m, off, lid, a, ret, res);
        self.lambda_stack.remove(&key);
    }

    fn invoke_lambda_body(&mut self, m: usize, off: u32, lid: u32, a: &Args, ret: Option<u32>, res: Option<Node>) {
        let l = self.lambdas[&lid].clone();
        let k = &l.imh.member;
        let via = Via::method("lambda", m, Some(off));
        let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, l.imh.interface) else {
            self.unresolved.insert(k.to_string());
            return;
        };
        let (o, n, d) = site.key();
        let resolved = MemberRef { owner: o, name: n, desc: d };
        let mut all: Args = l.cap.clone();
        all.extend(a.iter().cloned());
        let first = || all.first().cloned().flatten().unwrap_or_default();
        let rest = all.get(1..).unwrap_or(&[]).to_vec();
        match l.imh.kind {
            6 => {
                let t = self.method(resolved, via);
                self.edge(m, off, t, Recv::None, &all, ret, res);
            }
            7 => {
                let t = self.method(resolved, via);
                self.edge(m, off, t, Recv::Feeds(first()), &rest, ret, res);
            }
            8 => {
                let oid = self.id(&k.owner);
                let t = self.method(resolved, via);
                self.edge(m, off, t, Recv::Exact(oid), &all, None, None);
                if let (Some(res), Some(rt)) = (res, ret) {
                    let s = self.filter(&TypeSet::exact(oid), rt);
                    self.add_to(res, &s);
                }
            }
            _ => {
                let owner = self.id(&k.owner);
                let s = self.value_set(&first());
                let recv = self.receivers(m, &s, owner);
                for r in recv {
                    self.dispatch_one(m, off, r, &site, &rest, ret, res);
                }
            }
        }
    }

    /// 方法句柄（ldc MH / 引导方法及其静态实参）：由 VM / 库内部调用，实参按声明类型 open
    fn invoke_mh(&mut self, m: usize, off: u32, mh: &MethodHandle) {
        let k = &mh.member;
        let via = Via::method("method-handle", m, Some(off));
        self.touch(&k.owner, Level::Type, via.clone());
        match mh.kind {
            // getField / getStatic / putField / putStatic
            1..=4 => {
                let opc = [0, classfile::op::GETFIELD, classfile::op::GETSTATIC, classfile::op::PUTFIELD, classfile::op::PUTSTATIC]
                    [mh.kind as usize];
                self.field(m, off, opc, k, None, Node::S(m, off));
            }
            _ => {
                let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, mh.interface) else {
                    self.unresolved.insert(k.to_string());
                    return;
                };
                let (o, n, d) = site.key();
                let resolved = MemberRef { owner: o, name: n, desc: d };
                let a = self.args_from(&k.desc, |t| vec![Feed::S(TypeSet::open(t))]);
                let owner = self.id(&k.owner);
                match mh.kind {
                    6 => {
                        self.init(&resolved.owner, via.clone());
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, Recv::None, &a, None, None);
                    }
                    7 => {
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, Recv::Feeds(vec![Feed::S(TypeSet::open(owner))]), &a, None, None);
                    }
                    8 => {
                        self.instantiate(&k.owner, via.clone());
                        self.init(&k.owner, via.clone());
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, Recv::Exact(owner), &a, None, None);
                    }
                    _ => {
                        let recv = self.receivers(m, &TypeSet::open(owner), owner);
                        for r in recv {
                            self.dispatch_one(m, off, r, &site, &a, None, None);
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
        let Some(md) = parse_method(desc) else { return };
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
        match self.man.indy_kind(&bkey) {
            Some(IndyKind::Lambda) => {
                let (Some(Const::MethodHandle(imh)), Some(iface)) =
                    (b.args.get(1), md.ret.as_ref().and_then(|r| r.class_ref().map(String::from)))
                else {
                    return;
                };
                let mut cap: Args = Vec::with_capacity(md.params.len());
                for (p, v) in md.params.iter().zip(args.iter()) {
                    let f = self.ptype(p).map(|t| self.feeds(m, v, t));
                    cap.push(f);
                }
                let lname = format!("{}$$Lambda@{}:{}", cf.name, m, off);
                let lid = self.id(&lname);
                self.lambdas.insert(lid, Lambda { iface: iface.clone(), sam: name.to_string(), imh: imh.clone(), cap });
                self.touch(&iface, Level::Alloc, via.clone());
                for a in &b.args {
                    if let Const::MethodType(d) = a {
                        self.touch_desc(d, &via);
                    }
                }
                self.add_to(Node::S(m, off), &TypeSet::exact(lid));
                if self.g.insert(lid) {
                    self.on_g_grow(lid);
                }
                // 静态 / 私有 / 构造实现在创建点即入链（实参随 SAM 调用接入）
                let k = &imh.member;
                if matches!(imh.kind, 6..=8) {
                    if let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, imh.interface) {
                        let (o, n, d) = site.key();
                        if imh.kind == 6 {
                            self.init(&o, via.clone());
                        }
                        if imh.kind == 8 {
                            self.instantiate(&k.owner, via.clone());
                            self.init(&k.owner, via.clone());
                        }
                        self.method(MemberRef { owner: o, name: n, desc: d }, via);
                    } else {
                        self.unresolved.insert(k.to_string());
                    }
                }
            }
            Some(IndyKind::Concat) => {
                self.instantiate(STRING, via.clone());
                let sid = self.id(STRING);
                self.add_to(Node::S(m, off), &TypeSet::exact(sid));
                let Some(site) = self.h.resolve_method(OBJECT, TO_STRING.0, TO_STRING.1, false) else { return };
                for (p, v) in md.params.iter().zip(args.iter()) {
                    let Some(t) = self.ptype(p) else { continue };
                    let fs = self.feeds(m, v, t);
                    let s = self.value_set(&fs);
                    let recv = self.receivers(m, &s, t);
                    for r in recv {
                        self.dispatch_one(m, off, r, &site, &vec![], None, None);
                    }
                }
            }
            kind => {
                // 引导方法产出的调用点：结果按声明类型 open
                if let Some(rt) = ret {
                    self.add_to(Node::S(m, off), &TypeSet::open(rt));
                }
                if kind.is_none() {
                    let h = b.handle.clone();
                    self.invoke_mh(m, off, &h);
                }
                for a in &b.args {
                    match a {
                        Const::MethodHandle(x) => {
                            let x = x.clone();
                            self.invoke_mh(m, off, &x);
                        }
                        Const::Class(c) => {
                            self.touch(c, Level::Type, via.clone());
                        }
                        _ => {}
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
        if let Some(rt) = self.methods[m].rtype {
            if !self.man.returns_receiver(&key.to_string()) {
                self.add_to(Node::R(m), &TypeSet::open(rt));
            }
        }
        if key.name == "<clinit>" {
            return;
        }
        // 形参汇入值池（回调实参取自值池）
        let pts = self.methods[m].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            if let Some(pt) = pt {
                self.flow(Node::P(m, i as u16), Node::S(m, POOL), *pt);
            }
        }
        // 实参数组的元素经元素池互通（arraycopy 等），手写层产出的值也可能写入实参数组
        let obj = self.id(OBJECT);
        self.flow(Node::S(m, POOL), Node::S(m, ELEM), obj);
        for i in 0..pts.len() {
            let s = self.set_of(Node::P(m, i as u16));
            for x in s.classes {
                if let Some(&t) = self.arrays.get(&x) {
                    let Some(c) = absint::component(&self.names[t as usize].clone()).filter(|c| c.len() > 1) else { continue };
                    let cid = self.id(&c);
                    for p in PARITIES {
                        self.flow(Node::E(x, p), Node::S(m, ELEM), obj);
                        self.flow(Node::S(m, ELEM), Node::E(x, p), cid);
                    }
                }
            }
        }
        let Some(cf) = self.h.class(&key.owner) else { return };
        let mh = self.hw_member(&cf, &key.name, &key.desc);
        if self.methods[m].hw_fns.is_empty() {
            self.methods[m].hw_fns = mh.fns.clone();
        }
        let owner = key.owner.clone();
        self.apply_hw(m, &owner, &mh, &via);
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
            out.calls.extend(i.calls.iter().cloned());
            out.opaque.extend(i.opaque.iter().cloned());
        }
        out.fns = exact;
        out
    }

    fn resolve_tref(&self, host: &str, t: &TypeRef) -> Option<String> {
        self.hw.resolve_type(host, t).into_iter().find(|c| self.cp.contains(c))
    }

    /// 手写体调用点推断出的具体类型（须是可实例化的类）
    fn hw_type(&mut self, host: &str, t: &Option<TypeRef>) -> Option<u32> {
        let c = self.resolve_tref(host, t.as_ref()?)?;
        let cf = self.h.class(&c)?;
        if cf.is_interface() || cf.access & acc::ABSTRACT != 0 {
            return None;
        }
        Some(self.id(&c))
    }

    /// 回调 / 构造在手写体里的调用点：同名（Rust 名规则）、实参个数一致；
    /// 同名标识符出现在宏内（syn 不展开）或找不到调用点 → None（退回值池）
    fn hw_sites<'b>(mh: &'b MemberHw, matches: impl Fn(&TypedCall) -> bool, java_name: &str, nargs: usize) -> Option<Vec<&'b TypedCall>> {
        if mh.opaque.iter().any(|i| member_matches(i, java_name)) {
            return None;
        }
        let v: Vec<&TypedCall> = mh.calls.iter().filter(|c| c.args.len() == nargs && matches(c)).collect();
        (!v.is_empty()).then_some(v)
    }

    /// 实参来源：各调用点该位置都推断出具体类型 → 精确类型集；否则值池
    fn hw_args(&mut self, m: usize, host: &str, desc: &str, sites: &Option<Vec<&TypedCall>>) -> Args {
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

    /// 手写体效果：分配 / 构造 / 回调。实参按手写体调用点的语法推断精确接入，推断不出的经方法 m 的值池流转
    fn apply_hw(&mut self, m: usize, host: &str, mh: &MemberHw, via: &Via) {
        let pool = Node::S(m, POOL);
        for t in &mh.allocs {
            if let Some(c) = self.resolve_tref(host, t) {
                self.instantiate(&c, via.clone());
                self.init(&c, via.clone());
                let id = self.id(&c);
                self.add_to(pool, &TypeSet::exact(id));
            }
        }
        for (t, ctor) in &mh.ctors {
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
            self.instantiate(&c, via.clone());
            self.init(&c, via.clone());
            let id = self.id(&c);
            self.add_to(pool, &TypeSet::exact(id));
            for d in inits {
                let n = parse_method(&d).map_or(0, |md| md.params.len());
                let sites = Self::hw_sites(mh, |x| x.name == *ctor && x.path_ty.as_ref() == Some(t), "<init>", n);
                let a = self.hw_args(m, host, &d, &sites);
                let t = self.method(MemberRef { owner: c.clone(), name: "<init>".into(), desc: d }, via.clone());
                self.edge(m, 0, t, Recv::Exact(id), &a, None, None);
            }
        }
        for u in &mh.upcalls {
            match u {
                Upcall::Field(f) => {
                    let f = f.clone();
                    self.field(m, 0, classfile::op::GETSTATIC, &f, None, pool);
                }
                Upcall::Method(k) => {
                    let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, self.h.is_interface(&k.owner)) else {
                        self.unresolved.insert(k.to_string());
                        continue;
                    };
                    let n = parse_method(&k.desc).map_or(0, |md| md.params.len());
                    let sites = Self::hw_sites(mh, |x| member_matches(&x.name, &k.name), &k.name, n);
                    let a = self.hw_args(m, host, &k.desc, &sites);
                    let ret = parse_method(&k.desc).and_then(|d| d.ret).and_then(|r| self.ptype(&r));
                    let (o, nm, d) = site.key();
                    let resolved = MemberRef { owner: o, name: nm, desc: d };
                    let rm = site.method();
                    let owner = self.id(&k.owner);
                    if k.name == "<init>" {
                        self.instantiate(&k.owner, via.clone());
                        self.init(&k.owner, via.clone());
                        self.add_to(pool, &TypeSet::exact(owner));
                        let t = self.method(resolved, via.clone());
                        self.edge(m, 0, t, Recv::Exact(owner), &a, None, None);
                        continue;
                    }
                    // 接收者：各方法调用点都推断出具体类型 → 精确；否则来自手写层的 open(引用类)
                    let mut recv = TypeSet::default();
                    let mut typed = sites.as_ref().is_some_and(|v| v.iter().all(|c| c.recv.is_some()));
                    for c in sites.iter().flatten() {
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
                        self.edge(m, 0, t, Recv::Feeds(vec![Feed::S(recv)]), &a, ret, Some(pool));
                    } else {
                        // JNI Call<T>Method：按接收者实际类型分派
                        let rs = self.receivers(m, &recv, owner);
                        for r in rs {
                            self.dispatch_one(m, 0, r, &site, &a, ret, Some(pool));
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

    fn set_str(&self, s: &TypeSet) -> String {
        let mut v: Vec<String> = if s.classes.len() > 6 {
            vec![format!("{} 个类", s.classes.len())]
        } else {
            s.classes.iter().map(|i| self.names[*i as usize].to_string()).collect()
        };
        v.extend(s.open.iter().map(|i| format!("open({})", self.names[*i as usize])));
        v.join(", ")
    }

    fn node_str(&self, n: Node) -> String {
        match n {
            Node::P(m, i) => format!("P{i} {}", self.method_label(m)),
            Node::R(m) => format!("R {}", self.method_label(m)),
            Node::S(m, o) if o == POOL => format!("pool {}", self.method_label(m)),
            Node::S(m, o) if o == ELEM => format!("elem {}", self.method_label(m)),
            Node::S(m, o) if o & CATCH != 0 => format!("catch@{} {}", o & !CATCH, self.method_label(m)),
            Node::S(m, o) => format!("@{o} {}", self.method_label(m)),
            Node::F(f) => format!("field {}", self.fields.get_index(f).map(|(k, _)| k.to_string()).unwrap_or_default()),
            Node::E(x, p) => format!("elements[{}] {}", if p == 0 { "偶" } else { "奇" }, self.names[x as usize]),
            Node::Array => "array".into(),
        }
    }

    /// 类型流诊断：方法（标签含 `pat`）的形参 / 返回节点的类型集，以及流入它们的来源节点
    pub fn flows_of(&self, pat: &str) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(q) = pat.strip_prefix("elem:") {
            let mut es: Vec<Node> = self.sets.keys().filter(|n| matches!(n, Node::E(x, _) if self.names[*x as usize].contains(q))).copied().collect();
            es.sort_by_key(|n| format!("{n:?}"));
            for n in es {
                let s = self.sets.get(&n).cloned().unwrap_or_default();
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(&s)));
                for (src, edges) in &self.flows {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.sets.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.names[*f as usize], self.set_str(&s)));
                        }
                    }
                }
            }
            return out;
        }
        if pat == "@array" {
            for (src, edges) in &self.flows {
                if edges.iter().any(|(d, _)| *d == Node::Array) {
                    let s = self.sets.get(src).cloned().unwrap_or_default();
                    out.push(format!("  array ← {} {{{}}}", self.node_str(*src), self.set_str(&s)));
                }
            }
            return out;
        }
        for (i, mn) in self.methods.values().enumerate() {
            if !mn.key.to_string().contains(pat) {
                continue;
            }
            out.push(self.method_label(i));
            let mut nodes: Vec<Node> = (0..mn.ptypes.len()).map(|j| Node::P(i, j as u16)).collect();
            nodes.push(Node::R(i));
            // 站点与该方法分配的数组元素
            let mut extra: Vec<Node> = self
                .sets
                .keys()
                .filter(|n| match **n {
                    Node::S(j, _) => j == i,
                    Node::E(x, _) => self.names[x as usize].contains(&format!("@{i}:")),
                    _ => false,
                })
                .copied()
                .collect();
            extra.sort_by_key(|n| format!("{n:?}"));
            nodes.extend(extra);
            for n in nodes {
                let Some(s) = self.sets.get(&n) else { continue };
                out.push(format!("  {} = {{{}}}", self.node_str(n), self.set_str(s)));
                for (src, edges) in &self.flows {
                    for (dst, f) in edges {
                        if *dst == n {
                            let s = self.sets.get(src).cloned().unwrap_or_default();
                            out.push(format!("    ← {} [{}] {{{}}}", self.node_str(*src), self.names[*f as usize], self.set_str(&s)));
                        }
                    }
                }
            }
        }
        out
    }

    pub fn method_label(&self, i: usize) -> String {
        self.methods[i].key.to_string()
    }
}
