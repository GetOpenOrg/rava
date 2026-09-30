//! `rava build` / `rava emit` 的参数解析（纯函数，单测覆盖）。
//!
//! - `rava build <A.java>… [--jdk N | --java-home P] [--runtime R] [--out DIR] [--main 类]
//!   [--image D]… [--locale L]… [--root 类.方法:描述符]… [--clean] [--no-run] [--skeleton-only] [--strict]`
//! - `rava emit <closure.json> [--classes DIR] [--jdk N | --java-home P] [--runtime R] [--out DIR]
//!   [--java A.java]… [--image D]… [--clean] [--skeleton-only] [--strict]`（`--java`：源文件，决定用户类包布局与入口序）

use std::path::{Path, PathBuf};

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
    pub clean: bool,
    pub no_run: bool,
    pub skeleton_only: bool,
    pub strict: bool,
}

const VALUED: [&str; 11] =
    ["--jdk", "--java-home", "--runtime", "--out", "--main", "--classes", "--java", "--image", "--locale", "--root", "-o"];
const FLAGS: [&str; 4] = ["--clean", "--no-run", "--skeleton-only", "--strict"];
/// 只属于 build 的选项
const BUILD_ONLY: [&str; 5] = ["--main", "--locale", "--root", "--no-run", "-o"];
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
                    "--skeleton-only" => o.skeleton_only = true,
                    _ => o.strict = true,
                }
                continue;
            }
            let v = it.next().ok_or_else(|| format!("{a} 缺少取值"))?;
            match a.as_str() {
                "--jdk" => o.jdk = Some(v.parse().map_err(|_| format!("--jdk 需为数字：{v}"))?),
                "--java-home" => o.java_home = Some(PathBuf::from(v)),
                "--runtime" => o.runtime = Some(PathBuf::from(v)),
                "--out" | "-o" => o.out = Some(PathBuf::from(v)),
                "--main" => o.main = Some(v.replace('.', "/")),
                "--classes" => o.classes = Some(PathBuf::from(v)),
                "--java" => o.java.push(PathBuf::from(v)),
                "--image" => o.images.push(PathBuf::from(v)),
                "--locale" => o.locales.push(v.clone()),
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
            &args("A.java B.java --jdk 21 --image /i1 --image /i2 --locale fr --clean --no-run --skeleton-only --main p.Main"),
        )
        .unwrap();
        assert_eq!(o.inputs, vec![PathBuf::from("A.java"), PathBuf::from("B.java")]);
        assert_eq!(o.jdk, Some(21));
        assert_eq!(o.images, vec![PathBuf::from("/i1"), PathBuf::from("/i2")]);
        assert_eq!(o.locales, vec!["fr".to_string()]);
        assert!(o.clean && o.no_run && o.skeleton_only && !o.strict);
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
        let o = BuildOpts::parse(Mode::Emit, &args("/s/closure_input/closure.json --skeleton-only --java A.java")).unwrap();
        assert_eq!(o.emit_classes_dir(), PathBuf::from("/s/closure_input/classes"));
        assert_eq!(o.java_files(Mode::Emit), &[PathBuf::from("A.java")]);
        assert!(BuildOpts::parse(Mode::Emit, &args("c.json --no-run")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("a.json b.json")).is_err());
        assert!(BuildOpts::parse(Mode::Emit, &args("")).is_err());
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
