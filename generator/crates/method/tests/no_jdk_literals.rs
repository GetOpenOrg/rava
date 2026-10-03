//! 守卫：method 生成器源码不以字面量写 JDK 类名（项目规则「生成器代码里不写类名特判」，
//! 类型信息一律来自字节码 / 清单 / `ty::consts` 等锚点常量）。
//!
//! 扫描 `src/**/*.rs` 的**代码**中的字符串字面量（注释不算），以下形态判为违规：
//! - 以 `java/` / `jdk/` / `sun/` 开头（binary 名）；
//! - 恰为 JDK 短名 `String` / `Object` / `ArrayList` / `HashMap` / `System` / `Class`。
//!
//! 放行：
//! - `#[cfg(test)]` 之后的测试模块（到该模块的闭合花括号为止）；
//! - [`ALLOW`] 行白名单：（相对 `src/` 的路径, 该行须包含的片段, 理由）。新增条目须写明理由，
//!   且只用于「该名字是 Rust 侧锚点 / 语言关键字而非 JDK 类特判」这类情形。

use std::path::{Path, PathBuf};

/// 当前工作区的包目录：取运行期 `CARGO_MANIFEST_DIR`（cargo 按本次调用设置）。编译期 `env!` 在全机共享的
/// CARGO_TARGET_DIR 下可能指向另一工作区——cargo 对路径包按工作区相对路径算 metadata，源码相同时不重编，
/// 测试二进制里嵌的就是首次编译它的（可能已删除的）worktree
fn manifest_dir() -> PathBuf {
    std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

const ALLOW: &[(&str, &str, &str)] = &[];

const BINARY_PREFIXES: &[&str] = &["java/", "jdk/", "sun/"];
const SHORT_NAMES: &[&str] = &["String", "Object", "ArrayList", "HashMap", "System", "Class"];

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            rs_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// 一行代码里的字符串字面量（`//` 之后为注释；跨行字面量按行内片段处理）
fn literals(line: &str) -> Vec<String> {
    let b: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            '/' if b.get(i + 1) == Some(&'/') => break,
            '\'' => {
                // 字符字面量 `'x'` / `'\n'`；生命周期 `'a` 无闭合引号，原样跳过
                let close = if b.get(i + 1) == Some(&'\\') { i + 3 } else { i + 2 };
                i = if b.get(close) == Some(&'\'') { close + 1 } else { i + 1 };
            }
            '"' => {
                let mut s = String::new();
                i += 1;
                while i < b.len() && b[i] != '"' {
                    if b[i] == '\\' {
                        i += 1;
                    }
                    if let Some(&c) = b.get(i) {
                        s.push(c);
                    }
                    i += 1;
                }
                out.push(s);
                i += 1;
            }
            _ => i += 1,
        }
    }
    out
}

fn violates(lit: &str) -> bool {
    BINARY_PREFIXES.iter().any(|p| lit.starts_with(p)) || SHORT_NAMES.contains(&lit)
}

#[test]
fn no_jdk_class_literals_in_src() {
    let src = manifest_dir().join("src");
    assert!(src.is_dir(), "源码目录不存在：{}", src.display());
    let mut files = Vec::new();
    rs_files(&src, &mut files);
    assert!(!files.is_empty(), "未找到 {}", src.display());
    let mut bad = Vec::new();
    for f in &files {
        let rel = f.strip_prefix(&src).unwrap_or(f).to_string_lossy().replace('\\', "/");
        let text = std::fs::read_to_string(f).unwrap_or_default();
        // 测试模块：`#[cfg(test)]` 后第一个 `{` 起按花括号深度跳过
        let (mut in_test, mut armed, mut depth) = (false, false, 0i32);
        for (n, line) in text.lines().enumerate() {
            if line.trim_start().starts_with("#[cfg(test)]") {
                armed = true;
                continue;
            }
            if armed || in_test {
                let opens = line.matches('{').count() as i32;
                let closes = line.matches('}').count() as i32;
                if armed && opens > 0 {
                    (armed, in_test) = (false, true);
                }
                if in_test {
                    depth += opens - closes;
                    if depth <= 0 {
                        (in_test, depth) = (false, 0);
                    }
                    continue;
                }
            }
            for lit in literals(line) {
                if !violates(&lit) {
                    continue;
                }
                if ALLOW.iter().any(|(p, frag, _)| *p == rel && line.contains(frag)) {
                    continue;
                }
                bad.push(format!("src/{rel}:{}: \"{lit}\"  | {}", n + 1, line.trim()));
            }
        }
    }
    assert!(
        bad.is_empty(),
        "method 源码出现 JDK 类名字面量（改用字节码 / 清单 / ty::consts 锚点；确属非类名用途时登记 ALLOW 并写明理由）：\n{}",
        bad.join("\n")
    );
}

#[test]
fn literal_scanner() {
    assert_eq!(literals(r#"let a = "java/lang/String"; // "sun/x""#), vec!["java/lang/String"]);
    assert_eq!(literals(r#"f('"', "a\"b", 'x')"#), vec!["a\"b"]);
    assert_eq!(literals("fn f<'a>(x: &'a str) {}"), Vec::<String>::new());
    assert!(violates("jdk/internal/Foo") && violates("Object") && !violates("Objects") && !violates("into"));
}
