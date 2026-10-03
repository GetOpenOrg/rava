//! sealed 单测：合成字节码（虚构类名）上的值映射写入、常量数组写入与构建器链首判定。

use super::*;
use super::super::builder::builder_prefix;
use crate::manifest::{NameFacts, ValueMaps};
use classfile::{op, Code, Insn, Operand};

const NOP: u8 = 0x00;
const ACONST_NULL: u8 = 0x01;
const ICONST_0: u8 = 0x03;
const ICONST_1: u8 = 0x04;
const ICONST_2: u8 = 0x05;
const ILOAD_0: u8 = 0x1a;
const ALOAD_0: u8 = 0x2a;
const ALOAD_1: u8 = 0x2b;
const ASTORE_1: u8 = 0x4c;
const AALOAD: u8 = 0x32;
const AASTORE: u8 = 0x53;
const POP: u8 = 0x57;
const DUP: u8 = 0x59;
const IFEQ: u8 = 0x99;

fn i(offset: u32, opcode: u8, operand: Operand) -> Insn {
    Insn { offset, opcode, operand }
}

fn mref(owner: &str, name: &str, desc: &str) -> MemberRef {
    MemberRef { owner: owner.into(), name: name.into(), desc: desc.into() }
}

fn ldc(s: &str) -> Operand {
    Operand::Ldc(Const::String(s.into()))
}

fn code(insns: Vec<Insn>) -> Code {
    let code_len = insns.last().map_or(0, |x| x.offset + 1);
    Code { max_stack: 6, max_locals: 2, code_len, insns, exception_table: vec![] }
}

fn run(c: &Code) -> Analysis {
    let a = absint::analyze("a/H", "(La/M;)V", true, c, &Plain);
    assert!(!a.conservative);
    a
}

fn maps() -> ValueMaps {
    ValueMaps {
        classes: ["a/M".to_string()].into(),
        writers: ["put:(La/K;La/K;)La/K;".to_string()].into(),
        readers: ["get:(La/K;)La/K;".to_string()].into(),
    }
}

fn map_field() -> MemberRef {
    mref("a/H", "m", "La/M;")
}

/// `m = new M(); m.put("k", "v"); [leak(m);] H.m = m;`
fn map_code(leak: bool) -> Code {
    let put = mref("a/M", "put", "(La/K;La/K;)La/K;");
    let mut v = vec![
        i(0, op::NEW, Operand::Class("a/M".into())),
        i(3, DUP, Operand::None),
        i(4, op::INVOKESPECIAL, Operand::Method(mref("a/M", "<init>", "()V"), false)),
        i(7, DUP, Operand::None),
        i(8, op::LDC, ldc("k")),
        i(10, op::LDC, ldc("v")),
        i(12, op::INVOKEVIRTUAL, Operand::Method(put, false)),
        i(15, POP, Operand::None),
    ];
    if leak {
        v.push(i(16, DUP, Operand::None));
        v.push(i(17, op::INVOKESTATIC, Operand::Method(mref("a/H", "leak", "(La/M;)V"), false)));
    } else {
        v.push(i(16, NOP, Operand::None));
        v.push(i(17, NOP, Operand::None));
    }
    v.push(i(20, op::PUTSTATIC, Operand::Field(map_field())));
    v.push(i(23, op::RETURN, Operand::None));
    code(v)
}

/// 映射对象不逃逸：写入值即候选
#[test]
fn map_writes_collects_values() {
    let a = run(&map_code(false));
    assert_eq!(map_writes(&a, 0, &map_field(), &maps()), Some(vec![V::Str("v".into())]));
}

/// 映射对象被传出（可能被嵌套外写入）：不给候选；类不在清单内同样不给
#[test]
fn map_writes_rejects_escape_and_unlisted() {
    let a = run(&map_code(true));
    assert_eq!(map_writes(&a, 0, &map_field(), &maps()), None);
    let a = run(&map_code(false));
    assert_eq!(map_writes(&a, 0, &map_field(), &ValueMaps::default()), None);
}

fn array_field() -> MemberRef {
    mref("a/H", "g", "[La/S;")
}

/// `g = new S[]{"x", null};`，`param` 时第二个元素改存参数
fn array_code(param: bool) -> Code {
    let second = if param { i(11, ALOAD_0, Operand::None) } else { i(11, ACONST_NULL, Operand::None) };
    code(vec![
        i(0, ICONST_2, Operand::None),
        i(1, op::ANEWARRAY, Operand::Class("a/S".into())),
        i(4, DUP, Operand::None),
        i(5, ICONST_0, Operand::None),
        i(6, op::LDC, ldc("x")),
        i(8, AASTORE, Operand::None),
        i(9, DUP, Operand::None),
        i(10, ICONST_1, Operand::None),
        second,
        i(12, AASTORE, Operand::None),
        i(13, op::PUTSTATIC, Operand::Field(array_field())),
        i(16, op::RETURN, Operand::None),
    ])
}

#[test]
fn array_writes_constants_only() {
    let a = run(&array_code(false));
    let w = array_writes(&a, 1, Some(&array_field())).unwrap();
    assert_eq!(w.len(), 1);
    assert_eq!((w[0].0.clone(), w[0].1.as_ref()), (V::Int(0), "x"));
    let a = run(&array_code(true));
    assert!(array_writes(&a, 1, Some(&array_field())).is_none());
}

/// 局部数组 `S[] xs = {"x", "y"}; xs[0]`；`escape` 时数组另作为实参传出
fn local_array_code(escape: bool) -> Code {
    let mut v = vec![
        i(0, ICONST_2, Operand::None),
        i(1, op::ANEWARRAY, Operand::Class("a/S".into())),
        i(4, DUP, Operand::None),
        i(5, ICONST_0, Operand::None),
        i(6, op::LDC, ldc("x")),
        i(8, AASTORE, Operand::None),
        i(9, DUP, Operand::None),
        i(10, ICONST_1, Operand::None),
        i(11, op::LDC, ldc("y")),
        i(13, AASTORE, Operand::None),
        i(14, ASTORE_1, Operand::None),
    ];
    if escape {
        v.push(i(15, ALOAD_1, Operand::None));
        v.push(i(16, op::INVOKESTATIC, Operand::Method(mref("a/H", "f", "([La/S;)V"), false)));
    }
    v.push(i(19, ALOAD_1, Operand::None));
    v.push(i(20, ICONST_0, Operand::None));
    v.push(i(21, AALOAD, Operand::None));
    v.push(i(22, POP, Operand::None));
    v.push(i(23, op::RETURN, Operand::None));
    code(v)
}

#[test]
fn local_array_writes_require_no_escape() {
    let a = run(&local_array_code(false));
    let w = array_writes(&a, 1, None).unwrap();
    let names: Vec<&str> = w.iter().map(|(_, s)| s.as_ref()).collect();
    assert_eq!(names, ["x", "y"]);
    let a = run(&local_array_code(true));
    assert!(array_writes(&a, 1, None).is_none());
}

fn builder_facts() -> NameFacts {
    let v: toml::Value = toml::from_str(
        r#"
        builders = ["a/B.<init>:()V"]
        appends = ["a/B.add:(La/S;)La/B;"]
        results = ["a/B.str:()La/S;"]
        resets = ["a/B.clear:(I)V"]
        "#,
    )
    .unwrap();
    NameFacts::from_toml(None, Some(&v)).unwrap()
}

/// 循环复用构建器：`b = new B(); while (z) { [b.clear(0);] b.add("x").str(); }`
fn builder_loop(reset: bool) -> Code {
    let clear = |o, op, operand| if reset { i(o, op, operand) } else { i(o, NOP, Operand::None) };
    code(vec![
        i(0, op::NEW, Operand::Class("a/B".into())),
        i(3, DUP, Operand::None),
        i(4, op::INVOKESPECIAL, Operand::Method(mref("a/B", "<init>", "()V"), false)),
        i(7, ASTORE_1, Operand::None),
        i(8, ILOAD_0, Operand::None),
        i(9, IFEQ, Operand::Branch(30)),
        clear(12, ALOAD_1, Operand::None),
        clear(13, ICONST_0, Operand::None),
        clear(14, op::INVOKEVIRTUAL, Operand::Method(mref("a/B", "clear", "(I)V"), false)),
        i(17, ALOAD_1, Operand::None),
        i(18, op::LDC, ldc("x")),
        i(20, op::INVOKEVIRTUAL, Operand::Method(mref("a/B", "add", "(La/S;)La/B;"), false)),
        i(23, op::INVOKEVIRTUAL, Operand::Method(mref("a/B", "str", "()La/S;"), false)),
        i(26, POP, Operand::None),
        i(27, op::GOTO, Operand::Branch(8)),
        i(30, op::RETURN, Operand::None),
    ])
}

/// 循环内先清空再追加：链首成立（此前无内容）；不清空则上一轮内容残留，不成立
#[test]
fn builder_prefix_requires_reset_in_loop() {
    let f = builder_facts();
    let a = absint::analyze("a/H", "(Z)V", true, &builder_loop(true), &Plain);
    assert_eq!(builder_prefix(&f, &a, 0, 20).map(|x| x.len()), Some(0));
    let a = absint::analyze("a/H", "(Z)V", true, &builder_loop(false), &Plain);
    assert!(builder_prefix(&f, &a, 0, 20).is_none());
}

/// 独立追加语句：`b = new B(); [if (z)] b.add("x"); b.add("y").add("z"); b.str();`
fn builder_statements(cond: bool) -> Code {
    let add = || Operand::Method(mref("a/B", "add", "(La/S;)La/B;"), false);
    let guard = |o, op, operand| if cond { i(o, op, operand) } else { i(o, NOP, Operand::None) };
    code(vec![
        i(0, op::NEW, Operand::Class("a/B".into())),
        i(3, DUP, Operand::None),
        i(4, op::INVOKESPECIAL, Operand::Method(mref("a/B", "<init>", "()V"), false)),
        i(7, ASTORE_1, Operand::None),
        guard(8, ILOAD_0, Operand::None),
        guard(9, IFEQ, Operand::Branch(18)),
        i(12, ALOAD_1, Operand::None),
        i(13, op::LDC, ldc("x")),
        i(15, op::INVOKEVIRTUAL, add()),
        i(17, POP, Operand::None),
        i(18, ALOAD_1, Operand::None),
        i(19, op::LDC, ldc("y")),
        i(21, op::INVOKEVIRTUAL, add()),
        i(24, op::LDC, ldc("z")),
        i(26, op::INVOKEVIRTUAL, add()),
        i(29, POP, Operand::None),
        i(30, ALOAD_1, Operand::None),
        i(31, op::INVOKEVIRTUAL, Operand::Method(mref("a/B", "str", "()La/S;"), false)),
        i(34, POP, Operand::None),
        i(35, op::RETURN, Operand::None),
    ])
}

/// 每条追加语句在取结果前必经且先后确定：按序相接；可跳过（条件追加）时不成立
#[test]
fn builder_prefix_statements_in_order() {
    let f = builder_facts();
    let a = absint::analyze("a/H", "(Z)V", true, &builder_statements(false), &Plain);
    let segs = builder_prefix(&f, &a, 0, 31).unwrap();
    let lits: Vec<_> = segs.iter().map(|(v, _)| v.lits()).collect();
    assert_eq!(lits, vec![vec![Rc::from("x")], vec![Rc::from("y")], vec![Rc::from("z")]]);
    let a = absint::analyze("a/H", "(Z)V", true, &builder_statements(true), &Plain);
    assert!(builder_prefix(&f, &a, 0, 31).is_none());
}
