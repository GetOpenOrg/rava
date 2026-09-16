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
    #[binary_name       = "java/text/DateFormatSymbols"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable,java/lang/Cloneable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DateFormatSymbols.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/text/DateFormatSymbols"]
    #[has_hash_code_method = true]

    pub struct DateFormatSymbols {
        #[cfg_attr(any(), java_field(name = "eras", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub eras: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "months", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub months: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "shortMonths", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub shortMonths: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "weekdays", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub weekdays: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "shortWeekdays", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub shortWeekdays: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "ampms", descriptor = "[Ljava/lang/String;", is_static = false))]
        pub ampms: Rc<RefCell<Vec<String>>>,
        #[cfg_attr(any(), java_field(name = "zoneStrings", descriptor = "[[Ljava/lang/String;", is_static = false))]
        pub zoneStrings: Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>,
        #[cfg_attr(any(), java_field(name = "isZoneStringsSet", descriptor = "Z", access = "package", modifiers = "transient", is_static = false))]
        pub isZoneStringsSet: bool,
        #[cfg_attr(any(), java_field(name = "localPatternChars", descriptor = "Ljava/lang/String;", is_static = false))]
        pub localPatternChars: String,
        #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", is_static = false))]
        pub locale: Locale,
        #[cfg_attr(any(), java_field(name = "lastZoneIndex", descriptor = "I", access = "private", modifiers = "transient", is_static = false))]
        pub lastZoneIndex: i32,
        #[cfg_attr(any(), java_field(name = "cachedHashCode", descriptor = "I", access = "package", modifiers = "volatile transient", is_static = false))]
        pub cachedHashCode: i32,
    }

    impl DateFormatSymbols {
        #[cfg_attr(any(), java_field(name = "patternChars", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "GyMdkHmsSEDFwWahKzZYuXL"))]
        // static field: patternChars:Ljava/lang/String;
        pub fn patternChars() -> String {
            String::from("GyMdkHmsSEDFwWahKzZYuXL")
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ERA", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: PATTERN_ERA:I
        pub fn PATTERN_ERA() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_YEAR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: PATTERN_YEAR:I
        pub fn PATTERN_YEAR() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_MONTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: PATTERN_MONTH:I
        pub fn PATTERN_MONTH() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DAY_OF_MONTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: PATTERN_DAY_OF_MONTH:I
        pub fn PATTERN_DAY_OF_MONTH() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_HOUR_OF_DAY1", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: PATTERN_HOUR_OF_DAY1:I
        pub fn PATTERN_HOUR_OF_DAY1() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_HOUR_OF_DAY0", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: PATTERN_HOUR_OF_DAY0:I
        pub fn PATTERN_HOUR_OF_DAY0() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_MINUTE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "6"))]
        // static field: PATTERN_MINUTE:I
        pub fn PATTERN_MINUTE() -> i32 {
            6
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_SECOND", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "7"))]
        // static field: PATTERN_SECOND:I
        pub fn PATTERN_SECOND() -> i32 {
            7
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_MILLISECOND", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "8"))]
        // static field: PATTERN_MILLISECOND:I
        pub fn PATTERN_MILLISECOND() -> i32 {
            8
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DAY_OF_WEEK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: PATTERN_DAY_OF_WEEK:I
        pub fn PATTERN_DAY_OF_WEEK() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DAY_OF_YEAR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: PATTERN_DAY_OF_YEAR:I
        pub fn PATTERN_DAY_OF_YEAR() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DAY_OF_WEEK_IN_MONTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "11"))]
        // static field: PATTERN_DAY_OF_WEEK_IN_MONTH:I
        pub fn PATTERN_DAY_OF_WEEK_IN_MONTH() -> i32 {
            11
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_WEEK_OF_YEAR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "12"))]
        // static field: PATTERN_WEEK_OF_YEAR:I
        pub fn PATTERN_WEEK_OF_YEAR() -> i32 {
            12
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_WEEK_OF_MONTH", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "13"))]
        // static field: PATTERN_WEEK_OF_MONTH:I
        pub fn PATTERN_WEEK_OF_MONTH() -> i32 {
            13
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_AM_PM", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "14"))]
        // static field: PATTERN_AM_PM:I
        pub fn PATTERN_AM_PM() -> i32 {
            14
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_HOUR1", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "15"))]
        // static field: PATTERN_HOUR1:I
        pub fn PATTERN_HOUR1() -> i32 {
            15
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_HOUR0", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "16"))]
        // static field: PATTERN_HOUR0:I
        pub fn PATTERN_HOUR0() -> i32 {
            16
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ZONE_NAME", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "17"))]
        // static field: PATTERN_ZONE_NAME:I
        pub fn PATTERN_ZONE_NAME() -> i32 {
            17
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ZONE_VALUE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "18"))]
        // static field: PATTERN_ZONE_VALUE:I
        pub fn PATTERN_ZONE_VALUE() -> i32 {
            18
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_WEEK_YEAR", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "19"))]
        // static field: PATTERN_WEEK_YEAR:I
        pub fn PATTERN_WEEK_YEAR() -> i32 {
            19
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ISO_DAY_OF_WEEK", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "20"))]
        // static field: PATTERN_ISO_DAY_OF_WEEK:I
        pub fn PATTERN_ISO_DAY_OF_WEEK() -> i32 {
            20
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ISO_ZONE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "21"))]
        // static field: PATTERN_ISO_ZONE:I
        pub fn PATTERN_ISO_ZONE() -> i32 {
            21
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_MONTH_STANDALONE", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "22"))]
        // static field: PATTERN_MONTH_STANDALONE:I
        pub fn PATTERN_MONTH_STANDALONE() -> i32 {
            22
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-5987973545549424702"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -5987973545549424702i64
        }

        #[cfg_attr(any(), java_field(name = "cachedInstances", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static final", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/util/Locale;Ljava/lang/ref/SoftReference<Ljava/text/DateFormatSymbols;>;>;"))]
        // static field: cachedInstances:Ljava/util/concurrent/ConcurrentMap;
        pub fn cachedInstances() -> Object {
            panic!("stub: java/text/DateFormatSymbols.cachedInstances:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            panic!("stub: java/text/DateFormatSymbols.<init>:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_locale(locale: Locale) -> Result<Self> {
            panic!("stub: java/text/DateFormatSymbols.<init>:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_z(flag: bool) -> Result<Self> {
            panic!("stub: java/text/DateFormatSymbols.<init>:(Z)V")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/text/DateFormatSymbols.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "getInstance", descriptor = "()Ljava/text/DateFormatSymbols;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance() -> Result<DateFormatSymbols> {
            panic!("stub: java/text/DateFormatSymbols.getInstance:()Ljava/text/DateFormatSymbols;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;
        pub fn getInstance_locale(mut locale: Locale) -> Result<DateFormatSymbols> {
            let _t0: DateFormatSymbols = DateFormatSymbols::getProviderInstance(Clone::clone(&locale))?;
            let mut dfs: DateFormatSymbols = _t0;
            if !_is_jnull(&dfs) {
                return Ok(dfs);
            }
            return Err(JvmError::Custom("athrow".to_owned()));
        }

        #[java_method(name = "getInstanceRef", descriptor = "(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstanceRef(locale: Locale) -> Result<DateFormatSymbols> {
            panic!("stub: java/text/DateFormatSymbols.getInstanceRef:(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;")
        }

        #[java_method(name = "getProviderInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/DateFormatSymbols;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getProviderInstance(mut locale: Locale) -> Result<DateFormatSymbols> {
            let _t0: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&locale))?;
            let mut adapter: LocaleProviderAdapter = _t0;
            let _t1 = adapter.getDateFormatSymbolsProvider()?;
            let mut provider: DateFormatSymbolsProvider = _t1;
            let _t2 = provider.getInstance(Clone::clone(&locale))?;
            let mut dfsyms: DateFormatSymbols = _t2;
            if _is_jnull(&dfsyms) {
                let _t3: LocaleProviderAdapter = LocaleProviderAdapter::forJRE()?;
                let _t4 = _t3.getDateFormatSymbolsProvider()?;
                provider = _t4;
                let _t5 = provider.getInstance(Clone::clone(&locale))?;
                dfsyms = _t5;
            }
            Ok(dfsyms)
        }

        #[java_method(name = "getEras", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getEras(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/text/DateFormatSymbols.getEras:()[Ljava/lang/String;")
        }

        #[java_method(name = "setEras", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setEras(&self, newEras: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setEras:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getMonths", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonths(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (this.__get_months().borrow().len() as i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "setMonths", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMonths(&self, newMonths: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setMonths:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getShortMonths", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortMonths(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (this.__get_shortMonths().borrow().len() as i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "setShortMonths", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setShortMonths(&self, newShortMonths: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setShortMonths:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getWeekdays", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getWeekdays(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (this.__get_weekdays().borrow().len() as i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "setWeekdays", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setWeekdays(&self, newWeekdays: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setWeekdays:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getShortWeekdays", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getShortWeekdays(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (this.__get_shortWeekdays().borrow().len() as i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "setShortWeekdays", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setShortWeekdays(&self, newShortWeekdays: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setShortWeekdays:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getAmPmStrings", descriptor = "()[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAmPmStrings(&self) -> Result<Rc<RefCell<Vec<String>>>> {
            let this = self;
            let _t0: Rc<RefCell<Vec<Object>>> = Arrays::copyOf_arr_obj_i(Default::default(), (this.__get_ampms().borrow().len() as i32))?;
            Ok(Default::default())
        }

        #[java_method(name = "setAmPmStrings", descriptor = "([Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setAmPmStrings(&self, newAmpms: Rc<RefCell<Vec<String>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setAmPmStrings:([Ljava/lang/String;)V")
        }

        #[java_method(name = "getZoneStrings", descriptor = "()[[Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneStrings(&self) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: java/text/DateFormatSymbols.getZoneStrings:()[[Ljava/lang/String;")
        }

        #[java_method(name = "setZoneStrings", descriptor = "([[Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setZoneStrings(&self, newZoneStrings: Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setZoneStrings:([[Ljava/lang/String;)V")
        }

        #[java_method(name = "getLocalPatternChars", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocalPatternChars(&self) -> Result<String> {
            panic!("stub: java/text/DateFormatSymbols.getLocalPatternChars:()Ljava/lang/String;")
        }

        #[java_method(name = "setLocalPatternChars", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setLocalPatternChars(&self, newLocalPatternChars: String) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.setLocalPatternChars:(Ljava/lang/String;)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/DateFormatSymbols.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/DateFormatSymbols.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "initializeData", descriptor = "(Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initializeData(&self, locale: Locale) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.initializeData:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "toOneBasedArray", descriptor = "([Ljava/lang/String;)[Ljava/lang/String;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toOneBasedArray(src: Rc<RefCell<Vec<String>>>) -> Result<Rc<RefCell<Vec<String>>>> {
            panic!("stub: java/text/DateFormatSymbols.toOneBasedArray:([Ljava/lang/String;)[Ljava/lang/String;")
        }

        #[java_method(name = "getZoneIndex", descriptor = "(Ljava/lang/String;)I", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneIndex(&self, ID: String) -> Result<i32> {
            panic!("stub: java/text/DateFormatSymbols.getZoneIndex:(Ljava/lang/String;)I")
        }

        #[java_method(name = "getZoneStringsWrapper", descriptor = "()[[Ljava/lang/String;", access = "package", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneStringsWrapper(&self) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: java/text/DateFormatSymbols.getZoneStringsWrapper:()[[Ljava/lang/String;")
        }

        #[java_method(name = "getZoneStringsImpl", descriptor = "(Z)[[Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZoneStringsImpl(&self, needsCopy: bool) -> Result<Rc<RefCell<Vec<Rc<RefCell<Vec<String>>>>>>> {
            panic!("stub: java/text/DateFormatSymbols.getZoneStringsImpl:(Z)[[Ljava/lang/String;")
        }

        #[java_method(name = "isSubclassObject", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isSubclassObject(&self) -> Result<bool> {
            panic!("stub: java/text/DateFormatSymbols.isSubclassObject:()Z")
        }

        #[java_method(name = "copyMembers", descriptor = "(Ljava/text/DateFormatSymbols;Ljava/text/DateFormatSymbols;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn copyMembers(&self, src: DateFormatSymbols, dst: DateFormatSymbols) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.copyMembers:(Ljava/text/DateFormatSymbols;Ljava/text/DateFormatSymbols;)V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/text/DateFormatSymbols.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }
    }
}
