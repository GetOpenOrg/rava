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

impl From<IsoChronology> for AbstractChronology {
    fn from(v: IsoChronology) -> AbstractChronology { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/chrono/IsoChronology"]
    #[super_class       = "java/time/chrono/AbstractChronology"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "IsoChronology.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "AbstractChronology"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/chrono/AbstractChronology;java/time/chrono/Chronology;java/time/chrono/IsoChronology"]

    pub struct IsoChronology;

    impl IsoChronology {
        #[cfg_attr(any(), java_field(name = "INSTANCE", descriptor = "Ljava/time/chrono/IsoChronology;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSTANCE:Ljava/time/chrono/IsoChronology;
        pub fn INSTANCE() -> IsoChronology {
            panic!("stub: java/time/chrono/IsoChronology.INSTANCE:Ljava/time/chrono/IsoChronology;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-1440403870442975015"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -1440403870442975015i64
        }

        #[cfg_attr(any(), java_field(name = "DAYS_0000_TO_1970", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "719528"))]
        // static field: DAYS_0000_TO_1970:J
        pub fn DAYS_0000_TO_1970() -> i64 {
            719528i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/time/chrono/IsoChronology.<init>:()V")
        }

        #[java_method(name = "getId", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getId(&self) -> Result<String> {
            panic!("stub: java/time/chrono/IsoChronology.getId:()Ljava/lang/String;")
        }

        #[java_method(name = "getCalendarType", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarType(&self) -> Result<String> {
            panic!("stub: java/time/chrono/IsoChronology.getCalendarType:()Ljava/lang/String;")
        }

        #[java_method(name = "date", descriptor = "(Ljava/time/chrono/Era;III)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn date_era_i_i_i(&self, era: Object, yearOfEra: i32, month: i32, dayOfMonth: i32) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.date:(Ljava/time/chrono/Era;III)Ljava/time/LocalDate;")
        }

        #[java_method(name = "date", descriptor = "(III)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn date_i_i_i(&self, prolepticYear: i32, month: i32, dayOfMonth: i32) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.date:(III)Ljava/time/LocalDate;")
        }

        #[java_method(name = "dateYearDay", descriptor = "(Ljava/time/chrono/Era;II)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateYearDay_era_i_i(&self, era: Object, yearOfEra: i32, dayOfYear: i32) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateYearDay:(Ljava/time/chrono/Era;II)Ljava/time/LocalDate;")
        }

        #[java_method(name = "dateYearDay", descriptor = "(II)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateYearDay_i_i(&self, prolepticYear: i32, dayOfYear: i32) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateYearDay:(II)Ljava/time/LocalDate;")
        }

        #[java_method(name = "dateEpochDay", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateEpochDay(&self, epochDay: i64) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateEpochDay:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "date", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn date_tempor(&self, temporal: Object) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.date:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "epochSecond", descriptor = "(IIIIIILjava/time/ZoneOffset;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn epochSecond(&self, prolepticYear: i32, month: i32, dayOfMonth: i32, hour: i32, minute: i32, second: i32, zoneOffset: ZoneOffset) -> Result<i64> {
            panic!("stub: java/time/chrono/IsoChronology.epochSecond:(IIIIIILjava/time/ZoneOffset;)J")
        }

        #[java_method(name = "numberOfDaysOfMonth", descriptor = "(II)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn numberOfDaysOfMonth(&self, year: i32, month: i32) -> Result<i32> {
            panic!("stub: java/time/chrono/IsoChronology.numberOfDaysOfMonth:(II)I")
        }

        #[java_method(name = "localDateTime", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn localDateTime(&self, temporal: Object) -> Result<LocalDateTime> {
            panic!("stub: java/time/chrono/IsoChronology.localDateTime:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "zonedDateTime", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn zonedDateTime_tempor(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.zonedDateTime:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "zonedDateTime", descriptor = "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn zonedDateTime_instan_zoneid(&self, instant: Instant, zone: ZoneId) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.zonedDateTime:(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "dateNow", descriptor = "()Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateNow(&self) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateNow:()Ljava/time/LocalDate;")
        }

        #[java_method(name = "dateNow", descriptor = "(Ljava/time/ZoneId;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateNow_zoneid(&self, zone: ZoneId) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateNow:(Ljava/time/ZoneId;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "dateNow", descriptor = "(Ljava/time/Clock;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dateNow_clock(&self, clock: Object) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.dateNow:(Ljava/time/Clock;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "isLeapYear", descriptor = "(J)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear(&self, mut prolepticYear: i64) -> Result<bool> {
            let this = self;
            Ok((if ((((prolepticYear&(3i64))>(0i64)) as i32-(((prolepticYear&(3i64)))<(0i64)) as i32)==0) { ((((prolepticYear%(400i64))>(0i64)) as i32-(((prolepticYear%(400i64)))<(0i64)) as i32)==0) } else { (0i32 != 0) }))
        }

        #[java_method(name = "prolepticYear", descriptor = "(Ljava/time/chrono/Era;I)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prolepticYear(&self, era: Object, yearOfEra: i32) -> Result<i32> {
            panic!("stub: java/time/chrono/IsoChronology.prolepticYear:(Ljava/time/chrono/Era;I)I")
        }

        #[java_method(name = "eraOf", descriptor = "(I)Ljava/time/chrono/IsoEra;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn eraOf(&self, eraValue: i32) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.eraOf:(I)Ljava/time/chrono/IsoEra;")
        }

        #[java_method(name = "eras", descriptor = "()Ljava/util/List;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Ljava/time/chrono/Era;>;")]
        pub fn eras(&self) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.eras:()Ljava/util/List;")
        }

        #[java_method(name = "resolveDate", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")]
        pub fn resolveDate(&self, fieldValues: Object, resolverStyle: Object) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.resolveDate:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "resolveProlepticMonth", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)V")]
        pub fn resolveProlepticMonth(&self, fieldValues: Object, resolverStyle: Object) -> Result<()> {
            panic!("stub: java/time/chrono/IsoChronology.resolveProlepticMonth:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)V")
        }

        #[java_method(name = "resolveYearOfEra", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")]
        pub fn resolveYearOfEra(&self, fieldValues: Object, resolverStyle: Object) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.resolveYearOfEra:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "resolveYMD", descriptor = "(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")]
        pub fn resolveYMD(&self, fieldValues: Object, resolverStyle: Object) -> Result<LocalDate> {
            panic!("stub: java/time/chrono/IsoChronology.resolveYMD:(Ljava/util/Map;Ljava/time/format/ResolverStyle;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/ChronoField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: ChronoField) -> Result<ValueRange> {
            panic!("stub: java/time/chrono/IsoChronology.range:(Ljava/time/temporal/ChronoField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "period", descriptor = "(III)Ljava/time/Period;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn period(&self, years: i32, months: i32, days: i32) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.period:(III)Ljava/time/Period;")
        }

        #[java_method(name = "isIsoBased", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isIsoBased(&self) -> Result<bool> {
            panic!("stub: java/time/chrono/IsoChronology.isIsoBased:()Z")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/chrono/IsoChronology.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/chrono/IsoChronology.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
