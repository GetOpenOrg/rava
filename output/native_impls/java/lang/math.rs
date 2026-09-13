/// java/lang/Math.sin:(D)D
pub fn sin(a: f64) -> Result<f64> { Ok(a.sin()) }

/// java/lang/Math.cos:(D)D
pub fn cos(a: f64) -> Result<f64> { Ok(a.cos()) }

/// java/lang/Math.tan:(D)D
pub fn tan(a: f64) -> Result<f64> { Ok(a.tan()) }

/// java/lang/Math.asin:(D)D
pub fn asin(a: f64) -> Result<f64> { Ok(a.asin()) }

/// java/lang/Math.acos:(D)D
pub fn acos(a: f64) -> Result<f64> { Ok(a.acos()) }

/// java/lang/Math.atan:(D)D
pub fn atan(a: f64) -> Result<f64> { Ok(a.atan()) }

/// java/lang/Math.atan2:(DD)D
pub fn atan2(y: f64, x: f64) -> Result<f64> { Ok(y.atan2(x)) }

/// java/lang/Math.exp:(D)D
pub fn exp(a: f64) -> Result<f64> { Ok(a.exp()) }

/// java/lang/Math.log:(D)D
pub fn log(a: f64) -> Result<f64> { Ok(a.ln()) }

/// java/lang/Math.log10:(D)D
pub fn log10(a: f64) -> Result<f64> { Ok(a.log10()) }

/// java/lang/Math.sqrt:(D)D
pub fn sqrt(a: f64) -> Result<f64> { Ok(a.sqrt()) }

/// java/lang/Math.cbrt:(D)D
pub fn cbrt(a: f64) -> Result<f64> { Ok(a.cbrt()) }

/// java/lang/Math.IEEEremainder:(DD)D
pub fn IEEEremainder(f1: f64, f2: f64) -> Result<f64> { Ok(f1 - f2 * (f1 / f2).round()) }

/// java/lang/Math.ceil:(D)D
pub fn ceil(a: f64) -> Result<f64> { Ok(a.ceil()) }

/// java/lang/Math.floor:(D)D
pub fn floor(a: f64) -> Result<f64> { Ok(a.floor()) }

/// java/lang/Math.rint:(D)D
pub fn rint(a: f64) -> Result<f64> { Ok(a.round()) }

/// java/lang/Math.pow:(DD)D
pub fn pow(a: f64, b: f64) -> Result<f64> { Ok(a.powf(b)) }

/// java/lang/Math.round:(F)I
pub fn round__f(a: f32) -> Result<i32> { Ok(a.round() as i32) }

/// java/lang/Math.round:(D)J
pub fn round__d(a: f64) -> Result<i64> { Ok(a.round() as i64) }

/// java/lang/Math.random:()D
pub fn random() -> Result<f64> { Ok(0.5f64) }

/// java/lang/Math.sinh:(D)D
pub fn sinh(x: f64) -> Result<f64> { Ok(x.sinh()) }

/// java/lang/Math.cosh:(D)D
pub fn cosh(x: f64) -> Result<f64> { Ok(x.cosh()) }

/// java/lang/Math.tanh:(D)D
pub fn tanh(x: f64) -> Result<f64> { Ok(x.tanh()) }

/// java/lang/Math.hypot:(DD)D
pub fn hypot(x: f64, y: f64) -> Result<f64> { Ok(x.hypot(y)) }

/// java/lang/Math.expm1:(D)D
pub fn expm1(x: f64) -> Result<f64> { Ok(x.exp_m1()) }

/// java/lang/Math.log1p:(D)D
pub fn log1p(x: f64) -> Result<f64> { Ok(x.ln_1p()) }

/// java/lang/Math.toRadians:(D)D
pub fn toRadians(angdeg: f64) -> Result<f64> { Ok(angdeg.to_radians()) }

/// java/lang/Math.toDegrees:(D)D
pub fn toDegrees(angrad: f64) -> Result<f64> { Ok(angrad.to_degrees()) }

/// java/lang/Math.copySign:(DD)D
pub fn copySign__d_d(magnitude: f64, sign: f64) -> Result<f64> { Ok(magnitude.copysign(sign)) }

/// java/lang/Math.copySign:(FF)F
pub fn copySign__f_f(magnitude: f32, sign: f32) -> Result<f32> { Ok(magnitude.copysign(sign)) }

/// java/lang/Math.signum:(D)D
pub fn signum__d(d: f64) -> Result<f64> { Ok(d.signum()) }

/// java/lang/Math.signum:(F)F
pub fn signum__f(f: f32) -> Result<f32> { Ok(f.signum()) }

/// java/lang/Math.nextAfter:(DD)D
pub fn nextAfter__d_d(start: f64, direction: f64) -> Result<f64> {
    Ok(if direction > start { f64::from_bits(start.to_bits() + 1) }
       else if direction < start { f64::from_bits(start.to_bits() - 1) }
       else { direction })
}

/// java/lang/Math.nextAfter:(FD)F
pub fn nextAfter__f_d(start: f32, direction: f64) -> Result<f32> {
    Ok(if direction > start as f64 { f32::from_bits(start.to_bits() + 1) }
       else if direction < start as f64 { f32::from_bits(start.to_bits() - 1) }
       else { start })
}

/// java/lang/Math.nextUp:(D)D
pub fn nextUp__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() + 1)) }

/// java/lang/Math.nextUp:(F)F
pub fn nextUp__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() + 1)) }

/// java/lang/Math.nextDown:(D)D
pub fn nextDown__d(d: f64) -> Result<f64> { Ok(f64::from_bits(d.to_bits() - 1)) }

/// java/lang/Math.nextDown:(F)F
pub fn nextDown__f(f: f32) -> Result<f32> { Ok(f32::from_bits(f.to_bits() - 1)) }

/// java/lang/Math.scalb:(DI)D
pub fn scalb__d_i(d: f64, scale_factor: i32) -> Result<f64> {
    Ok(d * (2f64.powi(scale_factor)))
}

/// java/lang/Math.scalb:(FI)F
pub fn scalb__f_i(f: f32, scale_factor: i32) -> Result<f32> {
    Ok(f * (2f32.powi(scale_factor)))
}

/// java/lang/Math.getExponent:(D)I
pub fn getExponent__d(d: f64) -> Result<i32> {
    Ok(((d.to_bits() >> 52) as i32 & 0x7FF) - 1023)
}

/// java/lang/Math.getExponent:(F)I
pub fn getExponent__f(f: f32) -> Result<i32> {
    Ok(((f.to_bits() >> 23) as i32 & 0xFF) - 127)
}

/// java/lang/Math.fma:(DDD)D
pub fn fma__d_d_d(a: f64, b: f64, c: f64) -> Result<f64> { Ok(a.mul_add(b, c)) }

/// java/lang/Math.fma:(FFF)F
pub fn fma__f_f_f(a: f32, b: f32, c: f32) -> Result<f32> { Ok(a.mul_add(b, c)) }
