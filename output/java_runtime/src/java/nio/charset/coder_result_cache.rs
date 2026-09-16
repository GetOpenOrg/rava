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
    #[binary_name       = "java/nio/charset/CoderResult$Cache"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CoderResult.java"]
    #[inner_classes     = "java/nio/charset/CoderResult$Cache:java/nio/charset/CoderResult:Cache:26"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/nio/charset/CoderResult$Cache"]

    pub struct CoderResult_Cache {
        #[cfg_attr(any(), java_field(name = "unmappable", descriptor = "Ljava/util/Map;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/Integer;Ljava/nio/charset/CoderResult;>;"))]
        pub unmappable: Object,
        #[cfg_attr(any(), java_field(name = "malformed", descriptor = "Ljava/util/Map;", access = "package", modifiers = "final", is_static = false, generic_signature = "Ljava/util/Map<Ljava/lang/Integer;Ljava/nio/charset/CoderResult;>;"))]
        pub malformed: Object,
    }

    impl CoderResult_Cache {
        #[cfg_attr(any(), java_field(name = "INSTANCE", descriptor = "Ljava/nio/charset/CoderResult$Cache;", access = "package", modifiers = "static final", is_static = true))]
        // static field: INSTANCE:Ljava/nio/charset/CoderResult$Cache;
        pub fn INSTANCE() -> CoderResult_Cache {
            panic!("stub: java/nio/charset/CoderResult$Cache.INSTANCE:Ljava/nio/charset/CoderResult$Cache;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/nio/charset/CoderResult$Cache.<init>:()V")
        }
    }
}
