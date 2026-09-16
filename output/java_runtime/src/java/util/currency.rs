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
    #[binary_name       = "java/util/Currency"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Serializable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Currency.java"]
    #[inner_classes     = "java/util/Currency$1:::0;java/util/Currency$SpecialCaseEntry:java/util/Currency:SpecialCaseEntry:10;java/util/Currency$OtherCurrencyEntry:java/util/Currency:OtherCurrencyEntry:10;java/util/Locale$Category:java/util/Locale:Category:16409;java/util/Currency$CurrencyNameGetter:java/util/Currency:CurrencyNameGetter:10;sun/util/locale/provider/LocaleServiceProviderPool$LocalizedObjectGetter:sun/util/locale/provider/LocaleServiceProviderPool:LocalizedObjectGetter:1545;java/util/Currency$CurrencyProperty:java/util/Currency:CurrencyProperty:10;java/lang/invoke/MethodHandles$Lookup:java/lang/invoke/MethodHandles:Lookup:25"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Serializable;java/lang/Object;java/util/Currency"]
    #[has_to_string_method = true]

    pub struct Currency {
        #[cfg_attr(any(), java_field(name = "currencyCode", descriptor = "Ljava/lang/String;", access = "private", modifiers = "final", is_static = false))]
        pub currencyCode: String,
        #[cfg_attr(any(), java_field(name = "defaultFractionDigits", descriptor = "I", access = "private", modifiers = "final transient", is_static = false))]
        pub defaultFractionDigits: i32,
        #[cfg_attr(any(), java_field(name = "numericCode", descriptor = "I", access = "private", modifiers = "final transient", is_static = false))]
        pub numericCode: i32,
    }

    impl Currency {
        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "-158308464356906721"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -158308464356906721i64
        }

        #[cfg_attr(any(), java_field(name = "instances", descriptor = "Ljava/util/concurrent/ConcurrentMap;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/concurrent/ConcurrentMap<Ljava/lang/String;Ljava/util/Currency;>;"))]
        // static field: instances:Ljava/util/concurrent/ConcurrentMap;
        pub fn instances() -> Object {
            panic!("stub: java/util/Currency.instances:Ljava/util/concurrent/ConcurrentMap;")
        }

        #[cfg_attr(any(), java_field(name = "available", descriptor = "Ljava/util/HashSet;", access = "private", modifiers = "static", is_static = true, generic_signature = "Ljava/util/HashSet<Ljava/util/Currency;>;"))]
        // static field: available:Ljava/util/HashSet;
        pub fn available() -> HashSet<Currency> {
            panic!("stub: java/util/Currency.available:Ljava/util/HashSet;")
        }

        #[cfg_attr(any(), java_field(name = "formatVersion", descriptor = "I", access = "package", modifiers = "static", is_static = true))]
        // static field: formatVersion:I
        pub fn formatVersion() -> i32 {
            panic!("stub: java/util/Currency.formatVersion:I")
        }

        #[cfg_attr(any(), java_field(name = "dataVersion", descriptor = "I", access = "package", modifiers = "static", is_static = true))]
        // static field: dataVersion:I
        pub fn dataVersion() -> i32 {
            panic!("stub: java/util/Currency.dataVersion:I")
        }

        #[cfg_attr(any(), java_field(name = "mainTable", descriptor = "[I", access = "package", modifiers = "static", is_static = true))]
        // static field: mainTable:[I
        pub fn mainTable() -> Rc<RefCell<Vec<i32>>> {
            panic!("stub: java/util/Currency.mainTable:[I")
        }

        #[cfg_attr(any(), java_field(name = "specialCasesList", descriptor = "Ljava/util/List;", access = "package", modifiers = "static", is_static = true, generic_signature = "Ljava/util/List<Ljava/util/Currency$SpecialCaseEntry;>;"))]
        // static field: specialCasesList:Ljava/util/List;
        pub fn specialCasesList() -> Object {
            panic!("stub: java/util/Currency.specialCasesList:Ljava/util/List;")
        }

        #[cfg_attr(any(), java_field(name = "otherCurrenciesList", descriptor = "Ljava/util/List;", access = "package", modifiers = "static", is_static = true, generic_signature = "Ljava/util/List<Ljava/util/Currency$OtherCurrencyEntry;>;"))]
        // static field: otherCurrenciesList:Ljava/util/List;
        pub fn otherCurrenciesList() -> Object {
            panic!("stub: java/util/Currency.otherCurrenciesList:Ljava/util/List;")
        }

        #[cfg_attr(any(), java_field(name = "MAGIC_NUMBER", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1131770436"))]
        // static field: MAGIC_NUMBER:I
        pub fn MAGIC_NUMBER() -> i32 {
            1131770436
        }

        #[cfg_attr(any(), java_field(name = "A_TO_Z", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "26"))]
        // static field: A_TO_Z:I
        pub fn A_TO_Z() -> i32 {
            26
        }

        #[cfg_attr(any(), java_field(name = "INVALID_COUNTRY_ENTRY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "127"))]
        // static field: INVALID_COUNTRY_ENTRY:I
        pub fn INVALID_COUNTRY_ENTRY() -> i32 {
            127
        }

        #[cfg_attr(any(), java_field(name = "COUNTRY_WITHOUT_CURRENCY_ENTRY", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: COUNTRY_WITHOUT_CURRENCY_ENTRY:I
        pub fn COUNTRY_WITHOUT_CURRENCY_ENTRY() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "SIMPLE_CASE_COUNTRY_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: SIMPLE_CASE_COUNTRY_MASK:I
        pub fn SIMPLE_CASE_COUNTRY_MASK() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "SIMPLE_CASE_COUNTRY_FINAL_CHAR_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "31"))]
        // static field: SIMPLE_CASE_COUNTRY_FINAL_CHAR_MASK:I
        pub fn SIMPLE_CASE_COUNTRY_FINAL_CHAR_MASK() -> i32 {
            31
        }

        #[cfg_attr(any(), java_field(name = "SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "480"))]
        // static field: SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_MASK:I
        pub fn SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_MASK() -> i32 {
            480
        }

        #[cfg_attr(any(), java_field(name = "SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_SHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_SHIFT:I
        pub fn SIMPLE_CASE_COUNTRY_DEFAULT_DIGITS_SHIFT() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "SIMPLE_CASE_COUNTRY_MAX_DEFAULT_DIGITS", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "9"))]
        // static field: SIMPLE_CASE_COUNTRY_MAX_DEFAULT_DIGITS:I
        pub fn SIMPLE_CASE_COUNTRY_MAX_DEFAULT_DIGITS() -> i32 {
            9
        }

        #[cfg_attr(any(), java_field(name = "SPECIAL_CASE_COUNTRY_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: SPECIAL_CASE_COUNTRY_MASK:I
        pub fn SPECIAL_CASE_COUNTRY_MASK() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "SPECIAL_CASE_COUNTRY_INDEX_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "31"))]
        // static field: SPECIAL_CASE_COUNTRY_INDEX_MASK:I
        pub fn SPECIAL_CASE_COUNTRY_INDEX_MASK() -> i32 {
            31
        }

        #[cfg_attr(any(), java_field(name = "SPECIAL_CASE_COUNTRY_INDEX_DELTA", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: SPECIAL_CASE_COUNTRY_INDEX_DELTA:I
        pub fn SPECIAL_CASE_COUNTRY_INDEX_DELTA() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "COUNTRY_TYPE_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "512"))]
        // static field: COUNTRY_TYPE_MASK:I
        pub fn COUNTRY_TYPE_MASK() -> i32 {
            512
        }

        #[cfg_attr(any(), java_field(name = "NUMERIC_CODE_MASK", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1047552"))]
        // static field: NUMERIC_CODE_MASK:I
        pub fn NUMERIC_CODE_MASK() -> i32 {
            1047552
        }

        #[cfg_attr(any(), java_field(name = "NUMERIC_CODE_SHIFT", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "10"))]
        // static field: NUMERIC_CODE_SHIFT:I
        pub fn NUMERIC_CODE_SHIFT() -> i32 {
            10
        }

        #[cfg_attr(any(), java_field(name = "VALID_FORMAT_VERSION", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: VALID_FORMAT_VERSION:I
        pub fn VALID_FORMAT_VERSION() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "SYMBOL", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: SYMBOL:I
        pub fn SYMBOL() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "DISPLAYNAME", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: DISPLAYNAME:I
        pub fn DISPLAYNAME() -> i32 {
            1
        }

        #[java_method(name = "initStatic", descriptor = "()V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn initStatic() -> Result<()> {
            panic!("stub: java/util/Currency.initStatic:()V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new(mut currencyCode: String, mut defaultFractionDigits: i32, mut numericCode: i32) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_currencyCode(Clone::clone(&currencyCode));
            this.__set_defaultFractionDigits(defaultFractionDigits);
            this.__set_numericCode(numericCode);
            Ok(this)
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/lang/String;)Ljava/util/Currency;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/lang/String;)Ljava/util/Currency;
        pub fn getInstance_str(mut currencyCode: String) -> Result<Currency> {
            let _t0: Currency = Currency::getInstance_str_i_i(Clone::clone(&currencyCode), -2147483648i32, 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/lang/String;II)Ljava/util/Currency;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/lang/String;II)Ljava/util/Currency;
        pub fn getInstance_str_i_i(mut currencyCode: String, mut defaultFractionDigits: i32, mut numericCode: i32) -> Result<Currency> {
            let _vdispatch0: Object = if let Some(_d) = Currency::instances().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.get(Object::from_any(currencyCode.clone()))? } else if let Some(_d) = Currency::instances().0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(currencyCode.clone()))? } else if let Some(__f) = Currency::instances().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(currencyCode.clone()))? } else { Default::default() };
            let mut instance = (_vdispatch0).downcast::<Currency>();
            if !_is_jnull(&instance) {
                return Ok(instance);
            }
            let mut found: i32 = 0i32;
            let _t1 = currencyCode.length()?;
            if _t1 != 3i32 {
                let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr2.borrow_mut()[0i32 as usize] = Object::from_any(currencyCode.clone());
                let _t3 = String::from("The input currency code: \"%s\" must have a length of 3 characters").formatted(Clone::clone(&_arr2))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t2 = currencyCode.charAt(0i32)?;
            let mut char1: u16 = _t2;
            let _t3 = currencyCode.charAt(1i32)?;
            let mut char2: u16 = _t3;
            let _t4: i32 = Currency::getMainTableEntry(char1, char2)?;
            let mut tableEntry: i32 = _t4;
            if ((tableEntry&512i32)==0) {
                if tableEntry != 127i32 {
                    let _t5 = currencyCode.charAt(2i32)?;
                    if ((_t5 as i32)).wrapping_sub(65i32) == (tableEntry&31i32) {
                        defaultFractionDigits = ((tableEntry&480i32)>>((5i32&0x1f)));
                        numericCode = ((tableEntry&1047552i32)>>((10i32&0x1f)));
                        found = 1i32;
                    } else {
                        let _t6: Rc<RefCell<Vec<i32>>> = Currency_SpecialCaseEntry::findEntry(Clone::clone(&currencyCode))?;
                        let mut fractionAndNumericCode: Rc<RefCell<Vec<i32>>> = _t6;
                        if !_is_jnull(&fractionAndNumericCode) {
                            defaultFractionDigits = fractionAndNumericCode.borrow()[0i32 as usize];
                            numericCode = fractionAndNumericCode.borrow()[1i32 as usize];
                            found = 1i32;
                        }
                    }
                } else {
                    let _t5: Rc<RefCell<Vec<i32>>> = Currency_SpecialCaseEntry::findEntry(Clone::clone(&currencyCode))?;
                    let mut fractionAndNumericCode: Rc<RefCell<Vec<i32>>> = _t5;
                    if !_is_jnull(&fractionAndNumericCode) {
                        defaultFractionDigits = fractionAndNumericCode.borrow()[0i32 as usize];
                        numericCode = fractionAndNumericCode.borrow()[1i32 as usize];
                        found = 1i32;
                    }
                }
            } else {
                let _t5: Rc<RefCell<Vec<i32>>> = Currency_SpecialCaseEntry::findEntry(Clone::clone(&currencyCode))?;
                let mut fractionAndNumericCode: Rc<RefCell<Vec<i32>>> = _t5;
                if !_is_jnull(&fractionAndNumericCode) {
                    defaultFractionDigits = fractionAndNumericCode.borrow()[0i32 as usize];
                    numericCode = fractionAndNumericCode.borrow()[1i32 as usize];
                    found = 1i32;
                }
            }
            let _t5: Currency_OtherCurrencyEntry = Currency_OtherCurrencyEntry::findEntry(Clone::clone(&currencyCode))?;
            let mut fractionAndNumericCode: Currency_OtherCurrencyEntry = _t5;
            if _is_jnull(&fractionAndNumericCode) {
                let mut _arr6: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr6.borrow_mut()[0i32 as usize] = Object::from_any(currencyCode.clone());
                let _t7 = String::from("The input currency code: \"%s\" is not a valid ISO 4217 code").formatted(Clone::clone(&_arr6))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            defaultFractionDigits = fractionAndNumericCode.__get_fraction();
            numericCode = fractionAndNumericCode.__get_numericCode();
            let mut found = Currency::new(Clone::clone(&currencyCode), defaultFractionDigits, numericCode)?;
            let _vdispatch6: Object = if let Some(_d) = Currency::instances().0.as_any().downcast_ref::<ConcurrentHashMap<Object, Object>>() { _d.putIfAbsent(Object::from_any(currencyCode.clone()), Object::from_any(found.clone()))? } else if let Some(_d) = Currency::instances().0.as_any().downcast_ref::<Object>() { _d.putIfAbsent(Object::from_any(currencyCode.clone()), Object::from_any(found.clone()))? } else if let Some(__f) = Currency::instances().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object, Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(currencyCode.clone()), Object::from_any(found.clone()))? } else { Default::default() };
            instance = (_vdispatch6).downcast::<Currency>();
            Ok((if !_is_jnull(&instance) { instance } else { found }))
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/util/Currency;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/util/Locale;)Ljava/util/Currency;
        pub fn getInstance_locale(mut locale: Locale) -> Result<Currency> {
            let _t0 = locale.getUnicodeLocaleType(Clone::clone(&String::from("cu")))?;
            let mut override_: String = _t0;
            if !_is_jnull(&override_) {
                let _t1 = override_.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
                let _t2: Currency = Currency::getInstance_str(Clone::clone(&_t1))?;
                return Ok(_t2);
                let mut country = (panic!("stack underflow") as i32);
            }
            let _t1: Locale = CalendarDataUtility::findRegionOverride(Clone::clone(&locale))?;
            let _t2 = _t1.getCountry()?;
            let mut country: String = _t2;
            let _t3 = country.matches(Clone::clone(&String::from("^[a-zA-Z]{2}$")))?;
            if !(_t3) {
                let mut _arr4: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr4.borrow_mut()[0i32 as usize] = Object::from_any(locale.clone());
                let _t5 = String::from("The country of the input locale: \"%s\" is not a valid ISO 3166 country code").formatted(Clone::clone(&_arr4))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t4 = country.charAt(0i32)?;
            let mut char1: u16 = _t4;
            let _t5 = country.charAt(1i32)?;
            let mut char2: u16 = _t5;
            let _t6: i32 = Currency::getMainTableEntry(char1, char2)?;
            let mut tableEntry: i32 = _t6;
            if tableEntry != 127i32 {
                let mut finalChar = ((((tableEntry&31i32)).wrapping_add(65i32)) as u16 as i32);
                let mut defaultFractionDigits = ((tableEntry&480i32)>>((5i32&0x1f)));
                let mut numericCode = ((tableEntry&1047552i32)>>((10i32&0x1f)));
                let mut sb = StringBuilder::new_str(Clone::clone(&country))?;
                let _t7 = sb.append_c(((finalChar) as u16))?;
                let _t8 = sb.toString()?;
                let _t9: Currency = Currency::getInstance_str_i_i(Clone::clone(&_t8), defaultFractionDigits, numericCode)?;
                return Ok(_t9);
            }
            if tableEntry == 127i32 {
                let mut _arr7: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
                _arr7.borrow_mut()[0i32 as usize] = Object::from_any(locale.clone());
                let _t8 = String::from("The country of the input locale: \"%s\" is not a valid ISO 3166 country code").formatted(Clone::clone(&_arr7))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if tableEntry == 512i32 {
                return Ok(Default::default());
            }
            let _t7: i32 = Currency_SpecialCaseEntry::toIndex(tableEntry)?;
            let mut finalChar: i32 = _t7;
            let _vdispatch8: Object = if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<AbstractList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<LinkedList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<ArrayList<Object>>() { _d.get(finalChar)? } else if let Some(_d) = Currency::specialCasesList().0.as_any().downcast_ref::<Object>() { _d.get(finalChar)? } else if let Some(__f) = Currency::specialCasesList().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<Object>>>() { (__f)(finalChar)? } else { Default::default() };
            let mut defaultFractionDigits = (_vdispatch8).downcast::<Currency_SpecialCaseEntry>();
            let _t9: i64 = System::currentTimeMillis()?;
            if (((_t9>(defaultFractionDigits.__get_cutOverTime())) as i32-((_t9)<(defaultFractionDigits.__get_cutOverTime())) as i32)<0) {
                let _t10: Currency = Currency::getInstance_str_i_i(Clone::clone(&defaultFractionDigits.__get_oldCurrency()), defaultFractionDigits.__get_oldCurrencyFraction(), defaultFractionDigits.__get_oldCurrencyNumericCode())?;
                return Ok(_t10);
            }
            let _t10: Currency = Currency::getInstance_str_i_i(Clone::clone(&defaultFractionDigits.__get_newCurrency()), defaultFractionDigits.__get_newCurrencyFraction(), defaultFractionDigits.__get_newCurrencyNumericCode())?;
            Ok(_t10)
        }

        #[java_method(name = "getAvailableCurrencies", descriptor = "()Ljava/util/Set;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "()Ljava/util/Set<Ljava/util/Currency;>;")]
        pub fn getAvailableCurrencies() -> Result<Object> {
            panic!("stub: java/util/Currency.getAvailableCurrencies:()Ljava/util/Set;")
        }

        #[java_method(name = "getCurrencyCode", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrencyCode(&self) -> Result<String> {
            let this = self;
            Ok(this.__get_currencyCode())
        }

        #[java_method(name = "getSymbol", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getSymbol(&self) -> Result<String> {
            panic!("stub: java/util/Currency.getSymbol:()Ljava/lang/String;")
        }

        #[java_method(name = "getSymbol", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getSymbol(Ljava/util/Locale;)Ljava/lang/String;
        pub fn getSymbol_locale(&self, mut locale: Locale) -> Result<String> {
            let this = self;
            let _t0: LocaleServiceProviderPool = LocaleServiceProviderPool::getPool(Default::default())?;
            let mut pool: LocaleServiceProviderPool = _t0;
            let _t1: Locale = CalendarDataUtility::findRegionOverride(Clone::clone(&locale))?;
            locale = _t1;
            let mut _arr2: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 1i32 as usize]));
            _arr2.borrow_mut()[0i32 as usize] = 0i32.into();
            let _t3 = pool.getLocalizedObject_locale_locale_str_arr_obj(Clone::clone(&Currency_CurrencyNameGetter::INSTANCE()).into(), Clone::clone(&locale), Clone::clone(&this.__get_currencyCode()), Clone::clone(&_arr2))?;
            let mut symbol = (_t3).downcast::<String>();
            if !_is_jnull(&symbol) {
                return Ok(symbol);
            }
            Ok(this.__get_currencyCode())
        }

        #[java_method(name = "getDefaultFractionDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDefaultFractionDigits(&self) -> Result<i32> {
            panic!("stub: java/util/Currency.getDefaultFractionDigits:()I")
        }

        #[java_method(name = "getNumericCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericCode(&self) -> Result<i32> {
            panic!("stub: java/util/Currency.getNumericCode:()I")
        }

        #[java_method(name = "getNumericCodeAsString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumericCodeAsString(&self) -> Result<String> {
            panic!("stub: java/util/Currency.getNumericCodeAsString:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName(&self) -> Result<String> {
            panic!("stub: java/util/Currency.getDisplayName:()Ljava/lang/String;")
        }

        #[java_method(name = "getDisplayName", descriptor = "(Ljava/util/Locale;)Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDisplayName_locale(&self, locale: Locale) -> Result<String> {
            panic!("stub: java/util/Currency.getDisplayName:(Ljava/util/Locale;)Ljava/lang/String;")
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            Ok(String::from(Self::BINARY_NAME))
        }

        #[java_method(name = "readResolve", descriptor = "()Ljava/lang/Object;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn readResolve(&self) -> Result<Object> {
            panic!("stub: java/util/Currency.readResolve:()Ljava/lang/Object;")
        }

        #[java_method(name = "getMainTableEntry", descriptor = "(CC)I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMainTableEntry(mut char1: u16, mut char2: u16) -> Result<i32> {
            if (char2 as i32) > 90i32 {
                let mut _arr0: Rc<RefCell<Vec<Object>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
                let _t1: Character = Character::valueOf(char1)?;
                _arr0.borrow_mut()[0i32 as usize] = Object::from_any(_t1.clone());
                let _t2: Character = Character::valueOf(char2)?;
                _arr0.borrow_mut()[1i32 as usize] = Object::from_any(_t2.clone());
                let _t3 = String::from("The country code: \"%c%c\" is not a valid ISO 3166 code").formatted(Clone::clone(&_arr0))?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(Currency::mainTable().borrow()[((((char1 as i32)).wrapping_sub(65i32)).wrapping_mul(26i32)).wrapping_add(((char2 as i32)).wrapping_sub(65i32)) as usize])
        }

        #[java_method(name = "setMainTableEntry", descriptor = "(CCI)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMainTableEntry(char1: u16, char2: u16, entry: i32) -> Result<()> {
            panic!("stub: java/util/Currency.setMainTableEntry:(CCI)V")
        }

        #[java_method(name = "readIntArray", descriptor = "(Ljava/io/DataInputStream;I)[I", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn readIntArray(dis: DataInputStream, count: i32) -> Result<Rc<RefCell<Vec<i32>>>> {
            panic!("stub: java/util/Currency.readIntArray:(Ljava/io/DataInputStream;I)[I")
        }

        #[java_method(name = "readSpecialCases", descriptor = "(Ljava/io/DataInputStream;I)Ljava/util/List;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/io/DataInputStream;I)Ljava/util/List<Ljava/util/Currency$SpecialCaseEntry;>;")]
        pub fn readSpecialCases(dis: DataInputStream, count: i32) -> Result<Object> {
            panic!("stub: java/util/Currency.readSpecialCases:(Ljava/io/DataInputStream;I)Ljava/util/List;")
        }

        #[java_method(name = "readOtherCurrencies", descriptor = "(Ljava/io/DataInputStream;I)Ljava/util/List;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException", generic_signature = "(Ljava/io/DataInputStream;I)Ljava/util/List<Ljava/util/Currency$OtherCurrencyEntry;>;")]
        pub fn readOtherCurrencies(dis: DataInputStream, count: i32) -> Result<Object> {
            panic!("stub: java/util/Currency.readOtherCurrencies:(Ljava/io/DataInputStream;I)Ljava/util/List;")
        }

        #[java_method(name = "getValidCurrencyData", descriptor = "(Ljava/util/Properties;Ljava/util/regex/Pattern;)Ljava/util/List;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/util/Properties;Ljava/util/regex/Pattern;)Ljava/util/List<Ljava/util/Currency$CurrencyProperty;>;")]
        pub fn getValidCurrencyData(props: Properties, pattern: Pattern) -> Result<Object> {
            panic!("stub: java/util/Currency.getValidCurrencyData:(Ljava/util/Properties;Ljava/util/regex/Pattern;)Ljava/util/List;")
        }

        #[java_method(name = "replaceCurrencyData", descriptor = "(Ljava/util/Currency$CurrencyProperty;)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn replaceCurrencyData(prop: Object) -> Result<()> {
            panic!("stub: java/util/Currency.replaceCurrencyData:(Ljava/util/Currency$CurrencyProperty;)V")
        }

        #[java_method(name = "updateMainTableEntry", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn updateMainTableEntry(code: String, fraction: i32, numeric: i32) -> Result<()> {
            panic!("stub: java/util/Currency.updateMainTableEntry:(Ljava/lang/String;II)V")
        }
    }
}
