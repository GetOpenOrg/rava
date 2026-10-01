//! 编译阶段：`rava build` 的 compile 段与 `rava compile <scratch>` 共用。
//!
//! `rava compile <scratch> [--release] [--target-dir D] [--build-timeout SECS] [--runtime R]`：编译已由
//! `rava build --stop-after emit` 发射好的工作区（bin 名与重型判定输入读自 `build_status.json` 的 emit 段），
//! 更新 `build_status.json` / `build_artifacts.json`。批量编排先并行发射、再逐个编译时用它——发射进程
//! 不必常驻等待共享 target 的 cargo 文件锁。

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::build_cmd::repo_root;
use crate::build_opts::Stage;
use crate::cargo::{self, CargoOpts, Heavy};
use crate::closure_cmd::find_runtime_dir;
use crate::status::{BuildStatus, EmitSummary, STATUS_FILE};
use crate::Args;

/// 编译设置（命令行原样）
#[derive(Debug, Default, Clone)]
pub struct CompileArgs {
    pub release: bool,
    pub target_dir: Option<PathBuf>,
    pub build_timeout: Option<u64>,
}

/// 重型判定 → cargo build；进度与失败现场记入 `st`，返回可执行文件
pub fn compile_stage(out: &Path, repo: &Path, emit: &EmitSummary, c: &CompileArgs, st: &mut BuildStatus) -> Result<PathBuf, String> {
    st.stage = Stage::Compile;
    let heavy = Heavy::decide(emit.jdk_classes);
    st.heavy = Some(heavy.clone());
    let opts = CargoOpts {
        target_dir: c.target_dir.clone().unwrap_or_else(|| repo.join("build").join("target")),
        release: c.release,
        timeout: c.build_timeout.map(Duration::from_secs),
    };
    let exe = cargo::compile(out, &emit.bin, &heavy, &opts).map_err(|f| {
        let msg = f.summary();
        st.failure = Some(f);
        msg
    })?;
    st.exe = Some(exe.clone());
    Ok(exe)
}

fn parse(rest: &[String]) -> Result<(PathBuf, CompileArgs, Option<PathBuf>), String> {
    let mut scratch = None;
    let mut c = CompileArgs::default();
    let mut runtime = None;
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--release" => c.release = true,
            "--target-dir" | "--build-timeout" | "--runtime" => {
                let v = it.next().ok_or_else(|| format!("{a} 缺少取值"))?;
                match a.as_str() {
                    "--target-dir" => c.target_dir = Some(PathBuf::from(v)),
                    "--runtime" => runtime = Some(PathBuf::from(v)),
                    _ => c.build_timeout = Some(v.parse().map_err(|_| format!("--build-timeout 需为秒数：{v}"))?),
                }
            }
            s if s.starts_with('-') => return Err(format!("未知选项：{s}")),
            s if scratch.is_none() => scratch = Some(PathBuf::from(s)),
            s => return Err(format!("多余参数：{s}")),
        }
    }
    Ok((scratch.ok_or("缺少 scratch 工作区目录")?, c, runtime))
}

pub fn run_compile(args: &Args) -> Result<(), String> {
    let (scratch, c, runtime) = parse(&args.rest)?;
    let out = std::path::absolute(&scratch).map_err(|e| format!("{}：{e}", scratch.display()))?;
    let (emit, prev) = EmitSummary::read(&out)?;
    let repo = repo_root(&std::path::absolute(find_runtime_dir(runtime)?).map_err(|e| e.to_string())?);
    let artifacts = out.join(cargo::ARTIFACTS_FILE);
    if artifacts.exists() {
        std::fs::remove_file(&artifacts).map_err(|e| format!("{}：{e}", artifacts.display()))?;
    }
    let mut st = BuildStatus { emit: Some(emit.clone()), ..BuildStatus::default() };
    let r = compile_stage(&out, &repo, &emit, &c, &mut st).map(|_| ());
    // jdk 段沿用发射时的选择（编译不重新选 JDK）
    let mut v = st.to_json(r.as_ref().err());
    v["jdk"] = prev["jdk"].clone();
    let p = out.join(STATUS_FILE);
    std::fs::write(&p, serde_json::to_string_pretty(&v).unwrap_or_default()).map_err(|e| format!("{}：{e}", p.display()))?;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn parse_compile_args() {
        let (d, c, rt) = parse(&args("/s --release --target-dir /t --build-timeout 9 --runtime /r")).unwrap();
        assert_eq!(d, PathBuf::from("/s"));
        assert!(c.release);
        assert_eq!(c.target_dir, Some(PathBuf::from("/t")));
        assert_eq!(c.build_timeout, Some(9));
        assert_eq!(rt, Some(PathBuf::from("/r")));
        assert!(parse(&args("--release")).is_err(), "缺 scratch");
        assert!(parse(&args("/s /t")).is_err());
        assert!(parse(&args("/s --bogus")).is_err());
    }
}
