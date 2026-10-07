//! 构造器确定初始化（计划 c1d §30.9 的 P2）：构造器正常返回、且 `this` 交出前必然已写入的实例字段。
//!
//! 状态里的 `inits` 是当前路径上 `this` 必然已写的字段（解析后的声明键，排序），与 `finals` 同构：
//! 合流取交集，异常处理器入口为空。逐指令在**执行前**的状态上判定（操作数都还在栈上）：
//! - `putfield`：接收者恰为 `this`（来源只有形参 0）→ 字段并入 `inits`；写入值含 `this` → 交出；
//! - `getfield`：接收者可能是 `this` 且字段不在 `inits` → 记入 `bad`（构造链内读到初值）；
//! - 委托 / 超类构造器（`invokespecial <init>`，接收者恰为 `this`，其余实参不含 `this`）：按被调构造器摘要组合——
//!   被调方交出 `this` 时已写的是 本方 `inits` ∪ 被调方交出集，被调方的 `bad` 去掉本方已写，返回后本方并入被调方返回集；
//!   无摘要（手写 / 无法分析）→ 按交出处理；
//! - `this` 作为其余调用的实参 / 接收者、`putstatic` / 数组写入值、`athrow`、indy 实参、类型转换 / 测试的输入 → 交出：
//!   交出集与当前 `inits` 取交集（交出后他方只可能读到交出时已写的字段的写入值）；
//! - `return`：返回集与当前 `inits` 取交集。
//!
//! 只有第二阶段（不动点入口状态）记录交出 / bad / 返回；第一阶段只推 `inits`。

use classfile::descriptor::{parse_field, parse_method};
use classfile::{op, Insn, MemberRef, Operand};

use super::{Oracle, Src, State, V};

/// 构造器的确定初始化摘要（字段键均为解析后的声明键，排序）
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InitSum {
    /// 各正常返回点必然已写字段的交（None = 无正常返回）
    pub exit: Option<Vec<MemberRef>>,
    /// 各交出点必然已写字段的交（None = 从不交出 `this`）
    pub esc: Option<Vec<MemberRef>>,
    /// 构造链内读到时可能尚未写入的字段
    pub bad: Vec<MemberRef>,
}

impl InitSum {
    /// 确定初始化的字段：经构造器完成的对象上，任何读取都看不到其初值
    pub fn definite(&self) -> Vec<MemberRef> {
        let Some(exit) = &self.exit else { return vec![] };
        exit.iter().filter(|k| self.esc.as_ref().is_none_or(|e| e.binary_search(k).is_ok()) && self.bad.binary_search(k).is_err()).cloned().collect()
    }
}

fn has_this(v: &V) -> bool {
    v.srcs().contains(&Src::Param(0))
}

fn is_this(v: &V) -> bool {
    matches!(v, V::Ref { .. }) && *v.srcs() == [Src::Param(0)]
}

fn insert(set: &mut Vec<MemberRef>, k: MemberRef) {
    if let Err(i) = set.binary_search(&k) {
        set.insert(i, k);
    }
}

fn meet(acc: &mut Option<Vec<MemberRef>>, w: &[MemberRef]) {
    match acc {
        Some(a) => a.retain(|k| w.binary_search(k).is_ok()),
        None => *acc = Some(w.to_vec()),
    }
}

/// 合流：`inits` 取交集，返回是否缩小
pub(super) fn join(a: &mut Vec<MemberRef>, b: &[MemberRef]) -> bool {
    let n = a.len();
    a.retain(|k| b.binary_search(k).is_ok());
    a.len() != n
}

#[derive(Default)]
pub(super) struct Track {
    sum: InitSum,
}

impl Track {
    pub(super) fn finish(mut self) -> InitSum {
        self.sum.bad.sort();
        self.sum.bad.dedup();
        self.sum
    }

    fn escape(&mut self, w: &[MemberRef], record: bool) {
        if record {
            meet(&mut self.sum.esc, w);
        }
    }

    /// 指令执行前：按操作数推 `inits`，第二阶段（record）记录交出 / bad / 返回
    /// 指令执行前：按操作数推 `inits`，第二阶段（record）记录交出 / bad / 返回
    pub(super) fn pre<O: Oracle>(&mut self, oracle: &O, s: &mut State, ins: &Insn, record: bool) {
        let st = &s.stack;
        let top = |k: usize| st.len().checked_sub(k + 1).map(|i| &st[i]);
        match (ins.opcode, &ins.operand) {
            (op::PUTFIELD, Operand::Field(f)) => {
                let slots = parse_field(&f.desc).map_or(1, |t| t.slots() as usize);
                let (Some(v), Some(r)) = (top(slots - 1), top(slots)) else { return };
                if has_this(v) {
                    let w = s.inits.clone();
                    self.escape(&w, record);
                }
                if is_this(r) {
                    if let Some(k) = oracle.init_key(f) {
                        insert(&mut s.inits, k);
                    }
                }
            }
            (op::GETFIELD, Operand::Field(f)) => {
                let Some(r) = top(0) else { return };
                if record && has_this(r) {
                    if let Some(k) = oracle.init_key(f) {
                        if s.inits.binary_search(&k).is_err() {
                            self.sum.bad.push(k);
                        }
                    }
                }
            }
            (op::PUTSTATIC | 0x53 | op::ATHROW | op::CHECKCAST | op::INSTANCEOF | 0xb0, _) => {
                if top(0).is_some_and(has_this) {
                    let w = s.inits.clone();
                    self.escape(&w, record);
                }
            }
            (op::INVOKEDYNAMIC, Operand::InvokeDynamic { desc, .. }) => {
                let n = parse_method(desc).map_or(st.len(), |md| md.params.iter().map(|p| p.slots() as usize).sum());
                if st[st.len().saturating_sub(n)..].iter().any(has_this) {
                    let w = s.inits.clone();
                    self.escape(&w, record);
                }
            }
            (op::INVOKEVIRTUAL | op::INVOKESPECIAL | op::INVOKESTATIC | op::INVOKEINTERFACE, Operand::Method(m, _)) => {
                let n = parse_method(&m.desc).map_or(st.len(), |md| md.params.iter().map(|p| p.slots() as usize).sum::<usize>())
                    + usize::from(ins.opcode != op::INVOKESTATIC);
                let args = &st[st.len().saturating_sub(n)..];
                if !args.iter().any(has_this) {
                    return;
                }
                let delegate = ins.opcode == op::INVOKESPECIAL && m.name == "<init>" && args.first().is_some_and(is_this) && !args[1..].iter().any(has_this);
                let sum = if delegate { oracle.init_sum(m) } else { None };
                let Some(sum) = sum else {
                    let w = s.inits.clone();
                    self.escape(&w, record);
                    return;
                };
                if record {
                    if let Some(e) = &sum.esc {
                        let mut w = s.inits.clone();
                        for k in e {
                            insert(&mut w, k.clone());
                        }
                        meet(&mut self.sum.esc, &w);
                    }
                    let inits = &s.inits;
                    self.sum.bad.extend(sum.bad.iter().filter(|k| inits.binary_search(k).is_err()).cloned());
                }
                for k in sum.exit.iter().flatten() {
                    insert(&mut s.inits, k.clone());
                }
            }
            (op::RETURN, _) if record => meet(&mut self.sum.exit, &s.inits),
            _ => {}
        }
    }
}
