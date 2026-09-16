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
    #[binary_name       = "java/text/DecimalFormatSymbols"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/lang/Cloneable,java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DecimalFormatSymbols.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/text/DecimalFormatSymbols"]
    #[has_hash_code_method = true]

    pub struct DecimalFormatSymbols {
        #[cfg_attr(any(), java_field(name = "zeroDigit", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub zeroDigit: u16,
        #[cfg_attr(any(), java_field(name = "groupingSeparator", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub groupingSeparator: u16,
        #[cfg_attr(any(), java_field(name = "decimalSeparator", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub decimalSeparator: u16,
        #[cfg_attr(any(), java_field(name = "perMill", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub perMill: u16,
        #[cfg_attr(any(), java_field(name = "percent", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub percent: u16,
        #[cfg_attr(any(), java_field(name = "digit", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub digit: u16,
        #[cfg_attr(any(), java_field(name = "patternSeparator", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub patternSeparator: u16,
        #[cfg_attr(any(), java_field(name = "infinity", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub infinity: String,
        #[cfg_attr(any(), java_field(name = "NaN", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub NaN: String,
        #[cfg_attr(any(), java_field(name = "minusSign", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub minusSign: u16,
        #[cfg_attr(any(), java_field(name = "currencySymbol", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub currencySymbol: String,
        #[cfg_attr(any(), java_field(name = "intlCurrencySymbol", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub intlCurrencySymbol: String,
        #[cfg_attr(any(), java_field(name = "monetarySeparator", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub monetarySeparator: u16,
        #[cfg_attr(any(), java_field(name = "exponential", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub exponential: u16,
        #[cfg_attr(any(), java_field(name = "exponentialSeparator", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub exponentialSeparator: String,
        #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "", is_static = false))]
        pub locale: Locale,
        #[cfg_attr(any(), java_field(name = "perMillText", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub perMillText: String,
        #[cfg_attr(any(), java_field(name = "percentText", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub percentText: String,
        #[cfg_attr(any(), java_field(name = "minusSignText", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub minusSignText: String,
        #[cfg_attr(any(), java_field(name = "monetaryGroupingSeparator", descriptor = "C", access = "private", modifiers = "", is_static = false))]
        pub monetaryGroupingSeparator: u16,
        #[cfg_attr(any(), java_field(name = "currency", descriptor = "Ljava/util/Currency;", access = "private", modifiers = "transient", is_static = false))]
        pub currency: Currency,
        #[cfg_attr(any(), java_field(name = "currencyInitialized", descriptor = "Z", access = "private", modifiers = "volatile transient", is_static = false))]
        pub currencyInitialized: bool,
        #[cfg_attr(any(), java_field(name = "hashCode", descriptor = "I", access = "private", modifiers = "volatile transient", is_static = false))]
        pub hashCode: i32,
        #[cfg_attr(any(), java_field(name = "serialVersionOnStream", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub serialVersionOnStream: i32,
    }

    impl DecimalFormatSymbols {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "5772796243397350300"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            5772796243397350300i64
        }

        #[cfg_attr(any(), java_field(name = "currentSerialVersion", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: currentSerialVersion:I
        pub fn currentSerialVersion() -> i32 {
            5
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_serialVersionOnStream(5i32);
            let _t0: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            this.initialize(Clone::clone(&_t0))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_locale(locale: Locale) -> Result<Self> {
            panic!("stub: java/text/DecimalFormatSymbols.<init>:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/text/DecimalFormatSymbols.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "getInstance", descriptor = "()Ljava/text/DecimalFormatSymbols;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance() -> Result<DecimalFormatSymbols> {
            panic!("stub: java/text/DecimalFormatSymbols.getInstance:()Ljava/text/DecimalFormatSymbols;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;
        pub fn getInstance_locale(mut locale: Locale) -> Result<DecimalFormatSymbols> {
            let _t0: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&locale))?;
            let mut adapter: LocaleProviderAdapter = _t0;
            let _t1 = adapter.getDecimalFormatSymbolsProvider()?;
            let mut provider: DecimalFormatSymbolsProvider = _t1;
            let _t2 = provider.getInstance(Clone::clone(&locale))?;
            let mut dfsyms: DecimalFormatSymbols = _t2;
            if _is_jnull(&dfsyms) {
                let _t3: LocaleProviderAdapter = LocaleProviderAdapter::forJRE()?;
                let _t4 = _t3.getDecimalFormatSymbolsProvider()?;
                provider = _t4;
                let _t5 = provider.getInstance(Clone::clone(&locale))?;
                dfsyms = _t5;
            }
            Ok(dfsyms)
        }

        #[java_method(name = "getLocale", descriptor = "()Ljava/util/Locale;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getLocale(&self) -> Result<Locale> {
            let this = self;
            Ok(this.__get_locale())
        }

        #[java_method(name = "getZeroDigit", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZeroDigit(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_zeroDigit())
        }

        #[java_method(name = "setZeroDigit", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setZeroDigit(&self, zeroDigit: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setZeroDigit:(C)V")
        }

        #[java_method(name = "getGroupingSeparator", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGroupingSeparator(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_groupingSeparator())
        }

        #[java_method(name = "setGroupingSeparator", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGroupingSeparator(&self, groupingSeparator: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setGroupingSeparator:(C)V")
        }

        #[java_method(name = "getDecimalSeparator", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecimalSeparator(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_decimalSeparator())
        }

        #[java_method(name = "setDecimalSeparator", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDecimalSeparator(&self, decimalSeparator: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setDecimalSeparator:(C)V")
        }

        #[java_method(name = "getPerMill", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPerMill(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_perMill())
        }

        #[java_method(name = "setPerMill", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPerMill(&self, perMill: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setPerMill:(C)V")
        }

        #[java_method(name = "getPercent", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPercent(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_percent())
        }

        #[java_method(name = "setPercent", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPercent(&self, percent: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setPercent:(C)V")
        }

        #[java_method(name = "getDigit", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDigit(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_digit())
        }

        #[java_method(name = "setDigit", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDigit(&self, digit: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setDigit:(C)V")
        }

        #[java_method(name = "getPatternSeparator", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPatternSeparator(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_patternSeparator())
        }

        #[java_method(name = "setPatternSeparator", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPatternSeparator(&self, patternSeparator: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setPatternSeparator:(C)V")
        }

        #[java_method(name = "getInfinity", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInfinity(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormatSymbols.getInfinity:()Ljava/lang/String;")
        }

        #[java_method(name = "setInfinity", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInfinity(&self, infinity: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setInfinity:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getNaN", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNaN(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormatSymbols.getNaN:()Ljava/lang/String;")
        }

        #[java_method(name = "setNaN", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setNaN(&self, NaN: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setNaN:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getMinusSign", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinusSign(&self) -> Result<u16> {
            let this = self;
            Ok(this.__get_minusSign())
        }

        #[java_method(name = "setMinusSign", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinusSign(&self, minusSign: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setMinusSign:(C)V")
        }

        #[java_method(name = "getCurrencySymbol", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrencySymbol(&self) -> Result<String> {
            let this = self;
            this.initializeCurrency(Clone::clone(&this.__get_locale()))?;
            Ok(this.__get_currencySymbol())
        }

        #[java_method(name = "setCurrencySymbol", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCurrencySymbol(&self, currency: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setCurrencySymbol:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getInternationalCurrencySymbol", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInternationalCurrencySymbol(&self) -> Result<String> {
            let this = self;
            this.initializeCurrency(Clone::clone(&this.__get_locale()))?;
            Ok(this.__get_intlCurrencySymbol())
        }

        #[java_method(name = "setInternationalCurrencySymbol", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setInternationalCurrencySymbol(&self, currencyCode: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setInternationalCurrencySymbol:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getCurrency", descriptor = "()Ljava/util/Currency;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrency(&self) -> Result<Currency> {
            panic!("stub: java/text/DecimalFormatSymbols.getCurrency:()Ljava/util/Currency;")
        }

        #[java_method(name = "setCurrency", descriptor = "(Ljava/util/Currency;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCurrency(&self, currency: Currency) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setCurrency:(Ljava/util/Currency;)V")
        }

        #[java_method(name = "getMonetaryDecimalSeparator", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonetaryDecimalSeparator(&self) -> Result<u16> {
            panic!("stub: java/text/DecimalFormatSymbols.getMonetaryDecimalSeparator:()C")
        }

        #[java_method(name = "setMonetaryDecimalSeparator", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMonetaryDecimalSeparator(&self, sep: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setMonetaryDecimalSeparator:(C)V")
        }

        #[java_method(name = "getExponentSeparator", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponentSeparator(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_exponentialSeparator())
        }

        #[java_method(name = "setExponentSeparator", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setExponentSeparator(&self, exp: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setExponentSeparator:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getMonetaryGroupingSeparator", descriptor = "()C", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMonetaryGroupingSeparator(&self) -> Result<u16> {
            panic!("stub: java/text/DecimalFormatSymbols.getMonetaryGroupingSeparator:()C")
        }

        #[java_method(name = "setMonetaryGroupingSeparator", descriptor = "(C)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMonetaryGroupingSeparator(&self, monetaryGroupingSeparator: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setMonetaryGroupingSeparator:(C)V")
        }

        #[java_method(name = "getExponentialSymbol", descriptor = "()C", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getExponentialSymbol(&self) -> Result<u16> {
            panic!("stub: java/text/DecimalFormatSymbols.getExponentialSymbol:()C")
        }

        #[java_method(name = "setExponentialSymbol", descriptor = "(C)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setExponentialSymbol(&self, exp: u16) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setExponentialSymbol:(C)V")
        }

        #[java_method(name = "getPerMillText", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPerMillText(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_perMillText())
        }

        #[java_method(name = "setPerMillText", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPerMillText(&self, perMillText: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setPerMillText:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getPercentText", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPercentText(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_percentText())
        }

        #[java_method(name = "setPercentText", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPercentText(&self, percentText: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setPercentText:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getMinusSignText", descriptor = "()Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinusSignText(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_minusSignText())
        }

        #[java_method(name = "setMinusSignText", descriptor = "(Ljava/lang/String;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinusSignText(&self, minusSignText: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.setMinusSignText:(Ljava/lang/String;)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/DecimalFormatSymbols.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/DecimalFormatSymbols.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "initialize", descriptor = "(Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initialize(&self, mut locale: Locale) -> Result<()> {
            let this = self;
            this.__set_locale(Clone::clone(&locale));
            let _t0 = locale.getUnicodeLocaleType(Clone::clone(&String::from("nu")))?;
            let mut _merged2: Locale;
            if _is_jnull(&_t0) {
                let _t1: Locale = CalendarDataUtility::findRegionOverride(Clone::clone(&locale))?;
                _merged2 = _t1;
            } else {
                _merged2 = locale;
            }
            let mut override_: Locale = _merged2;
            let _t3: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&override_))?;
            let mut adapter: LocaleProviderAdapter = _t3;
            if !(false) {
                let _t4: LocaleProviderAdapter = LocaleProviderAdapter::getResourceBundleBased()?;
                adapter = _t4;
            }
            let _t4 = adapter.getLocaleResources(Clone::clone(&override_))?;
            let _t5 = _t4.getDecimalFormatSymbolsData()?;
            let mut data: Rc<RefCell<Vec<Object>>> = _t5;
            let mut numberElements = (Clone::clone(&data.borrow()[0i32 as usize])).downcast::<Rc<RefCell<Vec<String>>>>();
            let _t6 = Clone::clone(&numberElements.borrow()[0i32 as usize]).charAt(0i32)?;
            this.__set_decimalSeparator(_t6);
            let _t7 = Clone::clone(&numberElements.borrow()[1i32 as usize]).charAt(0i32)?;
            this.__set_groupingSeparator(_t7);
            let _t8 = Clone::clone(&numberElements.borrow()[2i32 as usize]).charAt(0i32)?;
            this.__set_patternSeparator(_t8);
            this.__set_percentText(Clone::clone(&numberElements.borrow()[3i32 as usize]));
            let _t9 = this.findNonFormatChar(Clone::clone(&this.__get_percentText()), ((37i32) as u16))?;
            this.__set_percent(_t9);
            let _t10 = Clone::clone(&numberElements.borrow()[4i32 as usize]).charAt(0i32)?;
            this.__set_zeroDigit(_t10);
            let _t11 = Clone::clone(&numberElements.borrow()[5i32 as usize]).charAt(0i32)?;
            this.__set_digit(_t11);
            this.__set_minusSignText(Clone::clone(&numberElements.borrow()[6i32 as usize]));
            let _t12 = this.findNonFormatChar(Clone::clone(&this.__get_minusSignText()), ((45i32) as u16))?;
            this.__set_minusSign(_t12);
            let _t13 = Clone::clone(&numberElements.borrow()[7i32 as usize]).charAt(0i32)?;
            this.__set_exponential(_t13);
            this.__set_exponentialSeparator(Clone::clone(&numberElements.borrow()[7i32 as usize]));
            this.__set_perMillText(Clone::clone(&numberElements.borrow()[8i32 as usize]));
            let _t14 = this.findNonFormatChar(Clone::clone(&this.__get_perMillText()), ((8240i32) as u16))?;
            this.__set_perMill(_t14);
            this.__set_infinity(Clone::clone(&numberElements.borrow()[9i32 as usize]));
            this.__set_NaN(Clone::clone(&numberElements.borrow()[10i32 as usize]));
            let _t15 = Clone::clone(&numberElements.borrow()[11i32 as usize]).isEmpty()?;
            let mut _merged17: u16;
            if _t15 {
                _merged17 = this.__get_decimalSeparator();
            } else {
                let _t16 = Clone::clone(&numberElements.borrow()[11i32 as usize]).charAt(0i32)?;
                _merged17 = _t16;
            }
            this.__set_monetarySeparator(_merged17);
            let _t18 = Clone::clone(&numberElements.borrow()[12i32 as usize]).isEmpty()?;
            let mut _merged20: u16;
            if _t18 {
                _merged20 = this.__get_groupingSeparator();
            } else {
                let _t19 = Clone::clone(&numberElements.borrow()[12i32 as usize]).charAt(0i32)?;
                _merged20 = _t19;
            }
            this.__set_monetaryGroupingSeparator(_merged20);
            this.__set_intlCurrencySymbol((Clone::clone(&data.borrow()[1i32 as usize])).downcast::<String>());
            this.__set_currencySymbol((Clone::clone(&data.borrow()[2i32 as usize])).downcast::<String>());
            Ok(())
        }

        #[java_method(name = "findNonFormatChar", descriptor = "(Ljava/lang/String;C)C", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn findNonFormatChar(&self, mut src: String, mut defChar: u16) -> Result<u16> {
            let this = self;
            let mut i: i32 = 0i32;
            loop {
                let _t0 = src.length()?;
                if i >= _t0 { break; }
                let _t0 = src.charAt(i)?;
                let mut c: u16 = _t0;
                let _t1: i32 = Character::getType_c(c)?;
                if _t1 != 16i32 {
                    return Ok(c);
                }
                i = i.wrapping_add(1i32);
            }
            Ok(defChar)
        }

        #[java_method(name = "initializeCurrency", descriptor = "(Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initializeCurrency(&self, mut locale: Locale) -> Result<()> {
            let this = self;
            if this.__get_currencyInitialized() {
                return Ok(());
            }
            let _t0 = locale.getCountry()?;
            let _t1 = _t0.isEmpty()?;
            if !(_t1) {
                let _t2: Currency = Currency::getInstance_locale(Clone::clone(&locale))?;
                this.__set_currency(Clone::clone(&_t2));
            }
            if !_is_jnull(&this.__get_currency()) {
                let _t2: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&locale))?;
                let mut adapter: LocaleProviderAdapter = _t2;
                if !(false) {
                    let _t3: LocaleProviderAdapter = LocaleProviderAdapter::getResourceBundleBased()?;
                    adapter = _t3;
                }
                let _t3 = adapter.getLocaleResources(Clone::clone(&locale))?;
                let _t4 = _t3.getDecimalFormatSymbolsData()?;
                let mut data: Rc<RefCell<Vec<Object>>> = _t4;
                let _t5 = this.__get_currency().getCurrencyCode()?;
                this.__set_intlCurrencySymbol(Clone::clone(&_t5));
                if !_is_jnull(&Clone::clone(&data.borrow()[1i32 as usize])) {
                    if Clone::clone(&data.borrow()[1i32 as usize]) == Object::from_any(this.__get_intlCurrencySymbol().clone()) {
                        this.__set_currencySymbol((Clone::clone(&data.borrow()[2i32 as usize])).downcast::<String>());
                    } else {
                        let _t6 = this.__get_currency().getSymbol_locale(Clone::clone(&locale))?;
                        this.__set_currencySymbol(Clone::clone(&_t6));
                        data.borrow_mut()[1i32 as usize] = Object::from_any(this.__get_intlCurrencySymbol().clone());
                        data.borrow_mut()[2i32 as usize] = Object::from_any(this.__get_currencySymbol().clone());
                    }
                } else {
                    let _t6 = this.__get_currency().getSymbol_locale(Clone::clone(&locale))?;
                    this.__set_currencySymbol(Clone::clone(&_t6));
                    data.borrow_mut()[1i32 as usize] = Object::from_any(this.__get_intlCurrencySymbol().clone());
                    data.borrow_mut()[2i32 as usize] = Object::from_any(this.__get_currencySymbol().clone());
                }
            } else {
                this.__set_intlCurrencySymbol(Clone::clone(&String::from("XXX")));
                let _t2: Currency = Currency::getInstance_str(Clone::clone(&this.__get_intlCurrencySymbol()))?;
                this.__set_currency(Clone::clone(&_t2));
                this.__set_currencySymbol(Clone::clone(&String::from("¤")));
            }
            this.__set_currencyInitialized((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/text/DecimalFormatSymbols.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
