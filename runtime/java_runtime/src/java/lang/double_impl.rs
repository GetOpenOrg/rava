use crate::prelude::*;
use super::*;

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
    /// 实现在 `crate::java_fmt_f64`（字符串拼接 / 装箱 toString 同源，S-19 #4 统一）。
    #[jvm_native]
    pub fn toString_d(value: f64) -> Result<String> {
        Ok(String::from(crate::java_fmt_f64(value).as_str()))
    }

    /// `Double.toString()`：实例版本（装箱后 `println(obj)` 走的路径）。
    #[jvm_native]
    pub fn __impl_toString(&self) -> Result<String> {
        Ok(String::from(crate::java_fmt_f64(self.__get_value()).as_str()))
    }
}
