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

impl From<GregorianCalendar> for Calendar {
    fn from(v: GregorianCalendar) -> Calendar { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/GregorianCalendar"]
    #[super_class       = "java/util/Calendar"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "GregorianCalendar.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;sun/util/calendar/Gregorian$Date:sun/util/calendar/Gregorian:Date:8;sun/util/calendar/BaseCalendar$Date:sun/util/calendar/BaseCalendar:Date:1033;sun/util/calendar/JulianCalendar$Date:sun/util/calendar/JulianCalendar:Date:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Calendar"]
    #[superclass_fields(fields: Rc<RefCell<Vec<i32>>>, isSet: Rc<RefCell<Vec<bool>>>, stamp: Rc<RefCell<Vec<i32>>>, time: i64, isTimeSet: bool, areFieldsSet: bool, areAllFieldsSet: bool, lenient: bool, zone: TimeZone, sharedZone: bool, firstDayOfWeek: i32, minimalDaysInFirstWeek: i32, nextStamp: i32, serialVersionOnStream: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Comparable;java/lang/Object;java/util/Calendar;java/util/GregorianCalendar"]
    #[has_hash_code_method = true]

    pub struct GregorianCalendar {
        #[cfg_attr(any(), java_field(name = "gregorianCutover", descriptor = "J", access = "private", modifiers = "", is_static = false))]
        pub gregorianCutover: i64,
        #[cfg_attr(any(), java_field(name = "gregorianCutoverDate", descriptor = "J", access = "private", modifiers = "transient", is_static = false))]
        pub gregorianCutoverDate: i64,
        #[cfg_attr(any(), java_field(name = "gregorianCutoverYear", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub gregorianCutoverYear: i32,
        #[cfg_attr(any(), java_field(name = "gregorianCutoverYearJulian", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub gregorianCutoverYearJulian: i32,
        #[cfg_attr(any(), java_field(name = "gdate", descriptor = "Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "transient", is_static = false))]
        pub gdate: BaseCalendar_Date,
        #[cfg_attr(any(), java_field(name = "cdate", descriptor = "Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "transient", is_static = false))]
        pub cdate: BaseCalendar_Date,
        #[cfg_attr(any(), java_field(name = "calsys", descriptor = "Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "transient", is_static = false))]
        pub calsys: BaseCalendar,
        #[cfg_attr(any(), java_field(name = "zoneOffsets", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub zoneOffsets: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "originalFields", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub originalFields: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "cachedFixedDate", descriptor = "J", access = "private", modifiers = "transient", is_static = false))]
        pub cachedFixedDate: i64,
    }

    impl GregorianCalendar {
        #[cfg_attr(any(), java_field(name = "BC", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: BC:I
        pub fn BC() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "BCE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: BCE:I
        pub fn BCE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "AD", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: AD:I
        pub fn AD() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "CE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: CE:I
        pub fn CE() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "EPOCH_OFFSET", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "719163"))]
        // static field: EPOCH_OFFSET:I
        pub fn EPOCH_OFFSET() -> i32 {
            719163
        }

        #[cfg_attr(any(), java_field(name = "EPOCH_YEAR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1970"))]
        // static field: EPOCH_YEAR:I
        pub fn EPOCH_YEAR() -> i32 {
            1970
        }

        #[cfg_attr(any(), java_field(name = "MONTH_LENGTH", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: MONTH_LENGTH:[I
        pub fn MONTH_LENGTH() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/GregorianCalendar.MONTH_LENGTH:[I")
        }

        #[cfg_attr(any(), java_field(name = "LEAP_MONTH_LENGTH", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: LEAP_MONTH_LENGTH:[I
        pub fn LEAP_MONTH_LENGTH() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/GregorianCalendar.LEAP_MONTH_LENGTH:[I")
        }

        #[cfg_attr(any(), java_field(name = "ONE_SECOND", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1000"))]
        // static field: ONE_SECOND:I
        pub fn ONE_SECOND() -> i32 {
            1000
        }

        #[cfg_attr(any(), java_field(name = "ONE_MINUTE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "60000"))]
        // static field: ONE_MINUTE:I
        pub fn ONE_MINUTE() -> i32 {
            60000
        }

        #[cfg_attr(any(), java_field(name = "ONE_HOUR", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3600000"))]
        // static field: ONE_HOUR:I
        pub fn ONE_HOUR() -> i32 {
            3600000
        }

        #[cfg_attr(any(), java_field(name = "ONE_DAY", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "86400000"))]
        // static field: ONE_DAY:J
        pub fn ONE_DAY() -> i64 {
            86400000i64
        }

        #[cfg_attr(any(), java_field(name = "ONE_WEEK", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "604800000"))]
        // static field: ONE_WEEK:J
        pub fn ONE_WEEK() -> i64 {
            604800000i64
        }

        #[cfg_attr(any(), java_field(name = "MIN_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: MIN_VALUES:[I
        pub fn MIN_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/GregorianCalendar.MIN_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "LEAST_MAX_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: LEAST_MAX_VALUES:[I
        pub fn LEAST_MAX_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/GregorianCalendar.LEAST_MAX_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: MAX_VALUES:[I
        pub fn MAX_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/GregorianCalendar.MAX_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-8125100834729963327"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -8125100834729963327i64
        }

        #[cfg_attr(any(), java_field(name = "gcal", descriptor = "Lsun/util/calendar/Gregorian;", access = "private", modifiers = "static final", is_static = true))]
        // static field: gcal:Lsun/util/calendar/Gregorian;
        pub fn gcal() -> Gregorian {
            panic!("stub: java/util/GregorianCalendar.gcal:Lsun/util/calendar/Gregorian;")
        }

        #[cfg_attr(any(), java_field(name = "jcal", descriptor = "Lsun/util/calendar/JulianCalendar;", access = "private", modifiers = "static", is_static = true))]
        // static field: jcal:Lsun/util/calendar/JulianCalendar;
        pub fn jcal() -> Object {
            panic!("stub: java/util/GregorianCalendar.jcal:Lsun/util/calendar/JulianCalendar;")
        }

        #[cfg_attr(any(), java_field(name = "jeras", descriptor = "[Lsun/util/calendar/Era;", access = "private", modifiers = "static", is_static = true))]
        // static field: jeras:[Lsun/util/calendar/Era;
        pub fn jeras() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/GregorianCalendar.jeras:[Lsun/util/calendar/Era;")
        }

        #[cfg_attr(any(), java_field(name = "DEFAULT_GREGORIAN_CUTOVER", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-12219292800000"))]
        // static field: DEFAULT_GREGORIAN_CUTOVER:J
        pub fn DEFAULT_GREGORIAN_CUTOVER() -> i64 {
            -12219292800000i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            let _t0: TimeZone = TimeZone::getDefaultRef()?;
            let _t1: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            this = GregorianCalendar::new_timezo_locale(Clone::clone(&_t0), Clone::clone(&_t1))?;
            this.__super().setZoneShared((1i32 != 0i32))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/TimeZone;)V
        pub fn new_timezo(mut zone: TimeZone) -> Result<Self> {
            let mut this = Self::default();
            let _t0: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            this = GregorianCalendar::new_timezo_locale(Clone::clone(&zone), Clone::clone(&_t0))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_locale(aLocale: Locale) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/TimeZone;Ljava/util/Locale;)V
        pub fn new_timezo_locale(mut zone: TimeZone, mut aLocale: Locale) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Calendar::new_timezo_locale(Clone::clone(&zone), Clone::clone(&aLocale))?);
            this.__set_gregorianCutover(-12219292800000i64);
            this.__set_gregorianCutoverDate(577736i64);
            this.__set_gregorianCutoverYear(1582i32);
            this.__set_gregorianCutoverYearJulian(1582i32);
            this.__set_cachedFixedDate(-9223372036854775808i64);
            let _t0 = GregorianCalendar::gcal().newCalendarDate_timezo(Clone::clone(&zone))?;
            this.__set_gdate(Clone::clone(&_t0));
            let _t1: i64 = System::currentTimeMillis()?;
            this.__super().setTimeInMillis(_t1)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i(year: i32, month: i32, dayOfMonth: i32) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(III)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hourOfDay: i32, minute: i32) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(IIIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIIII)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hourOfDay: i32, minute: i32, second: i32) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(IIIIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(IIIIIII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_i_i_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hourOfDay: i32, minute: i32, second: i32, millis: i32) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(IIIIIII)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo_locale_z(zone: TimeZone, locale: Locale, flag: bool) -> Result<Self> {
            panic!("stub: java/util/GregorianCalendar.<init>:(Ljava/util/TimeZone;Ljava/util/Locale;Z)V")
        }

        #[java_method(name = "setGregorianChange", descriptor = "(Ljava/util/Date;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGregorianChange_date(&self, date: Date) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.setGregorianChange:(Ljava/util/Date;)V")
        }

        #[java_method(name = "setGregorianChange", descriptor = "(J)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGregorianChange_l(&self, cutoverTime: i64) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.setGregorianChange:(J)V")
        }

        #[java_method(name = "getGregorianChange", descriptor = "()Ljava/util/Date;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGregorianChange(&self) -> Result<Date> {
            panic!("stub: java/util/GregorianCalendar.getGregorianChange:()Ljava/util/Date;")
        }

        #[java_method(name = "isLeapYear", descriptor = "(I)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear(&self, year: i32) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.isLeapYear:(I)Z")
        }

        #[java_method(name = "getCalendarType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarType(&self) -> Result<String> {
            panic!("stub: java/util/GregorianCalendar.getCalendarType:()Ljava/lang/String;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "add", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.add:(II)V")
        }

        #[java_method(name = "roll", descriptor = "(IZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll_i_z(&self, field: i32, up: bool) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.roll:(IZ)V")
        }

        #[java_method(name = "roll", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll_i_i(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.roll:(II)V")
        }

        #[java_method(name = "getMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getMinimum:(I)I")
        }

        #[java_method(name = "getMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getMaximum:(I)I")
        }

        #[java_method(name = "getGreatestMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGreatestMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getGreatestMinimum:(I)I")
        }

        #[java_method(name = "getLeastMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLeastMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getLeastMaximum:(I)I")
        }

        #[java_method(name = "getActualMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getActualMinimum:(I)I")
        }

        #[java_method(name = "getActualMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getActualMaximum:(I)I")
        }

        #[java_method(name = "getYearOffsetInMillis", descriptor = "()J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYearOffsetInMillis(&self) -> Result<i64> {
            panic!("stub: java/util/GregorianCalendar.getYearOffsetInMillis:()J")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/GregorianCalendar.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "getTimeZone", descriptor = "()Ljava/util/TimeZone;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZone(&self) -> Result<TimeZone> {
            panic!("stub: java/util/GregorianCalendar.getTimeZone:()Ljava/util/TimeZone;")
        }

        #[java_method(name = "setTimeZone", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeZone(&self, zone: TimeZone) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.setTimeZone:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "isWeekDateSupported", descriptor = "()Z", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isWeekDateSupported(&self) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.isWeekDateSupported:()Z")
        }

        #[java_method(name = "getWeekYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeekYear(&self) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getWeekYear:()I")
        }

        #[java_method(name = "setWeekDate", descriptor = "(III)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setWeekDate(&self, weekYear: i32, weekOfYear: i32, dayOfWeek: i32) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.setWeekDate:(III)V")
        }

        #[java_method(name = "getWeeksInWeekYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeeksInWeekYear(&self) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getWeeksInWeekYear:()I")
        }

        #[java_method(name = "computeFields", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeFields(&self) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.computeFields:()V")
        }

        #[java_method(name = "computeFields", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeFields_i_i(&self, fieldMask: i32, tzMask: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.computeFields:(II)I")
        }

        #[java_method(name = "getWeekNumber", descriptor = "(JJ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeekNumber(&self, fixedDay1: i64, arg1: i64) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getWeekNumber:(JJ)I")
        }

        #[java_method(name = "computeTime", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeTime(&self) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.computeTime:()V")
        }

        #[java_method(name = "getFixedDate", descriptor = "(Lsun/util/calendar/BaseCalendar;II)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDate(&self, cal: BaseCalendar, year: i32, fieldMask: i32) -> Result<i64> {
            panic!("stub: java/util/GregorianCalendar.getFixedDate:(Lsun/util/calendar/BaseCalendar;II)J")
        }

        #[java_method(name = "getNormalizedCalendar", descriptor = "()Ljava/util/GregorianCalendar;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNormalizedCalendar(&self) -> Result<GregorianCalendar> {
            panic!("stub: java/util/GregorianCalendar.getNormalizedCalendar:()Ljava/util/GregorianCalendar;")
        }

        #[java_method(name = "getJulianCalendarSystem", descriptor = "()Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getJulianCalendarSystem() -> Result<BaseCalendar> {
            panic!("stub: java/util/GregorianCalendar.getJulianCalendarSystem:()Lsun/util/calendar/BaseCalendar;")
        }

        #[java_method(name = "getCutoverCalendarSystem", descriptor = "()Lsun/util/calendar/BaseCalendar;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCutoverCalendarSystem(&self) -> Result<BaseCalendar> {
            panic!("stub: java/util/GregorianCalendar.getCutoverCalendarSystem:()Lsun/util/calendar/BaseCalendar;")
        }

        #[java_method(name = "isCutoverYear", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isCutoverYear(&self, normalizedYear: i32) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.isCutoverYear:(I)Z")
        }

        #[java_method(name = "isInvalidWeek1", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isInvalidWeek1(&self) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.isInvalidWeek1:()Z")
        }

        #[java_method(name = "dayInMinWeek", descriptor = "(III)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dayInMinWeek(&self, day: i32, startDay: i32, endDay: i32) -> Result<bool> {
            panic!("stub: java/util/GregorianCalendar.dayInMinWeek:(III)Z")
        }

        #[java_method(name = "getFixedDateJan1", descriptor = "(Lsun/util/calendar/BaseCalendar$Date;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDateJan1(&self, date: BaseCalendar_Date, fixedDate: i64) -> Result<i64> {
            panic!("stub: java/util/GregorianCalendar.getFixedDateJan1:(Lsun/util/calendar/BaseCalendar$Date;J)J")
        }

        #[java_method(name = "getFixedDateMonth1", descriptor = "(Lsun/util/calendar/BaseCalendar$Date;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDateMonth1(&self, date: BaseCalendar_Date, fixedDate: i64) -> Result<i64> {
            panic!("stub: java/util/GregorianCalendar.getFixedDateMonth1:(Lsun/util/calendar/BaseCalendar$Date;J)J")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(&self, fd: i64) -> Result<BaseCalendar_Date> {
            panic!("stub: java/util/GregorianCalendar.getCalendarDate:(J)Lsun/util/calendar/BaseCalendar$Date;")
        }

        #[java_method(name = "getGregorianCutoverDate", descriptor = "()Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGregorianCutoverDate(&self) -> Result<BaseCalendar_Date> {
            panic!("stub: java/util/GregorianCalendar.getGregorianCutoverDate:()Lsun/util/calendar/BaseCalendar$Date;")
        }

        #[java_method(name = "getLastJulianDate", descriptor = "()Lsun/util/calendar/BaseCalendar$Date;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLastJulianDate(&self) -> Result<BaseCalendar_Date> {
            panic!("stub: java/util/GregorianCalendar.getLastJulianDate:()Lsun/util/calendar/BaseCalendar$Date;")
        }

        #[java_method(name = "monthLength", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn monthLength_i_i(&self, month: i32, year: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.monthLength:(II)I")
        }

        #[java_method(name = "monthLength", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn monthLength_i(&self, month: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.monthLength:(I)I")
        }

        #[java_method(name = "actualMonthLength", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn actualMonthLength(&self) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.actualMonthLength:()I")
        }

        #[java_method(name = "yearLength", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn yearLength_i(&self, year: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.yearLength:(I)I")
        }

        #[java_method(name = "yearLength", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn yearLength(&self) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.yearLength:()I")
        }

        #[java_method(name = "pinDayOfMonth", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pinDayOfMonth(&self) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.pinDayOfMonth:()V")
        }

        #[java_method(name = "getCurrentFixedDate", descriptor = "()J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrentFixedDate(&self) -> Result<i64> {
            panic!("stub: java/util/GregorianCalendar.getCurrentFixedDate:()J")
        }

        #[java_method(name = "getRolledValue", descriptor = "(IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRolledValue(value: i32, amount: i32, min: i32, max: i32) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.getRolledValue:(IIII)I")
        }

        #[java_method(name = "internalGetEra", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn internalGetEra(&self) -> Result<i32> {
            panic!("stub: java/util/GregorianCalendar.internalGetEra:()I")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/util/GregorianCalendar.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "toZonedDateTime", descriptor = "()Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toZonedDateTime(&self) -> Result<Object> {
            panic!("stub: java/util/GregorianCalendar.toZonedDateTime:()Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/ZonedDateTime;)Ljava/util/GregorianCalendar;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(zdt: Object) -> Result<GregorianCalendar> {
            panic!("stub: java/util/GregorianCalendar.from:(Ljava/time/ZonedDateTime;)Ljava/util/GregorianCalendar;")
        }
    }
}
