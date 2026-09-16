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
    #[binary_name       = "sun/util/calendar/CalendarDate"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CalendarDate.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Cloneable;java/lang/Object;sun/util/calendar/CalendarDate"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct CalendarDate {
        #[cfg_attr(any(), java_field(name = "era", descriptor = "Lsun/util/calendar/Era;", access = "private", modifiers = "", is_static = false))]
        pub era: Object,
        #[cfg_attr(any(), java_field(name = "year", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub year: i32,
        #[cfg_attr(any(), java_field(name = "month", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub month: i32,
        #[cfg_attr(any(), java_field(name = "dayOfMonth", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub dayOfMonth: i32,
        #[cfg_attr(any(), java_field(name = "dayOfWeek", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub dayOfWeek: i32,
        #[cfg_attr(any(), java_field(name = "leapYear", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub leapYear: bool,
        #[cfg_attr(any(), java_field(name = "hours", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub hours: i32,
        #[cfg_attr(any(), java_field(name = "minutes", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minutes: i32,
        #[cfg_attr(any(), java_field(name = "seconds", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub seconds: i32,
        #[cfg_attr(any(), java_field(name = "millis", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub millis: i32,
        #[cfg_attr(any(), java_field(name = "fraction", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub fraction: i64,
        #[cfg_attr(any(), java_field(name = "normalized", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub normalized: bool,
        #[cfg_attr(any(), java_field(name = "zoneinfo", descriptor = "Ljava/util/TimeZone;", access = "private", modifiers = "", is_static = false))]
        pub zoneinfo: TimeZone,
        #[cfg_attr(any(), java_field(name = "zoneOffset", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub zoneOffset: i32,
        #[cfg_attr(any(), java_field(name = "daylightSaving", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub daylightSaving: i32,
        #[cfg_attr(any(), java_field(name = "forceStandardTime", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub forceStandardTime: bool,
        #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "", is_static = false))]
        pub locale: Locale,
    }

    impl CalendarDate {
        #[cfg_attr(any(), java_field(name = "FIELD_UNDEFINED", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "-2147483648"))]
        // static field: FIELD_UNDEFINED:I
        pub fn FIELD_UNDEFINED() -> i32 {
            -2147483648
        }

        #[cfg_attr(any(), java_field(name = "TIME_UNDEFINED", descriptor = "J", access = "public", modifiers = "static final", is_static = true, constant_value = "-9223372036854775808"))]
        // static field: TIME_UNDEFINED:J
        pub fn TIME_UNDEFINED() -> i64 {
            -9223372036854775808i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/CalendarDate.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo(zone: TimeZone) -> Result<Self> {
            panic!("stub: sun/util/calendar/CalendarDate.<init>:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "getEra", descriptor = "()Lsun/util/calendar/Era;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEra(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/CalendarDate.getEra:()Lsun/util/calendar/Era;")
        }

        #[java_method(name = "setEra", descriptor = "(Lsun/util/calendar/Era;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setEra(&self, era: Object) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setEra:(Lsun/util/calendar/Era;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYear(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getYear:()I")
        }

        #[java_method(name = "setYear", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setYear(&self, year: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setYear:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "addYear", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addYear(&self, n: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.addYear:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "isLeapYear", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.isLeapYear:()Z")
        }

        #[java_method(name = "setLeapYear", descriptor = "(Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLeapYear(&self, leapYear: bool) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setLeapYear:(Z)V")
        }

        #[java_method(name = "getMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonth(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getMonth:()I")
        }

        #[java_method(name = "setMonth", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMonth(&self, month: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setMonth:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "addMonth", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addMonth(&self, n: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.addMonth:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getDayOfMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfMonth(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getDayOfMonth:()I")
        }

        #[java_method(name = "setDayOfMonth", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDayOfMonth(&self, date: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setDayOfMonth:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getDayOfWeek", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeek(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getDayOfWeek:()I")
        }

        #[java_method(name = "getHours", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getHours(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getHours:()I")
        }

        #[java_method(name = "setHours", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setHours(&self, hours: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setHours:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "addHours", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addHours(&self, n: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.addHours:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getMinutes", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinutes(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getMinutes:()I")
        }

        #[java_method(name = "setMinutes", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinutes(&self, minutes: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setMinutes:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getSeconds", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSeconds(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getSeconds:()I")
        }

        #[java_method(name = "setSeconds", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setSeconds(&self, seconds: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setSeconds:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getMillis", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMillis(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getMillis:()I")
        }

        #[java_method(name = "setMillis", descriptor = "(I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMillis(&self, millis: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setMillis:(I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getTimeOfDay", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeOfDay(&self) -> Result<i64> {
            panic!("stub: sun/util/calendar/CalendarDate.getTimeOfDay:()J")
        }

        #[java_method(name = "setDate", descriptor = "(III)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDate(&self, year: i32, month: i32, dayOfMonth: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setDate:(III)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "setTimeOfDay", descriptor = "(IIII)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeOfDay_i_i_i_i(&self, hours: i32, minutes: i32, seconds: i32, millis: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setTimeOfDay:(IIII)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "setTimeOfDay", descriptor = "(J)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeOfDay_l(&self, fraction: i64) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setTimeOfDay:(J)V")
        }

        #[java_method(name = "isNormalized", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNormalized(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.isNormalized:()Z")
        }

        #[java_method(name = "isStandardTime", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isStandardTime(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.isStandardTime:()Z")
        }

        #[java_method(name = "isDaylightTime", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDaylightTime(&self) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.isDaylightTime:()Z")
        }

        #[java_method(name = "setLocale", descriptor = "(Ljava/util/Locale;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLocale(&self, loc: Locale) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setLocale:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "getZone", descriptor = "()Ljava/util/TimeZone;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZone(&self) -> Result<TimeZone> {
            panic!("stub: sun/util/calendar/CalendarDate.getZone:()Ljava/util/TimeZone;")
        }

        #[java_method(name = "setZone", descriptor = "(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setZone(&self, zoneinfo: TimeZone) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarDate.setZone:(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "isSameDate", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSameDate(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.isSameDate:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarDate.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/CalendarDate.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "setDayOfWeek", descriptor = "(I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDayOfWeek(&self, dayOfWeek: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setDayOfWeek:(I)V")
        }

        #[java_method(name = "setNormalized", descriptor = "(Z)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setNormalized(&self, normalized: bool) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setNormalized:(Z)V")
        }

        #[java_method(name = "getZoneOffset", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneOffset(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getZoneOffset:()I")
        }

        #[java_method(name = "setZoneOffset", descriptor = "(I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setZoneOffset(&self, offset: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setZoneOffset:(I)V")
        }

        #[java_method(name = "getDaylightSaving", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDaylightSaving(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarDate.getDaylightSaving:()I")
        }

        #[java_method(name = "setDaylightSaving", descriptor = "(I)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDaylightSaving(&self, daylightSaving: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarDate.setDaylightSaving:(I)V")
        }
    }
}
