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
    #[binary_name       = "java/lang/FdLibm"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "FdLibm.java"]
    #[inner_classes     = "java/lang/FdLibm$IEEEremainder:java/lang/FdLibm:IEEEremainder:24;java/lang/FdLibm$Tanh:java/lang/FdLibm:Tanh:24;java/lang/FdLibm$Cosh:java/lang/FdLibm:Cosh:24;java/lang/FdLibm$Sinh:java/lang/FdLibm:Sinh:24;java/lang/FdLibm$Expm1:java/lang/FdLibm:Expm1:24;java/lang/FdLibm$Log1p:java/lang/FdLibm:Log1p:24;java/lang/FdLibm$Log10:java/lang/FdLibm:Log10:24;java/lang/FdLibm$Log:java/lang/FdLibm:Log:24;java/lang/FdLibm$Exp:java/lang/FdLibm:Exp:24;java/lang/FdLibm$Pow:java/lang/FdLibm:Pow:24;java/lang/FdLibm$Hypot:java/lang/FdLibm:Hypot:24;java/lang/FdLibm$Cbrt:java/lang/FdLibm:Cbrt:24;java/lang/FdLibm$Sqrt:java/lang/FdLibm:Sqrt:24;java/lang/FdLibm$Atan2:java/lang/FdLibm:Atan2:24;java/lang/FdLibm$Atan:java/lang/FdLibm:Atan:24;java/lang/FdLibm$Acos:java/lang/FdLibm:Acos:24;java/lang/FdLibm$Asin:java/lang/FdLibm:Asin:24;java/lang/FdLibm$KernelRemPio2:java/lang/FdLibm:KernelRemPio2:24;java/lang/FdLibm$RemPio2:java/lang/FdLibm:RemPio2:24;java/lang/FdLibm$Tan:java/lang/FdLibm:Tan:24;java/lang/FdLibm$Cos:java/lang/FdLibm:Cos:24;java/lang/FdLibm$Sin:java/lang/FdLibm:Sin:24"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/FdLibm;java/lang/Object"]

    pub struct FdLibm;

    impl FdLibm {
        #[cfg_attr(any(), java_field(name = "INFINITY", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "inf"))]
        // static field: INFINITY:D
        pub fn INFINITY() -> f64 {
            f64::INFINITY
        }

        #[cfg_attr(any(), java_field(name = "TWO24", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "16777216.0"))]
        // static field: TWO24:D
        pub fn TWO24() -> f64 {
            16777216.0f64
        }

        #[cfg_attr(any(), java_field(name = "TWO54", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "1.8014398509481984e+16"))]
        // static field: TWO54:D
        pub fn TWO54() -> f64 {
            1.8014398509481984e+16f64
        }

        #[cfg_attr(any(), java_field(name = "HUGE", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "1e+300"))]
        // static field: HUGE:D
        pub fn HUGE() -> f64 {
            1e+300f64
        }

        #[cfg_attr(any(), java_field(name = "SIGN_BIT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "-2147483648"))]
        // static field: SIGN_BIT:I
        pub fn SIGN_BIT() -> i32 {
            -2147483648
        }

        #[cfg_attr(any(), java_field(name = "EXP_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2146435072"))]
        // static field: EXP_BITS:I
        pub fn EXP_BITS() -> i32 {
            2146435072
        }

        #[cfg_attr(any(), java_field(name = "EXP_SIGNIF_BITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: EXP_SIGNIF_BITS:I
        pub fn EXP_SIGNIF_BITS() -> i32 {
            2147483647
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/lang/FdLibm.<init>:()V")
        }

        #[java_method(name = "__LO", descriptor = "(D)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: __LO(D)I
        pub fn __LO_d(mut x: f64) -> Result<i32> {
            let _t0: i64 = Double::doubleToRawLongBits(x)?;
            let mut transducer: i64 = _t0;
            Ok((transducer as i32))
        }

        #[java_method(name = "__LO", descriptor = "(DI)D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn __LO_d_i(x: f64, arg1: i32) -> Result<f64> {
            panic!("stub: java/lang/FdLibm.__LO:(DI)D")
        }

        #[java_method(name = "__HI", descriptor = "(D)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: __HI(D)I
        pub fn __HI_d(mut x: f64) -> Result<i32> {
            let _t0: i64 = Double::doubleToRawLongBits(x)?;
            let mut transducer: i64 = _t0;
            Ok(((transducer).wrapping_shr((32i32&0x3f) as u32) as i32))
        }

        #[java_method(name = "__HI", descriptor = "(DI)D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: __HI(DI)D
        pub fn __HI_d_i(mut x: f64, mut high: i32) -> Result<f64> {
            let _t0: i64 = Double::doubleToRawLongBits(x)?;
            let mut transX: i64 = _t0;
            let _t1: f64 = Double::longBitsToDouble(((transX&(4294967295i64))|(((high as i64)).wrapping_shl((32i32&0x3f) as u32))))?;
            Ok(_t1)
        }

        #[java_method(name = "__HI_LO", descriptor = "(II)D", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn __HI_LO(mut high: i32, mut low: i32) -> Result<f64> {
            let _t0: f64 = Double::longBitsToDouble((((high as i64)).wrapping_shl((32i32&0x3f) as u32)|(((low as i64)&(4294967295i64)))))?;
            Ok(_t0)
        }
    }
}
