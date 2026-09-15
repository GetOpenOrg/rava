#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]
use crate::prelude::*;
use crate::java::io::*;
use crate::java::lang::*;
use crate::java::lang::reflect::*;
use crate::java::security::*;
use crate::java::util::*;
use crate::sun::nio::ch::*;
use crate::sun::nio::cs::*;
use crate::sun::security::util::*;

#[java_rta_macros::java_class(
    binary_name       = "java/lang/Math",
    super_class       = "java/lang/Object",
    interfaces        = "",
    access            = "public",
    modifiers         = "final",
    generic_signature = "",
    is_interface      = false,
    is_abstract       = false,
    is_enum           = false,
    is_deprecated     = false,
    source            = "Math.java",
    inner_classes     = "java/lang/Math$RandomNumberGeneratorHolder:java/lang/Math:RandomNumberGeneratorHolder:26",
)]
#[derive(Clone, Default, PartialEq)]
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

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn new() -> Result<Self> {
        panic!("stub: java/lang/Math.<init>:()V")
    }

    #[cfg_attr(any(), java_method(name = "round", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn round_f(a: f32) -> Result<i32> {
        panic!("stub: java/lang/Math.round:(F)I")
    }

    #[cfg_attr(any(), java_method(name = "round", descriptor = "(D)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn round_d(a: f64) -> Result<i64> {
        panic!("stub: java/lang/Math.round:(D)J")
    }

    #[cfg_attr(any(), java_method(name = "addExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn addExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.addExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "addExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn addExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.addExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "subtractExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn subtractExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.subtractExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "subtractExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn subtractExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.subtractExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "multiplyExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn multiplyExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.multiplyExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "multiplyExact", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn multiplyExact_l_i(x: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Math.multiplyExact:(JI)J")
    }

    #[cfg_attr(any(), java_method(name = "multiplyExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn multiplyExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.multiplyExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "divideExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn divideExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.divideExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "divideExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn divideExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.divideExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "floorDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorDivExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.floorDivExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "floorDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.floorDivExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "ceilDivExact", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilDivExact_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.ceilDivExact:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "ceilDivExact", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilDivExact_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.ceilDivExact:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "incrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn incrementExact_i(a: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.incrementExact:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "incrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn incrementExact_l(a: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.incrementExact:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "decrementExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decrementExact_i(a: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.decrementExact:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "decrementExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn decrementExact_l(a: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.decrementExact:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "negateExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn negateExact_i(a: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.negateExact:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "negateExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn negateExact_l(a: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.negateExact:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "toIntExact", descriptor = "(J)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn toIntExact(value: i64) -> Result<i32> {
        panic!("stub: java/lang/Math.toIntExact:(J)I")
    }

    #[cfg_attr(any(), java_method(name = "multiplyFull", descriptor = "(II)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn multiplyFull(x: i32, y: i32) -> Result<i64> {
        panic!("stub: java/lang/Math.multiplyFull:(II)J")
    }

    #[cfg_attr(any(), java_method(name = "multiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn multiplyHigh(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.multiplyHigh:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "unsignedMultiplyHigh", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn unsignedMultiplyHigh(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.unsignedMultiplyHigh:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "floorDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorDiv_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.floorDiv:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "floorDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorDiv_l_i(x: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Math.floorDiv:(JI)J")
    }

    #[cfg_attr(any(), java_method(name = "floorDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorDiv_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.floorDiv:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "floorMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorMod_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.floorMod:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "floorMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorMod_l_i(x: i64, arg1: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.floorMod:(JI)I")
    }

    #[cfg_attr(any(), java_method(name = "floorMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn floorMod_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.floorMod:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "ceilDiv", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilDiv_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.ceilDiv:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "ceilDiv", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilDiv_l_i(x: i64, arg1: i32) -> Result<i64> {
        panic!("stub: java/lang/Math.ceilDiv:(JI)J")
    }

    #[cfg_attr(any(), java_method(name = "ceilDiv", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilDiv_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.ceilDiv:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "ceilMod", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilMod_i_i(x: i32, y: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.ceilMod:(II)I")
    }

    #[cfg_attr(any(), java_method(name = "ceilMod", descriptor = "(JI)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilMod_l_i(x: i64, arg1: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.ceilMod:(JI)I")
    }

    #[cfg_attr(any(), java_method(name = "ceilMod", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ceilMod_l_l(x: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.ceilMod:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn abs_i(a: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.abs:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "absExact", descriptor = "(I)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn absExact_i(a: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.absExact:(I)I")
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn abs_l(a: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.abs:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "absExact", descriptor = "(J)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn absExact_l(a: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.absExact:(J)J")
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn abs_f(a: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.abs:(F)F")
    }

    #[cfg_attr(any(), java_method(name = "abs", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn abs_d(a: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.abs:(D)D")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: max(II)I
    pub fn max_i_i(mut a: i32, mut b: i32) -> Result<i32> {
        Ok((if a >= b { a } else { b }))
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn max_l_l(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.max:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn max_f_f(a: f32, b: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.max:(FF)F")
    }

    #[cfg_attr(any(), java_method(name = "max", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn max_d_d(a: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.max:(DD)D")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(II)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    // java: min(II)I
    pub fn min_i_i(mut a: i32, mut b: i32) -> Result<i32> {
        Ok((if a <= b { a } else { b }))
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(JJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min_l_l(a: i64, arg1: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.min:(JJ)J")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min_f_f(a: f32, b: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.min:(FF)F")
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn min_d_d(a: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.min:(DD)D")
    }

    #[cfg_attr(any(), java_method(name = "clamp", descriptor = "(JII)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clamp_l_i_i(value: i64, arg1: i32, min: i32) -> Result<i32> {
        panic!("stub: java/lang/Math.clamp:(JII)I")
    }

    #[cfg_attr(any(), java_method(name = "clamp", descriptor = "(JJJ)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clamp_l_l_l(value: i64, arg1: i64, min: i64) -> Result<i64> {
        panic!("stub: java/lang/Math.clamp:(JJJ)J")
    }

    #[cfg_attr(any(), java_method(name = "clamp", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clamp_d_d_d(value: f64, arg1: f64, min: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.clamp:(DDD)D")
    }

    #[cfg_attr(any(), java_method(name = "clamp", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn clamp_f_f_f(value: f32, min: f32, max: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.clamp:(FFF)F")
    }

    #[cfg_attr(any(), java_method(name = "fma", descriptor = "(DDD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn fma_d_d_d(a: f64, arg1: f64, b: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.fma:(DDD)D")
    }

    #[cfg_attr(any(), java_method(name = "fma", descriptor = "(FFF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn fma_f_f_f(a: f32, b: f32, c: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.fma:(FFF)F")
    }

    #[cfg_attr(any(), java_method(name = "ulp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ulp_d(d: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.ulp:(D)D")
    }

    #[cfg_attr(any(), java_method(name = "ulp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn ulp_f(f: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.ulp:(F)F")
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signum_d(d: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.signum:(D)D")
    }

    #[cfg_attr(any(), java_method(name = "signum", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn signum_f(f: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.signum:(F)F")
    }

    #[cfg_attr(any(), java_method(name = "copySign", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn copySign_d_d(magnitude: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.copySign:(DD)D")
    }

    #[cfg_attr(any(), java_method(name = "copySign", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn copySign_f_f(magnitude: f32, sign: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.copySign:(FF)F")
    }

    #[cfg_attr(any(), java_method(name = "getExponent", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getExponent_f(f: f32) -> Result<i32> {
        panic!("stub: java/lang/Math.getExponent:(F)I")
    }

    #[cfg_attr(any(), java_method(name = "getExponent", descriptor = "(D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn getExponent_d(d: f64) -> Result<i32> {
        panic!("stub: java/lang/Math.getExponent:(D)I")
    }

    #[cfg_attr(any(), java_method(name = "nextAfter", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextAfter_d_d(start: f64, arg1: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.nextAfter:(DD)D")
    }

    #[cfg_attr(any(), java_method(name = "nextAfter", descriptor = "(FD)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextAfter_f_d(start: f32, direction: f64) -> Result<f32> {
        panic!("stub: java/lang/Math.nextAfter:(FD)F")
    }

    #[cfg_attr(any(), java_method(name = "nextUp", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextUp_d(d: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.nextUp:(D)D")
    }

    #[cfg_attr(any(), java_method(name = "nextUp", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextUp_f(f: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.nextUp:(F)F")
    }

    #[cfg_attr(any(), java_method(name = "nextDown", descriptor = "(D)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextDown_d(d: f64) -> Result<f64> {
        panic!("stub: java/lang/Math.nextDown:(D)D")
    }

    #[cfg_attr(any(), java_method(name = "nextDown", descriptor = "(F)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn nextDown_f(f: f32) -> Result<f32> {
        panic!("stub: java/lang/Math.nextDown:(F)F")
    }

    #[cfg_attr(any(), java_method(name = "scalb", descriptor = "(DI)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn scalb_d_i(d: f64, arg1: i32) -> Result<f64> {
        panic!("stub: java/lang/Math.scalb:(DI)D")
    }

    #[cfg_attr(any(), java_method(name = "scalb", descriptor = "(FI)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn scalb_f_i(f: f32, scaleFactor: i32) -> Result<f32> {
        panic!("stub: java/lang/Math.scalb:(FI)F")
    }

    #[cfg_attr(any(), java_method(name = "powerOfTwoD", descriptor = "(I)D", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn powerOfTwoD(n: i32) -> Result<f64> {
        panic!("stub: java/lang/Math.powerOfTwoD:(I)D")
    }

    #[cfg_attr(any(), java_method(name = "powerOfTwoF", descriptor = "(I)F", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false))]
    pub fn powerOfTwoF(n: i32) -> Result<f32> {
        panic!("stub: java/lang/Math.powerOfTwoF:(I)F")
    }
}
