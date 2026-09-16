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
    #[binary_name       = "sun/util/locale/provider/LocaleProviderAdapter"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleProviderAdapter.java"]
    #[inner_classes     = "sun/util/locale/provider/LocaleProviderAdapter$Type:sun/util/locale/provider/LocaleProviderAdapter:Type:16409;java/util/ResourceBundle$Control:java/util/ResourceBundle:Control:9;java/lang/System$Logger:java/lang/System:Logger:1545;java/lang/System$Logger$Level:java/lang/System$Logger:Level:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/provider/LocaleProviderAdapter"]

    pub struct LocaleProviderAdapter;

    impl LocaleProviderAdapter {
        #[cfg_attr(any(), java_field(name = "adapterPreference", descriptor = "Ljava/util/List;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/List<Lsun/util/locale/provider/LocaleProviderAdapter$Type;>;"))]
        // static field: adapterPreference:Ljava/util/List;
        pub fn adapterPreference() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.adapterPreference:Ljava/util/List;")
        }

        #[cfg_attr(any(), java_field(name = "adapterInstances", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Lsun/util/locale/provider/LocaleProviderAdapter$Type;Lsun/util/locale/provider/LocaleProviderAdapter;>;"))]
        // static field: adapterInstances:Ljava/util/Map;
        pub fn adapterInstances() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.adapterInstances:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "defaultLocaleProviderAdapter", descriptor = "Lsun/util/locale/provider/LocaleProviderAdapter$Type;", access = "package", modifiers = "static volatile", is_static = true))]
        // static field: defaultLocaleProviderAdapter:Lsun/util/locale/provider/LocaleProviderAdapter$Type;
        pub fn defaultLocaleProviderAdapter() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.defaultLocaleProviderAdapter:Lsun/util/locale/provider/LocaleProviderAdapter$Type;")
        }

        #[cfg_attr(any(), java_field(name = "adapterCache", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;Ljava/util/concurrent/ConcurrentMap<Ljava/util/Locale;Lsun/util/locale/provider/LocaleProviderAdapter;>;>;"))]
        // static field: adapterCache:Ljava/util/concurrent/ConcurrentMap;
        pub fn adapterCache() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.adapterCache:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.<init>:()V")
        }

        #[java_method(name = "forType", descriptor = "(Lsun/util/locale/provider/LocaleProviderAdapter$Type;)Lsun/util/locale/provider/LocaleProviderAdapter;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forType(type_: Object) -> Result<LocaleProviderAdapter> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.forType:(Lsun/util/locale/provider/LocaleProviderAdapter$Type;)Lsun/util/locale/provider/LocaleProviderAdapter;")
        }

        #[java_method(name = "forJRE", descriptor = "()Lsun/util/locale/provider/LocaleProviderAdapter;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forJRE() -> Result<LocaleProviderAdapter> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.forJRE:()Lsun/util/locale/provider/LocaleProviderAdapter;")
        }

        #[java_method(name = "getResourceBundleBased", descriptor = "()Lsun/util/locale/provider/LocaleProviderAdapter;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getResourceBundleBased() -> Result<LocaleProviderAdapter> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getResourceBundleBased:()Lsun/util/locale/provider/LocaleProviderAdapter;")
        }

        #[java_method(name = "getAdapterPreference", descriptor = "()Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/List<Lsun/util/locale/provider/LocaleProviderAdapter$Type;>;")]
        pub fn getAdapterPreference() -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getAdapterPreference:()Ljava/util/List;")
        }

        #[java_method(name = "getAdapter", descriptor = "(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;")]
        pub fn getAdapter(providerClass: Object, locale: Locale) -> Result<LocaleProviderAdapter> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getAdapter:(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;")
        }

        #[java_method(name = "findAdapter", descriptor = "(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;")]
        pub fn findAdapter(providerClass: Object, locale: Locale) -> Result<LocaleProviderAdapter> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.findAdapter:(Ljava/lang/Class;Ljava/util/Locale;)Lsun/util/locale/provider/LocaleProviderAdapter;")
        }

        #[java_method(name = "isSupportedProviderLocale", descriptor = "(Ljava/util/Locale;Ljava/util/Set;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Locale;Ljava/util/Set<Ljava/lang/String;>;)Z")]
        pub fn isSupportedProviderLocale(&self, locale: Locale, langtags: Object) -> Result<bool> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.isSupportedProviderLocale:(Ljava/util/Locale;Ljava/util/Set;)Z")
        }

        #[java_method(name = "toLocaleArray", descriptor = "(Ljava/util/Set;)[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Set<Ljava/lang/String;>;)[Ljava/util/Locale;")]
        pub fn toLocaleArray(tags: Object) -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.toLocaleArray:(Ljava/util/Set;)[Ljava/util/Locale;")
        }

        #[java_method(name = "getAdapterType", descriptor = "()Lsun/util/locale/provider/LocaleProviderAdapter$Type;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getAdapterType(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getAdapterType:()Lsun/util/locale/provider/LocaleProviderAdapter$Type;")
        }

        #[java_method(name = "getLocaleServiceProvider", descriptor = "(Ljava/lang/Class;)Ljava/util/spi/LocaleServiceProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false, generic_signature = "<P:Ljava/util/spi/LocaleServiceProvider;>(Ljava/lang/Class<TP;>;)TP;")]
        pub fn getLocaleServiceProvider(&self, arg0: Object) -> Result<LocaleServiceProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getLocaleServiceProvider:(Ljava/lang/Class;)Ljava/util/spi/LocaleServiceProvider;")
        }

        #[java_method(name = "getBreakIteratorProvider", descriptor = "()Ljava/text/spi/BreakIteratorProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getBreakIteratorProvider(&self) -> Result<BreakIteratorProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getBreakIteratorProvider:()Ljava/text/spi/BreakIteratorProvider;")
        }

        #[java_method(name = "getCollatorProvider", descriptor = "()Ljava/text/spi/CollatorProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCollatorProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getCollatorProvider:()Ljava/text/spi/CollatorProvider;")
        }

        #[java_method(name = "getDateFormatProvider", descriptor = "()Ljava/text/spi/DateFormatProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDateFormatProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getDateFormatProvider:()Ljava/text/spi/DateFormatProvider;")
        }

        #[java_method(name = "getDateFormatSymbolsProvider", descriptor = "()Ljava/text/spi/DateFormatSymbolsProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDateFormatSymbolsProvider(&self) -> Result<DateFormatSymbolsProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getDateFormatSymbolsProvider:()Ljava/text/spi/DateFormatSymbolsProvider;")
        }

        #[java_method(name = "getDecimalFormatSymbolsProvider", descriptor = "()Ljava/text/spi/DecimalFormatSymbolsProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getDecimalFormatSymbolsProvider(&self) -> Result<DecimalFormatSymbolsProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getDecimalFormatSymbolsProvider:()Ljava/text/spi/DecimalFormatSymbolsProvider;")
        }

        #[java_method(name = "getNumberFormatProvider", descriptor = "()Ljava/text/spi/NumberFormatProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getNumberFormatProvider(&self) -> Result<NumberFormatProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getNumberFormatProvider:()Ljava/text/spi/NumberFormatProvider;")
        }

        #[java_method(name = "getCurrencyNameProvider", descriptor = "()Ljava/util/spi/CurrencyNameProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCurrencyNameProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getCurrencyNameProvider:()Ljava/util/spi/CurrencyNameProvider;")
        }

        #[java_method(name = "getLocaleNameProvider", descriptor = "()Ljava/util/spi/LocaleNameProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getLocaleNameProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getLocaleNameProvider:()Ljava/util/spi/LocaleNameProvider;")
        }

        #[java_method(name = "getTimeZoneNameProvider", descriptor = "()Ljava/util/spi/TimeZoneNameProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getTimeZoneNameProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getTimeZoneNameProvider:()Ljava/util/spi/TimeZoneNameProvider;")
        }

        #[java_method(name = "getCalendarDataProvider", descriptor = "()Ljava/util/spi/CalendarDataProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarDataProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getCalendarDataProvider:()Ljava/util/spi/CalendarDataProvider;")
        }

        #[java_method(name = "getCalendarNameProvider", descriptor = "()Ljava/util/spi/CalendarNameProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarNameProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getCalendarNameProvider:()Ljava/util/spi/CalendarNameProvider;")
        }

        #[java_method(name = "getCalendarProvider", descriptor = "()Lsun/util/spi/CalendarProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getCalendarProvider(&self) -> Result<CalendarProvider> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getCalendarProvider:()Lsun/util/spi/CalendarProvider;")
        }

        #[java_method(name = "getJavaTimeDateTimePatternProvider", descriptor = "()Lsun/text/spi/JavaTimeDateTimePatternProvider;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getJavaTimeDateTimePatternProvider(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getJavaTimeDateTimePatternProvider:()Lsun/text/spi/JavaTimeDateTimePatternProvider;")
        }

        #[java_method(name = "getLocaleResources", descriptor = "(Ljava/util/Locale;)Lsun/util/locale/provider/LocaleResources;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getLocaleResources(&self, arg0: Locale) -> Result<LocaleResources> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getLocaleResources:(Ljava/util/Locale;)Lsun/util/locale/provider/LocaleResources;")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn getAvailableLocales(&self) -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: sun/util/locale/provider/LocaleProviderAdapter.getAvailableLocales:()[Ljava/util/Locale;")
        }
    }
}
