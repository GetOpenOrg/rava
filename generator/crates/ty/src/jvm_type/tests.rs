//! `tests/unit/test_jvm_type.py` 的 41 个用例（同名同序；Python 专有的断言在注释里说明）。

use std::collections::{BTreeMap, BTreeSet, HashSet};

use super::subtype::{strict_erased_subtype, super_closure};
use super::{JvmType, PrimKind, WildKind};
use crate::rs_type::{Prim, RsType};
use crate::testutil::{class, ClassSpec, Fixture};
use crate::Registry;

use crate::consts::{CLONEABLE, OBJECT, SERIALIZABLE, STRING};

const NUMBER: &str = "p/Number";
const INTEGER: &str = "p/Integer";
const CHARSEQ: &str = "p/CharSequence";
const LIST: &str = "p/List";
const ALIST: &str = "p/ArrayList";

fn specs() -> Vec<ClassSpec> {
    vec![
        class(OBJECT).sup(""),
        class(CHARSEQ).iface(),
        class(NUMBER),
        class(STRING).ifaces(&[CHARSEQ]),
        class(INTEGER).sup(NUMBER).ifaces(&["p/Comparable"]),
        class("p/Comparable").iface(),
        class("p/Iterable").iface(),
        class("p/Collection").iface().ifaces(&["p/Iterable"]),
        class(LIST).iface().ifaces(&["p/Collection"]),
        class("p/RandomAccess").iface(),
        class("p/AbstractList").ifaces(&[LIST]),
        class(ALIST)
            .sup("p/AbstractList")
            .ifaces(&[LIST, "p/RandomAccess"]),
    ]
}

fn fixture() -> Fixture {
    Fixture::new(specs())
}

fn empty() -> Registry {
    Registry::new()
}

fn c(b: &str) -> JvmType {
    JvmType::class(b)
}
fn ca(b: &str, args: Vec<JvmType>) -> JvmType {
    JvmType::class_with(b, args, false)
}
fn arr(e: JvmType) -> JvmType {
    JvmType::array(e)
}
fn tv(n: &str) -> JvmType {
    JvmType::type_var(n, None)
}
fn tvb(n: &str, b: JvmType) -> JvmType {
    JvmType::type_var(n, Some(b))
}
fn wc(k: WildKind, b: Option<JvmType>) -> JvmType {
    JvmType::wildcard(k, b)
}
fn prim(k: PrimKind) -> JvmType {
    JvmType::Primitive(k)
}
fn map(pairs: Vec<(&str, JvmType)>) -> BTreeMap<String, JvmType> {
    pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

fn samples() -> Vec<JvmType> {
    vec![
        prim(PrimKind::Int),
        prim(PrimKind::Void),
        c(STRING),
        ca(ALIST, vec![c(STRING)]),
        ca(LIST, vec![wc(WildKind::Extends, Some(c(NUMBER)))]),
        ca(LIST, vec![wc(WildKind::Super, None)]),
        arr(prim(PrimKind::Int)),
        arr(arr(c(STRING))),
        arr(tv("T")),
        tv("T"),
        tvb("E", c(NUMBER)),
        wc(WildKind::Unbounded, None),
        wc(WildKind::Extends, Some(c(STRING))),
        JvmType::Null,
    ]
}

// ── ErasureTests ──

#[test]
fn erasure_idempotent() {
    for t in samples() {
        let e1 = t.erasure();
        assert_eq!(e1, e1.erasure(), "{t:?}");
    }
}

#[test]
fn erasure_drops_type_args() {
    assert_eq!(ca(ALIST, vec![c(STRING)]).erasure(), c(ALIST));
}

#[test]
fn erasure_typevar_erases_to_bound() {
    assert_eq!(tvb("E", c(NUMBER)).erasure(), c(NUMBER));
    assert_eq!(tv("T").erasure(), c(OBJECT));
}

#[test]
fn erasure_array_erases_element() {
    assert_eq!(arr(ca(ALIST, vec![c(STRING)])).erasure(), arr(c(ALIST)));
    assert_eq!(arr(tvb("E", c(NUMBER))).erasure(), arr(c(NUMBER)));
}

#[test]
fn erasure_wildcard_erases_to_bound() {
    assert_eq!(wc(WildKind::Extends, Some(c(STRING))).erasure(), c(STRING));
    assert_eq!(wc(WildKind::Unbounded, None).erasure(), c(OBJECT));
}

// ── SubstituteTests ──

#[test]
fn substitute_composition() {
    let f = map(vec![("T", c(STRING))]);
    let g = map(vec![("E", c(INTEGER))]);
    let t = ca(
        "p/Pair",
        vec![
            tv("T"),
            tv("E"),
            arr(tv("T")),
            wc(WildKind::Extends, Some(tv("E"))),
        ],
    );
    let mut fg = f.clone();
    fg.extend(g.clone());
    assert_eq!(t.substitute(&f).substitute(&g), t.substitute(&fg));
}

#[test]
fn substitute_noop_without_mapping() {
    for t in samples() {
        assert_eq!(t.substitute(&BTreeMap::new()), t);
        assert_eq!(t.substitute(&map(vec![("X", c(STRING))])), t);
    }
}

#[test]
fn substitute_recurses_into_structure() {
    let f = map(vec![("T", c(INTEGER))]);
    let t = arr(ca("p/Box", vec![tv("T"), tv("U")]));
    assert_eq!(
        t.substitute(&f),
        arr(ca("p/Box", vec![c(INTEGER), tv("U")]))
    );
}

#[test]
fn substitute_substitutes_bound() {
    let f = map(vec![("F", c(STRING))]);
    let t = tvb("E", ca("p/Enum", vec![tv("F")]));
    assert_eq!(t.substitute(&f), tvb("E", ca("p/Enum", vec![c(STRING)])));
    let t = tvb("E", ca("p/Enum", vec![tv("E")]));
    assert_eq!(t.substitute(&map(vec![("E", c(STRING))])), c(STRING));
}

#[test]
fn substitute_hit_value_not_resubstituted() {
    let both = map(vec![("T", tv("S")), ("S", c(STRING))]);
    assert_eq!(tv("T").substitute(&both), tv("S"));
}

// ── SubtypeClassTests ──

#[test]
fn subtype_reflexive() {
    let f = fixture();
    for t in samples() {
        assert!(t.is_subtype_of(&t, &f.reg), "{t:?}");
    }
}

#[test]
fn subtype_class_closure_via_super_chain() {
    let f = fixture();
    let al = c(ALIST);
    assert!(al.is_subtype_of(&c("p/AbstractList"), &f.reg));
    assert!(al.is_subtype_of(&c(OBJECT), &f.reg));
    assert!(!c(STRING).is_subtype_of(&c(ALIST), &f.reg));
}

#[test]
fn subtype_interface_edges_transitive() {
    let f = fixture();
    let al = c(ALIST);
    assert!(al.is_subtype_of(&c(LIST), &f.reg));
    assert!(al.is_subtype_of(&c("p/Collection"), &f.reg));
    assert!(al.is_subtype_of(&c("p/Iterable"), &f.reg));
    assert!(c(LIST).is_subtype_of(&c("p/Iterable"), &f.reg));
}

#[test]
fn subtype_object_is_top_for_refs() {
    let f = fixture();
    assert!(c(INTEGER).is_subtype_of(&c(OBJECT), &f.reg));
    assert!(!prim(PrimKind::Int).is_subtype_of(&c(OBJECT), &f.reg));
}

#[test]
fn subtype_primitive_only_reflexive() {
    let e = empty();
    assert!(prim(PrimKind::Int).is_subtype_of(&prim(PrimKind::Int), &e));
    assert!(!prim(PrimKind::Int).is_subtype_of(&prim(PrimKind::Long), &e));
    assert!(!prim(PrimKind::Int).is_subtype_of(&c(INTEGER), &e));
}

// ── SubtypeArrayTests ──

#[test]
fn array_covariance() {
    let f = fixture();
    assert!(arr(c(STRING)).is_subtype_of(&arr(c(CHARSEQ)), &f.reg));
    assert!(!arr(c(CHARSEQ)).is_subtype_of(&arr(c(STRING)), &f.reg));
    assert!(arr(c(INTEGER)).is_subtype_of(&arr(c(NUMBER)), &f.reg));
}

#[test]
fn array_primitive_arrays_invariant() {
    let f = fixture();
    let e = empty();
    assert!(arr(prim(PrimKind::Int)).is_subtype_of(&arr(prim(PrimKind::Int)), &e));
    assert!(!arr(prim(PrimKind::Int)).is_subtype_of(&arr(prim(PrimKind::Long)), &e));
    assert!(!arr(prim(PrimKind::Int)).is_subtype_of(&arr(c(INTEGER)), &f.reg));
}

#[test]
fn array_fixed_supertypes() {
    let f = fixture();
    let sarr = arr(c(STRING));
    for top in [OBJECT, CLONEABLE, SERIALIZABLE] {
        assert!(sarr.is_subtype_of(&c(top), &f.reg), "{top}");
    }
    assert!(!sarr.is_subtype_of(&c(STRING), &f.reg));
    assert!(!c(STRING).is_subtype_of(&sarr, &f.reg));
}

// ── SubtypeTypeVarNullTests ──

#[test]
fn typevar_via_bound() {
    let f = fixture();
    let t = tvb("T", c(NUMBER));
    assert!(t.is_subtype_of(&c(OBJECT), &f.reg));
    assert!(t.is_subtype_of(&c(NUMBER), &f.reg));
    assert!(!t.is_subtype_of(&c(INTEGER), &f.reg));
    assert!(c(INTEGER).is_subtype_of(&t, &f.reg));
    assert!(!c(STRING).is_subtype_of(&t, &f.reg));
    assert!(tv("X").is_subtype_of(&c(OBJECT), &f.reg));
    assert!(!tv("X").is_subtype_of(&c(NUMBER), &f.reg));
}

#[test]
fn null_bottom() {
    let f = fixture();
    let e = empty();
    for target in [
        c(STRING),
        arr(c(STRING)),
        tvb("T", c(NUMBER)),
        wc(WildKind::Extends, Some(c(STRING))),
    ] {
        assert!(JvmType::Null.is_subtype_of(&target, &f.reg), "{target:?}");
    }
    assert!(JvmType::Null.is_subtype_of(&JvmType::Null, &e));
    assert!(!JvmType::Null.is_subtype_of(&prim(PrimKind::Int), &e));
    assert!(!c(STRING).is_subtype_of(&JvmType::Null, &e));
}

// ── WildcardTests ──

fn list_of(arg: JvmType) -> JvmType {
    ca(LIST, vec![arg])
}

#[test]
fn wildcard_containment_extends() {
    let f = fixture();
    assert!(
        list_of(c(STRING)).is_subtype_of(&list_of(wc(WildKind::Extends, Some(c(CHARSEQ)))), &f.reg)
    );
    assert!(
        !list_of(c(STRING)).is_subtype_of(&list_of(wc(WildKind::Extends, Some(c(NUMBER)))), &f.reg)
    );
}

#[test]
fn wildcard_containment_unbounded() {
    let f = fixture();
    assert!(list_of(c(STRING)).is_subtype_of(&list_of(wc(WildKind::Unbounded, None)), &f.reg));
}

#[test]
fn wildcard_containment_super() {
    let f = fixture();
    let target = list_of(wc(WildKind::Super, Some(c(NUMBER))));
    assert!(list_of(c(NUMBER)).is_subtype_of(&target, &f.reg));
    assert!(list_of(c(OBJECT)).is_subtype_of(&target, &f.reg));
    assert!(!list_of(c(INTEGER)).is_subtype_of(&target, &f.reg));
    assert!(!list_of(c(STRING)).is_subtype_of(&target, &f.reg));
}

#[test]
fn wildcard_args_invariant_without_wildcard() {
    let f = fixture();
    assert!(!list_of(c(STRING)).is_subtype_of(&list_of(c(OBJECT)), &f.reg));
}

#[test]
fn wildcard_raw_target_unchecked() {
    let f = fixture();
    assert!(ca(ALIST, vec![c(STRING)]).is_subtype_of(&c(ALIST), &f.reg));
    assert!(!c(ALIST).is_subtype_of(&ca(ALIST, vec![c(STRING)]), &f.reg));
}

#[test]
fn wildcard_arity_mismatch() {
    let f = fixture();
    let a1 = ca("p/Box", vec![c(STRING)]);
    let a2 = ca("p/Box", vec![c(STRING), c(STRING)]);
    assert!(!a1.is_subtype_of(&a2, &f.reg));
    assert!(!a2.is_subtype_of(&a1, &f.reg));
}

// ── ClosureCacheTests ──

/// Python 版本在判定后向同一 registry 增类验证缓存失效；Rust 注册表构建后不可变，
/// 等价语义是「另建一个注册表，缓存互不串扰」
#[test]
fn closure_cache_visible_effect_and_invalidation() {
    let f = fixture();
    assert!(!c(ALIST).is_subtype_of(&c("p/List2"), &f.reg));
    let mut more = specs();
    more.push(class("p/ArrayList2").sup(ALIST).ifaces(&[LIST]));
    let f2 = Fixture::new(more);
    let al2 = c("p/ArrayList2");
    assert!(al2.is_subtype_of(&c(LIST), &f2.reg));
    assert!(al2.is_subtype_of(&c("p/Iterable"), &f2.reg));
    assert!(super_closure("p/ArrayList2", &f2.reg).contains("p/ArrayList2"));
    assert!(!super_closure("p/ArrayList2", &f.reg).contains(ALIST));
}

#[test]
fn closure_unregistered_ancestor_is_closure_leaf() {
    let mut s = specs();
    s.push(class("p/A").sup("q/Unknown"));
    let f = Fixture::new(s);
    assert!(super_closure("p/A", &f.reg).contains("q/Unknown"));
    assert!(c("p/A").is_subtype_of(&c("q/Unknown"), &f.reg));
    // Python 的短名占位目标命中（`_closure_hit`）不移植：binary 即身份
    assert!(!c("p/A").is_subtype_of(&c("Unknown"), &f.reg));
}

// ── ConstructionTests ──

#[test]
fn construction_from_descriptor() {
    assert_eq!(JvmType::from_descriptor("I"), Ok(prim(PrimKind::Int)));
    assert_eq!(JvmType::from_descriptor("V"), Ok(prim(PrimKind::Void)));
    assert_eq!(JvmType::from_descriptor("[I"), Ok(arr(prim(PrimKind::Int))));
    assert_eq!(
        JvmType::from_descriptor(&format!("L{STRING};")),
        Ok(c(STRING))
    );
    assert_eq!(
        JvmType::from_descriptor(&format!("[[L{OBJECT};")),
        Ok(arr(arr(c(OBJECT))))
    );
    for bad in ["", "X", "LI", "Lp/Foo", "I["] {
        assert!(JvmType::from_descriptor(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn construction_from_signature() {
    let e = empty();
    let p = |s: &str| JvmType::from_signature(s, &e);
    assert_eq!(p("I"), Ok(prim(PrimKind::Int)));
    assert_eq!(p(&format!("L{LIST}<L{STRING};>;")), Ok(list_of(c(STRING))));
    assert_eq!(p("TT;"), Ok(tv("T")));
    assert_eq!(p("[TT;"), Ok(arr(tv("T"))));
    assert_eq!(p("*"), Ok(wc(WildKind::Unbounded, None)));
    assert_eq!(p("+TT;"), Ok(wc(WildKind::Extends, Some(tv("T")))));
    assert_eq!(p("-Lp/Number;"), Ok(wc(WildKind::Super, Some(c(NUMBER)))));
    assert_eq!(p(&format!("[L{STRING};")), Ok(arr(c(STRING))));
    for bad in ["", "T", "Lp/List<;", "Lp/Foo;X"] {
        assert!(p(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn construction_from_signature_inner_class() {
    let t = JvmType::from_signature(&format!("LOuter<TT;>.Inner<L{STRING};>;"), &empty());
    assert_eq!(t, Ok(ca("Outer$Inner", vec![c(STRING)])));
}

#[test]
fn construction_from_signature_is_interface_flag() {
    let f = Fixture::new(vec![class("p/Iface").iface()]);
    assert!(JvmType::from_signature("Lp/Iface;", &f.reg).is_ok_and(|t| t.is_interface()));
    assert!(JvmType::from_signature("Lp/Iface;", &empty()).is_ok_and(|t| !t.is_interface()));
}

#[test]
fn construction_class_of() {
    let f = fixture();
    let t = JvmType::class_of(LIST, &f.reg);
    assert!(t.is_interface());
    assert_eq!(t, JvmType::class_with(LIST, vec![], true));
    assert_eq!(JvmType::class_of("p/Unknown", &f.reg), c("p/Unknown"));
    // strict_erased_subtype 附带覆盖：不自反、根类不作目标
    assert!(strict_erased_subtype(&c(ALIST), &c(LIST), &f.reg));
    assert!(!strict_erased_subtype(&c(ALIST), &c(ALIST), &f.reg));
    assert!(!strict_erased_subtype(&c(ALIST), &c(OBJECT), &f.reg));
}

// ── DisplayAndHashTests ──

/// Python 的 frozen 断言（赋值抛异常）由 Rust 所有权保证，不再运行期检查
#[test]
fn display_frozen_and_hashable() {
    let a = JvmType::class_with(LIST, vec![c(STRING)], true);
    let b = JvmType::class_with(LIST, vec![c(STRING)], true);
    assert_eq!(a, b);
    let set: HashSet<JvmType> = [a.clone(), b.clone(), c(LIST)].into_iter().collect();
    assert_eq!(set.len(), 2);
    let ordered: BTreeSet<JvmType> = [a, b, c(LIST)].into_iter().collect();
    assert_eq!(ordered.len(), 2);
}

#[test]
fn display_to_display() {
    assert_eq!(prim(PrimKind::Int).to_display(), "int");
    let dotted = STRING.replace('/', ".");
    assert_eq!(
        ca(ALIST, vec![c(STRING)]).to_display(),
        format!("p.ArrayList<{dotted}>")
    );
    assert_eq!(arr(prim(PrimKind::Int)).to_display(), "int[]");
    assert_eq!(tvb("T", c(NUMBER)).to_display(), "T extends p.Number");
    assert_eq!(
        wc(WildKind::Extends, Some(c(STRING))).to_display(),
        format!("? extends {dotted}")
    );
    assert_eq!(wc(WildKind::Super, None).to_display(), "? super Object");
    assert_eq!(JvmType::Null.to_display(), "null");
}

// ── FromRustTypeTests（文本解析 from_rust_type → 结构桥 from_rs_type）──

fn rs_class(b: &str, args: Vec<RsType>) -> RsType {
    RsType::class(b, args)
}

#[test]
fn from_rs_primitives_and_unit() {
    let f = fixture();
    let x = f.ctx();
    let none = BTreeSet::new();
    assert_eq!(
        x.from_rs_type(&RsType::Prim(Prim::I32), &none),
        prim(PrimKind::Int)
    );
    assert_eq!(
        x.from_rs_type(&RsType::Prim(Prim::I64), &none),
        prim(PrimKind::Long)
    );
    assert_eq!(
        x.from_rs_type(&RsType::Prim(Prim::Bool), &none),
        prim(PrimKind::Boolean)
    );
    assert_eq!(
        x.from_rs_type(&RsType::Prim(Prim::U16), &none),
        prim(PrimKind::Char)
    );
    assert_eq!(x.from_rs_type(&RsType::Unit, &none), prim(PrimKind::Void));
}

#[test]
fn from_rs_object_and_registry_resolution() {
    let f = fixture();
    let x = f.ctx();
    let none = BTreeSet::new();
    assert_eq!(x.from_rs_type(&RsType::Object, &none), c(OBJECT));
    assert_eq!(x.from_rs_type(&rs_class(ALIST, vec![]), &none), c(ALIST));
    assert_eq!(
        x.from_rs_type(&rs_class(LIST, vec![]), &none),
        JvmType::class_with(LIST, vec![], true)
    );
    assert_eq!(
        x.from_rs_type(&rs_class(ALIST, vec![rs_class(STRING, vec![])]), &none),
        ca(ALIST, vec![c(STRING)])
    );
    // 域外占位：反查不到的名字，binary = 名字本身
    assert_eq!(
        x.from_rs_type(&RsType::Param("NotRegistered".into()), &none),
        c("NotRegistered")
    );
    // 空注册表：一切短名均为占位（Object 例外：常量身份）
    let e = Fixture::new(vec![]);
    assert_eq!(
        e.ctx()
            .from_rs_type(&RsType::Param("ArrayList".into()), &none),
        c("ArrayList")
    );
}

#[test]
fn from_rs_array_and_placeholder_args() {
    let f = fixture();
    let x = f.ctx();
    let none = BTreeSet::new();
    assert_eq!(
        x.from_rs_type(&RsType::array(RsType::Prim(Prim::I32)), &none),
        arr(prim(PrimKind::Int))
    );
    assert_eq!(
        x.from_rs_type(&RsType::array(RsType::array(RsType::Object)), &none),
        arr(arr(c(OBJECT)))
    );
    // 通配符 `?` 在 RsType 中无对应节点（发射侧不产出），此处只验证占位实参
    let t = rs_class(
        "Parent",
        vec![RsType::Param("A".into()), RsType::Prim(Prim::I32)],
    );
    assert_eq!(
        x.from_rs_type(&t, &none),
        ca("Parent", vec![c("A"), prim(PrimKind::Int)])
    );
}

/// Python 的畸形文本降级用例在结构化输入下不存在；改测形参名优先于短名反查
#[test]
fn from_rs_type_params_shadow_short_names() {
    let f = fixture();
    let x = f.ctx();
    let tps: BTreeSet<String> = ["K".to_string(), "ArrayList".to_string()]
        .into_iter()
        .collect();
    assert_eq!(x.from_rs_type(&RsType::Param("K".into()), &tps), tv("K"));
    assert_eq!(
        x.from_rs_type(&rs_class(ALIST, vec![]), &tps),
        tv("ArrayList")
    );
    assert_eq!(
        x.from_rs_type(&rs_class(ALIST, vec![RsType::Object]), &tps),
        ca(ALIST, vec![c(OBJECT)])
    );
}

#[test]
fn from_rs_rust_head_name_roundtrip() {
    let f = fixture();
    let x = f.ctx();
    let none = BTreeSet::new();
    let cases = vec![
        rs_class(ALIST, vec![]),
        rs_class(ALIST, vec![rs_class(STRING, vec![])]),
        rs_class(LIST, vec![RsType::Object]),
        RsType::array(RsType::array(RsType::Object)),
        RsType::Object,
        RsType::Param("K".into()),
        RsType::Prim(Prim::I32),
        RsType::Unit,
        rs_class(
            "p/HashMap$Node",
            vec![RsType::Param("K".into()), RsType::Param("V".into())],
        ),
    ];
    for t in cases {
        let text = t.render(&f.names);
        let head = text.split('<').next().unwrap_or("").trim().to_string();
        assert_eq!(x.rust_head_name(&x.from_rs_type(&t, &none)), head, "{text}");
    }
    assert_eq!(x.rust_head_name(&wc(WildKind::Unbounded, None)), "?");
    assert_eq!(x.rust_head_name(&JvmType::Null), "");
    assert_eq!(x.rust_head_name(&prim(PrimKind::Void)), "()");
}

#[test]
fn from_rs_is_interface_via_parse() {
    let f = fixture();
    let x = f.ctx();
    let none = BTreeSet::new();
    let s = || vec![rs_class(STRING, vec![])];
    assert!(x.from_rs_type(&rs_class(LIST, s()), &none).is_interface());
    assert!(!x.from_rs_type(&rs_class(ALIST, s()), &none).is_interface());
    assert!(!x
        .from_rs_type(&RsType::Param("NotRegistered".into()), &none)
        .is_interface());
}
