//! `jdk/internal/math/FloatToDecimal` 手写实现（内部边界类；仅当
//! `float_to_decimal.rs` 进入闭包生成时编译，见 K-2 规则）。
//!
//! 按 JDK 25 形态书写（L1 兼容改写，2026-09-23 jdk25 语料适配，与
//! double_to_decimal_impl.rs 同构）：私有算法全部改为 **byte[]/index 显式
//! 传参**（JDK 22 引入 `ToDecimal` 抽象基类后的无实例字段模型），不依赖
//! 宿主类任何字段/访问器——JDK 21 与 JDK 25 两种生成形态下均编译。
//! javap -c 对照 `ToDecimal.putChar/putDigit/put8Digits/y/
//! removeTrailingZeroes/special` 与 `FloatToDecimal.toDecimal(×2)/rop/
//! toChars/toChars1..3/exponent` 全量方法体：float 的 shortest round-trip
//! 十进制表示（Schubfach，[1] §9 n=9）。与 double 版的结构对应差异：
//! - P=24/W=8：q/c/cb/vb 全 int 域（double 版为 long 域）；
//! - `g = g1(k) + 1` 单 long（double 版 g = g1·2^63 + g0 双半）；
//! - k ∈ [K_MIN=-45, K_MAX=31]，g 表取 MathUtils 全表的该区段；
//! - rop 以 2^95 为缩放（x1 >>> 31 | 进位位），double 版为 2^127；
//! - 特殊值编码左移 8 位（`ToDecimal.PLUS_ZERO=256 .. NAN=1280`），
//!   非特殊返回写入长度（`toDecimal` 结果 `- start`）。
//!
//! Java int 运算的溢出回绕与 `>>>` 逻辑右移在 Rust 显式建模（wrapping 系 +
//! u32/u64 中转）。入口仅 `toString`（`Float.toString` 链）与 `appendTo`
//! （JDK 21 语料 `ASB.append(float)` 消费；JDK 25 该方法已删，仅 dead_code）。
//! fd 参数恒为 null 路径（`split` 留给生成侧存根）。
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
const MAX_CHARS: i32 = H + 6;

// ── ToDecimal 常量（JDK 25 形态：特殊值编码左移 8 位） ───────────────
const PLUS_ZERO: i32 = 256;
const MINUS_ZERO: i32 = 512;
const PLUS_INF: i32 = 768;
const MINUS_INF: i32 = 1024;
const NAN_: i32 = 1280;

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

// ── 私有算法（JDK 25 形态：bytes/index 显式传参，LATIN1 通道；写指针为
//    预递增约定——putChar 写 bytes[index] 返回 index+1） ──────────────

/// `ToDecimal.putChar(byte[], int, int)`（LATIN1 分支）：`bytes[index] =
/// (byte) c`，返回 `index + 1`
fn _put_char(bytes: &JArray<i8>, index: i32, c: u8) -> Result<i32> {
    bytes.set(index, c as i8)?;
    Ok(index + 1)
}

/// `ToDecimal.putDigit(byte[], int, int)`：`'0' + d`
fn _put_digit(bytes: &JArray<i8>, index: i32, digit: i32) -> Result<i32> {
    _put_char(bytes, index, (b'0' as i32 + digit) as u8)
}

/// `y(int a)`（ToDecimal static）：floor((a + 1) 2^28 / 10^8) - 1（左到右
/// 逐位提取，[3] 算法 1）
fn _y(a: i32) -> i32 {
    (((_multiply_high(((a + 1) as i64) << 28, 193_428_131_138_340_668i64) as u64) >> 20) as i32) - 1
}

/// `ToDecimal.removeTrailingZeroes(byte[], int)`（LATIN1 分支）：去尾零，
/// 但保留 '.' 右侧那一个；返回新 index
fn _remove_trailing_zeroes(bytes: &JArray<i8>, index: i32) -> Result<i32> {
    let mut index = index;
    while bytes.get(index - 1)? == b'0' as i8 {
        index -= 1;
    }
    if bytes.get(index - 1)? == b'.' as i8 {
        index += 1;
    }
    Ok(index)
}

/// `ToDecimal.put8Digits(byte[], int, int)`（= put8DigitsLatin1）：8 位数字
/// 左到右写入，返回 `index + 8`
fn _put8_digits(bytes: &JArray<i8>, index: i32, m: i32) -> Result<i32> {
    let mut y = _y(m);
    for i in 0..8 {
        let t = 10i32.wrapping_mul(y);
        bytes.set(index + i, (b'0' as i32 + (t as u32 >> 28) as i32) as u8 as i8)?;
        y = t & MASK_28;
    }
    Ok(index + 8)
}

/// `exponent(byte[], int, int)`：'E' + 有符号指数（float 至多两位），
/// 返回新 index
fn _exponent(bytes: &JArray<i8>, index: i32, e: i32) -> Result<i32> {
    let mut index = _put_char(bytes, index, b'E')?;
    let mut e = e;
    if e < 0 {
        index = _put_char(bytes, index, b'-')?;
        e = -e;
    }
    if e < 10 {
        return _put_digit(bytes, index, e);
    }
    // floor(e / 10) = floor(103 e / 2^10)
    let dd = ((e.wrapping_mul(103) as u32) >> 10) as i32;
    index = _put_digit(bytes, index, dd)?;
    _put_digit(bytes, index, e - 10 * dd)
}

/// `toChars1(byte[], int, h, l, e)`：0 < e <= 7，无前导零的定点格式
fn _to_chars1(bytes: &JArray<i8>, index: i32, h: i32, l: i32, e: i32) -> Result<i32> {
    let mut index = _put_digit(bytes, index, h)?;
    let mut y = _y(l);
    let mut t: i32;
    let mut i = 1;
    while i < e {
        t = 10i32.wrapping_mul(y);
        index = _put_digit(bytes, index, (t as u32 >> 28) as i32)?;
        y = t & MASK_28;
        i += 1;
    }
    index = _put_char(bytes, index, b'.')?;
    while i <= 8 {
        t = 10i32.wrapping_mul(y);
        index = _put_digit(bytes, index, (t as u32 >> 28) as i32)?;
        y = t & MASK_28;
        i += 1;
    }
    _remove_trailing_zeroes(bytes, index)
}

/// `toChars2(byte[], int, h, l, e)`：-3 < e <= 0，带前导零的定点格式
fn _to_chars2(bytes: &JArray<i8>, index: i32, h: i32, l: i32, mut e: i32) -> Result<i32> {
    let mut index = _put_digit(bytes, index, 0)?;
    index = _put_char(bytes, index, b'.')?;
    while e < 0 {
        index = _put_digit(bytes, index, 0)?;
        e += 1;
    }
    index = _put_digit(bytes, index, h)?;
    index = _put8_digits(bytes, index, l)?;
    _remove_trailing_zeroes(bytes, index)
}

/// `toChars3(byte[], int, h, l, e)`：e > 7 或 e <= -3，计算机科学计数法
fn _to_chars3(bytes: &JArray<i8>, index: i32, h: i32, l: i32, e: i32) -> Result<i32> {
    let mut index = _put_digit(bytes, index, h)?;
    index = _put_char(bytes, index, b'.')?;
    index = _put8_digits(bytes, index, l)?;
    index = _remove_trailing_zeroes(bytes, index)?;
    _exponent(bytes, index, e - 1)
}

/// `toChars(byte[], int, int f, int e)`：格式化 f·10^e
fn _to_chars(bytes: &JArray<i8>, index: i32, f: i32, e: i32) -> Result<i32> {
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
        return _to_chars1(bytes, index, h, l, e);
    }
    if -3 < e && e <= 0 {
        return _to_chars2(bytes, index, h, l, e);
    }
    _to_chars3(bytes, index, h, l, e)
}

/// `rop(long g, long cp)`（static）：rop(cp·g·2^(-95))（[1] 附录，float
/// 缩放 2^95）
fn _rop(g: i64, cp: i64) -> i32 {
    let x1 = _multiply_high(g, cp);
    let vbp = (x1 as u64) >> 31;
    let carry = (((x1 & MASK_32) as u64).wrapping_add(MASK_32 as u64)) >> 32;
    (vbp | carry) as u32 as i32
}

/// `toDecimal(byte[], int, int q, int c, int dk)`：c·2^q 的最短十进制
/// （int 域版；返回写后 index）
fn _to_decimal_q_c_dk(bytes: &JArray<i8>, index: i32, q: i32, c: i32, dk: i32) -> Result<i32> {
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
            return _to_chars(bytes, index, if upin { sp10 } else { tp10 }, k);
        }
    }
    // u = s·10^k 与 w = t·10^k 恰一入 Rv
    let t = s + 1;
    let uin = vbl + out <= s << 2;
    let win = (t << 2) + out <= vbr;
    if uin != win {
        return _to_chars(bytes, index, if uin { s } else { t }, k + dk);
    }
    // 两者皆入 Rv：取更近 v 者（平局取偶）
    // JDK：vb - (s + t << 1)——Java 移位优先级低于加法，即 2(s+t)（s/t 中点），
    // 非先移 t（曾致 1/3f/16777216f 平局侧末位 +1）
    let cmp = vb - ((s + t) << 1);
    _to_chars(bytes, index, if cmp < 0 || (cmp == 0 && (s & 0x1) == 0) { s } else { t },
              k + dk)
}

/// `toDecimal(byte[], int, float)`：入口分流。非特殊返回写入长度
/// （`end - start`，与 `& 0xFF00` 判定配套：长度 < 256，特殊码 ≥ 256），
/// 特殊返回 `ToDecimal` 左移 8 位编码
fn _to_decimal(bytes: &JArray<i8>, index: i32, v: f32) -> Result<i32> {
    let bits = v.to_bits() as i32;
    let t = bits & T_MASK;
    let bq = ((bits as u32 >> (P - 1) as u32) as i32) & BQ_MASK;
    if bq < BQ_MASK {
        let start = index;
        let mut index = index;
        if bits < 0 {
            index = _put_char(bytes, index, b'-')?;
        }
        if bq != 0 {
            // 规格化值；快速路径（[1] §8.3）：mq = -q
            let mq = -Q_MIN + 1 - bq;
            let c = C_MIN | t;
            if 0 < mq && mq < P {
                let f = c >> mq;
                if f << mq == c {
                    return Ok(_to_chars(bytes, index, f, 0)? - start);
                }
            }
            return Ok(_to_decimal_q_c_dk(bytes, index, -mq, c, 0)? - start);
        }
        if t != 0 {
            // 次规格化值
            return Ok(if t < C_TINY {
                _to_decimal_q_c_dk(bytes, index, Q_MIN, 10 * t, -1)? - start
            } else {
                _to_decimal_q_c_dk(bytes, index, Q_MIN, t, 0)? - start
            });
        }
        return Ok(if bits == 0 {
            PLUS_ZERO
        } else {
            MINUS_ZERO
        });
    }
    if t != 0 {
        return Ok(NAN_);
    }
    Ok(if bits > 0 { PLUS_INF } else { MINUS_INF })
}

/// `ToDecimal.special(int)`（static）：特殊值 → 字符串（lookupswitch 的
/// default 分支是 NaN，覆盖 NAN=1280）
fn _special(m: i32) -> &'static str {
    match m {
        PLUS_ZERO => "0.0",
        MINUS_ZERO => "-0.0",
        PLUS_INF => "Infinity",
        MINUS_INF => "-Infinity",
        _ => "NaN",
    }
}

/// `new String(bytes, 0, n, ISO_8859_1.INSTANCE)`：bytes[0..n] 以 Latin1
/// 紧凑字符串构造（JDK 21 侧等价 `charsToString()` 的字符序列）
fn _chars_to_string(bytes: &JArray<i8>, n: i32) -> Result<String> {
    let mut chars = Vec::with_capacity(n as usize);
    for i in 0..n {
        chars.push(bytes.get(i)? as u8 as char);
    }
    let mut inst = String::default();
    inst._init_not_null();
    inst.__set_value(JArray::from(
        chars.into_iter().map(|c| c as u8 as i8).collect::<Vec<i8>>()));
    inst.__set_coder(0i8);
    Ok(inst)
}

/// JDK 25 `putDecimal` 的两个编码单例（`ToDecimal.latin1` 标志的承载）：
/// 内部边界类 <clinit> 不翻译，单例在此惰性构造——线程内唯一身份（JVM
/// static final 语义），latin1 判定按对象身份比对 LATIN1 单例。
thread_local! {
    static __ENCODERS: (FloatToDecimal, FloatToDecimal) = {
        let mut l = FloatToDecimal::default();
        l._init_not_null();
        let mut u = FloatToDecimal::default();
        u._init_not_null();
        (l, u)
    };
}

fn __is_latin1(this: &FloatToDecimal) -> bool {
    __ENCODERS.with(|(l, _)| {
        Object::from(Clone::clone(this)).0.__identity()
            == Object::from(Clone::clone(l)).0.__identity()
    })
}

/// UTF16 编码写一个 char（`StringUTF16.putChar` 同一字节序约定：
/// HI_BYTE_SHIFT = 大端 8 / 小端 0，与 Unsafe.isBigEndian 一致）。
fn __put_char_utf16(bytes: &JArray<i8>, index: i32, c: u16) -> Result<()> {
    let (hi, lo) = if cfg!(target_endian = "big") { (8, 0) } else { (0, 8) };
    bytes.set(index * 2, (c >> hi) as u8 as i8)?;
    bytes.set(index * 2 + 1, (c >> lo) as u8 as i8)?;
    Ok(())
}

impl FloatToDecimal {
    /// static `LATIN1` / `UTF16`：JDK 25 `AbstractStringBuilder.append(float)`
    /// 按 builder coder 取用的编码单例（JDK 21 模型无此二字段，伴生多出的静态
    /// 方法不参与其编译面）。
    pub fn LATIN1() -> Result<FloatToDecimal> {
        Ok(__ENCODERS.with(|(l, _)| Clone::clone(l)))
    }

    pub fn UTF16() -> Result<FloatToDecimal> {
        Ok(__ENCODERS.with(|(_, u)| Clone::clone(u)))
    }

    /// `putDecimal(byte[] str, int index, float v)`：把 v 的最短往返十进制
    /// 表示写入 builder 缓冲 `str` 的 `index` 处（UTF16 编码时 index 为 char
    /// 下标），返回写入后的下标。按字节码：非特殊值写入 `m & 0xFF` 个字符；
    /// 特殊值（±0 / ±Infinity / NaN，`m & 0xFF00`）经 putSpecial 写其文本。
    /// Latin1 直接在目标缓冲上运行算法；UTF16 先落 Latin1 临时缓冲再逐字符
    /// 按 2 字节写入（字符集全为 ASCII，两种写法的字符序列一致）。
    #[jvm_boundary]
    pub fn putDecimal(&self, str: JArray<i8>, index: i32, v: f32) -> Result<i32> {
        let latin1 = __is_latin1(self);
        let tmp = if latin1 { Clone::clone(&str) } else { JArray::new(MAX_CHARS) };
        let start = if latin1 { index } else { 0 };
        let m = _to_decimal(&tmp, start, v)?;
        let text: Vec<u16> = if m & 0xFF00 == 0 {
            if latin1 {
                return Ok(index + (m & 0xFF));
            }
            let mut cs = Vec::with_capacity((m & 0xFF) as usize);
            for i in 0..(m & 0xFF) {
                cs.push(tmp.get(i)? as u8 as u16);
            }
            cs
        } else {
            _special(m).encode_utf16().collect()
        };
        for (i, c) in text.iter().enumerate() {
            if latin1 {
                str.set(index + i as i32, *c as u8 as i8)?;
            } else {
                __put_char_utf16(&str, index + i as i32, *c)?;
            }
        }
        Ok(index + text.len() as i32)
    }

    /// `toString(float)`：`Float.toString` 的底层例程（JDK 25 形态：
    /// 自包含分配 MAX_CHARS 缓冲，`m & 0xFF00`（ldc 65280）分流特殊值）
    #[jvm_boundary]
    pub fn toString(v: f32) -> Result<String> {
        let bytes = JArray::new(MAX_CHARS);
        let m = _to_decimal(&bytes, 0, v)?;
        if m & 0xFF00 == 0 {
            _chars_to_string(&bytes, m & 0xFF)
        } else {
            Ok(String::from(_special(m)))
        }
    }

    /// `appendTo(float v, Appendable app)`：`AbstractStringBuilder.append(float)`
    /// 的底层例程（JDK 21 语料消费；JDK 25 已删除该方法，此处仅 dead_code）。
    /// NON_SPECIAL 分支 JDK 先 `instanceof StringBuilder`/`StringBuffer`
    /// 走 `append(char[])` 快路径，否则逐 char `append(c)`——三条路径可观察行为
    /// 一致，统一经 Appendable 接口分派（与 double 版同约定）。
    ///
    /// A-4 批次 6：形参/返回按接口载体形态书写（与 double 版同一约定，
    /// `append_seq` 的 CharSequence 实参按闭包形态推断定标）。
    #[jvm_boundary]
    pub fn appendTo(v: f32, arg1: Appendable) -> Result<Appendable> {
        let bytes = JArray::new(MAX_CHARS);
        let m = _to_decimal(&bytes, 0, v)?;
        if m & 0xFF00 != 0 {
            Clone::clone(&arg1)
                .append_seq(Into::into(Object::from(String::from(_special(m)))))?;
            return Ok(arg1);
        }
        let n = m & 0xFF;
        let mut app = Clone::clone(&arg1);
        for i in 0..n {
            // (char) bytes[i]：byte 符号扩展到 int 再截位到 char
            let c = bytes.get(i)? as i32 as u16;
            app = app.append_c(c)?;
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
