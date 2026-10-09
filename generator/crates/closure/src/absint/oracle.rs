//! 分析的外部接口：查询（[`Oracle`]）与产出（[`Event`] / [`Analysis`]）。

use super::*;

/// 分析所需的外部查询（事实、常量静态字段、catch 类型存活）
/// 调用结果
pub enum Ret {
    /// 值未知
    Unknown,
    /// 恒为该常量
    Value(V),
    /// 尚无返回路径（乐观假设：被调方法还没算出返回值，调用之后暂不可达）
    Never,
}

pub trait Oracle {
    /// 调用结果（返回值事实 / null→false 纯函数 / 被调方法的返回常量）；off = 调用指令偏移
    fn invoke_result(&self, opcode: u8, off: u32, m: &MemberRef, iface: bool, args: &[V]) -> Ret;
    /// 字段读（getstatic / getfield）的常量值；recv = getfield 的接收者
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V>;
    /// getfield 的结果：缺省取 [`Self::field`]；Never = 乐观假设下尚无可读到的值（按对象读的 ⊥，见引擎 `obj_fields.rs`），
    /// 读取之后暂不可达
    fn getfield(&self, f: &MemberRef, recv: Option<&V>) -> Ret {
        self.field(op::GETFIELD, f, recv).map_or(Ret::Unknown, Ret::Value)
    }
    /// `new C` 的对象经构造器 init（实参含接收者）完成后的身份标签（final 字段常量）
    fn construct(&self, _init: &MemberRef, _args: &[V]) -> Option<Rc<Obj>> {
        None
    }
    /// 形参的常量值（全部调用点传入同一常量；序号含实例方法的 this 槽）
    fn param(&self, _i: u16) -> Option<V> {
        None
    }
    /// 类型是否可能有实例（有已实例化的子类型）：catch 类型能否被抛出、instanceof 能否为真
    fn type_live(&self, ty: &str) -> bool;
    /// 字段是否为字节码可见的 static final 字段：putstatic 只能在声明类的 `<clinit>` 中成功执行，
    /// 同一帧内写入之后的 getstatic 必然读到写入值（中途的调用不可能再写它）
    fn final_static(&self, _f: &MemberRef) -> bool {
        false
    }
    /// 形参 i（Class 类型）能否是类 cls 的类镜像：Some(false) = 值集已知且不含（乐观答复，值集增长时由引擎重分析，
    /// 见 [`Analysis::mirror_assumed`]）；None = 未知
    fn param_mirror(&self, _i: u16, _cls: &str) -> Option<bool> {
        None
    }
    /// 形参 i（Class 类型）上读 VM 注入的接收者状态字段（清单 `[vm_state.field_hooks]` 的接收者钩子字段）：
    /// 值集已知且其中每个类镜像的该字段都由类的事实定出同一值时为该值（乐观答复，值集增长时由引擎重分析，
    /// 见 [`Analysis::mirror_field_assumed`]）；None = 未知
    fn param_mirror_field(&self, _i: u16, _f: &MemberRef) -> Option<V> {
        None
    }
    /// 形参 i（Class 类型）为接收者调用 m：值集已知且其中每个类镜像上 m 的结果由类的事实定出时为该结果
    /// （方法体为接收者钩子字段的平凡取值时按 [`Oracle::param_mirror_field`] 读该字段），乐观答复同 [`Oracle::param_mirror_field`]
    /// （记入 [`Analysis::mirror_field_assumed`]）；None = 未知
    fn param_mirror_call(&self, _i: u16, _m: &MemberRef) -> Option<V> {
        None
    }
    /// 偏移 off 处调用产出的 Class 值（如 @CallerSensitive 方法体内取调用者类）为接收者调用 m：该值的类镜像值集已知、
    /// 且每个镜像上 m 的结果由类的事实定出时为该结果，乐观答复记入 [`Analysis::site_mirror_assumed`]；None = 未知
    fn site_mirror_call(&self, _off: u32, _m: &MemberRef) -> Option<V> {
        None
    }
    /// 是否为类镜像子类型判定（清单 `[facts.reflect] mirror_subtype_tests`，`K.isAssignableFrom(x)` 形态：
    /// 接收者镜像所指类是实参镜像所指类的超类型时为真），见 `narrow.rs`
    fn mirror_subtype_test(&self, _m: &MemberRef) -> bool {
        false
    }
    /// 是否为键类的键读取方法（清单 `[facts.keyed_lookups]` 的 `getters`）：Some((键类, 键是否不区分大小写))，见 `narrow.rs`
    fn key_getter(&self, _m: &MemberRef) -> Option<(String, bool)> {
        None
    }
    /// 是否为字符串相等判定（清单 `[facts] value_equals` / `string_ops` 的 `equals_ignore_case`）：Some(是否不区分大小写)
    fn string_equality(&self, _m: &MemberRef) -> Option<bool> {
        None
    }
    /// 清单字符串操作的种类（构建器新建 / 追加 / 取结果、前后缀判定），见 `strs.rs`
    fn str_kind(&self, _opcode: u8, _m: &MemberRef, _iface: bool) -> Option<StrKind> {
        None
    }
    /// 字段是否为 final 实例字段，见 `narrow.rs` 的 final 字段重读
    fn final_field(&self, _f: &MemberRef) -> bool {
        false
    }
    /// 实例字段的解析后声明键（构造器确定初始化用，见 `init.rs`；None = 不跟踪）
    fn init_key(&self, _f: &MemberRef) -> Option<MemberRef> {
        None
    }
    /// 被委托 / 超类构造器的确定初始化摘要（None = 无法分析，按交出 `this` 处理）
    fn init_sum(&self, _init: &MemberRef) -> Option<Rc<InitSum>> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// invokevirtual / special / static / interface；args 含接收者
    Invoke { opcode: u8, mref: MemberRef, iface: bool, args: Vec<V> },
    Indy { bsm: u16, name: String, desc: String, args: Vec<V> },
    New(String),
    /// 数组分配（数组类型描述符；长度恒为 0）
    NewArray(String, bool),
    /// 字段访问；`recv` 为实例字段的接收者（static 为 None），`value` 为写入值
    Field { opcode: u8, mref: MemberRef, recv: Option<V>, value: Option<V> },
    Ldc(Const),
    /// 引用类型转换；非数组目标带输入值（结果以本偏移为来源，引擎按目标类型收窄）
    CheckCast(String, Option<V>),
    /// 类型测试；非数组目标带输入值（判定成立一侧的收窄值以本偏移为来源，见 `narrow.rs`）
    InstanceOf(String, Option<V>),
    /// `aload; instanceof C; ifeq/ifne` 判定不成立一侧的收窄值（发在条件跳转指令偏移，该偏移即其来源）：
    /// 输入值中 ⊄ C 的部分（含 null），见 `narrow.rs`
    NotInstance(String, V),
    /// `ldc K; aload; <类镜像子类型判定>; ifeq/ifne` 判定成立一侧的收窄值（发在条件跳转指令偏移，该偏移即其类型流节点）：
    /// 输入值的类镜像中所指类 ⊂ K 者，见 `narrow.rs`
    MirrorSub(String, V),
    /// `x.<键读取>().equals(name)`（或两侧互换 / 不区分大小写）判定成立一侧的收窄值（发在条件跳转指令偏移，该偏移即其
    /// 类型流节点）：输入值 x 中键类 `kc` 子类型的对象只取键可能等于 name 者，见 `narrow.rs`
    KeyTest { kc: String, fold: bool, input: V, name: V },
    ArrayLoad { array: V, index: V },
    ArrayStore { array: V, index: V, value: V },
    Throw(V),
    /// 返回指令及返回值（ireturn / lreturn / areturn 取栈顶值；freturn / dreturn / void return 为 Top）。
    /// void return 也发：「有可达的返回点」是乐观阶段判定被调方法会返回的依据
    Return(V),
    /// 进入的异常处理器的 catch 类型（None = finally）
    Catch(Option<String>),
    /// 读取点（getfield / getstatic / invoke 指令）的结果被折叠为常量
    Const { opcode: u8, value: V },
}

pub struct Analysis {
    /// 按指令下标
    pub reachable: Vec<bool>,
    /// (指令偏移, 事件)
    pub events: Vec<(u32, Event)>,
    /// 按「尚无实例」处理的类型：try 区间可达的未进入处理器的 catch 类型、可达 instanceof 的目标类型——类型存活后需重分析
    pub pending_types: Vec<String>,
    /// 按「形参 i 不是类 c 的镜像」折叠的引用比较（形参序号, 类）——形参值集增长后需重分析
    pub mirror_assumed: Vec<(u16, String)>,
    /// 按形参镜像值集折叠了 VM 注入字段读的形参序号（[`Oracle::param_mirror_field`]）——形参值集增长后需重分析
    pub mirror_field_assumed: Vec<u16>,
    /// 按调用点产出的类镜像值集折叠了实例调用（[`Oracle::site_mirror_call`]）——该值集增长后需重分析
    pub site_mirror_assumed: bool,
    /// 无法建模、按全部可达保守处理
    pub conservative: bool,
    /// 基本块控制流图（拼接链拆段的循环判定用）
    pub cfg: Rc<cfg::Cfg>,
    /// 选择子形参（按形参序号的位掩码，≥ 64 不计）：值未知时直接作 switch 键 / 条件跳转操作数的 int 族形参，
    /// 及可空性未知时直接作 ifnull / ifnonnull 操作数的引用形参（调用点传 null 常量即剪去一支）
    pub selector_params: u64,
}

impl Analysis {
    /// 返回值只来自形参（恒等 / 透传方法，如 requireNonNull）：返回这些形参序号。
    /// 调用点据此把实参直接接到结果，不经上下文无关的返回节点汇合
    pub fn returned_params(&self) -> Option<Vec<u16>> {
        if self.conservative {
            return None;
        }
        let mut ps: BTreeSet<u16> = BTreeSet::new();
        let mut any = false;
        for (_, e) in &self.events {
            let Event::Return(v) = e else { continue };
            any = true;
            match v {
                V::Null => {}
                V::Ref { .. } => {
                    for s in v.srcs().iter() {
                        let Src::Param(i) = s else { return None };
                        ps.insert(*i);
                    }
                }
                _ => return None,
            }
        }
        any.then(|| ps.into_iter().collect())
    }
}
