use crate::prelude::*;
use super::string::String;

impl String {
    /// Rust `String`（UTF-8）→ Java `String`（value + coder，JDK compact strings）。
    ///
    /// coder 按内容选择（与 JDK `StringUTF16.newStringNoRepl` 语义一致）：
    /// 全部字符 ≤ U+00FF → LATIN1（每 char 一字节）；否则 UTF16，字节序与
    /// 生成侧 `StringUTF16.putChar` 的 HI/LO_BYTE_SHIFT 一致（平台字节序）。
    /// 增补字符（非 BMP）必然走 UTF16：代理对各占一个 code unit（S-19 #3）。
    #[jvm_ext]
    pub fn from_owned(s: std::string::String) -> Self {
        let mut inst = String::default();
        inst._init_not_null();
        if s.chars().all(|c| (c as u32) <= 0xFF) {
            let bytes: Vec<i8> = s.chars().map(|c| c as u8 as i8).collect();
            inst.__set_value(JArray::from(bytes));
            inst.__set_coder(0i8);
        } else {
            let big_endian = cfg!(target_endian = "big");
            let mut bytes: Vec<i8> = Vec::with_capacity(s.chars().count() * 2);
            for u in s.encode_utf16() {
                // putChar：val[i] = c >> HI_BYTE_SHIFT；val[i+1] = c >> LO_BYTE_SHIFT
                let (first, second) = if big_endian {
                    ((u >> 8) as u8, u as u8)
                } else {
                    (u as u8, (u >> 8) as u8)
                };
                bytes.push(first as i8);
                bytes.push(second as i8);
            }
            inst.__set_value(JArray::from(bytes));
            inst.__set_coder(1i8);
        }
        inst
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.__get_value().to_vec();
        let len = val.len() as i32;
        if self.__get_coder() == 0i8 {
            let s: std::string::String = val.iter().map(|b| *b as u8 as char).collect();
            write!(f, "{}", s)
        } else {
            // 字节序与写入侧（putChar HI/LO_BYTE_SHIFT，平台字节序）一致
            let u16s: Vec<u16> = (0..len as usize / 2).map(|i| {
                let b0 = val[i * 2] as u8;
                let b1 = if (i as i32 * 2 + 1) < len { val[i * 2 + 1] as u8 } else { 0 };
                if cfg!(target_endian = "big") {
                    u16::from_be_bytes([b0, b1])
                } else {
                    u16::from_le_bytes([b0, b1])
                }
            }).collect();
            write!(f, "{}", std::string::String::from_utf16_lossy(&u16s))
        }
    }
}

impl From<&str> for String {
    /// 字面量加载路径（ldc 发射形态 `String::from("...")`，S-6）：Java 字符串
    /// 字面量属于常量池驻留项，故经全局驻留表取规范实例——相同内容的字面量
    /// 与 `intern()` 结果是同一对象（JLS §3.10.5）。拼接等非字面量构造走
    /// `from_owned`（新对象，不入表），与 Java 语义一致。
    fn from(s: &str) -> Self { String::from_owned(s.to_owned()).__interned() }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self { String::from_owned(s) }
}
