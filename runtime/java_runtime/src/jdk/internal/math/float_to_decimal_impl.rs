//! `jdk/internal/math/FloatToDecimal` 手写实现（内部边界类；仅当
//! `float_to_decimal.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! 算法按 OpenJDK 21 字节码逐指令还原（javap 对照 `toDecimal`/`toDecimal(III)`/
//! `rop`/`toChars`/`toChars1..3`/`y`/`exponent`/`append8Digits`/
//! `removeTrailingZeroes`/`append`/`appendDigit` 全量方法体）：float 的
//! shortest round-trip 十进制表示（Schubfach，[1] §9 n=9）。与 double 版
//! （double_to_decimal_impl.rs）的结构对应差异：
//! - P=24/W=8：q/c/cb/vb 全 int 域（double 版为 long 域）；
//! - `g = g1(k) + 1` 单 long（double 版 g = g1·2^63 + g0 双半）；
//! - k ∈ [K_MIN=-45, K_MAX=31]，g 表取 MathUtils 全表的该区段；
//! - rop 以 2^95 为缩放（x1 >>> 31 | 进位位），double 版为 2^127。
//!
//! Java int 运算的溢出回绕与 `>>>` 逻辑右移在 Rust 显式建模（wrapping 系 +
//! u32/u64 中转）。本文件只实现调用链触达的 `toString`/`appendTo`。
//!
//! [1] Giulietti, "The Schubfach way to render doubles", 2020.

use crate::prelude::*;
use super::float_to_decimal::FloatToDecimal;
use crate::java::lang::Appendable;
use crate::java::lang::String;

// ── FloatToDecimal 常量（javap ConstantValue 属性；生成侧为访问器 fn，
//    此处用局部 const 等价引用） ──────────────────────────────────
const P: i32 = 24;
const Q_MIN: i32 = -149;
const C_TINY: i32 = 8;
const K_MIN: i32 = -45;
const H: i32 = 9;
const C_MIN: i32 = 1 << (P - 1);
const BQ_MASK: i32 = (1 << 8) - 1;
const T_MASK: i32 = (1 << (P - 1)) - 1;
const MASK_32: i64 = (1i64 << 32) - 1;
const MASK_28: i32 = (1 << 28) - 1;
const NON_SPECIAL: i32 = 0;
const PLUS_ZERO: i32 = 1;
const MINUS_ZERO: i32 = 2;
const PLUS_INF: i32 = 3;
const MINUS_INF: i32 = 4;
const NAN: i32 = 5;
const MAX_CHARS: i32 = H + 6;

// ── MathUtils 常量（javap：ConstantValue 属性） ─────────────────────
const Q_10: i64 = 41;
const C_10: i64 = 661_971_961_083;
const A_10: i64 = -274_743_187_321;
const Q_2: i64 = 38;
const C_2: i64 = 913_124_641_741;

/// 10^0 .. 10^8（toChars 的 `f *= pow10(H - len)`，len ∈ [1, H]）
static POW10: [i64; 9] = [
    1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000,
];

/// MathUtils.flog10pow2(e) = floor(log10(2^e))
fn _flog10pow2(e: i32) -> i32 {
    ((e as i64).wrapping_mul(C_10) >> Q_10) as i32
}

/// MathUtils.flog10threeQuartersPow2(e) = floor(log10(3/4 · 2^e))
fn _flog10_three_quarters_pow2(e: i32) -> i32 {
    (((e as i64).wrapping_mul(C_10)).wrapping_add(A_10) >> Q_10) as i32
}

/// MathUtils.flog2pow10(e) = floor(log2(10^e))
fn _flog2pow10(e: i32) -> i32 {
    ((e as i64).wrapping_mul(C_2) >> Q_2) as i32
}

fn _pow10(e: i32) -> i64 {
    POW10[e as usize]
}

/// Math.multiplyHigh（有符号 64 位高半乘积）
fn _multiply_high(x: i64, y: i64) -> i64 {
    ((x as i128 * y as i128) >> 64) as i64
}

// ── 私有算法（实例态：bytes 缓冲 + index 写指针，经访问器读写） ──

/// `append(int c)`：`bytes[++index] = (byte) c`
fn _append(d: &FloatToDecimal, c: u8) -> Result<()> {
    let i = d.__get_index() + 1;
    d.__set_index(i);
    d.__get_bytes().set(i, c as i8)
}

/// `appendDigit(int d)`：`bytes[++index] = (byte) ('0' + d)`
fn _append_digit(d: &FloatToDecimal, digit: i32) -> Result<()> {
    _append(d, (b'0' as i32 + digit) as u8)
}

/// `y(int a)`：floor((a + 1) 2^28 / 10^8) - 1（左到右逐位提取，[3] 算法 1）
fn _y(a: i32) -> i32 {
    (((_multiply_high(((a + 1) as i64) << 28, 193_428_131_138_340_668i64) as u64) >> 20) as i32) - 1
}

/// `removeTrailingZeroes()`：去尾零，但保留 '.' 右侧那一个
fn _remove_trailing_zeroes(d: &FloatToDecimal) -> Result<()> {
    while d.__get_bytes().get(d.__get_index())? == b'0' as i8 {
        d.__set_index(d.__get_index() - 1);
    }
    if d.__get_bytes().get(d.__get_index())? == b'.' as i8 {
        d.__set_index(d.__get_index() + 1);
    }
    Ok(())
}

/// `append8Digits(int m)`：8 位数字左到右写入
fn _append8_digits(d: &FloatToDecimal, m: i32) -> Result<()> {
    let mut y = _y(m);
    for _ in 0..8 {
        let t = 10i32.wrapping_mul(y);
        _append_digit(d, (t as u32 >> 28) as i32)?;
        y = t & MASK_28;
    }
    Ok(())
}

/// `exponent(int e)`：'E' + 有符号指数（float 至多两位）
fn _exponent(d: &FloatToDecimal, e: i32) -> Result<()> {
    let mut e = e;
    _append(d, b'E')?;
    if e < 0 {
        _append(d, b'-')?;
        e = -e;
    }
    if e < 10 {
        _append_digit(d, e)?;
        return Ok(());
    }
    // floor(e / 10) = floor(103 e / 2^10)
    let dd = ((e.wrapping_mul(103) as u32) >> 10) as i32;
    _append_digit(d, dd)?;
    _append_digit(d, e - 10 * dd)
}

/// `toChars1(h, l, e)`：0 < e <= 7，无前导零的定点格式
fn _to_chars1(d: &FloatToDecimal, h: i32, l: i32, e: i32) -> Result<i32> {
    _append_digit(d, h)?;
    let mut y = _y(l);
    let mut t: i32;
    let mut i = 1;
    while i < e {
        t = 10i32.wrapping_mul(y);
        _append_digit(d, (t as u32 >> 28) as i32)?;
        y = t & MASK_28;
        i += 1;
    }
    _append(d, b'.')?;
    while i <= 8 {
        t = 10i32.wrapping_mul(y);
        _append_digit(d, (t as u32 >> 28) as i32)?;
        y = t & MASK_28;
        i += 1;
    }
    _remove_trailing_zeroes(d)?;
    Ok(NON_SPECIAL)
}

/// `toChars2(h, l, e)`：-3 < e <= 0，带前导零的定点格式
fn _to_chars2(d: &FloatToDecimal, h: i32, l: i32, mut e: i32) -> Result<i32> {
    _append_digit(d, 0)?;
    _append(d, b'.')?;
    while e < 0 {
        _append_digit(d, 0)?;
        e += 1;
    }
    _append_digit(d, h)?;
    _append8_digits(d, l)?;
    _remove_trailing_zeroes(d)?;
    Ok(NON_SPECIAL)
}

/// `toChars3(h, l, e)`：e > 7 或 e <= -3，计算机科学计数法
fn _to_chars3(d: &FloatToDecimal, h: i32, l: i32, e: i32) -> Result<i32> {
    _append_digit(d, h)?;
    _append(d, b'.')?;
    _append8_digits(d, l)?;
    _remove_trailing_zeroes(d)?;
    _exponent(d, e - 1)?;
    Ok(NON_SPECIAL)
}

/// `toChars(int f, int e)`：格式化 f·10^e
fn _to_chars(d: &FloatToDecimal, f: i32, e: i32) -> Result<i32> {
    let mut f = f;
    let mut e = e;
    // 10^(len-1) <= f < 10^len
    let mut len = _flog10pow2(32 - (f as u32).leading_zeros() as i32);
    if f as i64 >= _pow10(len) {
        len += 1;
    }
    // 变换到 10^(H-1) <= f < 10^H，fp·10^ep = 0.f·10^e
    f = f.wrapping_mul(_pow10(H - len) as i32);
    e += len;
    // H = 9 位拆 h(1) + l(8)
    let h = ((f as i64).wrapping_mul(1_441_151_881i64) as u64 >> 57) as i32;
    let l = f - 100_000_000 * h;

    if 0 < e && e <= 7 {
        return _to_chars1(d, h, l, e);
    }
    if -3 < e && e <= 0 {
        return _to_chars2(d, h, l, e);
    }
    _to_chars3(d, h, l, e)
}

/// `rop(long g, long cp)`：rop(cp·g·2^(-95))（[1] 附录，float 缩放 2^95）
fn _rop(g: i64, cp: i64) -> i32 {
    let x1 = _multiply_high(g, cp);
    let vbp = (x1 as u64) >> 31;
    let carry = (((x1 & MASK_32) as u64).wrapping_add(MASK_32 as u64)) >> 32;
    (vbp | carry) as u32 as i32
}

/// `toDecimal(int q, int c, int dk)`：c·2^q 的最短十进制（int 域版）
fn _to_decimal_q_c_dk(d: &FloatToDecimal, q: i32, c: i32, dk: i32) -> Result<i32> {
    let out = c & 0x1;
    let cb = (c << 2) as i64;
    let cbr = cb + 2;
    let cbl: i64;
    let k: i32;
    if c != C_MIN || q == Q_MIN {
        // 常规间距
        cbl = cb - 2;
        k = _flog10pow2(q);
    } else {
        // 非常规间距
        cbl = cb - 1;
        k = _flog10_three_quarters_pow2(q);
    }
    let h = q + _flog2pow10(-k) + 33;

    let g = _g1(k) + 1;

    let vb = _rop(g, cb.wrapping_shl(h as u32));
    let vbl = _rop(g, cbl.wrapping_shl(h as u32));
    let vbr = _rop(g, cbr.wrapping_shl(h as u32));

    let s = vb >> 2;
    if s >= 100 {
        // s' = floor(s/10) = floor(s·1717986919 / 2^34)
        let sp10 = 10i32.wrapping_mul(((s as i64).wrapping_mul(1_717_986_919i64) as u64 >> 34) as i32);
        let tp10 = sp10 + 10;
        let upin = vbl + out <= sp10 << 2;
        let wpin = (tp10 << 2) + out <= vbr;
        if upin != wpin {
            return _to_chars(d, if upin { sp10 } else { tp10 }, k);
        }
    }
    // u = s·10^k 与 w = t·10^k 恰一入 Rv
    let t = s + 1;
    let uin = vbl + out <= s << 2;
    let win = (t << 2) + out <= vbr;
    if uin != win {
        return _to_chars(d, if uin { s } else { t }, k + dk);
    }
    // 两者皆入 Rv：取更近 v 者（平局取偶）
    let cmp = vb - (s + (t << 1));
    _to_chars(d, if cmp < 0 || (cmp == 0 && (s & 0x1) == 0) { s } else { t },
              k + dk)
}

/// `toDecimal(float v)`：入口分流
fn _to_decimal(d: &FloatToDecimal, v: f32) -> Result<i32> {
    let bits = v.to_bits() as i32;
    let t = bits & T_MASK;
    let bq = ((bits as u32 >> (P - 1) as u32) as i32) & BQ_MASK;
    if bq < BQ_MASK {
        d.__set_index(-1);
        if bits < 0 {
            _append(d, b'-')?;
        }
        if bq != 0 {
            // 规格化值；快速路径（[1] §8.3）：mq = -q
            let mq = -Q_MIN + 1 - bq;
            let c = C_MIN | t;
            if 0 < mq && mq < P {
                let f = c >> mq;
                if f << mq == c {
                    return _to_chars(d, f, 0);
                }
            }
            return _to_decimal_q_c_dk(d, -mq, c, 0);
        }
        if t != 0 {
            // 次规格化值
            return if t < C_TINY {
                _to_decimal_q_c_dk(d, Q_MIN, 10 * t, -1)
            } else {
                _to_decimal_q_c_dk(d, Q_MIN, t, 0)
            };
        }
        return Ok(if bits == 0 {
            PLUS_ZERO
        } else {
            MINUS_ZERO
        });
    }
    if t != 0 {
        return Ok(NAN);
    }
    Ok(if bits > 0 { PLUS_INF } else { MINUS_INF })
}

/// `charsToString()`：bytes[0..=index] 以 Latin1 紧凑字符串构造（JDK 走废弃的
/// `new String(bytes, 0, 0, index+1)`，即逐字节为 char）
fn _chars_to_string(d: &FloatToDecimal) -> Result<String> {
    let n = d.__get_index() + 1;
    let mut chars = Vec::with_capacity(n as usize);
    for i in 0..n {
        chars.push(d.__get_bytes().get(i)? as u8 as char);
    }
    let mut inst = String::default();
    inst._init_not_null();
    inst.__set_value(JArray::from(
        chars.into_iter().map(|c| c as u8 as i8).collect::<Vec<i8>>()));
    inst.__set_coder(0i8);
    Ok(inst)
}

fn _new_instance() -> Result<FloatToDecimal> {
    let mut inst = FloatToDecimal::default();
    inst._init_not_null();
    inst.__set_bytes(JArray::new(MAX_CHARS));
    Ok(inst)
}

impl FloatToDecimal {
    /// `toString(float v)`：`Float.toString` 的底层例程
    #[jvm_boundary]
    pub fn toString(v: f32) -> Result<String> {
        let d = _new_instance()?;
        match _to_decimal(&d, v)? {
            NON_SPECIAL => _chars_to_string(&d),
            PLUS_ZERO => Ok(String::from("0.0")),
            MINUS_ZERO => Ok(String::from("-0.0")),
            PLUS_INF => Ok(String::from("Infinity")),
            MINUS_INF => Ok(String::from("-Infinity")),
            _ => Ok(String::from("NaN")),
        }
    }

    /// `appendTo(float v, Appendable app)`：`AbstractStringBuilder.append(float)`
    /// 的底层例程。NON_SPECIAL 分支 JDK 先 `instanceof StringBuilder`/`StringBuffer`
    /// 走 `append(char[])` 快路径，否则逐 char `append(c)`——三条路径可观察行为
    /// 一致，统一经 Appendable 接口分派（与 double 版同约定）。
    #[jvm_boundary]
    pub fn appendTo(v: f32, arg1: Object) -> Result<Object> {
        let d = _new_instance()?;
        let special: Option<&str> = match _to_decimal(&d, v)? {
            PLUS_ZERO => Some("0.0"),
            MINUS_ZERO => Some("-0.0"),
            PLUS_INF => Some("Infinity"),
            MINUS_INF => Some("-Infinity"),
            NAN => Some("NaN"),
            _ => None,
        };
        if let Some(s) = special {
            Into::<Appendable>::into(Clone::clone(&arg1))
                .append_seq(Object::from(String::from(s)))?;
            return Ok(arg1);
        }
        let bytes = d.__get_bytes();
        let n = d.__get_index() + 1;
        let mut app = Into::<Appendable>::into(Clone::clone(&arg1));
        for i in 0..n {
            // (char) bytes[i]：byte 符号扩展到 int 再截位到 char
            let c = bytes.get(i)? as i32 as u16;
            app = Into::<Appendable>::into(app.append_c(c)?);
        }
        Ok(arg1)
    }
}

/// MathUtils.g1 的 float 区段表：k ∈ [K_MIN=-45, K_MAX=31]，从 MathUtils
/// 全表（double_to_decimal_impl.rs 的 G，K_MIN=-324 起）提取。每行 (g1, g0)
/// 与 MathUtils 的交错布局一致，float 只消费 g1（偶槽）。
fn _g1(k: i32) -> i64 {
    G[(((k - K_MIN) as usize) << 1)]
}

/// 10^-k = β·2^r 的 g = floor(β)+1 拆半表（g1 偶槽 / g0 奇槽），k ∈ [-45, 31]。
#[rustfmt::skip]
static G: [i64; 154] = [
    0x59AEDFC10D7279C5, 0x7768A00B22A00001, // -45
    0x47BF19673DF52E37, 0x79208008E8800001, // -44
    0x72CB5BD86321E38C, 0x5B67334174000001, // -43
    0x5BD5E313828182D6, 0x7C528F6790000001, // -42
    0x4977E8DC68679BDF, 0x16A872B940000001, // -41
    0x758CA7C70D7292FE, 0x5773EAC200000001, // -40
    0x5E0A1FD271287598, 0x45F6556800000001, // -39
    0x4B3B4CA85A86C47A, 0x04C5112000000001, // -38
    0x785EE10D5DA46D90, 0x07A1B50000000001, // -37
    0x604BE73DE4838AD9, 0x52E7C40000000001, // -36
    0x4D0985CB1D3608AE, 0x0F1FD00000000001, // -35
    0x7B426FAB61F00DE3, 0x31CC800000000001, // -34
    0x629B8C891B267182, 0x5B0A000000000001, // -33
    0x4EE2D6D415B85ACE, 0x7C08000000000001, // -32
    0x7E37BE2022C0914B, 0x1340000000000001, // -31
    0x64F964E68233A76F, 0x2900000000000001, // -30
    0x50C783EB9B5C85F2, 0x5400000000000001, // -29
    0x409F9CBC7C4A04C2, 0x1000000000000001, // -28
    0x6765C793FA10079D, 0x0000000000000001, // -27
    0x52B7D2DCC80CD2E4, 0x0000000000000001, // -26
    0x422CA8B0A00A4250, 0x0000000000000001, // -25
    0x69E10DE76676D080, 0x0000000000000001, // -24
    0x54B40B1F852BDA00, 0x0000000000000001, // -23
    0x43C33C1937564800, 0x0000000000000001, // -22
    0x6C6B935B8BBD4000, 0x0000000000000001, // -21
    0x56BC75E2D6310000, 0x0000000000000001, // -20
    0x4563918244F40000, 0x0000000000000001, // -19
    0x6F05B59D3B200000, 0x0000000000000001, // -18
    0x58D15E1762800000, 0x0000000000000001, // -17
    0x470DE4DF82000000, 0x0000000000000001, // -16
    0x71AFD498D0000000, 0x0000000000000001, // -15
    0x5AF3107A40000000, 0x0000000000000001, // -14
    0x48C2739500000000, 0x0000000000000001, // -13
    0x746A528800000000, 0x0000000000000001, // -12
    0x5D21DBA000000000, 0x0000000000000001, // -11
    0x4A817C8000000000, 0x0000000000000001, // -10
    0x7735940000000000, 0x0000000000000001, // -9
    0x5F5E100000000000, 0x0000000000000001, // -8
    0x4C4B400000000000, 0x0000000000000001, // -7
    0x7A12000000000000, 0x0000000000000001, // -6
    0x61A8000000000000, 0x0000000000000001, // -5
    0x4E20000000000000, 0x0000000000000001, // -4
    0x7D00000000000000, 0x0000000000000001, // -3
    0x6400000000000000, 0x0000000000000001, // -2
    0x5000000000000000, 0x0000000000000001, // -1
    0x4000000000000000, 0x0000000000000001, // 0
    0x6666666666666666, 0x3333333333333334, // 1
    0x51EB851EB851EB85, 0x0F5C28F5C28F5C29, // 2
    0x4189374BC6A7EF9D, 0x5916872B020C49BB, // 3
    0x68DB8BAC710CB295, 0x74F0D844D013A92B, // 4
    0x53E2D6238DA3C211, 0x43F3E0370CDC8755, // 5
    0x431BDE82D7B634DA, 0x698FE69270B06C44, // 6
    0x6B5FCA6AF2BD215E, 0x0F4CA41D811A46D4, // 7
    0x55E63B88C230E77E, 0x3F70834ACDAE9F10, // 8
    0x44B82FA09B5A52CB, 0x4C5A02A23E254C0D, // 9
    0x6DF37F675EF6EADF, 0x2D5CD10396A21347, // 10
    0x57F5FF85E592557F, 0x3DE3DA69454E75D3, // 11
    0x465E6604B7A84465, 0x7E4FE1EDD10B9175, // 12
    0x709709A125DA0709, 0x4A19697C81AC1BEF, // 13
    0x5A126E1A84AE6C07, 0x54E1213067BCE326, // 14
    0x480EBE7B9D58566C, 0x43E74DC052FD8285, // 15
    0x734ACA5F6226F0AD, 0x530BAF9A1E626A6D, // 16
    0x5C3BD5191B525A24, 0x426FBFAE7EB521F1, // 17
    0x49C97747490EAE83, 0x4EBFCC8B9890E7F4, // 18
    0x760F253EDB4AB0D2, 0x4ACC7A78F41B0CBA, // 19
    0x5E72843249088D75, 0x223D2EC729AF3D62, // 20
    0x4B8ED0283A6D3DF7, 0x34FDBF05BAF29781, // 21
    0x78E480405D7B9658, 0x54C931A2C4B758CF, // 22
    0x60B6CD004AC94513, 0x5D6DC14F03C5E0A5, // 23
    0x4D5F0A66A23A9DA9, 0x31249AA59C9E4D51, // 24
    0x7BCB43D769F762A8, 0x4EA0F76F60FD4882, // 25
    0x63090312BB2C4EED, 0x254D92BF80CAA068, // 26
    0x4F3A68DBC8F03F24, 0x1DD7A89933D54D20, // 27
    0x7EC3DAF941806506, 0x62F2A75B86221500, // 28
    0x65697BFA9ACD1D9F, 0x025BB91604E810CD, // 29
    0x51212FFBAF0A7E18, 0x684960DE6A5340A4, // 30
    0x40E7599625A1FE7A, 0x203AB3E521DC33B6, // 31
];
