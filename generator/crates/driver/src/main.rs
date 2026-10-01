//! rava：Rust 生成器入口。当前子命令：
//! - `dump-classes`：按 golden 归一形态输出类解析结果（与 scripts/classfile_golden.py 对照）
//! - `closure`：精确闭包分析（XTA + 抽象解释 + 手写层 syn 扫描），输出 closure.json / 溯源 / 报告
//! - `build`：javac → 闭包 → 发射 scratch →（缺省）cargo run
//! - `emit`：既有 closure.json → 发射 scratch
//! - `image-dirs`：镜像独有 / VM 支持类目录（`build` / `emit` 未给 `--image` 时的缺省来源），每行一个

mod api_roots;
mod build_cmd;
mod build_libs;
mod build_opts;
mod closure_cmd;
mod closure_run;
mod dump;

use std::path::PathBuf;
use std::process::ExitCode;

/// 全局分配器：生成管线以大量小字符串 / 小向量分配为主，mimalloc 比系统分配器省约两成指令
/// （输出与分配器无关：生成结果不依赖地址或哈希种子）
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn usage() -> ExitCode {
    eprintln!(
        "用法：\n  rava dump-classes [--jdk <主版本> | --java-home <路径>] [--module <jmod 名>] [--prefix <包前缀>]\n  rava closure <Test.java | 类目录> [--jdk <主版本>] [--runtime <路径>] [--main <类>] [-o closure.json] [--why <类|方法>]… [--report <md>] [--flow-batch N] [--hash-seed N] [--cut <类.方法:描述符[@偏移]>]… [--cut-file <文件>]… [--dump-edges <文件>]\n  \
         rava build <A.java>… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--main 类] [--image D]… [--locale L]… [--root 类.方法:描述符]… [--lib NAME=JAR[:seed=FQN,…]]… [--batch] [--api-package P]… [--api-recursive] [--trace-class 类] [--clean] [--no-run] [--strict] [--debug] [--precheck-only] [--raw-sites FILE] [--perf] [--emit-jobs N] [--cut 条目]… [--cut-file F]… [--dump-edges F]\n  \
         rava emit <closure.json> [--classes DIR] [--java A.java]… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--image D]… [--clean] [--strict] [--debug] [--precheck-only] [--raw-sites FILE] [--perf] [--emit-jobs N]\n  \
         rava image-dirs [--jdk N | --java-home P] [--runtime R]"
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

pub fn java_home(args: &Args) -> Result<PathBuf, String> {
    if let Some(h) = args.opt("--java-home") {
        return Ok(PathBuf::from(h));
    }
    let major = args.opt("--jdk").map(|v| v.parse::<u32>().map_err(|_| format!("--jdk 需为数字：{v}"))).transpose()?;
    resolve::jdk::find_java_home(major).ok_or_else(|| "找不到含 jmods/ 的 JDK".to_string())
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
        "dump-classes" => dump::run(&args),
        "closure" => closure_cmd::run(&args),
        "build" => build_cmd::run_build(&args),
        "emit" => build_cmd::run_emit(&args),
        "image-dirs" => image_dirs(&args),
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
