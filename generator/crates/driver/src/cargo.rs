//! rava 内唯一的 cargo 调用点：编译生成的 workspace、记录产物清单与失败现场、执行产物。
//!
//! - 环境：共享 `CARGO_TARGET_DIR`（缺省 `<repo>/build/target`，`--target-dir` 覆盖）、`CARGO_INCREMENTAL=0`（scratch 每轮重生成源文件，
//!   增量命中趋零，而宽闭包 crate 上增量元数据的双份内存是 OOM 的压垮点）。调试信息级别由生成的
//!   workspace `[profile.dev]` 决定。
//! - 重型判定按**最大单 crate**：S4 拆层后实现层按体积均衡装箱（每箱峰值有界），峰值在声明层
//!   `java_runtime`，其生成类数达到阈值即单作业编译；调用方显式设置 `CARGO_BUILD_JOBS` 时尊重调用方。
//! - 产物清单 `<scratch>/build_artifacts.json`：本工作区 manifest 下的 compiler-artifact / build-script-executed
//!   （通过测试的产物清理只读它）；rustc 全文落 `<scratch>/logs/build.log`。

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use serde_json::{json, Value};

/// 声明层生成类数阈值：16G 机器上 `CARGO_BUILD_JOBS=2` 时约 1750 类闭包即被 OOM 杀
/// （2026-09-28 实测，拆层前口径）；达到阈值的工作区单作业编译
pub const HEAVY_CLASSES: usize = 1700;

/// 峰值所在 crate（声明层）
pub const PEAK_CRATE: &str = "java_runtime";

pub const ARTIFACTS_FILE: &str = "build_artifacts.json";
pub const BUILD_LOG: &str = "logs/build.log";

/// 编译超时缺省（秒）：普通工作区 / 重型工作区（单作业编译，墙钟数倍）；`--build-timeout` 覆盖
pub const DEFAULT_TIMEOUT_SECS: u64 = 600;
pub const HEAVY_TIMEOUT_SECS: u64 = 3000;

/// 一次 cargo 编译的调用设置
#[derive(Debug, Clone)]
pub struct CargoOpts {
    pub target_dir: PathBuf,
    pub release: bool,
    /// None = 按重型判定取缺省
    pub timeout: Option<Duration>,
}

/// 重型判定结果
#[derive(Debug, Clone)]
pub struct Heavy {
    pub classes: usize,
    /// 本次强制的作业数（None = 不干预）
    pub jobs: Option<u32>,
}

impl Heavy {
    pub fn decide(peak_classes: usize) -> Heavy {
        Heavy::decide_with(peak_classes, std::env::var_os("CARGO_BUILD_JOBS").is_some())
    }

    fn decide_with(peak_classes: usize, caller_jobs: bool) -> Heavy {
        let forced = peak_classes >= HEAVY_CLASSES && !caller_jobs;
        Heavy { classes: peak_classes, jobs: forced.then_some(1) }
    }

    /// 规模达阈值（不论作业数是否由调用方指定）
    pub fn is_heavy(&self) -> bool {
        self.classes >= HEAVY_CLASSES
    }

    pub fn default_timeout(&self) -> Duration {
        Duration::from_secs(if self.is_heavy() { HEAVY_TIMEOUT_SECS } else { DEFAULT_TIMEOUT_SECS })
    }

    pub fn to_json(&self) -> Value {
        json!({ "crate": PEAK_CRATE, "classes": self.classes, "jobs": self.jobs })
    }
}

/// 编译失败现场（写入 build_status.json）
#[derive(Debug, Default, Clone)]
pub struct Failure {
    pub exit: Option<i32>,
    pub signal: Option<i32>,
    pub timeout: bool,
    pub first_error: String,
    pub log: Option<PathBuf>,
}

impl Failure {
    pub fn summary(&self) -> String {
        if self.timeout {
            return format!("cargo build 超时（{}）", self.first_error);
        }
        if let Some(s) = self.signal {
            let oom = if s == 9 { "——疑似 OOM（rustc 被 OOM Killer 杀）" } else { "" };
            return format!("cargo build 被信号 {s} 终止{oom}");
        }
        let log = self.log.as_ref().map(|l| format!("（全文 → {}）", l.display())).unwrap_or_default();
        format!("cargo build 失败：{}{log}", self.first_error)
    }
}

/// 一个 cargo JSON 消息是否属于本 scratch（build-script-executed 无 manifest_path，按 package_id 判定）
fn owned(msg: &Value, scratch: &str) -> bool {
    let manifest = msg.get("manifest_path").and_then(Value::as_str).unwrap_or_default();
    manifest.starts_with(scratch) || msg.get("package_id").and_then(Value::as_str).is_some_and(|p| p.contains(scratch))
}

/// cargo JSON 消息流 → (本 scratch 的产物路径集合（升序去重）, bin 可执行文件)
pub fn collect_artifacts(stdout: &str, scratch: &Path, bin: &str) -> (Vec<PathBuf>, Option<PathBuf>) {
    let scratch = scratch.to_string_lossy();
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut exe = None;
    for line in stdout.lines().filter(|l| l.starts_with('{')) {
        let Ok(msg) = serde_json::from_str::<Value>(line) else { continue };
        if !owned(&msg, &scratch) {
            continue;
        }
        match msg.get("reason").and_then(Value::as_str) {
            Some("compiler-artifact") => {
                let files = msg.get("filenames").and_then(Value::as_array).into_iter().flatten();
                paths.extend(files.filter_map(Value::as_str).map(PathBuf::from));
                if let Some(e) = msg.get("executable").and_then(Value::as_str) {
                    paths.push(PathBuf::from(e));
                    if msg.pointer("/target/name").and_then(Value::as_str) == Some(bin) {
                        exe = Some(PathBuf::from(e));
                    }
                }
            }
            Some("build-script-executed") => {
                // build/<crate>-<hash>/（含 out/ 与 output）
                if let Some(d) = msg.get("out_dir").and_then(Value::as_str).and_then(|d| Path::new(d).parent()) {
                    paths.push(d.to_path_buf());
                }
            }
            _ => {}
        }
    }
    paths.sort();
    paths.dedup();
    (paths, exe)
}

/// stderr 中首个 `error` 行（截 160 字符）
pub fn first_error(stderr: &str) -> String {
    let l = stderr.lines().find(|l| l.starts_with("error")).unwrap_or("unknown build error");
    l.chars().take(160).collect()
}

/// 等待子进程；超时则杀整个进程组（子进程以自身为组长启动）
fn wait(child: &mut std::process::Child, limit: Duration) -> Result<Option<ExitStatus>, String> {
    let start = Instant::now();
    loop {
        if let Some(st) = child.try_wait().map_err(|e| format!("cargo：{e}"))? {
            return Ok(Some(st));
        }
        if start.elapsed() >= limit {
            let _ = Command::new("kill").args(["-KILL", "--", &format!("-{}", child.id())]).status();
            let _ = child.wait();
            return Ok(None);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(unix)]
fn own_group(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    cmd.process_group(0);
}

#[cfg(not(unix))]
fn own_group(_: &mut Command) {}

/// `cargo build --bin <bin>`：成功返回可执行文件路径；产物清单与 rustc 全文落 scratch
pub fn compile(out: &Path, bin: &str, heavy: &Heavy, c: &CargoOpts) -> Result<PathBuf, Failure> {
    let timeout = c.timeout.unwrap_or_else(|| heavy.default_timeout());
    let log = out.join(BUILD_LOG);
    let fail = |first_error: String| Failure { first_error, ..Failure::default() };
    std::fs::create_dir_all(log.parent().unwrap_or(out)).map_err(|e| fail(format!("{}：{e}", log.display())))?;
    let log_file = std::fs::File::create(&log).map_err(|e| fail(format!("{}：{e}", log.display())))?;
    let profile: &[&str] = if c.release { &["--release"] } else { &[] };
    println!("\n[build] cargo build {}--bin {bin}", if c.release { "--release " } else { "" });
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--bin", bin, "--message-format=json-render-diagnostics"])
        .args(profile)
        .current_dir(out)
        .env("CARGO_TARGET_DIR", &c.target_dir)
        .env("CARGO_INCREMENTAL", "0")
        .stdout(Stdio::piped())
        .stderr(Stdio::from(log_file));
    if let Some(j) = heavy.jobs {
        println!("[cargo-env] 声明层 {PEAK_CRATE} 生成类 {} ≥ {HEAVY_CLASSES}：CARGO_BUILD_JOBS={j}（内存上限）", heavy.classes);
        cmd.env("CARGO_BUILD_JOBS", j.to_string());
    }
    // 超时要连同 rustc 子进程一起终止：独立进程组（交互 Ctrl-C 只终止 rava；cargo 随 stdout 管道断开退出）
    own_group(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| fail(format!("cargo：{e}")))?;
    let stdout = child.stdout.take().expect("piped stdout");
    let reader = std::thread::spawn(move || {
        let mut s = String::new();
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            s.push_str(&line);
            s.push('\n');
        }
        s
    });
    let status = wait(&mut child, timeout).map_err(fail)?;
    let json_out = reader.join().unwrap_or_default();
    let (paths, exe) = collect_artifacts(&json_out, out, bin);
    let manifest = json!({ "bin": bin, "executable": exe, "paths": paths });
    let text = serde_json::to_string_pretty(&manifest).unwrap_or_default();
    std::fs::write(out.join(ARTIFACTS_FILE), text).map_err(|e| fail(format!("{ARTIFACTS_FILE}：{e}")))?;
    let stderr = std::fs::read_to_string(&log).unwrap_or_default();
    let Some(status) = status else {
        return Err(Failure { timeout: true, first_error: format!("{} s", timeout.as_secs()), log: Some(log), ..Failure::default() });
    };
    if !status.success() {
        #[cfg(unix)]
        let signal = std::os::unix::process::ExitStatusExt::signal(&status);
        #[cfg(not(unix))]
        let signal = None;
        return Err(Failure { exit: status.code(), signal, timeout: false, first_error: first_error(&stderr), log: Some(log) });
    }
    exe.ok_or_else(|| fail(format!("cargo 产物清单中没有 bin {bin} 的可执行文件")))
}

/// 执行编译产物（标准输入输出继承）
pub fn run(exe: &Path) -> Result<(), String> {
    println!("\n[run] {}", exe.display());
    let st = Command::new(exe).status().map_err(|e| format!("{}：{e}", exe.display()))?;
    if !st.success() {
        return Err(format!("运行失败（{st}）"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifacts_only_from_own_scratch() {
        let s = "/w/build/t1";
        let msgs = [
            json!({"reason":"compiler-artifact","manifest_path":"/w/build/t1/user/Cargo.toml","target":{"name":"t1"},
                   "filenames":["/tg/debug/deps/t1-ab12"],"executable":"/tg/debug/t1"}),
            json!({"reason":"compiler-artifact","manifest_path":"/reg/syn/Cargo.toml","target":{"name":"syn"},
                   "filenames":["/tg/debug/deps/libsyn-1.rlib"],"executable":null}),
            json!({"reason":"build-script-executed","package_id":"path+file:///w/build/t1/java_runtime#0.1.0",
                   "out_dir":"/tg/debug/build/java_runtime-9f/out"}),
            json!({"reason":"compiler-message","manifest_path":"/w/build/t1/user/Cargo.toml"}),
        ];
        let text: String = msgs.iter().map(|m| format!("{m}\n")).chain(["not json\n".to_string()]).collect();
        let (paths, exe) = collect_artifacts(&text, Path::new(s), "t1");
        assert_eq!(exe, Some(PathBuf::from("/tg/debug/t1")));
        let want: Vec<PathBuf> =
            ["/tg/debug/build/java_runtime-9f", "/tg/debug/deps/t1-ab12", "/tg/debug/t1"].iter().map(PathBuf::from).collect();
        assert_eq!(paths, want);
    }

    #[test]
    fn first_error_line() {
        assert_eq!(first_error("warning: x\nerror[E0308]: mismatched\nerror: again"), "error[E0308]: mismatched");
        assert_eq!(first_error("ok"), "unknown build error");
    }

    #[test]
    fn heavy_threshold() {
        assert_eq!(Heavy::decide_with(HEAVY_CLASSES, false).jobs, Some(1));
        assert_eq!(Heavy::decide_with(HEAVY_CLASSES - 1, false).jobs, None);
        assert_eq!(Heavy::decide_with(HEAVY_CLASSES * 2, true).jobs, None, "调用方显式设置时不干预");
        assert_eq!(Heavy::decide_with(HEAVY_CLASSES * 2, true).default_timeout(), Duration::from_secs(HEAVY_TIMEOUT_SECS));
        assert_eq!(Heavy::decide_with(1, false).default_timeout(), Duration::from_secs(DEFAULT_TIMEOUT_SECS));
    }
}
