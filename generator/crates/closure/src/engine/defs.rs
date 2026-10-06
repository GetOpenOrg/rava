//! 引擎：溯源（`Via` / 类层级 / 方法类别）与内部图节点、值来源、lambda 调用、派发枢纽的定义

use super::*;

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
    pub(super) fn root(kind: &'static str, what: &str) -> Via {
        Via { kind, from: From::Root(what.to_string()), off: None }
    }
    pub(super) fn method(kind: &'static str, m: usize, off: Option<u32>) -> Via {
        Via { kind, from: From::Method(m), off }
    }
    pub(super) fn class(kind: &'static str, c: &str) -> Via {
        Via { kind, from: From::Class(c.to_string()), off: None }
    }
}

/// 类在闭包中的层级（取最高）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// L1 名字：仅作类型出现（签名 / checkcast / instanceof / catch），值只可能是 null
    Type,
    /// L2 布局：需要 struct / 接口载体（非 L1 类的超类型、手写层引用、实例字段属主），见 `levels.rs`
    Layout,
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

/// 流边上的类镜像变换（镜像流边 `mflow`）
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum MirrorOp {
    /// 每个值的类镜像（`getClass`）
    Of,
    /// 每个类镜像所指类的直接超类镜像（`getSuperclass`）
    Super,
    /// 每个数组类镜像的元素类型镜像（`getComponentType`）
    Component,
    /// 每个类镜像所指成员类的声明类镜像（`getDeclaringClass0`）
    Declaring,
    /// 所指类 ⊂ 该类型（类型 id）的类镜像（类镜像子类型判定成立一侧，见 `absint/narrow.rs`）
    Sub(u32),
}

/// 返回值按调用点建模的清单声明（`vm_intrinsics.toml`）
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RetModel {
    /// 结果取返回值节点
    Plain,
    /// 类镜像：本调用点接收者各值的 Class 对象
    Mirror,
    /// 超类镜像：本调用点接收者各类镜像所指类的直接超类镜像
    Super,
    /// 元素类型镜像：本调用点接收者各数组类镜像的元素类型镜像
    Component,
    /// 声明类镜像：本调用点接收者各类镜像所指成员类的声明类镜像
    Declaring,
    /// 浅拷贝：本调用点的接收者
    Receiver,
    /// 按实参（序号，不含接收者）读内存
    Read(usize),
    /// 调用者类镜像：调用方（@CallerSensitive 方法）各调用边上调用方所在类的镜像
    Caller,
}

impl RetModel {
    /// 结果按接收者经镜像变换给出时的变换
    pub(super) fn mirror_op(self) -> Option<MirrorOp> {
        match self {
            RetModel::Mirror => Some(MirrorOp::Of),
            RetModel::Super => Some(MirrorOp::Super),
            RetModel::Component => Some(MirrorOp::Component),
            RetModel::Declaring => Some(MirrorOp::Declaring),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(super) enum Node {
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
    /// 字段汇集节点（`gathers` 序号）：同一字段、同一抽象对象集合的读取站点（或写入站点）共用（见 `gather.rs`）
    G(u32),
    /// 反射调用实参池（按池号：反射对象接收者 / 方法句柄 / 反射对象实参，见 `reflect_call.rs`）：该通道调用入口的接收者 /
    /// 实参汇入，实参池按声明类型流向经该通道调用的反射成员的形参；接收者池（方法句柄通道即其唯一池）新增值按接收者派发反射实例成员
    RP(u8),
    /// 反射调用实参池的去冗余视图（按通道）：池中某 open 类型已涵盖、且只能经未知接收者视图读写的值（已逃逸的抽象对象 /
    /// 数组、非抽象对象的类）不再逐个列出——open 形参经枢纽展开到全部成员、字段经 `U` / `F` 读写，结果与逐个列出一致。
    /// 反射成员的形参从这里接池
    RN(u8),
    /// 反射调用实参数组（按通道）：入口按数组打包传入的实参（反射对象入口的 `Object[]`），新增数组的元素并入该通道的 [`Node::RP`]
    RA(u8),
    /// 逃逸汇点：流入非建模代码（手写体 / native 的值池、VM 回调的返回值、未知数组）的值。
    /// 抽象对象到达这里即「已逃逸」——只有它们可能以 open / 非抽象接收者的身份被读写
    Esc,
    /// 按键查找闸门（`keyed.rs` 闸门序号）：调用点结果先汇入这里，按键放行到站点节点
    K(u32),
    /// 手写按名写入的接收者汇集节点（`hw_name_write.rs` 序号）：接收者取自按名读的字段内容，按值集解出被写字段
    NR(u32),
}

/// 值的类型来源：节点，或直接给定的类型集（字面量 / 未知值的 open）
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Feed {
    N(Node),
    S(TypeSet),
}

/// 手写方法写入某个形参数组的值来源（形参序号含接收者）
#[derive(Clone, Debug)]
pub(super) struct HwWrite {
    /// 这些形参的值本身
    pub(super) values: Vec<usize>,
    /// 这些形参里数组的元素
    pub(super) elements: Vec<usize>,
    /// 手写体产出（分配 / 字段读取 / 回调返回值）
    pub(super) produced: bool,
    /// 目标为对象时写入其引用实例字段
    pub(super) fields: bool,
    /// 写入值取自调用点最后一个实参（签名多态）
    pub(super) last: bool,
    /// 字段偏移实参（序号含接收者）
    pub(super) offset: Option<usize>,
}

/// 按声明形参位置的实参来源（基本类型为 None）
pub(super) type Args = Vec<Option<Vec<Feed>>>;

/// 一次 lambda 调用：(lambda, 实参, 返回类型, 结果节点)
pub(super) type LambdaCall = (u32, Args, Option<u32>, Option<Node>);

/// 字节码调用点（方法, 偏移）上的一次 lambda 调用（读者单元，见 [`Engine::invoke_lambda`]）
pub(super) struct LCall {
    pub(super) m: usize,
    pub(super) off: u32,
    pub(super) call: LambdaCall,
    /// 已接过的接收值
    pub(super) done: TypeSet,
    /// 调用方分析重算后作废（调用点重跑时按新分析重新登记）
    pub(super) live: bool,
}

/// 被调方法的接收者
pub(super) enum Recv {
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
pub(super) struct Hub {
    pub(super) site: resolve::MethodSite,
    pub(super) owner: u32,
    /// open 类型（open 枢纽）
    pub(super) open: Option<u32>,
    pub(super) parent: Option<u32>,
    /// 精确集合枢纽的接收者集合（升序，与枢纽键共享）；父枢纽承接其中的子集部分
    pub(super) set: Option<Rc<[u32]>>,
    /// 实参声明类型（不含接收者）与返回类型
    pub(super) ptypes: Vec<Option<u32>>,
    pub(super) ret: Option<u32>,
    /// 各调用点实参常量的汇合（None = 尚无调用点；首个调用点接入后才展开接收者）
    pub(super) vals: Option<Vec<PV>>,
    pub(super) via: Via,
    /// 待展开（精确集合）/ 本枢纽展开的接收者（不含父枢纽承接的）
    pub(super) expanded: bool,
    pub(super) pending: Vec<u32>,
    pub(super) recvs: BTreeSet<u32>,
    /// 经枢纽中转的目标
    pub(super) plain: BTreeSet<usize>,
    /// 逐调用点派发的 lambda / 手写实现对象接收者（含父枢纽的）
    /// 子枢纽建立时与父枢纽共享，自身展开追加时才复制（写时复制）
    pub(super) lambdas: Rc<Vec<u32>>,
    /// 按调用点建模的目标 → 其接收者（含父枢纽的）：逐调用点接边，同目标的接收者合成一条
    /// 各接收者表与父枢纽共享、写时复制：重放时同一张表即祖先已全部送达
    pub(super) special: BTreeMap<usize, Rc<Vec<u32>>>,
    /// 调用点（方法, 偏移）→ 接入记录（重接入时换新记录）
    pub(super) links: BTreeMap<(usize, u32), Rc<Link>>,
    /// 接入记录序号分配
    pub(super) link_seq: u32,
    /// 已对 (接入记录, 按调用点建模的目标) 完整接边：同一记录再派发该目标的新接收者时只接接收者相关部分
    pub(super) edged: HashSet<(u32, usize)>,
}

/// 调用点接入枢纽的记录：实参来源、结果节点、实参值
pub(super) struct Link {
    pub(super) id: u32,
    pub(super) a: Args,
    pub(super) res: Option<Node>,
    pub(super) cv: Option<Rc<[V]>>,
}

/// 枢纽的接收者集合键
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum HubSet {
    Open(u32),
    Exact(Rc<[u32]>),
    /// VM 按反射对象虚调用（`Method.invoke` / REF_invokeVirtual 的 MemberName）：接收者 open(类型)，
    /// 无字节码调用点；展开到的每个目标形参 open
    Vm(u32),
}

#[derive(Clone)]
pub(super) struct Lambda {
    /// 创建点（方法, 偏移）与创建时的克隆上下文：静态实现方法继承之，构造器引用在创建点分配
    pub(super) site: (usize, u32),
    pub(super) ctx: u32,
    pub(super) iface: String,
    /// `altMetafactory` 附加实现的接口（序列化标记 / 标记接口），参与子类型判定
    pub(super) markers: Vec<String>,
    pub(super) sam: String,
    pub(super) imh: MethodHandle,
    /// 捕获实参来源（按 indy 描述符形参位置）
    pub(super) cap: Args,
    /// SAM 与实现方法签名差异处的装箱 / 拆箱适配
    pub(super) adapt: Vec<lambda_adapt::Conv>,
}
