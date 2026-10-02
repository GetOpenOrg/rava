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
