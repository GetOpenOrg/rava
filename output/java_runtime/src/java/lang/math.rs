#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::r#ref::*;
use crate::java::lang::reflect::*;
use crate::java::math::*;
use crate::java::nio::*;
use crate::java::nio::charset::*;
use crate::java::security::*;
use crate::java::text::*;
use crate::java::text::spi::*;
use crate::java::time::*;
use crate::java::time::chrono::*;
use crate::java::time::temporal::*;
use crate::java::time::zone::*;
use crate::java::util::*;
use crate::java::util::concurrent::*;
use crate::java::util::concurrent::atomic::*;
use crate::java::util::concurrent::locks::*;
use crate::java::util::function::*;
use crate::java::util::regex::*;
use crate::java::util::spi::*;
use crate::java::util::stream::*;
use crate::java::util::zip::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::reflect::generics::factory::*;
use crate::sun::reflect::generics::repository::*;
use crate::sun::reflect::generics::scope::*;
use crate::sun::reflect::misc::*;
use crate::sun::security::action::*;
use crate::sun::security::util::*;
use crate::sun::text::*;
use crate::sun::util::*;
use crate::sun::util::calendar::*;
use crate::sun::util::locale::*;
use crate::sun::util::locale::provider::*;
use crate::sun::util::spi::*;
use crate::java::text::Normalizer;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Math"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Math.java"]
    #[inner_classes     = "java/lang/Math$RandomNumberGeneratorHolder:java/lang/Math:RandomNumberGeneratorHolder:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Math;java/lang/Object"]

    pub struct Math;

    impl Math {
        #[cfg_attr(any(), java_field(name = "E", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "2.718281828459045"))]
        // static field: E:D
        pub fn E() -> f64 {
            2.718281828459045f64
        }

        #[cfg_attr(any(), java_field(name = "PI", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "3.141592653589793"))]
        // static field: PI:D
        pub fn PI() -> f64 {
            3.141592653589793f64
        }

        #[cfg_attr(any(), java_field(name = "TAU", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "6.283185307179586"))]
        // static field: TAU:D
        pub fn TAU() -> f64 {
            6.283185307179586f64
        }

        #[cfg_attr(any(), java_field(name = "DEGREES_TO_RADIANS", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "0.017453292519943295"))]
        // static field: DEGREES_TO_RADIANS:D
        pub fn DEGREES_TO_RADIANS() -> f64 {
            0.017453292519943295f64
        }

        #[cfg_attr(any(), java_field(name = "RADIANS_TO_DEGREES", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "57.29577951308232"))]
        // static field: RADIANS_TO_DEGREES:D
        pub fn RADIANS_TO_DEGREES() -> f64 {
            57.29577951308232f64
        }

        #[cfg_attr(any(), java_field(name = "negativeZeroFloatBits", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: negativeZeroFloatBits:J
        pub fn negativeZeroFloatBits() -> i64 {
            panic!("stub: java/lang/Math.negativeZeroFloatBits:J")
        }

        #[cfg_attr(any(), java_field(name = "negativeZeroDoubleBits", descriptor = "J", access = "private", modifiers = "static final", is_static = true))]
        // static field: negativeZeroDoubleBits:J
        pub fn negativeZeroDoubleBits() -> i64 {
            panic!("stub: java/lang/Math.negativeZeroDoubleBits:J")
        }

        #[cfg_attr(any(), java_field(name = "twoToTheDoubleScaleUp", descriptor = "D", access = "package", modifiers = "static", is_static = true))]
        // static field: twoToTheDoubleScaleUp:D
        pub fn twoToTheDoubleScaleUp() -> f64 {
            panic!("stub: java/lang/Math.twoToTheDoubleScaleUp:D")
        }

        #[cfg_attr(any(), java_field(name = "twoToTheDoubleScaleDown", descriptor = "D", access = "package", modifiers = "static", is_static = true))]
        // static field: twoToTheDoubleScaleDown:D
        pub fn twoToTheDoubleScaleDown() -> f64 {
            panic!("stub: java/lang/Math.twoToTheDoubleScaleDown:D")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/Math.<init>:()V")
        }

        #[java_method(name = "round", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round_f(a: f32) -> Result<i32> {
            panic!("stub: java/lang/Math.round:(F)I")
        }

        #[java_method(name = "round", descriptor = "(D)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: round(D)J
        pub fn round_d(mut a: f64) -> Result<i64> {
            let _t0: i64 = Double::doubleToRawLongBits(a)?;
            let mut longBits: i64 = _t0;
            let mut biasedExp = ((longBits&(9218868437227405312i64))).wrapping_shr((52i32&0x3f) as u32);
            let mut shift = (1074i64).wrapping_sub(biasedExp);
            let mut r = ((longBits&(4503599627370495i64))|(4503599627370496i64));
            if (((longBits>(0i64)) as i32-((longBits)<(0i64)) as i32)<0) {
                r = (r).wrapping_neg();
            }
            return Ok((((r).wrapping_shr(((shift as i32)&0x3f) as u32)).wrapping_add(1i64)).wrapping_shr((1i32&0x3f) as u32));
            Ok((a as i64))
        }

        #[java_method(name = "addExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: addExact(II)I
        pub fn addExact_i_i(mut x: i32, mut y: i32) -> Result<i32> {
            let mut r = (x).wrapping_add(y);
            if (((x^r)&(y^r))<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(r)
        }

        #[java_method(name = "addExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: addExact(JJ)J
        pub fn addExact_l_l(mut x: i64, mut y: i64) -> Result<i64> {
            let mut r = (x).wrapping_add(y);
            if (((((x)^(r)&((y)^(r)))>(0i64)) as i32-((((x)^(r)&((y)^(r))))<(0i64)) as i32)<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(r)
        }

        #[java_method(name = "subtractExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtractExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.subtractExact:(II)I")
        }

        #[java_method(name = "subtractExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtractExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.subtractExact:(JJ)J")
        }

        #[java_method(name = "multiplyExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: multiplyExact(II)I
        pub fn multiplyExact_i_i(mut x: i32, mut y: i32) -> Result<i32> {
            let mut r = ((x as i64)).wrapping_mul((y as i64));
            if (((((r as i32) as i64)>(r)) as i32-((((r as i32) as i64))<(r)) as i32)!=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok((r as i32))
        }

        #[java_method(name = "multiplyExact", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyExact_l_i(x: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/Math.multiplyExact:(JI)J")
        }

        #[java_method(name = "multiplyExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.multiplyExact:(JJ)J")
        }

        #[java_method(name = "divideExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.divideExact:(II)I")
        }

        #[java_method(name = "divideExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.divideExact:(JJ)J")
        }

        #[java_method(name = "floorDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDivExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.floorDivExact:(II)I")
        }

        #[java_method(name = "floorDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.floorDivExact:(JJ)J")
        }

        #[java_method(name = "ceilDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDivExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.ceilDivExact:(II)I")
        }

        #[java_method(name = "ceilDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.ceilDivExact:(JJ)J")
        }

        #[java_method(name = "incrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.incrementExact:(I)I")
        }

        #[java_method(name = "incrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.incrementExact:(J)J")
        }

        #[java_method(name = "decrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.decrementExact:(I)I")
        }

        #[java_method(name = "decrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.decrementExact:(J)J")
        }

        #[java_method(name = "negateExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negateExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.negateExact:(I)I")
        }

        #[java_method(name = "negateExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negateExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.negateExact:(J)J")
        }

        #[java_method(name = "toIntExact", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toIntExact(value: i64) -> Result<i32> {
            panic!("stub: java/lang/Math.toIntExact:(J)I")
        }

        #[java_method(name = "multiplyFull", descriptor = "(II)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyFull(x: i32, y: i32) -> Result<i64> {
            panic!("stub: java/lang/Math.multiplyFull:(II)J")
        }

        #[java_method(name = "multiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyHigh(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.multiplyHigh:(JJ)J")
        }

        #[java_method(name = "unsignedMultiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unsignedMultiplyHigh(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.unsignedMultiplyHigh:(JJ)J")
        }

        #[java_method(name = "floorDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDiv_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.floorDiv:(II)I")
        }

        #[java_method(name = "floorDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: floorDiv(JI)J
        pub fn floorDiv_l_i(mut x: i64, mut y: i32) -> Result<i64> {
            let _t0: i64 = Math::floorDiv_l_l(x, (y as i64))?;
            Ok(_t0)
        }

        #[java_method(name = "floorDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: floorDiv(JJ)J
        pub fn floorDiv_l_l(mut x: i64, mut y: i64) -> Result<i64> {
            let mut q = (x/y);
            if ((((q).wrapping_mul(y)>(x)) as i32-(((q).wrapping_mul(y))<(x)) as i32)!=0) {
                return Ok((q).wrapping_sub(1i64));
            }
            Ok(q)
        }

        #[java_method(name = "floorMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: floorMod(II)I
        pub fn floorMod_i_i(mut x: i32, mut y: i32) -> Result<i32> {
            let mut r = (x%y);
            if (r!=0) {
                return Ok((r).wrapping_add(y));
            }
            Ok(r)
        }

        #[java_method(name = "floorMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorMod_l_i(x: i64, arg1: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.floorMod:(JI)I")
        }

        #[java_method(name = "floorMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: floorMod(JJ)J
        pub fn floorMod_l_l(mut x: i64, mut y: i64) -> Result<i64> {
            let mut r = (x%(y));
            if (((r>(0i64)) as i32-((r)<(0i64)) as i32)!=0) {
                return Ok((r).wrapping_add(y));
            }
            Ok(r)
        }

        #[java_method(name = "ceilDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.ceilDiv:(II)I")
        }

        #[java_method(name = "ceilDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_l_i(x: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/Math.ceilDiv:(JI)J")
        }

        #[java_method(name = "ceilDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.ceilDiv:(JJ)J")
        }

        #[java_method(name = "ceilMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.ceilMod:(II)I")
        }

        #[java_method(name = "ceilMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_l_i(x: i64, arg1: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.ceilMod:(JI)I")
        }

        #[java_method(name = "ceilMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.ceilMod:(JJ)J")
        }

        #[java_method(name = "abs", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.abs:(I)I")
        }

        #[java_method(name = "absExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn absExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/Math.absExact:(I)I")
        }

        #[java_method(name = "abs", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: abs(J)J
        pub fn abs_l(mut a: i64) -> Result<i64> {
            Ok((if (((a>(0i64)) as i32-((a)<(0i64)) as i32)<0) { (a).wrapping_neg() } else { a }))
        }

        #[java_method(name = "absExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn absExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.absExact:(J)J")
        }

        #[java_method(name = "abs", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_f(a: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.abs:(F)F")
        }

        #[java_method(name = "abs", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: abs(D)D
        pub fn abs_d(mut a: f64) -> Result<f64> {
            let _t0: i64 = Double::doubleToRawLongBits(a)?;
            let _t1: f64 = Double::longBitsToDouble((_t0&(9223372036854775807i64)))?;
            Ok(_t1)
        }

        #[java_method(name = "max", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: max(II)I
        pub fn max_i_i(mut a: i32, mut b: i32) -> Result<i32> {
            Ok((if a >= b { a } else { b }))
        }

        #[java_method(name = "max", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: max(JJ)J
        pub fn max_l_l(mut a: i64, mut b: i64) -> Result<i64> {
            Ok((if (((a>(b)) as i32-((a)<(b)) as i32)>=0) { a } else { b }))
        }

        #[java_method(name = "max", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_f_f(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.max:(FF)F")
        }

        #[java_method(name = "max", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_d_d(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.max:(DD)D")
        }

        #[java_method(name = "min", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: min(II)I
        pub fn min_i_i(mut a: i32, mut b: i32) -> Result<i32> {
            Ok((if a <= b { a } else { b }))
        }

        #[java_method(name = "min", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: min(JJ)J
        pub fn min_l_l(mut a: i64, mut b: i64) -> Result<i64> {
            Ok((if (((a>(b)) as i32-((a)<(b)) as i32)<=0) { a } else { b }))
        }

        #[java_method(name = "min", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_f_f(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.min:(FF)F")
        }

        #[java_method(name = "min", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_d_d(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.min:(DD)D")
        }

        #[java_method(name = "clamp", descriptor = "(JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: clamp(JII)I
        pub fn clamp_l_i_i(mut value: i64, mut min: i32, mut max: i32) -> Result<i32> {
            if min > max {
                let _t0 = StringBuilder::new()?.append_i(min)?;
                let _t1 = _t0.append_str(Clone::clone(&String::from(" > ")))?;
                let _t2 = _t1.append_i(max)?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: i64 = Math::max_l_l(value, (min as i64))?;
            let _t1: i64 = Math::min_l_l((max as i64), _t0)?;
            Ok((_t1 as i32))
        }

        #[java_method(name = "clamp", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_l_l_l(value: i64, arg1: i64, min: i64) -> Result<i64> {
            panic!("stub: java/lang/Math.clamp:(JJJ)J")
        }

        #[java_method(name = "clamp", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_d_d_d(value: f64, arg1: f64, min: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.clamp:(DDD)D")
        }

        #[java_method(name = "clamp", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_f_f_f(value: f32, min: f32, max: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.clamp:(FFF)F")
        }

        #[java_method(name = "fma", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fma_d_d_d(a: f64, arg1: f64, b: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.fma:(DDD)D")
        }

        #[java_method(name = "fma", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fma_f_f_f(a: f32, b: f32, c: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.fma:(FFF)F")
        }

        #[java_method(name = "ulp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ulp_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.ulp:(D)D")
        }

        #[java_method(name = "ulp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ulp_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.ulp:(F)F")
        }

        #[java_method(name = "signum", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.signum:(D)D")
        }

        #[java_method(name = "signum", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.signum:(F)F")
        }

        #[java_method(name = "copySign", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: copySign(DD)D
        pub fn copySign_d_d(mut magnitude: f64, mut sign: f64) -> Result<f64> {
            let _t0: i64 = Double::doubleToRawLongBits(sign)?;
            let _t1: i64 = Double::doubleToRawLongBits(magnitude)?;
            let _t2: f64 = Double::longBitsToDouble(((_t0&(-9223372036854775808i64))|((_t1&(9223372036854775807i64)))))?;
            Ok(_t2)
        }

        #[java_method(name = "copySign", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySign_f_f(magnitude: f32, sign: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.copySign:(FF)F")
        }

        #[java_method(name = "getExponent", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponent_f(f: f32) -> Result<i32> {
            panic!("stub: java/lang/Math.getExponent:(F)I")
        }

        #[java_method(name = "getExponent", descriptor = "(D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getExponent(D)I
        pub fn getExponent_d(mut d: f64) -> Result<i32> {
            let _t0: i64 = Double::doubleToRawLongBits(d)?;
            Ok(((((_t0&(9218868437227405312i64))).wrapping_shr((52i32&0x3f) as u32)).wrapping_sub(1023i64) as i32))
        }

        #[java_method(name = "nextAfter", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextAfter_d_d(start: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.nextAfter:(DD)D")
        }

        #[java_method(name = "nextAfter", descriptor = "(FD)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextAfter_f_d(start: f32, direction: f64) -> Result<f32> {
            panic!("stub: java/lang/Math.nextAfter:(FD)F")
        }

        #[java_method(name = "nextUp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextUp_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.nextUp:(D)D")
        }

        #[java_method(name = "nextUp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextUp_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.nextUp:(F)F")
        }

        #[java_method(name = "nextDown", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDown_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/Math.nextDown:(D)D")
        }

        #[java_method(name = "nextDown", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDown_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/Math.nextDown:(F)F")
        }

        #[java_method(name = "scalb", descriptor = "(DI)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scalb_d_i(d: f64, arg1: i32) -> Result<f64> {
            panic!("stub: java/lang/Math.scalb:(DI)D")
        }

        #[java_method(name = "scalb", descriptor = "(FI)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scalb_f_i(f: f32, scaleFactor: i32) -> Result<f32> {
            panic!("stub: java/lang/Math.scalb:(FI)F")
        }

        #[java_method(name = "powerOfTwoD", descriptor = "(I)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn powerOfTwoD(n: i32) -> Result<f64> {
            panic!("stub: java/lang/Math.powerOfTwoD:(I)D")
        }

        #[java_method(name = "powerOfTwoF", descriptor = "(I)F", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn powerOfTwoF(n: i32) -> Result<f32> {
            panic!("stub: java/lang/Math.powerOfTwoF:(I)F")
        }
    }
}
