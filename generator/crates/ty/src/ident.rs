//! Java 标识符 → 合法 Rust 标识符（`codegen/constants.safe_ident` 的移植）。

/// Rust 关键字（含保留字）
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

pub fn is_rust_keyword(name: &str) -> bool {
    RUST_KEYWORDS.contains(&name)
}

/// `$` → `_`；数字开头加 `_` 前缀；Rust 关键字加 `_` 后缀
pub fn safe_ident(name: &str) -> String {
    let mut s = name.replace('$', "_");
    if s.chars().next().is_some_and(|c| c.is_numeric()) {
        s.insert(0, '_');
    }
    if is_rust_keyword(&s) {
        s.push('_');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_ident_rules() {
        assert_eq!(safe_ident("a$b"), "a_b");
        assert_eq!(safe_ident("1x"), "_1x");
        assert_eq!(safe_ident("type"), "type_");
        assert_eq!(safe_ident("yield"), "yield_");
        assert_eq!(safe_ident("value"), "value");
    }
}
