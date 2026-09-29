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

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use classfile::descriptor::{parse_field, parse_method, FieldType};
use classfile::{op, Code, Const, Insn, MemberRef, Operand};

/// 引用值来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Src {
    /// 形参序号（实例方法 0 = this）
    Param(u16),
    /// 产生该值的指令偏移（new / 调用返回 / 字段读 / 数组读 / indy / ldc 句柄等）
    Site(u32),
    /// 异常处理器入口（处理器偏移）
    Catch(u32),
    /// 字符串字面量（与其它值合流后）
    Str,
    /// 类字面量（与其它值合流后）
    Class,
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
    /// 奇偶已知的 int（true = 奇）：数组下标奇偶敏感（键值交错数组等）
    Par(bool),
    Long(i64),
    Null,
    /// 引用：静态类型（binary name 或数组描述符）+ 是否确定非空 + 来源集合
    Ref { ty: Option<Rc<str>>, nonnull: bool, src: Srcs },
    Str(Rc<str>),
    /// 类字面量（ldc class）：值是 Class 对象，携带所指类
    Class(Rc<str>),
}

pub const STRING: &str = "java/lang/String";
pub const CLASS: &str = "java/lang/Class";

impl V {
    fn nonnull(&self) -> Option<bool> {
        match self {
            V::Null => Some(false),
            V::Ref { nonnull: true, .. } | V::Str(_) | V::Class(_) => Some(true),
            _ => None,
        }
    }

    /// 引用值的静态类型
    pub fn static_type(&self) -> Option<&str> {
        match self {
            V::Ref { ty, .. } => ty.as_deref(),
            V::Str(_) => Some(STRING),
            V::Class(_) => Some(CLASS),
            _ => None,
        }
    }

    fn is_ref(&self) -> bool {
        matches!(self, V::Null | V::Ref { .. } | V::Str(_) | V::Class(_))
    }

    /// 引用值的来源集合（Null 无来源）
    pub fn srcs(&self) -> Srcs {
        match self {
            V::Ref { src, .. } => src.clone(),
            V::Str(_) => src1(Src::Str),
            V::Class(_) => src1(Src::Class),
            _ => Rc::from([].as_slice()),
        }
    }

    /// int 值的奇偶（true = 奇）
    pub fn parity(&self) -> Option<bool> {
        match self {
            V::Int(x) => Some(x & 1 != 0),
            V::Par(p) => Some(*p),
            _ => None,
        }
    }

    fn join(&self, o: &V) -> V {
        if self == o {
            return self.clone();
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
            return V::Ref { ty: ty.map(Rc::from), nonnull, src: src_union(&self.srcs(), &o.srcs()) };
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
        V::Ref { ty: Some(ft_name(ft)), nonnull: false, src: src1(s) }
    } else {
        V::Top
    }
}

fn site_ref(ty: &str, nonnull: bool, off: u32) -> V {
    V::Ref { ty: Some(Rc::from(ty)), nonnull, src: src1(Src::Site(off)) }
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
    /// 字段读（getstatic / getfield）的常量值
    fn field(&self, opcode: u8, f: &MemberRef) -> Option<V>;
    /// 形参的常量值（全部调用点传入同一常量；序号含实例方法的 this 槽）
    fn param(&self, _i: u16) -> Option<V> {
        None
    }
    /// catch 类型是否可能被抛出（有已实例化的子类型）
    fn catch_live(&self, ty: &str) -> bool;
}

#[derive(Debug, Clone)]
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
    InstanceOf(String),
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
    /// try 区间可达、但 catch 类型尚不存活的处理器（catch 类型）——类型存活后需重分析
    pub pending_catch: Vec<String>,
    /// 无法建模、按全部可达保守处理
    pub conservative: bool,
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
}

impl State {
    fn join(&mut self, o: &State) -> Result<bool, ()> {
        if self.stack.len() != o.stack.len() || self.locals.len() != o.locals.len() {
            return Err(());
        }
        let mut changed = false;
        for (a, b) in self.locals.iter_mut().chain(self.stack.iter_mut()).zip(o.locals.iter().chain(o.stack.iter())) {
            let j = a.join(b);
            if j != *a {
                *a = j;
                changed = true;
            }
        }
        Ok(changed)
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
    fn folded(&mut self, opcode: u8, off: u32, v: Option<V>) -> Option<V> {
        let v = v?;
        self.ev(off, Event::Const { opcode, value: v.clone() });
        Some(v)
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
                    Const::String(x) => s.stack.push(V::Str(Rc::from(x.as_str()))),
                    Const::Class(x) => s.stack.push(V::Class(Rc::from(x.as_str()))),
                    Const::MethodType(_) => s.stack.push(site_ref("java/lang/invoke/MethodType", true, off)),
                    Const::MethodHandle(_) => s.stack.push(site_ref("java/lang/invoke/MethodHandle", true, off)),
                    Const::Dynamic(_, _, d) => {
                        let ft = parse_field(d).ok_or(())?;
                        push_typed(&mut s.stack, &ft, value_of(&ft, Src::Site(off)));
                    }
                }
                if matches!(c, Const::String(_) | Const::Class(_) | Const::MethodType(_) | Const::MethodHandle(_) | Const::Dynamic(..)) {
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
                s.stack.push(V::Ref { ty, nonnull: false, src: src1(Src::Site(off)) });
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
            }
            0x4f | 0x51 | 0x54 | 0x55 | 0x56 => popn(s, 3)?,
            0x50 | 0x52 => popn(s, 4)?,
            0x53 => {
                let value = pop(s)?;
                let index = pop(s)?;
                let array = pop(s)?;
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
                    (a, b) => int_bin_parity(opc, &a, &b).map_or(V::Top, V::Par),
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
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = if let V::Int(a) = a { Some(cond(opc, a, 0)) } else { None };
                return Ok(Flow::Cond(t, k));
            }
            0x9f..=0xa4 => {
                let b = pop(s)?;
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let k = match (a, b) {
                    (V::Int(a), V::Int(b)) => Some(cond(opc, a, b)),
                    _ => None,
                };
                return Ok(Flow::Cond(t, k));
            }
            0xa5 | 0xa6 => {
                let b = pop(s)?;
                let a = pop(s)?;
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                let eq = match (a.nonnull(), b.nonnull()) {
                    (Some(false), Some(false)) => Some(true),
                    (Some(false), Some(true)) | (Some(true), Some(false)) => Some(false),
                    _ => None,
                };
                return Ok(Flow::Cond(t, eq.map(|e| if opc == 0xa5 { e } else { !e })));
            }
            op::GOTO | op::GOTO_W => {
                let Operand::Branch(t) = ins.operand else { return Err(()) };
                return Ok(Flow::Goto(t));
            }
            op::JSR | op::RET | op::JSR_W => return Err(()),
            op::TABLESWITCH | op::LOOKUPSWITCH => {
                let key = pop(s)?;
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
                if let V::Int(k) = key {
                    let t = cases.iter().find(|c| c.0 == k).map_or(default, |c| c.1);
                    return Ok(Flow::Switch(vec![t]));
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
                        let v = self.folded(opc, off, self.oracle.field(opc, f)).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    op::PUTSTATIC => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        value = Some(pop(s)?);
                    }
                    op::GETFIELD => {
                        recv = Some(pop(s)?);
                        let v = self.folded(opc, off, self.oracle.field(opc, f)).unwrap_or_else(|| value_of(&ft, Src::Site(off)));
                        push_typed(&mut s.stack, &ft, v);
                    }
                    _ => {
                        if ft.slots() == 2 {
                            pop(s)?;
                        }
                        value = Some(pop(s)?);
                        recv = Some(pop(s)?);
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
                let r = self.oracle.invoke_result(opc, m, *iface, &args);
                self.ev(off, Event::Invoke { opcode: opc, mref: m.clone(), iface: *iface, args });
                let v = match r {
                    Ret::Never => return Ok(Flow::End),
                    Ret::Value(v) => Some(v),
                    Ret::Unknown => None,
                };
                if let Some(ret) = &md.ret {
                    let v = self.folded(opc, off, v).unwrap_or_else(|| value_of(ret, Src::Site(off)));
                    push_typed(&mut s.stack, ret, v);
                }
            }
            op::INVOKEDYNAMIC => {
                let Operand::InvokeDynamic { bsm, name, desc } = &ins.operand else { return Err(()) };
                let md = parse_method(desc).ok_or(())?;
                let args = pop_args(s, &md.params)?;
                if let Some(ret) = &md.ret {
                    push_typed(&mut s.stack, ret, value_of(ret, Src::Site(off)));
                }
                self.ev(off, Event::Indy { bsm: *bsm, name: name.clone(), desc: desc.clone(), args });
            }
            op::NEW => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                s.stack.push(site_ref(c, true, off));
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
                    V::Str(_) | V::Class(_) => (v, None),
                    // 数组目标：来源不变（数组类型不参与收窄）
                    V::Ref { nonnull, src, .. } if c.starts_with('[') => (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src }, None),
                    // 类目标：结果以本偏移为来源，跨汇合点仍保留按来源的收窄
                    V::Ref { nonnull, .. } => (V::Ref { ty: Some(Rc::from(c.as_str())), nonnull, src: src1(Src::Site(off)) }, Some(v)),
                    // 未知值（保守）：来源仍未知
                    other => (other, None),
                };
                s.stack.push(out);
                self.ev(off, Event::CheckCast(c.clone(), input));
            }
            op::INSTANCEOF => {
                let Operand::Class(c) = &ins.operand else { return Err(()) };
                let v = pop(s)?;
                s.stack.push(if v == V::Null { V::Int(0) } else { V::Top });
                self.ev(off, Event::InstanceOf(c.clone()));
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
        locals.push(V::Ref { ty: Some(Rc::from(owner)), nonnull: true, src: src1(Src::Param(0)) });
    }
    let base = u16::from(!is_static);
    for (i, p) in md.params.iter().enumerate() {
        let k = base + i as u16;
        locals.push(param(k).unwrap_or_else(|| value_of(p, Src::Param(k))));
        if p.slots() == 2 {
            locals.push(V::Hi);
        }
    }
    if locals.len() > max_locals as usize {
        return None;
    }
    locals.resize(max_locals as usize, V::Top);
    Some(State { locals, stack: Vec::new() })
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
            (Operand::InvokeDynamic { bsm, name, desc }, _) => {
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
            (Operand::Class(c), op::INSTANCEOF) => Some(Event::InstanceOf(c.clone())),
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
    Analysis { reachable: vec![true; code.insns.len()], events, pending_catch: vec![], conservative: true }
}

/// 分析一个方法体
pub fn analyze<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O) -> Analysis {
    match run(owner, desc, is_static, code, oracle) {
        Some(a) => a,
        None => conservative(code),
    }
}

fn run<O: Oracle>(owner: &str, desc: &str, is_static: bool, code: &Code, oracle: &O) -> Option<Analysis> {
    let insns = &code.insns;
    let n = insns.len();
    let idx: HashMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
    let at = |off: u32| idx.get(&off).copied();

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
    let mut interp = Interp { oracle, emit: None };

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
                match interp.step(&mut st, ins).ok()? {
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
                        if k != Some(false) {
                            merge(&mut entry, &mut work, at(t)?, &st)?;
                        }
                        if k != Some(true) {
                            if i + 1 >= n {
                                return None;
                            }
                            merge(&mut entry, &mut work, i + 1, &st)?;
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
                    if !oracle.catch_live(ct) {
                        continue;
                    }
                }
                handler_on[hi] = true;
            }
            let ty: Rc<str> = Rc::from(h.catch_type.as_deref().unwrap_or("java/lang/Throwable"));
            let st = State {
                locals: hl.clone(),
                stack: vec![V::Ref { ty: Some(ty), nonnull: true, src: src1(Src::Catch(h.handler)) }],
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
            match interp.step(&mut st, ins).ok()? {
                Flow::Next if i + 1 < n && !leader[i + 1] => {
                    i += 1;
                    continue;
                }
                _ => break,
            }
        }
    }
    let mut pending_catch = Vec::new();
    for (hi, h) in code.exception_table.iter().enumerate() {
        if handler_on[hi] {
            events.push((h.handler, Event::Catch(h.catch_type.clone())));
        } else if let Some(ct) = &h.catch_type {
            let live_range = insns.iter().enumerate().any(|(i, x)| reachable[i] && x.offset >= h.start && x.offset < h.end);
            if live_range {
                pending_catch.push(ct.clone());
            }
        }
    }
    events.sort_by_key(|e| e.0);
    Some(Analysis { reachable, events, pending_catch, conservative: false })
}

fn targets_empty(o: &Operand) -> bool {
    !matches!(o, Operand::Branch(_) | Operand::TableSwitch { .. } | Operand::LookupSwitch { .. })
}
