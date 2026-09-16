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
    #[binary_name       = "java/time/temporal/TemporalAdjusters"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TemporalAdjusters.java"]
    #[inner_classes     = "java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/time/temporal/TemporalAdjusters"]

    pub struct TemporalAdjusters;

    impl TemporalAdjusters {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/time/temporal/TemporalAdjusters.<init>:()V")
        }

        #[java_method(name = "ofDateAdjuster", descriptor = "(Ljava/util/function/UnaryOperator;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/function/UnaryOperator<Ljava/time/LocalDate;>;)Ljava/time/temporal/TemporalAdjuster;")]
        pub fn ofDateAdjuster(dateBasedAdjuster: Object) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.ofDateAdjuster:(Ljava/util/function/UnaryOperator;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "firstDayOfMonth", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstDayOfMonth() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.firstDayOfMonth:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "lastDayOfMonth", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastDayOfMonth() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.lastDayOfMonth:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "firstDayOfNextMonth", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstDayOfNextMonth() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.firstDayOfNextMonth:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "firstDayOfYear", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstDayOfYear() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.firstDayOfYear:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "lastDayOfYear", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastDayOfYear() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.lastDayOfYear:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "firstDayOfNextYear", descriptor = "()Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstDayOfNextYear() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.firstDayOfNextYear:()Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "firstInMonth", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn firstInMonth(dayOfWeek: DayOfWeek) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.firstInMonth:(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "lastInMonth", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn lastInMonth(dayOfWeek: DayOfWeek) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.lastInMonth:(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "dayOfWeekInMonth", descriptor = "(ILjava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn dayOfWeekInMonth(ordinal: i32, dayOfWeek: DayOfWeek) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.dayOfWeekInMonth:(ILjava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "next", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn next(dayOfWeek: DayOfWeek) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.next:(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "nextOrSame", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nextOrSame(mut dayOfWeek: DayOfWeek) -> Result<Object> {
            let _t0 = dayOfWeek.getValue()?;
            let mut dowValue: i32 = _t0;
            let __lam_cap48_0 = dowValue;
            let __lam_48: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TemporalAdjusters::lambda_nextOrSame_10(__lam_cap48_0.clone(), _la0) });
            Ok(Object::from_any(__lam_48))
        }

        #[java_method(name = "previous", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previous(dayOfWeek: DayOfWeek) -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalAdjusters.previous:(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;")
        }

        #[java_method(name = "previousOrSame", descriptor = "(Ljava/time/DayOfWeek;)Ljava/time/temporal/TemporalAdjuster;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn previousOrSame(mut dayOfWeek: DayOfWeek) -> Result<Object> {
            let _t0 = dayOfWeek.getValue()?;
            let mut dowValue: i32 = _t0;
            let __lam_cap50_0 = dowValue;
            let __lam_50: std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>> = std::rc::Rc::new(move |_la0: Object| -> crate::error::Result<Object> { TemporalAdjusters::lambda_previousOrSame_12(__lam_cap50_0.clone(), _la0) });
            Ok(Object::from_any(__lam_50))
        }
    }
}
