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
use crate::jdk::internal::math::DoubleToDecimal;

impl From<Double> for Number {
    fn from(v: Double) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Double"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable,java/lang/constant/Constable,java/lang/constant/ConstantDesc"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Double;>;Ljava/lang/constant/Constable;Ljava/lang/constant/ConstantDesc;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Double.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Double;java/lang/Number;java/lang/Object;java/lang/constant/Constable;java/lang/constant/ConstantDesc"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Double {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "D", access = "private", modifiers = "final", is_static = false))]
        pub value: f64,
    }

    impl Double {
        #[cfg_attr(any(), java_field(name = "POSITIVE_INFINITY", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "inf"))]
        // static field: POSITIVE_INFINITY:D
        pub fn POSITIVE_INFINITY() -> f64 {
            f64::INFINITY
        }

        #[cfg_attr(any(), java_field(name = "NEGATIVE_INFINITY", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "-inf"))]
        // static field: NEGATIVE_INFINITY:D
        pub fn NEGATIVE_INFINITY() -> f64 {
            f64::NEG_INFINITY
        }

        #[cfg_attr(any(), java_field(name = "NaN", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "NaN"))]
        // static field: NaN:D
        pub fn NaN() -> f64 {
            f64::NAN
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "1.7976931348623157e+308"))]
        // static field: MAX_VALUE:D
        pub fn MAX_VALUE() -> f64 {
            1.7976931348623157e+308f64
        }

        #[cfg_attr(any(), java_field(name = "MIN_NORMAL", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "2.2250738585072014e-308"))]
        // static field: MIN_NORMAL:D
        pub fn MIN_NORMAL() -> f64 {
            2.2250738585072014e-308f64
        }

        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "D", access = "public", modifiers = "static final", is_static = true, constant_value = "5e-324"))]
        // static field: MIN_VALUE:D
        pub fn MIN_VALUE() -> f64 {
            5e-324f64
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "PRECISION", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "53"))]
        // static field: PRECISION:I
        pub fn PRECISION() -> i32 {
            53
        }

        #[cfg_attr(any(), java_field(name = "MAX_EXPONENT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1023"))]
        // static field: MAX_EXPONENT:I
        pub fn MAX_EXPONENT() -> i32 {
            1023
        }

        #[cfg_attr(any(), java_field(name = "MIN_EXPONENT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-1022"))]
        // static field: MIN_EXPONENT:I
        pub fn MIN_EXPONENT() -> i32 {
            -1022
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Double;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<f64> {
            panic!("stub: java/lang/Double.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-9172774392245257468"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -9172774392245257468i64
        }

        #[java_method(name = "toString", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: toString(D)Ljava/lang/String;
        pub fn toString_d(mut d: f64) -> Result<String> {
            let _t0: String = DoubleToDecimal::toString(d)?;
            Ok(_t0)
        }

        #[java_method(name = "toHexString", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toHexString(mut d: f64) -> Result<String> {
            let _t0: bool = Double::isFinite(d)?;
            if !(_t0) {
                let _t1: String = Double::toString_d(d)?;
                return Ok(_t1);
            }
            let mut answer = StringBuilder::new_i(24i32)?;
            let _t1: f64 = Math::copySign_d_d(1f64, d)?;
            if (((_t1>(-1.0f64)) as i32-((_t1)<(-1.0f64)) as i32)==0) {
                let _t2 = answer.append_str(Clone::clone(&String::from("-")))?;
            }
            let _t2 = answer.append_str(Clone::clone(&String::from("0x")))?;
            let _t3: f64 = Math::abs_d(d)?;
            d = _t3;
            if (((d>(0f64)) as i32-((d)<(0f64)) as i32)==0) {
                let _t4 = answer.append_str(Clone::clone(&String::from("0.0p0")))?;
            } else {
                let mut subnormal = ((((d>(2.2250738585072014e-308f64)) as i32-((d)<(2.2250738585072014e-308f64)) as i32)<0)) as i32;
                let _t4: i64 = Double::doubleToLongBits(d)?;
                let mut signifBits = ((_t4&(4503599627370495i64))|(1152921504606846976i64));
                let _t5 = answer.append_str(Clone::clone(&(if (subnormal!=0) { String::from("0.") } else { String::from("1.") })))?;
                let _t6: String = Long::toHexString(signifBits)?;
                let _t7 = _t6.substring_i_i(3i32, 16i32)?;
                let mut signif: String = _t7;
                let _t8 = signif.equals(Object::from_any(String::from("0000000000000").clone()))?;
                let mut _merged10: String;
                if _t8 {
                    _merged10 = String::from("0");
                } else {
                    let _t9 = signif.replaceFirst(Clone::clone(&String::from("0{1,12}$")), Clone::clone(&String::from("")))?;
                    _merged10 = _t9;
                }
                let _t11 = answer.append_str(Clone::clone(&_merged10))?;
                let _t12 = answer.append_c(((112i32) as u16))?;
                let mut _merged14: i32;
                if (subnormal!=0) {
                    _merged14 = -1022i32;
                } else {
                    let _t13: i32 = Math::getExponent_d(d)?;
                    _merged14 = _t13;
                }
                let _t15 = answer.append_i(_merged14)?;
            }
            let _t4 = answer.toString()?;
            Ok(_t4)
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Double;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str(s: String) -> Result<f64> {
            panic!("stub: java/lang/Double.valueOf:(Ljava/lang/String;)Ljava/lang/Double;")
        }

        #[java_method(name = "valueOf", descriptor = "(D)Ljava/lang/Double;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: valueOf(D)Ljava/lang/Double;
        pub fn valueOf_d(mut d: f64) -> Result<f64> {
            Ok(d)
        }

        #[java_method(name = "parseDouble", descriptor = "(Ljava/lang/String;)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseDouble(s: String) -> Result<f64> {
            panic!("stub: java/lang/Double.parseDouble:(Ljava/lang/String;)D")
        }

        #[java_method(name = "isNaN", descriptor = "(D)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isNaN(D)Z
        pub fn isNaN_d(mut v: f64) -> Result<bool> {
            Ok((((v>(v)) as i32-((v)<(v)) as i32)!=0))
        }

        #[java_method(name = "isInfinite", descriptor = "(D)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: isInfinite(D)Z
        pub fn isInfinite_d(mut v: f64) -> Result<bool> {
            let _t0: f64 = Math::abs_d(v)?;
            Ok((((_t0>(1.7976931348623157e+308f64)) as i32-((_t0)<(1.7976931348623157e+308f64)) as i32)>0))
        }

        #[java_method(name = "isFinite", descriptor = "(D)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFinite(mut d: f64) -> Result<bool> {
            let _t0: f64 = Math::abs_d(d)?;
            Ok((((_t0>(1.7976931348623157e+308f64)) as i32-((_t0)<(1.7976931348623157e+308f64)) as i32)<=0))
        }

        #[java_method(name = "<init>", descriptor = "(D)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        // java: <init>(D)V
        pub fn new_d(mut value: f64) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Number::new()?);
            this.__set_value(value);
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Double.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "isNaN", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNaN(&self) -> Result<bool> {
            panic!("stub: java/lang/Double.isNaN:()Z")
        }

        #[java_method(name = "isInfinite", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInfinite(&self) -> Result<bool> {
            panic!("stub: java/lang/Double.isInfinite:()Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValue(&self) -> Result<i8> {
            panic!("stub: java/lang/Double.byteValue:()B")
        }

        #[java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValue(&self) -> Result<i16> {
            panic!("stub: java/lang/Double.shortValue:()S")
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/lang/Double.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/lang/Double.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/lang/Double.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(D)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_d(value: f64) -> Result<i32> {
            panic!("stub: java/lang/Double.hashCode:(D)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Double.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "doubleToLongBits", descriptor = "(D)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleToLongBits(mut value: f64) -> Result<i64> {
            let _t0: bool = Double::isNaN_d(value)?;
            if !(_t0) {
                let _t1: i64 = Double::doubleToRawLongBits(value)?;
                return Ok(_t1);
            }
            Ok(9221120237041090560i64)
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Double;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, mut anotherDouble: f64) -> Result<i32> {
            let this = self;
            let _t0: i32 = Double::compare(this.__get_value(), anotherDouble.__get_value())?;
            Ok(_t0)
        }

        #[java_method(name = "compare", descriptor = "(DD)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut d1: f64, mut d2: f64) -> Result<i32> {
            if (((d1>(d2)) as i32-((d1)<(d2)) as i32)<0) {
                return Ok(-1i32);
            }
            if (((d1>(d2)) as i32-((d1)<(d2)) as i32)>0) {
                return Ok(1i32);
            }
            let _t0: i64 = Double::doubleToLongBits(d1)?;
            let mut thisBits: i64 = _t0;
            let _t1: i64 = Double::doubleToLongBits(d2)?;
            let mut anotherBits: i64 = _t1;
            Ok(((if (((thisBits>(anotherBits)) as i32-((thisBits)<(anotherBits)) as i32)==0) { (0i32 != 0) } else { (((thisBits>(anotherBits)) as i32-((thisBits)<(anotherBits)) as i32)>=0) })) as i32)
        }

        #[java_method(name = "sum", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn sum(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Double.sum:(DD)D")
        }

        #[java_method(name = "max", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn max(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Double.max:(DD)D")
        }

        #[java_method(name = "min", descriptor = "(DD)D", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn min(a: f64, arg1: f64) -> Result<f64> {
            panic!("stub: java/lang/Double.min:(DD)D")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/Double;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Double.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "resolveConstantDesc", descriptor = "(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveConstantDesc(&self, lookup: Object) -> Result<f64> {
            panic!("stub: java/lang/Double.resolveConstantDesc:(Ljava/lang/invoke/MethodHandles$Lookup;)Ljava/lang/Double;")
        }
    }
}
