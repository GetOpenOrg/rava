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
    #[binary_name       = "sun/util/locale/provider/CalendarDataUtility"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "CalendarDataUtility.java"]
    #[inner_classes     = "sun/util/locale/provider/CalendarDataUtility$CalendarWeekParameterGetter:sun/util/locale/provider/CalendarDataUtility:CalendarWeekParameterGetter:10;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545;sun/util/locale/provider/CalendarDataUtility$CalendarFieldValueNameGetter:sun/util/locale/provider/CalendarDataUtility:CalendarFieldValueNameGetter:10;sun/util/locale/provider/CalendarDataUtility$CalendarFieldValueNamesMapGetter:sun/util/locale/provider/CalendarDataUtility:CalendarFieldValueNamesMapGetter:10;java/util/Locale$Builder:java/util/Locale:Builder:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/provider/CalendarDataUtility"]

    pub struct CalendarDataUtility;

    impl CalendarDataUtility {
        #[cfg_attr(any(), java_field(name = "FIRST_DAY_OF_WEEK", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "firstDayOfWeek"))]
        // static field: FIRST_DAY_OF_WEEK:Ljava/lang/String;
        pub fn FIRST_DAY_OF_WEEK() -> String {
            String::from("firstDayOfWeek")
        }

        #[cfg_attr(any(), java_field(name = "MINIMAL_DAYS_IN_FIRST_WEEK", descriptor = "Ljava/lang/String;", access = "public", modifiers = "static final", is_static = true, constant_value = "minimalDaysInFirstWeek"))]
        // static field: MINIMAL_DAYS_IN_FIRST_WEEK:Ljava/lang/String;
        pub fn MINIMAL_DAYS_IN_FIRST_WEEK() -> String {
            String::from("minimalDaysInFirstWeek")
        }

        #[cfg_attr(any(), java_field(name = "OVERRIDE_BUILDER", descriptor = "Ljava/util/Locale$Builder;", access = "private", modifiers = "static final", is_static = true))]
        // static field: OVERRIDE_BUILDER:Ljava/util/Locale$Builder;
        pub fn OVERRIDE_BUILDER() -> Object {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.OVERRIDE_BUILDER:Ljava/util/Locale$Builder;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.<init>:()V")
        }

        #[java_method(name = "retrieveFirstDayOfWeek", descriptor = "(Ljava/util/Locale;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveFirstDayOfWeek(locale: Locale) -> Result<i32> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveFirstDayOfWeek:(Ljava/util/Locale;)I")
        }

        #[java_method(name = "retrieveMinimalDaysInFirstWeek", descriptor = "(Ljava/util/Locale;)I", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveMinimalDaysInFirstWeek(locale: Locale) -> Result<i32> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveMinimalDaysInFirstWeek:(Ljava/util/Locale;)I")
        }

        #[java_method(name = "retrieveFieldValueName", descriptor = "(Ljava/lang/String;IIILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveFieldValueName(id: String, field: i32, value: i32, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveFieldValueName:(Ljava/lang/String;IIILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "retrieveJavaTimeFieldValueName", descriptor = "(Ljava/lang/String;IIILjava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn retrieveJavaTimeFieldValueName(id: String, field: i32, value: i32, style: i32, locale: Locale) -> Result<String> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveJavaTimeFieldValueName:(Ljava/lang/String;IIILjava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "retrieveFieldValueNames", descriptor = "(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn retrieveFieldValueNames(id: String, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveFieldValueNames:(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "retrieveJavaTimeFieldValueNames", descriptor = "(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map<Ljava/lang/String;Ljava/lang/Integer;>;")]
        pub fn retrieveJavaTimeFieldValueNames(id: String, field: i32, style: i32, locale: Locale) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.retrieveJavaTimeFieldValueNames:(Ljava/lang/String;IILjava/util/Locale;)Ljava/util/Map;")
        }

        #[java_method(name = "findRegionOverride", descriptor = "(Ljava/util/Locale;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findRegionOverride(l: Locale) -> Result<Locale> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.findRegionOverride:(Ljava/util/Locale;)Ljava/util/Locale;")
        }

        #[java_method(name = "normalizeCalendarType", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn normalizeCalendarType(requestID: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/CalendarDataUtility.normalizeCalendarType:(Ljava/lang/String;)Ljava/lang/String;")
        }
    }
}
