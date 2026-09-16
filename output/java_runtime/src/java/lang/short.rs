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

impl From<Short> for Number {
    fn from(v: Short) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Short"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable,java/lang/constant/Constable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Short;>;Ljava/lang/constant/Constable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Short.java"]
    #[inner_classes     = "java/lang/Short$ShortCache:java/lang/Short:ShortCache:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Number;java/lang/Object;java/lang/Short;java/lang/constant/Constable"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Short {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "S", access = "private", modifiers = "final", is_static = false))]
        pub value: i16,
    }

    impl Short {
        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "S", access = "public", modifiers = "static final", is_static = true, constant_value = "-32768"))]
        // static field: MIN_VALUE:S
        pub fn MIN_VALUE() -> i16 {
            -32768
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "S", access = "public", modifiers = "static final", is_static = true, constant_value = "32767"))]
        // static field: MAX_VALUE:S
        pub fn MAX_VALUE() -> i16 {
            32767
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Short;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<Short> {
            panic!("stub: java/lang/Short.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "7515723908773894738"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            7515723908773894738i64
        }

        #[java_method(name = "toString", descriptor = "(S)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_s(s: i16) -> Result<String> {
            panic!("stub: java/lang/Short.toString:(S)Ljava/lang/String;")
        }

        #[java_method(name = "parseShort", descriptor = "(Ljava/lang/String;I)S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseShort_str_i(s: String, radix: i32) -> Result<i16> {
            panic!("stub: java/lang/Short.parseShort:(Ljava/lang/String;I)S")
        }

        #[java_method(name = "parseShort", descriptor = "(Ljava/lang/String;)S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseShort_str(s: String) -> Result<i16> {
            panic!("stub: java/lang/Short.parseShort:(Ljava/lang/String;)S")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Short;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str_i(s: String, radix: i32) -> Result<Short> {
            panic!("stub: java/lang/Short.valueOf:(Ljava/lang/String;I)Ljava/lang/Short;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Short;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str(s: String) -> Result<Short> {
            panic!("stub: java/lang/Short.valueOf:(Ljava/lang/String;)Ljava/lang/Short;")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/DynamicConstantDesc<Ljava/lang/Short;>;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Short.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "valueOf", descriptor = "(S)Ljava/lang/Short;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_s(s: i16) -> Result<Short> {
            panic!("stub: java/lang/Short.valueOf:(S)Ljava/lang/Short;")
        }

        #[java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Short;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn decode(nm: String) -> Result<Short> {
            panic!("stub: java/lang/Short.decode:(Ljava/lang/String;)Ljava/lang/Short;")
        }

        #[java_method(name = "<init>", descriptor = "(S)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_s(value: i16) -> Result<Self> {
            panic!("stub: java/lang/Short.<init>:(S)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Short.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValue(&self) -> Result<i8> {
            panic!("stub: java/lang/Short.byteValue:()B")
        }

        #[java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValue(&self) -> Result<i16> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/lang/Short.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/lang/Short.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/lang/Short.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/lang/Short.doubleValue:()D")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_s(value: i16) -> Result<i32> {
            panic!("stub: java/lang/Short.hashCode:(S)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Short.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Short;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, anotherShort: Short) -> Result<i32> {
            panic!("stub: java/lang/Short.compareTo:(Ljava/lang/Short;)I")
        }

        #[java_method(name = "compare", descriptor = "(SS)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(x: i16, y: i16) -> Result<i32> {
            panic!("stub: java/lang/Short.compare:(SS)I")
        }

        #[java_method(name = "compareUnsigned", descriptor = "(SS)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned(x: i16, y: i16) -> Result<i32> {
            panic!("stub: java/lang/Short.compareUnsigned:(SS)I")
        }

        #[java_method(name = "reverseBytes", descriptor = "(S)S", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn reverseBytes(i: i16) -> Result<i16> {
            panic!("stub: java/lang/Short.reverseBytes:(S)S")
        }

        #[java_method(name = "toUnsignedInt", descriptor = "(S)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedInt(x: i16) -> Result<i32> {
            panic!("stub: java/lang/Short.toUnsignedInt:(S)I")
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(S)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong(x: i16) -> Result<i64> {
            panic!("stub: java/lang/Short.toUnsignedLong:(S)J")
        }
    }
}
