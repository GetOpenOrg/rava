//! 预定义类：构建输入通道与训练运行（docs/plans/2026-10-10-xsltc-translet.md §4–§5，X1 = S2 训练运行）。
//!
//! 预定义类目录（`rava trace` / `rava build --train-predefined` 的产物，`rava build --predefined` 的输入）：
//! - `classes/`：类目录（`<binary name>.class`，字节即训练运行中被定义的原样字节），以 `Origin::Predefined` 加入类路径；
//! - `predefined.toml`：索引——每个类的 SHA-256 与定义次数、同名不同内容的冲突、被剔除的记录计数（只供查看）。
//!
//! 训练运行：参考 JDK 以记录代理（`scripts/dyn_agent/define_record.c`）运行程序，代理把非引导加载器定义的类文件
//! 逐个落盘（`raw/<序号>.class`），[`normalize`] 剔除类路径上已有的类（内建加载器从类路径定义的类）、解析失败的字节
//! （定义随后失败的非法类文件），按名归并成上述目录。清单里 VM 另行承载的类定义点（动态代理等）经代理的 exclude
//! 文件排除。语料构建在闭包发现类定义 native 可达时按需触发（[`needs_training`]），产物写 scratch；
//! 生产构建由用户显式执行 `rava trace`，产物归用户项目。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use closure::manifest::Manifest;
use input::ClosureFacts;
use resolve::{ClassPath, Origin};

/// scratch 内的训练产物目录（语料构建）
pub const SCRATCH_DIR: &str = "predefined";
const CLASSES: &str = "classes";
const INDEX: &str = "predefined.toml";
const RAW: &str = "raw";
const STAMP: &str = "train.stamp";
const LOG: &str = "train.log";
const EXCLUDE: &str = "exclude.txt";
/// 记录代理源码（仓库相对）
const AGENT_SRC: &str = "scripts/dyn_agent/define_record.c";
/// 训练运行超时：语料程序为秒级；超时即终止，已记录的类照常归并（索引注明）
const TRAIN_TIMEOUT: Duration = Duration::from_secs(600);

/// 预定义类目录里的类目录（存在且非空时）
pub fn classes_dir(dir: &Path) -> Option<PathBuf> {
    let d = dir.join(CLASSES);
    let non_empty = std::fs::read_dir(&d).ok()?.next().is_some();
    non_empty.then_some(d)
}

/// 闭包调用链上是否有类定义 native（训练运行的触发条件；`definers` 取清单 `predefined_definers`）
pub fn needs_training(facts: &ClosureFacts, definers: &[String]) -> bool {
    facts.methods.iter().any(|m| definers.contains(&m.id.to_string()))
}

/// 训练产物所依赖的输入（新鲜度戳的内容）
pub struct Inputs<'a> {
    pub home: &'a Path,
    pub repo: &'a Path,
    /// 用户类目录（javac 产物）
    pub classes: &'a Path,
    pub jars: &'a [PathBuf],
    /// 显式入口类（`--main`）
    pub main: Option<&'a str>,
}

/// 一次训练运行
pub struct Train<'a> {
    pub inputs: &'a Inputs<'a>,
    /// 入口类（`/` 分隔）
    pub main: &'a str,
    /// 产物目录（预定义类目录）
    pub out: &'a Path,
    /// 类路径（剔除其上已有的类；其中的预定义类不算）
    pub cp: &'a ClassPath,
    pub man: &'a Manifest,
}

/// 训练产物的新鲜度戳：用户类与库 jar 的内容、记录代理源码、参考 JDK、显式入口类
pub fn stamp(t: &Inputs<'_>) -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::new();
    for (rel, bytes) in tree_files(t.classes)? {
        buf.extend_from_slice(rel.as_bytes());
        buf.push(0);
        buf.extend_from_slice(&classfile::sha256::digest(&bytes));
    }
    for j in t.jars {
        let b = std::fs::read(j).map_err(|e| format!("{}：{e}", j.display()))?;
        buf.extend_from_slice(&classfile::sha256::digest(&b));
    }
    let agent = t.repo.join(AGENT_SRC);
    buf.extend_from_slice(&std::fs::read(&agent).map_err(|e| format!("{}：{e}", agent.display()))?);
    buf.extend_from_slice(t.home.display().to_string().as_bytes());
    buf.extend_from_slice(t.main.unwrap_or_default().as_bytes());
    Ok(classfile::sha256::hex(&buf))
}

/// 目录里的训练产物与输入相符（戳一致）
pub fn fresh(dir: &Path, t: &Inputs<'_>) -> bool {
    let Ok(s) = stamp(t) else { return false };
    std::fs::read_to_string(dir.join(STAMP)).is_ok_and(|x| x.trim() == s)
}

/// `rava trace <A.java>… --predefined <目录> [build 的 javac / 类路径选项]`：生产构建的显式训练——
/// javac 后无条件跑一次训练运行，产物写 `--predefined` 指定的目录（归用户项目，随后的 `rava build --predefined` 读取）
pub fn run_trace(args: &crate::Args) -> Result<(), String> {
    use crate::build_cmd::{abs, class_path, image_dirs, javac, repo_root, user_order};
    use crate::build_opts::{BuildOpts, Mode, CLOSURE_INPUT_DIR};
    let o = BuildOpts::parse(Mode::Build, &args.rest)?;
    let out = abs(o.predefined.as_deref().ok_or("rava trace 需要 --predefined <产物目录>")?);
    let rt = abs(&crate::closure_cmd::find_runtime_dir(o.runtime.clone())?);
    let repo = repo_root(&rt);
    let jdk = resolve::jdk::choose(o.jdk, o.java_home.as_deref(), Some(&repo))?;
    println!("{}", jdk.describe());
    let home = jdk.home;
    let classes = abs(&o.scratch_dir(Mode::Build, &repo)?).join(CLOSURE_INPUT_DIR).join("classes");
    let libs_sel = crate::build_libs::select_entries(&o)?;
    let jars: Vec<PathBuf> = libs_sel.iter().map(|e| e.path.clone()).collect();
    javac(&home, &o.inputs, &jars, &classes)?;
    let cp = class_path(&classes, &libs_sel, &home, &image_dirs(&o, &home, &rt), None)?;
    let user = user_order(&cp, o.java_files(Mode::Build), o.main.as_deref())?;
    let man = Manifest::load(&rt)?;
    let inputs = Inputs { home: &home, repo: &repo, classes: &classes, jars: &jars, main: o.main.as_deref() };
    let t = Train { inputs: &inputs, main: &user[0], out: &out, cp: &cp, man: &man };
    println!("[predefined] {}", train(&t)?);
    println!("[predefined] 产物：{}", out.display());
    Ok(())
}

/// 训练运行 + 归并；返回归并摘要（一行）
pub fn train(t: &Train<'_>) -> Result<String, String> {
    if t.out.is_dir() {
        std::fs::remove_dir_all(t.out).map_err(|e| format!("{}：{e}", t.out.display()))?;
    }
    let raw = t.out.join(RAW);
    std::fs::create_dir_all(&raw).map_err(|e| format!("{}：{e}", raw.display()))?;
    let (home, repo) = (t.inputs.home, t.inputs.repo);
    let lib = ensure_agent(home, repo)?;
    let exclude = t.out.join(EXCLUDE);
    let mut ex = t.man.class_definitions().join("\n");
    ex.push('\n');
    std::fs::write(&exclude, ex).map_err(|e| format!("{}：{e}", exclude.display()))?;
    let mut cpath = vec![t.inputs.classes.display().to_string()];
    cpath.extend(t.inputs.jars.iter().map(|j| j.display().to_string()));
    let log_path = t.out.join(LOG);
    let log = std::fs::File::create(&log_path).map_err(|e| format!("{}：{e}", log_path.display()))?;
    let log2 = log.try_clone().map_err(|e| e.to_string())?;
    let mut child = Command::new(home.join("bin/java"))
        .arg("-Xshare:off")
        .arg(format!("-agentpath:{}={},exclude={}", lib.display(), raw.display(), exclude.display()))
        .arg("-cp")
        .arg(cpath.join(":"))
        .arg(t.main.replace('/', "."))
        .stdin(Stdio::null())
        .stdout(log)
        .stderr(log2)
        .spawn()
        .map_err(|e| format!("训练运行启动失败：{e}"))?;
    let t0 = Instant::now();
    let status = loop {
        match child.try_wait().map_err(|e| e.to_string())? {
            Some(s) => break format!("exit {}", s.code().map_or_else(|| "signal".to_string(), |c| c.to_string())),
            None if t0.elapsed() > TRAIN_TIMEOUT => {
                let _ = child.kill();
                let _ = child.wait();
                break format!("timeout {}s", TRAIN_TIMEOUT.as_secs());
            }
            None => std::thread::sleep(Duration::from_millis(50)),
        }
    };
    let summary = normalize(&raw, t.out, t.cp, &status)?;
    std::fs::write(t.out.join(STAMP), stamp(t.inputs)?).map_err(|e| format!("{}：{e}", t.out.display()))?;
    Ok(format!("训练运行 {status}（{:.1}s）：{summary}", t0.elapsed().as_secs_f64()))
}

/// 一个被记录的内容
struct Content {
    sha: String,
    bytes: Vec<u8>,
    definitions: usize,
}

/// 代理记录（`raw/<序号>.class`）→ 预定义类目录：剔除类路径上已有的类与解析失败的字节，按名归并；
/// 同名不同内容的类不写入类目录（索引 `[[conflict]]` 列出，运行期未命中时报出期望哈希）
pub fn normalize(raw: &Path, out: &Path, cp: &ClassPath, status: &str) -> Result<String, String> {
    let mut records: Vec<(u64, PathBuf)> = std::fs::read_dir(raw)
        .map_err(|e| format!("{}：{e}", raw.display()))?
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            let n = p.file_stem()?.to_str()?.parse::<u64>().ok()?;
            Some((n, p))
        })
        .collect();
    records.sort();
    let (mut on_class_path, mut unparsable) = (0usize, 0usize);
    let mut by_name: BTreeMap<String, Vec<Content>> = BTreeMap::new();
    for (_, p) in &records {
        let bytes = std::fs::read(p).map_err(|e| format!("{}：{e}", p.display()))?;
        let Ok(cf) = classfile::parse(&bytes) else {
            unparsable += 1;
            continue;
        };
        if cp.origin(&cf.name).is_some_and(|o| o != Origin::Predefined) {
            on_class_path += 1;
            continue;
        }
        let sha = classfile::sha256::hex(&bytes);
        let list = by_name.entry(cf.name.clone()).or_default();
        match list.iter_mut().find(|c| c.sha == sha) {
            Some(c) => c.definitions += 1,
            None => list.push(Content { sha, bytes, definitions: 1 }),
        }
    }
    let classes = out.join(CLASSES);
    std::fs::create_dir_all(&classes).map_err(|e| format!("{}：{e}", classes.display()))?;
    let mut index = String::from(
        "# 预定义类索引（rava 训练运行产物；docs/plans/2026-10-10-xsltc-translet.md）。classes/ 下为原样类文件\n",
    );
    index.push_str(&format!("status = \"{status}\"\nrecorded = {}\non_class_path = {on_class_path}\nunparsable = {unparsable}\n", records.len()));
    let (mut written, mut conflicts) = (0usize, 0usize);
    for (name, list) in &by_name {
        if let [c] = list.as_slice() {
            let f = classes.join(format!("{name}.class"));
            if let Some(d) = f.parent() {
                std::fs::create_dir_all(d).map_err(|e| format!("{}：{e}", d.display()))?;
            }
            std::fs::write(&f, &c.bytes).map_err(|e| format!("{}：{e}", f.display()))?;
            index.push_str(&format!("\n[[class]]\nname = \"{name}\"\nsha256 = \"{}\"\ndefinitions = {}\n", c.sha, c.definitions));
            written += 1;
        } else {
            let shas: Vec<String> = list.iter().map(|c| format!("\"{}\"", c.sha)).collect();
            index.push_str(&format!("\n[[conflict]]\nname = \"{name}\"\nsha256 = [{}]\n", shas.join(", ")));
            conflicts += 1;
        }
    }
    std::fs::write(out.join(INDEX), index).map_err(|e| format!("{}：{e}", out.display()))?;
    let raw_dir = raw.to_path_buf();
    std::fs::remove_dir_all(&raw_dir).map_err(|e| format!("{}：{e}", raw_dir.display()))?;
    Ok(format!(
        "记录 {} 个定义 → 预定义类 {written} 个，同名不同内容 {conflicts} 个（未纳入），类路径已有 {on_class_path}，非法字节 {unparsable}",
        records.len()
    ))
}

/// 编译记录代理（按源码 + JDK 取哈希缓存于 `<仓库>/build/define_record/<哈希>/`；临时文件构建后原子改名）
fn ensure_agent(home: &Path, repo: &Path) -> Result<PathBuf, String> {
    let src = repo.join(AGENT_SRC);
    let code = std::fs::read(&src).map_err(|e| format!("{}：{e}", src.display()))?;
    let mut key = code.clone();
    key.extend_from_slice(home.display().to_string().as_bytes());
    let name = if cfg!(target_os = "macos") { "libdefine_record.dylib" } else { "libdefine_record.so" };
    let dir = repo.join("build").join("define_record").join(&classfile::sha256::hex(&key)[..12]);
    let lib = dir.join(name);
    if lib.exists() {
        return Ok(lib);
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}：{e}", dir.display()))?;
    let inc = home.join("include");
    let mut cmd = Command::new(std::env::var("CC").unwrap_or_else(|_| "cc".into()));
    cmd.args(["-shared", "-fPIC", "-O1"]).arg(format!("-I{}", inc.display()));
    if let Ok(rd) = std::fs::read_dir(&inc) {
        for d in rd.flatten().filter(|e| e.path().is_dir()) {
            cmd.arg(format!("-I{}", d.path().display()));
        }
    }
    let part = dir.join(format!("{name}.{}.part", std::process::id()));
    let o = cmd.arg("-o").arg(&part).arg(&src).output().map_err(|e| format!("记录代理编译：{e}"))?;
    if !o.status.success() {
        let _ = std::fs::remove_file(&part);
        return Err(format!("记录代理编译失败：{}", String::from_utf8_lossy(&o.stderr)));
    }
    std::fs::rename(&part, &lib).map_err(|e| format!("{}：{e}", lib.display()))?;
    Ok(lib)
}

/// 目录树全部文件（相对路径, 字节），按相对路径排序
fn tree_files(root: &Path) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).map_err(|e| format!("{}：{e}", d.display()))?.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else {
                let rel = p.strip_prefix(root).unwrap_or(&p).display().to_string();
                out.push((rel, std::fs::read(&p).map_err(|e| format!("{}：{e}", p.display()))?));
            }
        }
    }
    out.sort();
    Ok(out)
}
