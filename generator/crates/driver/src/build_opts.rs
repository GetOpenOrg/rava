//! `rava build` / `rava emit` 的参数解析（纯函数，单测覆盖）。
//!
//! - `rava build <A.java>… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--main 类]
//!   [--image D]… [--locale L]… [--root 类.方法:描述符]…
//!   [--deps deps.lock.toml] [--cp 锁条目名[,…]] [--launch "<启动选项>"] [--seed-class FQN[,…]]… [--batch]
//!   [--api-package P]… [--api-recursive] [--trace-class 类] [--clean] [--strict] [--debug]
//!   [--stop-after javac|closure|emit|compile|run] [--full-precheck] [--build-timeout SECS] [--release | --dev-opt] [--target-dir D]
//!   [--keep-artifacts]
//!   [--raw-sites FILE] [--perf] [--emit-jobs N] [--closure-json]
//!   [--cut 类.方法:描述符[@偏移]]… [--cut-file F]… [--dump-edges F]（后三项为闭包诊断，同 `rava closure`）
//!   [--closure-cache DIR（缺省 <仓库>/build/closure_cache）] [--closure-cache-max-mb N] [--profile profile.json]`
//! - `rava emit <closure.json> [--classes DIR] [--jdk N | --java-home P] [--runtime R] [--out DIR]
//!   [--java A.java]… [--image D]… [--clean] [--strict] [--debug] [--full-precheck] [--raw-sites FILE]
//!   [--perf] [--emit-jobs N] [--profile profile.json]`（`--java`：源文件，决定用户类包布局与入口序）
//!
//! 选项语义见 docs/environment-variables.md。

use std::path::{Path, PathBuf};

/// `--stop-after`：build 的最后一个阶段（缺省 run）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stage {
    Javac,
    Closure,
    Emit,
    Compile,
    #[default]
    Run,
}

impl Stage {
    pub fn parse(s: &str) -> Result<Stage, String> {
        Ok(match s {
            "javac" => Stage::Javac,
            "closure" => Stage::Closure,
            "emit" => Stage::Emit,
            "compile" => Stage::Compile,
            "run" => Stage::Run,
            _ => return Err(format!("--stop-after 取值应为 javac|closure|emit|compile|run，收到：{s}")),
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Stage::Javac => "javac",
            Stage::Closure => "closure",
            Stage::Emit => "emit",
            Stage::Compile => "compile",
            Stage::Run => "run",
        }
    }
}

/// 子命令
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Build,
    Emit,
}

/// 解析结果
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BuildOpts {
    /// build：.java 源文件（首个决定入口类与 scratch 名）；emit：唯一的 closure.json
    pub inputs: Vec<PathBuf>,
    pub jdk: Option<u32>,
    pub java_home: Option<PathBuf>,
    pub runtime: Option<PathBuf>,
    pub out: Option<PathBuf>,
    pub main: Option<String>,
    /// emit：用户类目录（缺省 closure.json 同目录的 `classes/`）
    pub classes: Option<PathBuf>,
    /// emit：用户类的 .java 源文件（build 即 `inputs`）
    pub java: Vec<PathBuf>,
    pub images: Vec<PathBuf>,
    pub locales: Vec<String>,
    pub roots: Vec<String>,
    /// 闭包诊断：反事实切除条目 / 条目文件 / 触发边转储文件
    pub cuts: Vec<String>,
    pub cut_files: Vec<String>,
    pub dump_edges: Option<String>,
    /// 公开 API 包为调用链入口（包内 public 类的 public / protected 方法，[`crate::api_roots`]）
    pub api_packages: Vec<String>,
    /// `--api-package` 含子包
    pub api_recursive: bool,
    /// 依赖锁（V12 §3.2）：`--deps <deps.lock.toml>`；库输入唯一来源（`--lib` 已删除）
    pub deps: Option<PathBuf>,
    /// 入口类路径：`--cp <锁条目名>[,…]`（实际顺序取锁序，与给出序无关）
    pub cp: Vec<String>,
    /// 启动选项（`--launch "<启动选项>"`）：进入分析与运行期（本步入库传递）
    pub launch: Option<String>,
    /// 显式种子类（斜线形态）：整包翻译的 crate 验收以显式种子给出（`:seed=` 已删除）
    pub seed_classes: Vec<String>,
    /// 批量模式：入口写 `user/src/bin/<bin>.rs` 并向 user/Cargo.toml 追加 `[[bin]]`
    pub batch: bool,
    /// 打印该类 / 方法入闭包的最短 provenance 链（`rava closure --why`）
    pub trace_class: Option<String>,
    pub clean: bool,
    /// build 的最后一个阶段
    pub stop_after: Stage,
    /// cargo build 超时（秒；超时终止整个 cargo 进程组；缺省按重型判定，见 `cargo::Heavy::default_timeout`）
    pub build_timeout: Option<u64>,
    /// cargo 构建档位（`--release` / `--dev-opt`，缺省 dev）
    pub build_profile: crate::cargo::BuildProfile,
    /// 保留本例编译产物（缺省链接后删中间产物、运行后删可执行文件，见 [`crate::artifacts`]）
    pub keep_artifacts: bool,
    /// 共享编译缓存（CARGO_TARGET_DIR；缺省 `<仓库>/build/target`）
    pub target_dir: Option<PathBuf>,
    pub strict: bool,
    /// 诊断明细：存根兜底逐条 / 闭包未解析调用
    pub debug: bool,
    /// 输出完整预检明细（不截断），不输出审计与发射汇总；build 须配合 `--stop-after emit`
    pub full_precheck: bool,
    /// Raw 逃生舱构造位点剖面追加写入的文件
    pub raw_sites: Option<PathBuf>,
    /// 输出 `[perf]` 分阶段耗时 / 峰值 RSS / 逐类逐方法 Top-N
    pub perf: bool,
    /// 逐类发射并行度（缺省 0 = 可用核数；输出与并行度无关）
    pub emit_jobs: usize,
    /// build：另写出 `<scratch>/closure_input/closure.json`（调试 / 审计 / `rava emit` 输入），并校验
    /// 由它解析的发射输入与进程内直传的一致；缺省不写（闭包结果只在内存中交给发射层）
    pub closure_json: bool,
    /// build：闭包分析跨运行结果缓存目录（`closure::cache`；缺省不用缓存）
    pub closure_cache: Option<PathBuf>,
    /// 缓存总量上限（MB；缺省 `closure::cache::DEFAULT_MAX_MB`）
    pub closure_cache_max_mb: Option<u64>,
    /// 档案发射（T1 1b）：非用户侧事实取该 `profile.json`（`rava profile` 产物），用户侧取本程序单例闭包
    pub profile: Option<PathBuf>,
}

const VALUED: [&str; 28] = [
    "--profile",
    "--stop-after",
    "--target-dir",
    "--build-timeout",
    "--closure-cache",
    "--closure-cache-max-mb",
    "--jdk",
    "--java-home",
    "--runtime",
    "--out",
    "--main",
    "--classes",
    "--java",
    "--image",
    "--locale",
    "--root",
    "-o",
    "--deps",
    "--cp",
    "--launch",
    "--seed-class",
    "--trace-class",
    "--raw-sites",
    "--api-package",
    "--emit-jobs",
    "--cut",
    "--cut-file",
    "--dump-edges",
];
const FLAGS: [&str; 11] = [
    "--clean",
    "--strict",
    "--batch",
    "--debug",
    "--full-precheck",
    "--api-recursive",
    "--perf",
    "--closure-json",
    "--release",
    "--dev-opt",
    "--keep-artifacts",
];
/// 只属于 build 的选项
const BUILD_ONLY: [&str; 24] = [
    "--release",
    "--dev-opt",
    "--keep-artifacts",
    "--target-dir",
    "--closure-cache",
    "--closure-cache-max-mb",
    "--main",
    "--locale",
    "--root",
    "--stop-after",
    "--build-timeout",
    "-o",
    "--deps",
    "--cp",
    "--launch",
    "--seed-class",
    "--batch",
    "--trace-class",
    "--api-package",
    "--api-recursive",
    "--cut",
    "--cut-file",
    "--dump-edges",
    "--closure-json",
];
/// 只属于 emit 的选项
const EMIT_ONLY: [&str; 2] = ["--classes", "--java"];

impl BuildOpts {
    pub fn parse(mode: Mode, rest: &[String]) -> Result<BuildOpts, String> {
        let mut o = BuildOpts::default();
        let (mut dev_opt, mut release) = (false, false);
        let mut it = rest.iter();
        while let Some(a) = it.next() {
            if !a.starts_with('-') {
                o.inputs.push(PathBuf::from(a));
                continue;
            }
            let foreign = match mode {
                Mode::Build => EMIT_ONLY.contains(&a.as_str()),
                Mode::Emit => BUILD_ONLY.contains(&a.as_str()),
            };
            if a == "--lib" {
                return Err("--lib 已删除：库输入改用 --deps <deps.lock.toml> + --cp <锁条目名>[,…]，整包种子改用 --seed-class".into());
            }
            if foreign || !(VALUED.contains(&a.as_str()) || FLAGS.contains(&a.as_str())) {
                return Err(format!("未知选项：{a}"));
            }
            if FLAGS.contains(&a.as_str()) {
                match a.as_str() {
                    "--clean" => o.clean = true,
                    "--batch" => o.batch = true,
                    "--debug" => o.debug = true,
                    "--full-precheck" => o.full_precheck = true,
                    "--api-recursive" => o.api_recursive = true,
                    "--perf" => o.perf = true,
                    "--closure-json" => o.closure_json = true,
                    "--release" => release = true,
                    "--dev-opt" => dev_opt = true,
                    "--keep-artifacts" => o.keep_artifacts = true,
                    _ => o.strict = true,
                }
                continue;
            }
            let v = it.next().ok_or_else(|| format!("{a} 缺少取值"))?;
            match a.as_str() {
                "--jdk" => o.jdk = Some(v.parse().map_err(|_| format!("--jdk 需为数字：{v}"))?),
                "--stop-after" => o.stop_after = Stage::parse(v)?,
                "--build-timeout" => {
                    o.build_timeout = Some(v.parse().map_err(|_| format!("--build-timeout 需为秒数：{v}"))?)
                }
                "--emit-jobs" => o.emit_jobs = v.parse().map_err(|_| format!("--emit-jobs 需为数字：{v}"))?,
                "--java-home" => o.java_home = Some(PathBuf::from(v)),
                "--runtime" => o.runtime = Some(PathBuf::from(v)),
                "--target-dir" => o.target_dir = Some(PathBuf::from(v)),
                "--out" | "-o" => o.out = Some(PathBuf::from(v)),
                "--main" => o.main = Some(v.replace('.', "/")),
                "--classes" => o.classes = Some(PathBuf::from(v)),
                "--java" => o.java.push(PathBuf::from(v)),
                "--image" => o.images.push(PathBuf::from(v)),
                "--locale" => o.locales.push(v.clone()),
                "--cut" => o.cuts.push(v.clone()),
                "--cut-file" => o.cut_files.push(v.clone()),
                "--dump-edges" => o.dump_edges = Some(v.clone()),
                "--deps" => o.deps = Some(PathBuf::from(v)),
                "--cp" => o.cp.extend(v.split(',').map(str::trim).filter(|n| !n.is_empty()).map(String::from)),
                "--launch" => o.launch = Some(v.clone()),
                "--seed-class" => o
                    .seed_classes
                    .extend(v.split(',').map(str::trim).filter(|f| !f.is_empty()).map(|f| f.replace('.', "/"))),
                "--trace-class" => o.trace_class = Some(v.clone()),
                "--raw-sites" => o.raw_sites = Some(PathBuf::from(v)),
                "--api-package" => o.api_packages.push(v.clone()),
                "--closure-cache" => o.closure_cache = Some(PathBuf::from(v)),
                "--profile" => o.profile = Some(PathBuf::from(v)),
                "--closure-cache-max-mb" => {
                    o.closure_cache_max_mb = Some(v.parse().map_err(|_| format!("--closure-cache-max-mb 需为数字：{v}"))?)
                }
                _ => o.roots.push(v.clone()),
            }
        }
        o.build_profile = crate::cargo::BuildProfile::from_flags(dev_opt, release)?;
        o.validate(mode)?;
        Ok(o)
    }

    fn validate(&self, mode: Mode) -> Result<(), String> {
        if self.jdk.is_some() && self.java_home.is_some() {
            return Err("--jdk 与 --java-home 互斥".into());
        }
        if self.api_recursive && self.api_packages.is_empty() {
            return Err("--api-recursive 需配合 --api-package".into());
        }
        if mode == Mode::Build && self.full_precheck && self.stop_after != Stage::Emit {
            return Err("--full-precheck 需配合 --stop-after emit".into());
        }
        if self.build_timeout.is_some() && self.stop_after < Stage::Compile {
            return Err("--build-timeout 只对 compile / run 阶段有效".into());
        }
        match (&self.deps, self.cp.is_empty(), mode) {
            (Some(_), true, _) => return Err("--deps 须配合 --cp 给出入口类路径（锁条目名）".into()),
            (None, false, _) => return Err("--cp 须配合 --deps（库输入唯一来源是依赖锁）".into()),
            _ => {}
        }
        if !self.seed_classes.is_empty() && self.deps.is_none() {
            return Err("--seed-class 是库类的种子，须配合 --deps".into());
        }
        if !self.cp.is_empty() && self.batch {
            return Err("带依赖锁的构建（--deps）不支持 --batch（单 bin 消费形态）".into());
        }
        match mode {
            Mode::Build => {
                if self.inputs.is_empty() {
                    return Err("缺少输入 .java 文件".into());
                }
                if let Some(bad) = self.inputs.iter().find(|p| p.extension().is_none_or(|e| e != "java")) {
                    return Err(format!("输入须为 .java 文件：{}", bad.display()));
                }
            }
            Mode::Emit => {
                if self.inputs.len() != 1 {
                    return Err("emit 需要恰好一个 closure.json".into());
                }
            }
        }
        Ok(())
    }

    /// 用户类的 .java 源文件
    pub fn java_files(&self, mode: Mode) -> &[PathBuf] {
        match mode {
            Mode::Build => &self.inputs,
            Mode::Emit => &self.java,
        }
    }

    /// emit 的 closure.json 所在目录
    fn closure_dir(&self) -> PathBuf {
        self.inputs[0].parent().map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    }

    /// emit 的用户类目录：显式 `--classes`，否则 closure.json 同目录的 `classes/`
    pub fn emit_classes_dir(&self) -> PathBuf {
        self.classes.clone().unwrap_or_else(|| self.closure_dir().join("classes"))
    }

    /// scratch 目录：显式 `--out`；build 缺省 `<仓库>/build/<入口 snake 名>`；emit 缺省为
    /// `<scratch>/closure_input/closure.json` 布局（`rava build` 产物）的 scratch
    pub fn scratch_dir(&self, mode: Mode, repo_root: &Path) -> Result<PathBuf, String> {
        if let Some(o) = &self.out {
            return Ok(o.clone());
        }
        match mode {
            Mode::Build => {
                let stem = self.inputs[0].file_stem().and_then(|s| s.to_str()).unwrap_or("main");
                Ok(repo_root.join("build").join(emit::text::to_snake(stem)))
            }
            Mode::Emit => {
                let dir = self.closure_dir();
                match (dir.file_name().and_then(|n| n.to_str()), dir.parent()) {
                    (Some(CLOSURE_INPUT_DIR), Some(p)) => Ok(p.to_path_buf()),
                    _ => Err("closure.json 不在 <scratch>/closure_input/ 下：用 --out 指定 scratch".into()),
                }
            }
        }
    }
}

/// scratch 内闭包输入目录（用户类 `classes/` + `closure.json`）
pub const CLOSURE_INPUT_DIR: &str = "closure_input";

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn build_parses_flags_values_and_repeats() {
        let o = BuildOpts::parse(
            Mode::Build,
            &args("A.java B.java --jdk 21 --image /i1 --image /i2 --locale fr --clean --stop-after emit --main p.Main"),
        )
        .unwrap();
        assert_eq!(o.inputs, vec![PathBuf::from("A.java"), PathBuf::from("B.java")]);
        assert_eq!(o.jdk, Some(21));
        assert_eq!(o.images, vec![PathBuf::from("/i1"), PathBuf::from("/i2")]);
        assert_eq!(o.locales, vec!["fr".to_string()]);
        assert!(o.clean && o.stop_after == Stage::Emit && !o.strict);
        assert_eq!(BuildOpts::parse(Mode::Build, &args("A.java")).unwrap().stop_after, Stage::Run);
        assert_eq!(o.main.as_deref(), Some("p/Main"));
    }

    #[test]
    fn build_parses_closure_diag() {
        let o = BuildOpts::parse(
            Mode::Build,
            &args("A.java --cut a/B.m:(Ljava/lang/String;)V --cut a/B.n:()V@7 --cut-file /c.txt --dump-edges /e.tsv"),
        )
        .unwrap();
        assert_eq!(o.cuts, vec!["a/B.m:(Ljava/lang/String;)V".to_string(), "a/B.n:()V@7".to_string()]);
        assert_eq!(o.cut_files, vec!["/c.txt".to_string()]);
        assert_eq!(o.dump_edges.as_deref(), Some("/e.tsv"));
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --cut a/B.m:()V")).is_err());
    }

    #[test]
    fn build_rejects_bad_input() {
        assert!(BuildOpts::parse(Mode::Build, &args("--clean")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --no-run")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --stop-after link")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --full-precheck")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --stop-after emit --build-timeout 60")).is_err());
        let o = BuildOpts::parse(Mode::Build, &args("A.java --stop-after compile --build-timeout 60")).unwrap();
        assert_eq!((o.stop_after, o.build_timeout), (Stage::Compile, Some(60)));
        assert!(BuildOpts::parse(Mode::Build, &args("A.class")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --jdk x")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --jdk")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --bogus")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --classes d")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --jdk 21 --java-home /j")).is_err());
    }

    #[test]
    fn emit_parses_and_rejects_build_only() {
        let o = BuildOpts::parse(Mode::Emit, &args("/s/closure_input/closure.json --java A.java")).unwrap();
        assert_eq!(o.emit_classes_dir(), PathBuf::from("/s/closure_input/classes"));
        assert_eq!(o.java_files(Mode::Emit), &[PathBuf::from("A.java")]);
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --stop-after emit")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("a.json b.json")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("")).is_err());
    }

    #[test]
    fn deps_cp_seed_class_and_diagnostic_flags() {
        let o = BuildOpts::parse(
            Mode::Build,
            &args("A.java --deps /d/deps.lock.toml --cp junit,hamcrest --launch add-opens:a/b=c \
                   --seed-class org.junit.Assert,org.junit.Test --seed-class org/hamcrest/CoreMatchers \
                   --trace-class java/net/X --debug --stop-after emit --full-precheck --raw-sites /tmp/r.txt"),
        )
        .unwrap();
        assert_eq!(o.deps.as_deref(), Some(Path::new("/d/deps.lock.toml")));
        assert_eq!(o.cp, ["junit", "hamcrest"]);
        assert_eq!(o.launch.as_deref(), Some("add-opens:a/b=c"));
        assert_eq!(o.seed_classes, ["org/junit/Assert", "org/junit/Test", "org/hamcrest/CoreMatchers"]);
        assert_eq!(o.trace_class.as_deref(), Some("java/net/X"));
        assert!(o.debug && o.full_precheck && !o.batch);
        assert_eq!(o.raw_sites, Some(PathBuf::from("/tmp/r.txt")));
        for bad in [
            "A.java --lib h=/j.jar",
            "A.java --deps /d.lock",
            "A.java --cp junit",
            "A.java --seed-class org.junit.Assert",
            "A.java --deps /d.lock --cp j --batch",
            "A.java --deps",
            "A.java --deps /d.lock --cp",
        ] {
            assert!(BuildOpts::parse(Mode::Build, &args(bad)).is_err(), "{bad}");
        }
        let err = BuildOpts::parse(Mode::Build, &args("A.java --lib h=/j.jar")).unwrap_err();
        assert!(err.contains("--lib 已删除"), "{err}");
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --batch")).unwrap().batch);
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --deps /d.lock --cp j")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --trace-class X")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --debug --full-precheck")).is_ok());
    }

    #[test]
    fn api_package_options() {
        let o = BuildOpts::parse(Mode::Build, &args("E.java --api-package java/util --api-package java.text --api-recursive")).unwrap();
        assert_eq!(o.api_packages, ["java/util", "java.text"]);
        assert!(o.api_recursive);
        assert!(BuildOpts::parse(Mode::Build, &args("E.java --api-recursive")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --api-package java/util")).is_err());
    }

    #[test]
    fn closure_cache_options() {
        let o = BuildOpts::parse(Mode::Build, &args("E.java --closure-cache /c --closure-cache-max-mb 64")).unwrap();
        assert_eq!(o.closure_cache.as_deref(), Some(Path::new("/c")));
        assert_eq!(o.closure_cache_max_mb, Some(64));
        assert!(BuildOpts::parse(Mode::Build, &args("E.java --closure-cache-max-mb 64")).is_ok(), "缓存目录有缺省");
        let o = BuildOpts::parse(Mode::Build, &args("E.java --release --target-dir /t")).unwrap();
        assert_eq!(o.build_profile, crate::cargo::BuildProfile::Release);
        assert_eq!(o.target_dir.as_deref(), Some(Path::new("/t")));
        let o = BuildOpts::parse(Mode::Build, &args("E.java --dev-opt")).unwrap();
        assert_eq!(o.build_profile, crate::cargo::BuildProfile::DevOpt);
        assert!(BuildOpts::parse(Mode::Build, &args("E.java --dev-opt --release")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --dev-opt")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --release")).is_err());
        assert!(BuildOpts::parse(Mode::Build, &args("E.java --closure-cache /c --closure-cache-max-mb x")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --closure-cache /c")).is_err());
    }

    #[test]
    fn scratch_dir_defaults() {
        let repo = Path::new("/repo");
        let b = BuildOpts::parse(Mode::Build, &args("t/TestHashMapOps.java")).unwrap();
        assert_eq!(b.scratch_dir(Mode::Build, repo).unwrap(), PathBuf::from("/repo/build/test_hash_map_ops"));
        let b = BuildOpts::parse(Mode::Build, &args("t/A.java --out /x")).unwrap();
        assert_eq!(b.scratch_dir(Mode::Build, repo).unwrap(), PathBuf::from("/x"));
        let e = BuildOpts::parse(Mode::Emit, &args("/s/closure_input/closure.json")).unwrap();
        assert_eq!(e.scratch_dir(Mode::Emit, repo).unwrap(), PathBuf::from("/s"));
        let e = BuildOpts::parse(Mode::Emit, &args("/tmp/closure.json")).unwrap();
        assert!(e.scratch_dir(Mode::Emit, repo).is_err());
    }
}
