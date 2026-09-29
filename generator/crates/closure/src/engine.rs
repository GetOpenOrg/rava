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

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::hash::{BuildHasherDefault, Hasher};
use std::rc::Rc;

use classfile::descriptor::{class_refs, parse_field, parse_method, FieldType};
use classfile::{acc, ClassFile, Const, MemberRef, MethodHandle};
use indexmap::IndexMap;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::absint::{self, Analysis, Event, Oracle, Ret, Src, V};
use crate::handwritten::{member_matches, FieldAccess, Handwritten, MemberHw, SType, TypeRef, TypedCall, Upcall};
use crate::manifest::{Domain, Fact, IndyKind, Manifest};

/// 整数键为主的内部表用的快速哈希（FxHash 乘法混合；遍历顺序不参与任何输出）
#[derive(Default, Clone, Copy)]
pub struct FxHasher(u64);

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.add(u64::from(b));
        }
    }
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

impl FxHasher {
    #[inline]
    fn add(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type HashSet<K> = std::collections::HashSet<K, BuildHasherDefault<FxHasher>>;

const OBJECT: &str = "java/lang/Object";
const STRING: &str = "java/lang/String";
const CLASS: &str = "java/lang/Class";
const THROWABLE: &str = "java/lang/Throwable";
const TO_STRING: (&str, &str) = ("toString", "()Ljava/lang/String;");
/// 站点键：异常处理器入口
const CATCH: u32 = 1 << 31;
/// 站点键：手写方法的值池
const POOL: u32 = u32::MAX;
/// 站点键：手写体产出的值（分配 / 构造 / 字段读取 / 回调返回值），汇入值池
const PROD: u32 = u32::MAX - 1;
/// 数组元素节点的下标奇偶槽
const PARITIES: [u8; 2] = [0, 1];
/// 方法克隆的上下文：无（按声明类型 / open 接收者进入的方法本体）
const NOCTX: u32 = u32::MAX;
/// 容器对象的堆上下文深度（分配点链长度，含自身）：`KeyIterator` ← `KeySet` ← `HashMap` 需要两层才能按 map 分开
const HEAP_DEPTH: usize = 2;

/// 字段泛型签名是否引用类型变量（`T<名>;`）：类名标识符内的字母不算
fn has_type_var(sig: &str) -> bool {
    let b = sig.as_bytes();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'T' => return true,
            b'L' | b'.' => {
                i += 1;
                while i < b.len() && !matches!(b[i], b'<' | b';' | b'.') {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    false
}

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
    pub classes: IdSet,
    pub open: IdSet,
}

/// 有序去重的 id 小集合（类型流集合绝大多数只有几个到几十个元素，有序 Vec 比 B 树快）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IdSet(Vec<u32>);

impl IdSet {
    pub fn insert(&mut self, x: u32) -> bool {
        match self.0.binary_search(&x) {
            Ok(_) => false,
            Err(i) => {
                self.0.insert(i, x);
                true
            }
        }
    }
    pub fn contains(&self, x: &u32) -> bool {
        self.0.binary_search(x).is_ok()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }
    /// self \ o
    fn minus(&self, o: &IdSet) -> IdSet {
        if o.is_empty() {
            return self.clone();
        }
        IdSet(self.0.iter().copied().filter(|x| !o.contains(x)).collect())
    }
    /// 并入 o（有序归并）
    fn union_with(&mut self, o: &IdSet) {
        if o.is_empty() {
            return;
        }
        if self.is_empty() {
            self.0 = o.0.clone();
            return;
        }
        let (a, b) = (&self.0, &o.0);
        let mut out = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            match a[i].cmp(&b[j]) {
                std::cmp::Ordering::Less => {
                    out.push(a[i]);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    out.push(b[j]);
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    out.push(a[i]);
                    i += 1;
                    j += 1;
                }
            }
        }
        out.extend_from_slice(&a[i..]);
        out.extend_from_slice(&b[j..]);
        self.0 = out;
    }
}

impl<'a> IntoIterator for &'a IdSet {
    type Item = &'a u32;
    type IntoIter = std::slice::Iter<'a, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for IdSet {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<u32> for IdSet {
    fn from_iter<I: IntoIterator<Item = u32>>(it: I) -> Self {
        let mut v: Vec<u32> = it.into_iter().collect();
        v.sort_unstable();
        v.dedup();
        IdSet(v)
    }
}

impl Extend<u32> for IdSet {
    fn extend<I: IntoIterator<Item = u32>>(&mut self, it: I) {
        let o: IdSet = it.into_iter().collect();
        self.union_with(&o);
    }
}

impl std::convert::From<[u32; 1]> for IdSet {
    fn from(a: [u32; 1]) -> Self {
        IdSet(a.to_vec())
    }
}

impl TypeSet {
    fn exact(id: u32) -> TypeSet {
        TypeSet { classes: [id].into(), open: IdSet::default() }
    }
    fn open(id: u32) -> TypeSet {
        TypeSet { classes: IdSet::default(), open: [id].into() }
    }
    fn add_all(&mut self, o: &TypeSet) -> bool {
        let n = self.classes.len() + self.open.len();
        self.classes.union_with(&o.classes);
        self.open.union_with(&o.open);
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
    /// 克隆上下文：接收者抽象对象（容器对象敏感），NOCTX = 方法本体
    pub ctx: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Node {
    /// 形参（方法, 序号）
    P(usize, u16),
    /// 返回值
    R(usize),
    /// 方法内站点（方法, 偏移 | CATCH | POOL）
    S(usize, u32),
    /// 字段（全部写入的并；接收者未知 / 非抽象对象的读取取这里）
    F(usize),
    /// 接收者未知的字段写入（open 接收者、非抽象对象、手写层）：流入 `F` 与每个抽象对象的该字段
    U(usize),
    /// 抽象对象（容器分配点）的字段
    O(u32, usize),
    /// 数组分配点的元素（分配点, 下标奇偶 0 偶 / 1 奇）：键值交错数组的键、值分开
    E(u32, u8),
    /// 写入未知数组（open / 保守分析）的元素：流入每个数组分配点
    Array,
    /// 手写方法调用点（`hw_sites` 序号）的第 i 个实参（含接收者）
    A(u32, u16),
    /// 手写方法调用点写入第 j 个实参数组的元素来源
    W(u32, u16),
}

/// 值的类型来源：节点，或直接给定的类型集（字面量 / 未知值的 open）
#[derive(Clone, Debug)]
enum Feed {
    N(Node),
    S(TypeSet),
}

/// 手写方法写入某个形参数组的值来源（形参序号含接收者）
#[derive(Clone, Debug)]
struct HwWrite {
    /// 这些形参的值本身
    values: Vec<usize>,
    /// 这些形参里数组的元素
    elements: Vec<usize>,
    /// 手写体产出（分配 / 字段读取 / 回调返回值）
    produced: bool,
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
    /// 创建点（方法, 偏移）与创建时的克隆上下文：静态实现方法继承之，构造器引用在创建点分配
    site: (usize, u32),
    ctx: u32,
    iface: String,
    sam: String,
    imh: MethodHandle,
    /// 捕获实参来源（按 indy 描述符形参位置）
    cap: Args,
}

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

// ── 常量 / 事实查询（absint 的 Oracle）──────────────────────────────────────

/// 常量格上的值：缺席（⊥，尚无值）→ 单一常量 → Top
#[derive(Clone, Debug, PartialEq)]
enum PV {
    Const(V),
    Top,
}

impl PV {
    /// 抽象值 → 常量格（只折叠 int / long / null / 字符串常量）
    fn of(v: &V) -> PV {
        match v {
            V::Int(_) | V::Long(_) | V::Null | V::Str(_) => PV::Const(v.clone()),
            _ => PV::Top,
        }
    }
    fn join(a: Option<&PV>, b: &PV) -> PV {
        match (a, b) {
            (None, x) => x.clone(),
            (Some(PV::Const(x)), PV::Const(y)) if x == y => PV::Const(x.clone()),
            _ => PV::Top,
        }
    }
    fn value(&self) -> Option<V> {
        match self {
            PV::Const(v) => Some(v.clone()),
            PV::Top => None,
        }
    }
}

/// 字段初值（默认值）；float / double 不折叠
fn default_pv(desc: &str) -> PV {
    match desc.as_bytes().first() {
        Some(b'B' | b'C' | b'I' | b'S' | b'Z') => PV::Const(V::Int(0)),
        Some(b'J') => PV::Const(V::Long(0)),
        Some(b'L' | b'[') => PV::Const(V::Null),
        _ => PV::Top,
    }
}

struct Ctx<'a> {
    h: &'a Hierarchy<'a>,
    cp: &'a ClassPath,
    man: &'a Manifest,
    hw: &'a Handwritten,
    /// static final 字段常量缓存（None = 非常量）
    consts: RefCell<HashMap<MemberRef, Option<V>>>,
    in_progress: RefCell<HashSet<String>>,
    /// 非 static final 字段的值集（初值 ∪ 可达写入；缺席 = 只有初值）
    fvals: RefCell<HashMap<MemberRef, PV>>,
    /// 字节码方法的返回常量（缺席 = 尚无返回路径）
    rvals: RefCell<HashMap<MemberRef, PV>>,
    /// 不折叠的字段：手写写入、反射按名写入
    fopen: RefCell<HashSet<MemberRef>>,
    /// 不折叠的字段名（手写写入 / 反射写入推不出所属类）
    fopen_names: RefCell<HashSet<String>>,
    /// 反射枚举式写入推不出所属类：全部字段不折叠
    fopen_all: Cell<bool>,
    /// 反序列化可达：非 static、非 transient 字段不折叠
    deser: Cell<bool>,
    /// 字段 → 读取过它的方法（值集变化时失效重算）
    fdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 被调方法 → 查询过其返回常量的方法
    rdeps: RefCell<HashMap<MemberRef, BTreeSet<usize>>>,
    /// 乐观阶段：被调方法尚无返回路径时，调用之后按不可达处理
    optimistic: Cell<bool>,
    /// 乐观阶段得到过「尚无返回」答复的方法（收尾时按值未知重算）
    never: RefCell<BTreeSet<usize>>,
}

struct Facts<'c, 'a> {
    ctx: &'c Ctx<'a>,
    live: &'c dyn Fn(&str) -> bool,
    /// 被分析的方法（None = 静态常量求值用的 `<clinit>` 分析：只用清单事实与 static final 常量）
    m: Option<usize>,
    /// 形参常量（按形参槽序号）
    params: Vec<Option<V>>,
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
    fn kind_of(&self, cf: &ClassFile, m: &classfile::Method) -> Kind {
        let member = format!("{}.{}:{}", cf.name, m.name, m.desc);
        match self.domain(&cf.name) {
            // 内部包边界：BFS 截断，整体手写（未手写的成员是 panic 存根，运行时不执行字节码）。
            // VM 耦合边界（公开包，`[vm_boundary]`）按方法划分：手写承载（native / VM 内建 /
            // 共置手写体按精确名提供）的取手写效果，其余被调用到的方法运行时执行的就是其字节码
            // （发射层同样翻译），按字节码建模——否则其体内的调用与写入（如经 native 手写体
            // 写入的字段）从分析中消失，成为漏报
            Domain::Boundary => {
                let hw = m.is_native()
                    || m.code.is_none()
                    || m.name == "<clinit>"
                    || self.man.is_intrinsic(&member)
                    || self.provided(cf, &m.name, &m.desc)
                    || !self.man.is_vm_boundary(&cf.name);
                return if hw { Kind::Handwritten("boundary") } else { Kind::Bytecode };
            }
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

    fn domain(&self, cls: &str) -> Domain {
        self.man.domain(cls, self.cp.origin(cls) == Some(Origin::User))
    }

    /// 字段的写入来源超出字节码（手写 / 边界类 / 反射 / 反序列化）：不折叠
    fn field_open(&self, key: &MemberRef, access: u16) -> bool {
        matches!(self.domain(&key.owner), Domain::Boundary | Domain::Root)
            || !self.hw.member(&key.owner, &key.name).fns.is_empty()
            || self.fopen_all.get()
            || self.fopen.borrow().contains(key)
            || self.fopen_names.borrow().contains(&key.name)
            || self.deser.get() && access & (acc::STATIC | acc::TRANSIENT) == 0
    }

    /// 字段读的常量值；方法 m 登记为该字段的读者
    fn field_value(&self, m: Option<usize>, f: &MemberRef) -> Option<V> {
        let site = self.h.resolve_field(&f.owner, &f.name, &f.desc)?;
        let fd = site.field();
        let key = MemberRef { owner: site.class.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
        if let Some(m) = m {
            self.fdeps.borrow_mut().entry(key.clone()).or_default().insert(m);
        }
        if self.field_open(&key, fd.access) {
            return None;
        }
        if fd.access & acc::STATIC != 0 && fd.access & acc::FINAL != 0 {
            return self.static_const(&key, fd.constant_value.as_ref());
        }
        m?;
        self.fvals.borrow().get(&key).cloned().unwrap_or_else(|| default_pv(&key.desc)).value()
    }

    /// 调用的唯一目标（静态 / 构造 / 私有 / final 方法 / final 类）
    fn exact_target(&self, opcode: u8, m: &MemberRef, iface: bool) -> Option<(Rc<ClassFile>, MemberRef)> {
        use classfile::op;
        let site = self.h.resolve_method(&m.owner, &m.name, &m.desc, iface)?;
        let rm = site.method();
        let exact = matches!(opcode, op::INVOKESTATIC | op::INVOKESPECIAL)
            || rm.is_private()
            || rm.is_static()
            || rm.is_final()
            || site.class.access & acc::FINAL != 0 && !site.class.is_interface();
        let (o, n, d) = site.key();
        exact.then(|| (site.class.clone(), MemberRef { owner: o, name: n, desc: d }))
    }

    /// static final 字段：ConstantValue，或 `<clinit>` 唯一一次常量赋值
    fn static_const(&self, key: &MemberRef, cv: Option<&Const>) -> Option<V> {
        if let Some(c) = cv {
            return const_value(c);
        }
        if let Some(v) = self.consts.borrow().get(key) {
            return v.clone();
        }
        // `<clinit>` 唯一一次常量赋值（递归保护：分析中的类不再展开）
        let cls = self.h.class(&key.owner)?;
        if !self.in_progress.borrow_mut().insert(cls.name.clone()) {
            return None;
        }
        let v = cls.method("<clinit>", "()V").and_then(|m| m.code.as_ref()).and_then(|code| {
            let live = |_: &str| true;
            let a = absint::analyze(&cls.name, "()V", true, code, &Facts { ctx: self, live: &live, m: None, params: vec![] });
            let puts: Vec<&Option<V>> = a
                .events
                .iter()
                .filter_map(|(_, e)| match e {
                    Event::Field { opcode: classfile::op::PUTSTATIC, mref, value, .. }
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
        self.consts.borrow_mut().insert(key.clone(), v.clone());
        v
    }
}

impl Oracle for Facts<'_, '_> {
    fn invoke_result(&self, opcode: u8, m: &MemberRef, iface: bool, args: &[V]) -> Ret {
        let k = m.to_string();
        if let Some(f) = self.ctx.man.return_fact(&k) {
            return Ret::Value(match f {
                Fact::Null => V::Null,
                Fact::Int(i) => V::Int(*i),
            });
        }
        if self.ctx.man.is_null_to_false(&k) && args.contains(&V::Null) {
            return Ret::Value(V::Int(0));
        }
        let Some(me) = self.m else { return Ret::Unknown };
        // 唯一目标且为字节码方法：取其返回常量（被调方法返回常量变化时本方法失效重算）
        let Some((cf, t)) = self.ctx.exact_target(opcode, m, iface) else { return Ret::Unknown };
        let Some(tm) = cf.method(&t.name, &t.desc) else { return Ret::Unknown };
        if self.ctx.kind_of(&cf, tm) != Kind::Bytecode {
            return Ret::Unknown;
        }
        let r = self.ctx.rvals.borrow().get(&t).cloned();
        self.ctx.rdeps.borrow_mut().entry(t).or_default().insert(me);
        match r {
            Some(PV::Const(v)) => Ret::Value(v),
            Some(PV::Top) => Ret::Unknown,
            None if self.ctx.optimistic.get() => {
                self.ctx.never.borrow_mut().insert(me);
                Ret::Never
            }
            None => Ret::Unknown,
        }
    }
    fn field(&self, _opcode: u8, f: &MemberRef) -> Option<V> {
        self.ctx.field_value(self.m, f)
    }
    fn param(&self, i: u16) -> Option<V> {
        self.params.get(i as usize).cloned().flatten()
    }
    fn catch_live(&self, ty: &str) -> bool {
        (self.live)(ty)
    }
}

// ── 折叠点（folds v1）────────────────────────────────────────────────────────

/// 一个方法的折叠点（计划 §7.3「折叠点导出」）
pub struct Fold {
    pub method: String,
    /// 不可达指令的半开区间 [start, end)，两端落在指令起点（或代码末尾），有序、不重叠、相邻合并
    pub dead_pcs: Vec<(u32, u32)>,
    /// 不进入的异常处理器（起点 pc）：try 区间全部不可达，或 catch 类型从不被实例化
    pub dead_handlers: Vec<u32>,
    /// (pc, 指令, 常量值, 类型描述符)
    pub consts: Vec<(u32, u8, V, String)>,
    /// 违反「活的非跳转指令落到死区」约定的 pc（应恒为空）
    pub violations: Vec<u32>,
}

fn fold_of(method: String, code: &classfile::Code, all: &[Rc<Analysis>]) -> Fold {
    use classfile::insn::Operand;
    use classfile::op;
    let insns = &code.insns;
    let reachable: Vec<bool> = (0..insns.len()).map(|i| all.iter().any(|a| a.reachable[i])).collect();
    let mut dead_pcs: Vec<(u32, u32)> = Vec::new();
    for (i, x) in insns.iter().enumerate() {
        if reachable[i] {
            continue;
        }
        let end = insns.get(i + 1).map_or(code.code_len, |n| n.offset);
        match dead_pcs.last_mut() {
            Some(last) if last.1 == x.offset => last.1 = end,
            _ => dead_pcs.push((x.offset, end)),
        }
    }
    let index: HashMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
    let live_at = |pc: u32| index.get(&pc).is_some_and(|&i| reachable[i]);
    let mut dead_handlers: Vec<u32> = code.exception_table.iter().map(|h| h.handler).filter(|&h| !live_at(h)).collect();
    dead_handlers.sort();
    dead_handlers.dedup();
    let mut violations = Vec::new();
    for (i, x) in insns.iter().enumerate() {
        // 跳转（含条件跳转 / ifnull / ifnonnull / goto_w / jsr）、switch、return、athrow 之后允许是死区
        let falls = !matches!(x.opcode, 0x99..=0xab | 0xc6..=0xc9 | op::ATHROW) && !(op::IRETURN..=op::RETURN).contains(&x.opcode);
        if reachable[i] && falls && insns.get(i + 1).is_some_and(|_| !reachable[i + 1]) {
            violations.push(x.offset);
        }
    }
    // 各克隆的常量点（同一 pc 取首个事件）
    let per: Vec<HashMap<u32, (u8, &V)>> = all
        .iter()
        .map(|a| {
            let mut m = HashMap::default();
            for (pc, e) in &a.events {
                if let Event::Const { opcode, value } = e {
                    m.entry(*pc).or_insert((*opcode, value));
                }
            }
            m
        })
        .collect();
    let mut consts: Vec<(u32, u8, V, String)> = Vec::new();
    for (pc, e) in all.iter().flat_map(|a| a.events.iter()) {
        let Event::Const { opcode, value } = e else { continue };
        let Some(&i) = index.get(pc) else { continue };
        let agree = all.iter().zip(&per).all(|(a, p)| !a.reachable[i] || p.get(pc) == Some(&(*opcode, value)));
        if !agree {
            continue;
        }
        let ins = &insns[i];
        let ty = match &ins.operand {
            Operand::Field(f) => f.desc.clone(),
            Operand::Method(m, _) => match m.desc.rfind(')') {
                Some(p) => m.desc[p + 1..].to_string(),
                None => continue,
            },
            _ => continue,
        };
        if consts.iter().any(|c| c.0 == *pc) {
            continue;
        }
        consts.push((*pc, *opcode, value.clone(), ty));
    }
    consts.sort_by_key(|c| c.0);
    Fold { method, dead_pcs, dead_handlers, consts, violations }
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
    /// 方法节点（成员, 克隆上下文）
    pub methods: IndexMap<(MemberRef, u32), MNode>,
    /// 成员 → 首个方法节点（输出按成员去重）
    mbase: HashMap<MemberRef, usize>,
    fields: IndexMap<MemberRef, ()>,
    sets: HashMap<Node, TypeSet>,

    /// G：全局已实例化（类型 id）
    g: BTreeSet<u32>,
    lambdas: HashMap<u32, Lambda>,
    /// 数组分配点（抽象对象 id）→ 数组类型 id
    arrays: HashMap<u32, u32>,
    /// 长度恒为 0 的数组分配点：任何元素读写都抛异常，元素节点不接收值；
    /// 值按元素节点暂存，分配点长度不再恒为 0 时补回
    empty_arrays: HashMap<u32, HashMap<Node, TypeSet>>,
    /// 容器抽象对象 id → 类型 id；分配点链（`@方法:偏移#…`，堆上下文）
    pub objs: HashMap<u32, u32>,
    obj_chain: HashMap<u32, Rc<str>>,
    /// 容器形态判定缓存（类型 id）
    containers: HashMap<u32, bool>,
    /// 新鲜工厂方法判定缓存（按成员）
    factories: HashMap<MemberRef, bool>,
    pub inited: IndexMap<String, Via>,

    flows: HashMap<Node, Vec<(Node, u32)>>,
    flow_seen: HashSet<(Node, Node, u32)>,
    /// 调用点分派结果：(方法, 偏移) → 目标方法
    pub dispatch: BTreeMap<(usize, u32), BTreeSet<usize>>,
    /// 形参常量（方法 → 按形参槽；缺席 = 尚无调用点）
    pvals: HashMap<usize, Vec<PV>>,
    /// 调用边的反向表（被调 → 调用方）：被调方法重算后调用方重处理（透传摘要可能变化）
    callers: HashMap<usize, BTreeSet<usize>>,
    /// 当前字节码调用点的实参值（不含接收者）；其余入口（手写 / 方法句柄 / lambda）为 None = 形参值未知
    call_vals: Option<Rc<[V]>>,
    pub unresolved: BTreeSet<String>,

    mwork: VecDeque<usize>,
    in_mwork: HashSet<usize>,
    /// 类型集读者：节点 → 读过它做分派 / 拆分决策的字节码站点（方法, 偏移）；节点增长只重跑这些站点
    watch: HashMap<Node, HashSet<(usize, u32)>>,
    swork: VecDeque<(usize, u32)>,
    in_swork: HashSet<(usize, u32)>,
    /// 正在处理的字节码站点（读类型集时登记为读者）
    cur_site: Option<(usize, u32)>,
    /// 字节码调用点已分派过的接收者（方法 → (偏移, 接收者)）；同一分析结果下分派是确定的，重跑站点只处理新增接收者
    dispatched: HashMap<usize, HashSet<(u32, u32)>>,
    /// 手写方法调用点（调用方, 偏移, 被调方法）→ 序号；数组写入按调用点建模
    hw_site_ids: HashMap<(usize, u32, usize), u32>,
    hw_sites: Vec<(usize, u32, usize)>,
    hw_writes: HashMap<usize, Rc<[Option<HwWrite>]>>,
    fwork: VecDeque<Node>,
    in_fwork: HashSet<Node>,
    /// 按 open 在 G 上展开过接收者的方法（G 增长时重处理）
    open_methods: BTreeSet<usize>,
    /// 按 open 在 G 上展开过接收者的字节码站点（G 增长时只重跑这些站点）
    open_sites: BTreeSet<(usize, u32)>,
    pending_catch: BTreeMap<usize, Vec<String>>,
    /// 进行中的 lambda 调用（lambda, 实参）：绑定方法引用的接收者可能是 lambda 自身，同一调用重入即成环
    lambda_stack: HashSet<(u32, String)>,
    /// 待沿流边推送的新增类型（差分传播）
    fdelta: HashMap<Node, TypeSet>,
    /// 手写层写入的字段（`__set_` 接收者类型已定位）
    pub hw_written: BTreeSet<MemberRef>,
    /// 手写层写入但接收者类型推不出的字段名：所有同名字段按有手写写入处理
    pub hw_written_names: BTreeSet<String>,
    /// 手写层读取但接收者类型推不出的字段名 → 读出值汇入的值池：所有同名字段流入
    hw_read_names: BTreeMap<String, BTreeSet<Node>>,
}

impl<'a> Engine<'a> {
    pub fn new(h: &'a Hierarchy<'a>, cp: &'a ClassPath, man: &'a Manifest, hw: &'a Handwritten) -> Self {
        Engine {
            ctx: Ctx {
                h,
                cp,
                man,
                hw,
                consts: Default::default(),
                in_progress: Default::default(),
                fvals: Default::default(),
                rvals: Default::default(),
                fopen: Default::default(),
                fopen_names: Default::default(),
                fopen_all: Cell::new(false),
                deser: Cell::new(false),
                fdeps: Default::default(),
                rdeps: Default::default(),
                optimistic: Cell::new(true),
                never: Default::default(),
            },
            h,
            cp,
            man,
            hw,
            names: Vec::new(),
            ids: HashMap::default(),
            sub_cache: HashMap::default(),
            classes: IndexMap::new(),
            missing: BTreeMap::new(),
            methods: IndexMap::new(),
            mbase: HashMap::default(),
            fields: IndexMap::new(),
            sets: HashMap::default(),
            g: BTreeSet::new(),
            lambdas: HashMap::default(),
            arrays: HashMap::default(),
            empty_arrays: HashMap::default(),
            objs: HashMap::default(),
            obj_chain: HashMap::default(),
            containers: HashMap::default(),
            factories: HashMap::default(),
            inited: IndexMap::new(),
            flows: HashMap::default(),
            flow_seen: HashSet::default(),
            dispatch: BTreeMap::new(),
            pvals: HashMap::default(),
            callers: HashMap::default(),
            call_vals: None,
            unresolved: BTreeSet::new(),
            mwork: VecDeque::new(),
            in_mwork: HashSet::default(),
            watch: HashMap::default(),
            swork: VecDeque::new(),
            in_swork: HashSet::default(),
            cur_site: None,
            dispatched: HashMap::default(),
            hw_site_ids: HashMap::default(),
            hw_sites: Vec::new(),
            hw_writes: HashMap::default(),
            fwork: VecDeque::new(),
            in_fwork: HashSet::default(),
            open_methods: BTreeSet::new(),
            open_sites: BTreeSet::new(),
            pending_catch: BTreeMap::new(),
            lambda_stack: HashSet::default(),
            hw_written: BTreeSet::new(),
            hw_written_names: BTreeSet::new(),
            hw_read_names: BTreeMap::new(),
            fdelta: HashMap::default(),
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
        let r = if let Some(&t) = self.arrays.get(&x).or_else(|| self.objs.get(&x)) {
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
            } else if self.sub(t, o) {
                out.open.insert(t);
            } else if !self.is_iface(o) && self.is_iface(t) {
                // 类 × 接口：交集是「o 的子类中实现 t 者」。保留 open(o)——展开时按接收者类型再求交；
                // final 类没有子类，不实现 t 即为空
                if !self.h.class(&self.names[o as usize]).is_some_and(|c| c.access & 0x0010 != 0) {
                    out.open.insert(o);
                }
            } else if self.is_iface(o) {
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
            match self.cur_site {
                Some(w) => {
                    self.open_sites.insert(w);
                }
                None => {
                    self.open_methods.insert(m);
                }
            }
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
        self.ctx.domain(cls)
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
    fn array_site(&mut self, m: usize, off: u32, t: &str, empty: bool, via: Via) -> u32 {
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

    /// 数组分配点出现非 0 长度：不再按空数组处理，补回暂存的元素值
    fn array_sized(&mut self, id: u32) {
        if let Some(held) = self.empty_arrays.remove(&id) {
            let mut v: Vec<(Node, TypeSet)> = held.into_iter().collect();
            v.sort_by_key(|(n, _)| format!("{n:?}"));
            for (n, s) in v {
                self.add_to(n, &s);
            }
        }
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

    /// 容器形态类（按字节码判定，不列类名）：翻译域的类（含超类）持有实例字段，其泛型签名引用类型变量、
    /// 或（泛型类链中）擦除为 `Object[]`，或声明类型本身是容器形态类（内部类的 `this$0`、`HashSet.map` 等）
    fn container(&mut self, cls: &str) -> bool {
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

    fn container_shape(&mut self, cls: &str) -> bool {
        let obj_arr = format!("[L{OBJECT};");
        let mut chain = Vec::new();
        let mut cur = self.h.class(cls);
        while let Some(cf) = cur {
            cur = cf.super_name.as_deref().and_then(|s| self.h.class(s));
            chain.push(cf);
        }
        let inst = |cf: &Rc<ClassFile>| cf.fields.iter().filter(|f| !f.is_static()).cloned().collect::<Vec<_>>();
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
    fn functional(&self, iface: &str) -> bool {
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
    fn obj_at(&mut self, m: usize, off: u32, cls: &str) -> u32 {
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
    fn site_ctx(&mut self, m: usize, off: u32) -> u32 {
        let chain = format!("@{}:{off}", self.mbase[&self.methods[m].key]);
        if let Some(&id) = self.ids.get(chain.as_str()) {
            return id;
        }
        let id = self.id(&chain);
        self.obj_chain.insert(id, Rc::from(chain));
        id
    }

    /// 新鲜工厂：有引用形参的静态字节码方法，返回值来自本方法分配的容器对象 / 引用数组，或来自另一个新鲜工厂
    fn fresh_factory(&mut self, key: &MemberRef) -> bool {
        if let Some(&r) = self.factories.get(key) {
            return r;
        }
        // 递归保护：成环部分取最小不动点
        self.factories.insert(key.clone(), false);
        let r = self.fresh_factory_uncached(key);
        self.factories.insert(key.clone(), r);
        r
    }

    fn fresh_factory_uncached(&mut self, key: &MemberRef) -> bool {
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
        let a = absint::analyze(&key.owner, &key.desc, true, code, &Facts { ctx: &self.ctx, live: &live, m: None, params: vec![] });
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

    /// 接收者对应的克隆上下文
    fn ctx_of(&self, r: u32) -> u32 {
        if self.objs.contains_key(&r) {
            r
        } else {
            NOCTX
        }
    }

    fn on_g_grow(&mut self, id: u32) {
        let open: Vec<usize> = self.open_methods.iter().copied().collect();
        for m in open {
            self.push_m(m);
        }
        let sites: Vec<(usize, u32)> = self.open_sites.iter().copied().collect();
        for w in sites {
            if self.in_swork.insert(w) {
                self.swork.push_back(w);
            }
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
        self.ctx.kind_of(cf, m)
    }

    /// (类内无重载时的裸名, mangle 名)
    fn rust_names(&self, cf: &ClassFile, name: &str, desc: &str) -> (Option<String>, String) {
        self.ctx.rust_names(cf, name, desc)
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

    /// 方法本体节点（无克隆上下文）
    fn method(&mut self, key: MemberRef, via: Via) -> usize {
        self.method_ctx(key, NOCTX, via)
    }

    /// 方法节点（按声明类 + 名字 + 描述符 + 克隆上下文）；首次登记入队。只有字节码方法按上下文克隆
    fn method_ctx(&mut self, key: MemberRef, ctx: u32, via: Via) -> usize {
        let k = (key, ctx);
        if let Some(i) = self.methods.get_index_of(&k) {
            return i;
        }
        let (key, ctx) = k;
        if ctx != NOCTX && self.mbase.get(&key).is_some_and(|&b| self.methods[b].kind != Kind::Bytecode) {
            return self.method_ctx(key, NOCTX, via);
        }
        let (kind, cf, is_static) = match self.h.class(&key.owner) {
            Some(cf) => match cf.method(&key.name, &key.desc) {
                Some(m) => (self.kind_of(&cf, m), Some(cf.clone()), m.is_static()),
                None => (Kind::Missing, None, false),
            },
            None => (Kind::Missing, None, false),
        };
        if ctx != NOCTX && kind != Kind::Bytecode {
            return self.method_ctx(key, NOCTX, via);
        }
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
            (key.clone(), ctx),
            MNode { key: key.clone(), kind, via: via.clone(), is_static, ptypes, rtype, analysis: None, hw_fns: vec![], ctx },
        );
        self.mbase.entry(key.clone()).or_insert(idx);
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
        if let Node::E(x, _) = n {
            if let Some(held) = self.empty_arrays.get_mut(&x) {
                held.entry(n).or_default().add_all(s);
                return;
            }
        }
        let cur = self.sets.entry(n).or_default();
        let delta = TypeSet {
            classes: s.classes.minus(&cur.classes),
            open: s.open.minus(&cur.open),
        };
        if delta.is_empty() {
            return;
        }
        cur.add_all(&delta);
        // 手写方法调用点的实参新增数组分配点：接上该数组的元素读写
        if let Node::A(s, i) = n {
            let ys: Vec<u32> = delta.classes.iter().copied().filter(|x| self.arrays.contains_key(x)).collect();
            if !ys.is_empty() {
                self.hw_site_arrays(s, i, &ys);
            }
        }
        if let Some(ws) = self.watch.get(&n) {
            for &w in ws {
                if self.in_swork.insert(w) {
                    self.swork.push_back(w);
                }
            }
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

    /// 来源的当前类型集；处理字节码站点时登记该站点为来源节点的读者
    fn value_set(&mut self, fs: &[Feed]) -> TypeSet {
        let mut out = TypeSet::default();
        for f in fs {
            match f {
                Feed::N(n) => {
                    if let Some(w) = self.cur_site {
                        self.watch.entry(*n).or_default().insert(w);
                    }
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

    /// 字段节点；首次登记时接上「未知接收者写入 → 字段并集」
    fn field_node(&mut self, key: MemberRef) -> usize {
        let (fi, _) = self.fields.insert_full(key.clone(), ());
        if let Some(tid) = parse_field(&key.desc).and_then(|t| self.ptype(&t)) {
            self.flow(Node::U(fi), Node::F(fi), tid);
        }
        fi
    }

    /// 抽象对象 o 的字段节点：收未知接收者写入，汇入字段并集
    fn obj_field(&mut self, o: u32, fi: usize, tid: u32) -> Node {
        let n = Node::O(o, fi);
        self.flow(Node::U(fi), n, tid);
        self.flow(n, Node::F(fi), tid);
        n
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
            // 先处理方法（图扩张），读者站点最后重跑：集合增长在两次重跑之间尽量合并
            let Some(m) = self.mwork.pop_front() else {
                if let Some((m, off)) = self.swork.pop_front() {
                    self.in_swork.remove(&(m, off));
                    self.rerun_site(m, off);
                    continue;
                }
                // 乐观阶段收敛：仍「尚无返回」的被调方法确实不返回。关掉乐观假设，把得到过该答复的
                // 方法按值未知重算——导出的不可达代码只从跳转 / switch / return / athrow 之后开始
                if !self.ctx.optimistic.replace(false) {
                    break;
                }
                let never: Vec<usize> = std::mem::take(&mut *self.ctx.never.borrow_mut()).into_iter().collect();
                for m in never {
                    self.invalidate(m);
                }
                continue;
            };
            self.in_mwork.remove(&m);
            self.process(m);
        }
    }

    /// 方法的分析结果失效：重分析，调用方重处理
    fn invalidate(&mut self, m: usize) {
        if self.methods[m].analysis.take().is_none() && self.in_mwork.contains(&m) {
            return;
        }
        self.push_m(m);
        // 被调方法重算：调用方按新摘要（透传等）重接调用边
        for c in self.callers.get(&m).cloned().unwrap_or_default() {
            self.dispatched.remove(&c);
            self.push_m(c);
        }
    }

    fn invalidate_all(&mut self, ms: Option<BTreeSet<usize>>) {
        for m in ms.unwrap_or_default() {
            self.invalidate(m);
        }
    }

    /// 字段写入值并入值集；变化时读者失效
    fn field_put(&mut self, key: &MemberRef, v: PV) {
        let cur = self.ctx.fvals.borrow().get(key).cloned().unwrap_or_else(|| default_pv(&key.desc));
        let new = PV::join(Some(&cur), &v);
        if new == cur {
            return;
        }
        self.ctx.fvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.fdeps.borrow().get(key).cloned();
        self.invalidate_all(deps);
    }

    /// 字段的写入来源超出字节码：不折叠，读者失效
    fn open_field(&mut self, key: MemberRef) {
        if self.ctx.fopen.borrow_mut().insert(key.clone()) {
            let deps = self.ctx.fdeps.borrow().get(&key).cloned();
            self.invalidate_all(deps);
        }
    }

    fn open_field_name(&mut self, name: &str) {
        if self.ctx.fopen_names.borrow_mut().insert(name.to_string()) {
            let deps: BTreeSet<usize> =
                self.ctx.fdeps.borrow().iter().filter(|(k, _)| k.name == name).flat_map(|(_, v)| v.iter().copied()).collect();
            self.invalidate_all(Some(deps));
        }
    }

    /// 全局开关（反射枚举 / 反序列化）打开：所有读过字段的方法失效
    fn open_fields_all(&mut self) {
        let deps: BTreeSet<usize> = self.ctx.fdeps.borrow().values().flat_map(|v| v.iter().copied()).collect();
        self.invalidate_all(Some(deps));
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
        let mut live_cache: HashMap<String, bool> = HashMap::default();
        for h in &code.exception_table {
            if let Some(ct) = &h.catch_type {
                let tid = self.id(ct);
                let live = g.iter().any(|&x| self.sub(x, tid));
                live_cache.insert(ct.clone(), live);
            }
        }
        let live = |t: &str| live_cache.get(t).copied().unwrap_or(true);
        // 尚无调用点记录的入口（根 / 手写 / VM）：形参值未知，并固定为 Top 保证单调
        let n = self.methods[m].ptypes.len();
        let pv = self.pvals.entry(m).or_insert_with(|| vec![PV::Top; n]);
        let params: Vec<Option<V>> = pv.iter().map(PV::value).collect();
        let facts = Facts { ctx: &self.ctx, live: &live, m: Some(m), params };
        let a = Rc::new(absint::analyze(&key.owner, &key.desc, meth.is_static(), code, &facts));
        self.dispatched.remove(&m);
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
        self.returns(m, &a);
        for (off, e) in a.events.iter() {
            self.event(m, *off, e, &cf);
        }
    }

    /// 读者站点的类型集增长：只重跑该偏移处的事件（分析已失效时方法整体在队列里）
    fn rerun_site(&mut self, m: usize, off: u32) {
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
            self.event(m, *o, e, &cf);
        }
    }

    fn event(&mut self, m: usize, off: u32, e: &Event, cf: &Option<Rc<ClassFile>>) {
        let prev = self.cur_site.replace((m, off));
        self.event_inner(m, off, e, cf);
        self.cur_site = prev;
    }

    fn event_inner(&mut self, m: usize, off: u32, e: &Event, cf: &Option<Rc<ClassFile>>) {
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
                Event::InstanceOf(c) => {
                    self.touch(c, Level::Type, via("instanceof"));
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
                Event::Throw(_) | Event::Const { .. } => {}
            }
        }
    }

    /// 返回常量 = 各返回路径值的并；变化时查询过它的调用方失效
    fn returns(&mut self, m: usize, a: &Analysis) {
        let mut r: Option<PV> = None;
        for (_, e) in &a.events {
            if let Event::Return(v) = e {
                r = Some(PV::join(r.as_ref(), &PV::of(v)));
            }
        }
        let Some(r) = r else { return };
        let key = self.methods[m].key.clone();
        let cur = self.ctx.rvals.borrow().get(&key).cloned();
        let new = PV::join(cur.as_ref(), &r);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.ctx.rvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.rdeps.borrow().get(&key).cloned();
        self.invalidate_all(deps);
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

    /// 字段访问：写入值来源 → 字段节点；读 → 结果节点 `res`。实例字段按接收者拆分：
    /// 抽象对象（容器分配点）走各自的字段节点，其余接收者（open / 非抽象对象 / 未知）写 `U`、读 `F`
    #[allow(clippy::too_many_arguments)]
    fn field(&mut self, m: usize, off: u32, opcode: u8, f: &MemberRef, recv: Option<&V>, value: Option<&V>, res: Node) {
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
        if opcode == op::PUTSTATIC || opcode == op::PUTFIELD {
            let fd = site.field();
            // static final 由 `<clinit>` 常量求值（Ctx::static_const），其余字段并入值集
            if fd.access & acc::STATIC == 0 || fd.access & acc::FINAL == 0 {
                let key = MemberRef { owner: decl.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
                self.field_put(&key, value.map_or(PV::Top, PV::of));
            }
        }
        let Some(ft) = parse_field(&f.desc) else { return };
        let Some(tid) = self.ptype(&ft) else {
            // 手写字段访问器仍需沿其回调入链
            self.field_handwritten(m, &decl, &f.name, &via, None);
            return;
        };
        let key = MemberRef { owner: decl.clone(), name: f.name.clone(), desc: f.desc.clone() };
        let fi = self.field_node(key);
        if self.hw_written_names.contains(&f.name) {
            self.add_to(Node::U(fi), &TypeSet::open(tid));
        }
        for p in self.hw_read_names.get(&f.name).cloned().unwrap_or_default() {
            self.flow(Node::F(fi), p, tid);
        }
        let instance = opcode == op::GETFIELD || opcode == op::PUTFIELD;
        let (objs, other) = match recv.filter(|_| instance) {
            Some(v) => {
                let oid = self.id(&f.owner);
                let fs = self.feeds(m, v, oid);
                let s = self.value_set(&fs);
                let s = self.filter(&s, oid);
                let objs: Vec<u32> = s.classes.iter().copied().filter(|x| self.objs.contains_key(x)).collect();
                (objs.clone(), !s.open.is_empty() || s.classes.len() > objs.len())
            }
            None => (vec![], true),
        };
        let nodes: Vec<Node> = objs.iter().map(|&o| self.obj_field(o, fi, tid)).collect();
        if opcode == op::PUTSTATIC || opcode == op::PUTFIELD {
            let fs = match value {
                Some(v) => self.feeds(m, v, tid),
                None => vec![Feed::S(TypeSet::open(tid))],
            };
            for n in nodes {
                self.feed(&fs, n, tid);
            }
            if other {
                self.feed(&fs, Node::U(fi), tid);
            }
        } else {
            for n in nodes {
                self.flow(n, res, tid);
            }
            if other {
                self.flow(Node::F(fi), res, tid);
            }
            self.field_handwritten(m, &decl, &f.name, &via, Some((fi, tid)));
        }
    }

    /// 字段读：声明类是边界类（struct 与字段整体手写）或字段有手写访问器 → 按 open 处理（公开 API 类的
    /// 手写写入经 `__set_` 在 [`Self::hw_fields`] 精确接入）；手写访问器声明的回调入链
    fn field_handwritten(&mut self, m: usize, decl: &str, name: &str, via: &Via, node: Option<(usize, u32)>) {
        let boundary = matches!(self.domain(decl), Domain::Boundary | Domain::Root);
        let mh = self.hw.member(decl, name);
        if let Some((fi, tid)) = node {
            // 边界类字段，或值由手写访问器提供（如标准流 `System::out()`）：按 open 处理
            if boundary || !mh.fns.is_empty() {
                self.add_to(Node::U(fi), &TypeSet::open(tid));
            }
        }
        if mh.fns.is_empty() {
            return;
        }
        self.apply_hw(m, decl, &mh, via);
    }

    fn invoke(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
        self.reflective_writes(mref, opcode, args);
        let pargs = if opcode == classfile::op::INVOKESTATIC { args } else { args.get(1..).unwrap_or(&[]) };
        self.call_vals = Some(Rc::from(pargs));
        self.invoke_inner(m, off, opcode, mref, iface, args);
        self.call_vals = None;
    }

    /// 反射式字段写入（按字节码形状）：
    /// - 同一调用里有字符串常量，且形参含 Class 或接收者是 Class：点名字段不折叠
    ///   （所属类取 Class 常量实参 / 接收者，取不到时同名字段全部不折叠）；
    /// - 清单 `[facts.field_writes] enumerators`（返回字段句柄数组）：接收者类的全部字段不折叠，推不出时全部字段；
    /// - 清单 `deserializers` 可达：非 static、非 transient 字段全部不折叠
    fn reflective_writes(&mut self, mref: &MemberRef, opcode: u8, args: &[V]) {
        let class_param = parse_method(&mref.desc)
            .is_some_and(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(c) if c == CLASS)));
        let class_recv = opcode != classfile::op::INVOKESTATIC && mref.owner == CLASS;
        let classes: Vec<String> = args
            .iter()
            .filter_map(|a| match a {
                V::Class(c) => Some(c.to_string()),
                _ => None,
            })
            .collect();
        if class_param || class_recv {
            for a in args {
                let V::Str(name) = a else { continue };
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
        let k = mref.to_string();
        if self.man.is_field_enumerator(&k) {
            match args.first() {
                Some(V::Class(c)) => {
                    let mut cur = Some(c.to_string());
                    while let Some(cls) = cur {
                        let Some(cf) = self.h.class(&cls) else { break };
                        for f in &cf.fields {
                            self.open_field(MemberRef { owner: cls.clone(), name: f.name.clone(), desc: f.desc.clone() });
                        }
                        cur = cf.super_name.clone();
                    }
                }
                _ => {
                    if !self.ctx.fopen_all.replace(true) {
                        self.open_fields_all();
                    }
                }
            }
        }
        if self.man.is_deserializer(&k) && !self.ctx.deser.replace(true) {
            self.open_fields_all();
        }
    }

    fn invoke_inner(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) {
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
                let caller_ctx = self.methods[m].ctx;
                let ctx = match caller_ctx {
                    _ if !md.ret.as_ref().is_some_and(|r| r.is_reference()) => NOCTX,
                    NOCTX if self.fresh_factory(&resolved) => self.site_ctx(m, off),
                    c => c,
                };
                let t = self.method_ctx(resolved, ctx, via);
                self.edge(m, off, t, Recv::None, &a, ret, res);
            }
            op::INVOKESPECIAL => {
                let r = recv_feeds(self);
                self.edge_recv(m, off, resolved, via, r, &a, ret, res);
            }
            _ => {
                let rm = site.method();
                if rm.is_private() || rm.is_static() || rm.is_final() || site.class.access & acc::FINAL != 0 && !site.class.is_interface() {
                    // 非虚：直接到已解析方法，接收者值流入 this
                    let r = recv_feeds(self);
                    self.edge_recv(m, off, resolved, via, r, &a, ret, res);
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
        if self.methods[m].kind == Kind::Bytecode && !self.dispatched.entry(m).or_default().insert((off, r)) {
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
            let iface = l.iface.clone();
            if let Some(sel) = self.h.select(&iface, site) {
                let (o, n, d) = sel.key();
                let t = self.method(MemberRef { owner: o, name: n, desc: d }, via);
                self.edge(m, off, t, Recv::Exact(r), a, ret, res);
            }
            return;
        }
        let rt = self.arrays.get(&r).or_else(|| self.objs.get(&r)).copied().unwrap_or(r);
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
    fn edge_recv(&mut self, m: usize, off: u32, key: MemberRef, via: Via, fs: Vec<Feed>, a: &Args, ret: Option<u32>, res: Option<Node>) {
        // 接收者按被调方法声明类收窄（checkcast 不改变值来源，来源节点可能更宽）
        let owner = self.id(&key.owner);
        let s = self.value_set(&fs);
        let s = self.filter(&s, owner);
        let mut rest = TypeSet { classes: IdSet::default(), open: s.open.clone() };
        for &x in &s.classes {
            if self.objs.contains_key(&x) {
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
    fn edge(&mut self, m: usize, off: u32, t: usize, recv: Recv, a: &[Option<Vec<Feed>>], ret: Option<u32>, res: Option<Node>) {
        self.dispatch.entry((m, off)).or_default().insert(t);
        self.callers.entry(t).or_default().insert(m);
        let is_static = self.methods[t].is_static;
        let ptypes = self.methods[t].ptypes.clone();
        let base = usize::from(!is_static);
        self.bind_params(t, base, ptypes.len());
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
            if self.man.returns_receiver(&self.methods[t].key.to_string()) {
                // 浅拷贝：返回值 = 本调用点的接收者类型集（数组共享元素节点；逐调用点，不经被调方形参汇合）
                if let Some(fs) = &recv_fs {
                    self.feed(fs, res, rt);
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
    fn bind_params(&mut self, t: usize, base: usize, n: usize) {
        let vals = self.call_vals.clone();
        let cur = self.pvals.get(&t).cloned();
        let new: Vec<PV> = (0..n)
            .map(|i| {
                let v = match (&vals, i.checked_sub(base)) {
                    (Some(vs), Some(j)) => vs.get(j).map_or(PV::Top, PV::of),
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
            self.invalidate(t);
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
                // 静态实现方法继承 lambda 创建时的克隆上下文
                let t = self.method_ctx(resolved, l.ctx, via);
                self.edge(m, off, t, Recv::None, &all, ret, res);
            }
            7 => {
                let f = first();
                self.edge_recv(m, off, resolved, via, f, &rest, ret, res);
            }
            8 => {
                // 构造器引用：容器类在 lambda 创建点分配抽象对象
                let oid = if self.container(&k.owner) { self.obj_at(l.site.0, l.site.1, &k.owner) } else { self.id(&k.owner) };
                let t = self.method_ctx(resolved, self.ctx_of(oid), via);
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
                self.field(m, off, opc, k, None, None, Node::S(m, off));
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
                let ctx = self.methods[m].ctx;
                self.lambdas.insert(lid, Lambda { site: (m, off), ctx, iface: iface.clone(), sam: name.to_string(), imh: imh.clone(), cap });
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
                        let c = if imh.kind == 6 { self.methods[m].ctx } else { NOCTX };
                        self.method_ctx(MemberRef { owner: o, name: n, desc: d }, c, via);
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

    /// 手写方法的数组写入（arraycopy、Unsafe 引用写入等）按调用点建模：写入目标是本调用点实参里的数组，
    /// 写入值按 `hw_writes` 给出的来源（实参值 / 实参数组的元素 / 手写体产出）逐调用点接入。
    /// 不经被调方法的形参汇合：arraycopy 等被全程序共享，汇合会把所有数组的元素并成同一个集合
    fn hw_site(&mut self, m: usize, off: u32, t: usize, recv: Option<&[Feed]>, a: &[Option<Vec<Feed>>]) {
        let ws = self.hw_writes(t);
        if ws.iter().all(Option::is_none) || self.hw_site_ids.contains_key(&(m, off, t)) {
            return;
        }
        let s = self.hw_sites.len() as u32;
        self.hw_site_ids.insert((m, off, t), s);
        self.hw_sites.push((m, off, t));
        let base = usize::from(!self.methods[t].is_static);
        let ptypes = self.methods[t].ptypes.clone();
        let obj = self.id(OBJECT);
        let feeds: Vec<Option<&[Feed]>> = (0..ptypes.len()).map(|i| if i < base { recv } else { a.get(i - base).and_then(|f| f.as_deref()) }).collect();
        let mut watched: BTreeSet<usize> = BTreeSet::new();
        for (j, w) in ws.iter().enumerate() {
            let Some(w) = w else { continue };
            let wn = Node::W(s, j as u16);
            if w.produced {
                self.flow(Node::S(t, PROD), wn, obj);
            }
            for &i in &w.values {
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

    /// 手写方法按形参（含接收者序号）的数组写入来源。`[facts.array_writes]` 声明的按声明；
    /// 其余取得数组视图的手写体保守处理：每个非接收者引用形参都可被写入，来源为其它形参的值、
    /// 全部实参数组的元素与手写体产出。数组只有 Object 的方法，没有一个改写元素，接收者不是写入目标
    fn hw_writes(&mut self, t: usize) -> Rc<[Option<HwWrite>]> {
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
                        })
                    })
                    .collect()
            }
        };
        self.hw_writes.insert(t, w.clone());
        w
    }

    /// 调用点 s 的第 i 个实参新增数组 ys：元素来源含 i 的写入目标接上其元素；i 是写入目标则接收写入
    fn hw_site_arrays(&mut self, s: u32, i: u16, ys: &[u32]) {
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
            out.fields.extend(i.fields.iter().cloned());
            out.array_access |= i.array_access;
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

    /// 类（含超类 / 超接口）中按名字找字段 → (声明类, 描述符)
    fn field_by_name(&self, cls: &str, name: &str) -> Option<(String, String)> {
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
    fn stype_class(&self, host: &str, s: &SType) -> Option<String> {
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
            SType::Ret(t, m) => {
                let mut cur = self.resolve_tref(host, t);
                // 自类起沿超类找 Rust 名匹配的方法，返回类型须唯一
                while let Some(c) = cur {
                    let cf = self.h.class(&c)?;
                    let rets: BTreeSet<String> = cf
                        .methods
                        .iter()
                        .filter(|x| {
                            let (plain, mangled) = self.rust_names(&cf, &x.name, &x.desc);
                            plain.as_deref() == Some(m.as_str()) || mangled == *m
                        })
                        .filter_map(|x| parse_method(&x.desc).and_then(|d| d.ret).map(|r| r.descriptor()))
                        .collect();
                    if !rets.is_empty() {
                        return if rets.len() == 1 { of_desc(rets.first()?) } else { None };
                    }
                    cur = cf.super_name.clone();
                }
                None
            }
        }
    }

    /// 手写体字段访问器：写入值接进字段节点并登记「有手写写入」；读出值汇入值池。
    /// 接收者推不出 → 所有同名字段按 open 处理（安全回退）
    fn hw_fields(&mut self, m: usize, host: &str, fields: &[FieldAccess]) {
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
                        self.add_to(Node::U(i), &TypeSet::open(tid));
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

    /// 手写体效果：分配 / 构造 / 回调。实参按手写体调用点的语法推断精确接入，推断不出的经方法 m 的值池流转
    /// 手写体第 k 个分配点的值：容器类取抽象对象（伪偏移自 u32::MAX 递减，不与字节码偏移相撞），其余取类本身。
    /// 手写分配若只取类本身，其字段写入落到 U(f) 并流向该类全部对象，污染所有同类容器
    fn hw_obj(&mut self, m: usize, k: &mut u32, cls: &str) -> u32 {
        *k += 1;
        if self.container(cls) { self.obj_at(m, u32::MAX - *k, cls) } else { self.id(cls) }
    }

    fn apply_hw(&mut self, m: usize, host: &str, mh: &MemberHw, via: &Via) {
        let prod = Node::S(m, PROD);
        self.hw_fields(m, host, &mh.fields);
        let mut k = 0u32;
        for t in &mh.allocs {
            if let Some(c) = self.resolve_tref(host, t) {
                self.instantiate(&c, via.clone());
                self.init(&c, via.clone());
                let id = self.hw_obj(m, &mut k, &c);
                self.add_to(prod, &TypeSet::exact(id));
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
            let id = self.hw_obj(m, &mut k, &c);
            self.add_to(prod, &TypeSet::exact(id));
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
                            self.dispatch_one(m, 0, r, &site, &a, ret, Some(prod));
                        }
                    }
                }
            }
        }
    }

    // ── 结果查询 ────────────────────────────────────────────────────────────

    /// 折叠点导出（folds v1）：不可达指令区间、不进入的异常处理器、折叠为常量的读取点。
    /// 按方法标签排序；只含有折叠内容的字节码方法
    /// 同一成员的各克隆合并：任一克隆可达即可达，常量须在其可达的全部克隆里一致
    pub fn folds(&self) -> Vec<Fold> {
        let mut groups: IndexMap<&MemberRef, Vec<Option<Rc<Analysis>>>> = IndexMap::new();
        for mn in self.methods.values() {
            if mn.kind == Kind::Bytecode {
                groups.entry(&mn.key).or_default().push(mn.analysis.clone());
            }
        }
        let mut out = Vec::new();
        for (key, group) in groups {
            let Some(all) = group.into_iter().collect::<Option<Vec<_>>>() else { continue };
            if all.iter().any(|a| a.conservative) {
                continue;
            }
            let Some(cf) = self.h.class(&key.owner) else { continue };
            let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { continue };
            let f = fold_of(key.to_string(), code, &all);
            // 自检：活指令顺序落入 dead_pcs（v1 规则禁止），出现即分析缺陷
            if !f.violations.is_empty() {
                eprintln!("[closure] folds 自检违约：{} @{:?}", f.method, f.violations);
            }
            if !f.dead_pcs.is_empty() || !f.dead_handlers.is_empty() || !f.consts.is_empty() {
                out.push(f);
            }
        }
        out.sort_by(|a, b| a.method.cmp(&b.method));
        out
    }

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
            Node::P(m, i) => format!("P{i} {}", self.ctx_label(m)),
            Node::R(m) => format!("R {}", self.ctx_label(m)),
            Node::S(m, o) if o == POOL => format!("pool {}", self.ctx_label(m)),
            Node::S(m, o) if o == PROD => format!("prod {}", self.ctx_label(m)),
            Node::S(m, o) if o & CATCH != 0 => format!("catch@{} {}", o & !CATCH, self.ctx_label(m)),
            Node::S(m, o) => format!("@{o} {}", self.ctx_label(m)),
            Node::F(f) => format!("field {}", self.field_label(f)),
            Node::U(f) => format!("field? {}", self.field_label(f)),
            Node::O(o, f) => format!("field {} of {}", self.field_label(f), self.names[o as usize]),
            Node::E(x, p) => format!("elements[{}] {}", if p == 0 { "偶" } else { "奇" }, self.names[x as usize]),
            Node::Array => "array".into(),
            Node::A(s, i) | Node::W(s, i) => {
                let (m, off, t) = self.hw_sites[s as usize];
                let k = if matches!(n, Node::A(..)) { "实参" } else { "写入" };
                format!("{k}{i} {}@{off} → {}", self.ctx_label(m), self.methods[t].key)
            }
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
        // 污染路径诊断：`@path:<节点子串>|<类名>`——从匹配节点沿流边反向，经含该类的节点走到源头（最短路径）
        if let Some((np, cls)) = pat.strip_prefix("@path:").and_then(|v| v.split_once('|')) {
            let (open, cls) = match cls.strip_prefix("open:") {
                Some(c) => (true, c),
                None => (false, cls),
            };
            let Some(&cid) = self.ids.get(cls) else { return vec![format!("无此类：{cls}")] };
            let has = |x: &Node| self.sets.get(x).is_some_and(|s| if open { s.open.contains(&cid) } else { s.classes.contains(&cid) });
            let mut rev: HashMap<Node, Vec<Node>> = HashMap::default();
            for (src, edges) in &self.flows {
                for (dst, _) in edges {
                    rev.entry(*dst).or_default().push(*src);
                }
            }
            let starts: Vec<Node> = self.sets.keys().filter(|n| has(n) && self.node_str(**n).contains(np)).copied().collect();
            let mut prev: HashMap<Node, Option<Node>> = HashMap::default();
            let mut q: VecDeque<Node> = VecDeque::new();
            for n in starts.into_iter().take(1) {
                prev.insert(n, None);
                q.push_back(n);
            }
            let mut last = None;
            while let Some(n) = q.pop_front() {
                last = Some(n);
                let mut ps: Vec<Node> = rev.get(&n).map(|v| v.iter().filter(|p| has(p) && !prev.contains_key(*p)).copied().collect()).unwrap_or_default();
                ps.sort_by_key(|p| format!("{p:?}"));
                for p in ps {
                    prev.insert(p, Some(n));
                    q.push_back(p);
                }
            }
            // 最远的源头往回打印到起点
            let mut cur = last;
            while let Some(n) = cur {
                let sz = self.sets.get(&n).map_or(0, |s| s.classes.len());
                out.push(format!("  {} (|{sz}|)", self.node_str(n)));
                cur = prev.get(&n).copied().flatten();
            }
            return out;
        }
        // open 统计诊断：`@openstat`——各 open 类型：含它的节点数、引入点数、按 G 展开的类数
        if pat == "@openstat" {
            let mut fed: HashSet<(Node, u32)> = HashSet::default();
            for (src, edges) in &self.flows {
                if let Some(ss) = self.sets.get(src) {
                    for &o in &ss.open {
                        for (dst, _) in edges {
                            fed.insert((*dst, o));
                        }
                    }
                }
            }
            let mut stat: HashMap<u32, (usize, Vec<String>)> = HashMap::default();
            for (n, ss) in &self.sets {
                for &o in &ss.open {
                    let e = stat.entry(o).or_default();
                    e.0 += 1;
                    if !fed.contains(&(*n, o)) {
                        e.1.push(self.node_str(*n));
                    }
                }
            }
            let g: Vec<u32> = self.g.iter().copied().collect();
            let mut v: Vec<(usize, String)> = Vec::new();
            for (o, (cnt, intro)) in stat {
                let on = &self.names[o as usize];
                let exp = g.iter().filter(|&&x| &*self.names[x as usize] == &**on || self.h.is_subtype(&self.names[x as usize], on)).count();
                let mut intro = intro;
                intro.sort();
                let head: Vec<String> = intro.iter().take(4).map(|x| x.chars().take(140).collect()).collect();
                v.push((exp * cnt, format!("  {} 节点 {cnt} 展开 {exp} 引入 {}：{}", self.names[o as usize], intro.len(), head.join(" ｜ "))));
            }
            v.sort_by(|a, b| b.0.cmp(&a.0));
            return v.into_iter().take(40).map(|x| x.1).collect();
        }
        // open 源头诊断：`@opens:<类型>`——含 open(类型)、但没有任何含同一 open 的前驱的节点（open 的引入点）
        if let Some(q) = pat.strip_prefix("@opens:") {
            let Some(&cid) = self.ids.get(q) else { return vec![format!("无此类：{q}")] };
            let has = |x: &Node| self.sets.get(x).is_some_and(|s| s.open.contains(&cid));
            let mut fed: HashSet<Node> = HashSet::default();
            for (src, edges) in &self.flows {
                if has(src) {
                    for (dst, _) in edges {
                        fed.insert(*dst);
                    }
                }
            }
            let mut v: Vec<String> = self.sets.keys().filter(|n| has(n) && !fed.contains(*n)).map(|n| format!("  {}", self.node_str(*n))).collect();
            v.sort();
            return v;
        }
        // 方法节点序号诊断：`@m:<序号>`（数组分配点名里的方法序号）
        if let Some(i) = pat.strip_prefix("@m:").and_then(|v| v.parse::<usize>().ok()) {
            return vec![format!("  {i} = {}", self.ctx_label(i))];
        }
        // 调用方诊断：`@callers:<方法子串>`——列出 NOCTX 方法本体的全部调用点
        if let Some(q) = pat.strip_prefix("@callers:") {
            let mut v: Vec<String> = Vec::new();
            for (&(m, off), ts) in &self.dispatch {
                for &t in ts.iter() {
                    if self.methods[t].ctx == NOCTX && self.method_label(t).contains(q) {
                        v.push(format!("  {} ← {} @{off}", self.method_label(t), self.ctx_label(m)));
                    }
                }
            }
            v.sort();
            return v;
        }
        // 汇合点诊断：值集 ≥ N 的节点中，由小值集（< N）来源直接汇入的类最多者（污染的起始汇点）
        if let Some(n) = pat.strip_prefix("@merge:").and_then(|v| v.parse::<usize>().ok()) {
            let size = |x: &Node| self.sets.get(x).map_or(0, |s| s.classes.len());
            let mut inc: HashMap<Node, (IdSet, usize)> = HashMap::default();
            for (src, edges) in &self.flows {
                if size(src) >= n {
                    continue;
                }
                for (dst, _) in edges {
                    if size(dst) >= n {
                        let e = inc.entry(*dst).or_default();
                        e.1 += 1;
                        if let Some(s) = self.sets.get(src) {
                            for c in s.classes.iter() {
                                e.0.insert(*c);
                            }
                        }
                    }
                }
            }
            let mut v: Vec<_> = inc.into_iter().collect();
            v.sort_by_key(|(k, (c, _))| (std::cmp::Reverse(c.len()), format!("{k:?}")));
            for (k, (c, k2)) in v.into_iter().take(40) {
                out.push(format!("  {} 类 / {} 来源 → {} (|{}|)", c.len(), k2, self.node_str(k), size(&k)));
            }
            return out;
        }
        if pat == "@array" {
            let s = self.sets.get(&Node::Array).cloned().unwrap_or_default();
            out.push(format!("  array = {{{}}}", self.set_str(&s)));
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
            out.push(self.ctx_label(i));
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

    /// 带克隆上下文的方法标签（诊断用）
    fn ctx_label(&self, i: usize) -> String {
        match self.methods[i].ctx {
            NOCTX => self.method_label(i),
            c => format!("{} #{}", self.method_label(i), self.names[c as usize]),
        }
    }

    fn field_label(&self, f: usize) -> String {
        self.fields.get_index(f).map(|(k, _)| k.to_string()).unwrap_or_default()
    }

    /// 方法节点按成员去重（克隆只是分析内部的上下文区分；输出按成员）
    pub fn method_nodes(&self) -> impl Iterator<Item = &MNode> {
        self.methods.values().enumerate().filter(|(i, m)| self.mbase[&m.key] == *i).map(|(_, m)| m)
    }

    pub fn method_count(&self) -> usize {
        self.mbase.len()
    }

    /// 调用点分派（按成员合并克隆）：调用方法标签@偏移 → 目标方法标签
    pub fn dispatch_sites(&self) -> BTreeMap<(String, u32), BTreeSet<String>> {
        let mut out: BTreeMap<(String, u32), BTreeSet<String>> = BTreeMap::new();
        for ((m, off), ts) in &self.dispatch {
            out.entry((self.method_label(*m), *off)).or_default().extend(ts.iter().map(|t| self.method_label(*t)));
        }
        out
    }
}
