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
    #[binary_name       = "sun/util/locale/LocaleUtils"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleUtils.java"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/LocaleUtils"]

    pub struct LocaleUtils;

    impl LocaleUtils {
        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/LocaleUtils.<init>:()V")
        }

        #[java_method(name = "caseIgnoreMatch", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn caseIgnoreMatch(s1: String, s2: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.caseIgnoreMatch:(Ljava/lang/String;Ljava/lang/String;)Z")
        }

        #[java_method(name = "caseIgnoreCompare", descriptor = "(Ljava/lang/String;Ljava/lang/String;)I", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn caseIgnoreCompare(s1: String, s2: String) -> Result<i32> {
            panic!("stub: sun/util/locale/LocaleUtils.caseIgnoreCompare:(Ljava/lang/String;Ljava/lang/String;)I")
        }

        #[java_method(name = "toUpper", descriptor = "(C)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpper(c: u16) -> Result<u16> {
            panic!("stub: sun/util/locale/LocaleUtils.toUpper:(C)C")
        }

        #[java_method(name = "toLower", descriptor = "(C)C", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLower(c: u16) -> Result<u16> {
            panic!("stub: sun/util/locale/LocaleUtils.toLower:(C)C")
        }

        #[java_method(name = "toLowerString", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLowerString(s: String) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleUtils.toLowerString:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "toUpperString", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperString(s: String) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleUtils.toUpperString:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "toTitleString", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toTitleString(s: String) -> Result<String> {
            panic!("stub: sun/util/locale/LocaleUtils.toTitleString:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "isUpper", descriptor = "(C)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUpper(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isUpper:(C)Z")
        }

        #[java_method(name = "isLower", descriptor = "(C)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isLower(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isLower:(C)Z")
        }

        #[java_method(name = "isAlpha", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlpha(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isAlpha:(C)Z")
        }

        #[java_method(name = "isAlphaString", descriptor = "(Ljava/lang/String;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlphaString(s: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isAlphaString:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "isNumeric", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNumeric(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isNumeric:(C)Z")
        }

        #[java_method(name = "isNumericString", descriptor = "(Ljava/lang/String;)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isNumericString(s: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isNumericString:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "isAlphaNumeric", descriptor = "(C)Z", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlphaNumeric(c: u16) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isAlphaNumeric:(C)Z")
        }

        #[java_method(name = "isAlphaNumericString", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isAlphaNumericString(s: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isAlphaNumericString:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "isEmpty", descriptor = "(Ljava/lang/String;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isEmpty_str(str: String) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isEmpty:(Ljava/lang/String;)Z")
        }

        #[java_method(name = "isEmpty", descriptor = "(Ljava/util/Set;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Set<*>;)Z")]
        pub fn isEmpty_set(set: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isEmpty:(Ljava/util/Set;)Z")
        }

        #[java_method(name = "isEmpty", descriptor = "(Ljava/util/Map;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Map<**>;)Z")]
        pub fn isEmpty_map(map: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isEmpty:(Ljava/util/Map;)Z")
        }

        #[java_method(name = "isEmpty", descriptor = "(Ljava/util/List;)Z", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<*>;)Z")]
        pub fn isEmpty_list(list: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/LocaleUtils.isEmpty:(Ljava/util/List;)Z")
        }
    }
}
