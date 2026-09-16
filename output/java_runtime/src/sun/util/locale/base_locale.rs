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
use crate::jdk::internal::util::StaticProperty;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "sun/util/locale/BaseLocale"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "BaseLocale.java"]
    #[inner_classes     = "sun/util/locale/BaseLocale$Key:sun/util/locale/BaseLocale:Key:26;sun/util/locale/BaseLocale$Cache:sun/util/locale/BaseLocale:Cache:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/BaseLocale"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct BaseLocale {
        #[cfg_attr(any(), java_field(name = "language", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub language: String,
        #[cfg_attr(any(), java_field(name = "script", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub script: String,
        #[cfg_attr(any(), java_field(name = "region", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub region: String,
        #[cfg_attr(any(), java_field(name = "variant", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub variant: String,
        #[cfg_attr(any(), java_field(name = "hash", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub hash: i32,
    }

    impl BaseLocale {
        #[cfg_attr(any(), java_field(name = "constantBaseLocales", descriptor = "[Lsun/util/locale/BaseLocale;", access = "public", modifiers = "static", is_static = true))]
        // static field: constantBaseLocales:[Lsun/util/locale/BaseLocale;
        pub fn constantBaseLocales() -> Rc<RefCell<Vec<BaseLocale>>> {
            panic!("stub: sun/util/locale/BaseLocale.constantBaseLocales:[Lsun/util/locale/BaseLocale;")
        }

        #[cfg_attr(any(), java_field(name = "ENGLISH", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: ENGLISH:B
        pub fn ENGLISH() -> i8 {
            0
        }

        #[cfg_attr(any(), java_field(name = "FRENCH", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: FRENCH:B
        pub fn FRENCH() -> i8 {
            1
        }

        #[cfg_attr(any(), java_field(name = "GERMAN", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: GERMAN:B
        pub fn GERMAN() -> i8 {
            2
        }

        #[cfg_attr(any(), java_field(name = "ITALIAN", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: ITALIAN:B
        pub fn ITALIAN() -> i8 {
            3
        }

        #[cfg_attr(any(), java_field(name = "JAPANESE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: JAPANESE:B
        pub fn JAPANESE() -> i8 {
            4
        }

        #[cfg_attr(any(), java_field(name = "KOREAN", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: KOREAN:B
        pub fn KOREAN() -> i8 {
            5
        }

        #[cfg_attr(any(), java_field(name = "CHINESE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: CHINESE:B
        pub fn CHINESE() -> i8 {
            6
        }

        #[cfg_attr(any(), java_field(name = "SIMPLIFIED_CHINESE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: SIMPLIFIED_CHINESE:B
        pub fn SIMPLIFIED_CHINESE() -> i8 {
            7
        }

        #[cfg_attr(any(), java_field(name = "TRADITIONAL_CHINESE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: TRADITIONAL_CHINESE:B
        pub fn TRADITIONAL_CHINESE() -> i8 {
            8
        }

        #[cfg_attr(any(), java_field(name = "FRANCE", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: FRANCE:B
        pub fn FRANCE() -> i8 {
            9
        }

        #[cfg_attr(any(), java_field(name = "GERMANY", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: GERMANY:B
        pub fn GERMANY() -> i8 {
            10
        }

        #[cfg_attr(any(), java_field(name = "ITALY", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: ITALY:B
        pub fn ITALY() -> i8 {
            11
        }

        #[cfg_attr(any(), java_field(name = "JAPAN", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: JAPAN:B
        pub fn JAPAN() -> i8 {
            12
        }

        #[cfg_attr(any(), java_field(name = "KOREA", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: KOREA:B
        pub fn KOREA() -> i8 {
            13
        }

        #[cfg_attr(any(), java_field(name = "UK", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: UK:B
        pub fn UK() -> i8 {
            14
        }

        #[cfg_attr(any(), java_field(name = "US", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: US:B
        pub fn US() -> i8 {
            15
        }

        #[cfg_attr(any(), java_field(name = "CANADA", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: CANADA:B
        pub fn CANADA() -> i8 {
            16
        }

        #[cfg_attr(any(), java_field(name = "CANADA_FRENCH", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "17"))]
        // static field: CANADA_FRENCH:B
        pub fn CANADA_FRENCH() -> i8 {
            17
        }

        #[cfg_attr(any(), java_field(name = "ROOT", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: ROOT:B
        pub fn ROOT() -> i8 {
            18
        }

        #[cfg_attr(any(), java_field(name = "NUM_CONSTANTS", descriptor = "B", access = "public", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: NUM_CONSTANTS:B
        pub fn NUM_CONSTANTS() -> i8 {
            19
        }

        #[cfg_attr(any(), java_field(name = "SEP", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "_"))]
        // static field: SEP:Ljava/lang/String;
        pub fn SEP() -> String {
            String::from("_")
        }

        #[cfg_attr(any(), java_field(name = "OLD_ISO_CODES", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: OLD_ISO_CODES:Z
        pub fn OLD_ISO_CODES() -> bool {
            panic!("stub: sun/util/locale/BaseLocale.OLD_ISO_CODES:Z")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(language: String, script: String, region: String, variant: String, normalize: bool) -> Result<Self> {
            panic!("stub: sun/util/locale/BaseLocale.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V")
        }

        #[java_method(name = "createInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/BaseLocale;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createInstance(language: String, region: String) -> Result<BaseLocale> {
            panic!("stub: sun/util/locale/BaseLocale.createInstance:(Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/BaseLocale;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/BaseLocale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance(language: String, script: String, region: String, variant: String) -> Result<BaseLocale> {
            panic!("stub: sun/util/locale/BaseLocale.getInstance:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/BaseLocale;")
        }

        #[java_method(name = "convertOldISOCodes", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertOldISOCodes(language: String) -> Result<String> {
            panic!("stub: sun/util/locale/BaseLocale.convertOldISOCodes:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getLanguage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLanguage(&self) -> Result<String> {
            panic!("stub: sun/util/locale/BaseLocale.getLanguage:()Ljava/lang/String;")
        }

        #[java_method(name = "getScript", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScript(&self) -> Result<String> {
            panic!("stub: sun/util/locale/BaseLocale.getScript:()Ljava/lang/String;")
        }

        #[java_method(name = "getRegion", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRegion(&self) -> Result<String> {
            panic!("stub: sun/util/locale/BaseLocale.getRegion:()Ljava/lang/String;")
        }

        #[java_method(name = "getVariant", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getVariant(&self) -> Result<String> {
            panic!("stub: sun/util/locale/BaseLocale.getVariant:()Ljava/lang/String;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/BaseLocale.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }
    }
}
