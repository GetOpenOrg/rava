use crate::prelude::*;
use super::*;

impl Math {
    #[jvm_native] pub fn sin(a: f64) -> Result<f64> { Ok(a.sin()) }
    #[jvm_native] pub fn cos(a: f64) -> Result<f64> { Ok(a.cos()) }
    #[jvm_native] pub fn tan(a: f64) -> Result<f64> { Ok(a.tan()) }
    #[jvm_native] pub fn asin(a: f64) -> Result<f64> { Ok(a.asin()) }
    #[jvm_native] pub fn acos(a: f64) -> Result<f64> { Ok(a.acos()) }
    #[jvm_native] pub fn atan(a: f64) -> Result<f64> { Ok(a.atan()) }
    #[jvm_native] pub fn atan2(y: f64, x: f64) -> Result<f64> { Ok(y.atan2(x)) }
    #[jvm_native] pub fn exp(a: f64) -> Result<f64> { Ok(a.exp()) }
    #[jvm_native] pub fn log(a: f64) -> Result<f64> { Ok(a.ln()) }
    #[jvm_native] pub fn log10(a: f64) -> Result<f64> { Ok(a.log10()) }
    #[jvm_native] pub fn sqrt(a: f64) -> Result<f64> { Ok(a.sqrt()) }
    #[jvm_native] pub fn cbrt(a: f64) -> Result<f64> { Ok(a.cbrt()) }
    #[jvm_native] pub fn IEEEremainder(f1: f64, f2: f64) -> Result<f64> { Ok(f1 - f2 * (f1 / f2).round()) }
    #[jvm_native] pub fn ceil(a: f64) -> Result<f64> { Ok(a.ceil()) }
    #[jvm_native] pub fn floor(a: f64) -> Result<f64> { Ok(a.floor()) }
    // rint：不手写。JDK 21 起 StrictMath.rint 为纯 Java 实现（(2^52+|a|)-2^52 的
    // HALF_EVEN 舍入），Math.rint 转译后直接落到该字节码链，逐位与 JVM 一致；
    // 此前手写的 a.round() 是 half-away-from-zero，曾致 rint(2.5)=3.0 规格破坏（S-19）。
    #[jvm_native] pub fn pow(a: f64, b: f64) -> Result<f64> { Ok(a.powf(b)) }
    #[jvm_native] pub fn round__f(a: f32) -> Result<i32> { Ok(a.round() as i32) }
    #[jvm_native] pub fn round__d(a: f64) -> Result<i64> { Ok(a.round() as i64) }
    #[jvm_native] pub fn random() -> Result<f64> { Ok(0.5f64) }
    #[jvm_native] pub fn sinh(x: f64) -> Result<f64> { Ok(x.sinh()) }
    #[jvm_native] pub fn cosh(x: f64) -> Result<f64> { Ok(x.cosh()) }
    #[jvm_native] pub fn tanh(x: f64) -> Result<f64> { Ok(x.tanh()) }
    #[jvm_native] pub fn hypot(x: f64, y: f64) -> Result<f64> { Ok(x.hypot(y)) }
    #[jvm_native] pub fn expm1(x: f64) -> Result<f64> { Ok(x.exp_m1()) }
    #[jvm_native] pub fn log1p(x: f64) -> Result<f64> { Ok(x.ln_1p()) }
    #[jvm_native] pub fn toRadians(angdeg: f64) -> Result<f64> { Ok(angdeg.to_radians()) }
    #[jvm_native] pub fn toDegrees(angrad: f64) -> Result<f64> { Ok(angrad.to_degrees()) }
    #[jvm_native] pub fn copySign__d_d(magnitude: f64, sign: f64) -> Result<f64> { Ok(magnitude.copysign(sign)) }
    #[jvm_native] pub fn copySign__f_f(magnitude: f32, sign: f32) -> Result<f32> { Ok(magnitude.copysign(sign)) }
    #[jvm_native] pub fn signum__d(d: f64) -> Result<f64> { Ok(d.signum()) }
    #[jvm_native] pub fn signum__f(f: f32) -> Result<f32> { Ok(f.signum()) }
    #[jvm_native] pub fn nextAfter__d_d(start: f64, direction: f64) -> Result<f64> {
        Ok(if direction > start { f64::from_bits(start.to_bits() + 1) }
           else if direction < start { f64::from_bits(start.to_bits() - 1) }
           else { direction })
    }
    #[jvm_native] pub fn nextAfter__f_d(start: f32, direction: f64) -> Result<f32> {
        Ok(if direction > start as f64 { f32::from_bits(start.to_bits() + 1) }
           else if direction < start as f64 { f32::from_bits(start.to_bits() - 1) }
           else { start })
    }
    #[jvm_native] pub fn nextUp__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() + 1)) }
    #[jvm_native] pub fn nextUp__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() + 1)) }
    #[jvm_native] pub fn nextDown__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() - 1)) }
    #[jvm_native] pub fn nextDown__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() - 1)) }
    #[jvm_native] pub fn scalb__d_i(d: f64, scale_factor: i32) -> Result<f64> {
        Ok(d * (2f64.powi(scale_factor)))
    }
    #[jvm_native] pub fn scalb__f_i(f: f32, scale_factor: i32) -> Result<f32> {
        Ok(f * (2f32.powi(scale_factor)))
    }
    #[jvm_native] pub fn getExponent__d(d: f64) -> Result<i32> {
        Ok(((d.to_bits() >> 52) as i32 & 0x7FF) - 1023)
    }
    #[jvm_native] pub fn getExponent__f(f: f32) -> Result<i32> {
        Ok(((f.to_bits() >> 23) as i32 & 0xFF) - 127)
    }
    #[jvm_native] pub fn fma__d_d_d(a: f64, b: f64, c: f64) -> Result<f64> { Ok(a.mul_add(b, c)) }
    #[jvm_native] pub fn fma__f_f_f(a: f32, b: f32, c: f32) -> Result<f32> { Ok(a.mul_add(b, c)) }
}
