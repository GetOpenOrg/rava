//! rava：Rust 生成器入口。当前子命令：
//! - `closure`：精确闭包分析（XTA + 抽象解释 + 手写层 syn 扫描），输出 closure.json / 溯源 / 报告
//! - `build`：javac → 闭包 → 发射 scratch → cargo 编译 → 运行（`--stop-after` 截停）
//! - `compile`：编译已发射的 scratch（`build --stop-after emit` 之后；批量编排的编译段，见 [`compile_cmd`]）
//! - `prune`：运行完成后删除 scratch 登记的剩余编译产物（可执行文件，见 [`artifacts`]）
//! - `emit`：既有 closure.json → 发射 scratch
//! - `image-dirs`：镜像独有 / VM 支持类目录（`build` / `emit` 未给 `--image` 时的缺省来源），每行一个
//! - `audit`：编译前缺口审计（api / corpus / native，报告写 docs/reports/，见 [`audit_cmd`]）
//! - `jdk`：JDK 选择结果与来源 / 已安装列表（与 `build` 同一选择逻辑，见 [`resolve::jdk`]）

mod api_roots;
mod artifacts;
mod audit_cmd;
mod audit_report;
mod build_cmd;
mod build_libs;
mod build_opts;
mod cargo;
mod closure_cmd;
mod closure_run;
mod compile_cmd;
mod deps_lock;
mod profile_cmd;
mod profile_emit;
mod status;

use std::path::PathBuf;
use std::process::ExitCode;

/// 全局分配器：生成管线以大量小字符串 / 小向量分配为主，mimalloc 比系统分配器省约两成指令
/// （输出与分配器无关：生成结果不依赖地址或哈希种子）
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn usage() -> ExitCode {
    eprintln!(
        "用法：\n  rava closure <Test.java | 类目录> [--jdk <主版本>] [--runtime <路径>] [--main <类>] [-o closure.json] [--why <类|方法>]… [--report <md>] [--flow-batch N] [--hash-seed N] [--cut <类.方法:描述符[@偏移]>]… [--cut-file <文件>]… [--dump-edges <文件>] [--site-prof]\n  \
         rava build <A.java>… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--main 类] [--image D]… [--locale L]… [--root 类.方法:描述符]… [--deps deps.lock.toml] [--cp 锁条目名[,…]] [--launch \"<启动选项>\"] [--seed-class FQN[,…]]… [--batch] [--api-package P]… [--api-recursive] [--trace-class 类] [--clean] [--stop-after javac|closure|emit|compile|run] [--build-timeout 秒] [--release | --dev-opt] [--target-dir D] [--keep-artifacts] [--closure-cache D] [--strict] [--debug] [--full-precheck] [--raw-sites FILE] [--perf] [--emit-jobs N] [--cut 条目]… [--cut-file F]… [--dump-edges F] [--profile profile.json]\n  \
         rava compile <scratch> [--release | --dev-opt] [--target-dir D] [--build-timeout 秒] [--keep-artifacts] [--runtime R]\n  \
         rava prune <scratch>…\n  \
         rava emit <closure.json> [--classes DIR] [--java A.java]… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--image D]… [--clean] [--strict] [--debug] [--full-precheck] [--raw-sites FILE] [--perf] [--emit-jobs N] [--profile profile.json]\n  \
         rava profile [<A.java | 类目录>]… [--entries <清单>] [--closure <closure.json>]… [--jdk N | --java-home P] [--runtime R] [--image D]… [-o profile.json] [--entry-out DIR] [--closure-cache D] [--flow-batch N] [--hash-seed N] | rava profile --covers <profile.json> <closure.json>…
         rava image-dirs [--jdk N | --java-home P] [--runtime R]\n  \
         rava jdk [--jdk N | --java-home P] [--runtime R] [--home-only | --json] | rava jdk --list\n  \
         rava audit api <包>… [--recursive] | rava audit corpus|native [--filter S…] [-j N]（另可带 --jdk / --java-home / --runtime / --closure-cache）"
    );
    ExitCode::from(2)
}

pub struct Args {
    pub rest: Vec<String>,
}

impl Args {
    pub fn opt(&self, name: &str) -> Option<String> {
        let i = self.rest.iter().position(|a| a == name)?;
        self.rest.get(i + 1).cloned()
    }
}

/// `--jdk` / `--java-home` / 环境 / 仓库固定版本 → 选中的 JDK（[`resolve::jdk::choose`]）
pub fn choose_jdk(args: &Args) -> Result<resolve::jdk::JdkChoice, String> {
    let major = args.opt("--jdk").map(|v| v.parse::<u32>().map_err(|_| format!("--jdk 需为数字：{v}"))).transpose()?;
    let explicit = args.opt("--java-home").map(PathBuf::from);
    if major.is_some() && explicit.is_some() {
        return Err("--jdk 与 --java-home 互斥".into());
    }
    // 仓库根（读 .jdk-version）：runtime 目录上两级；找不到 runtime 时跳过固定版本这一级
    let repo = closure_cmd::find_runtime_dir(args.opt("--runtime").map(PathBuf::from))
        .ok()
        .and_then(|rt| std::path::absolute(rt).ok()?.parent()?.parent().map(PathBuf::from));
    resolve::jdk::choose(major, explicit.as_deref(), repo.as_deref())
}

pub fn java_home(args: &Args) -> Result<PathBuf, String> {
    Ok(choose_jdk(args)?.home)
}

/// `rava jdk`：打印选中的 JDK 与来源（`--home-only` 只打印 home，供 shell 取 JAVA_HOME；`--json` 打印
/// `{home, major, source}`，与 build_status.json 的 jdk 段同形，供编排读取）；`--list` 列出已安装版本
fn jdk_cmd(args: &Args) -> Result<(), String> {
    if args.rest.iter().any(|a| a == "--list") {
        for (m, h) in resolve::jdk::installed_jdks() {
            println!("JDK {m}: {}", h.display());
        }
        return Ok(());
    }
    let c = choose_jdk(args)?;
    if args.rest.iter().any(|a| a == "--home-only") {
        println!("{}", c.home.display());
    } else if args.rest.iter().any(|a| a == "--json") {
        println!("{}", serde_json::json!({ "home": c.home, "major": c.major, "source": c.source.to_string() }));
    } else {
        println!("{}", c.describe());
    }
    Ok(())
}

/// `rava image-dirs`：与 `build` 缺省派生同一实现（[`resolve::image`]）
fn image_dirs(args: &Args) -> Result<(), String> {
    let home = java_home(args)?;
    let rt = closure_cmd::find_runtime_dir(args.opt("--runtime").map(PathBuf::from))?;
    for d in resolve::image::image_class_dirs(&home, &build_cmd::support_root(&rt)) {
        println!("{}", d.display());
    }
    Ok(())
}

fn main() -> ExitCode {
    let mut argv = std::env::args().skip(1);
    let Some(cmd) = argv.next() else { return usage() };
    let args = Args { rest: argv.collect() };
    let r = match cmd.as_str() {
        "closure" => closure_cmd::run(&args),
        "build" => build_cmd::run_build(&args),
        "emit" => build_cmd::run_emit(&args),
        "compile" => compile_cmd::run_compile(&args),
        "prune" => artifacts::run_prune(&args.rest),
        "image-dirs" => image_dirs(&args),
        "jdk" => jdk_cmd(&args),
        "audit" => audit_cmd::run(&args),
        "profile" => profile_cmd::run(&args),
        _ => return usage(),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("错误：{e}");
            ExitCode::FAILURE
        }
    }
}
