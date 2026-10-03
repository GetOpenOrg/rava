//! `rava profile` 端到端：多入口档案 = 各单例闭包非用户侧的并集；档案键对入口给出顺序、入口来源（源码 / 已算闭包）
//! 不敏感；`--flow-batch` / `--hash-seed` 不改变档案内容；`--covers` 判定各单例被档案覆盖。找不到 JDK 21 时跳过。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use serde_json::Value;

static RAVA: Mutex<()> = Mutex::new(());

/// 见 `closure_cli.rs` 同名函数
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// 一次 `rava profile`：返回 stdout；缺 JDK → None
fn profile(args: &[&str]) -> Option<String> {
    let dir = manifest_dir();
    let _guard = RAVA.lock().unwrap_or_else(|e| e.into_inner());
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("profile")
        .args(["--jdk", "21", "--runtime"])
        .arg(dir.join("../../../runtime/java_runtime"))
        .args(args)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[profile_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava profile 失败：\n{stderr}");
    }
    Some(String::from_utf8_lossy(&o.stdout).to_string())
}

fn read(p: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap()
}

fn non_user_classes(v: &Value) -> BTreeSet<String> {
    v["classes"].as_array().unwrap().iter().filter(|c| c["domain"] != "user").map(|c| c["name"].as_str().unwrap().to_string()).collect()
}

fn s(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

#[test]
fn profile_union_key_and_coverage() {
    let fx = manifest_dir().join("tests/fixtures");
    let tmp = std::env::temp_dir().join(format!("rava-profile-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let (a, b) = (fx.join("MinimalMain.java"), fx.join("NullView.java"));
    let (p1, p2, p3, d1) = (tmp.join("p1.json"), tmp.join("p2.json"), tmp.join("p3.json"), tmp.join("entries"));

    // 源码入口，写出各单例闭包
    let Some(out) = profile(&[&s(&a), &s(&b), "-o", &s(&p1), "--entry-out", &s(&d1)]) else { return };
    assert!(out.contains("入口 2 个"), "{out}");
    let v1 = read(&p1);
    let (ca, cb) = (read(&d1.join("MinimalMain.closure.json")), read(&d1.join("NullView.closure.json")));

    // 档案类集合 = 各单例非用户类的并集；用户类不入档案
    let union: BTreeSet<String> = non_user_classes(&ca).union(&non_user_classes(&cb)).cloned().collect();
    assert_eq!(non_user_classes(&v1), union);
    assert!(v1["classes"].as_array().unwrap().iter().all(|c| c["domain"] != "user"));
    assert_eq!(v1["profile"]["entries"].as_array().unwrap().len(), 2);

    // 已算闭包、逆序给出：键与内容摘要不变
    profile(&["--closure", &s(&d1.join("NullView.closure.json")), "--closure", &s(&d1.join("MinimalMain.closure.json")), "-o", &s(&p2)]);
    let v2 = read(&p2);
    assert_eq!(v1["profile"]["key"], v2["profile"]["key"]);
    assert_eq!(v1["profile"]["content_digest"], v2["profile"]["content_digest"]);

    // 入口清单（逆序）+ 改流批量与哈希种子：档案内容不变
    let list = tmp.join("entries.txt");
    std::fs::write(&list, format!("# 档案入口\n{}\n{}  --name MinimalMain\n", s(&b), s(&a))).unwrap();
    profile(&["--entries", &s(&list), "--flow-batch", "1", "--hash-seed", "7", "-o", &s(&p3)]);
    let v3 = read(&p3);
    assert_eq!(v1["profile"]["content_digest"], v3["profile"]["content_digest"]);
    assert_eq!(v1["profile"]["key"], v3["profile"]["key"]);
    assert_eq!(v1["profile"]["inputs"]["entries"], v3["profile"]["inputs"]["entries"], "同一源码入口的输入摘要与给出方式无关");

    // 覆盖判定：两个单例都被档案覆盖
    let cov = profile(&["--covers", &s(&p1), &s(&d1.join("MinimalMain.closure.json")), &s(&d1.join("NullView.closure.json"))]).unwrap();
    assert_eq!(cov.lines().filter(|l| l.starts_with("covered ")).count(), 2, "{cov}");
    let _ = std::fs::remove_dir_all(&tmp);
}
