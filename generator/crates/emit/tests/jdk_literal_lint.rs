//! 生成器代码不得以字面量形式出现 JDK 类名（CLAUDE.md 核心架构原则 4）。
//!
//! 扫描 `src/**/*.rs` 非测试代码的字符串字面量：JDK 包路径（`java/` `javax/` `jdk/` `sun/`
//! 起首，或点分 `java.` 起首）一律禁止。语言锚点集中在 `lang.rs`（豁免）；
//! `#[cfg(test)]` 之后的测试代码与注释行不扫描。

use std::fs;
use std::path::{Path, PathBuf};

/// 当前工作区的包目录：取运行期 `CARGO_MANIFEST_DIR`（cargo 按本次调用设置）。编译期 `env!` 在全机共享的
/// CARGO_TARGET_DIR 下可能指向另一工作区——cargo 对路径包按工作区相对路径算 metadata，源码相同时不重编，
/// 测试二进制里嵌的就是首次编译它的（可能已删除的）worktree
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

/// lang.rs：语言锚点；tests.rs：`#[cfg(test)] mod tests;` 的外置测试模块
const EXEMPT_FILES: [&str; 2] = ["lang.rs", "tests.rs"];
const FORBIDDEN_PREFIXES: [&str; 6] = ["java/", "javax/", "jdk/", "sun/", "java.", "javax."];

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// 行内字符串字面量内容（按 `"` 成对切分，忽略转义引号）
fn literals(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(s) = rest.find('"') {
        let after = &rest[s + 1..];
        let mut end = None;
        let bytes = after.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'\\' => i += 2,
                b'"' => {
                    end = Some(i);
                    break;
                }
                _ => i += 1,
            }
        }
        let Some(e) = end else { break };
        out.push(&after[..e]);
        rest = &after[e + 1..];
    }
    out
}

fn violations(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (no, line) in text.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("#[cfg(test)]") {
            break;
        }
        if t.starts_with("//") {
            continue;
        }
        let code = t.split(" //").next().unwrap_or(t);
        for lit in literals(code) {
            if FORBIDDEN_PREFIXES.iter().any(|p| lit.starts_with(p) || lit.contains(&format!("L{p}"))) {
                out.push((no + 1, lit.to_string()));
            }
        }
    }
    out
}

#[test]
fn no_jdk_class_literals_outside_lang() {
    let src = manifest_dir().join("src");
    assert!(src.is_dir(), "源码目录不存在：{}", src.display());
    let mut files = Vec::new();
    rs_files(&src, &mut files);
    assert!(!files.is_empty());
    let mut bad = Vec::new();
    for f in files {
        if f.file_name().is_some_and(|n| EXEMPT_FILES.iter().any(|x| n == *x)) {
            continue;
        }
        let text = fs::read_to_string(&f).unwrap();
        for (no, lit) in violations(&text) {
            bad.push(format!("{}:{no}: \"{lit}\"", f.strip_prefix(&src).unwrap().display()));
        }
    }
    assert!(bad.is_empty(), "JDK 类名字面量须经 lang.rs 锚点：\n{}", bad.join("\n"));
}

#[test]
fn lint_detects_literals() {
    assert_eq!(violations("let x = \"java/util/List\";").len(), 1);
    assert_eq!(violations("f(\"(Ljava/lang/Object;)V\")").len(), 1);
    assert!(violations("// \"java/lang\"").is_empty());
    assert!(violations("let x = \"crate::java::util\";").is_empty());
    assert!(violations("#[cfg(test)]\nlet x = \"java/x\";").is_empty());
}
