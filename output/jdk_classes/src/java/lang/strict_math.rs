#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/StrictMath",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "StrictMath.java",
))]
pub struct StrictMath;

impl StrictMath {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: sin(D)D
    pub fn sin(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Sin::compute(a)?;
        Ok(_t0)
    }

    // java: cos(D)D
    pub fn cos(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Cos::compute(a)?;
        Ok(_t0)
    }

    // java: tan(D)D
    pub fn tan(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Tan::compute(a)?;
        Ok(_t0)
    }

    // java: asin(D)D
    pub fn asin(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Asin::compute(a)?;
        Ok(_t0)
    }

    // java: acos(D)D
    pub fn acos(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Acos::compute(a)?;
        Ok(_t0)
    }

    // java: atan(D)D
    pub fn atan(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Atan::compute(a)?;
        Ok(_t0)
    }

    // java: toRadians(D)D
    pub fn toRadians(angdeg: f64) -> Result<f64> {
        let _t0: f64 = (angdeg).abs();
        Ok(_t0)
    }

    // java: toDegrees(D)D
    pub fn toDegrees(angrad: f64) -> Result<f64> {
        let _t0: f64 = (angrad).abs();
        Ok(_t0)
    }

    // java: exp(D)D
    pub fn exp(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Exp::compute(a)?;
        Ok(_t0)
    }

    // java: log(D)D
    pub fn log(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Log::compute(a)?;
        Ok(_t0)
    }

    // java: log10(D)D
    pub fn log10(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Log10::compute(a)?;
        Ok(_t0)
    }

    // java: sqrt(D)D
    pub fn sqrt(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Sqrt::compute(a)?;
        Ok(_t0)
    }

    // java: cbrt(D)D
    pub fn cbrt(a: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Cbrt::compute(a)?;
        Ok(_t0)
    }

    // java: IEEEremainder(DD)D
    pub fn IEEEremainder(f1: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_IEEEremainder::compute(f1, local_2)?;
        Ok(_t0)
    }

    // java: ceil(D)D
    pub fn ceil(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::floorOrCeil(a, -0.0f64, 1f64, 1f64)?;
        Ok(_t0)
    }

    // java: floor(D)D
    pub fn floor(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::floorOrCeil(a, -1.0f64, 0f64, -1.0f64)?;
        Ok(_t0)
    }

    // java: floorOrCeil(DDDD)D
    pub fn floorOrCeil(a: f64, arg_1: f64, negativeBoundary: f64, arg_3: f64) -> Result<f64> {
        let _t0: i32 = (a).abs();
        let mut exponent: i32 = _t0;
        /* TODO: dcmpl  */
        /* TODO: dcmpg  */
        return Ok(local_4);
        return Ok(a);
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: i64 = Double::doubleToRawLongBits(a)?;
        let mut doppel: i64 = _t1;
        /* TODO: lshr  */
        let mut mask: i64 = exponent;
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok(a);
        /* TODO: lxor  */
        /* TODO: land  */
        let _t2: f64 = Double::longBitsToDouble(18446744073709551615i64)?;
        let mut result: f64 = _t2;
        /* TODO: dcmpl  */
        result = (result+local_6);
        Ok(result)
    }

    // java: rint(D)D
    pub fn rint(a: f64) -> Result<f64> {
        let mut twoToThe52: f64 = 4503599627370496.0f64;
        let _t0: f64 = (1f64).abs();
        let mut sign: f64 = _t0;
        let _t1: f64 = (a).abs();
        a = _t1;
        /* TODO: dcmpg  */
        a = ((twoToThe52+a)-twoToThe52);
        Ok((sign*a))
    }

    // java: atan2(DD)D
    pub fn atan2(y: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Atan2::compute(y, local_2)?;
        Ok(_t0)
    }

    // java: pow(DD)D
    pub fn pow(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Pow::compute(a, local_2)?;
        Ok(_t0)
    }

    // java: round(F)I
    // java: round(F)I
    pub fn round__f(a: f32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: round(D)J
    // java: round(D)J
    pub fn round__d(a: f64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: random()D
    pub fn random() -> Result<f64> {
        let _t0 = StrictMath_RandomNumberGeneratorHolder::randomNumberGenerator().nextDouble()?;
        Ok(_t0)
    }

    // java: addExact(II)I
    // java: addExact(II)I
    pub fn addExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: addExact(JJ)J
    // java: addExact(JJ)J
    pub fn addExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: subtractExact(II)I
    // java: subtractExact(II)I
    pub fn subtractExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: subtractExact(JJ)J
    // java: subtractExact(JJ)J
    pub fn subtractExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: multiplyExact(II)I
    // java: multiplyExact(II)I
    pub fn multiplyExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: multiplyExact(JI)J
    // java: multiplyExact(JI)J
    pub fn multiplyExact__l_i(x: i64, arg_1: i32) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: multiplyExact(JJ)J
    // java: multiplyExact(JJ)J
    pub fn multiplyExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: divideExact(II)I
    // java: divideExact(II)I
    pub fn divideExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: divideExact(JJ)J
    // java: divideExact(JJ)J
    pub fn divideExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: floorDivExact(II)I
    // java: floorDivExact(II)I
    pub fn floorDivExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: floorDivExact(JJ)J
    // java: floorDivExact(JJ)J
    pub fn floorDivExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: ceilDivExact(II)I
    // java: ceilDivExact(II)I
    pub fn ceilDivExact__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: ceilDivExact(JJ)J
    // java: ceilDivExact(JJ)J
    pub fn ceilDivExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: incrementExact(I)I
    // java: incrementExact(I)I
    pub fn incrementExact__i(a: i32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: incrementExact(J)J
    // java: incrementExact(J)J
    pub fn incrementExact__l(a: i64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: decrementExact(I)I
    // java: decrementExact(I)I
    pub fn decrementExact__i(a: i32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: decrementExact(J)J
    // java: decrementExact(J)J
    pub fn decrementExact__l(a: i64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: negateExact(I)I
    // java: negateExact(I)I
    pub fn negateExact__i(a: i32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: negateExact(J)J
    // java: negateExact(J)J
    pub fn negateExact__l(a: i64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: toIntExact(J)I
    pub fn toIntExact(value: i64) -> Result<i32> {
        let _t0: i32 = (value).abs();
        Ok(_t0)
    }

    // java: multiplyFull(II)J
    pub fn multiplyFull(x: i32, y: i32) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: multiplyHigh(JJ)J
    pub fn multiplyHigh(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: unsignedMultiplyHigh(JJ)J
    pub fn unsignedMultiplyHigh(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: floorDiv(II)I
    // java: floorDiv(II)I
    pub fn floorDiv__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: floorDiv(JI)J
    // java: floorDiv(JI)J
    pub fn floorDiv__l_i(x: i64, arg_1: i32) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: floorDiv(JJ)J
    // java: floorDiv(JJ)J
    pub fn floorDiv__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: floorMod(II)I
    // java: floorMod(II)I
    pub fn floorMod__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: floorMod(JI)I
    // java: floorMod(JI)I
    pub fn floorMod__l_i(x: i64, arg_1: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: floorMod(JJ)J
    // java: floorMod(JJ)J
    pub fn floorMod__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: ceilDiv(II)I
    // java: ceilDiv(II)I
    pub fn ceilDiv__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: ceilDiv(JI)J
    // java: ceilDiv(JI)J
    pub fn ceilDiv__l_i(x: i64, arg_1: i32) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: ceilDiv(JJ)J
    // java: ceilDiv(JJ)J
    pub fn ceilDiv__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: ceilMod(II)I
    // java: ceilMod(II)I
    pub fn ceilMod__i_i(x: i32, y: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: ceilMod(JI)I
    // java: ceilMod(JI)I
    pub fn ceilMod__l_i(x: i64, arg_1: i32) -> Result<i32> {
        let _t0: i32 = (x).abs();
        Ok(_t0)
    }

    // java: ceilMod(JJ)J
    // java: ceilMod(JJ)J
    pub fn ceilMod__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        Ok(_t0)
    }

    // java: abs(I)I
    // java: abs(I)I
    pub fn abs__i(a: i32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: absExact(I)I
    // java: absExact(I)I
    pub fn absExact__i(a: i32) -> Result<i32> {
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: abs(J)J
    // java: abs(J)J
    pub fn abs__l(a: i64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: absExact(J)J
    // java: absExact(J)J
    pub fn absExact__l(a: i64) -> Result<i64> {
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: abs(F)F
    // java: abs(F)F
    pub fn abs__f(a: f32) -> Result<f32> {
        let _t0: f32 = (a).abs();
        Ok(_t0)
    }

    // java: abs(D)D
    // java: abs(D)D
    pub fn abs__d(a: f64) -> Result<f64> {
        let _t0: f64 = (a).abs();
        Ok(_t0)
    }

    // java: max(II)I
    // java: max(II)I
    pub fn max__i_i(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).max(b);
        Ok(_t0)
    }

    // java: max(JJ)J
    // java: max(JJ)J
    pub fn max__l_l(a: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (a).max(local_2);
        Ok(_t0)
    }

    // java: max(FF)F
    // java: max(FF)F
    pub fn max__f_f(a: f32, b: f32) -> Result<f32> {
        let _t0: f32 = (a).max(b);
        Ok(_t0)
    }

    // java: max(DD)D
    // java: max(DD)D
    pub fn max__d_d(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = (a).max(local_2);
        Ok(_t0)
    }

    // java: min(II)I
    // java: min(II)I
    pub fn min__i_i(a: i32, b: i32) -> Result<i32> {
        let _t0: i32 = (a).min(b);
        Ok(_t0)
    }

    // java: min(JJ)J
    // java: min(JJ)J
    pub fn min__l_l(a: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (a).min(local_2);
        Ok(_t0)
    }

    // java: min(FF)F
    // java: min(FF)F
    pub fn min__f_f(a: f32, b: f32) -> Result<f32> {
        let _t0: f32 = (a).min(b);
        Ok(_t0)
    }

    // java: min(DD)D
    // java: min(DD)D
    pub fn min__d_d(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = (a).min(local_2);
        Ok(_t0)
    }

    // java: clamp(JII)I
    // java: clamp(JII)I
    pub fn clamp__l_i_i(value: i64, arg_1: i32, min: i32) -> Result<i32> {
        let _t0: i32 = (value).abs();
        Ok(_t0)
    }

    // java: clamp(JJJ)J
    // java: clamp(JJJ)J
    pub fn clamp__l_l_l(value: i64, arg_1: i64, min: i64) -> Result<i64> {
        let _t0: i64 = (value).abs();
        Ok(_t0)
    }

    // java: clamp(DDD)D
    // java: clamp(DDD)D
    pub fn clamp__d_d_d(value: f64, arg_1: f64, min: f64) -> Result<f64> {
        let _t0: f64 = (value).abs();
        Ok(_t0)
    }

    // java: clamp(FFF)F
    // java: clamp(FFF)F
    pub fn clamp__f_f_f(value: f32, min: f32, max: f32) -> Result<f32> {
        let _t0: f32 = (value).abs();
        Ok(_t0)
    }

    // java: fma(DDD)D
    // java: fma(DDD)D
    pub fn fma__d_d_d(a: f64, arg_1: f64, b: f64) -> Result<f64> {
        let _t0: f64 = (a).abs();
        Ok(_t0)
    }

    // java: fma(FFF)F
    // java: fma(FFF)F
    pub fn fma__f_f_f(a: f32, b: f32, c: f32) -> Result<f32> {
        let _t0: f32 = (a).abs();
        Ok(_t0)
    }

    // java: ulp(D)D
    // java: ulp(D)D
    pub fn ulp__d(d: f64) -> Result<f64> {
        let _t0: f64 = (d).abs();
        Ok(_t0)
    }

    // java: ulp(F)F
    // java: ulp(F)F
    pub fn ulp__f(f: f32) -> Result<f32> {
        let _t0: f32 = (f).abs();
        Ok(_t0)
    }

    // java: signum(D)D
    // java: signum(D)D
    pub fn signum__d(d: f64) -> Result<f64> {
        let _t0: f64 = (d).abs();
        Ok(_t0)
    }

    // java: signum(F)F
    // java: signum(F)F
    pub fn signum__f(f: f32) -> Result<f32> {
        let _t0: f32 = (f).abs();
        Ok(_t0)
    }

    // java: sinh(D)D
    pub fn sinh(x: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Sinh::compute(x)?;
        Ok(_t0)
    }

    // java: cosh(D)D
    pub fn cosh(x: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Cosh::compute(x)?;
        Ok(_t0)
    }

    // java: tanh(D)D
    pub fn tanh(x: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Tanh::compute(x)?;
        Ok(_t0)
    }

    // java: hypot(DD)D
    pub fn hypot(x: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Hypot::compute(x, local_2)?;
        Ok(_t0)
    }

    // java: expm1(D)D
    pub fn expm1(x: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Expm1::compute(x)?;
        Ok(_t0)
    }

    // java: log1p(D)D
    pub fn log1p(x: f64) -> Result<f64> {
        let _t0: f64 = FdLibm_Log1p::compute(x)?;
        Ok(_t0)
    }

    // java: copySign(DD)D
    // java: copySign(DD)D
    pub fn copySign__d_d(magnitude: f64, arg_1: f64) -> Result<f64> {
        let _t0: bool = Double::isNaN(local_2)?;
        let _t1: f64 = (1f64).abs();
        Ok(_t1)
    }

    // java: copySign(FF)F
    // java: copySign(FF)F
    pub fn copySign__f_f(magnitude: f32, sign: f32) -> Result<f32> {
        let _t0: bool = Float::isNaN(sign)?;
        let _t1: f32 = (1f32).abs();
        Ok(_t1)
    }

    // java: getExponent(F)I
    // java: getExponent(F)I
    pub fn getExponent__f(f: f32) -> Result<i32> {
        let _t0: i32 = (f).abs();
        Ok(_t0)
    }

    // java: getExponent(D)I
    // java: getExponent(D)I
    pub fn getExponent__d(d: f64) -> Result<i32> {
        let _t0: i32 = (d).abs();
        Ok(_t0)
    }

    // java: nextAfter(DD)D
    // java: nextAfter(DD)D
    pub fn nextAfter__d_d(start: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = (start).abs();
        Ok(_t0)
    }

    // java: nextAfter(FD)F
    // java: nextAfter(FD)F
    pub fn nextAfter__f_d(start: f32, direction: f64) -> Result<f32> {
        let _t0: f32 = (start).abs();
        Ok(_t0)
    }

    // java: nextUp(D)D
    // java: nextUp(D)D
    pub fn nextUp__d(d: f64) -> Result<f64> {
        let _t0: f64 = (d).abs();
        Ok(_t0)
    }

    // java: nextUp(F)F
    // java: nextUp(F)F
    pub fn nextUp__f(f: f32) -> Result<f32> {
        let _t0: f32 = (f).abs();
        Ok(_t0)
    }

    // java: nextDown(D)D
    // java: nextDown(D)D
    pub fn nextDown__d(d: f64) -> Result<f64> {
        let _t0: f64 = (d).abs();
        Ok(_t0)
    }

    // java: nextDown(F)F
    // java: nextDown(F)F
    pub fn nextDown__f(f: f32) -> Result<f32> {
        let _t0: f32 = (f).abs();
        Ok(_t0)
    }

    // java: scalb(DI)D
    // java: scalb(DI)D
    pub fn scalb__d_i(d: f64, arg_1: i32) -> Result<f64> {
        let _t0: f64 = (d).abs();
        Ok(_t0)
    }

    // java: scalb(FI)F
    // java: scalb(FI)F
    pub fn scalb__f_i(f: f32, scaleFactor: i32) -> Result<f32> {
        let _t0: f32 = (f).abs();
        Ok(_t0)
    }
}
