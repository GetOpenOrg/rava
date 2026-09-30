//! 构造入口：字段描述符（JVMS §4.3.2）与泛型签名单类型（JVMS §4.7.9）→ [`JvmType`]。
//! 非法输入返回错误（类型层宁严不宽）。

use std::fmt;

use super::{JvmType, PrimKind, WildKind};
use crate::registry::Registry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParseError {
    pub input: String,
    pub reason: &'static str,
}

impl fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.reason, self.input)
    }
}

impl std::error::Error for TypeParseError {}

fn err(input: &str, reason: &'static str) -> TypeParseError {
    TypeParseError {
        input: input.to_string(),
        reason,
    }
}

impl JvmType {
    /// 字段描述符 → 类型：`I`、`[I`、`[[Ljava/lang/String;`
    pub fn from_descriptor(desc: &str) -> Result<JvmType, TypeParseError> {
        let b = desc.as_bytes();
        let Some(&c) = b.first() else {
            return Err(err(desc, "empty descriptor"));
        };
        if let Some(k) = PrimKind::from_desc(c) {
            if b.len() != 1 {
                return Err(err(desc, "trailing text after primitive descriptor"));
            }
            return Ok(JvmType::Primitive(k));
        }
        match c {
            b'[' => Ok(JvmType::array(JvmType::from_descriptor(&desc[1..])?)),
            b'L' => match desc.find(';') {
                Some(end) if end == desc.len() - 1 => Ok(JvmType::class(&desc[1..end])),
                _ => Err(err(desc, "malformed class descriptor")),
            },
            _ => Err(err(desc, "unknown descriptor")),
        }
    }

    /// 泛型签名单类型 → 类型；内部类 `LOuter<A;>.Inner<B;>;` → `Outer$Inner`，实参取最内段。
    /// 注册表可查时补全 is_interface
    pub fn from_signature(sig: &str, reg: &Registry) -> Result<JvmType, TypeParseError> {
        let (t, i) = parse_sig_type(sig, 0, reg)?;
        if i != sig.len() {
            return Err(err(sig, "trailing text after signature"));
        }
        Ok(t)
    }
}

fn parse_sig_type(sig: &str, i: usize, reg: &Registry) -> Result<(JvmType, usize), TypeParseError> {
    let b = sig.as_bytes();
    let Some(&c) = b.get(i) else {
        return Err(err(sig, "unexpected end of signature"));
    };
    if c != b'V' {
        if let Some(k) = PrimKind::from_desc(c) {
            return Ok((JvmType::Primitive(k), i + 1));
        }
    }
    match c {
        b'T' => match sig[i..].find(';') {
            Some(p) => Ok((JvmType::type_var(&sig[i + 1..i + p], None), i + p + 1)),
            None => Err(err(sig, "unterminated type variable")),
        },
        b'[' => {
            let (elem, j) = parse_sig_type(sig, i + 1, reg)?;
            Ok((JvmType::array(elem), j))
        }
        b'*' => Ok((JvmType::wildcard(WildKind::Unbounded, None), i + 1)),
        b'+' | b'-' => {
            let (bound, j) = parse_sig_type(sig, i + 1, reg)?;
            let kind = if c == b'+' {
                WildKind::Extends
            } else {
                WildKind::Super
            };
            Ok((JvmType::wildcard(kind, Some(bound)), j))
        }
        b'L' => parse_class_sig(sig, i, reg),
        _ => Err(err(sig, "unknown signature char")),
    }
}

fn scan_name(b: &[u8], mut j: usize) -> usize {
    while j < b.len() && !matches!(b[j], b'<' | b';' | b'.') {
        j += 1;
    }
    j
}

fn parse_class_sig(
    sig: &str,
    i: usize,
    reg: &Registry,
) -> Result<(JvmType, usize), TypeParseError> {
    let b = sig.as_bytes();
    let mut j = scan_name(b, i + 1);
    let mut binary = sig[i + 1..j].to_string();
    let mut args = Vec::new();
    if b.get(j) == Some(&b'<') {
        (args, j) = parse_sig_args(sig, j, reg)?;
    }
    while b.get(j) == Some(&b'.') {
        let k = scan_name(b, j + 1);
        binary = format!("{binary}${}", &sig[j + 1..k]);
        j = k;
        if b.get(j) == Some(&b'<') {
            (args, j) = parse_sig_args(sig, j, reg)?;
        }
    }
    if b.get(j) != Some(&b';') {
        return Err(err(sig, "malformed class signature"));
    }
    let is_interface = reg.get(&binary).is_some_and(|c| c.is_interface());
    Ok((JvmType::class_with(binary, args, is_interface), j + 1))
}

fn parse_sig_args(
    sig: &str,
    mut i: usize,
    reg: &Registry,
) -> Result<(Vec<JvmType>, usize), TypeParseError> {
    let b = sig.as_bytes();
    i += 1;
    let mut args = Vec::new();
    while i < b.len() && b[i] != b'>' {
        let (a, j) = parse_sig_type(sig, i, reg)?;
        args.push(a);
        i = j;
    }
    if i >= b.len() {
        return Err(err(sig, "unterminated type arguments"));
    }
    Ok((args, i + 1))
}
