//! 方法体生成 golden 对照：`scripts/golden/dump_method.py` 截获的 Python `gen_method_body`
//! 输入视图与输出文本 vs 本 crate 的 [`method::gen_method_body`]，逐条逐字节比较。
//!
//! 环境按 meta 行以 [`BuildInput`] 重建（类路径：用户类目录 → JDK jmods → 镜像类目录；
//! 同一 closure.json 与 runtime 清单）。每条记录的方法视图取自记录（名字 / 描述符 / 访问
//! 标志 / 适配后的泛型签名与局部变量表），字节码取自出处类（`owner`）的规范化指令；
//! SAM 合成对象路径按记录回答（invokedynamic 常量池下标由指令操作数携带）。
//! 失配明细写入 `build/golden/method/<Test>.diff.txt`；golden 文件缺失时跳过。

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use classfile::extras::LocalVar;
use classfile::Method;
use input::{BuildInput, ClosureFacts, EmitInput, RuntimeManifest};
use instr::{Effect, InstrCtx, InstrEnv, InstrFacts, InstrHooks};
use ir::{Ident, Path as IrPath, PathSegment};
use method::{gen_method_body, MethodRequest, MethodSink};
use resolve::classpath::{ClassPath, Origin};
use serde_json::Value;
use ty::{Manifest, ShortNames, TyCtx};

const TESTS: [(&str, &str); 3] = [
    ("TestHashMapOps", "tests/e2e/33_maps/TestHashMapOps.java"),
    ("TestStreamBasic", "tests/e2e/30_streams/TestStreamBasic.java"),
    ("TestCompletableFuture", "tests/e2e/54_concurrency_api/TestCompletableFuture.java"),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// 记录的用户类目录不存在时换成 snake_case 的 scratch 目录名
fn user_dir(recorded: &str) -> PathBuf {
    let p = PathBuf::from(recorded);
    if p.exists() {
        return p;
    }
    let parts: Vec<_> = p.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect();
    let Some(i) = parts.iter().rposition(|c| c == "closure_input").and_then(|i| i.checked_sub(1)) else {
        return p;
    };
    parts.iter().enumerate().map(|(k, c)| if k == i { instr::text::to_snake(c) } else { c.clone() }).collect()
}

fn str_list(v: &Value) -> Vec<String> {
    v.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect()
}

struct Env {
    emit: EmitInput,
    names: ShortNames,
    manifest: Manifest,
    rt: RuntimeManifest,
    facts: InstrFacts,
}

fn build_env(meta: &Value) -> Env {
    let s = |k: &str| meta[k].as_str().unwrap_or_default().to_string();
    let mut cp = ClassPath::new();
    cp.add(Origin::User, &user_dir(&s("user_dir"))).expect("用户类目录");
    cp.add_jdk(Path::new(&s("java_home"))).expect("JDK jmods");
    for d in str_list(&meta["image_dirs"]) {
        cp.add(Origin::Image, Path::new(&d)).expect("镜像类目录");
    }
    let cj = std::fs::read_to_string(s("closure_json")).expect("closure.json");
    let cfacts = ClosureFacts::from_json(&serde_json::from_str(&cj).expect("closure.json 解析")).expect("闭包事实");
    let runtime = PathBuf::from(s("runtime"));
    let rt = RuntimeManifest::load(&runtime).expect("runtime 清单");
    let user = str_list(&meta["user_classes"]);
    let emit = BuildInput {
        cp: &cp,
        facts: &cfacts,
        manifest: &rt,
        user_classes: &user,
        libs: &[],
        runtime_src: &runtime.join("src"),
        jobs: 0,
    }
    .build()
    .expect("构建发射层输入");
    let names = ShortNames::build(&emit.registry);
    let manifest = Manifest::load(&runtime).expect("ty 清单");
    let root = cp.get(ty::consts::OBJECT);
    let facts = InstrFacts::build(&emit.registry, root.as_deref(), &runtime.join("src"));
    Env { emit, names, manifest, rt, facts }
}

/// 按记录回答 SAM 合成对象路径
struct GoldenHooks {
    sam: Vec<(String, String, Option<String>)>,
}

impl InstrHooks for GoldenHooks {
    fn sam_ctor_path(&self, iface: &str, current_class: &str) -> Option<IrPath> {
        let (_, _, out) = self.sam.iter().find(|(i, c, _)| i == iface && c == current_class)?;
        let segs = out.as_deref()?.split("::").map(|s| Ident::new(s).ok().map(PathSegment::new)).collect::<Option<Vec<_>>>()?;
        Some(IrPath::new(segs))
    }
}

fn hooks_of(rec: &Value) -> GoldenHooks {
    let sam = rec["sam"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_array)
        .map(|f| {
            let a = |i: usize| f.get(i).and_then(Value::as_str).unwrap_or("").to_string();
            (a(0), a(1), f.last().and_then(Value::as_str).map(str::to_string))
        })
        .collect();
    GoldenHooks { sam }
}

fn local_vars(rec: &Value) -> Vec<LocalVar> {
    rec["lv"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| LocalVar {
            slot: e[0].as_u64().unwrap_or(0) as u16,
            start: e[1].as_u64().unwrap_or(0) as u16,
            len: e[2].as_u64().unwrap_or(0) as u16,
            name: e[3].as_str().unwrap_or("").to_string(),
            desc: e[4].as_str().unwrap_or("").to_string(),
            signature: e[5].as_str().unwrap_or("").to_string(),
        })
        .collect()
}

/// Python 外部登记 → 行（`sam_ctor` 是查询，不参与比对；audit 按 count 展开）
fn fx_py(rec: &Value) -> Vec<String> {
    let mut out = Vec::new();
    for f in rec["fx"].as_array().into_iter().flatten() {
        let a: Vec<String> =
            f.as_array().into_iter().flatten().map(|x| x.as_str().map_or_else(|| x.to_string(), str::to_string)).collect();
        match a.first().map(String::as_str) {
            Some("audit") => {
                let n = a.get(2).and_then(|c| c.parse::<usize>().ok()).unwrap_or(1);
                out.extend(std::iter::repeat_n(format!("audit {}", a.get(1).map_or("", String::as_str)), n));
            }
            Some(name) => out.push(format!("{name} {}", a[1..].join(" | "))),
            None => {}
        }
    }
    out
}

fn fx_rs(e: &Effect) -> String {
    match e {
        Effect::Audit(a) => format!("audit {}", a.as_str()),
        Effect::InstanceofFold => "instanceof_fold ".to_string(),
        Effect::Inherited { receiver, method, param_desc } => format!("inherited {receiver} | {method} | {param_desc}"),
        Effect::LambdaRef { class, method, rust_name } => format!("lambda_ref {class} | {method} | {rust_name}"),
        Effect::SamSite { iface, sam_desc, class } => format!("sam_site {iface} | {sam_desc} | {class}"),
    }
}

/// 一条记录的 Rust 侧结果：(文本或错误, 外部登记行)
fn eval(env: &Env, rec: &Value) -> Result<(String, Vec<String>), String> {
    let reg = &env.emit.registry;
    let cls = rec["cls"].as_str().unwrap_or_default();
    let owner = rec["owner"].as_str().unwrap_or(cls);
    let (name, desc) = (rec["name"].as_str().unwrap_or_default(), rec["desc"].as_str().unwrap_or_default());
    let ci = reg.get(cls).ok_or_else(|| format!("注册表缺发射类 {cls}"))?;
    let oci = reg.get(owner).ok_or_else(|| format!("注册表缺出处类 {owner}"))?;
    let om = oci
        .methods()
        .iter()
        .find(|m| m.name == name && m.desc == desc)
        .ok_or_else(|| format!("出处类 {owner} 无方法 {name}{desc}"))?;
    let mut view: Method = om.clone();
    view.access = rec["acc"].as_u64().unwrap_or(0) as u16;
    let gsig = rec["gsig"].as_str().unwrap_or("");
    view.signature = (!gsig.is_empty()).then(|| gsig.to_string());
    let code = env.emit.code(owner, om);
    let lvs = local_vars(rec);
    let hooks = hooks_of(rec);
    let ctp = str_list(&rec["ctp"]);
    let ctx = InstrCtx::new(TyCtx::new(reg, &env.names, &env.manifest), &env.rt, &env.facts, &hooks, cls).with_code_owner(owner);
    let ienv = InstrEnv::new(ctx, &ctp);
    let req = MethodRequest {
        class: ci,
        method: &view,
        code: code.as_deref(),
        local_vars: &lvs,
        overloaded: rec["ovl"].as_bool().unwrap_or(false),
        rust_name: rec["rust_name"].as_str(),
        in_vtable_body: rec["vt"].as_bool().unwrap_or(false),
    };
    let mut sink = MethodSink::default();
    let text = gen_method_body(&ienv, &req, &mut sink).unwrap_or_else(|e| format!("ERR {e}"));
    Ok((text, sink.log.effects.iter().map(fx_rs).collect()))
}

struct Tally {
    total: usize,
    text_bad: usize,
    fx_bad: usize,
    report: String,
}

fn run_golden(path: &Path) -> Tally {
    let text = std::fs::read_to_string(path).expect("读 golden");
    let mut lines = text.lines();
    let meta: Value = serde_json::from_str(lines.next().expect("meta 行")).expect("meta JSON");
    let env = build_env(&meta);
    let mut t = Tally { total: 0, text_bad: 0, fx_bad: 0, report: String::new() };
    for l in lines {
        let rec: Value = serde_json::from_str(l).expect("记录 JSON");
        t.total += 1;
        let key = rec["key"].as_str().unwrap_or("?");
        let want = rec["text"].as_str().map_or_else(|| format!("ERR {}", rec["err"].as_str().unwrap_or("")), str::to_string);
        let (got, fx) = match eval(&env, &rec) {
            Ok(r) => r,
            Err(e) => (format!("SETUP {e}"), Vec::new()),
        };
        if got != want {
            t.text_bad += 1;
            let _ = writeln!(t.report, "=== TEXT {key}\n--- py\n{want}\n--- rs\n{got}\n");
        } else if fx != fx_py(&rec) {
            t.fx_bad += 1;
            let _ = writeln!(t.report, "=== FX {key}\n--- py\n{:?}\n--- rs\n{fx:?}\n", fx_py(&rec));
        }
    }
    t
}

#[test]
fn golden_method() {
    let root = repo_root();
    let mut failed = Vec::new();
    for (stem, java) in TESTS {
        let path = root.join("build/golden/method").join(format!("{stem}.jsonl"));
        if !path.exists() {
            eprintln!("跳过 {stem}：缺 golden，先运行 python3 scripts/golden/dump_method.py {java} --jdk 21");
            continue;
        }
        let t = run_golden(&path);
        eprintln!("{stem}: {} 条，文本失配 {}，登记失配 {}", t.total, t.text_bad, t.fx_bad);
        let diff = root.join("build/golden/method").join(format!("{stem}.diff.txt"));
        std::fs::write(&diff, &t.report).expect("写失配明细");
        if t.text_bad + t.fx_bad > 0 {
            failed.push(stem);
        }
    }
    assert!(failed.is_empty(), "golden 失配：{failed:?}");
}
