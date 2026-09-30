//! 基本块划分（← `graph.py` 前半：`Terminator` / `Block` / `build_blocks`）。
//!
//! 只描述「指令序列的形状」，不涉及栈模拟与 Rust 生成。
//! 与 Python 的差异：指令操作数已由 `classfile` 结构化解码（分支目标为绝对偏移、
//! switch 为 [`Operand::TableSwitch`] / [`Operand::LookupSwitch`]），不再解析
//! `parse_switch_operand` 的文本格式。

use std::collections::{BTreeMap, BTreeSet};

use classfile::class::ExceptionEntry;
use classfile::insn::{Insn, Operand};

use crate::opcodes;
use crate::CfgError;

/// 基本块 / 归约图节点编号（块编号即 [`build_blocks`] 结果的下标）。
pub type NodeId = u32;

/// 基本块的出口。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Terminator {
    /// 条件跳转：`target` = 跳转目标块，`fallthrough` = 不跳转时的后继块
    Cond { pc: u32, target: NodeId, fallthrough: NodeId },
    /// 无条件跳转
    Goto { pc: u32, target: NodeId },
    /// 自然落入下一块
    Fall { target: NodeId },
    /// `cases` = [(case 值, 目标块)]（同目标合并，按首次出现序），`default` = 默认目标块
    Switch { pc: u32, cases: Vec<(Vec<i32>, NodeId)>, default: NodeId },
    /// return / athrow（无后继）
    Exit,
}

impl Terminator {
    /// 跳转指令的字节码偏移（fall / exit 为 None）
    pub fn pc(&self) -> Option<u32> {
        match self {
            Terminator::Cond { pc, .. } | Terminator::Goto { pc, .. } | Terminator::Switch { pc, .. } => Some(*pc),
            Terminator::Fall { .. } | Terminator::Exit => None,
        }
    }

    /// 后继（去重，保持目标 → 落入 / case 序 → default 的顺序）
    pub fn successors(&self) -> Vec<NodeId> {
        match self {
            Terminator::Cond { target, fallthrough, .. } => {
                if target == fallthrough {
                    vec![*target]
                } else {
                    vec![*target, *fallthrough]
                }
            }
            Terminator::Goto { target, .. } | Terminator::Fall { target } => vec![*target],
            Terminator::Switch { cases, default, .. } => switch_successors(cases, *default),
            Terminator::Exit => Vec::new(),
        }
    }
}

/// switch 的后继：各 case 目标（首次出现序）+ default（去重）。
pub fn switch_successors(cases: &[(Vec<i32>, NodeId)], default: NodeId) -> Vec<NodeId> {
    let mut out: Vec<NodeId> = Vec::new();
    for (_, b) in cases {
        if !out.contains(b) {
            out.push(*b);
        }
    }
    if !out.contains(&default) {
        out.push(default);
    }
    out
}

/// 基本块：指令下标区间 `[start_idx, end_idx)`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub id: NodeId,
    pub start_idx: usize,
    pub end_idx: usize,
    pub start_pc: u32,
    pub term: Terminator,
    pub is_handler_entry: bool,
}

/// switch 操作数 → (default 偏移, [(case 值, 目标偏移)])。
pub fn switch_targets(ins: &Insn) -> Result<(u32, Vec<(i32, u32)>), CfgError> {
    match &ins.operand {
        Operand::TableSwitch { default, low, targets, .. } => {
            let pairs = targets.iter().enumerate().map(|(i, t)| (low.wrapping_add(i as i32), *t)).collect();
            Ok((*default, pairs))
        }
        Operand::LookupSwitch { default, pairs } => Ok((*default, pairs.clone())),
        _ => Err(CfgError::new(format!("switch 指令缺少跳转表（pc={}）", ins.offset))),
    }
}

fn branch_target(ins: &Insn) -> Result<u32, CfgError> {
    match ins.operand {
        Operand::Branch(t) => Ok(t),
        _ => Err(CfgError::new(format!("跳转指令缺少目标（pc={}）", ins.offset))),
    }
}

struct Index<'a> {
    off2idx: BTreeMap<u32, usize>,
    insns: &'a [Insn],
}

impl Index<'_> {
    fn idx_of(&self, off: u32, pc: u32) -> Result<usize, CfgError> {
        self.off2idx
            .get(&off)
            .copied()
            .ok_or_else(|| CfgError::new(format!("跳转目标 {off} 不是指令边界（pc={pc}）")))
    }

    /// 块首指令下标集合（leaders）与处理器入口下标。
    fn leaders(
        &self,
        exception_table: &[ExceptionEntry],
        boundaries: &[u32],
    ) -> Result<(BTreeSet<usize>, BTreeSet<usize>), CfgError> {
        let mut leaders: BTreeSet<usize> = BTreeSet::from([0]);
        let mut handlers: BTreeSet<usize> = BTreeSet::new();
        for e in exception_table {
            if let Some(&h) = self.off2idx.get(&e.handler) {
                leaders.insert(h);
                handlers.insert(h);
            }
            // 受保护区间的两端是块边界：每个块要么整体受某个处理器保护，要么整体不受
            for pc in [e.start, e.end] {
                if let Some(&i) = self.off2idx.get(&pc) {
                    leaders.insert(i);
                }
            }
        }
        for pc in boundaries {
            if let Some(&i) = self.off2idx.get(pc) {
                leaders.insert(i);
            }
        }
        for (i, ins) in self.insns.iter().enumerate() {
            let opc = ins.opcode;
            if opcodes::is_subroutine(opc) {
                return Err(CfgError::new(format!("不支持的子例程指令 {}（pc={}）", ins.name(), ins.offset)));
            }
            if opcodes::is_cond_branch(opc) || opcodes::is_goto(opc) {
                leaders.insert(self.idx_of(branch_target(ins)?, ins.offset)?);
            } else if opcodes::is_switch(opc) {
                let (default_off, pairs) = switch_targets(ins)?;
                leaders.insert(self.idx_of(default_off, ins.offset)?);
                for (_, off) in pairs {
                    leaders.insert(self.idx_of(off, ins.offset)?);
                }
            } else if !opcodes::is_exit(opc) {
                continue;
            }
            if i + 1 < self.insns.len() {
                leaders.insert(i + 1);
            }
        }
        Ok((leaders, handlers))
    }
}

/// 指令序列 → 基本块列表（id 即列表下标，按字节码顺序）。
///
/// `boundaries`：额外的块边界 pc（catch 体的文本终点：其后的代码属于 try 语句之后）。
pub fn build_blocks(
    insns: &[Insn],
    exception_table: &[ExceptionEntry],
    boundaries: &[u32],
) -> Result<Vec<Block>, CfgError> {
    if insns.is_empty() {
        return Ok(Vec::new());
    }
    let index = Index { off2idx: insns.iter().enumerate().map(|(i, ins)| (ins.offset, i)).collect(), insns };
    let (leaders, handlers) = index.leaders(exception_table, boundaries)?;
    let starts: Vec<usize> = leaders.into_iter().collect();
    let by_start: BTreeMap<usize, NodeId> = starts.iter().enumerate().map(|(n, &s)| (s, n as NodeId)).collect();
    let block_at = |off: u32, pc: u32| -> Result<NodeId, CfgError> {
        let idx = index.idx_of(off, pc)?;
        Ok(by_start[&idx])
    };
    let mut blocks = Vec::with_capacity(starts.len());
    for (n, &s) in starts.iter().enumerate() {
        let e = starts.get(n + 1).copied().unwrap_or(insns.len());
        let last = &insns[e - 1];
        let opc = last.opcode;
        let nxt = by_start.get(&e).copied();
        let term = if opcodes::is_exit(opc) {
            Terminator::Exit
        } else if opcodes::is_goto(opc) {
            Terminator::Goto { pc: last.offset, target: block_at(branch_target(last)?, last.offset)? }
        } else if opcodes::is_cond_branch(opc) {
            let fallthrough =
                nxt.ok_or_else(|| CfgError::new(format!("条件跳转位于方法末尾（pc={}）", last.offset)))?;
            Terminator::Cond { pc: last.offset, target: block_at(branch_target(last)?, last.offset)?, fallthrough }
        } else if opcodes::is_switch(opc) {
            let (default_off, pairs) = switch_targets(last)?;
            let mut grouped: Vec<(Vec<i32>, NodeId)> = Vec::new();
            for (val, off) in pairs {
                let tgt = block_at(off, last.offset)?;
                match grouped.iter_mut().find(|(_, t)| *t == tgt) {
                    Some((vals, _)) => vals.push(val),
                    None => grouped.push((vec![val], tgt)),
                }
            }
            Terminator::Switch { pc: last.offset, cases: grouped, default: block_at(default_off, last.offset)? }
        } else {
            let target =
                nxt.ok_or_else(|| CfgError::new(format!("方法末尾缺少 return/athrow（pc={}）", last.offset)))?;
            Terminator::Fall { target }
        };
        blocks.push(Block {
            id: n as NodeId,
            start_idx: s,
            end_idx: e,
            start_pc: insns[s].offset,
            term,
            is_handler_entry: handlers.contains(&s),
        });
    }
    Ok(blocks)
}

/// 方法的所有出口是否都是 return/athrow（合法字节码恒为真；保留为后处理的判据）。
pub fn function_always_returns(insns: &[Insn]) -> bool {
    insns.is_empty() || build_blocks(insns, &[], &[]).is_ok()
}
