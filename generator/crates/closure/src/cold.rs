//! 冷路径：方法体内「必然以抛出结束」的指令（异常构造与错误消息拼装所在的路径）。
//!
//! 指令 i 为冷，当且仅当从 i 出发的每条正常执行路径都在本方法内以 `athrow` 结束，
//! 且覆盖这些指令的异常处理器同样是冷的（被本方法捕获后能正常返回的不算）。
//! 取最小不动点：循环（含无出口循环）不因自身成环而变冷。
//!
//! 冷路径仍是可达代码（运行期抛出的异常属于程序语义），闭包不能据此剪枝；
//! 它是分层产出（冷路径方法按低成本形态生成）的事实来源，见 §6.11 G5。

use classfile::{op, Code, Operand};
use std::collections::HashMap;

/// 按指令下标给出冷标记
pub fn doomed(code: &Code) -> Vec<bool> {
    let insns = &code.insns;
    let n = insns.len();
    let idx: HashMap<u32, usize> = insns.iter().enumerate().map(|(i, x)| (x.offset, i)).collect();
    // 正常后继（下标）；None = 离开方法或后继不可知（return / ret / jsr）
    let succ: Vec<Option<Vec<usize>>> = insns
        .iter()
        .enumerate()
        .map(|(i, ins)| {
            let o = ins.opcode;
            if o == op::ATHROW {
                return Some(vec![]);
            }
            if (op::IRETURN..=op::RETURN).contains(&o) || o == op::RET || o == op::JSR || o == op::JSR_W {
                return None;
            }
            let mut v: Vec<u32> = match &ins.operand {
                Operand::Branch(t) => vec![*t],
                Operand::TableSwitch { default, targets, .. } => targets.iter().copied().chain([*default]).collect(),
                Operand::LookupSwitch { default, pairs } => pairs.iter().map(|p| p.1).chain([*default]).collect(),
                _ => vec![],
            };
            let falls = !classfile::insn::is_terminal(o);
            let mut out: Vec<usize> = v.drain(..).map(|t| idx.get(&t).copied()).collect::<Option<_>>()?;
            if falls {
                out.push(if i + 1 < n { i + 1 } else { return None });
            }
            Some(out)
        })
        .collect();
    let handlers: Vec<Vec<usize>> = insns
        .iter()
        .map(|x| {
            code.exception_table
                .iter()
                .filter(|h| x.offset >= h.start && x.offset < h.end)
                .filter_map(|h| idx.get(&h.handler).copied())
                .collect()
        })
        .collect();
    let mut cold = vec![false; n];
    loop {
        let mut changed = false;
        for i in (0..n).rev() {
            if cold[i] {
                continue;
            }
            let Some(s) = &succ[i] else { continue };
            let is_throw = insns[i].opcode == op::ATHROW;
            if (is_throw || !s.is_empty()) && s.iter().all(|&j| cold[j]) && handlers[i].iter().all(|&h| cold[h]) {
                cold[i] = true;
                changed = true;
            }
        }
        if !changed {
            return cold;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile::{ExceptionEntry, Insn};

    fn ins(offset: u32, opcode: u8, operand: Operand) -> Insn {
        Insn { offset, opcode, operand }
    }

    fn code(insns: Vec<Insn>, ex: Vec<ExceptionEntry>) -> Code {
        Code { max_stack: 4, max_locals: 4, code_len: insns.last().map_or(0, |x| x.offset + 1), insns, exception_table: ex }
    }

    const IFEQ: u8 = 0x99;
    const ALOAD_0: u8 = 0x2a;
    const NOP: u8 = 0x00;

    /// `if (c) { msg...; throw } return`：抛出分支为冷，条件与返回不冷
    #[test]
    fn throw_branch_is_cold() {
        let c = code(
            vec![
                ins(0, IFEQ, Operand::Branch(4)),
                ins(1, NOP, Operand::None),
                ins(2, ALOAD_0, Operand::None),
                ins(3, op::ATHROW, Operand::None),
                ins(4, op::RETURN, Operand::None),
            ],
            vec![],
        );
        assert_eq!(doomed(&c), vec![false, true, true, true, false]);
    }

    /// 被本方法捕获并正常返回的抛出不冷；无出口循环不冷
    #[test]
    fn caught_throw_and_loops_not_cold() {
        let c = code(
            vec![ins(0, ALOAD_0, Operand::None), ins(1, op::ATHROW, Operand::None), ins(2, op::RETURN, Operand::None)],
            vec![ExceptionEntry { start: 0, end: 2, handler: 2, catch_type: None }],
        );
        assert_eq!(doomed(&c), vec![false, false, false]);
        let l = code(vec![ins(0, NOP, Operand::None), ins(1, op::GOTO, Operand::Branch(0))], vec![]);
        assert_eq!(doomed(&l), vec![false, false]);
    }
}
