//! 具体求值器的状态：值、非正常完成、堆对象 / 字段 / 方法信息的表示与 `Vm` 本体（堆与字段访问见 `vm_heap.rs`，
//! 字符串 / 类镜像驻留与方法解析见 `vm_link.rs`）。
//!
//! 堆对象带纪元：0 = 静态映像（`<clinit>` 具体执行时分配，跨求值共享），k = 第 k 次求值的分配。
//! 求值只能改写本纪元的对象；映像对象只有清单声明的内存缓存字段可写（写入记撤销日志，求值后复原），
//! 其余写入映像即求值失败（共享状态被改写，轨迹不再只取决于实参）。

use resolve::hierarchy::MethodSite;

use super::*;

/// 单次求值的指令步数上限
pub(super) const STEP_LIMIT: u64 = 4_000_000;
/// 调用深度上限
pub(super) const DEPTH_LIMIT: usize = 400;

/// 具体值（long / double 在操作数栈上占一项，局部变量表里占两槽，第二槽为 `N`）
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum CV {
    I(i32),
    J(i64),
    F(f32),
    D(f64),
    N,
    R(u32),
    /// 引导求值的污点标量（宿主标量及其派生值）：表达式表下标（`bj.taint`）、类型（`b'I'` / `b'J'`）。
    /// 构建期只知其区间；写入映像的位置物化为启动重算槽（concrete/taint.rs）
    T(u32, u8),
}

impl CV {
    pub(super) fn wide(self) -> bool {
        matches!(self, CV::J(_) | CV::D(_) | CV::T(_, b'J'))
    }
    pub(super) fn i(self) -> R<i32> {
        match self {
            CV::I(v) => Ok(v),
            CV::T(..) => defer("延迟值参与求值：宿主标量（污点）取具体值"),
            v => fail(format!("期望 int：{v:?}")),
        }
    }
    pub(super) fn j(self) -> R<i64> {
        match self {
            CV::J(v) => Ok(v),
            CV::T(..) => defer("延迟值参与求值：宿主标量（污点）取具体值"),
            v => fail(format!("期望 long：{v:?}")),
        }
    }
    pub(super) fn f(self) -> R<f32> {
        match self {
            CV::F(v) => Ok(v),
            v => fail(format!("期望 float：{v:?}")),
        }
    }
    pub(super) fn d(self) -> R<f64> {
        match self {
            CV::D(v) => Ok(v),
            v => fail(format!("期望 double：{v:?}")),
        }
    }
    /// 引用（null = None）
    pub(super) fn r(self) -> R<Option<u32>> {
        match self {
            CV::N => Ok(None),
            CV::R(o) => Ok(Some(o)),
            v => fail(format!("期望引用：{v:?}")),
        }
    }
    /// 非空引用（null 即空指针隐式异常）
    pub(super) fn obj(self) -> R<u32> {
        self.r()?.map_or_else(|| implicit("null"), Ok)
    }
    /// 描述符的缺省值
    pub(super) fn zero(desc: &str) -> CV {
        match desc.as_bytes().first() {
            Some(b'J') => CV::J(0),
            Some(b'F') => CV::F(0.0),
            Some(b'D') => CV::D(0.0),
            Some(b'L' | b'[') => CV::N,
            _ => CV::I(0),
        }
    }
}

/// 非正常完成：抛出 Java 异常（对象）/ 求值失败（不可建模，整次回退抽象调用边）
#[derive(Debug)]
pub(super) enum Flow {
    Throw(u32),
    /// 隐式异常（种类见清单 `[concrete.implicit]`）：由解释循环分配异常对象后按 `Throw` 处理
    Implicit(&'static str),
    Fail(String),
    /// 引导求值：宿主相关值（延迟值）参与求值——内容被读、身份被比较、取值改变控制流。
    /// 不被异常处理器捕获；在调用序列的根帧残差化为运行期重放，在 `<clinit>` 边界使该类转为运行期初始化
    Defer(String),
}

pub(super) type R<T> = Result<T, Flow>;

pub(super) fn defer<T>(why: impl Into<String>) -> R<T> {
    Err(Flow::Defer(why.into()))
}

pub(super) fn implicit<T>(kind: &'static str) -> R<T> {
    Err(Flow::Implicit(kind))
}

pub(super) fn fail<T>(why: impl Into<String>) -> R<T> {
    Err(Flow::Fail(why.into()))
}

/// lambda 对象（LambdaMetafactory 产物）
#[derive(Debug)]
pub(in crate::engine) struct Lam {
    pub iface: String,
    pub markers: Vec<String>,
    pub sam: String,
    pub imp: classfile::MethodHandle,
    /// LambdaMetafactory 静态实参（物化为抽象 lambda 时计算适配表）
    pub bargs: Rc<[Const]>,
    /// indy 调用点描述符（形参为捕获值类型）
    pub desc: Rc<str>,
    pub captured: Vec<CV>,
}

pub(super) enum Body {
    /// 实例字段（字段键 → 值；缺席 = 缺省值）
    Inst(Vec<(u32, CV)>),
    Arr(Vec<CV>),
    Lam(Rc<Lam>),
}

pub(super) struct HObj {
    /// binary name / 数组描述符（lambda 为函数式接口名）
    pub ty: Rc<str>,
    pub epoch: u32,
    pub body: Body,
}

/// 已解析字段
pub(super) struct FRes {
    pub key: u32,
    pub decl: Rc<str>,
    pub name: String,
    pub desc: String,
    pub fin: bool,
    pub memo: bool,
    pub constant: Option<classfile::Const>,
}

impl FRes {
    pub(super) fn mref(&self) -> MemberRef {
        MemberRef { owner: self.decl.to_string(), name: self.name.clone(), desc: self.desc.clone() }
    }
}

/// 可执行方法
pub(super) struct MInfo {
    pub key: MemberRef,
    pub site: MethodSite,
    /// 指令偏移 → 下标
    pub index: HashMap<u32, usize>,
    /// 白名单 native 操作（`[concrete.natives]`）
    pub op: Option<String>,
    /// 按字节码执行
    pub bytecode: bool,
}

impl MInfo {
    pub(super) fn code(&self) -> &classfile::Code {
        self.site.method().code.as_ref().expect("MInfo 只为有字节码的方法建立")
    }
}

#[derive(Clone)]
pub(super) enum Init {
    Running,
    Done,
    Failed(Rc<str>),
}

/// 实例字段写入值的常量格投影（物化时并入字段值集，见 concrete.rs）
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Put {
    Int(i32),
    Long(i64),
    Null,
    /// 内容可读的字符串（String 字段的写入名字并入字段字符串槽，见 apply.rs）
    Str(Rc<str>),
    /// 非空引用 / 浮点
    Other,
}

/// 一次求值的轨迹（只记求值纪元内执行的字节码；`<clinit>` 由抽象分析的类初始化覆盖）
#[derive(Default)]
pub(super) struct Trace {
    /// 执行过的指令偏移（按方法）
    pub pcs: BTreeMap<MemberRef, BTreeSet<u32>>,
    /// 调用：(调用方, 偏移) → 实际执行的目标
    pub calls: BTreeMap<(MemberRef, u32), BTreeSet<MemberRef>>,
    /// 实例字段写入（声明字段, 值）
    pub puts: BTreeMap<MemberRef, Vec<Put>>,
    /// 求值中触发初始化的类
    pub inited: BTreeSet<String>,
    /// 按白名单操作执行的手写承载方法
    pub natives: BTreeSet<MemberRef>,
    /// 写入映像（内存缓存字段 / 静态字段）的值：撤销后对程序仍可见，物化进字段值集
    pub memo_vals: Vec<(MemberRef, CV)>,
}

pub(super) struct Vm {
    pub heap: Vec<HObj>,
    pub epoch: u32,
    /// 正在执行 `<clinit>` 的嵌套层数（> 0 时分配进映像、不记轨迹）
    pub image: u32,
    pub statics: HashMap<u32, CV>,
    pub init: HashMap<Rc<str>, Init>,
    /// 静态状态由 VM / 手写层承载的类（`<clinit>` 操作名 `opaque`）
    pub opaque: HashSet<Rc<str>>,
    /// 完成初始化的类（按完成次序）/ 正在执行的 `<clinit>` 开始时的堆大小 / 映像纪元内改写既有映像对象的次数
    pub done_log: Vec<Rc<str>>,
    pub clinit_floor: Vec<usize>,
    pub foreign: u64,
    pub fkeys: HashMap<String, u32>,
    /// 字段键 → (声明类, 字段名)
    pub fnames: Vec<(Rc<str>, Rc<str>)>,
    /// 不可变的映像数组（字符串内容）：求值纪元内可读
    pub frozen: HashSet<u32>,
    /// 类型 → 是否发布后不再改写（`[concrete] stable_types`）
    pub stable_ty: HashMap<Rc<str>, bool>,
    /// 包 → 包内类名（静态字段写入点扫描，concrete/stable.rs）
    pub pkgs: Option<HashMap<String, Vec<String>>>,
    pub fres: HashMap<MemberRef, Option<Rc<FRes>>>,
    pub minfo: HashMap<MemberRef, Rc<MInfo>>,
    pub mres: HashMap<(MemberRef, bool), Option<MethodSite>>,
    pub sel: HashMap<(Rc<str>, MemberRef), Option<MethodSite>>,
    pub strings: HashMap<Vec<u16>, u32>,
    pub mirrors: HashMap<Rc<str>, u32>,
    pub mirror_of: HashMap<u32, Rc<str>>,
    /// VM 持有的单例对象（操作 `vm_singleton`，按类型）
    pub singletons: HashMap<Rc<str>, u32>,
    /// 映像实例对象 → 首个持有它的不变静态字段（类初始化写入的 final / 只在 `<clinit>` 写入的字段）：
    /// 物化时以该静态字段的抽象值代表（抽象分析对 `<clinit>` 的建模给出同一对象）
    pub image_roots: HashMap<u32, MemberRef>,
    pub ihash: HashMap<u32, i32>,
    pub steps: u64,
    /// 调用栈（调用方类查询）
    pub frames: Vec<MemberRef>,
    pub trace: Trace,
    /// 内存缓存字段写入的撤销日志：(对象 / u32::MAX = 静态, 字段键, 原值)
    pub undo: Vec<(u32, u32, Option<CV>)>,
    /// 构建期引导求值：全部分配与写入永久进映像，可变静态可读，跨类静态写入放行
    pub boot: bool,
    pub step_limit: u64,
    /// 宿主相关值（`@deferred`）的字符串内容数组 → 属性名：引导求值读到其内容即「延迟值参与求值」
    pub deferred: HashMap<u32, Rc<str>>,
    /// 宿主相关内容数组的运行期来源（native 键，结果数组下标）：见 `image::IObj::host`
    pub host_src: HashMap<u32, (Rc<str>, Option<u32>)>,
    /// 首个失败点的调用栈（引导求值诊断）
    pub fail_frames: Option<Vec<String>>,
    /// 最近一次隐式异常的调用栈（引导求值诊断）
    pub throw_frames: Option<Vec<String>>,
    /// VM 构造的引导对象（`[concrete.boot] objects`）
    pub boot_objs: Vec<u32>,
    /// VM 原生单元（名字, 位模式）
    pub cells: Vec<(Rc<str>, i64)>,
    /// VM 侧状态登记计数（操作 `vm_record`）
    pub vm_tables: BTreeMap<String, usize>,
    /// 包（内部形式）→ 模块对象（defineModule0 登记）
    pub pkg_module: HashMap<Rc<str>, u32>,
    pub base_module: Option<u32>,
    /// VM 模块表（defineModule0 次序）：（模块, 定义加载器, open, 位置, 包）
    pub modules: Vec<(u32, CV, bool, Option<String>, Vec<Rc<str>>)>,
    /// 引导求值的写入日志、脏位置与残差记录（concrete/journal.rs）
    pub bj: super::journal::Journal,
    /// 构建期初始化扩展（引导映像导出之后，concrete/ext_init.rs）
    pub ext: Option<Box<super::ext_init::Ext>>,
}

impl Vm {
    pub(super) fn new() -> Self {
        Vm {
            heap: Vec::new(),
            epoch: 0,
            image: 0,
            statics: HashMap::default(),
            init: HashMap::default(),
            opaque: HashSet::default(),
            done_log: Vec::new(),
            clinit_floor: Vec::new(),
            foreign: 0,
            fkeys: HashMap::default(),
            fnames: Vec::new(),
            frozen: HashSet::default(),
            stable_ty: HashMap::default(),
            pkgs: None,
            fres: HashMap::default(),
            minfo: HashMap::default(),
            mres: HashMap::default(),
            sel: HashMap::default(),
            strings: HashMap::default(),
            mirrors: HashMap::default(),
            mirror_of: HashMap::default(),
            singletons: HashMap::default(),
            image_roots: HashMap::default(),
            ihash: HashMap::default(),
            steps: 0,
            frames: Vec::new(),
            trace: Trace::default(),
            undo: Vec::new(),
            boot: false,
            step_limit: STEP_LIMIT,
            deferred: HashMap::default(),
            host_src: HashMap::default(),
            fail_frames: None,
            throw_frames: None,
            boot_objs: Vec::new(),
            cells: Vec::new(),
            vm_tables: BTreeMap::default(),
            pkg_module: HashMap::default(),
            base_module: None,
            modules: Vec::new(),
            bj: Default::default(),
            ext: None,
        }
    }

    pub(super) fn tracing(&self) -> bool {
        self.image == 0 && self.epoch > 0
    }

    pub(super) fn cur_epoch(&self) -> u32 {
        if self.image > 0 {
            0
        } else {
            self.epoch
        }
    }
}

/// 求值环境：类层次、清单与方法承载判定
pub(super) struct Env<'e, 'a> {
    pub ctx: &'e Ctx<'a>,
    pub cp: &'a ClassPath,
}

impl<'e, 'a> Env<'e, 'a> {
    pub(super) fn h(&self) -> &'a Hierarchy<'a> {
        self.ctx.h
    }
    pub(super) fn man(&self) -> &'a Manifest {
        self.ctx.man
    }
    pub(super) fn cfg(&self) -> &'a crate::manifest::ConcreteCfg {
        &self.ctx.man.concrete
    }
}
