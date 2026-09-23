//! `jdk/internal/util/DecimalDigits` 手写实现（JDK 25 语料）。
//!
//! JDK 25 把整数→十进制字符的底层算法从 `Integer`/`Long` 的私有静态方法
//! 抽到了这个内部类。消费面【实测 javap -c】：`Integer.toString`、
//! `Long.toString`、`StringConcatHelper`（字符串拼接 fast path）、
//! `AbstractStringBuilder`（`stringSize(I/J)` + `uncheckedGetCharsLatin1/
//! UTF16(I/J, int, byte[])`）、`BigDecimal`（`getChars` +
//! `uncheckedPutPairLatin1`）。算法按字节码逐指令还原（负数幅值比较，
//! 规避 MIN_VALUE 取反溢出）：
//!
//! - `stringSize`：十进制位数，**负数含符号位**（stringSize(-123)=4、
//!   MIN_VALUE=11）
//! - `uncheckedGetCharsLatin1/UTF16`：从 `index`（终点，不含）向左写入数字
//!   字符，返回新的起点；负数在数字前补 `-`
//! - `getChars(long, int, char[])`：同语义的 char[] 版本（类内唯一形态，
//!   无重载 → Rust 侧裸名 `getChars`）
//! - `putPair` 族：两位数对查表写入——JDK 的 `DIGITS` short 表用算术等价
//!   替换（`r/10`、`r%10`；表项 `(short)('0'+r/10)<<8 | '0'+r%10` 的低/高
//!   字节即两位数字）。`uncheckedPutPairLatin1` 是 `BigDecimal` 的外部
//!   调用点，`putPair`/`uncheckedPutPairUTF16` 补齐公开方法集（无调用方
//!   时无害 dead）
//!
//! UTF16 变体的字节序：JDK 经 `UNSAFE.putCharUnaligned`（平台字节序）写入，
//! 与生成侧 `StringUTF16.putChar/getChar` 的 `HI/LO_BYTE_SHIFT`（由
//! `isBigEndian()` = 宿主字节序决定）自洽——本文件按 `cfg!(target_endian)`
//! 同一约定双字节写全（高字节不依赖零初始化，缓冲复用安全）。UNSAFE 字段
//! 访问统一用 `JArray::with_vec`（set/get 等价）承载。

use crate::prelude::*;
use super::decimal_digits::DecimalDigits;

fn _string_size_i(x: i32) -> i32 {
    let (mut v, sign) = if x >= 0 { (-x, 0i32) } else { (x, 1i32) };
    let mut p: i32 = -10;
    let mut i: i32 = 1;
    while i < 10 {
        if v > p {
            return i + sign;
        }
        p = p.wrapping_mul(10);
        i += 1;
    }
    10 + sign
}

fn _string_size_l(x: i64) -> i32 {
    let (mut v, sign) = if x >= 0 { (-x, 0i64) } else { (x, 1i64) };
    let mut p: i64 = -10;
    let mut i: i32 = 1;
    while i < 19 {
        if v > p {
            return i + sign as i32;
        }
        p = p.wrapping_mul(10);
        i += 1;
    }
    19 + sign as i32
}

/// `uncheckedPutCharUTF16`（private，无外部调用方，内部等价承载）：
/// char 单元双字节写全，字节序按平台（HI/LO_BYTE_SHIFT 同源）
fn _put_char_utf16(arr: &mut [i8], char_pos: usize, c: i32) {
    let c = c as u16;
    let b = char_pos << 1;
    if cfg!(target_endian = "big") {
        arr[b] = (c >> 8) as u8 as i8;
        arr[b + 1] = c as u8 as i8;
    } else {
        arr[b] = c as u8 as i8;
        arr[b + 1] = (c >> 8) as u8 as i8;
    }
}

/// `uncheckedPutPairUTF16` 的数对核心：r ∈ [0,99] → 两位数字（左高位）
fn _put_pair_utf16(arr: &mut [i8], char_pos: usize, r: i32) {
    _put_char_utf16(arr, char_pos, b'0' as i32 + r / 10);
    _put_char_utf16(arr, char_pos + 1, b'0' as i32 + r % 10);
}

impl DecimalDigits {
    #[jvm_native]
    pub fn stringSize_i(x: i32) -> Result<i32> {
        Ok(_string_size_i(x))
    }

    #[jvm_native]
    pub fn stringSize_l(x: i64) -> Result<i32> {
        Ok(_string_size_l(x))
    }

    #[jvm_native]
    pub fn uncheckedGetCharsLatin1_i_i_arr_b(i: i32, index: i32, buf: JArray<i8>) -> Result<i32> {
        let mut v = i;
        let negative = v < 0;
        if !negative {
            v = -v;
        }
        let mut pos = index as usize;
        buf.with_vec(|arr| {
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let d = q * 100 - v;          // 剩余两位（正数）
                arr[pos] = b'0' as i8 + (d / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (d % 10) as i8;
                v = q;
            }
            if v <= -10 {
                // 两位剩余（[-99,-10]）：-v 即两位数字面
                pos -= 2;
                let d = -v;
                arr[pos] = b'0' as i8 + (d / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (d % 10) as i8;
            } else {
                // 单位剩余（[-9,0]）
                pos -= 1;
                arr[pos] = b'0' as i8 - v as i8;
            }
            if negative {
                pos -= 1;
                arr[pos] = b'-' as i8;
            }
        })?;
        Ok(pos as i32)
    }

    #[jvm_native]
    pub fn uncheckedGetCharsLatin1_l_i_arr_b(i: i64, index: i32, buf: JArray<i8>) -> Result<i32> {
        let mut v = i;
        let negative = v < 0;
        if !negative {
            v = -v;
        }
        let mut pos = index as usize;
        buf.with_vec(|arr| {
            while v <= -100_000_000 {
                let q = v / 100_000_000;
                pos -= 8;
                let mut r = v - q * 100_000_000;
                for k in (0..4).rev() {
                    let pair = (0i64 - r % 100) as i64;
                    arr[pos + k * 2] = b'0' as i8 + (pair / 10) as i8;
                    arr[pos + k * 2 + 1] = b'0' as i8 + (pair % 10) as i8;
                    r /= 100;
                }
                v = q;
            }
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let d = q * 100 - v;
                arr[pos] = b'0' as i8 + (d / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (d % 10) as i8;
                v = q;
            }
            if v <= -10 {
                pos -= 2;
                let d = -v;
                arr[pos] = b'0' as i8 + (d / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (d % 10) as i8;
            } else {
                pos -= 1;
                arr[pos] = b'0' as i8 - v as i8;
            }
            if negative {
                pos -= 1;
                arr[pos] = b'-' as i8;
            }
        })?;
        Ok(pos as i32)
    }

    #[jvm_native]
    pub fn uncheckedGetCharsUTF16_i_i_arr_b(i: i32, index: i32, buf: JArray<i8>) -> Result<i32> {
        let mut v = i;
        let negative = v < 0;
        if !negative {
            v = -v;
        }
        let mut pos = index as usize;
        buf.with_vec(|arr| {
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let d = q * 100 - v;
                _put_pair_utf16(arr, pos, d);
                v = q;
            }
            if v <= -10 {
                // 两位剩余（[-99,-10]）：-v 即两位数字面
                pos -= 2;
                _put_pair_utf16(arr, pos, -v);
            } else {
                // 单位剩余（[-9,0]）
                pos -= 1;
                _put_char_utf16(arr, pos, b'0' as i32 - v);
            }
            if negative {
                pos -= 1;
                _put_char_utf16(arr, pos, b'-' as i32);
            }
        })?;
        Ok(pos as i32)
    }

    #[jvm_native]
    pub fn uncheckedGetCharsUTF16_l_i_arr_b(i: i64, index: i32, buf: JArray<i8>) -> Result<i32> {
        let mut v = i;
        let negative = v < 0;
        if !negative {
            v = -v;
        }
        let mut pos = index as usize;
        buf.with_vec(|arr| {
            while v <= -100_000_000 {
                let q = v / 100_000_000;
                pos -= 8;
                let mut r = v - q * 100_000_000;
                for k in (0..4).rev() {
                    let pair = (0i64 - r % 100) as i64 as i32;
                    _put_pair_utf16(arr, pos + k, pair);
                    r /= 100;
                }
                v = q;
            }
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let d = q * 100 - v;
                _put_pair_utf16(arr, pos, d as i32);
                v = q;
            }
            if v <= -10 {
                pos -= 2;
                _put_pair_utf16(arr, pos, (-v) as i32);
            } else {
                pos -= 1;
                _put_char_utf16(arr, pos, (b'0' as i64 - v) as i32);
            }
            if negative {
                pos -= 1;
                _put_char_utf16(arr, pos, b'-' as i32);
            }
        })?;
        Ok(pos as i32)
    }

    /// `getChars(long, int, char[])`：类内唯一形态（无重载），Rust 侧裸名
    #[jvm_native]
    pub fn getChars(i: i64, index: i32, buf: JArray<u16>) -> Result<i32> {
        let mut v = i;
        let negative = v < 0;
        if !negative {
            v = -v;
        }
        let mut pos = index as usize;
        buf.with_vec(|arr| {
            while v <= -100_000_000 {
                let q = v / 100_000_000;
                pos -= 8;
                let mut r = v - q * 100_000_000;
                for k in (0..4).rev() {
                    let pair = (0i64 - r % 100) as i64;
                    arr[pos + k * 2] = b'0' as u16 + (pair / 10) as u16;
                    arr[pos + k * 2 + 1] = b'0' as u16 + (pair % 10) as u16;
                    r /= 100;
                }
                v = q;
            }
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let d = q * 100 - v;
                arr[pos] = b'0' as u16 + (d / 10) as u16;
                arr[pos + 1] = b'0' as u16 + (d % 10) as u16;
                v = q;
            }
            if v <= -10 {
                pos -= 2;
                let d = -v;
                arr[pos] = b'0' as u16 + (d / 10) as u16;
                arr[pos + 1] = b'0' as u16 + (d % 10) as u16;
            } else {
                pos -= 1;
                arr[pos] = (b'0' as i64 - v) as u16;
            }
            if negative {
                pos -= 1;
                arr[pos] = b'-' as u16;
            }
        })?;
        Ok(pos as i32)
    }

    /// `putPair(char[], int, int)`：两位数对（r ∈ [0,99]）写入 char[]
    #[jvm_native]
    pub fn putPair(buf: JArray<u16>, index: i32, r: i32) -> Result<()> {
        buf.with_vec(|arr| {
            let index = index as usize;
            arr[index] = b'0' as u16 + (r / 10) as u16;
            arr[index + 1] = b'0' as u16 + (r % 10) as u16;
        })
    }

    /// `uncheckedPutPairLatin1(byte[], int, int)`：两位数对写入 byte[]
    /// （`BigDecimal` 的外部调用点）
    #[jvm_native]
    pub fn uncheckedPutPairLatin1(buf: JArray<i8>, index: i32, r: i32) -> Result<()> {
        buf.with_vec(|arr| {
            let index = index as usize;
            arr[index] = b'0' as i8 + (r / 10) as i8;
            arr[index + 1] = b'0' as i8 + (r % 10) as i8;
        })
    }

    /// `uncheckedPutPairUTF16(byte[], int, int)`：两位数对按 char 单元写入
    /// byte[]（平台字节序，公开方法集补齐）
    #[jvm_native]
    pub fn uncheckedPutPairUTF16(buf: JArray<i8>, index: i32, r: i32) -> Result<()> {
        buf.with_vec(|arr| {
            _put_pair_utf16(arr, index as usize, r);
        })
    }
}
