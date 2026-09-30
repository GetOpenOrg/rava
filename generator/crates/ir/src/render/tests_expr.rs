//! 表达式 / 转换渲染单测（← tests/unit/test_upcast_expr.py、test_try_expr.py 及补充形态）。

use crate::{
    BinOp, BlockExpr, CastExpr, CastMode, Expr, FnPath, IfExpr, Ident, Lit, Path, PathSegment, Raw,
    Renderer, ShortNames, StaticFieldRef, Type, UnOp, UpcastWrap,
};

/// 测试用短名：binary 名末段，`$` → `_`。
pub(super) struct TailNames;

impl ShortNames for TailNames {
    fn short_cls(&self, binary: &str) -> String {
        binary.rsplit('/').next().unwrap_or(binary).replace('$', "_")
    }
}

pub(super) fn id(s: &str) -> Ident {
    Ident::new(s).unwrap()
}

pub(super) fn var(s: &str) -> Expr {
    Expr::Var(id(s))
}

fn raw(s: &str) -> Expr {
    Expr::Raw(Raw(s.into()))
}

fn named(s: &str) -> Type {
    Type::named(id(s), vec![])
}

fn r(e: &Expr) -> String {
    Renderer::new(&TailNames).expr(e)
}

/// `Foo::new()?`
fn ctor_try() -> Expr {
    Expr::try_(Expr::call(Path::from_idents([id("Foo"), id("new")]), vec![]))
}

/// `a.b()?`
fn chain_try() -> Expr {
    Expr::try_(Expr::method(var("a"), id("b"), vec![]))
}

// ── test_upcast_expr ───────────────────────────────────────────────────────

#[test]
fn upcast_clone() {
    let field = Expr::Field { recv: Box::new(var("this")), name: id("field") };
    for (e, text) in [(var("x"), "x"), (field, "this.field"), (var("arr_3"), "arr_3")] {
        assert_eq!(r(&Expr::upcast(e, UpcastWrap::Clone)), format!("Clone::clone(&{text}).into()"));
    }
}

#[test]
fn upcast_paren() {
    for (e, text) in [(chain_try(), "a.b()?"), (ctor_try(), "Foo::new()?"), (var("x"), "x")] {
        assert_eq!(r(&Expr::upcast(e, UpcastWrap::Paren)), format!("({text}).into()"));
    }
}

#[test]
fn upcast_auto_matches_old_vars_form() {
    let if_e = Expr::If(IfExpr {
        cond: Box::new(var("c")),
        then: BlockExpr { stmts: vec![], tail: Some(Box::new(var("a"))) },
        else_: Some(BlockExpr { stmts: vec![], tail: Some(Box::new(var("b"))) }),
    });
    let cases = [
        (var("x"), true),
        (chain_try(), true),
        (ctor_try(), true),
        (if_e, false),
        (Expr::binary(BinOp::Add, var("a"), var("b")), false),
        (Expr::paren(var("a")), true),
    ];
    let rd = Renderer::new(&TailNames);
    for (e, atomic) in cases {
        let text = r(&e);
        assert_eq!(rd.is_atomic(&e), atomic, "{text}");
        assert_eq!(rd.is_atomic(&e), super::atomic::scan_atomic(&text), "{text}");
        let want = if atomic { format!("{text}.into()") } else { format!("({text}).into()") };
        assert_eq!(r(&Expr::upcast(e, UpcastWrap::Auto)), want);
    }
}

#[test]
fn upcast_bare() {
    let clone_this = Expr::call(Path::from_idents([id("Clone"), id("clone")]), vec![var("this")]);
    let clone_e = Expr::call(Path::from_idents([id("Clone"), id("clone")]), vec![Expr::reference(var("e"))]);
    let foo = Expr::try_(Expr::call(Path::from_idents([id("foo")]), vec![]));
    for (e, text) in [(clone_this, "Clone::clone(this)"), (clone_e, "Clone::clone(&e)"), (foo, "foo()?")] {
        assert_eq!(r(&Expr::upcast(e, UpcastWrap::Bare)), format!("{text}.into()"));
    }
}

#[test]
fn upcast_owned() {
    assert_eq!(r(&Expr::upcast(var("this"), UpcastWrap::Owned)), "Clone::clone(&this).into()");
    assert_eq!(r(&Expr::upcast(chain_try(), UpcastWrap::Owned)), "(a.b()?).into()");
}

#[test]
fn structural_atomicity() {
    let rd = Renderer::new(&TailNames);
    let neg = Expr::Unary { op: UnOp::Neg, expr: Box::new(var("x")) };
    assert!(!rd.is_atomic(&neg));
    assert!(!rd.is_atomic(&raw("a + b")));
    assert!(rd.is_atomic(&raw("f(a + b)")));
    // turbofish 在顶层 → 非原子
    let tf = Expr::call(
        Path::new(vec![PathSegment::with_generics(id("Foo"), vec![named("Object")]), PathSegment::new(id("new"))]),
        vec![],
    );
    assert_eq!(r(&tf), "Foo::<Object>::new()");
    assert!(!rd.is_atomic(&tf));
    let sf = StaticFieldRef { class: "p/Foo".into(), field: id("X"), ty: Type::I32, turbofish: vec![] };
    assert!(rd.is_atomic(&Expr::StaticField(sf)));
}

// ── test_try_expr ──────────────────────────────────────────────────────────

#[test]
fn try_method_call() {
    assert_eq!(r(&Expr::try_(Expr::method(var("list"), id("size"), vec![]))), "list.size()?");
}

// ── 补充形态 ───────────────────────────────────────────────────────────────

#[test]
fn binary_precedence() {
    // (a + b) * c；a - (b - c)；a - b - c
    let add = Expr::binary(BinOp::Add, var("a"), var("b"));
    assert_eq!(r(&Expr::binary(BinOp::Mul, add, var("c"))), "(a + b) * c");
    let sub = Expr::binary(BinOp::Sub, var("b"), var("c"));
    assert_eq!(r(&Expr::binary(BinOp::Sub, var("a"), sub)), "a - (b - c)");
    let sub = Expr::binary(BinOp::Sub, var("a"), var("b"));
    assert_eq!(r(&Expr::binary(BinOp::Sub, sub, var("c"))), "a - b - c");
    let and = Expr::binary(BinOp::And, var("b"), var("c"));
    assert_eq!(r(&Expr::binary(BinOp::Or, var("a"), and)), "a || b && c");
}

#[test]
fn numeric_cast_and_calls() {
    let c = Expr::Cast { expr: Box::new(var("x")), ty: Type::I64, outer_paren: true };
    assert_eq!(r(&c), "(x as i64)");
    let q = Expr::Call {
        func: FnPath::Qualified {
            self_ty: Box::new(Type::named(id("X"), vec![named("A")])),
            trait_: Some(Path {
                global: true,
                segments: vec![
                    PathSegment::new(id("std")),
                    PathSegment::new(id("convert")),
                    PathSegment::with_generics(id("From"), vec![Type::Infer]),
                ],
            }),
            rest: vec![PathSegment::new(id("from"))],
        },
        args: vec![var("v")],
    };
    assert_eq!(r(&q), "<X<A> as ::std::convert::From<_>>::from(v)");
    let m = Expr::MethodCall { recv: Box::new(var("o")), method: id("get"), turbofish: vec![Type::I32], args: vec![] };
    assert_eq!(r(&m), "o.get::<i32>()");
}

#[test]
fn jvm_nodes() {
    assert_eq!(r(&Expr::NewPending { class: "p/Outer$Inner".into() }), "Outer_Inner::new()");
    let sf = StaticFieldRef { class: "p/Foo".into(), field: id("X"), ty: Type::I32, turbofish: vec![named("Object")] };
    assert_eq!(r(&Expr::StaticField(sf)), "Foo::<Object>::X()?");
    let io = Expr::InstanceOf { expr: Box::new(var("o")), binary: "p/Foo".into() };
    assert_eq!(r(&io), "(o).is_instance_of(\"p/Foo\")");
}

#[test]
fn checkcast_forms() {
    let mk = |e: Expr, target: Type, mode: CastMode, box_first: bool| {
        r(&Expr::CheckCast(CastExpr { expr: Box::new(e), target, mode, box_first }))
    };
    let checked = CastMode::Checked { binary: "p/Foo".into() };
    assert_eq!(mk(var("o"), named("Foo"), checked.clone(), false), "Clone::clone(&o).try_cast::<Foo>(\"p/Foo\")?");
    assert_eq!(mk(var("this"), named("Foo"), checked, true), "Object::from(Clone::clone(this)).try_cast::<Foo>(\"p/Foo\")?");
    let arr = Type::named(id("JArray"), vec![Type::I32]);
    let checked = CastMode::Checked { binary: "[I".into() };
    assert_eq!(mk(var("o"), arr, checked, false), "Clone::clone(&o).try_cast_array::<i32>(\"[I\")?");
    let iface = CastMode::CheckedInterface { binary: "p/I".into() };
    assert_eq!(mk(var("o"), named("Object"), iface, false), "Clone::clone(&o).try_cast_iface(\"p/I\")?");
    assert_eq!(
        mk(var("o"), named("Foo"), CastMode::Unchecked, false),
        "<Foo as ::std::convert::From<Object>>::from(Clone::clone(&o))"
    );
}

#[test]
fn if_expr_and_block() {
    let dead = Expr::If(IfExpr {
        cond: Box::new(Expr::Lit(Lit::Bool(false))),
        then: BlockExpr { stmts: vec![], tail: Some(Box::new(var("a"))) },
        else_: Some(BlockExpr { stmts: vec![], tail: Some(Box::new(var("b"))) }),
    });
    assert_eq!(r(&dead), "{\n    b\n}");
    let dead_no_else = Expr::If(IfExpr {
        cond: Box::new(Expr::Lit(Lit::Bool(false))),
        then: BlockExpr::default(),
        else_: None,
    });
    assert_eq!(r(&dead_no_else), "{}");
    let live = Expr::If(IfExpr {
        cond: Box::new(var("c")),
        then: BlockExpr { stmts: vec![], tail: Some(Box::new(Expr::Lit(Lit::i32(1)))) },
        else_: Some(BlockExpr { stmts: vec![], tail: Some(Box::new(Expr::Lit(Lit::i32(2)))) }),
    });
    assert_eq!(r(&live), "if c {\n    1i32\n} else {\n    2i32\n}");
}
