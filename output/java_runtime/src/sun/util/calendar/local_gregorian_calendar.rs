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

impl From<LocalGregorianCalendar> for BaseCalendar {
    fn from(v: LocalGregorianCalendar) -> BaseCalendar { v.__into_super() }
}

impl From<LocalGregorianCalendar> for AbstractCalendar {
    fn from(v: LocalGregorianCalendar) -> AbstractCalendar { v.__into_super().__into_super() }
}

impl From<LocalGregorianCalendar> for CalendarSystem {
    fn from(v: LocalGregorianCalendar) -> CalendarSystem { v.__into_super().__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/calendar/LocalGregorianCalendar"]
    #[super_class       = "sun/util/calendar/BaseCalendar"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocalGregorianCalendar.java"]
    #[inner_classes     = "sun/util/calendar/LocalGregorianCalendar$Date:sun/util/calendar/LocalGregorianCalendar:Date:9"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "BaseCalendar"]
    #[superclass_fields(eras: Rc<RefCell<Vec<Object>>>)]
    #[all_supertypes    = "java/lang/Object;sun/util/calendar/AbstractCalendar;sun/util/calendar/BaseCalendar;sun/util/calendar/CalendarSystem;sun/util/calendar/LocalGregorianCalendar"]

    pub struct LocalGregorianCalendar {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "eras", descriptor = "[Lsun/util/calendar/Era;", access = "private", modifiers = "", is_static = false))]
        pub eras: Rc<RefCell<Vec<Object>>>,
    }

    impl LocalGregorianCalendar {
        #[cfg_attr(any(), java_field(name = "JAPANESE_ERAS", descriptor = "[Lsun/util/calendar/Era;", access = "private", modifiers = "static final", is_static = true))]
        // static field: JAPANESE_ERAS:[Lsun/util/calendar/Era;
        pub fn JAPANESE_ERAS() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.JAPANESE_ERAS:[Lsun/util/calendar/Era;")
        }

        #[java_method(name = "isValidEra", descriptor = "(Lsun/util/calendar/Era;[Lsun/util/calendar/Era;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isValidEra(newEra: Object, eras: Rc<RefCell<Vec<Object>>>) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.isValidEra:(Lsun/util/calendar/Era;[Lsun/util/calendar/Era;)Z")
        }

        #[java_method(name = "getLocalGregorianCalendar", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/LocalGregorianCalendar;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalGregorianCalendar(name: String) -> Result<LocalGregorianCalendar> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getLocalGregorianCalendar:(Ljava/lang/String;)Lsun/util/calendar/LocalGregorianCalendar;")
        }

        #[java_method(name = "parseEraEntry", descriptor = "(Ljava/lang/String;)Lsun/util/calendar/Era;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseEraEntry(entry: String) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.parseEraEntry:(Ljava/lang/String;)Lsun/util/calendar/Era;")
        }

        #[java_method(name = "convertUnicodeEscape", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertUnicodeEscape(src: String) -> Result<String> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.convertUnicodeEscape:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;[Lsun/util/calendar/Era;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(name: String, eras: Rc<RefCell<Vec<Object>>>) -> Result<Self> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.<init>:(Ljava/lang/String;[Lsun/util/calendar/Era;)V")
        }

        #[java_method(name = "getName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getName(&self) -> Result<String> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getName:()Ljava/lang/String;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "()Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getCalendarDate:()Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l(&self, millis: i64) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getCalendarDate:(J)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLjava/util/TimeZone;)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_timezo(&self, millis: i64, arg1: TimeZone) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getCalendarDate:(JLjava/util/TimeZone;)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate_l_calend(&self, millis: i64, arg1: CalendarDate) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getCalendarDate:(JLsun/util/calendar/CalendarDate;)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "adjustYear", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;JI)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustYear(&self, ldate: Object, millis: i64, arg2: i32) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.adjustYear:(Lsun/util/calendar/LocalGregorianCalendar$Date;JI)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "()Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCalendarDate(&self) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.newCalendarDate:()Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "newCalendarDate", descriptor = "(Ljava/util/TimeZone;)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn newCalendarDate_timezo(&self, zone: TimeZone) -> Result<Object> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.newCalendarDate:(Ljava/util/TimeZone;)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "validate", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validate(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.validate:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "validateEra", descriptor = "(Lsun/util/calendar/Era;)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn validateEra(&self, era: Object) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.validateEra:(Lsun/util/calendar/Era;)Z")
        }

        #[java_method(name = "normalize", descriptor = "(Lsun/util/calendar/CalendarDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalize(&self, date: CalendarDate) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.normalize:(Lsun/util/calendar/CalendarDate;)Z")
        }

        #[java_method(name = "normalizeMonth", descriptor = "(Lsun/util/calendar/CalendarDate;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeMonth(&self, date: CalendarDate) -> Result<()> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.normalizeMonth:(Lsun/util/calendar/CalendarDate;)V")
        }

        #[java_method(name = "normalizeYear", descriptor = "(Lsun/util/calendar/CalendarDate;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeYear(&self, date: CalendarDate) -> Result<()> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.normalizeYear:(Lsun/util/calendar/CalendarDate;)V")
        }

        #[java_method(name = "isLeapYear", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear_i(&self, gregorianYear: i32) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.isLeapYear:(I)Z")
        }

        #[java_method(name = "isLeapYear", descriptor = "(Lsun/util/calendar/Era;I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear_era_i(&self, era: Object, year: i32) -> Result<bool> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.isLeapYear:(Lsun/util/calendar/Era;I)Z")
        }

        #[java_method(name = "getCalendarDateFromFixedDate", descriptor = "(Lsun/util/calendar/CalendarDate;J)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDateFromFixedDate(&self, date: CalendarDate, fixedDate: i64) -> Result<()> {
            panic!("stub: sun/util/calendar/LocalGregorianCalendar.getCalendarDateFromFixedDate:(Lsun/util/calendar/CalendarDate;J)V")
        }
    }
}
