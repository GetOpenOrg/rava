//! VM 常量守卫的死分支剪除，在折叠之后执行。
//!
//! 原生二进制中部分 VM 边界方法返回恒定值（清单 vm_intrinsics.toml `[vm_constants]`），
//! 以其为守卫的分支恒不执行。识别形态（均为恒跳转，剪除分支指令与跳转目标之间的直线段）：
//!   `<null 调用> [astore k; aload k] ifnull L`
//!   `<null 调用> [astore k; aload k] <null→false 调用> ifeq L`
//! 安全条件：段外无跳入段内的目标、异常表端点不落在段内。

use std::collections::{BTreeSet, HashSet};

use classfile::{op, ExceptionEntry, Insn, Operand};

use crate::norm::{switch_targets, NInsn};

const ALOAD: u8 = 0x19;
const ALOAD_0: u8 = 0x2a;
const ALOAD_3: u8 = 0x2d;
const ASTORE: u8 = 0x3a;
const ASTORE_0: u8 = 0x4b;
const ASTORE_3: u8 = 0x4e;
const IFEQ: u8 = 0x99;

/// 清单中的 VM 常量（`类.方法:描述符`）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VmConstants {
    /// 恒返回 null 的 VM 边界方法
    pub null_returns: BTreeSet<String>,
    /// null 实参 → false 的纯函数
    pub null_to_false: BTreeSet<String>,
}

fn call_ref(ins: &NInsn) -> Option<String> {
    match ins.insn()? {
        Insn { opcode: op::INVOKESTATIC, operand: Operand::Method(m, _), .. } => Some(m.to_string()),
        _ => None,
    }
}

fn slot(ins: &NInsn, plain: u8, first: u8, last: u8) -> Option<u16> {
    let i = ins.insn()?;
    if i.opcode == plain {
        return match i.operand {
            Operand::Local(n) => Some(n),
            _ => None,
        };
    }
    (first..=last).contains(&i.opcode).then(|| u16::from(i.opcode - first))
}

fn branch_target(ins: &NInsn, opcode: u8) -> Option<u32> {
    match ins.insn()? {
        Insn { opcode: o, operand: Operand::Branch(t), .. } if *o == opcode => Some(*t),
        _ => None,
    }
}

/// 跳转类指令的全部目标（条件跳转 / goto / jsr / switch）
fn jump_targets(ins: &NInsn) -> Vec<u32> {
    let Some(i) = ins.insn() else { return Vec::new() };
    match &i.operand {
        Operand::Branch(t) => vec![*t],
        o @ (Operand::TableSwitch { .. } | Operand::LookupSwitch { .. }) => switch_targets(o),
        _ => Vec::new(),
    }
}

impl VmConstants {
    /// 恒跳转分支的 (分支指令下标, 目标 pc)
    fn dead_ranges(&self, insns: &[NInsn]) -> Vec<(usize, u32)> {
        if self.null_returns.is_empty() {
            return Vec::new();
        }
        let n = insns.len();
        let mut out = Vec::new();
        for (i, ins) in insns.iter().enumerate() {
            if !call_ref(ins).is_some_and(|r| self.null_returns.contains(&r)) {
                continue;
            }
            let mut j = i + 1;
            if j + 1 < n {
                let st = slot(&insns[j], ASTORE, ASTORE_0, ASTORE_3);
                if st.is_some() && st == slot(&insns[j + 1], ALOAD, ALOAD_0, ALOAD_3) {
                    j += 2;
                }
            }
            if let Some(t) = insns.get(j).and_then(|x| branch_target(x, op::IFNULL)) {
                out.push((j, t));
            } else if j + 1 < n && call_ref(&insns[j]).is_some_and(|r| self.null_to_false.contains(&r)) {
                if let Some(t) = branch_target(&insns[j + 1], IFEQ) {
                    out.push((j + 1, t));
                }
            }
        }
        out
    }

    /// 剪除 VM 常量守卫恒不执行的分支体；不满足安全条件的形态原样保留
    pub fn prune(&self, insns: Vec<NInsn>, table: &[ExceptionEntry]) -> Vec<NInsn> {
        let ranges = self.dead_ranges(&insns);
        if ranges.is_empty() {
            return insns;
        }
        let offsets: HashSet<u32> = insns.iter().map(NInsn::offset).collect();
        let mut drop: BTreeSet<usize> = BTreeSet::new();
        for (bi, target) in ranges {
            let lo = insns[bi].offset();
            if target <= lo || !offsets.contains(&target) {
                continue;
            }
            let inside = |pc: u32| lo < pc && pc < target;
            let ext = insns
                .iter()
                .filter(|x| !inside(x.offset()))
                .any(|x| jump_targets(x).into_iter().any(inside));
            let in_table = table.iter().any(|e| inside(e.start) || inside(e.end) || inside(e.handler));
            if ext || in_table {
                continue;
            }
            drop.extend(insns.iter().enumerate().filter(|(_, x)| inside(x.offset())).map(|(k, _)| k));
        }
        if drop.is_empty() {
            return insns;
        }
        insns.into_iter().enumerate().filter(|(k, _)| !drop.contains(k)).map(|(_, x)| x).collect()
    }
}
