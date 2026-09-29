//! JDK 语料定位（与 `codegen/jdk_resolver.py` 的 find_java_home / _installed_jdks 同规则）。

use std::path::{Path, PathBuf};

fn major_of(home: &Path) -> Option<u32> {
    let s = home.to_string_lossy();
    for pat in ["openjdk@", "Cellar/openjdk/", "java-", "/openjdk-"] {
        if let Some(i) = s.find(pat) {
            let digits: String = s[i + pat.len()..].chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(v) = digits.parse() {
                return Some(v);
            }
        }
    }
    None
}

/// 本机已安装、含 jmods/ 的 JDK：(主版本, JAVA_HOME)，版本升序
pub fn installed_jdks() -> Vec<(u32, PathBuf)> {
    let mut out = Vec::new();
    let mut add = |home: PathBuf| {
        if home.join("jmods").is_dir() {
            if let Some(m) = major_of(&home) {
                out.push((m, home));
            }
        }
    };
    if let Ok(rd) = std::fs::read_dir("/opt/homebrew/Cellar") {
        for f in rd.flatten() {
            if !f.file_name().to_string_lossy().starts_with("openjdk") {
                continue;
            }
            if let Ok(vs) = std::fs::read_dir(f.path()) {
                for v in vs.flatten() {
                    add(v.path().join("libexec/openjdk.jdk/Contents/Home"));
                }
            }
        }
    }
    if let Ok(rd) = std::fs::read_dir("/usr/lib/jvm") {
        for j in rd.flatten() {
            add(j.path());
        }
    }
    out.sort();
    out
}

/// 优先级：显式主版本（精确 → 最小上界）→ JAVA_HOME → 已安装最高版本
pub fn find_java_home(prefer_major: Option<u32>) -> Option<PathBuf> {
    if let Some(m) = prefer_major {
        let all = installed_jdks();
        if let Some((_, h)) = all.iter().find(|(v, _)| *v == m).or_else(|| all.iter().find(|(v, _)| *v >= m)) {
            return Some(h.clone());
        }
    }
    if let Ok(h) = std::env::var("JAVA_HOME") {
        let p = PathBuf::from(h);
        if p.join("jmods").is_dir() {
            return Some(p);
        }
    }
    installed_jdks().pop().map(|(_, h)| h)
}
