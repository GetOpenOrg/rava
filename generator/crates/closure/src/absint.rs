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
mod final_field_tests;
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
pub use obj::{Obj, IMAGE_PENDING};
pub use shape::Shape;
pub use strs::StrKind;
mod value;
pub use value::{component, Src, Srcs, CLASS, STRING, V};
use value::{ft_name, push_typed, site_ref, src1, typed, value_of};
mod oracle;
pub use oracle::{Analysis, Event, Oracle, Ret};
mod step;
mod fixpoint;
pub use fixpoint::{analyze, analyze_init};

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
    /// 已作出调用点镜像值集答复
    site_assumed: bool,
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
        if let (Some(x), Some(y)) = (a.obj(), b.obj()) {
            if let (Obj::Image(a, _), Obj::Image(b, _)) = (&**x, &**y) {
                return Some(a == b);
            }
        }
        let (c, v) = match (a, b) {
            // 内容不同的两个确定字符串必是不同对象（内容相同不能断言同一：非字面量来源可能是副本）
            (V::Str(x, _), V::Str(y, _)) if x != y => return Some(false),
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

    /// 接收者只来自一个 Class 形参的实例调用按形参镜像值集求结果（记为乐观答复）
    /// 接收者只来自一个调用点产出的 Class 值时按该调用点的类镜像值集求结果（同样记为乐观答复）
    fn param_mirror_call(&mut self, recv: Option<&V>, m: &MemberRef) -> Option<V> {
        let Some(V::Ref { src, .. }) = recv else { return None };
        match &src[..] {
            [Src::Param(i)] => {
                let v = self.oracle.param_mirror_call(*i, m)?;
                self.field_assumed.push(*i);
                Some(v)
            }
            [Src::Site(o)] => {
                let v = self.oracle.site_mirror_call(*o, m)?;
                self.site_assumed = true;
                Some(v)
            }
            _ => None,
        }
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
