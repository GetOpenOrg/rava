//! 字节码规范化：折叠点（folds v2）应用（`codegen/closure_folds.py` 的移植）。
//!
//! 闭包分析器剪掉的不可达代码与常量读取点，发射层同样不翻译。规范化在 CFG 结构化之前
//! 单点执行，产出 [`NormCode`]：
//! 1. dead_pcs 内的指令删除；
//! 2. 条件跳转只剩一个活后继 → `pop`（×操作数个数）+ `goto`（或直通），偏移沿用原指令字节；
//!    switch 死目标改指向回退活目标，只剩一个活目标时改写为 `pop` + `goto`；
//! 3. dead_handlers、dead_catches 列出的表项与受保护区间全死的表项删除，其余端点收拢到活指令起点；
//! 4. 常量读取点：getstatic 直接替换为装载指令；getfield 替换为 [`NInsn::FoldField`]
//!    （弹出 receiver 后压入常量）；invoke 替换为 [`NInsn::FoldCall`]：调用照常执行（被调方
//!    副作用保留），只丢弃返回值、改压常量。
//!
//! 违反格式约定的输入返回 [`InputError::Fold`]。

use std::collections::BTreeSet;

use classfile::{op, Code, Const, ExceptionEntry, Insn, Operand};
use ty::consts;

use crate::facts::{DeadCatch, FoldConst, FoldValue, MethodFold, ReadKind};
use crate::InputError;

pub const ACONST_NULL: u8 = 0x01;
pub const POP: u8 = 0x57;
const IFEQ: u8 = 0x99;
const IFLE: u8 = 0x9e;
const IF_ICMPEQ: u8 = 0x9f;
const IF_ACMPNE: u8 = 0xa6;

/// 规范化后的指令
#[derive(Debug, Clone)]
pub enum NInsn {
    /// 原始 / 改写后的 JVM 指令
    Op(Insn),
    /// getfield 折叠点：弹出 receiver 后执行装载指令 `load`
    FoldField { offset: u32, load: Insn },
    /// invoke 折叠点：调用 `call` 照常翻译、丢弃返回值，再执行装载指令 `load`
    FoldCall { call: Insn, load: Insn },
}

impl NInsn {
    pub fn offset(&self) -> u32 {
        match self {
            NInsn::Op(i) => i.offset,
            NInsn::FoldField { offset, .. } => *offset,
            NInsn::FoldCall { call, .. } => call.offset,
        }
    }
    /// JVM 指令本体（FoldCall 为保留的调用指令；FoldField 为 None）
    pub fn insn(&self) -> Option<&Insn> {
        match self {
            NInsn::Op(i) => Some(i),
            NInsn::FoldCall { call, .. } => Some(call),
            NInsn::FoldField { .. } => None,
        }
    }
}

/// 方法体的只读指令视图（规范化体或原始体，不复制指令）
#[derive(Debug, Clone, Copy)]
pub enum CodeOps<'s> {
    Norm(&'s NormCode),
    Raw(&'s Code),
}

impl<'s> CodeOps<'s> {
    /// JVM 指令序列（等同逐条 [`NInsn::insn`] 过滤：FoldField 不出现）
    pub fn ops(self) -> Box<dyn DoubleEndedIterator<Item = &'s Insn> + 's> {
        match self {
            CodeOps::Norm(n) => Box::new(n.insns.iter().filter_map(NInsn::insn)),
            CodeOps::Raw(c) => Box::new(c.insns.iter()),
        }
    }

    pub fn exception_table(self) -> &'s [ExceptionEntry] {
        match self {
            CodeOps::Norm(n) => &n.exception_table,
            CodeOps::Raw(c) => &c.exception_table,
        }
    }
}

/// 规范化后的方法体
#[derive(Debug, Clone)]
pub struct NormCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_len: u32,
    pub insns: Vec<NInsn>,
    pub exception_table: Vec<ExceptionEntry>,
}

impl NormCode {
    /// 未折叠的原样视图
    pub fn raw(code: &Code) -> NormCode {
        NormCode {
            max_stack: code.max_stack,
            max_locals: code.max_locals,
            code_len: code.code_len,
            insns: code.insns.iter().cloned().map(NInsn::Op).collect(),
            exception_table: code.exception_table.clone(),
        }
    }
}

fn one_operand_branch(o: u8) -> bool {
    (IFEQ..=IFLE).contains(&o) || o == op::IFNULL || o == op::IFNONNULL
}

fn two_operand_branch(o: u8) -> bool {
    (IF_ICMPEQ..=IF_ACMPNE).contains(&o)
}

fn is_goto(o: u8) -> bool {
    o == op::GOTO || o == op::GOTO_W
}

fn is_switch(o: u8) -> bool {
    o == op::TABLESWITCH || o == op::LOOKUPSWITCH
}

fn no_fallthrough(o: u8) -> bool {
    is_goto(o) || is_switch(o) || (op::IRETURN..=op::RETURN).contains(&o) || o == op::ATHROW
}

fn is_invoke(o: u8) -> bool {
    matches!(o, op::INVOKEVIRTUAL | op::INVOKESPECIAL | op::INVOKESTATIC | op::INVOKEINTERFACE)
}

fn err(where_: &str, msg: String) -> InputError {
    InputError::Fold(format!("{where_}: {msg}"))
}

fn simple(offset: u32, opcode: u8) -> Insn {
    Insn { offset, opcode, operand: Operand::None }
}

fn ldc(offset: u32, opcode: u8, c: Const) -> Insn {
    Insn { offset, opcode, operand: Operand::Ldc(c) }
}

/// 常量 → 等价的 JVM 装载指令
fn push_insn(pc: u32, c: &FoldConst, where_: &str) -> Result<Insn, InputError> {
    let ty = c.ty.as_str();
    let string_desc = format!("L{};", consts::STRING);
    match (&c.value, ty) {
        (FoldValue::Null, _) if ty.starts_with('L') || ty.starts_with('[') => Ok(simple(pc, ACONST_NULL)),
        (FoldValue::Null, _) => Err(err(where_, format!("null 常量的 type 必须是引用类型，得到 {ty:?}"))),
        (FoldValue::Bool(b), "Z") => Ok(ldc(pc, op::LDC, Const::Int(i32::from(*b)))),
        (v, "Z") => Err(err(where_, format!("Z 常量须为 JSON 布尔值，得到 {v:?}"))),
        (FoldValue::Int(i), "B" | "C" | "S" | "I") => {
            let i = i32::try_from(*i).map_err(|_| err(where_, format!("{ty} 常量越界：{i}")))?;
            Ok(ldc(pc, op::LDC, Const::Int(i)))
        }
        (v, "B" | "C" | "S" | "I") => Err(err(where_, format!("{ty} 常量须为 JSON 整数，得到 {v:?}"))),
        (FoldValue::Long(l) | FoldValue::Int(l), "J") => Ok(ldc(pc, op::LDC2_W, Const::Long(*l))),
        (FoldValue::Str(s), _) if ty == string_desc => Ok(ldc(pc, op::LDC, Const::String(s.clone()))),
        (v, _) if ty == string_desc => Err(err(where_, format!("String 常量须为 JSON 字符串，得到 {v:?}"))),
        (v, "J" | "F" | "D") => Err(err(where_, format!("未移植：{ty} 常量编码 {v:?}"))),
        _ => Err(err(where_, format!("不支持的常量类型 {ty:?}"))),
    }
}

fn const_insn(ins: &Insn, c: &FoldConst, where_: &str) -> Result<NInsn, InputError> {
    let ok = match c.kind {
        ReadKind::GetField => ins.opcode == op::GETFIELD,
        ReadKind::GetStatic => ins.opcode == op::GETSTATIC,
        ReadKind::Invoke => is_invoke(ins.opcode),
    };
    if !ok {
        return Err(err(where_, format!("const kind={:?} 与指令 {} 不符", c.kind, ins.name())));
    }
    let load = push_insn(ins.offset, c, where_)?;
    Ok(match c.kind {
        ReadKind::GetStatic => NInsn::Op(load),
        ReadKind::GetField => NInsn::FoldField { offset: ins.offset, load },
        ReadKind::Invoke => NInsn::FoldCall { call: ins.clone(), load },
    })
}

fn pops(pc: u32, n: u32) -> Vec<NInsn> {
    (0..n).map(|i| NInsn::Op(simple(pc + i, POP))).collect()
}

fn goto(pc: u32, target: u32) -> Insn {
    Insn { offset: pc, opcode: op::GOTO, operand: Operand::Branch(target) }
}

/// switch 的全部目标（default 在前，按操作数顺序）
pub fn switch_targets(operand: &Operand) -> Vec<u32> {
    match operand {
        Operand::TableSwitch { default, targets, .. } => std::iter::once(*default).chain(targets.iter().copied()).collect(),
        Operand::LookupSwitch { default, pairs } => std::iter::once(*default).chain(pairs.iter().map(|p| p.1)).collect(),
        _ => Vec::new(),
    }
}

fn rewrite_switch(ins: &Insn, is_dead: &dyn Fn(u32) -> bool, where_: &str) -> Result<Vec<NInsn>, InputError> {
    let all = switch_targets(&ins.operand);
    let live: Vec<u32> = all.iter().copied().filter(|t| !is_dead(*t)).collect();
    let Some(&first) = live.first() else {
        return Err(err(where_, format!("switch pc={} 的目标全在 dead_pcs 内", ins.offset)));
    };
    if live.iter().collect::<BTreeSet<_>>().len() == 1 {
        return Ok(vec![NInsn::Op(simple(ins.offset, POP)), NInsn::Op(goto(ins.offset + 1, first))]);
    }
    if live.len() == all.len() {
        return Ok(vec![NInsn::Op(ins.clone())]);
    }
    let fallback = if is_dead(all[0]) { first } else { all[0] };
    let fix = |t: u32| if is_dead(t) { fallback } else { t };
    let operand = match &ins.operand {
        Operand::TableSwitch { default, low, high, targets } => Operand::TableSwitch {
            default: fix(*default),
            low: *low,
            high: *high,
            targets: targets.iter().map(|t| fix(*t)).collect(),
        },
        Operand::LookupSwitch { default, pairs } => Operand::LookupSwitch {
            default: fix(*default),
            pairs: pairs.iter().map(|(k, t)| (*k, fix(*t))).collect(),
        },
        _ => return Err(err(where_, format!("switch pc={} 操作数不合法", ins.offset))),
    };
    Ok(vec![NInsn::Op(Insn { offset: ins.offset, opcode: ins.opcode, operand })])
}

fn rewrite_branch(ins: &Insn, next_dead: bool, is_dead: &dyn Fn(u32) -> bool, where_: &str) -> Result<Vec<NInsn>, InputError> {
    let o = ins.opcode;
    if one_operand_branch(o) || two_operand_branch(o) {
        let n = if one_operand_branch(o) { 1 } else { 2 };
        let Operand::Branch(t) = ins.operand else {
            return Err(err(where_, format!("条件跳转 pc={} 缺目标", ins.offset)));
        };
        let target_dead = is_dead(t);
        return Ok(match (target_dead, next_dead) {
            (true, true) => return Err(err(where_, format!("条件跳转 pc={} 两个后继都在 dead_pcs 内", ins.offset))),
            (true, false) => pops(ins.offset, n),
            (false, true) => {
                let mut v = pops(ins.offset, n);
                v.push(NInsn::Op(goto(ins.offset + n, t)));
                v
            }
            (false, false) => vec![NInsn::Op(ins.clone())],
        });
    }
    if is_goto(o) {
        if let Operand::Branch(t) = ins.operand {
            if is_dead(t) {
                return Err(err(where_, format!("活跳转 pc={} 的目标在 dead_pcs 内", ins.offset)));
            }
        }
        return Ok(vec![NInsn::Op(ins.clone())]);
    }
    if is_switch(o) {
        return rewrite_switch(ins, is_dead, where_);
    }
    Ok(vec![NInsn::Op(ins.clone())])
}

fn validate(fold: &MethodFold, code: &Code, is_dead: &dyn Fn(u32) -> bool, where_: &str) -> Result<(), InputError> {
    let starts: BTreeSet<u32> = code.insns.iter().map(|x| x.offset).collect();
    for &(s, e) in &fold.dead_pcs {
        if !starts.contains(&s) || !(starts.contains(&e) || e == code.code_len) || s >= e {
            return Err(err(where_, format!("dead_pcs [{s}, {e}) 端点不在指令起点")));
        }
    }
    for h in &fold.dead_handlers {
        if !starts.contains(h) {
            return Err(err(where_, format!("dead_handlers {h} 不在指令起点")));
        }
    }
    for pc in fold.consts.keys() {
        if !starts.contains(pc) || is_dead(*pc) {
            return Err(err(where_, format!("const pc={pc} 不是活指令起点")));
        }
    }
    Ok(())
}

/// 表项列入 dead_catches（catch-any 从不列入）
fn is_dead_catch(fold: &MethodFold, ent: &ExceptionEntry) -> bool {
    let Some(ct) = &ent.catch_type else { return false };
    fold.dead_catches.contains(&DeadCatch { start: ent.start, end: ent.end, handler: ent.handler, catch_type: ct.clone() })
}

/// 按折叠点规范化一个方法体（`method_key` = `类.方法:描述符`，只用于报错定位）
pub fn apply_fold(method_key: &str, code: &Code, fold: &MethodFold) -> Result<NormCode, InputError> {
    let where_ = method_key;
    let is_dead = |pc: u32| fold.dead_pcs.iter().any(|&(s, e)| s <= pc && pc < e);
    validate(fold, code, &is_dead, where_)?;
    let dead: Vec<bool> = code.insns.iter().map(|x| is_dead(x.offset)).collect();
    let mut out = Vec::with_capacity(code.insns.len());
    for (i, ins) in code.insns.iter().enumerate() {
        if dead[i] {
            continue;
        }
        let next_dead = dead.get(i + 1).copied().unwrap_or(false);
        let o = ins.opcode;
        if next_dead && !no_fallthrough(o) && !one_operand_branch(o) && !two_operand_branch(o) {
            return Err(err(where_, format!("活指令 pc={} {} 顺序落入 dead_pcs", ins.offset, ins.name())));
        }
        if let Some(c) = fold.consts.get(&ins.offset) {
            out.push(const_insn(ins, c, where_)?);
            continue;
        }
        out.extend(rewrite_branch(ins, next_dead, &is_dead, where_)?);
    }
    let live_offs: Vec<u32> = code.insns.iter().zip(&dead).filter(|(_, d)| !**d).map(|(x, _)| x.offset).collect();
    let first_live = |pc: u32| live_offs.iter().copied().find(|o| *o >= pc);
    let mut table = Vec::new();
    for ent in &code.exception_table {
        if fold.dead_handlers.contains(&ent.handler) || is_dead_catch(fold, ent) {
            continue;
        }
        if is_dead(ent.handler) {
            return Err(err(where_, format!("handler {} 在 dead_pcs 内但未列入 dead_handlers", ent.handler)));
        }
        let Some(ns) = first_live(ent.start).filter(|ns| *ns < ent.end) else {
            continue;
        };
        table.push(ExceptionEntry {
            start: ns,
            end: first_live(ent.end).unwrap_or(ent.end),
            handler: ent.handler,
            catch_type: ent.catch_type.clone(),
        });
    }
    Ok(NormCode {
        max_stack: code.max_stack,
        max_locals: code.max_locals,
        code_len: code.code_len,
        insns: out,
        exception_table: table,
    })
}
