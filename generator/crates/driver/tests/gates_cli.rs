//! `rava closure --gates` 端到端：已知门的小夹具——main 里唯一的接口调用点 `p.run()` 派发到 16 个实现，
//! 每个实现各 `new` 100 个互不相干的类。该调用点是唯一把这 1600 个类带进闭包的门：
//! 断言它排名第一、模型与实测单切 Δ 均为 1600、类别为精度缺口（派发扇出证据）。找不到 JDK 21 时跳过。

use std::path::PathBuf;
use std::process::Command;

const IMPLS: usize = 16;
const PER: usize = 100;

fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

fn fixture(dir: &std::path::Path) -> PathBuf {
    let mut s = String::from("interface Plugin { void run(); }\n\npublic class GateFixture {\n    public static void main(String[] args) {\n        Plugin[] ps = {");
    s.push_str(&(0..IMPLS).map(|i| format!("new Impl{i}()")).collect::<Vec<_>>().join(", "));
    s.push_str("};\n        for (Plugin p : ps) p.run();\n    }\n}\n");
    for i in 0..IMPLS {
        s.push_str(&format!("\nclass Impl{i} implements Plugin {{\n    public void run() {{\n"));
        for j in 0..PER {
            s.push_str(&format!("        new D{}();\n", i * PER + j));
        }
        s.push_str("    }\n}\n");
    }
    for k in 0..IMPLS * PER {
        s.push_str(&format!("class D{k} {{}}\n"));
    }
    let p = dir.join("GateFixture.java");
    std::fs::write(&p, s).unwrap();
    p
}

#[test]
fn known_gate_ranks_first() {
    let tmp = std::env::temp_dir().join(format!("rava-gates-cli-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let java = fixture(&tmp);
    let out = tmp.join("gates.json");
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("closure")
        .arg(&java)
        .args(["--jdk", "21", "--runtime"])
        .arg(manifest_dir().join("../../../runtime/java_runtime"))
        .args(["--gates", "--gates-top", "8", "--gates-verify", "2", "--gates-jobs", "2", "--gates-out"])
        .arg(&out)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr);
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[gates_cli] 跳过：无 JDK 21");
            return;
        }
        panic!("rava closure --gates 失败：\n{stderr}");
    }
    let v: serde_json::Value = serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
    let _ = std::fs::remove_dir_all(&tmp);
    let n = (IMPLS * PER) as u64;
    let g = &v["gates"][0];
    let id = g["id"].as_str().unwrap();
    assert!(id.starts_with("GateFixture.main:([Ljava/lang/String;)V@"), "排名第一应为 main 的接口调用点：{g:#}");
    assert_eq!(g["kind"], "site");
    assert_eq!(g["single"]["model"]["classes"].as_u64(), Some(n), "{g:#}");
    assert_eq!(g["single"]["real"]["classes"].as_u64(), Some(n), "{g:#}");
    assert_eq!(g["single"]["real"]["added"].as_u64(), Some(0), "{g:#}");
    assert_eq!(g["category"], "precision", "{g:#}");
    let ev = g["evidence"].to_string();
    assert!(ev.contains(&format!("派发目标 {IMPLS} 个")), "{ev}");
    // 贪心第一步即该门，累计 Δ 同单切
    assert_eq!(v["greedy"][0]["gate"].as_str(), Some(id), "{:#}", v["greedy"]);
    assert_eq!(v["greedy"][0]["model"]["classes"].as_u64(), Some(n));
    // 示例首达链从下游类走回该门
    assert!(g["example"].as_str().is_some_and(|e| e.starts_with('D')), "{g:#}");
}
