//! rava：Rust 生成器入口。当前子命令：
//! - `dump-classes`：按 golden 归一形态输出类解析结果（与 scripts/classfile_golden.py 对照）

mod dump;

use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!(
        "用法：\n  rava dump-classes [--jdk <主版本> | --java-home <路径>] [--module <jmod 名>] [--prefix <包前缀>]"
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

fn main() -> ExitCode {
    let mut argv = std::env::args().skip(1);
    let Some(cmd) = argv.next() else { return usage() };
    let args = Args { rest: argv.collect() };
    let r = match cmd.as_str() {
        "dump-classes" => dump::run(&args),
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
