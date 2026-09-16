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
    #[binary_name       = "sun/util/calendar/CalendarSystem"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CalendarSystem.java"]
    #[inner_classes     = "sun/util/calendar/CalendarSystem$GregorianHolder:sun/util/calendar/CalendarSystem:GregorianHolder:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/CalendarSystem"]

    pub struct CalendarSystem;

    impl CalendarSystem {
        #[cfg_attr(any(), java_field(name = "initialized", descriptor = "Z", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: initialized:Z
        pub fn initialized() -> bool {
            panic!("stub: sun/util/calendar/CalendarSystem.initialized:Z")
        }

        #[cfg_attr(any(), java_field(name = "names", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/String;Ljava/lang/String;>;"))]
        // static field: names:Ljava/util/concurrent/ConcurrentMap;
        pub fn names() -> Object {
            panic!("stub: sun/util/calendar/CalendarSystem.names:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "calendars", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/String;Lsun/util/calendar/CalendarSystem;>;"))]
        // static field: calendars:Ljava/util/concurrent/ConcurrentMap;
        pub fn calendars() -> Object {
            panic!("stub: sun/util/calendar/CalendarSystem.calendars:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "PACKAGE_NAME", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "sun.util.calendar."))]
        // static field: PACKAGE_NAME:Ljava/lang/String;
        pub fn PACKAGE_NAME() -> String {
            String::from("sun.util.calendar.")
        }

        #[cfg_attr(any(), java_field(name = "namePairs", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true))]
        // static field: namePairs:[Ljava/lang/String;
        pub fn namePairs() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: sun/util/calendar/CalendarSystem.namePairs:[Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/CalendarSystem.<init>:()V")
        }

        #[java_method(name = "initNames", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initNames() -> Result<()> {
            panic!("stub: sun/util/calendar/CalendarSystem.initNames:()V")
        }

        #[java_method(name = "getGregorianCalendar", descriptor = "()Lsun/util/calendar/Gregorian;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGregorianCalendar() -> Result<Gregorian> {
            panic!("stub: sun/util/calendar/CalendarSystem.getGregorianCalendar:()Lsun/util/calendar/Gregorian;")
        }

        #[java_method(name = "forName", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/CalendarSystem;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forName(calendarName: String) -> Result<CalendarSystem> {
            panic!("stub: sun/util/calendar/CalendarSystem.forName:(Ljava/lang/String;)Lsun/util/calendar/CalendarSystem;")
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: sun/util/calendar/CalendarSystem.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "()Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDate(&self) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.getCalendarDate:()Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDate_l(&self, arg0: i64) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.getCalendarDate:(J)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDate_l_calend(&self, arg0: i64, arg1: CalendarDate) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.getCalendarDate:(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLjava/util/TimeZone;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDate_l_timezo(&self, arg0: i64, arg1: TimeZone) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.getCalendarDate:(JLjava/util/TimeZone;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "()Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn newCalendarDate(&self) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.newCalendarDate:()Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn newCalendarDate_timezo(&self, arg0: TimeZone) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.newCalendarDate:(Ljava/util/TimeZone;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "getTime", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getTime(&self, arg0: CalendarDate) -> Result<i64> {
            panic!("stub: sun/util/calendar/CalendarSystem.getTime:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "getYearLength", descriptor = "(Lsun/util/calendar/CalendarDate;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getYearLength(&self, arg0: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarSystem.getYearLength:(Lsun/util/calendar/CalendarDate;)I")
        }

        #[java_method(name = "getMonthLength", descriptor = "(Lsun/util/calendar/CalendarDate;)I", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getMonthLength(&self, arg0: CalendarDate) -> Result<i32> {
            panic!("stub: sun/util/calendar/CalendarSystem.getMonthLength:(Lsun/util/calendar/CalendarDate;)I")
        }

        #[java_method(name = "getEra", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/Era;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getEra(&self, arg0: String) -> Result<Object> {
            panic!("stub: sun/util/calendar/CalendarSystem.getEra:(Ljava/lang/String;)Lsun/util/calendar/Era;")
        }

        #[java_method(name = "getEras", descriptor = "()[Lsun/util/calendar/Era;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getEras(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/util/calendar/CalendarSystem.getEras:()[Lsun/util/calendar/Era;")
        }

        #[java_method(name = "getNthDayOfWeek", descriptor = "(IILsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getNthDayOfWeek(&self, arg0: i32, arg1: i32, arg2: CalendarDate) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.getNthDayOfWeek:(IILsun/util/calendar/CalendarDate;)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "setTimeOfDay", descriptor = "(Lsun/util/calendar/CalendarDate;I)Lsun/util/calendar/CalendarDate;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn setTimeOfDay(&self, arg0: CalendarDate, arg1: i32) -> Result<CalendarDate> {
            panic!("stub: sun/util/calendar/CalendarSystem.setTimeOfDay:(Lsun/util/calendar/CalendarDate;I)Lsun/util/calendar/CalendarDate;")
        }

        #[java_method(name = "validate", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn validate(&self, arg0: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarSystem.validate:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "normalize", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn normalize(&self, arg0: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/CalendarSystem.normalize:(Lsun/util/calendar/CalendarDate;)Z")
        }
    }
}
