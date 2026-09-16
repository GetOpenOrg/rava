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
    #[binary_name       = "java/time/LocalTime"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/time/temporal/Temporal,java/time/temporal/TemporalAdjuster,java/lang/Comparable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalAdjuster;Ljava/lang/Comparable<Ljava/time/LocalTime;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocalTime.java"]
    #[inner_classes     = "java/time/LocalTime$1:::4104;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Object;java/time/LocalTime;java/time/temporal/Temporal;java/time/temporal/TemporalAdjuster"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct LocalTime {
        #[cfg_attr(any(), java_field(name = "hour", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub hour: i8,
        #[cfg_attr(any(), java_field(name = "minute", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub minute: i8,
        #[cfg_attr(any(), java_field(name = "second", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub second: i8,
        #[cfg_attr(any(), java_field(name = "nano", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub nano: i32,
    }

    impl LocalTime {
        #[cfg_attr(any(), java_field(name = "MIN", descriptor = "Ljava/time/LocalTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIN:Ljava/time/LocalTime;
        pub fn MIN() -> LocalTime {
            panic!("stub: java/time/LocalTime.MIN:Ljava/time/LocalTime;")
        }

        #[cfg_attr(any(), java_field(name = "MAX", descriptor = "Ljava/time/LocalTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAX:Ljava/time/LocalTime;
        pub fn MAX() -> LocalTime {
            panic!("stub: java/time/LocalTime.MAX:Ljava/time/LocalTime;")
        }

        #[cfg_attr(any(), java_field(name = "MIDNIGHT", descriptor = "Ljava/time/LocalTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIDNIGHT:Ljava/time/LocalTime;
        pub fn MIDNIGHT() -> LocalTime {
            panic!("stub: java/time/LocalTime.MIDNIGHT:Ljava/time/LocalTime;")
        }

        #[cfg_attr(any(), java_field(name = "NOON", descriptor = "Ljava/time/LocalTime;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NOON:Ljava/time/LocalTime;
        pub fn NOON() -> LocalTime {
            panic!("stub: java/time/LocalTime.NOON:Ljava/time/LocalTime;")
        }

        #[cfg_attr(any(), java_field(name = "HOURS", descriptor = "[Ljava/time/LocalTime;", access = "private", modifiers = "static final", is_static = true))]
        // static field: HOURS:[Ljava/time/LocalTime;
        pub fn HOURS() -> Rc<RefCell<Vec<LocalTime>>> {
            panic!("stub: java/time/LocalTime.HOURS:[Ljava/time/LocalTime;")
        }

        #[cfg_attr(any(), java_field(name = "HOURS_PER_DAY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "24"))]
        // static field: HOURS_PER_DAY:I
        pub fn HOURS_PER_DAY() -> i32 {
            24
        }

        #[cfg_attr(any(), java_field(name = "MINUTES_PER_HOUR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "60"))]
        // static field: MINUTES_PER_HOUR:I
        pub fn MINUTES_PER_HOUR() -> i32 {
            60
        }

        #[cfg_attr(any(), java_field(name = "MINUTES_PER_DAY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1440"))]
        // static field: MINUTES_PER_DAY:I
        pub fn MINUTES_PER_DAY() -> i32 {
            1440
        }

        #[cfg_attr(any(), java_field(name = "SECONDS_PER_MINUTE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "60"))]
        // static field: SECONDS_PER_MINUTE:I
        pub fn SECONDS_PER_MINUTE() -> i32 {
            60
        }

        #[cfg_attr(any(), java_field(name = "SECONDS_PER_HOUR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3600"))]
        // static field: SECONDS_PER_HOUR:I
        pub fn SECONDS_PER_HOUR() -> i32 {
            3600
        }

        #[cfg_attr(any(), java_field(name = "SECONDS_PER_DAY", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "86400"))]
        // static field: SECONDS_PER_DAY:I
        pub fn SECONDS_PER_DAY() -> i32 {
            86400
        }

        #[cfg_attr(any(), java_field(name = "MILLIS_PER_SECOND", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "1000"))]
        // static field: MILLIS_PER_SECOND:J
        pub fn MILLIS_PER_SECOND() -> i64 {
            1000i64
        }

        #[cfg_attr(any(), java_field(name = "MILLIS_PER_DAY", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "86400000"))]
        // static field: MILLIS_PER_DAY:J
        pub fn MILLIS_PER_DAY() -> i64 {
            86400000i64
        }

        #[cfg_attr(any(), java_field(name = "MICROS_PER_SECOND", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "1000000"))]
        // static field: MICROS_PER_SECOND:J
        pub fn MICROS_PER_SECOND() -> i64 {
            1000000i64
        }

        #[cfg_attr(any(), java_field(name = "MICROS_PER_DAY", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "86400000000"))]
        // static field: MICROS_PER_DAY:J
        pub fn MICROS_PER_DAY() -> i64 {
            86400000000i64
        }

        #[cfg_attr(any(), java_field(name = "NANOS_PER_MILLI", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "1000000"))]
        // static field: NANOS_PER_MILLI:J
        pub fn NANOS_PER_MILLI() -> i64 {
            1000000i64
        }

        #[cfg_attr(any(), java_field(name = "NANOS_PER_SECOND", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "1000000000"))]
        // static field: NANOS_PER_SECOND:J
        pub fn NANOS_PER_SECOND() -> i64 {
            1000000000i64
        }

        #[cfg_attr(any(), java_field(name = "NANOS_PER_MINUTE", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "60000000000"))]
        // static field: NANOS_PER_MINUTE:J
        pub fn NANOS_PER_MINUTE() -> i64 {
            60000000000i64
        }

        #[cfg_attr(any(), java_field(name = "NANOS_PER_HOUR", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "3600000000000"))]
        // static field: NANOS_PER_HOUR:J
        pub fn NANOS_PER_HOUR() -> i64 {
            3600000000000i64
        }

        #[cfg_attr(any(), java_field(name = "NANOS_PER_DAY", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "86400000000000"))]
        // static field: NANOS_PER_DAY:J
        pub fn NANOS_PER_DAY() -> i64 {
            86400000000000i64
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6414437269572265201"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6414437269572265201i64
        }

        #[java_method(name = "now", descriptor = "()Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now() -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.now:()Ljava/time/LocalTime;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/ZoneId;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_zoneid(zone: ZoneId) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.now:(Ljava/time/ZoneId;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/Clock;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_clock(clock: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.now:(Ljava/time/Clock;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "of", descriptor = "(II)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i(hour: i32, minute: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.of:(II)Ljava/time/LocalTime;")
        }

        #[java_method(name = "of", descriptor = "(III)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i(hour: i32, minute: i32, second: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.of:(III)Ljava/time/LocalTime;")
        }

        #[java_method(name = "of", descriptor = "(IIII)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i_i(hour: i32, minute: i32, second: i32, nanoOfSecond: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.of:(IIII)Ljava/time/LocalTime;")
        }

        #[java_method(name = "ofInstant", descriptor = "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofInstant(instant: Instant, zone: ZoneId) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.ofInstant:(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "ofSecondOfDay", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofSecondOfDay(secondOfDay: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.ofSecondOfDay:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "ofNanoOfDay", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofNanoOfDay(mut nanoOfDay: i64) -> Result<LocalTime> {
            let _t0 = ChronoField::NANO_OF_DAY().checkValidValue(nanoOfDay)?;
            let mut hours: i32 = ((nanoOfDay/3600000000000i64) as i32);
            nanoOfDay = (nanoOfDay).wrapping_sub(((hours as i64)).wrapping_mul(3600000000000i64));
            let mut minutes: i32 = ((nanoOfDay/60000000000i64) as i32);
            nanoOfDay = (nanoOfDay).wrapping_sub(((minutes as i64)).wrapping_mul(60000000000i64));
            let mut seconds: i32 = ((nanoOfDay/1000000000i64) as i32);
            nanoOfDay = (nanoOfDay).wrapping_sub(((seconds as i64)).wrapping_mul(1000000000i64));
            let _t1: LocalTime = LocalTime::create(hours, minutes, seconds, (nanoOfDay as i32))?;
            Ok(_t1)
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq(text: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.parse:(Ljava/lang/CharSequence;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalTime;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq_dateti(text: Object, formatter: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.parse:(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "create", descriptor = "(IIII)Ljava/time/LocalTime;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn create(mut hour: i32, mut minute: i32, mut second: i32, mut nanoOfSecond: i32) -> Result<LocalTime> {
            if (((minute|second)|nanoOfSecond)==0) {
                return Ok(Clone::clone(&LocalTime::HOURS().borrow()[hour as usize]));
            }
            Ok(LocalTime::new(hour, minute, second, nanoOfSecond)?)
        }

        #[java_method(name = "<init>", descriptor = "(IIII)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut hour: i32, mut minute: i32, mut second: i32, mut nanoOfSecond: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_hour(((((hour) as i8 as i32)) as i8));
            this.__set_minute(((((minute) as i8 as i32)) as i8));
            this.__set_second(((((second) as i8 as i32)) as i8));
            this.__set_nano(nanoOfSecond);
            Ok(this)
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/LocalTime.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalUnit;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor_1(&self, unit: Object) -> Result<bool> {
            panic!("stub: java/time/LocalTime.isSupported:(Ljava/time/temporal/TemporalUnit;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/LocalTime.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/LocalTime.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/LocalTime.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "get0", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get0(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/LocalTime.get0:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getHour", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getHour(&self) -> Result<i32> {
            panic!("stub: java/time/LocalTime.getHour:()I")
        }

        #[java_method(name = "getMinute", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinute(&self) -> Result<i32> {
            panic!("stub: java/time/LocalTime.getMinute:()I")
        }

        #[java_method(name = "getSecond", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSecond(&self) -> Result<i32> {
            panic!("stub: java/time/LocalTime.getSecond:()I")
        }

        #[java_method(name = "getNano", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNano(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_nano())
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor(&self, adjuster: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.with:(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor_l(&self, field: Object, newValue: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.with:(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "withHour", descriptor = "(I)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withHour(&self, hour: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.withHour:(I)Ljava/time/LocalTime;")
        }

        #[java_method(name = "withMinute", descriptor = "(I)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withMinute(&self, minute: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.withMinute:(I)Ljava/time/LocalTime;")
        }

        #[java_method(name = "withSecond", descriptor = "(I)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withSecond(&self, second: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.withSecond:(I)Ljava/time/LocalTime;")
        }

        #[java_method(name = "withNano", descriptor = "(I)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withNano(&self, nanoOfSecond: i32) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.withNano:(I)Ljava/time/LocalTime;")
        }

        #[java_method(name = "truncatedTo", descriptor = "(Ljava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn truncatedTo(&self, unit: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.truncatedTo:(Ljava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_tempor(&self, amountToAdd: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_l_tempor(&self, amountToAdd: i64, arg1: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plusHours", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusHours(&self, hoursToAdd: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plusHours:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plusMinutes", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusMinutes(&self, minutesToAdd: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plusMinutes:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plusSeconds", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusSeconds(&self, secondstoAdd: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plusSeconds:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "plusNanos", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusNanos(&self, nanosToAdd: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.plusNanos:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_tempor(&self, amountToSubtract: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_l_tempor(&self, amountToSubtract: i64, arg1: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minusHours", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusHours(&self, hoursToSubtract: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minusHours:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minusMinutes", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusMinutes(&self, minutesToSubtract: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minusMinutes:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minusSeconds", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusSeconds(&self, secondsToSubtract: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minusSeconds:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "minusNanos", descriptor = "(J)Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusNanos(&self, nanosToSubtract: i64) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.minusNanos:(J)Ljava/time/LocalTime;")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/LocalTime.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/LocalTime.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "until", descriptor = "(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn until(&self, endExclusive: Object, unit: Object) -> Result<i64> {
            panic!("stub: java/time/LocalTime.until:(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J")
        }

        #[java_method(name = "format", descriptor = "(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format(&self, formatter: Object) -> Result<String> {
            panic!("stub: java/time/LocalTime.format:(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;")
        }

        #[java_method(name = "atDate", descriptor = "(Ljava/time/LocalDate;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atDate(&self, date: LocalDate) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalTime.atDate:(Ljava/time/LocalDate;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atOffset", descriptor = "(Ljava/time/ZoneOffset;)Ljava/time/OffsetTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atOffset(&self, offset: ZoneOffset) -> Result<Object> {
            panic!("stub: java/time/LocalTime.atOffset:(Ljava/time/ZoneOffset;)Ljava/time/OffsetTime;")
        }

        #[java_method(name = "toSecondOfDay", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toSecondOfDay(&self) -> Result<i32> {
            panic!("stub: java/time/LocalTime.toSecondOfDay:()I")
        }

        #[java_method(name = "toNanoOfDay", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toNanoOfDay(&self) -> Result<i64> {
            let this = self;
            let mut total = ((this.__get_hour() as i64)).wrapping_mul(3600000000000i64);
            total = (total).wrapping_add(((this.__get_minute() as i64)).wrapping_mul(60000000000i64));
            total = (total).wrapping_add(((this.__get_second() as i64)).wrapping_mul(1000000000i64));
            total = (total).wrapping_add((this.__get_nano() as i64));
            Ok(total)
        }

        #[java_method(name = "toEpochSecond", descriptor = "(Ljava/time/LocalDate;Ljava/time/ZoneOffset;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEpochSecond(&self, date: LocalDate, offset: ZoneOffset) -> Result<i64> {
            panic!("stub: java/time/LocalTime.toEpochSecond:(Ljava/time/LocalDate;Ljava/time/ZoneOffset;)J")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/LocalTime;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, other: LocalTime) -> Result<i32> {
            panic!("stub: java/time/LocalTime.compareTo:(Ljava/time/LocalTime;)I")
        }

        #[java_method(name = "isAfter", descriptor = "(Ljava/time/LocalTime;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAfter(&self, other: LocalTime) -> Result<bool> {
            panic!("stub: java/time/LocalTime.isAfter:(Ljava/time/LocalTime;)Z")
        }

        #[java_method(name = "isBefore", descriptor = "(Ljava/time/LocalTime;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBefore(&self, other: LocalTime) -> Result<bool> {
            panic!("stub: java/time/LocalTime.isBefore:(Ljava/time/LocalTime;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/LocalTime.equals:(Ljava/lang/Object;)Z")
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
            panic!("stub: java/time/LocalTime.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/LocalTime.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/LocalTime.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/LocalTime;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<LocalTime> {
            panic!("stub: java/time/LocalTime.readExternal:(Ljava/io/DataInput;)Ljava/time/LocalTime;")
        }
    }
}
