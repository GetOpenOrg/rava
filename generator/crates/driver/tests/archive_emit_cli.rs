//! 档案发射端到端（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）：
//! 两个用户程序建同一档案 P，各自 `rava build --profile P --stop-after emit`：
//! - 档案 crate（模块 crate / 根声明层 `<根>_decl` 与实现层 `<根>_body_*` / `java_meta` 与其引入的 `closure_input/*.rs` 表）逐字节相同，
//!   包版本取内容摘要（不含 scratch 路径）；
//! - 用户元数据行生成在用户 crate（`rava_user_meta.rs`），入口启动时登记；档案侧表不含用户类。
//! 找不到 JDK 21 时跳过。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

static RAVA: Mutex<()> = Mutex::new(());

/// 见 `closure_cli.rs` 同名函数
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn runtime_dir() -> PathBuf {
    manifest_dir().join("../../../runtime/java_runtime")
}

/// 一次 rava 子命令；缺 JDK → None
fn rava(sub: &str, args: &[&str]) -> Option<String> {
    let _guard = RAVA.lock().unwrap_or_else(|e| e.into_inner());
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg(sub)
        .args(["--jdk", "21", "--runtime"])
        .arg(runtime_dir())
        .args(args)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[archive_emit_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava {sub} 失败：\n{stderr}");
    }
    Some(stderr + &String::from_utf8_lossy(&o.stdout))
}

fn s(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// scratch 下档案 crate 的全部文件：相对路径 → 内容
fn archive_tree(scratch: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack: Vec<PathBuf> = std::fs::read_dir(scratch)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        // 档案 crate = user 以外的全部 crate（模块 crate、根声明层 / 实现层 / 门面、java_meta）
        .filter(|p| p.join("Cargo.toml").is_file() && p.file_name().is_some_and(|n| n != "user"))
        .collect();
    for f in ["meta_tables.rs", "line_tables.rs"] {
        stack.push(scratch.join("closure_input").join(f));
    }
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            stack.extend(std::fs::read_dir(&p).unwrap().flatten().map(|e| e.path()));
        } else {
            let rel = p.strip_prefix(scratch).unwrap().to_string_lossy().replace('\\', "/");
            out.insert(rel, std::fs::read(&p).unwrap());
        }
    }
    out
}

#[test]
fn archive_crates_identical_across_programs() {
    let fx = manifest_dir().join("tests/fixtures");
    let tmp = std::env::temp_dir().join(format!("rava-archive-emit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let (a, b) = (fx.join("MinimalMain.java"), fx.join("NullView.java"));
    let p = tmp.join("p.json");
    if rava("profile", &[&s(&a), &s(&b), "-o", &s(&p)]).is_none() {
        return;
    }
    let mut trees = Vec::new();
    for (java, user_class) in [(&a, "MinimalMain"), (&b, "NullView")] {
        let out = tmp.join(user_class);
        let log = rava("build", &[&s(java), "--profile", &s(&p), "--stop-after", "emit", "--clean", "--out", &s(&out)]).unwrap();
        assert!(!log.contains("[archive-leak]"), "{log}");
        let tree = archive_tree(&out);
        assert!(tree.contains_key("java_meta/Cargo.toml") && tree.keys().any(|k| k.contains("_body_")), "{:?}", tree.keys().take(5).collect::<Vec<_>>());
        // 档案侧表不含用户类；用户元数据行在用户 crate，入口登记
        let meta = String::from_utf8_lossy(&tree["closure_input/meta_tables.rs"]).to_string();
        // 表是字符串池 + 字节流：类名以池项原文出现
        assert!(!meta.contains(user_class), "档案侧表含用户类 {user_class}");
        let rows = std::fs::read_to_string(out.join("user/src/rava_user_meta.rs")).unwrap();
        assert!(rows.contains(user_class) && rows.contains("pub static USER_META"), "{rows}");
        let main = std::fs::read_to_string(out.join("user/src/main.rs")).unwrap();
        assert!(main.contains("java_base::meta::register_user(&rava_user_meta::USER_META);"), "{main}");
        trees.push(tree);
    }
    let (t1, t2) = (&trees[0], &trees[1]);
    let diff: Vec<&String> = t1.keys().chain(t2.keys()).filter(|k| t1.get(*k) != t2.get(*k)).collect();
    assert!(diff.is_empty(), "同一档案下两个程序的档案 crate 不同：{:?}", diff.iter().take(10).collect::<Vec<_>>());
    let _ = std::fs::remove_dir_all(&tmp);
}
