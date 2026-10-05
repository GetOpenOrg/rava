//! 引导求值根帧的控制流事实：指令级后支配树（残差区段的终点）与延迟值分支的栈形态。
//!
//! 图的节点是根方法的指令下标，另加一个出口节点（返回 / athrow）；边含异常边（try 区间内每条指令到处理器）。
//! 残差区段 `[s, m)` 的终点 m 取依赖延迟值的分支 b 的直接后支配点：从 b 出发的全部路径都经过 m。

use classfile::{Code, Insn, Operand};

use super::interp::{nparams, returns_void};
use super::vm::*;
use super::*;

/// 根方法的后支配关系（位集迭代求不动点）
pub(super) struct PDom {
    n: usize,
    words: usize,
    sets: Vec<Vec<u64>>,
}

/// 条件分支 / 多路分支（目标依赖栈顶值）
pub(super) fn is_branch(op: u8) -> bool {
    matches!(op, 0x99..=0xa6 | 0xc6 | 0xc7 | 0xaa | 0xab)
}

/// 分支指令弹出的操作数个数
fn branch_pops(op: u8) -> usize {
    match op {
        0x9f..=0xa6 => 2,
        _ => 1,
    }
}

/// 产生延迟值的指令的栈效果（弹出, 压入）：只认产生单个值的指令
fn producer_effect(insn: &Insn) -> Option<usize> {
    match (insn.opcode, &insn.operand) {
        (0xb6..=0xb9, Operand::Method(m, _)) if !returns_void(&m.desc) => Some(nparams(&m.desc) + usize::from(insn.opcode != 0xb8)),
        (0xb2, _) => Some(0),
        (0xb4 | 0xbe | 0xc0 | 0xc1, _) => Some(1),
        (0x2e..=0x35, _) => Some(2),
        _ => None,
    }
}

fn idx(index: &HashMap<u32, usize>, off: u32) -> R<usize> {
    index.get(&off).copied().map_or_else(|| fail("分支目标非指令边界"), Ok)
}

/// 指令的正常后继（下标；`n` = 出口）
pub(super) fn succs(code: &Code, index: &HashMap<u32, usize>, i: usize) -> R<Vec<usize>> {
    let insn = &code.insns[i];
    let n = code.insns.len();
    Ok(match (insn.opcode, &insn.operand) {
        (0x99..=0xa6 | 0xc6 | 0xc7, Operand::Branch(t)) => vec![i + 1, idx(index, *t)?],
        (0xa7 | 0xc8 | 0xa8 | 0xc9, Operand::Branch(t)) => vec![idx(index, *t)?],
        (0xaa, Operand::TableSwitch { default, targets, .. }) => {
            let mut v = vec![idx(index, *default)?];
            for t in targets {
                v.push(idx(index, *t)?);
            }
            v
        }
        (0xab, Operand::LookupSwitch { default, pairs }) => {
            let mut v = vec![idx(index, *default)?];
            for (_, t) in pairs {
                v.push(idx(index, *t)?);
            }
            v
        }
        (0xac..=0xb1 | 0xbf | 0xa9, _) => vec![n],
        _ => vec![i + 1],
    })
}

impl PDom {
    pub(super) fn new(code: &Code, index: &HashMap<u32, usize>) -> R<PDom> {
        let n = code.insns.len();
        let words = (n + 1).div_ceil(64);
        let mut succ: Vec<Vec<usize>> = Vec::with_capacity(n);
        for (i, insn) in code.insns.iter().enumerate() {
            let mut s = succs(code, index, i)?;
            for e in &code.exception_table {
                if e.start <= insn.offset && insn.offset < e.end {
                    s.push(idx(index, e.handler)?);
                }
            }
            s.sort_unstable();
            s.dedup();
            succ.push(s);
        }
        let full = vec![u64::MAX; words];
        let mut sets = vec![full; n + 1];
        sets[n] = vec![0; words];
        sets[n][n / 64] |= 1 << (n % 64);
        let mut changed = true;
        while changed {
            changed = false;
            for i in (0..n).rev() {
                let mut new = vec![u64::MAX; words];
                for &s in &succ[i] {
                    for (w, x) in new.iter_mut().zip(&sets[s]) {
                        *w &= x;
                    }
                }
                new[i / 64] |= 1 << (i % 64);
                if new != sets[i] {
                    sets[i] = new;
                    changed = true;
                }
            }
        }
        Ok(PDom { n, words, sets })
    }

    fn has(&self, set: &[u64], j: usize) -> bool {
        set[j / 64] & (1 << (j % 64)) != 0
    }

    /// 直接后支配点（None = 出口）
    pub(super) fn ipdom(&self, i: usize) -> Option<usize> {
        let mut strict = self.sets[i].clone();
        strict[i / 64] &= !(1 << (i % 64));
        let card = |s: &[u64]| -> u32 { s.iter().map(|w| w.count_ones()).sum() };
        // 位集补全的高位（> n）在全集初值中为 1：只数 0..=n
        let live = |s: &[u64]| -> Vec<u64> {
            let mut v = s.to_vec();
            let top = self.n + 1;
            if top % 64 != 0 {
                v[self.words - 1] &= (1u64 << (top % 64)) - 1;
            }
            v
        };
        let strict = live(&strict);
        let k = card(&strict);
        (0..=self.n).find(|&d| self.has(&strict, d) && card(&live(&self.sets[d])) == k).filter(|&d| d < self.n)
    }
}

/// 延迟值在根帧指令 x 处参与求值（执行前栈深 depth）：依赖它的分支下标与分支后保留的栈深
pub(super) fn fork_shape(code: &Code, x: usize, depth: usize) -> Option<(usize, usize)> {
    let insn = &code.insns[x];
    if is_branch(insn.opcode) {
        return Some((x, depth.checked_sub(branch_pops(insn.opcode))?));
    }
    let pops = producer_effect(insn)?;
    let b = code.insns.get(x + 1)?;
    if !is_branch(b.opcode) {
        return None;
    }
    Some((x + 1, (depth + 1).checked_sub(pops + branch_pops(b.opcode))?))
}
