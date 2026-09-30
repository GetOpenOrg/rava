//! cfg 共用的 JVM opcode 分类（← `cfg/_opcodes.py`）。
//!
//! Python 以指令名字符串集合表达；这里按 opcode 字节判定（JVMS §6.5 编号），
//! 条件跳转另有结构化的 [`BranchCond`]。

use classfile::insn::op;

pub const IFEQ: u8 = 0x99;
pub const IFNE: u8 = 0x9a;
pub const IFLT: u8 = 0x9b;
pub const IFGE: u8 = 0x9c;
pub const IFGT: u8 = 0x9d;
pub const IFLE: u8 = 0x9e;
pub const IF_ICMPEQ: u8 = 0x9f;
pub const IF_ICMPNE: u8 = 0xa0;
pub const IF_ICMPLT: u8 = 0xa1;
pub const IF_ICMPGE: u8 = 0xa2;
pub const IF_ICMPGT: u8 = 0xa3;
pub const IF_ICMPLE: u8 = 0xa4;
pub const IF_ACMPEQ: u8 = 0xa5;
pub const IF_ACMPNE: u8 = 0xa6;

/// 双操作数条件跳转（`if_icmpXX` / `if_acmpXX`）
pub fn is_two_operand_branch(opc: u8) -> bool {
    (IF_ICMPEQ..=IF_ACMPNE).contains(&opc)
}

/// 单操作数条件跳转（`ifXX` / `ifnull` / `ifnonnull`）
pub fn is_one_operand_branch(opc: u8) -> bool {
    (IFEQ..=IFLE).contains(&opc) || opc == op::IFNULL || opc == op::IFNONNULL
}

pub fn is_cond_branch(opc: u8) -> bool {
    is_two_operand_branch(opc) || is_one_operand_branch(opc)
}

pub fn is_goto(opc: u8) -> bool {
    opc == op::GOTO || opc == op::GOTO_W
}

pub fn is_switch(opc: u8) -> bool {
    opc == op::TABLESWITCH || opc == op::LOOKUPSWITCH
}

/// 返回 / 抛出（所有退出指令）
pub fn is_exit(opc: u8) -> bool {
    (op::IRETURN..=op::RETURN).contains(&opc) || opc == op::ATHROW
}

/// 子例程指令（class 文件版本 < 50 的 finally 实现）：不支持，生成期报错
pub fn is_subroutine(opc: u8) -> bool {
    opc == op::JSR || opc == op::JSR_W || opc == op::RET
}

/// 所有需要被结构化消费的跳转指令（自检口径）
pub fn is_jump(opc: u8) -> bool {
    is_cond_branch(opc) || is_goto(opc) || is_switch(opc)
}
