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

impl From<BaseCalendar_Date> for CalendarDate {
    fn from(v: BaseCalendar_Date) -> CalendarDate { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/BaseCalendar$Date"]
    #[super_class       = "sun/util/calendar/CalendarDate"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BaseCalendar.java"]
    #[inner_classes     = "sun/util/calendar/BaseCalendar$Date:sun/util/calendar/BaseCalendar:Date:1033"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CalendarDate"]
    #[superclass_fields(era: Object, year: i32, month: i32, dayOfMonth: i32, dayOfWeek: i32, leapYear: bool, hours: i32, minutes: i32, seconds: i32, millis: i32, fraction: i64, normalized: bool, zoneinfo: TimeZone, zoneOffset: i32, daylightSaving: i32, forceStandardTime: bool, locale: Locale)]
    #[all_supertypes    = "java/lang/Cloneable;java/lang/Object;sun/util/calendar/BaseCalendar$Date;sun/util/calendar/CalendarDate"]

    pub struct BaseCalendar_Date {
        #[cfg_attr(any(), java_field(name = "cachedYear", descriptor = "I", is_static = false))]
        pub cachedYear: i32,
        #[cfg_attr(any(), java_field(name = "cachedFixedDateJan1", descriptor = "J", is_static = false))]
        pub cachedFixedDateJan1: i64,
        #[cfg_attr(any(), java_field(name = "cachedFixedDateNextJan1", descriptor = "J", is_static = false))]
        pub cachedFixedDateNextJan1: i64,
    }

    impl BaseCalendar_Date {
        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo(zone: TimeZone) -> Result<Self> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.<init>:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "setNormalizedDate", descriptor = "(III)Lsun/util/calendar/BaseCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setNormalizedDate(&self, normalizedYear: i32, month: i32, dayOfMonth: i32) -> Result<BaseCalendar_Date> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.setNormalizedDate:(III)Lsun/util/calendar/BaseCalendar$Date;")
        }

        #[java_method(name = "getNormalizedYear", descriptor = "()I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getNormalizedYear(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.getNormalizedYear:()I")
        }

        #[java_method(name = "setNormalizedYear", descriptor = "(I)V", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn setNormalizedYear(&self, arg0: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.setNormalizedYear:(I)V")
        }

        #[java_method(name = "hit", descriptor = "(I)Z", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hit_i(&self, year: i32) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.hit:(I)Z")
        }

        #[java_method(name = "hit", descriptor = "(J)Z", access = "protected", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hit_l(&self, fixedDate: i64) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.hit:(J)Z")
        }

        #[java_method(name = "getCachedYear", descriptor = "()I", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCachedYear(&self) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.getCachedYear:()I")
        }

        #[java_method(name = "getCachedJan1", descriptor = "()J", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCachedJan1(&self) -> Result<i64> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.getCachedJan1:()J")
        }

        #[java_method(name = "setCache", descriptor = "(IJI)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCache(&self, year: i32, jan1: i64, arg2: i32) -> Result<()> {
            panic!("stub: sun/util/calendar/BaseCalendar$Date.setCache:(IJI)V")
        }
    }
}
