//! 发射文本的定形扫描（取代逐次编译的动态正则）。每个函数与注释里的正则语义逐字等价：
//! `\b` 用 regex 的 Unicode 词字符定义，`\s` 即 Unicode White_Space（`char::is_whitespace`），
//! 候选起点按最左优先依次尝试（含重叠出现）。

/// regex `\w`（Unicode）
fn is_word(c: char) -> bool {
    regex_syntax::is_word_character(c)
}

/// regex `\b`：位置 i 两侧词字符性不同
fn at_boundary(hay: &str, i: usize) -> bool {
    let prev = hay[..i].chars().next_back().is_some_and(is_word);
    let next = hay[i..].chars().next().is_some_and(is_word);
    prev != next
}

/// needle 在 hay 中的全部出现位置（含重叠，升序）
fn hits<'h, 'n>(hay: &'h str, needle: &'n str) -> impl Iterator<Item = usize> + use<'h, 'n> {
    let mut from = 0;
    std::iter::from_fn(move || {
        let i = from + hay.get(from..)?.find(needle)?;
        from = i + hay[i..].chars().next().map_or(1, char::len_utf8);
        Some(i)
    })
}

/// `\b{key}\s*=\s*"([^"]*)"` 的第 1 组
pub fn attr_str<'t>(hay: &'t str, key: &str) -> Option<&'t str> {
    hits(hay, key).filter(|&i| at_boundary(hay, i)).find_map(|i| {
        let r = hay[i + key.len()..].trim_start().strip_prefix('=')?.trim_start().strip_prefix('"')?;
        r.find('"').map(|e| &r[..e])
    })
}

/// `\b{key}\s*=\s*true` 是否匹配
pub fn attr_true(hay: &str, key: &str) -> bool {
    hits(hay, key).filter(|&i| at_boundary(hay, i)).any(|i| {
        hay[i + key.len()..].trim_start().strip_prefix('=').is_some_and(|r| r.trim_start().starts_with("true"))
    })
}

/// `pub struct {short}<([^>{]*)>` 的第 1 组
pub fn struct_generics<'t>(text: &'t str, short: &str) -> Option<&'t str> {
    let head = format!("pub struct {short}<");
    let found = hits(text, &head).find_map(|i| {
        let r = &text[i + head.len()..];
        let e = r.find(['>', '{'])?;
        r[e..].starts_with('>').then(|| &r[..e])
    });
    found
}

/// `\bfn {name}\s*\(` 是否匹配
pub fn has_fn(text: &str, name: &str) -> bool {
    let head = format!("fn {name}");
    let found = hits(text, &head).any(|i| at_boundary(text, i) && text[i + head.len()..].trim_start().starts_with('('));
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    const HAYS: [&str; 9] = [
        r#"#[java_method(name = "foo", descriptor = "()V", is_static = true)]"#,
        r#"#[java_field(binary_name = "x", name   =  "bar")]"#,
        r#"xname = "no" , name="yes""#,
        r#"name = "unterminated"#,
        "is_static=truex is_static = false",
        "é name = \"u\" is_static\u{3000}=\u{3000}true",
        "pub struct Foo<T: Clone, U> { } pub struct Foo<{> pub struct Foo<V>",
        "fn newX (a) fn __init_onX( fn __init_onXY() afn __init_onX()",
        "",
    ];

    fn re_str<'t>(pat: &str, hay: &'t str) -> Option<&'t str> {
        Regex::new(pat).unwrap().captures(hay).and_then(|c| c.get(1)).map(|m| m.as_str())
    }

    #[test]
    fn matches_regex_semantics() {
        for h in HAYS {
            for key in ["name", "descriptor", "is_static", "binary_name"] {
                let esc = regex::escape(key);
                assert_eq!(attr_str(h, key), re_str(&format!(r#"\b{esc}\s*=\s*"([^"]*)""#), h), "{h} / {key}");
                assert_eq!(attr_true(h, key), Regex::new(&format!(r"\b{esc}\s*=\s*true")).unwrap().is_match(h), "{h} / {key}");
            }
            for s in ["Foo", "Bar"] {
                assert_eq!(struct_generics(h, s), re_str(&format!(r"pub struct {s}<([^>{{]*)>"), h), "{h} / {s}");
            }
            for n in ["newX", "__init_onX", "__init_onXY", "X"] {
                assert_eq!(has_fn(h, n), Regex::new(&format!(r"\bfn {n}\s*\(")).unwrap().is_match(h), "{h} / {n}");
            }
        }
    }
}
