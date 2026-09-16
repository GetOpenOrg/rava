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

impl From<Gregorian> for BaseCalendar {
    fn from(v: Gregorian) -> BaseCalendar { v.__into_super() }
}

impl From<Gregorian> for AbstractCalendar {
    fn from(v: Gregorian) -> AbstractCalendar { v.__into_super().__into_super() }
}

impl From<Gregorian> for CalendarSystem {
    fn from(v: Gregorian) -> CalendarSystem { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/Gregorian"]
    #[super_class       = "sun/util/calendar/BaseCalendar"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Gregorian.java"]
    #[inner_classes     = "sun/util/calendar/Gregorian$Date:sun/util/calendar/Gregorian:Date:8"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "BaseCalendar"]
    #[superclass_fields(eras: Rc<RefCell<Vec<Object>>>)]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/AbstractCalendar;sun/util/calendar/BaseCalendar;sun/util/calendar/CalendarSystem;sun/util/calendar/Gregorian"]

    pub struct Gregorian;

    impl Gregorian {
        #[java_method(name = "<init>", descriptor = "()V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/calendar/Gregorian.<init>:()V")
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: sun/util/calendar/Gregorian.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "()Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.getCalendarDate:()Lsun/util/calendar/Gregorian$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l(&self, millis: i64) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.getCalendarDate:(J)Lsun/util/calendar/Gregorian$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_calend(&self, millis: i64, arg1: CalendarDate) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.getCalendarDate:(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/Gregorian$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLjava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_timezo(&self, millis: i64, arg1: TimeZone) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.getCalendarDate:(JLjava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "()Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCalendarDate(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.newCalendarDate:()Lsun/util/calendar/Gregorian$Date;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "(Ljava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCalendarDate_timezo(&self, zone: TimeZone) -> Result<Object> {
            panic!("stub: sun/util/calendar/Gregorian.newCalendarDate:(Ljava/util/TimeZone;)Lsun/util/calendar/Gregorian$Date;")
        }
    }
}
