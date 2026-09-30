//! 已生成签名文本的切分与代换（← `inherited_gen._param_idents` / `_sig_param_types` /
//! `_result_inner`、`type_args.substitute_type_params`、`interface_gen._split_top_level`）。
//!
//! 第二阶段的定义侧真源是**已生成文本**（`EmittedMethod::signature`），这里的操作都是文本级的。

use std::collections::BTreeMap;
use std::sync::OnceLock;

use regex::Regex;

fn ident_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\b[A-Za-z_][A-Za-z0-9_]*\b").expect("静态正则"))
}

/// 文本中的标识符（`\b[A-Za-z_][A-Za-z0-9_]*\b`，出现序，含重复）
pub fn idents(text: &str) -> impl Iterator<Item = &str> {
    ident_re().find_iter(text).map(|m| m.as_str())
}

fn is_ident(s: &str) -> bool {
    let mut cs = s.chars();
    cs.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') && cs.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn opens(c: char) -> bool {
    matches!(c, '(' | '<' | '[')
}

fn closes(c: char) -> bool {
    matches!(c, ')' | '>' | ']')
}

/// 形参表：`(` 起深度计数回到 0 处的 `)`（返回类型里的括号不计入）；(左括号位置, 右括号位置)
fn param_span(sig: &str) -> Option<(usize, Option<usize>)> {
    let start = sig.find('(')?;
    let mut depth = 0i32;
    for (i, c) in sig[start..].char_indices() {
        if opens(c) {
            depth += 1;
        } else if closes(c) {
            depth -= 1;
            if depth == 0 {
                return Some((start, Some(start + i)));
            }
        }
    }
    Some((start, None))
}

/// 顶层逗号切分（`(<[` 计深度）；各段不裁剪，尾段全空白时丢弃
fn split_raw(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in s.chars() {
        if opens(ch) {
            depth += 1;
        } else if closes(ch) {
            depth -= 1;
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

/// 顶层逗号切分，各段裁剪（`interface_gen._split_top_level`）
pub fn split_top_level_trimmed(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in s.chars() {
        if opens(ch) {
            depth += 1;
        } else if closes(ch) {
            depth -= 1;
        }
        if ch == ',' && depth == 0 {
            parts.push(cur.trim().to_string());
            cur.clear();
        } else {
            cur.push(ch);
        }
    }
    let tail = cur.trim();
    if !tail.is_empty() {
        parts.push(tail.to_string());
    }
    parts
}

/// `pub fn name(&self, a: T, b: U) -> R` → `[a, b]`（跳过 self 接收者、去 mut 绑定）
pub fn param_idents(sig: &str) -> Vec<String> {
    let Some((start, end)) = param_span(sig) else { return Vec::new() };
    let end = end.unwrap_or(sig.len());
    let mut out = Vec::new();
    for p in split_raw(&sig[start + 1..end]) {
        let p = p.trim();
        if p.is_empty() || p == "self" || p == "mut self" || p.starts_with('&') {
            continue;
        }
        let mut name = p.split(':').next().unwrap_or("").trim();
        if let Some(n) = name.strip_prefix("mut ") {
            name = n.trim();
        }
        if !name.is_empty() && is_ident(name) {
            out.push(name.to_string());
        }
    }
    out
}

/// `pub fn name(&self, a: T, b: U) -> R` → (`[T, U]`, `R`)
pub fn sig_param_types(sig: &str) -> (Vec<String>, String) {
    let Some((start, Some(end))) = param_span(sig) else { return (Vec::new(), String::new()) };
    let mut types = Vec::new();
    for p in split_raw(&sig[start + 1..end]) {
        let p = p.trim();
        if p.is_empty() || p.starts_with('&') || p == "self" || p == "mut self" {
            continue;
        }
        types.push(match p.split_once(':') {
            Some((_, t)) => t.trim().to_string(),
            None => p.to_string(),
        });
    }
    let ret = sig.rfind("->").map(|a| sig[a + 2..].trim().to_string()).unwrap_or_default();
    (types, ret)
}

/// `Result<T>` → `T`（不裁剪）；其余 None
pub fn result_inner(ty: &str) -> Option<&str> {
    ty.strip_prefix("Result<").and_then(|r| r.strip_suffix('>'))
}

/// 文本中的类型参数名按 mapping 同步替换（整词）
pub fn substitute_type_params(text: &str, mapping: &BTreeMap<String, String>) -> String {
    if mapping.is_empty() {
        return text.to_string();
    }
    ident_re()
        .replace_all(text, |c: &regex::Captures<'_>| mapping.get(&c[0]).cloned().unwrap_or_else(|| c[0].to_string()))
        .into_owned()
}

/// 形参 → 实参（缺位按擦除语义取 Object）
pub fn param_mapping(params: &[String], args: &[String]) -> BTreeMap<String, String> {
    params
        .iter()
        .enumerate()
        .map(|(i, p)| (p.clone(), args.get(i).cloned().unwrap_or_else(|| "Object".into())))
        .collect()
}

/// `Short<A, B>`（无实参时为裸短名）
pub fn rust_type(short: &str, args: &[String]) -> String {
    if args.is_empty() {
        short.to_string()
    } else {
        format!("{short}<{}>", args.join(", "))
    }
}

/// `a::B<X, Y>` → `a::B`
pub fn rust_type_head(ty: &str) -> &str {
    ty.split('<').next().unwrap_or(ty)
}

/// 形参描述符部分 `(...)`
pub fn param_part(desc: &str) -> &str {
    desc.find(')').map_or(desc, |i| &desc[..=i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_signature() {
        let sig = "pub fn f(&self, a: Map<K, V>, mut b: i32) -> Result<Vec<(K, V)>>";
        assert_eq!(param_idents(sig), vec!["a", "b"]);
        let (t, r) = sig_param_types(sig);
        assert_eq!(t, vec!["Map<K, V>", "i32"]);
        assert_eq!(r, "Result<Vec<(K, V)>>");
        assert_eq!(result_inner(&r), Some("Vec<(K, V)>"));
        let m = param_mapping(&["K".into(), "V".into()], &["String".into()]);
        assert_eq!(substitute_type_params("Map<K, V>", &m), "Map<String, Object>");
    }
}
