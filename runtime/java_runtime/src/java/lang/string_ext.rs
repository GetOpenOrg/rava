use crate::prelude::*;
use super::string::String;

impl String {
    /// Rust `String`（UTF-8）→ Java `String`（新对象，不入驻留表）。
    #[jvm_ext]
    pub fn from_owned(s: std::string::String) -> Self {
        String::from_units(&s.encode_utf16().collect::<Vec<u16>>())
    }

    /// UTF-16 码元 → Java `String`（value + coder，JDK compact strings；新对象，不入驻留表）。
    ///
    /// coder 按内容选择（与 JDK `StringUTF16.newStringNoRepl` 语义一致）：
    /// 全部码元 ≤ 0xFF → LATIN1（每 char 一字节）；否则 UTF16，字节序与
    /// 生成侧 `StringUTF16.putChar` 的 HI/LO_BYTE_SHIFT 一致（平台字节序）。
    /// 增补字符（非 BMP）必然走 UTF16：代理对各占一个 code unit（S-19 #3）；孤立代理项原样保留。
    pub fn from_units(units: &[u16]) -> Self {
        let mut inst = String::default();
        inst._init_not_null();
        if units.iter().all(|u| *u <= 0xFF) {
            inst.__set_value(JArray::from(units.iter().map(|u| *u as u8 as i8).collect::<Vec<i8>>()));
            inst.__set_coder(0i8);
        } else {
            let big_endian = cfg!(target_endian = "big");
            let mut bytes: Vec<i8> = Vec::with_capacity(units.len() * 2);
            for &u in units {
                // putChar：val[i] = c >> HI_BYTE_SHIFT；val[i+1] = c >> LO_BYTE_SHIFT
                let (first, second) = if big_endian { ((u >> 8) as u8, u as u8) } else { (u as u8, (u >> 8) as u8) };
                bytes.push(first as i8);
                bytes.push(second as i8);
            }
            inst.__set_value(JArray::from(bytes));
            inst.__set_coder(1i8);
        }
        inst
    }

    /// Java 字符串的 UTF-16 码元（字符串转换语义：null 载体 → `"null"`，JLS §5.1.11）。
    /// null 载体的 value 为 JArray Repr::Null，直接解码会在 to_vec panic——先守卫再取值；
    /// 判定走 wrapper 固有 is_jvm_null()（句柄为空）。
    pub fn units(&self) -> Vec<u16> {
        if self.is_jvm_null() {
            return "null".encode_utf16().collect();
        }
        let val = self.__get_value().to_vec();
        if self.__get_coder() == 0i8 {
            return val.iter().map(|b| *b as u8 as u16).collect();
        }
        // 字节序与写入侧（putChar HI/LO_BYTE_SHIFT，平台字节序）一致
        (0..val.len() / 2)
            .map(|i| {
                let (b0, b1) = (val[i * 2] as u8, val[i * 2 + 1] as u8);
                if cfg!(target_endian = "big") { u16::from_be_bytes([b0, b1]) } else { u16::from_le_bytes([b0, b1]) }
            })
            .collect()
    }

    /// 字符串拼接（`makeConcatWithConstants`）的起点：以配方首段常量文本构造新对象，
    /// 后续片段经 `+` 在 UTF-16 层追加（见下方 `Add` 实现），孤立代理项不经 Rust 文本。
    pub fn of(text: &str) -> Self {
        String::from_owned(text.to_owned())
    }

    /// 字面量加载路径的 UTF-16 码元形态：常量含孤立代理项（`"\uD83D"`）时 codegen 发射本
    /// 构造（Rust `&str` 无法表示孤立代理项）。与 `From<&str>` 同样经常量池驻留（JLS §3.10.5）。
    pub fn from_utf16_lit(units: &[u16]) -> Self {
        String::from_units(units).__interned()
    }

    /// 拼接追加：本对象码元 + `tail`，结果为新对象（Java 拼接结果不与操作数共享身份）
    fn appended(&self, tail: impl IntoIterator<Item = u16>) -> Self {
        let mut units = self.units();
        units.extend(tail);
        String::from_units(&units)
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Java 语义：null 字符串的字符串转换是字面 "null"（units() 已处理）；
        // Rust 文本无法承载孤立代理项，此处按 U+FFFD 呈现（仅诊断 / 宿主文本用途）
        write!(f, "{}", std::string::String::from_utf16_lossy(&self.units()))
    }
}

/// 字符串拼接片段（JLS §15.18.1 字符串转换）：`String::of("x=") + &s + c + n` 的各 `+`。
/// 常量文本 / 码元段 / Java 字符串 / char 码元均在 UTF-16 层追加。
impl std::ops::Add<&str> for String {
    type Output = String;
    fn add(self, rhs: &str) -> String { self.appended(rhs.encode_utf16()) }
}

impl std::ops::Add<&[u16]> for String {
    type Output = String;
    fn add(self, rhs: &[u16]) -> String { self.appended(rhs.iter().copied()) }
}

impl std::ops::Add<&String> for String {
    type Output = String;
    fn add(self, rhs: &String) -> String { self.appended(rhs.units()) }
}

/// Java `char`（`u16` 码元）按字符追加，孤立代理项原样保留
impl std::ops::Add<u16> for String {
    type Output = String;
    fn add(self, rhs: u16) -> String { self.appended([rhs]) }
}

impl std::ops::Add<bool> for String {
    type Output = String;
    fn add(self, rhs: bool) -> String { self + if rhs { "true" } else { "false" } }
}

impl std::ops::Add<f64> for String {
    type Output = String;
    fn add(self, rhs: f64) -> String { self + crate::java_fmt_f64(rhs).as_str() }
}

impl std::ops::Add<f32> for String {
    type Output = String;
    fn add(self, rhs: f32) -> String { self + crate::java_fmt_f32(rhs).as_str() }
}

/// 整数（byte / short / int / long）按十进制文本追加（与 `Integer.toString` / `Long.toString` 同形）
macro_rules! concat_int {
    ($($t:ty),*) => {$(
        impl std::ops::Add<$t> for String {
            type Output = String;
            fn add(self, rhs: $t) -> String { self + rhs.to_string().as_str() }
        }
    )*};
}
concat_int!(i8, i16, i32, i64);

impl From<&str> for String {
    /// 字面量加载路径（ldc 发射形态 `String::from("...")`，S-6）：Java 字符串
    /// 字面量属于常量池驻留项，故经全局驻留表取规范实例——相同内容的字面量
    /// 与 `intern()` 结果是同一对象（JLS §3.10.5）。拼接等非字面量构造走
    /// `from_owned` / `of`（新对象，不入表），与 Java 语义一致。
    fn from(s: &str) -> Self { String::from_owned(s.to_owned()).__interned() }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self { String::from_owned(s) }
}
