//! 发射层 golden 对照：`scripts/golden/dump_emit.py` 采集的 Python 生成树 vs 本 crate。
//!
//! 按 meta.json 以同一类路径、closure.json 与 runtime 清单重建 [`EmitInput`]，发射到
//! `build/golden/emit/<Test>.rs-out/` 后逐文件对照（与 runtime 手写真源逐字节相同的文件
//! golden 不转储，比较同样跳过）。
//!
//! 对照口径随移植阶段收紧（[`CLASS_STAGE`]）：类文件当前只比到 impl 头（文件头 / 导入 /
//! 类块头 / struct），方法块接入后改为全文；非类文件（Cargo.toml / mod.rs / main.rs …）全文。
//! golden 缺失时跳过并提示采集命令。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use emit::body::PlaceholderBodies;
use emit::class_writer::INHERITED_IMPORTS_SLOT;
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
    /// 到 `java_class!` 块内 impl 头为止
    UpToImplHeader,
    /// 全文（方法块接入后启用）
    #[allow(dead_code)]
    Full,
}
const CLASS_STAGE: ClassStage = ClassStage::UpToImplHeader;
const CLASS_MARK: &str = "rava_macros::java_class! {";
/// 各 crate 目录（相对 scratch 根）：包版本按目录路径派生
const CRATE_DIRS: [&str; 3] = ["", "java_runtime", "user"];
/// 待后续步骤接入的已知失配（报告但不判失败；全部移植后须清空）
const PENDING: [(&str, &str); 1] = [("user/src/main.rs", "反射分派注册表（步骤 d：dispatch_gen）")];

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

/// `java_class!` 块从开头到 impl 头（含）
fn block_to_impl_header(lines: &[&str]) -> String {
    let mut out = Vec::new();
    for l in lines {
        out.push(*l);
        let t = l.trim_start();
        if l.len() - t.len() == 4 && t.starts_with("impl") && t.ends_with('{') {
            break;
        }
    }
    out.join("\n")
}

/// 按阶段比较类文件：文件头（rs 去继承导入插入位后须是 py 文件头的前缀——py 在插入位
/// 已填入继承成员导入）+ `java_class!` 块到 impl 头。返回首个差异描述
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
    let (wb, gb) = (block_to_impl_header(&w[wm..]), block_to_impl_header(&g[gm..]));
    (wb != gb).then(|| {
        let (ln, a, b) = first_diff(&wb, &gb);
        format!("块 L{ln}\n      py: {a}\n      rs: {b}")
    })
}

fn file_diff(want: &str, got: &str) -> Option<String> {
    if CLASS_STAGE == ClassStage::UpToImplHeader && want.contains(CLASS_MARK) {
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
    if let Err(e) = write_project(&ctx, &out, &mut PlaceholderBodies) {
        diffs.insert("<write_project>".to_string(), e.to_string());
        return (0, diffs);
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
