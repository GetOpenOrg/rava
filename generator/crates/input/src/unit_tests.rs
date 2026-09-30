//! 折叠规范化与 VM 常量剪枝的单元测试（手工构造字节码）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::{op, Code, ExceptionEntry, Insn, MemberRef, Operand};

use crate::facts::{FoldConst, FoldValue, MethodFold, ReadKind};
use crate::norm::{apply_fold, NInsn, POP};
use crate::prune::VmConstants;

const NOP: u8 = 0x00;
const ICONST_0: u8 = 0x03;
const ILOAD_1: u8 = 0x1b;
const IFEQ: u8 = 0x99;
const ASTORE_1: u8 = 0x4c;
const ALOAD_1: u8 = 0x2b;

fn i(offset: u32, opcode: u8) -> Insn {
    Insn { offset, opcode, operand: Operand::None }
}

fn br(offset: u32, opcode: u8, t: u32) -> Insn {
    Insn { offset, opcode, operand: Operand::Branch(t) }
}

fn call(offset: u32, opcode: u8, owner: &str, name: &str, desc: &str) -> Insn {
    let m = MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() };
    Insn { offset, opcode, operand: Operand::Method(m, false) }
}

fn code(insns: Vec<Insn>, code_len: u32, table: Vec<ExceptionEntry>) -> Code {
    Code { max_stack: 2, max_locals: 2, code_len, insns, exception_table: table }
}

fn shape(v: &[NInsn]) -> Vec<(u32, u8)> {
    v.iter()
        .map(|x| match x {
            NInsn::Op(i) => (i.offset, i.opcode),
            NInsn::FoldField { offset, .. } => (*offset, 0),
            NInsn::FoldCall { call, .. } => (call.offset, call.opcode),
        })
        .collect()
}

/// iload_1; ifeq 6; iconst_0; ireturn; [6] iconst_0; ireturn
fn if_code() -> Code {
    code(
        vec![i(0, ILOAD_1), br(1, IFEQ, 6), i(4, ICONST_0), i(5, op::IRETURN), i(6, ICONST_0), i(7, op::IRETURN)],
        8,
        Vec::new(),
    )
}

#[test]
fn branch_always_taken_becomes_pop_goto() {
    let fold = MethodFold { dead_pcs: vec![(4, 6)], ..Default::default() };
    let n = apply_fold("A.m:()I", &if_code(), &fold).unwrap();
    assert_eq!(shape(&n.insns), vec![(0, ILOAD_1), (1, POP), (2, op::GOTO), (6, ICONST_0), (7, op::IRETURN)]);
    assert!(matches!(n.insns[2], NInsn::Op(Insn { operand: Operand::Branch(6), .. })));
}

#[test]
fn branch_never_taken_becomes_pop() {
    let fold = MethodFold { dead_pcs: vec![(6, 8)], ..Default::default() };
    let n = apply_fold("A.m:()I", &if_code(), &fold).unwrap();
    assert_eq!(shape(&n.insns), vec![(0, ILOAD_1), (1, POP), (4, ICONST_0), (5, op::IRETURN)]);
}

#[test]
fn fallthrough_into_dead_is_error() {
    let fold = MethodFold { dead_pcs: vec![(5, 6)], ..Default::default() };
    assert!(apply_fold("A.m:()I", &if_code(), &fold).is_err());
}

#[test]
fn switch_single_live_target() {
    let sw = Insn {
        offset: 1,
        opcode: op::TABLESWITCH,
        operand: Operand::TableSwitch { default: 20, low: 0, high: 1, targets: vec![20, 22] },
    };
    let c = code(vec![i(0, ILOAD_1), sw, i(20, ICONST_0), i(21, op::IRETURN), i(22, ICONST_0), i(23, op::IRETURN)], 24, Vec::new());
    let fold = MethodFold { dead_pcs: vec![(22, 24)], ..Default::default() };
    let n = apply_fold("A.m:()I", &c, &fold).unwrap();
    assert_eq!(shape(&n.insns), vec![(0, ILOAD_1), (1, POP), (2, op::GOTO), (20, ICONST_0), (21, op::IRETURN)]);
}

#[test]
fn const_invoke_keeps_call() {
    let c = code(
        vec![i(0, ALOAD_1), i(1, ICONST_0), call(2, op::INVOKEVIRTUAL, "p/A", "f", "(I)Z"), i(5, op::IRETURN)],
        6,
        Vec::new(),
    );
    let mut consts = BTreeMap::new();
    consts.insert(2, FoldConst { pc: 2, kind: ReadKind::Invoke, value: FoldValue::Bool(true), ty: "Z".into() });
    let fold = MethodFold { consts, ..Default::default() };
    let n = apply_fold("A.m:()Z", &c, &fold).unwrap();
    // 调用保留（被调方副作用不随返回值折叠而丢失），insn() 对扫描方暴露原调用
    assert!(matches!(&n.insns[2], NInsn::FoldCall { call, .. } if call.opcode == op::INVOKEVIRTUAL));
    assert_eq!(n.insns[2].insn().map(|i| i.offset), Some(2));
}

#[test]
fn exception_table_collapses() {
    let table = vec![
        ExceptionEntry { start: 4, end: 6, handler: 6, catch_type: None },
        ExceptionEntry { start: 0, end: 5, handler: 6, catch_type: None },
    ];
    let mut c = if_code();
    c.exception_table = table;
    let fold = MethodFold { dead_pcs: vec![(4, 6)], dead_handlers: BTreeSet::new(), ..Default::default() };
    let n = apply_fold("A.m:()I", &c, &fold).unwrap();
    // [4,6) 全死 → 删除；[0,5) 的 end 收拢到 6
    assert_eq!(n.exception_table.len(), 1);
    assert_eq!((n.exception_table[0].start, n.exception_table[0].end), (0, 6));
}

#[test]
fn prune_null_guard() {
    // invokestatic V.get; astore_1; aload_1; ifnull 12; aload_1; athrow; [12] return
    let c = vec![
        call(0, op::INVOKESTATIC, "p/V", "get", "()Ljava/lang/Object;"),
        i(3, ASTORE_1),
        i(4, ALOAD_1),
        br(5, op::IFNULL, 10),
        i(8, ALOAD_1),
        i(9, op::ATHROW),
        i(10, op::RETURN),
    ];
    let vc = VmConstants { null_returns: ["p/V.get:()Ljava/lang/Object;".to_string()].into(), ..Default::default() };
    let out = vc.prune(c.into_iter().map(NInsn::Op).collect(), &[]);
    assert_eq!(shape(&out), vec![(0, op::INVOKESTATIC), (3, ASTORE_1), (4, ALOAD_1), (5, op::IFNULL), (10, op::RETURN)]);
}

#[test]
fn prune_skips_external_jump_in() {
    let c = vec![
        call(0, op::INVOKESTATIC, "p/V", "get", "()Ljava/lang/Object;"),
        br(3, op::IFNULL, 8),
        i(6, NOP),
        i(7, NOP),
        br(8, op::GOTO, 7),
    ];
    let vc = VmConstants { null_returns: ["p/V.get:()Ljava/lang/Object;".to_string()].into(), ..Default::default() };
    let out = vc.prune(c.into_iter().map(NInsn::Op).collect(), &[]);
    assert_eq!(out.len(), 5);
}
