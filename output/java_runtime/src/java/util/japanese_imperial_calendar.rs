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

impl From<JapaneseImperialCalendar> for Calendar {
    fn from(v: JapaneseImperialCalendar) -> Calendar { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/JapaneseImperialCalendar"]
    #[super_class       = "java/util/Calendar"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "JapaneseImperialCalendar.java"]
    #[inner_classes     = "sun/util/calendar/LocalGregorianCalendar$Date:sun/util/calendar/LocalGregorianCalendar:Date:9;sun/util/calendar/Gregorian$Date:sun/util/calendar/Gregorian:Date:8;sun/util/calendar/BaseCalendar$Date:sun/util/calendar/BaseCalendar:Date:1033"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Calendar"]
    #[superclass_fields(fields: Rc<RefCell<Vec<i32>>>, isSet: Rc<RefCell<Vec<bool>>>, stamp: Rc<RefCell<Vec<i32>>>, time: i64, isTimeSet: bool, areFieldsSet: bool, areAllFieldsSet: bool, lenient: bool, zone: TimeZone, sharedZone: bool, firstDayOfWeek: i32, minimalDaysInFirstWeek: i32, nextStamp: i32, serialVersionOnStream: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Comparable;java/lang/Object;java/util/Calendar;java/util/JapaneseImperialCalendar"]
    #[has_hash_code_method = true]

    pub struct JapaneseImperialCalendar {
        #[cfg_attr(any(), java_field(name = "jdate", descriptor = "Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "private", modifiers = "transient", is_static = false))]
        pub jdate: Object,
        #[cfg_attr(any(), java_field(name = "zoneOffsets", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub zoneOffsets: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "originalFields", descriptor = "[I", access = "private", modifiers = "transient", is_static = false))]
        pub originalFields: Rc<RefCell<Vec<i32>>>,
        #[cfg_attr(any(), java_field(name = "cachedFixedDate", descriptor = "J", access = "private", modifiers = "transient", is_static = false))]
        pub cachedFixedDate: i64,
    }

    impl JapaneseImperialCalendar {
        #[cfg_attr(any(), java_field(name = "BEFORE_MEIJI", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: BEFORE_MEIJI:I
        pub fn BEFORE_MEIJI() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "MEIJI", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: MEIJI:I
        pub fn MEIJI() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "TAISHO", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: TAISHO:I
        pub fn TAISHO() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "SHOWA", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: SHOWA:I
        pub fn SHOWA() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "HEISEI", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: HEISEI:I
        pub fn HEISEI() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "REIWA", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: REIWA:I
        pub fn REIWA() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "EPOCH_OFFSET", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "719163"))]
        // static field: EPOCH_OFFSET:I
        pub fn EPOCH_OFFSET() -> i32 {
            719163
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

        #[cfg_attr(any(), java_field(name = "jcal", descriptor = "Lsun/util/calendar/LocalGregorianCalendar;", access = "private", modifiers = "static final", is_static = true))]
        // static field: jcal:Lsun/util/calendar/LocalGregorianCalendar;
        pub fn jcal() -> LocalGregorianCalendar {
            panic!("stub: java/util/JapaneseImperialCalendar.jcal:Lsun/util/calendar/LocalGregorianCalendar;")
        }

        #[cfg_attr(any(), java_field(name = "gcal", descriptor = "Lsun/util/calendar/Gregorian;", access = "private", modifiers = "static final", is_static = true))]
        // static field: gcal:Lsun/util/calendar/Gregorian;
        pub fn gcal() -> Gregorian {
            panic!("stub: java/util/JapaneseImperialCalendar.gcal:Lsun/util/calendar/Gregorian;")
        }

        #[cfg_attr(any(), java_field(name = "BEFORE_MEIJI_ERA", descriptor = "Lsun/util/calendar/Era;", access = "private", modifiers = "static final", is_static = true))]
        // static field: BEFORE_MEIJI_ERA:Lsun/util/calendar/Era;
        pub fn BEFORE_MEIJI_ERA() -> Object {
            panic!("stub: java/util/JapaneseImperialCalendar.BEFORE_MEIJI_ERA:Lsun/util/calendar/Era;")
        }

        #[cfg_attr(any(), java_field(name = "eras", descriptor = "[Lsun/util/calendar/Era;", access = "private", modifiers = "static final", is_static = true))]
        // static field: eras:[Lsun/util/calendar/Era;
        pub fn eras() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/JapaneseImperialCalendar.eras:[Lsun/util/calendar/Era;")
        }

        #[cfg_attr(any(), java_field(name = "sinceFixedDates", descriptor = "[J", access = "private", modifiers = "static final", is_static = true))]
        // static field: sinceFixedDates:[J
        pub fn sinceFixedDates() -> Rc<RefCell<Vec<i64>>> {
            panic!("stub: java/util/JapaneseImperialCalendar.sinceFixedDates:[J")
        }

        #[cfg_attr(any(), java_field(name = "currentEra", descriptor = "I", access = "private", modifiers = "static final", is_static = true))]
        // static field: currentEra:I
        pub fn currentEra() -> i32 {
            panic!("stub: java/util/JapaneseImperialCalendar.currentEra:I")
        }

        #[cfg_attr(any(), java_field(name = "MIN_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: MIN_VALUES:[I
        pub fn MIN_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/JapaneseImperialCalendar.MIN_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "LEAST_MAX_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: LEAST_MAX_VALUES:[I
        pub fn LEAST_MAX_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/JapaneseImperialCalendar.LEAST_MAX_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "MAX_VALUES", descriptor = "[I", access = "package", modifiers = "static final", is_static = true))]
        // static field: MAX_VALUES:[I
        pub fn MAX_VALUES() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/JapaneseImperialCalendar.MAX_VALUES:[I")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-3364572813905467929"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -3364572813905467929i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/TimeZone;Ljava/util/Locale;)V
        pub fn new_timezo_locale(mut zone: TimeZone, mut aLocale: Locale) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Calendar::new_timezo_locale(Clone::clone(&zone), Clone::clone(&aLocale))?);
            this.__set_cachedFixedDate(-9223372036854775808i64);
            let _t0 = JapaneseImperialCalendar::jcal().newCalendarDate_timezo(Clone::clone(&zone))?;
            this.__set_jdate(Clone::clone(&_t0));
            let _t1: i64 = System::currentTimeMillis()?;
            this.__super().setTimeInMillis(_t1)?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/TimeZone;Ljava/util/Locale;Z)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_timezo_locale_z(zone: TimeZone, aLocale: Locale, flag: bool) -> Result<Self> {
            panic!("stub: java/util/JapaneseImperialCalendar.<init>:(Ljava/util/TimeZone;Ljava/util/Locale;Z)V")
        }

        #[java_method(name = "getCalendarType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarType(&self) -> Result<String> {
            panic!("stub: java/util/JapaneseImperialCalendar.getCalendarType:()Ljava/lang/String;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/util/JapaneseImperialCalendar.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "add", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn add(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.add:(II)V")
        }

        #[java_method(name = "roll", descriptor = "(IZ)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll_i_z(&self, field: i32, up: bool) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.roll:(IZ)V")
        }

        #[java_method(name = "roll", descriptor = "(II)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn roll_i_i(&self, field: i32, amount: i32) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.roll:(II)V")
        }

        #[java_method(name = "getDisplayName", descriptor = "(IILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, field: i32, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: java/util/JapaneseImperialCalendar.getDisplayName:(IILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayNames", descriptor = "(IILjava/util/Locale;)Ljava/util/Map;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn getDisplayNames(&self, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: java/util/JapaneseImperialCalendar.getDisplayNames:(IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "getMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getMinimum:(I)I")
        }

        #[java_method(name = "getMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getMaximum:(I)I")
        }

        #[java_method(name = "getGreatestMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGreatestMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getGreatestMinimum:(I)I")
        }

        #[java_method(name = "getLeastMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLeastMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getLeastMaximum:(I)I")
        }

        #[java_method(name = "getActualMinimum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMinimum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getActualMinimum:(I)I")
        }

        #[java_method(name = "getActualMaximum", descriptor = "(I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getActualMaximum(&self, field: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getActualMaximum:(I)I")
        }

        #[java_method(name = "getYearOffsetInMillis", descriptor = "(Lsun/util/calendar/CalendarDate;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYearOffsetInMillis(&self, date: CalendarDate) -> Result<i64> {
            panic!("stub: java/util/JapaneseImperialCalendar.getYearOffsetInMillis:(Lsun/util/calendar/CalendarDate;)J")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/JapaneseImperialCalendar.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "getTimeZone", descriptor = "()Ljava/util/TimeZone;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZone(&self) -> Result<TimeZone> {
            panic!("stub: java/util/JapaneseImperialCalendar.getTimeZone:()Ljava/util/TimeZone;")
        }

        #[java_method(name = "setTimeZone", descriptor = "(Ljava/util/TimeZone;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setTimeZone(&self, zone: TimeZone) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.setTimeZone:(Ljava/util/TimeZone;)V")
        }

        #[java_method(name = "computeFields", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeFields(&self) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.computeFields:()V")
        }

        #[java_method(name = "computeFields", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeFields_i_i(&self, fieldMask: i32, tzMask: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.computeFields:(II)I")
        }

        #[java_method(name = "getWeekNumber", descriptor = "(JJ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeekNumber(&self, fixedDay1: i64, arg1: i64) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getWeekNumber:(JJ)I")
        }

        #[java_method(name = "computeTime", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn computeTime(&self) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.computeTime:()V")
        }

        #[java_method(name = "getFixedDate", descriptor = "(III)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDate(&self, era: i32, year: i32, fieldMask: i32) -> Result<i64> {
            panic!("stub: java/util/JapaneseImperialCalendar.getFixedDate:(III)J")
        }

        #[java_method(name = "getFixedDateJan1", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDateJan1(&self, date: Object, fixedDate: i64) -> Result<i64> {
            panic!("stub: java/util/JapaneseImperialCalendar.getFixedDateJan1:(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J")
        }

        #[java_method(name = "getFixedDateMonth1", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFixedDateMonth1(&self, date: Object, fixedDate: i64) -> Result<i64> {
            panic!("stub: java/util/JapaneseImperialCalendar.getFixedDateMonth1:(Lsun/util/calendar/LocalGregorianCalendar$Date;J)J")
        }

        #[java_method(name = "getCalendarDate", descriptor = "(J)Lsun/util/calendar/LocalGregorianCalendar$Date;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarDate(fd: i64) -> Result<Object> {
            panic!("stub: java/util/JapaneseImperialCalendar.getCalendarDate:(J)Lsun/util/calendar/LocalGregorianCalendar$Date;")
        }

        #[java_method(name = "monthLength", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn monthLength_i_i(&self, month: i32, gregorianYear: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.monthLength:(II)I")
        }

        #[java_method(name = "monthLength", descriptor = "(I)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn monthLength_i(&self, month: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.monthLength:(I)I")
        }

        #[java_method(name = "actualMonthLength", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn actualMonthLength(&self) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.actualMonthLength:()I")
        }

        #[java_method(name = "getTransitionEraIndex", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTransitionEraIndex(date: Object) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getTransitionEraIndex:(Lsun/util/calendar/LocalGregorianCalendar$Date;)I")
        }

        #[java_method(name = "isTransitionYear", descriptor = "(I)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTransitionYear(&self, normalizedYear: i32) -> Result<bool> {
            panic!("stub: java/util/JapaneseImperialCalendar.isTransitionYear:(I)Z")
        }

        #[java_method(name = "getEraIndex", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEraIndex(date: Object) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getEraIndex:(Lsun/util/calendar/LocalGregorianCalendar$Date;)I")
        }

        #[java_method(name = "getNormalizedCalendar", descriptor = "()Ljava/util/JapaneseImperialCalendar;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNormalizedCalendar(&self) -> Result<JapaneseImperialCalendar> {
            panic!("stub: java/util/JapaneseImperialCalendar.getNormalizedCalendar:()Ljava/util/JapaneseImperialCalendar;")
        }

        #[java_method(name = "pinDayOfMonth", descriptor = "(Lsun/util/calendar/LocalGregorianCalendar$Date;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn pinDayOfMonth(&self, date: Object) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.pinDayOfMonth:(Lsun/util/calendar/LocalGregorianCalendar$Date;)V")
        }

        #[java_method(name = "getRolledValue", descriptor = "(IIII)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRolledValue(value: i32, amount: i32, min: i32, max: i32) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.getRolledValue:(IIII)I")
        }

        #[java_method(name = "internalGetEra", descriptor = "()I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn internalGetEra(&self) -> Result<i32> {
            panic!("stub: java/util/JapaneseImperialCalendar.internalGetEra:()I")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/util/JapaneseImperialCalendar.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
