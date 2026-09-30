//! `rava build` / `rava emit` 的参数解析（纯函数，单测覆盖）。
//!
//! - `rava build <A.java>… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--main 类]
//!   [--image D]… [--locale L]… [--root 类.方法:描述符]… [--lib NAME=JAR[:seed=FQN,…]]… [--batch]
//!   [--api-package P]… [--api-recursive] [--trace-class 类] [--clean] [--no-run] [--strict] [--debug]
//!   [--precheck-only] [--raw-sites FILE] [--perf] [--emit-jobs N]`
//! - `rava emit <closure.json> [--classes DIR] [--jdk N | --java-home P] [--runtime R] [--out DIR]
//!   [--java A.java]… [--image D]… [--clean] [--strict] [--debug] [--precheck-only] [--raw-sites FILE]
//!   [--perf] [--emit-jobs N]`（`--java`：源文件，决定用户类包布局与入口序）
//!
//! 选项语义与 `scripts/main.py` 同名选项一致（docs/environment-variables.md）。

use std::path::{Path, PathBuf};

/// 子命令
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Build,
    Emit,
}

/// `--lib NAME=JAR[:seed=FQN[,FQN…]]`：一个 lib crate 的 jar 输入（声明序即 crate 依赖序）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibSpec {
    pub name: String,
    pub jar: PathBuf,
    /// None = 整包模式（jar 全部类进 crate，种子 = 全部类的 public 方法）；
    /// Some = 子集模式（种子类的 public 方法，只收闭包触达的 jar 类）。斜线形态
    pub seeds: Option<Vec<String>>,
}

impl LibSpec {
    pub fn parse(raw: &str) -> Result<LibSpec, String> {
        let (head, seed_part) = match raw.split_once(":seed=") {
            Some((h, s)) => (h, Some(s)),
            None => (raw, None),
        };
        let (name, jar) = head
            .split_once('=')
            .ok_or_else(|| format!("--lib 格式应为 NAME=JAR[:seed=FQN[,FQN...]]，收到：{raw}"))?;
        if name.is_empty() || jar.is_empty() {
            return Err(format!("--lib 的 NAME/JAR 不能为空：{raw}"));
        }
        let seeds = match seed_part {
            None => None,
            Some(s) => {
                let v: Vec<String> =
                    s.split(',').map(str::trim).filter(|f| !f.is_empty()).map(|f| f.replace('.', "/")).collect();
                if v.is_empty() {
                    return Err(format!("--lib seed 为空：{raw}"));
                }
                Some(v)
            }
        };
        Ok(LibSpec { name: name.to_string(), jar: PathBuf::from(jar), seeds })
    }
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
    /// 公开 API 包为调用链入口（包内 public 类的 public / protected 方法，[`crate::api_roots`]）
    pub api_packages: Vec<String>,
    /// `--api-package` 含子包
    pub api_recursive: bool,
    pub libs: Vec<LibSpec>,
    /// 批量模式：入口写 `user/src/bin/<bin>.rs` 并向 user/Cargo.toml 追加 `[[bin]]`
    pub batch: bool,
    /// 打印该类 / 方法入闭包的最短 provenance 链（`rava closure --why`）
    pub trace_class: Option<String>,
    pub clean: bool,
    pub no_run: bool,
    pub strict: bool,
    /// 诊断明细：存根兜底逐条 / 闭包未解析调用
    pub debug: bool,
    /// 只发射并输出完整预检明细，不输出审计、不编译运行
    pub precheck_only: bool,
    /// Raw 逃生舱构造位点剖面追加写入的文件
    pub raw_sites: Option<PathBuf>,
    /// 输出 `[perf]` 分阶段耗时 / 峰值 RSS / 逐类逐方法 Top-N
    pub perf: bool,
    /// 逐类发射并行度（缺省 0 = 可用核数；输出与并行度无关）
    pub emit_jobs: usize,
}

const VALUED: [&str; 16] = [
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
    "--lib",
    "--trace-class",
    "--raw-sites",
    "--api-package",
    "--emit-jobs",
];
const FLAGS: [&str; 8] = ["--clean", "--no-run", "--strict", "--batch", "--debug", "--precheck-only", "--api-recursive", "--perf"];
/// 只属于 build 的选项
const BUILD_ONLY: [&str; 10] =
    ["--main", "--locale", "--root", "--no-run", "-o", "--lib", "--batch", "--trace-class", "--api-package", "--api-recursive"];
/// 只属于 emit 的选项
const EMIT_ONLY: [&str; 2] = ["--classes", "--java"];

impl BuildOpts {
    pub fn parse(mode: Mode, rest: &[String]) -> Result<BuildOpts, String> {
        let mut o = BuildOpts::default();
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
            if foreign || !(VALUED.contains(&a.as_str()) || FLAGS.contains(&a.as_str())) {
                return Err(format!("未知选项：{a}"));
            }
            if FLAGS.contains(&a.as_str()) {
                match a.as_str() {
                    "--clean" => o.clean = true,
                    "--no-run" => o.no_run = true,
                    "--batch" => o.batch = true,
                    "--debug" => o.debug = true,
                    "--precheck-only" => o.precheck_only = true,
                    "--api-recursive" => o.api_recursive = true,
                    "--perf" => o.perf = true,
                    _ => o.strict = true,
                }
                continue;
            }
            let v = it.next().ok_or_else(|| format!("{a} 缺少取值"))?;
            match a.as_str() {
                "--jdk" => o.jdk = Some(v.parse().map_err(|_| format!("--jdk 需为数字：{v}"))?),
                "--emit-jobs" => o.emit_jobs = v.parse().map_err(|_| format!("--emit-jobs 需为数字：{v}"))?,
                "--java-home" => o.java_home = Some(PathBuf::from(v)),
                "--runtime" => o.runtime = Some(PathBuf::from(v)),
                "--out" | "-o" => o.out = Some(PathBuf::from(v)),
                "--main" => o.main = Some(v.replace('.', "/")),
                "--classes" => o.classes = Some(PathBuf::from(v)),
                "--java" => o.java.push(PathBuf::from(v)),
                "--image" => o.images.push(PathBuf::from(v)),
                "--locale" => o.locales.push(v.clone()),
                "--lib" => o.libs.push(LibSpec::parse(v)?),
                "--trace-class" => o.trace_class = Some(v.clone()),
                "--raw-sites" => o.raw_sites = Some(PathBuf::from(v)),
                "--api-package" => o.api_packages.push(v.clone()),
                _ => o.roots.push(v.clone()),
            }
        }
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
        if !self.libs.is_empty() && self.batch {
            return Err("jar 输入模式（--lib）不支持 --batch（单 bin 消费形态）".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        if let Some(d) = self.libs.iter().find(|l| !seen.insert(l.name.as_str())) {
            return Err(format!("--lib crate 名重复：{}", d.name));
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
    /// `<scratch>/closure_input/closure.json` 布局（`rava build` / main.py 产物）的 scratch
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

/// scratch 内闭包输入目录（用户类 `classes/` + `closure.json`；与 main.py 同布局）
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
            &args("A.java B.java --jdk 21 --image /i1 --image /i2 --locale fr --clean --no-run --main p.Main"),
        )
        .unwrap();
        assert_eq!(o.inputs, vec![PathBuf::from("A.java"), PathBuf::from("B.java")]);
        assert_eq!(o.jdk, Some(21));
        assert_eq!(o.images, vec![PathBuf::from("/i1"), PathBuf::from("/i2")]);
        assert_eq!(o.locales, vec!["fr".to_string()]);
        assert!(o.clean && o.no_run && !o.strict);
        assert_eq!(o.main.as_deref(), Some("p/Main"));
    }

    #[test]
    fn build_rejects_bad_input() {
        assert!(BuildOpts::parse(Mode::Build, &args("--no-run")).is_err());
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
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --no-run")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("a.json b.json")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("")).is_err());
    }

    #[test]
    fn lib_specs_and_diagnostic_flags() {
        let o = BuildOpts::parse(
            Mode::Build,
            &args("A.java --lib h=/j/h.jar --lib ju=/j/ju.jar:seed=org.junit.Assert,,org.junit.Test --trace-class java/net/X --debug --precheck-only --raw-sites /tmp/r.txt"),
        )
        .unwrap();
        assert_eq!(o.libs.len(), 2);
        assert_eq!(o.libs[0], LibSpec { name: "h".into(), jar: PathBuf::from("/j/h.jar"), seeds: None });
        assert_eq!(o.libs[1].seeds, Some(vec!["org/junit/Assert".to_string(), "org/junit/Test".to_string()]));
        assert_eq!(o.trace_class.as_deref(), Some("java/net/X"));
        assert!(o.debug && o.precheck_only && !o.batch);
        assert_eq!(o.raw_sites, Some(PathBuf::from("/tmp/r.txt")));
        for bad in [
            "A.java --lib h",
            "A.java --lib =/j.jar",
            "A.java --lib h=",
            "A.java --lib h=/j.jar:seed=,",
            "A.java --lib h=/a.jar --lib h=/b.jar",
            "A.java --lib h=/a.jar --batch",
        ] {
            assert!(BuildOpts::parse(Mode::Build, &args(bad)).is_err(), "{bad}");
        }
        assert!(BuildOpts::parse(Mode::Build, &args("A.java --batch")).unwrap().batch);
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --lib h=/a.jar")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --trace-class X")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --debug --precheck-only")).is_ok());
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
