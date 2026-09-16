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
    #[binary_name       = "java/time/Instant"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/time/temporal/Temporal,java/time/temporal/TemporalAdjuster,java/lang/Comparable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = "Ljava/lang/Object;Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalAdjuster;Ljava/lang/Comparable<Ljava/time/Instant;>;Ljava/io/Serializable;"]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Instant.java"]
    #[inner_classes     = "java/time/Instant$1:::4104;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Object;java/time/Instant;java/time/temporal/Temporal;java/time/temporal/TemporalAdjuster"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Instant {
        #[cfg_attr(any(), java_field(name = "seconds", descriptor = "J", access = "private", modifiers = "final", is_static = false))]
        pub seconds: i64,
        #[cfg_attr(any(), java_field(name = "nanos", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub nanos: i32,
    }

    impl Instant {
        #[cfg_attr(any(), java_field(name = "EPOCH", descriptor = "Ljava/time/Instant;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EPOCH:Ljava/time/Instant;
        pub fn EPOCH() -> Instant {
            panic!("stub: java/time/Instant.EPOCH:Ljava/time/Instant;")
        }

        #[cfg_attr(any(), java_field(name = "MIN_SECOND", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-31557014167219200"))]
        // static field: MIN_SECOND:J
        pub fn MIN_SECOND() -> i64 {
            -31557014167219200i64
        }

        #[cfg_attr(any(), java_field(name = "MAX_SECOND", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "31556889864403199"))]
        // static field: MAX_SECOND:J
        pub fn MAX_SECOND() -> i64 {
            31556889864403199i64
        }

        #[cfg_attr(any(), java_field(name = "MIN", descriptor = "Ljava/time/Instant;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIN:Ljava/time/Instant;
        pub fn MIN() -> Instant {
            panic!("stub: java/time/Instant.MIN:Ljava/time/Instant;")
        }

        #[cfg_attr(any(), java_field(name = "MAX", descriptor = "Ljava/time/Instant;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAX:Ljava/time/Instant;
        pub fn MAX() -> Instant {
            panic!("stub: java/time/Instant.MAX:Ljava/time/Instant;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-665713676816604388"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -665713676816604388i64
        }

        #[java_method(name = "now", descriptor = "()Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now() -> Result<Instant> {
            panic!("stub: java/time/Instant.now:()Ljava/time/Instant;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/Clock;)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_clock(clock: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.now:(Ljava/time/Clock;)Ljava/time/Instant;")
        }

        #[java_method(name = "ofEpochSecond", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofEpochSecond_l(epochSecond: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.ofEpochSecond:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "ofEpochSecond", descriptor = "(JJ)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: ofEpochSecond(JJ)Ljava/time/Instant;
        pub fn ofEpochSecond_l_l(mut epochSecond: i64, mut nanoAdjustment: i64) -> Result<Instant> {
            let _t0: i64 = Math::floorDiv_l_l(nanoAdjustment, 1000000000i64)?;
            let _t1: i64 = Math::addExact_l_l(epochSecond, _t0)?;
            let mut secs: i64 = _t1;
            let _t2: i64 = Math::floorMod_l_l(nanoAdjustment, 1000000000i64)?;
            let mut nos: i32 = (_t2 as i32);
            let _t3: Instant = Instant::create(secs, nos)?;
            Ok(_t3)
        }

        #[java_method(name = "ofEpochMilli", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofEpochMilli(epochMilli: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.ofEpochMilli:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Instant;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;)Ljava/time/Instant;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse(text: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.parse:(Ljava/lang/CharSequence;)Ljava/time/Instant;")
        }

        #[java_method(name = "create", descriptor = "(JI)Ljava/time/Instant;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn create(mut seconds: i64, mut nanoOfSecond: i32) -> Result<Instant> {
            if ((((seconds|((nanoOfSecond as i64)))>(0i64)) as i32-(((seconds|((nanoOfSecond as i64))))<(0i64)) as i32)==0) {
                return Ok(Instant::EPOCH());
            }
            if (((seconds>(31556889864403199i64)) as i32-((seconds)<(31556889864403199i64)) as i32)>0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(Instant::new(seconds, nanoOfSecond)?)
        }

        #[java_method(name = "<init>", descriptor = "(JI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut epochSecond: i64, mut nanos: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_seconds(epochSecond);
            this.__set_nanos(nanos);
            Ok(this)
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/Instant.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalUnit;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor_1(&self, unit: Object) -> Result<bool> {
            panic!("stub: java/time/Instant.isSupported:(Ljava/time/temporal/TemporalUnit;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/Instant.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/Instant.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/Instant.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "getEpochSecond", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEpochSecond(&self) -> Result<i64> {
            let this = self;
            Ok(this.__get_seconds())
        }

        #[java_method(name = "getNano", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNano(&self) -> Result<i32> {
            panic!("stub: java/time/Instant.getNano:()I")
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor(&self, adjuster: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.with:(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/Instant;")
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalField;J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor_l(&self, field: Object, newValue: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.with:(Ljava/time/temporal/TemporalField;J)Ljava/time/Instant;")
        }

        #[java_method(name = "truncatedTo", descriptor = "(Ljava/time/temporal/TemporalUnit;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn truncatedTo(&self, unit: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.truncatedTo:(Ljava/time/temporal/TemporalUnit;)Ljava/time/Instant;")
        }

        #[java_method(name = "plus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_tempor(&self, amountToAdd: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.plus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/Instant;")
        }

        #[java_method(name = "plus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_l_tempor(&self, amountToAdd: i64, arg1: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.plus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/Instant;")
        }

        #[java_method(name = "plusSeconds", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusSeconds(&self, secondsToAdd: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.plusSeconds:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "plusMillis", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusMillis(&self, millisToAdd: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.plusMillis:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "plusNanos", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusNanos(&self, nanosToAdd: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.plusNanos:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "plus", descriptor = "(JJ)Ljava/time/Instant;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_l_l(&self, secondsToAdd: i64, arg1: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.plus:(JJ)Ljava/time/Instant;")
        }

        #[java_method(name = "minus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_tempor(&self, amountToSubtract: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.minus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/Instant;")
        }

        #[java_method(name = "minus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_l_tempor(&self, amountToSubtract: i64, arg1: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.minus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/Instant;")
        }

        #[java_method(name = "minusSeconds", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusSeconds(&self, secondsToSubtract: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.minusSeconds:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "minusMillis", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusMillis(&self, millisToSubtract: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.minusMillis:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "minusNanos", descriptor = "(J)Ljava/time/Instant;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusNanos(&self, nanosToSubtract: i64) -> Result<Instant> {
            panic!("stub: java/time/Instant.minusNanos:(J)Ljava/time/Instant;")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/Instant.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/Instant.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "until", descriptor = "(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn until(&self, endExclusive: Object, unit: Object) -> Result<i64> {
            panic!("stub: java/time/Instant.until:(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J")
        }

        #[java_method(name = "nanosUntil", descriptor = "(Ljava/time/Instant;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nanosUntil(&self, end: Instant) -> Result<i64> {
            panic!("stub: java/time/Instant.nanosUntil:(Ljava/time/Instant;)J")
        }

        #[java_method(name = "microsUntil", descriptor = "(Ljava/time/Instant;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn microsUntil(&self, end: Instant) -> Result<i64> {
            panic!("stub: java/time/Instant.microsUntil:(Ljava/time/Instant;)J")
        }

        #[java_method(name = "millisUntil", descriptor = "(Ljava/time/Instant;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn millisUntil(&self, end: Instant) -> Result<i64> {
            panic!("stub: java/time/Instant.millisUntil:(Ljava/time/Instant;)J")
        }

        #[java_method(name = "secondsUntil", descriptor = "(Ljava/time/Instant;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn secondsUntil(&self, end: Instant) -> Result<i64> {
            panic!("stub: java/time/Instant.secondsUntil:(Ljava/time/Instant;)J")
        }

        #[java_method(name = "atOffset", descriptor = "(Ljava/time/ZoneOffset;)Ljava/time/OffsetDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atOffset(&self, offset: ZoneOffset) -> Result<Object> {
            panic!("stub: java/time/Instant.atOffset:(Ljava/time/ZoneOffset;)Ljava/time/OffsetDateTime;")
        }

        #[java_method(name = "atZone", descriptor = "(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atZone(&self, zone: ZoneId) -> Result<Object> {
            panic!("stub: java/time/Instant.atZone:(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "toEpochMilli", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEpochMilli(&self) -> Result<i64> {
            panic!("stub: java/time/Instant.toEpochMilli:()J")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/Instant;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, otherInstant: Instant) -> Result<i32> {
            panic!("stub: java/time/Instant.compareTo:(Ljava/time/Instant;)I")
        }

        #[java_method(name = "isAfter", descriptor = "(Ljava/time/Instant;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAfter(&self, otherInstant: Instant) -> Result<bool> {
            panic!("stub: java/time/Instant.isAfter:(Ljava/time/Instant;)Z")
        }

        #[java_method(name = "isBefore", descriptor = "(Ljava/time/Instant;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBefore(&self, otherInstant: Instant) -> Result<bool> {
            panic!("stub: java/time/Instant.isBefore:(Ljava/time/Instant;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/Instant.equals:(Ljava/lang/Object;)Z")
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
            panic!("stub: java/time/Instant.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/Instant.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/Instant.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/Instant;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<Instant> {
            panic!("stub: java/time/Instant.readExternal:(Ljava/io/DataInput;)Ljava/time/Instant;")
        }
    }
}
