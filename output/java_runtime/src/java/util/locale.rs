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
    #[binary_name       = "java/util/Locale"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Locale.java"]
    #[inner_classes     = "java/util/Locale$Cache:java/util/Locale:Cache:10;java/util/Locale$LocaleKey:java/util/Locale:LocaleKey:26;java/util/Locale$Category:java/util/Locale:Category:16409;java/util/Locale$IsoCountryCode:java/util/Locale:IsoCountryCode:17417;java/util/Locale$LocaleNameGetter:java/util/Locale:LocaleNameGetter:10;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545;java/io/ObjectOutputStream$PutField:java/io/ObjectOutputStream:PutField:1033;java/io/ObjectInputStream$GetField:java/io/ObjectInputStream:GetField:1033;java/util/Locale$FilteringMode:java/util/Locale:FilteringMode:16409;java/util/Locale$LanguageRange:java/util/Locale:LanguageRange:25;java/util/Locale$Builder:java/util/Locale:Builder:25;java/util/Locale$IsoCountryCode$3:::16400;java/util/Locale$IsoCountryCode$2:::16400;java/util/Locale$IsoCountryCode$1:::16400;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/util/Locale"]
    #[has_to_string_method = true]
    #[has_hash_code_method = true]

    pub struct Locale {
        #[cfg_attr(any(), java_field(name = "baseLocale", descriptor = "Lsun/util/locale/BaseLocale;", access = "private", modifiers = "transient", is_static = false))]
        pub baseLocale: BaseLocale,
        #[cfg_attr(any(), java_field(name = "localeExtensions", descriptor = "Lsun/util/locale/LocaleExtensions;", access = "private", modifiers = "transient", is_static = false))]
        pub localeExtensions: LocaleExtensions,
        #[cfg_attr(any(), java_field(name = "hashCodeValue", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub hashCodeValue: i32,
        #[cfg_attr(any(), java_field(name = "languageTag", descriptor = "Ljava/lang/String;", access = "private", modifiers = "volatile transient", is_static = false))]
        pub languageTag: String,
    }

    impl Locale {
        #[cfg_attr(any(), java_field(name = "ENGLISH", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ENGLISH:Ljava/util/Locale;
        pub fn ENGLISH() -> Locale {
            panic!("stub: java/util/Locale.ENGLISH:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "FRENCH", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FRENCH:Ljava/util/Locale;
        pub fn FRENCH() -> Locale {
            panic!("stub: java/util/Locale.FRENCH:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "GERMAN", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GERMAN:Ljava/util/Locale;
        pub fn GERMAN() -> Locale {
            panic!("stub: java/util/Locale.GERMAN:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "ITALIAN", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ITALIAN:Ljava/util/Locale;
        pub fn ITALIAN() -> Locale {
            panic!("stub: java/util/Locale.ITALIAN:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "JAPANESE", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JAPANESE:Ljava/util/Locale;
        pub fn JAPANESE() -> Locale {
            panic!("stub: java/util/Locale.JAPANESE:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "KOREAN", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KOREAN:Ljava/util/Locale;
        pub fn KOREAN() -> Locale {
            panic!("stub: java/util/Locale.KOREAN:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "CHINESE", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHINESE:Ljava/util/Locale;
        pub fn CHINESE() -> Locale {
            panic!("stub: java/util/Locale.CHINESE:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "SIMPLIFIED_CHINESE", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: SIMPLIFIED_CHINESE:Ljava/util/Locale;
        pub fn SIMPLIFIED_CHINESE() -> Locale {
            panic!("stub: java/util/Locale.SIMPLIFIED_CHINESE:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "TRADITIONAL_CHINESE", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TRADITIONAL_CHINESE:Ljava/util/Locale;
        pub fn TRADITIONAL_CHINESE() -> Locale {
            panic!("stub: java/util/Locale.TRADITIONAL_CHINESE:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "FRANCE", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: FRANCE:Ljava/util/Locale;
        pub fn FRANCE() -> Locale {
            panic!("stub: java/util/Locale.FRANCE:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "GERMANY", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: GERMANY:Ljava/util/Locale;
        pub fn GERMANY() -> Locale {
            panic!("stub: java/util/Locale.GERMANY:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "ITALY", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ITALY:Ljava/util/Locale;
        pub fn ITALY() -> Locale {
            panic!("stub: java/util/Locale.ITALY:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "JAPAN", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: JAPAN:Ljava/util/Locale;
        pub fn JAPAN() -> Locale {
            panic!("stub: java/util/Locale.JAPAN:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "KOREA", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: KOREA:Ljava/util/Locale;
        pub fn KOREA() -> Locale {
            panic!("stub: java/util/Locale.KOREA:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "UK", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: UK:Ljava/util/Locale;
        pub fn UK() -> Locale {
            panic!("stub: java/util/Locale.UK:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "US", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: US:Ljava/util/Locale;
        pub fn US() -> Locale {
            panic!("stub: java/util/Locale.US:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "CANADA", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CANADA:Ljava/util/Locale;
        pub fn CANADA() -> Locale {
            panic!("stub: java/util/Locale.CANADA:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "CANADA_FRENCH", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CANADA_FRENCH:Ljava/util/Locale;
        pub fn CANADA_FRENCH() -> Locale {
            panic!("stub: java/util/Locale.CANADA_FRENCH:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "ROOT", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: ROOT:Ljava/util/Locale;
        pub fn ROOT() -> Locale {
            panic!("stub: java/util/Locale.ROOT:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "CONSTANT_LOCALES", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/Map<Lsun/util/locale/BaseLocale;Ljava/util/Locale;>;"))]
        // static field: CONSTANT_LOCALES:Ljava/util/Map;
        pub fn CONSTANT_LOCALES() -> Object {
            panic!("stub: java/util/Locale.CONSTANT_LOCALES:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "CHINA", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: CHINA:Ljava/util/Locale;
        pub fn CHINA() -> Locale {
            panic!("stub: java/util/Locale.CHINA:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "PRC", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: PRC:Ljava/util/Locale;
        pub fn PRC() -> Locale {
            panic!("stub: java/util/Locale.PRC:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "TAIWAN", descriptor = "Ljava/util/Locale;", access = "public", modifiers = "static final", is_static = true))]
        // static field: TAIWAN:Ljava/util/Locale;
        pub fn TAIWAN() -> Locale {
            panic!("stub: java/util/Locale.TAIWAN:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "PRIVATE_USE_EXTENSION", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "120"))]
        // static field: PRIVATE_USE_EXTENSION:C
        pub fn PRIVATE_USE_EXTENSION() -> u16 {
            120
        }

        #[cfg_attr(any(), java_field(name = "UNICODE_LOCALE_EXTENSION", descriptor = "C", access = "public", modifiers = "static final", is_static = true, constant_value = "117"))]
        // static field: UNICODE_LOCALE_EXTENSION:C
        pub fn UNICODE_LOCALE_EXTENSION() -> u16 {
            117
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "9149081749638150636"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            9149081749638150636i64
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_LANGUAGE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: DISPLAY_LANGUAGE:I
        pub fn DISPLAY_LANGUAGE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_COUNTRY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: DISPLAY_COUNTRY:I
        pub fn DISPLAY_COUNTRY() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_VARIANT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: DISPLAY_VARIANT:I
        pub fn DISPLAY_VARIANT() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_SCRIPT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: DISPLAY_SCRIPT:I
        pub fn DISPLAY_SCRIPT() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_UEXT_KEY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: DISPLAY_UEXT_KEY:I
        pub fn DISPLAY_UEXT_KEY() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "DISPLAY_UEXT_TYPE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: DISPLAY_UEXT_TYPE:I
        pub fn DISPLAY_UEXT_TYPE() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "defaultLocale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: defaultLocale:Ljava/util/Locale;
        pub fn defaultLocale() -> Locale {
            panic!("stub: java/util/Locale.defaultLocale:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "defaultDisplayLocale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: defaultDisplayLocale:Ljava/util/Locale;
        pub fn defaultDisplayLocale() -> Locale {
            panic!("stub: java/util/Locale.defaultDisplayLocale:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "defaultFormatLocale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: defaultFormatLocale:Ljava/util/Locale;
        pub fn defaultFormatLocale() -> Locale {
            panic!("stub: java/util/Locale.defaultFormatLocale:Ljava/util/Locale;")
        }

        #[cfg_attr(any(), java_field(name = "serialPersistentFields", descriptor = "[Ljava/io/ObjectStreamField;", access = "private", modifiers = "static final", is_static = true))]
        // static field: serialPersistentFields:[Ljava/io/ObjectStreamField;
        pub fn serialPersistentFields() -> Rc<RefCell<Vec<Object>>> {
            panic!("stub: java/util/Locale.serialPersistentFields:[Ljava/io/ObjectStreamField;")
        }

        #[cfg_attr(any(), java_field(name = "isoLanguages", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: isoLanguages:[Ljava/lang/String;
        pub fn isoLanguages() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/util/Locale.isoLanguages:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "isoCountries", descriptor = "[Ljava/lang/String;", access = "private", modifiers = "static volatile", is_static = true))]
        // static field: isoCountries:[Ljava/lang/String;
        pub fn isoCountries() -> Rc<RefCell<Vec<String>>> {
            panic!("stub: java/util/Locale.isoCountries:[Ljava/lang/String;")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "createConstant", descriptor = "(B)Ljava/util/Locale;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn createConstant(baseType: i8) -> Result<Locale> {
            panic!("stub: java/util/Locale.createConstant:(B)Ljava/util/Locale;")
        }

        #[java_method(name = "<init>", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_baselo_locale(baseLocale: BaseLocale, extensions: LocaleExtensions) -> Result<Self> {
            panic!("stub: java/util/Locale.<init>:(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_str_str_str(language: String, country: String, variant: String) -> Result<Self> {
            panic!("stub: java/util/Locale.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_str_str(language: String, country: String) -> Result<Self> {
            panic!("stub: java/util/Locale.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, is_deprecated = true)]
        pub fn new_str(language: String) -> Result<Self> {
            panic!("stub: java/util/Locale.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_str_str_str(language: String, country: String, variant: String) -> Result<Locale> {
            panic!("stub: java/util/Locale.of:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_str_str(language: String, country: String) -> Result<Locale> {
            panic!("stub: java/util/Locale.of:(Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;")
        }

        #[java_method(name = "of", descriptor = "(Ljava/lang/String;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn of_str(language: String) -> Result<Locale> {
            panic!("stub: java/util/Locale.of:(Ljava/lang/String;)Ljava/util/Locale;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance_str_str_str(language: String, country: String, variant: String) -> Result<Locale> {
            panic!("stub: java/util/Locale.getInstance:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/Locale;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;
        pub fn getInstance_str_str_str_str_locale(mut language: String, mut script: String, mut country: String, mut variant: String, mut extensions: LocaleExtensions) -> Result<Locale> {
            if _is_jnull(&variant) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if _is_jnull(&extensions) {
                let _t0: LocaleExtensions = Locale::getCompatibilityExtensions(Clone::clone(&language), Clone::clone(&script), Clone::clone(&country), Clone::clone(&variant))?;
                extensions = _t0;
            }
            let _t0: String = Locale::convertOldISOCodes(Clone::clone(&language))?;
            let _t1: BaseLocale = BaseLocale::getInstance(Clone::clone(&_t0), Clone::clone(&script), Clone::clone(&country), Clone::clone(&variant))?;
            let mut baseloc: BaseLocale = _t1;
            let _t2: Locale = Locale::getInstance_baselo_locale(Clone::clone(&baseloc), Clone::clone(&extensions))?;
            Ok(_t2)
        }

        #[java_method(name = "getInstance", descriptor = "(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Lsun/util/locale/BaseLocale;Lsun/util/locale/LocaleExtensions;)Ljava/util/Locale;
        pub fn getInstance_baselo_locale(mut baseloc: BaseLocale, mut extensions: LocaleExtensions) -> Result<Locale> {
            let _vdispatch0: Object = if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<ImmutableCollections_MapN<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<ImmutableCollections_Map1<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<Properties>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<ImmutableCollections_AbstractImmutableMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<TreeMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<LinkedHashMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<Hashtable<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<AbstractMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<HashMap<Object, Object>>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(_d) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(baseloc.clone()))? } else if let Some(__f) = Locale::CONSTANT_LOCALES().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(baseloc.clone()))? } else { Default::default() };
            let mut locale = (_vdispatch0).downcast::<Locale>();
            if !_is_jnull(&locale) {
                return Ok(locale);
            }
            let _t1 = Locale_Cache::LOCALECACHE().__super().get(Object::from_any(baseloc.clone()))?;
            return Ok((_t1).downcast::<Locale>());
            let mut locale = Locale_LocaleKey::new(Clone::clone(&baseloc), Clone::clone(&extensions))?;
            let _t2 = Locale_Cache::LOCALECACHE().__super().get(Object::from_any(locale.clone()))?;
            Ok((_t2).downcast::<Locale>())
        }

        #[java_method(name = "getDefault", descriptor = "()Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getDefault()Ljava/util/Locale;
        pub fn getDefault() -> Result<Locale> {
            Ok(Locale::defaultLocale())
        }

        #[java_method(name = "getDefault", descriptor = "(Ljava/util/Locale$Category;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;
        pub fn getDefault_locale(mut category: Locale_Category) -> Result<Locale> {
            let _t0: Object = Objects::requireNonNull_obj(Object::from_any(category.clone()))?;
            let mut loc: Locale = Locale::defaultDisplayLocale();
            if _is_jnull(&loc) {
                let _t1: Locale = Locale::getDisplayLocale()?;
                loc = _t1;
            }
            return Ok(loc);
            if Object::from_any(category.clone()) != Object::from_any(Locale_Category::FORMAT().clone()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            loc = Locale::defaultFormatLocale();
            if _is_jnull(&loc) {
                let _t1: Locale = Locale::getFormatLocale()?;
                loc = _t1;
            }
            Ok(loc)
        }

        #[java_method(name = "getDisplayLocale", descriptor = "()Ljava/util/Locale;", access = "private", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayLocale() -> Result<Locale> {
            let mut loc: Locale = Locale::defaultDisplayLocale();
            if _is_jnull(&loc) {
                let _t0: Locale = Locale::initDefault_locale(Clone::clone(&Locale_Category::DISPLAY()))?;
                Locale::set_defaultDisplayLocale(_t0);
                loc = _t0;
            }
            Ok(loc)
        }

        #[java_method(name = "getFormatLocale", descriptor = "()Ljava/util/Locale;", access = "private", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getFormatLocale() -> Result<Locale> {
            let mut loc: Locale = Locale::defaultFormatLocale();
            if _is_jnull(&loc) {
                let _t0: Locale = Locale::initDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
                Locale::set_defaultFormatLocale(_t0);
                loc = _t0;
            }
            Ok(loc)
        }

        #[java_method(name = "initDefault", descriptor = "()Ljava/util/Locale;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initDefault() -> Result<Locale> {
            panic!("stub: java/util/Locale.initDefault:()Ljava/util/Locale;")
        }

        #[java_method(name = "initDefault", descriptor = "(Ljava/util/Locale$Category;)Ljava/util/Locale;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: initDefault(Ljava/util/Locale$Category;)Ljava/util/Locale;
        pub fn initDefault_locale(mut category: Locale_Category) -> Result<Locale> {
            let mut locale: Locale = Locale::defaultLocale();
            let _t0: Optional<Object> = Locale::getDefaultExtensions(Clone::clone(&(if Object::from_any(category.clone()) == Object::from_any(Locale_Category::DISPLAY().clone()) { StaticProperty::USER_EXTENSIONS_DISPLAY() } else { StaticProperty::USER_EXTENSIONS_FORMAT() })))?;
            let _t1 = locale.getLocaleExtensions()?;
            let _t2 = _t0.orElse(Object::from_any(_t1.clone()))?;
            let _t3: Locale = Locale::getInstance_str_str_str_str_locale(Clone::clone(&(if Object::from_any(category.clone()) == Object::from_any(Locale_Category::DISPLAY().clone()) { StaticProperty::USER_LANGUAGE_DISPLAY() } else { StaticProperty::USER_LANGUAGE_FORMAT() })), Clone::clone(&(if Object::from_any(category.clone()) == Object::from_any(Locale_Category::DISPLAY().clone()) { StaticProperty::USER_SCRIPT_DISPLAY() } else { StaticProperty::USER_SCRIPT_FORMAT() })), Clone::clone(&(if Object::from_any(category.clone()) == Object::from_any(Locale_Category::DISPLAY().clone()) { StaticProperty::USER_COUNTRY_DISPLAY() } else { StaticProperty::USER_COUNTRY_FORMAT() })), Clone::clone(&(if Object::from_any(category.clone()) == Object::from_any(Locale_Category::DISPLAY().clone()) { StaticProperty::USER_VARIANT_DISPLAY() } else { StaticProperty::USER_VARIANT_FORMAT() })), Clone::clone(&(_t2).downcast::<LocaleExtensions>()))?;
            Ok(_t3)
        }

        #[java_method(name = "getDefaultExtensions", descriptor = "(Ljava/lang/String;)Ljava/util/Optional;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/Optional<Lsun/util/locale/LocaleExtensions;>;")]
        pub fn getDefaultExtensions(mut extensionsProp: String) -> Result<Optional<Object>> {
            let _t0: bool = LocaleUtils::isEmpty_str(Clone::clone(&extensionsProp))?;
            if _t0 {
                let _t1: Optional<Object> = Optional::<Object>::empty()?;
                return Ok(_t1);
            }
            let mut exts: Object = Object::default();
            let _t1 = InternalLocaleBuilder::new()?.setExtensions_str(Clone::clone(&extensionsProp))?;
            let _t2 = _t1.getLocaleExtensions()?;
            let mut exts: LocaleExtensions = _t2;
            let _t3: Optional<Object> = Optional::<Object>::ofNullable(Object::from_any(exts.clone()))?;
            Ok(_t3)
        }

        #[java_method(name = "setDefault", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDefault_locale(newLocale: Locale) -> Result<()> {
            panic!("stub: java/util/Locale.setDefault:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "setDefault", descriptor = "(Ljava/util/Locale$Category;Ljava/util/Locale;)V", access = "public", modifiers = "static synchronized", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDefault_locale_locale(category: Locale_Category, newLocale: Locale) -> Result<()> {
            panic!("stub: java/util/Locale.setDefault:(Ljava/util/Locale$Category;Ljava/util/Locale;)V")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/util/Locale.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "availableLocales", descriptor = "()Ljava/util/stream/Stream;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/stream/Stream<Ljava/util/Locale;>;")]
        pub fn availableLocales() -> Result<Object> {
            panic!("stub: java/util/Locale.availableLocales:()Ljava/util/stream/Stream;")
        }

        #[java_method(name = "getISOCountries", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getISOCountries() -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/Locale.getISOCountries:()[Ljava/lang/String;")
        }

        #[java_method(name = "getISOCountries", descriptor = "(Ljava/util/Locale$IsoCountryCode;)Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Locale$IsoCountryCode;)Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getISOCountries_locale(type_: Object) -> Result<Object> {
            panic!("stub: java/util/Locale.getISOCountries:(Ljava/util/Locale$IsoCountryCode;)Ljava/util/Set;")
        }

        #[java_method(name = "getISOLanguages", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getISOLanguages() -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/Locale.getISOLanguages:()[Ljava/lang/String;")
        }

        #[java_method(name = "getISO2Table", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getISO2Table(table: String) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/Locale.getISO2Table:(Ljava/lang/String;)[Ljava/lang/String;")
        }

        #[java_method(name = "getLanguage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLanguage(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__get_baseLocale().getLanguage()?;
            Ok(_t0)
        }

        #[java_method(name = "getScript", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScript(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getScript:()Ljava/lang/String;")
        }

        #[java_method(name = "getCountry", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCountry(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__get_baseLocale().getRegion()?;
            Ok(_t0)
        }

        #[java_method(name = "getVariant", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getVariant(&self) -> Result<String> {
            let this = self;
            let _t0 = this.__get_baseLocale().getVariant()?;
            Ok(_t0)
        }

        #[java_method(name = "hasExtensions", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hasExtensions(&self) -> Result<bool> {
            let this = self;
            Ok(!_is_jnull(&this.__get_localeExtensions()))
        }

        #[java_method(name = "stripExtensions", descriptor = "()Ljava/util/Locale;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn stripExtensions(&self) -> Result<Locale> {
            panic!("stub: java/util/Locale.stripExtensions:()Ljava/util/Locale;")
        }

        #[java_method(name = "getExtension", descriptor = "(C)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExtension(&self, key: u16) -> Result<String> {
            panic!("stub: java/util/Locale.getExtension:(C)Ljava/lang/String;")
        }

        #[java_method(name = "getExtensionKeys", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/Character;>;")]
        pub fn getExtensionKeys(&self) -> Result<Object> {
            panic!("stub: java/util/Locale.getExtensionKeys:()Ljava/util/Set;")
        }

        #[java_method(name = "getUnicodeLocaleAttributes", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getUnicodeLocaleAttributes(&self) -> Result<Object> {
            panic!("stub: java/util/Locale.getUnicodeLocaleAttributes:()Ljava/util/Set;")
        }

        #[java_method(name = "getUnicodeLocaleType", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getUnicodeLocaleType(&self, mut key: String) -> Result<String> {
            let this = self;
            let _t0: bool = Locale::isUnicodeExtensionKey(Clone::clone(&key))?;
            if !(_t0) {
                let _t1 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Ill-formed Unicode locale key: ")))?;
                let _t2 = _t1.append_str(Clone::clone(&key))?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1 = this.hasExtensions()?;
            let mut _merged3: String;
            if _t1 {
                let _t2 = this.__get_localeExtensions().getUnicodeLocaleType(Clone::clone(&key))?;
                _merged3 = _t2;
            } else {
                _merged3 = Default::default();
            }
            Ok(_merged3)
        }

        #[java_method(name = "getUnicodeLocaleKeys", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getUnicodeLocaleKeys(&self) -> Result<Object> {
            panic!("stub: java/util/Locale.getUnicodeLocaleKeys:()Ljava/util/Set;")
        }

        #[java_method(name = "getBaseLocale", descriptor = "()Lsun/util/locale/BaseLocale;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBaseLocale(&self) -> Result<BaseLocale> {
            panic!("stub: java/util/Locale.getBaseLocale:()Lsun/util/locale/BaseLocale;")
        }

        #[java_method(name = "getLocaleExtensions", descriptor = "()Lsun/util/locale/LocaleExtensions;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocaleExtensions(&self) -> Result<LocaleExtensions> {
            let this = self;
            Ok(this.__get_localeExtensions())
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "toLanguageTag", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLanguageTag(&self) -> Result<String> {
            panic!("stub: java/util/Locale.toLanguageTag:()Ljava/lang/String;")
        }

        #[java_method(name = "caseFoldLanguageTag", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn caseFoldLanguageTag(languageTag: String) -> Result<String> {
            panic!("stub: java/util/Locale.caseFoldLanguageTag:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "forLanguageTag", descriptor = "(Ljava/lang/String;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn forLanguageTag(languageTag: String) -> Result<Locale> {
            panic!("stub: java/util/Locale.forLanguageTag:(Ljava/lang/String;)Ljava/util/Locale;")
        }

        #[java_method(name = "getISO3Language", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/util/MissingResourceException")]
        pub fn getISO3Language(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getISO3Language:()Ljava/lang/String;")
        }

        #[java_method(name = "getISO3Country", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/util/MissingResourceException")]
        pub fn getISO3Country(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getISO3Country:()Ljava/lang/String;")
        }

        #[java_method(name = "getISO3Code", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getISO3Code(iso2Code: String, table: String) -> Result<String> {
            panic!("stub: java/util/Locale.getISO3Code:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayLanguage", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayLanguage(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayLanguage:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayLanguage", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayLanguage_locale(&self, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayLanguage:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayScript", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayScript(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayScript:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayScript", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayScript_locale(&self, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayScript:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayCountry", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayCountry(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayCountry:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayCountry", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayCountry_locale(&self, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayCountry:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayString", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;I)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayString(&self, code: String, cat: String, inLocale: Locale, type_: i32) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayString:(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;I)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayVariant", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayVariant(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayVariant:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayVariant", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayVariant_locale(&self, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayVariant:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayName:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName_locale(&self, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayName:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/util/Locale.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, mut obj: Object) -> Result<bool> {
            let this = self;
            if Object::from_any(this.clone()) == obj {
                return Ok((1i32 != 0i32));
            }
            if !((obj.is_instance_of("java/util/Locale"))) {
                return Ok((0i32 != 0i32));
            }
            let mut otherBase = (obj).downcast::<Locale>().__get_baseLocale();
            let _t0 = this.__get_baseLocale().equals(Object::from_any(otherBase.clone()))?;
            if !(_t0) {
                return Ok((0i32 != 0i32));
            }
            return Ok(_is_jnull(&(obj).downcast::<Locale>().__get_localeExtensions()));
            let _t1 = this.__get_localeExtensions().equals(Object::from_any((obj).downcast::<Locale>().__get_localeExtensions().clone()))?;
            Ok(_t1)
        }

        #[java_method(name = "getDisplayVariantArray", descriptor = "(Ljava/util/Locale;)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayVariantArray(&self, inLocale: Locale) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/util/Locale.getDisplayVariantArray:(Ljava/util/Locale;)[Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayKeyTypeExtensionString", descriptor = "(Ljava/lang/String;Lsun/util/locale/provider/LocaleResources;Ljava/util/Locale;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayKeyTypeExtensionString(&self, key: String, lr: LocaleResources, inLocale: Locale) -> Result<String> {
            panic!("stub: java/util/Locale.getDisplayKeyTypeExtensionString:(Ljava/lang/String;Lsun/util/locale/provider/LocaleResources;Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "formatList", descriptor = "([Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatList(stringList: Rc<RefCell<Vec<String>>>, pattern: String) -> Result<String> {
            panic!("stub: java/util/Locale.formatList:([Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "isUnicodeExtensionKey", descriptor = "(Ljava/lang/String;)Z", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isUnicodeExtensionKey(mut s: String) -> Result<bool> {
            let _t0 = s.length()?;
            let mut _merged2: bool;
            if _t0 == 2i32 {
                let _t1: bool = LocaleUtils::isAlphaNumericString(Clone::clone(&s))?;
                _merged2 = !(!(_t1));
            } else {
                _merged2 = (0i32 != 0);
            }
            Ok(_merged2)
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, out: Object) -> Result<()> {
            panic!("stub: java/util/Locale.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, in_: Object) -> Result<()> {
            panic!("stub: java/util/Locale.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "readResolve", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/ObjectStreamException")]
        pub fn readResolve(&self) -> Result<Object> {
            panic!("stub: java/util/Locale.readResolve:()Ljava/lang/Object;")
        }

        #[java_method(name = "convertOldISOCodes", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn convertOldISOCodes(mut language: String) -> Result<String> {
            let _t0: String = LocaleUtils::toLowerString(Clone::clone(&language))?;
            let _t1 = _t0.intern()?;
            let _t2: String = BaseLocale::convertOldISOCodes(Clone::clone(&_t1))?;
            Ok(_t2)
        }

        #[java_method(name = "getCompatibilityExtensions", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Lsun/util/locale/LocaleExtensions;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCompatibilityExtensions(mut language: String, mut script: String, mut country: String, mut variant: String) -> Result<LocaleExtensions> {
            let mut extensions: Object = Object::default();
            let _t0: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&language), Clone::clone(&String::from("ja")))?;
            if _t0 {
                let _t1 = script.isEmpty()?;
                if _t1 {
                    let _t2: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&country), Clone::clone(&String::from("jp")))?;
                    if _t2 {
                        let _t3 = String::from("JP").equals(Object::from_any(variant.clone()))?;
                        if _t3 {
                            let mut extensions: LocaleExtensions = LocaleExtensions::CALENDAR_JAPANESE();
                        } else {
                            let _t4: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&language), Clone::clone(&String::from("th")))?;
                            let _t5 = script.isEmpty()?;
                            let _t6: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&country), Clone::clone(&String::from("th")))?;
                            let _t7 = String::from("TH").equals(Object::from_any(variant.clone()))?;
                            if _t7 {
                                let mut extensions: LocaleExtensions = LocaleExtensions::NUMBER_THAI();
                            }
                        }
                    } else {
                        let _t3: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&language), Clone::clone(&String::from("th")))?;
                        let _t4 = script.isEmpty()?;
                        let _t5: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&country), Clone::clone(&String::from("th")))?;
                        let _t6 = String::from("TH").equals(Object::from_any(variant.clone()))?;
                        if _t6 {
                            let mut extensions: LocaleExtensions = LocaleExtensions::NUMBER_THAI();
                        }
                    }
                } else {
                    let _t2: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&language), Clone::clone(&String::from("th")))?;
                    let _t3 = script.isEmpty()?;
                    let _t4: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&country), Clone::clone(&String::from("th")))?;
                    let _t5 = String::from("TH").equals(Object::from_any(variant.clone()))?;
                    if _t5 {
                        let mut extensions: LocaleExtensions = LocaleExtensions::NUMBER_THAI();
                    }
                }
            } else {
                let _t1: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&language), Clone::clone(&String::from("th")))?;
                let _t2 = script.isEmpty()?;
                let _t3: bool = LocaleUtils::caseIgnoreMatch(Clone::clone(&country), Clone::clone(&String::from("th")))?;
                let _t4 = String::from("TH").equals(Object::from_any(variant.clone()))?;
                if _t4 {
                    let mut extensions: LocaleExtensions = LocaleExtensions::NUMBER_THAI();
                }
            }
            Ok(extensions)
        }

        #[java_method(name = "filter", descriptor = "(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/util/Locale;>;Ljava/util/Locale$FilteringMode;)Ljava/util/List<Ljava/util/Locale;>;")]
        pub fn filter_list_coll_locale(priorityList: Object, locales: Object, mode: Object) -> Result<Object> {
            panic!("stub: java/util/Locale.filter:(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;")
        }

        #[java_method(name = "filter", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/util/Locale;>;)Ljava/util/List<Ljava/util/Locale;>;")]
        pub fn filter_list_coll(priorityList: Object, locales: Object) -> Result<Object> {
            panic!("stub: java/util/Locale.filter:(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;")
        }

        #[java_method(name = "filterTags", descriptor = "(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/lang/String;>;Ljava/util/Locale$FilteringMode;)Ljava/util/List<Ljava/lang/String;>;")]
        pub fn filterTags_list_coll_locale(priorityList: Object, tags: Object, mode: Object) -> Result<Object> {
            panic!("stub: java/util/Locale.filterTags:(Ljava/util/List;Ljava/util/Collection;Ljava/util/Locale$FilteringMode;)Ljava/util/List;")
        }

        #[java_method(name = "filterTags", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/lang/String;>;)Ljava/util/List<Ljava/lang/String;>;")]
        pub fn filterTags_list_coll(priorityList: Object, tags: Object) -> Result<Object> {
            panic!("stub: java/util/Locale.filterTags:(Ljava/util/List;Ljava/util/Collection;)Ljava/util/List;")
        }

        #[java_method(name = "lookup", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/util/Locale;>;)Ljava/util/Locale;")]
        pub fn lookup(priorityList: Object, locales: Object) -> Result<Locale> {
            panic!("stub: java/util/Locale.lookup:(Ljava/util/List;Ljava/util/Collection;)Ljava/util/Locale;")
        }

        #[java_method(name = "lookupTag", descriptor = "(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/List<Ljava/util/Locale$LanguageRange;>;Ljava/util/Collection<Ljava/lang/String;>;)Ljava/lang/String;")]
        pub fn lookupTag(priorityList: Object, tags: Object) -> Result<String> {
            panic!("stub: java/util/Locale.lookupTag:(Ljava/util/List;Ljava/util/Collection;)Ljava/lang/String;")
        }
    }
}
