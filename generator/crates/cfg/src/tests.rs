//! 单元测试：条件代数、结构化、规整、状态机、自检。

use std::collections::{BTreeMap, BTreeSet};

use ir::{Expr, Ident, Renderer, ShortNames};

use crate::*;

struct NoNames;

impl ShortNames for NoNames {
    fn short_cls(&self, binary: &str) -> String {
        binary.to_string()
    }
}

fn raw(s: &str) -> Expr {
    Expr::raw(s.to_string())
}

fn var(s: &str) -> Expr {
    Expr::Var(Ident::new(s).expect("ident"))
}

fn render(e: &Expr) -> String {
    Renderer::new(&NoNames).expr(e)
}

fn node(pc: u32, term: Terminal) -> FlowNode {
    FlowNode { start_pc: pc, term, has_stmts: true, has_decls: false, ctx: BTreeSet::new(), jump_pcs: vec![] }
}

fn run(nodes: &mut BTreeMap<NodeId, FlowNode>) -> Vec<Item> {
    let succs: Succs = nodes.iter().map(|(k, n)| (*k, n.term.successors())).collect();
    let flow = analyze(0, &succs);
    simplify(structure(nodes, &flow).expect("structure"))
}

#[test]
fn cond_negate_de_morgan() {
    let c = Cond::or(Cond::atom(var("a")), Cond::and(Cond::atom(var("b")), Cond::atom(var("c"))));
    assert_eq!(render(&c.to_expr()), "a || (b && c)");
    assert_eq!(render(&c.negate().to_expr()), "!a && (!b || !c)");
    assert_eq!(Cond::and(Cond::Const(true), Cond::atom(var("x"))), Cond::atom(var("x")));
    assert_eq!(Cond::or(Cond::Const(true), Cond::atom(var("x"))), Cond::Const(true));
}

#[test]
fn cmp_op_forms() {
    let e = cmp_op(opcodes::IF_ICMPLT, raw("a == b"), var("c")).expect("cmp");
    assert_eq!(render(&e), "(a == b) < c");
    let e = neg_cmp_op(classfile::op::IFNULL, var("x"), var("x")).expect("cmp");
    assert_eq!(render(&e), "!_is_jnull(&x)");
    assert!(cmp_op(opcodes::IFEQ, var("x"), var("x")).is_ok());
    assert!(cmp_op(classfile::op::GOTO, var("x"), var("x")).is_err());
}

#[test]
fn while_loop_shape() {
    // 0: cond(!c) → 2 else 1；1: goto 0；2: exit
    let mut nodes = BTreeMap::from([
        (0, node(0, Terminal::Cond { cond: Cond::atom(var("c")), target: 2, fallthrough: 1 })),
        (1, node(3, Terminal::Goto { target: 0 })),
        (2, node(6, Terminal::Goto { target: 3 })),
        (3, node(9, Terminal::Exit)),
    ]);
    nodes.get_mut(&0).expect("n0").has_stmts = false;
    let tree = run(&mut nodes);
    let Item::Loop(l) = &tree[0] else { panic!("{tree:?}") };
    assert_eq!(l.while_cond, Some(Cond::atom(var("c")).negate()));
    assert_eq!(l.cond_origin, Some(0));
    assert!(matches!(tree[1], Item::Code { block: 2, exits: false, .. }), "{tree:?}");
}

#[test]
fn diamond_merge_and_audit() {
    let mut nodes = BTreeMap::from([
        (0, node(0, Terminal::Cond { cond: Cond::atom(var("c")), target: 2, fallthrough: 1 })),
        (1, node(3, Terminal::Goto { target: 3 })),
        (2, node(6, Terminal::Goto { target: 3 })),
        (3, node(9, Terminal::Exit)),
    ]);
    nodes.get_mut(&0).expect("n0").jump_pcs = vec![2];
    nodes.get_mut(&3).expect("n3").has_decls = true;
    let succs: Succs = nodes.iter().map(|(k, n)| (*k, n.term.successors())).collect();
    let flow = analyze(0, &succs);
    let tree = simplify(structure(&mut nodes, &flow).expect("structure"));
    assert!(matches!(&tree[0], Item::Decl(ids) if ids == &vec![3]));
    assert!(matches!(tree[2], Item::If(_)));
    let mut ledger = JumpLedger::new("T.m()V");
    ledger.expect(2);
    ledger.expect(5);
    verify_tree(&tree, &nodes, &flow, &mut ledger).expect("verify_tree");
    assert!(ledger.verify().is_err());
    ledger.consume(5, JumpKind::ShortCircuit);
    ledger.verify().expect("verify");
    let mut stats = AuditStats::default();
    stats.record(&ledger, false);
    stats.record(&ledger, false);
    assert_eq!(stats.methods, 1);
    assert!(stats.summary().contains("short-circuit=1 structured=1"));
}

#[test]
fn dispatch_stmts() {
    let r = Renderer::new(&NoNames);
    let term = Terminal::Cond { cond: Cond::atom(var("c")), target: 4, fallthrough: 1 };
    let s = next_pc_stmts(&term).expect("dispatch");
    assert_eq!(r.stmt(&s[0], 0), "__pc = if c {\n    4\n} else {\n    1\n};");
    let term = Terminal::Switch { key: var("k"), cases: vec![(vec![1, 2], 3)], default: 5 };
    let text = r.stmt(&next_pc_stmts(&term).expect("dispatch")[0], 0);
    assert!(text.contains("1 | 2 =>") && text.contains("__pc = 5;"), "{text}");
    assert_eq!(build_dispatch(0, &[3, 1, 0]).blocks, vec![0, 1, 3]);
}

/// 回归夹具：XMLDTDScannerImpl.scanDTDInternalSubset 的 CFG 形态（J0 发现，块图实测）。
/// 循环 L6={6,14}：头 6 的 Cond 跳转臂指向空块 14，14 只含回边。六个块为空块（1/3/6/7/10/14，
/// 与实测 stmts=0 一致）——simplify 把空 Code 视为可忽略项后尾上下文计算丢失块 14。
#[test]
fn loop_cond_target_empty_backedge_block_kept() {
    let mut nodes = BTreeMap::new();
    let mut put = |id: NodeId, pc: u32, term: Terminal, stmts: bool| {
        nodes.insert(
            id,
            FlowNode { start_pc: pc, term, has_stmts: stmts, has_decls: false, ctx: BTreeSet::new(), jump_pcs: vec![] },
        );
    };
    put(0, 0, Terminal::Cond { cond: Cond::atom(var("c0")), target: 6, fallthrough: 1 }, true);
    put(1, 32, Terminal::Cond { cond: Cond::atom(var("c1")), target: 3, fallthrough: 2 }, false);
    put(2, 39, Terminal::Goto { target: 3 }, true);
    put(3, 58, Terminal::Cond { cond: Cond::atom(var("c3")), target: 5, fallthrough: 4 }, false);
    put(4, 65, Terminal::Goto { target: 5 }, true);
    put(5, 82, Terminal::Goto { target: 6 }, true);
    put(6, 87, Terminal::Cond { cond: Cond::atom(var("c6")), target: 14, fallthrough: 7 }, false);
    put(7, 95, Terminal::Cond { cond: Cond::atom(var("c7")), target: 10, fallthrough: 9 }, false);
    put(9, 106, Terminal::Goto { target: 10 }, true);
    put(10, 116, Terminal::Cond { cond: Cond::atom(var("c10")), target: 13, fallthrough: 12 }, false);
    put(12, 127, Terminal::Goto { target: 13 }, true);
    put(13, 135, Terminal::Exit, true);
    put(14, 162, Terminal::Goto { target: 6 }, false);
    let succs: Succs = nodes.iter().map(|(k, n)| (*k, n.term.successors())).collect();
    let flow = analyze(0, &succs);
    assert!(flow.reducible, "夹具须可归约");
    let tree = simplify(structure(&mut nodes, &flow).expect("structure"));
    let mut ledger = JumpLedger::default();
    verify_tree(&tree, &nodes, &flow, &mut ledger).expect("活块 14 须保留在结构树中");
}
