//! 发射层 golden 对照：`scripts/golden/dump_emit.py` 采集的 Python 生成树 vs 本 crate。
//!
//! 按 meta.json 以同一类路径、closure.json 与 runtime 清单重建 [`EmitInput`]，发射到
//! `build/golden/emit/<Test>.rs-out/` 后逐文件对照（与 runtime 手写真源逐字节相同的文件
//! golden 不转储，比较同样跳过）。
//!
//! 方法体以 [`Replay`] 回放 `bodies.jsonl`（Python 每次方法体生成的真实文本与登记事实），
//! 回放文本用与采集脚本相同的哨兵行包裹方法体，落盘后同样替换为 `/*BODY key*/` 占位再比较——
//! 发射层基于文本的处理（VTable 导入扫描、record 补丁）看到的仍是真实方法体。
//!
//! 对照口径随移植阶段收紧（[`CLASS_STAGE`]）：类文件当前比到方法段为止（文件头 / 导入 /
//! 类块头 / struct / static 字段 / 方法块；继承段在步骤 (d) 接入后改为全文）；
//! 非类文件（Cargo.toml / mod.rs / main.rs …）全文。golden 缺失时跳过并提示采集命令。

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::path::{Path, PathBuf};

use emit::body::{BodyEffects, BodyError, BodyOutput, BodyRequest, MethodBodyEmitter};
use emit::class_writer::{INHERITED_IMPORTS_SLOT, INHERITED_MEMBERS_SLOT};
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

/// 类文件对照范围
#[derive(Clone, Copy, PartialEq, Eq)]
enum ClassStage {
    /// `java_class!` 块到继承成员插入位为止（rs 方法段须是 py 方法段的前缀：py 在其后接续
    /// 接口 default / special / 超类继承段）
    UpToMethodBlocks,
    /// 全文（继承段接入后启用）
    #[allow(dead_code)]
    Full,
}
const CLASS_STAGE: ClassStage = ClassStage::UpToMethodBlocks;
const CLASS_MARK: &str = "rava_macros::java_class! {";
/// 各 crate 目录（相对 scratch 根）：包版本按目录路径派生
const CRATE_DIRS: [&str; 3] = ["", "java_runtime", "user"];
/// 待后续步骤接入的已知失配（报告但不判失败；全部移植后须清空）
const PENDING: [(&str, &str); 4] = [
    ("user/src/main.rs", "反射分派注册表（步骤 d：dispatch_gen）"),
    ("<bodies.jsonl>", "接口 default / special / 超类虚方法继承段的方法体请求（步骤 d）"),
    ("java_runtime/src/java/util/stream/collectors_collector_impl.rs", "record 访问器补丁（步骤 d）"),
    ("java_runtime/src/jdk/internal/reflect/reflection_factory_config.rs", "record 访问器补丁（步骤 d）"),
];

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

/// 按阶段比较类文件：文件头（rs 去继承导入插入位后须是 py 文件头的前缀——py 在插入位
/// 已填入继承成员导入）+ `java_class!` 块（rs 到继承成员插入位为止，须是 py 块的行前缀）。
/// 返回首个差异描述
fn staged_class_diff(want: &str, got: &str) -> Option<String> {
    let (w, g): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
    let (Some(wm), Some(gm)) = (w.iter().position(|l| *l == CLASS_MARK), g.iter().position(|l| *l == CLASS_MARK)) else {
        return Some("缺 java_class! 块".into());
    };
    let g_head: Vec<&str> = g[..gm].iter().copied().filter(|l| *l != INHERITED_IMPORTS_SLOT && !l.is_empty()).collect();
    let w_head: Vec<&str> = w[..wm].iter().copied().filter(|l| !l.is_empty()).collect();
    for (i, gl) in g_head.iter().enumerate() {
        let wl = w_head.get(i).copied().unwrap_or("<EOF>");
        if wl != *gl {
            return Some(format!("文件头第 {} 条\n      py: {wl}\n      rs: {gl}", i + 1));
        }
    }
    let g_end = g[gm..].iter().position(|l| l.trim() == INHERITED_MEMBERS_SLOT).map_or(g.len(), |p| gm + p);
    for (i, gl) in g[gm..g_end].iter().enumerate() {
        let wl = w.get(wm + i).copied().unwrap_or("<EOF>");
        if wl != *gl {
            return Some(format!("块 L{}\n      py: {wl}\n      rs: {gl}", i + 1));
        }
    }
    None
}

fn file_diff(want: &str, got: &str) -> Option<String> {
    if CLASS_STAGE == ClassStage::UpToMethodBlocks && want.contains(CLASS_MARK) {
        return staged_class_diff(want, got);
    }
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

/// 回放记录键：(类.方法:描述符, 显式 Rust 名, in_vtable_body)
type ReplayKey = (String, Option<String>, bool);

/// `bodies.jsonl` 回放：按 (键, Rust 名, in_vtable_body) 队列依次取 Python 的方法体文本与登记事实
struct Replay {
    queue: HashMap<ReplayKey, VecDeque<Result<BodyOutput, BodyError>>>,
}

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

impl Replay {
    fn load(path: &Path) -> Replay {
        let mut queue: HashMap<ReplayKey, VecDeque<_>> = HashMap::new();
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
                Err(BodyError::Fallback(f.to_string()))
            } else if r["unsplit"].as_bool() == Some(true) {
                Ok(BodyOutput { text: s("text"), effects })
            } else {
                let body = s("body");
                let text = if body.is_empty() {
                    format!("{}{BODY_BEGIN}{key}\n{BODY_END}\n}}", s("head"))
                } else {
                    format!("{}{BODY_BEGIN}{key}\n{body}\n{BODY_END}\n}}", s("head"))
                };
                Ok(BodyOutput { text, effects })
            };
            let rk = (key, r["rust_name"].as_str().map(str::to_string), r["in_vtable_body"].as_bool().unwrap_or(false));
            queue.entry(rk).or_default().push_back(out);
        }
        Replay { queue }
    }
}

impl MethodBodyEmitter for Replay {
    fn emit_body(&mut self, _ctx: &EmitCtx<'_>, req: &BodyRequest<'_>) -> Result<BodyOutput, BodyError> {
        let key = format!("{}.{}:{}", req.class.name(), req.method.name, req.method.desc);
        let rk = (key, req.rust_name.map(str::to_string), req.in_vtable_body);
        match self.queue.get_mut(&rk).and_then(VecDeque::pop_front) {
            Some(out) => out,
            None => Err(BodyError::Fatal(format!("回放缺记录：{rk:?}"))),
        }
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
    };
    let ctx = EmitCtx::new(&inp, &names, &manifest, &cp, &runtime, opts).expect("发射上下文");
    let out = root.join("build/golden/emit").join(format!("{stem}.rs-out"));
    prepare_scratch(&out, &runtime, &ctx.macros_crate, true).expect("scratch overlay");
    let mut diffs = BTreeMap::new();
    let mut replay = Replay::load(&gdir.join("bodies.jsonl"));
    if let Err(e) = write_project(&ctx, &out, &mut replay) {
        diffs.insert("<write_project>".to_string(), e.to_string());
        return (0, diffs);
    }
    let unused: usize = replay.queue.values().map(VecDeque::len).sum();
    if unused > 0 {
        let sample: Vec<String> = replay.queue.iter().filter(|(_, q)| !q.is_empty()).take(5).map(|(k, _)| format!("{k:?}")).collect();
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
        let mut hard = 0;
        for (p, d) in diffs.iter().take(60) {
            match PENDING.iter().find(|(f, _)| f == p) {
                Some((_, why)) => eprintln!("  [待接入：{why}] {p} {d}"),
                None => {
                    hard += 1;
                    eprintln!("  {p} {d}");
                }
            }
        }
        if hard > 0 || diffs.len() > 60 {
            failed.push(stem);
        }
    }
    if !any {
        eprintln!("golden_emit：无 golden，已跳过");
    }
    assert!(failed.is_empty(), "golden 失配：{failed:?}");
}
