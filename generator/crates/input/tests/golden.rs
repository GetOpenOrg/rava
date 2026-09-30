//! 发射层输入 golden 对照：`scripts/golden/dump_input.py` 采集的 Python 事实 vs 本 crate。
//!
//! 按 meta 行以同一类路径（用户类目录 → JDK jmods → 镜像类目录）、同一 closure.json 与
//! runtime 清单重建 [`EmitInput`]，逐条记录编码为与 Python 相同形态的 JSON 后比较。
//! golden 文件缺失时跳过并提示采集命令。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use classfile::{Const, Operand};
use input::norm::switch_targets;
use input::{BuildInput, ClassPlan, ClosureFacts, EmitInput, NInsn, NormCode, Planner, Role, RuntimeManifest, Verdict};
use resolve::classpath::{ClassPath, Origin};
use serde_json::{json, Value};
use ty::ShortNames;

const TESTS: [(&str, &str); 3] = [
    ("TestHashMapOps", "tests/e2e/33_maps/TestHashMapOps.java"),
    ("TestStreamBasic", "tests/e2e/30_streams/TestStreamBasic.java"),
    ("TestCompletableFuture", "tests/e2e/54_concurrency_api/TestCompletableFuture.java"),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn str_list(v: &Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn class_path(meta: &Value) -> ClassPath {
    let s = |k: &str| meta[k].as_str().unwrap_or_default().to_string();
    let mut cp = ClassPath::new();
    cp.add(Origin::User, Path::new(&s("user_dir"))).expect("用户类目录");
    cp.add_jdk(Path::new(&s("java_home"))).expect("JDK jmods");
    for d in str_list(&meta["image_dirs"]) {
        cp.add(Origin::Image, Path::new(&d)).expect("镜像类目录");
    }
    cp
}

// ── 编码（与 dump_input.py 同形）──

fn const_proj(c: &Const) -> String {
    match c {
        Const::String(s) => format!("String {s}"),
        Const::StringUtf16(u) => format!("String {}", std::string::String::from_utf16_lossy(u)),
        Const::Int(i) => format!("int {i}"),
        Const::Long(l) => format!("long {l}"),
        Const::Class(c) => format!("class {c}"),
        Const::Float(_) => "float".into(),
        Const::Double(_) => "double".into(),
        _ => "other".into(),
    }
}

fn insn_proj(i: &classfile::Insn) -> String {
    let mut s = format!("{} {}", i.offset, i.name());
    match &i.operand {
        Operand::Branch(t) => s += &format!(" ->{t}"),
        o @ (Operand::TableSwitch { .. } | Operand::LookupSwitch { .. }) => {
            let t: Vec<String> = switch_targets(o).iter().map(u32::to_string).collect();
            s += &format!(" ->{}", t.join(","));
        }
        Operand::Ldc(c) => s += &format!(" ={}", const_proj(c)),
        _ => {}
    }
    s
}

fn tail(load: &classfile::Insn) -> String {
    let inner = insn_proj(load);
    inner.split_once(' ').map_or(String::new(), |(_, t)| t.to_string())
}

fn ninsn_proj(n: &NInsn) -> String {
    match n {
        NInsn::Op(i) => insn_proj(i),
        NInsn::FoldField { offset, load } => format!("{offset} fold_field {}", tail(load)),
        NInsn::FoldCall { call, load } => format!("{} fold_call {}", insn_proj(call), tail(load)),
    }
}

fn code_json(n: &NormCode) -> Value {
    let et: Vec<Value> = n
        .exception_table
        .iter()
        .map(|e| json!([e.start, e.end, e.handler, e.catch_type.clone().unwrap_or_default()]))
        .collect();
    json!({"insns": n.insns.iter().map(ninsn_proj).collect::<Vec<_>>(), "et": et})
}

fn role_str(r: Role) -> &'static str {
    match r {
        Role::Clinit => "clinit",
        Role::IfaceLambda => "iface_lambda",
        Role::IfacePrivate => "iface_private",
        Role::Member => "member",
    }
}

fn verdict_str(v: Verdict) -> &'static str {
    match v {
        Verdict::Bytecode => "bytecode",
        Verdict::StubNotInChain => "stub_not_in_chain",
        Verdict::StubNative => "stub_native",
        Verdict::StubAbstract => "stub_abstract",
        Verdict::Handwritten => "handwritten",
        Verdict::HandwrittenBody => "handwritten_body",
        Verdict::Core => "core",
        Verdict::IfaceDefaultBody => "iface_default_body",
        Verdict::IfaceDecl => "iface_decl",
    }
}

fn plan_json(p: &ClassPlan) -> Value {
    let methods: Vec<Value> = p
        .methods
        .iter()
        .map(|m| {
            json!({"n": m.name, "d": m.desc, "rust": m.rust_name, "role": role_str(m.role),
                   "v": verdict_str(m.verdict), "inh": m.inherited_override})
        })
        .collect();
    json!({"c": p.name, "type_only": p.type_only, "methods": methods, "supp": p.iface_supplement})
}

fn fnv(b: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for x in b {
        h = (h ^ u64::from(*x)).wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

fn manifest_json(m: &RuntimeManifest) -> Value {
    let indy: BTreeMap<&String, &str> = m.indy_kinds.iter().map(|(k, v)| (k, v.as_str())).collect();
    json!({
        "boundary_packages": m.boundary_packages,
        "vm_boundary_classes": m.vm_boundary_classes,
        "release": m.release,
        "jca_release": m.jca_release,
        "module_resource_paths": m.module_resource_paths,
        "boot_init_classes": m.boot_init_classes,
        "data_bundle_carriers": m.data_bundle_carriers,
        "intrinsic_members": m.intrinsic_members,
        "caller_sensitive_annotations": m.caller_sensitive_annotations,
        "sigpoly_callsite_typed": m.sigpoly_callsite_typed,
        "indy_kinds": indy,
        "null_returns": m.vm_constants.null_returns,
        "null_to_false": m.vm_constants.null_to_false,
    })
}

fn key_str(k: &(String, String, String)) -> String {
    format!("{}.{}:{}", k.0, k.1, k.2)
}

struct Ctx<'a> {
    emit: &'a EmitInput,
    manifest: &'a RuntimeManifest,
    planner: &'a Planner<'a>,
}

impl Ctx<'_> {
    /// 一条记录的 Rust 侧值（未知记录种类 → None）
    fn eval(&self, r: &Value) -> Option<Value> {
        let e = self.emit;
        let c = r["c"].as_str().unwrap_or_default();
        Some(match r["f"].as_str()? {
            "registry_order" => json!(e.registry.iter_insertion().map(|c| c.name()).collect::<Vec<_>>()),
            "user_classes" => json!(e.user_classes),
            "lib_crates" => json!(e.lib_crates),
            "jdk_classes" => json!(e.jdk_classes),
            "visited" => json!(e.visited.iter().map(key_str).collect::<BTreeSet<_>>()),
            "field_stubs" => json!(e.field_stubs),
            "reflect_consts" => json!(e.reflect.consts),
            "reflect_all" => json!(e.reflect.all_members),
            "reflect_field_names" => json!(e.reflect.field_names),
            "data_bundle_seeds" => json!(e.data_bundle_seeds),
            "annotation_enum_seeds" => json!(e.annotation_enum_seeds),
            "jca_seeds" => json!(e.jca_seeds.iter().map(|s| [&s.ty, &s.algorithm, &s.imp, &s.provider]).collect::<Vec<_>>()),
            "module_resources" => json!(e.module_resources.iter().map(|(p, b)| json!([p, b.len(), fnv(b)])).collect::<Vec<_>>()),
            "precheck_visited" => json!(e.precheck_visited),
            "manifest" => manifest_json(self.manifest),
            "handwritten" => match e.handwritten.get(c) {
                Some(h) => json!({"methods": h.methods, "cores": h.method_cores, "sigs": h.iface_method_sigs}),
                None => Value::Null,
            },
            "handwritten_classes" => json!(e.handwritten.classes.keys().collect::<Vec<_>>()),
            "code" => {
                let k = (c.to_string(), r["n"].as_str()?.to_string(), r["d"].as_str()?.to_string());
                e.normalized().get(&k).map_or(Value::Null, code_json)
            }
            "normalized_keys" => json!(e.normalized().keys().map(key_str).collect::<BTreeSet<_>>()),
            "plan" => self.planner.plan(c).as_ref().map_or(Value::Null, plan_json),
            _ => return None,
        })
    }
}

type Diff = (String, String, Value, Value);

fn run_golden(path: &Path) -> (usize, Vec<Diff>) {
    let text = std::fs::read_to_string(path).expect("读 golden");
    let mut lines = text.lines();
    let meta: Value = serde_json::from_str(lines.next().expect("meta 行")).expect("meta JSON");
    let recs: Vec<Value> = lines.map(|l| serde_json::from_str(l).expect("记录 JSON")).collect();
    let cp = class_path(&meta);
    let cj_text = std::fs::read_to_string(meta["closure_json"].as_str().unwrap_or_default()).expect("closure.json");
    let facts = ClosureFacts::from_json(&serde_json::from_str(&cj_text).expect("closure.json 解析")).expect("闭包事实");
    let runtime = PathBuf::from(meta["runtime"].as_str().unwrap_or_default());
    let manifest = RuntimeManifest::load(&runtime).expect("runtime 清单");
    let user = str_list(&meta["user_classes"]);
    let emit = BuildInput {
        cp: &cp,
        facts: &facts,
        manifest: &manifest,
        user_classes: &user,
        libs: &[],
        runtime_src: &runtime.join("src"),
    }
    .build()
    .expect("构建发射层输入");
    let names = ShortNames::build(&emit.registry);
    let planner = Planner::new(&emit, &names, &manifest, &cp);
    let ctx = Ctx {
        emit: &emit,
        manifest: &manifest,
        planner: &planner,
    };
    let mut recs = recs;
    let hw: Vec<Value> = recs.iter().filter(|r| r["f"] == "handwritten").map(|r| r["c"].clone()).collect();
    recs.push(json!({"f": "handwritten_classes", "v": hw}));
    let mut diffs = Vec::new();
    for r in &recs {
        let f = r["f"].as_str().unwrap_or_default().to_string();
        let got = ctx.eval(r).unwrap_or_else(|| json!({"unknown_record": f}));
        if got != r["v"] {
            let id = format!("{} {} {}", r["c"].as_str().unwrap_or(""), r["n"].as_str().unwrap_or(""), r["d"].as_str().unwrap_or(""));
            diffs.push((f, id.trim().to_string(), r["v"].clone(), got));
        }
    }
    (recs.len(), diffs)
}

/// 集合型记录的差异摘要（只列两侧各自独有的前若干项）
fn brief(want: &Value, got: &Value) -> String {
    match (want.as_array(), got.as_array()) {
        (Some(w), Some(g)) => {
            let ws: BTreeSet<String> = w.iter().map(Value::to_string).collect();
            let gs: BTreeSet<String> = g.iter().map(Value::to_string).collect();
            let only_py: Vec<&String> = ws.difference(&gs).take(8).collect();
            let only_rs: Vec<&String> = gs.difference(&ws).take(8).collect();
            format!("仅 py（{}）：{only_py:?}\n    仅 rs（{}）：{only_rs:?}", ws.difference(&gs).count(), gs.difference(&ws).count())
        }
        _ => format!("py: {want}\n    rs: {got}"),
    }
}

#[test]
fn golden_input() {
    let root = repo_root();
    let mut any = false;
    let mut failed = Vec::new();
    for (stem, java) in TESTS {
        let path = root.join("build/golden/input").join(format!("{stem}.jsonl"));
        if !path.exists() {
            eprintln!("跳过 {stem}：缺 golden，先运行 python3 scripts/golden/dump_input.py {java}");
            continue;
        }
        any = true;
        let (total, diffs) = run_golden(&path);
        let mut by_f: BTreeMap<&str, usize> = BTreeMap::new();
        for d in &diffs {
            *by_f.entry(d.0.as_str()).or_default() += 1;
        }
        eprintln!("{stem}: {total} 条，失配 {}（{by_f:?}）", diffs.len());
        for (f, id, want, got) in diffs.iter().take(40) {
            eprintln!("  {f} {id}\n    {}", brief(want, got));
        }
        if !diffs.is_empty() {
            failed.push(stem);
        }
    }
    if !any {
        eprintln!("golden_input：无 golden 文件，已跳过");
    }
    assert!(failed.is_empty(), "golden 失配：{failed:?}");
}
