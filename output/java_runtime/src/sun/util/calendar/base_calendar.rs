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

impl From<BaseCalendar> for AbstractCalendar {
    fn from(v: BaseCalendar) -> AbstractCalendar { v.__into_super() }
}

impl From<BaseCalendar> for CalendarSystem {
    fn from(v: BaseCalendar) -> CalendarSystem { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/BaseCalendar"]
    #[super_class       = "sun/util/calendar/AbstractCalendar"]
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
    #[superclass        = "AbstractCalendar"]
    #[superclass_fields(eras: Rc<RefCell<Vec<Object>>>)]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/AbstractCalendar;sun/util/calendar/BaseCalendar;sun/util/calendar/CalendarSystem"]

    pub struct BaseCalendar;

    impl BaseCalendar {
        #[cfg_attr(any(), java_field(name = "JANUARY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: JANUARY:I
        pub fn JANUARY() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "FEBRUARY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: FEBRUARY:I
        pub fn FEBRUARY() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "MARCH", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: MARCH:I
        pub fn MARCH() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "APRIL", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: APRIL:I
        pub fn APRIL() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "MAY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: MAY:I
        pub fn MAY() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "JUNE", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: JUNE:I
        pub fn JUNE() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "JULY", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: JULY:I
        pub fn JULY() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "AUGUST", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: AUGUST:I
        pub fn AUGUST() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "SEPTEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: SEPTEMBER:I
        pub fn SEPTEMBER() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "OCTOBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: OCTOBER:I
        pub fn OCTOBER() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "NOVEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: NOVEMBER:I
        pub fn NOVEMBER() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "DECEMBER", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: DECEMBER:I
        pub fn DECEMBER() -> i32 {
            12
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

        #[cfg_attr(any(), java_field(name = "BASE_YEAR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1970"))]
        // static field: BASE_YEAR:I
        pub fn BASE_YEAR() -> i32 {
            1970
        }

        #[cfg_attr(any(), java_field(name = "FIXED_DATES", descriptor = "[I", access = "private", modifiers = "static final", is_static = true))]
        // static field: FIXED_DATES:[I
        pub fn FIXED_DATES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/BaseCalendar.FIXED_DATES:[I")
        }

        #[cfg_attr(any(), java_field(name = "DAYS_IN_MONTH", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: DAYS_IN_MONTH:[I
        pub fn DAYS_IN_MONTH() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/BaseCalendar.DAYS_IN_MONTH:[I")
        }

        #[cfg_attr(any(), java_field(name = "ACCUMULATED_DAYS_IN_MONTH", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: ACCUMULATED_DAYS_IN_MONTH:[I
        pub fn ACCUMULATED_DAYS_IN_MONTH() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/BaseCalendar.ACCUMULATED_DAYS_IN_MONTH:[I")
        }

        #[cfg_attr(any(), java_field(name = "ACCUMULATED_DAYS_IN_MONTH_LEAP", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: ACCUMULATED_DAYS_IN_MONTH_LEAP:[I
        pub fn ACCUMULATED_DAYS_IN_MONTH_LEAP() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: sun/util/calendar/BaseCalendar.ACCUMULATED_DAYS_IN_MONTH_LEAP:[I")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/BaseCalendar.<init>:()V")
        }

        #[java_method(name = "validate", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validate(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar.validate:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "normalize", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar.normalize:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "normalizeMonth", descriptor = "(Lsun/util/calendar/CalendarDate;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeMonth(&self, date: CalendarDate) -> Result<()> {
            panic!("stub: sun/util/calendar/BaseCalendar.normalizeMonth:(Lsun/util/calendar/CalendarDate;)V")
        }

        #[java_method(name = "getYearLength", descriptor = "(Lsun/util/calendar/CalendarDate;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYearLength(&self, date: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getYearLength:(Lsun/util/calendar/CalendarDate;)I")
        }

        #[java_method(name = "getMonthLength", descriptor = "(Lsun/util/calendar/CalendarDate;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonthLength_calend(&self, date: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getMonthLength:(Lsun/util/calendar/CalendarDate;)I")
        }

        #[java_method(name = "getMonthLength", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonthLength_i_i(&self, year: i32, month: i32) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getMonthLength:(II)I")
        }

        #[java_method(name = "getDayOfYear", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfYear_calend(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/BaseCalendar.getDayOfYear:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getDayOfYear", descriptor = "(III)J", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfYear_i_i_i(&self, year: i32, month: i32, dayOfMonth: i32) -> Result<i64> {
            panic!("stub: sun/util/calendar/BaseCalendar.getDayOfYear:(III)J")
        }

        #[java_method(name = "getFixedDate", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDate_calend(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/BaseCalendar.getFixedDate:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getFixedDate", descriptor = "(IIILsun/util/calendar/BaseCalendar$Date;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDate_i_i_i_baseca(&self, year: i32, month: i32, dayOfMonth: i32, cache: BaseCalendar_Date) -> Result<i64> {
            panic!("stub: sun/util/calendar/BaseCalendar.getFixedDate:(IIILsun/util/calendar/BaseCalendar$Date;)J")
        }

        #[java_method(name = "getCalendarDateFromFixedDate", descriptor = "(Lsun/util/calendar/CalendarDate;J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDateFromFixedDate(&self, date: CalendarDate, fixedDate: i64) -> Result<()> {
            panic!("stub: sun/util/calendar/BaseCalendar.getCalendarDateFromFixedDate:(Lsun/util/calendar/CalendarDate;J)V")
        }

        #[java_method(name = "getDayOfWeek", descriptor = "(Lsun/util/calendar/CalendarDate;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeek(&self, date: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getDayOfWeek:(Lsun/util/calendar/CalendarDate;)I")
        }

        #[java_method(name = "getDayOfWeekFromFixedDate", descriptor = "(J)I", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeekFromFixedDate(fixedDate: i64) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getDayOfWeekFromFixedDate:(J)I")
        }

        #[java_method(name = "getYearFromFixedDate", descriptor = "(J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYearFromFixedDate(&self, fixedDate: i64) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getYearFromFixedDate:(J)I")
        }

        #[java_method(name = "getGregorianYearFromFixedDate", descriptor = "(J)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGregorianYearFromFixedDate(&self, fixedDate: i64) -> Result<i32> {
            panic!("stub: sun/util/calendar/BaseCalendar.getGregorianYearFromFixedDate:(J)I")
        }

        #[java_method(name = "isLeapYear", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear_calend(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar.isLeapYear:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "isLeapYear", descriptor = "(I)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear_i(&self, normalizedYear: i32) -> Result<bool> {
            panic!("stub: sun/util/calendar/BaseCalendar.isLeapYear:(I)Z")
        }
    }
}
