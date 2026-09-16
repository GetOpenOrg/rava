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

impl From<Byte> for Number {
    fn from(v: Byte) -> Number { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/lang/Byte"]
    #[super_class       = "java/lang/Number"]
    #[interfaces        = "java/lang/Comparable,java/lang/constant/Constable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Number;Ljava/lang/Comparable<Ljava/lang/Byte;>;Ljava/lang/constant/Constable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Byte.java"]
    #[inner_classes     = "java/lang/Byte$ByteCache:java/lang/Byte:ByteCache:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Number"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Byte;java/lang/Comparable;java/lang/Number;java/lang/Object;java/lang/constant/Constable"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Byte {
        #[cfg_attr(any(), java_field(name = "value", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub value: i8,
    }

    impl Byte {
        #[cfg_attr(any(), java_field(name = "MIN_VALUE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "-128"))]
        // static field: MIN_VALUE:B
        pub fn MIN_VALUE() -> i8 {
            -128
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "127"))]
        // static field: MAX_VALUE:B
        pub fn MAX_VALUE() -> i8 {
            127
        }

        #[cfg_attr(any(), java_field(name = "TYPE", descriptor = "Ljava/lang/Class;", access = "public", modifiers = "static final", is_static = true, generic_signature = "Ljava/lang/Class<Ljava/lang/Byte;>;"))]
        // static field: TYPE:Ljava/lang/Class;
        pub fn TYPE() -> Class<Byte> {
            panic!("stub: java/lang/Byte.TYPE:Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "SIZE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: SIZE:I
        pub fn SIZE() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "BYTES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: BYTES:I
        pub fn BYTES() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-7183698231559129828"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -7183698231559129828i64
        }

        #[java_method(name = "toString", descriptor = "(B)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString_b(b: i8) -> Result<String> {
            panic!("stub: java/lang/Byte.toString:(B)Ljava/lang/String;")
        }

        #[java_method(name = "describeConstable", descriptor = "()Ljava/util/Optional;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Optional<Ljava/lang/constant/DynamicConstantDesc<Ljava/lang/Byte;>;>;")]
        pub fn describeConstable(&self) -> Result<Optional<Object>> {
            panic!("stub: java/lang/Byte.describeConstable:()Ljava/util/Optional;")
        }

        #[java_method(name = "valueOf", descriptor = "(B)Ljava/lang/Byte;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_b(b: i8) -> Result<Byte> {
            panic!("stub: java/lang/Byte.valueOf:(B)Ljava/lang/Byte;")
        }

        #[java_method(name = "parseByte", descriptor = "(Ljava/lang/String;I)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseByte_str_i(s: String, radix: i32) -> Result<i8> {
            panic!("stub: java/lang/Byte.parseByte:(Ljava/lang/String;I)B")
        }

        #[java_method(name = "parseByte", descriptor = "(Ljava/lang/String;)B", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn parseByte_str(s: String) -> Result<i8> {
            panic!("stub: java/lang/Byte.parseByte:(Ljava/lang/String;)B")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;I)Ljava/lang/Byte;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str_i(s: String, radix: i32) -> Result<Byte> {
            panic!("stub: java/lang/Byte.valueOf:(Ljava/lang/String;I)Ljava/lang/Byte;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Byte;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn valueOf_str(s: String) -> Result<Byte> {
            panic!("stub: java/lang/Byte.valueOf:(Ljava/lang/String;)Ljava/lang/Byte;")
        }

        #[java_method(name = "decode", descriptor = "(Ljava/lang/String;)Ljava/lang/Byte;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException")]
        pub fn decode(nm: String) -> Result<Byte> {
            panic!("stub: java/lang/Byte.decode:(Ljava/lang/String;)Ljava/lang/Byte;")
        }

        #[java_method(name = "<init>", descriptor = "(B)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_b(value: i8) -> Result<Self> {
            panic!("stub: java/lang/Byte.<init>:(B)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/lang/NumberFormatException", is_deprecated = true)]
        pub fn new_str(s: String) -> Result<Self> {
            panic!("stub: java/lang/Byte.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "byteValue", descriptor = "()B", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn byteValue(&self) -> Result<i8> {
            let this = self;
            Ok(this.__get_value())
        }

        #[java_method(name = "shortValue", descriptor = "()S", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn shortValue(&self) -> Result<i16> {
            panic!("stub: java/lang/Byte.shortValue:()S")
        }

        #[java_method(name = "intValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn intValue(&self) -> Result<i32> {
            panic!("stub: java/lang/Byte.intValue:()I")
        }

        #[java_method(name = "longValue", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn longValue(&self) -> Result<i64> {
            panic!("stub: java/lang/Byte.longValue:()J")
        }

        #[java_method(name = "floatValue", descriptor = "()F", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn floatValue(&self) -> Result<f32> {
            panic!("stub: java/lang/Byte.floatValue:()F")
        }

        #[java_method(name = "doubleValue", descriptor = "()D", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleValue(&self) -> Result<f64> {
            panic!("stub: java/lang/Byte.doubleValue:()D")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "hashCode", descriptor = "(B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode_b(value: i8) -> Result<i32> {
            panic!("stub: java/lang/Byte.hashCode:(B)I")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/lang/Byte.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/lang/Byte;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, anotherByte: Byte) -> Result<i32> {
            panic!("stub: java/lang/Byte.compareTo:(Ljava/lang/Byte;)I")
        }

        #[java_method(name = "compare", descriptor = "(BB)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compare(mut x: i8, mut y: i8) -> Result<i32> {
            Ok(((x as i32)).wrapping_sub((y as i32)))
        }

        #[java_method(name = "compareUnsigned", descriptor = "(BB)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareUnsigned(x: i8, y: i8) -> Result<i32> {
            panic!("stub: java/lang/Byte.compareUnsigned:(BB)I")
        }

        #[java_method(name = "toUnsignedInt", descriptor = "(B)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedInt(x: i8) -> Result<i32> {
            panic!("stub: java/lang/Byte.toUnsignedInt:(B)I")
        }

        #[java_method(name = "toUnsignedLong", descriptor = "(B)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUnsignedLong(x: i8) -> Result<i64> {
            panic!("stub: java/lang/Byte.toUnsignedLong:(B)J")
        }
    }
}
