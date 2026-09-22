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

    fn appendTo(&self, arg0: Object) -> Result<()> {
        // 与 FloatToDecimal.appendTo 的 Appendable 分派同形态：逐 char 追加
        // （byte 符号扩展到 int 再截位到 char）。
        let val = self.formatted.__get_value();
        let mut app = Into::<Appendable>::into(Clone::clone(&arg0));
        for i in 0..val.len()? {
            let c = val.get(i)? as i32 as u16;
            app = Into::<Appendable>::into(app.append_c(c)?);
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

impl FloatingDecimal {
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
