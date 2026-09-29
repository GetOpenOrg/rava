//! `rava closure <Test.java | 类目录>`：精确闭包分析（计划 docs/plans/2026-09-29-rust-closure-analyzer.md）。
//!
//! 选项：`--jdk N | --java-home P`、`--runtime <runtime/java_runtime>`、`--main <类>`、
//! `-o <closure.json>`、`--why <类 | 类.方法:描述符>`（可多次）、`--flows <方法标签片段>`（类型流诊断，可多次）、`--report <报告.md>`。

use std::path::{Path, PathBuf};

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::Args;

const MAIN: (&str, &str) = ("main", "([Ljava/lang/String;)V");

fn runtime_dir(args: &Args) -> Result<PathBuf, String> {
    if let Some(p) = args.opt("--runtime") {
        return Ok(PathBuf::from(p));
    }
    let mut cur = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let cand = cur.join("runtime/java_runtime");
        if cand.join("closure.toml").is_file() {
            return Ok(cand);
        }
        if !cur.pop() {
            break;
        }
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime");
    if fallback.join("closure.toml").is_file() {
        return Ok(fallback);
    }
    Err("找不到 runtime/java_runtime（用 --runtime 指定）".into())
}

/// .java → javac 编译到临时目录；目录原样返回
fn user_classes(input: &Path, home: &Path) -> Result<PathBuf, String> {
    if input.is_dir() {
        return Ok(input.to_path_buf());
    }
    let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("input");
    let out = std::env::temp_dir().join(format!("rava-closure-{stem}-{}", std::process::id()));
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let st = std::process::Command::new(home.join("bin/javac"))
        .arg("-g")
        .arg("-d")
        .arg(&out)
        .arg(input)
        .status()
        .map_err(|e| format!("javac：{e}"))?;
    if !st.success() {
        return Err(format!("javac 编译失败：{}", input.display()));
    }
    Ok(out)
}

pub fn run(args: &Args) -> Result<(), String> {
    let input = args.rest.first().filter(|a| !a.starts_with('-')).ok_or("缺少输入（.java 文件或类目录）")?;
    let home = crate::java_home(args)?;
    let rt = runtime_dir(args)?;
    let classes = user_classes(Path::new(input), &home)?;

    let mut cp = ClassPath::new();
    cp.add(Origin::User, &classes).map_err(|e| e.to_string())?;
    cp.add_jdk(&home).map_err(|e| e.to_string())?;

    let users = cp.names_of(Origin::User);
    let main = match args.opt("--main") {
        Some(m) => m.replace('.', "/"),
        None => users
            .iter()
            .find(|n| cp.get(n).is_some_and(|c| c.method(MAIN.0, MAIN.1).is_some_and(|m| m.is_static())))
            .cloned()
            .ok_or("用户类中没有 static main(String[])")?,
    };

    let man = Manifest::load(&rt)?;
    let hw = Handwritten::new(&rt);
    let h = Hierarchy::new(&cp);
    let input_desc = closure::Input {
        cp: &cp,
        runtime_dir: &rt,
        roots: vec![MemberRef { owner: main.clone(), name: MAIN.0.into(), desc: MAIN.1.into() }],
    };
    let c = closure::analyze(&input_desc, &h, &man, &hw);

    for e in hw.errors.borrow().iter() {
        eprintln!("[closure] 手写文件解析失败：{e}");
    }
    for (n, e) in cp.failures() {
        eprintln!("[closure] 类解析失败：{n}：{e}");
    }

    if let Some(o) = args.opt("-o") {
        let s = serde_json::to_string_pretty(&c.to_json()).map_err(|e| e.to_string())?;
        std::fs::write(&o, s).map_err(|e| format!("{o}：{e}"))?;
    }
    if let Some(r) = args.opt("--report") {
        std::fs::write(&r, c.report_md(&main)).map_err(|e| format!("{r}：{e}"))?;
    }
    let multi = |flag: &str| -> Vec<&String> {
        args.rest.iter().zip(args.rest.iter().skip(1)).filter(|(a, _)| *a == flag).map(|(_, v)| v).collect()
    };
    for w in multi("--why") {
        for line in c.why(w) {
            println!("{line}");
        }
        println!();
    }
    for w in multi("--flows") {
        for line in c.flows(w) {
            println!("{line}");
        }
        println!();
    }
    println!("{}", serde_json::to_string_pretty(&c.summary()).map_err(|e| e.to_string())?);
    Ok(())
}
