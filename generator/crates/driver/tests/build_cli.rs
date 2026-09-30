//! `rava build --no-run` 端到端：夹具（tests/fixtures/*.java）经 javac → 闭包分析 → 发射，
//! 断言生成文本形态与命令行输出（不编译生成物）。找不到 JDK 21 时跳过。

use std::path::{Path, PathBuf};
use std::process::Command;

fn runtime_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

/// 一次 `rava build`：返回 (stdout, scratch 目录)；缺 JDK → None
fn build(java: &str, tag: &str, extra: &[&str]) -> Option<(String, PathBuf)> {
    let out = std::env::temp_dir().join(format!("rava-it-{tag}-{}", std::process::id()));
    let o = Command::new(env!("CARGO_BIN_EXE_rava"))
        .arg("build")
        .arg(fixture(java))
        .args(["--jdk", "21", "--no-run", "--clean", "--runtime"])
        .arg(runtime_dir())
        .arg("--out")
        .arg(&out)
        .args(extra)
        .output()
        .expect("启动 rava");
    let stderr = String::from_utf8_lossy(&o.stderr).to_string();
    if !o.status.success() {
        if stderr.contains("找不到含 jmods/ 的 JDK") {
            eprintln!("[build_cli] 跳过：无 JDK 21");
            return None;
        }
        panic!("rava build 失败：\n{stderr}");
    }
    Some((String::from_utf8_lossy(&o.stdout).to_string(), out))
}

/// 目录下全部 .rs 文本（路径序）
fn rs_text(dir: &Path) -> String {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                files.push(p);
            }
        }
    }
    files.sort();
    files.iter().map(|f| std::fs::read_to_string(f).unwrap_or_default()).collect::<Vec<_>>().join("\n")
}

/// record 的 ObjectMethods、typeSwitch（含限定枚举常量）、enumSwitch 全部翻译，不落存根 / 占位
#[test]
fn record_and_switch_bootstraps_translate() {
    let Some((_, out)) = build("RecordSwitch.java", "record-switch", &[]) else { return };
    let user = rs_text(&out.join("user/src"));
    for bad in ["panic!(\"stub:", "TODO", "Object::from_any", ".downcast::<", "Default::default() /*"] {
        assert!(!user.contains(bad), "用户类生成文本含 {bad}");
    }
    // toString：简单名 + 分量模板；hashCode：31 累乘；equals：instanceof 后逐分量比较
    assert!(user.contains("format!(\"Point[x={}, y={}, z={}, b={}, name={}]\""), "record toString 模板");
    assert!(user.contains("wrapping_mul(31)"), "record hashCode");
    assert!(user.contains("o.is_instance_of(\"RecordSwitch$Point\") && {"), "record equals");
    // 首分量为引用（块表达式开头）时整体加括号，否则语句位置的 `{ .. } && ..` 被解析成块语句
    let label_eq = user.lines().find(|l| l.contains("o.is_instance_of(\"RecordSwitch$Label\") && {")).expect("Label equals");
    assert!(label_eq.contains("); ({ let x"), "引用首分量的比较整体加括号：{label_eq}");
    // typeSwitch 的限定枚举常量标签按身份比较常量；enumSwitch 同一翻译
    assert!(user.contains("__ts_sel1 == Object::from(Clone::clone(&RecordSwitch_Color::RED()?))"), "枚举常量标签");
    // 拼接实参的 toString 物化按 Java 求值序（从左到右）
    let main_rs = std::fs::read_to_string(out.join("user/src/record_switch.rs")).unwrap();
    let def = |t: &str| main_rs.lines().find(|l| l.trim_start().starts_with(&format!("let {t}: String = "))).unwrap_or_default().to_string();
    assert!(main_rs.contains("format!(\"{} / {}\", _t0, _t1)"), "拼接模板");
    assert!(def("_t0").contains("(&p)") && def("_t1").contains("(&q)"), "拼接实参字符串化次序");
    std::fs::remove_dir_all(&out).ok();
}
