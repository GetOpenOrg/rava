use crate::prelude::*;
use super::*;

/// `format!("{:e}", v)` 的解析结果：尾数数字串 + 指数 + 符号。
struct SciDigits {
    negative: bool,
    body: std::string::String,
}

fn split_scientific(s: &str) -> (SciDigits, i32) {
    let (mantissa, exp) = match s.split_once('e') {
        Some((m, e)) => (m, e.parse::<i32>().unwrap_or(0)),
        None => (s, 0),
    };
    let negative = mantissa.starts_with('-');
    (SciDigits { negative, body: mantissa.trim_start_matches('-').replace('.', "") }, exp)
}

/// 平局判定：Rust 给出的尾数 `d`（`n` 位、指数 `exp`）是否恰好落在两个候选的正中间。
///
/// 记末位单位 `u = 10^(exp - n + 1)`，平局 ⟺ `2v/u` 是奇整数且等于 `2d - 1`
///（Rust 对平局远离零进位，故它给的是较大的那个候选）。
/// `v` 按 `m × 2^E` 精确分解后做整数运算；溢出 / 不可整除一律判为非平局。
fn is_tie(value: f64, d: u128, exp: i32, n: usize) -> bool {
    fn try_tie(value: f64, exp: i32, n: usize) -> Option<u128> {
        let bits = value.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i32;
        let (mut m, mut e) = if biased == 0 {
            (bits & ((1u64 << 52) - 1), -1074) // 非规格化
        } else {
            ((bits & ((1u64 << 52) - 1)) | (1u64 << 52), biased - 1075)
        };
        if m == 0 {
            return None;
        }
        let tz = m.trailing_zeros();
        m >>= tz; // 归一化：m 为奇数
        e += tz as i32;
        let k = exp - n as i32 + 1;
        let j = e + 1 - k;
        if j < 0 {
            return None;
        }
        let mut a = (m as u128).checked_mul(1u128.checked_shl(j as u32)?)?;
        if k < 0 {
            a = a.checked_mul(5u128.checked_pow((-k) as u32)?)?;
        } else if k > 0 {
            let f5 = 5u128.checked_pow(k as u32)?;
            if a % f5 != 0 {
                return None;
            }
            a /= f5;
        }
        Some(a)
    }
    match try_tie(value, exp, n) {
        Some(a) => a % 2 == 1 && a == 2 * d - 1,
        None => false,
    }
}

/// Java `Double.toString(double)` 的输出格式（等价 JDK 19+ 的 `DoubleToDecimal`）。
///
/// 规则：取**最短往返**十进制数字串，再按数量级排版
///   - `1e-3 ≤ |v| < 1e7`：普通小数，至少有 1 位小数（`1.0` / `0.001` / `1234567.0`）
///   - 其余：科学计数法 `d.dddE±n`（E 后不带 `+`、不补零）
///
/// Rust 的 `{:e}` 同样是"最短往返"表示，直接取其尾数数字与指数重排即可。
fn java_double_string(value: f64) -> std::string::String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return (if value.is_sign_negative() { "-Infinity" } else { "Infinity" }).to_string();
    }
    let (mut digits, mut exp) = split_scientific(&std::format!("{:e}", value));
    let negative = digits.negative;
    // Java 的科学计数法尾数至少 2 位有效数字（"至少有 1 位小数"）。Rust 的 `{:e}`
    // 在最短表示只有 1 位时会给 `5e-324`，Java 取该长度下的最近值 `4.9E-324`。
    // 该规则只作用于科学计数法区间，普通小数区（`0.5` / `0.001`）保持原样。
    let scientific = !(0..7).contains(&exp) && !(-3..0).contains(&exp);
    if scientific && digits.body.len() < 2 {
        let (d2, e2) = split_scientific(&std::format!("{:.1e}", value));
        digits = d2;
        exp = e2;
    }
    // 末位恰好是 `.5` 的平局：Rust 进位（远离零），Java 取偶数末位。
    // 平局 ⟺ 2v/u 是奇整数且等于 2D-1（u = 10^(exp-n+1) 是末位的单位）。
    // 用 v 的精确二进制分解做整数判定，避免"解析中点"这种恒真判据。
    if let Some(d) = digits.body.parse::<u128>().ok() {
        if d >= 2 && d % 2 == 1 && is_tie(value, d, exp, digits.body.len()) {
            digits.body = (d - 1).to_string();
        }
    }
    let digits = digits.body;
    let n = digits.len() as i32; // 有效数字位数（≥1）
    let body = if (0..7).contains(&exp) {
        // 普通小数：整数部分位数 = exp + 1
        let int_len = exp + 1;
        if n <= int_len {
            std::format!("{}{}.0", digits, "0".repeat((int_len - n) as usize))
        } else {
            std::format!("{}.{}", &digits[..int_len as usize], &digits[int_len as usize..])
        }
    } else if (-3..0).contains(&exp) {
        std::format!("0.{}{}", "0".repeat((-exp - 1) as usize), digits)
    } else {
        let frac = if n > 1 {
            std::format!(".{}", &digits[1..])
        } else {
            ".0".to_string()
        };
        std::format!("{}{}E{}", &digits[..1], frac, exp)
    };
    std::format!("{}{}", if negative { "-" } else { "" }, body)
}

impl Double {
    #[jvm_native]
    pub fn doubleToRawLongBits(value: f64) -> Result<i64> {
        Ok(value.to_bits() as i64)
    }

    #[jvm_native]
    pub fn longBitsToDouble(bits: i64) -> Result<f64> {
        Ok(f64::from_bits(bits as u64))
    }

    /// `Double.toString(double)`：静态版本，`String.valueOf(double)` 与
    /// `PrintStream.println(double)` 都经此，故不再走到 `DoubleToDecimal` 的字节码存根。
    #[jvm_native]
    pub fn toString_d(value: f64) -> Result<String> {
        Ok(String::from(java_double_string(value).as_str()))
    }

    /// `Double.toString()`：实例版本（装箱后 `println(obj)` 走的路径）。
    #[jvm_native]
    pub fn __impl_toString(&self) -> Result<String> {
        Ok(String::from(java_double_string(self.__get_value()).as_str()))
    }
}
