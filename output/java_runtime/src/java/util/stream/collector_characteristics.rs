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

impl From<Collector_Characteristics> for Enum<Object> {
    fn from(v: Collector_Characteristics) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/stream/Collector$Characteristics"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/util/stream/Collector$Characteristics;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Collector.java"]
    #[inner_classes     = "java/util/stream/Collector$Characteristics:java/util/stream/Collector:Characteristics:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/util/stream/Collector$Characteristics"]

    pub struct Collector_Characteristics;

    impl Collector_Characteristics {
        #[cfg_attr(any(), java_field(name = "CONCURRENT", descriptor = "Ljava/util/stream/Collector$Characteristics;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CONCURRENT:Ljava/util/stream/Collector$Characteristics;
        pub fn CONCURRENT() -> Collector_Characteristics {
            panic!("stub: java/util/stream/Collector$Characteristics.CONCURRENT:Ljava/util/stream/Collector$Characteristics;")
        }

        #[cfg_attr(any(), java_field(name = "UNORDERED", descriptor = "Ljava/util/stream/Collector$Characteristics;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNORDERED:Ljava/util/stream/Collector$Characteristics;
        pub fn UNORDERED() -> Collector_Characteristics {
            panic!("stub: java/util/stream/Collector$Characteristics.UNORDERED:Ljava/util/stream/Collector$Characteristics;")
        }

        #[cfg_attr(any(), java_field(name = "IDENTITY_FINISH", descriptor = "Ljava/util/stream/Collector$Characteristics;", access = "public", modifiers = "static final", is_static = true))]
        // static field: IDENTITY_FINISH:Ljava/util/stream/Collector$Characteristics;
        pub fn IDENTITY_FINISH() -> Collector_Characteristics {
            panic!("stub: java/util/stream/Collector$Characteristics.IDENTITY_FINISH:Ljava/util/stream/Collector$Characteristics;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/util/stream/Collector$Characteristics;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/util/stream/Collector$Characteristics;
        pub fn _VALUES() -> Rc<RefCell<Vec<Collector_Characteristics>>> {
            panic!("stub: java/util/stream/Collector$Characteristics.$VALUES:[Ljava/util/stream/Collector$Characteristics;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/util/stream/Collector$Characteristics;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Collector_Characteristics>>>> {
            panic!("stub: java/util/stream/Collector$Characteristics.values:()[Ljava/util/stream/Collector$Characteristics;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/util/stream/Collector$Characteristics;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<Collector_Characteristics> {
            panic!("stub: java/util/stream/Collector$Characteristics.valueOf:(Ljava/lang/String;)Ljava/util/stream/Collector$Characteristics;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/util/stream/Collector$Characteristics.<init>:(Ljava/lang/String;I)V")
        }
    }
}
