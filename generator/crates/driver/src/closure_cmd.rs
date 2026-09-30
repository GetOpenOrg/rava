//! `rava closure <Test.java | 类目录>`：精确闭包分析（计划 docs/plans/2026-09-29-rust-closure-analyzer.md）。
//!
//! 选项：`--jdk N | --java-home P`、`--runtime <runtime/java_runtime>`、`--main <类>`、
//! `-o <closure.json>`、`--why <类 | 类.方法:描述符>`（可多次）、`--flows <方法标签片段>`（类型流诊断，可多次）、`--report <报告.md>`、
//! `--release <包前缀/ | 类>`（分析期视同 `[release]` 放行，可多次；C1d 放行实测）、
//! `--release-bytecode <包前缀/ | 类>`（放行并模拟删除其中按精确名提供的共置手写，可多次）；
//! 转译接入（均可多次）：`--lib <jar>`（依赖库）、`--image <目录>`（镜像独有 / VM 支持类）、
//! `--root <类.方法:描述符>`（外部种子方法）、`--seed-class <类>`（lib 公开 API 面：全部 public 方法入链，main 除外）、
//! `--locale <标签>`（locale 资源束种子）；诊断 `--cold-cut`（丢弃冷路径事件，测量冷路径独占规模，结果不健全）；
//! 顺序无关检验：`--flow-batch N`（流传播批量，缺省 64，1 = 逐个排空）、`--hash-seed N`（内部表哈希初值，缺省 0）。

use std::path::{Path, PathBuf};

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::Args;

pub(crate) const MAIN: (&str, &str) = ("main", "([Ljava/lang/String;)V");

fn runtime_dir(args: &Args) -> Result<PathBuf, String> {
    find_runtime_dir(args.opt("--runtime").map(PathBuf::from))
}

/// 手写层真源 `runtime/java_runtime`：显式路径优先，否则自当前目录向上找，最后取本仓库
pub(crate) fn find_runtime_dir(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return Ok(p);
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
    let num = |k: &str| args.opt(k).map(|v| v.parse::<u64>().map_err(|_| format!("{k} 需为非负整数：{v}"))).transpose();
    if let Some(s) = num("--hash-seed")? {
        closure::engine::set_hash_seed(s);
    }
    let input = args.rest.first().filter(|a| !a.starts_with('-')).ok_or("缺少输入（.java 文件或类目录）")?;
    let home = crate::java_home(args)?;
    let rt = runtime_dir(args)?;
    let classes = user_classes(Path::new(input), &home)?;

    let multi = |flag: &str| -> Vec<&String> {
        args.rest.iter().zip(args.rest.iter().skip(1)).filter(|(a, _)| *a == flag).map(|(_, v)| v).collect()
    };
    // 同名类先加入者优先：用户 → 依赖库 → JDK → 镜像独有 / VM 支持类
    let mut cp = ClassPath::new();
    cp.add(Origin::User, &classes).map_err(|e| e.to_string())?;
    for jar in multi("--lib") {
        cp.add(Origin::Lib, Path::new(jar)).map_err(|e| format!("{jar}：{e}"))?;
    }
    cp.add_jdk(&home).map_err(|e| e.to_string())?;
    for d in multi("--image") {
        cp.add(Origin::Image, Path::new(d)).map_err(|e| format!("{d}：{e}"))?;
    }

    let users = cp.names_of(Origin::User);
    let main = match args.opt("--main") {
        Some(m) => m.replace('.', "/"),
        None => users
            .iter()
            .find(|n| cp.get(n).is_some_and(|c| c.method(MAIN.0, MAIN.1).is_some_and(|m| m.is_static())))
            .cloned()
            .ok_or("用户类中没有 static main(String[])")?,
    };

    let mut man = Manifest::load(&rt)?;
    man.release_more(multi("--release").into_iter().cloned());
    man.release_bytecode(multi("--release-bytecode").into_iter().cloned());
    let hw = Handwritten::new(&rt);
    let h = Hierarchy::new(&cp);
    let input_desc = closure::Input {
        cp: &cp,
        runtime_dir: &rt,
        roots: vec![MemberRef { owner: main.clone(), name: MAIN.0.into(), desc: MAIN.1.into() }],
        seed_roots: seed_roots(&cp, &multi("--root"), &multi("--seed-class"))?,
        locales: multi("--locale").into_iter().cloned().collect(),
        cold_cut: args.rest.iter().any(|a| a == "--cold-cut"),
        flow_batch: num("--flow-batch")?.map(|n| n as usize),
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

/// `--root 类.方法:描述符` 与 `--seed-class 类`（全部 public 方法，命令行入口 main 除外）展开为种子方法
pub(crate) fn seed_roots(cp: &ClassPath, roots: &[&String], classes: &[&String]) -> Result<Vec<MemberRef>, String> {
    let mut out = Vec::new();
    for r in roots {
        let (head, desc) = r.split_once(':').ok_or_else(|| format!("--root 格式应为 类.方法:描述符：{r}"))?;
        let (owner, name) = head.rsplit_once('.').ok_or_else(|| format!("--root 格式应为 类.方法:描述符：{r}"))?;
        out.push(MemberRef { owner: owner.replace('.', "/"), name: name.into(), desc: desc.into() });
    }
    for c in classes {
        let c = c.replace('.', "/");
        let Some(cf) = cp.get(&c) else {
            eprintln!("[closure] --seed-class 未命中类路径：{c}");
            continue;
        };
        for m in cf.methods.iter().filter(|m| m.access & classfile::acc::PUBLIC != 0 && !(m.is_static() && (m.name.as_str(), m.desc.as_str()) == MAIN)) {
            out.push(MemberRef { owner: c.clone(), name: m.name.clone(), desc: m.desc.clone() });
        }
    }
    Ok(out)
}
