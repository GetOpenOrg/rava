#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/lang/Math",
    super_class = "java/lang/Object",
    interfaces  = "",
    access      = "public final",
    source      = "Math.java",
))]
pub struct Math;

impl Math {
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self {};
        /* invokespecial Method java/lang/Object.<init>:()V */
        Ok(this)
    }

    // java: sin(D)D
    pub fn sin(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::sin(a)?;
        Ok(_t0)
    }

    // java: cos(D)D
    pub fn cos(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::cos(a)?;
        Ok(_t0)
    }

    // java: tan(D)D
    pub fn tan(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::tan(a)?;
        Ok(_t0)
    }

    // java: asin(D)D
    pub fn asin(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::asin(a)?;
        Ok(_t0)
    }

    // java: acos(D)D
    pub fn acos(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::acos(a)?;
        Ok(_t0)
    }

    // java: atan(D)D
    pub fn atan(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::atan(a)?;
        Ok(_t0)
    }

    // java: toRadians(D)D
    pub fn toRadians(angdeg: f64) -> Result<f64> {
        Ok((angdeg*0.017453292519943295f64))
    }

    // java: toDegrees(D)D
    pub fn toDegrees(angrad: f64) -> Result<f64> {
        Ok((angrad*57.29577951308232f64))
    }

    // java: exp(D)D
    pub fn exp(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::exp(a)?;
        Ok(_t0)
    }

    // java: log(D)D
    pub fn log(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::log(a)?;
        Ok(_t0)
    }

    // java: log10(D)D
    pub fn log10(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::log10(a)?;
        Ok(_t0)
    }

    // java: sqrt(D)D
    pub fn sqrt(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::sqrt(a)?;
        Ok(_t0)
    }

    // java: cbrt(D)D
    pub fn cbrt(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::cbrt(a)?;
        Ok(_t0)
    }

    // java: IEEEremainder(DD)D
    pub fn IEEEremainder(f1: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::IEEEremainder(f1, local_2)?;
        Ok(_t0)
    }

    // java: ceil(D)D
    pub fn ceil(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::ceil(a)?;
        Ok(_t0)
    }

    // java: floor(D)D
    pub fn floor(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::floor(a)?;
        Ok(_t0)
    }

    // java: rint(D)D
    pub fn rint(a: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::rint(a)?;
        Ok(_t0)
    }

    // java: atan2(DD)D
    pub fn atan2(y: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::atan2(y, local_2)?;
        Ok(_t0)
    }

    // java: pow(DD)D
    pub fn pow(a: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::pow(a, local_2)?;
        Ok(_t0)
    }

    // java: round(F)I
    // java: round(F)I
    pub fn round__f(a: f32) -> Result<i32> {
        let _t0: i32 = Float::floatToRawIntBits(a)?;
        let mut intBits: i32 = _t0;
        let mut biasedExp: i32 = ((intBits&2139095040i32)>>((23i32&0x1f)));
        let mut shift: i32 = (149i32).wrapping_sub(biasedExp);
        let mut r: i32 = ((intBits&8388607i32)|8388608i32);
        r = (r).wrapping_neg();
        return Ok((((r>>((shift&0x1f)))).wrapping_add(1i32)>>((1i32&0x1f))));
        Ok((a as i32))
    }

    // java: round(D)J
    // java: round(D)J
    pub fn round__d(a: f64) -> Result<i64> {
        let _t0: i64 = Double::doubleToRawLongBits(a)?;
        let mut longBits: i64 = _t0;
        /* TODO: land  */
        /* TODO: lshr  */
        let mut biasedExp: i64 = 52i32;
        let mut shift: i64 = (1074i64).wrapping_sub(biasedExp);
        /* TODO: land  */
        /* TODO: lcmp  */
        /* TODO: land  */
        /* TODO: lor  */
        let mut r: i64 = 4503599627370496i64;
        /* TODO: lcmp  */
        /* TODO: lneg  */
        r = r;
        /* TODO: lshr  */
        /* TODO: lshr  */
        return Ok(1i32);
        /* TODO: d2l  */
        Ok(a)
    }

    // java: random()D
    pub fn random() -> Result<f64> {
        let _t0 = Math_RandomNumberGeneratorHolder::randomNumberGenerator().nextDouble()?;
        Ok(_t0)
    }

    // java: addExact(II)I
    // java: addExact(II)I
    pub fn addExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut r: i32 = (x).wrapping_add(y);
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(r)
    }

    // java: addExact(JJ)J
    // java: addExact(JJ)J
    pub fn addExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut r: i64 = (x).wrapping_add(local_2);
        /* TODO: lxor  */
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(r)
    }

    // java: subtractExact(II)I
    // java: subtractExact(II)I
    pub fn subtractExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut r: i32 = (x).wrapping_sub(y);
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(r)
    }

    // java: subtractExact(JJ)J
    // java: subtractExact(JJ)J
    pub fn subtractExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut r: i64 = (x).wrapping_sub(local_2);
        /* TODO: lxor  */
        /* TODO: lxor  */
        /* TODO: land  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(r)
    }

    // java: multiplyExact(II)I
    // java: multiplyExact(II)I
    pub fn multiplyExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut r: i64 = ((x as i64)).wrapping_mul((y as i64));
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((r as i32))
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
        let mut r: i64 = (x).wrapping_mul(local_2);
        let _t0: i64 = (x).abs();
        let mut ax: i64 = _t0;
        let _t1: i64 = (local_2).abs();
        let mut ay: i64 = _t1;
        /* TODO: lor  */
        /* TODO: lushr  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(r)
    }

    // java: divideExact(II)I
    // java: divideExact(II)I
    pub fn divideExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut q: i32 = (x/y);
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: divideExact(JJ)J
    // java: divideExact(JJ)J
    pub fn divideExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut q: i64 = (x/local_2);
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lcmp  */
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: floorDivExact(II)I
    // java: floorDivExact(II)I
    pub fn floorDivExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut q: i32 = (x/y);
        return Ok((q).wrapping_sub(1i32));
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: floorDivExact(JJ)J
    // java: floorDivExact(JJ)J
    pub fn floorDivExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut q: i64 = (x/local_2);
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lcmp  */
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((q).wrapping_sub(1i64));
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: ceilDivExact(II)I
    // java: ceilDivExact(II)I
    pub fn ceilDivExact__i_i(x: i32, y: i32) -> Result<i32> {
        let mut q: i32 = (x/y);
        return Ok((q).wrapping_add(1i32));
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: ceilDivExact(JJ)J
    // java: ceilDivExact(JJ)J
    pub fn ceilDivExact__l_l(x: i64, arg_1: i64) -> Result<i64> {
        let mut q: i64 = (x/local_2);
        /* TODO: land  */
        /* TODO: land  */
        /* TODO: lcmp  */
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((q).wrapping_add(1i64));
        return Ok(q);
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: incrementExact(I)I
    // java: incrementExact(I)I
    pub fn incrementExact__i(a: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((a).wrapping_add(1i32))
    }

    // java: incrementExact(J)J
    // java: incrementExact(J)J
    pub fn incrementExact__l(a: i64) -> Result<i64> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((a).wrapping_add(1i64))
    }

    // java: decrementExact(I)I
    // java: decrementExact(I)I
    pub fn decrementExact__i(a: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((a).wrapping_sub(1i32))
    }

    // java: decrementExact(J)J
    // java: decrementExact(J)J
    pub fn decrementExact__l(a: i64) -> Result<i64> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((a).wrapping_sub(1i64))
    }

    // java: negateExact(I)I
    // java: negateExact(I)I
    pub fn negateExact__i(a: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((a).wrapping_neg())
    }

    // java: negateExact(J)J
    // java: negateExact(J)J
    pub fn negateExact__l(a: i64) -> Result<i64> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: lneg  */
        Ok(a)
    }

    // java: toIntExact(J)I
    pub fn toIntExact(value: i64) -> Result<i32> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok((value as i32))
    }

    // java: multiplyFull(II)J
    pub fn multiplyFull(x: i32, y: i32) -> Result<i64> {
        Ok(((x as i64)).wrapping_mul((y as i64)))
    }

    // java: multiplyHigh(JJ)J
    pub fn multiplyHigh(x: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lshr  */
        let mut x1: i64 = 32i32;
        /* TODO: land  */
        let mut x2: i64 = 4294967295i64;
        /* TODO: lshr  */
        let mut y1: i64 = 32i32;
        /* TODO: land  */
        let mut y2: i64 = 4294967295i64;
        let mut z2: i64 = (x2).wrapping_mul(y2);
        /* TODO: lushr  */
        let mut t: i64 = (z2).wrapping_add(32i32);
        /* TODO: land  */
        let mut z1: i64 = 4294967295i64;
        /* TODO: lshr  */
        let mut z0: i64 = 32i32;
        z1 = (z1).wrapping_add((x2).wrapping_mul(y1));
        /* TODO: lshr  */
        Ok((z1).wrapping_add(32i32))
    }

    // java: unsignedMultiplyHigh(JJ)J
    pub fn unsignedMultiplyHigh(x: i64, arg_1: i64) -> Result<i64> {
        let _t0: i64 = (x).abs();
        let mut result: i64 = _t0;
        /* TODO: lshr  */
        /* TODO: land  */
        result = (x).wrapping_add(63i32);
        /* TODO: lshr  */
        /* TODO: land  */
        result = (local_2).wrapping_add(63i32);
        Ok(result)
    }

    // java: floorDiv(II)I
    // java: floorDiv(II)I
    pub fn floorDiv__i_i(x: i32, y: i32) -> Result<i32> {
        let mut q: i32 = (x/y);
        return Ok((q).wrapping_sub(1i32));
        Ok(q)
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
        let mut q: i64 = (x/local_2);
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((q).wrapping_sub(1i64));
        Ok(q)
    }

    // java: floorMod(II)I
    // java: floorMod(II)I
    pub fn floorMod__i_i(x: i32, y: i32) -> Result<i32> {
        let mut r: i32 = (x%y);
        return Ok((r).wrapping_add(y));
        Ok(r)
    }

    // java: floorMod(JI)I
    // java: floorMod(JI)I
    pub fn floorMod__l_i(x: i64, arg_1: i32) -> Result<i32> {
        let _t0: i64 = (x).abs();
        Ok((_t0 as i32))
    }

    // java: floorMod(JJ)J
    // java: floorMod(JJ)J
    pub fn floorMod__l_l(x: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lrem  */
        let mut r: i64 = local_2;
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((r).wrapping_add(local_2));
        Ok(r)
    }

    // java: ceilDiv(II)I
    // java: ceilDiv(II)I
    pub fn ceilDiv__i_i(x: i32, y: i32) -> Result<i32> {
        let mut q: i32 = (x/y);
        return Ok((q).wrapping_add(1i32));
        Ok(q)
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
        let mut q: i64 = (x/local_2);
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((q).wrapping_add(1i64));
        Ok(q)
    }

    // java: ceilMod(II)I
    // java: ceilMod(II)I
    pub fn ceilMod__i_i(x: i32, y: i32) -> Result<i32> {
        let mut r: i32 = (x%y);
        return Ok((r).wrapping_sub(y));
        Ok(r)
    }

    // java: ceilMod(JI)I
    // java: ceilMod(JI)I
    pub fn ceilMod__l_i(x: i64, arg_1: i32) -> Result<i32> {
        let _t0: i64 = (x).abs();
        Ok((_t0 as i32))
    }

    // java: ceilMod(JJ)J
    // java: ceilMod(JJ)J
    pub fn ceilMod__l_l(x: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lrem  */
        let mut r: i64 = local_2;
        /* TODO: lxor  */
        /* TODO: lcmp  */
        /* TODO: lcmp  */
        return Ok((r).wrapping_sub(local_2));
        Ok(r)
    }

    // java: abs(I)I
    // java: abs(I)I
    pub fn abs__i(a: i32) -> Result<i32> {
        Ok(a)
    }

    // java: absExact(I)I
    // java: absExact(I)I
    pub fn absExact__i(a: i32) -> Result<i32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i32 = (a).abs();
        Ok(_t0)
    }

    // java: abs(J)J
    // java: abs(J)J
    pub fn abs__l(a: i64) -> Result<i64> {
        /* TODO: lcmp  */
        /* TODO: lneg  */
        Ok(a)
    }

    // java: absExact(J)J
    // java: absExact(J)J
    pub fn absExact__l(a: i64) -> Result<i64> {
        /* TODO: lcmp  */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i64 = (a).abs();
        Ok(_t0)
    }

    // java: abs(F)F
    // java: abs(F)F
    pub fn abs__f(a: f32) -> Result<f32> {
        let _t0: i32 = Float::floatToRawIntBits(a)?;
        let _t1: f32 = Float::intBitsToFloat((_t0&2147483647i32))?;
        Ok(_t1)
    }

    // java: abs(D)D
    // java: abs(D)D
    pub fn abs__d(a: f64) -> Result<f64> {
        let _t0: i64 = Double::doubleToRawLongBits(a)?;
        /* TODO: land  */
        let _t1: f64 = Double::longBitsToDouble(9223372036854775807i64)?;
        Ok(_t1)
    }

    // java: max(II)I
    // java: max(II)I
    pub fn max__i_i(a: i32, b: i32) -> Result<i32> {
        Ok(b)
    }

    // java: max(JJ)J
    // java: max(JJ)J
    pub fn max__l_l(a: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lcmp  */
        Ok(local_2)
    }

    // java: max(FF)F
    // java: max(FF)F
    pub fn max__f_f(a: f32, b: f32) -> Result<f32> {
        /* TODO: fcmpl  */
        return Ok(a);
        /* TODO: fcmpl  */
        /* TODO: fcmpl  */
        let _t0: i32 = Float::floatToRawIntBits(a)?;
        /* TODO: lcmp  */
        return Ok(b);
        /* TODO: fcmpl  */
        Ok(b)
    }

    // java: max(DD)D
    // java: max(DD)D
    pub fn max__d_d(a: f64, arg_1: f64) -> Result<f64> {
        /* TODO: dcmpl  */
        return Ok(a);
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        let _t0: i64 = Double::doubleToRawLongBits(a)?;
        /* TODO: lcmp  */
        return Ok(local_2);
        /* TODO: dcmpl  */
        Ok(local_2)
    }

    // java: min(II)I
    // java: min(II)I
    pub fn min__i_i(a: i32, b: i32) -> Result<i32> {
        Ok(b)
    }

    // java: min(JJ)J
    // java: min(JJ)J
    pub fn min__l_l(a: i64, arg_1: i64) -> Result<i64> {
        /* TODO: lcmp  */
        Ok(local_2)
    }

    // java: min(FF)F
    // java: min(FF)F
    pub fn min__f_f(a: f32, b: f32) -> Result<f32> {
        /* TODO: fcmpl  */
        return Ok(a);
        /* TODO: fcmpl  */
        /* TODO: fcmpl  */
        let _t0: i32 = Float::floatToRawIntBits(b)?;
        /* TODO: lcmp  */
        return Ok(b);
        /* TODO: fcmpg  */
        Ok(b)
    }

    // java: min(DD)D
    // java: min(DD)D
    pub fn min__d_d(a: f64, arg_1: f64) -> Result<f64> {
        /* TODO: dcmpl  */
        return Ok(a);
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        let _t0: i64 = Double::doubleToRawLongBits(local_2)?;
        /* TODO: lcmp  */
        return Ok(local_2);
        /* TODO: dcmpg  */
        Ok(local_2)
    }

    // java: clamp(JII)I
    // java: clamp(JII)I
    pub fn clamp__l_i_i(value: i64, arg_1: i32, min: i32) -> Result<i32> {
        String::new().append(&min)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&local_3)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i64 = (value).max((min as i64));
        let _t1: i64 = ((local_3 as i64)).min(_t0);
        Ok((_t1 as i32))
    }

    // java: clamp(JJJ)J
    // java: clamp(JJJ)J
    pub fn clamp__l_l_l(value: i64, arg_1: i64, min: i64) -> Result<i64> {
        /* TODO: lcmp  */
        String::new().append(&min)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&local_4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: i64 = (value).max(min);
        let _t1: i64 = (local_4).min(_t0);
        Ok(_t1)
    }

    // java: clamp(DDD)D
    // java: clamp(DDD)D
    pub fn clamp__d_d_d(value: f64, arg_1: f64, min: f64) -> Result<f64> {
        /* TODO: dcmpg  */
        let _t0: bool = Double::isNaN(min)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: bool = Double::isNaN(local_4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: i32 = Double::compare(min, local_4)?;
        String::new().append(&min)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&local_4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: f64 = (value).max(min);
        let _t4: f64 = (local_4).min(_t3);
        Ok(_t4)
    }

    // java: clamp(FFF)F
    // java: clamp(FFF)F
    pub fn clamp__f_f_f(value: f32, min: f32, max: f32) -> Result<f32> {
        /* TODO: fcmpg  */
        let _t0: bool = Float::isNaN(min)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t1: bool = Float::isNaN(max)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t2: i32 = Float::compare(min, max)?;
        String::new().append(&min)?;
        String::new().append(&String::from(">"))?;
        String::new().append(&max)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t3: f32 = (value).max(min);
        let _t4: f32 = (max).min(_t3);
        Ok(_t4)
    }

    // java: fma(DDD)D
    // java: fma(DDD)D
    pub fn fma__d_d_d(a: f64, arg_1: f64, b: f64) -> Result<f64> {
        let _t0: bool = Double::isNaN(a)?;
        let _t1: bool = Double::isNaN(b)?;
        let _t2: bool = Double::isNaN(local_4)?;
        return Ok(nanf64);
        let _t3: bool = Double::isInfinite(a)?;
        let mut infiniteA: i32 = _t3;
        let _t4: bool = Double::isInfinite(b)?;
        let mut infiniteB: i32 = _t4;
        let _t5: bool = Double::isInfinite(local_4)?;
        let mut infiniteC: i32 = _t5;
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        return Ok(nanf64);
        let mut product: f64 = (a*b);
        let _t6: bool = Double::isInfinite(product)?;
        let _t7: bool = Double::isInfinite(local_4)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(local_4);
        let mut result: f64 = (product+local_4);
        let _t8: bool = Double::isFinite(result)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(result);
        let _t9 = BigDecimal::new(a)?.multiply(BigDecimal::new(b)?)?;
        product = _t9;
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        return Ok(((a*b)+local_4));
        return Ok(product);
        let _t10 = product.add(BigDecimal::new(local_4)?)?;
        Ok(_t10)
    }

    // java: fma(FFF)F
    // java: fma(FFF)F
    pub fn fma__f_f_f(a: f32, b: f32, c: f32) -> Result<f32> {
        let _t0: bool = Float::isFinite(a)?;
        let _t1: bool = Float::isFinite(b)?;
        let _t2: bool = Float::isFinite(c)?;
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        return Ok(((a*b)+c));
        let _t3 = BigDecimal::new(((a as f64)*(b as f64)))?.add(BigDecimal::new((c as f64))?)?;
        return Ok(_t3);
        let _t4: f64 = ((a as f64)).abs();
        Ok((_t4 as f32))
    }

    // java: ulp(D)D
    // java: ulp(D)D
    pub fn ulp__d(d: f64) -> Result<f64> {
        let _t0: i32 = (d).abs();
        let mut exp: i32 = _t0;
        /* TODO: lookupswitch default:45 -1023:39 1024:32 */
        let _t1: f64 = (d).abs();
        return Err(JvmError::Custom("athrow".to_owned()));
        exp = (exp).wrapping_sub(52i32);
        let _t2: f64 = (exp).abs();
        /* TODO: lshl  */
        let _t3: f64 = Double::longBitsToDouble((exp).wrapping_sub(-1074i32))?;
        Ok(_t3)
    }

    // java: ulp(F)F
    // java: ulp(F)F
    pub fn ulp__f(f: f32) -> Result<f32> {
        let _t0: i32 = (f).abs();
        let mut exp: i32 = _t0;
        /* TODO: lookupswitch default:45 -127:39 128:32 */
        let _t1: f32 = (f).abs();
        return Err(JvmError::Custom("athrow".to_owned()));
        exp = (exp).wrapping_sub(23i32);
        let _t2: f32 = (exp).abs();
        let _t3: f32 = Float::intBitsToFloat((1i32<<((exp).wrapping_sub(-149i32)&0x1f)))?;
        Ok(_t3)
    }

    // java: signum(D)D
    // java: signum(D)D
    pub fn signum__d(d: f64) -> Result<f64> {
        /* TODO: dcmpl  */
        let _t0: bool = Double::isNaN(d)?;
        let _t1: f64 = (1f64).abs();
        Ok(_t1)
    }

    // java: signum(F)F
    // java: signum(F)F
    pub fn signum__f(f: f32) -> Result<f32> {
        /* TODO: fcmpl  */
        let _t0: bool = Float::isNaN(f)?;
        let _t1: f32 = (1f32).abs();
        Ok(_t1)
    }

    // java: sinh(D)D
    pub fn sinh(x: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::sinh(x)?;
        Ok(_t0)
    }

    // java: cosh(D)D
    pub fn cosh(x: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::cosh(x)?;
        Ok(_t0)
    }

    // java: tanh(D)D
    pub fn tanh(x: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::tanh(x)?;
        Ok(_t0)
    }

    // java: hypot(DD)D
    pub fn hypot(x: f64, arg_1: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::hypot(x, local_2)?;
        Ok(_t0)
    }

    // java: expm1(D)D
    pub fn expm1(x: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::expm1(x)?;
        Ok(_t0)
    }

    // java: log1p(D)D
    pub fn log1p(x: f64) -> Result<f64> {
        let _t0: f64 = StrictMath::log1p(x)?;
        Ok(_t0)
    }

    // java: copySign(DD)D
    // java: copySign(DD)D
    pub fn copySign__d_d(magnitude: f64, arg_1: f64) -> Result<f64> {
        let _t0: i64 = Double::doubleToRawLongBits(local_2)?;
        /* TODO: land  */
        let _t1: i64 = Double::doubleToRawLongBits(magnitude)?;
        /* TODO: land  */
        /* TODO: lor  */
        let _t2: f64 = Double::longBitsToDouble(9223372036854775807i64)?;
        Ok(_t2)
    }

    // java: copySign(FF)F
    // java: copySign(FF)F
    pub fn copySign__f_f(magnitude: f32, sign: f32) -> Result<f32> {
        let _t0: i32 = Float::floatToRawIntBits(sign)?;
        let _t1: i32 = Float::floatToRawIntBits(magnitude)?;
        let _t2: f32 = Float::intBitsToFloat(((_t0&-2147483648i32)|(_t1&2147483647i32)))?;
        Ok(_t2)
    }

    // java: getExponent(F)I
    // java: getExponent(F)I
    pub fn getExponent__f(f: f32) -> Result<i32> {
        let _t0: i32 = Float::floatToRawIntBits(f)?;
        Ok((((_t0&2139095040i32)>>((23i32&0x1f)))).wrapping_sub(127i32))
    }

    // java: getExponent(D)I
    // java: getExponent(D)I
    pub fn getExponent__d(d: f64) -> Result<i32> {
        let _t0: i64 = Double::doubleToRawLongBits(d)?;
        /* TODO: land  */
        /* TODO: lshr  */
        Ok(((52i32).wrapping_sub(1023i64) as i32))
    }

    // java: nextAfter(DD)D
    // java: nextAfter(DD)D
    pub fn nextAfter__d_d(start: f64, arg_1: f64) -> Result<f64> {
        /* TODO: dcmpl  */
        /* TODO: dcmpl  */
        let _t0: i64 = Double::doubleToRawLongBits(start)?;
        let mut transducer: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: f64 = Double::longBitsToDouble((18446744073709551615i64).wrapping_add(1i64))?;
        return Ok(_t1);
        return Ok(-5e-324f64);
        /* TODO: dcmpg  */
        let _t2: i64 = Double::doubleToRawLongBits((start+0f64))?;
        transducer = _t2;
        /* TODO: lcmp  */
        let _t3: f64 = Double::longBitsToDouble((1i64).wrapping_add(18446744073709551615i64))?;
        return Ok(_t3);
        /* TODO: dcmpl  */
        return Ok(local_2);
        Ok((start+local_2))
    }

    // java: nextAfter(FD)F
    // java: nextAfter(FD)F
    pub fn nextAfter__f_d(start: f32, direction: f64) -> Result<f32> {
        /* TODO: dcmpl  */
        /* TODO: fcmpl  */
        let _t0: i32 = Float::floatToRawIntBits(start)?;
        let mut transducer: i32 = _t0;
        let _t1: f32 = Float::intBitsToFloat((transducer).wrapping_add(transducer<=0i32))?;
        return Ok(_t1);
        return Ok(326i32);
        /* TODO: dcmpg  */
        let _t2: i32 = Float::floatToRawIntBits((start+0f32))?;
        transducer = _t2;
        let _t3: f32 = Float::intBitsToFloat((transducer).wrapping_add(transducer<0i32))?;
        return Ok(_t3);
        /* TODO: dcmpl  */
        return Ok((direction as f32));
        Ok((start+(direction as f32)))
    }

    // java: nextUp(D)D
    // java: nextUp(D)D
    pub fn nextUp__d(d: f64) -> Result<f64> {
        /* TODO: dcmpg  */
        let _t0: i64 = Double::doubleToRawLongBits((d+0f64))?;
        let mut transducer: i64 = _t0;
        /* TODO: lcmp  */
        let _t1: f64 = Double::longBitsToDouble((1i64).wrapping_add(18446744073709551615i64))?;
        return Ok(_t1);
        Ok(d)
    }

    // java: nextUp(F)F
    // java: nextUp(F)F
    pub fn nextUp__f(f: f32) -> Result<f32> {
        /* TODO: fcmpg  */
        let _t0: i32 = Float::floatToRawIntBits((f+0f32))?;
        let mut transducer: i32 = _t0;
        let _t1: f32 = Float::intBitsToFloat((transducer).wrapping_add(transducer<0i32))?;
        return Ok(_t1);
        Ok(f)
    }

    // java: nextDown(D)D
    // java: nextDown(D)D
    pub fn nextDown__d(d: f64) -> Result<f64> {
        let _t0: bool = Double::isNaN(d)?;
        /* TODO: dcmpl  */
        return Ok(d);
        /* TODO: dcmpl  */
        return Ok(-5e-324f64);
        let _t1: i64 = Double::doubleToRawLongBits(d)?;
        /* TODO: dcmpl  */
        let _t2: f64 = Double::longBitsToDouble((18446744073709551615i64).wrapping_add(1i64))?;
        Ok(_t2)
    }

    // java: nextDown(F)F
    // java: nextDown(F)F
    pub fn nextDown__f(f: f32) -> Result<f32> {
        let _t0: bool = Float::isNaN(f)?;
        /* TODO: fcmpl  */
        return Ok(f);
        /* TODO: fcmpl  */
        return Ok(326i32);
        let _t1: i32 = Float::floatToRawIntBits(f)?;
        /* TODO: fcmpl  */
        let _t2: f32 = Float::intBitsToFloat((f).wrapping_add(0f32<=0i32))?;
        Ok(_t2)
    }

    // java: scalb(DI)D
    // java: scalb(DI)D
    pub fn scalb__d_i(d: f64, arg_1: i32) -> Result<f64> {
        let mut MAX_SCALE: i32 = 2099i32;
        let mut exp_adjust: i32 = 0i32;
        let mut scale_increment: i32 = 0i32;
        let mut exp_delta: f64 = nanf64;
        let _t0: i32 = (local_2).max(-2099i32);
        let mut scaleFactor: i32 = _t0;
        scale_increment = -512i32;
        exp_delta = Math::twoToTheDoubleScaleDown();
        let _t1: i32 = (scaleFactor).min(2099i32);
        scaleFactor = _t1;
        scale_increment = 512i32;
        exp_delta = Math::twoToTheDoubleScaleUp();
        let mut t: i32 = (((scaleFactor>>((8i32&0x1f))) as u32>>(23i32&0x1f)) as i32);
        exp_adjust = (((scaleFactor).wrapping_add(t)&511i32)).wrapping_sub(t);
        let _t2: f64 = (exp_adjust).abs();
        d = (d*_t2);
        scaleFactor = (scaleFactor).wrapping_sub(exp_adjust);
        loop {
            if scaleFactor==0i32 { break; }
            d = (d*exp_delta);
            scaleFactor = (scaleFactor).wrapping_sub(scale_increment);
        }
        Ok(d)
    }

    // java: scalb(FI)F
    // java: scalb(FI)F
    pub fn scalb__f_i(f: f32, scaleFactor: i32) -> Result<f32> {
        let mut MAX_SCALE: i32 = 278i32;
        let _t0: i32 = (scaleFactor).min(278i32);
        let _t1: i32 = (_t0).max(-278i32);
        scaleFactor = _t1;
        let _t2: f64 = (scaleFactor).abs();
        Ok((((f as f64)*_t2) as f32))
    }

    // java: powerOfTwoD(I)D
    pub fn powerOfTwoD(n: i32) -> Result<f64> {
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: lshl  */
        /* TODO: land  */
        let _t0: f64 = Double::longBitsToDouble(9218868437227405312i64)?;
        Ok(_t0)
    }

    // java: powerOfTwoF(I)F
    pub fn powerOfTwoF(n: i32) -> Result<f32> {
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0: f32 = Float::intBitsToFloat((((n).wrapping_add(127i32)<<(23i32&0x1f))&2139095040i32))?;
        Ok(_t0)
    }
}
