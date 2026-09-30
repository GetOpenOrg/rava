//! 文本工具：渲染出口与 Python 正则判定的手写等价物（`regex` crate 无后顾断言）。

use instr::InstrEnv;
use ir::{Expr, Renderer, Stmt};
use sim::TyNames;
use ty::RsType;

/// 表达式渲染文本（Python `render_expr`）
pub fn expr(env: &InstrEnv, e: &Expr) -> String {
    Renderer::new(&TyNames(env)).expr(e)
}

/// 语句渲染文本（Python `render_stmt`，无缩进）
pub fn stmt(env: &InstrEnv, s: &Stmt) -> String {
    Renderer::new(&TyNames(env)).stmt(s, 0)
}

/// 类型渲染文本（Python `render_type`）
pub fn ty(env: &InstrEnv, t: &RsType) -> String {
    sim::type_text(t, env)
}

pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// `[A-Za-z_]\w*` 的全部匹配（Python `re.findall` 口径：首字符 ASCII 字母 / 下划线）
pub fn identifiers(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in text.char_indices() {
        match start {
            Some(s) if !is_word_char(c) => {
                out.push(&text[s..i]);
                start = None;
            }
            None if c.is_ascii_alphabetic() || c == '_' => start = Some(i),
            _ => {}
        }
    }
    if let Some(s) = start {
        out.push(&text[s..]);
    }
    out
}

/// 去掉字符串字面量 `"(?:[^"\\]|\\.)*"`
pub fn strip_str_lits(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        if c != '"' {
            out.push(c);
            continue;
        }
        // 尝试匹配字面量；未闭合则原样保留引号
        let rest: String = it.clone().collect();
        let mut end: Option<usize> = None;
        let mut chars = rest.char_indices();
        while let Some((i, ch)) = chars.next() {
            match ch {
                '\\' => {
                    if chars.next().is_none() {
                        break;
                    }
                }
                '"' => {
                    end = Some(i);
                    break;
                }
                _ => {}
            }
        }
        match end {
            Some(e) => {
                let consumed = rest[..=e].chars().count();
                for _ in 0..consumed {
                    it.next();
                }
            }
            None => out.push(c),
        }
    }
    out
}

/// `\bname\b` 的全部匹配起点（`name` 以单词字符开头结尾时）
pub fn word_positions(text: &str, name: &str) -> Vec<usize> {
    find_word(text, name, false)
}

/// `(?<![\w.])name\b` 的全部匹配起点
pub fn word_positions_no_dot(text: &str, name: &str) -> Vec<usize> {
    find_word(text, name, true)
}

fn find_word(text: &str, name: &str, no_dot: bool) -> Vec<usize> {
    let mut out = Vec::new();
    if name.is_empty() {
        return out;
    }
    let first_word = name.chars().next().is_some_and(is_word_char);
    let last_word = name.chars().last().is_some_and(is_word_char);
    let mut from = 0;
    while let Some(p) = text[from..].find(name) {
        let s = from + p;
        let e = s + name.len();
        let before = text[..s].chars().next_back();
        let after = text[e..].chars().next();
        let ok_before = if no_dot {
            !before.is_some_and(|c| is_word_char(c) || c == '.')
        } else {
            // \b：边界两侧单词性不同
            before.is_some_and(is_word_char) != first_word
        };
        let ok_after = after.is_some_and(is_word_char) != last_word;
        if ok_before && ok_after {
            out.push(s);
        }
        from = s + name.chars().next().map_or(1, char::len_utf8);
    }
    out
}

/// `\bname\b` 是否出现
pub fn has_word(text: &str, name: &str) -> bool {
    !word_positions(text, name).is_empty()
}

/// 以 `f(匹配)` 替换全部 `(?<![\w.])name\b`
pub fn replace_word_no_dot(text: &str, name: &str, rep: &str) -> String {
    let pos = word_positions_no_dot(text, name);
    if pos.is_empty() {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for p in pos {
        out.push_str(&text[last..p]);
        out.push_str(rep);
        last = p + name.len();
    }
    out.push_str(&text[last..]);
    out
}

/// `\bname\b` 的全部替换
pub fn replace_word(text: &str, name: &str, rep: &str) -> String {
    let pos = word_positions(text, name);
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for p in pos {
        out.push_str(&text[last..p]);
        out.push_str(rep);
        last = p + name.len();
    }
    out.push_str(&text[last..]);
    out
}

/// `^[A-Za-z_]\w*$`
pub fn is_ident(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c.is_alphabetic() || c == '_') && cs.all(is_word_char)
}

/// Python `str.strip()`（Unicode 空白）
pub fn py_strip(s: &str) -> &str {
    s.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words() {
        assert_eq!(word_positions("a _t1 x._t1 _t10 (_t1)", "_t1"), vec![2, 8, 18]);
        assert_eq!(word_positions_no_dot("a _t1 x._t1 _t10 (_t1)", "_t1"), vec![2, 18]);
        assert_eq!(replace_word_no_dot("_t1 + x._t1", "_t1", "(a+b)"), "(a+b) + x._t1");
        assert!(has_word("let mut x = 1;", "x"));
        assert!(!has_word("xy", "x"));
    }

    #[test]
    fn idents_and_lits() {
        assert_eq!(identifiers("foo(bar_1, 2x, _z)"), vec!["foo", "bar_1", "x", "_z"]);
        assert_eq!(strip_str_lits(r#"a("x\"y", b) + "c""#), "a(, b) + ");
        assert_eq!(strip_str_lits(r#"a " b"#), r#"a " b"#);
    }
}
