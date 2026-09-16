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
    #[binary_name       = "sun/util/locale/provider/LocaleResources"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "LocaleResources.java"]
    #[inner_classes     = "sun/util/locale/provider/LocaleProviderAdapter$Type:sun/util/locale/provider/LocaleProviderAdapter:Type:16409;sun/util/locale/provider/LocaleResources$ResourceReference:sun/util/locale/provider/LocaleResources:ResourceReference:10;java/text/NumberFormat$Style:java/text/NumberFormat:Style:16409;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;sun/util/locale/provider/LocaleResources"]

    pub struct LocaleResources {
        #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "final", is_static = false))]
        pub locale: Locale,
        #[cfg_attr(any(), java_field(name = "localeData", descriptor = "Lsun/util/resources/LocaleData;", access = "private", modifiers = "final", is_static = false))]
        pub localeData: Object,
        #[cfg_attr(any(), java_field(name = "type", descriptor = "Lsun/util/locale/provider/LocaleProviderAdapter$Type;", access = "private", modifiers = "final", is_static = false))]
        pub type_: Object,
        #[cfg_attr(any(), java_field(name = "cache", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/String;Lsun/util/locale/provider/LocaleResources$ResourceReference;>;"))]
        pub cache: Object,
        #[cfg_attr(any(), java_field(name = "referenceQueue", descriptor = "Ljava/lang/ref/ReferenceQueue;", access = "private", modifiers = "final", is_static = false, generic_signature = "Ljava/lang/ref/ReferenceQueue<Ljava/lang/Object;>;"))]
        pub referenceQueue: ReferenceQueue<Object>,
        #[cfg_attr(any(), java_field(name = "jPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub jPattern: String,
        #[cfg_attr(any(), java_field(name = "CPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub CPattern: String,
    }

    impl LocaleResources {
        #[cfg_attr(any(), java_field(name = "BREAK_ITERATOR_INFO", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "BII."))]
        // static field: BREAK_ITERATOR_INFO:Ljava/lang/String;
        pub fn BREAK_ITERATOR_INFO() -> String {
            String::from("BII.")
        }

        #[cfg_attr(any(), java_field(name = "CALENDAR_DATA", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "CALD."))]
        // static field: CALENDAR_DATA:Ljava/lang/String;
        pub fn CALENDAR_DATA() -> String {
            String::from("CALD.")
        }

        #[cfg_attr(any(), java_field(name = "COLLATION_DATA", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "COLD."))]
        // static field: COLLATION_DATA:Ljava/lang/String;
        pub fn COLLATION_DATA() -> String {
            String::from("COLD.")
        }

        #[cfg_attr(any(), java_field(name = "DECIMAL_FORMAT_SYMBOLS_DATA_CACHEKEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "DFSD"))]
        // static field: DECIMAL_FORMAT_SYMBOLS_DATA_CACHEKEY:Ljava/lang/String;
        pub fn DECIMAL_FORMAT_SYMBOLS_DATA_CACHEKEY() -> String {
            String::from("DFSD")
        }

        #[cfg_attr(any(), java_field(name = "CURRENCY_NAMES", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "CN."))]
        // static field: CURRENCY_NAMES:Ljava/lang/String;
        pub fn CURRENCY_NAMES() -> String {
            String::from("CN.")
        }

        #[cfg_attr(any(), java_field(name = "LOCALE_NAMES", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "LN."))]
        // static field: LOCALE_NAMES:Ljava/lang/String;
        pub fn LOCALE_NAMES() -> String {
            String::from("LN.")
        }

        #[cfg_attr(any(), java_field(name = "TIME_ZONE_NAMES", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "TZN."))]
        // static field: TIME_ZONE_NAMES:Ljava/lang/String;
        pub fn TIME_ZONE_NAMES() -> String {
            String::from("TZN.")
        }

        #[cfg_attr(any(), java_field(name = "ZONE_IDS_CACHEKEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "ZID"))]
        // static field: ZONE_IDS_CACHEKEY:Ljava/lang/String;
        pub fn ZONE_IDS_CACHEKEY() -> String {
            String::from("ZID")
        }

        #[cfg_attr(any(), java_field(name = "CALENDAR_NAMES", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "CALN."))]
        // static field: CALENDAR_NAMES:Ljava/lang/String;
        pub fn CALENDAR_NAMES() -> String {
            String::from("CALN.")
        }

        #[cfg_attr(any(), java_field(name = "NUMBER_PATTERNS_CACHEKEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "NP"))]
        // static field: NUMBER_PATTERNS_CACHEKEY:Ljava/lang/String;
        pub fn NUMBER_PATTERNS_CACHEKEY() -> String {
            String::from("NP")
        }

        #[cfg_attr(any(), java_field(name = "COMPACT_NUMBER_PATTERNS_CACHEKEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "CNP"))]
        // static field: COMPACT_NUMBER_PATTERNS_CACHEKEY:Ljava/lang/String;
        pub fn COMPACT_NUMBER_PATTERNS_CACHEKEY() -> String {
            String::from("CNP")
        }

        #[cfg_attr(any(), java_field(name = "DATE_TIME_PATTERN", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "DTP."))]
        // static field: DATE_TIME_PATTERN:Ljava/lang/String;
        pub fn DATE_TIME_PATTERN() -> String {
            String::from("DTP.")
        }

        #[cfg_attr(any(), java_field(name = "RULES_CACHEKEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "RULE"))]
        // static field: RULES_CACHEKEY:Ljava/lang/String;
        pub fn RULES_CACHEKEY() -> String {
            String::from("RULE")
        }

        #[cfg_attr(any(), java_field(name = "SKELETON_PATTERN", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "SP."))]
        // static field: SKELETON_PATTERN:Ljava/lang/String;
        pub fn SKELETON_PATTERN() -> String {
            String::from("SP.")
        }

        #[cfg_attr(any(), java_field(name = "SKELETON_INPUT_REGIONS_KEY", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "DateFormatItemInputRegions"))]
        // static field: SKELETON_INPUT_REGIONS_KEY:Ljava/lang/String;
        pub fn SKELETON_INPUT_REGIONS_KEY() -> String {
            String::from("DateFormatItemInputRegions")
        }

        #[cfg_attr(any(), java_field(name = "TZNB_EXCITY_PREFIX", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "timezone.excity."))]
        // static field: TZNB_EXCITY_PREFIX:Ljava/lang/String;
        pub fn TZNB_EXCITY_PREFIX() -> String {
            String::from("timezone.excity.")
        }

        #[cfg_attr(any(), java_field(name = "NULLOBJECT", descriptor = "Ljava/lang/Object;", access = "private", modifiers = "static final", is_static = true))]
        // static field: NULLOBJECT:Ljava/lang/Object;
        pub fn NULLOBJECT() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleResources.NULLOBJECT:Ljava/lang/Object;")
        }

        #[cfg_attr(any(), java_field(name = "VALID_SKELETON_PATTERN", descriptor = "Ljava/util/regex/Pattern;", access = "private", modifiers = "static final", is_static = true))]
        // static field: VALID_SKELETON_PATTERN:Ljava/util/regex/Pattern;
        pub fn VALID_SKELETON_PATTERN() -> Pattern {
            panic!("stub: sun/util/locale/provider/LocaleResources.VALID_SKELETON_PATTERN:Ljava/util/regex/Pattern;")
        }

        #[cfg_attr(any(), java_field(name = "inputSkeletons", descriptor = "Ljava/util/Map;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/Map<Ljava/lang/String;Ljava/util/Map<Ljava/lang/String;Ljava/lang/String;>;>;"))]
        // static field: inputSkeletons:Ljava/util/Map;
        pub fn inputSkeletons() -> Object {
            panic!("stub: sun/util/locale/provider/LocaleResources.inputSkeletons:Ljava/util/Map;")
        }

        #[cfg_attr(any(), java_field(name = "TRACE_ON", descriptor = "Z", access = "private", modifiers = "static final", is_static = true))]
        // static field: TRACE_ON:Z
        pub fn TRACE_ON() -> bool {
            panic!("stub: sun/util/locale/provider/LocaleResources.TRACE_ON:Z")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "(Lsun/util/locale/provider/ResourceBundleBasedAdapter;Ljava/util/Locale;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(adapter: Object, locale: Locale) -> Result<Self> {
            panic!("stub: sun/util/locale/provider/LocaleResources.<init>:(Lsun/util/locale/provider/ResourceBundleBasedAdapter;Ljava/util/Locale;)V")
        }

        #[java_method(name = "removeEmptyReferences", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn removeEmptyReferences(&self) -> Result<()> {
            panic!("stub: sun/util/locale/provider/LocaleResources.removeEmptyReferences:()V")
        }

        #[java_method(name = "getBreakIteratorInfo", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBreakIteratorInfo(&self, key: String) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getBreakIteratorInfo:(Ljava/lang/String;)Ljava/lang/Object;")
        }

        #[java_method(name = "getBreakIteratorResources", descriptor = "(Ljava/lang/String;)[B", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBreakIteratorResources(&self, key: String) -> Result<Rc<RefCell<Vec<i8>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getBreakIteratorResources:(Ljava/lang/String;)[B")
        }

        #[java_method(name = "getCalendarData", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarData(&self, key: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getCalendarData:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getCollationData", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCollationData(&self) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getCollationData:()Ljava/lang/String;")
        }

        #[java_method(name = "getDecimalFormatSymbolsData", descriptor = "()[Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecimalFormatSymbolsData(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getDecimalFormatSymbolsData:()[Ljava/lang/Object;")
        }

        #[java_method(name = "getNumberStrings", descriptor = "(Ljava/util/ResourceBundle;Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumberStrings(&self, rb: Object, type_: String) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getNumberStrings:(Ljava/util/ResourceBundle;Ljava/lang/String;)[Ljava/lang/String;")
        }

        #[java_method(name = "getCurrencyName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrencyName(&self, key: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getCurrencyName:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getLocaleName", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocaleName(&self, key: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getLocaleName:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getTimeZoneNames", descriptor = "(Ljava/lang/String;)Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getTimeZoneNames(&self, key: String) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getTimeZoneNames:(Ljava/lang/String;)Ljava/lang/Object;")
        }

        #[java_method(name = "getZoneIDs", descriptor = "()Ljava/util/Set;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/lang/String;>;")]
        pub fn getZoneIDs(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getZoneIDs:()Ljava/util/Set;")
        }

        #[java_method(name = "getZoneStrings", descriptor = "()[[Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneStrings(&self) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getZoneStrings:()[[Ljava/lang/String;")
        }

        #[java_method(name = "getCalendarNames", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCalendarNames(&self, key: String) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getCalendarNames:(Ljava/lang/String;)[Ljava/lang/String;")
        }

        #[java_method(name = "getJavaTimeNames", descriptor = "(Ljava/lang/String;)[Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getJavaTimeNames(&self, key: String) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getJavaTimeNames:(Ljava/lang/String;)[Ljava/lang/String;")
        }

        #[java_method(name = "getDateTimePattern", descriptor = "(IILjava/util/Calendar;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDateTimePattern_i_i_calend(&self, timeStyle: i32, dateStyle: i32, cal: Calendar) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getDateTimePattern:(IILjava/util/Calendar;)Ljava/lang/String;")
        }

        #[java_method(name = "getJavaTimeDateTimePattern", descriptor = "(IILjava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getJavaTimeDateTimePattern(&self, timeStyle: i32, dateStyle: i32, calType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getJavaTimeDateTimePattern:(IILjava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getDateTimePattern", descriptor = "(Ljava/lang/String;IILjava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDateTimePattern_str_i_i_str(&self, prefix: String, timeStyle: i32, dateStyle: i32, calType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getDateTimePattern:(Ljava/lang/String;IILjava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getNumberPatterns", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumberPatterns(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getNumberPatterns:()[Ljava/lang/String;")
        }

        #[java_method(name = "getCNPatterns", descriptor = "(Ljava/text/NumberFormat$Style;)[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCNPatterns(&self, formatStyle: Object) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getCNPatterns:(Ljava/text/NumberFormat$Style;)[Ljava/lang/String;")
        }

        #[java_method(name = "getJavaTimeFormatData", descriptor = "()Ljava/util/ResourceBundle;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getJavaTimeFormatData(&self) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getJavaTimeFormatData:()Ljava/util/ResourceBundle;")
        }

        #[java_method(name = "getLocalizedPattern", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalizedPattern(&self, requestedTemplate: String, calType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getLocalizedPattern:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getLocalizedPatternImpl", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalizedPatternImpl(&self, requestedTemplate: String, calType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getLocalizedPatternImpl:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "matchSkeleton", descriptor = "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn matchSkeleton(&self, skeleton: String, calType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.matchSkeleton:(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "initSkeletonIfNeeded", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initSkeletonIfNeeded(&self) -> Result<()> {
            panic!("stub: sun/util/locale/provider/LocaleResources.initSkeletonIfNeeded:()V")
        }

        #[java_method(name = "resolveInputSkeleton", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resolveInputSkeleton(&self, type_: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.resolveInputSkeleton:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "substituteInputSkeletons", descriptor = "(Ljava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn substituteInputSkeletons(&self, requestedTemplate: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.substituteInputSkeletons:(Ljava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "possibleInferred", descriptor = "(Ljava/lang/String;)Ljava/util/stream/Stream;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/stream/Stream<Ljava/lang/String;>;")]
        pub fn possibleInferred(&self, skeleton: String) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.possibleInferred:(Ljava/lang/String;)Ljava/util/stream/Stream;")
        }

        #[java_method(name = "priorityList", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List<Ljava/lang/String;>;")]
        pub fn priorityList(&self, skeleton: String, pChar: String, subChar: String) -> Result<Object> {
            panic!("stub: sun/util/locale/provider/LocaleResources.priorityList:(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;")
        }

        #[java_method(name = "getDateTimePattern", descriptor = "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDateTimePattern_str_str_i_str(&self, prefix: String, key: String, styleIndex: i32, calendarType: String) -> Result<String> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getDateTimePattern:(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;)Ljava/lang/String;")
        }

        #[java_method(name = "getRules", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRules(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: sun/util/locale/provider/LocaleResources.getRules:()[Ljava/lang/String;")
        }

        #[java_method(name = "trace", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "public", modifiers = "static varargs", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trace(format: String, params: Rc<RefCell<Vec<Object>>>) -> Result<()> {
            panic!("stub: sun/util/locale/provider/LocaleResources.trace:(Ljava/lang/String;[Ljava/lang/Object;)V")
        }
    }
}
