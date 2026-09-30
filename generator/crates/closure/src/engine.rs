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

use classfile::descriptor::{class_refs, parse_field, parse_method, FieldType, MethodDesc};
use classfile::{acc, ClassFile, Const, MemberRef, MethodHandle};
use indexmap::IndexMap;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::absint::{self, Analysis, Event, Obj, Oracle, Ret, Src, V};
use crate::handwritten::{member_matches, to_snake, MODULE_SUFFIXES, FieldAccess, Handwritten, MemberHw, SType, TypeRef, TypedCall, Upcall};
use crate::manifest::{Domain, Fact, IndyKind, Manifest, Members, PropValue};

mod sets;
mod idset;
mod facts;
mod consteval;
mod construct;
mod sysprops;
mod fold;
mod forward;
mod classes;
mod reflect;
mod flow;
mod bytecode;
mod invoke;
mod hub;
mod lambda;
mod lambda_adapt;
mod hw;
mod hw_mem;
mod hw_syntax;
mod hw_infer;
mod hwobj;
mod report;
mod diag;
mod seeds;
mod services;
mod class_init;
mod noreturn;
mod class_lookup;
mod method_lookup;
mod new;
mod methods;
mod worklist;
mod stats;
mod graph;
mod scc;

use graph::FlowGraph;
use stats::{Phase, Why};
pub use stats::peak_rss_mb;

pub use seeds::SeedState;

pub use fold::{DeadCatch, Fold};
use facts::*;
use sysprops::{PropSum, PropUnstable};
use fold::*;
pub use sets::*;
pub use idset::{IdIter, IdSet};
use hwobj::{HwObj, HWOBJ_KIND};


/// 精确接收者达到此数时经集合枢纽派发
const HUB_MIN: usize = 8;
/// 字段站点在 `recv_done` 中的哨兵：与值无关的部分已接 / 未知接收者视图已接（抽象对象 id 不会取到）
const FIELD_STATIC: u32 = u32::MAX;
const FIELD_OTHER: u32 = u32::MAX - 1;

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
/// 站点键：清单声明元素类型的手写返回数组（`[facts.array_returns]`）的分配点
const ARRAY_RET: u32 = u32::MAX - 2;
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
    /// 事件已全部执行过的分析（None = 须完整执行）：重分析后只执行与之不同的事件
    applied: Option<Rc<Analysis>>,
    /// 最近一次分析的透传摘要（None = 尚未分析）
    returned: Option<Option<Vec<u16>>>,
    /// 手写体命中的 fn 名（溯源）
    pub hw_fns: Vec<String>,
    /// 克隆上下文：接收者抽象对象（容器对象敏感），NOCTX = 方法本体
    pub ctx: u32,
    /// 清单声明的返回值模型
    ret_model: RetModel,
}

/// 返回值按调用点建模的清单声明（`vm_intrinsics.toml`）
#[derive(Clone, Copy, PartialEq, Eq)]
enum RetModel {
    /// 结果取返回值节点
    Plain,
    /// 类镜像：本调用点接收者各值的 Class 对象
    Mirror,
    /// 浅拷贝：本调用点的接收者
    Receiver,
    /// 按实参（序号，不含接收者）读内存
    Read(usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Node {
    /// 形参（方法, 序号）
    P(usize, u16),
    /// 返回值
    R(usize),
    /// 方法内站点（方法, 偏移 | CATCH | POOL）
    S(usize, u32),
    /// 字段的未知接收者视图（`U` ∪ 已逃逸抽象对象的该字段；接收者未知 / 非抽象对象的读取取这里）
    F(usize),
    /// 接收者未知的字段写入（open 接收者、非抽象对象、手写层）：流入 `F` 与每个已逃逸抽象对象的该字段
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
    /// 开放接收者派发枢纽（枢纽序号, 实参序号，不含接收者）：各调用点的实参汇入，再流向各目标形参
    HP(u32, u16),
    /// 开放接收者派发枢纽的返回值：各目标的返回值汇入，再流向各调用点的结果
    HR(u32),
    /// 逃逸汇点：流入非建模代码（手写体 / native 的值池、VM 回调的返回值、未知数组）的值。
    /// 抽象对象到达这里即「已逃逸」——只有它们可能以 open / 非抽象接收者的身份被读写
    Esc,
}

/// 值的类型来源：节点，或直接给定的类型集（字面量 / 未知值的 open）
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    /// 目标为对象时写入其引用实例字段
    fields: bool,
    /// 写入值取自调用点最后一个实参（签名多态）
    last: bool,
}

/// 按声明形参位置的实参来源（基本类型为 None）
type Args = Vec<Option<Vec<Feed>>>;

/// 一次 lambda 调用：(lambda, 实参, 返回类型, 结果节点)
type LambdaCall = (u32, Args, Option<u32>, Option<Node>);

/// 字节码调用点（方法, 偏移）上的一次 lambda 调用（读者单元，见 [`Engine::invoke_lambda`]）
struct LCall {
    m: usize,
    off: u32,
    call: LambdaCall,
    /// 已接过的接收值
    done: TypeSet,
    /// 调用方分析重算后作废（调用点重跑时按新分析重新登记）
    live: bool,
}

/// 被调方法的接收者
enum Recv {
    None,
    Exact(u32),
    Feeds(Vec<Feed>),
}

/// 派发枢纽：同一调用成员在同一接收者集合上的派发。
///
/// 虚调用的派发目标只由接收者决定（抽象对象取其克隆上下文、类与数组取方法本体），与调用方上下文无关；
/// 逐调用点派发时边数是「调用点 × 接收者」，被调方重算还让全部调用方重新派发。枢纽把调用点实参汇入 `HP`、
/// 各目标返回值汇入 `HR`，边数降为「调用点 + 接收者」。两种接收者集合：
/// - open(类型)：G 中该类型的成员（已逃逸的任意实例），G 增长 / 数组逃逸时在枢纽上增量展开；
/// - 精确集合：按内容共享（不同上下文的同一调用点常持有相同集合）。调用点的集合增长时换接新集合的枢纽，
///   新枢纽以该调用点原枢纽（旧集合 ⊂ 新集合）为父，只派发增量；原接入保留（目标是新枢纽的子集，结果不变）。
///
/// 目标的形参 / 返回值节点本就按全部调用方汇合，经枢纽中转的结果与逐调用点派发相同；
/// 按调用点建模的目标（手写 / 清单特判返回 / 透传 / lambda）仍逐调用点派发
struct Hub {
    site: resolve::MethodSite,
    owner: u32,
    /// open 类型（open 枢纽）
    open: Option<u32>,
    parent: Option<u32>,
    /// 实参声明类型（不含接收者）与返回类型
    ptypes: Vec<Option<u32>>,
    ret: Option<u32>,
    /// 各调用点实参常量的汇合（None = 尚无调用点；首个调用点接入后才展开接收者）
    vals: Option<Vec<PV>>,
    via: Via,
    /// 待展开（精确集合）/ 已展开的接收者
    expanded: bool,
    pending: Vec<u32>,
    recvs: BTreeSet<u32>,
    /// 经枢纽中转的目标
    plain: BTreeSet<usize>,
    /// 逐调用点派发的 lambda / 手写实现对象接收者（含父枢纽的）
    lambdas: Vec<u32>,
    /// 按调用点建模的目标 → 其接收者（含父枢纽的）：逐调用点接边，同目标的接收者合成一条
    special: BTreeMap<usize, Vec<u32>>,
    /// 调用点（方法, 偏移）→ 实参来源、结果节点、实参值
    links: BTreeMap<(usize, u32), (Args, Option<Node>, Option<Rc<[V]>>)>,
}

/// 枢纽的接收者集合键
#[derive(Clone, PartialEq, Eq, Hash)]
enum HubSet {
    Open(u32),
    Exact(Vec<u32>),
    /// VM 按反射对象虚调用（`Method.invoke` / REF_invokeVirtual 的 MemberName）：接收者 open(类型)，
    /// 无字节码调用点；展开到的每个目标形参 open
    Vm(u32),
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
    /// SAM 与实现方法签名差异处的装箱 / 拆箱适配
    adapt: Vec<lambda_adapt::Conv>,
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
    /// open(o) 按过滤类型 t 收窄的结果缓存（`u32::MAX` = 空）
    narrow_cache: HashMap<(u32, u32), u32>,
    /// 按过滤类型 f 的子类型判定行（下标为类型 id；0 未判定、1 否、2 是）：类型集收窄逐元素只做一次数组索引
    sub_rows: Vec<Vec<u8>>,

    pub classes: IndexMap<String, ClassNode>,
    pub missing: BTreeMap<String, Via>,
    /// 方法节点（成员, 克隆上下文）
    pub methods: IndexMap<(MemberRef, u32), MNode>,
    /// 成员 → 首个方法节点（输出按成员去重）
    mbase: HashMap<MemberRef, usize>,
    fields: IndexMap<MemberRef, ()>,
    /// 类型流图：节点类型集 / 流边 / 待推增量（节点驻留为序号）
    graph: FlowGraph,

    /// G：全局已实例化（类型 id）
    g: BTreeSet<u32>,
    /// G 按类型 t 的子集索引：t → G 中 ⊂ t 的成员（有序）。按查询到的 t 惰性建立，G 增长时增量维护，
    /// open 展开与 catch 存活判定因此与 |G| 无关
    g_sub: HashMap<u32, Vec<u32>>,
    lambdas: HashMap<u32, Lambda>,
    /// 手写实现对象（伪类型 id → 对象）
    hwobjs: HashMap<u32, HwObj>,
    /// 数组分配点（抽象对象 id）→ 数组类型 id
    arrays: HashMap<u32, u32>,
    /// 长度恒为 0 的数组分配点：任何元素读写都抛异常，元素节点不接收值；
    /// 值按元素节点暂存，分配点长度不再恒为 0 时补回
    empty_arrays: HashMap<u32, HashMap<Node, TypeSet>>,
    /// 已逃逸的抽象对象（到达 [`Node::Esc`]）
    escaped: HashSet<u32>,
    /// 手写体对接收者自身字段的访问：P(m, 0) → [(字段, 字段类型, 写, 写入值来源)]，按接收者对象逐个接入
    self_fields: HashMap<Node, Vec<(usize, u32, bool, Rc<[Feed]>, Node)>>,
    /// 抽象对象已登记的字段节点（逃逸时补接未知接收者视图）
    obj_fields: HashMap<u32, Vec<(usize, u32)>>,
    /// 容器抽象对象 id → 类型 id；分配点链（`@方法:偏移#…`，堆上下文）
    pub objs: HashMap<u32, u32>,
    obj_chain: HashMap<u32, Rc<str>>,
    /// 容器形态判定缓存（类型 id）
    containers: HashMap<u32, bool>,
    /// 新鲜工厂方法判定缓存（按成员）
    factories: HashMap<MemberRef, bool>,
    /// 分派转发槽判定缓存（按成员）：流到分派接收者的形参槽；静态方法非空即按调用点区分上下文（`forward`）
    forwarders: HashMap<MemberRef, u64>,
    pub inited: IndexMap<String, Via>,

    /// 调用点分派结果：(方法, 偏移) → 目标方法
    pub dispatch: BTreeMap<(usize, u32), BTreeSet<usize>>,
    /// 有接收者到达过的虚调用点（含选不出目标的）：接收者恒为 null 的判定（folds `null_recv`）
    recv_sites: HashSet<(usize, u32)>,
    /// 形参常量（方法 → 按形参槽；缺席 = 尚无调用点）
    pvals: HashMap<usize, Vec<PV>>,
    /// 形参上出现过的字符串常量实参（(方法, 形参槽) → 常量集；按名查找的名字来自形参时逐个展开）
    pstrs: HashMap<(usize, usize), BTreeSet<Rc<str>>>,
    /// 读过 pstrs 的按名查找站点（(方法, 形参槽) → 偏移）：常量集增长时重跑
    pstr_sites: HashMap<(usize, usize), BTreeSet<u32>>,
    /// 派发枢纽；(调用成员, 接口调用, 接收者集合) → 序号；open 类型 → 枢纽
    hubs: Vec<Hub>,
    hub_ids: HashMap<(MemberRef, bool, HubSet), u32>,
    hubs_by_open: BTreeMap<u32, Vec<u32>>,
    /// 调用点 → 所连枢纽（输出分派结果用）；调用点当前的精确集合枢纽
    hub_sites: BTreeMap<(usize, u32), BTreeSet<u32>>,
    hub_last: HashMap<(usize, u32), u32>,
    /// VM 反射虚调用枢纽（[`HubSet::Vm`]）
    vm_hubs: HashSet<u32>,
    /// 调用边的反向表（被调 → 调用方）：被调方法重算后调用方重处理（透传摘要可能变化）
    callers: HashMap<usize, BTreeSet<usize>>,
    /// 当前字节码调用点的实参值（不含接收者）；其余入口（手写 / 方法句柄 / lambda）为 None = 形参值未知
    call_vals: Option<Rc<[V]>>,
    pub unresolved: BTreeSet<String>,
    /// 活代码调用点的符号引用（常量池 owner.name:desc）：发射层槽位需求按调用点键消费
    pub refs: BTreeSet<String>,
    /// 运行模型替换的 indy 调用点（`方法@偏移` → (引导方法, 类别)）
    pub indy_models: BTreeMap<String, (String, IndyKind)>,
    /// 诊断：丢弃冷路径（`cold::doomed`）上的事件，量化冷路径独占的闭包规模（不健全，只用于测量）
    pub cold_cut: bool,

    mwork: VecDeque<usize>,
    in_mwork: HashSet<usize>,
    /// 类型集读者：节点 → 读过它做分派 / 拆分决策的字节码站点（方法, 偏移）；节点增长只重跑这些站点
    watch: HashMap<Node, HashSet<(usize, u32)>>,
    swork: VecDeque<(usize, u32)>,
    in_swork: HashSet<(usize, u32)>,
    /// 正在处理的字节码站点（读类型集时登记为读者）
    cur_site: Option<(usize, u32)>,
    /// 字节码调用点已分派过的接收者（方法 → (偏移, 接收者, 经由的 lambda 或 NOCTX)）；同一分析结果下分派是确定的，
    /// 重跑站点只处理新增接收者。lambda 接收者不登记（见 [`Self::dispatch_one`]）
    dispatched: HashMap<usize, HashSet<(u32, u32, u32)>>,
    /// 已接入枢纽的调用点：方法 → (偏移, 枢纽)（同 `dispatched`，分析重算时清空）
    hub_linked: HashMap<usize, HashSet<(u32, u32)>>,
    /// 字节码调用点经枢纽已分派的 lambda / 手写实现对象接收者：方法 → (偏移, 接收者)（同 `hub_linked` 清空）。
    /// 调用点换接子枢纽时继承的接收者、同一接收者经多个枢纽到达时，同一分析结果下重派发是恒等重放
    hub_lsent: HashMap<usize, HashSet<(u32, u32)>>,
    /// 字节码调用点经枢纽已接的按调用点建模目标：方法 → (偏移, 目标) → 已送达的接收者（同上）。
    /// `edge` 对接收者值集逐元素单调（首接生效的手写站点登记已在首次完成），接收者全已送达即恒等重放
    hub_ssent: HashMap<usize, HashMap<(u32, usize), HashSet<u32>>>,
    /// 字段读写 / 非虚调用站点已接上的接收者抽象对象：方法 → (偏移, 对象)（同 `dispatched`）。
    /// 站点因接收者集合增长重跑时只接新增对象
    recv_done: HashMap<usize, HashSet<(u32, u32)>>,
    /// 求值读取同一分析里其他偏移事件的站点（按名取类 / 按名查方法：沿拼接链、常量表接收者回溯）：方法 → 偏移。
    /// 重分析时只要有事件变化，这些站点连同变化的偏移一起重跑（见 `process_bytecode`）
    whole_sites: HashMap<usize, BTreeSet<u32>>,
    /// 字节码调用点上已登记的 lambda 调用：方法 → 偏移 → 调用 → `lcalls` 序号（同 `dispatched`，分析重算时作废）
    lambda_done: HashMap<usize, HashMap<u32, HashMap<LambdaCall, u32>>>,
    lcalls: Vec<LCall>,
    /// 正在读值集的 lambda 调用（优先于 `cur_site` 登记为读者）
    cur_call: Option<u32>,
    /// 类型集节点 → 读它的 lambda 调用；open 展开过的 lambda 调用，按 (open 类型, 接收者上界) 索引
    call_watch: HashMap<Node, HashSet<u32>>,
    open_calls: BTreeMap<(u32, u32), BTreeSet<u32>>,
    cwork: VecDeque<u32>,
    in_cwork: HashSet<u32>,
    /// 手写方法调用点（调用方, 偏移, 被调方法）→ 序号；数组写入按调用点建模
    hw_site_ids: HashMap<(usize, u32, usize), u32>,
    hw_sites: Vec<(usize, u32, usize)>,
    /// 读内存的手写调用点（`[facts.memory_reads]`）：站点 → (源实参序号（含接收者）, 结果节点, 返回类型)
    hw_reads: HashMap<u32, (u16, Node, u32)>,
    /// 类（含超类）的引用实例字段节点（内存读取的对象分量）
    ref_fields: HashMap<u32, Rc<[(usize, u32)]>>,
    hw_writes: HashMap<usize, Rc<[Option<HwWrite>]>>,
    /// 目标可能是任一按名打开的静态字段的写入值节点：签名多态写入调用点（静态字段句柄无 holder 坐标）、
    /// 以所指未知的类镜像为静态字段基址的按偏移写入
    poly_writes: Vec<Node>,
    /// 按名打开（反射 / VarHandle / Unsafe 按名写入）的静态引用字段
    open_statics: Vec<(usize, u32)>,
    /// 待沿流边推送增量的节点序号
    fwork: VecDeque<u32>,
    /// 按 open 在 G 上展开过接收者的方法，按 (open 类型, 接收者上界) 索引：新成员落在两者之下时重处理
    open_methods: BTreeMap<(u32, u32), BTreeSet<usize>>,
    /// 按 open 在 G 上展开过接收者的字节码站点，索引同上（只重跑这些站点）
    open_sites: BTreeMap<(u32, u32), BTreeSet<(usize, u32)>>,
    /// 分析时按「尚无实例」处理的类型（catch / instanceof 目标）：其子类型进入 G 时方法重分析
    pending_types: BTreeMap<usize, Vec<String>>,
    /// 进行中的 lambda 调用（lambda, 实参）：绑定方法引用的接收者可能是 lambda 自身，同一调用重入即成环
    lambda_stack: HashSet<LambdaCall>,
    /// 下一次 `add_to` 来自流边推送（诊断：区分 open 的直接注入点）
    via_flow: bool,
    /// open 的直接注入点：节点 → 注入的 open 类型（诊断 `@openorig`）
    open_inj: HashMap<Node, BTreeSet<u32>>,
    /// 类镜像（Class 对象按所指类区分）：镜像 id → 所指类型 id。镜像的类型是 Class，不做克隆上下文
    mirrors: HashMap<u32, u32>,
    /// 类型序号 → 其类镜像序号（`mirror` 的记忆，免逐值格式化镜像名）；未登记为 `u32::MAX`
    mirror_of: Vec<u32>,
    /// 流边上的镜像变换 src → dst：src 中每个值的类镜像流入 dst（`getClass` 逐调用点）
    mflows: HashMap<Node, Vec<Node>>,
    mflow_seen: HashSet<(Node, Node)>,
    /// 成员枚举的接收者节点 → 枚举类别；节点增长的新增部分排队处理
    enum_recv: HashMap<Node, (Members, usize)>,
    rpending: Vec<(Members, usize, TypeSet)>,
    /// 已枚举（类别, 所指类）；有对应反射调用可达时成员入链
    enumerated: BTreeSet<(Members, u32)>,
    /// 可达的反射调用类别
    invokable: BTreeSet<Members>,
    /// 反射点名：类型 id → 在以 Class 为接收者 / 实参的调用里与之同现的字符串常量（按名取成员）
    reflect_names: HashMap<u32, BTreeSet<String>>,
    /// 按名取类（常量名解析）取到的类：其构造器随构造器枚举进入反射面
    named_ctors: BTreeSet<u32>,
    /// 反射缺口：接收者镜像推不出的成员枚举
    pub reflect_gaps: BTreeSet<String>,
    /// 类初始化事实（`[facts.class_init]`）
    pub class_init: class_init::ClassInitFacts,
    /// 反射成员面：（类别, 成员）
    pub reflect_members: BTreeSet<(Members, MemberRef)>,
    /// 手写层写入的字段（`__set_` 接收者类型已定位）
    pub hw_written: BTreeSet<MemberRef>,
    /// 按字段句柄写字段的入口已可达
    fwriter_live: bool,
    /// 返回属性表对象的方法与其调用方可见性（sysprops.rs）
    spret: sysprops::SpRet,
    /// 等待句柄写入口可达的字段枚举：Some(类) = 该类及其超类的字段，None = 全部字段
    fenum_pending: BTreeSet<Option<String>>,
    /// 手写层写入但接收者类型推不出的字段名：所有同名字段按有手写写入处理
    pub hw_written_names: BTreeSet<String>,
    /// 手写层读取但接收者类型推不出的字段名 → 读出值汇入的值池：所有同名字段流入
    hw_read_names: BTreeMap<String, BTreeSet<Node>>,
    /// `包/蛇形名` → 类（手写 `use super::<类>_impl` 模块引用的反查；首次使用时建立）
    snake_index: Option<HashMap<String, String>>,
    /// 清单种子状态与输出
    pub seeds: SeedState,
}

impl<'a> Engine<'a> {
    /// 接收者对应的克隆上下文
    fn ctx_of(&self, r: u32) -> u32 {
        if self.objs.contains_key(&r) {
            r
        } else {
            NOCTX
        }
    }

    fn on_g_grow(&mut self, id: u32) {
        let ts: Vec<u32> = self.g_sub.keys().copied().collect();
        for t in ts {
            if self.sub(id, t) {
                // 有序插入
                let v = self.g_sub.get_mut(&t).unwrap();
                if let Err(i) = v.binary_search(&id) {
                    v.insert(i, id);
                }
            }
        }
        self.hubs_grow(id);
        self.reopen(id);
        let pend: Vec<(usize, Vec<String>)> = self.pending_types.iter().map(|(k, v)| (*k, v.clone())).collect();
        for (m, tys) in pend {
            let hit = tys.iter().any(|t| {
                let tid = self.id(t);
                self.sub(id, tid)
            });
            if hit {
                self.pending_types.remove(&m);
                let had = self.methods[m].analysis.take().is_some();
                if had {
                    self.nr_dropped(m);
                }
                self.ctx.stats.borrow_mut().invalidated(m, Why::Catch, had);
                self.push_m(m);
            }
        }
    }

    /// open 展开的取值面扩大（G 增长 / 数组逃逸）：x 落在其 open 类型与接收者上界之下的方法与站点重跑
    fn reopen(&mut self, x: u32) {
        let keys: Vec<(u32, u32)> =
            self.open_methods.keys().chain(self.open_sites.keys()).chain(self.open_calls.keys()).copied().collect();
        let hit: HashSet<(u32, u32)> = keys.into_iter().filter(|&(o, owner)| self.sub(x, o) && self.sub(x, owner)).collect();
        let mut open: BTreeSet<usize> = BTreeSet::new();
        let mut sites: BTreeSet<(usize, u32)> = BTreeSet::new();
        for (k, ms) in &self.open_methods {
            if hit.contains(k) {
                open.extend(ms.iter().copied());
            }
        }
        for (k, ws) in &self.open_sites {
            if hit.contains(k) {
                sites.extend(ws.iter().copied());
            }
        }
        let mut calls: BTreeSet<u32> = BTreeSet::new();
        for (k, cs) in &self.open_calls {
            if hit.contains(k) {
                calls.extend(cs.iter().copied());
            }
        }
        for c in calls {
            if self.in_cwork.insert(c) {
                self.cwork.push_back(c);
            }
        }
        for m in open {
            self.push_m(m);
        }
        for w in sites {
            if self.in_swork.insert(w) {
                self.swork.push_back(w);
            }
        }
    }

}
