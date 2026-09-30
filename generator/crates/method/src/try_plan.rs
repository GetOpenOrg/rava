//! 异常表 → try/catch 区域规划（← `method/try_catch.py`，JVMS §2.10 / §4.7.3）。
//!
//! - [`TryGroup`]：受保护区间集合完全相同的一组处理器 = 一个 Java `try` 语句的兄弟 catch
//!   子句（multi-catch 的多个 catch_type 指向同一处理器，合并为一个子句）；
//! - 受保护区间之外的指令（javac 内联的 finally 副本、try 体尾部的 goto）不属于 try 体：
//!   CFG 层按「块是否被区间覆盖」给每个块标注所属 try 组。
//!
//! 本模块只依赖指令偏移与操作码，不依赖控制流结构化实现。

use std::collections::{BTreeMap, BTreeSet};

use classfile::extras::LocalVar;
use classfile::insn::op;
use classfile::{ExceptionEntry, Insn, Operand};
use instr::InstrCtx;
use ty::RsType;

use crate::error::{MethodError, MethodResult};

const ASTORE: u8 = 0x3a;
const ASTORE_0: u8 = 0x4b;
const ASTORE_3: u8 = 0x4e;

/// 一个 catch 子句（同一处理器入口）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchClause {
    pub handler_pc: u32,
    pub handler_idx: usize,
    /// 捕获类 binary 名；空 = catch-any（finally / monitor 兜底）
    pub types: Vec<String>,
    pub any: bool,
    /// catch 体的文本终点（异常变量 LVT 作用域终点）；无调试信息时 None
    pub body_end_pc: Option<u32>,
}

impl CatchClause {
    pub fn is_catch_any(&self) -> bool {
        self.any || self.types.is_empty()
    }
}

/// 一个 try 语句
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryGroup {
    /// 已排序、已合并的受保护区间
    pub ranges: Vec<(u32, u32)>,
    /// 按异常表顺序（= 匹配优先级）
    pub clauses: Vec<CatchClause>,
    pub start_idx: usize,
    pub body_end_idx: usize,
}

impl TryGroup {
    pub fn covers(&self, pc: u32) -> bool {
        self.ranges.iter().any(|&(s, e)| s <= pc && pc < e)
    }

    pub fn first_handler_pc(&self) -> u32 {
        self.clauses.iter().map(|c| c.handler_pc).min().unwrap_or(0)
    }
}

/// 一个方法的全部 try 区域
#[derive(Debug, Clone, Default)]
pub struct TryPlan {
    pub groups: Vec<TryGroup>,
    /// try 体首指令下标 → 在该处开始的组编号（外层在前；按首次出现序）
    by_start: Vec<(usize, Vec<usize>)>,
}

fn is_return(opc: u8) -> bool {
    (op::IRETURN..=op::RETURN).contains(&opc)
}

/// 处理器首条 astore 的目标槽
fn astore_slot(ins: &Insn) -> Option<u16> {
    match ins.opcode {
        ASTORE => match ins.operand {
            Operand::Local(s) => Some(s),
            _ => Some(0),
        },
        ASTORE_0..=ASTORE_3 => Some(u16::from(ins.opcode - ASTORE_0)),
        _ => None,
    }
}

struct HandlerAcc {
    ranges: BTreeSet<(u32, u32)>,
    types: Vec<String>,
    any: bool,
}

impl TryPlan {
    pub fn new(table: &[ExceptionEntry], insns: &[Insn], local_vars: &[LocalVar]) -> TryPlan {
        let off2idx: BTreeMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
        let groups = build(table, insns, local_vars, &off2idx);
        let mut by_start: Vec<(usize, Vec<usize>)> = Vec::new();
        for (gid, g) in groups.iter().enumerate() {
            match by_start.iter_mut().find(|(s, _)| *s == g.start_idx) {
                Some((_, l)) => l.push(gid),
                None => by_start.push((g.start_idx, vec![gid])),
            }
        }
        // 同一起点的多个组：处理器越靠后的越外层（稳定排序）
        for (_, l) in &mut by_start {
            l.sort_by_key(|&g| std::cmp::Reverse(groups[g].first_handler_pc()));
        }
        TryPlan { groups, by_start }
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn groups_by_start(&self) -> &[(usize, Vec<usize>)] {
        &self.by_start
    }

    /// 全部 catch 体终点（升序去重）
    pub fn catch_body_ends(&self) -> Vec<u32> {
        let set: BTreeSet<u32> =
            self.groups.iter().flat_map(|g| g.clauses.iter().filter_map(|c| c.body_end_pc)).collect();
        set.into_iter().collect()
    }
}

fn build(table: &[ExceptionEntry], insns: &[Insn], lvs: &[LocalVar], off2idx: &BTreeMap<u32, usize>) -> Vec<TryGroup> {
    // 1. 按处理器聚合
    let mut by_handler: BTreeMap<u32, HandlerAcc> = BTreeMap::new();
    let mut order: Vec<u32> = Vec::new();
    for e in table {
        if e.start >= e.handler {
            // 处理器保护自身（finally / monitorexit 的重试条目）：不构成 try 区域
            continue;
        }
        // finally 区间越过处理器入口的部分同样是自保护：截断到入口
        let end = e.end.min(e.handler);
        let h = by_handler.entry(e.handler).or_insert_with(|| {
            order.push(e.handler);
            HandlerAcc { ranges: BTreeSet::new(), types: Vec::new(), any: false }
        });
        h.ranges.insert((e.start, end));
        match &e.catch_type {
            None => h.any = true,
            Some(t) if !h.types.contains(t) => h.types.push(t.clone()),
            Some(_) => {}
        }
    }
    // 2. 区间集合相同的处理器 = 同一 try 语句的兄弟 catch
    let mut groups: Vec<TryGroup> = Vec::new();
    for hpc in order {
        let h = &by_handler[&hpc];
        let Some(&handler_idx) = off2idx.get(&hpc) else {
            continue;
        };
        let key = merge_ranges(&coalesce_return_gaps(&h.ranges, insns));
        let clause = CatchClause {
            handler_pc: hpc,
            handler_idx,
            types: h.types.clone(),
            any: h.any,
            body_end_pc: catch_body_end(handler_idx, insns, lvs, off2idx),
        };
        match groups.iter_mut().find(|g| g.ranges == key) {
            Some(g) => g.clauses.push(clause),
            None => groups.push(TryGroup { ranges: key, clauses: vec![clause], start_idx: 0, body_end_idx: 0 }),
        }
    }
    let mut out = Vec::new();
    for mut g in groups {
        let Some(&start_idx) = g.ranges.first().and_then(|r| off2idx.get(&r.0)) else {
            continue;
        };
        g.start_idx = start_idx;
        g.body_end_idx = g.clauses.iter().map(|c| c.handler_idx).min().unwrap_or(0);
        if g.body_end_idx <= g.start_idx {
            continue;
        }
        out.push(g);
    }
    out
}

/// 处理器首条 astore 的目标槽 → 该异常变量的作用域终点（必须是指令边界）
fn catch_body_end(handler_idx: usize, insns: &[Insn], lvs: &[LocalVar], off2idx: &BTreeMap<u32, usize>) -> Option<u32> {
    let slot = astore_slot(&insns[handler_idx])?;
    let next = insns.get(handler_idx + 1)?;
    let scope_start = next.offset;
    let lv = lvs.iter().find(|lv| lv.slot == slot && u32::from(lv.start) == scope_start)?;
    let end = u32::from(lv.start) + u32::from(lv.len);
    off2idx.contains_key(&end).then_some(end)
}

fn merge_ranges(ranges: &BTreeSet<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut merged: Vec<(u32, u32)> = Vec::new();
    for &(s, e) in ranges {
        match merged.last_mut() {
            Some(last) if s <= last.1 => last.1 = last.1.max(e),
            _ => merged.push((s, e)),
        }
    }
    merged
}

/// 合并「裸 return 间隙」：同一处理器相邻区间之间若全部是 xreturn，并入受护区间
/// （return 不抛异常，捕获行为不变）
fn coalesce_return_gaps(ranges: &BTreeSet<(u32, u32)>, insns: &[Insn]) -> BTreeSet<(u32, u32)> {
    if ranges.len() < 2 {
        return ranges.clone();
    }
    let mut it = ranges.iter().copied();
    let mut out: Vec<(u32, u32)> = it.next().into_iter().collect();
    for (s, e) in it {
        let Some(last) = out.last_mut() else { break };
        let (ps, pe) = *last;
        let gap_all_return = insns.iter().filter(|i| pe <= i.offset && i.offset < s).all(|i| is_return(i.opcode));
        if gap_all_return {
            *last = (ps, pe.max(e));
        } else {
            out.push((s, e));
        }
    }
    out.into_iter().collect()
}

/// catch 绑定变量的静态类型：catch-any → Throwable；多类型 → 公共父类（找不到回落 Throwable）
pub fn binding_type(ctx: &InstrCtx, clause: &CatchClause) -> MethodResult<RsType> {
    let root = ty::consts::THROWABLE;
    let throwable = || -> MethodResult<RsType> {
        if !ctx.reg().contains(root) {
            return Err(MethodError::Runtime(format!(
                "catch-any 绑定类型 {root} 不在 registry，无法生成 catch 分派"
            )));
        }
        Ok(RsType::class(root, Vec::new()))
    };
    if clause.is_catch_any() {
        return throwable();
    }
    let mut lub = RsType::class(clause.types[0].clone(), Vec::new());
    for other in &clause.types[1..] {
        let o = RsType::class(other.clone(), Vec::new());
        lub = match instr::hierarchy::common_ref_type(ctx, &lub, &o) {
            Some(c) => c,
            None => throwable()?,
        };
    }
    Ok(lub)
}

/// `java_try!` 的 catch 子句头：`catch (e)` / `catch (e: A)` / `catch (e: A | B as LUB)`
pub fn catch_head(ctx: &InstrCtx, clause: &CatchClause, bind: &str, bind_ty: &str) -> String {
    if clause.is_catch_any() {
        return format!("catch ({bind})");
    }
    let names: Vec<String> = clause.types.iter().map(|t| ctx.short(t)).collect();
    let mut head = format!("catch ({bind}: {}", names.join(" | "));
    if names.len() > 1 || names[0] != bind_ty {
        head.push_str(&format!(" as {bind_ty}"));
    }
    head.push(')');
    head
}
