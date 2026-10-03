//! absint 单测：以桩 Oracle 驱动的分支折叠规则。

use super::*;
use classfile::Insn;

/// 桩 Oracle：`live` 中的类型有实例，其余没有
struct Stub {
    live: Vec<&'static str>,
}

impl Oracle for Stub {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, ty: &str) -> bool {
        self.live.contains(&ty)
    }
}

const ALOAD_0: u8 = 0x2a;
const IFEQ: u8 = 0x99;

/// `static void f(Object o) { if (o instanceof X) { throw (Throwable) o; } }`
fn instanceof_code() -> Code {
    let i = |offset, opcode, operand| Insn { offset, opcode, operand };
    let insns = vec![
        i(0, ALOAD_0, Operand::None),
        i(1, op::INSTANCEOF, Operand::Class("p/X".into())),
        i(4, IFEQ, Operand::Branch(9)),
        i(7, ALOAD_0, Operand::None),
        i(8, op::ATHROW, Operand::None),
        i(9, op::RETURN, Operand::None),
    ];
    Code { max_stack: 2, max_locals: 1, code_len: 10, insns, exception_table: vec![] }
}

/// 目标类型无实例：instanceof 恒 false，真分支不可达，类型登记为待定（存活后重分析）
#[test]
fn instanceof_dead_type_folds() {
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &instanceof_code(), &Stub { live: vec![] });
    assert!(!a.conservative);
    assert_eq!(a.reachable, vec![true, true, true, false, false, true]);
    assert_eq!(a.pending_types, vec!["p/X".to_string()]);
}

/// 目标类型有实例：两支都可达，无待定类型
#[test]
fn instanceof_live_type_keeps_both() {
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &instanceof_code(), &Stub { live: vec!["p/X"] });
    assert_eq!(a.reachable, vec![true; 6]);
    assert!(a.pending_types.is_empty());
}

/// instanceof 成立一侧：被测局部变量以 instanceof 偏移为来源、类型收窄为目标类型（throw 的值即收窄值）
#[test]
fn instanceof_narrows_taken_side() {
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &instanceof_code(), &Stub { live: vec!["p/X"] });
    let input = a.events.iter().find_map(|(o, e)| match e {
        Event::InstanceOf(c, Some(v)) if *o == 1 && c == "p/X" => Some(v.clone()),
        _ => None,
    });
    assert!(matches!(input, Some(V::Ref { ref src, .. }) if src.as_ref() == [Src::Param(0)]));
    let thrown = a.events.iter().find_map(|(o, e)| match e {
        Event::Throw(v) if *o == 8 => Some(v.clone()),
        _ => None,
    });
    match thrown {
        Some(V::Ref { ty, nonnull, src, .. }) => {
            assert_eq!(ty.as_deref(), Some("p/X"));
            assert!(nonnull);
            assert_eq!(src.as_ref(), [Src::Site(1)]);
        }
        other => panic!("收窄值 {other:?}"),
    }
}

/// instanceof 不成立一侧：`if (!(o instanceof X)) throw (Throwable) o;`——被测局部变量以条件跳转偏移为来源，
/// 该偏移发 NotInstance（输入为原值）；类型与非 null 性不变
#[test]
fn instanceof_narrows_else_side() {
    let mut code = instanceof_code();
    code.insns[2].opcode = 0x9a;
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &code, &Stub { live: vec!["p/X"] });
    let input = a.events.iter().find_map(|(o, e)| match e {
        Event::NotInstance(c, v) if *o == 4 && c == "p/X" => Some(v.clone()),
        _ => None,
    });
    assert!(matches!(input, Some(V::Ref { ref src, .. }) if src.as_ref() == [Src::Param(0)]));
    let thrown = a.events.iter().find_map(|(o, e)| match e {
        Event::Throw(v) if *o == 8 => Some(v.clone()),
        _ => None,
    });
    match thrown {
        Some(V::Ref { ty, nonnull, src, .. }) => {
            assert_eq!(ty.as_deref(), Some("java/lang/Object"));
            assert!(!nonnull);
            assert_eq!(src.as_ref(), [Src::Site(4)]);
        }
        other => panic!("收窄值 {other:?}"),
    }
}

/// 目标类型无实例（判定恒 false）：只剩不成立一侧，照发 NotInstance
#[test]
fn instanceof_dead_type_still_has_else_side() {
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &instanceof_code(), &Stub { live: vec![] });
    assert!(a.events.iter().any(|(o, e)| *o == 4 && matches!(e, Event::NotInstance(..))));
}

/// 桩 Oracle：形参 0 的类镜像值集（None = 未知）
struct Mirrors(Option<Vec<&'static str>>);

impl Oracle for Mirrors {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn param_mirror(&self, i: u16, cls: &str) -> Option<bool> {
        (i == 0).then(|| self.0.as_ref().map(|s| s.contains(&cls))).flatten()
    }
}

/// `static void f(Class c) { if (c == X.class) throw …; }`
fn class_eq_code() -> Code {
    let i = |offset, opcode, operand| Insn { offset, opcode, operand };
    let insns = vec![
        i(0, ALOAD_0, Operand::None),
        i(1, op::LDC, Operand::Ldc(classfile::Const::Class("p/X".into()))),
        i(3, 0xa6, Operand::Branch(8)),
        i(6, ALOAD_0, Operand::None),
        i(7, op::ATHROW, Operand::None),
        i(8, op::RETURN, Operand::None),
    ];
    Code { max_stack: 2, max_locals: 1, code_len: 9, insns, exception_table: vec![] }
}

/// 形参值集不含该类镜像：相等分支不可达，登记乐观答复；含或未知：两支都可达
#[test]
fn class_literal_eq_folds_by_param_mirrors() {
    let a = analyze("p/A", "(Lp/C;)V", true, &class_eq_code(), &Mirrors(Some(vec!["p/Y"])));
    assert_eq!(a.reachable, vec![true, true, true, false, false, true]);
    assert_eq!(a.mirror_assumed, vec![(0, "p/X".to_string())]);
    let a = analyze("p/A", "(Lp/C;)V", true, &class_eq_code(), &Mirrors(Some(vec!["p/X"])));
    assert_eq!(a.reachable, vec![true; 6]);
    assert!(a.mirror_assumed.is_empty());
    let a = analyze("p/A", "(Lp/C;)V", true, &class_eq_code(), &Mirrors(None));
    assert_eq!(a.reachable, vec![true; 6]);
}

/// 桩 Oracle：形参 0 取整数集 {0, 256}；`p/A.F:I` 为 static final 字段
struct Sets;

impl Oracle for Sets {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        false
    }
    fn param(&self, i: u16) -> Option<V> {
        (i == 0).then(|| V::Ints(Rc::from([0, 256].as_slice())))
    }
    fn final_static(&self, f: &MemberRef) -> bool {
        f.owner == "p/A" && f.name == "F"
    }
}

fn code_of(insns: Vec<(u32, u8, Operand)>, len: u32) -> Code {
    let insns = insns.into_iter().map(|(offset, opcode, operand)| Insn { offset, opcode, operand }).collect();
    Code { max_stack: 2, max_locals: 1, code_len: len, insns, exception_table: vec![] }
}

/// `(flags & 2) == 2`，flags ∈ {0, 256}：逐值运算后恒不等，相等分支不可达（Formatter$Flags.contains 形态）
#[test]
fn int_set_arithmetic_decides_branch() {
    let code = code_of(
        vec![
            (0, 0x1a, Operand::None),         // iload_0
            (1, 0x05, Operand::None),         // iconst_2
            (2, 0x7e, Operand::None),         // iand
            (3, 0x05, Operand::None),         // iconst_2
            (4, 0xa0, Operand::Branch(8)),    // if_icmpne
            (7, op::RETURN, Operand::None),
            (8, op::RETURN, Operand::None),
        ],
        9,
    );
    let a = analyze("p/A", "(I)V", true, &code, &Sets);
    assert_eq!(a.reachable, vec![true, true, true, true, true, false, true]);
}

/// 同一帧内写入 static final 字段之后的读取取写入值（`<clinit>` 里后续字段由前一字段派生的形态）
#[test]
fn final_static_read_after_put() {
    let f = MemberRef { owner: "p/A".into(), name: "F".into(), desc: "I".into() };
    let code = code_of(
        vec![
            (0, 0x06, Operand::None), // iconst_3
            (1, op::PUTSTATIC, Operand::Field(f.clone())),
            (4, op::GETSTATIC, Operand::Field(f)),
            (7, 0x9a, Operand::Branch(11)), // ifne
            (10, op::RETURN, Operand::None),
            (11, op::RETURN, Operand::None),
        ],
        12,
    );
    let a = analyze("p/A", "()V", true, &code, &Sets);
    assert_eq!(a.reachable, vec![true, true, true, true, false, true]);
}

/// 桩 Oracle：static 字段 p/A.OFF 的值是符号偏移
struct Offsets;

impl Oracle for Offsets {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, f: &MemberRef, _: Option<&V>) -> Option<V> {
        (f.name == "OFF").then(|| V::Offset(Rc::new(MemberRef { owner: "p/N".into(), name: "next".into(), desc: "Lp/N;".into() })))
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
}

/// 符号偏移经字段读原样作为调用实参；参与运算（数组下标式偏移）即为 Top
#[test]
fn symbolic_offset_flows_to_call_and_dies_in_arithmetic() {
    let off = MemberRef { owner: "p/A".into(), name: "OFF".into(), desc: "J".into() };
    let put = MemberRef { owner: "p/U".into(), name: "put".into(), desc: "(Ljava/lang/Object;JLjava/lang/Object;)V".into() };
    let i = |offset, opcode, operand| Insn { offset, opcode, operand };
    let insns = vec![
        i(0, ALOAD_0, Operand::None),
        i(1, op::GETSTATIC, Operand::Field(off.clone())),
        i(4, 0x01, Operand::None), // aconst_null
        i(5, op::INVOKESTATIC, Operand::Method(put.clone(), false)),
        i(8, ALOAD_0, Operand::None),
        i(9, op::GETSTATIC, Operand::Field(off)),
        i(12, 0x0a, Operand::None), // lconst_1
        i(13, 0x61, Operand::None), // ladd
        i(14, 0x01, Operand::None), // aconst_null
        i(15, op::INVOKESTATIC, Operand::Method(put, false)),
        i(18, op::RETURN, Operand::None),
    ];
    let code = Code { max_stack: 6, max_locals: 1, code_len: 19, insns, exception_table: vec![] };
    let a = analyze("p/A", "(Ljava/lang/Object;)V", true, &code, &Offsets);
    let arg = |at: u32| {
        a.events.iter().find_map(|(o, e)| match e {
            Event::Invoke { args, .. } if *o == at => Some(args[1].clone()),
            _ => None,
        })
    };
    assert!(matches!(arg(5), Some(V::Offset(f)) if f.name == "next"));
    assert_eq!(arg(15), Some(V::Top));
}
