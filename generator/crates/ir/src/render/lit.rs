//! 字面量渲染：形态与 Python 各产出点（`instr/sim/consts.py`、`coerce._escape_str` /
//! `_float_lit`）的文本逐字一致。

use crate::{anchors, FloatLit, Lit};
use std::fmt::Write as _;

pub(crate) fn write_lit(out: &mut String, lit: &Lit) {
    match lit {
        Lit::Int { value, ty } => {
            let _ = write!(out, "{value}");
            if let Some(t) = ty {
                out.push_str(t.suffix());
            }
        }
        Lit::Float { value, ty } => match value {
            FloatLit::Finite(tok) => {
                out.push_str(tok);
                out.push_str(ty.suffix());
            }
            FloatLit::Nan => {
                let _ = write!(out, "{}::NAN", ty.suffix());
            }
            FloatLit::Inf => {
                let _ = write!(out, "{}::INFINITY", ty.suffix());
            }
            FloatLit::NegInf => {
                let _ = write!(out, "{}::NEG_INFINITY", ty.suffix());
            }
        },
        Lit::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Lit::Unit => out.push_str("()"),
        Lit::Str(s) => write_str_token(out, s),
        Lit::JString(s) => {
            let _ = write!(out, "{}::from(", anchors::STRING);
            write_str_token(out, s);
            out.push(')');
        }
        Lit::JStringUtf16(units) => {
            let _ = write!(out, "{}::from_utf16_lit(&[", anchors::STRING);
            for (i, u) in units.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let _ = write!(out, "0x{u:04X}");
            }
            out.push_str("])");
        }
        Lit::ClassRef(binary) => {
            let _ = write!(out, "{}::for_class({}::from(", anchors::CLASS, anchors::STRING);
            write_str_token(out, binary);
            out.push_str("))");
        }
        Lit::Null => {
            let _ = write!(out, "{}::default()", anchors::OBJECT);
        }
    }
}

/// `"..."` 字符串记号（转义规则同 Python `_escape_str`）。
pub(crate) fn write_str_token(out: &mut String, s: &str) {
    out.push('"');
    escape_str_into(out, s);
    out.push('"');
}

/// 常量池解码值 → Rust 字符串字面量内容（不含两端引号）：反斜杠一律双写，
/// `"` `\n` `\r` `\t` 转义，其余 C0 / C1 控制字符为 `\u{xxxx}`。
pub fn escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    escape_str_into(&mut out, s);
    out
}

fn escape_str_into(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || (0x7f..=0x9f).contains(&(c as u32)) => {
                let _ = write!(out, "\\u{{{:04x}}}", c as u32);
            }
            c => out.push(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FloatTy, IntTy};

    fn r(l: &Lit) -> String {
        let mut s = String::new();
        write_lit(&mut s, l);
        s
    }

    #[test]
    fn ints_and_floats() {
        assert_eq!(r(&Lit::i32(-1)), "-1i32");
        assert_eq!(r(&Lit::int(2611923443488327891, IntTy::I64)), "2611923443488327891i64");
        assert_eq!(r(&Lit::Int { value: 7, ty: None }), "7");
        let f = Lit::Float { value: FloatLit::parse("0.75").unwrap(), ty: FloatTy::F32 };
        assert_eq!(r(&f), "0.75f32");
        let f = Lit::Float { value: FloatLit::parse("1.0E10").unwrap(), ty: FloatTy::F64 };
        assert_eq!(r(&f), "1.0E10f64");
        let nan = Lit::Float { value: FloatLit::parse("NaN").unwrap(), ty: FloatTy::F64 };
        assert_eq!(r(&nan), "f64::NAN");
        let ninf = Lit::Float { value: FloatLit::parse("-Infinity").unwrap(), ty: FloatTy::F32 };
        assert_eq!(r(&ninf), "f32::NEG_INFINITY");
        assert!(FloatLit::parse("x1").is_err());
    }

    #[test]
    fn java_constants() {
        assert_eq!(r(&Lit::JString("a\"b\\c\n".into())), "String::from(\"a\\\"b\\\\c\\n\")");
        assert_eq!(r(&Lit::JString("\u{1}\u{85}".into())), "String::from(\"\\u{0001}\\u{0085}\")");
        assert_eq!(r(&Lit::JStringUtf16(vec![0xD800, 0x41])), "String::from_utf16_lit(&[0xD800, 0x0041])");
        assert_eq!(
            r(&Lit::ClassRef("java/lang/Thread".into())),
            "Class::for_class(String::from(\"java/lang/Thread\"))"
        );
        assert_eq!(r(&Lit::Null), "Object::default()");
        assert_eq!(r(&Lit::Unit), "()");
        assert_eq!(r(&Lit::Str("x={}".into())), "\"x={}\"");
    }
}
