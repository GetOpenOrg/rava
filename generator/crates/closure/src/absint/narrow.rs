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
//! [`Obj::Narrowed`] 标记，类型流改取条件跳转偏移处的收窄节点；该偏移另发 [`Event::MirrorSub`]（引擎把输入值集中
//! 所指类 ⊂ K 的类镜像流入该节点）。不成立一侧不收窄（所指类 ⊄ K 的镜像不构成可表示的值集）。四条指令中后三条
//! 须处于同一基本块。
//!
//! 键判定收窄：`aload k; invokevirtual <键读取>; …name…; invokevirtual <字符串相等>; ifeq/ifne`（或 `…name…; aload k;
//! invokevirtual <键读取>; invokevirtual <字符串相等>; ifeq/ifne`）——按键筛选对象集合（`s.getType().equals(type)`）。
//! 键读取方法由清单 `[facts.keyed_lookups]` 的 `getters` 给出（返回对象的键），字符串相等由 `[facts] value_equals` /
//! `string_ops` 的 `equals_ignore_case` 给出。成立一侧 k 的键等于 name：收窄值保留原值，只打 [`Obj::Narrowed`] 标记，
//! 类型流改取条件跳转偏移处的收窄节点；该偏移另发 [`Event::KeyTest`]（引擎把输入值中键可能等于 name 的对象流入该节点，
//! 键类之外的值原样流入）。不成立一侧不收窄。相等判定的两个操作数取自判定调用前的栈（`pre`），键读取结果按来源
//! （调用偏移）认定；从 `aload k` 到条件跳转须处于同一基本块且其间无局部变量写入（k 仍是被读取键的对象）。
//!
//! null 收窄：`aload k; ifnull/ifnonnull` 两侧各自收窄局部变量 k——为 null 一侧即 null，非 null 一侧
//! 保留原值（来源 / 类型 / 标签不变）并确定非空。两条指令须处于同一基本块。
//!
//! final 字段重读：`aload k; getfield f; ifnull/ifnonnull`（三条同块，f 为 [`Oracle::final_field`]）非 null 一侧
//! 记下「局部 k 所指对象的 f 非空」（[`State::nnf`]）；此后（沿控制流，合流取交集）`aload k; getfield f`（两条同块）
//! 的结果确定非空。依据：
//! - 本线程内：final 实例字段的字节码写入只在声明类 `<init>` 里（JVMS §6.5 putfield），字节码外写入（反射 / Unsafe /
//!   反序列化 / 手写）都要经调用；事实在本帧 `putfield f` 与任一调用处撤掉，所以两次读取之间本线程不会改写它。
//! - 他线程：两次读取之间本线程无调用，他线程的写入与第二次读取之间没有 happens-before 边，第二次读取取到第一次的值
//!   是 JLS §17.4.5 允许的结果；final 字段另有 JLS §17.5.3：实现可以缓存 final 字段的值、不重新读取，即使它在构造后
//!   被反射改写（HotSpot C2 对无中间副作用的同字段读取同样做公共子表达式合并）。因此不看字段的开放判定。
//! 事实在写 k（含宽类型覆盖）、本帧 `putfield f`、任一调用处撤掉，异常处理器入口为空。

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
    let yes = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: Some(Rc::new(Obj::Narrowed(at))) };
    let event = (at, Event::MirrorSub(c.clone(), input.clone()));
    Some(Narrow { slot: k, yes, no: input.clone(), taken, event, event_when: taken })
}

/// 下标 i 处是字符串相等判定、其后紧跟同一基本块内的 ifeq/ifne 时，判定调用前栈顶的两个操作数（接收者, 实参）
pub(super) fn eq_operands(insns: &[Insn], leader: &[bool], i: usize, st: &State, is_eq: impl Fn(&MemberRef) -> bool) -> Option<Vec<V>> {
    let (op::INVOKEVIRTUAL, Operand::Method(mref, _)) = (insns[i].opcode, &insns[i].operand) else { return None };
    let next = insns.get(i + 1)?;
    if !matches!(next.opcode, 0x99 | 0x9a) || leader[i + 1] || !is_eq(mref) || st.stack.len() < 2 {
        return None;
    }
    Some(st.stack[st.stack.len() - 2..].to_vec())
}

/// 下标 i 处条件分支的键判定收窄；`pre` = 前一条指令（字符串相等判定）调用前的两个操作数（见 [`eq_operands`]），
/// `getter` 给出键读取方法的（键类, 键是否不区分大小写），`eq` 给出相等判定是否不区分大小写
pub(super) fn key_test_narrow(
    insns: &[Insn],
    leader: &[bool],
    i: usize,
    st: &State,
    pre: Option<&[V]>,
    getter: impl Fn(&MemberRef) -> Option<(String, bool)>,
    eq: impl Fn(&MemberRef) -> Option<bool>,
) -> Option<Narrow> {
    let taken = match insns[i].opcode {
        0x9a => true,
        0x99 => false,
        _ => return None,
    };
    let [recv, arg] = pre? else { return None };
    if i < 3 || leader[i] || leader[i - 1] {
        return None;
    }
    let (op::INVOKEVIRTUAL, Operand::Method(em, _)) = (insns[i - 1].opcode, &insns[i - 1].operand) else { return None };
    let efold = eq(em)?;
    for (kv, name) in [(recv, arg), (arg, recv)] {
        let V::Ref { src, .. } = kv else { continue };
        let [Src::Site(go)] = src[..] else { continue };
        // 键读取调用：与条件跳转同一基本块（其间无汇入）
        let mut j = i - 1;
        let found = loop {
            if j == 0 || leader[j] {
                break None;
            }
            j -= 1;
            if insns[j].offset == go {
                break (j > 0 && !leader[j]).then_some(j);
            }
        };
        let Some(j) = found else { continue };
        let (op::INVOKEVIRTUAL, Operand::Method(gm, _)) = (insns[j].opcode, &insns[j].operand) else { continue };
        let Some((kc, kfold)) = getter(gm) else { continue };
        let Some(k) = aload_slot(&insns[j - 1]) else { continue };
        // 读取键之后局部变量未被改写：条件跳转处的 k 仍是被读取键的对象
        if insns[j + 1..i].iter().any(|x| (0x36..=0x4e).contains(&x.opcode) || x.opcode == op::IINC) {
            continue;
        }
        let input = st.locals.get(k)?;
        let V::Ref { ty, src, .. } = input else { continue };
        let at = insns[i].offset;
        let yes = V::Ref { ty: ty.clone(), nonnull: true, src: src.clone(), obj: Some(Rc::new(Obj::Narrowed(at))) };
        let event = (at, Event::KeyTest { kc, fold: kfold || efold, input: input.clone(), name: name.clone() });
        return Some(Narrow { slot: k, yes, no: input.clone(), taken, event, event_when: taken });
    }
    None
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

/// 下标 i 处 `aload k; getfield f; ifnull/ifnonnull` 的 final 字段非空判定：返回 (k, f, 非 null 一侧是否为跳转侧)
pub(super) fn final_null_test(insns: &[Insn], leader: &[bool], i: usize, is_final: impl Fn(&MemberRef) -> bool) -> Option<(usize, MemberRef, bool)> {
    let nonnull_taken = match insns[i].opcode {
        op::IFNULL => false,
        op::IFNONNULL => true,
        _ => return None,
    };
    if i < 2 || leader[i] || leader[i - 1] {
        return None;
    }
    let (op::GETFIELD, Operand::Field(f)) = (insns[i - 1].opcode, &insns[i - 1].operand) else { return None };
    let k = aload_slot(&insns[i - 2])?;
    is_final(f).then(|| (k, f.clone(), nonnull_taken))
}

/// 刚执行完下标 i 的 `getfield f`：前一条是同块的 `aload k` 且 (k, f) 已知非空时，栈顶结果确定非空
pub(super) fn final_reread(insns: &[Insn], leader: &[bool], i: usize, st: &mut State) {
    if st.nnf.is_empty() || i < 1 || leader[i] {
        return;
    }
    let (op::GETFIELD, Operand::Field(f)) = (insns[i].opcode, &insns[i].operand) else { return };
    let Some(k) = aload_slot(&insns[i - 1]) else { return };
    if !st.nnf.iter().any(|(j, g)| *j == k && g == f) {
        return;
    }
    if let Some(V::Ref { nonnull, .. }) = st.stack.last_mut() {
        *nonnull = true;
    }
}
