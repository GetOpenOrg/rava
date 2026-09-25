//! `jdk/internal/math/FloatingDecimal` 手写伴生（仅当 `floating_decimal.rs`
//! 进入闭包生成时编译，见 K-2 规则）。
//!
//! JDK 的 `getBinaryToASCIIConverter` 走传统 dtoa 算法（二进制→最短十进制 ASCII，
//! `FloatingDecimal$BinaryToASCIIToStringBuffer`）。本实现按「单一最短表示引擎」
//! 复用 Schubfach 移植（`double_to_decimal_impl.rs` / `float_to_decimal_impl.rs`，
//! 与 `Double.toString`/`Float.toString` 同源）——最短往返位串等价，语料消费面
//! （`DigitList.set` 的舍入路径）观察一致。

use crate::prelude::*;
use super::floating_decimal::FloatingDecimal;
use super::floating_decimal_binary_to_ascii_converter::FloatingDecimal_BinaryToASCIIConverter__VTable;
use super::double_to_decimal::DoubleToDecimal;
use super::float_to_decimal::FloatToDecimal;
use crate::java::lang::{Appendable, String};
use std::rc::Rc;

/// Schubfach 引擎包装的 `BinaryToASCIIConverter`（接口实现对象，经
/// `ObjectVTable::__interface` 应答；与手写 access 对象同形态）。
struct SchubfachConverter {
    negative: bool,
    exceptional: bool,
    exact_as_decimal: bool,
    formatted: String,
}

impl SchubfachConverter {
    fn new(v: f64) -> Self {
        Self {
            negative: v.is_sign_negative(),
            exceptional: !v.is_finite(),
            exact_as_decimal: _exact_as_decimal(v),
            formatted: DoubleToDecimal::toString(v).unwrap_or_default(),
        }
    }

    fn new_f32(v: f32) -> Self {
        let d = v as f64;
        Self {
            negative: v.is_sign_negative(),
            exceptional: !v.is_finite(),
            exact_as_decimal: d.is_finite() && _exact_as_decimal(d),
            formatted: FloatToDecimal::toString(v).unwrap_or_default(),
        }
    }
}

/// dtoa 的 `decimalDigitsExact` 语义近似：值的精确十进制展开在 19 位内终止
/// （dtoa 的 MAX_DECIMAL_DIGITS 域）。`v * 10^k` 在 k ≤ 19 内为精确整数即可
/// 判真（0.25/25.0 → true；99.99 的二进制近似 → false，与 dtoa 判定一致）。
fn _exact_as_decimal(v: f64) -> bool {
    if !v.is_finite() {
        return false;
    }
    let mut scaled = v.abs();
    for _ in 0..=19 {
        let trunc = scaled.trunc();
        if (scaled - trunc).abs() < f64::EPSILON * trunc.max(1.0) / 2.0 {
            return true;
        }
        scaled *= 10.0;
    }
    false
}

impl FloatingDecimal_BinaryToASCIIConverter__VTable for SchubfachConverter {
    fn toJavaFormatString(&self) -> Result<String> {
        Ok(Clone::clone(&self.formatted))
    }

    fn appendTo(&self, arg0: Appendable) -> Result<()> {
        // 与 FloatToDecimal.appendTo 的 Appendable 分派同形态：逐 char 追加
        // （byte 符号扩展到 int 再截位到 char）。
        // A-4 批次 6：itable 擦除签名的接口位按载体形态书写（本文件被编译的
        // 闭包内 Appendable 必经 T88 入闭包——本方法体本就引用其载体）。
        let val = self.formatted.__get_value();
        let mut app = Clone::clone(&arg0);
        for i in 0..val.len()? {
            let c = val.get(i)? as i32 as u16;
            app = app.append_c(c)?;
        }
        Ok(())
    }

    fn isNegative(&self) -> Result<bool> {
        Ok(self.negative)
    }

    fn isExceptional(&self) -> Result<bool> {
        Ok(self.exceptional)
    }

    /// dtoa 在最短化中产生进位时置位；Schubfach 的 s/t 选择不外露进位信息，
    /// 语料消费面（DigitList 舍入边界）不受影响，恒 false。
    fn digitsRoundedUp(&self) -> Result<bool> {
        Ok(false)
    }

    fn decimalDigitsExact(&self) -> Result<bool> {
        Ok(self.exact_as_decimal)
    }
}

impl ObjectVTable for SchubfachConverter {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __class_name(&self) -> &'static str {
        "jdk/internal/math/FloatingDecimal$BinaryToASCIIToStringBuffer"
    }
    fn __obj_str(&self) -> std::string::String {
        "jdk.internal.math.FloatingDecimal$BinaryToASCIIToStringBuffer".to_owned()
    }
    /// 接口视图查询（invokeinterface / 载体转换的运行时入口）。
    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {
        if let Some(s) = slot.downcast_mut::<Option<Rc<dyn FloatingDecimal_BinaryToASCIIConverter__VTable>>>() {
            *s = Some(self);
        }
    }
}

/// `readJavaFormatString` 的解析结果（符号已并入数值）。
enum Parsed {
    Decimal(std::string::String),   // Rust 可直接按目标精度正确舍入解析的十进制文本
    Hex { negative: bool, mantissa: u128, sticky: bool, exp2: i64 },
    NaN,
    Infinity(bool),
}

/// JDK `FloatingDecimal.readJavaFormatString` 的语法：`String.trim()` 后
/// `[+-]? (NaN | Infinity | 十进制 | 0x十六进制p指数) [fFdD]?`，整串须耗尽。
/// 十进制：至少一位数字，小数点可在任意位置，`e/E[+-]?数字` 可选。
/// 失败 → `None`（调用方抛 NumberFormatException，消息为 trim 后原文）。
fn read_java_format_string(t: &str) -> Option<Parsed> {
    let b = t.as_bytes();
    let mut i = 0;
    let mut negative = false;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        negative = b[i] == b'-';
        i += 1;
    }
    let rest = &t[i..];
    if rest == "NaN" {
        return Some(Parsed::NaN);
    }
    if rest == "Infinity" {
        return Some(Parsed::Infinity(negative));
    }
    if rest.len() >= 2 && (rest.starts_with("0x") || rest.starts_with("0X")) {
        return read_hex(&rest[2..], negative);
    }
    let start = i;
    let mut digits = 0usize;
    while i < b.len() && b[i].is_ascii_digit() { i += 1; digits += 1; }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() { i += 1; digits += 1; }
    }
    if digits == 0 {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        i += 1;
        if i < b.len() && (b[i] == b'+' || b[i] == b'-') { i += 1; }
        let exp_start = i;
        while i < b.len() && b[i].is_ascii_digit() { i += 1; }
        if i == exp_start {
            return None;
        }
    }
    let end = i;
    if i < b.len() && matches!(b[i], b'f' | b'F' | b'd' | b'D') { i += 1; }
    if i != b.len() {
        return None;
    }
    let mut text = std::string::String::with_capacity(end - start + 1);
    if negative { text.push('-'); }
    text.push_str(&t[start..end]);
    Some(Parsed::Decimal(text))
}

/// 十六进制浮点（`0x` 之后）：`十六进制位[.十六进制位] p[+-]?十进制指数 [fFdD]?`，p 段必需。
fn read_hex(s: &str, negative: bool) -> Option<Parsed> {
    let b = s.as_bytes();
    let (mut i, mut digits, mut mantissa, mut sticky, mut exp2) = (0usize, 0usize, 0u128, false, 0i64);
    let mut seen_dot = false;
    while i < b.len() {
        let c = b[i];
        if c == b'.' && !seen_dot {
            seen_dot = true;
        } else if let Some(v) = (c as char).to_digit(16) {
            digits += 1;
            if mantissa >> 120 == 0 {
                mantissa = (mantissa << 4) | v as u128;
                if seen_dot { exp2 -= 4; }
            } else {
                sticky |= v != 0;           // 超出 124 位的低位只影响舍入粘滞位
                if !seen_dot { exp2 += 4; }
            }
        } else {
            break;
        }
        i += 1;
    }
    if digits == 0 || i >= b.len() || (b[i] != b'p' && b[i] != b'P') {
        return None;
    }
    i += 1;
    let mut exp_neg = false;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') { exp_neg = b[i] == b'-'; i += 1; }
    let exp_start = i;
    let mut e: i64 = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        e = (e * 10 + (b[i] - b'0') as i64).min(1 << 40);
        i += 1;
    }
    if i == exp_start {
        return None;
    }
    if i < b.len() && matches!(b[i], b'f' | b'F' | b'd' | b'D') { i += 1; }
    if i != b.len() {
        return None;
    }
    exp2 += if exp_neg { -e } else { e };
    Some(Parsed::Hex { negative, mantissa, sticky, exp2 })
}

/// mantissa·2^exp2 按 IEEE 就近偶舍入到 `mant_bits` 位有效数（含隐藏位）的浮点，
/// 以 f64 位型返回（f32 调用方再按同一规则取 24 位后无损转换）。
fn hex_to_f64(mantissa: u128, sticky: bool, exp2: i64, mant_bits: u32, min_exp: i64, max_exp: i64) -> f64 {
    if mantissa == 0 {
        return 0.0;
    }
    let width = 128 - mantissa.leading_zeros() as i64;      // 有效位数
    let unbiased = exp2 + width - 1;                         // 最高位的二进制指数
    if unbiased > max_exp {
        return f64::INFINITY;
    }
    // 目标有效位数：正规数 mant_bits；次正规数随指数递减
    let keep = if unbiased >= min_exp { mant_bits as i64 } else { mant_bits as i64 - (min_exp - unbiased) };
    if keep <= 0 {
        // 低于最小次正规数的一半以下 → 0；恰在一半以上 → 最小次正规数
        let half = keep == 0 && (mantissa > (1u128 << (width - 1)) || sticky || mantissa.count_ones() > 1);
        return if half { scale_pow2(1.0, min_exp - mant_bits as i64 + 1) } else { 0.0 };
    }
    let drop = width - keep;
    let (m, lsb_exp) = if drop > 0 {
        let d = drop as u32;
        let kept = mantissa >> d;
        let rem = mantissa & ((1u128 << d) - 1);
        let half = 1u128 << (d - 1);
        let round_up = rem > half || (rem == half && (sticky || kept & 1 == 1));
        (kept + round_up as u128, exp2 + drop)
    } else {
        (mantissa, exp2)
    };
    // m ≤ 2^(mant_bits+1)，f64 精确表示；按 2 的幂缩放在结果可表示时精确
    //（进位到 2^(max_exp+1) 时自然得到 Infinity）
    scale_pow2(m as f64, lsb_exp)
}

/// v·2^e：分段缩放，避免 2^e 本身越界（结果可表示时保持精确）。
fn scale_pow2(mut v: f64, mut e: i64) -> f64 {
    while e > 1000 {
        v *= 2f64.powi(1000);
        e -= 1000;
        if v.is_infinite() { return v; }
    }
    while e < -1000 {
        v *= 2f64.powi(-1000);
        e += 1000;
        if v == 0.0 { return v; }
    }
    v * 2f64.powi(e as i32)
}

fn number_format_error(t: &str) -> JvmError {
    match crate::java::lang::NumberFormatException::new_str(String::from(
        if t.is_empty() { "empty String".to_owned() } else { format!("For input string: \"{}\"", t) })) {
        Ok(e) => JvmError::from(e),
        Err(e) => e,
    }
}

/// `String.trim()`：去除首尾 <= U+0020 的字符。
fn java_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c <= ' ')
}

impl FloatingDecimal {
    /// `parseDouble(String)`（`Double.parseDouble` / `Double.valueOf(String)` 的落点）：
    /// JDK `readJavaFormatString(in).doubleValue()` 语义——语法见 `read_java_format_string`，
    /// 十进制经 Rust 标准库正确舍入（与 JDK 的正确舍入一致），十六进制按 IEEE 就近偶。
    /// null → NullPointerException；语法错误 → NumberFormatException（消息同 JDK）。
    #[jvm_boundary(upcalls = "java/lang/NumberFormatException.<init>:(Ljava/lang/String;)V")]
    pub fn parseDouble(s: String) -> Result<f64> {
        if s.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let raw = format!("{}", s);
        let t = java_trim(&raw);
        match read_java_format_string(t) {
            Some(Parsed::Decimal(text)) => text.parse::<f64>().map_err(|_| number_format_error(t)),
            Some(Parsed::Hex { negative, mantissa, sticky, exp2 }) => {
                let v = hex_to_f64(mantissa, sticky, exp2, 53, -1022, 1023);
                Ok(if negative { -v } else { v })
            }
            Some(Parsed::NaN) => Ok(f64::NAN),
            Some(Parsed::Infinity(neg)) => Ok(if neg { f64::NEG_INFINITY } else { f64::INFINITY }),
            None => Err(number_format_error(t)),
        }
    }

    /// `parseFloat(String)`：同 `parseDouble` 语法，直接舍入到 float（非经 double 二次舍入）。
    #[jvm_boundary(upcalls = "java/lang/NumberFormatException.<init>:(Ljava/lang/String;)V")]
    pub fn parseFloat(s: String) -> Result<f32> {
        if s.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let raw = format!("{}", s);
        let t = java_trim(&raw);
        match read_java_format_string(t) {
            Some(Parsed::Decimal(text)) => text.parse::<f32>().map_err(|_| number_format_error(t)),
            Some(Parsed::Hex { negative, mantissa, sticky, exp2 }) => {
                let v = hex_to_f64(mantissa, sticky, exp2, 24, -126, 127) as f32;
                Ok(if negative { -v } else { v })
            }
            Some(Parsed::NaN) => Ok(f32::NAN),
            Some(Parsed::Infinity(neg)) => Ok(if neg { f32::NEG_INFINITY } else { f32::INFINITY }),
            None => Err(number_format_error(t)),
        }
    }

    /// `getBinaryToASCIIConverter(D)`：异常值（NaN/Inf）在 JDK 里返回
    /// ExceptionalBinaryToASCIIToStringBuffer——Schubfach 包装的 toString 同样
    /// 产出 "NaN"/"Infinity" 文本，统一走同一包装。
    #[jvm_boundary]
    pub fn getBinaryToASCIIConverter_d(d: f64) -> Result<Object> {
        Ok(Object::from(SchubfachConverter::new(d)))
    }

    /// `getBinaryToASCIIConverter(D, Z)`：isCompatibleFormat 只影响旧 dtoa 的
    /// 兼容格式微调；最短表示引擎无该形态差异，同包装。
    #[jvm_boundary]
    pub fn getBinaryToASCIIConverter_d_z(d: f64, isCompatibleFormat: bool) -> Result<Object> {
        let _ = isCompatibleFormat;
        Ok(Object::from(SchubfachConverter::new(d)))
    }

    /// `getBinaryToASCIIConverter(F)`：float 版（DigitList 无此消费，float
    /// 语料邻域按需）。
    #[jvm_boundary]
    pub fn getBinaryToASCIIConverter_f(f: f32) -> Result<Object> {
        Ok(Object::from(SchubfachConverter::new_f32(f)))
    }
}
