//! `jdk/internal/util/DecimalDigits` 手写实现（JDK 25 语料）。
//!
//! JDK 25 把整数→十进制字符的底层算法从 `Integer`/`Long` 的私有静态方法
//! 抽到了这个内部类（`Integer.toString` 经 `stringSize` + `uncheckedGetCharsLatin1`
//! 构造紧凑字符串）。算法按字节码逐指令还原（负数幅值比较，规避 MIN_VALUE 取反溢出）：
//!
//! - `stringSize`：十进制位数，**负数含符号位**（stringSize(-123)=4、MIN_VALUE=11）
//! - `uncheckedGetCharsLatin1/UTF16`：从 `index`（终点，不含）向左写入数字字符，
//!   返回新的起点；负数在数字前补 `-`。UTF16 变体按 Java 紧凑字符串的
//!   UTF-16BE 双字节写入
//! - `getChars(long, int, char[])`：同语义的 char[] 版本
//! - `putPair` 族：两位数对查表写入——表用算术等价替换（r/10、r%10）

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
                let r = v - q * 100;
                arr[pos] = b'0' as i8 + (r / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (r % 10) as i8;
                v = q;
            }
            pos -= 1;
            arr[pos] = b'0' as i8 - v as i8; // v ∈ [-9,0]，数字 = -v
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
                    let pair = (r % 100) as i64;
                    arr[pos + k * 2] = b'0' as i8 + (pair / 10) as i8;
                    arr[pos + k * 2 + 1] = b'0' as i8 + (pair % 10) as i8;
                    r /= 100;
                }
                v = q;
            }
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let r = v - q * 100;
                arr[pos] = b'0' as i8 + (r / 10) as i8;
                arr[pos + 1] = b'0' as i8 + (r % 10) as i8;
                v = q;
            }
            pos -= 1;
            arr[pos] = b'0' as i8 - v as i8;
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
                let r = v - q * 100;
                let p2 = pos << 1;
                arr[p2] = b'0' as i8 + (r / 10) as i8;
                arr[p2 + 2] = b'0' as i8 + (r % 10) as i8;
                v = q;
            }
            pos -= 1;
            let p2 = pos << 1;
            arr[p2] = b'0' as i8 - v as i8;
            arr[p2 + 1] = 0;
            if negative {
                pos -= 1;
                let p2 = pos << 1;
                arr[p2] = b'-' as i8;
                arr[p2 + 1] = 0;
            }
        })?;
        Ok(pos as i32)
    }

    #[jvm_native]
    pub fn getChars_l_i_arr_c(i: i64, index: i32, buf: JArray<u16>) -> Result<i32> {
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
                    let pair = (r % 100) as i64;
                    arr[pos + k * 2] = b'0' as u16 + (pair / 10) as u16;
                    arr[pos + k * 2 + 1] = b'0' as u16 + (pair % 10) as u16;
                    r /= 100;
                }
                v = q;
            }
            while v <= -100 {
                let q = v / 100;
                pos -= 2;
                let r = v - q * 100;
                arr[pos] = b'0' as u16 + (r / 10) as u16;
                arr[pos + 1] = b'0' as u16 + (r % 10) as u16;
                v = q;
            }
            pos -= 1;
            arr[pos] = (b'0' as i64 - v) as u16;
            if negative {
                pos -= 1;
                arr[pos] = b'-' as u16;
            }
        })?;
        Ok(pos as i32)
    }
}
