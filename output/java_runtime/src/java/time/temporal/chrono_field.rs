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

impl From<ChronoField> for Enum<Object> {
    fn from(v: ChronoField) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/temporal/ChronoField"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = "java/time/temporal/TemporalField"]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/time/temporal/ChronoField;>;Ljava/time/temporal/TemporalField;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "ChronoField.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/time/temporal/ChronoField;java/time/temporal/TemporalField"]
    #[has_to_string_method = true]

    pub struct ChronoField {
        #[cfg_attr(any(), java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub name: String,
        #[cfg_attr(any(), java_field(name = "baseUnit", descriptor = "Ljava/time/temporal/TemporalUnit;", access = "private", modifiers = "final", is_static = false))]
        pub baseUnit: Object,
        #[cfg_attr(any(), java_field(name = "rangeUnit", descriptor = "Ljava/time/temporal/TemporalUnit;", access = "private", modifiers = "final", is_static = false))]
        pub rangeUnit: Object,
        #[cfg_attr(any(), java_field(name = "range", descriptor = "Ljava/time/temporal/ValueRange;", access = "private", modifiers = "final", is_static = false))]
        pub range: ValueRange,
        #[cfg_attr(any(), java_field(name = "displayNameKey", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub displayNameKey: String,
    }

    impl ChronoField {
        #[cfg_attr(any(), java_field(name = "NANO_OF_SECOND", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NANO_OF_SECOND:Ljava/time/temporal/ChronoField;
        pub fn NANO_OF_SECOND() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.NANO_OF_SECOND:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "NANO_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NANO_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn NANO_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.NANO_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MICRO_OF_SECOND", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MICRO_OF_SECOND:Ljava/time/temporal/ChronoField;
        pub fn MICRO_OF_SECOND() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MICRO_OF_SECOND:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MICRO_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MICRO_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn MICRO_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MICRO_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MILLI_OF_SECOND", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MILLI_OF_SECOND:Ljava/time/temporal/ChronoField;
        pub fn MILLI_OF_SECOND() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MILLI_OF_SECOND:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MILLI_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MILLI_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn MILLI_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MILLI_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "SECOND_OF_MINUTE", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SECOND_OF_MINUTE:Ljava/time/temporal/ChronoField;
        pub fn SECOND_OF_MINUTE() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.SECOND_OF_MINUTE:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "SECOND_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SECOND_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn SECOND_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.SECOND_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MINUTE_OF_HOUR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MINUTE_OF_HOUR:Ljava/time/temporal/ChronoField;
        pub fn MINUTE_OF_HOUR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MINUTE_OF_HOUR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MINUTE_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MINUTE_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn MINUTE_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MINUTE_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "HOUR_OF_AMPM", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HOUR_OF_AMPM:Ljava/time/temporal/ChronoField;
        pub fn HOUR_OF_AMPM() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.HOUR_OF_AMPM:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "CLOCK_HOUR_OF_AMPM", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CLOCK_HOUR_OF_AMPM:Ljava/time/temporal/ChronoField;
        pub fn CLOCK_HOUR_OF_AMPM() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.CLOCK_HOUR_OF_AMPM:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "HOUR_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HOUR_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn HOUR_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.HOUR_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "CLOCK_HOUR_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CLOCK_HOUR_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn CLOCK_HOUR_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.CLOCK_HOUR_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "AMPM_OF_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AMPM_OF_DAY:Ljava/time/temporal/ChronoField;
        pub fn AMPM_OF_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.AMPM_OF_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_WEEK", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DAY_OF_WEEK:Ljava/time/temporal/ChronoField;
        pub fn DAY_OF_WEEK() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.DAY_OF_WEEK:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "ALIGNED_DAY_OF_WEEK_IN_MONTH", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALIGNED_DAY_OF_WEEK_IN_MONTH:Ljava/time/temporal/ChronoField;
        pub fn ALIGNED_DAY_OF_WEEK_IN_MONTH() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.ALIGNED_DAY_OF_WEEK_IN_MONTH:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "ALIGNED_DAY_OF_WEEK_IN_YEAR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALIGNED_DAY_OF_WEEK_IN_YEAR:Ljava/time/temporal/ChronoField;
        pub fn ALIGNED_DAY_OF_WEEK_IN_YEAR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.ALIGNED_DAY_OF_WEEK_IN_YEAR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_MONTH", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DAY_OF_MONTH:Ljava/time/temporal/ChronoField;
        pub fn DAY_OF_MONTH() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.DAY_OF_MONTH:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "DAY_OF_YEAR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DAY_OF_YEAR:Ljava/time/temporal/ChronoField;
        pub fn DAY_OF_YEAR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.DAY_OF_YEAR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "EPOCH_DAY", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: EPOCH_DAY:Ljava/time/temporal/ChronoField;
        pub fn EPOCH_DAY() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.EPOCH_DAY:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "ALIGNED_WEEK_OF_MONTH", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALIGNED_WEEK_OF_MONTH:Ljava/time/temporal/ChronoField;
        pub fn ALIGNED_WEEK_OF_MONTH() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.ALIGNED_WEEK_OF_MONTH:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "ALIGNED_WEEK_OF_YEAR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ALIGNED_WEEK_OF_YEAR:Ljava/time/temporal/ChronoField;
        pub fn ALIGNED_WEEK_OF_YEAR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.ALIGNED_WEEK_OF_YEAR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "MONTH_OF_YEAR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONTH_OF_YEAR:Ljava/time/temporal/ChronoField;
        pub fn MONTH_OF_YEAR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.MONTH_OF_YEAR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "PROLEPTIC_MONTH", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PROLEPTIC_MONTH:Ljava/time/temporal/ChronoField;
        pub fn PROLEPTIC_MONTH() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.PROLEPTIC_MONTH:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "YEAR_OF_ERA", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YEAR_OF_ERA:Ljava/time/temporal/ChronoField;
        pub fn YEAR_OF_ERA() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.YEAR_OF_ERA:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "YEAR", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: YEAR:Ljava/time/temporal/ChronoField;
        pub fn YEAR() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.YEAR:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "ERA", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ERA:Ljava/time/temporal/ChronoField;
        pub fn ERA() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.ERA:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "INSTANT_SECONDS", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INSTANT_SECONDS:Ljava/time/temporal/ChronoField;
        pub fn INSTANT_SECONDS() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.INSTANT_SECONDS:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "OFFSET_SECONDS", descriptor = "Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OFFSET_SECONDS:Ljava/time/temporal/ChronoField;
        pub fn OFFSET_SECONDS() -> ChronoField {
            panic!("stub: java/time/temporal/ChronoField.OFFSET_SECONDS:Ljava/time/temporal/ChronoField;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/time/temporal/ChronoField;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/time/temporal/ChronoField;
        pub fn _VALUES() -> Rc<RefCell<Vec<ChronoField>>> {
            panic!("stub: java/time/temporal/ChronoField.$VALUES:[Ljava/time/temporal/ChronoField;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<ChronoField>>>> {
            panic!("stub: java/time/temporal/ChronoField.values:()[Ljava/time/temporal/ChronoField;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/time/temporal/ChronoField;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<ChronoField> {
            panic!("stub: java/time/temporal/ChronoField.valueOf:(Ljava/lang/String;)Ljava/time/temporal/ChronoField;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;ILjava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;)V", method_parameters = ":4096;:4096;:0;:0;:0;:0")]
        pub fn new_str_i_str_tempor_tempor_valuer(arg0: String, arg1: i32, name: String, baseUnit: Object, rangeUnit: Object, range: ValueRange) -> Result<Self> {
            panic!("stub: java/time/temporal/ChronoField.<init>:(Ljava/lang/String;ILjava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;ILjava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;Ljava/lang/String;)V", method_parameters = ":4096;:4096;:0;:0;:0;:0;:0")]
        pub fn new_str_i_str_tempor_tempor_valuer_str(arg0: String, arg1: i32, name: String, baseUnit: Object, rangeUnit: Object, range: ValueRange, displayNameKey: String) -> Result<Self> {
            panic!("stub: java/time/temporal/ChronoField.<init>:(Ljava/lang/String;ILjava/lang/String;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/TemporalUnit;Ljava/time/temporal/ValueRange;Ljava/lang/String;)V")
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, locale: Locale) -> Result<String> {
            panic!("stub: java/time/temporal/ChronoField.getDisplayName:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getBaseUnit", descriptor = "()Ljava/time/temporal/TemporalUnit;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBaseUnit(&self) -> Result<Object> {
            panic!("stub: java/time/temporal/ChronoField.getBaseUnit:()Ljava/time/temporal/TemporalUnit;")
        }

        #[java_method(name = "getRangeUnit", descriptor = "()Ljava/time/temporal/TemporalUnit;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRangeUnit(&self) -> Result<Object> {
            panic!("stub: java/time/temporal/ChronoField.getRangeUnit:()Ljava/time/temporal/TemporalUnit;")
        }

        #[java_method(name = "range", descriptor = "()Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self) -> Result<ValueRange> {
            let this = self;
            Ok(this.__get_range())
        }

        #[java_method(name = "isDateBased", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDateBased(&self) -> Result<bool> {
            panic!("stub: java/time/temporal/ChronoField.isDateBased:()Z")
        }

        #[java_method(name = "isTimeBased", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isTimeBased(&self) -> Result<bool> {
            panic!("stub: java/time/temporal/ChronoField.isTimeBased:()Z")
        }

        #[java_method(name = "checkValidValue", descriptor = "(J)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkValidValue(&self, mut value: i64) -> Result<i64> {
            let this = self;
            let _t0 = this.range()?;
            let _t1 = _t0.checkValidValue(value, Object::from_any(Clone::clone(self)))?;
            Ok(_t1)
        }

        #[java_method(name = "checkValidIntValue", descriptor = "(J)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkValidIntValue(&self, value: i64) -> Result<i32> {
            panic!("stub: java/time/temporal/ChronoField.checkValidIntValue:(J)I")
        }

        #[java_method(name = "isSupportedBy", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupportedBy(&self, temporal: Object) -> Result<bool> {
            panic!("stub: java/time/temporal/ChronoField.isSupportedBy:(Ljava/time/temporal/TemporalAccessor;)Z")
        }

        #[java_method(name = "rangeRefinedBy", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn rangeRefinedBy(&self, temporal: Object) -> Result<ValueRange> {
            panic!("stub: java/time/temporal/ChronoField.rangeRefinedBy:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "getFrom", descriptor = "(Ljava/time/temporal/TemporalAccessor;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFrom(&self, temporal: Object) -> Result<i64> {
            panic!("stub: java/time/temporal/ChronoField.getFrom:(Ljava/time/temporal/TemporalAccessor;)J")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R::Ljava/time/temporal/Temporal;>(TR;J)TR;")]
        pub fn adjustInto(&self, temporal: Object, newValue: i64) -> Result<Object> {
            panic!("stub: java/time/temporal/ChronoField.adjustInto:(Ljava/time/temporal/Temporal;J)Ljava/time/temporal/Temporal;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "resolve", descriptor = "(Ljava/util/Map;Ljava/time/temporal/TemporalAccessor;Ljava/time/format/ResolverStyle;)Ljava/time/temporal/TemporalAccessor;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<Ljava/time/temporal/TemporalField;Ljava/lang/Long;>;Ljava/time/temporal/TemporalAccessor;Ljava/time/format/ResolverStyle;)Ljava/time/temporal/TemporalAccessor;")]
        pub fn resolve(&self, fieldValues: Object, partialTemporal: Object, resolverStyle: Object) -> Result<Object> {
            panic!("stub: java/time/temporal/ChronoField.resolve:(Ljava/util/Map;Ljava/time/temporal/TemporalAccessor;Ljava/time/format/ResolverStyle;)Ljava/time/temporal/TemporalAccessor;")
        }
    }
}
