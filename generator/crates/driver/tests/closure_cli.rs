//! `rava closure` 端到端：记录型 `--flows` 查询（`@grow:` / `@trace:` / `@edge:`）分析前登记、传播中记录，
//! 结果随查询输出并实时写 stderr；不带记录型查询时不产生任何记录。找不到 JDK 21 时跳过。

use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

/// 同一进程内的 rava 子进程串行（同 `build_cli.rs`）
static RAVA: Mutex<()> = Mutex::new(());

/// 当前工作区的包目录：取运行期 `CARGO_MANIFEST_DIR`（cargo 按本次调用设置）。编译期 `env!` 在全机共享的
/// CARGO_TARGET_DIR 下可能指向另一工作区——cargo 对路径包按工作区相对路径算 metadata，源码相同时不重编，
/// 测试二进制里嵌的就是首次编译它的（可能已删除的）worktree
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// 一次 `rava closure`：返回 (stdout, stderr)；缺 JDK → None
fn closure(java: &str, extra: &[&str]) -> Option<(String, String)> {
    let dir = manifest_dir();
    let _guard = RAVA.lock().unwrap_or_else(|e| e.into_inner());
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("closure")
        .arg(dir.join("tests/fixtures").join(java))
        .args(["--jdk", "21", "--runtime"])
        .arg(dir.join("../../../runtime/java_runtime"))
        .args(extra)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("未找到 JDK") {
            eprintln!("[closure_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava closure 失败：\n{stderr}");
    }
    Some((String::from_utf8_lossy(&o.stdout).to_string(), stderr))
}

/// 查询结果段：从标题行起到空行止
fn section<'s>(out: &'s str, query: &str) -> Vec<&'s str> {
    let head = format!("  {query}：");
    let lines: Vec<&str> = out.lines().skip_while(|l| !l.starts_with(&head)).take_while(|l| !l.is_empty()).collect();
    assert!(!lines.is_empty(), "缺查询段 {query}：{out}");
    lines
}

#[test]
fn recording_flow_queries() {
    const PASS: &str = "FlowProbe.pass:(Ljava/lang/Object;)Ljava/lang/Object;";
    let grow = "@grow:R FlowProbe.pass";
    let trace = "@trace:FlowProbe$Box";
    let edge = "@edge:R FlowProbe.pass";
    let Some((out, err)) = closure("FlowProbe.java", &["--flows", grow, "--flows", trace, "--flows", edge, "--flows", "@array"]) else {
        return;
    };
    // @grow：返回节点经形参 P0 获得 StringBuilder
    let g = section(&out, grow);
    assert!(g[0].ends_with("：1 条"), "{g:?}");
    assert!(g[1].contains(&format!("R {PASS} ← P0 {PASS} +{{java/lang/StringBuilder}}")), "{g:?}");
    // @edge：P0 → R 流边
    let e = section(&out, edge);
    assert!(e.iter().any(|l| l.contains(&format!("P0 {PASS} → R {PASS} [java/lang/Object]"))), "{e:?}");
    // @trace：Box 在 main 的分配站点直接注入，再流入构造器接收者；序号按到达先后递增
    let t = section(&out, trace);
    assert!(t[1].contains("← 直接 FlowProbe.main:([Ljava/lang/String;)V@"), "{t:?}");
    assert!(t.iter().any(|l| l.contains("P0 FlowProbe$Box.<init>:()V ← ")), "{t:?}");
    let seqs: Vec<u64> = t[1..].iter().map(|l| l.trim_start()[1..].split(' ').next().unwrap().parse().unwrap()).collect();
    assert!(seqs.windows(2).all(|w| w[0] < w[1]), "{seqs:?}");
    // 实时记录与查询结果逐条一致
    for l in g[1..].iter().chain(&t[1..]).chain(&e[1..]) {
        assert!(err.contains(l.trim_start()), "stderr 缺实时记录 {l}");
    }
    // 非记录型查询照常在分析后求值
    assert!(out.contains("  array = {"), "{out}");
}

#[test]
fn no_recording_without_queries() {
    let Some((out, err)) = closure("FlowProbe.java", &["--flows", "FlowProbe.pass"]) else {
        return;
    };
    assert!(!err.contains("[flows "), "未登记记录型查询却有记录：{err}");
    assert!(out.contains("FlowProbe.pass:(Ljava/lang/Object;)Ljava/lang/Object;"), "{out}");
}
