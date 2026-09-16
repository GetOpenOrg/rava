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
    #[binary_name       = "java/time/LocalDateTime"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/time/temporal/Temporal,java/time/temporal/TemporalAdjuster,java/time/chrono/ChronoLocalDateTime,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalAdjuster;Ljava/time/chrono/ChronoLocalDateTime<Ljava/time/LocalDate;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocalDateTime.java"]
    #[inner_classes     = "java/time/LocalDateTime$1:::4104;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/LocalDateTime;java/time/chrono/ChronoLocalDateTime;java/time/temporal/Temporal;java/time/temporal/TemporalAdjuster"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct LocalDateTime {
        #[cfg_attr(any(), java_field(name = "date", descriptor = "Ljava/time/LocalDate;", access = "private", modifiers = "final", is_static = false))]
        pub date: LocalDate,
        #[cfg_attr(any(), java_field(name = "time", descriptor = "Ljava/time/LocalTime;", access = "private", modifiers = "final", is_static = false))]
        pub time: LocalTime,
    }

    impl LocalDateTime {
        #[cfg_attr(any(), java_field(name = "MIN", descriptor = "Ljava/time/LocalDateTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIN:Ljava/time/LocalDateTime;
        pub fn MIN() -> LocalDateTime {
            panic!("stub: java/time/LocalDateTime.MIN:Ljava/time/LocalDateTime;")
        }

        #[cfg_attr(any(), java_field(name = "MAX", descriptor = "Ljava/time/LocalDateTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAX:Ljava/time/LocalDateTime;
        pub fn MAX() -> LocalDateTime {
            panic!("stub: java/time/LocalDateTime.MAX:Ljava/time/LocalDateTime;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6207766400415563566"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6207766400415563566i64
        }

        #[java_method(name = "now", descriptor = "()Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now() -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.now:()Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/ZoneId;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_zoneid(zone: ZoneId) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.now:(Ljava/time/ZoneId;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/Clock;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_clock(clock: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.now:(Ljava/time/Clock;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(ILjava/time/Month;III)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_month_i_i_i(year: i32, month: Month, dayOfMonth: i32, hour: i32, minute: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(ILjava/time/Month;III)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(ILjava/time/Month;IIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_month_i_i_i_i(year: i32, month: Month, dayOfMonth: i32, hour: i32, minute: i32, second: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(ILjava/time/Month;IIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(ILjava/time/Month;IIIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_month_i_i_i_i_i(year: i32, month: Month, dayOfMonth: i32, hour: i32, minute: i32, second: i32, nanoOfSecond: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(ILjava/time/Month;IIIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(IIIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hour: i32, minute: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(IIIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(IIIIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hour: i32, minute: i32, second: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(IIIIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(IIIIIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i_i_i_i_i(year: i32, month: i32, dayOfMonth: i32, hour: i32, minute: i32, second: i32, nanoOfSecond: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.of:(IIIIIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: of(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;
        pub fn of_locald_localt(mut date: LocalDate, mut time: LocalTime) -> Result<LocalDateTime> {
            let _t0: Object = Objects::requireNonNull_obj_str(Object::from_any(date.clone()), Clone::clone(&String::from("date")))?;
            let _t1: Object = Objects::requireNonNull_obj_str(Object::from_any(time.clone()), Clone::clone(&String::from("time")))?;
            Ok(LocalDateTime::new(Clone::clone(&date), Clone::clone(&time))?)
        }

        #[java_method(name = "ofInstant", descriptor = "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofInstant(instant: Instant, zone: ZoneId) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.ofInstant:(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "ofEpochSecond", descriptor = "(JILjava/time/ZoneOffset;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofEpochSecond(epochSecond: i64, arg1: i32, nanoOfSecond: ZoneOffset) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.ofEpochSecond:(JILjava/time/ZoneOffset;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq(text: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.parse:(Ljava/lang/CharSequence;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalDateTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq_dateti(text: Object, formatter: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.parse:(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/time/LocalDate;Ljava/time/LocalTime;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut date: LocalDate, mut time: LocalTime) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_date(Clone::clone(&date));
            this.__set_time(Clone::clone(&time));
            Ok(this)
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: with(Ljava/time/LocalDate;Ljava/time/LocalTime;)Ljava/time/LocalDateTime;
        pub fn with_locald_localt(&self, mut newDate: LocalDate, mut newTime: LocalTime) -> Result<LocalDateTime> {
            let this = self;
            if Object::from_any(this.__get_time().clone()) == Object::from_any(newTime.clone()) {
                return Ok(Clone::clone(this));
            }
            Ok(LocalDateTime::new(Clone::clone(&newDate), Clone::clone(&newTime))?)
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalUnit;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor_1(&self, unit: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.isSupported:(Ljava/time/temporal/TemporalUnit;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/LocalDateTime.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/LocalDateTime.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "toLocalDate", descriptor = "()Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLocalDate(&self) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDateTime.toLocalDate:()Ljava/time/LocalDate;")
        }

        #[java_method(name = "getYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYear(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getYear:()I")
        }

        #[java_method(name = "getMonthValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonthValue(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getMonthValue:()I")
        }

        #[java_method(name = "getMonth", descriptor = "()Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonth(&self) -> Result<Month> {
            panic!("stub: java/time/LocalDateTime.getMonth:()Ljava/time/Month;")
        }

        #[java_method(name = "getDayOfMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfMonth(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getDayOfMonth:()I")
        }

        #[java_method(name = "getDayOfYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfYear(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getDayOfYear:()I")
        }

        #[java_method(name = "getDayOfWeek", descriptor = "()Ljava/time/DayOfWeek;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeek(&self) -> Result<DayOfWeek> {
            panic!("stub: java/time/LocalDateTime.getDayOfWeek:()Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "toLocalTime", descriptor = "()Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLocalTime(&self) -> Result<LocalTime> {
            panic!("stub: java/time/LocalDateTime.toLocalTime:()Ljava/time/LocalTime;")
        }

        #[java_method(name = "getHour", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getHour(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getHour:()I")
        }

        #[java_method(name = "getMinute", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinute(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getMinute:()I")
        }

        #[java_method(name = "getSecond", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSecond(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.getSecond:()I")
        }

        #[java_method(name = "getNano", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNano(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__get_time().getNano()?;
            Ok(_t0)
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor(&self, adjuster: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.with:(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor_l(&self, field: Object, newValue: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.with:(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withYear", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withYear(&self, year: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withYear:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withMonth", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withMonth(&self, month: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withMonth:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withDayOfMonth", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withDayOfMonth(&self, dayOfMonth: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withDayOfMonth:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withDayOfYear", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withDayOfYear(&self, dayOfYear: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withDayOfYear:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withHour", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withHour(&self, hour: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withHour:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withMinute", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withMinute(&self, minute: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withMinute:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withSecond", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withSecond(&self, second: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withSecond:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "withNano", descriptor = "(I)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withNano(&self, nanoOfSecond: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.withNano:(I)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "truncatedTo", descriptor = "(Ljava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn truncatedTo(&self, unit: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.truncatedTo:(Ljava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_tempor(&self, amountToAdd: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_l_tempor(&self, amountToAdd: i64, arg1: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusYears", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusYears(&self, years: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusYears:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusMonths", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusMonths(&self, months: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusMonths:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusWeeks", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusWeeks(&self, weeks: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusWeeks:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusDays", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusDays(&self, days: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusDays:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusHours", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusHours(&self, hours: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusHours:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusMinutes", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusMinutes(&self, minutes: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusMinutes:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusSeconds", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusSeconds(&self, mut seconds: i64) -> Result<LocalDateTime> {
            let this = self;
            let _t0 = this.plusWithOverflow(Clone::clone(&this.__get_date()), 0i64, 0i64, seconds, 0i64, 1i32)?;
            Ok(_t0)
        }

        #[java_method(name = "plusNanos", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusNanos(&self, nanos: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.plusNanos:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_tempor(&self, amountToSubtract: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_l_tempor(&self, amountToSubtract: i64, arg1: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusYears", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusYears(&self, years: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusYears:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusMonths", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusMonths(&self, months: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusMonths:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusWeeks", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusWeeks(&self, weeks: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusWeeks:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusDays", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusDays(&self, days: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusDays:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusHours", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusHours(&self, hours: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusHours:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusMinutes", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusMinutes(&self, minutes: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusMinutes:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusSeconds", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusSeconds(&self, seconds: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusSeconds:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "minusNanos", descriptor = "(J)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusNanos(&self, nanos: i64) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.minusNanos:(J)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "plusWithOverflow", descriptor = "(Ljava/time/LocalDate;JJJJI)Ljava/time/LocalDateTime;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusWithOverflow(&self, mut newDate: LocalDate, mut hours: i64, mut minutes: i64, mut seconds: i64, mut nanos: i64, mut sign: i32) -> Result<LocalDateTime> {
            let this = self;
            if ((((((hours|(minutes))|(seconds))|(nanos))>(0i64)) as i32-(((((hours|(minutes))|(seconds))|(nanos)))<(0i64)) as i32)==0) {
                let _t0 = this.with_locald_localt(Clone::clone(&newDate), Clone::clone(&this.__get_time()))?;
                return Ok(_t0);
            }
            let mut totDays = ((((nanos/86400000000000i64)).wrapping_add((seconds/86400i64))).wrapping_add((minutes/1440i64))).wrapping_add((hours/24i64));
            totDays = (totDays).wrapping_mul((sign as i64));
            let mut totNanos = ((((nanos%(86400000000000i64))).wrapping_add(((seconds%(86400i64))).wrapping_mul(1000000000i64))).wrapping_add(((minutes%(1440i64))).wrapping_mul(60000000000i64))).wrapping_add(((hours%(24i64))).wrapping_mul(3600000000000i64));
            let _t0 = this.__get_time().toNanoOfDay()?;
            let mut curNoD: i64 = _t0;
            totNanos = ((totNanos).wrapping_mul((sign as i64))).wrapping_add(curNoD);
            let _t1: i64 = Math::floorDiv_l_l(totNanos, 86400000000000i64)?;
            totDays = (totDays).wrapping_add(_t1);
            let _t2: i64 = Math::floorMod_l_l(totNanos, 86400000000000i64)?;
            let mut newNoD: i64 = _t2;
            let mut _merged4: LocalTime;
            if (((newNoD>(curNoD)) as i32-((newNoD)<(curNoD)) as i32)==0) {
                _merged4 = this.__get_time();
            } else {
                let _t3: LocalTime = LocalTime::ofNanoOfDay(newNoD)?;
                _merged4 = _t3;
            }
            let mut newTime: LocalTime = _merged4;
            let _t5 = newDate.plusDays(totDays)?;
            let _t6 = this.with_locald_localt(Clone::clone(&_t5), Clone::clone(&newTime))?;
            Ok(_t6)
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDateTime.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDateTime.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "until", descriptor = "(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn until(&self, endExclusive: Object, unit: Object) -> Result<i64> {
            panic!("stub: java/time/LocalDateTime.until:(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J")
        }

        #[java_method(name = "format", descriptor = "(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format(&self, formatter: Object) -> Result<String> {
            panic!("stub: java/time/LocalDateTime.format:(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;")
        }

        #[java_method(name = "atOffset", descriptor = "(Ljava/time/ZoneOffset;)Ljava/time/OffsetDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atOffset(&self, offset: ZoneOffset) -> Result<Object> {
            panic!("stub: java/time/LocalDateTime.atOffset:(Ljava/time/ZoneOffset;)Ljava/time/OffsetDateTime;")
        }

        #[java_method(name = "atZone", descriptor = "(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atZone(&self, zone: ZoneId) -> Result<Object> {
            panic!("stub: java/time/LocalDateTime.atZone:(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/chrono/ChronoLocalDateTime;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/chrono/ChronoLocalDateTime<*>;)I")]
        pub fn compareTo(&self, other: Object) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.compareTo:(Ljava/time/chrono/ChronoLocalDateTime;)I")
        }

        #[java_method(name = "compareTo0", descriptor = "(Ljava/time/LocalDateTime;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo0(&self, other: LocalDateTime) -> Result<i32> {
            panic!("stub: java/time/LocalDateTime.compareTo0:(Ljava/time/LocalDateTime;)I")
        }

        #[java_method(name = "isAfter", descriptor = "(Ljava/time/chrono/ChronoLocalDateTime;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/chrono/ChronoLocalDateTime<*>;)Z")]
        pub fn isAfter(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.isAfter:(Ljava/time/chrono/ChronoLocalDateTime;)Z")
        }

        #[java_method(name = "isBefore", descriptor = "(Ljava/time/chrono/ChronoLocalDateTime;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/chrono/ChronoLocalDateTime<*>;)Z")]
        pub fn isBefore(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.isBefore:(Ljava/time/chrono/ChronoLocalDateTime;)Z")
        }

        #[java_method(name = "isEqual", descriptor = "(Ljava/time/chrono/ChronoLocalDateTime;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/chrono/ChronoLocalDateTime<*>;)Z")]
        pub fn isEqual(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.isEqual:(Ljava/time/chrono/ChronoLocalDateTime;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDateTime.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/LocalDateTime.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/LocalDateTime.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/LocalDateTime.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/LocalDateTime;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDateTime.readExternal:(Ljava/io/DataInput;)Ljava/time/LocalDateTime;")
        }
    }
}
