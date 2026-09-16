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
use crate::jdk::internal::math::FloatToDecimal;

impl From<Float> for Number {
    fn from(v: Float) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Float"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Float;>;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Float.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Float;java/lang/Number;java/lang/Object;java/lang/constant/Constable;java/lang/constant/ConstantDesc"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Float {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "F", access = "private", modifiers = "final", is_static = false))]
        pub value: f32,
    }

    impl Float {
        #[cfg_attr(any(), java_field(name = "POSITIVE_INFINITY", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "inf"))]
        // static field: POSITIVE_INFINITY:F
        pub fn POSITIVE_INFINITY() -> f32 {
            f32::INFINITY
        }

        #[cfg_attr(any(), java_field(name = "NEGATIVE_INFINITY", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "-inf"))]
        // static field: NEGATIVE_INFINITY:F
        pub fn NEGATIVE_INFINITY() -> f32 {
            f32::NEG_INFINITY
        }

        #[cfg_attr(any(), java_field(name = "NaN", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "NaN"))]
        // static field: NaN:F
        pub fn NaN() -> f32 {
            f32::NAN
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "3.4028234663852886e+38"))]
        // static field: MAX_VALUE:F
        pub fn MAX_VALUE() -> f32 {
            3.4028234663852886e+38f32
        }

        #[cfg_attr(any(), java_field(name = "MIN_NORMAL", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "1.1754943508222875e-38"))]
        // static field: MIN_NORMAL:F
        pub fn MIN_NORMAL() -> f32 {
            1.1754943508222875e-38f32
        }

        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "F", access = "public", modifiers = "static final", is_static = true, constant_value = "1.401298464324817e-45"))]
        // static field: MIN_VALUE:F
        pub fn MIN_VALUE() -> f32 {
            1.401298464324817e-45f32
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "PRECISION", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: PRECISION:I
        pub fn PRECISION() -> i32 {
            24
        }

        #[cfg_attr(any(), java_field(name = "MAX_EXPONENT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "127"))]
        // static field: MAX_EXPONENT:I
        pub fn MAX_EXPONENT() -> i32 {
            127
        }

        #[cfg_attr(any(), java_field(name = "MIN_EXPONENT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-126"))]
        // static field: MIN_EXPONENT:I
        pub fn MIN_EXPONENT() -> i32 {
            -126
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Float;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<f32> {
            panic!("stub: java/lang/Float.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-2671257302660747028"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2671257302660747028i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "toString", descriptor = "(F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_f(f: f32) -> Result<String> {
            panic!("stub: java/lang/Float.toString:(F)Ljava/lang/String;")
        }

        #[java_method(name = "toHexString", descriptor = "(F)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toHexString(f: f32) -> Result<String> {
            panic!("stub: java/lang/Float.toHexString:(F)Ljava/lang/String;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Float;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str(s: String) -> Result<Float> {
            panic!("stub: java/lang/Float.valueOf:(Ljava/lang/String;)Ljava/lang/Float;")
        }

        #[java_method(name = "valueOf", descriptor = "(F)Ljava/lang/Float;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_f(f: f32) -> Result<Float> {
            panic!("stub: java/lang/Float.valueOf:(F)Ljava/lang/Float;")
        }

        #[java_method(name = "parseFloat", descriptor = "(Ljava/lang/String;)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseFloat(s: String) -> Result<f32> {
            panic!("stub: java/lang/Float.parseFloat:(Ljava/lang/String;)F")
        }

        #[java_method(name = "isNaN", descriptor = "(F)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isNaN(F)Z
        pub fn isNaN_f(mut v: f32) -> Result<bool> {
            Ok((((v>(v)) as i32-((v)<(v)) as i32)!=0))
        }

        #[java_method(name = "isInfinite", descriptor = "(F)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInfinite_f(v: f32) -> Result<bool> {
            panic!("stub: java/lang/Float.isInfinite:(F)Z")
        }

        #[java_method(name = "isFinite", descriptor = "(F)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFinite(f: f32) -> Result<bool> {
            panic!("stub: java/lang/Float.isFinite:(F)Z")
        }

        #[java_method(name = "<init>", descriptor = "(F)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_f(value: f32) -> Result<Self> {
            panic!("stub: java/lang/Float.<init>:(F)V")
        }

        #[java_method(name = "<init>", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_d(value: f64) -> Result<Self> {
            panic!("stub: java/lang/Float.<init>:(D)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Float.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "isNaN", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNaN(&self) -> Result<bool> {
            panic!("stub: java/lang/Float.isNaN:()Z")
        }

        #[java_method(name = "isInfinite", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInfinite(&self) -> Result<bool> {
            panic!("stub: java/lang/Float.isInfinite:()Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValue(&self) -> Result<i8> {
            panic!("stub: java/lang/Float.byteValue:()B")
        }

        #[java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValue(&self) -> Result<i16> {
            panic!("stub: java/lang/Float.shortValue:()S")
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/lang/Float.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/lang/Float.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/lang/Float.doubleValue:()D")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_f(value: f32) -> Result<i32> {
            panic!("stub: java/lang/Float.hashCode:(F)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Float.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "floatToIntBits", descriptor = "(F)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatToIntBits(mut value: f32) -> Result<i32> {
            let _t0: bool = Float::isNaN_f(value)?;
            if !(_t0) {
                let _t1: i32 = Float::floatToRawIntBits(value)?;
                return Ok(_t1);
            }
            Ok(2143289344i32)
        }

        #[java_method(name = "float16ToFloat", descriptor = "(S)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn float16ToFloat(floatBinary16: i16) -> Result<f32> {
            panic!("stub: java/lang/Float.float16ToFloat:(S)F")
        }

        #[java_method(name = "floatToFloat16", descriptor = "(F)S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatToFloat16(f: f32) -> Result<i16> {
            panic!("stub: java/lang/Float.floatToFloat16:(F)S")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Float;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut anotherFloat: Float) -> Result<i32> {
            let this = self;
            let _t0: i32 = Float::compare(this.__get_value(), anotherFloat.__get_value())?;
            Ok(_t0)
        }

        #[java_method(name = "compare", descriptor = "(FF)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut f1: f32, mut f2: f32) -> Result<i32> {
            if (((f1>(f2)) as i32-((f1)<(f2)) as i32)<0) {
                return Ok(-1i32);
            }
            if (((f1>(f2)) as i32-((f1)<(f2)) as i32)>0) {
                return Ok(1i32);
            }
            let _t0: i32 = Float::floatToIntBits(f1)?;
            let mut thisBits: i32 = _t0;
            let _t1: i32 = Float::floatToIntBits(f2)?;
            let mut anotherBits: i32 = _t1;
            Ok(((if thisBits == anotherBits { (0i32 != 0) } else { thisBits >= anotherBits })) as i32)
        }

        #[java_method(name = "sum", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sum(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/Float.sum:(FF)F")
        }

        #[java_method(name = "max", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/Float.max:(FF)F")
        }

        #[java_method(name = "min", descriptor = "(FF)F", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(a: f32, b: f32) -> Result<f32> {
            panic!("stub: java/lang/Float.min:(FF)F")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Float;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Float.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveConstantDesc(&self, lookup: Object) -> Result<Float> {
            panic!("stub: java/lang/Float.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Float;")
        }
    }
}
