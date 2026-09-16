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
    #[binary_name       = "sun/util/locale/provider/TimeZoneNameUtility"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "TimeZoneNameUtility.java"]
    #[inner_classes     = "sun/util/locale/provider/LocaleProviderAdapter$Type:sun/util/locale/provider/LocaleProviderAdapter:Type:16409;sun/util/locale/provider/TimeZoneNameUtility$TimeZoneNameGetter:sun/util/locale/provider/TimeZoneNameUtility:TimeZoneNameGetter:10;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/provider/TimeZoneNameUtility"]

    pub struct TimeZoneNameUtility;

    impl TimeZoneNameUtility {
        #[cfg_attr(any(), java_field(name = "cachedZoneData", descriptor = "Ljava/util/concurrent/ConcurrentHashMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentHashMap<Ljava/util/Locale;Ljava/lang/ref/SoftReference<[[Ljava/lang/String;>;>;"))]
        // static field: cachedZoneData:Ljava/util/concurrent/ConcurrentHashMap;
        pub fn cachedZoneData() -> ConcurrentHashMap<Locale, SoftReference<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.cachedZoneData:Ljava/util/concurrent/ConcurrentHashMap;")
        }

        #[cfg_attr(any(), java_field(name = "cachedDisplayNames", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/lang/ref/SoftReference<Ljava/util/Map<Ljava/util/Locale;[Ljava/lang/String;>;>;>;"))]
        // static field: cachedDisplayNames:Ljava/util/Map;
        pub fn cachedDisplayNames() -> Object {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.cachedDisplayNames:Ljava/util/Map;")
        }

        #[java_method(name = "getZoneStrings", descriptor = "(Ljava/util/Locale;)[[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneStrings(locale: Locale) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.getZoneStrings:(Ljava/util/Locale;)[[Ljava/lang/String;")
        }

        #[java_method(name = "loadZoneStrings", descriptor = "(Ljava/util/Locale;)[[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn loadZoneStrings(locale: Locale) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.loadZoneStrings:(Ljava/util/Locale;)[[Ljava/lang/String;")
        }

        #[java_method(name = "retrieveDisplayNames", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveDisplayNames(id: String, locale: Locale) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.retrieveDisplayNames:(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;")
        }

        #[java_method(name = "retrieveGenericDisplayName", descriptor = "(Ljava/lang/String;ILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveGenericDisplayName(id: String, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.retrieveGenericDisplayName:(Ljava/lang/String;ILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "retrieveDisplayName", descriptor = "(Ljava/lang/String;ZILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveDisplayName(id: String, daylight: bool, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.retrieveDisplayName:(Ljava/lang/String;ZILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "convertLDMLShortID", descriptor = "(Ljava/lang/String;)Ljava/util/Optional;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/Optional<Ljava/lang/String;>;")]
        pub fn convertLDMLShortID(shortID: String) -> Result<Optional<Object>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.convertLDMLShortID:(Ljava/lang/String;)Ljava/util/Optional;")
        }

        #[java_method(name = "canonicalTZID", descriptor = "(Ljava/lang/String;)Ljava/util/Optional;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/Optional<Ljava/lang/String;>;")]
        pub fn canonicalTZID(id: String) -> Result<Optional<Object>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.canonicalTZID:(Ljava/lang/String;)Ljava/util/Optional;")
        }

        #[java_method(name = "retrieveDisplayNamesImpl", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveDisplayNamesImpl(id: String, locale: Locale) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.retrieveDisplayNamesImpl:(Ljava/lang/String;Ljava/util/Locale;)[Ljava/lang/String;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/provider/TimeZoneNameUtility.<init>:()V")
        }
    }
}
