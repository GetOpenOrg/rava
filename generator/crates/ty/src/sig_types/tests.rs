//! sig_types 最小回归：重载判定的继承传播、覆盖方法取最远祖先、外围方法形参继承。

use crate::consts::{OBJECT, STRING};
use crate::rs_type::RsType;
use crate::testutil::{class, method, Fixture};

const PUBLIC: u16 = 0x0001;
const STATIC: u16 = 0x0008;

fn base_specs() -> Vec<crate::testutil::ClassSpec> {
    vec![class(OBJECT).sup(""), class(STRING)]
}

#[test]
fn overloaded_names_inherit_and_static_rule() {
    let mut s = base_specs();
    s.push(
        class("p/A")
            .method(method(PUBLIC, "put", "(I)V", None))
            .method(method(PUBLIC, "get", "()I", None)),
    );
    s.push(
        class("p/B")
            .sup("p/A")
            .method(method(PUBLIC, "put", "(J)V", None))
            .method(method(PUBLIC | STATIC, "get", "(I)I", None)),
    );
    s.push(class("p/C").sup("p/B"));
    let f = Fixture::new(s);
    let x = f.ctx();
    let a = f.reg.get("p/A").map(|c| x.hierarchy_overloaded_names(c));
    let b = f.reg.get("p/B").map(|c| x.hierarchy_overloaded_names(c));
    let c = f.reg.get("p/C").map(|c| x.hierarchy_overloaded_names(c));
    assert!(a.is_some_and(|n| n.is_empty()));
    assert!(b.is_some_and(|n| n.contains("put") && n.contains("get")));
    assert!(c.is_some_and(|n| n.contains("put") && n.contains("get")));
}

#[test]
fn method_sig_types_uses_farthest_ancestor() {
    let mut s = base_specs();
    s.push(
        class("p/Base")
            .sig(&format!("<T:L{OBJECT};>L{OBJECT};"))
            .method(method(
                PUBLIC,
                "take",
                &format!("(L{OBJECT};)L{OBJECT};"),
                Some("(TT;)TT;"),
            )),
    );
    s.push(
        class("p/Mid")
            .sup("p/Base")
            .sig(&format!("Lp/Base<L{STRING};>;"))
            .method(method(
                PUBLIC,
                "take",
                &format!("(L{OBJECT};)L{OBJECT};"),
                None,
            )),
    );
    s.push(
        class("p/Leaf")
            .sup("p/Mid")
            .method(method(
                PUBLIC,
                "take",
                &format!("(L{OBJECT};)L{OBJECT};"),
                None,
            ))
            .method(method(PUBLIC | STATIC, "take", "(I)I", None)),
    );
    let f = Fixture::new(s);
    let x = f.ctx();
    let Some(leaf) = f.reg.get("p/Leaf") else {
        panic!("fixture")
    };
    let inst = &leaf.methods()[0];
    let sig = x.method_sig_types(leaf, inst, &[]);
    let string = RsType::class(STRING, vec![]);
    assert_eq!(sig.params, vec![string.clone()]);
    assert_eq!(sig.ret, Some(string));
    // static 方法不参与覆盖链；无泛型签名 → 退回描述符形态（ret = None）
    let st = x.method_sig_types(leaf, &leaf.methods()[1], &[]);
    assert!(st.params.is_empty() && st.ret.is_none());
}

#[test]
fn local_class_inherits_enclosing_method_tparams() {
    let mut s = base_specs();
    s.push(class("p/Outer").method(method(
        PUBLIC | STATIC,
        "make",
        "()V",
        Some(&format!("<U:L{OBJECT};>()V")),
    )));
    s.push(
        class("p/Outer$1Local")
            .sig(&format!("<V:L{OBJECT};>L{OBJECT};"))
            .enclosing("p/Outer", Some(("make", "()V"))),
    );
    let f = Fixture::new(s);
    let x = f.ctx();
    let Some(local) = f.reg.get("p/Outer$1Local") else {
        panic!("fixture")
    };
    assert_eq!(
        *x.effective_class_type_params(local),
        vec!["U".to_string(), "V".to_string()]
    );
}
