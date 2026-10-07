//! 档案并集 / 折叠之并 / 摘要的单元测试（合成 closure.json）。

use serde_json::{json, Value};

use super::*;

fn closure(classes: &[(&str, &str, &str)], methods: &[(&str, &str)], folds: Vec<Value>) -> Value {
    json!({
        "summary": {"elapsed_ms": 1},
        "classes": classes.iter().map(|(n, d, l)| json!({"name": n, "domain": d, "level": l, "via": {"kind": "test", "from": {"root": n}}})).collect::<Vec<_>>(),
        "methods": methods.iter().map(|(id, k)| json!({"id": id, "kind": k, "via": {"kind": "test"}})).collect::<Vec<_>>(),
        "instantiated": classes.iter().filter(|c| c.2 == "alloc" || c.2 == "code").map(|c| c.0).collect::<Vec<_>>(),
        "clinit": classes.iter().map(|c| c.0).collect::<Vec<_>>(),
        "missing": [],
        "unresolved": [],
        "refs": methods.iter().map(|m| m.0).collect::<Vec<_>>(),
        "dispatched": [],
        "hw_inherited": [],
        "folds_version": crate::FOLDS_VERSION,
        "folds": folds,
        "system_properties": {"values": {"file.encoding": "UTF-8"}, "dynamic": ["java.home"]},
        "boot_image_data": {"live": []},
        "reflect": {"members": [], "gaps": [], "fields": [], "field_names": [], "static_fields": [], "field_enum_gaps": []},
        "seeds": {"annotation_enums": [], "mirror_inits": [], "reflect_names": {}, "reflect_all": [], "services": [], "services_unknown": false},
    })
}

fn entry(name: &str, c: Value) -> EntryClosure {
    EntryClosure { name: name.into(), closure: c, cp: Vec::new(), launch: None }
}

fn no_table(_: &str) -> Option<Vec<u32>> {
    Some(vec![])
}

fn names(v: &Value, key: &str, field: &str) -> Vec<String> {
    v[key].as_array().unwrap().iter().map(|x| x[field].as_str().unwrap().to_string()).collect()
}

fn inputs() -> KeyInputs {
    KeyInputs { generator: "g".into(), runtime: "r".into(), jdk_major: 21, archives: "a".into(), entries: "e".into(), deps_lock: String::new() }
}

/// 两个入口有同名用户类 Main（语料常见）：档案只含非用户侧，类层级取最高，方法 / 集合取并
fn pair() -> (EntryClosure, EntryClosure) {
    let a = closure(
        &[("Main", "user", "code"), ("x/P", "translate", "code"), ("x/Q", "translate", "type")],
        &[("Main.main:([Ljava/lang/String;)V", "bytecode"), ("x/P.f:()V", "bytecode")],
        vec![],
    );
    let b = closure(
        &[("Main", "user", "code"), ("x/Q", "translate", "code"), ("x/R", "boundary", "layout")],
        &[("Main.main:([Ljava/lang/String;)V", "bytecode"), ("x/Q.g:()V", "bytecode"), ("x/R.n:()V", "handwritten:native")],
        vec![],
    );
    (entry("TestA", a), entry("TestB", b))
}

#[test]
fn union_of_non_user_sides() {
    let (a, b) = pair();
    let p = merge(&[a, b], &no_table).unwrap();
    assert_eq!(names(&p, "classes", "name"), ["x/P", "x/Q", "x/R"]);
    let q = p["classes"].as_array().unwrap().iter().find(|c| c["name"] == "x/Q").unwrap();
    assert_eq!(q["level"], "code");
    assert_eq!(q["entry"], "TestA");
    assert_eq!(names(&p, "methods", "id"), ["x/P.f:()V", "x/Q.g:()V", "x/R.n:()V"]);
    assert_eq!(p["clinit"], json!(["x/P", "x/Q", "x/R"]));
    assert_eq!(p["refs"], json!(["x/P.f:()V", "x/Q.g:()V", "x/R.n:()V"]));
    // 与 closure.json 同形：发射层的读入口可直接解析（字段齐全）
    for k in ["classes", "methods", "instantiated", "clinit", "missing", "unresolved", "refs", "dispatched", "hw_inherited", "folds"] {
        assert!(p[k].is_array(), "{k}");
    }
    assert!(p["system_properties"]["values"].is_object());
}

#[test]
fn independent_of_entry_order() {
    let (a, b) = pair();
    let (a2, b2) = pair();
    let p1 = merge(&[a, b], &no_table).unwrap();
    let p2 = merge(&[b2, a2], &no_table).unwrap();
    assert_eq!(p1, p2);
}

#[test]
fn key_is_stable_and_order_free() {
    let (a, b) = pair();
    let (a2, b2) = pair();
    let k1 = build(&[a, b], &no_table, &inputs(), None).unwrap();
    let k2 = build(&[b2, a2], &no_table, &inputs(), None).unwrap();
    assert_eq!(k1["profile"]["key"], k2["profile"]["key"]);
    assert_eq!(k1, k2);
}

#[test]
fn key_is_sensitive_to_inputs_and_entries() {
    let key = |es: Vec<EntryClosure>, i: &KeyInputs| build(&es, &no_table, i, None).unwrap()["profile"]["key"].as_str().unwrap().to_string();
    let base = || {
        let (a, b) = pair();
        vec![a, b]
    };
    let k0 = key(base(), &inputs());
    for change in [
        KeyInputs { generator: "g2".into(), ..inputs() },
        KeyInputs { runtime: "r2".into(), ..inputs() },
        KeyInputs { jdk_major: 25, ..inputs() },
        KeyInputs { archives: "a2".into(), ..inputs() },
    ] {
        assert_ne!(key(base(), &change), k0);
    }
    // 入口输入摘要只作记录，不入 P
    assert_eq!(key(base(), &KeyInputs { entries: "e2".into(), ..inputs() }), k0);
    // 换掉一个入口：JDK 侧闭包变化 → 键变化
    let mut es = base();
    es[1] = entry("TestB", closure(&[("Main", "user", "code"), ("x/S", "translate", "code")], &[("x/S.h:()V", "bytecode")], vec![]));
    assert_ne!(key(es, &inputs()), k0);
    // 只改用户侧（JDK 侧闭包不变）→ 键不变（开放世界：档案与用户代码无关）
    let mut es = base();
    let mut c = es[0].closure.clone();
    c["classes"].as_array_mut().unwrap().push(json!({"name": "Helper", "domain": "user", "level": "code", "via": null}));
    c["methods"].as_array_mut().unwrap().push(json!({"id": "Helper.run:()V", "kind": "bytecode", "via": null}));
    c["refs"].as_array_mut().unwrap().push(json!("Helper.run:()V"));
    es[0].closure = c;
    assert_eq!(key(es, &inputs()), k0);
}

#[test]
fn rejects_duplicate_names_and_domain_conflicts() {
    let (a, _) = pair();
    let (a2, _) = pair();
    assert!(merge(&[a, a2], &no_table).unwrap_err().contains("入口名重复"));
    let (a, _) = pair();
    let b = entry("TestB", closure(&[("x/P", "boundary", "code")], &[], vec![]));
    assert!(merge(&[a, b], &no_table).unwrap_err().contains("domain"));
}

fn fold(method: &str, extra: Value) -> Value {
    let mut f = json!({
        "method": method, "dead_pcs": [], "dead_handlers": [], "dead_catches": [], "consts": [],
        "null_recv": [], "noreturn_calls": [], "noreturn_dead_pcs": [],
    });
    for (k, v) in extra.as_object().unwrap() {
        f[k] = v.clone();
    }
    f
}

fn one_method(name: &str, folds: Vec<Value>) -> EntryClosure {
    entry(name, closure(&[("x/P", "translate", "code")], &[("x/P.f:()V", "bytecode")], folds))
}

fn folds_of(p: &Value) -> Vec<Value> {
    p["folds"].as_array().unwrap().clone()
}

#[test]
fn fold_join_is_pointwise_lub() {
    let m = "x/P.f:()V";
    let c7 = json!({"pc": 3, "kind": "invoke", "type": "I", "value": 7});
    let a = one_method("A", vec![fold(m, json!({"consts": [c7], "null_recv": [5], "noreturn_calls": [8], "dead_pcs": [[10, 20]], "dead_handlers": [12]}))]);
    let b = one_method("B", vec![fold(m, json!({"consts": [c7], "null_recv": [8], "noreturn_dead_pcs": [[15, 25]]}))]);
    let table = |id: &str| (id == m).then(|| vec![12, 16, 30]);
    let p = merge(&[a, b], &table).unwrap();
    let f = &folds_of(&p)[0];
    assert_eq!(f["consts"], json!([c7]));
    // 5：A 为 null_recv、B 照常调用 → 不折叠；8：noreturn ⊔ null_recv → noreturn
    assert_eq!(f["null_recv"], json!([]));
    assert_eq!(f["noreturn_calls"], json!([8]));
    // 死区取交；死区内的处理器全部列入；12 在 A 列出、在 B 活 → 不列
    assert_eq!(f["dead_pcs"], json!([[15, 20]]));
    assert_eq!(f["dead_handlers"], json!([16]));
    assert_eq!(f["noreturn_dead_pcs"], json!([]));
}

#[test]
fn fold_join_disagreeing_consts_and_unfolded_entry() {
    let m = "x/P.f:()V";
    let a = one_method("A", vec![fold(m, json!({"consts": [{"pc": 3, "kind": "invoke", "type": "I", "value": 7}]}))]);
    let b = one_method("B", vec![fold(m, json!({"consts": [{"pc": 3, "kind": "invoke", "type": "I", "value": 8}]}))]);
    assert!(folds_of(&merge(&[a, b], &no_table).unwrap()).is_empty());
    // 到达但无折叠记录的入口按空折叠参与
    let a = one_method("A", vec![fold(m, json!({"null_recv": [5]}))]);
    let b = one_method("B", vec![]);
    assert!(folds_of(&merge(&[a, b], &no_table).unwrap()).is_empty());
    // 未到达该方法的入口不削弱折叠
    let a = one_method("A", vec![fold(m, json!({"null_recv": [5]}))]);
    let c = entry("C", closure(&[("x/Z", "translate", "code")], &[("x/Z.z:()V", "bytecode")], vec![]));
    assert_eq!(folds_of(&merge(&[a, c], &no_table).unwrap())[0]["null_recv"], json!([5]));
}

#[test]
fn fold_join_dead_catches() {
    let m = "x/P.f:()V";
    let catch = json!({"start": 0, "end": 4, "handler": 40, "catch_type": "x/E"});
    // A 列为死 catch；B 里保护区间整段不可达 → 并后仍为死 catch
    let a = one_method("A", vec![fold(m, json!({"dead_catches": [catch]}))]);
    let b = one_method("B", vec![fold(m, json!({"dead_pcs": [[0, 6]], "dead_handlers": []}))]);
    let p = merge(&[a, b], &no_table).unwrap();
    assert_eq!(folds_of(&p)[0]["dead_catches"], json!([catch]));
    // C 里该表项照常生效 → 不再是死 catch
    let a = one_method("A", vec![fold(m, json!({"dead_catches": [catch]}))]);
    let c = one_method("C", vec![]);
    assert!(folds_of(&merge(&[a, c], &no_table).unwrap()).is_empty());
}

#[test]
fn fold_join_direct_calls() {
    let m = "x/P.f:()V";
    let d = |t: &str| json!({"direct_calls": [{"pc": 4, "target": t}]});
    // 两入口同为直连（同一特化入口）→ 直连
    let p = merge(&[one_method("A", vec![fold(m, d("x/H.h:(Lx/M;)V"))]), one_method("B", vec![fold(m, d("x/H.h:(Lx/M;)V"))])], &no_table).unwrap();
    assert_eq!(folds_of(&p)[0]["direct_calls"], json!([{"pc": 4, "target": "x/H.h:(Lx/M;)V"}]));
    // 直连 ⊔ null_recv → 直连（null_recv 的入口不给原入口入链）
    let p = merge(&[one_method("A", vec![fold(m, d("x/H.h:(Lx/M;)V"))]), one_method("B", vec![fold(m, json!({"null_recv": [4]}))])], &no_table).unwrap();
    assert_eq!(folds_of(&p)[0]["direct_calls"], json!([{"pc": 4, "target": "x/H.h:(Lx/M;)V"}]));
    assert_eq!(folds_of(&p)[0]["null_recv"], json!([]));
    // 直连 ⊔ 照常调用（含无记录的到达入口）→ 不折叠
    let p = merge(&[one_method("A", vec![fold(m, d("x/H.h:(Lx/M;)V"))]), one_method("B", vec![])], &no_table).unwrap();
    assert!(folds_of(&p).is_empty());
    // 该点在另一入口不可达：不削弱
    let p = merge(&[one_method("A", vec![fold(m, d("x/H.h:(Lx/M;)V"))]), one_method("B", vec![fold(m, json!({"dead_pcs": [[2, 8]]}))])], &no_table).unwrap();
    assert_eq!(folds_of(&p)[0]["direct_calls"], json!([{"pc": 4, "target": "x/H.h:(Lx/M;)V"}]));
}

#[test]
fn coverage_and_idempotence() {
    let m = "x/F.f:()V";
    let f_entry = |name: &str, folds: Vec<Value>| entry(name, closure(&[("x/F", "translate", "code")], &[(m, "bytecode")], folds));
    let table = |_: &str| Some(vec![14]);
    let (a, b) = pair();
    let a_copy = entry("TestA", a.closure.clone());
    let p = merge(&[a, b, f_entry("F", vec![fold(m, json!({"dead_pcs": [[10, 20]]}))])], &table).unwrap();
    assert_eq!(folds_of(&p)[0]["dead_handlers"], json!([14]));
    assert!(covers(&p, &a_copy, &table).unwrap());
    // 档案与自身之并不变
    let again = merge(&[entry("P", p.clone())], &table).unwrap();
    assert_eq!(content_digest(&again), content_digest(&p));
    // 新类 / 削弱折叠的入口不被覆盖
    let n = entry("N", closure(&[("x/New", "translate", "code")], &[], vec![]));
    assert!(!covers(&p, &n, &no_table).unwrap());
    assert!(!covers(&p, &f_entry("G", vec![]), &table).unwrap());
    assert!(covers(&p, &f_entry("H", vec![fold(m, json!({"dead_pcs": [[8, 22]]}))]), &table).unwrap());
}

#[test]
fn entries_digest_is_order_free() {
    let x = vec![("A".to_string(), "1".to_string()), ("B".to_string(), "2".to_string())];
    let y = vec![x[1].clone(), x[0].clone()];
    assert_eq!(entries_digest(&x), entries_digest(&y));
    assert_ne!(entries_digest(&x), entries_digest(&[x[0].clone(), ("B".into(), "3".into())]));
}
