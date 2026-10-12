//! Java 浮点十进制表示与字符排版（自 java_runtime lib.rs 随标记层基础迁入，§9.9 K4 阶段 3：
//! 基本类型盒的 `impl ObjectVTable`（object.rs）引用这三个函数）。
//!
//! `Double.toString` / `Float.toString` 语义的唯一实现，三处入口共用：字符串拼接（codegen
//! 拼接发射 `java_fmt_*`）、装箱值 `toString`（object.rs `impl_vtable_primitive!`）、
//! `Double/Float.toString`（行为层 double_impl.rs / float_impl.rs 转发，经 java_runtime
//! 再导出取同名函数）。
//!
//! 规则（JLS / Double.toString）：取**最短往返**十进制数字串，再按数量级排版
//!   - `1e-3 ≤ |v| < 1e7`：普通小数，至少有 1 位小数（`1.0` / `0.001` / `1234567.0`）
//!   - 其余：科学计数法 `d.dddE±n`（E 后不带 `+`、不补零）
//!
//! Rust 的 `{:e}` 同样是"最短往返"表示，直接取其尾数数字与指数重排；两处与
//! Rust 的差异在下方逐点修正（科学计数法至少 2 位有效数字、平局取偶数末位）。

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

/// 平局判定核心：尾数 `d`（`n` 位、指数 `exp`）是否恰好落在两个候选的正中间。
///
/// 记末位单位 `u = 10^(exp - n + 1)`，平局 ⟺ `2v/u` 是奇整数且等于 `2d - 1`
///（Rust 对平局远离零进位，故它给的是较大的那个候选）。
/// `v` 按 `m × 2^e`（m 为奇数）精确分解后做整数运算；溢出 / 不可整除一律判为非平局。
fn tie_odd_half(m: u64, e: i32, d: u128, exp: i32, n: usize) -> bool {
    let k = exp - n as i32 + 1;
    let j = e + 1 - k;
    if j < 0 || j > 127 {
        return false;
    }
    let mut a = match (m as u128).checked_mul(1u128 << j) {
        Some(a) => a,
        None => return false,
    };
    if k < 0 {
        match 5u128.checked_pow((-k) as u32) {
            Some(f5) => match a.checked_mul(f5) { Some(p) => a = p, None => return false },
            None => return false,
        }
    } else if k > 0 {
        match 5u128.checked_pow(k as u32) {
            Some(f5) => {
                if a % f5 != 0 { return false; }
                a /= f5;
            }
            None => return false,
        }
    }
    a % 2 == 1 && a == 2 * d - 1
}

/// f64 的 `m × 2^e`（m 奇）精确分解；零返回 None。
fn f64_odd_mantissa(value: f64) -> Option<(u64, i32)> {
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
    Some((m, e))
}

/// f32 的 `m × 2^e`（m 奇）精确分解；零返回 None。
fn f32_odd_mantissa(value: f32) -> Option<(u64, i32)> {
    let bits = value.to_bits();
    let biased = ((bits >> 23) & 0xff) as i32;
    let (mut m, mut e) = if biased == 0 {
        ((bits & ((1u32 << 23) - 1)) as u64, -149) // 非规格化
    } else {
        (((bits & ((1u32 << 23) - 1)) | (1u32 << 23)) as u64, biased - 151)
    };
    if m == 0 {
        return None;
    }
    let tz = m.trailing_zeros();
    m >>= tz;
    e += tz as i32;
    Some((m, e))
}

/// 数字串 + 指数 → Java 排版（普通小数 / 科学计数）。
fn java_decimal_layout(digits: &str, negative: bool, exp: i32) -> std::string::String {
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
    if negative { std::format!("-{}", body) } else { body }
}

/// 共用管线：最短往返数字串 →（科学区间）至少 2 位 →（平局）取偶数末位 → 排版。
/// `sci`/`sci2` 依次为 `{:e}` / `{:.1e}` 的格式化闭包，`mantissa` 给出精确分解。
fn java_float_repr(
    sci: std::string::String,
    sci2: std::string::String,
    mantissa: Option<(u64, i32)>,
) -> std::string::String {
    let (mut digits, mut exp) = split_scientific(&sci);
    let negative = digits.negative;
    let scientific = !(0..7).contains(&exp) && !(-3..0).contains(&exp);
    if scientific && digits.body.len() < 2 {
        // Java 的科学计数法尾数至少 2 位有效数字（"至少有 1 位小数"）。Rust 的
        // `{:e}` 在最短表示只有 1 位时会给 `5e-324`，Java 取该长度下的最近值
        // `4.9E-324`。该规则只作用于科学计数法区间，普通小数区保持原样。
        let (d2, e2) = split_scientific(&sci2);
        digits = d2;
        exp = e2;
    }
    // 末位恰好是 `.5` 的平局：Rust 进位（远离零），Java 取偶数末位。
    if let Some(d) = digits.body.parse::<u128>().ok() {
        if d >= 2 && d % 2 == 1 {
            if let Some((m, e)) = mantissa {
                if tie_odd_half(m, e, d, exp, digits.body.len()) {
                    digits.body = (d - 1).to_string();
                }
            }
        }
    }
    java_decimal_layout(&digits.body, negative, exp)
}

/// Java `Double.toString(double)` 的输出格式（字符串拼接 / 装箱 toString 同语义）。
pub fn java_fmt_f64(v: f64) -> std::string::String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-Infinity" } else { "Infinity" }).to_string();
    }
    java_float_repr(
        std::format!("{:e}", v),
        std::format!("{:.1e}", v),
        f64_odd_mantissa(v),
    )
}

/// Java `Character.toString(char)`：UTF-16 码元按字符输出（孤立代理项 → U+FFFD，
/// 与 String 的 UTF-8 呈现同一替换策略）。原生 char 盒（`Object::from(u16)`）的 toString。
pub fn java_fmt_char(v: u16) -> std::string::String {
    char::decode_utf16([v]).next()
        .map(|r| r.unwrap_or('\u{FFFD}'))
        .map(|c| c.to_string())
        .unwrap_or_default()
}

/// Java `Float.toString(float)` 的输出格式（规则同 Double，数字串按 f32 精度）。
pub fn java_fmt_f32(v: f32) -> std::string::String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return (if v.is_sign_negative() { "-Infinity" } else { "Infinity" }).to_string();
    }
    java_float_repr(
        std::format!("{:e}", v),
        std::format!("{:.1e}", v),
        f32_odd_mantissa(v),
    )
}
