//! absint 单测：以桩 Oracle 驱动的分支折叠规则。

use super::*;
use classfile::Insn;

/// 桩 Oracle：`live` 中的类型有实例，其余没有
struct Stub {
    live: Vec<&'static str>,
}

impl Oracle for Stub {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
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
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
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
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
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
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
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

#[test]
fn empty_collection_tag_join() {
    let e = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Empty)) };
    let other = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: None };
    // 与 null 合流保留标签（可空性另记）；与其它对象合流即丢
    assert!(super::obj::join_obj(&e, &V::Null).is_some_and(|o| *o == Obj::Empty));
    assert!(super::obj::join_obj(&e, &e.clone()).is_some_and(|o| *o == Obj::Empty));
    assert!(super::obj::join_obj(&e, &other).is_none());
}

/// 桩 Oracle：`p/M.isSub` 是类镜像子类型判定
struct SubTests;

impl Oracle for SubTests {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn mirror_subtype_test(&self, m: &MemberRef) -> bool {
        m.owner == "p/M" && m.name == "isSub"
    }
}

/// `if (K.class.isSub(c)) g(c); h(c);`：成立一侧的 c 来源不变、带收窄标记（类型流取跳转偏移处的节点），
/// 该偏移发 MirrorSub（输入为原值）；汇合后标记消失
#[test]
fn mirror_subtype_test_narrows_true_side() {
    let test = MemberRef { owner: "p/M".into(), name: "isSub".into(), desc: "(Lp/M;)Z".into() };
    let g = MemberRef { owner: "p/A".into(), name: "g".into(), desc: "(Lp/M;)V".into() };
    let h = MemberRef { owner: "p/A".into(), name: "h".into(), desc: "(Lp/M;)V".into() };
    let code = code_of(
        vec![
            (0, op::LDC, Operand::Ldc(Const::Class("p/K".into()))),
            (2, ALOAD_0, Operand::None),
            (3, op::INVOKEVIRTUAL, Operand::Method(test, false)),
            (6, IFEQ, Operand::Branch(13)),
            (9, ALOAD_0, Operand::None),
            (10, op::INVOKESTATIC, Operand::Method(g, false)),
            (13, ALOAD_0, Operand::None),
            (14, op::INVOKESTATIC, Operand::Method(h, false)),
            (17, op::RETURN, Operand::None),
        ],
        18,
    );
    let a = analyze("p/A", "(Lp/M;)V", true, &code, &SubTests);
    let arg = |at: u32| {
        a.events.iter().find_map(|(o, e)| match e {
            Event::Invoke { args, .. } if *o == at => Some(args[0].clone()),
            _ => None,
        })
    };
    let narrowed = arg(10).expect("g 调用");
    assert_eq!(narrowed.narrowed(), Some(6));
    assert_eq!(narrowed.srcs().as_ref(), &[Src::Param(0)]);
    assert!(narrowed.obj().is_none());
    assert_eq!(arg(14).expect("h 调用").narrowed(), None);
    let ev = a.events.iter().find_map(|(o, e)| match e {
        Event::MirrorSub(c, v) if *o == 6 => Some((c.clone(), v.clone())),
        _ => None,
    });
    let (c, v) = ev.expect("MirrorSub 事件");
    assert_eq!(c, "p/K");
    assert_eq!(v.narrowed(), None);
}

/// 桩 Oracle：`p/S.key` 是键类 p/S 的键读取方法，`p/Str.equals` 是字符串相等判定
struct KeyTests;

impl Oracle for KeyTests {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn key_getter(&self, m: &MemberRef) -> Option<(String, bool)> {
        (m.owner == "p/S" && m.name == "key").then(|| ("p/S".to_string(), false))
    }
    fn string_equality(&self, m: &MemberRef) -> Option<bool> {
        (m.owner == "p/Str" && m.name == "equals").then_some(false)
    }
}

/// `static void f(S s, String n)`：按 body 给出的判定 + `ifeq 跳过 g(s)`，之后 h(s)
fn key_test_code(body: Vec<(u32, u8, Operand)>) -> (Analysis, Vec<(u32, V)>) {
    let g = MemberRef { owner: "p/A".into(), name: "g".into(), desc: "(Lp/S;)V".into() };
    let h = MemberRef { owner: "p/A".into(), name: "h".into(), desc: "(Lp/S;)V".into() };
    let b = body.last().unwrap().0 + 3;
    let mut insns = body;
    insns.push((b, IFEQ, Operand::Branch(b + 7)));
    insns.push((b + 3, ALOAD_0, Operand::None));
    insns.push((b + 4, op::INVOKESTATIC, Operand::Method(g, false)));
    insns.push((b + 7, ALOAD_0, Operand::None));
    insns.push((b + 8, op::INVOKESTATIC, Operand::Method(h, false)));
    insns.push((b + 11, op::RETURN, Operand::None));
    let mut code = code_of(insns, b + 12);
    code.max_locals = 2;
    let a = analyze("p/A", "(Lp/S;Lp/Str;)V", true, &code, &KeyTests);
    let args = a
        .events
        .iter()
        .filter_map(|(o, e)| match e {
            Event::Invoke { opcode: op::INVOKESTATIC, args, .. } => Some((*o, args[0].clone())),
            _ => None,
        })
        .collect();
    (a, args)
}

fn key_mref() -> MemberRef {
    MemberRef { owner: "p/S".into(), name: "key".into(), desc: "()Lp/Str;".into() }
}

fn equals_mref() -> MemberRef {
    MemberRef { owner: "p/Str".into(), name: "equals".into(), desc: "(Lp/O;)Z".into() }
}

/// `if (s.key().equals(n)) g(s); h(s);` 与 `if (n.equals(s.key())) …`：成立一侧的 s 来源不变、带收窄标记，
/// 该偏移发 KeyTest（输入为原值、名字为 n）；汇合后标记消失
#[test]
fn key_test_narrows_true_side() {
    let fwd = vec![
        (0, ALOAD_0, Operand::None),
        (1, op::INVOKEVIRTUAL, Operand::Method(key_mref(), false)),
        (4, 0x2b, Operand::None),
        (5, op::INVOKEVIRTUAL, Operand::Method(equals_mref(), false)),
    ];
    let rev = vec![
        (0, 0x2b, Operand::None),
        (1, ALOAD_0, Operand::None),
        (2, op::INVOKEVIRTUAL, Operand::Method(key_mref(), false)),
        (5, op::INVOKEVIRTUAL, Operand::Method(equals_mref(), false)),
    ];
    for body in [fwd, rev] {
        let (a, args) = key_test_code(body);
        assert_eq!(args.len(), 2);
        assert_eq!(args[0].1.narrowed(), Some(8));
        assert_eq!(args[0].1.srcs().as_ref(), &[Src::Param(0)]);
        assert!(args[0].1.obj().is_none());
        assert_eq!(args[1].1.narrowed(), None);
        let ev = a.events.iter().find_map(|(o, e)| match e {
            Event::KeyTest { kc, fold, input, name } if *o == 8 => Some((kc.clone(), *fold, input.clone(), name.clone())),
            _ => None,
        });
        let (kc, fold, input, name) = ev.expect("KeyTest 事件");
        assert_eq!((kc.as_str(), fold), ("p/S", false));
        assert_eq!(input.srcs().as_ref(), &[Src::Param(0)]);
        assert_eq!(input.narrowed(), None);
        assert_eq!(name.srcs().as_ref(), &[Src::Param(1)]);
    }
}

/// 读取键之后被测局部变量被改写：条件跳转处的 s 不再是被读取键的对象，不收窄
#[test]
fn key_test_skips_rewritten_local() {
    let body = vec![
        (0, ALOAD_0, Operand::None),
        (1, op::INVOKEVIRTUAL, Operand::Method(key_mref(), false)),
        (4, 0x01, Operand::None),
        (5, 0x4b, Operand::None),
        (6, 0x2b, Operand::None),
        (7, op::INVOKEVIRTUAL, Operand::Method(equals_mref(), false)),
    ];
    let (a, args) = key_test_code(body);
    assert!(args.iter().all(|(_, v)| v.narrowed().is_none()));
    assert!(!a.events.iter().any(|(_, e)| matches!(e, Event::KeyTest { .. })));
}

/// 桩 Oracle：形参 0 镜像值集上的接收者钩子字段读（Some = 每个镜像的该字段都为该值）
struct MirrorField(Option<V>);

impl Oracle for MirrorField {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn param_mirror_field(&self, i: u16, _: &MemberRef) -> Option<V> {
        (i == 0).then(|| self.0.clone()).flatten()
    }
}

/// `void f() { if (this.cl == null) return; throw …; }`
fn mirror_field_code() -> Code {
    let i = |offset, opcode, operand| Insn { offset, opcode, operand };
    let f = MemberRef { owner: "p/C".into(), name: "cl".into(), desc: "Lp/L;".into() };
    let insns = vec![
        i(0, ALOAD_0, Operand::None),
        i(1, op::GETFIELD, Operand::Field(f)),
        i(4, 0xc6, Operand::Branch(9)),
        i(7, ALOAD_0, Operand::None),
        i(8, op::ATHROW, Operand::None),
        i(9, op::RETURN, Operand::None),
    ];
    Code { max_stack: 1, max_locals: 1, code_len: 10, insns, exception_table: vec![] }
}

/// this 的镜像值集上字段恒 null：非空分支不可达，登记乐观答复；未知：两支都可达、不登记
#[test]
fn receiver_hook_field_folds_by_param_mirrors() {
    let a = analyze("p/C", "()V", false, &mirror_field_code(), &MirrorField(Some(V::Null)));
    assert_eq!(a.reachable, vec![true, true, true, false, false, true]);
    assert_eq!(a.mirror_field_assumed, vec![0]);
    let a = analyze("p/C", "()V", false, &mirror_field_code(), &MirrorField(None));
    assert_eq!(a.reachable, vec![true; 6]);
    assert!(a.mirror_field_assumed.is_empty());
}

/// 常量长度数组经局部变量往返后 `arraylength` 折叠：`new Object[0]` 的长度判零分支只走一侧
#[test]
fn const_length_array_folds_arraylength() {
    let code = code_of(
        vec![
            (0, 0x03, Operand::None),                         // iconst_0
            (1, op::ANEWARRAY, Operand::Class("p/B".into())), // anewarray
            (4, 0x4b, Operand::None),                         // astore_0
            (5, 0x2a, Operand::None),                         // aload_0
            (6, 0xbe, Operand::None),                         // arraylength
            (7, 0x99, Operand::Branch(11)),                   // ifeq
            (10, op::RETURN, Operand::None),
            (11, op::RETURN, Operand::None),
        ],
        12,
    );
    let a = analyze("p/A", "()V", true, &code, &Sets);
    assert_eq!(a.reachable, vec![true, true, true, true, true, true, false, true]);
}

/// 桩 Oracle：Class 形参上的映像对象取值方法——形参 i 的结果为映像对象 `self.0[i]`（None = 未知）
struct ImageGetter([Option<u32>; 2]);

impl Oracle for ImageGetter {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn param_mirror_call(&self, i: u16, _: &MemberRef) -> Option<V> {
        let o = self.0.get(i as usize).copied().flatten()?;
        Some(V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Image(o, Vec::new()))) })
    }
}

/// `static boolean f(Class a, Class b) { return a.m() == b.m(); }` 形态：两侧为同一映像对象时引用相等折叠、
/// 不同映像对象时引用不等折叠，两个形参都登记乐观答复；未知时两支都可达、不登记
#[test]
fn image_object_results_compare_by_identity() {
    let m = MemberRef { owner: "p/K".into(), name: "m".into(), desc: "()Lp/M;".into() };
    let code = code_of(
        vec![
            (0, 0x2a, Operand::None), // aload_0
            (1, op::INVOKEVIRTUAL, Operand::Method(m.clone(), false)),
            (4, 0x2b, Operand::None), // aload_1
            (5, op::INVOKEVIRTUAL, Operand::Method(m, false)),
            (8, 0xa6, Operand::Branch(12)), // if_acmpne
            (11, op::RETURN, Operand::None),
            (12, op::RETURN, Operand::None),
        ],
        13,
    );
    let code = Code { max_locals: 2, ..code };
    let a = analyze("p/A", "(Ljava/lang/Class;Ljava/lang/Class;)V", true, &code, &ImageGetter([Some(7), Some(7)]));
    assert_eq!(a.reachable, vec![true, true, true, true, true, true, false]);
    assert_eq!(a.mirror_field_assumed, vec![0, 1]);
    let a = analyze("p/A", "(Ljava/lang/Class;Ljava/lang/Class;)V", true, &code, &ImageGetter([Some(7), Some(8)]));
    assert_eq!(a.reachable, vec![true, true, true, true, true, false, true]);
    assert_eq!(a.mirror_field_assumed, vec![0, 1]);
    let a = analyze("p/A", "(Ljava/lang/Class;Ljava/lang/Class;)V", true, &code, &ImageGetter([Some(7), None]));
    assert_eq!(a.reachable, vec![true; 7]);
    assert_eq!(a.mirror_field_assumed, vec![0]);
}

/// 桩 Oracle：静态字段读的常量格答复（`Some(非空引用)` = 映像值与全部可达写入都非空）
struct StaticNonNull(Option<V>);

impl Oracle for StaticNonNull {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        self.0.clone()
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
}

/// `X x = B.f; if (x == null) { ... }` 形态（`System$LoggerFinder.accessProvider` 同形）：静态字段常量格为非空引用时
/// 空分支不可达；答复未知时两支都可达
#[test]
fn nonnull_static_field_folds_null_test() {
    let f = MemberRef { owner: "p/B".into(), name: "f".into(), desc: "Lp/X;".into() };
    let code = code_of(
        vec![
            (0, op::GETSTATIC, Operand::Field(f)),
            (3, 0x4b, Operand::None),         // astore_0
            (4, 0x2a, Operand::None),         // aload_0
            (5, 0xc7, Operand::Branch(9)),    // ifnonnull
            (8, op::RETURN, Operand::None),
            (9, op::RETURN, Operand::None),
        ],
        10,
    );
    let nonnull = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: None };
    let a = analyze("p/A", "()V", true, &code, &StaticNonNull(Some(nonnull)));
    assert_eq!(a.reachable, vec![true, true, true, true, false, true]);
    let a = analyze("p/A", "()V", true, &code, &StaticNonNull(None));
    assert_eq!(a.reachable, vec![true; 6]);
}

/// `if ("a" == "b") ...`：内容不同的两个确定字符串必是不同对象，相等分支不可达；内容相同不断言同一（两支都可达）
#[test]
fn distinct_string_constants_ref_ne() {
    let code = |b: &str| {
        code_of(
            vec![
                (0, op::LDC, Operand::Ldc(Const::String("a".into()))),
                (2, op::LDC, Operand::Ldc(Const::String(b.into()))),
                (4, 0xa6, Operand::Branch(8)), // if_acmpne
                (7, op::RETURN, Operand::None),
                (8, op::RETURN, Operand::None),
            ],
            9,
        )
    };
    let a = analyze("p/A", "()V", true, &code("b"), &Stub { live: vec![] });
    assert_eq!(a.reachable, vec![true, true, true, false, true]);
    let a = analyze("p/A", "()V", true, &code("a"), &Stub { live: vec![] });
    assert_eq!(a.reachable, vec![true; 5]);
}
