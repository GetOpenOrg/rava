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
    #[binary_name       = "java/util/Calendar"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Cloneable,java/lang/Comparable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = "Ljava/lang/Object;Ljava/io/Serializable;Ljava/lang/Cloneable;Ljava/lang/Comparable<Ljava/util/Calendar;>;"]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Calendar.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/util/Calendar$AvailableCalendarTypes:java/util/Calendar:AvailableCalendarTypes:10;java/util/Calendar$1:::0;java/util/Calendar$CalendarAccessControlContext:java/util/Calendar:CalendarAccessControlContext:10;java/util/Calendar$Builder:java/util/Calendar:Builder:9;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Comparable;java/lang/Object;java/util/Calendar"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Calendar {
        #[cfg_attr(any(), java_field(name = "fields", descriptor = "[I", access = "protected", modifiers = "", is_static = false))]
        pub fields: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "isSet", descriptor = "[Z", access = "protected", modifiers = "", is_static = false))]
        pub isSet: Rc<RefCell<Vec<bool>>>,
        #[cfg_attr(any(), java_field(name = "stamp", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub stamp: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "time", descriptor = "J", access = "protected", modifiers = "", is_static = false))]
        pub time: i64,
        #[cfg_attr(any(), java_field(name = "isTimeSet", descriptor = "Z", access = "protected", modifiers = "", is_static = false))]
        pub isTimeSet: bool,
        #[cfg_attr(any(), java_field(name = "areFieldsSet", descriptor = "Z", access = "protected", modifiers = "", is_static = false))]
        pub areFieldsSet: bool,
        #[cfg_attr(any(), java_field(name = "areAllFieldsSet", descriptor = "Z", access = "package", modifiers = "transient", is_static = false))]
        pub areAllFieldsSet: bool,
        #[cfg_attr(any(), java_field(name = "lenient", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub lenient: bool,
        #[cfg_attr(any(), java_field(name = "zone", descriptor = "Ljava/util/TimeZone;", access = "private", modifiers = "", is_static = false))]
        pub zone: TimeZone,
        #[cfg_attr(any(), java_field(name = "sharedZone", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub sharedZone: bool,
        #[cfg_attr(any(), java_field(name = "firstDayOfWeek", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub firstDayOfWeek: i32,
        #[cfg_attr(any(), java_field(name = "minimalDaysInFirstWeek", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minimalDaysInFirstWeek: i32,
        #[cfg_attr(any(), java_field(name = "nextStamp", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub nextStamp: i32,
        #[cfg_attr(any(), java_field(name = "serialVersionOnStream", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub serialVersionOnStream: i32,
    }

    impl Calendar {
        #[cfg_attr(any(), java_field(name = "ERA", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: ERA:I
        pub fn ERA() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "YEAR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: YEAR:I
        pub fn YEAR() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MONTH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MONTH:I
        pub fn MONTH() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "WEEK_OF_YEAR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: WEEK_OF_YEAR:I
        pub fn WEEK_OF_YEAR() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "WEEK_OF_MONTH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: WEEK_OF_MONTH:I
        pub fn WEEK_OF_MONTH() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "DATE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: DATE:I
        pub fn DATE() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_MONTH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: DAY_OF_MONTH:I
        pub fn DAY_OF_MONTH() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_YEAR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: DAY_OF_YEAR:I
        pub fn DAY_OF_YEAR() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_WEEK", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: DAY_OF_WEEK:I
        pub fn DAY_OF_WEEK() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_WEEK_IN_MONTH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: DAY_OF_WEEK_IN_MONTH:I
        pub fn DAY_OF_WEEK_IN_MONTH() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "AM_PM", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: AM_PM:I
        pub fn AM_PM() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "HOUR", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: HOUR:I
        pub fn HOUR() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "HOUR_OF_DAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: HOUR_OF_DAY:I
        pub fn HOUR_OF_DAY() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "MINUTE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: MINUTE:I
        pub fn MINUTE() -> i32 {
            12
        }

        #[cfg_attr(any(), java_field(name = "SECOND", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: SECOND:I
        pub fn SECOND() -> i32 {
            13
        }

        #[cfg_attr(any(), java_field(name = "MILLISECOND", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: MILLISECOND:I
        pub fn MILLISECOND() -> i32 {
            14
        }

        #[cfg_attr(any(), java_field(name = "ZONE_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: ZONE_OFFSET:I
        pub fn ZONE_OFFSET() -> i32 {
            15
        }

        #[cfg_attr(any(), java_field(name = "DST_OFFSET", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: DST_OFFSET:I
        pub fn DST_OFFSET() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "FIELD_COUNT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "17"))]
        // static field: FIELD_COUNT:I
        pub fn FIELD_COUNT() -> i32 {
            17
        }

        #[cfg_attr(any(), java_field(name = "SUNDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: SUNDAY:I
        pub fn SUNDAY() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MONDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MONDAY:I
        pub fn MONDAY() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "TUESDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: TUESDAY:I
        pub fn TUESDAY() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "WEDNESDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: WEDNESDAY:I
        pub fn WEDNESDAY() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "THURSDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: THURSDAY:I
        pub fn THURSDAY() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "FRIDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: FRIDAY:I
        pub fn FRIDAY() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "SATURDAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: SATURDAY:I
        pub fn SATURDAY() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "JANUARY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: JANUARY:I
        pub fn JANUARY() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "FEBRUARY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: FEBRUARY:I
        pub fn FEBRUARY() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MARCH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MARCH:I
        pub fn MARCH() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "APRIL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: APRIL:I
        pub fn APRIL() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "MAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: MAY:I
        pub fn MAY() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "JUNE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: JUNE:I
        pub fn JUNE() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "JULY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: JULY:I
        pub fn JULY() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "AUGUST", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: AUGUST:I
        pub fn AUGUST() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "SEPTEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: SEPTEMBER:I
        pub fn SEPTEMBER() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "OCTOBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: OCTOBER:I
        pub fn OCTOBER() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "NOVEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: NOVEMBER:I
        pub fn NOVEMBER() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "DECEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: DECEMBER:I
        pub fn DECEMBER() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "UNDECIMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: UNDECIMBER:I
        pub fn UNDECIMBER() -> i32 {
            12
        }

        #[cfg_attr(any(), java_field(name = "AM", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: AM:I
        pub fn AM() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "PM", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: PM:I
        pub fn PM() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "ALL_STYLES", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: ALL_STYLES:I
        pub fn ALL_STYLES() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "STANDALONE_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32768"))]
        // static field: STANDALONE_MASK:I
        pub fn STANDALONE_MASK() -> i32 {
            32768
        }

        #[cfg_attr(any(), java_field(name = "SHORT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: SHORT:I
        pub fn SHORT() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "LONG", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: LONG:I
        pub fn LONG() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "NARROW_FORMAT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: NARROW_FORMAT:I
        pub fn NARROW_FORMAT() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "NARROW_STANDALONE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32772"))]
        // static field: NARROW_STANDALONE:I
        pub fn NARROW_STANDALONE() -> i32 {
            32772
        }

        #[cfg_attr(any(), java_field(name = "SHORT_FORMAT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: SHORT_FORMAT:I
        pub fn SHORT_FORMAT() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "LONG_FORMAT", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: LONG_FORMAT:I
        pub fn LONG_FORMAT() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "SHORT_STANDALONE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32769"))]
        // static field: SHORT_STANDALONE:I
        pub fn SHORT_STANDALONE() -> i32 {
            32769
        }

        #[cfg_attr(any(), java_field(name = "LONG_STANDALONE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "32770"))]
        // static field: LONG_STANDALONE:I
        pub fn LONG_STANDALONE() -> i32 {
            32770
        }

        #[cfg_attr(any(), java_field(name = "cachedLocaleData", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/util/Locale;[I>;"))]
        // static field: cachedLocaleData:Ljava/util/concurrent/ConcurrentMap;
        pub fn cachedLocaleData() -> Object {
            panic!("stub: java/util/Calendar.cachedLocaleData:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "UNSET", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: UNSET:I
        pub fn UNSET() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "COMPUTED", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: COMPUTED:I
        pub fn COMPUTED() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "MINIMUM_USER_STAMP", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: MINIMUM_USER_STAMP:I
        pub fn MINIMUM_USER_STAMP() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ALL_FIELDS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "131071"))]
        // static field: ALL_FIELDS:I
        pub fn ALL_FIELDS() -> i32 {
            131071
        }

        #[cfg_attr(any(), java_field(name = "currentSerialVersion", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: currentSerialVersion:I
        pub fn currentSerialVersion() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-1807547505821590642"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -1807547505821590642i64
        }

        #[cfg_attr(any(), java_field(name = "ERA_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: ERA_MASK:I
        pub fn ERA_MASK() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "YEAR_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: YEAR_MASK:I
        pub fn YEAR_MASK() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "MONTH_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: MONTH_MASK:I
        pub fn MONTH_MASK() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "WEEK_OF_YEAR_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: WEEK_OF_YEAR_MASK:I
        pub fn WEEK_OF_YEAR_MASK() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "WEEK_OF_MONTH_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: WEEK_OF_MONTH_MASK:I
        pub fn WEEK_OF_MONTH_MASK() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_MONTH_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32"))]
        // static field: DAY_OF_MONTH_MASK:I
        pub fn DAY_OF_MONTH_MASK() -> i32 {
            32
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_YEAR_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "64"))]
        // static field: DAY_OF_YEAR_MASK:I
        pub fn DAY_OF_YEAR_MASK() -> i32 {
            64
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_WEEK_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "128"))]
        // static field: DAY_OF_WEEK_MASK:I
        pub fn DAY_OF_WEEK_MASK() -> i32 {
            128
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_WEEK_IN_MONTH_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "256"))]
        // static field: DAY_OF_WEEK_IN_MONTH_MASK:I
        pub fn DAY_OF_WEEK_IN_MONTH_MASK() -> i32 {
            256
        }

        #[cfg_attr(any(), java_field(name = "AM_PM_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: AM_PM_MASK:I
        pub fn AM_PM_MASK() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "HOUR_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1024"))]
        // static field: HOUR_MASK:I
        pub fn HOUR_MASK() -> i32 {
            1024
        }

        #[cfg_attr(any(), java_field(name = "HOUR_OF_DAY_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2048"))]
        // static field: HOUR_OF_DAY_MASK:I
        pub fn HOUR_OF_DAY_MASK() -> i32 {
            2048
        }

        #[cfg_attr(any(), java_field(name = "MINUTE_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4096"))]
        // static field: MINUTE_MASK:I
        pub fn MINUTE_MASK() -> i32 {
            4096
        }

        #[cfg_attr(any(), java_field(name = "SECOND_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8192"))]
        // static field: SECOND_MASK:I
        pub fn SECOND_MASK() -> i32 {
            8192
        }

        #[cfg_attr(any(), java_field(name = "MILLISECOND_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16384"))]
        // static field: MILLISECOND_MASK:I
        pub fn MILLISECOND_MASK() -> i32 {
            16384
        }

        #[cfg_attr(any(), java_field(name = "ZONE_OFFSET_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "32768"))]
        // static field: ZONE_OFFSET_MASK:I
        pub fn ZONE_OFFSET_MASK() -> i32 {
            32768
        }

        #[cfg_attr(any(), java_field(name = "DST_OFFSET_MASK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "65536"))]
        // static field: DST_OFFSET_MASK:I
        pub fn DST_OFFSET_MASK() -> i32 {
            65536
        }

        #[cfg_attr(any(), java_field(name = "FIELD_NAME", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: FIELD_NAME:[Ljava/lang/String;
        pub fn FIELD_NAME() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/util/Calendar.FIELD_NAME:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            let _t0: TimeZone = TimeZone::getDefaultRef()?;
            let _t1: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            this = Calendar::new_timezo_locale(Clone::clone(&_t0), Clone::clone(&_t1))?;
            this.__set_sharedZone((1i32 != 0i32));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/TimeZone;Ljava/util/Locale;)V
        pub fn new_timezo_locale(mut zone: TimeZone, mut aLocale: Locale) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_lenient((1i32 != 0i32));
            this.__set_sharedZone((0i32 != 0i32));
            this.__set_nextStamp(2i32);
            this.__set_serialVersionOnStream(1i32);
            let mut _arr0: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 17i32 as usize]));
            this.__set_fields(Clone::clone(&_arr0));
            let mut _arr1: Rc<RefCell<Vec<bool>>> = Rc::new(RefCell::new(vec![false; 17i32 as usize]));
            this.__set_isSet(Clone::clone(&_arr1));
            let mut _arr2: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 17i32 as usize]));
            this.__set_stamp(Clone::clone(&_arr2));
            this.__set_zone(Clone::clone(&zone));
            this.setWeekCountData(Clone::clone(&aLocale))?;
            Ok(this)
        }

        #[java_method(name = "getInstance", descriptor = "()Ljava/util/Calendar;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance() -> Result<Calendar> {
            panic!("stub: java/util/Calendar.getInstance:()Ljava/util/Calendar;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/TimeZone;)Ljava/util/Calendar;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance_timezo(zone: TimeZone) -> Result<Calendar> {
            panic!("stub: java/util/Calendar.getInstance:(Ljava/util/TimeZone;)Ljava/util/Calendar;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/util/Calendar;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/util/Locale;)Ljava/util/Calendar;
        pub fn getInstance_locale(mut aLocale: Locale) -> Result<Calendar> {
            let _t0: TimeZone = Calendar::defaultTimeZone(Clone::clone(&aLocale))?;
            let _t1: Calendar = Calendar::createCalendar(Clone::clone(&_t0), Clone::clone(&aLocale))?;
            Ok(_t1)
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)Ljava/util/Calendar;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance_timezo_locale(zone: TimeZone, aLocale: Locale) -> Result<Calendar> {
            panic!("stub: java/util/Calendar.getInstance:(Ljava/util/TimeZone;Ljava/util/Locale;)Ljava/util/Calendar;")
        }

        #[java_method(name = "defaultTimeZone", descriptor = "(Ljava/util/Locale;)Ljava/util/TimeZone;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn defaultTimeZone(mut l: Locale) -> Result<TimeZone> {
            let _t0: TimeZone = TimeZone::getDefault()?;
            let mut defaultTZ: TimeZone = _t0;
            let _t1 = l.getUnicodeLocaleType(Clone::clone(&String::from("tz")))?;
            let mut shortTZID: String = _t1;
            let mut _merged5: TimeZone;
            if !_is_jnull(&shortTZID) {
                let _t2: Optional<Object> = TimeZoneNameUtility::convertLDMLShortID(Clone::clone(&shortTZID))?;
                let __lam_85: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TimeZone::getTimeZone(_la0) });
                let _t3 = _t2.map(Clone::clone(&Object::from_any(__lam_85)))?;
                let _t4 = _t3.orElse(Object::from_any(defaultTZ.clone()))?;
                _merged5 = (_t4).downcast::<TimeZone>();
            } else {
                _merged5 = defaultTZ;
            }
            Ok(_merged5)
        }

        #[java_method(name = "createCalendar", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)Ljava/util/Calendar;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createCalendar(zone: TimeZone, aLocale: Locale) -> Result<Calendar> {
            panic!("stub: java/util/Calendar.createCalendar:(Ljava/util/TimeZone;Ljava/util/Locale;)Ljava/util/Calendar;")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/util/Calendar.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "computeTime", descriptor = "()V", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn computeTime(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.computeTime:()V")
        }

        #[java_method(name = "computeFields", descriptor = "()V", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn computeFields(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.computeFields:()V")
        }

        #[java_method(name = "getTime", descriptor = "()Ljava/util/Date;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTime(&self) -> Result<Date> {
            panic!("stub: java/util/Calendar.getTime:()Ljava/util/Date;")
        }

        #[java_method(name = "setTime", descriptor = "(Ljava/util/Date;)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTime(&self, mut date: Date) -> Result<()> {
            let this = self;
            let _t0: Object = Objects::requireNonNull_obj_str(Object::from_any(date.clone()), Clone::clone(&String::from("date must not be null")))?;
            let _t1 = date.getTime()?;
            this.setTimeInMillis(_t1)?;
            Ok(())
        }

        #[java_method(name = "getTimeInMillis", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeInMillis(&self) -> Result<i64> {
            let this = self;
            if !(this.__get_isTimeSet()) {
                this.updateTime()?;
            }
            Ok(this.__get_time())
        }

        #[java_method(name = "setTimeInMillis", descriptor = "(J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeInMillis(&self, mut millis: i64) -> Result<()> {
            let this = self;
            let _t0 = Default::default().isDirty()?;
            if !(_t0) {
                return Ok(());
            }
            this.__set_time(millis);
            this.__set_isTimeSet((1i32 != 0i32));
            this.__set_areFieldsSet((0i32 != 0i32));
            this.computeFields()?;
            this.__set_areFieldsSet((1i32 != 0i32));
            this.__set_areAllFieldsSet((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "get", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, mut field: i32) -> Result<i32> {
            let this = self;
            this.complete()?;
            let _t0 = this.internalGet(field)?;
            Ok(_t0)
        }

        #[java_method(name = "internalGet", descriptor = "(I)I", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn internalGet(&self, mut field: i32) -> Result<i32> {
            let this = self;
            Ok(this.__get_fields().borrow()[field as usize])
        }

        #[java_method(name = "internalSet", descriptor = "(II)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn internalSet(&self, field: i32, value: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.internalSet:(II)V")
        }

        #[java_method(name = "set", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_i_i(&self, field: i32, value: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.set:(II)V")
        }

        #[java_method(name = "set", descriptor = "(III)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_i_i_i(&self, year: i32, month: i32, date: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.set:(III)V")
        }

        #[java_method(name = "set", descriptor = "(IIIII)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_i_i_i_i_i(&self, year: i32, month: i32, date: i32, hourOfDay: i32, minute: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.set:(IIIII)V")
        }

        #[java_method(name = "set", descriptor = "(IIIIII)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn set_i_i_i_i_i_i(&self, year: i32, month: i32, date: i32, hourOfDay: i32, minute: i32, second: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.set:(IIIIII)V")
        }

        #[java_method(name = "clear", descriptor = "()V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.clear:()V")
        }

        #[java_method(name = "clear", descriptor = "(I)V", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clear_i(&self, field: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.clear:(I)V")
        }

        #[java_method(name = "isSet", descriptor = "(I)Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSet(&self, field: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isSet:(I)Z")
        }

        #[java_method(name = "getDisplayName", descriptor = "(IILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, field: i32, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: java/util/Calendar.getDisplayName:(IILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayNames", descriptor = "(IILjava/util/Locale;)Ljava/util/Map;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn getDisplayNames(&self, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: java/util/Calendar.getDisplayNames:(IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "getDisplayNamesImpl", descriptor = "(IILjava/util/Locale;)Ljava/util/Map;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn getDisplayNamesImpl(&self, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: java/util/Calendar.getDisplayNamesImpl:(IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "checkDisplayNameParams", descriptor = "(IIIILjava/util/Locale;I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkDisplayNameParams(&self, field: i32, style: i32, minStyle: i32, maxStyle: i32, locale: Locale, fieldMask: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.checkDisplayNameParams:(IIIILjava/util/Locale;I)Z")
        }

        #[java_method(name = "getFieldStrings", descriptor = "(IILjava/text/DateFormatSymbols;)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFieldStrings(&self, field: i32, style: i32, symbols: DateFormatSymbols) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/Calendar.getFieldStrings:(IILjava/text/DateFormatSymbols;)[Ljava/lang/String;")
        }

        #[java_method(name = "complete", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn complete(&self) -> Result<()> {
            let this = self;
            if !(this.__get_isTimeSet()) {
                this.updateTime()?;
            }
            if !(this.__get_areAllFieldsSet()) {
                this.computeFields()?;
                this.__set_areFieldsSet((1i32 != 0i32));
                this.__set_areAllFieldsSet((1i32 != 0i32));
            }
            Ok(())
        }

        #[java_method(name = "isExternallySet", descriptor = "(I)Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isExternallySet(&self, field: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isExternallySet:(I)Z")
        }

        #[java_method(name = "getSetStateFields", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSetStateFields(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.getSetStateFields:()I")
        }

        #[java_method(name = "setFieldsComputed", descriptor = "(I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setFieldsComputed(&self, fieldMask: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.setFieldsComputed:(I)V")
        }

        #[java_method(name = "setFieldsNormalized", descriptor = "(I)V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setFieldsNormalized(&self, fieldMask: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.setFieldsNormalized:(I)V")
        }

        #[java_method(name = "isPartiallyNormalized", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isPartiallyNormalized(&self) -> Result<bool> {
            panic!("stub: java/util/Calendar.isPartiallyNormalized:()Z")
        }

        #[java_method(name = "isFullyNormalized", descriptor = "()Z", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFullyNormalized(&self) -> Result<bool> {
            panic!("stub: java/util/Calendar.isFullyNormalized:()Z")
        }

        #[java_method(name = "setUnnormalized", descriptor = "()V", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setUnnormalized(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.setUnnormalized:()V")
        }

        #[java_method(name = "isFieldSet", descriptor = "(II)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isFieldSet(fieldMask: i32, field: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isFieldSet:(II)Z")
        }

        #[java_method(name = "selectFields", descriptor = "()I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn selectFields(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.selectFields:()I")
        }

        #[java_method(name = "getBaseStyle", descriptor = "(I)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBaseStyle(&self, style: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getBaseStyle:(I)I")
        }

        #[java_method(name = "toStandaloneStyle", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toStandaloneStyle(&self, style: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.toStandaloneStyle:(I)I")
        }

        #[java_method(name = "isStandaloneStyle", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStandaloneStyle(&self, style: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isStandaloneStyle:(I)Z")
        }

        #[java_method(name = "isNarrowStyle", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNarrowStyle(&self, style: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isNarrowStyle:(I)Z")
        }

        #[java_method(name = "isNarrowFormatStyle", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNarrowFormatStyle(&self, style: i32) -> Result<bool> {
            panic!("stub: java/util/Calendar.isNarrowFormatStyle:(I)Z")
        }

        #[java_method(name = "aggregateStamp", descriptor = "(II)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn aggregateStamp(stamp_a: i32, stamp_b: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.aggregateStamp:(II)I")
        }

        #[java_method(name = "getAvailableCalendarTypes", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getAvailableCalendarTypes() -> Result<Object> {
            panic!("stub: java/util/Calendar.getAvailableCalendarTypes:()Ljava/util/Set;")
        }

        #[java_method(name = "getCalendarType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarType(&self) -> Result<String> {
            panic!("stub: java/util/Calendar.getCalendarType:()Ljava/lang/String;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/Calendar.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "before", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn before(&self, when: Object) -> Result<bool> {
            panic!("stub: java/util/Calendar.before:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "after", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn after(&self, when: Object) -> Result<bool> {
            panic!("stub: java/util/Calendar.after:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/util/Calendar;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo_calend(&self, anotherCalendar: Calendar) -> Result<i32> {
            panic!("stub: java/util/Calendar.compareTo:(Ljava/util/Calendar;)I")
        }

        #[java_method(name = "add", descriptor = "(II)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn add(&self, arg0: i32, arg1: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.add:(II)V")
        }

        #[java_method(name = "roll", descriptor = "(IZ)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn roll_i_z(&self, arg0: i32, arg1: bool) -> Result<()> {
            panic!("stub: java/util/Calendar.roll:(IZ)V")
        }

        #[java_method(name = "roll", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll_i_i(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.roll:(II)V")
        }

        #[java_method(name = "setTimeZone", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeZone(&self, value: TimeZone) -> Result<()> {
            panic!("stub: java/util/Calendar.setTimeZone:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "getTimeZone", descriptor = "()Ljava/util/TimeZone;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZone(&self) -> Result<TimeZone> {
            let this = self;
            if this.__get_sharedZone() {
                let _t0: Object = Object::from_any(this.__get_zone().clone());
                this.__set_zone(Clone::clone(&(_t0).downcast::<TimeZone>()));
                this.__set_sharedZone((0i32 != 0i32));
            }
            Ok(this.__get_zone())
        }

        #[java_method(name = "getZone", descriptor = "()Ljava/util/TimeZone;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZone(&self) -> Result<TimeZone> {
            panic!("stub: java/util/Calendar.getZone:()Ljava/util/TimeZone;")
        }

        #[java_method(name = "setZoneShared", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setZoneShared(&self, shared: bool) -> Result<()> {
            panic!("stub: java/util/Calendar.setZoneShared:(Z)V")
        }

        #[java_method(name = "setLenient", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLenient(&self, mut lenient: bool) -> Result<()> {
            let this = self;
            this.__set_lenient(lenient);
            Ok(())
        }

        #[java_method(name = "isLenient", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLenient(&self) -> Result<bool> {
            panic!("stub: java/util/Calendar.isLenient:()Z")
        }

        #[java_method(name = "setFirstDayOfWeek", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setFirstDayOfWeek(&self, value: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.setFirstDayOfWeek:(I)V")
        }

        #[java_method(name = "getFirstDayOfWeek", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFirstDayOfWeek(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.getFirstDayOfWeek:()I")
        }

        #[java_method(name = "setMinimalDaysInFirstWeek", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinimalDaysInFirstWeek(&self, value: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.setMinimalDaysInFirstWeek:(I)V")
        }

        #[java_method(name = "getMinimalDaysInFirstWeek", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimalDaysInFirstWeek(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.getMinimalDaysInFirstWeek:()I")
        }

        #[java_method(name = "isWeekDateSupported", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWeekDateSupported(&self) -> Result<bool> {
            panic!("stub: java/util/Calendar.isWeekDateSupported:()Z")
        }

        #[java_method(name = "getWeekYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeekYear(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.getWeekYear:()I")
        }

        #[java_method(name = "setWeekDate", descriptor = "(III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setWeekDate(&self, weekYear: i32, weekOfYear: i32, dayOfWeek: i32) -> Result<()> {
            panic!("stub: java/util/Calendar.setWeekDate:(III)V")
        }

        #[java_method(name = "getWeeksInWeekYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeeksInWeekYear(&self) -> Result<i32> {
            panic!("stub: java/util/Calendar.getWeeksInWeekYear:()I")
        }

        #[java_method(name = "getMinimum", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getMinimum(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getMinimum:(I)I")
        }

        #[java_method(name = "getMaximum", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getMaximum(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getMaximum:(I)I")
        }

        #[java_method(name = "getGreatestMinimum", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getGreatestMinimum(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getGreatestMinimum:(I)I")
        }

        #[java_method(name = "getLeastMaximum", descriptor = "(I)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getLeastMaximum(&self, arg0: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getLeastMaximum:(I)I")
        }

        #[java_method(name = "getActualMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getActualMinimum:(I)I")
        }

        #[java_method(name = "getActualMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/Calendar.getActualMaximum:(I)I")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/Calendar.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "getFieldName", descriptor = "(I)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFieldName(field: i32) -> Result<String> {
            panic!("stub: java/util/Calendar.getFieldName:(I)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "appendValue", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;ZJ)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendValue(sb: StringBuilder, item: String, valid: bool, value: i64) -> Result<()> {
            panic!("stub: java/util/Calendar.appendValue:(Ljava/lang/StringBuilder;Ljava/lang/String;ZJ)V")
        }

        #[java_method(name = "setWeekCountData", descriptor = "(Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setWeekCountData(&self, mut desiredLocale: Locale) -> Result<()> {
            let this = self;
            let _vdispatch0: Object = if let Some(_d) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(desiredLocale.clone()))? } else if let Some(_d) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(desiredLocale.clone()))? } else if let Some(__f) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(desiredLocale.clone()))? } else { Default::default() };
            let mut data = (_vdispatch0).downcast::<Rc<RefCell<Vec<i32>>>>();
            if _is_jnull(&data) {
                let mut _arr1: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![0i32; 2i32 as usize]));
                data = _arr1;
                let _t2: i32 = CalendarDataUtility::retrieveFirstDayOfWeek(Clone::clone(&desiredLocale))?;
                data.borrow_mut()[0i32 as usize] = _t2;
                let _t3: i32 = CalendarDataUtility::retrieveMinimalDaysInFirstWeek(Clone::clone(&desiredLocale))?;
                data.borrow_mut()[1i32 as usize] = _t3;
                let _vdispatch4: Object = if let Some(_d) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.putIfAbsent(Object::from_any(desiredLocale.clone()), Object::from_any(data.clone()))? } else if let Some(_d) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<Object>() { _d.putIfAbsent(Object::from_any(desiredLocale.clone()), Object::from_any(data.clone()))? } else if let Some(__f) = Calendar::cachedLocaleData().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(desiredLocale.clone()), Object::from_any(data.clone()))? } else { Default::default() };
            }
            this.__set_firstDayOfWeek(data.borrow()[0i32 as usize]);
            this.__set_minimalDaysInFirstWeek(data.borrow()[1i32 as usize]);
            Ok(())
        }

        #[java_method(name = "updateTime", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateTime(&self) -> Result<()> {
            let this = self;
            this.computeTime()?;
            this.__set_isTimeSet((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "compareTo", descriptor = "(J)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo_l(&self, t: i64) -> Result<i32> {
            panic!("stub: java/util/Calendar.compareTo:(J)I")
        }

        #[java_method(name = "getMillisOf", descriptor = "(Ljava/util/Calendar;)J", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMillisOf(calendar: Calendar) -> Result<i64> {
            panic!("stub: java/util/Calendar.getMillisOf:(Ljava/util/Calendar;)J")
        }

        #[java_method(name = "adjustStamp", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustStamp(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.adjustStamp:()V")
        }

        #[java_method(name = "invalidateWeekFields", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn invalidateWeekFields(&self) -> Result<()> {
            panic!("stub: java/util/Calendar.invalidateWeekFields:()V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/util/Calendar.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/util/Calendar.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "toInstant", descriptor = "()Ljava/time/Instant;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toInstant(&self) -> Result<Instant> {
            panic!("stub: java/util/Calendar.toInstant:()Ljava/time/Instant;")
        }
    }
}
