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

const ABSTRACT: u16 = 0x0400;

/// 接口层次：子接口自有方法与超接口（传递）同名异参 → 子接口的该名按描述符区分，
/// 超接口不受子接口影响；泛型特化覆盖（`extends Cmp<Path>` 的 `compareTo(Path)`）不构成重载
fn iface_specs() -> Vec<crate::testutil::ClassSpec> {
    let s3 = format!("(L{STRING};L{STRING};L{STRING};)V");
    let s1 = format!("(L{STRING};)V");
    let mut s = base_specs();
    s.push(class("p/Handler").iface().method(method(PUBLIC | ABSTRACT, "end", &s3, None)));
    s.push(class("p/ExtHandler").iface().ifaces(&["p/Handler"]).method(method(PUBLIC | ABSTRACT, "end", &s1, None)));
    s.push(class("p/SerHandler").iface().ifaces(&["p/ExtHandler"]).method(method(PUBLIC | ABSTRACT, "flush", "()V", None)));
    s.push(
        class("p/Cmp")
            .iface()
            .sig(&format!("<T:L{OBJECT};>L{OBJECT};"))
            .method(method(PUBLIC | ABSTRACT, "compareTo", &format!("(L{OBJECT};)I"), Some("(TT;)I"))),
    );
    s.push(
        class("p/Path")
            .iface()
            .ifaces(&["p/Cmp"])
            .sig(&format!("L{OBJECT};Lp/Cmp<Lp/Path;>;"))
            .method(method(PUBLIC | ABSTRACT, "compareTo", "(Lp/Path;)I", None)),
    );
    s.push(class("p/SubPath").iface().ifaces(&["p/Path"]).method(method(PUBLIC | ABSTRACT, "compareTo", "(Lp/Path;)I", None)));
    // 代入后擦除不同（Cmp<String> 的 compareTo(T) 擦除为 String）：真重载
    s.push(
        class("p/Odd")
            .iface()
            .ifaces(&["p/Cmp"])
            .sig(&format!("L{OBJECT};Lp/Cmp<L{STRING};>;"))
            .method(method(PUBLIC | ABSTRACT, "compareTo", "(Lp/Path;)I", None)),
    );
    // 兄弟超接口各自声明同名异参
    s.push(class("p/Left").iface().method(method(PUBLIC | ABSTRACT, "put", &s1, None)));
    s.push(class("p/Right").iface().method(method(PUBLIC | ABSTRACT, "put", "(I)V", None)));
    s.push(class("p/Both").iface().ifaces(&["p/Left", "p/Right"]));
    s.push(class("p/OnlyLeft").iface().ifaces(&["p/Left"]));
    s
}

#[test]
fn interface_overload_spans_superinterfaces() {
    let f = Fixture::new(iface_specs());
    let x = f.ctx();
    let names = |c: &str| f.reg.get(c).map(|ci| x.hierarchy_overloaded_names(ci));
    assert!(names("p/Handler").is_some_and(|n| !n.contains("end")));
    assert!(names("p/ExtHandler").is_some_and(|n| n.contains("end")));
    // SerHandler 不自有 end：名字由声明接口决定
    assert!(names("p/SerHandler").is_some_and(|n| !n.contains("end")));
    assert!(names("p/Path").is_some_and(|n| !n.contains("compareTo")));
    assert!(names("p/SubPath").is_some_and(|n| !n.contains("compareTo")));
    assert!(names("p/Odd").is_some_and(|n| n.contains("compareTo")));
}

#[test]
fn interface_view_names_consistent_and_disjoint() {
    let f = Fixture::new(iface_specs());
    let x = f.ctx();
    let ci = |c: &str| f.reg.get(c).expect(c);
    let s3 = format!("(L{STRING};L{STRING};L{STRING};)V");
    let s1 = format!("(L{STRING};)V");
    // 子接口视图沿用声明名：Handler.end 原名，ExtHandler.end 带后缀，两者不同
    let h = x.interface_view_member_name(ci("p/SerHandler"), ci("p/Handler"), "end", &s3);
    let e = x.interface_view_member_name(ci("p/SerHandler"), ci("p/ExtHandler"), "end", &s1);
    assert_eq!(h, "end");
    assert_ne!(e, "end");
    assert_eq!(e, x.interface_view_member_name(ci("p/ExtHandler"), ci("p/ExtHandler"), "end", &s1));
    // 兄弟超接口同名异参：视图内两者按描述符区分；只继承一侧时取声明名
    let l = x.interface_view_member_name(ci("p/Both"), ci("p/Left"), "put", &s1);
    let r = x.interface_view_member_name(ci("p/Both"), ci("p/Right"), "put", "(I)V");
    assert!(l != "put" && r != "put" && l != r);
    assert_eq!(x.interface_view_member_name(ci("p/OnlyLeft"), ci("p/Left"), "put", &s1), "put");
    // 被特化覆盖的超接口成员不制造视图冲突
    let p = x.interface_view_member_name(ci("p/SubPath"), ci("p/Path"), "compareTo", "(Lp/Path;)I");
    assert_eq!(p, "compareTo");
}

#[test]
fn class_view_disjoint_for_interface_only_members() {
    let mut s = iface_specs();
    // 抽象类不声明 put / end：经兄弟接口、超接口层次继承到同名异参的抽象成员
    s.push(class("p/AbsBoth").ifaces(&["p/Left", "p/Right"]));
    s.push(class("p/AbsSer").ifaces(&["p/SerHandler"]));
    s.push(class("p/SubSer").sup("p/AbsSer"));
    // 只经一侧继承：不制造重载
    s.push(class("p/AbsLeft").ifaces(&["p/Left"]));
    // 祖先类声明 put(I)，接口只声明 put(String)：类视图两种参数段
    s.push(class("p/HasInt").method(method(PUBLIC, "put", "(I)V", None)));
    s.push(class("p/AbsMixed").sup("p/HasInt").ifaces(&["p/Left"]));
    let f = Fixture::new(s);
    let x = f.ctx();
    let ci = |c: &str| f.reg.get(c).expect(c);
    let s3 = format!("(L{STRING};L{STRING};L{STRING};)V");
    let s1 = format!("(L{STRING};)V");
    let local = |c: &str, n: &str, d: &str| x.interface_member_local_name(ci(c), n, d);
    let (l, r) = (local("p/AbsBoth", "put", &s1), local("p/AbsBoth", "put", "(I)V"));
    assert!(l != "put" && r != "put" && l != r);
    let (e3, e1) = (local("p/AbsSer", "end", &s3), local("p/AbsSer", "end", &s1));
    assert!(e3 != "end" && e1 != "end" && e3 != e1);
    // 子类沿用（单调继承），同一成员在类层次上同名
    assert_eq!(e3, local("p/SubSer", "end", &s3));
    assert_eq!(local("p/AbsLeft", "put", &s1), "put");
    // 祖先类成员与接口成员在类视图上同按描述符区分
    assert_ne!(x.receiver_member_name("put", "(I)V", ci("p/AbsMixed")), "put");
    assert_ne!(local("p/AbsMixed", "put", &s1), "put");
    // 接口自身的声明名不受实现类影响
    assert!(x.hierarchy_overloaded_names(ci("p/Handler")).is_empty());
    assert!(!x.hierarchy_overloaded_names(ci("p/Left")).contains("put"));
}
