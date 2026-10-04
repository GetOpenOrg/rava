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
fn overloaded_names_count_unimplemented_interface_members() {
    const ABSTRACT: u16 = 0x0400;
    const SYNTHETIC: u16 = 0x1000;
    let mut s = base_specs();
    s.push(class("p/View").iface().method(method(PUBLIC | ABSTRACT, "read", "()Ljava/lang/String;", None)));
    s.push(class("p/Named").iface().ifaces(&["p/View"]));
    // 自有 read(String[]) 与接口抽象 read() 同名：直接接口与超接口传递两种形态都须 mangle
    s.push(
        class("p/AbsDirect")
            .ifaces(&["p/View"])
            .method(method(PUBLIC, "read", "([Ljava/lang/String;)Ljava/lang/String;", None)),
    );
    s.push(
        class("p/AbsTransitive")
            .ifaces(&["p/Named"])
            .method(method(PUBLIC, "read", "([Ljava/lang/String;)Ljava/lang/String;", None)),
    );
    // 泛型桥：实现 compareTo(T) + 合成桥 compareTo(Object)，接口成员由桥承载，不制造重载
    s.push(class("p/Cmp").iface().method(method(PUBLIC | ABSTRACT, "compareTo", &format!("(L{OBJECT};)I"), None)));
    s.push(
        class("p/Impl")
            .ifaces(&["p/Cmp"])
            .method(method(PUBLIC, "compareTo", "(Lp/Impl;)I", None))
            .method(method(PUBLIC | SYNTHETIC, "compareTo", &format!("(L{OBJECT};)I"), None)),
    );
    let f = Fixture::new(s);
    let x = f.ctx();
    let names = |c: &str| f.reg.get(c).map(|ci| x.hierarchy_overloaded_names(ci));
    assert!(names("p/AbsDirect").is_some_and(|n| n.contains("read")));
    assert!(names("p/AbsTransitive").is_some_and(|n| n.contains("read")));
    assert!(names("p/Impl").is_some_and(|n| !n.contains("compareTo")));
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

/// 父类经接口 default 继承的槽位（`Headers implements Map<String, List>` 未声明 `replace`）：子类桥方法
/// `replace(Object, Object)` 以该 default 为根，按接口实参得 `(String, String) -> String`，与父类槽签名一致
#[test]
fn method_sig_types_roots_at_ancestor_inherited_default() {
    const SYNTHETIC: u16 = 0x1000;
    const BRIDGE: u16 = 0x0040;
    let obj2 = format!("(L{OBJECT};L{OBJECT};)L{OBJECT};");
    let mut s = base_specs();
    s.push(
        class("p/Dict")
            .iface()
            .sig(&format!("<K:L{OBJECT};V:L{OBJECT};>L{OBJECT};"))
            .method(method(PUBLIC, "replace", &obj2, Some("(TK;TV;)TV;"))),
    );
    s.push(
        class("p/Headers")
            .ifaces(&["p/Dict"])
            .sig(&format!("L{OBJECT};Lp/Dict<L{STRING};L{STRING};>;")),
    );
    s.push(
        class("p/Unmod")
            .sup("p/Headers")
            .method(method(PUBLIC, "replace", &format!("(L{STRING};L{STRING};)L{STRING};"), None))
            .method(method(PUBLIC | SYNTHETIC | BRIDGE, "replace", &obj2, None)),
    );
    // 本类自己引入接口时不改变：本类声明即槽位
    s.push(
        class("p/Own")
            .ifaces(&["p/Dict"])
            .sig(&format!("L{OBJECT};Lp/Dict<L{STRING};L{STRING};>;"))
            .method(method(PUBLIC | SYNTHETIC | BRIDGE, "replace", &obj2, None)),
    );
    let f = Fixture::new(s);
    let x = f.ctx();
    let Some(unmod) = f.reg.get("p/Unmod") else {
        panic!("fixture")
    };
    let string = RsType::class(STRING, vec![]);
    let sig = x.method_sig_types(unmod, &unmod.methods()[1], &[]);
    assert_eq!(sig.params, vec![string.clone(), string.clone()]);
    assert_eq!(sig.ret, Some(string));
    let Some(own) = f.reg.get("p/Own") else {
        panic!("fixture")
    };
    let st = x.method_sig_types(own, &own.methods()[0], &[]);
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
