//! JDK 定位：按主版本找本机已安装的 JDK home，并按固定优先级选出本次运行的 JDK。
//!
//! 选择优先级（多 JDK 并存时不随系统缺省 java 或「最新已安装」漂移）：
//! 1. 显式 `--jdk N`（找不到精确主版本即报错，不取近似版本：语料与 javac 必须同源）
//! 2. 显式 `--java-home P`
//! 3. 仓库根 `.jdk-version` 固定的主版本（语料基线版本，语料的唯一真源）——与
//!    `JAVA_HOME` 的主版本一致时采用 `JAVA_HOME` 指定的 home；不一致时忽略
//!    `JAVA_HOME`（开发机 shell 环境变量不干扰语料）并按 pin 选，打印被忽略的版本
//! 4. 无 pin 时（如生产构建的用户项目）：已设置且有效（含 jmods/）的 `JAVA_HOME`
//! 5. 已安装的最新版
//!
//! 扫描面：brew Cellar（`HOMEBREW_PREFIX`、`/opt/homebrew`、`/usr/local`；`openjdk@NN` 与裸 `openjdk`）、
//! Linux `/usr/lib/jvm`；按主版本找不到时回退 macOS `/usr/libexec/java_home -v N`（以 release 文件校验主版本）。
//! 扫描面与 `JAVA_HOME` 由 [`JdkEnv`] 承载：生产入口取 [`JdkEnv::host`]，测试注入构造值（不改进程环境变量）。

use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// 仓库根固定缺省主版本的文件名
pub const PIN_FILE: &str = ".jdk-version";

/// `<home>/release` 的 `JAVA_VERSION` 主版本
pub fn release_major(home: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(home.join("release")).ok()?;
    let v = text.lines().find_map(|l| l.strip_prefix("JAVA_VERSION="))?.trim_matches('"');
    v.split(['.', '-', '+']).next()?.parse().ok()
}

/// 安装路径推断主版本（brew `openjdk@21/…`、`Cellar/openjdk/26.0.2`；Linux `java-21-openjdk-*`、`openjdk-21`）
fn path_major(home: &Path) -> Option<u32> {
    let s = home.to_string_lossy();
    ["openjdk@", "Cellar/openjdk/", "java-", "/openjdk-"].iter().find_map(|pat| {
        let i = s.find(pat)?;
        let digits: String = s[i + pat.len()..].chars().take_while(|c| c.is_ascii_digit()).collect();
        digits.parse().ok()
    })
}

/// 主版本：release 文件优先，路径命名兜底
pub fn major_of(home: &Path) -> Option<u32> {
    release_major(home).or_else(|| path_major(home))
}

fn is_jdk(home: &Path) -> bool {
    home.join("jmods").is_dir()
}

/// 选择所依据的宿主环境：`JAVA_HOME` 与 JDK 扫描面。生产入口取 [`JdkEnv::host`]（读进程环境变量与
/// 固定安装路径）；测试注入构造值，不改进程环境变量
#[derive(Debug, Clone, Default)]
pub struct JdkEnv {
    /// `JAVA_HOME`（空串视同未设置）
    pub java_home: Option<PathBuf>,
    /// brew Cellar 目录（`openjdk@NN` / 裸 `openjdk` 公式）
    pub cellars: Vec<PathBuf>,
    /// 平铺 JDK home 的目录（Linux `/usr/lib/jvm`）
    pub jvm_dirs: Vec<PathBuf>,
    /// 按主版本找不到时是否回退 macOS `/usr/libexec/java_home -v N`
    pub macos_java_home: bool,
}

impl JdkEnv {
    /// 本机环境：`JAVA_HOME`；Cellar 以 `HOMEBREW_PREFIX` 置顶，Apple Silicon / Intel 两个缺省前缀，去重
    pub fn host() -> Self {
        let mut cellars: Vec<PathBuf> = Vec::new();
        let prefix = std::env::var("HOMEBREW_PREFIX").ok().filter(|p| !p.is_empty());
        for p in prefix.iter().map(String::as_str).chain(["/opt/homebrew", "/usr/local"]) {
            let c = Path::new(p).join("Cellar");
            if !cellars.contains(&c) {
                cellars.push(c);
            }
        }
        JdkEnv {
            java_home: std::env::var_os("JAVA_HOME").filter(|h| !h.is_empty()).map(PathBuf::from),
            cellars,
            jvm_dirs: vec![PathBuf::from("/usr/lib/jvm")],
            macos_java_home: true,
        }
    }

    /// 已安装、含 jmods/ 的 JDK：(主版本, home)，按 (版本, 路径) 升序
    pub fn installed_jdks(&self) -> Vec<(u32, PathBuf)> {
        let mut out: Vec<(u32, PathBuf)> = Vec::new();
        let mut add = |home: PathBuf| {
            if is_jdk(&home) && !out.iter().any(|(_, h)| *h == home) {
                if let Some(m) = major_of(&home) {
                    out.push((m, home));
                }
            }
        };
        for cellar in &self.cellars {
            for formula in sorted_entries(cellar) {
                if !formula.file_name().is_some_and(|n| n.to_string_lossy().starts_with("openjdk")) {
                    continue;
                }
                for v in sorted_entries(&formula) {
                    add(v.join("libexec/openjdk.jdk/Contents/Home"));
                }
            }
        }
        for dir in &self.jvm_dirs {
            for j in sorted_entries(dir) {
                add(j);
            }
        }
        out.sort();
        out
    }

    /// 按主版本精确定位（已安装扫描 → macOS `java_home` 兜底）；找不到为 None
    pub fn find_major(&self, major: u32) -> Option<PathBuf> {
        self.installed_jdks()
            .into_iter()
            .find(|(v, _)| *v == major)
            .map(|(_, h)| h)
            .or_else(|| if self.macos_java_home { macos_java_home(major) } else { None })
    }
}

fn sorted_entries(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir).map(|rd| rd.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    v.sort();
    v
}

/// 本机已安装、含 jmods/ 的 JDK（[`JdkEnv::host`] 扫描面）
pub fn installed_jdks() -> Vec<(u32, PathBuf)> {
    JdkEnv::host().installed_jdks()
}

/// macOS `java_home -v N` 兜底（该命令对不存在的版本可能回退缺省 JVM，须以 release 校验主版本）
fn macos_java_home(major: u32) -> Option<PathBuf> {
    let o = Command::new("/usr/libexec/java_home").args(["-v", &major.to_string()]).output().ok()?;
    let home = PathBuf::from(String::from_utf8_lossy(&o.stdout).trim());
    (o.status.success() && is_jdk(&home) && release_major(&home) == Some(major)).then_some(home)
}

/// 本机按主版本精确定位（[`JdkEnv::host`]）；找不到为 None
pub fn find_major(major: u32) -> Option<PathBuf> {
    JdkEnv::host().find_major(major)
}

/// 本次选择的来源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JdkSource {
    JdkFlag,
    JavaHomeFlag,
    JavaHomeEnv,
    Pinned,
    Latest,
}

impl fmt::Display for JdkSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            JdkSource::JdkFlag => "--jdk",
            JdkSource::JavaHomeFlag => "--java-home",
            JdkSource::JavaHomeEnv => "JAVA_HOME",
            JdkSource::Pinned => PIN_FILE,
            JdkSource::Latest => "最新已安装",
        })
    }
}

/// 选中的 JDK
#[derive(Debug, Clone)]
pub struct JdkChoice {
    pub major: Option<u32>,
    pub home: PathBuf,
    pub source: JdkSource,
}

impl JdkChoice {
    /// 选择结果的一行说明
    pub fn describe(&self) -> String {
        let major = self.major.map_or_else(|| "?".to_string(), |m| m.to_string());
        format!("[jdk] JAVA_HOME → {} (JDK {major}，来源：{})", self.home.display(), self.source)
    }
}

/// 仓库根固定主版本（文件中首个整数）；缺失 / 不可解析为 None
pub fn pinned_major(repo: &Path) -> Option<u32> {
    let text = std::fs::read_to_string(repo.join(PIN_FILE)).ok()?;
    let digits: String = text.chars().skip_while(|c| !c.is_ascii_digit()).take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn need(env: &JdkEnv, major: u32, source: JdkSource) -> Result<JdkChoice, String> {
    match env.find_major(major) {
        Some(home) => Ok(JdkChoice { major: Some(major), home, source }),
        None => {
            let list: Vec<String> =
                env.installed_jdks().iter().map(|(m, h)| format!("  JDK {m}: {}", h.display())).collect();
            let list = if list.is_empty() { "  （无）".to_string() } else { list.join("\n") };
            Err(format!("未找到 JDK {major}（{source}）。本机已安装：\n{list}"))
        }
    }
}

/// 按优先级在本机环境（[`JdkEnv::host`]）选择 JDK。`repo` 为仓库根（读 `.jdk-version`；None 时跳过该级）
pub fn choose(jdk: Option<u32>, java_home: Option<&Path>, repo: Option<&Path>) -> Result<JdkChoice, String> {
    choose_in(&JdkEnv::host(), jdk, java_home, repo)
}

/// 按优先级在给定环境中选择 JDK（见模块文档）
pub fn choose_in(env: &JdkEnv, jdk: Option<u32>, java_home: Option<&Path>, repo: Option<&Path>) -> Result<JdkChoice, String> {
    if let Some(m) = jdk {
        return need(env, m, JdkSource::JdkFlag);
    }
    if let Some(h) = java_home {
        if !is_jdk(h) {
            return Err(format!("--java-home {} 不是含 jmods/ 的 JDK", h.display()));
        }
        return Ok(JdkChoice { major: major_of(h), home: h.to_path_buf(), source: JdkSource::JavaHomeFlag });
    }
    let env_home = env.java_home.clone().filter(|h| is_jdk(h));
    if let Some(m) = repo.and_then(pinned_major) {
        // pin 是语料的唯一真源：JAVA_HOME 主版本与 pin 一致时采用其 home（保留
        // 同版本的发行版选择权），不一致时忽略 JAVA_HOME（开发机 shell 环境变量
        // 不干扰语料）——语料与 javac 必须同源
        if let Some(h) = env_home {
            if major_of(&h) == Some(m) {
                return Ok(JdkChoice { major: Some(m), home: h, source: JdkSource::JavaHomeEnv });
            }
            eprintln!(
                "[jdk] JAVA_HOME {}（主版本 {}）与语料 pin {m} 不符，已忽略，按 .jdk-version 选择",
                h.display(),
                major_of(&h).map(|v| v.to_string()).unwrap_or_else(|| "未知".into())
            );
        }
        return need(env, m, JdkSource::Pinned);
    }
    if let Some(h) = env_home {
        return Ok(JdkChoice { major: major_of(&h), home: h, source: JdkSource::JavaHomeEnv });
    }
    let (m, home) = env.installed_jdks().pop().ok_or("未找到任何已安装的 JDK（含 jmods/），请安装 JDK 21 或设置 JAVA_HOME")?;
    Ok(JdkChoice { major: Some(m), home, source: JdkSource::Latest })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_major_patterns() {
        let m = |s: &str| path_major(Path::new(s));
        assert_eq!(m("/opt/homebrew/Cellar/openjdk@21/21.0.8/libexec/openjdk.jdk/Contents/Home"), Some(21));
        assert_eq!(m("/usr/local/Cellar/openjdk/26.0.2/libexec/openjdk.jdk/Contents/Home"), Some(26));
        assert_eq!(m("/usr/lib/jvm/java-25-openjdk-amd64"), Some(25));
        assert_eq!(m("/usr/lib/jvm/openjdk-17"), Some(17));
        assert_eq!(m("/somewhere/else"), None);
    }

    #[test]
    fn pinned_major_reads_first_integer() {
        let d = std::env::temp_dir().join(format!("rava-jdk-pin-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        assert_eq!(pinned_major(&d), None);
        std::fs::write(d.join(PIN_FILE), "# 语料基线\n21\n").unwrap();
        assert_eq!(pinned_major(&d), Some(21));
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn installed_are_jdks_with_known_major() {
        for (m, h) in installed_jdks() {
            assert!(is_jdk(&h) && major_of(&h) == Some(m), "{}", h.display());
        }
    }

    /// 临时目录下的隔离环境：`jvm/` 为唯一扫描面，`JAVA_HOME` 未设置，不回退 macOS `java_home`
    struct Sandbox {
        root: PathBuf,
        env: JdkEnv,
    }

    impl Sandbox {
        fn new(tag: &str) -> Self {
            // 前缀 sb- 与其他测试的临时目录隔开（曾与 pinned_major_reads_first_integer 的 rava-jdk-pin-<pid> 同名，并行时被删）
            let root = std::env::temp_dir().join(format!("rava-jdk-sb-{tag}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("jvm")).unwrap();
            let env = JdkEnv { java_home: None, cellars: Vec::new(), jvm_dirs: vec![root.join("jvm")], macos_java_home: false };
            Sandbox { root, env }
        }

        /// 伪 JDK home：release 声明主版本，jmods/ 满足 is_jdk；`installed` 为真时放进扫描面
        fn jdk(&self, major: u32, installed: bool) -> PathBuf {
            let d = self.root.join(if installed { "jvm" } else { "other" }).join(format!("jdk{major}"));
            std::fs::create_dir_all(d.join("jmods")).unwrap();
            std::fs::write(d.join("release"), format!("JAVA_VERSION=\"{major}\"\n")).unwrap();
            d
        }

        fn repo(&self, pin: Option<u32>) -> PathBuf {
            let d = self.root.join(format!("repo-{}", pin.map_or("none".into(), |m| m.to_string())));
            std::fs::create_dir_all(&d).unwrap();
            if let Some(m) = pin {
                std::fs::write(d.join(PIN_FILE), format!("{m}\n")).unwrap();
            }
            d
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn explicit_missing_major_is_error() {
        let sb = Sandbox::new("missing");
        sb.jdk(21, true);
        let e = choose_in(&sb.env, Some(17), None, None).unwrap_err();
        assert!(e.contains("未找到 JDK 17（--jdk）") && e.contains("JDK 21:"), "{e}");
        assert_eq!(choose_in(&sb.env, Some(21), None, None).unwrap().source, JdkSource::JdkFlag);
    }

    #[test]
    fn choose_pin_wins_over_mismatched_java_home() {
        // 裁决规则（2026-10-05）：pin 是语料唯一真源。JAVA_HOME 主版本与 pin 不符 → 忽略
        // JAVA_HOME 按 pin 选；一致 → 采用 JAVA_HOME 的 home；无 pin → JAVA_HOME 照常生效。
        // 环境经 JdkEnv 注入，不读写进程环境变量，不依赖本机已安装的 JDK
        let mut sb = Sandbox::new("pin");
        let installed21 = sb.jdk(21, true);
        let other21 = sb.jdk(21, false);
        let j25 = sb.jdk(25, false);
        let pinned = sb.repo(Some(21));

        // 不符：JAVA_HOME=25 + pin=21 → 按 pin 选扫描面里的 21
        sb.env.java_home = Some(j25.clone());
        let c = choose_in(&sb.env, None, None, Some(&pinned)).unwrap();
        assert_eq!((c.major, &c.home, &c.source), (Some(21), &installed21, &JdkSource::Pinned));

        // 一致：JAVA_HOME 指向扫描面外的 21 → 采用该 home（保留同版本发行版选择权）
        sb.env.java_home = Some(other21.clone());
        let c = choose_in(&sb.env, None, None, Some(&pinned)).unwrap();
        assert_eq!((&c.home, &c.source), (&other21, &JdkSource::JavaHomeEnv));

        // 无 pin：JAVA_HOME 照常生效
        sb.env.java_home = Some(j25.clone());
        let c = choose_in(&sb.env, None, None, Some(&sb.repo(None))).unwrap();
        assert_eq!((&c.home, &c.source), (&j25, &JdkSource::JavaHomeEnv));

        // 无 pin 且无 JAVA_HOME：取扫描面最新
        sb.env.java_home = None;
        sb.jdk(25, true);
        let c = choose_in(&sb.env, None, None, Some(&sb.repo(None))).unwrap();
        assert_eq!((c.major, &c.source), (Some(25), &JdkSource::Latest));
    }
}
