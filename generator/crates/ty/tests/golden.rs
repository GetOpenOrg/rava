//! 类型层 golden 对照：`scripts/golden/dump_ty.py` 采集的 Python 输出 vs 本 crate。
//!
//! 按 meta 行重建与 Python 发射层相同的注册表（类路径：用户类目录 → JDK jmods →
//! 镜像 / VM 支持类目录；按 Python registry 插入序装入），逐条记录调用 Rust API，
//! 输出编码为与 Python 相同形态的 JSON 后比较。golden 文件缺失时跳过并提示采集命令。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use resolve::classpath::{ClassPath, Origin};
use serde_json::{json, Value};
use ty::ident::safe_ident;
use ty::jvm_type::subtype::{strict_erased_subtype, super_closure};
use ty::registry::ClassInfo;
use ty::rs_type::render_arg_list;
use ty::sig_types::is_anonymous_class;
use ty::type_map::mangle_name;
use ty::{JvmType, Manifest, Registry, RsType, ShortNames, TyCtx, WildKind};

const TESTS: [(&str, &str); 2] = [
    ("TestHashMapOps", "tests/e2e/33_maps/TestHashMapOps.java"),
    (
        "TestStreamBasic",
        "tests/e2e/30_streams/TestStreamBasic.java",
    ),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

struct Env {
    reg: Registry,
    names: ShortNames,
    manifest: Manifest,
}

fn build_env(meta: &Value) -> Env {
    let s = |k: &str| meta[k].as_str().unwrap_or_default().to_string();
    let mut cp = ClassPath::new();
    cp.add(Origin::User, Path::new(&s("user_dir")))
        .expect("用户类目录");
    cp.add_jdk(Path::new(&s("java_home"))).expect("JDK jmods");
    for d in meta["image_dirs"].as_array().into_iter().flatten() {
        cp.add(Origin::Image, Path::new(d.as_str().unwrap_or_default()))
            .expect("镜像类目录");
    }
    let mut reg = Registry::new();
    let mut missing = Vec::new();
    for n in meta["registry"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        match cp.get(n) {
            Some(cf) => {
                reg.insert(cf);
            }
            None => missing.push(n.to_string()),
        }
    }
    assert!(missing.is_empty(), "类路径缺类：{missing:?}");
    let manifest = Manifest::load(Path::new(&s("runtime"))).expect("runtime 清单");
    let names = ShortNames::build(&reg);
    Env {
        reg,
        names,
        manifest,
    }
}

// ── 输出编码（与 dump_ty.py 同形）──

fn jt(t: &JvmType) -> Value {
    match t {
        JvmType::Primitive(k) => json!({"p": k.java_name()}),
        JvmType::Class {
            binary,
            args,
            is_interface,
        } => {
            json!({"c": binary, "a": args.iter().map(jt).collect::<Vec<_>>(), "i": is_interface})
        }
        JvmType::Array(e) => json!({"arr": jt(e)}),
        JvmType::TypeVar { name, bound } => json!({"tv": name, "b": bound.as_deref().map(jt)}),
        JvmType::Wildcard { kind, bound } => {
            let k = match kind {
                WildKind::Unbounded => "*",
                WildKind::Extends => "+",
                WildKind::Super => "-",
            };
            json!({"w": k, "b": bound.as_deref().map(jt)})
        }
        JvmType::Null => json!({"null": true}),
        JvmType::HostPrim(h) => json!({"h": h.rust_name()}),
    }
}

struct Ctx<'a> {
    x: TyCtx<'a>,
}

impl<'a> Ctx<'a> {
    fn r(&self, t: &RsType) -> Value {
        Value::String(t.render(self.x.names))
    }
    fn ro(&self, t: Option<&RsType>) -> Value {
        Value::String(t.map(|t| t.render(self.x.names)).unwrap_or_default())
    }
    fn rl(&self, ts: &[RsType]) -> Value {
        Value::Array(ts.iter().map(|t| self.r(t)).collect())
    }
    fn pairs(&self, v: &[(String, Vec<RsType>)]) -> Value {
        Value::Array(v.iter().map(|(b, a)| json!([b, self.rl(a)])).collect())
    }
    fn class(&self, inp: &Value) -> &'a ClassInfo {
        let c = inp["c"].as_str().unwrap_or_default();
        self.x
            .reg
            .get(c)
            .unwrap_or_else(|| panic!("注册表无类 {c}"))
    }
    fn tps(&self, ci: &ClassInfo) -> Vec<String> {
        self.x.effective_class_type_params(ci).to_vec()
    }
    fn bounds(&self, sig: &str, tps: &[String]) -> Value {
        let mut bin = BTreeMap::new();
        let b = self
            .x
            .extract_method_tparam_bounds(sig, tps, Some(&mut bin));
        let b: BTreeMap<_, _> = b.iter().map(|(k, v)| (k.clone(), self.r(v))).collect();
        json!({"bounds": b, "bin": bin})
    }

    fn eval(&self, f: &str, inp: &Value) -> Value {
        let st = |k: &str| inp[k].as_str().unwrap_or_default().to_string();
        match f {
            "short" => json!(self.x.names.short(&st("c"))),
            "jvm_to_rust" => self.r(&self.x.jvm_to_rust(&st("d"))),
            "from_descriptor" => {
                JvmType::from_descriptor(&st("d")).map_or(json!({"err": "ValueError"}), |t| jt(&t))
            }
            "from_signature" => JvmType::from_signature(&st("s"), self.x.reg)
                .map_or(json!({"err": "ValueError"}), |t| jt(&t)),
            "subtype" => self.subtype(&st("s"), &st("d")),
            "infer_type_args" => {
                let tps = str_list(&inp["tps"]);
                match self
                    .x
                    .infer_type_args_from_declared(&st("a"), &st("s"), &tps)
                {
                    Some(v) => self.rl(&v),
                    None => Value::Null,
                }
            }
            _ if inp.get("n").is_some() => self.eval_member(f, inp),
            _ => self.eval_class(f, self.class(inp)),
        }
    }

    fn subtype(&self, sig: &str, desc: &str) -> Value {
        let (Ok(a), Ok(e)) = (
            JvmType::from_signature(sig, self.x.reg),
            JvmType::from_descriptor(desc),
        ) else {
            return json!({"err": "ValueError"});
        };
        json!([
            a.is_subtype_of(&e, self.x.reg),
            strict_erased_subtype(&a, &e, self.x.reg),
            jt(&a.erasure()),
            a.to_display()
        ])
    }

    fn eval_class(&self, f: &str, ci: &ClassInfo) -> Value {
        let x = &self.x;
        let c = ci.name();
        match f {
            "effective_params" => json!(self.tps(ci)),
            "outer_instance_class" => json!(ty::class_params::outer_instance_class(ci)),
            "is_anonymous" => json!(is_anonymous_class(ci)),
            "carrier_type" => x.carrier_type(c).map_or(Value::Null, |t| self.r(&t)),
            "superclass_type_args" => self.rl(&x.superclass_type_args(ci)),
            "superinterface_type_args" => self.pairs(&x.superinterface_type_args(ci)),
            "ancestor_type_args" => self.pairs(&x.ancestor_type_args(ci, None)),
            "ancestor_vtable_args_by_short" => {
                let recv = x.jvm_to_rust(&format!("L{c};"));
                let m: BTreeMap<_, _> = x
                    .ancestor_vtable_args_by_short(ci, &recv)
                    .iter()
                    .map(|(k, v)| (k.clone(), render_arg_list(v, x.names)))
                    .collect();
                json!(m)
            }
            "implemented_interface_views" => self.pairs(&x.implemented_interface_views(ci)),
            "supertype_signature_args" => json!(ty::type_args::supertype_signature_args(ci)),
            "interface_signature_views" => json!(x.interface_signature_views(ci)),
            "class_type_param_bounds" => {
                let m: BTreeMap<_, _> = x
                    .class_type_param_bounds(ci)
                    .into_iter()
                    .map(|(k, (t, b))| (k, json!([self.r(&t), b])))
                    .collect();
                json!(m)
            }
            "enclosing_scope_type_args" => self.rl(&x.enclosing_scope_type_args(ci, &[])),
            "overloaded_names" => {
                json!(x.hierarchy_overloaded_names(ci).iter().collect::<Vec<_>>())
            }
            "super_closure" => json!(super_closure(c, x.reg).iter().collect::<Vec<_>>()),
            "class_of" => jt(&JvmType::class_of(c, x.reg)),
            "class_bounds_bin" => self.bounds(ci.generic_signature(), &self.tps(ci)),
            _ => panic!("未知记录类型 {f}"),
        }
    }

    fn eval_member(&self, f: &str, inp: &Value) -> Value {
        let ci = self.class(inp);
        let (n, d) = (
            inp["n"].as_str().unwrap_or_default(),
            inp["d"].as_str().unwrap_or_default(),
        );
        if let Some(m) = ci.methods().iter().find(|m| m.name == n && m.desc == d) {
            if let Some(v) = self.eval_method(f, ci, m) {
                return v;
            }
        }
        let fld = ci.fields().iter().find(|x| x.name == n && x.desc == d);
        let fld = fld.unwrap_or_else(|| panic!("{}.{n}:{d} 不存在", ci.name()));
        self.eval_field(f, ci, fld)
    }

    fn eval_method(&self, f: &str, ci: &ClassInfo, m: &classfile::Method) -> Option<Value> {
        let x = &self.x;
        let tps = self.tps(ci);
        Some(match f {
            "mangle" => json!(mangle_name(x.manifest, &m.name, &m.desc)),
            "method_sig_types" => {
                let s = x.method_sig_types(ci, m, &tps);
                json!([self.rl(&s.params), self.ro(s.ret.as_ref())])
            }
            "emitted_method_sig_types" => {
                let s = x.emitted_method_sig_types(ci, m, &tps);
                json!([self.rl(&s.params), self.r(&s.ret)])
            }
            "receiver_member_name" => json!(x.receiver_member_name(&m.name, &m.desc, ci)),
            "interface_member_local_name" => {
                json!(x.interface_member_local_name(ci, &m.name, &m.desc))
            }
            "method_name_is_mangled" => json!(x.method_name_is_mangled(ci, &m.name)),
            "parse_method_param_types" => {
                let sig = ty::registry::method_signature(m);
                match x.parse_method_param_types(sig, &tps, m.is_static()) {
                    Some(s) => json!([self.rl(&s.params), self.r(&s.ret)]),
                    None => json!([[], ""]),
                }
            }
            "method_bounds" => self.bounds(ty::registry::method_signature(m), &[]),
            _ => return None,
        })
    }

    fn eval_field(&self, f: &str, ci: &ClassInfo, fld: &classfile::Field) -> Value {
        let x = &self.x;
        let tps = self.tps(ci);
        let sig = ty::registry::field_signature(fld);
        match f {
            "instance_field_rust_name" => {
                json!(x.instance_field_rust_name(ci.name(), &safe_ident(&fld.name)))
            }
            "outer_ref_field_type" => {
                self.ro(x.outer_ref_field_type(&fld.name, &fld.desc, &tps).as_ref())
            }
            "jvm_to_rs_type" => self.r(&x.jvm_to_rs_type(&fld.desc, sig, &tps)),
            "parse_field_type" => self.ro(x.parse_field_type(sig, &tps).as_ref()),
            "from_rust_type" => {
                let set: BTreeSet<String> = tps.iter().cloned().collect();
                let t = match x.parse_field_type(sig, &tps) {
                    Some(rs) => x.from_rs_type(&rs, &set),
                    None => JvmType::Null,
                };
                json!({"t": jt(&t), "head": x.rust_head_name(&t)})
            }
            _ => panic!("未知记录类型 {f}"),
        }
    }
}

fn str_list(v: &Value) -> Vec<String> {
    v.as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// 单个 golden 文件的对照结果：(总条数, 失配 [(函数, 输入, 期望, 实际)])
fn run_golden(path: &Path) -> (usize, Vec<(String, Value, Value, Value)>) {
    let text = std::fs::read_to_string(path).expect("读 golden");
    let mut lines = text.lines();
    let meta: Value = serde_json::from_str(lines.next().unwrap_or("{}")).expect("meta 行");
    let env = build_env(&meta);
    let pd: Vec<String> = env.names.prelude_disambiguated().to_vec();
    assert_eq!(json!(pd), meta["prelude_disambiguated"], "prelude 限定集");
    let ctx = Ctx {
        x: TyCtx::new(&env.reg, &env.names, &env.manifest),
    };
    let mut total = 0;
    let mut diffs = Vec::new();
    for line in lines {
        let rec: Value = serde_json::from_str(line).expect("记录行");
        let f = rec["f"].as_str().unwrap_or_default();
        let got = ctx.eval(f, &rec["in"]);
        total += 1;
        if got != rec["out"] {
            diffs.push((f.to_string(), rec["in"].clone(), rec["out"].clone(), got));
        }
    }
    (total, diffs)
}

#[test]
fn golden_ty() {
    let root = repo_root();
    let mut any = false;
    let mut failed = Vec::new();
    for (stem, java) in TESTS {
        let path = root.join("build/golden/ty").join(format!("{stem}.jsonl"));
        if !path.exists() {
            eprintln!("跳过 {stem}：缺 golden，先运行 python3 scripts/golden/dump_ty.py {java}");
            continue;
        }
        any = true;
        let (total, diffs) = run_golden(&path);
        let mut by_f: BTreeMap<&str, usize> = BTreeMap::new();
        for d in &diffs {
            *by_f.entry(d.0.as_str()).or_default() += 1;
        }
        eprintln!("{stem}: {total} 条，失配 {}（{by_f:?}）", diffs.len());
        for (f, inp, want, got) in diffs.iter().take(40) {
            eprintln!("  {f} {inp}\n    py: {want}\n    rs: {got}");
        }
        if !diffs.is_empty() {
            failed.push(stem);
        }
    }
    if !any {
        eprintln!("golden_ty：无 golden 文件，已跳过");
    }
    assert!(failed.is_empty(), "golden 失配：{failed:?}");
}
