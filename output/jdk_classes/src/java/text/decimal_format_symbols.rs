#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/text/DecimalFormatSymbols",
    super_class = "java/lang/Object",
    interfaces  = "java/lang/Cloneable,java/io/Serializable",
    access      = "public",
    source      = "DecimalFormatSymbols.java",
))]
pub struct DecimalFormatSymbols {
    #[cfg_attr(any(), java_field(name = "zeroDigit", descriptor = "C", access = "private"))]
    pub zeroDigit: Field<u16>,
    #[cfg_attr(any(), java_field(name = "groupingSeparator", descriptor = "C", access = "private"))]
    pub groupingSeparator: Field<u16>,
    #[cfg_attr(any(), java_field(name = "decimalSeparator", descriptor = "C", access = "private"))]
    pub decimalSeparator: Field<u16>,
    #[cfg_attr(any(), java_field(name = "perMill", descriptor = "C", access = "private"))]
    pub perMill: Field<u16>,
    #[cfg_attr(any(), java_field(name = "percent", descriptor = "C", access = "private"))]
    pub percent: Field<u16>,
    #[cfg_attr(any(), java_field(name = "digit", descriptor = "C", access = "private"))]
    pub digit: Field<u16>,
    #[cfg_attr(any(), java_field(name = "patternSeparator", descriptor = "C", access = "private"))]
    pub patternSeparator: Field<u16>,
    #[cfg_attr(any(), java_field(name = "infinity", descriptor = "Ljava/lang/String;", access = "private"))]
    pub infinity: Field<String>,
    #[cfg_attr(any(), java_field(name = "NaN", descriptor = "Ljava/lang/String;", access = "private"))]
    pub NaN: Field<String>,
    #[cfg_attr(any(), java_field(name = "minusSign", descriptor = "C", access = "private"))]
    pub minusSign: Field<u16>,
    #[cfg_attr(any(), java_field(name = "currencySymbol", descriptor = "Ljava/lang/String;", access = "private"))]
    pub currencySymbol: Field<String>,
    #[cfg_attr(any(), java_field(name = "intlCurrencySymbol", descriptor = "Ljava/lang/String;", access = "private"))]
    pub intlCurrencySymbol: Field<String>,
    #[cfg_attr(any(), java_field(name = "monetarySeparator", descriptor = "C", access = "private"))]
    pub monetarySeparator: Field<u16>,
    #[cfg_attr(any(), java_field(name = "exponential", descriptor = "C", access = "private"))]
    pub exponential: Field<u16>,
    #[cfg_attr(any(), java_field(name = "exponentialSeparator", descriptor = "Ljava/lang/String;", access = "private"))]
    pub exponentialSeparator: Field<String>,
    #[cfg_attr(any(), java_field(name = "locale", descriptor = "Ljava/util/Locale;", access = "private"))]
    pub locale: Field<Object>,
    #[cfg_attr(any(), java_field(name = "perMillText", descriptor = "Ljava/lang/String;", access = "private"))]
    pub perMillText: Field<String>,
    #[cfg_attr(any(), java_field(name = "percentText", descriptor = "Ljava/lang/String;", access = "private"))]
    pub percentText: Field<String>,
    #[cfg_attr(any(), java_field(name = "minusSignText", descriptor = "Ljava/lang/String;", access = "private"))]
    pub minusSignText: Field<String>,
    #[cfg_attr(any(), java_field(name = "monetaryGroupingSeparator", descriptor = "C", access = "private"))]
    pub monetaryGroupingSeparator: Field<u16>,
    #[cfg_attr(any(), java_field(name = "currency", descriptor = "Ljava/util/Currency;", access = "private"))]
    pub currency: Field<Object>,
    #[cfg_attr(any(), java_field(name = "currencyInitialized", descriptor = "Z", access = "private"))]
    pub currencyInitialized: Field<bool>,
    #[cfg_attr(any(), java_field(name = "hashCode", descriptor = "I", access = "private"))]
    pub hashCode: Field<i32>,
    #[cfg_attr(any(), java_field(name = "serialVersionOnStream", descriptor = "I", access = "private"))]
    pub serialVersionOnStream: Field<i32>,
}

impl DecimalFormatSymbols {
    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { zeroDigit: Field::new(Default::default()), groupingSeparator: Field::new(Default::default()), decimalSeparator: Field::new(Default::default()), perMill: Field::new(Default::default()), percent: Field::new(Default::default()), digit: Field::new(Default::default()), patternSeparator: Field::new(Default::default()), infinity: Field::new(String::new()), NaN: Field::new(String::new()), minusSign: Field::new(Default::default()), currencySymbol: Field::new(String::new()), intlCurrencySymbol: Field::new(String::new()), monetarySeparator: Field::new(Default::default()), exponential: Field::new(Default::default()), exponentialSeparator: Field::new(String::new()), locale: Field::new(Default::default()), perMillText: Field::new(String::new()), percentText: Field::new(String::new()), minusSignText: Field::new(String::new()), monetaryGroupingSeparator: Field::new(Default::default()), currency: Field::new(Default::default()), currencyInitialized: Field::new(false), hashCode: Field::new(0), serialVersionOnStream: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.serialVersionOnStream.set(5i32);
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        this.initialize(_t0)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/util/Locale;)V
    pub fn new__locale(locale: Object) -> Result<Self> {
        let this = Self { zeroDigit: Field::new(Default::default()), groupingSeparator: Field::new(Default::default()), decimalSeparator: Field::new(Default::default()), perMill: Field::new(Default::default()), percent: Field::new(Default::default()), digit: Field::new(Default::default()), patternSeparator: Field::new(Default::default()), infinity: Field::new(String::new()), NaN: Field::new(String::new()), minusSign: Field::new(Default::default()), currencySymbol: Field::new(String::new()), intlCurrencySymbol: Field::new(String::new()), monetarySeparator: Field::new(Default::default()), exponential: Field::new(Default::default()), exponentialSeparator: Field::new(String::new()), locale: Field::new(Default::default()), perMillText: Field::new(String::new()), percentText: Field::new(String::new()), minusSignText: Field::new(String::new()), monetaryGroupingSeparator: Field::new(Default::default()), currency: Field::new(Default::default()), currencyInitialized: Field::new(false), hashCode: Field::new(0), serialVersionOnStream: Field::new(0) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.serialVersionOnStream.set(5i32);
        this.initialize(locale)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public static"))]
    pub fn getAvailableLocales() -> Result<Vec<Object>> {
        let _t0: Object = LocaleServiceProviderPool::getPool(29i32)?;
        let mut pool: Object = _t0;
        let _t1 = pool.getAvailableLocales()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getInstance", descriptor = "()Ljava/text/DecimalFormatSymbols;", access = "public static final"))]
    // java: getInstance()Ljava/text/DecimalFormatSymbols;
    pub fn getInstance() -> Result<Object> {
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t1: Object = DecimalFormatSymbols::getInstance(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;", access = "public static final"))]
    // java: getInstance(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;
    pub fn getInstance__locale(locale: Object) -> Result<Object> {
        let _t0: Object = LocaleProviderAdapter::getAdapter(29i32, locale)?;
        let mut adapter: Object = _t0;
        let _t1 = adapter.getDecimalFormatSymbolsProvider()?;
        let mut provider: Object = _t1;
        let _t2 = provider.getInstance(locale)?;
        let mut dfsyms: Object = _t2;
        let _t3: Object = LocaleProviderAdapter::forJRE()?;
        let _t4 = _t3.getDecimalFormatSymbolsProvider()?;
        provider = _t4;
        let _t5 = provider.getInstance(locale)?;
        dfsyms = _t5;
        Ok(dfsyms)
    }

    #[cfg_attr(any(), java_method(name = "getLocale", descriptor = "()Ljava/util/Locale;", access = "public"))]
    pub fn getLocale(&self) -> Result<Object> {
        let this = self;
        Ok(this.locale.get())
    }

    #[cfg_attr(any(), java_method(name = "getZeroDigit", descriptor = "()C", access = "public"))]
    pub fn getZeroDigit(&self) -> Result<u16> {
        let this = self;
        Ok(this.zeroDigit.get())
    }

    #[cfg_attr(any(), java_method(name = "setZeroDigit", descriptor = "(C)V", access = "public"))]
    pub fn setZeroDigit(&self, zeroDigit: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.zeroDigit.set(zeroDigit);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getGroupingSeparator", descriptor = "()C", access = "public"))]
    pub fn getGroupingSeparator(&self) -> Result<u16> {
        let this = self;
        Ok(this.groupingSeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setGroupingSeparator", descriptor = "(C)V", access = "public"))]
    pub fn setGroupingSeparator(&self, groupingSeparator: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.groupingSeparator.set(groupingSeparator);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getDecimalSeparator", descriptor = "()C", access = "public"))]
    pub fn getDecimalSeparator(&self) -> Result<u16> {
        let this = self;
        Ok(this.decimalSeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setDecimalSeparator", descriptor = "(C)V", access = "public"))]
    pub fn setDecimalSeparator(&self, decimalSeparator: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.decimalSeparator.set(decimalSeparator);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPerMill", descriptor = "()C", access = "public"))]
    pub fn getPerMill(&self) -> Result<u16> {
        let this = self;
        Ok(this.perMill.get())
    }

    #[cfg_attr(any(), java_method(name = "setPerMill", descriptor = "(C)V", access = "public"))]
    pub fn setPerMill(&self, perMill: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.perMill.set(perMill);
        let _t0: String = Character::toString(perMill)?;
        this.perMillText.set(_t0);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPercent", descriptor = "()C", access = "public"))]
    pub fn getPercent(&self) -> Result<u16> {
        let this = self;
        Ok(this.percent.get())
    }

    #[cfg_attr(any(), java_method(name = "setPercent", descriptor = "(C)V", access = "public"))]
    pub fn setPercent(&self, percent: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.percent.set(percent);
        let _t0: String = Character::toString(percent)?;
        this.percentText.set(_t0);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getDigit", descriptor = "()C", access = "public"))]
    pub fn getDigit(&self) -> Result<u16> {
        let this = self;
        Ok(this.digit.get())
    }

    #[cfg_attr(any(), java_method(name = "setDigit", descriptor = "(C)V", access = "public"))]
    pub fn setDigit(&self, digit: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.digit.set(digit);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPatternSeparator", descriptor = "()C", access = "public"))]
    pub fn getPatternSeparator(&self) -> Result<u16> {
        let this = self;
        Ok(this.patternSeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setPatternSeparator", descriptor = "(C)V", access = "public"))]
    pub fn setPatternSeparator(&self, patternSeparator: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.patternSeparator.set(patternSeparator);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getInfinity", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getInfinity(&self) -> Result<String> {
        let this = self;
        Ok(this.infinity.get())
    }

    #[cfg_attr(any(), java_method(name = "setInfinity", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn setInfinity(&self, infinity: String) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.infinity.set(infinity);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getNaN", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getNaN(&self) -> Result<String> {
        let this = self;
        Ok(this.NaN.get())
    }

    #[cfg_attr(any(), java_method(name = "setNaN", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn setNaN(&self, NaN: String) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.NaN.set(NaN);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getMinusSign", descriptor = "()C", access = "public"))]
    pub fn getMinusSign(&self) -> Result<u16> {
        let this = self;
        Ok(this.minusSign.get())
    }

    #[cfg_attr(any(), java_method(name = "setMinusSign", descriptor = "(C)V", access = "public"))]
    pub fn setMinusSign(&self, minusSign: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.minusSign.set(minusSign);
        let _t0: String = Character::toString(minusSign)?;
        this.minusSignText.set(_t0);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getCurrencySymbol", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getCurrencySymbol(&self) -> Result<String> {
        let this = self;
        this.initializeCurrency(this.locale.get())?;
        Ok(this.currencySymbol.get())
    }

    #[cfg_attr(any(), java_method(name = "setCurrencySymbol", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn setCurrencySymbol(&self, currency: String) -> Result<()> {
        let this = self;
        this.initializeCurrency(this.locale.get())?;
        this.hashCode.set(0i32);
        this.currencySymbol.set(currency);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getInternationalCurrencySymbol", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getInternationalCurrencySymbol(&self) -> Result<String> {
        let this = self;
        this.initializeCurrency(this.locale.get())?;
        Ok(this.intlCurrencySymbol.get())
    }

    #[cfg_attr(any(), java_method(name = "setInternationalCurrencySymbol", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn setInternationalCurrencySymbol(&self, currencyCode: String) -> Result<()> {
        let this = self;
        this.initializeCurrency(this.locale.get())?;
        this.hashCode.set(0i32);
        this.intlCurrencySymbol.set(currencyCode);
        /* TODO: aconst_null  */
        todo!("stack underflow").currency.set(this);
        let _t0: Object = Currency::getInstance(currencyCode)?;
        this.currency.set(_t0);
        let _t1 = this.currency.get().getSymbol()?;
        this.currencySymbol.set(_t1);
        let mut local_2: String = currencyCode;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getCurrency", descriptor = "()Ljava/util/Currency;", access = "public"))]
    pub fn getCurrency(&self) -> Result<Object> {
        let this = self;
        this.initializeCurrency(this.locale.get())?;
        Ok(this.currency.get())
    }

    #[cfg_attr(any(), java_method(name = "setCurrency", descriptor = "(Ljava/util/Currency;)V", access = "public"))]
    pub fn setCurrency(&self, currency: Object) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.initializeCurrency(this.locale.get())?;
        this.hashCode.set(0i32);
        this.currency.set(currency);
        let _t0 = currency.getCurrencyCode()?;
        this.intlCurrencySymbol.set(_t0);
        let _t1 = currency.getSymbol(this.locale.get())?;
        this.currencySymbol.set(_t1);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getMonetaryDecimalSeparator", descriptor = "()C", access = "public"))]
    pub fn getMonetaryDecimalSeparator(&self) -> Result<u16> {
        let this = self;
        Ok(this.monetarySeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setMonetaryDecimalSeparator", descriptor = "(C)V", access = "public"))]
    pub fn setMonetaryDecimalSeparator(&self, sep: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.monetarySeparator.set(sep);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getExponentSeparator", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getExponentSeparator(&self) -> Result<String> {
        let this = self;
        Ok(this.exponentialSeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setExponentSeparator", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn setExponentSeparator(&self, exp: String) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hashCode.set(0i32);
        this.exponentialSeparator.set(exp);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getMonetaryGroupingSeparator", descriptor = "()C", access = "public"))]
    pub fn getMonetaryGroupingSeparator(&self) -> Result<u16> {
        let this = self;
        Ok(this.monetaryGroupingSeparator.get())
    }

    #[cfg_attr(any(), java_method(name = "setMonetaryGroupingSeparator", descriptor = "(C)V", access = "public"))]
    pub fn setMonetaryGroupingSeparator(&self, monetaryGroupingSeparator: u16) -> Result<()> {
        let this = self;
        this.hashCode.set(0i32);
        this.monetaryGroupingSeparator.set(monetaryGroupingSeparator);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getExponentialSymbol", descriptor = "()C"))]
    pub fn getExponentialSymbol(&self) -> Result<u16> {
        let this = self;
        Ok(this.exponential.get())
    }

    #[cfg_attr(any(), java_method(name = "setExponentialSymbol", descriptor = "(C)V"))]
    pub fn setExponentialSymbol(&self, exp: u16) -> Result<()> {
        let this = self;
        this.exponential.set(exp);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPerMillText", descriptor = "()Ljava/lang/String;"))]
    pub fn getPerMillText(&self) -> Result<String> {
        let this = self;
        Ok(this.perMillText.get())
    }

    #[cfg_attr(any(), java_method(name = "setPerMillText", descriptor = "(Ljava/lang/String;)V"))]
    pub fn setPerMillText(&self, perMillText: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(perMillText)?;
        let _t1 = perMillText.isEmpty()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hashCode.set(0i32);
        this.perMillText.set(perMillText);
        let _t2 = this.findNonFormatChar(perMillText, 8240i32)?;
        this.perMill.set(_t2);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getPercentText", descriptor = "()Ljava/lang/String;"))]
    pub fn getPercentText(&self) -> Result<String> {
        let this = self;
        Ok(this.percentText.get())
    }

    #[cfg_attr(any(), java_method(name = "setPercentText", descriptor = "(Ljava/lang/String;)V"))]
    pub fn setPercentText(&self, percentText: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(percentText)?;
        let _t1 = percentText.isEmpty()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hashCode.set(0i32);
        this.percentText.set(percentText);
        let _t2 = this.findNonFormatChar(percentText, 37i32)?;
        this.percent.set(_t2);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "getMinusSignText", descriptor = "()Ljava/lang/String;"))]
    pub fn getMinusSignText(&self) -> Result<String> {
        let this = self;
        Ok(this.minusSignText.get())
    }

    #[cfg_attr(any(), java_method(name = "setMinusSignText", descriptor = "(Ljava/lang/String;)V"))]
    pub fn setMinusSignText(&self, minusSignText: String) -> Result<()> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(minusSignText)?;
        let _t1 = minusSignText.isEmpty()?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.hashCode.set(0i32);
        this.minusSignText.set(minusSignText);
        let _t2 = this.findNonFormatChar(minusSignText, 45i32)?;
        this.minusSign.set(_t2);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public"))]
    pub fn clone(&self) -> Result<Object> {
        let this = self;
        return Ok(this);
        let mut e: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn equals(&self, obj: Object) -> Result<bool> {
        let this = self;
        return Ok(0i32);
        return Ok(1i32);
        let _t0 = this.getClass()?;
        let _t1 = obj.getClass()?;
        return Ok(0i32);
        let mut other: Object = obj;
        let _t2 = this.percentText.get().equals(other.percentText.get())?;
        let _t3 = this.perMillText.get().equals(other.perMillText.get())?;
        let _t4 = this.minusSignText.get().equals(other.minusSignText.get())?;
        let _t5 = this.infinity.get().equals(other.infinity.get())?;
        let _t6 = this.NaN.get().equals(other.NaN.get())?;
        let _t7 = this.getCurrencySymbol()?;
        let _t8 = other.getCurrencySymbol()?;
        let _t9 = _t7.equals(_t8)?;
        let _t10 = this.intlCurrencySymbol.get().equals(other.intlCurrencySymbol.get())?;
        let _t11 = this.exponentialSeparator.get().equals(other.exponentialSeparator.get())?;
        let _t12 = this.locale.get().equals(other.locale.get())?;
        Ok(_t12!=0i32)
    }

    #[cfg_attr(any(), java_method(name = "hashCode", descriptor = "()I", access = "public"))]
    pub fn hashCode(&self) -> Result<i32> {
        let this = self;
        let mut _arr0: Vec<Object> = Vec::with_capacity(20i32 as usize);
        let _t1: Object = Character::valueOf(this.zeroDigit.get())?;
        _arr0[0i32 as usize] = _t1;
        let _t2: Object = Character::valueOf(this.groupingSeparator.get())?;
        _arr0[1i32 as usize] = _t2;
        let _t3: Object = Character::valueOf(this.decimalSeparator.get())?;
        _arr0[2i32 as usize] = _t3;
        let _t4: Object = Character::valueOf(this.percent.get())?;
        _arr0[3i32 as usize] = _t4;
        _arr0[4i32 as usize] = this.percentText.get();
        let _t5: Object = Character::valueOf(this.perMill.get())?;
        _arr0[5i32 as usize] = _t5;
        _arr0[6i32 as usize] = this.perMillText.get();
        let _t6: Object = Character::valueOf(this.digit.get())?;
        _arr0[7i32 as usize] = _t6;
        let _t7: Object = Character::valueOf(this.minusSign.get())?;
        _arr0[8i32 as usize] = _t7;
        _arr0[9i32 as usize] = this.minusSignText.get();
        let _t8: Object = Character::valueOf(this.patternSeparator.get())?;
        _arr0[10i32 as usize] = _t8;
        _arr0[11i32 as usize] = this.infinity.get();
        _arr0[12i32 as usize] = this.NaN.get();
        let _t9 = this.getCurrencySymbol()?;
        _arr0[13i32 as usize] = _t9;
        _arr0[14i32 as usize] = this.intlCurrencySymbol.get();
        _arr0[15i32 as usize] = this.currency.get();
        let _t10: Object = Character::valueOf(this.monetarySeparator.get())?;
        _arr0[16i32 as usize] = _t10;
        let _t11: Object = Character::valueOf(this.monetaryGroupingSeparator.get())?;
        _arr0[17i32 as usize] = _t11;
        _arr0[18i32 as usize] = this.exponentialSeparator.get();
        _arr0[19i32 as usize] = this.locale.get();
        let _t12: i32 = Objects::hash(&_arr0)?;
        this.hashCode.set(_t12);
        Ok(this.hashCode.get())
    }

    #[cfg_attr(any(), java_method(name = "initialize", descriptor = "(Ljava/util/Locale;)V", access = "private"))]
    pub fn initialize(&self, locale: Object) -> Result<()> {
        let this = self;
        this.locale.set(locale);
        let _t0 = locale.getUnicodeLocaleType(String::from("nu"))?;
        let _t1: Object = CalendarDataUtility::findRegionOverride(locale)?;
        let mut override_: Object = locale;
        let _t2: Object = LocaleProviderAdapter::getAdapter(29i32, override_)?;
        let mut adapter: Object = _t2;
        let _t3: Object = LocaleProviderAdapter::getResourceBundleBased()?;
        adapter = _t3;
        let _t4 = adapter.getLocaleResources(override_)?;
        let _t5 = _t4.getDecimalFormatSymbolsData()?;
        let mut data: Vec<Object> = _t5;
        let mut numberElements: Object = data[0i32 as usize].clone();
        let _t6 = numberElements[0i32 as usize].clone().charAt(0i32)?;
        this.decimalSeparator.set(_t6);
        let _t7 = numberElements[1i32 as usize].clone().charAt(0i32)?;
        this.groupingSeparator.set(_t7);
        let _t8 = numberElements[2i32 as usize].clone().charAt(0i32)?;
        this.patternSeparator.set(_t8);
        this.percentText.set(numberElements[3i32 as usize].clone());
        let _t9 = this.findNonFormatChar(this.percentText.get(), 37i32)?;
        this.percent.set(_t9);
        let _t10 = numberElements[4i32 as usize].clone().charAt(0i32)?;
        this.zeroDigit.set(_t10);
        let _t11 = numberElements[5i32 as usize].clone().charAt(0i32)?;
        this.digit.set(_t11);
        this.minusSignText.set(numberElements[6i32 as usize].clone());
        let _t12 = this.findNonFormatChar(this.minusSignText.get(), 45i32)?;
        this.minusSign.set(_t12);
        let _t13 = numberElements[7i32 as usize].clone().charAt(0i32)?;
        this.exponential.set(_t13);
        this.exponentialSeparator.set(numberElements[7i32 as usize].clone());
        this.perMillText.set(numberElements[8i32 as usize].clone());
        let _t14 = this.findNonFormatChar(this.perMillText.get(), 8240i32)?;
        this.perMill.set(_t14);
        this.infinity.set(numberElements[9i32 as usize].clone());
        this.NaN.set(numberElements[10i32 as usize].clone());
        let _t15 = numberElements[11i32 as usize].clone().isEmpty()?;
        let _t16 = numberElements[11i32 as usize].clone().charAt(0i32)?;
        this.decimalSeparator.get().monetarySeparator.set(_t16);
        let _t17 = numberElements[12i32 as usize].clone().isEmpty()?;
        let _t18 = numberElements[12i32 as usize].clone().charAt(0i32)?;
        this.groupingSeparator.get().monetaryGroupingSeparator.set(_t18);
        this.intlCurrencySymbol.set(data[1i32 as usize].clone());
        this.currencySymbol.set(data[2i32 as usize].clone());
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "findNonFormatChar", descriptor = "(Ljava/lang/String;C)C", access = "private"))]
    pub fn findNonFormatChar(&self, src: String, defChar: u16) -> Result<u16> {
        let this = self;
        let mut i: i32 = 0i32;
        loop {
            let _t0 = src.length()?;
            if i >= _t0 { break; }
            let _t0 = src.charAt(i)?;
            let mut c: i32 = _t0;
            let _t1: i32 = Character::getType(c)?;
            return Ok(c);
            i = i.wrapping_add(1i32);
        }
        Ok(defChar)
    }

    #[cfg_attr(any(), java_method(name = "initializeCurrency", descriptor = "(Ljava/util/Locale;)V", access = "private"))]
    pub fn initializeCurrency(&self, locale: Object) -> Result<()> {
        let this = self;
        return Ok(());
        let _t0 = locale.getCountry()?;
        let _t1 = _t0.isEmpty()?;
        let _t2: Object = Currency::getInstance(locale)?;
        this.currency.set(_t2);
        let mut adapter: bool = _t1;
        let _t3: Object = LocaleProviderAdapter::getAdapter(29i32, locale)?;
        adapter = _t3;
        let _t4: Object = LocaleProviderAdapter::getResourceBundleBased()?;
        adapter = _t4;
        let _t5 = adapter.getLocaleResources(locale)?;
        let _t6 = _t5.getDecimalFormatSymbolsData()?;
        let mut data: Vec<Object> = _t6;
        let _t7 = this.currency.get().getCurrencyCode()?;
        this.intlCurrencySymbol.set(_t7);
        this.currencySymbol.set(data[2i32 as usize].clone());
        let _t8 = this.currency.get().getSymbol(locale)?;
        this.currencySymbol.set(_t8);
        data[1i32 as usize] = this.intlCurrencySymbol.get();
        data[2i32 as usize] = this.currencySymbol.get();
        this.intlCurrencySymbol.set(String::from("XXX"));
        let _t9: Object = Currency::getInstance(this.intlCurrencySymbol.get())?;
        this.currency.set(_t9);
        adapter = this.intlCurrencySymbol.get();
        this.currencySymbol.set(String::from("¤"));
        this.currencyInitialized.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private"))]
    pub fn readObject(&self, stream: Object) -> Result<()> {
        let this = self;
        stream.defaultReadObject()?;
        this.monetarySeparator.set(this.decimalSeparator.get());
        this.exponential.set(69i32);
        this.locale.set(Locale::ROOT());
        let _t0: String = Character::toString(this.exponential.get())?;
        this.exponentialSeparator.set(_t0);
        let _t1: String = Character::toString(this.perMill.get())?;
        this.perMillText.set(_t1);
        let _t2: String = Character::toString(this.percent.get())?;
        this.percentText.set(_t2);
        let _t3: String = Character::toString(this.minusSign.get())?;
        this.minusSignText.set(_t3);
        let _t4 = this.findNonFormatChar(this.perMillText.get(), 274i32)?;
        let _t5 = this.findNonFormatChar(this.percentText.get(), 274i32)?;
        let _t6 = this.findNonFormatChar(this.minusSignText.get(), 274i32)?;
        return Err(JvmError::Custom(String::from("athrow")));
        this.monetaryGroupingSeparator.set(this.groupingSeparator.get());
        this.serialVersionOnStream.set(5i32);
        let _t7: Object = Currency::getInstance(this.intlCurrencySymbol.get())?;
        this.currency.set(_t7);
        let mut local_2: String = this.intlCurrencySymbol.get();
        this.currencyInitialized.set(1i32);
        Ok(())
    }
}
