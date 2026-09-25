//! `jdk/internal/util/HexDigits` 手写实现（JDK 25 语料，内部边界类，按调用链按需）。
//!
//! 消费面：`java.util.HexFormat`（toHexDigits 族经 `digitPair` 取两位十六进制）。
//! JDK 的 `DIGITS` short 表按 `<clinit>` 字节码【javap -c 实测】构造：
//! `DIGITS[(hi << 4) + lo] = (hex(lo) << 8) | hex(hi)`，hex 为小写（`0-9a-f`）——
//! 低字节是高位 nibble 的字符、高字节是低位 nibble 的字符。此处用算术等价替换
//! 查表（与 DecimalDigits 同惯例）。

use crate::prelude::*;
use super::hex_digits::HexDigits;

fn hex_lower(n: i32) -> i32 {
    if n < 10 { n + 0x30 } else { n - 10 + 0x61 }
}

impl HexDigits {
    /// `digitPair(int i, boolean ucase)`：`i & 0xff` 的两位十六进制字符打包成
    /// short（见模块注释的字节布局）。ucase 时 `v - ((v & 0x4040) >> 1)`：字母
    /// 字节（0x61..0x66 含 0x40 位）各减 0x20 转大写，数字字节（0x30..0x39）不变。
    #[jvm_boundary]
    pub fn digitPair(i: i32, ucase: bool) -> Result<i16> {
        let b = i & 0xff;
        let v = (hex_lower(b & 0xf) << 8) | hex_lower(b >> 4);
        let v = if ucase { v - ((v & 0x4040) >> 1) } else { v };
        Ok(v as i16)
    }
}
