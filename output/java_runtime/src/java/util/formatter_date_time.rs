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
    #[binary_name       = "java/util/Formatter$DateTime"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$DateTime:java/util/Formatter:DateTime:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$DateTime"]

    pub struct Formatter_DateTime;

    impl Formatter_DateTime {
        #[cfg_attr(any(), java_field(name = "HOUR_OF_DAY_0", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "72"))]
        // static field: HOUR_OF_DAY_0:C
        pub fn HOUR_OF_DAY_0() -> u16 {
            72
        }

        #[cfg_attr(any(), java_field(name = "HOUR_0", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "73"))]
        // static field: HOUR_0:C
        pub fn HOUR_0() -> u16 {
            73
        }

        #[cfg_attr(any(), java_field(name = "HOUR_OF_DAY", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "107"))]
        // static field: HOUR_OF_DAY:C
        pub fn HOUR_OF_DAY() -> u16 {
            107
        }

        #[cfg_attr(any(), java_field(name = "HOUR", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "108"))]
        // static field: HOUR:C
        pub fn HOUR() -> u16 {
            108
        }

        #[cfg_attr(any(), java_field(name = "MINUTE", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "77"))]
        // static field: MINUTE:C
        pub fn MINUTE() -> u16 {
            77
        }

        #[cfg_attr(any(), java_field(name = "NANOSECOND", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "78"))]
        // static field: NANOSECOND:C
        pub fn NANOSECOND() -> u16 {
            78
        }

        #[cfg_attr(any(), java_field(name = "MILLISECOND", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "76"))]
        // static field: MILLISECOND:C
        pub fn MILLISECOND() -> u16 {
            76
        }

        #[cfg_attr(any(), java_field(name = "MILLISECOND_SINCE_EPOCH", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "81"))]
        // static field: MILLISECOND_SINCE_EPOCH:C
        pub fn MILLISECOND_SINCE_EPOCH() -> u16 {
            81
        }

        #[cfg_attr(any(), java_field(name = "AM_PM", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "112"))]
        // static field: AM_PM:C
        pub fn AM_PM() -> u16 {
            112
        }

        #[cfg_attr(any(), java_field(name = "SECONDS_SINCE_EPOCH", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "115"))]
        // static field: SECONDS_SINCE_EPOCH:C
        pub fn SECONDS_SINCE_EPOCH() -> u16 {
            115
        }

        #[cfg_attr(any(), java_field(name = "SECOND", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "83"))]
        // static field: SECOND:C
        pub fn SECOND() -> u16 {
            83
        }

        #[cfg_attr(any(), java_field(name = "TIME", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "84"))]
        // static field: TIME:C
        pub fn TIME() -> u16 {
            84
        }

        #[cfg_attr(any(), java_field(name = "ZONE_NUMERIC", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "122"))]
        // static field: ZONE_NUMERIC:C
        pub fn ZONE_NUMERIC() -> u16 {
            122
        }

        #[cfg_attr(any(), java_field(name = "ZONE", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "90"))]
        // static field: ZONE:C
        pub fn ZONE() -> u16 {
            90
        }

        #[cfg_attr(any(), java_field(name = "NAME_OF_DAY_ABBREV", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "97"))]
        // static field: NAME_OF_DAY_ABBREV:C
        pub fn NAME_OF_DAY_ABBREV() -> u16 {
            97
        }

        #[cfg_attr(any(), java_field(name = "NAME_OF_DAY", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "65"))]
        // static field: NAME_OF_DAY:C
        pub fn NAME_OF_DAY() -> u16 {
            65
        }

        #[cfg_attr(any(), java_field(name = "NAME_OF_MONTH_ABBREV", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "98"))]
        // static field: NAME_OF_MONTH_ABBREV:C
        pub fn NAME_OF_MONTH_ABBREV() -> u16 {
            98
        }

        #[cfg_attr(any(), java_field(name = "NAME_OF_MONTH", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "66"))]
        // static field: NAME_OF_MONTH:C
        pub fn NAME_OF_MONTH() -> u16 {
            66
        }

        #[cfg_attr(any(), java_field(name = "CENTURY", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "67"))]
        // static field: CENTURY:C
        pub fn CENTURY() -> u16 {
            67
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_MONTH_0", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "100"))]
        // static field: DAY_OF_MONTH_0:C
        pub fn DAY_OF_MONTH_0() -> u16 {
            100
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_MONTH", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "101"))]
        // static field: DAY_OF_MONTH:C
        pub fn DAY_OF_MONTH() -> u16 {
            101
        }

        #[cfg_attr(any(), java_field(name = "NAME_OF_MONTH_ABBREV_X", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "104"))]
        // static field: NAME_OF_MONTH_ABBREV_X:C
        pub fn NAME_OF_MONTH_ABBREV_X() -> u16 {
            104
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_YEAR", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "106"))]
        // static field: DAY_OF_YEAR:C
        pub fn DAY_OF_YEAR() -> u16 {
            106
        }

        #[cfg_attr(any(), java_field(name = "MONTH", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "109"))]
        // static field: MONTH:C
        pub fn MONTH() -> u16 {
            109
        }

        #[cfg_attr(any(), java_field(name = "YEAR_2", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "121"))]
        // static field: YEAR_2:C
        pub fn YEAR_2() -> u16 {
            121
        }

        #[cfg_attr(any(), java_field(name = "YEAR_4", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "89"))]
        // static field: YEAR_4:C
        pub fn YEAR_4() -> u16 {
            89
        }

        #[cfg_attr(any(), java_field(name = "TIME_12_HOUR", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "114"))]
        // static field: TIME_12_HOUR:C
        pub fn TIME_12_HOUR() -> u16 {
            114
        }

        #[cfg_attr(any(), java_field(name = "TIME_24_HOUR", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "82"))]
        // static field: TIME_24_HOUR:C
        pub fn TIME_24_HOUR() -> u16 {
            82
        }

        #[cfg_attr(any(), java_field(name = "DATE_TIME", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "99"))]
        // static field: DATE_TIME:C
        pub fn DATE_TIME() -> u16 {
            99
        }

        #[cfg_attr(any(), java_field(name = "DATE", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "68"))]
        // static field: DATE:C
        pub fn DATE() -> u16 {
            68
        }

        #[cfg_attr(any(), java_field(name = "ISO_STANDARD_DATE", descriptor = "C", access = "package", modifiers = "static final", is_static = true, constant_value = "70"))]
        // static field: ISO_STANDARD_DATE:C
        pub fn ISO_STANDARD_DATE() -> u16 {
            70
        }

        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/util/Formatter$DateTime.<init>:()V")
        }

        #[java_method(name = "isValid", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValid(c: u16) -> Result<bool> {
            panic!("stub: java/util/Formatter$DateTime.isValid:(C)Z")
        }
    }
}
