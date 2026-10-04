//! instanceof 收窄：`aload k; instanceof C; ifeq/ifne` 两侧各自收窄局部变量 k。
//!
//! - 成立一侧：k 是 C 的非 null 实例。收窄值以 instanceof 的偏移为来源（同 checkcast：引擎把输入值按 C
//!   过滤后流入该站点）。
//! - 不成立一侧：k 为 null 或不是 C 的实例。收窄值以条件跳转指令的偏移为来源，该偏移另发
//!   [`Event::NotInstance`]（引擎把输入值中 ⊄ C 的部分流入该站点）。
//!
//! 只在三条指令处于同一基本块时适用（中间无汇入，栈顶判定值即该局部变量的类型测试）。
//!
//! 类镜像子类型判定收窄：`ldc K; aload k; invokevirtual <清单 mirror_subtype_tests>; ifeq/ifne`（`K.class.isAssignableFrom(k)`）
//! 成立一侧，k 是所指类 ⊂ K 的类镜像。收窄值保留原值（来源 / 类型不变：按来源上溯的名字 / 类求值照旧），只打
//! [`Obj::MirrorSub`] 标记，类型流改取条件跳转偏移处的收窄节点；该偏移另发 [`Event::MirrorSub`]（引擎把输入值集中
//! 所指类 ⊂ K 的类镜像流入该节点）。不成立一侧不收窄（所指类 ⊄ K 的镜像不构成可表示的值集）。四条指令中后三条
//! 须处于同一基本块。
//!
//! null 收窄：`aload k; ifnull/ifnonnull` 两侧各自收窄局部变量 k——为 null 一侧即 null，非 null 一侧
//! 保留原值（来源 / 类型 / 标签不变）并确定非空。两条指令须处于同一基本块。

use std::rc::Rc;

use classfile::{op, Const, Insn, MemberRef, Operand};

use super::{src1, Event, Obj, Src, State, V};

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
    /// 以事件给出的一侧的来源事件（条件跳转指令偏移, 事件）
    pub event: (u32, Event),
    /// 事件所给的一侧：走跳转目标（true）或顺序后继（false）
    pub event_when: bool,
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
    let k = aload_slot(&insns[i - 2])?;
    let input = st.locals.get(k)?;
    let V::Ref { ty, nonnull, obj, .. } = input else { return None };
    let yes = V::Ref { ty: Some(c.as_str().into()), nonnull: true, src: src1(Src::Site(io.offset)), obj: obj.clone() };
    let at = insns[i].offset;
    let no = V::Ref { ty: ty.clone(), nonnull: *nonnull, src: src1(Src::Site(at)), obj: obj.clone() };
    let event = (at, Event::NotInstance(c.clone(), input.clone()));
    Some(Narrow { slot: k, yes, no, taken, event, event_when: !taken })
}

/// 下标 i 处条件分支的类镜像子类型判定收窄；`is_test` 判定调用的方法是否为清单声明的类镜像子类型判定
pub(super) fn mirror_sub_narrow(insns: &[Insn], leader: &[bool], i: usize, st: &State, is_test: impl Fn(&MemberRef) -> bool) -> Option<Narrow> {
    let taken = match insns[i].opcode {
        0x9a => true,
        0x99 => false,
        _ => return None,
    };
    if i < 3 || leader[i] || leader[i - 1] || leader[i - 2] {
        return None;
    }
    let (op::INVOKEVIRTUAL, Operand::Method(mref, _)) = (insns[i - 1].opcode, &insns[i - 1].operand) else { return None };
    if !is_test(mref) {
        return None;
    }
    let Operand::Ldc(Const::Class(c)) = &insns[i - 3].operand else { return None };
    let k = aload_slot(&insns[i - 2])?;
    let input = st.locals.get(k)?;
    let V::Ref { ty, src, .. } = input else { return None };
    let at = insns[i].offset;
    let yes = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: Some(Rc::new(Obj::MirrorSub(at))) };
    let event = (at, Event::MirrorSub(c.clone(), input.clone()));
    Some(Narrow { slot: k, yes, no: input.clone(), taken, event, event_when: taken })
}

/// 下标 i 处 `aload k; ifnull/ifnonnull` 的两侧收窄：返回 (k, 跳转侧的值, 顺延侧的值)
pub(super) fn null_narrow(insns: &[Insn], leader: &[bool], i: usize, st: &State) -> Option<(usize, V, V)> {
    let null_taken = match insns[i].opcode {
        op::IFNULL => true,
        op::IFNONNULL => false,
        _ => return None,
    };
    if i < 1 || leader[i] {
        return None;
    }
    let k = aload_slot(&insns[i - 1])?;
    let V::Ref { ty, src, obj, .. } = st.locals.get(k)? else { return None };
    let nonnull = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: obj.clone() };
    Some(if null_taken { (k, V::Null, nonnull) } else { (k, nonnull, V::Null) })
}

/// `aload k` / `aload_<k>` 读取的局部变量槽
fn aload_slot(ld: &Insn) -> Option<usize> {
    match (ld.opcode, &ld.operand) {
        (0x19, Operand::Local(k)) => Some(*k as usize),
        (0x2a..=0x2d, _) => Some((ld.opcode - 0x2a) as usize),
        _ => None,
    }
}
