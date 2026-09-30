//! 字面量（← `Lit(value: str)`）。
//!
//! Python 的 `Lit` 原样携带渲染文本（`"42i32"`、`'String::from("x")'`、
//! `'Class::for_class(String::from("java/lang/X"))'`、`'Object::default()'`）；
//! 这里按语义分成变体，渲染形态由 `render::lit` 唯一决定。

use crate::IrError;

/// 整数后缀。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntTy {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Usize,
}

impl IntTy {
    pub fn suffix(self) -> &'static str {
        match self {
            IntTy::I8 => "i8",
            IntTy::I16 => "i16",
            IntTy::I32 => "i32",
            IntTy::I64 => "i64",
            IntTy::U8 => "u8",
            IntTy::U16 => "u16",
            IntTy::U32 => "u32",
            IntTy::U64 => "u64",
            IntTy::Usize => "usize",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatTy {
    F32,
    F64,
}

impl FloatTy {
    pub fn suffix(self) -> &'static str {
        match self {
            FloatTy::F32 => "f32",
            FloatTy::F64 => "f64",
        }
    }
}

/// 浮点值：有限值保留常量池给出的十进制记号（与 JVM `Float/Double.toString`
/// 同形，渲染时原样加后缀），特殊值渲染为 `f32::NAN` 等关联常量。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatLit {
    Finite(String),
    Nan,
    Inf,
    NegInf,
}

impl FloatLit {
    /// 解析十进制记号（`0.75`、`-1.0E10`、`1.4E-45`）或特殊值拼写（`NaN`、`Infinity`、`-Infinity`、`inf`）。
    pub fn parse(token: &str) -> Result<FloatLit, IrError> {
        match token.to_ascii_lowercase().as_str() {
            "nan" => return Ok(FloatLit::Nan),
            "inf" | "infinity" => return Ok(FloatLit::Inf),
            "-inf" | "-infinity" => return Ok(FloatLit::NegInf),
            _ => {}
        }
        let digits = token.strip_prefix('-').unwrap_or(token);
        let ok = digits.starts_with(|c: char| c.is_ascii_digit())
            && digits.chars().all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '-' | '+'));
        if ok {
            Ok(FloatLit::Finite(token.to_string()))
        } else {
            Err(IrError::BadFloat(token.to_string()))
        }
    }
}

/// 字面量。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    /// 整数：`42i32` / `-1i64`；`ty` 为 None 时无后缀（match 臂模式、状态机块号）
    Int { value: i128, ty: Option<IntTy> },
    Float { value: FloatLit, ty: FloatTy },
    Bool(bool),
    /// 单元值 `()`
    Unit,
    /// Rust 字符串字面量记号 `"..."`（宏实参如 `format!` 的格式串）；值为解码后的文本
    Str(String),
    /// Java 字符串常量：`String::from("...")`（值为常量池解码文本，渲染时转义）
    JString(String),
    /// 含孤立代理项的 Java 字符串常量：`String::from_utf16_lit(&[0x0041, ..])`
    JStringUtf16(Vec<u16>),
    /// 类字面量（`X.class`）：`Class::for_class(String::from("<binary>"))`
    ClassRef(String),
    /// 空引用 `aconst_null`：`Object::default()`
    Null,
}

impl Lit {
    pub fn int(value: i128, ty: IntTy) -> Lit {
        Lit::Int { value, ty: Some(ty) }
    }

    pub fn i32(value: i32) -> Lit {
        Lit::int(value.into(), IntTy::I32)
    }
}
