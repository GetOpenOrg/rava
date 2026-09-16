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

impl From<AbstractCalendar> for CalendarSystem {
    fn from(v: AbstractCalendar) -> CalendarSystem { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/AbstractCalendar"]
    #[super_class       = "sun/util/calendar/CalendarSystem"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "AbstractCalendar.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "CalendarSystem"]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/AbstractCalendar;sun/util/calendar/CalendarSystem"]

    pub struct AbstractCalendar {
        #[cfg_attr(any(), java_field(name = "eras", descriptor = "[Lsun/util/calendar/Era;", access = "private", modifiers = "", is_static = false))]
        pub eras: Rc<RefCell<Vec<Object>>>,
    }

    impl AbstractCalendar {
        #[cfg_attr(any(), java_field(name = "SECOND_IN_MILLIS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1000"))]
        // static field: SECOND_IN_MILLIS:I
        pub fn SECOND_IN_MILLIS() -> i32 {
            1000
        }

        #[cfg_attr(any(), java_field(name = "MINUTE_IN_MILLIS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "60000"))]
        // static field: MINUTE_IN_MILLIS:I
        pub fn MINUTE_IN_MILLIS() -> i32 {
            60000
        }

        #[cfg_attr(any(), java_field(name = "HOUR_IN_MILLIS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3600000"))]
        // static field: HOUR_IN_MILLIS:I
        pub fn HOUR_IN_MILLIS() -> i32 {
            3600000
        }

        #[cfg_attr(any(), java_field(name = "DAY_IN_MILLIS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "86400000"))]
        // static field: DAY_IN_MILLIS:I
        pub fn DAY_IN_MILLIS() -> i32 {
            86400000
        }

        #[cfg_attr(any(), java_field(name = "EPOCH_OFFSET", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "719163"))]
        // static field: EPOCH_OFFSET:I
        pub fn EPOCH_OFFSET() -> i32 {
            719163
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/AbstractCalendar.<init>:()V")
        }

        #[java_method(name = "getEra", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/Era;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEra(&self, eraName: String) -> Result<Object> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getEra:(Ljava/lang/String;)Lsun/util/calendar/Era;")
        }

        #[java_method(name = "getEras", descriptor = "()[Lsun/util/calendar/Era;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEras(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getEras:()[Lsun/util/calendar/Era;")
        }

        #[java_method(name = "setEras", descriptor = "([Lsun/util/calendar/Era;)V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setEras(&self, eras: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: sun/util/calendar/AbstractCalendar.setEras:([Lsun/util/calendar/Era;)V")
        }

        #[java_method(name = "getCalendarDate", descriptor = "()Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(&self) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getCalendarDate:()Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l(&self, millis: i64) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getCalendarDate:(J)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLjava/util/TimeZone;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_timezo(&self, millis: i64, arg1: TimeZone) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getCalendarDate:(JLjava/util/TimeZone;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_calend(&self, millis: i64, arg1: CalendarDate) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getCalendarDate:(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getTime", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTime(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getTime:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getTimeOfDay", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeOfDay(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getTimeOfDay:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getTimeOfDayValue", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeOfDayValue(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getTimeOfDayValue:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "setTimeOfDay", descriptor = "(Lsun/util/calendar/CalendarDate;I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeOfDay(&self, cdate: CalendarDate, fraction: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.setTimeOfDay:(Lsun/util/calendar/CalendarDate;I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "isLeapYear", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn isLeapYear(&self, arg0: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/AbstractCalendar.isLeapYear:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "getNthDayOfWeek", descriptor = "(IILsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNthDayOfWeek(&self, nth: i32, dayOfWeek: i32, date: CalendarDate) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getNthDayOfWeek:(IILsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getDayOfWeekDateBefore", descriptor = "(JI)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeekDateBefore(fixedDate: i64, arg1: i32) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getDayOfWeekDateBefore:(JI)J")
        }

        #[java_method(name = "getDayOfWeekDateAfter", descriptor = "(JI)J", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeekDateAfter(fixedDate: i64, arg1: i32) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getDayOfWeekDateAfter:(JI)J")
        }

        #[java_method(name = "getDayOfWeekDateOnOrBefore", descriptor = "(JI)J", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeekDateOnOrBefore(fixedDate: i64, arg1: i32) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getDayOfWeekDateOnOrBefore:(JI)J")
        }

        #[java_method(name = "getFixedDate", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getFixedDate(&self, arg0: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getFixedDate:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getCalendarDateFromFixedDate", descriptor = "(Lsun/util/calendar/CalendarDate;J)V", access = "protected", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDateFromFixedDate(&self, arg0: CalendarDate, arg1: i64) -> Result<()> {
            panic!("stub: sun/util/calendar/AbstractCalendar.getCalendarDateFromFixedDate:(Lsun/util/calendar/CalendarDate;J)V")
        }

        #[java_method(name = "validateTime", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validateTime(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/AbstractCalendar.validateTime:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "normalizeTime", descriptor = "(Lsun/util/calendar/CalendarDate;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeTime(&self, date: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/AbstractCalendar.normalizeTime:(Lsun/util/calendar/CalendarDate;)I")
        }
    }
}
