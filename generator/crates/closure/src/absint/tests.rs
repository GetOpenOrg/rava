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
