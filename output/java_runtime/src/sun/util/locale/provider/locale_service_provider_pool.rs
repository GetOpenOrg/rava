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
    #[binary_name       = "sun/util/locale/provider/LocaleServiceProviderPool"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleServiceProviderPool.java"]
    #[inner_classes     = "sun/util/locale/provider/LocaleServiceProviderPool$AllAvailableLocales:sun/util/locale/provider/LocaleServiceProviderPool:AllAvailableLocales:10;sun/util/locale/provider/LocaleProviderAdapter$Type:sun/util/locale/provider/LocaleProviderAdapter:Type:16409;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545;java/lang/System$Logger:java/lang/System:Logger:1545;java/lang/System$Logger$Level:java/lang/System$Logger:Level:16409;java/util/ResourceBundle$Control:java/util/ResourceBundle:Control:9;java/util/Locale$Builder:java/util/Locale:Builder:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/provider/LocaleServiceProviderPool"]

    pub struct LocaleServiceProviderPool {
        #[cfg_attr(any(), java_field(name = "providersCache", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/util/Locale;Ljava/util/List<Ljava/util/spi/LocaleServiceProvider;>;>;"))]
        pub providersCache: Object,
        #[cfg_attr(any(), java_field(name = "availableLocales", descriptor = "Ljava/util/Set;", access = "private", modifiers = "", is_static = false, generic_signature = "Ljava/util/Set<Ljava/util/Locale;>;"))]
        pub availableLocales: Object,
        #[cfg_attr(any(), java_field(name = "providerClass", descriptor = "Ljava/lang/Class;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;"))]
        pub providerClass: Class<LocaleServiceProvider>,
    }

    impl LocaleServiceProviderPool {
        #[cfg_attr(any(), java_field(name = "poolOfPools", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;Lsun/util/locale/provider/LocaleServiceProviderPool;>;"))]
        // static field: poolOfPools:Ljava/util/concurrent/ConcurrentMap;
        pub fn poolOfPools() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.poolOfPools:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "spiClasses", descriptor = "[Ljava/lang/Class;", access = "package", modifiers = "static final", is_static = true, generic_signature = "[Ljava/lang/Class<Ljava/util/spi/LocaleServiceProvider;>;"))]
        // static field: spiClasses:[Ljava/lang/Class;
        pub fn spiClasses() -> Rc<RefCell<Vec<Class<LocaleServiceProvider>>>> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.spiClasses:[Ljava/lang/Class;")
        }

        #[cfg_attr(any(), java_field(name = "NULL_LIST", descriptor = "Ljava/util/List;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/List<Ljava/util/spi/LocaleServiceProvider;>;"))]
        // static field: NULL_LIST:Ljava/util/List;
        pub fn NULL_LIST() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.NULL_LIST:Ljava/util/List;")
        }

        #[java_method(name = "getPool", descriptor = "(Ljava/lang/Class;)Lsun/util/locale/provider/LocaleServiceProviderPool;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;)Lsun/util/locale/provider/LocaleServiceProviderPool;")]
        pub fn getPool(providerClass: Object) -> Result<LocaleServiceProviderPool> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getPool:(Ljava/lang/Class;)Lsun/util/locale/provider/LocaleServiceProviderPool;")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Class;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/Class<+Ljava/util/spi/LocaleServiceProvider;>;)V")]
        pub fn new(c: Object) -> Result<Self> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.<init>:(Ljava/lang/Class;)V")
        }

        #[java_method(name = "streamAllAvailableLocales", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/util/Locale;>;")]
        pub fn streamAllAvailableLocales() -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.streamAllAvailableLocales:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "getAllAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAllAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getAllAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales(&self) -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "getAvailableLocaleSet", descriptor = "()Ljava/util/Set;", access = "private", modifiers = "synchronized", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Locale;>;")]
        pub fn getAvailableLocaleSet(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getAvailableLocaleSet:()Ljava/util/Set;")
        }

        #[java_method(name = "getLocalizedObject", descriptor = "(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;[Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P:Ljava/util/spi/LocaleServiceProvider;S:Ljava/lang/Object;>(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter<TP;TS;>;Ljava/util/Locale;[Ljava/lang/Object;)TS;")]
        pub fn getLocalizedObject_locale_locale_arr_obj(&self, getter: Object, locale: Locale, params: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLocalizedObject:(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;[Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getLocalizedObject", descriptor = "(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P:Ljava/util/spi/LocaleServiceProvider;S:Ljava/lang/Object;>(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter<TP;TS;>;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)TS;")]
        pub fn getLocalizedObject_locale_locale_str_arr_obj(&self, getter: Object, locale: Locale, key: String, params: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLocalizedObject:(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getLocalizedObject", descriptor = "(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;Ljava/lang/Boolean;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P:Ljava/util/spi/LocaleServiceProvider;S:Ljava/lang/Object;>(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter<TP;TS;>;Ljava/util/Locale;Ljava/lang/Boolean;Ljava/lang/String;[Ljava/lang/Object;)TS;")]
        pub fn getLocalizedObject_locale_locale_bool_str_arr_obj(&self, getter: Object, locale: Locale, isObjectProvider: bool, key: String, params: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLocalizedObject:(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;Ljava/lang/Boolean;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "getLocalizedObjectImpl", descriptor = "(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;ZLjava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;", access = "private", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "<P:Ljava/util/spi/LocaleServiceProvider;S:Ljava/lang/Object;>(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter<TP;TS;>;Ljava/util/Locale;ZLjava/lang/String;[Ljava/lang/Object;)TS;")]
        pub fn getLocalizedObjectImpl(&self, getter: Object, locale: Locale, isObjectProvider: bool, key: String, params: Rc<RefCell<Vec<Object>>>) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLocalizedObjectImpl:(Lsun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter;Ljava/util/Locale;ZLjava/lang/String;[Ljava/lang/Object;)Ljava/lang/Object;")
        }

        #[java_method(name = "findProviders", descriptor = "(Ljava/util/Locale;Z)Ljava/util/List;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Locale;Z)Ljava/util/List<Ljava/util/spi/LocaleServiceProvider;>;")]
        pub fn findProviders(&self, locale: Locale, isObjectProvider: bool) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.findProviders:(Ljava/util/Locale;Z)Ljava/util/List;")
        }

        #[java_method(name = "getLookupLocales", descriptor = "(Ljava/util/Locale;)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Locale;)Ljava/util/List<Ljava/util/Locale;>;")]
        pub fn getLookupLocales(locale: Locale) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLookupLocales:(Ljava/util/Locale;)Ljava/util/List;")
        }

        #[java_method(name = "getLookupLocale", descriptor = "(Ljava/util/Locale;)Ljava/util/Locale;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLookupLocale(locale: Locale) -> Result<Locale> {
            panic!("stub: sun/util/locale/provider/LocaleServiceProviderPool.getLookupLocale:(Ljava/util/Locale;)Ljava/util/Locale;")
        }
    }
}
