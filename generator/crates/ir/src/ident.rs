//! 标识符与循环 / 块标签。

use crate::IrError;
use std::fmt;

/// 合法的 Rust 标识符（构造时校验）。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Result<Ident, IrError> {
        let s = s.into();
        if is_ident(&s) {
            Ok(Ident(s))
        } else {
            Err(IrError::BadIdent(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// `[A-Za-z_][A-Za-z0-9_]*`（不含单独的 `_`）或 `r#` + 该形态。
pub(crate) fn is_ident(s: &str) -> bool {
    let body = s.strip_prefix("r#").unwrap_or(s);
    let mut chars = body.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    body != "_" && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// 循环 / 带标签块的标签，渲染为 `'name`（如 `'l0`、`'b1`）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub Ident);

impl Label {
    pub fn new(name: impl Into<String>) -> Result<Label, IrError> {
        Ident::new(name).map(Label)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident_validation() {
        for ok in ["x", "_t0", "this", "r#type", "HashMap_Node", "__get_first"] {
            assert!(Ident::new(ok).is_ok(), "{ok}");
        }
        for bad in ["", "_", "0a", "a.b", "a b", "a::b", "x?", "r#"] {
            assert!(Ident::new(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn label_display() {
        assert_eq!(Label::new("l0").unwrap().to_string(), "'l0");
    }
}
