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
    #[binary_name       = "java/util/Formatter$Conversion"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$Conversion:java/util/Formatter:Conversion:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$Conversion"]

    pub struct Formatter_Conversion;

    impl Formatter_Conversion {
        #[cfg_attr(any(), java_field(name = "DECIMAL_INTEGER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "100"))]
        // static field: DECIMAL_INTEGER:C
        pub fn DECIMAL_INTEGER() -> u16 {
            100
        }

        #[cfg_attr(any(), java_field(name = "OCTAL_INTEGER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "111"))]
        // static field: OCTAL_INTEGER:C
        pub fn OCTAL_INTEGER() -> u16 {
            111
        }

        #[cfg_attr(any(), java_field(name = "HEXADECIMAL_INTEGER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "120"))]
        // static field: HEXADECIMAL_INTEGER:C
        pub fn HEXADECIMAL_INTEGER() -> u16 {
            120
        }

        #[cfg_attr(any(), java_field(name = "HEXADECIMAL_INTEGER_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "88"))]
        // static field: HEXADECIMAL_INTEGER_UPPER:C
        pub fn HEXADECIMAL_INTEGER_UPPER() -> u16 {
            88
        }

        #[cfg_attr(any(), java_field(name = "SCIENTIFIC", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "101"))]
        // static field: SCIENTIFIC:C
        pub fn SCIENTIFIC() -> u16 {
            101
        }

        #[cfg_attr(any(), java_field(name = "SCIENTIFIC_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "69"))]
        // static field: SCIENTIFIC_UPPER:C
        pub fn SCIENTIFIC_UPPER() -> u16 {
            69
        }

        #[cfg_attr(any(), java_field(name = "GENERAL", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "103"))]
        // static field: GENERAL:C
        pub fn GENERAL() -> u16 {
            103
        }

        #[cfg_attr(any(), java_field(name = "GENERAL_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "71"))]
        // static field: GENERAL_UPPER:C
        pub fn GENERAL_UPPER() -> u16 {
            71
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL_FLOAT", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "102"))]
        // static field: DECIMAL_FLOAT:C
        pub fn DECIMAL_FLOAT() -> u16 {
            102
        }

        #[cfg_attr(any(), java_field(name = "HEXADECIMAL_FLOAT", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "97"))]
        // static field: HEXADECIMAL_FLOAT:C
        pub fn HEXADECIMAL_FLOAT() -> u16 {
            97
        }

        #[cfg_attr(any(), java_field(name = "HEXADECIMAL_FLOAT_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "65"))]
        // static field: HEXADECIMAL_FLOAT_UPPER:C
        pub fn HEXADECIMAL_FLOAT_UPPER() -> u16 {
            65
        }

        #[cfg_attr(any(), java_field(name = "CHARACTER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "99"))]
        // static field: CHARACTER:C
        pub fn CHARACTER() -> u16 {
            99
        }

        #[cfg_attr(any(), java_field(name = "CHARACTER_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "67"))]
        // static field: CHARACTER_UPPER:C
        pub fn CHARACTER_UPPER() -> u16 {
            67
        }

        #[cfg_attr(any(), java_field(name = "DATE_TIME", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "116"))]
        // static field: DATE_TIME:C
        pub fn DATE_TIME() -> u16 {
            116
        }

        #[cfg_attr(any(), java_field(name = "DATE_TIME_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "84"))]
        // static field: DATE_TIME_UPPER:C
        pub fn DATE_TIME_UPPER() -> u16 {
            84
        }

        #[cfg_attr(any(), java_field(name = "BOOLEAN", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "98"))]
        // static field: BOOLEAN:C
        pub fn BOOLEAN() -> u16 {
            98
        }

        #[cfg_attr(any(), java_field(name = "BOOLEAN_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "66"))]
        // static field: BOOLEAN_UPPER:C
        pub fn BOOLEAN_UPPER() -> u16 {
            66
        }

        #[cfg_attr(any(), java_field(name = "STRING", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "115"))]
        // static field: STRING:C
        pub fn STRING() -> u16 {
            115
        }

        #[cfg_attr(any(), java_field(name = "STRING_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "83"))]
        // static field: STRING_UPPER:C
        pub fn STRING_UPPER() -> u16 {
            83
        }

        #[cfg_attr(any(), java_field(name = "HASHCODE", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "104"))]
        // static field: HASHCODE:C
        pub fn HASHCODE() -> u16 {
            104
        }

        #[cfg_attr(any(), java_field(name = "HASHCODE_UPPER", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "72"))]
        // static field: HASHCODE_UPPER:C
        pub fn HASHCODE_UPPER() -> u16 {
            72
        }

        #[cfg_attr(any(), java_field(name = "LINE_SEPARATOR", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "110"))]
        // static field: LINE_SEPARATOR:C
        pub fn LINE_SEPARATOR() -> u16 {
            110
        }

        #[cfg_attr(any(), java_field(name = "PERCENT_SIGN", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "37"))]
        // static field: PERCENT_SIGN:C
        pub fn PERCENT_SIGN() -> u16 {
            37
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Formatter$Conversion.<init>:()V")
        }

        #[java_method(name = "isValid", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValid(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isValid:(C)Z")
        }

        #[java_method(name = "isGeneral", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isGeneral(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isGeneral:(C)Z")
        }

        #[java_method(name = "isCharacter", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCharacter(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isCharacter:(C)Z")
        }

        #[java_method(name = "isInteger", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInteger(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isInteger:(C)Z")
        }

        #[java_method(name = "isFloat", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFloat(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isFloat:(C)Z")
        }

        #[java_method(name = "isText", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isText(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$Conversion.isText:(C)Z")
        }
    }
}
