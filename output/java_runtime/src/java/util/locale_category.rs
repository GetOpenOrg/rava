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

impl From<Locale_Category> for Enum<Object> {
    fn from(v: Locale_Category) -> Enum<Object> { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Locale$Category"]
    #[super_class       = "java/lang/Enum"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final enum"]
    #[generic_signature = "Ljava/lang/Enum<Ljava/util/Locale$Category;>;"]
    #[is_abstract       = false]
    #[is_enum           = true]
    #[is_deprecated     = false]
    #[source            = "Locale.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Enum<Object>"]
    #[superclass_fields(name: String, ordinal: i32, hash: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Comparable;java/lang/Enum;java/lang/Object;java/lang/constant/Constable;java/util/Locale$Category"]

    pub struct Locale_Category {
        #[cfg_attr(any(), java_field(name = "languageKey", descriptor = "Ljava/lang/String;", access = "package", modifiers = "final", is_static = false))]
        pub languageKey: String,
        #[cfg_attr(any(), java_field(name = "scriptKey", descriptor = "Ljava/lang/String;", access = "package", modifiers = "final", is_static = false))]
        pub scriptKey: String,
        #[cfg_attr(any(), java_field(name = "countryKey", descriptor = "Ljava/lang/String;", access = "package", modifiers = "final", is_static = false))]
        pub countryKey: String,
        #[cfg_attr(any(), java_field(name = "variantKey", descriptor = "Ljava/lang/String;", access = "package", modifiers = "final", is_static = false))]
        pub variantKey: String,
        #[cfg_attr(any(), java_field(name = "extensionsKey", descriptor = "Ljava/lang/String;", access = "package", modifiers = "final", is_static = false))]
        pub extensionsKey: String,
    }

    impl Locale_Category {
        #[cfg_attr(any(), java_field(name = "DISPLAY", descriptor = "Ljava/util/Locale$Category;", access = "public", modifiers = "static final", is_static = true))]
        // static field: DISPLAY:Ljava/util/Locale$Category;
        pub fn DISPLAY() -> Locale_Category {
            panic!("stub: java/util/Locale$Category.DISPLAY:Ljava/util/Locale$Category;")
        }

        #[cfg_attr(any(), java_field(name = "FORMAT", descriptor = "Ljava/util/Locale$Category;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FORMAT:Ljava/util/Locale$Category;
        pub fn FORMAT() -> Locale_Category {
            panic!("stub: java/util/Locale$Category.FORMAT:Ljava/util/Locale$Category;")
        }

        #[cfg_attr(any(), java_field(name = "$VALUES", descriptor = "[Ljava/util/Locale$Category;", access = "private", modifiers = "static final synthetic", is_static = true))]
        // static field: $VALUES:[Ljava/util/Locale$Category;
        pub fn _VALUES() -> Rc<RefCell<Vec<Locale_Category>>> {
            panic!("stub: java/util/Locale$Category.$VALUES:[Ljava/util/Locale$Category;")
        }

        #[java_method(name = "values", descriptor = "()[Ljava/util/Locale$Category;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn values() -> Result<Rc<RefCell<Vec<Locale_Category>>>> {
            panic!("stub: java/util/Locale$Category.values:()[Ljava/util/Locale$Category;")
        }

        #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/util/Locale$Category;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, method_parameters = ":32768")]
        pub fn valueOf(name: String) -> Result<Locale_Category> {
            panic!("stub: java/util/Locale$Category.valueOf:(Ljava/lang/String;)Ljava/util/Locale$Category;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", method_parameters = ":4096;:4096;:0;:0;:0;:0;:0")]
        pub fn new(arg0: String, arg1: i32, languageKey: String, scriptKey: String, countryKey: String, variantKey: String, extensionsKey: String) -> Result<Self> {
            panic!("stub: java/util/Locale$Category.<init>:(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V")
        }
    }
}
