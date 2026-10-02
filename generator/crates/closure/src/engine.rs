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
/// 插入序映射（遍历按插入序，与哈希无关）；哈希用引擎的 Fx
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<sets::FxHasher>>;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::absint::{self, Analysis, Event, Obj, Oracle, Ret, Src, V};
use crate::handwritten::{member_matches, CRATE_ROOT, to_snake, MODULE_SUFFIXES, ClassHw, FieldAccess, Handwritten, MemberHw, SType, TypeRef, TypedCall, Upcall};
use crate::manifest::{Domain, Fact, IndyKind, Manifest, Members, PropValue};

mod sets;
mod idset;
mod facts;
mod consteval;
mod construct;
mod sysprops;
mod fold;
mod unmodeled;
mod forward;
mod ctxsel;
mod classes;
mod reflect;
mod reflect_call;
use reflect_call::{RHook, RcallMember};
mod flow;
mod bytecode;
mod invoke;
mod hub;
mod gather;
mod defs;
pub use defs::{ClassNode, From, Kind, Level, Via};
use defs::*;
mod lambda;
mod lambda_adapt;
mod hw;
mod hw_mem;
mod hw_syntax;
mod hw_stype;
mod hw_infer;
mod hw_inherit;
mod hwobj;
mod vmhook;
mod field_hooks;
mod rtfn;
mod vmrules;
mod hwfield;
mod report;
mod diag;
mod seeds;
mod services;
mod class_init;
mod memo;
mod mirror_eq;
mod selector;
mod noreturn;
mod class_lookup;
mod sealed;
mod nest;
mod method_lookup;
mod field_lookup;
mod pstrs;
mod share;
mod new;
mod methods;
mod worklist;
pub use worklist::FLOW_BATCH;
mod stats;
mod graph;
pub mod cut;
mod setstore;
use setstore::SetStore;
mod scc;
mod levels;

use graph::FlowGraph;
use share::Dep;
use ctxsel::Call;
use stats::{Phase, Why};
pub use cut::Diag;
pub use stats::{peak_mem_mb, peak_rss_mb};

pub use seeds::SeedState;

pub use fold::{DeadCatch, Fold};
use facts::*;
use sysprops::{PropSum, PropUnstable};
use fold::*;
pub use sets::*;
pub use idset::{IdIter, IdSet};
use hwfield::HWFIELD_KIND;
use hwobj::{HwObj, HWOBJ_KIND};
use vmhook::VMHOOK_KIND;
use rtfn::RTFN_KIND;

/// 精确接收者达到此数时经集合枢纽派发
const HUB_MIN: usize = 8;
/// 字段站点在 `recv_done` 中的哨兵：与值无关的部分已接 / 未知接收者视图已接（抽象对象 id 不会取到）
const FIELD_STATIC: u32 = u32::MAX;
const FIELD_OTHER: u32 = u32::MAX - 1;
/// 站点接收者登记里 open 值的标记位（类型编号远小于此）
const OPEN_MARK: u32 = 1 << 30;

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
    /// `analysis` 的装入序号（每次装入加一）；`applied_seq` 为 `applied` 装入时的序号。
    /// 摘要在上下文间共享（`share.rs`），「同一次装入」不能再按 `Rc` 身份判定
    aseq: u32,
    applied_seq: u32,
    /// 最近一次分析的透传摘要（None = 尚未分析）
    returned: Option<Option<Vec<u16>>>,
    /// 手写体命中的 fn 名（溯源）
    pub hw_fns: Vec<String>,
    /// 克隆上下文：接收者抽象对象（容器对象敏感），NOCTX = 方法本体
    pub ctx: u32,
    /// 清单声明的返回值模型
    ret_model: RetModel,
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
    /// open(o) 按过滤类型 t 收窄的结果缓存（`u32::MAX` = 空）
    narrow_cache: HashMap<(u32, u32), u32>,
    /// 子类型判定缓存，按上界 f 分行、每个类型 id 两位（已判定 / 结果）：`sub` 与类型集收窄共用
    sub_rows: Vec<sets::SubRow>,

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
    /// 按非虚处理的虚指令调用点（invokevirtual / invokeinterface 指向 final 方法或 final 类）：
    /// 目标照样经 vtable 槽到达，并入 `dispatched` 输出
    direct_virtual_sites: HashSet<(usize, u32)>,
    /// 形参常量（方法 → 按形参槽；缺席 = 尚无调用点）
    pvals: HashMap<usize, Vec<PV>>,
    /// 流到形参的字符串常量集（按名查找的名字来自形参时逐个展开；只并不减，见 `pstrs.rs`）
    pstr: pstrs::PStrs,
    /// 派发枢纽；(调用成员, 接口调用, 接收者集合) → 序号；open 类型 → 枢纽
    hubs: Vec<Hub>,
    hub_ids: HashMap<(MemberRef, bool, HubSet), u32>,
    hubs_by_open: BTreeMap<u32, Vec<u32>>,
    /// 调用点 → 所连枢纽（输出分派结果用）；调用点当前的精确集合枢纽
    hub_sites: BTreeMap<(usize, u32), BTreeSet<u32>>,
    /// 调用点当前的精确集合枢纽及其接收者集合（集合未变的重跑免查 `hub_ids`）
    hub_last: HashMap<(usize, u32), (u32, Rc<[u32]>)>,
    /// 字段汇集节点：序号 → (字段, 对象数, 写入向)；(字段, 写入向, 对象集合) → 序号；字节码字段站点 → (字段, 当前汇集节点, 累计对象)
    gathers: Vec<(usize, u32, bool)>,
    gather_ids: HashMap<(usize, bool, Rc<[u32]>), u32>,
    gather_last: HashMap<usize, HashMap<u32, (usize, u32, Rc<[u32]>)>>,
    /// VM 反射虚调用枢纽（[`HubSet::Vm`]）
    vm_hubs: HashSet<u32>,
    /// VM 反射虚调用枢纽选中的目标（按接收者虚分派到的实现；并入 `dispatched` 输出）
    vm_targets: HashSet<usize>,
    /// VM 反射虚调用枢纽的目标接法：形参按声明类型 open 的枢纽（记录分量访问器等）、按反射调用实参池接形参的枢纽
    /// （→ 反射成员序号）；已选中的目标（接法扩大时补接）
    vm_open_hubs: HashSet<u32>,
    vm_rcall_hubs: HashMap<u32, usize>,
    vm_hub_targets: HashMap<u32, Vec<usize>>,
    /// 反射调用（`reflect_call.rs`）：反射方法成员（按声明键）及其序号、各目标已接的实参池通道位集、
    /// 方法 → 作为调用入口的通道（缓存）、已接入的入口调用点、已接值池的句柄入口、推不出所指方法的反射对象转成了
    /// 方法句柄（清单 `method_to_handle`）、派发统计
    rcall_members: Vec<RcallMember>,
    rcall_ix: HashMap<MemberRef, usize>,
    rcall_bound: HashMap<usize, u8>,
    rcall_entries: HashMap<usize, Option<u8>>,
    rcall_sites: HashSet<(usize, u32, usize)>,
    rcall_pooled: HashSet<usize>,
    rcall_m2h: bool,
    /// 转成方法句柄的反射对象形参（方法键, 形参序号含接收者）；尚未登记的反射对象形参上的调用点实参（按被调方法键）
    rcall_conv: HashSet<(String, u16)>,
    rcall_conv_pending: HashMap<String, Vec<(usize, u16, V)>>,
    rcall_conv_seen: HashSet<(usize, u32, u16)>,
    rcall_stats: reflect_call::RcallStats,
    /// 调用边的反向表（被调 → 调用方）：被调方法重算后调用方重处理（透传摘要可能变化）
    callers: HashMap<usize, BTreeSet<usize>>,
    /// 当前字节码调用点的实参值（不含接收者）；其余入口（手写 / 方法句柄 / lambda）为 None = 形参值未知
    call_vals: Option<Rc<[V]>>,
    pub unresolved: BTreeSet<String>,
    /// 活代码调用点的符号引用（常量池 owner.name:desc）：发射层槽位需求按调用点键消费
    pub refs: BTreeSet<String>,
    /// 成员引用的文本键（清单按文本查询；字节码事件反复查同一引用）及是否已记入 `refs`
    mref_keys: HashMap<MemberRef, (Rc<str>, bool)>,
    /// 运行模型替换的 indy 调用点（`方法@偏移` → (引导方法, 类别)）
    pub indy_models: BTreeMap<String, (String, IndyKind)>,
    /// 签名多态调用点（`方法@偏移`，JVMS §2.9.3）：JVM 链接到 LambdaForm 调用器，发射层走手写 `__site` 伴生——
    /// 与 indy 同属运行模型替换，动态对照据此归因其上方的调用器帧
    pub sigpoly_sites: BTreeSet<String>,
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
    /// 字段读写 / 非虚调用站点已接上的接收者抽象对象：方法 → (偏移, 对象)（同 `dispatched`）。
    /// 站点因接收者集合增长重跑时只接新增对象。按站点存升序表（站点多有几十到上百个对象，比逐条哈希省内存）
    recv_done: HashMap<usize, HashMap<u32, Vec<u32>>>,
    /// 字节码调用点上已登记的 lambda 调用：方法 → 偏移 → 调用 → `lcalls` 序号（同 `dispatched`，分析重算时作废）
    lambda_done: HashMap<usize, HashMap<u32, HashMap<LambdaCall, u32>>>,
    lcalls: Vec<LCall>,
    /// 正在读值集的 lambda 调用（优先于 `cur_site` 登记为读者）
    cur_call: Option<u32>,
    /// 类型集节点 → 读它的 lambda 调用；open 展开过的 lambda 调用，按 (open 类型, 接收者上界) 索引
    call_watch: HashMap<Node, HashSet<u32>>,
    /// Class 形参节点 → 依赖「值集不含某类镜像」答复的（方法, 类序号）：值集增长到可能含该镜像时重分析
    mirror_watch: HashMap<Node, BTreeSet<(usize, u32)>>,
    /// 方法 → 可共享的摘要（按入口状态，见 `share.rs`）
    shared: HashMap<MemberRef, Vec<share::Shared>>,
    open_calls: BTreeMap<(u32, u32), BTreeSet<u32>>,
    cwork: VecDeque<u32>,
    in_cwork: HashSet<u32>,
    /// 手写方法调用点（调用方, 偏移, 被调方法）→ 序号；数组写入按调用点建模
    hw_site_ids: HashMap<(usize, u32, usize), u32>,
    /// 同一数组自拷贝的手写调用点（站点, 元素来源形参, 写入目标形参）：两实参是同一个入口形参值，
    /// 运行期是同一数组，元素集不变，不在两者的各数组之间交叉接元素
    hw_self_copies: HashSet<(u32, u16, u16)>,
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
    /// 以已知类镜像为静态字段基址的按偏移写入值节点（键 = 镜像所指类）：只接该类按名打开的静态引用字段
    mirror_writes: HashMap<u32, Vec<Node>>,
    /// 目标字段尚未开放的实例字段偏移写入（字段 → (写入值节点, 字段节点, 字段类型, 口径)），字段按口径开放时接上
    offset_waits: HashMap<usize, Vec<(Node, Node, u32, hw_mem::Gate)>>,
    /// 字段偏移尚未取得的按偏移读取（字段 → (字段节点, 读取结果节点, 结果类型, 口径)），偏移按口径可得时接上
    offset_read_waits: HashMap<usize, Vec<(Node, Node, u32, hw_mem::Gate)>>,
    /// 待沿流边推送增量的节点序号
    fwork: VecDeque<u32>,
    /// 跨偏移读者：求值读本方法其它偏移事件的站点（方法 → 偏移；按名查找），重分析时一并重跑
    xreaders: HashMap<usize, BTreeSet<u32>>,
    /// 按名取类已推不出的调用点：恒按推不出处理（`class_lookup` 单调）
    lookup_top: HashSet<(usize, u32)>,
    /// 本次按名取类求值中，常量表读取的接收者含非常量表的值（候选只覆盖常量表部分，结果另接所指未知的 Class）
    lookup_partial: bool,
    /// 两次排空流传播之间最多处理的方法 / 站点数（`worklist.rs::run`；`rava closure --flow-batch N` 可改，1 = 逐个排空）
    pub flow_batch: usize,
    /// 按 open 在 G 上展开过接收者的方法，按 (open 类型, 接收者上界) 索引：新成员落在两者之下时重处理
    open_methods: BTreeMap<(u32, u32), BTreeSet<usize>>,
    /// 按 open 在 G 上展开过接收者的字节码站点，索引同上（只重跑这些站点）
    open_sites: BTreeMap<(u32, u32), BTreeSet<(usize, u32)>>,
    /// 分析时按「尚无实例」处理的类型（catch / instanceof 目标）：其子类型进入 G 时方法重分析
    pending_types: BTreeMap<usize, Vec<String>>,
    /// 进行中的 lambda 调用（lambda, 实参）：绑定方法引用的接收者可能是 lambda 自身，同一调用重入即成环
    lambda_stack: HashSet<LambdaCall>,
    /// 下一次 `add_to` 来自流边推送时为源节点序号，否则为 [`diag::NO_SRC`]（诊断：区分直接注入点、记录型查询的来源）
    flow_src: u32,
    /// open 的直接注入点：节点 → 注入的 open 类型（诊断 `@openorig`）
    open_inj: HashMap<Node, BTreeSet<u32>>,
    /// 类镜像（Class 对象按所指类区分）：镜像 id → 所指类型 id。镜像的类型是 Class，不做克隆上下文
    mirrors: HashMap<u32, u32>,
    /// 类型序号 → 其类镜像序号（`mirror` 的记忆，免逐值格式化镜像名）；未登记为 `u32::MAX`
    mirror_of: Vec<u32>,
    /// 流边上的镜像变换 src → dst：src 中每个值的类镜像（`getClass`）/ 各镜像所指类的超类镜像（`getSuperclass`）
    /// 流入 dst（逐调用点）
    mflows: HashMap<Node, Vec<(Node, MirrorOp)>>,
    mflow_seen: HashSet<(Node, Node, MirrorOp)>,
    /// `getClass` 作用于 open(T) 的结果节点：T → 节点（T 的已实例化子类型增长时补入其类镜像，见 `reflect.rs`）
    mirror_open: BTreeMap<u32, Vec<Node>>,
    mirror_open_seen: HashSet<(u32, Node)>,
    /// 非字节码类（lambda 合成类、手写实现对象）的共用类镜像：Class 类型的抽象对象，不指向任何字节码类、无 Java 字段
    synth_mirror: Option<u32>,
    /// 成员枚举的接收者节点 → 枚举类别；节点增长的新增部分排队处理
    /// 反射调用的实参池 / 实参数组节点同样登记于此（[`RHook`]：通道号代替方法序号）
    enum_recv: HashMap<Node, (RHook, usize)>,
    rpending: Vec<(RHook, usize, TypeSet)>,
    /// 已枚举（类别, 所指类）；有对应反射调用可达时成员入链
    enumerated: BTreeSet<(Members, u32)>,
    /// 可达的反射调用类别
    invokable: BTreeSet<Members>,
    /// 反射点名：类型 id → 在以 Class 为接收者 / 实参的调用里与之同现的字符串常量（按名取成员）
    /// → 经哪些反射调用通道调用（通道位集，见 `reflect_call.rs`）
    reflect_names: HashMap<u32, BTreeMap<String, u8>>,
    /// 按名取类（常量名解析）取到的类：其构造器随构造器枚举进入反射面
    named_ctors: BTreeSet<u32>,
    /// 反射缺口：接收者镜像推不出的成员枚举
    pub reflect_gaps: BTreeSet<String>,
    /// 按名查字段点到的字段（声明类, 名字），见 `field_lookup.rs`
    pub reflect_fields: BTreeSet<(String, String)>,
    /// 按名查字段目标类推不出时的字面量名（任意类的同名字段）
    pub reflect_field_names: BTreeSet<String>,
    /// String 字段各写入处的字符串常量（None = 有非常量写入）；名字经字段到达按名查找点时取用
    field_strs: HashMap<MemberRef, Option<BTreeSet<Rc<str>>>>,
    /// 类初始化事实（`[facts.class_init]`）
    pub class_init: class_init::ClassInitFacts,
    /// 反射成员面：（类别, 成员）
    pub reflect_members: BTreeSet<(Members, MemberRef)>,
    /// 经按名查找 / 字段枚举取得字段句柄的字段与（目标类推不出时的）字段名：方法句柄解释器读写口径（`hw_mem::Gate::Handle`）
    handle_fields: HashSet<MemberRef>,
    handle_names: HashSet<String>,
    /// 手写层写入的字段（`__set_` 接收者类型已定位）
    pub hw_written: BTreeSet<MemberRef>,
    /// 按字段句柄写字段的入口已可达
    fwriter_live: bool,
    /// 反事实切除（诊断，缺省为空）
    pub(crate) cuts: cut::Cuts,
    /// 记录型 `--flows` 查询（诊断；未登记为 None，热路径只判空）
    probes: Option<Box<diag::Probes>>,
    /// 返回属性表对象的方法与其调用方可见性（sysprops.rs）
    spret: sysprops::SpRet,
    /// 等待句柄写入口可达的字段枚举：Some(类) = 该类及其超类的字段，None = 全部字段
    fenum_pending: BTreeSet<Option<String>>,
    /// 字段枚举缺口：接收者 Class 值集含所指未知的 Class 的枚举调用点（`方法@偏移`）；句柄写入口可达时全部字段不折叠
    pub field_enum_gaps: BTreeSet<String>,
    /// 手写层写入但接收者类型推不出的字段名：所有同名字段按有手写写入处理
    pub hw_written_names: BTreeSet<String>,
    /// 手写层读取但接收者类型推不出的字段名 → 读出值汇入的值池：所有同名字段流入
    hw_read_names: BTreeMap<String, BTreeSet<Node>>,
    /// `包/蛇形名` → 类（手写 `use super::<类>_impl` 模块引用的反查；首次使用时建立）
    snake_index: Option<HashMap<String, String>>,
    /// 清单种子状态与输出
    pub seeds: SeedState,
    /// 已触发的 VM 规则（位图，见 `vmrules.rs`）
    vm_rules_fired: u64,
    /// 手写体 static 字段读已接入的 (方法, 字段)：访问器手写体自引用时不重入
    hw_static_reads: HashSet<(usize, MemberRef)>,
}

impl<'a> Engine<'a> {
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
        self.vm_hooks_on_alloc(id);
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
        self.mirror_reopen(x);
    }

}
