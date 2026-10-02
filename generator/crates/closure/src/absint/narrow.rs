//! instanceof 收窄：`aload k; instanceof C; ifeq/ifne` 成立的一侧，局部变量 k 是 C 的非 null 实例。
//!
//! 收窄后的值以 instanceof 的偏移为来源（同 checkcast：引擎把输入值按 C 过滤后流入该站点），
//! 只在三条指令处于同一基本块时适用（中间无汇入，栈顶判定值即该局部变量的类型测试）。

use classfile::{op, Insn, Operand};

use super::{src1, Src, State, V};

/// 下标 i 处条件分支的收窄：(局部变量槽, 收窄值, 判定成立时是否走跳转目标)
pub(super) fn instanceof_narrow(insns: &[Insn], leader: &[bool], i: usize, st: &State) -> Option<(usize, V, bool)> {
    let taken = match insns[i].opcode {
        0x9a => true,
        0x99 => false,
        _ => return None,
    };
    if i < 2 || leader[i] || leader[i - 1] {
        return None;
    }
    let io = &insns[i - 1];
    let Operand::Class(c) = &io.operand else { return None };
    if io.opcode != op::INSTANCEOF || c.starts_with('[') {
        return None;
    }
    let ld = &insns[i - 2];
    let k = match (ld.opcode, &ld.operand) {
        (0x19, Operand::Local(k)) => *k as usize,
        (0x2a..=0x2d, _) => (ld.opcode - 0x2a) as usize,
        _ => return None,
    };
    let V::Ref { obj, .. } = st.locals.get(k)? else { return None };
    let v = V::Ref { ty: Some(c.as_str().into()), nonnull: true, src: src1(Src::Site(io.offset)), obj: obj.clone() };
    Some((k, v, taken))
}
