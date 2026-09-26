//! `java/lang/StrictMath` 的 VM 内建函数（intrinsic，准入清单 `intrinsics.txt`）。
//!
//! 只收录规范给出唯一精确结果的 @IntrinsicCandidate 方法：原生运算与 Java 体逐位相同，
//! 与 HotSpot 以硬件指令内建同一处置。其余方法全部按字节码翻译（FdLibm 链）。

use crate::prelude::*;
use super::*;

impl StrictMath {
    /// `sqrt(double)`：IEEE 754 squareRoot（正确舍入）。`f64::sqrt` 同为正确舍入，
    /// NaN / ±0 / +∞ / 负数的特殊值与规范一致。
    #[jvm_native]
    pub fn sqrt(a: f64) -> Result<f64> {
        Ok(a.sqrt())
    }
}
