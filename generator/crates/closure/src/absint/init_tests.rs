//! 构造器确定初始化（`init.rs`）单测：桩 Oracle 给出字段键（原样）与被调构造器摘要。

use super::*;
use classfile::{ExceptionEntry, Insn};

const ALOAD_0: u8 = 0x2a;
const ACONST_NULL: u8 = 0x01;
const ILOAD_1: u8 = 0x1b;
const IFEQ: u8 = 0x99;
const POP: u8 = 0x57;

struct Stub {
    /// 被调构造器的摘要（按构造器所属类）
    sums: Vec<(&'static str, InitSum)>,
}

impl Oracle for Stub {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
    fn init_key(&self, f: &MemberRef) -> Option<MemberRef> {
        Some(f.clone())
    }
    fn init_sum(&self, init: &MemberRef) -> Option<Rc<InitSum>> {
        self.sums.iter().find(|(c, _)| *c == init.owner).map(|(_, s)| Rc::new(s.clone()))
    }
}

fn fld(n: &str) -> MemberRef {
    MemberRef { owner: "p/C".into(), name: n.into(), desc: "Ljava/lang/Object;".into() }
}

fn init_of(owner: &str, desc: &str) -> Operand {
    Operand::Method(MemberRef { owner: owner.into(), name: "<init>".into(), desc: desc.into() }, false)
}

/// 超类构造器 `p/S.<init>()V`（摘要：不写、不交出）
fn sup() -> Vec<(u8, Operand)> {
    vec![(ALOAD_0, Operand::None), (op::INVOKESPECIAL, init_of("p/S", "()V"))]
}

fn put(n: &str) -> Vec<(u8, Operand)> {
    vec![(ALOAD_0, Operand::None), (ACONST_NULL, Operand::None), (op::PUTFIELD, Operand::Field(fld(n)))]
}

/// `this` 交给静态方法
fn leak() -> Vec<(u8, Operand)> {
    let m = MemberRef { owner: "p/X".into(), name: "use".into(), desc: "(Ljava/lang/Object;)V".into() };
    vec![(ALOAD_0, Operand::None), (op::INVOKESTATIC, Operand::Method(m, false))]
}

/// 指令偏移 = 下标（分支目标按下标给）；构造器描述符 `(Z)V`
fn code(ops: Vec<(u8, Operand)>, handlers: Vec<ExceptionEntry>) -> Code {
    let insns: Vec<Insn> = ops.into_iter().enumerate().map(|(i, (opcode, operand))| Insn { offset: i as u32, opcode, operand }).collect();
    Code { max_stack: 4, max_locals: 2, code_len: insns.len() as u32, insns, exception_table: handlers }
}

fn empty_sup() -> Vec<(&'static str, InitSum)> {
    vec![("p/S", InitSum { exit: Some(vec![]), esc: None, bad: vec![] })]
}

fn definite(ops: Vec<(u8, Operand)>, sums: Vec<(&'static str, InitSum)>) -> Vec<String> {
    let s = analyze_init("p/C", "(Z)V", &code(ops, vec![]), &Stub { sums }).expect("可建模");
    s.definite().into_iter().map(|k| k.name).collect()
}

/// 全部路径写入、从不交出：确定初始化
#[test]
fn init_all_paths_written() {
    let ops = [sup(), put("a"), put("b"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, empty_sup()), ["a", "b"]);
}

/// 交出 `this` 之后才写的字段不算
#[test]
fn init_escape_cuts() {
    let ops = [sup(), put("a"), leak(), put("b"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, empty_sup()), ["a"]);
}

/// 写入值是 `this`（`this.self = this`）：交出
#[test]
fn init_self_store_escapes() {
    let ops = [
        sup(),
        put("a"),
        vec![(ALOAD_0, Operand::None), (ALOAD_0, Operand::None), (op::PUTFIELD, Operand::Field(fld("s")))],
        put("b"),
        vec![(op::RETURN, Operand::None)],
    ]
    .concat();
    assert_eq!(definite(ops, empty_sup()), ["a"]);
}

/// 写入前在构造器内读：该字段不算
#[test]
fn init_read_before_write() {
    let ops = [
        sup(),
        vec![(ALOAD_0, Operand::None), (op::GETFIELD, Operand::Field(fld("a"))), (POP, Operand::None)],
        put("a"),
        put("b"),
        vec![(op::RETURN, Operand::None)],
    ]
    .concat();
    assert_eq!(definite(ops, empty_sup()), ["b"]);
}

/// 只在一侧分支写入：不算
#[test]
fn init_branch_one_side() {
    // 0..1 super；2 iload_1；3 ifeq → 7；4..6 put a；7..9 put b；10 return
    let ops = [sup(), vec![(ILOAD_1, Operand::None), (IFEQ, Operand::Branch(7))], put("a"), put("b"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, empty_sup()), ["b"]);
}

/// 超类构造器无摘要（手写 / 无法分析）：按交出处理，之前无写入则全不算
#[test]
fn init_unknown_super_escapes() {
    let ops = [sup(), put("a"), vec![(op::RETURN, Operand::None)]].concat();
    assert!(definite(ops, vec![]).is_empty());
}

/// 委托 / 超类构造器：返回集并入；被调方交出时本方已写的字段保留；被调方的 bad 去掉本方已写
#[test]
fn init_delegate_compose() {
    // 被调方写 a、b 后返回，从不交出
    let s1 = vec![("p/S", InitSum { exit: Some(vec![fld("a"), fld("b")]), esc: None, bad: vec![] })];
    let ops = [sup(), put("c"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, s1), ["a", "b", "c"]);
    // 被调方在只写了 a 时交出（超类构造器调用可覆盖的方法）：本方此后写的 c 不算
    let s2 = vec![("p/S", InitSum { exit: Some(vec![fld("a"), fld("b")]), esc: Some(vec![fld("a")]), bad: vec![] })];
    let ops = [sup(), put("c"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, s2), ["a"]);
    // 被调方读 d 时未写；本方在调用前已写 d（外部类引用 this$0 形态）→ 不算 bad
    let s3 = vec![("p/S", InitSum { exit: Some(vec![]), esc: None, bad: vec![fld("d"), fld("e")] })];
    let ops = [put("d"), put("e2"), sup(), put("e"), vec![(op::RETURN, Operand::None)]].concat();
    assert_eq!(definite(ops, s3), ["d", "e2"]);
}

/// 无正常返回（恒抛异常）：无确定字段
#[test]
fn init_no_return() {
    let ops = [sup(), put("a"), vec![(ACONST_NULL, Operand::None), (op::ATHROW, Operand::None)]].concat();
    assert!(definite(ops, empty_sup()).is_empty());
}

/// 异常处理器入口不继承已写集合：处理器路径正常返回时只保留处理器内写入的字段
#[test]
fn init_handler_path() {
    // 0..1 super；2..4 put a；5..6 leak 之外的调用（静态无实参）；7..9 put b；10 return；11 pop；12..14 put b；15 return
    let g = MemberRef { owner: "p/X".into(), name: "g".into(), desc: "()V".into() };
    let ops = [
        sup(),
        put("a"),
        vec![(op::INVOKESTATIC, Operand::Method(g, false)), (0x00, Operand::None)],
        put("b"),
        vec![(op::RETURN, Operand::None), (POP, Operand::None)],
        put("b"),
        vec![(op::RETURN, Operand::None)],
    ]
    .concat();
    let h = ExceptionEntry { start: 5, end: 6, handler: 11, catch_type: None };
    let s = analyze_init("p/C", "(Z)V", &code(ops, vec![h]), &Stub { sums: empty_sup() }).expect("可建模");
    assert_eq!(s.definite().into_iter().map(|k| k.name).collect::<Vec<_>>(), ["b"]);
}

/// 普通分析不跟踪（无摘要）
#[test]
fn init_untracked_by_default() {
    let ops = [sup(), put("a"), vec![(op::RETURN, Operand::None)]].concat();
    let c = code(ops, vec![]);
    let st = Stub { sums: empty_sup() };
    assert!(run("p/C", "(Z)V", false, &c, &st, false).is_some_and(|(_, s)| s.is_none()));
}
