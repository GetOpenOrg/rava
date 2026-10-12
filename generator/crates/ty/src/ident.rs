//! Java 标识符 → 合法 Rust 标识符。

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

/// wrapper / 接口载体已实现的 std prelude trait（及 blanket impl）的方法名
///（S7 计划 §9.9 K6 前置项；宏侧重名防护名单 `rava_macros_core … method_trait::STD_COLLISIONS` 同源）：
/// 这类 Java 方法名移入方法 trait `X__Methods` 后，调用点与 `Clone::clone` / `From::from` 等
/// prelude trait 方法构成 E0034 歧义，定义侧与调用侧统一改为 `__` 前缀内部名（`clone` → `__clone`）。
/// 带描述符后缀的重载名（`clone_xxx`）不在此列——不与 std trait 方法同名。
pub const STD_TRAIT_METHOD_NAMES: [&str; 11] = [
    "clone", "clone_from", "clone_into", "to_owned", "eq", "ne", "default", "from", "into", "try_from",
    "try_into",
];

/// 方法 trait 表面上的成员名：与 std prelude trait 方法同名时加 `__` 前缀
pub fn escape_std_trait_method(name: &str) -> String {
    if STD_TRAIT_METHOD_NAMES.contains(&name) {
        format!("__{name}")
    } else {
        name.to_string()
    }
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

    #[test]
    fn std_trait_method_names_get_prefix() {
        assert_eq!(escape_std_trait_method("clone"), "__clone");
        assert_eq!(escape_std_trait_method("from"), "__from");
        assert_eq!(escape_std_trait_method("clone_int"), "clone_int");
        assert_eq!(escape_std_trait_method("hashCode"), "hashCode");
    }
}
