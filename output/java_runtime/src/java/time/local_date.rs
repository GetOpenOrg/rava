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
    #[binary_name       = "java/time/LocalDate"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/time/temporal/Temporal,java/time/temporal/TemporalAdjuster,java/time/chrono/ChronoLocalDate,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocalDate.java"]
    #[inner_classes     = "java/time/LocalDate$1:::4104;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/LocalDate;java/time/chrono/ChronoLocalDate;java/time/temporal/Temporal;java/time/temporal/TemporalAdjuster"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct LocalDate {
        #[cfg_attr(any(), java_field(name = "year", descriptor = "I", access = "private", modifiers = "final", is_static = false))]
        pub year: i32,
        #[cfg_attr(any(), java_field(name = "month", descriptor = "S", access = "private", modifiers = "final", is_static = false))]
        pub month: i16,
        #[cfg_attr(any(), java_field(name = "day", descriptor = "S", access = "private", modifiers = "final", is_static = false))]
        pub day: i16,
    }

    impl LocalDate {
        #[cfg_attr(any(), java_field(name = "MIN", descriptor = "Ljava/time/LocalDate;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MIN:Ljava/time/LocalDate;
        pub fn MIN() -> LocalDate {
            panic!("stub: java/time/LocalDate.MIN:Ljava/time/LocalDate;")
        }

        #[cfg_attr(any(), java_field(name = "MAX", descriptor = "Ljava/time/LocalDate;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAX:Ljava/time/LocalDate;
        pub fn MAX() -> LocalDate {
            panic!("stub: java/time/LocalDate.MAX:Ljava/time/LocalDate;")
        }

        #[cfg_attr(any(), java_field(name = "EPOCH", descriptor = "Ljava/time/LocalDate;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EPOCH:Ljava/time/LocalDate;
        pub fn EPOCH() -> LocalDate {
            panic!("stub: java/time/LocalDate.EPOCH:Ljava/time/LocalDate;")
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "2942565459149668126"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            2942565459149668126i64
        }

        #[cfg_attr(any(), java_field(name = "DAYS_PER_CYCLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "146097"))]
        // static field: DAYS_PER_CYCLE:I
        pub fn DAYS_PER_CYCLE() -> i32 {
            146097
        }

        #[cfg_attr(any(), java_field(name = "DAYS_0000_TO_1970", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "719528"))]
        // static field: DAYS_0000_TO_1970:J
        pub fn DAYS_0000_TO_1970() -> i64 {
            719528i64
        }

        #[java_method(name = "now", descriptor = "()Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now() -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.now:()Ljava/time/LocalDate;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/ZoneId;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_zoneid(zone: ZoneId) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.now:(Ljava/time/ZoneId;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "now", descriptor = "(Ljava/time/Clock;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn now_clock(clock: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.now:(Ljava/time/Clock;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "of", descriptor = "(ILjava/time/Month;I)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: of(ILjava/time/Month;I)Ljava/time/LocalDate;
        pub fn of_i_month_i(mut year: i32, mut month: Month, mut dayOfMonth: i32) -> Result<LocalDate> {
            let _t0 = ChronoField::YEAR().checkValidValue((year as i64))?;
            let _t1: Object = Objects::requireNonNull_obj_str(Object::from_any(month.clone()), Clone::clone(&String::from("month")))?;
            let _t2 = ChronoField::DAY_OF_MONTH().checkValidValue((dayOfMonth as i64))?;
            let _t3 = month.getValue()?;
            let _t4: LocalDate = LocalDate::create(year, _t3, dayOfMonth)?;
            Ok(_t4)
        }

        #[java_method(name = "of", descriptor = "(III)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_i_i_i(year: i32, month: i32, dayOfMonth: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.of:(III)Ljava/time/LocalDate;")
        }

        #[java_method(name = "ofYearDay", descriptor = "(II)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofYearDay(year: i32, dayOfYear: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.ofYearDay:(II)Ljava/time/LocalDate;")
        }

        #[java_method(name = "ofInstant", descriptor = "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofInstant(instant: Instant, zone: ZoneId) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.ofInstant:(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "ofEpochDay", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ofEpochDay(mut epochDay: i64) -> Result<LocalDate> {
            let _t0 = ChronoField::EPOCH_DAY().checkValidValue(epochDay)?;
            let mut zeroDay = (epochDay).wrapping_add(719528i64);
            zeroDay = (zeroDay).wrapping_sub(60i64);
            let mut adjust: i64 = 0i64;
            if (((zeroDay>(0i64)) as i32-((zeroDay)<(0i64)) as i32)<0) {
                let mut adjustCycles = (((zeroDay).wrapping_add(1i64)/146097i64)).wrapping_sub(1i64);
                adjust = (adjustCycles).wrapping_mul(400i64);
                zeroDay = (zeroDay).wrapping_add(((adjustCycles).wrapping_neg()).wrapping_mul(146097i64));
            }
            let mut adjustCycles = (((400i64).wrapping_mul(zeroDay)).wrapping_add(591i64)/146097i64);
            let mut doyEst = (zeroDay).wrapping_sub(((((365i64).wrapping_mul(adjustCycles)).wrapping_add((adjustCycles/4i64))).wrapping_sub((adjustCycles/100i64))).wrapping_add((adjustCycles/400i64)));
            if (((doyEst>(0i64)) as i32-((doyEst)<(0i64)) as i32)<0) {
                adjustCycles = (adjustCycles).wrapping_sub(1i64);
                doyEst = (zeroDay).wrapping_sub(((((365i64).wrapping_mul(adjustCycles)).wrapping_add((adjustCycles/4i64))).wrapping_sub((adjustCycles/100i64))).wrapping_add((adjustCycles/400i64)));
            }
            adjustCycles = (adjustCycles).wrapping_add(adjust);
            let mut marchDoy0: i32 = (doyEst as i32);
            let mut marchMonth0 = (((marchDoy0).wrapping_mul(5i32)).wrapping_add(2i32)/153i32);
            let mut month = (marchMonth0).wrapping_add(3i32);
            if month > 12i32 {
                month = month.wrapping_sub(12i32);
            }
            let mut dom = ((marchDoy0).wrapping_sub((((marchMonth0).wrapping_mul(306i32)).wrapping_add(5i32)/10i32))).wrapping_add(1i32);
            if marchDoy0 >= 306i32 {
                adjustCycles = (adjustCycles).wrapping_add(1i64);
            }
            Ok(LocalDate::new((adjustCycles as i32), month, dom)?)
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq(text: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.parse:(Ljava/lang/CharSequence;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalDate;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse_seq_dateti(text: Object, formatter: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.parse:(Ljava/lang/CharSequence;Ljava/time/format/DateTimeFormatter;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "create", descriptor = "(III)Ljava/time/LocalDate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn create(year: i32, month: i32, dayOfMonth: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.create:(III)Ljava/time/LocalDate;")
        }

        #[java_method(name = "resolvePreviousValid", descriptor = "(III)Ljava/time/LocalDate;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolvePreviousValid(year: i32, month: i32, day: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.resolvePreviousValid:(III)Ljava/time/LocalDate;")
        }

        #[java_method(name = "<init>", descriptor = "(III)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut year: i32, mut month: i32, mut dayOfMonth: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_year(year);
            this.__set_month(((((month) as i16 as i32)) as i16));
            this.__set_day(((((dayOfMonth) as i16 as i32)) as i16));
            Ok(this)
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalUnit;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported_tempor_1(&self, unit: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.isSupported:(Ljava/time/temporal/TemporalUnit;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/LocalDate.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/LocalDate.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/LocalDate.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "get0", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get0(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/LocalDate.get0:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getProlepticMonth", descriptor = "()J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProlepticMonth(&self) -> Result<i64> {
            panic!("stub: java/time/LocalDate.getProlepticMonth:()J")
        }

        #[java_method(name = "getChronology", descriptor = "()Ljava/time/chrono/IsoChronology;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getChronology(&self) -> Result<IsoChronology> {
            panic!("stub: java/time/LocalDate.getChronology:()Ljava/time/chrono/IsoChronology;")
        }

        #[java_method(name = "getEra", descriptor = "()Ljava/time/chrono/IsoEra;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEra(&self) -> Result<Object> {
            panic!("stub: java/time/LocalDate.getEra:()Ljava/time/chrono/IsoEra;")
        }

        #[java_method(name = "getYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getYear(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.getYear:()I")
        }

        #[java_method(name = "getMonthValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonthValue(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.getMonthValue:()I")
        }

        #[java_method(name = "getMonth", descriptor = "()Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonth(&self) -> Result<Month> {
            panic!("stub: java/time/LocalDate.getMonth:()Ljava/time/Month;")
        }

        #[java_method(name = "getDayOfMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfMonth(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.getDayOfMonth:()I")
        }

        #[java_method(name = "getDayOfYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfYear(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.getDayOfYear:()I")
        }

        #[java_method(name = "getDayOfWeek", descriptor = "()Ljava/time/DayOfWeek;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeek(&self) -> Result<DayOfWeek> {
            panic!("stub: java/time/LocalDate.getDayOfWeek:()Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "isLeapYear", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLeapYear(&self) -> Result<bool> {
            let this = self;
            let _t0 = IsoChronology::INSTANCE().isLeapYear((this.__get_year() as i64))?;
            Ok(_t0)
        }

        #[java_method(name = "lengthOfMonth", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lengthOfMonth(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.lengthOfMonth:()I")
        }

        #[java_method(name = "lengthOfYear", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lengthOfYear(&self) -> Result<i32> {
            panic!("stub: java/time/LocalDate.lengthOfYear:()I")
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: with(Ljava/time/temporal/TemporalAdjuster;)Ljava/time/LocalDate;
        pub fn with_tempor(&self, mut adjuster: Object) -> Result<LocalDate> {
            let this = self;
            if (adjuster.is_instance_of("java/time/LocalDate")) {
                return Ok((adjuster).downcast::<LocalDate>());
            }
            let _vdispatch0: Object = if let Some(_d) = adjuster.0.as_any().downcast_ref::<LocalTime>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<DayOfWeek>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<LocalDateTime>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<LocalDate>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<Month>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<Instant>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<ZoneOffset>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(_d) = adjuster.0.as_any().downcast_ref::<Object>() { _d.adjustInto(Object::from_any(Clone::clone(self)))? } else if let Some(__f) = adjuster.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(Clone::clone(self)))? } else { Default::default() };
            Ok((_vdispatch0).downcast::<LocalDate>())
        }

        #[java_method(name = "with", descriptor = "(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn with_tempor_l(&self, field: Object, newValue: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.with:(Ljava/time/temporal/TemporalField;J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "withYear", descriptor = "(I)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withYear(&self, year: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.withYear:(I)Ljava/time/LocalDate;")
        }

        #[java_method(name = "withMonth", descriptor = "(I)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withMonth(&self, month: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.withMonth:(I)Ljava/time/LocalDate;")
        }

        #[java_method(name = "withDayOfMonth", descriptor = "(I)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withDayOfMonth(&self, dayOfMonth: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.withDayOfMonth:(I)Ljava/time/LocalDate;")
        }

        #[java_method(name = "withDayOfYear", descriptor = "(I)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn withDayOfYear(&self, dayOfYear: i32) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.withDayOfYear:(I)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_tempor(&self, amountToAdd: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.plus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus_l_tempor(&self, amountToAdd: i64, arg1: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.plus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plusYears", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusYears(&self, yearsToAdd: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.plusYears:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plusMonths", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusMonths(&self, monthsToAdd: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.plusMonths:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plusWeeks", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusWeeks(&self, weeksToAdd: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.plusWeeks:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "plusDays", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plusDays(&self, mut daysToAdd: i64) -> Result<LocalDate> {
            let this = self;
            if (((daysToAdd>(0i64)) as i32-((daysToAdd)<(0i64)) as i32)==0) {
                return Ok(Clone::clone(this));
            }
            let mut dom = ((this.__get_day() as i64)).wrapping_add(daysToAdd);
            if (((dom>(28i64)) as i32-((dom)<(28i64)) as i32)<=0) {
                return Ok(LocalDate::new(this.__get_year(), (this.__get_month() as i32), (dom as i32))?);
            }
            let _t0 = this.lengthOfMonth()?;
            let mut monthLen: i64 = (_t0 as i64);
            if (((dom>(monthLen)) as i32-((dom)<(monthLen)) as i32)<=0) {
                return Ok(LocalDate::new(this.__get_year(), (this.__get_month() as i32), (dom as i32))?);
            }
            if (this.__get_month() as i32) < 12i32 {
                return Ok(LocalDate::new(this.__get_year(), ((this.__get_month() as i32)).wrapping_add(1i32), ((dom).wrapping_sub(monthLen) as i32))?);
            }
            let _t1 = ChronoField::YEAR().checkValidValue(((this.__get_year()).wrapping_add(1i32) as i64))?;
            return Ok(LocalDate::new((this.__get_year()).wrapping_add(1i32), 1i32, ((dom).wrapping_sub(monthLen) as i32))?);
            let _t2 = this.toEpochDay()?;
            let _t3: i64 = Math::addExact_l_l(_t2, daysToAdd)?;
            monthLen = _t3;
            let _t4: LocalDate = LocalDate::ofEpochDay(monthLen)?;
            Ok(_t4)
        }

        #[java_method(name = "minus", descriptor = "(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_tempor(&self, amountToSubtract: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minus:(Ljava/time/temporal/TemporalAmount;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "minus", descriptor = "(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus_l_tempor(&self, amountToSubtract: i64, arg1: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minus:(JLjava/time/temporal/TemporalUnit;)Ljava/time/LocalDate;")
        }

        #[java_method(name = "minusYears", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusYears(&self, yearsToSubtract: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minusYears:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "minusMonths", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusMonths(&self, monthsToSubtract: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minusMonths:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "minusWeeks", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusWeeks(&self, weeksToSubtract: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minusWeeks:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "minusDays", descriptor = "(J)Ljava/time/LocalDate;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minusDays(&self, daysToSubtract: i64) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.minusDays:(J)Ljava/time/LocalDate;")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDate.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDate.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "until", descriptor = "(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn until_tempor_tempor(&self, endExclusive: Object, unit: Object) -> Result<i64> {
            panic!("stub: java/time/LocalDate.until:(Ljava/time/temporal/Temporal;Ljava/time/temporal/TemporalUnit;)J")
        }

        #[java_method(name = "daysUntil", descriptor = "(Ljava/time/LocalDate;)J", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn daysUntil(&self, end: LocalDate) -> Result<i64> {
            panic!("stub: java/time/LocalDate.daysUntil:(Ljava/time/LocalDate;)J")
        }

        #[java_method(name = "monthsUntil", descriptor = "(Ljava/time/LocalDate;)J", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn monthsUntil(&self, end: LocalDate) -> Result<i64> {
            panic!("stub: java/time/LocalDate.monthsUntil:(Ljava/time/LocalDate;)J")
        }

        #[java_method(name = "until", descriptor = "(Ljava/time/chrono/ChronoLocalDate;)Ljava/time/Period;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn until_chrono(&self, endDateExclusive: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDate.until:(Ljava/time/chrono/ChronoLocalDate;)Ljava/time/Period;")
        }

        #[java_method(name = "datesUntil", descriptor = "(Ljava/time/LocalDate;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/LocalDate;)Ljava/util/stream/Stream<Ljava/time/LocalDate;>;")]
        pub fn datesUntil_locald(&self, endExclusive: LocalDate) -> Result<Object> {
            panic!("stub: java/time/LocalDate.datesUntil:(Ljava/time/LocalDate;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "datesUntil", descriptor = "(Ljava/time/LocalDate;Ljava/time/Period;)Ljava/util/stream/Stream;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/time/LocalDate;Ljava/time/Period;)Ljava/util/stream/Stream<Ljava/time/LocalDate;>;")]
        pub fn datesUntil_locald_period(&self, endExclusive: LocalDate, step: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDate.datesUntil:(Ljava/time/LocalDate;Ljava/time/Period;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format(&self, formatter: Object) -> Result<String> {
            panic!("stub: java/time/LocalDate.format:(Ljava/time/format/DateTimeFormatter;)Ljava/lang/String;")
        }

        #[java_method(name = "atTime", descriptor = "(Ljava/time/LocalTime;)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atTime_localt(&self, time: LocalTime) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDate.atTime:(Ljava/time/LocalTime;)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atTime", descriptor = "(II)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atTime_i_i(&self, hour: i32, minute: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDate.atTime:(II)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atTime", descriptor = "(III)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atTime_i_i_i(&self, hour: i32, minute: i32, second: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDate.atTime:(III)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atTime", descriptor = "(IIII)Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atTime_i_i_i_i(&self, hour: i32, minute: i32, second: i32, nanoOfSecond: i32) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDate.atTime:(IIII)Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atTime", descriptor = "(Ljava/time/OffsetTime;)Ljava/time/OffsetDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atTime_offset(&self, time: Object) -> Result<Object> {
            panic!("stub: java/time/LocalDate.atTime:(Ljava/time/OffsetTime;)Ljava/time/OffsetDateTime;")
        }

        #[java_method(name = "atStartOfDay", descriptor = "()Ljava/time/LocalDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atStartOfDay(&self) -> Result<LocalDateTime> {
            panic!("stub: java/time/LocalDate.atStartOfDay:()Ljava/time/LocalDateTime;")
        }

        #[java_method(name = "atStartOfDay", descriptor = "(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn atStartOfDay_zoneid(&self, zone: ZoneId) -> Result<Object> {
            panic!("stub: java/time/LocalDate.atStartOfDay:(Ljava/time/ZoneId;)Ljava/time/ZonedDateTime;")
        }

        #[java_method(name = "toEpochDay", descriptor = "()J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEpochDay(&self) -> Result<i64> {
            let this = self;
            let mut y: i64 = (this.__get_year() as i64);
            let mut m: i64 = (this.__get_month() as i64);
            let mut total: i64 = 0i64;
            total = (total).wrapping_add((365i64).wrapping_mul(y));
            if (((y>(0i64)) as i32-((y)<(0i64)) as i32)>=0) {
                total = (total).wrapping_add(((((y).wrapping_add(3i64)/4i64)).wrapping_sub(((y).wrapping_add(99i64)/100i64))).wrapping_add(((y).wrapping_add(399i64)/400i64)));
            } else {
                total = (total).wrapping_sub((((y/-4i64)).wrapping_sub((y/-100i64))).wrapping_add((y/-400i64)));
            }
            total = (total).wrapping_add((((367i64).wrapping_mul(m)).wrapping_sub(362i64)/12i64));
            total = (total).wrapping_add((((this.__get_day() as i32)).wrapping_sub(1i32) as i64));
            total = (total).wrapping_sub(1i64);
            let _t0 = this.isLeapYear()?;
            if !(_t0) {
                total = (total).wrapping_sub(1i64);
            }
            Ok((total).wrapping_sub(719528i64))
        }

        #[java_method(name = "toEpochSecond", descriptor = "(Ljava/time/LocalTime;Ljava/time/ZoneOffset;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toEpochSecond(&self, time: LocalTime, offset: ZoneOffset) -> Result<i64> {
            panic!("stub: java/time/LocalDate.toEpochSecond:(Ljava/time/LocalTime;Ljava/time/ZoneOffset;)J")
        }

        #[java_method(name = "compareTo", descriptor = "(Ljava/time/chrono/ChronoLocalDate;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo(&self, other: Object) -> Result<i32> {
            panic!("stub: java/time/LocalDate.compareTo:(Ljava/time/chrono/ChronoLocalDate;)I")
        }

        #[java_method(name = "compareTo0", descriptor = "(Ljava/time/LocalDate;)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn compareTo0(&self, otherDate: LocalDate) -> Result<i32> {
            panic!("stub: java/time/LocalDate.compareTo0:(Ljava/time/LocalDate;)I")
        }

        #[java_method(name = "isAfter", descriptor = "(Ljava/time/chrono/ChronoLocalDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAfter(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.isAfter:(Ljava/time/chrono/ChronoLocalDate;)Z")
        }

        #[java_method(name = "isBefore", descriptor = "(Ljava/time/chrono/ChronoLocalDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isBefore(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.isBefore:(Ljava/time/chrono/ChronoLocalDate;)Z")
        }

        #[java_method(name = "isEqual", descriptor = "(Ljava/time/chrono/ChronoLocalDate;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEqual(&self, other: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.isEqual:(Ljava/time/chrono/ChronoLocalDate;)Z")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/time/LocalDate.equals:(Ljava/lang/Object;)Z")
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
            panic!("stub: java/time/LocalDate.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/LocalDate.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/LocalDate.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/LocalDate;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<LocalDate> {
            panic!("stub: java/time/LocalDate.readExternal:(Ljava/io/DataInput;)Ljava/time/LocalDate;")
        }
    }
}
