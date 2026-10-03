//! instanceof 收窄：`aload k; instanceof C; ifeq/ifne` 两侧各自收窄局部变量 k。
//!
//! - 成立一侧：k 是 C 的非 null 实例。收窄值以 instanceof 的偏移为来源（同 checkcast：引擎把输入值按 C
//!   过滤后流入该站点）。
//! - 不成立一侧：k 为 null 或不是 C 的实例。收窄值以条件跳转指令的偏移为来源，该偏移另发
//!   [`Event::NotInstance`]（引擎把输入值中 ⊄ C 的部分流入该站点）。
//!
//! 只在三条指令处于同一基本块时适用（中间无汇入，栈顶判定值即该局部变量的类型测试）。

use classfile::{op, Insn, Operand};

use super::{src1, Event, Src, State, V};

/// 条件分支处的 instanceof 收窄
pub(super) struct Narrow {
    /// 被测局部变量槽
    pub slot: usize,
    /// 判定成立一侧的值
    pub yes: V,
    /// 判定不成立一侧的值
    pub no: V,
    /// 判定成立时是否走跳转目标
    pub taken: bool,
    /// 不成立一侧的来源事件（条件跳转指令偏移, 事件）
    pub event: (u32, Event),
}

impl Narrow {
    /// 走跳转目标（taken）或顺序后继一侧的局部变量值
    pub fn side(&self, taken: bool) -> &V {
        if taken == self.taken {
            &self.yes
        } else {
            &self.no
        }
    }
}

/// 下标 i 处条件分支的收窄
pub(super) fn instanceof_narrow(insns: &[Insn], leader: &[bool], i: usize, st: &State) -> Option<Narrow> {
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
    let input = st.locals.get(k)?;
    let V::Ref { ty, nonnull, obj, .. } = input else { return None };
    let yes = V::Ref { ty: Some(c.as_str().into()), nonnull: true, src: src1(Src::Site(io.offset)), obj: obj.clone() };
    let at = insns[i].offset;
    let no = V::Ref { ty: ty.clone(), nonnull: *nonnull, src: src1(Src::Site(at)), obj: obj.clone() };
    let event = (at, Event::NotInstance(c.clone(), input.clone()));
    Some(Narrow { slot: k, yes, no, taken, event })
}
