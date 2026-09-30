//! 发射层 golden 对照：`scripts/golden/dump_emit.py` 采集的 Python 生成树 vs 本 crate。
//!
//! 按 meta.json 以同一类路径、closure.json 与 runtime 清单重建 [`EmitInput`]，发射到
//! `build/golden/emit/<Test>.rs-out/` 后逐文件对照（与 runtime 手写真源逐字节相同的文件
//! golden 不转储，比较同样跳过）。
//!
//! 方法体由真实生成器 [`MethodBodies`]（P4c `method` crate）生成，每次生成与 `bodies.jsonl`
//! （Python 每次方法体生成的文本、兜底与登记事实）逐次对照；返回文本用与采集脚本相同的哨兵行
//! 包裹方法体，落盘后同样替换为 `/*BODY key*/` 占位再与 golden 全文对照——发射层基于文本的处理
//! （VTable 导入扫描、record 补丁）看到的是真实方法体。两者合起来即生成树逐字节对照。
//! 方法体失配明细写入 `build/golden/emit/<Test>.bodies.diff.txt`。golden 缺失时跳过并提示采集命令。

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use emit::body::{BodyEffects, BodyError, BodyLog, BodyOutput, BodyRequest, MethodBodyEmitter};
use emit::method_bodies::MethodBodies;
use emit::text::scratch_pkg_version;
use emit::ctx::{EmitCtx, EmitOptions};
use emit::project::{prepare_scratch, write_project};
use input::{BuildInput, ClosureFacts, RuntimeManifest};
use resolve::classpath::{ClassPath, Origin};
use serde_json::Value;
use ty::ShortNames;

const TESTS: [(&str, &str); 3] = [
    ("TestHashMapOps", "tests/e2e/33_maps/TestHashMapOps.java"),
    ("TestStreamBasic", "tests/e2e/30_streams/TestStreamBasic.java"),
    ("TestCompletableFuture", "tests/e2e/54_concurrency_api/TestCompletableFuture.java"),
];

/// 各 crate 目录（相对 scratch 根）：包版本按目录路径派生
const CRATE_DIRS: [&str; 3] = ["", "java_runtime", "user"];
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn str_list(v: &Value) -> Vec<String> {
    v.as_array().into_iter().flatten().filter_map(Value::as_str).map(str::to_string).collect()
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

fn files_under(dir: &Path, base: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.map(|e| e.expect("目录项").path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            files_under(&p, base, out);
        } else {
            out.push(p.strip_prefix(base).expect("相对路径").to_path_buf());
        }
    }
}

fn file_diff(want: &str, got: &str) -> Option<String> {
    (want != got).then(|| {
        let (ln, a, b) = first_diff(want, got);
        format!("L{ln}\n      py: {a}\n      rs: {b}")
    })
}

/// 语料 JDK 特性版本（`<java_home>/release` 的 JAVA_VERSION 首段）
fn jdk_major(java_home: &str) -> Option<u32> {
    let text = std::fs::read_to_string(Path::new(java_home).join("release")).ok()?;
    let v = text.lines().find_map(|l| l.strip_prefix("JAVA_VERSION="))?.trim_matches('"');
    v.split('.').next()?.parse().ok()
}

/// 首个差异行（行号, py, rs）
fn first_diff(want: &str, got: &str) -> (usize, String, String) {
    let (w, g): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
    let n = w.len().max(g.len());
    for i in 0..n {
        let (a, b) = (w.get(i).copied().unwrap_or("<EOF>"), g.get(i).copied().unwrap_or("<EOF>"));
        if a != b {
            return (i + 1, a.to_string(), b.to_string());
        }
    }
    (n, String::new(), String::new())
}

const BODY_BEGIN: &str = "//@@BODY_BEGIN ";
const BODY_END: &str = "//@@BODY_END";

/// 方法体调用键：(类.方法:描述符, 显式 Rust 名, in_vtable_body)
type CallKey = (String, Option<String>, bool);

/// Python 一次方法体生成的结果：完整函数文本或兜底异常文本，及登记事实
type Expected = (Result<String, String>, BodyEffects);

fn triples(v: &Value) -> Vec<(String, String, String)> {
    v.as_array()
        .into_iter()
        .flatten()
        .map(|t| {
            let s = |i: usize| t[i].as_str().unwrap_or_default().to_string();
            (s(0), s(1), s(2))
        })
        .collect()
}

/// `bodies.jsonl` → 按 (键, Rust 名, in_vtable_body) 分组的 Python 结果队列
fn load_expected(path: &Path) -> HashMap<CallKey, VecDeque<Expected>> {
    let mut queue: HashMap<CallKey, VecDeque<Expected>> = HashMap::new();
    let text = std::fs::read_to_string(path).unwrap_or_default();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let r: Value = serde_json::from_str(line).expect("bodies.jsonl 行");
        let key = r["key"].as_str().unwrap_or_default().to_string();
        let effects = BodyEffects {
            requests: triples(&r["requests"]),
            lambda_refs: triples(&r["lambda_refs"]),
            sam_sites: triples(&r["sam_sites"]),
        };
        let s = |k: &str| r[k].as_str().unwrap_or_default().to_string();
        let out = if let Some(f) = r["fallback"].as_str() {
            Err(f.to_string())
        } else if r["unsplit"].as_bool() == Some(true) {
            Ok(s("text"))
        } else if s("body").is_empty() {
            Ok(format!("{}}}", s("head")))
        } else {
            Ok(format!("{}{}\n}}", s("head"), s("body")))
        };
        let rk = (key, r["rust_name"].as_str().map(str::to_string), r["in_vtable_body"].as_bool().unwrap_or(false));
        queue.entry(rk).or_default().push_back((out, effects));
    }
    queue
}

/// 函数文本 → (头部含 ` {\n`, 方法体, 尾部)（采集脚本 `_split_body` 同口径：构造器双入口取最后一个 fn）
fn split_body(text: &str) -> Option<(&str, &str, &str)> {
    if !text.ends_with("\n}") {
        return None;
    }
    let from = text.rfind("#[doc(hidden)]\n").unwrap_or(0);
    let head_end = from + text[from..].find(" {\n")? + 3;
    let body_end = text.len() - 2;
    if body_end + 1 < head_end {
        return None;
    }
    if body_end < head_end {
        return Some((&text[..head_end], "", &text[head_end..]));
    }
    Some((&text[..head_end], &text[head_end..body_end], &text[body_end..]))
}

/// 真实方法体生成 + 逐次对照 Python 记录；返回文本按采集脚本同形包裹哨兵行，
/// 落盘后替换为 `/*BODY key*/` 与占位 golden 做全文对照
/// （生成器跨线程共享；对照用 `jobs = 1` 串行发射，失配明细保持发射序）
struct Checked {
    real: MethodBodies,
    expected: Mutex<HashMap<CallKey, VecDeque<Expected>>>,
    /// 方法体级失配明细
    mismatches: Mutex<Vec<String>>,
    calls: AtomicUsize,
}

impl MethodBodyEmitter for Checked {
    fn emit_body(&self, ctx: &EmitCtx<'_>, req: &BodyRequest<'_>, log: &mut BodyLog) -> Result<BodyOutput, BodyError> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let key = format!("{}.{}:{}", req.class.name(), req.method.name, req.method.desc);
        let rk = (key.clone(), req.rust_name.map(str::to_string), req.in_vtable_body);
        let got = self.real.emit_body(ctx, req, log);
        let want = self.expected.lock().unwrap().get_mut(&rk).and_then(VecDeque::pop_front);
        let mut mismatches = self.mismatches.lock().unwrap();
        match want {
            None => mismatches.push(format!("=== EXTRA {rk:?}（Python 无此请求）")),
            Some((want, fx)) => {
                let got_text = match &got {
                    Ok(o) => Ok(o.text.clone()),
                    Err(BodyError::Fallback(e) | BodyError::Fatal(e)) => Err(e.clone()),
                };
                let text_ok = match (&want, &got_text) {
                    (Ok(a), Ok(b)) => a == b,
                    (Err(_), Err(_)) => matches!(got, Err(BodyError::Fallback(_))),
                    _ => false,
                };
                if !text_ok {
                    mismatches.push(format!("=== TEXT {rk:?}\n--- py\n{want:?}\n--- rs\n{got_text:?}\n"));
                } else if let Ok(o) = &got {
                    if o.effects != fx {
                        mismatches.push(format!("=== FX {rk:?}\n--- py\n{fx:?}\n--- rs\n{:?}\n", o.effects));
                    }
                }
            }
        }
        let mut out = got?;
        if let Some((head, body, tail)) = split_body(&out.text) {
            out.text = if body.is_empty() {
                format!("{head}{BODY_BEGIN}{key}\n{BODY_END}{tail}")
            } else {
                format!("{head}{BODY_BEGIN}{key}\n{body}\n{BODY_END}{tail}")
            };
        }
        Ok(out)
    }
}

/// 哨兵段 → `{体内缩进}/*BODY key*/`（与采集脚本 `_replace_bodies` 同形，计嵌套）
fn replace_bodies(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        let t = l.trim_start_matches([' ', '\t']);
        let Some(key) = t.strip_prefix(BODY_BEGIN) else {
            out.push(l.to_string());
            i += 1;
            continue;
        };
        let indent = &l[..l.len() - t.len()];
        let mut depth = 1;
        let mut j = i + 1;
        while j < lines.len() {
            let s = lines[j].trim();
            if s.starts_with(BODY_BEGIN) {
                depth += 1;
            } else if s == BODY_END {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            j += 1;
        }
        out.push(format!("{indent}    /*BODY {key}*/"));
        i = j + 1;
    }
    out.join("\n")
}

/// (对照文件数, 差异：路径 → 描述)
fn run_golden(root: &Path, stem: &str) -> (usize, BTreeMap<String, String>) {
    let gdir = root.join("build/golden/emit").join(stem);
    let meta: Value = serde_json::from_str(&std::fs::read_to_string(gdir.join("meta.json")).expect("meta.json")).expect("meta");
    let cp = class_path(&meta);
    let cj_text = std::fs::read_to_string(meta["closure_json"].as_str().unwrap_or_default()).expect("closure.json");
    let facts = ClosureFacts::from_json(&serde_json::from_str(&cj_text).expect("closure.json 解析")).expect("闭包事实");
    let runtime = PathBuf::from(meta["runtime"].as_str().unwrap_or_default());
    let manifest = RuntimeManifest::load(&runtime).expect("runtime 清单");
    let user = str_list(&meta["user_classes"]);
    let inp = BuildInput { cp: &cp, facts: &facts, manifest: &manifest, user_classes: &user, libs: &[], runtime_src: &runtime.join("src") }
        .build()
        .expect("构建发射层输入");
    let names = ShortNames::build(&inp.registry);
    let opts = EmitOptions {
        strict: false,
        jdk_major: jdk_major(meta["java_home"].as_str().unwrap_or_default()),
        java_files: str_list(&meta["java_files"]).into_iter().map(PathBuf::from).collect(),
        jobs: 1,
    };
    let ctx = EmitCtx::new(&inp, &names, &manifest, &cp, &runtime, opts).expect("发射上下文");
    let out = root.join("build/golden/emit").join(format!("{stem}.rs-out"));
    prepare_scratch(&out, &runtime, &ctx.macros_crate, true).expect("scratch overlay");
    let mut diffs = BTreeMap::new();
    let checked = Checked {
        real: MethodBodies::new(&ctx),
        expected: Mutex::new(load_expected(&gdir.join("bodies.jsonl"))),
        mismatches: Mutex::new(Vec::new()),
        calls: AtomicUsize::new(0),
    };
    let written = write_project(&ctx, &out, &checked);
    let Checked { expected, mismatches, calls, .. } = checked;
    let (expected, mismatches, calls) = (expected.into_inner().unwrap(), mismatches.into_inner().unwrap(), calls.into_inner());
    let report = root.join("build/golden/emit").join(format!("{stem}.bodies.diff.txt"));
    std::fs::write(&report, mismatches.join("\n")).expect("写方法体失配明细");
    eprintln!("{stem}: 方法体生成 {} 次，失配 {}（明细 {}）", calls, mismatches.len(), report.display());
    if let Err(e) = written {
        diffs.insert("<write_project>".to_string(), e.to_string());
        return (0, diffs);
    }
    if !mismatches.is_empty() {
        diffs.insert("<bodies>".to_string(), format!("{} 次方法体生成与 Python 失配", mismatches.len()));
    }
    let unused: usize = expected.values().map(VecDeque::len).sum();
    if unused > 0 {
        let sample: Vec<String> =
            expected.iter().filter(|(_, q)| !q.is_empty()).take(5).map(|(k, _)| format!("{k:?}")).collect();
        diffs.insert("<bodies.jsonl>".to_string(), format!("{unused} 条方法体记录未被请求，例：{}", sample.join(" / ")));
    }
    // 包版本由 crate 目录路径派生：rs 输出目录不同，按 py 输出目录的同一派生归一
    let py_out = PathBuf::from(meta["out_dir"].as_str().unwrap_or_default());
    let versions: Vec<(String, String)> = CRATE_DIRS
        .iter()
        .map(|d| {
            let v = |base: &Path| format!("version = \"{}\"", scratch_pkg_version(&base.join(d)));
            (v(&out), v(&py_out))
        })
        .collect();
    let mut rel = Vec::new();
    files_under(&gdir.join("files"), &gdir.join("files"), &mut rel);
    for r in &rel {
        let want = std::fs::read(gdir.join("files").join(r)).unwrap_or_default();
        let key = r.display().to_string();
        let Ok(got) = std::fs::read(out.join(r)) else {
            diffs.insert(key, "rs 未生成".into());
            continue;
        };
        if want == got {
            continue;
        }
        let (Ok(want), Ok(mut got)) = (String::from_utf8(want), String::from_utf8(got)) else {
            diffs.insert(key, "二进制内容不同".into());
            continue;
        };
        if got.contains(BODY_BEGIN) {
            got = replace_bodies(&got);
        }
        for (rs_v, py_v) in &versions {
            got = got.replace(rs_v, py_v);
        }
        if let Some(d) = file_diff(&want, &got) {
            diffs.insert(key, d);
        }
    }
    (rel.len(), diffs)
}

#[test]
fn golden_emit() {
    let root = repo_root();
    let mut any = false;
    let mut failed = Vec::new();
    for (stem, java) in TESTS {
        if !root.join("build/golden/emit").join(stem).join("meta.json").exists() {
            eprintln!("跳过 {stem}：缺 golden，先运行 python3 scripts/golden/dump_emit.py {java}");
            continue;
        }
        any = true;
        let (total, diffs) = run_golden(&root, stem);
        eprintln!("{stem}: 对照 {total} 个文件，失配 {}", diffs.len());
        for (p, d) in diffs.iter().take(60) {
            eprintln!("  {p} {d}");
        }
        if !diffs.is_empty() {
            failed.push(stem);
        }
    }
    if !any {
        eprintln!("golden_emit：无 golden，已跳过");
    }
    assert!(failed.is_empty(), "golden 失配：{failed:?}");
}
