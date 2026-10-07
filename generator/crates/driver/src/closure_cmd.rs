//! `rava closure <Test.java | 类目录>`：精确闭包分析（计划 docs/plans/2026-09-29-rust-closure-analyzer.md）。
//!
//! 选项：`--jdk N | --java-home P`、`--runtime <runtime/java_runtime>`、`--main <类>`、
//! `-o <closure.json>`、`--why <类 | 类.方法:描述符>`（可多次）、`--flows <方法标签片段 | @查询>`（类型流诊断，可多次；
//! `@grow:` / `@trace:` / `@edge:` 为记录型，分析前登记、传播中记录，见 `closure/src/engine/diag.rs`）、`--report <报告.md>`、
//! `--release <包前缀/ | 类>`（分析期视同 `[release]` 放行，可多次；C1d 放行实测）、
//! `--release-bytecode <包前缀/ | 类>`（放行并模拟删除其中按精确名提供的共置手写，可多次）；
//! 转译接入（均可多次）：`--deps <deps.lock.toml>` + `--cp <锁条目名>[,…]`（依赖库）、`--image <目录>`（镜像独有 / VM 支持类；缺省与 `rava build` 同源派生，
//! 见 [`crate::build_cmd::image_dirs`]）、
//! `--root <类.方法:描述符>`（外部种子方法）、`--seed-class <类>`（lib 公开 API 面：全部 public 方法入链，main 除外）、
//! `--locale <标签>`（locale 资源束种子）；
//! 诊断（缺省关闭，不影响结果）：`--cut <类.方法:描述符[@偏移]>`（反事实切除，可多次）、`--cut-file <文件>`（每行一条，`#` 注释）、
//! `--dump-edges <文件>`（触发边转储）、`--site-prof`（读者站点重跑剖析，进 `summary.perf.site_prof`）；`--cold-cut`（丢弃冷路径事件，测量冷路径独占规模，结果不健全）。
//! 顺序无关检验：`--flow-batch N`（流传播批量，缺省 64，1 = 逐个排空）、`--hash-seed N`（内部表哈希初值，缺省 0）；
//! 引导映像：`--boot-report <报告.md>`（写构建期引导映像审计报告；求值失败即命令失败，`rava audit boot` 用）。
//! 门自动排名：`--gates`（见 [`crate::gates_cmd`]；`--gates-top N`、`--gates-pool N`、`--gates-verify N`、`--gates-jobs N`、
//! `--gates-mem-mb N`、`--gates-timeout 秒`、`--gates-out <json>`、`--gates-md <md>`；`--gates-child` 为内部子进程模式）。
//! 跨运行结果缓存：`--closure-cache <目录>`、`--closure-cache-max-mb N`（缺省 4096；`--why` / `--flows` / `--report` 时不读缓存）。
//!
//! 参数逐个校验：未知参数、多余的位置参数一律报错。闭包结果取决于输入（类路径、镜像目录），静默忽略的参数会
//! 让同一用例得出不同闭包——例如 zsh 不对未加引号的 `$IMGS` 分词，`--image A --image B` 作为单个参数传入时
//! 镜像目录全部丢失，闭包少掉镜像独有 / VM 支持类。

use std::path::{Path, PathBuf};

use classfile::MemberRef;
use closure::handwritten::Handwritten;
use closure::manifest::Manifest;
use resolve::{ClassPath, Hierarchy, Origin};

use crate::Args;

pub(crate) const MAIN: (&str, &str) = ("main", "([Ljava/lang/String;)V");

/// 带值选项（后随一个参数）
const VALUE_OPTS: &[&str] = &[
    "--jdk", "--java-home", "--runtime", "--main", "-o", "--why", "--flows", "--report", "--release", "--release-bytecode",
    "--deps",
    "--cp", "--image", "--root", "--seed-class", "--locale", "--cut", "--cut-file", "--dump-edges", "--flow-batch",
    "--hash-seed", "--closure-cache", "--closure-cache-max-mb", "--boot-report",
];
/// 开关选项
const FLAG_OPTS: &[&str] = &["--cold-cut", "--site-prof", "--gates"];

/// 参数校验：恰一个位置参数（输入），其余都是已知选项（带值选项须有值）
fn check_args(rest: &[String]) -> Result<(), String> {
    let mut input = None;
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        if VALUE_OPTS.contains(&a.as_str()) || crate::gates_cmd::VALUE_OPTS.contains(&a.as_str()) {
            it.next().ok_or_else(|| format!("{a} 缺少值"))?;
        } else if FLAG_OPTS.contains(&a.as_str()) {
        } else if a.starts_with('-') {
            return Err(format!("未知参数：{a:?}"));
        } else if let Some(i) = input.replace(a) {
            return Err(format!("多余的位置参数：{a:?}（输入已是 {i:?}）"));
        }
    }
    input.map(|_| ()).ok_or_else(|| "缺少输入（.java 文件或类目录）".into())
}

fn runtime_dir(args: &Args) -> Result<PathBuf, String> {
    find_runtime_dir(args.opt("--runtime").map(PathBuf::from))
}

/// 手写层真源 `runtime/java_runtime`：显式路径优先，否则自可执行文件所在目录向上找（二进制在
/// `<仓库>/build/analyzer-target/release/` 下，与 cwd 无关），再自当前目录向上找，最后取编译本二进制的仓库
pub(crate) fn find_runtime_dir(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return Ok(p);
    }
    let exe_dir = std::env::current_exe().ok().and_then(|e| e.canonicalize().ok()).and_then(|e| e.parent().map(Path::to_path_buf));
    let cwd = std::env::current_dir().ok();
    for start in exe_dir.into_iter().chain(cwd) {
        if let Some(found) = start.ancestors().map(|d| d.join("runtime/java_runtime")).find(|c| c.join("closure.toml").is_file()) {
            return Ok(found);
        }
    }
    let fallback = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime");
    if fallback.join("closure.toml").is_file() {
        return Ok(fallback);
    }
    Err("找不到 runtime/java_runtime（用 --runtime 指定）".into())
}

/// 诊断选项：`--cut` 条目 + `--cut-file` 文件逐行条目（空行 / `#` 注释跳过）+ `--dump-edges` 路径
pub(crate) fn diag_opts<S: AsRef<str>>(cuts: &[S], cut_files: &[S], dump_edges: Option<String>) -> Result<closure::engine::Diag, String> {
    let mut all: Vec<String> = cuts.iter().map(|c| c.as_ref().to_string()).collect();
    for f in cut_files {
        let f = f.as_ref();
        let text = std::fs::read_to_string(f).map_err(|e| format!("--cut-file {f}：{e}"))?;
        all.extend(text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).map(String::from));
    }
    Ok(closure::engine::Diag { cuts: all, dump_edges: dump_edges.map(PathBuf::from), flows: Vec::new(), site_prof: false, keep_edges: false })
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
    check_args(&args.rest)?;
    let input = args.rest.first().filter(|a| !a.starts_with('-')).ok_or("输入（.java 文件或类目录）须为第一个参数")?;
    let home = crate::java_home(args)?;
    let rt = runtime_dir(args)?;
    let classes = user_classes(Path::new(input), &home)?;

    let multi = |flag: &str| -> Vec<&String> {
        args.rest.iter().zip(args.rest.iter().skip(1)).filter(|(a, _)| *a == flag).map(|(_, v)| v).collect()
    };
    // 依赖锁：--deps + --cp（库输入唯一来源，--lib 已删除）
    let libs_sel = match args.opt("--deps") {
        Some(d) => {
            let names: Vec<String> = args
                .opt("--cp")
                .map(|c| c.split(',').map(str::trim).filter(|n| !n.is_empty()).map(String::from).collect())
                .ok_or_else(|| "--deps 须配合 --cp 给出入口类路径（锁条目名）".to_string())?;
            let lock = crate::deps_lock::DepsLock::load(Path::new(d.as_str()))?;
            lock.select(&names)?
                .iter()
                .map(|j| crate::build_libs::LibEntry {
                    path: j.path.clone(),
                    meta: resolve::classpath::LibMeta {
                        coordinate: j.coordinate.clone(),
                        module: j.module.clone(),
                        sha256: Some(j.sha256.clone()),
                    },
                })
                .collect()
        }
        None => {
            if args.opt("--cp").is_some() {
                return Err("--cp 须配合 --deps（库输入唯一来源是依赖锁）".into());
            }
            Vec::new()
        }
    };
    // 同名类先加入者优先：用户 → 依赖库 → JDK → 镜像独有 / VM 支持类；随后 JDK 包遮蔽 + 模块图硬校验
    let release = resolve::jdk::major_of(&home).ok_or(format!("{}：无法识别 JDK 主版本", home.display()))?;
    let mut cp = ClassPath::new(release);
    cp.add(Origin::User, &classes).map_err(|e| e.to_string())?;
    for e in &libs_sel {
        cp.set_lib_meta(&e.path, e.meta.clone());
        cp.add(Origin::Lib, &e.path).map_err(|err| format!("{}：{err}", e.path.display()))?;
    }
    cp.add_jdk(&home).map_err(|e| e.to_string())?;
    let images: Vec<PathBuf> = multi("--image").into_iter().map(PathBuf::from).collect();
    let images = if images.is_empty() { resolve::image::image_class_dirs(&home, &crate::build_cmd::support_root(&rt)) } else { images };
    for d in &images {
        cp.add(Origin::Image, d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    cp.shadow_jdk_owned_packages();
    resolve::modules::check(&cp).map_err(|e| format!("[modules] {e}"))?;

    let flows = multi("--flows");
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
        diag: closure::engine::Diag {
            flows: flows.iter().map(|f| f.to_string()).collect(),
            site_prof: args.rest.iter().any(|a| a == "--site-prof"),
            ..diag_opts(&multi("--cut"), &multi("--cut-file"), args.opt("--dump-edges"))? },
        cold_cut: args.rest.iter().any(|a| a == "--cold-cut"),
        flow_batch: num("--flow-batch")?.map(|n| n as usize),
    };
    if let Some(out) = args.opt("--gates-child") {
        return crate::gates_cmd::child(&input_desc, &h, &man, &hw, &out);
    }
    if args.rest.iter().any(|a| a == "--gates") {
        let diag = ["--why", "--flows", "--report", "--boot-report", "--dump-edges", "--site-prof"];
        if let Some(a) = args.rest.iter().find(|a| diag.contains(&a.as_str())) {
            return Err(format!("--gates 不与 {a} 同用（门排名自带溯源链；诊断请单独运行）"));
        }
        return crate::gates_cmd::run(args, input_desc, &h, &man, &hw, &crate::gates_cmd::abs(&classes), &main);
    }
    let whys = multi("--why");
    let boot_report = args.opt("--boot-report");
    let need_engine = args.opt("--report").is_some() || boot_report.is_some() || !whys.is_empty() || !flows.is_empty();
    let cache = crate::closure_run::CacheOpts {
        dir: args.opt("--closure-cache").map(PathBuf::from),
        max_mb: num("--closure-cache-max-mb")?,
    };
    let out = match crate::closure_run::analyze(&cache, &input_desc, &h, &man, &hw, need_engine, true) {
        Ok(o) => o,
        Err(f) => {
            if let Some(r) = &boot_report {
                write_report(r, &f.report)?;
                eprintln!("[boot] 报告 {r}：失败");
            }
            return Err(f.to_string());
        }
    };
    let v = out.json.as_ref().ok_or("闭包产物缺失")?;
    if let Some(o) = args.opt("-o") {
        let s = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
        std::fs::write(&o, s).map_err(|e| format!("{o}：{e}"))?;
    }
    let Some(c) = out.closure else {
        println!("{}", serde_json::to_string_pretty(&v["summary"]).map_err(|e| e.to_string())?);
        return Ok(());
    };
    if let Some(r) = args.opt("--report") {
        std::fs::write(&r, c.report_md(&main)).map_err(|e| format!("{r}：{e}"))?;
    }
    if let Some(r) = &boot_report {
        let b = &c.boot_image;
        let outside = |v: &[String]| -> Vec<String> { v.iter().filter(|t| !c.engine.classes.contains_key(t.as_str())).map(|t| format!("`{t}`")).collect() };
        let (ti, ri) = (outside(&b.types), outside(&b.runtime_classes));
        let md = format!("{}
## 闭包

- 入口 `{main}`：闭包 {} 类；引导映像求值 {} ms
- 映像类型 {} 个，不在闭包 {} 个：{}
- 运行期部分入口类 {} 个，不在闭包 {} 个：{}
", b.report, c.engine.classes.len(), c.boot_ms, b.types.len(), ti.len(), ti.join(" "), b.runtime_classes.len(), ri.len(), ri.join(" "));
        write_report(r, &md)?;
        eprintln!("[boot] 报告 {r}：通过，摘要 {}", b.digest);
    }
    for w in whys {
        for line in c.why(w) {
            println!("{line}");
        }
        println!();
    }
    for w in flows {
        for line in c.flows(w) {
            println!("{line}");
        }
        println!();
    }
    println!("{}", serde_json::to_string_pretty(&v["summary"]).map_err(|e| e.to_string())?);
    Ok(())
}

/// 写引导映像审计报告（建上级目录）
fn write_report(r: &str, md: &str) -> Result<(), String> {
    if let Some(d) = Path::new(r).parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
    }
    std::fs::write(r, md).map_err(|e| format!("{r}：{e}"))
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

#[cfg(test)]
mod tests {
    use super::check_args;

    fn v(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn args_checked() {
        assert!(check_args(&v(&["A.java", "--image", "/i1", "--image", "/i2", "--cold-cut", "-o", "o.json"])).is_ok());
        // 未分词的镜像参数串（zsh 未加引号的 $IMGS）不得静默忽略
        assert!(check_args(&v(&["A.java", "--image /i1 --image /i2 "])).unwrap_err().contains("未知参数"));
        assert!(check_args(&v(&["A.java", "B.java"])).unwrap_err().contains("多余的位置参数"));
        assert!(check_args(&v(&["A.java", "--why"])).unwrap_err().contains("缺少值"));
        assert!(check_args(&v(&["--why", "X"])).is_err());
    }
}
