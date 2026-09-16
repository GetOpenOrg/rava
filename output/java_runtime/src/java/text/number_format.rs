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

impl From<NumberFormat> for Format {
    fn from(v: NumberFormat) -> Format { v.__into_super() }
}

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/text/NumberFormat"]
    #[super_class       = "java/text/Format"]
    #[interfaces        = ""]
    #[access            = "public"]
    #[modifiers         = "abstract"]
    #[generic_signature = ""]
    #[is_abstract       = true]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "NumberFormat.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/text/NumberFormat$Style:java/text/NumberFormat:Style:16409;java/text/NumberFormat$Field:java/text/NumberFormat:Field:9"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[superclass        = "Format"]
    #[all_supertypes    = "java/io/Serializable;java/lang/Cloneable;java/lang/Object;java/text/Format;java/text/NumberFormat"]
    #[has_hash_code_method = true]

    pub struct NumberFormat {
        #[cfg_attr(any(), java_field(name = "groupingUsed", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub groupingUsed: bool,
        #[cfg_attr(any(), java_field(name = "maxIntegerDigits", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub maxIntegerDigits: i8,
        #[cfg_attr(any(), java_field(name = "minIntegerDigits", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub minIntegerDigits: i8,
        #[cfg_attr(any(), java_field(name = "maxFractionDigits", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub maxFractionDigits: i8,
        #[cfg_attr(any(), java_field(name = "minFractionDigits", descriptor = "B", access = "private", modifiers = "", is_static = false))]
        pub minFractionDigits: i8,
        #[cfg_attr(any(), java_field(name = "parseIntegerOnly", descriptor = "Z", access = "private", modifiers = "", is_static = false))]
        pub parseIntegerOnly: bool,
        #[cfg_attr(any(), java_field(name = "maximumIntegerDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maximumIntegerDigits: i32,
        #[cfg_attr(any(), java_field(name = "minimumIntegerDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minimumIntegerDigits: i32,
        #[cfg_attr(any(), java_field(name = "maximumFractionDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub maximumFractionDigits: i32,
        #[cfg_attr(any(), java_field(name = "minimumFractionDigits", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub minimumFractionDigits: i32,
        #[cfg_attr(any(), java_field(name = "serialVersionOnStream", descriptor = "I", access = "private", modifiers = "", is_static = false))]
        pub serialVersionOnStream: i32,
    }

    impl NumberFormat {
        #[cfg_attr(any(), java_field(name = "INTEGER_FIELD", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: INTEGER_FIELD:I
        pub fn INTEGER_FIELD() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "FRACTION_FIELD", descriptor = "I", access = "public", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: FRACTION_FIELD:I
        pub fn FRACTION_FIELD() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "NUMBERSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "0"))]
        // static field: NUMBERSTYLE:I
        pub fn NUMBERSTYLE() -> i32 {
            0
        }

        #[cfg_attr(any(), java_field(name = "CURRENCYSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: CURRENCYSTYLE:I
        pub fn CURRENCYSTYLE() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "PERCENTSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "2"))]
        // static field: PERCENTSTYLE:I
        pub fn PERCENTSTYLE() -> i32 {
            2
        }

        #[cfg_attr(any(), java_field(name = "SCIENTIFICSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "3"))]
        // static field: SCIENTIFICSTYLE:I
        pub fn SCIENTIFICSTYLE() -> i32 {
            3
        }

        #[cfg_attr(any(), java_field(name = "INTEGERSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "4"))]
        // static field: INTEGERSTYLE:I
        pub fn INTEGERSTYLE() -> i32 {
            4
        }

        #[cfg_attr(any(), java_field(name = "COMPACTSTYLE", descriptor = "I", access = "private", modifiers = "static final", is_static = true, constant_value = "5"))]
        // static field: COMPACTSTYLE:I
        pub fn COMPACTSTYLE() -> i32 {
            5
        }

        #[cfg_attr(any(), java_field(name = "currentSerialVersion", descriptor = "I", access = "package", modifiers = "static final", is_static = true, constant_value = "1"))]
        // static field: currentSerialVersion:I
        pub fn currentSerialVersion() -> i32 {
            1
        }

        #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "package", modifiers = "static final", is_static = true, constant_value = "-2308460125733713944"))]
        // static field: serialVersionUID:J
        pub fn serialVersionUID() -> i64 {
            -2308460125733713944i64
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "protected", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this = Self::__new_with_super(Format::new()?);
            this.__set_groupingUsed((1i32 != 0i32));
            this.__set_maxIntegerDigits(((40i32) as i8));
            this.__set_minIntegerDigits(((1i32) as i8));
            this.__set_maxFractionDigits(((3i32) as i8));
            this.__set_minFractionDigits(((0i32) as i8));
            this.__set_parseIntegerOnly((0i32 != 0i32));
            this.__set_maximumIntegerDigits(40i32);
            this.__set_minimumIntegerDigits(1i32);
            this.__set_maximumFractionDigits(3i32);
            this.__set_minimumFractionDigits(0i32);
            this.__set_serialVersionOnStream(1i32);
            Ok(this)
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_obj_string_fieldp(&self, number: Object, toAppendTo: Object, pos: Object) -> Result<Object> {
            panic!("stub: java/text/NumberFormat.format:(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "parseObject", descriptor = "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn parseObject(&self, source: String, pos: Object) -> Result<Object> {
            panic!("stub: java/text/NumberFormat.parseObject:(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;")
        }

        #[java_method(name = "format", descriptor = "(D)Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_d(&self, number: f64) -> Result<String> {
            panic!("stub: java/text/NumberFormat.format:(D)Ljava/lang/String;")
        }

        #[java_method(name = "fastFormat", descriptor = "(D)Ljava/lang/String;", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn fastFormat(&self, number: f64) -> Result<String> {
            panic!("stub: java/text/NumberFormat.fastFormat:(D)Ljava/lang/String;")
        }

        #[java_method(name = "format", descriptor = "(J)Ljava/lang/String;", access = "public", modifiers = "final", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn format_l(&self, number: i64) -> Result<String> {
            panic!("stub: java/text/NumberFormat.format:(J)Ljava/lang/String;")
        }

        #[java_method(name = "format", descriptor = "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn format_d_string_fieldp(&self, arg0: f64, arg1: Object, arg2: Object) -> Result<Object> {
            panic!("stub: java/text/NumberFormat.format:(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "format", descriptor = "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn format_l_string_fieldp(&self, arg0: i64, arg1: Object, arg2: Object) -> Result<Object> {
            panic!("stub: java/text/NumberFormat.format:(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;", access = "public", modifiers = "abstract", is_static    = false, is_native    = false, is_abstract  = true, is_synthetic = false)]
        pub fn parse_str_parsep(&self, arg0: String, arg1: Object) -> Result<Number> {
            panic!("stub: java/text/NumberFormat.parse:(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;")
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;)Ljava/lang/Number;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/text/ParseException")]
        pub fn parse_str(&self, source: String) -> Result<Number> {
            panic!("stub: java/text/NumberFormat.parse:(Ljava/lang/String;)Ljava/lang/Number;")
        }

        #[java_method(name = "isParseIntegerOnly", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isParseIntegerOnly(&self) -> Result<bool> {
            panic!("stub: java/text/NumberFormat.isParseIntegerOnly:()Z")
        }

        #[java_method(name = "setParseIntegerOnly", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setParseIntegerOnly(&self, value: bool) -> Result<()> {
            panic!("stub: java/text/NumberFormat.setParseIntegerOnly:(Z)V")
        }

        #[java_method(name = "getInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance_locale(inLocale: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getNumberInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getNumberInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getNumberInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getNumberInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getNumberInstance(Ljava/util/Locale;)Ljava/text/NumberFormat;
        pub fn getNumberInstance_locale(mut inLocale: Locale) -> Result<NumberFormat> {
            let _t0: NumberFormat = NumberFormat::getInstance_locale_number_i(Clone::clone(&inLocale), Clone::clone(&Object::default()), 0i32)?;
            Ok(_t0)
        }

        #[java_method(name = "getIntegerInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntegerInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getIntegerInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getIntegerInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getIntegerInstance_locale(inLocale: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getIntegerInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getCurrencyInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrencyInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getCurrencyInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getCurrencyInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrencyInstance_locale(inLocale: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getCurrencyInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getPercentInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPercentInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getPercentInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getPercentInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getPercentInstance_locale(inLocale: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getPercentInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getScientificInstance", descriptor = "()Ljava/text/NumberFormat;", access = "package", modifiers = "static final", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScientificInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getScientificInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getScientificInstance", descriptor = "(Ljava/util/Locale;)Ljava/text/NumberFormat;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getScientificInstance_locale(inLocale: Locale) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getScientificInstance:(Ljava/util/Locale;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getCompactNumberInstance", descriptor = "()Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCompactNumberInstance() -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getCompactNumberInstance:()Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getCompactNumberInstance", descriptor = "(Ljava/util/Locale;Ljava/text/NumberFormat$Style;)Ljava/text/NumberFormat;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCompactNumberInstance_locale_number(locale: Locale, formatStyle: Object) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getCompactNumberInstance:(Ljava/util/Locale;Ljava/text/NumberFormat$Style;)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "getAvailableLocales", descriptor = "()[Ljava/util/Locale;", access = "public", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getAvailableLocales() -> Result<Rc<RefCell<Vec<Locale>>>> {
            panic!("stub: java/text/NumberFormat.getAvailableLocales:()[Ljava/util/Locale;")
        }

        #[java_method(name = "hashCode", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hashCode(&self) -> Result<i32> {
            Ok(0)
        }

        #[java_method(name = "equals", descriptor = "(Ljava/lang/Object;)Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn equals(&self, obj: Object) -> Result<bool> {
            panic!("stub: java/text/NumberFormat.equals:(Ljava/lang/Object;)Z")
        }

        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn clone(&self) -> Result<Object> {
            panic!("stub: java/text/NumberFormat.clone:()Ljava/lang/Object;")
        }

        #[java_method(name = "isGroupingUsed", descriptor = "()Z", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn isGroupingUsed(&self) -> Result<bool> {
            panic!("stub: java/text/NumberFormat.isGroupingUsed:()Z")
        }

        #[java_method(name = "setGroupingUsed", descriptor = "(Z)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setGroupingUsed(&self, mut newValue: bool) -> Result<()> {
            let this = self;
            this.__set_groupingUsed(newValue);
            Ok(())
        }

        #[java_method(name = "getMaximumIntegerDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximumIntegerDigits(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_maximumIntegerDigits())
        }

        #[java_method(name = "setMaximumIntegerDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMaximumIntegerDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::max_i_i(0i32, newValue)?;
            this.__set_maximumIntegerDigits(_t0);
            if this.__get_minimumIntegerDigits() > this.__get_maximumIntegerDigits() {
                this.__set_minimumIntegerDigits(this.__get_maximumIntegerDigits());
            }
            Ok(())
        }

        #[java_method(name = "getMinimumIntegerDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimumIntegerDigits(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_minimumIntegerDigits())
        }

        #[java_method(name = "setMinimumIntegerDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinimumIntegerDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::max_i_i(0i32, newValue)?;
            this.__set_minimumIntegerDigits(_t0);
            if this.__get_minimumIntegerDigits() > this.__get_maximumIntegerDigits() {
                this.__set_maximumIntegerDigits(this.__get_minimumIntegerDigits());
            }
            Ok(())
        }

        #[java_method(name = "getMaximumFractionDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMaximumFractionDigits(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_maximumFractionDigits())
        }

        #[java_method(name = "setMaximumFractionDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMaximumFractionDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::max_i_i(0i32, newValue)?;
            this.__set_maximumFractionDigits(_t0);
            if this.__get_maximumFractionDigits() < this.__get_minimumFractionDigits() {
                this.__set_minimumFractionDigits(this.__get_maximumFractionDigits());
            }
            Ok(())
        }

        #[java_method(name = "getMinimumFractionDigits", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getMinimumFractionDigits(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_minimumFractionDigits())
        }

        #[java_method(name = "setMinimumFractionDigits", descriptor = "(I)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setMinimumFractionDigits(&self, mut newValue: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Math::max_i_i(0i32, newValue)?;
            this.__set_minimumFractionDigits(_t0);
            if this.__get_maximumFractionDigits() < this.__get_minimumFractionDigits() {
                this.__set_maximumFractionDigits(this.__get_minimumFractionDigits());
            }
            Ok(())
        }

        #[java_method(name = "getCurrency", descriptor = "()Ljava/util/Currency;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getCurrency(&self) -> Result<Currency> {
            panic!("stub: java/text/NumberFormat.getCurrency:()Ljava/util/Currency;")
        }

        #[java_method(name = "setCurrency", descriptor = "(Ljava/util/Currency;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setCurrency(&self, currency: Currency) -> Result<()> {
            panic!("stub: java/text/NumberFormat.setCurrency:(Ljava/util/Currency;)V")
        }

        #[java_method(name = "getRoundingMode", descriptor = "()Ljava/math/RoundingMode;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getRoundingMode(&self) -> Result<RoundingMode> {
            panic!("stub: java/text/NumberFormat.getRoundingMode:()Ljava/math/RoundingMode;")
        }

        #[java_method(name = "setRoundingMode", descriptor = "(Ljava/math/RoundingMode;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn setRoundingMode(&self, roundingMode: RoundingMode) -> Result<()> {
            panic!("stub: java/text/NumberFormat.setRoundingMode:(Ljava/math/RoundingMode;)V")
        }

        #[java_method(name = "getInstance", descriptor = "(Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: getInstance(Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;
        pub fn getInstance_locale_number_i(mut desiredLocale: Locale, mut formatStyle: Object, mut choice: i32) -> Result<NumberFormat> {
            let _t0: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&desiredLocale))?;
            let mut adapter: LocaleProviderAdapter = _t0;
            let _t1: NumberFormat = NumberFormat::getInstance_locale_locale_number_i(Clone::clone(&adapter), Clone::clone(&desiredLocale), Clone::clone(&formatStyle), choice)?;
            let mut numberFormat: NumberFormat = _t1;
            if _is_jnull(&numberFormat) {
                let _t2: LocaleProviderAdapter = LocaleProviderAdapter::forJRE()?;
                let _t3: NumberFormat = NumberFormat::getInstance_locale_locale_number_i(Clone::clone(&_t2), Clone::clone(&desiredLocale), Clone::clone(&formatStyle), choice)?;
                numberFormat = _t3;
            }
            Ok(numberFormat)
        }

        #[java_method(name = "getInstance", descriptor = "(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getInstance_locale_locale_number_i(adapter: LocaleProviderAdapter, locale: Locale, formatStyle: Object, choice: i32) -> Result<NumberFormat> {
            panic!("stub: java/text/NumberFormat.getInstance:(Lsun/util/locale/provider/LocaleProviderAdapter;Ljava/util/Locale;Ljava/text/NumberFormat$Style;I)Ljava/text/NumberFormat;")
        }

        #[java_method(name = "readObject", descriptor = "(Ljava/io/ObjectInputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException,java/lang/ClassNotFoundException")]
        pub fn readObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/text/NumberFormat.readObject:(Ljava/io/ObjectInputStream;)V")
        }

        #[java_method(name = "writeObject", descriptor = "(Ljava/io/ObjectOutputStream;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn writeObject(&self, stream: Object) -> Result<()> {
            panic!("stub: java/text/NumberFormat.writeObject:(Ljava/io/ObjectOutputStream;)V")
        }
    }
}
