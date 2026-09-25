#![allow(unused_imports)]
pub mod annotation_meta;
pub mod array;
pub mod data_bundles;
pub mod reflect_dispatch;
pub mod error;
pub mod java;
pub mod jdk;
pub mod jdk_resources;
pub mod monitor;
pub mod sun;

pub use array::JArray;
pub use error::{JvmError, Result};
pub use java::lang::Object;

/// Java 浮点十进制表示（`Double.toString` / `Float.toString` 语义）的唯一实现，
/// 三处入口共用：字符串拼接（codegen 拼接发射 `java_fmt_*`）、装箱值
/// `toString`（object.rs `impl_vtable_primitive!`）、`Double/Float.toString`
///（double_impl.rs 转发）。
///
/// 规则（JLS / Double.toString）：取**最短往返**十进制数字串，再按数量级排版
///   - `1e-3 ≤ |v| < 1e7`：普通小数，至少有 1 位小数（`1.0` / `0.001` / `1234567.0`）
///   - 其余：科学计数法 `d.dddE±n`（E 后不带 `+`、不补零）
///
/// Rust 的 `{:e}` 同样是"最短往返"表示，直接取其尾数数字与指数重排；两处与
/// Rust 的差异在下方逐点修正（科学计数法至少 2 位有效数字、平局取偶数末位）。

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
/// 语料 JDK 的特性版本（21 / 25 …）：生成侧按本次转译使用的 JDK 写入
/// scratch 的 jdk_feature.txt，build.rs 转为编译期环境变量。手写边界类中
/// **随 JDK 版本变化的数据**（非签名——签名差异由 core_ 适配与模型缺席补发承担）
/// 按此选择；未提供时按项目默认语料 21。
pub fn jdk_feature() -> u32 {
    option_env!("JAVA_RTA_JDK_FEATURE")
        .and_then(|v| v.parse().ok())
        .unwrap_or(21)
}

pub fn java_fmt_f64(v: f64) -> String {
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

/// Java `Float.toString(float)` 的输出格式（规则同 Double，数字串按 f32 精度）。
pub fn java_fmt_f32(v: f32) -> String {
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

/// 整数除法/取余（JVMS §6.5 idiv / irem / ldiv / lrem）：
/// 除数为 0 抛 `ArithmeticException("/ by zero")`；`MIN / -1` 按二进制补码回绕。
pub fn idiv(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn irem(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}
pub fn ldiv(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn lrem(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}

/// JVM null 检查：ifnull/ifnonnull 字节码翻译辅助。
/// Object 内部的 vtable `is_jvm_null()` 是唯一判定（S-3.1 后统一）：
/// `Object::default()`（内部 vtable = `()`）、null 包装类值（`_jvm_null` 未清零）、
/// null 数组（`JArray` 的 `Repr::Null`）都以同一 vtable 钩子呈现 null 语义。
#[inline(always)]
pub fn _is_jnull<T: 'static>(val: &T) -> bool {
    if let Some(obj) = (val as &dyn std::any::Any).downcast_ref::<Object>() {
        obj.0.is_jvm_null()
    } else {
        false
    }
}


/// S-17: typeSwitch String 常量标签判定（SwitchBootstraps 语义 `label.equals(selector)`：
/// selector 非 String 恒 false）。经运行时类判定 + `__obj_str` 内容比较，不依赖闭包内
/// 是否生成 java/lang/String 类型定义（K-2 规则：运行时手写层不引用转译类）。
#[inline]
pub fn _ts_str_label_eq(label: &str, sel: &Object) -> bool {
    sel.is_instance_of("java/lang/String") && sel.0.__obj_str() == label
}

/// S-17: typeSwitch Integer 常量标签判定（selector instanceof Integer 且值相等；
/// Integer 的 `__obj_str` 为十进制表示，与 i32 的 Display 逐字符一致）。
#[inline]
pub fn _ts_int_label_eq(label: i32, sel: &Object) -> bool {
    sel.is_instance_of("java/lang/Integer") && sel.0.__obj_str() == std::format!("{}", label)
}


/// MutexHolder：包装 parking_lot::ReentrantMutex，为 InternalLock 等需要 PartialEq 的结构体使用
#[derive(Clone)]
pub struct MutexHolder(pub std::sync::Arc<parking_lot::ReentrantMutex<()>>);
impl Default for MutexHolder {
    fn default() -> Self { Self(std::sync::Arc::new(parking_lot::ReentrantMutex::new(()))) }
}
impl PartialEq for MutexHolder {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

// ── 常量目录（JDK `Class.enumConstantDirectory` 的数据面）────────────────────
//
// `java_class!` 宏在类初始化（JVMS §5.5）完成后，为声明了「自身类型 static 字段」
// 的类登记（常量名 → 取值闭包）——Java 枚举常量即该形态（`static final Day MONDAY`，
// 值由 `<clinit>` 写入线程局部存储）。查询侧（手写 `Enum.valueOf`）按类对象的
// binary name（点分）取常量。登记只看结构形态，不感知枚举语义；非枚举类的
// 同形态 static 字段一并登记，无副作用（目录仅被常量名查找消费）。

type ConstantGetter = std::rc::Rc<dyn Fn() -> Result<Object>>;

std::thread_local! {
    static CONSTANT_DIRECTORY: std::cell::RefCell<
        std::collections::HashMap<std::string::String, Vec<(std::string::String, ConstantGetter)>>
    > = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 登记一个类的常量目录项。`binary_name` 为 JVM binary name（斜线 / $ 形态），
/// 内部归一为点分形态（与 `Class` 对象承载的名字一致）。
pub fn register_constant_directory(binary_name: &str, entries: Vec<(std::string::String, ConstantGetter)>) {
    CONSTANT_DIRECTORY.with(|dir| {
        dir.borrow_mut().insert(binary_name.replace('/', "."), entries);
    });
}

/// 按类名 + 常量名取常量。类未登记（无该形态 static 字段 / 尚未初始化）或
/// 常量不存在 → None；常量取值闭包失败（如 erroneous 类初始化后置访问）→ None。
pub fn lookup_constant(binary_name: &str, constant_name: &str) -> Option<Object> {
    CONSTANT_DIRECTORY.with(|dir| {
        let dir = dir.borrow();
        dir.get(binary_name)?
            .iter()
            .find(|(name, _)| name == constant_name)
            .and_then(|(_, get)| get().ok())
    })
}

/// 按类名取常量宇宙（JDK `JavaLangAccess.getEnumConstantsShared` 的数据面）：
/// 返回该类登记的全部常量，登记序 == 字段声明序（枚举常量即 ordinal 序）。
/// 类未登记 → None；任一常量取值失败 → None。取值闭包经访问器触发类初始化，
/// 已初始化类（枚举宇宙的常态消费方）直接命中。
pub fn constant_directory_universe(binary_name: &str) -> Option<Vec<Object>> {
    CONSTANT_DIRECTORY.with(|dir| {
        let dir = dir.borrow();
        dir.get(binary_name)?
            .iter()
            .map(|(_, get)| get().ok())
            .collect::<Option<Vec<Object>>>()
    })
}

// ── 类初始化钩子（JVM 反射路径强制初始化的数据面）──────────────────────────
//
// JVM 语义：`Class` 字面量（ldc）不触发初始化（JVMS §5.5 无 passivity 例外），
// 但反射式消费方（`getEnumConstantsShared`、`Class.getEnumConstants`、
// `Enum.valueOf(Class, name)` 等）会强制目标类初始化后再读常量。名字 token
// `Class::for_class` 与用户类的 `__class_init` 之间没有通道——生成项目在 main
// 启动时按语料登记钩子（枚举形态类，与常量目录同一结构谓词），运行时按名代调。

pub type ClassInitHook = std::rc::Rc<dyn Fn() -> Result<()>>;

std::thread_local! {
    static CLASS_INIT_HOOKS: std::cell::RefCell<
        std::collections::HashMap<std::string::String, ClassInitHook>
    > = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 生成项目 main 启动时登记类初始化钩子。`binary_name` 归一规则与常量目录一致。
pub fn register_class_init_hooks(hooks: &[(&str, ClassInitHook)]) {
    CLASS_INIT_HOOKS.with(|h| {
        let mut h = h.borrow_mut();
        for (name, hook) in hooks {
            h.insert(name.replace('/', "."), Clone::clone(hook));
        }
    });
}

/// 按名强制类初始化（JVM 反射路径语义）。未登记（非语料类 / 手写边界类）→
/// no-op；已初始化 → `__class_init` 状态机立即返回；初始化抛错 → 原样传播
/// （erroneous 状态与异常包装由状态机承载）。
pub fn ensure_class_initialized(binary_name: &str) -> Result<()> {
    let hook = CLASS_INIT_HOOKS.with(|h| {
        let h = h.borrow();
        h.get(binary_name.replace('/', ".").as_str()).map(Clone::clone)
    });
    match hook {
        Some(f) => f(),
        None => Ok(()),
    }
}

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::array::JArray;
    pub use super::error::{JvmError, Result};
    pub use super::java::lang::Object;
    pub use super::java::lang::ObjectVTable;
    pub use super::java::lang::Object__clone_base;
    pub use super::java::lang::String;
    pub use super::_is_jnull;
    pub use super::_ts_str_label_eq;
    pub use super::_ts_int_label_eq;
    pub use super::{idiv, irem, ldiv, lrem};

    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use super::{register_constant_directory, lookup_constant, constant_directory_universe};
    pub use super::monitor::{MonitorGuard, class_monitor};
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use super::MutexHolder;
    pub use java_rta_macros::{jvm_native, jvm_boundary, jvm_ext};
    pub use java_rta_macros::java_try;
}
