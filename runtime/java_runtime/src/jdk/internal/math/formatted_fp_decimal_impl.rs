//! `jdk/internal/math/FormattedFPDecimal` 手写实现（内部边界类；仅当
//! `formatted_fp_decimal.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! 语义与 OpenJDK 21 同名类一致：`%e` / `%f` / `%g` 对 double 的十进制展开。
//! `valueOf` 把 v 分解为最短十进制表示 `d = f · 10^e`（Rust `{:e}` 与 JDK
//! DoubleToDecimal 同为最短往返表示），按 form 舍入（r(x) = floor(x + 1/2)，
//! HALF_UP）后生成尾数（`getMantissa`，含小数点）与指数（`getExponent`，
//! `[+-]ee` 形态）字符数组。符号、宽度、补零、本地化由调用方（Formatter）负责。

use crate::prelude::*;
use crate::java::lang::IllegalArgumentException;
use super::formatted_fp_decimal::FormattedFPDecimal;

impl FormattedFPDecimal {
    /// `valueOf(double, int, char)`：按 form（SCIENTIFIC='e'/PLAIN='f'/GENERAL='g'）
    /// 与 prec（精度）生成十进制展开。v 按约定非负（符号位由调用方剥离），
    /// 防御性地取绝对值。
    pub fn valueOf(v: f64, arg1: i32, prec: u16) -> Result<FormattedFPDecimal> {
        let mut fd = FormattedFPDecimal::default();
        fd._init_not_null();
        // split(v)：最短十进制 d = f·10^e；`{:e}` 给出 "d.ddde±x"（最短往返），
        // 尾数数字即 f（n 位），指数换算到 e。f = 0 时 n = 0（set 的契约）。
        let a = v.abs();
        let s = format!("{:e}", a);
        let (mant, exp10) = s.split_once('e').unwrap_or((s.as_str(), "0"));
        let exp10: i32 = exp10.parse().unwrap_or(0);
        let int_digits: Vec<u8> = mant.bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();
        // 去掉尾随零（最短表示下不会出现，防御）
        let mut end = int_digits.len();
        while end > 1 && int_digits[end - 1] == 0 {
            end -= 1;
        }
        let mut f: i64 = 0;
        for d in &int_digits[..end] {
            f = f * 10 + (*d as i64);
        }
        if f == 0 {
            fd.set(0, 0, 0)?; // n = 0 if f = 0（round 归一化为 f=0,e=0,n=1）
        } else {
            let n = end as i32;
            // d.ddd × 10^exp10 = f(整数数字串) × 10^(exp10 - (n - 1))
            fd.set(f, exp10 - (n - 1), n)?;
        }
        match prec as u32 {
            101 => {                                                     // SCIENTIFIC
                fd.fp_round((arg1 as i64) + 1);
                fd.fp_scientific_chars(arg1);
            }
            102 => {                                                     // PLAIN
                fd.fp_round((fd.__get_n() as i64) + (fd.__get_e() as i64) + arg1 as i64);
                fd.fp_plain_chars();
            }
            103 => {                                                     // GENERAL
                fd.fp_round(arg1 as i64);
                let er = fd.__get_n() + fd.__get_e() - 1;
                if -4 <= er && er < arg1 {
                    fd.fp_plain_chars();
                } else {
                    fd.fp_scientific_chars(arg1 - 1);
                }
            }
            _ => {
                return Err(JvmError::from(IllegalArgumentException::new_str(
                    String::from(format!("unsupported form '{}'", prec)))?));
            }
        }
        Ok(fd)
    }

    /// `set(long f, int e, int n)`：`DoubleToDecimal.split` 的落点。
    /// 契约：f = 0 时 n = 0；否则 10^(n-1) <= f < 10^n，v = f·10^e。
    pub fn set(&self, f: i64, arg1: i32, e: i32) -> Result<()> {
        self.__set_f(f);
        self.__set_e(arg1);
        self.__set_n(e);
        Ok(())
    }

    pub fn getExponent(&self) -> Result<JArray<u16>> {
        Ok(Clone::clone(&self.__get_exp()))
    }

    pub fn getMantissa(&self) -> Result<JArray<u16>> {
        Ok(Clone::clone(&self.__get_digits()))
    }

    pub fn getExponentRounded(&self) -> Result<i32> {
        Ok(self.__get_n() + self.__get_e() - 1)
    }

    // ── 以下为私有算法（非 `pub fn`，不参与 codegen 的手写方法扫描）──────────

    /// 把 d = f·10^e 舍入到最显著 pp 位（r(x) = floor(x + 1/2)，HALF_UP）。
    fn fp_round(&self, pp: i64) {
        let mut f = self.__get_f();
        let mut e = self.__get_e();
        let mut n = self.__get_n();
        if n == 0 || pp < 0 {
            self.__set_f(0);
            self.__set_e(0);
            self.__set_n(1);
            return;
        }
        if pp >= n as i64 {
            return;
        }
        let p = pp as i32;
        e += n - p;
        let pow10 = pow10_i64(n - p);
        f = (f + (pow10 >> 1)) / pow10;
        if p == 0 {
            self.__set_f(f);
            self.__set_e(if f == 0 { 0 } else { e });
            self.__set_n(1);
            return;
        }
        n = p;
        if f == pow10_i64(p) {
            // f 有 n + 1 位：吸收一个尾随零到 e
            f /= 10;
            e += 1;
        }
        self.__set_f(f);
        self.__set_e(e);
        self.__set_n(n);
    }

    fn fp_plain_chars(&self) {
        let (e, n) = (self.__get_e(), self.__get_n());
        if e >= 0 {
            self.fp_plain_pure_integer();
        } else if n + e > 0 {
            self.fp_plain_mixed();
        } else {
            self.fp_plain_pure_fraction();
        }
    }

    fn fp_plain_pure_integer(&self) {
        let (f, e, n) = (self.__get_f(), self.__get_e(), self.__get_n());
        let mut digits = JArray::<u16>::new((n + e) as i32);
        fill_with_zeros(&mut digits, n, n + e);
        fill_with_digits(&mut digits, f, 0, n);
        self.__set_digits(digits);
    }

    fn fp_plain_mixed(&self) {
        let (f, e, n) = (self.__get_f(), self.__get_e(), self.__get_n());
        let mut digits = JArray::<u16>::new(n + 1);
        let x = fill_with_digits(&mut digits, f, n + 1 + e, n + 1);
        let _ = digits.set((n + e) as i32, '.' as u16);
        let _ = fill_with_digits(&mut digits, x, 0, n + e);
        self.__set_digits(digits);
    }

    fn fp_plain_pure_fraction(&self) {
        let (f, e, n) = (self.__get_f(), self.__get_e(), self.__get_n());
        let mut digits = JArray::<u16>::new(2 - e);
        let _ = fill_with_digits(&mut digits, f, 2 - e - n, 2 - e);
        fill_with_zeros(&mut digits, 0, 2 - e - n);
        let _ = digits.set(1, '.' as u16);
        self.__set_digits(digits);
    }

    fn fp_scientific_chars(&self, prec: i32) {
        if prec != 0 {
            self.fp_scientific_with_fraction();
        } else {
            self.fp_scientific_no_fraction();
        }
        self.fp_exp_chars();
    }

    fn fp_scientific_with_fraction(&self) {
        let (f, n) = (self.__get_f(), self.__get_n());
        let mut digits = JArray::<u16>::new(1 + n);
        let x = fill_with_digits(&mut digits, f, 2, 1 + n);
        let _ = digits.set(1, '.' as u16);
        let _ = digits.set(0, to_digit(x));
        self.__set_digits(digits);
    }

    fn fp_scientific_no_fraction(&self) {
        let mut digits = JArray::<u16>::new(1);
        let _ = digits.set(0, to_digit(self.__get_f()));
        self.__set_digits(digits);
    }

    fn fp_exp_chars(&self) {
        let mut er = self.__get_n() + self.__get_e() - 1;
        let mut aer = er.abs();
        let mut exp = JArray::<u16>::new(if aer >= 100 { 4 } else { 3 });
        if aer >= 100 {
            let q = aer / 10;
            let _ = exp.set(3, to_digit((aer - 10 * q) as i64));
            aer = q;
        }
        let q = aer / 10;
        let _ = exp.set(2, to_digit((aer - 10 * q) as i64));
        let _ = exp.set(1, to_digit(q as i64));
        let _ = exp.set(0, if er >= 0 { '+' as u16 } else { '-' as u16 });
        self.__set_exp(exp);
    }
}

fn pow10_i64(i: i32) -> i64 {
    let mut r: i64 = 1;
    for _ in 0..i {
        r *= 10;
    }
    r
}

fn to_digit(d: i64) -> u16 {
    (d as u8 + b'0') as u16
}

/// 把 x 的低 (to - from) 位数字填入 digits 的 [from, to)（从右往左），
/// 返回剥离后的 x。
fn fill_with_digits(digits: &mut JArray<u16>, mut x: i64, from: i32, to: i32) -> i64 {
    let mut to = to;
    while to > from {
        let q = x / 10;
        to -= 1;
        let _ = digits.set(to, to_digit(x - q * 10));
        x = q;
    }
    x
}

fn fill_with_zeros(digits: &mut JArray<u16>, from: i32, to: i32) {
    let mut to = to;
    while to > from {
        to -= 1;
        let _ = digits.set(to, '0' as u16);
    }
}
