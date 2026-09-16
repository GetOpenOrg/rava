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
    #[binary_name       = "java/time/zone/ZoneOffsetTransitionRule"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "ZoneOffsetTransitionRule.java"]
    #[inner_classes     = "java/time/zone/ZoneOffsetTransitionRule$TimeDefinition:java/time/zone/ZoneOffsetTransitionRule:TimeDefinition:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/time/zone/ZoneOffsetTransitionRule"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct ZoneOffsetTransitionRule {
        #[cfg_attr(any(), java_field(name = "month", descriptor = "Ljava/time/Month;", access = "private", modifiers = "final", is_static = false))]
        pub month: Month,
        #[cfg_attr(any(), java_field(name = "dom", descriptor = "B", access = "private", modifiers = "final", is_static = false))]
        pub dom: i8,
        #[cfg_attr(any(), java_field(name = "dow", descriptor = "Ljava/time/DayOfWeek;", access = "private", modifiers = "final", is_static = false))]
        pub dow: DayOfWeek,
        #[cfg_attr(any(), java_field(name = "time", descriptor = "Ljava/time/LocalTime;", access = "private", modifiers = "final", is_static = false))]
        pub time: LocalTime,
        #[cfg_attr(any(), java_field(name = "timeEndOfDay", descriptor = "Z", access = "private", modifiers = "final", is_static = false))]
        pub timeEndOfDay: bool,
        #[cfg_attr(any(), java_field(name = "timeDefinition", descriptor = "Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "private", modifiers = "final", is_static = false))]
        pub timeDefinition: ZoneOffsetTransitionRule_TimeDefinition,
        #[cfg_attr(any(), java_field(name = "standardOffset", descriptor = "Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub standardOffset: ZoneOffset,
        #[cfg_attr(any(), java_field(name = "offsetBefore", descriptor = "Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub offsetBefore: ZoneOffset,
        #[cfg_attr(any(), java_field(name = "offsetAfter", descriptor = "Ljava/time/ZoneOffset;", access = "private", modifiers = "final", is_static = false))]
        pub offsetAfter: ZoneOffset,
    }

    impl ZoneOffsetTransitionRule {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "6889046316657758795"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            6889046316657758795i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "of", descriptor = "(Ljava/time/Month;ILjava/time/DayOfWeek;Ljava/time/LocalTime;ZLjava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneOffsetTransitionRule;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of(month: Month, dayOfMonthIndicator: i32, dayOfWeek: DayOfWeek, time: LocalTime, timeEndOfDay: bool, timeDefinition: ZoneOffsetTransitionRule_TimeDefinition, standardOffset: ZoneOffset, offsetBefore: ZoneOffset, offsetAfter: ZoneOffset) -> Result<ZoneOffsetTransitionRule> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.of:(Ljava/time/Month;ILjava/time/DayOfWeek;Ljava/time/LocalTime;ZLjava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)Ljava/time/zone/ZoneOffsetTransitionRule;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/time/Month;ILjava/time/DayOfWeek;Ljava/time/LocalTime;ZLjava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(month: Month, dayOfMonthIndicator: i32, dayOfWeek: DayOfWeek, time: LocalTime, timeEndOfDay: bool, timeDefinition: ZoneOffsetTransitionRule_TimeDefinition, standardOffset: ZoneOffset, offsetBefore: ZoneOffset, offsetAfter: ZoneOffset) -> Result<Self> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.<init>:(Ljava/time/Month;ILjava/time/DayOfWeek;Ljava/time/LocalTime;ZLjava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;Ljava/time/ZoneOffset;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/InvalidObjectException")]
        pub fn readObject(&self, s: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeReplace", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn writeReplace(&self) -> Result<Object> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.writeReplace:()Ljava/lang/Object;")
        }

        #[java_method(name = "writeExternal", descriptor = "(Ljava/io/DataOutput;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeExternal(&self, out: Object) -> Result<()> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.writeExternal:(Ljava/io/DataOutput;)V")
        }

        #[java_method(name = "readExternal", descriptor = "(Ljava/io/DataInput;)Ljava/time/zone/ZoneOffsetTransitionRule;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readExternal(in_: Object) -> Result<ZoneOffsetTransitionRule> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.readExternal:(Ljava/io/DataInput;)Ljava/time/zone/ZoneOffsetTransitionRule;")
        }

        #[java_method(name = "getMonth", descriptor = "()Ljava/time/Month;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonth(&self) -> Result<Month> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getMonth:()Ljava/time/Month;")
        }

        #[java_method(name = "getDayOfMonthIndicator", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfMonthIndicator(&self) -> Result<i32> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getDayOfMonthIndicator:()I")
        }

        #[java_method(name = "getDayOfWeek", descriptor = "()Ljava/time/DayOfWeek;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDayOfWeek(&self) -> Result<DayOfWeek> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getDayOfWeek:()Ljava/time/DayOfWeek;")
        }

        #[java_method(name = "getLocalTime", descriptor = "()Ljava/time/LocalTime;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalTime(&self) -> Result<LocalTime> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getLocalTime:()Ljava/time/LocalTime;")
        }

        #[java_method(name = "isMidnightEndOfDay", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isMidnightEndOfDay(&self) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.isMidnightEndOfDay:()Z")
        }

        #[java_method(name = "getTimeDefinition", descriptor = "()Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeDefinition(&self) -> Result<ZoneOffsetTransitionRule_TimeDefinition> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getTimeDefinition:()Ljava/time/zone/ZoneOffsetTransitionRule$TimeDefinition;")
        }

        #[java_method(name = "getStandardOffset", descriptor = "()Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getStandardOffset(&self) -> Result<ZoneOffset> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getStandardOffset:()Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "getOffsetBefore", descriptor = "()Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetBefore(&self) -> Result<ZoneOffset> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getOffsetBefore:()Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "getOffsetAfter", descriptor = "()Ljava/time/ZoneOffset;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getOffsetAfter(&self) -> Result<ZoneOffset> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.getOffsetAfter:()Ljava/time/ZoneOffset;")
        }

        #[java_method(name = "createTransition", descriptor = "(I)Ljava/time/zone/ZoneOffsetTransition;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createTransition(&self, mut year: i32) -> Result<ZoneOffsetTransition> {
            let this = self;
        let mut date: LocalDate = Default::default();
            if (this.__get_dom()<0) {
                let _t0 = IsoChronology::INSTANCE().isLeapYear((year as i64))?;
                let _t1 = this.__get_month().length(_t0)?;
                let _t2: LocalDate = LocalDate::of_i_month_i(year, Clone::clone(&this.__get_month()), ((_t1).wrapping_add(1i32)).wrapping_add((this.__get_dom() as i32)))?;
                date = _t2;
                let _t3: Object = TemporalAdjusters::previousOrSame(Clone::clone(&this.__get_dow()))?;
                let _t4 = date.with_tempor(Clone::clone(&_t3))?;
                date = _t4;
            } else {
                let _t0: LocalDate = LocalDate::of_i_month_i(year, Clone::clone(&this.__get_month()), (this.__get_dom() as i32))?;
                date = _t0;
                if !_is_jnull(&this.__get_dow()) {
                    let _t1: Object = TemporalAdjusters::nextOrSame(Clone::clone(&this.__get_dow()))?;
                    let _t2 = date.with_tempor(Clone::clone(&_t1))?;
                    date = _t2;
                }
            }
            if this.__get_timeEndOfDay() {
                let _t0 = date.plusDays(1i64)?;
                date = _t0;
            }
            let _t0: LocalDateTime = LocalDateTime::of_locald_localt(Clone::clone(&date), Clone::clone(&this.__get_time()))?;
            let mut localDT: LocalDateTime = _t0;
            let _t1 = this.__get_timeDefinition().createDateTime(Clone::clone(&localDT), Clone::clone(&this.__get_standardOffset()), Clone::clone(&this.__get_offsetBefore()))?;
            let mut transition: LocalDateTime = _t1;
            Ok(ZoneOffsetTransition::new_locald_zoneof_zoneof(Clone::clone(&transition), Clone::clone(&this.__get_offsetBefore()), Clone::clone(&this.__get_offsetAfter()))?)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, otherRule: Object) -> Result<bool> {
            panic!("stub: java/time/zone/ZoneOffsetTransitionRule.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }
    }
}
