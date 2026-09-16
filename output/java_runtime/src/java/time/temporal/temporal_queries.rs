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
    #[binary_name       = "java/time/temporal/TemporalQueries"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TemporalQueries.java"]
    #[inner_classes     = "java/time/temporal/TemporalQueries$1:::0;java/time/temporal/TemporalQueries$2:::0;java/time/temporal/TemporalQueries$3:::0;java/time/temporal/TemporalQueries$4:::0;java/time/temporal/TemporalQueries$5:::0;java/time/temporal/TemporalQueries$6:::0;java/time/temporal/TemporalQueries$7:::0"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/time/temporal/TemporalQueries"]

    pub struct TemporalQueries;

    impl TemporalQueries {
        #[cfg_attr(any(), java_field(name = "ZONE_ID", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneId;>;"))]
        // static field: ZONE_ID:Ljava/time/temporal/TemporalQuery;
        pub fn ZONE_ID() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.ZONE_ID:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "CHRONO", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/chrono/Chronology;>;"))]
        // static field: CHRONO:Ljava/time/temporal/TemporalQuery;
        pub fn CHRONO() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.CHRONO:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "PRECISION", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/temporal/TemporalUnit;>;"))]
        // static field: PRECISION:Ljava/time/temporal/TemporalQuery;
        pub fn PRECISION() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.PRECISION:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "OFFSET", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneOffset;>;"))]
        // static field: OFFSET:Ljava/time/temporal/TemporalQuery;
        pub fn OFFSET() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.OFFSET:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "ZONE", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneId;>;"))]
        // static field: ZONE:Ljava/time/temporal/TemporalQuery;
        pub fn ZONE() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.ZONE:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "LOCAL_DATE", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/LocalDate;>;"))]
        // static field: LOCAL_DATE:Ljava/time/temporal/TemporalQuery;
        pub fn LOCAL_DATE() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.LOCAL_DATE:Ljava/time/temporal/TemporalQuery;")
        }

        #[cfg_attr(any(), java_field(name = "LOCAL_TIME", descriptor = "Ljava/time/temporal/TemporalQuery;", access = "package", modifiers = "static final", is_static = true, generic_signature = "Ljava/time/temporal/TemporalQuery<Ljava/time/LocalTime;>;"))]
        // static field: LOCAL_TIME:Ljava/time/temporal/TemporalQuery;
        pub fn LOCAL_TIME() -> Object {
            panic!("stub: java/time/temporal/TemporalQueries.LOCAL_TIME:Ljava/time/temporal/TemporalQuery;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/time/temporal/TemporalQueries.<init>:()V")
        }

        #[java_method(name = "zoneId", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneId;>;")]
        pub fn zoneId() -> Result<Object> {
            Ok(TemporalQueries::ZONE_ID())
        }

        #[java_method(name = "chronology", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/chrono/Chronology;>;")]
        pub fn chronology() -> Result<Object> {
            Ok(TemporalQueries::CHRONO())
        }

        #[java_method(name = "precision", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/temporal/TemporalUnit;>;")]
        pub fn precision() -> Result<Object> {
            Ok(TemporalQueries::PRECISION())
        }

        #[java_method(name = "zone", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneId;>;")]
        pub fn zone() -> Result<Object> {
            Ok(TemporalQueries::ZONE())
        }

        #[java_method(name = "offset", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/ZoneOffset;>;")]
        pub fn offset() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalQueries.offset:()Ljava/time/temporal/TemporalQuery;")
        }

        #[java_method(name = "localDate", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/LocalDate;>;")]
        pub fn localDate() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalQueries.localDate:()Ljava/time/temporal/TemporalQuery;")
        }

        #[java_method(name = "localTime", descriptor = "()Ljava/time/temporal/TemporalQuery;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/time/temporal/TemporalQuery<Ljava/time/LocalTime;>;")]
        pub fn localTime() -> Result<Object> {
            panic!("stub: java/time/temporal/TemporalQueries.localTime:()Ljava/time/temporal/TemporalQuery;")
        }
    }
}
