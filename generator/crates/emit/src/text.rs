//! 文本工具（`emitter/attrs.py` `to_snake` / `pkg_from_java`、`constants.scratch_pkg_version`、
//! Python `repr(float)` 等的移植）。

use std::path::Path;

use ty::ident::is_rust_keyword;

/// PascalCase / camelCase → snake_case（Rust 模块文件名）：`$` → `_`，
/// `([A-Z]+)([A-Z][a-z])` → `\1_\2`，`([a-z\d])([A-Z])` → `\1_\2`，小写，关键字加 `_` 后缀
pub fn to_snake(name: &str) -> String {
    let chars: Vec<char> = name.replace('$', "_").chars().collect();
    // 第一趟：大写串后接「大写+小写」处切分（Python re.sub 从左到右不重叠匹配）
    let mut s1: Vec<char> = Vec::with_capacity(chars.len() + 4);
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_uppercase() {
            let mut j = i;
            while j < chars.len() && chars[j].is_ascii_uppercase() {
                j += 1;
            }
            // 贪婪 [A-Z]+ 回溯：需要其后 [A-Z][a-z]，即 j-1 为大写、j 为小写，且 [A-Z]+ 至少一个
            if j < chars.len() && chars[j].is_ascii_lowercase() && j - i >= 2 {
                s1.extend_from_slice(&chars[i..j - 1]);
                s1.push('_');
                s1.push(chars[j - 1]);
                s1.push(chars[j]);
                i = j + 1;
                continue;
            }
            s1.extend_from_slice(&chars[i..j]);
            i = j;
            continue;
        }
        s1.push(chars[i]);
        i += 1;
    }
    // 第二趟：`([a-z\d])([A-Z])`
    let mut out = String::with_capacity(s1.len() + 4);
    let mut k = 0;
    while k < s1.len() {
        let c = s1[k];
        out.push(c);
        if (c.is_ascii_lowercase() || c.is_ascii_digit()) && k + 1 < s1.len() && s1[k + 1].is_ascii_uppercase() {
            out.push('_');
            out.push(s1[k + 1]);
            k += 2;
            continue;
        }
        k += 1;
    }
    let lower = out.to_lowercase();
    if is_rust_keyword(&lower) {
        lower + "_"
    } else {
        lower
    }
}

/// 从 .java 源文件读取 package 声明，无则空串
pub fn pkg_from_java(java_file: &Path) -> String {
    let Ok(text) = std::fs::read_to_string(java_file) else {
        return String::new();
    };
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("package ") {
            return rest.trim_end_matches(';').trim().to_string();
        }
        if !line.is_empty() && !line.starts_with("//") && !line.starts_with("/*") {
            let stop = ["import ", "public ", "class ", "@"].iter().any(|k| line.starts_with(k));
            if stop {
                break;
            }
        }
    }
    String::new()
}

/// `os.path.abspath`：绝对化并按词法消去 `.` / `..`（不解析符号链接）
pub fn lexical_abspath(dir: &Path) -> std::path::PathBuf {
    use std::path::Component;
    let abs = std::path::absolute(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut out = std::path::PathBuf::new();
    for c in abs.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out
}

/// scratch 包版本：`0.0.<crc32(abspath)>`
pub fn scratch_pkg_version(dir: &Path) -> String {
    let crc = crc32fast::hash(lexical_abspath(dir).to_string_lossy().as_bytes());
    format!("0.0.{crc}")
}

/// 关键字包段 `r#` 转义
pub fn safe_pkg_part(p: &str) -> String {
    if is_rust_keyword(p) {
        format!("r#{p}")
    } else {
        p.to_string()
    }
}

/// 按顶层逗号切分（`<(` 与 `>)` 计深度）；尾段全空白时丢弃
pub fn split_top_level(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            _ => {}
        }
        if ch == ',' && depth == 0 {
            parts.push(std::mem::take(&mut cur));
        } else {
            cur.push(ch);
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur);
    }
    parts
}

/// 字节序列 → 小写十六进制
pub fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Python `repr(float)`：最短往返表示；指数 < -4 或 ≥ 16 用科学计数（`1e-05` / `1e+16`）
pub fn py_float_repr(v: f64) -> String {
    if v.is_nan() {
        return "nan".into();
    }
    if v.is_infinite() {
        return if v > 0.0 { "inf".into() } else { "-inf".into() };
    }
    if v == 0.0 {
        return if v.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    // Rust `{:e}` 给出最短往返的有效数字
    let e = format!("{v:e}");
    let (mant, exp) = e.split_once('e').unwrap_or((&e, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let neg = mant.starts_with('-');
    let digits: String = mant.chars().filter(char::is_ascii_digit).collect();
    let sign = if neg { "-" } else { "" };
    if !(-4..16).contains(&exp) {
        let m = if digits.len() > 1 {
            format!("{}.{}", &digits[..1], &digits[1..])
        } else {
            digits.clone()
        };
        let es = if exp < 0 { format!("-{:02}", -exp) } else { format!("+{exp:02}") };
        return format!("{sign}{m}e{es}");
    }
    let n = digits.len() as i32;
    let body = if exp < 0 {
        format!("0.{}{}", "0".repeat((-exp - 1) as usize), digits)
    } else if exp + 1 >= n {
        format!("{}{}.0", digits, "0".repeat((exp + 1 - n) as usize))
    } else {
        let p = (exp + 1) as usize;
        format!("{}.{}", &digits[..p], &digits[p..])
    };
    format!("{sign}{body}")
}

/// `\bword\b` 检索（单词字符 = 字母数字与 `_`，与 Python `re` 的 ASCII 语义一致）
pub fn contains_word(hay: &str, word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    let is_w = |c: char| c.is_alphanumeric() || c == '_';
    hay.match_indices(word).any(|(i, _)| {
        let before = hay[..i].chars().next_back().is_none_or(|c| !is_w(c));
        let after = hay[i + word.len()..].chars().next().is_none_or(|c| !is_w(c));
        before && after
    })
}

/// 每行前加缩进（空行保持为空，与 Python `_indent` 同形）
pub fn indent(text: &str, pad: &str) -> String {
    text.split('\n')
        .map(|l| if l.trim().is_empty() { String::new() } else { format!("{pad}{l}") })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 源文本中的 `pub fn` 名（`\bpub fn\s+(\w+)\s*[(<]`）
pub fn pub_fn_names(text: &str) -> Vec<String> {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| regex::Regex::new(r"\bpub fn\s+(\w+)\s*[(<]").expect("静态正则"));
    re.captures_iter(text).map(|c| c[1].to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_cases() {
        assert_eq!(to_snake("TestHashMapOps"), "test_hash_map_ops");
        assert_eq!(to_snake("HashMap$TreeNode"), "hash_map_tree_node");
        assert_eq!(to_snake("URLConnection"), "url_connection");
        assert_eq!(to_snake("ABCDef"), "abc_def");
        assert_eq!(to_snake("Ref"), "ref_");
        assert_eq!(to_snake("A1B"), "a1_b");
        assert_eq!(to_snake("UTF8Encoder"), "utf8_encoder");
    }

    #[test]
    fn abspath_is_lexical() {
        assert_eq!(lexical_abspath(Path::new("/a/b/../c/./d")), Path::new("/a/c/d"));
        assert_eq!(scratch_pkg_version(Path::new("/a/b/../c")), scratch_pkg_version(Path::new("/a/c")));
    }

    #[test]
    fn float_repr() {
        assert_eq!(py_float_repr(1.0), "1.0");
        assert_eq!(py_float_repr(0.1), "0.1");
        assert_eq!(py_float_repr(1e-5), "1e-05");
        assert_eq!(py_float_repr(1e16), "1e+16");
        assert_eq!(py_float_repr(123456.789), "123456.789");
        assert_eq!(py_float_repr(0.0001), "0.0001");
        assert_eq!(py_float_repr(1.7976931348623157e308), "1.7976931348623157e+308");
        assert_eq!(py_float_repr(4.9e-324), "5e-324");
        assert_eq!(py_float_repr(-2.5), "-2.5");
    }

    #[test]
    fn top_level_split() {
        assert_eq!(split_top_level("a: A<B, C>, d: D"), vec!["a: A<B, C>", " d: D"]);
    }
}
