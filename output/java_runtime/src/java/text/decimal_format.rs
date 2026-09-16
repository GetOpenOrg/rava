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

impl From<DecimalFormat> for NumberFormat {
    fn from(v: DecimalFormat) -> NumberFormat { v.__into_super() }
}

impl From<DecimalFormat> for Format {
    fn from(v: DecimalFormat) -> Format { v.__into_super().__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/text/DecimalFormat"]
    #[super_class       = "java/text/NumberFormat"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "DecimalFormat.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/text/Format$FieldDelegate:java/text/Format:FieldDelegate:1544;java/text/NumberFormat$Field:java/text/NumberFormat:Field:9;java/text/Format$Field:java/text/Format:Field:9;java/text/DecimalFormat$FastPathData:java/text/DecimalFormat:FastPathData:10;java/text/DecimalFormat$DigitArrays:java/text/DecimalFormat:DigitArrays:10"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "NumberFormat"]
    #[superclass_fields(groupingUsed: bool, maxIntegerDigits: i8, minIntegerDigits: i8, maxFractionDigits: i8, minFractionDigits: i8, parseIntegerOnly: bool, maximumIntegerDigits: i32, minimumIntegerDigits: i32, maximumFractionDigits: i32, minimumFractionDigits: i32, serialVersionOnStream: i32)]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/text/DecimalFormat;java/text/Format;java/text/NumberFormat"]
    #[has_hash_code_method = true]

    pub struct DecimalFormat {
        #[cfg_attr(any(), java_field(name = "bigIntegerMultiplier", descriptor = "Ljava/math/BigInteger;", access = "private", modifiers = "transient", is_static = false))]
        pub bigIntegerMultiplier: BigInteger,
        #[cfg_attr(any(), java_field(name = "bigDecimalMultiplier", descriptor = "Ljava/math/BigDecimal;", access = "private", modifiers = "transient", is_static = false))]
        pub bigDecimalMultiplier: BigDecimal,
        #[cfg_attr(any(), java_field(name = "digitList", descriptor = "Ljava/text/DigitList;", access = "private", modifiers = "transient", is_static = false))]
        pub digitList: DigitList,
        #[cfg_attr(any(), java_field(name = "positivePrefix", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub positivePrefix: String,
        #[cfg_attr(any(), java_field(name = "positiveSuffix", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub positiveSuffix: String,
        #[cfg_attr(any(), java_field(name = "negativePrefix", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub negativePrefix: String,
        #[cfg_attr(any(), java_field(name = "negativeSuffix", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub negativeSuffix: String,
        #[cfg_attr(any(), java_field(name = "posPrefixPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub posPrefixPattern: String,
        #[cfg_attr(any(), java_field(name = "posSuffixPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub posSuffixPattern: String,
        #[cfg_attr(any(), java_field(name = "negPrefixPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub negPrefixPattern: String,
        #[cfg_attr(any(), java_field(name = "negSuffixPattern", descriptor = "Ljava/lang/String;", access = "private", modifiers = "", is_static = false))]
        pub negSuffixPattern: String,
        #[cfg_attr(any(), java_field(name = "multiplier", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub multiplier: i32,
        #[cfg_attr(any(), java_field(name = "groupingSize", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub groupingSize: i8,
        #[cfg_attr(any(), java_field(name = "decimalSeparatorAlwaysShown", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub decimalSeparatorAlwaysShown: bool,
        #[cfg_attr(any(), java_field(name = "parseBigDecimal", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub parseBigDecimal: bool,
        #[cfg_attr(any(), java_field(name = "isCurrencyFormat", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub isCurrencyFormat: bool,
        #[cfg_attr(any(), java_field(name = "symbols", descriptor = "Ljava/text/DecimalFormatSymbols;", access = "private", modifiers = "", is_static = false))]
        pub symbols: DecimalFormatSymbols,
        #[cfg_attr(any(), java_field(name = "useExponentialNotation", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub useExponentialNotation: bool,
        #[cfg_attr(any(), java_field(name = "positivePrefixFieldPositions", descriptor = "[Ljava/text/FieldPosition;", access = "private", modifiers = "transient", is_static = false))]
        pub positivePrefixFieldPositions: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "positiveSuffixFieldPositions", descriptor = "[Ljava/text/FieldPosition;", access = "private", modifiers = "transient", is_static = false))]
        pub positiveSuffixFieldPositions: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "negativePrefixFieldPositions", descriptor = "[Ljava/text/FieldPosition;", access = "private", modifiers = "transient", is_static = false))]
        pub negativePrefixFieldPositions: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "negativeSuffixFieldPositions", descriptor = "[Ljava/text/FieldPosition;", access = "private", modifiers = "transient", is_static = false))]
        pub negativeSuffixFieldPositions: Rc<RefCell<Vec<Object>>>,
        #[cfg_attr(any(), java_field(name = "minExponentDigits", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub minExponentDigits: i8,
        #[cfg_attr(any(), java_field(name = "maximumIntegerDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maximumIntegerDigits: i32,
        #[cfg_attr(any(), java_field(name = "minimumIntegerDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minimumIntegerDigits: i32,
        #[cfg_attr(any(), java_field(name = "maximumFractionDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maximumFractionDigits: i32,
        #[cfg_attr(any(), java_field(name = "minimumFractionDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minimumFractionDigits: i32,
        #[cfg_attr(any(), java_field(name = "roundingMode", descriptor = "Ljava/math/RoundingMode;", access = "private", modifiers = "", is_static = false))]
        pub roundingMode: RoundingMode,
        #[cfg_attr(any(), java_field(name = "isFastPath", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub isFastPath: bool,
        #[cfg_attr(any(), java_field(name = "fastPathCheckNeeded", descriptor = "Z", access = "private", modifiers = "transient", is_static = false))]
        pub fastPathCheckNeeded: bool,
        #[cfg_attr(any(), java_field(name = "fastPathData", descriptor = "Ljava/text/DecimalFormat$FastPathData;", access = "private", modifiers = "transient", is_static = false))]
        pub fastPathData: Object,
        #[cfg_attr(any(), java_field(name = "serialVersionOnStream", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub serialVersionOnStream: i32,
    }

    impl DecimalFormat {
        #[cfg_attr(any(), java_field(name = "STATUS_INFINITE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: STATUS_INFINITE:I
        pub fn STATUS_INFINITE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "STATUS_POSITIVE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: STATUS_POSITIVE:I
        pub fn STATUS_POSITIVE() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "STATUS_LENGTH", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: STATUS_LENGTH:I
        pub fn STATUS_LENGTH() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "currentSerialVersion", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: currentSerialVersion:I
        pub fn currentSerialVersion() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "MAX_INT_AS_DOUBLE", descriptor = "D", access = "private", modifiers = "static final", is_static = true, constant_value = "2147483647.0"))]
        // static field: MAX_INT_AS_DOUBLE:D
        pub fn MAX_INT_AS_DOUBLE() -> f64 {
            2147483647.0f64
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_ZERO_DIGIT", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "48"))]
        // static field: PATTERN_ZERO_DIGIT:C
        pub fn PATTERN_ZERO_DIGIT() -> u16 {
            48
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_GROUPING_SEPARATOR", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "44"))]
        // static field: PATTERN_GROUPING_SEPARATOR:C
        pub fn PATTERN_GROUPING_SEPARATOR() -> u16 {
            44
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DECIMAL_SEPARATOR", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "46"))]
        // static field: PATTERN_DECIMAL_SEPARATOR:C
        pub fn PATTERN_DECIMAL_SEPARATOR() -> u16 {
            46
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_PER_MILLE", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "8240"))]
        // static field: PATTERN_PER_MILLE:C
        pub fn PATTERN_PER_MILLE() -> u16 {
            8240
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_PERCENT", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "37"))]
        // static field: PATTERN_PERCENT:C
        pub fn PATTERN_PERCENT() -> u16 {
            37
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_DIGIT", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "35"))]
        // static field: PATTERN_DIGIT:C
        pub fn PATTERN_DIGIT() -> u16 {
            35
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_SEPARATOR", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "59"))]
        // static field: PATTERN_SEPARATOR:C
        pub fn PATTERN_SEPARATOR() -> u16 {
            59
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_EXPONENT", descriptor = "Ljava/lang/String;", access = "private", modifiers = "static final", is_static = true, constant_value = "E"))]
        // static field: PATTERN_EXPONENT:Ljava/lang/String;
        pub fn PATTERN_EXPONENT() -> String {
            String::from("E")
        }

        #[cfg_attr(any(), java_field(name = "PATTERN_MINUS", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "45"))]
        // static field: PATTERN_MINUS:C
        pub fn PATTERN_MINUS() -> u16 {
            45
        }

        #[cfg_attr(any(), java_field(name = "CURRENCY_SIGN", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "164"))]
        // static field: CURRENCY_SIGN:C
        pub fn CURRENCY_SIGN() -> u16 {
            164
        }

        #[cfg_attr(any(), java_field(name = "QUOTE", descriptor = "C", access = "private", modifiers = "static final", is_static = true, constant_value = "39"))]
        // static field: QUOTE:C
        pub fn QUOTE() -> u16 {
            39
        }

        #[cfg_attr(any(), java_field(name = "EmptyFieldPositionArray", descriptor = "[Ljava/text/FieldPosition;", access = "private", modifiers = "static", is_static = true))]
        // static field: EmptyFieldPositionArray:[Ljava/text/FieldPosition;
        pub fn EmptyFieldPositionArray() -> Rc<RefCell<Vec<Object>>> {
            Rc::new(RefCell::new(Vec::new()))
        }

        #[cfg_attr(any(), java_field(name = "DOUBLE_INTEGER_DIGITS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "309"))]
        // static field: DOUBLE_INTEGER_DIGITS:I
        pub fn DOUBLE_INTEGER_DIGITS() -> i32 {
            309
        }

        #[cfg_attr(any(), java_field(name = "DOUBLE_FRACTION_DIGITS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "340"))]
        // static field: DOUBLE_FRACTION_DIGITS:I
        pub fn DOUBLE_FRACTION_DIGITS() -> i32 {
            340
        }

        #[cfg_attr(any(), java_field(name = "MAXIMUM_INTEGER_DIGITS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: MAXIMUM_INTEGER_DIGITS:I
        pub fn MAXIMUM_INTEGER_DIGITS() -> i32 {
            2147483647
        }

        #[cfg_attr(any(), java_field(name = "MAXIMUM_FRACTION_DIGITS", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "2147483647"))]
        // static field: MAXIMUM_FRACTION_DIGITS:I
        pub fn MAXIMUM_FRACTION_DIGITS() -> i32 {
            2147483647
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "864413376551465018"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            864413376551465018i64
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(NumberFormat::new()?);
            this.__set_digitList(Clone::clone(&DigitList::new()?));
            this.__set_positivePrefix(Clone::clone(&String::from("")));
            this.__set_positiveSuffix(Clone::clone(&String::from("")));
            this.__set_negativePrefix(Clone::clone(&String::from("-")));
            this.__set_negativeSuffix(Clone::clone(&String::from("")));
            this.__set_multiplier(1i32);
            this.__set_groupingSize(((3i32) as i8));
            this.__set_decimalSeparatorAlwaysShown((0i32 != 0i32));
            this.__set_parseBigDecimal((0i32 != 0i32));
            this.__set_isCurrencyFormat((0i32 != 0i32));
            this.__set_symbols(Default::default());
            let _t0 = this.__super().getMaximumIntegerDigits()?;
            this.__set_maximumIntegerDigits(_t0);
            let _t1 = this.__super().getMinimumIntegerDigits()?;
            this.__set_minimumIntegerDigits(_t1);
            let _t2 = this.__super().getMaximumFractionDigits()?;
            this.__set_maximumFractionDigits(_t2);
            let _t3 = this.__super().getMinimumFractionDigits()?;
            this.__set_minimumFractionDigits(_t3);
            this.__set_roundingMode(Clone::clone(&RoundingMode::HALF_EVEN()));
            this.__set_isFastPath((0i32 != 0i32));
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            this.__set_serialVersionOnStream(4i32);
            let _t4: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            let mut def: Locale = _t4;
            let _t5: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&def))?;
            let mut adapter: LocaleProviderAdapter = _t5;
            if !(false) {
                let _t6: LocaleProviderAdapter = LocaleProviderAdapter::getResourceBundleBased()?;
                adapter = _t6;
            }
            let _t6 = adapter.getLocaleResources(Clone::clone(&def))?;
            let _t7 = _t6.getNumberPatterns()?;
            let mut all: Rc<RefCell<Vec<String>>> = _t7;
            let _t8: DecimalFormatSymbols = DecimalFormatSymbols::getInstance_locale(Clone::clone(&def))?;
            this.__set_symbols(Clone::clone(&_t8));
            this.applyPattern_str_z(Clone::clone(&Clone::clone(&all.borrow()[0i32 as usize])), (0i32 != 0i32))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_str(pattern: String) -> Result<Self> {
            panic!("stub: java/text/DecimalFormat.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/text/DecimalFormatSymbols;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;Ljava/text/DecimalFormatSymbols;)V
        pub fn new_str_decima(mut pattern: String, mut symbols: DecimalFormatSymbols) -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(NumberFormat::new()?);
            this.__set_digitList(Clone::clone(&DigitList::new()?));
            this.__set_positivePrefix(Clone::clone(&String::from("")));
            this.__set_positiveSuffix(Clone::clone(&String::from("")));
            this.__set_negativePrefix(Clone::clone(&String::from("-")));
            this.__set_negativeSuffix(Clone::clone(&String::from("")));
            this.__set_multiplier(1i32);
            this.__set_groupingSize(((3i32) as i8));
            this.__set_decimalSeparatorAlwaysShown((0i32 != 0i32));
            this.__set_parseBigDecimal((0i32 != 0i32));
            this.__set_isCurrencyFormat((0i32 != 0i32));
            this.__set_symbols(Default::default());
            let _t0 = this.__super().getMaximumIntegerDigits()?;
            this.__set_maximumIntegerDigits(_t0);
            let _t1 = this.__super().getMinimumIntegerDigits()?;
            this.__set_minimumIntegerDigits(_t1);
            let _t2 = this.__super().getMaximumFractionDigits()?;
            this.__set_maximumFractionDigits(_t2);
            let _t3 = this.__super().getMinimumFractionDigits()?;
            this.__set_minimumFractionDigits(_t3);
            this.__set_roundingMode(Clone::clone(&RoundingMode::HALF_EVEN()));
            this.__set_isFastPath((0i32 != 0i32));
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            this.__set_serialVersionOnStream(4i32);
            let _t4: Object = Object::from_any(symbols.clone());
            this.__set_symbols(Clone::clone(&(_t4).downcast::<DecimalFormatSymbols>()));
            this.applyPattern_str_z(Clone::clone(&pattern), (0i32 != 0i32))?;
            Ok(this)
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_obj_string_fieldp(&self, number: Object, toAppendTo: Object, pos: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_d_string_fieldp(&self, number: f64, arg1: Object, result: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_d_string_format(&self, number: f64, arg1: Object, result: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "handleNaN", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn handleNaN(&self, number: f64, arg1: Object, result: Object) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.handleNaN:(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Z")
        }

        #[java_method(name = "handleInfinity", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Z", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn handleInfinity(&self, number: f64, arg1: Object, result: Object, delegate: bool) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.handleInfinity:(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Z")
        }

        #[java_method(name = "doubleSubformat", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Ljava/lang/StringBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn doubleSubformat(&self, number: f64, arg1: Object, result: Object, delegate: bool) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.doubleSubformat:(DLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_l_string_fieldp(&self, number: i64, arg1: Object, result: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(JLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_l_string_format(&self, number: i64, arg1: Object, result: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(JLjava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/math/BigDecimal;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_bigdec_string_fieldp(&self, number: BigDecimal, result: Object, fieldPosition: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(Ljava/math/BigDecimal;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/math/BigDecimal;Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_bigdec_string_format(&self, number: BigDecimal, result: Object, delegate: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(Ljava/math/BigDecimal;Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/math/BigInteger;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_bigint_string_fieldp(&self, number: BigInteger, result: Object, fieldPosition: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(Ljava/math/BigInteger;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/math/BigInteger;Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Ljava/lang/StringBuffer;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_bigint_string_format_z(&self, number: BigInteger, result: Object, delegate: Object, formatLong: bool) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.format:(Ljava/math/BigInteger;Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;Z)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "formatToCharacterIterator", descriptor = "(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn formatToCharacterIterator(&self, obj: Object) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.formatToCharacterIterator:(Ljava/lang/Object;)Ljava/text/AttributedCharacterIterator;")
        }

        #[java_method(name = "checkAndSetFastPathStatus", descriptor = "()Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkAndSetFastPathStatus(&self) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.checkAndSetFastPathStatus:()Z")
        }

        #[java_method(name = "resetFastPathData", descriptor = "(Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn resetFastPathData(&self, fastPathWasOn: bool) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.resetFastPathData:(Z)V")
        }

        #[java_method(name = "exactRoundUp", descriptor = "(DI)Z", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn exactRoundUp(&self, fractionalPart: f64, arg1: i32) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.exactRoundUp:(DI)Z")
        }

        #[java_method(name = "collectIntegralDigits", descriptor = "(I[CI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn collectIntegralDigits(&self, number: i32, digitsBuffer: Rc<RefCell<Vec<u16>>>, backwardIndex: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.collectIntegralDigits:(I[CI)V")
        }

        #[java_method(name = "collectFractionalDigits", descriptor = "(I[CI)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn collectFractionalDigits(&self, number: i32, digitsBuffer: Rc<RefCell<Vec<u16>>>, startIndex: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.collectFractionalDigits:(I[CI)V")
        }

        #[java_method(name = "addAffixes", descriptor = "([C[C[C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addAffixes(&self, container: Rc<RefCell<Vec<u16>>>, prefix: Rc<RefCell<Vec<u16>>>, suffix: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.addAffixes:([C[C[C)V")
        }

        #[java_method(name = "prependPrefix", descriptor = "([CI[C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn prependPrefix(&self, prefix: Rc<RefCell<Vec<u16>>>, len: i32, container: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.prependPrefix:([CI[C)V")
        }

        #[java_method(name = "appendSuffix", descriptor = "([CI[C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendSuffix(&self, suffix: Rc<RefCell<Vec<u16>>>, len: i32, container: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.appendSuffix:([CI[C)V")
        }

        #[java_method(name = "localizeDigits", descriptor = "([C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn localizeDigits(&self, digitsBuffer: Rc<RefCell<Vec<u16>>>) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.localizeDigits:([C)V")
        }

        #[java_method(name = "fastDoubleFormat", descriptor = "(DZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fastDoubleFormat(&self, d: f64, arg1: bool) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.fastDoubleFormat:(DZ)V")
        }

        #[java_method(name = "fastFormat", descriptor = "(D)Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fastFormat(&self, d: f64) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.fastFormat:(D)Ljava/lang/String;")
        }

        #[java_method(name = "setDigitList", descriptor = "(Ljava/lang/Number;ZI)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDigitList(&self, number: Number, isNegative: bool, maxDigits: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setDigitList:(Ljava/lang/Number;ZI)V")
        }

        #[java_method(name = "subformat", descriptor = "(Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;ZZIIII)Ljava/lang/StringBuffer;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subformat(&self, result: Object, delegate: Object, isNegative: bool, isInteger: bool, maxIntDigits: i32, minIntDigits: i32, maxFraDigits: i32, minFraDigits: i32) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.subformat:(Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;ZZIIII)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "subformatNumber", descriptor = "(Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;ZZIIII)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subformatNumber(&self, result: Object, delegate: Object, isNegative: bool, isInteger: bool, maxIntDigits: i32, minIntDigits: i32, maxFraDigits: i32, minFraDigits: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.subformatNumber:(Ljava/lang/StringBuffer;Ljava/text/Format$FieldDelegate;ZZIIII)V")
        }

        #[java_method(name = "append", descriptor = "(Ljava/lang/StringBuffer;Ljava/lang/String;Ljava/text/Format$FieldDelegate;[Ljava/text/FieldPosition;Ljava/text/Format$Field;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn append(&self, result: Object, string: String, delegate: Object, positions: Rc<RefCell<Vec<Object>>>, signAttribute: Object) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.append:(Ljava/lang/StringBuffer;Ljava/lang/String;Ljava/text/Format$FieldDelegate;[Ljava/text/FieldPosition;Ljava/text/Format$Field;)V")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parse(&self, text: String, pos: Object) -> Result<Number> {
            panic!("stub: java/text/DecimalFormat.parse:(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;")
        }

        #[java_method(name = "getBigIntegerMultiplier", descriptor = "()Ljava/math/BigInteger;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBigIntegerMultiplier(&self) -> Result<BigInteger> {
            panic!("stub: java/text/DecimalFormat.getBigIntegerMultiplier:()Ljava/math/BigInteger;")
        }

        #[java_method(name = "getBigDecimalMultiplier", descriptor = "()Ljava/math/BigDecimal;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getBigDecimalMultiplier(&self) -> Result<BigDecimal> {
            panic!("stub: java/text/DecimalFormat.getBigDecimalMultiplier:()Ljava/math/BigDecimal;")
        }

        #[java_method(name = "subparse", descriptor = "(Ljava/lang/String;Ljava/text/ParsePosition;Ljava/lang/String;Ljava/lang/String;Ljava/text/DigitList;Z[Z)Z", access = "private", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subparse(&self, text: String, parsePosition: Object, positivePrefix: String, negativePrefix: String, digits: DigitList, isExponent: bool, status: Rc<RefCell<Vec<bool>>>) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.subparse:(Ljava/lang/String;Ljava/text/ParsePosition;Ljava/lang/String;Ljava/lang/String;Ljava/text/DigitList;Z[Z)Z")
        }

        #[java_method(name = "subparseNumber", descriptor = "(Ljava/lang/String;ILjava/text/DigitList;ZZ[Z)I", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn subparseNumber(&self, text: String, position: i32, digits: DigitList, checkExponent: bool, isExponent: bool, status: Rc<RefCell<Vec<bool>>>) -> Result<i32> {
            panic!("stub: java/text/DecimalFormat.subparseNumber:(Ljava/lang/String;ILjava/text/DigitList;ZZ[Z)I")
        }

        #[java_method(name = "getDecimalFormatSymbols", descriptor = "()Ljava/text/DecimalFormatSymbols;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecimalFormatSymbols(&self) -> Result<DecimalFormatSymbols> {
            panic!("stub: java/text/DecimalFormat.getDecimalFormatSymbols:()Ljava/text/DecimalFormatSymbols;")
        }

        #[java_method(name = "setDecimalFormatSymbols", descriptor = "(Ljava/text/DecimalFormatSymbols;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDecimalFormatSymbols(&self, newSymbols: DecimalFormatSymbols) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setDecimalFormatSymbols:(Ljava/text/DecimalFormatSymbols;)V")
        }

        #[java_method(name = "getPositivePrefix", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPositivePrefix(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.getPositivePrefix:()Ljava/lang/String;")
        }

        #[java_method(name = "setPositivePrefix", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPositivePrefix(&self, newValue: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setPositivePrefix:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getPositivePrefixFieldPositions", descriptor = "()[Ljava/text/FieldPosition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPositivePrefixFieldPositions(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/text/DecimalFormat.getPositivePrefixFieldPositions:()[Ljava/text/FieldPosition;")
        }

        #[java_method(name = "getNegativePrefix", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNegativePrefix(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.getNegativePrefix:()Ljava/lang/String;")
        }

        #[java_method(name = "setNegativePrefix", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setNegativePrefix(&self, newValue: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setNegativePrefix:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getNegativePrefixFieldPositions", descriptor = "()[Ljava/text/FieldPosition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNegativePrefixFieldPositions(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/text/DecimalFormat.getNegativePrefixFieldPositions:()[Ljava/text/FieldPosition;")
        }

        #[java_method(name = "getPositiveSuffix", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPositiveSuffix(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.getPositiveSuffix:()Ljava/lang/String;")
        }

        #[java_method(name = "setPositiveSuffix", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setPositiveSuffix(&self, newValue: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setPositiveSuffix:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getPositiveSuffixFieldPositions", descriptor = "()[Ljava/text/FieldPosition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPositiveSuffixFieldPositions(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/text/DecimalFormat.getPositiveSuffixFieldPositions:()[Ljava/text/FieldPosition;")
        }

        #[java_method(name = "getNegativeSuffix", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNegativeSuffix(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.getNegativeSuffix:()Ljava/lang/String;")
        }

        #[java_method(name = "setNegativeSuffix", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setNegativeSuffix(&self, newValue: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setNegativeSuffix:(Ljava/lang/String;)V")
        }

        #[java_method(name = "getNegativeSuffixFieldPositions", descriptor = "()[Ljava/text/FieldPosition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNegativeSuffixFieldPositions(&self) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/text/DecimalFormat.getNegativeSuffixFieldPositions:()[Ljava/text/FieldPosition;")
        }

        #[java_method(name = "getMultiplier", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMultiplier(&self) -> Result<i32> {
            panic!("stub: java/text/DecimalFormat.getMultiplier:()I")
        }

        #[java_method(name = "setMultiplier", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMultiplier(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setMultiplier:(I)V")
        }

        #[java_method(name = "setGroupingUsed", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGroupingUsed(&self, mut newValue: bool) -> Result<()> {
            let this = self;
            this.__super().setGroupingUsed(newValue)?;
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "getGroupingSize", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGroupingSize(&self) -> Result<i32> {
            let this = self;
            Ok((this.__get_groupingSize()) as i32)
        }

        #[java_method(name = "setGroupingSize", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGroupingSize(&self, newValue: i32) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setGroupingSize:(I)V")
        }

        #[java_method(name = "isDecimalSeparatorAlwaysShown", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isDecimalSeparatorAlwaysShown(&self) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.isDecimalSeparatorAlwaysShown:()Z")
        }

        #[java_method(name = "setDecimalSeparatorAlwaysShown", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setDecimalSeparatorAlwaysShown(&self, mut newValue: bool) -> Result<()> {
            let this = self;
            this.__set_decimalSeparatorAlwaysShown(newValue);
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "isParseBigDecimal", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isParseBigDecimal(&self) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.isParseBigDecimal:()Z")
        }

        #[java_method(name = "setParseBigDecimal", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setParseBigDecimal(&self, newValue: bool) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setParseBigDecimal:(Z)V")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/DecimalFormat.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/DecimalFormat.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "toPattern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toPattern(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.toPattern:()Ljava/lang/String;")
        }

        #[java_method(name = "toLocalizedPattern", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toLocalizedPattern(&self) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.toLocalizedPattern:()Ljava/lang/String;")
        }

        #[java_method(name = "expandAffixes", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expandAffixes(&self) -> Result<()> {
            let this = self;
            let mut buffer = StringBuilder::new()?;
            if !_is_jnull(&this.__get_posPrefixPattern()) {
                let _t0 = this.expandAffix_str_sb(Clone::clone(&this.__get_posPrefixPattern()), Clone::clone(&buffer))?;
                this.__set_positivePrefix(Clone::clone(&_t0));
                this.__set_positivePrefixFieldPositions(Default::default());
            }
            if !_is_jnull(&this.__get_posSuffixPattern()) {
                let _t0 = this.expandAffix_str_sb(Clone::clone(&this.__get_posSuffixPattern()), Clone::clone(&buffer))?;
                this.__set_positiveSuffix(Clone::clone(&_t0));
                this.__set_positiveSuffixFieldPositions(Default::default());
            }
            if !_is_jnull(&this.__get_negPrefixPattern()) {
                let _t0 = this.expandAffix_str_sb(Clone::clone(&this.__get_negPrefixPattern()), Clone::clone(&buffer))?;
                this.__set_negativePrefix(Clone::clone(&_t0));
                this.__set_negativePrefixFieldPositions(Default::default());
            }
            if !_is_jnull(&this.__get_negSuffixPattern()) {
                let _t0 = this.expandAffix_str_sb(Clone::clone(&this.__get_negSuffixPattern()), Clone::clone(&buffer))?;
                this.__set_negativeSuffix(Clone::clone(&_t0));
                this.__set_negativeSuffixFieldPositions(Default::default());
            }
            Ok(())
        }

        #[java_method(name = "expandAffix", descriptor = "(Ljava/lang/String;Ljava/lang/StringBuilder;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: expandAffix(Ljava/lang/String;Ljava/lang/StringBuilder;)Ljava/lang/String;
        pub fn expandAffix_str_sb(&self, mut pattern: String, mut buffer: StringBuilder) -> Result<String> {
            let this = self;
            buffer.__super().setLength(0i32)?;
            let mut i: i32 = 0i32;
            loop {
                let _t0 = pattern.length()?;
                if i >= _t0 { break; }
                i = i.wrapping_add(1i32);
                let _t0 = pattern.charAt(i)?;
                let mut c: u16 = _t0;
                i = i.wrapping_add(1i32);
                let _t1 = pattern.charAt(i)?;
                c = _t1;
                let _switch_key = c;
                let _t2 = pattern.length()?;
                let _t3 = pattern.charAt(i)?;
                if (_t3 as i32) == 164i32 {
                    i = i.wrapping_add(1i32);
                    let _t4 = this.__get_symbols().getInternationalCurrencySymbol()?;
                    let _t5 = buffer.append_str(Clone::clone(&_t4))?;
                    continue;
                }
                let _t4 = this.__get_symbols().getCurrencySymbol()?;
                let _t5 = buffer.append_str(Clone::clone(&_t4))?;
                continue;
                let _t6 = this.__get_symbols().getPercentText()?;
                let _t7 = buffer.append_str(Clone::clone(&_t6))?;
                continue;
                let _t8 = this.__get_symbols().getPerMillText()?;
                let _t9 = buffer.append_str(Clone::clone(&_t8))?;
                continue;
                let _t10 = this.__get_symbols().getMinusSignText()?;
                let _t11 = buffer.append_str(Clone::clone(&_t10))?;
                continue;
                let _t12 = buffer.append_c(c)?;
            }
            let _t0 = buffer.toString()?;
            Ok(_t0)
        }

        #[java_method(name = "expandAffix", descriptor = "(Ljava/lang/String;)[Ljava/text/FieldPosition;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn expandAffix_str(&self, pattern: String) -> Result<Rc<RefCell<Vec<Object>>>> {
            panic!("stub: java/text/DecimalFormat.expandAffix:(Ljava/lang/String;)[Ljava/text/FieldPosition;")
        }

        #[java_method(name = "appendAffix", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;Ljava/lang/String;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendAffix_sb_str_str_z(&self, buffer: StringBuilder, affixPattern: String, expAffix: String, localized: bool) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.appendAffix:(Ljava/lang/StringBuilder;Ljava/lang/String;Ljava/lang/String;Z)V")
        }

        #[java_method(name = "appendAffix", descriptor = "(Ljava/lang/StringBuilder;Ljava/lang/String;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn appendAffix_sb_str_z(&self, buffer: StringBuilder, affix: String, localized: bool) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.appendAffix:(Ljava/lang/StringBuilder;Ljava/lang/String;Z)V")
        }

        #[java_method(name = "toPattern", descriptor = "(Z)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toPattern_z(&self, localized: bool) -> Result<String> {
            panic!("stub: java/text/DecimalFormat.toPattern:(Z)Ljava/lang/String;")
        }

        #[java_method(name = "applyPattern", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn applyPattern_str(&self, pattern: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.applyPattern:(Ljava/lang/String;)V")
        }

        #[java_method(name = "applyLocalizedPattern", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn applyLocalizedPattern(&self, pattern: String) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.applyLocalizedPattern:(Ljava/lang/String;)V")
        }

        #[java_method(name = "applyPattern", descriptor = "(Ljava/lang/String;Z)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: applyPattern(Ljava/lang/String;Z)V
        pub fn applyPattern_str_z(&self, mut pattern: String, mut localized: bool) -> Result<()> {
            let this = self;
            let mut zeroDigit: i32 = 48i32;
            let mut groupingSeparator: i32 = 44i32;
            let mut decimalSeparator: i32 = 46i32;
            let mut percent: i32 = 37i32;
            let mut perMill: i32 = 8240i32;
            let mut digit: i32 = 35i32;
            let mut separator: i32 = 59i32;
            let mut exponent: String = String::from("E");
            let mut minus: i32 = 45i32;
            if localized {
                let _t0 = this.__get_symbols().getZeroDigit()?;
                let mut zeroDigit: u16 = _t0;
                let _t1 = this.__get_symbols().getGroupingSeparator()?;
                let mut groupingSeparator: u16 = _t1;
                let _t2 = this.__get_symbols().getDecimalSeparator()?;
                let mut decimalSeparator: u16 = _t2;
                let _t3 = this.__get_symbols().getPercent()?;
                let mut percent: u16 = _t3;
                let _t4 = this.__get_symbols().getPerMill()?;
                let mut perMill: u16 = _t4;
                let _t5 = this.__get_symbols().getDigit()?;
                let mut digit: u16 = _t5;
                let _t6 = this.__get_symbols().getPatternSeparator()?;
                let mut separator: u16 = _t6;
                let _t7 = this.__get_symbols().getExponentSeparator()?;
                exponent = _t7;
                let _t8 = this.__get_symbols().getMinusSign()?;
                let mut minus: u16 = _t8;
            }
            let mut gotNegative: i32 = 0i32;
            this.__set_decimalSeparatorAlwaysShown((0i32 != 0i32));
            this.__set_isCurrencyFormat((0i32 != 0i32));
            this.__set_useExponentialNotation((0i32 != 0i32));
            let mut start: i32 = 0i32;
            let mut j: i32 = 1i32;
            loop {
                if (j<0) { break; }
                let _t0 = pattern.length()?;
                let mut inQuote: i32 = 0i32;
                let mut prefix = StringBuilder::new()?;
                let mut suffix = StringBuilder::new()?;
                let mut decimalPos: i32 = -1i32;
                let mut multiplier: i32 = 1i32;
                let mut digitLeftCount: i32 = 0i32;
                let mut zeroDigitCount: i32 = 0i32;
                let mut digitRightCount: i32 = 0i32;
                let mut groupingCount: i32 = -1i32;
                let mut phase: i32 = 0i32;
                let mut affix: StringBuilder = prefix;
                let mut pos: i32 = start;
                loop {
                    let _t1 = pattern.length()?;
                    if pos >= _t1 { break; }
                    let _t1 = pattern.charAt(pos)?;
                    let mut ch: u16 = _t1;
                    match phase {
                        0 => {
                            if (inQuote!=0) {
                                if (ch as i32) == 39i32 {
                                    let _t2 = pattern.length()?;
                                    if (pos).wrapping_add(1i32) < _t2 {
                                        let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                        if (_t3 as i32) == 39i32 {
                                            pos = pos.wrapping_add(1i32);
                                            let _t4 = affix.append_str(Clone::clone(&String::from("''")))?;
                                        } else {
                                            inQuote = 0i32;
                                        }
                                    } else {
                                        inQuote = 0i32;
                                    }
                                } else {
                                    let _t2 = affix.append_c(ch)?;
                                }
                            } else {
                                if (ch as i32) == (decimalSeparator as i32) {
                                    phase = 1i32;
                                    pos = pos.wrapping_sub(1i32);
                                } else {
                                    if (ch as i32) == 164i32 {
                                        let _t2 = pattern.length()?;
                                        let mut _merged4: bool;
                                        if (pos).wrapping_add(1i32) < _t2 {
                                            let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                            _merged4 = (_t3 as i32) == 164i32;
                                        } else {
                                            _merged4 = (0i32 != 0);
                                        }
                                        let mut doubled = (_merged4) as i32;
                                        if (doubled!=0) {
                                            pos = pos.wrapping_add(1i32);
                                        }
                                        this.__set_isCurrencyFormat((1i32 != 0i32));
                                        let _t5 = affix.append_str(Clone::clone(&(if (doubled!=0) { String::from("'¤¤") } else { String::from("'¤") })))?;
                                    } else {
                                        if (ch as i32) == 39i32 {
                                            let _t2 = pattern.length()?;
                                            if (pos).wrapping_add(1i32) < _t2 {
                                                let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                                if (_t3 as i32) == 39i32 {
                                                    pos = pos.wrapping_add(1i32);
                                                    let _t4 = affix.append_str(Clone::clone(&String::from("''")))?;
                                                } else {
                                                    inQuote = 1i32;
                                                }
                                            } else {
                                                inQuote = 1i32;
                                            }
                                        } else {
                                            if (ch as i32) == (separator as i32) {
                                                if (j==0) {
                                                    let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unquoted special character '")))?;
                                                    let _t3 = _t2.append_c(ch)?;
                                                    let _t4 = _t3.append_str(Clone::clone(&String::from("' in pattern \"")))?;
                                                    let _t5 = _t4.append_str(Clone::clone(&pattern))?;
                                                    let _t6 = _t5.append_c(((34i32) as u16))?;
                                                    let _t7 = _t6.toString()?;
                                                    return Err(JvmError::Custom("athrow".to_owned()));
                                                }
                                                start = (pos).wrapping_add(1i32);
                                                let _t2 = pattern.length()?;
                                                pos = _t2;
                                            } else {
                                                if (ch as i32) == (percent as i32) {
                                                    if multiplier != 1i32 {
                                                        let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Too many percent/per mille characters in pattern \"")))?;
                                                        let _t3 = _t2.append_str(Clone::clone(&pattern))?;
                                                        let _t4 = _t3.append_c(((34i32) as u16))?;
                                                        let _t5 = _t4.toString()?;
                                                        return Err(JvmError::Custom("athrow".to_owned()));
                                                    }
                                                    multiplier = 100i32;
                                                    let _t2 = affix.append_str(Clone::clone(&String::from("'%")))?;
                                                } else {
                                                    if (ch as i32) == (perMill as i32) {
                                                        if multiplier != 1i32 {
                                                            let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Too many percent/per mille characters in pattern \"")))?;
                                                            let _t3 = _t2.append_str(Clone::clone(&pattern))?;
                                                            let _t4 = _t3.append_c(((34i32) as u16))?;
                                                            let _t5 = _t4.toString()?;
                                                            return Err(JvmError::Custom("athrow".to_owned()));
                                                        }
                                                        multiplier = 1000i32;
                                                        let _t2 = affix.append_str(Clone::clone(&String::from("'‰")))?;
                                                    } else {
                                                        if (ch as i32) == (minus as i32) {
                                                            let _t2 = affix.append_str(Clone::clone(&String::from("'-")))?;
                                                        } else {
                                                            let _t2 = affix.append_c(ch)?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        1 => {
                            let _t2 = pattern.length()?;
                            let _t3 = pattern.charAt(pos)?;
                            let mut doubled: u16 = _t3;
                            if (doubled as i32) == (decimalSeparator as i32) {
                                pos = pos.wrapping_add(1i32);
                            } else {
                                let _t4 = exponent.length()?;
                                let _t5 = pattern.regionMatches_i_str_i_i(pos, Clone::clone(&exponent), 0i32, _t4)?;
                                if _t5 {
                                    let _t6 = exponent.length()?;
                                    pos = (pos).wrapping_add(_t6);
                                } else {
                                    pos = pos.wrapping_sub(1i32);
                                    phase = 2i32;
                                    affix = suffix;
                                    break;
                                }
                            }
                        }
                        2 => {
                            if (inQuote!=0) {
                                if (ch as i32) == 39i32 {
                                    let _t2 = pattern.length()?;
                                    if (pos).wrapping_add(1i32) < _t2 {
                                        let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                        if (_t3 as i32) == 39i32 {
                                            pos = pos.wrapping_add(1i32);
                                            let _t4 = affix.append_str(Clone::clone(&String::from("''")))?;
                                        } else {
                                            inQuote = 0i32;
                                        }
                                    } else {
                                        inQuote = 0i32;
                                    }
                                } else {
                                    let _t2 = affix.append_c(ch)?;
                                }
                            } else {
                                if (ch as i32) == (decimalSeparator as i32) {
                                    phase = 1i32;
                                    pos = pos.wrapping_sub(1i32);
                                } else {
                                    if (ch as i32) == 164i32 {
                                        let _t2 = pattern.length()?;
                                        let mut _merged4: bool;
                                        if (pos).wrapping_add(1i32) < _t2 {
                                            let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                            _merged4 = (_t3 as i32) == 164i32;
                                        } else {
                                            _merged4 = (0i32 != 0);
                                        }
                                        let mut doubled = (_merged4) as i32;
                                        if (doubled!=0) {
                                            pos = pos.wrapping_add(1i32);
                                        }
                                        this.__set_isCurrencyFormat((1i32 != 0i32));
                                        let _t5 = affix.append_str(Clone::clone(&(if (doubled!=0) { String::from("'¤¤") } else { String::from("'¤") })))?;
                                    } else {
                                        if (ch as i32) == 39i32 {
                                            let _t2 = pattern.length()?;
                                            if (pos).wrapping_add(1i32) < _t2 {
                                                let _t3 = pattern.charAt((pos).wrapping_add(1i32))?;
                                                if (_t3 as i32) == 39i32 {
                                                    pos = pos.wrapping_add(1i32);
                                                    let _t4 = affix.append_str(Clone::clone(&String::from("''")))?;
                                                } else {
                                                    inQuote = 1i32;
                                                }
                                            } else {
                                                inQuote = 1i32;
                                            }
                                        } else {
                                            if (ch as i32) == (separator as i32) {
                                                if (j==0) {
                                                    let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Unquoted special character '")))?;
                                                    let _t3 = _t2.append_c(ch)?;
                                                    let _t4 = _t3.append_str(Clone::clone(&String::from("' in pattern \"")))?;
                                                    let _t5 = _t4.append_str(Clone::clone(&pattern))?;
                                                    let _t6 = _t5.append_c(((34i32) as u16))?;
                                                    let _t7 = _t6.toString()?;
                                                    return Err(JvmError::Custom("athrow".to_owned()));
                                                }
                                                start = (pos).wrapping_add(1i32);
                                                let _t2 = pattern.length()?;
                                                pos = _t2;
                                            } else {
                                                if (ch as i32) == (percent as i32) {
                                                    if multiplier != 1i32 {
                                                        let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Too many percent/per mille characters in pattern \"")))?;
                                                        let _t3 = _t2.append_str(Clone::clone(&pattern))?;
                                                        let _t4 = _t3.append_c(((34i32) as u16))?;
                                                        let _t5 = _t4.toString()?;
                                                        return Err(JvmError::Custom("athrow".to_owned()));
                                                    }
                                                    multiplier = 100i32;
                                                    let _t2 = affix.append_str(Clone::clone(&String::from("'%")))?;
                                                } else {
                                                    if (ch as i32) == (perMill as i32) {
                                                        if multiplier != 1i32 {
                                                            let _t2 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Too many percent/per mille characters in pattern \"")))?;
                                                            let _t3 = _t2.append_str(Clone::clone(&pattern))?;
                                                            let _t4 = _t3.append_c(((34i32) as u16))?;
                                                            let _t5 = _t4.toString()?;
                                                            return Err(JvmError::Custom("athrow".to_owned()));
                                                        }
                                                        multiplier = 1000i32;
                                                        let _t2 = affix.append_str(Clone::clone(&String::from("'‰")))?;
                                                    } else {
                                                        if (ch as i32) == (minus as i32) {
                                                            let _t2 = affix.append_str(Clone::clone(&String::from("'-")))?;
                                                        } else {
                                                            let _t2 = affix.append_c(ch)?;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                        }
                    }
                    pos = pos.wrapping_add(1i32);
                }
                pos = decimalPos;
                if (pos==0) {
                    pos = pos.wrapping_add(1i32);
                }
                digitRightCount = (digitLeftCount).wrapping_sub(pos);
                digitLeftCount = (pos).wrapping_sub(1i32);
                zeroDigitCount = 1i32;
                if (inQuote!=0) {
                    let _t1 = StringBuilder::new()?.append_str(Clone::clone(&String::from("Malformed pattern \"")))?;
                    let _t2 = _t1.append_str(Clone::clone(&pattern))?;
                    let _t3 = _t2.append_c(((34i32) as u16))?;
                    let _t4 = _t3.toString()?;
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                if j == 1i32 {
                    let _t1 = prefix.toString()?;
                    this.__set_posPrefixPattern(Clone::clone(&_t1));
                    let _t2 = suffix.toString()?;
                    this.__set_posSuffixPattern(Clone::clone(&_t2));
                    this.__set_negPrefixPattern(Clone::clone(&this.__get_posPrefixPattern()));
                    this.__set_negSuffixPattern(Clone::clone(&this.__get_posSuffixPattern()));
                    pos = ((digitLeftCount).wrapping_add(zeroDigitCount)).wrapping_add(digitRightCount);
                    let mut ch = (if (decimalPos>=0) { decimalPos } else { pos });
                    this.setMinimumIntegerDigits((ch).wrapping_sub(digitLeftCount))?;
                    let mut _merged4: i32;
                    if this.__get_useExponentialNotation() {
                        let _t3 = this.getMinimumIntegerDigits()?;
                        _merged4 = (digitLeftCount).wrapping_add(_t3);
                    } else {
                        _merged4 = 344i32;
                    }
                    this.setMaximumIntegerDigits(_merged4)?;
                    this.setMaximumFractionDigits((if (decimalPos>=0) { (pos).wrapping_sub(decimalPos) } else { 0i32 }))?;
                    this.setMinimumFractionDigits((if (decimalPos>=0) { ((digitLeftCount).wrapping_add(zeroDigitCount)).wrapping_sub(decimalPos) } else { 0i32 }))?;
                    this.setGroupingUsed((groupingCount>0))?;
                    this.__set_groupingSize((((if (groupingCount>0) { groupingCount } else { 0i32 })) as i8));
                    this.__set_multiplier(multiplier);
                    this.setDecimalSeparatorAlwaysShown(decimalPos == pos)?;
                } else {
                    let _t1 = prefix.toString()?;
                    this.__set_negPrefixPattern(Clone::clone(&_t1));
                    let _t2 = suffix.toString()?;
                    this.__set_negSuffixPattern(Clone::clone(&_t2));
                    gotNegative = 1i32;
                }
                j = j.wrapping_sub(1i32);
            }
            let _t0 = pattern.isEmpty()?;
            if _t0 {
                this.__set_posSuffixPattern(Clone::clone(&String::from("")));
                this.__set_posPrefixPattern(Clone::clone(&String::from("")));
                this.setMinimumIntegerDigits(0i32)?;
                this.setMaximumIntegerDigits(344i32)?;
                this.setMinimumFractionDigits(0i32)?;
                this.setMaximumFractionDigits(344i32)?;
            }
            let _t1 = this.__get_negPrefixPattern().equals(Object::from_any(this.__get_posPrefixPattern().clone()))?;
            let _t2 = this.__get_negSuffixPattern().equals(Object::from_any(this.__get_posSuffixPattern().clone()))?;
            if _t2 {
                this.__set_negSuffixPattern(Clone::clone(&this.__get_posSuffixPattern()));
                let _t3 = StringBuilder::new()?.append_str(Clone::clone(&String::from("'-")))?;
                let _t4 = _t3.append_str(Clone::clone(&this.__get_posPrefixPattern()))?;
                let _t5 = _t4.toString()?;
                this.__set_negPrefixPattern(Clone::clone(&_t5));
            }
            this.expandAffixes()?;
            Ok(())
        }

        #[java_method(name = "setMaximumIntegerDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMaximumIntegerDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::clamp_l_i_i((newValue as i64), 0i32, 344i32)?;
            this.__set_maximumIntegerDigits(_t0);
            let _t1: i32 = Math::min_i_i(this.__get_maximumIntegerDigits(), 309i32)?;
            this.__super().setMaximumIntegerDigits(_t1)?;
            if this.__get_minimumIntegerDigits() > this.__get_maximumIntegerDigits() {
                this.__set_minimumIntegerDigits(this.__get_maximumIntegerDigits());
                let _t2: i32 = Math::min_i_i(this.__get_minimumIntegerDigits(), 309i32)?;
                this.__super().setMinimumIntegerDigits(_t2)?;
            }
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "setMinimumIntegerDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinimumIntegerDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::clamp_l_i_i((newValue as i64), 0i32, 344i32)?;
            this.__set_minimumIntegerDigits(_t0);
            let _t1: i32 = Math::min_i_i(this.__get_minimumIntegerDigits(), 309i32)?;
            this.__super().setMinimumIntegerDigits(_t1)?;
            if this.__get_minimumIntegerDigits() > this.__get_maximumIntegerDigits() {
                this.__set_maximumIntegerDigits(this.__get_minimumIntegerDigits());
                let _t2: i32 = Math::min_i_i(this.__get_maximumIntegerDigits(), 309i32)?;
                this.__super().setMaximumIntegerDigits(_t2)?;
            }
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "setMaximumFractionDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMaximumFractionDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::clamp_l_i_i((newValue as i64), 0i32, 344i32)?;
            this.__set_maximumFractionDigits(_t0);
            let _t1: i32 = Math::min_i_i(this.__get_maximumFractionDigits(), 340i32)?;
            this.__super().setMaximumFractionDigits(_t1)?;
            if this.__get_minimumFractionDigits() > this.__get_maximumFractionDigits() {
                this.__set_minimumFractionDigits(this.__get_maximumFractionDigits());
                let _t2: i32 = Math::min_i_i(this.__get_minimumFractionDigits(), 340i32)?;
                this.__super().setMinimumFractionDigits(_t2)?;
            }
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "setMinimumFractionDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinimumFractionDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::clamp_l_i_i((newValue as i64), 0i32, 344i32)?;
            this.__set_minimumFractionDigits(_t0);
            let _t1: i32 = Math::min_i_i(this.__get_minimumFractionDigits(), 340i32)?;
            this.__super().setMinimumFractionDigits(_t1)?;
            if this.__get_minimumFractionDigits() > this.__get_maximumFractionDigits() {
                this.__set_maximumFractionDigits(this.__get_minimumFractionDigits());
                let _t2: i32 = Math::min_i_i(this.__get_maximumFractionDigits(), 340i32)?;
                this.__super().setMaximumFractionDigits(_t2)?;
            }
            this.__set_fastPathCheckNeeded((1i32 != 0i32));
            Ok(())
        }

        #[java_method(name = "getMaximumIntegerDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximumIntegerDigits(&self) -> Result<i32> {
            panic!("stub: java/text/DecimalFormat.getMaximumIntegerDigits:()I")
        }

        #[java_method(name = "getMinimumIntegerDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimumIntegerDigits(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_minimumIntegerDigits())
        }

        #[java_method(name = "getMaximumFractionDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximumFractionDigits(&self) -> Result<i32> {
            panic!("stub: java/text/DecimalFormat.getMaximumFractionDigits:()I")
        }

        #[java_method(name = "getMinimumFractionDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimumFractionDigits(&self) -> Result<i32> {
            panic!("stub: java/text/DecimalFormat.getMinimumFractionDigits:()I")
        }

        #[java_method(name = "getCurrency", descriptor = "()Ljava/util/Currency;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrency(&self) -> Result<Currency> {
            panic!("stub: java/text/DecimalFormat.getCurrency:()Ljava/util/Currency;")
        }

        #[java_method(name = "setCurrency", descriptor = "(Ljava/util/Currency;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCurrency(&self, currency: Currency) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setCurrency:(Ljava/util/Currency;)V")
        }

        #[java_method(name = "getRoundingMode", descriptor = "()Ljava/math/RoundingMode;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRoundingMode(&self) -> Result<RoundingMode> {
            panic!("stub: java/text/DecimalFormat.getRoundingMode:()Ljava/math/RoundingMode;")
        }

        #[java_method(name = "setRoundingMode", descriptor = "(Ljava/math/RoundingMode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRoundingMode(&self, roundingMode: RoundingMode) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.setRoundingMode:(Ljava/math/RoundingMode;)V")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/text/DecimalFormat.readObject:(Ljava/io/ObjectInputStream;)V")
        }
    }
}
