//! 引擎：选择子形参的调用点上下文（计划 boundary-narrowing §6.11 项 9「按调用点常量克隆」）。
//!
//! 选择子形参：静态字节码方法的 int 族形参，其入口值直接作 switch 键 / 条件跳转操作数，或经静态调用原样转给
//! 被调方法的选择子形参（递归）。形如 `getFunction(byte) → createFunction(byte)` 的按编号分派：形参常量按全部
//! 调用点汇合为 Top 后，所有分支都按可达处理，任一调用点的编号都把全部分支带入闭包。
//!
//! 策略：调用点在选择子形参上传常量时，被调方按调用点克隆，链尾接调用方上下文（截断到 HEAP_DEPTH，
//! 同一调用方克隆里的多个调用点各传不同编号时互不汇合）；否则调用方已在某个上下文中时继承之——
//! 常量可能来自调用方克隆上的形参常量，转发链随外层调用点分开。
//! 克隆只细分形参常量，分支判定仍由字节码与形参常量决定。
//!
//! 判定只看字节码（不读分析事实的 Oracle），结果按成员记忆；成环处按记忆帧规则（`memo.rs`）不写入记忆，
//! 与处理次序无关。

use super::sealed::Plain;
use super::*;

impl Ctx<'_> {
    /// 成员 key 的选择子形参（按形参序号的位掩码）
    pub(super) fn selector_slots(&self, key: &MemberRef) -> u64 {
        if let Some(&r) = self.selectors.borrow().get(key) {
            return r;
        }
        let Some(frame) = self.memo_enter(format!("sel:{key}"), true) else { return 0 };
        let r = self.selector_uncached(key);
        if self.memo_leave(frame) {
            self.selectors.borrow_mut().insert(key.clone(), r);
        }
        r
    }

    fn selector_uncached(&self, key: &MemberRef) -> u64 {
        let Some(cf) = self.h.class(&key.owner) else { return 0 };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return 0 };
        if !meth.is_static() || self.kind_of(&cf, meth) != Kind::Bytecode {
            return 0;
        }
        let Some(code) = meth.code.as_ref() else { return 0 };
        let a = absint::analyze(&key.owner, &key.desc, true, code, &Plain);
        if a.conservative {
            return 0;
        }
        let mut out = a.selector_params;
        for (_, e) in &a.events {
            let Event::Invoke { opcode: classfile::op::INVOKESTATIC, mref, iface, args } = e else { continue };
            if !args.iter().any(|v| matches!(v, V::Arg(_))) {
                continue;
            }
            let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) else { continue };
            let (o, n, d) = site.key();
            let callee = MemberRef { owner: o, name: n, desc: d };
            if callee == *key {
                continue;
            }
            out |= forwarded(self.selector_slots(&callee), args);
        }
        out
    }
}

/// 实参原样转给被调方选择子形参（掩码 mask）的本方法形参
fn forwarded(mask: u64, args: &[V]) -> u64 {
    let mut out = 0;
    for (j, v) in args.iter().enumerate() {
        if let V::Arg(i) = v {
            if j < 64 && *i < 64 && mask & (1 << j) != 0 {
                out |= 1 << i;
            }
        }
    }
    out
}

/// 选择子形参上有常量实参
pub(super) fn const_selector(mask: u64, args: &[V]) -> bool {
    args.iter().enumerate().any(|(j, v)| j < 64 && mask & (1 << j) != 0 && matches!(v, V::Int(_)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile::Operand;

    #[test]
    fn forwarding_maps_callee_slots_to_own_params() {
        // 被调方形参 1 是选择子；本方法形参 2 原样传到该位置
        let args = [V::Int(3), V::Arg(2), V::Top];
        assert_eq!(forwarded(0b10, &args), 0b100);
        assert_eq!(forwarded(0b01, &args), 0);
        assert!(const_selector(0b01, &args));
        assert!(!const_selector(0b10, &args));
    }

    fn switch_code(prefix: Vec<(u8, Operand)>) -> classfile::Code {
        use classfile::{op, Code, Insn};
        let mut insns: Vec<Insn> = prefix.into_iter().enumerate().map(|(k, (opcode, operand))| Insn { offset: k as u32, opcode, operand }).collect();
        let n = insns.len() as u32;
        insns.push(Insn { offset: n, opcode: op::TABLESWITCH, operand: Operand::TableSwitch { default: 40, low: 0, high: 1, targets: vec![38, 39] } });
        for o in [38, 39, 40] {
            insns.push(Insn { offset: o, opcode: op::RETURN, operand: Operand::None });
        }
        Code { max_stack: 2, max_locals: 1, code_len: 41, insns, exception_table: vec![] }
    }

    #[test]
    fn switch_on_param_marks_selector() {
        // static (I)V：iload_0；tableswitch
        let a = absint::analyze("p/X", "(I)V", true, &switch_code(vec![(0x1a, Operand::None)]), &Plain);
        assert_eq!(a.selector_params, 1);
        // 形参先参与运算（iload_0；iconst_1；iadd）：不是选择子
        let code = switch_code(vec![(0x1a, Operand::None), (0x04, Operand::None), (0x60, Operand::None)]);
        let a = absint::analyze("p/X", "(I)V", true, &code, &Plain);
        assert_eq!(a.selector_params, 0);
    }
}
