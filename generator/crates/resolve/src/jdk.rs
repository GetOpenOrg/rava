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

/// brew Cellar 目录：`HOMEBREW_PREFIX` 置顶，Apple Silicon / Intel 两个缺省前缀，去重
fn cellars() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = Vec::new();
    let env = std::env::var("HOMEBREW_PREFIX").ok().filter(|p| !p.is_empty());
    for p in env.iter().map(String::as_str).chain(["/opt/homebrew", "/usr/local"]) {
        let c = Path::new(p).join("Cellar");
        if !out.contains(&c) {
            out.push(c);
        }
    }
    out
}

fn sorted_entries(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir).map(|rd| rd.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    v.sort();
    v
}

/// 本机已安装、含 jmods/ 的 JDK：(主版本, home)，按 (版本, 路径) 升序
pub fn installed_jdks() -> Vec<(u32, PathBuf)> {
    let mut out: Vec<(u32, PathBuf)> = Vec::new();
    let mut add = |home: PathBuf| {
        if is_jdk(&home) && !out.iter().any(|(_, h)| *h == home) {
            if let Some(m) = major_of(&home) {
                out.push((m, home));
            }
        }
    };
    for cellar in cellars() {
        for formula in sorted_entries(&cellar) {
            if !formula.file_name().is_some_and(|n| n.to_string_lossy().starts_with("openjdk")) {
                continue;
            }
            for v in sorted_entries(&formula) {
                add(v.join("libexec/openjdk.jdk/Contents/Home"));
            }
        }
    }
    for j in sorted_entries(Path::new("/usr/lib/jvm")) {
        add(j);
    }
    out.sort();
    out
}

/// macOS `java_home -v N` 兜底（该命令对不存在的版本可能回退缺省 JVM，须以 release 校验主版本）
fn macos_java_home(major: u32) -> Option<PathBuf> {
    let o = Command::new("/usr/libexec/java_home").args(["-v", &major.to_string()]).output().ok()?;
    let home = PathBuf::from(String::from_utf8_lossy(&o.stdout).trim());
    (o.status.success() && is_jdk(&home) && release_major(&home) == Some(major)).then_some(home)
}

/// 按主版本精确定位（已安装扫描 → macOS `java_home` 兜底）；找不到为 None
pub fn find_major(major: u32) -> Option<PathBuf> {
    installed_jdks().into_iter().find(|(v, _)| *v == major).map(|(_, h)| h).or_else(|| macos_java_home(major))
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

fn need(major: u32, source: JdkSource) -> Result<JdkChoice, String> {
    match find_major(major) {
        Some(home) => Ok(JdkChoice { major: Some(major), home, source }),
        None => {
            let list: Vec<String> =
                installed_jdks().iter().map(|(m, h)| format!("  JDK {m}: {}", h.display())).collect();
            let list = if list.is_empty() { "  （无）".to_string() } else { list.join("\n") };
            Err(format!("未找到 JDK {major}（{source}）。本机已安装：\n{list}"))
        }
    }
}

/// 按优先级选择 JDK。`repo` 为仓库根（读 `.jdk-version`；None 时跳过该级）
pub fn choose(jdk: Option<u32>, java_home: Option<&Path>, repo: Option<&Path>) -> Result<JdkChoice, String> {
    if let Some(m) = jdk {
        return need(m, JdkSource::JdkFlag);
    }
    if let Some(h) = java_home {
        if !is_jdk(h) {
            return Err(format!("--java-home {} 不是含 jmods/ 的 JDK", h.display()));
        }
        return Ok(JdkChoice { major: major_of(h), home: h.to_path_buf(), source: JdkSource::JavaHomeFlag });
    }
    let env_home = std::env::var_os("JAVA_HOME").map(PathBuf::from)
        .filter(|h| !h.as_os_str().is_empty() && is_jdk(h));
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
        return need(m, JdkSource::Pinned);
    }
    if let Some(h) = env_home {
        return Ok(JdkChoice { major: major_of(&h), home: h, source: JdkSource::JavaHomeEnv });
    }
    let (m, home) = installed_jdks().pop().ok_or("未找到任何已安装的 JDK（含 jmods/），请安装 JDK 21 或设置 JAVA_HOME")?;
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
    fn explicit_missing_major_is_error() {
        let e = choose(Some(1), None, None).unwrap_err();
        assert!(e.contains("未找到 JDK 1（--jdk）"), "{e}");
    }

    #[test]
    fn installed_are_jdks_with_known_major() {
        for (m, h) in installed_jdks() {
            assert!(is_jdk(&h) && major_of(&h) == Some(m), "{}", h.display());
        }
    }
}

    #[test]
    fn choose_pin_wins_over_mismatched_java_home() {
        // 裁决规则（2026-10-05）：pin 是语料唯一真源。JAVA_HOME 主版本与 pin 不符 → 忽略
        // JAVA_HOME 按 pin 选；一致 → 采用 JAVA_HOME 的 home；无 pin → JAVA_HOME 照常生效。
        // 隔离测试环境：清掉可能存在的真实 JAVA_HOME / pin，构造临时仓库与假安装表。
        let tmp = std::env::temp_dir().join(format!("rava_jdk_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        let pin_repo = tmp.join("pinned");
        std::fs::create_dir_all(&pin_repo).unwrap();
        std::fs::write(pin_repo.join(PIN_FILE), "21\n").unwrap();

        // 伪 JDK home：release 文件声明主版本，jmods/ 目录满足 is_jdk
        let fake = |major: u32| -> PathBuf {
            let d = tmp.join(format!("jdk{major}"));
            std::fs::create_dir_all(d.join("jmods")).unwrap();
            std::fs::write(d.join("release"), format!("JAVA_VERSION=\"{major}\"\n")).unwrap();
            d
        };
        let j25 = fake(25);

        // 不符：JAVA_HOME=25 + pin=21 → 按 pin 选 21（不取 25 的 home）
        unsafe { std::env::set_var("JAVA_HOME", &j25) };
        let r = choose(None, None, Some(&pin_repo));
        assert!(r.is_ok(), "pin 21 应能选出（本机有 21 或回退发现）: {r:?}");
        if let Ok(c) = r {
            assert_eq!(c.major, Some(21), "pin 优先于主版本不符的 JAVA_HOME");
            assert_ne!(c.home, j25, "不符的 JAVA_HOME home 不被采用");
        }

        // 一致：JAVA_HOME 指向假 21 → 采用该 home（保留同版本发行版选择权）
        let j21 = fake(21);
        unsafe { std::env::set_var("JAVA_HOME", &j21) };
        let c = choose(None, None, Some(&pin_repo)).expect("一致时应选出");
        assert_eq!(c.home, j21, "主版本一致时采用 JAVA_HOME 的 home");
        assert!(matches!(c.source, JdkSource::JavaHomeEnv));

        // 无 pin：JAVA_HOME 照常生效（重新指向 25 的假 home）
        unsafe { std::env::set_var("JAVA_HOME", &j25) };
        let no_pin = tmp.join("nopin");
        std::fs::create_dir_all(&no_pin).unwrap();
        let c = choose(None, None, Some(&no_pin)).expect("无 pin 时 JAVA_HOME 生效");
        assert_eq!(c.home, j25, "无 pin 时 JAVA_HOME（25 假 home）照常生效");

        unsafe { std::env::remove_var("JAVA_HOME") };
        let _ = std::fs::remove_dir_all(&tmp);
    }
