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

impl From<DayOfWeek> for Enum<Object> {
    fn from(v: DayOfWeek) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/time/DayOfWeek"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = "java/time/temporal/TemporalAccessor,java/time/temporal/TemporalAdjuster"]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/time/DayOfWeek;>;Ljava/time/temporal/TemporalAccessor;Ljava/time/temporal/TemporalAdjuster;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "DayOfWeek.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/time/DayOfWeek;java/time/temporal/TemporalAccessor;java/time/temporal/TemporalAdjuster"]

    pub struct DayOfWeek;

    impl DayOfWeek {
        #[cfg_attr(any(), java_field(name = "MONDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: MONDAY:Ljava/time/DayOfWeek;
        pub fn MONDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.MONDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "TUESDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TUESDAY:Ljava/time/DayOfWeek;
        pub fn TUESDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.TUESDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "WEDNESDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: WEDNESDAY:Ljava/time/DayOfWeek;
        pub fn WEDNESDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.WEDNESDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "THURSDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: THURSDAY:Ljava/time/DayOfWeek;
        pub fn THURSDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.THURSDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "FRIDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FRIDAY:Ljava/time/DayOfWeek;
        pub fn FRIDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.FRIDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "SATURDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SATURDAY:Ljava/time/DayOfWeek;
        pub fn SATURDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.SATURDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "SUNDAY", descriptor = "Ljava/time/DayOfWeek;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SUNDAY:Ljava/time/DayOfWeek;
        pub fn SUNDAY() -> DayOfWeek {
            panic!("stub: java/time/DayOfWeek.SUNDAY:Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "ENUMS", descriptor = "[Ljava/time/DayOfWeek;", access = "private", modifiers = "static final", is_static = true))]
        // static field: ENUMS:[Ljava/time/DayOfWeek;
        pub fn ENUMS() -> Rc<RefCell<Vec<DayOfWeek>>> {
            panic!("stub: java/time/DayOfWeek.ENUMS:[Ljava/time/DayOfWeek;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/time/DayOfWeek;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/time/DayOfWeek;
        pub fn _VALUES() -> Rc<RefCell<Vec<DayOfWeek>>> {
            panic!("stub: java/time/DayOfWeek.$VALUES:[Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/time/DayOfWeek;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<DayOfWeek>>>> {
            panic!("stub: java/time/DayOfWeek.values:()[Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/time/DayOfWeek;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<DayOfWeek> {
            panic!("stub: java/time/DayOfWeek.valueOf:(Ljava/lang/String;)Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/time/DayOfWeek.<init>:(Ljava/lang/String;I)V")
        }

        #[java_method(name = "of", descriptor = "(I)Ljava/time/DayOfWeek;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(dayOfWeek: i32) -> Result<DayOfWeek> {
            panic!("stub: java/time/DayOfWeek.of:(I)Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "from", descriptor = "(Ljava/time/temporal/TemporalAccessor;)Ljava/time/DayOfWeek;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn from(temporal: Object) -> Result<DayOfWeek> {
            panic!("stub: java/time/DayOfWeek.from:(Ljava/time/temporal/TemporalAccessor;)Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "getValue", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getValue(&self) -> Result<i32> {
            let this = self;
            let _t0 = this.__super().ordinal()?;
            Ok((_t0).wrapping_add(1i32))
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self, style: Object, locale: Locale) -> Result<String> {
            panic!("stub: java/time/DayOfWeek.getDisplayName:(Ljava/time/format/TextStyle;Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "isSupported", descriptor = "(Ljava/time/temporal/TemporalField;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSupported(&self, field: Object) -> Result<bool> {
            panic!("stub: java/time/DayOfWeek.isSupported:(Ljava/time/temporal/TemporalField;)Z")
        }

        #[java_method(name = "range", descriptor = "(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn range(&self, field: Object) -> Result<ValueRange> {
            panic!("stub: java/time/DayOfWeek.range:(Ljava/time/temporal/TemporalField;)Ljava/time/temporal/ValueRange;")
        }

        #[java_method(name = "get", descriptor = "(Ljava/time/temporal/TemporalField;)I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn get(&self, field: Object) -> Result<i32> {
            panic!("stub: java/time/DayOfWeek.get:(Ljava/time/temporal/TemporalField;)I")
        }

        #[java_method(name = "getLong", descriptor = "(Ljava/time/temporal/TemporalField;)J", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLong(&self, field: Object) -> Result<i64> {
            panic!("stub: java/time/DayOfWeek.getLong:(Ljava/time/temporal/TemporalField;)J")
        }

        #[java_method(name = "plus", descriptor = "(J)Ljava/time/DayOfWeek;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn plus(&self, days: i64) -> Result<DayOfWeek> {
            panic!("stub: java/time/DayOfWeek.plus:(J)Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "minus", descriptor = "(J)Ljava/time/DayOfWeek;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn minus(&self, days: i64) -> Result<DayOfWeek> {
            panic!("stub: java/time/DayOfWeek.minus:(J)Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "query", descriptor = "(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<R:Ljava/lang/Object;>(Ljava/time/temporal/TemporalQuery<TR;>;)TR;")]
        pub fn query(&self, query: Object) -> Result<Object> {
            panic!("stub: java/time/DayOfWeek.query:(Ljava/time/temporal/TemporalQuery;)Ljava/lang/Object;")
        }

        #[java_method(name = "adjustInto", descriptor = "(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustInto(&self, temporal: Object) -> Result<Object> {
            panic!("stub: java/time/DayOfWeek.adjustInto:(Ljava/time/temporal/Temporal;)Ljava/time/temporal/Temporal;")
        }
    }
}
