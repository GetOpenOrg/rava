//! 方法内抽象解释（计划 §3.2 控制流视角 / §3.4 异常视角）。
//!
//! 值域按 JVM 槽位建模（long / double 占两槽，第二槽为 `V::Hi`），dup 族因此统一按槽处理。
//! 常量：int / long / null / 字符串 / 类字面量；引用带静态类型（描述符给出）与非空性。
//! 条件分支两侧只在**能证明**条件恒定时剪掉一侧；异常处理器只在 try 区间有可达指令、
//! 且 catch 类型存活（oracle 判定）时进入。
//!
//! 两阶段：先做数据流到不动点（只记状态），再对每个可达基本块用最终入口状态走一遍产出事件——
//! 事件里的实参抽象值是不动点值（反射常量数据流等下游据此读取常量）。
//! 遇到 jsr / ret、栈失衡等无法建模的形态 → 保守模式：全部指令可达、全部值 Top。
//!
//! 引用值携带来源集合（[`Src`]：形参序号 / 产生该值的指令偏移 / 异常处理器 / 字面量），
//! 引擎据此把实参、返回值、字段写入、数组写入精确连到各自的类型节点（值级 VTA），
//! 而不是整个方法共用一个类型集。

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use classfile::descriptor::{parse_field, parse_method, FieldType};
use classfile::{op, Code, Const, Insn, MemberRef, Operand};

pub mod cfg;
mod init;
#[cfg(test)]
mod init_tests;
pub mod ints;
mod lit;
mod narrow;
mod obj;
mod shape;
mod strs;
#[cfg(test)]
mod strs_tests;
#[cfg(test)]
mod tests;
pub use init::InitSum;
pub use lit::{lit_id, lit_str};
pub use obj::Obj;
pub use shape::Shape;
pub use strs::StrKind;

/// 引用值来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Src {
    /// 形参序号（实例方法 0 = this）
    Param(u16),
    /// 产生该值的指令偏移（new / 调用返回 / 字段读 / 数组读 / indy / ldc 句柄等）
    Site(u32),
    /// 异常处理器入口（处理器偏移）
    Catch(u32),
    /// 字符串字面量（与其它值合流后；携带字面量序号，见 [`lit_id`]）
    Str(u32),
}

pub type Srcs = Rc<[Src]>;

fn src1(s: Src) -> Srcs {
    Rc::from([s].as_slice())
}

fn src_union(a: &[Src], b: &[Src]) -> Srcs {
    let mut v: Vec<Src> = a.iter().chain(b.iter()).copied().collect();
    v.sort();
    v.dedup();
    Rc::from(v)
}

#[derive(Clone, Debug, PartialEq)]
pub enum V {
    Top,
    /// long / double 的第二槽
    Hi,
    Int(i32),
    /// int 族常量的小集合（≥ 2 个，见 [`ints`]）
    Ints(Rc<[i32]>),
    /// 奇偶已知的 int（true = 奇）：数组下标奇偶敏感（键值交错数组等）
    Par(bool),
    /// 值未知、但恒等于入口处 int 族形参 i 的值（只经复制保持；参与运算 / 合流即为 Top）：判定形参是否选择分支
    Arg(u16),
    Long(i64),
    Null,
    /// 引用：静态类型（binary name 或数组描述符）+ 是否确定非空 + 来源集合 + 对象身份标签（见 [`Obj`]）
    Ref { ty: Option<Rc<str>>, nonnull: bool, src: Srcs, obj: Option<Rc<Obj>> },
    /// 字符串常量 + 来源集合：本方法 ldc 字面量的来源为 `Src::Str`（本点字面量）；常量格给出的值落到本方法
    /// （形参 / 字段读 / 调用返回的常量）的来源为该读取点，即常量格推不出时同一值的来源；同值合流取来源并。
    /// 值用于折叠；来源决定它是否算本点字面量（[`V::site_lits`]）及合流后的来源——
    /// 常量格的中间态常量与其终态（Top，来源同上）给出同样的来源，引擎的点名与处理顺序无关。
    /// 常量格存储形态（[`V::stripped`]）来源为空
    Str(Rc<str>, Srcs),
    /// 类字面量（ldc class）：值是 Class 对象，携带所指类与 ldc 偏移（合流后以该偏移为来源，引擎在此处给出类镜像）
    Class(Rc<str>, u32),
    /// 符号字段偏移（long）：按名取得的实例字段偏移即字段身份（声明类上的字段键），只经复制 / 字段传递保持，
    /// 参与运算即为 Top；按偏移读写的手写调用点据此只触及该字段。不作为折叠常量导出
    Offset(Rc<MemberRef>),
}

pub const STRING: &str = "java/lang/String";
pub const CLASS: &str = "java/lang/Class";

impl V {
    /// 可空性：Some(true) 恒非 null（实例方法的 this、`new` 结果、catch 值、字面量）；Some(false) 恒 null
    pub(crate) fn nonnull(&self) -> Option<bool> {
        match self {
            V::Null => Some(false),
            V::Ref { nonnull: true, .. } | V::Str(..) | V::Class(..) => Some(true),
            _ => None,
        }
    }

    /// 引用值的静态类型
    pub fn static_type(&self) -> Option<&str> {
        match self {
            V::Ref { ty, .. } => ty.as_deref(),
            V::Str(..) => Some(STRING),
            V::Class(..) => Some(CLASS),
            _ => None,
        }
    }

    fn is_ref(&self) -> bool {
        matches!(self, V::Null | V::Ref { .. } | V::Str(..) | V::Class(..))
    }

    /// 引用值的来源集合（Null 无来源）
    pub fn srcs(&self) -> Srcs {
        match self {
            V::Ref { src, .. } => src.clone(),
            V::Str(_, src) => src.clone(),
            V::Class(_, off) => src1(Src::Site(*off)),
            _ => Rc::from([].as_slice()),
        }
    }

    /// 值可能是的字符串字面量：字面量本身，或合流引用来源里的各个字面量
    pub fn lits(&self) -> Vec<Rc<str>> {
        match self {
            V::Str(s, _) => vec![s.clone()],
            V::Ref { src, .. } => src
                .iter()
                .filter_map(|s| match *s {
                    Src::Str(id) => Some(lit_str(id)),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
    }

    /// 本方法的字面量（ldc，或合流来源里的字面量）：不含常量格给出的值（见 [`V::Str`]）
    pub fn site_lits(&self) -> Vec<Rc<str>> {
        match self {
            V::Str(..) if self.derived_str() => vec![],
            _ => self.lits(),
        }
    }

    /// 本方法的 ldc 字符串字面量（来源只有字面量本身）
    pub fn lit(s: impl Into<Rc<str>>) -> V {
        let s: Rc<str> = s.into();
        let id = lit_id(&s);
        V::Str(s, src1(Src::Str(id)))
    }

    /// 来源不全是本方法字面量的字符串常量（常量格给出的值，或与之同值合流）：按其来源处理，不算本点字面量
    pub fn derived_str(&self) -> bool {
        matches!(self, V::Str(_, src) if src.is_empty() || src.iter().any(|s| !matches!(s, Src::Str(_))))
    }

    /// 同 [`V::lits`]，取字面量序号（见 `lit.rs`）
    pub fn lit_ids(&self) -> Vec<u32> {
        match self {
            V::Str(s, _) => vec![lit_id(s)],
            V::Ref { src, .. } => src
                .iter()
                .filter_map(|s| match *s {
                    Src::Str(id) => Some(id),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
    }

    /// int 值的奇偶（true = 奇）
    pub fn parity(&self) -> Option<bool> {
        match self {
            V::Int(x) => Some(x & 1 != 0),
            V::Ints(s) => s.iter().all(|x| x & 1 == s[0] & 1).then(|| s[0] & 1 != 0),
            V::Par(p) => Some(*p),
            _ => None,
        }
    }

    pub fn join(&self, o: &V) -> V {
        if self == o {
            return self.clone();
        }
        if let (V::Str(a, sa), V::Str(b, sb)) = (self, o) {
            if a == b {
                return V::Str(a.clone(), src_union(sa, sb));
            }
        }
        if self.is_ref() && o.is_ref() {
            let (a, b) = (self.static_type(), o.static_type());
            let ty = match (self, o) {
                (V::Null, _) => b,
                (_, V::Null) => a,
                _ if a == b => a,
                _ => None,
            };
            let nonnull = self.nonnull() == Some(true) && o.nonnull() == Some(true);
            return V::Ref { ty: ty.map(Rc::from), nonnull, src: src_union(&self.srcs(), &o.srcs()), obj: obj::join_obj(self, o) };
        }
        match (self.parity(), o.parity()) {
            (Some(a), Some(b)) if a == b => V::Par(a),
            _ => V::Top,
        }
    }
}

fn ft_name(ft: &FieldType) -> Rc<str> {
    match ft {
        FieldType::Object(c) => Rc::from(c.as_str()),
        other => Rc::from(other.descriptor().as_str()),
    }
}

/// 描述符类型的抽象值（引用 → 带类型、来源为 s 的可空引用；基本类型 → Top）
fn value_of(ft: &FieldType, s: Src) -> V {
    if ft.is_reference() {
        V::Ref { ty: Some(ft_name(ft)), nonnull: false, src: src1(s), obj: None }
    } else {
        match (ft, s) {
            (FieldType::Prim(b'B' | b'C' | b'I' | b'S' | b'Z'), Src::Param(i)) => V::Arg(i),
            _ => V::Top,
        }
    }
}

fn site_ref(ty: &str, nonnull: bool, off: u32) -> V {
    V::Ref { ty: Some(Rc::from(ty)), nonnull, src: src1(Src::Site(off)), obj: None }
}

fn push_typed(stack: &mut Vec<V>, ft: &FieldType, v: V) {
    stack.push(v);
    if ft.slots() == 2 {
        stack.push(V::Hi);
    }
}

/// 数组类型（描述符）的元素类型
pub fn component(arr: &str) -> Option<Rc<str>> {
    let e = arr.strip_prefix('[')?;
    if let Some(c) = e.strip_prefix('L').and_then(|c| c.strip_suffix(';')) {
        Some(Rc::from(c))
    } else if e.starts_with('[') {
        Some(Rc::from(e))
    } else {
        None
    }
}

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
    /// 调用结果（返回值事实 / null→false 纯函数 / 被调方法的返回常量）
    fn invoke_result(&self, opcode: u8, m: &MemberRef, iface: bool, args: &[V]) -> Ret;
    /// 字段读（getstatic / getfield）的常量值；recv = getfield 的接收者
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V>;
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
    /// 是否为类镜像子类型判定（清单 `[facts.reflect] mirror_subtype_tests`，`K.isAssignableFrom(x)` 形态：
    /// 接收者镜像所指类是实参镜像所指类的超类型时为真），见 `narrow.rs`
    fn mirror_subtype_test(&self, _m: &MemberRef) -> bool {
        false
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
    /// 无法建模、按全部可达保守处理
    pub conservative: bool,
    /// 基本块控制流图（拼接链拆段的循环判定用）
    pub cfg: Rc<cfg::Cfg>,
    /// 值未知时直接作 switch 键 / 条件跳转操作数的 int 族形参（按形参序号的位掩码，≥ 64 不计）
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

#[derive(Clone, Debug, PartialEq)]
struct State {
    locals: Vec<V>,
    stack: Vec<V>,
    /// 本帧在全部路径上都已写入的 static final 字段及其值（见 [`Oracle::final_static`]）
    finals: Vec<(MemberRef, V)>,
    /// 已判定非空的 (局部变量槽, final 实例字段)（见 `narrow.rs` 的 final 字段重读）
    nnf: Vec<(usize, MemberRef)>,
    /// 构造器跟踪：`this` 在全部路径上都已写入的实例字段（排序，见 `init.rs`）；不跟踪时恒空
    inits: Vec<MemberRef>,
}

impl State {
    fn join(&mut self, o: &State) -> Result<bool, ()> {
        if self.stack.len() != o.stack.len() || self.locals.len() != o.locals.len() {
            return Err(());
        }
        if !strs::has_builders(self) && !strs::has_builders(o) {
            return Ok(self.join_values(o));
        }
        let before_join = self.clone();
        let mut changed = self.join_values(o);
        changed |= strs::drop_lost_groups(self, &before_join, o);
        Ok(changed)
    }

    fn join_values(&mut self, o: &State) -> bool {
        let mut changed = false;
        let before = self.finals.len();
        self.finals.retain_mut(|(f, a)| match o.finals.iter().find(|(g, _)| g == f) {
            Some((_, b)) => {
                let j = a.join(b);
                if j != *a {
                    *a = j;
                    changed = true;
                }
                true
            }
            None => false,
        });
        changed |= self.finals.len() != before;
        let before = self.nnf.len();
        self.nnf.retain(|x| o.nnf.contains(x));
        changed |= self.nnf.len() != before;
        changed |= init::join(&mut self.inits, &o.inits);
        for (a, b) in self.locals.iter_mut().chain(self.stack.iter_mut()).zip(o.locals.iter().chain(o.stack.iter())) {
            let j = a.join(b);
            if j != *a {
                *a = j;
                changed = true;
            }
        }
        changed
    }
}

/// 一条指令的控制流后继
enum Flow {
    Next,
    /// 条件分支：(目标, 恒走目标 Some(true) / 恒落空 Some(false) / 不定 None)
    Cond(u32, Option<bool>),
    Goto(u32),
    /// switch：可能的目标列表
    Switch(Vec<u32>),
    End,
}

type Step = Result<Flow, ()>;

struct Interp<'a, O: Oracle> {
    oracle: &'a O,
    emit: Option<&'a mut Vec<(u32, Event)>>,
    /// 已作出的「形参不是该类镜像」乐观答复
    assumed: Vec<(u16, String)>,
    /// 已作出的形参镜像字段答复（形参序号）
    field_assumed: Vec<u16>,
    /// 见 [`Analysis::selector_params`]
    selects: u64,
}

impl<O: Oracle> Interp<'_, O> {
    /// 引用相等（if_acmp）：null 性已知；两个类字面量（同名即同一镜像）；类字面量与只来自一个 Class 形参的值——
    /// 形参值集不含该类镜像时不等（记为乐观答复）
    fn select(&mut self, v: &V) {
        if let V::Arg(i) = v {
            if *i < 64 {
                self.selects |= 1 << i;
            }
        }
    }

    fn ref_eq(&mut self, a: &V, b: &V) -> Option<bool> {
        match (a.nonnull(), b.nonnull()) {
            (Some(false), Some(false)) => return Some(true),
            (Some(false), Some(true)) | (Some(true), Some(false)) => return Some(false),
            _ => {}
        }
        let (c, v) = match (a, b) {
            (V::Class(x, _), V::Class(y, _)) => return Some(x == y),
            (V::Class(c, _), v) | (v, V::Class(c, _)) => (c, v),
            _ => return None,
        };
        let V::Ref { src, .. } = v else { return None };
        let [Src::Param(i)] = &src[..] else { return None };
        if self.oracle.param_mirror(*i, c)? {
            return None;
        }
        if !self.assumed.iter().any(|(j, x)| j == i && **x == **c) {
            self.assumed.push((*i, c.to_string()));
        }
        Some(false)
    }
}

impl<O: Oracle> Interp<'_, O> {
    /// 接收者只来自一个 Class 形参时按形参镜像值集读 VM 注入字段（记为乐观答复）
    fn param_mirror_field(&mut self, recv: Option<&V>, f: &MemberRef) -> Option<V> {
        let Some(V::Ref { src, .. }) = recv else { return None };
        let [Src::Param(i)] = &src[..] else { return None };
        let v = self.oracle.param_mirror_field(*i, f)?;
        self.field_assumed.push(*i);
        Some(v)
    }
}

fn pop(st: &mut State) -> Result<V, ()> {
    st.stack.pop().ok_or(())
}

fn popn(st: &mut State, n: usize) -> Result<(), ()> {
    if st.stack.len() < n {
        return Err(());
    }
    st.stack.truncate(st.stack.len() - n);
    Ok(())
}

/// 按描述符弹出实参（逆序），返回按形参顺序的值（long / double 取首槽）
fn pop_args(st: &mut State, params: &[FieldType]) -> Result<Vec<V>, ()> {
    let mut out = Vec::with_capacity(params.len());
    for p in params.iter().rev() {
        if p.slots() == 2 {
            pop(st)?;
        }
        out.push(pop(st)?);
    }
    out.reverse();
    Ok(out)
}

fn int_bin(opc: u8, a: i32, b: i32) -> Option<i32> {
    Some(match opc {
        0x60 => a.wrapping_add(b),
        0x64 => a.wrapping_sub(b),
        0x68 => a.wrapping_mul(b),
        0x6c if b != 0 => a.wrapping_div(b),
        0x70 if b != 0 => a.wrapping_rem(b),
        0x78 => a.wrapping_shl(b as u32 & 31),
        0x7a => a.wrapping_shr(b as u32 & 31),
        0x7c => ((a as u32) >> (b as u32 & 31)) as i32,
        0x7e => a & b,
        0x80 => a | b,
        0x82 => a ^ b,
        _ => return None,
    })
}

/// 至少一侧非常量时的结果奇偶
fn int_bin_parity(opc: u8, a: &V, b: &V) -> Option<bool> {
    let (pa, pb) = (a.parity(), b.parity());
    match opc {
        0x60 | 0x64 | 0x82 => Some(pa? ^ pb?),
        0x68 => match (pa, pb) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        },
        0x78 => match b {
            V::Int(k) if k & 31 != 0 => Some(false),
            V::Int(_) => pa,
            _ => None,
        },
        0x7e => match (pa, pb) {
            (Some(false), _) | (_, Some(false)) => Some(false),
            (Some(true), Some(true)) => Some(true),
            _ => None,
        },
        0x80 => match (pa, pb) {
            (Some(true), _) | (_, Some(true)) => Some(true),
            (Some(false), Some(false)) => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn cond(opc: u8, a: i32, b: i32) -> bool {
    match opc {
        0x99 | 0x9f => a == b,
        0x9a | 0xa0 => a != b,
        0x9b | 0xa1 => a < b,
        0x9c | 0xa2 => a >= b,
        0x9d | 0xa3 => a > b,
        _ => a <= b,
    }
}

impl<'a, O: Oracle> Interp<'a, O> {
    fn ev(&mut self, off: u32, e: Event) {
        if let Some(out) = self.emit.as_deref_mut() {
            out.push((off, e));
        }
    }

    /// 读取点折叠出的常量：登记折叠事件，原样返回
    /// 带对象标签的引用不是可导出的常量：来源换成本读取点，不登记折叠事件
    fn folded(&mut self, opcode: u8, off: u32, v: Option<V>) -> Option<V> {
        let v = v?;
        if let V::Ref { .. } = v {
            return Some(v.rebased(Src::Site(off)));
        }
        self.ev(off, Event::Const { opcode, value: v.stripped() });
        Some(v.rebased(Src::Site(off)))
    }

    /// 构造器返回：新建对象（`Uninit` 标签）的各份拷贝换成构造完成的标签（final 字段常量，推不出则无标签）
    fn constructed(&mut self, s: &mut State, init: &MemberRef, args: &[V]) {
        let Some(recv @ V::Ref { obj: Some(o), .. }) = args.first() else { return };
        if **o != Obj::Uninit {
            return;
        }
        let done = match self.oracle.str_kind(op::INVOKESPECIAL, init, false) {
            Some(StrKind::Init) => parse_method(&init.desc).and_then(|md| strs::init_tag(&md.params, args)),
            _ => self.oracle.construct(init, args),
        };
        let V::Ref { ty, nonnull, src, .. } = recv else { return };
        let v = V::Ref { ty: ty.clone(), nonnull: *nonnull, src: src.clone(), obj: done };
        for x in s.locals.iter_mut().chain(s.stack.iter_mut()) {
            if x == recv {
                *x = v.clone();
            }
        }
    }

    fn step(&mut self, st: &mut State, ins: &Insn) -> Step {
        let opc = ins.opcode;
        let off = ins.offset;
        let s = &mut *st;
        match opc {
            0x00 => {}
            0x01 => s.stack.push(V::Null),
            0x02..=0x08 => s.stack.push(V::Int(opc as i32 - 3)),
            0x09 | 0x0a => {
                s.stack.push(V::Long(opc as i64 - 9));
                s.stack.push(V::Hi);
            }
            0x0b..=0x0d => s.stack.push(V::Top),
            0x0e | 0x0f => {
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x10 | 0x11 => match ins.operand {
                Operand::Int(v) => s.stack.push(V::Int(v)),
                _ => return Err(()),
            },
            op::LDC | op::LDC_W | op::LDC2_W => {
                let Operand::Ldc(c) = &ins.operand else { return Err(()) };
                match c {
                    Const::Int(v) => s.stack.push(V::Int(*v)),
                    Const::Float(_) => s.stack.push(V::Top),
                    Const::Long(v) => {
                        s.stack.push(V::Long(*v));
                        s.stack.push(V::Hi);
                    }
                    Const::Double(_) => {
                        s.stack.push(V::Top);
                        s.stack.push(V::Hi);
                    }
                    Const::String(x) => s.stack.push(V::lit(Rc::from(x.as_str()))),
                    // 含孤立代理项：值不入常量格（格上字符串为 Rust 文本，无法无损表示），按非空 String 站点值
                    Const::StringUtf16(_) => s.stack.push(site_ref(STRING, true, off)),
                    Const::Class(x) => s.stack.push(V::Class(Rc::from(x.as_str()), off)),
                    Const::MethodType(_) => s.stack.push(site_ref("java/lang/invoke/MethodType", true, off)),
                    Const::MethodHandle(_) => s.stack.push(site_ref("java/lang/invoke/MethodHandle", true, off)),
                    Const::Dynamic(_, _, d) => {
                        let ft = parse_field(d).ok_or(())?;
                        push_typed(&mut s.stack, &ft, value_of(&ft, Src::Site(off)));
                    }
                }
                if matches!(c, Const::String(_) | Const::StringUtf16(_) | Const::Class(_) | Const::MethodType(_) | Const::MethodHandle(_) | Const::Dynamic(..)) {
                    self.ev(off, Event::Ldc(c.clone()));
                }
            }
            // xload
            0x15..=0x19 | 0x1a..=0x2d => {
                let (kind, idx) = if opc <= 0x19 {
                    let Operand::Local(i) = ins.operand else { return Err(()) };
                    (opc - 0x15, i as usize)
                } else {
                    ((opc - 0x1a) / 4, ((opc - 0x1a) % 4) as usize)
                };
                let v = s.locals.get(idx).cloned().ok_or(())?;
                s.stack.push(v);
                if kind == 1 || kind == 3 {
                    s.stack.push(V::Hi);
                }
            }
            0x2e | 0x30 | 0x33 | 0x34 | 0x35 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            0x2f | 0x31 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x32 => {
                let index = pop(s)?;
                let arr = pop(s)?;
                let ty = arr.static_type().and_then(component);
                self.ev(off, Event::ArrayLoad { array: arr, index });
                s.stack.push(V::Ref { ty, nonnull: false, src: src1(Src::Site(off)), obj: None });
            }
            // xstore
            0x36..=0x3a | 0x3b..=0x4e => {
                let (kind, idx) = if opc <= 0x3a {
                    let Operand::Local(i) = ins.operand else { return Err(()) };
                    (opc - 0x36, i as usize)
                } else {
                    ((opc - 0x3b) / 4, ((opc - 0x3b) % 4) as usize)
                };
                let wide = kind == 1 || kind == 3;
                if wide {
                    pop(s)?;
                }
                let v = pop(s)?;
                let need = idx + if wide { 2 } else { 1 };
                if s.locals.len() < need {
                    return Err(());
                }
                s.locals[idx] = v;
                if wide {
                    s.locals[idx + 1] = V::Hi;
                }
                if !s.nnf.is_empty() {
                    s.nnf.retain(|(k, _)| *k != idx && !(wide && *k == idx + 1));
                }
            }
            0x4f | 0x51 | 0x54 | 0x55 | 0x56 => popn(s, 3)?,
            0x50 | 0x52 => popn(s, 4)?,
            0x53 => {
                let value = pop(s)?;
                let index = pop(s)?;
                let array = pop(s)?;
                strs::escape(s, &value);
                self.ev(off, Event::ArrayStore { array, index, value });
            }
            0x57 => popn(s, 1)?,
            0x58 => popn(s, 2)?,
            0x59..=0x5f => {
                let n = s.stack.len();
                let need = match opc {
                    0x59 => 1,
                    0x5a | 0x5c | 0x5f => 2,
                    0x5b | 0x5d => 3,
                    _ => 4,
                };
                if n < need {
                    return Err(());
                }
                let top: Vec<V> = s.stack[n - need..].to_vec();
                s.stack.truncate(n - need);
                let seq: Vec<usize> = match opc {
                    0x59 => vec![0, 0],             // dup: a → a a
                    0x5a => vec![1, 0, 1],          // dup_x1: b a → a b a
                    0x5b => vec![2, 0, 1, 2],       // dup_x2: c b a → a c b a
                    0x5c => vec![0, 1, 0, 1],       // dup2: b a → b a b a
                    0x5d => vec![1, 2, 0, 1, 2],    // dup2_x1: c b a → b a c b a
                    0x5e => vec![2, 3, 0, 1, 2, 3], // dup2_x2: d c b a → b a d c b a
                    _ => vec![1, 0],                // swap: b a → a b
                };
                s.stack.extend(seq.into_iter().map(|i| top[i].clone()));
            }
            // int 二元
            0x60 | 0x64 | 0x68 | 0x6c | 0x70 | 0x78 | 0x7a | 0x7c | 0x7e | 0x80 | 0x82 => {
                let b = pop(s)?;
                let a = pop(s)?;
                s.stack.push(match (a, b) {
                    (V::Int(a), V::Int(b)) => int_bin(opc, a, b).map_or(V::Top, V::Int),
                    (a, b) => match ints::map2(&a, &b, |x, y| int_bin(opc, x, y)) {
                        Some(v) => v,
                        None => int_bin_parity(opc, &a, &b).map_or(V::Top, V::Par),
                    },
                });
            }
            // long 二元（含移位：long, int）
            0x61 | 0x65 | 0x69 | 0x6d | 0x71 | 0x7f | 0x81 | 0x83 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x79 | 0x7b | 0x7d => {
                popn(s, 3)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            // float 二元
            0x62 | 0x66 | 0x6a | 0x6e | 0x72 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            // double 二元
            0x63 | 0x67 | 0x6b | 0x6f | 0x73 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x74 => {
                let a = pop(s)?;
                s.stack.push(match a {
                    V::Int(a) => V::Int(a.wrapping_neg()),
                    V::Par(p) => V::Par(p),
                    _ => V::Top,
                });
            }
            0x75 | 0x77 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
                s.stack.push(V::Hi);
            }
            0x76 => {
                pop(s)?;
                s.stack.push(V::Top);
            }
            op::IINC => {
                let Operand::Iinc { index, delta } = ins.operand else { return Err(()) };
                let slot = s.locals.get_mut(index as usize).ok_or(())?;
                *slot = match slot {
                    V::Int(v) => V::Int(v.wrapping_add(delta as i32)),
                    V::Par(p) => V::Par(*p ^ (delta & 1 != 0)),
                    _ => V::Top,
                };
            }
            // 类型转换：(弹出槽数, 压入槽数)
            0x85..=0x93 => {
                let (inn, out) = match opc {
                    0x85 | 0x87 | 0x8c | 0x8d => (1, 2),
                    0x86 | 0x8b | 0x91..=0x93 => (1, 1),
                    0x88 | 0x89 | 0x8e | 0x90 => (2, 1),
                    _ => (2, 2),
                };
                let top = s.stack.len().checked_sub(inn).ok_or(())?;
                let a = s.stack[top].clone();
                s.stack.truncate(top);
                let v = match (opc, a) {
                    (0x85, V::Int(x)) => V::Long(x as i64),
                    (0x88, V::Long(x)) => V::Int(x as i32),
                    (0x91, V::Int(x)) => V::Int(x as i8 as i32),
                    (0x92, V::Int(x)) => V::Int(x as u16 as i32),
                    (0x93, V::Int(x)) => V::Int(x as i16 as i32),
                    _ => V::Top,
                };
                s.stack.push(v);
                if out == 2 {
                    s.stack.push(V::Hi);
                }
            }
            0x94 => {
                pop(s)?;
                let b = pop(s)?;
                pop(s)?;
                let a = pop(s)?;
                s.stack.push(match (a, b) {
                    (V::Long(a), V::Long(b)) => V::Int(a.cmp(&b) as i32),
                    _ => V::Top,
                });
            }
            0x95 | 0x96 => {
                popn(s, 2)?;
                s.stack.push(V::Top);
            }
            0x97 | 0x98 => {
                popn(s, 4)?;
                s.stack.push(V::Top);
            }
            0x99..=0x9e => {
                let a = pop(s)?;
                self.select(&a);
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = ints::decide(&a, &V::Int(0), |x, y| cond(opc, x, y));
                return Ok(Flow::Cond(t, k));
            }
            0x9f..=0xa4 => {
                let b = pop(s)?;
                let a = pop(s)?;
                self.select(&a);
                self.select(&b);
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = ints::decide(&a, &b, |x, y| cond(opc, x, y));
                return Ok(Flow::Cond(t, k));
            }
            0xa5 | 0xa6 => {
                let b = pop(s)?;
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let eq = self.ref_eq(&a, &b);
                return Ok(Flow::Cond(t, eq.map(|e| if opc == 0xa5 { e } else { !e })));
            }
            op::GOTO | op::GOTO_W => {
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                return Ok(Flow::Goto(t));
            }
            op::JSR | op::RET | op::JSR_W => return Err(()),
            op::TABLESWITCH | op::LOOKUPSWITCH => {
                let key = pop(s)?;
                self.select(&key);
                let (default, cases): (u32, Vec<(i32, u32)>) = match &ins.operand {
                    Operand::TableSwitch { default, low, targets, .. } => {
                        (*default, targets.iter().enumerate().map(|(i, t)| (low + i as i32, *t)).collect())
                    }
                    Operand::LookupSwitch { default, pairs } => (*default, pairs.clone()),
                    _ => return Err(()),
                };
                let mut all: Vec<u32> = cases.iter().map(|c| c.1).collect();
                all.push(default);
                all.sort();
                all.dedup();
                if let Some(ks) = ints::members(&key) {
                    let mut ts: Vec<u32> = ks.iter().map(|k| cases.iter().find(|c| c.0 == *k).map_or(default, |c| c.1)).collect();
                    ts.sort();
                    ts.dedup();
                    return Ok(Flow::Switch(ts));
                }
                return Ok(Flow::Switch(all));
            }
            0xb0 => {
                let v = pop(s)?;
                self.ev(off, Event::Return(v));
                return Ok(Flow::End);
            }
            0xac | 0xae => {
                let v = pop(s)?;
                self.ev(off, Event::Return(if opc == 0xac { v } else { V::Top }));
                return Ok(Flow::End);
            }
            0xad | 0xaf => {
                pop(s)?;
                let v = pop(s)?;
                self.ev(off, Event::Return(if opc == 0xad { v } else { V::Top }));
                return Ok(Flow::End);
            }
            op::RETURN => {
                self.ev(off, Event::Return(V::Top));
                return Ok(Flow::End);
            }
            op::GETSTATIC | op::PUTSTATIC | op::GETFIELD | op::PUTFIELD => {
                let Operand::Field(f) = &ins.operand else { return Err(()) };
                let ft = parse_field(&f.desc).ok_or(())?;
                let mut value = None;
                let mut recv = None;
                match opc {
                    op::GETSTATIC => {
                        let own = s.finals.iter().find(|(g, _)| g == f).map(|(_, v)| v.clone());
                        let v = own.or_else(|| self.folded(opc, off, self.oracle.field(opc, f, None))).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    op::PUTSTATIC => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        let v = pop(s)?;
                        strs::escape(s, &v);
                        if self.oracle.final_static(f) {
                            s.finals.retain(|(g, _)| g != f);
                            s.finals.push((f.clone(), v.clone()));
                            strs::escape(s, &v);
                        }
                        value = Some(v);
                    }
                    op::GETFIELD => {
                        recv = Some(pop(s)?);
                        let known = self.oracle.field(opc, f, recv.as_ref()).or_else(|| self.param_mirror_field(recv.as_ref(), f));
                        let v = self.folded(opc, off, known).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    _ => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        value = Some(pop(s)?);
                        recv = Some(pop(s)?);
                        if let Some(v) = &value {
                            strs::escape(s, v);
                        }
                        s.nnf.retain(|(_, g)| g != f);
                    }
                }
                self.ev(off, Event::Field { opcode: opc, mref: f.clone(), recv, value });
            }
            op::INVOKEVIRTUAL | op::INVOKESPECIAL | op::INVOKESTATIC | op::INVOKEINTERFACE => {
                let Operand::Method(m, iface) = &ins.operand else { return Err(()) };
                let md = parse_method(&m.desc).ok_or(())?;
                let mut args = pop_args(s, &md.params)?;
                if opc != op::INVOKESTATIC {
                    let recv = pop(s)?;
                    args.insert(0, recv);
                }
                s.nnf.clear();
                let kind = self.oracle.str_kind(opc, m, *iface);
                let retag = strs::invoke(s, kind, &md.params, &args, opc == op::INVOKESTATIC);
                let r = self.oracle.invoke_result(opc, m, *iface, &args);
                if opc == op::INVOKESPECIAL && m.name == "<init>" {
                    self.constructed(s, m, &args);
                }
                self.ev(off, Event::Invoke { opcode: opc, mref: m.clone(), iface: *iface, args });
                let v = match r {
                    Ret::Never => return Ok(Flow::End),
                    Ret::Value(v) => Some(v),
                    Ret::Unknown => None,
                };
                if let Some(ret) = &md.ret {
                    // 返回常量格的非空引用不带类型：补上声明返回类型
                    let v = match self.folded(opc, off, v) {
                        Some(V::Ref { ty: None, nonnull, src, obj }) => V::Ref { ty: Some(ft_name(ret)), nonnull, src, obj },
                        Some(v) => v,
                        None => value_of(ret, Src::Site(off)),
                    };
                    // 构建器追加 / 取结果：结果换成内容标签（常量结果保持常量）
                    let v = match (retag, &v) {
                        (Some(tag), V::Ref { ty, nonnull, src, obj }) if tag.is_some() || matches!(obj.as_deref(), Some(Obj::Builder { .. })) => {
                            let nonnull = *nonnull || tag.is_some();
                            V::Ref { ty: ty.clone(), nonnull, src: src.clone(), obj: tag }
                        }
                        _ => v,
                    };
                    push_typed(&mut s.stack, ret, v);
                }
            }
            op::INVOKEDYNAMIC => {
                let Operand::InvokeDynamic { bsm, name, desc, .. } = &ins.operand else { return Err(()) };
                let md = parse_method(desc).ok_or(())?;
                let args = pop_args(s, &md.params)?;
                s.nnf.clear();
                for a in &args {
                    strs::escape(s, a);
                }
                if let Some(ret) = &md.ret {
                    push_typed(&mut s.stack, ret, value_of(ret, Src::Site(off)));
                }
                self.ev(off, Event::Indy { bsm: *bsm, name: name.clone(), desc: desc.clone(), args });
            }
            op::NEW => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                // 再次执行分配点：旧对象的构建器标签整组撤掉（新对象独占该组）
                strs::drop_group(s, off);
                s.stack.push(V::Ref { ty: Some(Rc::from(c.as_str())), nonnull: true, src: src1(Src::Site(off)), obj: Some(Rc::new(Obj::Uninit)) });
                self.ev(off, Event::New(c.clone()));
            }
            op::NEWARRAY | op::ANEWARRAY => {
                let empty = pop(s)? == V::Int(0);
                let ty = match &ins.operand {
                    Operand::NewArray(t) => {
                        let c = b"ZCFDBSIJ".get((*t as usize).wrapping_sub(4)).ok_or(())?;
                        format!("[{}", *c as char)
                    }
                    Operand::Class(c) if c.starts_with('[') => format!("[{c}"),
                    Operand::Class(c) => format!("[L{c};"),
                    _ => return Err(()),
                };
                s.stack.push(site_ref(&ty, true, off));
                self.ev(off, Event::NewArray(ty, empty));
            }
            0xbe => {
                pop(s)?;
                s.stack.push(V::Top);
            }
            op::ATHROW => {
                let v = pop(s)?;
                self.ev(off, Event::Throw(v));
                return Ok(Flow::End);
            }
            op::CHECKCAST => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                let v = pop(s)?;
                let (out, input) = match v {
                    V::Null => (V::Null, None),
                    V::Str(..) | V::Class(..) => (v, None),
                    // 数组目标：来源不变（数组类型不参与收窄）
                    V::Ref { nonnull, src, obj, .. } if c.starts_with('[') => (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src, obj }, None),
                    // 类目标：结果以本偏移为来源，跨汇合点仍保留按来源的收窄
                    V::Ref { nonnull, ref obj, .. } => {
                        let obj = obj.clone();
                        (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src: src1(Src::Site(off)), obj }, Some(v))
                    }
                    // 未知值（保守）：来源仍未知
                    other => (other, None),
                };
                s.stack.push(out);
                self.ev(off, Event::CheckCast(c.clone(), input));
            }
            op::INSTANCEOF => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                let v = pop(s)?;
                // 目标类型（非数组）无已实例化子类型时恒为 false：值只可能是 null 或其它类型的对象
                let dead = matches!(v, V::Ref { .. }) && !c.starts_with('[') && !self.oracle.type_live(c);
                s.stack.push(if v == V::Null || dead { V::Int(0) } else { V::Top });
                let input = (matches!(v, V::Ref { .. }) && !c.starts_with('[')).then_some(v);
                self.ev(off, Event::InstanceOf(c.clone(), input));
            }
            0xc2 | 0xc3 => popn(s, 1)?,
            op::MULTIANEWARRAY => {
                let Operand::MultiANewArray(c, dims) = &ins.operand else { return Err(()) };
                popn(s, *dims as usize)?;
                s.stack.push(site_ref(c, true, off));
                self.ev(off, Event::NewArray(c.clone(), false));
            }
            op::IFNULL | op::IFNONNULL => {
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = a.nonnull().map(|nn| if opc == op::IFNULL { !nn } else { nn });
                return Ok(Flow::Cond(t, k));
            }
            _ => return Err(()),
        }
        Ok(Flow::Next)
    }
}

/// 方法入口状态：this + 形参（按描述符类型，来源 = 形参序号）
fn entry_state(owner: &str, desc: &str, is_static: bool, max_locals: u16, param: impl Fn(u16) -> Option<V>) -> Option<State> {
    let md = parse_method(desc)?;
    let mut locals = Vec::with_capacity(max_locals as usize);
    if !is_static {
        // 接收者：求值器给出的非空常量（字符串 / 类字面量）按其值，否则为属主类型的非空引用
        let this = param(0).filter(|v| v.nonnull() == Some(true) && !matches!(v, V::Ref { .. }));
        locals.push(this.map_or_else(|| V::Ref { ty: Some(Rc::from(owner)), nonnull: true, src: src1(Src::Param(0)), obj: None }, |v| v.rebased(Src::Param(0))));
    }
    let base = u16::from(!is_static);
    for (i, p) in md.params.iter().enumerate() {
        let k = base + i as u16;
        locals.push(param(k).map_or_else(|| value_of(p, Src::Param(k)), |v| v.rebased(Src::Param(k))));
        if p.slots() == 2 {
            locals.push(V::Hi);
        }
    }
    if locals.len() > max_locals as usize {
        return None;
    }
    locals.resize(max_locals as usize, V::Top);
    Some(State { locals, stack: Vec::new(), finals: Vec::new(), nnf: Vec::new(), inits: Vec::new() })
}

fn conservative(code: &Code) -> Analysis {
    let mut events = Vec::new();
    for ins in &code.insns {
        // 保守模式：值全部未知，事件照常产出（实参按 Top）
        let e = match (&ins.operand, ins.opcode) {
            (Operand::Method(m, iface), o) => {
                let n = parse_method(&m.desc).map_or(0, |d| d.params.len()) + usize::from(o != op::INVOKESTATIC);
                Some(Event::Invoke { opcode: o, mref: m.clone(), iface: *iface, args: vec![V::Top; n] })
            }
            (Operand::InvokeDynamic { bsm, name, desc, .. }, _) => {
                let n = parse_method(desc).map_or(0, |d| d.params.len());
                Some(Event::Indy { bsm: *bsm, name: name.clone(), desc: desc.clone(), args: vec![V::Top; n] })
            }
            (Operand::Field(f), o) => {
                let recv = matches!(o, op::GETFIELD | op::PUTFIELD).then_some(V::Top);
                Some(Event::Field { opcode: o, mref: f.clone(), recv, value: None })
            }
            (Operand::Class(c), op::NEW) => Some(Event::New(c.clone())),
            (Operand::Class(c), op::ANEWARRAY) => Some(Event::NewArray(format!("[L{c};"), false)),
            (Operand::Class(c), op::CHECKCAST) => Some(Event::CheckCast(c.clone(), None)),
            (Operand::Class(c), op::INSTANCEOF) => Some(Event::InstanceOf(c.clone(), None)),
            (Operand::MultiANewArray(c, _), _) => Some(Event::NewArray(c.clone(), false)),
            (Operand::Ldc(c), _) => Some(Event::Ldc(c.clone())),
            (_, 0x32) => Some(Event::ArrayLoad { array: V::Top, index: V::Top }),
            (_, 0x53) => Some(Event::ArrayStore { array: V::Top, index: V::Top, value: V::Top }),
            (_, op::ATHROW) => Some(Event::Throw(V::Top)),
            (_, 0xac..=0xb1) => Some(Event::Return(V::Top)),
            _ => None,
        };
        if let Some(e) = e {
            events.push((ins.offset, e));
        }
    }
    for h in &code.exception_table {
        events.push((h.handler, Event::Catch(h.catch_type.clone())));
    }
    events.sort_by_key(|e| e.0);
    Analysis { reachable: vec![true; code.insns.len()], events, pending_types: vec![], mirror_assumed: vec![], mirror_field_assumed: vec![], conservative: true, cfg: Rc::new(cfg::Cfg::build(code)), selector_params: 0 }
}

/// 分析一个方法体
pub fn analyze<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O) -> Analysis {
    match run(owner, desc, is_static, code, oracle, false) {
        Some((a, _)) => a,
        None => conservative(code),
    }
}

/// 构造器的确定初始化摘要（见 `init.rs`）；无法建模（保守模式）→ None
pub fn analyze_init<O: Oracle>(owner: &str, desc: &str, code: &Code, oracle: &O) -> Option<InitSum> {
    run(owner, desc, false, code, oracle, true)?.1
}

fn run<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O, track: bool) -> Option<(Analysis, Option<InitSum>)> {
    let insns = &code.insns;
    let n = insns.len();
    // 指令按偏移升序：偏移 → 下标用二分（免逐次分析建表）
    let at = |off: u32| insns.binary_search_by_key(&off, |x| x.offset).ok();

    // 基本块首指令
    let mut leader = vec![false; n];
    if n == 0 {
        return None;
    }
    leader[0] = true;
    for (i, ins) in insns.iter().enumerate() {
        let targets: Vec<u32> = match &ins.operand {
            Operand::Branch(t) => vec![*t],
            Operand::TableSwitch { default, targets, .. } => targets.iter().copied().chain([*default]).collect(),
            Operand::LookupSwitch { default, pairs } => pairs.iter().map(|p| p.1).chain([*default]).collect(),
            _ => vec![],
        };
        for t in targets {
            leader[at(t)?] = true;
        }
        if (!targets_empty(&ins.operand) || classfile::insn::is_terminal(ins.opcode)) && i + 1 < n {
            leader[i + 1] = true;
        }
    }
    for h in &code.exception_table {
        leader[at(h.handler)?] = true;
    }

    let mut entry: BTreeMap<usize, State> = BTreeMap::new();
    entry.insert(0, entry_state(owner, desc, is_static, code.max_locals, |i| oracle.param(i))?);
    let mut work: Vec<usize> = vec![0];
    let mut reachable = vec![false; n];
    let mut handler_on = vec![false; code.exception_table.len()];
    // 各指令所在的 try 区间（处理器下标）；处理器入口局部变量 = 区间内各指令前状态的并集
    let cover: Vec<Vec<usize>> = insns
        .iter()
        .map(|x| {
            code.exception_table
                .iter()
                .enumerate()
                .filter(|(_, h)| x.offset >= h.start && x.offset < h.end)
                .map(|(hi, _)| hi)
                .collect()
        })
        .collect();
    let mut hlocals: Vec<Option<Vec<V>>> = vec![None; code.exception_table.len()];
    let mut interp = Interp { oracle, emit: None, assumed: vec![], field_assumed: vec![], selects: 0 };
    let mut tr = track.then(init::Track::default);

    let merge = |entry: &mut BTreeMap<usize, State>, work: &mut Vec<usize>, i: usize, st: &State| -> Option<()> {
        match entry.get_mut(&i) {
            Some(old) => {
                if old.join(st).ok()? {
                    work.push(i);
                }
            }
            None => {
                entry.insert(i, st.clone());
                work.push(i);
            }
        }
        Some(())
    };

    loop {
        while let Some(l) = work.pop() {
            let mut st = entry[&l].clone();
            let mut i = l;
            loop {
                reachable[i] = true;
                for &hi in &cover[i] {
                    match &mut hlocals[hi] {
                        Some(hl) => {
                            for (a, b) in hl.iter_mut().zip(st.locals.iter()) {
                                *a = a.join(b);
                            }
                        }
                        slot => *slot = Some(st.locals.clone()),
                    }
                }
                let ins = &insns[i];
                if let Some(t) = &mut tr {
                    t.pre(oracle, &mut st, ins, false);
                }
                let fl = interp.step(&mut st, ins).ok()?;
                narrow::final_reread(insns, &leader, i, &mut st);
                match fl {
                    Flow::Next => {
                        if i + 1 >= n {
                            return None;
                        }
                        if leader[i + 1] {
                            merge(&mut entry, &mut work, i + 1, &st)?;
                            break;
                        }
                        i += 1;
                    }
                    Flow::Cond(t, k) => {
                        // instanceof 判定成立的一侧收窄被测局部变量
                        let narrow = narrow::instanceof_narrow(insns, &leader, i, &st)
                            .or_else(|| narrow::mirror_sub_narrow(insns, &leader, i, &st, |m| interp.oracle.mirror_subtype_test(m)));
                        // ifnull / ifnonnull 两侧收窄被测局部变量的可空性；前后缀判定成立一侧收窄形状
                        let nulls = narrow::null_narrow(insns, &leader, i, &st)
                            .or_else(|| strs::affix_narrow(insns, &leader, i, &st, |m| interp.oracle.str_kind(op::INVOKEVIRTUAL, m, false)));
                        let ffield = narrow::final_null_test(insns, &leader, i, |f| interp.oracle.final_field(f));
                        let edge = |taken: bool| {
                            let mut s2 = match (&narrow, &nulls) {
                                (Some(nw), _) => {
                                    let mut s2 = st.clone();
                                    s2.locals[nw.slot] = nw.side(taken).clone();
                                    s2
                                }
                                (None, Some((k, on_taken, on_next))) => {
                                    let mut s2 = st.clone();
                                    s2.locals[*k] = if taken { on_taken.clone() } else { on_next.clone() };
                                    s2
                                }
                                (None, None) => st.clone(),
                            };
                            if let Some((k, f, nn_taken)) = &ffield {
                                if taken == *nn_taken && !s2.nnf.iter().any(|(j, g)| j == k && g == f) {
                                    s2.nnf.push((*k, f.clone()));
                                }
                            }
                            s2
                        };
                        if k != Some(false) {
                            merge(&mut entry, &mut work, at(t)?, &edge(true))?;
                        }
                        if k != Some(true) {
                            if i + 1 >= n {
                                return None;
                            }
                            merge(&mut entry, &mut work, i + 1, &edge(false))?;
                        }
                        break;
                    }
                    Flow::Goto(t) => {
                        merge(&mut entry, &mut work, at(t)?, &st)?;
                        break;
                    }
                    Flow::Switch(ts) => {
                        for t in ts {
                            merge(&mut entry, &mut work, at(t)?, &st)?;
                        }
                        break;
                    }
                    Flow::End => break,
                }
            }
        }
        // 异常处理器：try 区间有可达指令且 catch 类型存活 → 进入（局部变量 = 区间内前状态并集，
        // 栈 = 异常对象）；已进入的处理器随区间状态增长重新合并
        for (hi, h) in code.exception_table.iter().enumerate() {
            let Some(hl) = &hlocals[hi] else { continue };
            if !handler_on[hi] {
                if let Some(ct) = &h.catch_type {
                    if !oracle.type_live(ct) {
                        continue;
                    }
                }
                handler_on[hi] = true;
            }
            let ty: Rc<str> = Rc::from(h.catch_type.as_deref().unwrap_or("java/lang/Throwable"));
            let mut hl = hl.clone();
            strs::strip_builders(&mut hl);
            let st = State {
                locals: hl,
                stack: vec![V::Ref { ty: Some(ty), nonnull: true, src: src1(Src::Catch(h.handler)), obj: None }],
                finals: Vec::new(),
                nnf: Vec::new(),
                inits: Vec::new(),
            };
            merge(&mut entry, &mut work, at(h.handler)?, &st)?;
        }
        if work.is_empty() {
            break;
        }
    }

    // 第二阶段：按不动点入口状态产出事件
    let mut events = Vec::new();
    interp.emit = Some(&mut events);
    for (&l, st0) in &entry {
        let mut st = st0.clone();
        let mut i = l;
        loop {
            let ins = &insns[i];
            if let Some(t) = &mut tr {
                t.pre(oracle, &mut st, ins, true);
            }
            let fl = interp.step(&mut st, ins).ok()?;
            narrow::final_reread(insns, &leader, i, &mut st);
            match fl {
                Flow::Next if i + 1 < n && !leader[i + 1] => {
                    i += 1;
                    continue;
                }
                // 收窄值以事件给出的一侧可达（instanceof 不成立一侧、类镜像子类型判定成立一侧）：发其来源事件
                Flow::Cond(_, k) => {
                    let narrow = narrow::instanceof_narrow(insns, &leader, i, &st)
                        .or_else(|| narrow::mirror_sub_narrow(insns, &leader, i, &st, |m| interp.oracle.mirror_subtype_test(m)));
                    if let Some(nw) = narrow {
                        if k != Some(!nw.event_when) {
                            interp.ev(nw.event.0, nw.event.1);
                        }
                    }
                    break;
                }
                _ => break,
            }
        }
    }
    let mut mirror_assumed = std::mem::take(&mut interp.assumed);
    let mut mirror_field_assumed = std::mem::take(&mut interp.field_assumed);
    mirror_field_assumed.sort();
    mirror_field_assumed.dedup();
    let selector_params = interp.selects;
    drop(interp);
    mirror_assumed.sort();
    let mut pending_types: Vec<String> = insns
        .iter()
        .enumerate()
        .filter(|(i, x)| reachable[*i] && x.opcode == op::INSTANCEOF)
        .filter_map(|(_, x)| match &x.operand {
            Operand::Class(c) if !c.starts_with('[') && !oracle.type_live(c) => Some(c.clone()),
            _ => None,
        })
        .collect();
    for (hi, h) in code.exception_table.iter().enumerate() {
        if handler_on[hi] {
            events.push((h.handler, Event::Catch(h.catch_type.clone())));
        } else if let Some(ct) = &h.catch_type {
            let live_range = insns.iter().enumerate().any(|(i, x)| reachable[i] && x.offset >= h.start && x.offset < h.end);
            if live_range {
                pending_types.push(ct.clone());
            }
        }
    }
    events.sort_by_key(|e| e.0);
    pending_types.sort();
    pending_types.dedup();
    let a = Analysis { reachable, events, pending_types, mirror_assumed, mirror_field_assumed, conservative: false, cfg: Rc::new(cfg::Cfg::build(code)), selector_params };
    Some((a, tr.map(init::Track::finish)))
}

fn targets_empty(o: &Operand) -> bool {
    !matches!(o, Operand::Branch(_) | Operand::TableSwitch { .. } | Operand::LookupSwitch { .. })
}
