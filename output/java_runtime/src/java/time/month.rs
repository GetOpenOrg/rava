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

impl From<Month> for Enum<Object> {
    fn from(v: Month) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/Month"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = "java/time/temporal/TemporalAccessor,java/time/temporal/TemporalAdjuster"]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/time/Month;>;Ljava/time/temporal/TemporalAccessor;Ljava/time/temporal/TemporalAdjuster;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Month.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/time/Month;java/time/temporal/TemporalAccessor;java/time/temporal/TemporalAdjuster"]

    pub struct Month;

    impl Month {
        #[cfg_attr(any(), java_field(name = "JANUARY", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JANUARY:Ljava/time/Month;
        pub fn JANUARY() -> Month {
            panic!("stub: java/time/Month.JANUARY:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "FEBRUARY", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FEBRUARY:Ljava/time/Month;
        pub fn FEBRUARY() -> Month {
            panic!("stub: java/time/Month.FEBRUARY:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "MARCH", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MARCH:Ljava/time/Month;
        pub fn MARCH() -> Month {
            panic!("stub: java/time/Month.MARCH:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "APRIL", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: APRIL:Ljava/time/Month;
        pub fn APRIL() -> Month {
            panic!("stub: java/time/Month.APRIL:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "MAY", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MAY:Ljava/time/Month;
        pub fn MAY() -> Month {
            panic!("stub: java/time/Month.MAY:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "JUNE", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JUNE:Ljava/time/Month;
        pub fn JUNE() -> Month {
            panic!("stub: java/time/Month.JUNE:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "JULY", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JULY:Ljava/time/Month;
        pub fn JULY() -> Month {
            panic!("stub: java/time/Month.JULY:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "AUGUST", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: AUGUST:Ljava/time/Month;
        pub fn AUGUST() -> Month {
            panic!("stub: java/time/Month.AUGUST:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "SEPTEMBER", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SEPTEMBER:Ljava/time/Month;
        pub fn SEPTEMBER() -> Month {
            panic!("stub: java/time/Month.SEPTEMBER:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "OCTOBER", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: OCTOBER:Ljava/time/Month;
        pub fn OCTOBER() -> Month {
            panic!("stub: java/time/Month.OCTOBER:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "NOVEMBER", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: NOVEMBER:Ljava/time/Month;
        pub fn NOVEMBER() -> Month {
            panic!("stub: java/time/Month.NOVEMBER:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "DECEMBER", descriptor = "Ljava/time/Month;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DECEMBER:Ljava/time/Month;
        pub fn DECEMBER() -> Month {
            panic!("stub: java/time/Month.DECEMBER:Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "ENUMS", descriptor = "[Ljava/time/Month;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ENUMS:[Ljava/time/Month;
        pub fn ENUMS() -> Rc<RefCell<Vec<Month>>> {
            panic!("stub: java/time/Month.ENUMS:[Ljava/time/Month;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/time/Month;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/time/Month;
        pub fn _VALUES() -> Rc<RefCell<Vec<Month>>> {
            panic!("stub: java/time/Month.$VALUES:[Ljava/time/Month;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/time/Month;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Month>>>> {
            panic!("stub: java/time/Month.values:()[Ljava/time/Month;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/time/Month;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<Month> {
            panic!("stub: java/time/Month.valueOf:(Ljava/lang/String;)Ljava/time/Month;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/time/Month.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "of", descriptor = "(I)Ljava/time/Month;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(mut month: i32) -> Result<Month> {
            if month > 12i32 {
                let _t0 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Invalid value for MonthOfYear: ")))?;
                let _t1 = _t0.append_i(month)?;
                let _t2 = _t1.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(Clone::clone(&Month::ENUMS().borrow()[(month).wrapping_sub(1i32) as usize]))
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Month;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<Month> {
            panic!("stub: java/time/Month.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/Month;")
        }

        #[java_method(name = "getValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getValue(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__super().ordinal()?;
            Ok((_t0).wrapping_add(1i32))
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, style: Object, locale: Locale) -> Result<String> {
            panic!("stub: java/time/Month.getDisplayName:(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/Month.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/Month.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/Month.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/Month.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "plus", descriptor = "(J)Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus(&self, months: i64) -> Result<Month> {
            panic!("stub: java/time/Month.plus:(J)Ljava/time/Month;")
        }

        #[java_method(name = "minus", descriptor = "(J)Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus(&self, months: i64) -> Result<Month> {
            panic!("stub: java/time/Month.minus:(J)Ljava/time/Month;")
        }

        #[java_method(name = "length", descriptor = "(Z)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn length(&self, leapYear: bool) -> Result<i32> {
            panic!("stub: java/time/Month.length:(Z)I")
        }

        #[java_method(name = "minLength", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minLength(&self) -> Result<i32> {
            panic!("stub: java/time/Month.minLength:()I")
        }

        #[java_method(name = "maxLength", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn maxLength(&self) -> Result<i32> {
            panic!("stub: java/time/Month.maxLength:()I")
        }

        #[java_method(name = "firstDayOfYear", descriptor = "(Z)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstDayOfYear(&self, leapYear: bool) -> Result<i32> {
            panic!("stub: java/time/Month.firstDayOfYear:(Z)I")
        }

        #[java_method(name = "firstMonthOfQuarter", descriptor = "()Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstMonthOfQuarter(&self) -> Result<Month> {
            panic!("stub: java/time/Month.firstMonthOfQuarter:()Ljava/time/Month;")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/Month.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/Month.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }
    }
}
