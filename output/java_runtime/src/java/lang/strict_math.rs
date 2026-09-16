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
    #[binary_name       = "java/lang/StrictMath"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "StrictMath.java"]
    #[inner_classes     = "java/lang/FdLibm$Sin:java/lang/FdLibm:Sin:24;java/lang/FdLibm$Cos:java/lang/FdLibm:Cos:24;java/lang/FdLibm$Tan:java/lang/FdLibm:Tan:24;java/lang/FdLibm$Asin:java/lang/FdLibm:Asin:24;java/lang/FdLibm$Acos:java/lang/FdLibm:Acos:24;java/lang/FdLibm$Atan:java/lang/FdLibm:Atan:24;java/lang/FdLibm$Exp:java/lang/FdLibm:Exp:24;java/lang/FdLibm$Log:java/lang/FdLibm:Log:24;java/lang/FdLibm$Log10:java/lang/FdLibm:Log10:24;java/lang/FdLibm$Sqrt:java/lang/FdLibm:Sqrt:24;java/lang/FdLibm$Cbrt:java/lang/FdLibm:Cbrt:24;java/lang/FdLibm$IEEEremainder:java/lang/FdLibm:IEEEremainder:24;java/lang/FdLibm$Atan2:java/lang/FdLibm:Atan2:24;java/lang/FdLibm$Pow:java/lang/FdLibm:Pow:24;java/lang/StrictMath$RandomNumberGeneratorHolder:java/lang/StrictMath:RandomNumberGeneratorHolder:26;java/lang/FdLibm$Sinh:java/lang/FdLibm:Sinh:24;java/lang/FdLibm$Cosh:java/lang/FdLibm:Cosh:24;java/lang/FdLibm$Tanh:java/lang/FdLibm:Tanh:24;java/lang/FdLibm$Hypot:java/lang/FdLibm:Hypot:24;java/lang/FdLibm$Expm1:java/lang/FdLibm:Expm1:24;java/lang/FdLibm$Log1p:java/lang/FdLibm:Log1p:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/lang/StrictMath"]

    pub struct StrictMath;

    impl StrictMath {
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

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/StrictMath.<init>:()V")
        }

        #[java_method(name = "sin", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sin(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.sin:(D)D")
        }

        #[java_method(name = "cos", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cos(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.cos:(D)D")
        }

        #[java_method(name = "tan", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tan(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.tan:(D)D")
        }

        #[java_method(name = "asin", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn asin(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.asin:(D)D")
        }

        #[java_method(name = "acos", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn acos(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.acos:(D)D")
        }

        #[java_method(name = "atan", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atan(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.atan:(D)D")
        }

        #[java_method(name = "toRadians", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toRadians(angdeg: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.toRadians:(D)D")
        }

        #[java_method(name = "toDegrees", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toDegrees(angrad: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.toDegrees:(D)D")
        }

        #[java_method(name = "exp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exp(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.exp:(D)D")
        }

        #[java_method(name = "log", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn log(mut a: f64) -> Result<f64> {
            let _t0: f64 = FdLibm_Log::compute(a)?;
            Ok(_t0)
        }

        #[java_method(name = "log10", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn log10(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.log10:(D)D")
        }

        #[java_method(name = "sqrt", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sqrt(mut a: f64) -> Result<f64> {
            let _t0: f64 = FdLibm_Sqrt::compute(a)?;
            Ok(_t0)
        }

        #[java_method(name = "cbrt", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cbrt(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.cbrt:(D)D")
        }

        #[java_method(name = "IEEEremainder", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn IEEEremainder(f1: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.IEEEremainder:(DD)D")
        }

        #[java_method(name = "ceil", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceil(mut a: f64) -> Result<f64> {
            let _t0: f64 = StrictMath::floorOrCeil(a, -0.0f64, 1f64, 1f64)?;
            Ok(_t0)
        }

        #[java_method(name = "floor", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floor(mut a: f64) -> Result<f64> {
            let _t0: f64 = StrictMath::floorOrCeil(a, -1.0f64, 0f64, -1.0f64)?;
            Ok(_t0)
        }

        #[java_method(name = "floorOrCeil", descriptor = "(DDDD)D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorOrCeil(mut a: f64, mut negativeBoundary: f64, mut positiveBoundary: f64, mut sign: f64) -> Result<f64> {
            let _t0: i32 = Math::getExponent_d(a)?;
            let mut exponent: i32 = _t0;
            return Ok((if (((a>(0f64)) as i32-((a)<(0f64)) as i32)==0) { a } else { (if (((a>(0f64)) as i32-((a)<(0f64)) as i32)<0) { negativeBoundary } else { positiveBoundary }) }));
            if exponent >= 52i32 {
                return Ok(a);
            }
            if exponent > 51i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: i64 = Double::doubleToRawLongBits(a)?;
            let mut doppel: i64 = _t1;
            let mut mask = (4503599627370495i64).wrapping_shr((exponent&0x3f) as u32);
            if ((((mask&(doppel))>(0i64)) as i32-(((mask&(doppel)))<(0i64)) as i32)==0) {
                return Ok(a);
            }
            let _t2: f64 = Double::longBitsToDouble((doppel&((mask)^(-1i64))))?;
            let mut result: f64 = _t2;
            if ((((sign*a)>(0f64)) as i32-(((sign*a))<(0f64)) as i32)>0) {
                result = (result+sign);
            }
            Ok(result)
        }

        #[java_method(name = "rint", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rint(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.rint:(D)D")
        }

        #[java_method(name = "atan2", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atan2(y: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.atan2:(DD)D")
        }

        #[java_method(name = "pow", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pow(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.pow:(DD)D")
        }

        #[java_method(name = "round", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round_f(a: f32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.round:(F)I")
        }

        #[java_method(name = "round", descriptor = "(D)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn round_d(a: f64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.round:(D)J")
        }

        #[java_method(name = "random", descriptor = "()D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn random() -> Result<f64> {
            panic!("stub: java/lang/StrictMath.random:()D")
        }

        #[java_method(name = "addExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.addExact:(II)I")
        }

        #[java_method(name = "addExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.addExact:(JJ)J")
        }

        #[java_method(name = "subtractExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtractExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.subtractExact:(II)I")
        }

        #[java_method(name = "subtractExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subtractExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.subtractExact:(JJ)J")
        }

        #[java_method(name = "multiplyExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.multiplyExact:(II)I")
        }

        #[java_method(name = "multiplyExact", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyExact_l_i(x: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.multiplyExact:(JI)J")
        }

        #[java_method(name = "multiplyExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.multiplyExact:(JJ)J")
        }

        #[java_method(name = "divideExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.divideExact:(II)I")
        }

        #[java_method(name = "divideExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn divideExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.divideExact:(JJ)J")
        }

        #[java_method(name = "floorDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDivExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.floorDivExact:(II)I")
        }

        #[java_method(name = "floorDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.floorDivExact:(JJ)J")
        }

        #[java_method(name = "ceilDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDivExact_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.ceilDivExact:(II)I")
        }

        #[java_method(name = "ceilDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.ceilDivExact:(JJ)J")
        }

        #[java_method(name = "incrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.incrementExact:(I)I")
        }

        #[java_method(name = "incrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn incrementExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.incrementExact:(J)J")
        }

        #[java_method(name = "decrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.decrementExact:(I)I")
        }

        #[java_method(name = "decrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn decrementExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.decrementExact:(J)J")
        }

        #[java_method(name = "negateExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negateExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.negateExact:(I)I")
        }

        #[java_method(name = "negateExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn negateExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.negateExact:(J)J")
        }

        #[java_method(name = "toIntExact", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toIntExact(value: i64) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.toIntExact:(J)I")
        }

        #[java_method(name = "multiplyFull", descriptor = "(II)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyFull(x: i32, y: i32) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.multiplyFull:(II)J")
        }

        #[java_method(name = "multiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn multiplyHigh(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.multiplyHigh:(JJ)J")
        }

        #[java_method(name = "unsignedMultiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn unsignedMultiplyHigh(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.unsignedMultiplyHigh:(JJ)J")
        }

        #[java_method(name = "floorDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDiv_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.floorDiv:(II)I")
        }

        #[java_method(name = "floorDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDiv_l_i(x: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.floorDiv:(JI)J")
        }

        #[java_method(name = "floorDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorDiv_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.floorDiv:(JJ)J")
        }

        #[java_method(name = "floorMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorMod_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.floorMod:(II)I")
        }

        #[java_method(name = "floorMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorMod_l_i(x: i64, arg1: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.floorMod:(JI)I")
        }

        #[java_method(name = "floorMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floorMod_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.floorMod:(JJ)J")
        }

        #[java_method(name = "ceilDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.ceilDiv:(II)I")
        }

        #[java_method(name = "ceilDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_l_i(x: i64, arg1: i32) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.ceilDiv:(JI)J")
        }

        #[java_method(name = "ceilDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilDiv_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.ceilDiv:(JJ)J")
        }

        #[java_method(name = "ceilMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_i_i(x: i32, y: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.ceilMod:(II)I")
        }

        #[java_method(name = "ceilMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_l_i(x: i64, arg1: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.ceilMod:(JI)I")
        }

        #[java_method(name = "ceilMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ceilMod_l_l(x: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.ceilMod:(JJ)J")
        }

        #[java_method(name = "abs", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.abs:(I)I")
        }

        #[java_method(name = "absExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn absExact_i(a: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.absExact:(I)I")
        }

        #[java_method(name = "abs", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.abs:(J)J")
        }

        #[java_method(name = "absExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn absExact_l(a: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.absExact:(J)J")
        }

        #[java_method(name = "abs", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_f(a: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.abs:(F)F")
        }

        #[java_method(name = "abs", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn abs_d(a: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.abs:(D)D")
        }

        #[java_method(name = "max", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_i_i(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.max:(II)I")
        }

        #[java_method(name = "max", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_l_l(a: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.max:(JJ)J")
        }

        #[java_method(name = "max", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_f_f(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.max:(FF)F")
        }

        #[java_method(name = "max", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max_d_d(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.max:(DD)D")
        }

        #[java_method(name = "min", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_i_i(a: i32, b: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.min:(II)I")
        }

        #[java_method(name = "min", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_l_l(a: i64, arg1: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.min:(JJ)J")
        }

        #[java_method(name = "min", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_f_f(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.min:(FF)F")
        }

        #[java_method(name = "min", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min_d_d(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.min:(DD)D")
        }

        #[java_method(name = "clamp", descriptor = "(JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_l_i_i(value: i64, arg1: i32, min: i32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.clamp:(JII)I")
        }

        #[java_method(name = "clamp", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_l_l_l(value: i64, arg1: i64, min: i64) -> Result<i64> {
            panic!("stub: java/lang/StrictMath.clamp:(JJJ)J")
        }

        #[java_method(name = "clamp", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_d_d_d(value: f64, arg1: f64, min: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.clamp:(DDD)D")
        }

        #[java_method(name = "clamp", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clamp_f_f_f(value: f32, min: f32, max: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.clamp:(FFF)F")
        }

        #[java_method(name = "fma", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fma_d_d_d(a: f64, arg1: f64, b: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.fma:(DDD)D")
        }

        #[java_method(name = "fma", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fma_f_f_f(a: f32, b: f32, c: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.fma:(FFF)F")
        }

        #[java_method(name = "ulp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ulp_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.ulp:(D)D")
        }

        #[java_method(name = "ulp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ulp_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.ulp:(F)F")
        }

        #[java_method(name = "signum", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.signum:(D)D")
        }

        #[java_method(name = "signum", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn signum_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.signum:(F)F")
        }

        #[java_method(name = "sinh", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sinh(x: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.sinh:(D)D")
        }

        #[java_method(name = "cosh", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn cosh(x: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.cosh:(D)D")
        }

        #[java_method(name = "tanh", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn tanh(x: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.tanh:(D)D")
        }

        #[java_method(name = "hypot", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hypot(x: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.hypot:(DD)D")
        }

        #[java_method(name = "expm1", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expm1(x: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.expm1:(D)D")
        }

        #[java_method(name = "log1p", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn log1p(x: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.log1p:(D)D")
        }

        #[java_method(name = "copySign", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySign_d_d(magnitude: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.copySign:(DD)D")
        }

        #[java_method(name = "copySign", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copySign_f_f(magnitude: f32, sign: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.copySign:(FF)F")
        }

        #[java_method(name = "getExponent", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponent_f(f: f32) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.getExponent:(F)I")
        }

        #[java_method(name = "getExponent", descriptor = "(D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponent_d(d: f64) -> Result<i32> {
            panic!("stub: java/lang/StrictMath.getExponent:(D)I")
        }

        #[java_method(name = "nextAfter", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextAfter_d_d(start: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.nextAfter:(DD)D")
        }

        #[java_method(name = "nextAfter", descriptor = "(FD)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextAfter_f_d(start: f32, direction: f64) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.nextAfter:(FD)F")
        }

        #[java_method(name = "nextUp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextUp_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.nextUp:(D)D")
        }

        #[java_method(name = "nextUp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextUp_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.nextUp:(F)F")
        }

        #[java_method(name = "nextDown", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDown_d(d: f64) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.nextDown:(D)D")
        }

        #[java_method(name = "nextDown", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextDown_f(f: f32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.nextDown:(F)F")
        }

        #[java_method(name = "scalb", descriptor = "(DI)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scalb_d_i(d: f64, arg1: i32) -> Result<f64> {
            panic!("stub: java/lang/StrictMath.scalb:(DI)D")
        }

        #[java_method(name = "scalb", descriptor = "(FI)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn scalb_f_i(f: f32, scaleFactor: i32) -> Result<f32> {
            panic!("stub: java/lang/StrictMath.scalb:(FI)F")
        }
    }
}
