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

impl From<Pattern_Qtype> for Enum<Object> {
    fn from(v: Pattern_Qtype) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/regex/Pattern$Qtype"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "package"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/util/regex/Pattern$Qtype;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Pattern.java"]
    #[inner_classes     = "java/util/regex/Pattern$Qtype:java/util/regex/Pattern:Qtype:16408"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/util/regex/Pattern$Qtype"]

    pub struct Pattern_Qtype;

    impl Pattern_Qtype {
        #[cfg_attr(any(), java_field(name = "GREEDY", descriptor = "Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GREEDY:Ljava/util/regex/Pattern$Qtype;
        pub fn GREEDY() -> Pattern_Qtype {
            panic!("stub: java/util/regex/Pattern$Qtype.GREEDY:Ljava/util/regex/Pattern$Qtype;")
        }

        #[cfg_attr(any(), java_field(name = "LAZY", descriptor = "Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static final", is_static = true))]
        // static field: LAZY:Ljava/util/regex/Pattern$Qtype;
        pub fn LAZY() -> Pattern_Qtype {
            panic!("stub: java/util/regex/Pattern$Qtype.LAZY:Ljava/util/regex/Pattern$Qtype;")
        }

        #[cfg_attr(any(), java_field(name = "POSSESSIVE", descriptor = "Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static final", is_static = true))]
        // static field: POSSESSIVE:Ljava/util/regex/Pattern$Qtype;
        pub fn POSSESSIVE() -> Pattern_Qtype {
            panic!("stub: java/util/regex/Pattern$Qtype.POSSESSIVE:Ljava/util/regex/Pattern$Qtype;")
        }

        #[cfg_attr(any(), java_field(name = "INDEPENDENT", descriptor = "Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static final", is_static = true))]
        // static field: INDEPENDENT:Ljava/util/regex/Pattern$Qtype;
        pub fn INDEPENDENT() -> Pattern_Qtype {
            panic!("stub: java/util/regex/Pattern$Qtype.INDEPENDENT:Ljava/util/regex/Pattern$Qtype;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/util/regex/Pattern$Qtype;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/util/regex/Pattern$Qtype;
        pub fn _VALUES() -> Rc<RefCell<Vec<Pattern_Qtype>>> {
            panic!("stub: java/util/regex/Pattern$Qtype.$VALUES:[Ljava/util/regex/Pattern$Qtype;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Pattern_Qtype>>>> {
            panic!("stub: java/util/regex/Pattern$Qtype.values:()[Ljava/util/regex/Pattern$Qtype;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/util/regex/Pattern$Qtype;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<Pattern_Qtype> {
            panic!("stub: java/util/regex/Pattern$Qtype.valueOf:(Ljava/lang/String;)Ljava/util/regex/Pattern$Qtype;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()V", method_parameters = ":4096;:4096")]
        pub fn new(arg0: String, arg1: i32) -> Result<Self> {
            panic!("stub: java/util/regex/Pattern$Qtype.<init>:(Ljava/lang/String;I)V")
        }
    }
}
