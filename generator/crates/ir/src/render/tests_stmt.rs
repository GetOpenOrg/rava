//! 语句 / 条目渲染单测（← tests/unit/test_try_expr.py 语句部分 + 结构化控制流形态）。

use super::tests_expr::{id, var, TailNames};
use crate::{
    ArmBody, AssignStmt, CatchClause, ElseBranch, Expr, FnItem, IfStmt, ImplItem, Item, Label,
    LetStmt, Lit, LoopStmt, MacroCall, MatchArm, MatchStmt, ModItem, Param, Path, Pattern, Raw,
    Renderer, Stmt, StructField, StructItem, TryStmt, Type, TypeAlias, UseTree, VarOrigin, Vis,
    WhileStmt,
};

fn s(st: &Stmt, indent: usize) -> String {
    Renderer::new(&TailNames).stmt(st, indent)
}

fn raw(t: &str) -> Stmt {
    Stmt::Raw(Raw(t.into()))
}

fn label(n: &str) -> Label {
    Label::new(n).unwrap()
}

// ── test_try_expr ──────────────────────────────────────────────────────────

#[test]
fn try_static_call_in_let() {
    let e = Expr::try_(Expr::call(
        Path::new(vec![
            crate::PathSegment::with_generics(id("Foo"), vec![Type::named(id("Object"), vec![])]),
            crate::PathSegment::new(id("new")),
        ]),
        vec![],
    ));
    assert_eq!(s(&Stmt::Let(LetStmt::new(id("_t0"), None, Some(e))), 0), "let _t0 = Foo::<Object>::new()?;");
}

#[test]
fn try_expr_stmt() {
    let e = Expr::try_(Expr::method(var("sb"), id("append"), vec![var("x")]));
    assert_eq!(s(&Stmt::Expr(e), 0), "sb.append(x)?;");
}

// ── 叶子语句 ───────────────────────────────────────────────────────────────

#[test]
fn leaf_statements() {
    let mut l = LetStmt::new(id("x"), Some(Type::I32), Some(Expr::Lit(Lit::i32(0))));
    l.mutable = true;
    assert_eq!(s(&Stmt::Let(l), 1), "    let mut x: i32 = 0i32;");
    assert_eq!(s(&Stmt::Let(LetStmt::new(id("y"), Some(Type::I64), None)), 0), "let y: i64;");
    let a = AssignStmt { target: var("x"), value: var("y"), origin: VarOrigin::default() };
    assert_eq!(s(&Stmt::Assign(a), 2), "        x = y;");
    assert_eq!(s(&Stmt::Return(None), 0), "return;");
    assert_eq!(s(&Stmt::Return(Some(var("r"))), 1), "    return r;");
    assert_eq!(s(&Stmt::Break(None), 0), "break;");
    assert_eq!(s(&Stmt::Break(Some(label("b3"))), 0), "break 'b3;");
    assert_eq!(s(&Stmt::Continue(Some(label("l1"))), 0), "continue 'l1;");
    // 多行 Raw 只缩进首行（Python 同形）
    assert_eq!(s(&raw("a;\nb;"), 1), "    a;\nb;");
}

// ── 结构化控制流（emit.py 结构行形态）─────────────────────────────────────

#[test]
fn loops_and_blocks() {
    let lp = Stmt::Loop(LoopStmt { label: Some(label("l0")), body: vec![raw("f();"), Stmt::Break(None)] });
    assert_eq!(s(&lp, 1), "    'l0: loop {\n        f();\n        break;\n    }");
    let wh = Stmt::While(WhileStmt { label: None, cond: var("c"), body: vec![] });
    assert_eq!(s(&wh, 0), "while c {\n}");
    let blk = Stmt::LabeledBlock { label: label("b1"), body: vec![Stmt::Break(Some(label("b1")))] };
    assert_eq!(s(&blk, 0), "'b1: {\n    break 'b1;\n}");
}

#[test]
fn if_chain() {
    let inner = IfStmt { cond: var("d"), then: vec![raw("g();")], else_: ElseBranch::Block(vec![raw("h();")]) };
    let outer = IfStmt { cond: var("c"), then: vec![raw("f();")], else_: ElseBranch::If(Box::new(inner)) };
    assert_eq!(
        s(&Stmt::If(outer), 1),
        "    if c {\n        f();\n    } else if d {\n        g();\n    } else {\n        h();\n    }"
    );
    let bare = IfStmt { cond: var("c"), then: vec![], else_: ElseBranch::None };
    assert_eq!(s(&Stmt::If(bare), 0), "if c {\n}");
}

#[test]
fn match_dispatch() {
    let unreachable = Expr::Macro(MacroCall { name: id("unreachable"), args: vec![] });
    let m = MatchStmt {
        scrutinee: var("_pc"),
        arms: vec![
            MatchArm {
                pattern: Pattern::Alts(vec![Lit::Int { value: 0, ty: None }, Lit::Int { value: -1, ty: None }]),
                body: ArmBody::Block(vec![raw("_pc = 2;")]),
            },
            MatchArm { pattern: Pattern::Wildcard, body: ArmBody::Expr(unreachable) },
        ],
    };
    assert_eq!(
        s(&Stmt::Match(m), 1),
        "    match _pc {\n        0 | -1 => {\n            _pc = 2;\n        }\n        _ => unreachable!(),\n    }"
    );
}

#[test]
fn java_try_catch() {
    let t = TryStmt {
        body: vec![raw("f()?;")],
        catches: vec![
            CatchClause {
                bind: id("e"),
                types: vec!["p/AEx".into(), "p/BEx".into()],
                lub: Some(Type::named(id("Ex"), vec![])),
                body: vec![raw("g();")],
            },
            CatchClause { bind: id("t"), types: vec![], lub: None, body: vec![] },
        ],
    };
    assert_eq!(
        s(&Stmt::JavaTry(t), 1),
        "    java_try! {\n        try {\n            f()?;\n        } catch (e: AEx | BEx as Ex) {\n            g();\n        } catch (t) {\n        }\n    }"
    );
}

// ── 条目（← render_fn / render_struct / render_impl / render_item）─────────

fn fn_item(name: &str, body: Vec<Stmt>) -> FnItem {
    FnItem {
        name: id(name),
        params: vec![Param { name: id("a"), ty: Type::I32 }, Param { name: id("b"), ty: Type::BOOL }],
        ret: Some(Type::I64),
        body,
        vis: Vis::Pub,
        is_unsafe: false,
    }
}

#[test]
fn items() {
    let rd = Renderer::new(&TailNames);
    let f = fn_item("f", vec![raw("todo;")]);
    assert_eq!(rd.item(&Item::Fn(f.clone()), 0), "pub fn f(a: i32, b: bool) -> i64 {\n    todo;\n}");
    let mut empty = fn_item("g", vec![]);
    empty.vis = Vis::Private;
    empty.is_unsafe = true;
    assert_eq!(rd.item(&Item::Fn(empty.clone()), 1), "    unsafe fn g(a: i32, b: bool) -> i64 {}");

    let st = StructItem {
        name: id("S"),
        fields: vec![
            StructField { name: id("x"), ty: Type::I32, vis: Vis::Pub },
            StructField { name: id("y"), ty: Type::BOOL, vis: Vis::Private },
        ],
        vis: Vis::PubCrate,
        derives: vec![Path::from_idents([id("Clone")]), Path::from_idents([id("Debug")])],
    };
    assert_eq!(
        rd.item(&Item::Struct(st), 0),
        "#[derive(Clone, Debug)]\npub(crate) struct S {\n    pub x: i32,\n    y: bool,\n}"
    );
    let unit = StructItem { name: id("U"), fields: vec![], vis: Vis::Pub, derives: vec![] };
    assert_eq!(rd.item(&Item::Struct(unit), 0), "pub struct U;");

    let imp = ImplItem {
        self_ty: Type::named(id("S"), vec![]),
        trait_: Some(Path::from_idents([id("T")])),
        items: vec![f, empty],
    };
    assert_eq!(
        rd.item(&Item::Impl(imp), 0),
        "impl T for S {\n    pub fn f(a: i32, b: bool) -> i64 {\n        todo;\n    }\n\n    unsafe fn g(a: i32, b: bool) -> i64 {}\n}"
    );

    let u = UseTree::Path {
        prefix: Path::from_idents([id("crate"), id("x")]),
        tail: Box::new(UseTree::Group(vec![
            UseTree::Name(id("A")),
            UseTree::Rename { name: id("B"), alias: id("C") },
            UseTree::Glob,
        ])),
    };
    assert_eq!(rd.item(&Item::Use(u), 0), "use crate::x::{A, B as C, *};");
    assert_eq!(rd.item(&Item::Mod(ModItem { name: id("m"), vis: Vis::Pub }), 0), "pub mod m;");
    let ta = TypeAlias { name: id("T"), ty: Type::I32, vis: Vis::Private };
    assert_eq!(rd.item(&Item::TypeAlias(ta), 0), "type T = i32;");
}

#[test]
fn file() {
    let rd = Renderer::new(&TailNames);
    let items = vec![Item::Mod(ModItem { name: id("a"), vis: Vis::Pub }), Item::Raw(Raw("// x".into()))];
    assert_eq!(rd.file(&items, "//! pre\n\n"), "//! pre\n\npub mod a;\n\n// x\n");
    assert_eq!(rd.file(&[], ""), "\n");
}
