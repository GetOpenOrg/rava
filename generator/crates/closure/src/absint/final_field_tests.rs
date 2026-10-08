//! absint 单测：final 实例字段判非 null 后复读的收窄（`narrow.rs` 的 final 字段重读）。

use super::*;
use classfile::Insn;

const ALOAD_0: u8 = 0x2a;

/// 桩 Oracle：实例字段 p/C.v 是 final（可选），调用无结果
struct FinalField(bool);

impl Oracle for FinalField {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn final_field(&self, f: &MemberRef) -> bool {
        self.0 && f.owner == "p/C" && f.name == "v"
    }
}

/// `if (this.v == null) return; <between>; if (this.v == null) X; return;`（`Optional.orElseThrow` 的复读形态）。
/// between 插在复读之前；返回 X 处（最后一条 return）是否可达
fn final_reread_reaches_null_side(final_field: bool, between: Vec<(u8, Operand)>) -> bool {
    let f = MemberRef { owner: "p/C".into(), name: "v".into(), desc: "Ljava/lang/Object;".into() };
    let mut ops: Vec<(u8, Operand, u32)> = vec![
        (ALOAD_0, Operand::None, 1),
        (op::GETFIELD, Operand::Field(f.clone()), 3),
        (0xc7, Operand::Branch(0), 3), // ifnonnull → 复读块（下面回填）
        (op::RETURN, Operand::None, 1),
    ];
    let reread = ops.iter().map(|o| o.2).sum::<u32>();
    for (opc, opd) in between {
        let len = match opc {
            op::INVOKEVIRTUAL | op::PUTFIELD => 3,
            _ => 1,
        };
        ops.push((opc, opd, len));
    }
    ops.push((ALOAD_0, Operand::None, 1));
    ops.push((op::GETFIELD, Operand::Field(f), 3));
    let at = ops.iter().map(|o| o.2).sum::<u32>();
    ops.push((0xc6, Operand::Branch(at + 4), 3)); // ifnull → X
    ops.push((op::RETURN, Operand::None, 1));
    ops.push((op::RETURN, Operand::None, 1)); // X
    ops[2].1 = Operand::Branch(reread);
    let mut off = 0;
    let mut insns = vec![];
    for (opcode, operand, len) in ops {
        insns.push(Insn { offset: off, opcode, operand });
        off += len;
    }
    let code = Code { max_stack: 2, max_locals: 1, code_len: off, insns, exception_table: vec![] };
    let a = analyze("p/C", "()V", false, &code, &FinalField(final_field));
    *a.reachable.last().unwrap()
}

/// final 实例字段判非 null 后同一接收者局部的复读非 null（JLS §17.5.3）；非 final、中间有调用 / 写该字段 /
/// 改写接收者局部时不收窄
#[test]
fn final_field_reread_keeps_nonnull() {
    let m = MemberRef { owner: "p/C".into(), name: "m".into(), desc: "()V".into() };
    let f = MemberRef { owner: "p/C".into(), name: "v".into(), desc: "Ljava/lang/Object;".into() };
    assert!(!final_reread_reaches_null_side(true, vec![]));
    assert!(final_reread_reaches_null_side(false, vec![]));
    assert!(final_reread_reaches_null_side(true, vec![(ALOAD_0, Operand::None), (op::INVOKEVIRTUAL, Operand::Method(m, false))]));
    assert!(final_reread_reaches_null_side(
        true,
        vec![(ALOAD_0, Operand::None), (0x01, Operand::None), (op::PUTFIELD, Operand::Field(f))]
    ));
    // aconst_null; astore_0：接收者局部被改写
    assert!(final_reread_reaches_null_side(true, vec![(0x01, Operand::None), (0x4b, Operand::None)]));
}
