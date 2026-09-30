//! 文本工具：Python 侧依赖字符串形态的判定（副作用启发、局部变量引用扫描、`as` 原子性）、
//! Python `repr(float)` 口径、`to_snake` 与手写源码的 `fn` 名扫描。
//!
//! 这些函数只作用于**渲染后的文本**做判定（与 Python 同一口径），不回灌为表达式。

use std::collections::BTreeSet;

/// Python `repr(float)`（短表示，`-4 < decpt <= 16` 用定点，否则科学计数、指数至少两位）
pub fn py_float_repr(x: f64) -> String {
    if x.is_nan() {
        return "nan".to_string();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0" } else { "0.0" }.to_string();
    }
    let sci = format!("{:e}", x.abs());
    let (mant, exp) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let digits: String = mant.chars().filter(char::is_ascii_digit).collect();
    let sign = if x < 0.0 { "-" } else { "" };
    let decpt = exp + 1;
    let body = if (-4 < decpt) && (decpt <= 16) {
        if decpt <= 0 {
            format!("0.{}{}", "0".repeat(decpt.unsigned_abs() as usize), digits)
        } else {
            let d = decpt as usize;
            if digits.len() <= d {
                format!("{}{}.0", digits, "0".repeat(d - digits.len()))
            } else {
                format!("{}.{}", &digits[..d], &digits[d..])
            }
        }
    } else {
        let m = if digits.len() > 1 { format!("{}.{}", &digits[..1], &digits[1..]) } else { digits.clone() };
        let es = if exp < 0 { '-' } else { '+' };
        format!("{m}e{es}{:02}", exp.unsigned_abs())
    };
    format!("{sign}{body}")
}

/// PascalCase / camelCase → snake_case（`emitter/attrs.to_snake`：`$` → `_`，关键字加 `_`）
pub fn to_snake(name: &str) -> String {
    let chars: Vec<char> = name.replace('$', "_").chars().collect();
    let mut out = String::with_capacity(chars.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(char::is_ascii_lowercase);
            // `([A-Z]+)([A-Z][a-z])` 与 `([a-z\d])([A-Z])` 两条规则
            if (prev.is_ascii_uppercase() && next_lower) || prev.is_ascii_lowercase() || prev.is_ascii_digit() {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    if ty::ident::is_rust_keyword(&out) {
        out.push('_');
    }
    out
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Rust 源码中的函数名：`pub_only` 时匹配 `\bpub fn\s+(\w+)\s*[(<]`，
/// 否则匹配 `fn (\w+)\s*[(<]`（`fn` 后恰一个空格，与 Python 探测正则一致）
pub fn scan_fn_names(src: &str, pub_only: bool) -> BTreeSet<String> {
    let key = if pub_only { "pub fn" } else { "fn " };
    let mut out = BTreeSet::new();
    let mut from = 0;
    while let Some(pos) = src[from..].find(key) {
        let at = from + pos;
        from = at + key.len();
        if pub_only && src[..at].chars().next_back().is_some_and(is_word) {
            continue;
        }
        let mut rest = &src[from..];
        if pub_only {
            let t = rest.trim_start();
            if t.len() == rest.len() {
                continue;
            }
            rest = t;
        }
        let end = rest.find(|c: char| !is_word(c)).unwrap_or(rest.len());
        if end == 0 {
            continue;
        }
        if rest[end..].trim_start().starts_with(['(', '<']) {
            out.insert(rest[..end].to_string());
        }
    }
    out
}

/// 弹出值是否可能有副作用（Python `any(c in e for c in ['(', 'push', 'insert'])`）
pub fn looks_effectful(rendered: &str) -> bool {
    rendered.contains('(') || rendered.contains("push") || rendered.contains("insert")
}

/// 表达式文本是否在字符串 / 字符字面量之外以整词引用 `name`（`locals._refs_local`）
pub fn refs_local(code: &str, name: &str) -> bool {
    let b: Vec<char> = code.chars().collect();
    let n = b.len();
    let mut i = 0;
    while i < n {
        let c = b[i];
        if c == '"' || c == '\'' {
            i += 1;
            while i < n && b[i] != c {
                i += if b[i] == '\\' { 2 } else { 1 };
            }
            i += 1;
        } else if c.is_alphabetic() || c == '_' {
            let j = (i..n).find(|&j| !is_word(b[j])).unwrap_or(n);
            if b[i..j].iter().collect::<String>() == name {
                return true;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    false
}

/// 可直接作 `as` 左操作数（`coerce._is_atomic_expr`：括号外只含标识符 / 字面量 / 路径 / 调用链字符）
pub fn is_atomic(rendered: &str) -> bool {
    let mut depth = 0i32;
    for ch in rendered.trim().chars() {
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            c if depth == 0 && !(c.is_alphanumeric() || matches!(c, '_' | '.' | ':' | '?')) => return false,
            _ => {}
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_repr() {
        for (x, s) in [
            (1.0, "1.0"),
            (0.5, "0.5"),
            (1e16, "1e+16"),
            (1e15, "1000000000000000.0"),
            (1e-5, "1e-05"),
            (0.0001, "0.0001"),
            (-2.5e-7, "-2.5e-07"),
            (f64::from(0.1f32), "0.10000000149011612"),
            (1.7976931348623157e308, "1.7976931348623157e+308"),
            (123.456, "123.456"),
        ] {
            assert_eq!(py_float_repr(x), s);
        }
        assert_eq!(py_float_repr(f64::NAN), "nan");
        assert_eq!(py_float_repr(f64::NEG_INFINITY), "-inf");
    }

    #[test]
    fn snake() {
        assert_eq!(to_snake("HashMap"), "hash_map");
        assert_eq!(to_snake("URLDecoder"), "url_decoder");
        assert_eq!(to_snake("Map$Entry"), "map_entry");
        assert_eq!(to_snake("Type"), "type_");
        assert_eq!(to_snake("UTF8x"), "utf8x");
    }

    #[test]
    fn fn_scan() {
        let src = "pub fn wait_l(&self) {}\n fn  bad() {}\nfn add_obj<T>(x) {}\nxpub fn q() {}\npub fn\tr (x)";
        let all = scan_fn_names(src, false);
        assert!(all.contains("wait_l") && all.contains("add_obj") && !all.contains("bad"));
        let p = scan_fn_names(src, true);
        assert!(p.contains("wait_l") && p.contains("r") && !p.contains("q"));
    }

    #[test]
    fn refs_and_atomic() {
        assert!(refs_local("_pre.wrapping_add(x)", "x"));
        assert!(!refs_local("format!(\"x\")", "x"));
        assert!(!refs_local("xx + 1", "x"));
        assert!(is_atomic("a.b(c + d)?"));
        assert!(!is_atomic("a + b"));
    }
}
