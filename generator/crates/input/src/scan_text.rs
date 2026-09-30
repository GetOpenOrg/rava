//! 手写文件文本扫描的小工具（Python `re` 形态的逐字符等价实现，字节下标）。

/// 正则 `\w`
pub fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `\b` 位于 `p` 之前（`p` 处为单词字符时）：前一字符不是单词字符
pub fn starts_word(s: &str, p: usize) -> bool {
    !s[..p].chars().next_back().is_some_and(is_word)
}

/// 跳过 `\s*`，返回下一个非空白字符的下标
pub fn skip_ws(s: &str, i: usize) -> usize {
    i + s[i..].find(|c: char| !c.is_whitespace()).unwrap_or(s.len() - i)
}

/// 从 `start`（开括号之后）起配对，返回配对闭括号之后的下标；未闭合返回 `s.len()`
pub fn balanced(s: &str, start: usize, open: char, close: char) -> usize {
    let mut depth = 1usize;
    for (k, c) in s[start..].char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return start + k + c.len_utf8();
            }
        }
    }
    s.len()
}

/// 从 `from` 起查找 `\bfn\s+(\w+)\s*(?:<[^>]*>)?\s*\(`，返回 (名字, `(` 之后的下标)
pub fn match_fn_head(s: &str, from: usize) -> Option<(&str, usize)> {
    let mut at = from;
    while let Some(p) = s[at..].find("fn").map(|p| p + at) {
        at = p + 2;
        if !starts_word(s, p) {
            continue;
        }
        let i = skip_ws(s, at);
        if i == at {
            continue;
        }
        let end = i + s[i..].find(|c: char| !is_word(c)).unwrap_or(s.len() - i);
        if end == i {
            continue;
        }
        let mut j = skip_ws(s, end);
        if s[j..].starts_with('<') {
            match s[j..].find('>') {
                Some(g) => j = skip_ws(s, j + g + 1),
                None => continue,
            }
        }
        if s[j..].starts_with('(') {
            return Some((&s[i..end], j + 1));
        }
    }
    None
}

/// `^\s*(?:&(?:mut\s+)?self\s*,?\s*)` 剥除接收者
pub fn strip_self(params: &str) -> &str {
    let i = skip_ws(params, 0);
    let Some(rest) = params[i..].strip_prefix('&') else { return params };
    let mut j = params.len() - rest.len();
    if let Some(r) = params[j..].strip_prefix("mut") {
        let k = params.len() - r.len();
        let k2 = skip_ws(params, k);
        if k2 > k && params[k2..].starts_with("self") {
            j = k2;
        }
    }
    let Some(r) = params[j..].strip_prefix("self") else { return params };
    let mut k = skip_ws(params, params.len() - r.len());
    if params[k..].starts_with(',') {
        k += 1;
    }
    &params[skip_ws(params, k)..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers() {
        assert_eq!(strip_self(" &mut self, a: i32"), "a: i32");
        assert_eq!(strip_self("&self"), "");
        assert_eq!(strip_self("a: i32"), "a: i32");
        assert_eq!(match_fn_head("pub fn f<T: X>(x)", 0), Some(("f", 15)));
        assert_eq!(match_fn_head("ffn g()", 0), None);
        assert_eq!(balanced("a(b)c)d", 0, '(', ')'), 6);
    }
}
