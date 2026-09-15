use java_runtime::prelude::*;
use super::*;

impl Math {
    pub fn sin(a: f64) -> Result<f64> { Ok(a.sin()) }
    pub fn cos(a: f64) -> Result<f64> { Ok(a.cos()) }
    pub fn tan(a: f64) -> Result<f64> { Ok(a.tan()) }
    pub fn asin(a: f64) -> Result<f64> { Ok(a.asin()) }
    pub fn acos(a: f64) -> Result<f64> { Ok(a.acos()) }
    pub fn atan(a: f64) -> Result<f64> { Ok(a.atan()) }
    pub fn atan2(y: f64, x: f64) -> Result<f64> { Ok(y.atan2(x)) }
    pub fn exp(a: f64) -> Result<f64> { Ok(a.exp()) }
    pub fn log(a: f64) -> Result<f64> { Ok(a.ln()) }
    pub fn log10(a: f64) -> Result<f64> { Ok(a.log10()) }
    pub fn sqrt(a: f64) -> Result<f64> { Ok(a.sqrt()) }
    pub fn cbrt(a: f64) -> Result<f64> { Ok(a.cbrt()) }
    pub fn IEEEremainder(f1: f64, f2: f64) -> Result<f64> { Ok(f1 - f2 * (f1 / f2).round()) }
    pub fn ceil(a: f64) -> Result<f64> { Ok(a.ceil()) }
    pub fn floor(a: f64) -> Result<f64> { Ok(a.floor()) }
    pub fn rint(a: f64) -> Result<f64> { Ok(a.round()) }
    pub fn pow(a: f64, b: f64) -> Result<f64> { Ok(a.powf(b)) }
    pub fn round__f(a: f32) -> Result<i32> { Ok(a.round() as i32) }
    pub fn round__d(a: f64) -> Result<i64> { Ok(a.round() as i64) }
    pub fn random() -> Result<f64> { Ok(0.5f64) }
    pub fn sinh(x: f64) -> Result<f64> { Ok(x.sinh()) }
    pub fn cosh(x: f64) -> Result<f64> { Ok(x.cosh()) }
    pub fn tanh(x: f64) -> Result<f64> { Ok(x.tanh()) }
    pub fn hypot(x: f64, y: f64) -> Result<f64> { Ok(x.hypot(y)) }
    pub fn expm1(x: f64) -> Result<f64> { Ok(x.exp_m1()) }
    pub fn log1p(x: f64) -> Result<f64> { Ok(x.ln_1p()) }
    pub fn toRadians(angdeg: f64) -> Result<f64> { Ok(angdeg.to_radians()) }
    pub fn toDegrees(angrad: f64) -> Result<f64> { Ok(angrad.to_degrees()) }
    pub fn copySign__d_d(magnitude: f64, sign: f64) -> Result<f64> { Ok(magnitude.copysign(sign)) }
    pub fn copySign__f_f(magnitude: f32, sign: f32) -> Result<f32> { Ok(magnitude.copysign(sign)) }
    pub fn signum__d(d: f64) -> Result<f64> { Ok(d.signum()) }
    pub fn signum__f(f: f32) -> Result<f32> { Ok(f.signum()) }
    pub fn nextAfter__d_d(start: f64, direction: f64) -> Result<f64> {
        Ok(if direction > start { f64::from_bits(start.to_bits() + 1) }
           else if direction < start { f64::from_bits(start.to_bits() - 1) }
           else { direction })
    }
    pub fn nextAfter__f_d(start: f32, direction: f64) -> Result<f32> {
        Ok(if direction > start as f64 { f32::from_bits(start.to_bits() + 1) }
           else if direction < start as f64 { f32::from_bits(start.to_bits() - 1) }
           else { start })
    }
    pub fn nextUp__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() + 1)) }
    pub fn nextUp__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() + 1)) }
    pub fn nextDown__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() - 1)) }
    pub fn nextDown__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() - 1)) }
    pub fn scalb__d_i(d: f64, scale_factor: i32) -> Result<f64> {
        Ok(d * (2f64.powi(scale_factor)))
    }
    pub fn scalb__f_i(f: f32, scale_factor: i32) -> Result<f32> {
        Ok(f * (2f32.powi(scale_factor)))
    }
    pub fn getExponent__d(d: f64) -> Result<i32> {
        Ok(((d.to_bits() >> 52) as i32 & 0x7FF) - 1023)
    }
    pub fn getExponent__f(f: f32) -> Result<i32> {
        Ok(((f.to_bits() >> 23) as i32 & 0xFF) - 127)
    }
    pub fn fma__d_d_d(a: f64, b: f64, c: f64) -> Result<f64> { Ok(a.mul_add(b, c)) }
    pub fn fma__f_f_f(a: f32, b: f32, c: f32) -> Result<f32> { Ok(a.mul_add(b, c)) }
}
