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

impl From<RoundingMode> for Enum<Object> {
    fn from(v: RoundingMode) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/math/RoundingMode"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/math/RoundingMode;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "RoundingMode.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/math/RoundingMode"]

    pub struct RoundingMode {
        #[cfg_attr(any(), java_field(name = "oldMode", descriptor = "I", access = "package", modifiers = "final", is_static = false))]
        pub oldMode: i32,
    }

    impl RoundingMode {
        #[cfg_attr(any(), java_field(name = "UP", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UP:Ljava/math/RoundingMode;
        pub fn UP() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.UP:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "DOWN", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DOWN:Ljava/math/RoundingMode;
        pub fn DOWN() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.DOWN:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "CEILING", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CEILING:Ljava/math/RoundingMode;
        pub fn CEILING() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.CEILING:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "FLOOR", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FLOOR:Ljava/math/RoundingMode;
        pub fn FLOOR() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.FLOOR:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "HALF_UP", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HALF_UP:Ljava/math/RoundingMode;
        pub fn HALF_UP() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.HALF_UP:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "HALF_DOWN", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HALF_DOWN:Ljava/math/RoundingMode;
        pub fn HALF_DOWN() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.HALF_DOWN:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "HALF_EVEN", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: HALF_EVEN:Ljava/math/RoundingMode;
        pub fn HALF_EVEN() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.HALF_EVEN:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "UNNECESSARY", descriptor = "Ljava/math/RoundingMode;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UNNECESSARY:Ljava/math/RoundingMode;
        pub fn UNNECESSARY() -> RoundingMode {
            panic!("stub: java/math/RoundingMode.UNNECESSARY:Ljava/math/RoundingMode;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/math/RoundingMode;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/math/RoundingMode;
        pub fn _VALUES() -> Rc<RefCell<Vec<RoundingMode>>> {
            panic!("stub: java/math/RoundingMode.$VALUES:[Ljava/math/RoundingMode;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/math/RoundingMode;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<RoundingMode>>>> {
            panic!("stub: java/math/RoundingMode.values:()[Ljava/math/RoundingMode;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/math/RoundingMode;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf_str(name: String) -> Result<RoundingMode> {
            panic!("stub: java/math/RoundingMode.valueOf:(Ljava/lang/String;)Ljava/math/RoundingMode;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(I)V", method_parameters = ":4096;:4096;:0")]
        pub fn new(arg0: String, arg1: i32, oldMode: i32) -> Result<Self> {
            panic!("stub: java/math/RoundingMode.<init>:(Ljava/lang/String;II)V")
        }

        #[java_method(name = "valueOf", descriptor = "(I)Ljava/math/RoundingMode;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn valueOf_i(rm: i32) -> Result<RoundingMode> {
            panic!("stub: java/math/RoundingMode.valueOf:(I)Ljava/math/RoundingMode;")
        }
    }
}
