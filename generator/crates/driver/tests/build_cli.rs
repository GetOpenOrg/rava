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

/// `--batch`：入口写 `user/src/bin/<bin>.rs`（`#[path]` 引用同级类文件），user/Cargo.toml 追加 `[[bin]]`；
/// `--trace-class` 打印 provenance 链；`--debug` 时审计照常输出
#[test]
fn batch_trace_and_debug() {
    let Some((stdout, out)) =
        build("RecordSwitch.java", "batch", &["--batch", "--debug", "--trace-class", "RecordSwitch$Point"])
    else {
        return;
    };
    let bin = std::fs::read_to_string(out.join("user/src/bin/record_switch.rs")).expect("批量入口");
    assert!(bin.contains("#[path = \"../record_switch.rs\"]"), "批量入口 #[path]：{bin}");
    let cargo = std::fs::read_to_string(out.join("user/Cargo.toml")).unwrap();
    assert!(cargo.contains("[[bin]]\nname = \"record_switch\"\npath = \"src/bin/record_switch.rs\""), "[[bin]] 段：{cargo}");
    assert!(stdout.contains("      [why] RecordSwitch$Point"), "provenance 链：{stdout}");
    assert!(stdout.contains("[precheck] native-missing="), "预检行");
    assert!(stdout.contains("[cfg-audit] ") && stdout.contains("[equiv-audit] "), "审计行");
    std::fs::remove_dir_all(&out).ok();
}

/// `--lib`：jar 类进独立 lib crate（Java 可见性、lib.rs 包模块、user 依赖）；`--precheck-only`
/// 只出预检，不出审计与发射汇总
#[test]
fn lib_crate_and_precheck_only() {
    let Some(home) = resolve::jdk::find_java_home(Some(21)) else { return };
    let work = std::env::temp_dir().join(format!("rava-it-libjar-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&work);
    let classes = work.join("classes");
    let src = fixture("lib/greet/Greeter.java");
    assert!(Command::new(home.join("bin/javac")).arg("-d").arg(&classes).arg(&src).status().unwrap().success());
    let jar = work.join("greet.jar");
    assert!(Command::new(home.join("bin/jar")).arg("cf").arg(&jar).arg("-C").arg(&classes).arg(".").status().unwrap().success());
    let spec = format!("greet={}", jar.display());
    let Some((stdout, out)) = build("LibUser.java", "lib", &["--lib", &spec, "--precheck-only"]) else { return };
    assert!(stdout.contains("[jar] greet ← greet.jar（1 类）"), "{stdout}");
    assert!(stdout.contains("[precheck] native-missing="), "预检行");
    assert!(!stdout.contains("[equiv-audit]") && !stdout.contains("[emit]"), "--precheck-only 不出审计 / 汇总：{stdout}");
    let lib_cargo = std::fs::read_to_string(out.join("greet/Cargo.toml")).unwrap();
    assert!(lib_cargo.contains("crate-type = [\"lib\"]"), "{lib_cargo}");
    assert!(std::fs::read_to_string(out.join("greet/src/lib.rs")).unwrap().contains("pub mod greet;"));
    let greeter = std::fs::read_to_string(out.join("greet/src/greet/greeter.rs")).unwrap();
    assert!(greeter.contains("pub fn greet(&self"), "public 方法跨 crate 可见");
    assert!(greeter.contains("pub(crate) fn prefix("), "包可见方法降级：{greeter}");
    let user_cargo = std::fs::read_to_string(out.join("user/Cargo.toml")).unwrap();
    assert!(user_cargo.contains("greet           = { path = \"../greet\" }"), "{user_cargo}");
    let ws = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
    assert!(ws.contains("\"greet\""), "workspace 成员：{ws}");
    assert!(rs_text(&out.join("user/src")).contains("greet::greet::Greeter"), "user 经 lib crate 名引用");
    std::fs::remove_dir_all(&out).ok();
    std::fs::remove_dir_all(&work).ok();
}

/// try/finally 内 `return e;` 的返回值暂存：各臂存储同槽、类型不一（汇合为根类）时，已是汇合类型
/// 那一臂的存储不得丢失（回归：DeepCopy `ObjectInputStream.readObject0` 的 E0381）
#[test]
fn try_finally_return_temp_kept_in_every_arm() {
    let Some((_, out)) = build("TryFinallyReturn.java", "try-finally-return", &[]) else { return };
    let rs = std::fs::read_to_string(out.join("user/src/try_finally_return.rs")).unwrap();
    let body: Vec<&str> = rs.lines().skip_while(|l| !l.contains("pub fn pick(")).take_while(|l| !l.contains("pub fn main(")).collect();
    let arm1 = body.iter().position(|l| l.trim() == "1 => {").expect("case 1 臂");
    assert!(body[arm1 + 1].contains("Self::a()?"), "{}", body.join("\n"));
    assert_eq!(body[arm1 + 2].trim(), "local_1 = Clone::clone(&_t1);", "{}", body.join("\n"));
    assert_eq!(body.iter().filter(|l| l.trim_start().starts_with("local_1 = ")).count(), 3, "三个臂都存储返回值");
    std::fs::remove_dir_all(&out).ok();
}
