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
use crate::jdk::internal::math::FormattedFPDecimal;

java_rta_macros::java_class! {
    // ── 字节码元数据 ──────────────────────────────────────────────
    #[binary_name       = "java/util/Formatter$FormatSpecifier"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/util/Formatter$FormatString"]
    #[access            = "package"]
    #[modifiers         = ""]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Formatter$FormatSpecifier:java/util/Formatter:FormatSpecifier:8;java/util/Formatter$Flags:java/util/Formatter:Flags:8;java/util/Formatter$Conversion:java/util/Formatter:Conversion:8;java/util/Locale$Category:java/util/Locale:Category:16409;java/util/Formatter$DateTime:java/util/Formatter:DateTime:8;java/util/Formatter$FormatSpecifier$BigDecimalLayout:java/util/Formatter$FormatSpecifier:BigDecimalLayout:2;java/util/Formatter$BigDecimalLayoutForm:java/util/Formatter:BigDecimalLayoutForm:16409;java/util/Formatter$FormatString:java/util/Formatter:FormatString:1544"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/lang/Object;java/util/Formatter$FormatSpecifier;java/util/Formatter$FormatString"]
    #[has_to_string_method = true]

    pub struct Formatter_FormatSpecifier {
        #[cfg_attr(any(), java_field(name = "index", descriptor = "I", is_static = false))]
        pub index: i32,
        #[cfg_attr(any(), java_field(name = "flags", descriptor = "I", is_static = false))]
        pub flags: i32,
        #[cfg_attr(any(), java_field(name = "width", descriptor = "I", is_static = false))]
        pub width: i32,
        #[cfg_attr(any(), java_field(name = "precision", descriptor = "I", is_static = false))]
        pub precision: i32,
        #[cfg_attr(any(), java_field(name = "dt", descriptor = "Z", is_static = false))]
        pub dt: bool,
        #[cfg_attr(any(), java_field(name = "c", descriptor = "C", is_static = false))]
        pub c: u16,
    }

    impl Formatter_FormatSpecifier {
        #[cfg_attr(any(), java_field(name = "SCALEUP", descriptor = "D", access = "private", modifiers = "static final", is_static = true))]
        // static field: SCALEUP:D
        pub fn SCALEUP() -> f64 {
            panic!("stub: java/util/Formatter$FormatSpecifier.SCALEUP:D")
        }

        #[cfg_attr(any(), java_field(name = "$assertionsDisabled", descriptor = "Z", access = "package", modifiers = "static final synthetic", is_static = true))]
        // static field: $assertionsDisabled:Z
        pub fn _assertionsDisabled() -> bool {
            false
        }

        #[java_method(name = "index", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: index(Ljava/lang/String;II)V
        pub fn index_str_i_i(&self, mut s: String, mut start: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Integer::parseInt_seq_i_i_i(Object::from_any(s.clone()), start, (end).wrapping_sub(1i32), 10i32)?;
            this.__set_index(_t0);
            if (this.__get_index()<=0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "index", descriptor = "()I", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: index()I
        pub fn index(&self) -> Result<i32> {
            let this = self;
            Ok(this.__get_index())
        }

        #[java_method(name = "flags", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flags(&self, mut s: String, mut start: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Formatter_Flags::parse_str_i_i(Clone::clone(&s), start, end)?;
            this.__set_flags(_t0);
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 256i32)?;
            if _t1 {
                this.__set_index(-1i32);
            }
            Ok(())
        }

        #[java_method(name = "width", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn width(&self, mut s: String, mut start: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Integer::parseInt_seq_i_i_i(Object::from_any(s.clone()), start, end, 10i32)?;
            this.__set_width(_t0);
            if (this.__get_width()<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "precision", descriptor = "(Ljava/lang/String;II)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn precision(&self, mut s: String, mut start: i32, mut end: i32) -> Result<()> {
            let this = self;
            let _t0: i32 = Integer::parseInt_seq_i_i_i(Object::from_any(s.clone()), (start).wrapping_add(1i32), end, 10i32)?;
            this.__set_precision(_t0);
            if (this.__get_precision()<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "conversion", descriptor = "(C)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn conversion(&self, mut conv: u16) -> Result<()> {
            let this = self;
            this.__set_c(conv);
            let _t0: bool = Formatter_Conversion::isValid(this.__get_c())?;
            if !(_t0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: bool = Character::isUpperCase_c(this.__get_c())?;
            if _t1 {
                let _t2: i32 = Formatter_Flags::add(this.__get_flags(), 2i32)?;
                this.__set_flags(_t2);
                let _t3: u16 = Character::toLowerCase_c(this.__get_c())?;
                this.__set_c(_t3);
            }
            let _t2: bool = Formatter_Conversion::isText(this.__get_c())?;
            if _t2 {
                this.__set_index(-2i32);
            }
            Ok(())
        }

        #[java_method(name = "<init>", descriptor = "(C)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(C)V
        pub fn new_c(mut conv: u16) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_index(0i32);
            this.__set_flags(0i32);
            this.__set_width(-1i32);
            this.__set_precision(-1i32);
            this.__set_dt((0i32 != 0i32));
            this.__set_c(conv);
            let _t0: bool = Character::isUpperCase_c(conv)?;
            if _t0 {
                this.__set_flags(2i32);
                let _t1: u16 = Character::toLowerCase_c(conv)?;
                this.__set_c(_t1);
            }
            let _t1: bool = Formatter_Conversion::isText(conv)?;
            if _t1 {
                this.__set_index(-2i32);
            }
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/util/regex/Matcher;)V", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/String;Ljava/util/regex/Matcher;)V
        pub fn new_str_matche(mut s: String, mut m: Matcher) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_index(0i32);
            this.__set_flags(0i32);
            this.__set_width(-1i32);
            this.__set_precision(-1i32);
            this.__set_dt((0i32 != 0i32));
            let _t0 = m.start_i(1i32)?;
            let _t1 = m.end_i(1i32)?;
            this.index_str_i_i(Clone::clone(&s), _t0, _t1)?;
            let _t2 = m.start_i(2i32)?;
            let _t3 = m.end_i(2i32)?;
            this.flags(Clone::clone(&s), _t2, _t3)?;
            let _t4 = m.start_i(3i32)?;
            let _t5 = m.end_i(3i32)?;
            this.width(Clone::clone(&s), _t4, _t5)?;
            let _t6 = m.start_i(4i32)?;
            let _t7 = m.end_i(4i32)?;
            this.precision(Clone::clone(&s), _t6, _t7)?;
            let _t8 = m.start_i(5i32)?;
            let mut tTStart: i32 = _t8;
            this.__set_dt((1i32 != 0i32));
            let _t9 = s.charAt(tTStart)?;
            if (_t9 as i32) == 84i32 {
                let _t10: i32 = Formatter_Flags::add(this.__get_flags(), 2i32)?;
                this.__set_flags(_t10);
            }
            let _t10 = m.start_i(6i32)?;
            let _t11 = s.charAt(_t10)?;
            this.conversion(_t11)?;
            if this.__get_dt() {
                this.checkDateTime()?;
            } else {
                let _t12: bool = Formatter_Conversion::isGeneral(this.__get_c())?;
                if _t12 {
                    this.checkGeneral()?;
                } else {
                    let _t13: bool = Formatter_Conversion::isCharacter(this.__get_c())?;
                    if _t13 {
                        this.checkCharacter()?;
                    } else {
                        let _t14: bool = Formatter_Conversion::isInteger(this.__get_c())?;
                        if _t14 {
                            this.checkInteger()?;
                        } else {
                            let _t15: bool = Formatter_Conversion::isFloat(this.__get_c())?;
                            if _t15 {
                                this.checkFloat()?;
                            } else {
                                let _t16: bool = Formatter_Conversion::isText(this.__get_c())?;
                                if _t16 {
                                    this.checkText()?;
                                } else {
                                    return Err(JvmError::Custom("athrow".to_owned()));
                                }
                            }
                        }
                    }
                }
            }
            Ok(this)
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V
        pub fn print_format_obj_locale(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if this.__get_dt() {
                this.printDateTime(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                return Ok(());
            }
            match this.__get_c() {
                37 => {
                    this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("%")), Clone::clone(&l))?;
                }
                97 => {
                    this.printFloat(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                98 => {
                    this.printBoolean(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                99 => {
                    this.printCharacter(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                100 => {
                    this.printInteger(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                101 => {
                    this.printFloat(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                102 => {
                    this.printFloat(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                103 => {
                    this.printFloat(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                104 => {
                    this.printHashCode(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                110 => {
                    let _t0: String = System::lineSeparator()?;
                    let _vdispatch1: Object = if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<Writer>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<PrintStream>() { _d.append_seq(Object::from_any(_t0.clone()))? } else if let Some(_d) = fmt.__get_a().0.as_any().downcast_ref::<Object>() { _d.append(Object::from_any(_t0.clone()))? } else if let Some(__f) = fmt.__get_a().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Object::from_any(_t0.clone()))? } else { Default::default() };
                }
                111 => {
                    this.printInteger(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                115 => {
                    this.printString(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                120 => {
                    this.printInteger(Clone::clone(&fmt), Clone::clone(&arg), Clone::clone(&l))?;
                }
                _ => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "printInteger", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printInteger(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if _is_jnull(&arg) {
                this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("null")), Clone::clone(&l))?;
            } else {
                if (arg.is_instance_of("java/lang/Byte")) {
                    let _t0: i8 = (arg).downcast::<Byte>().byteValue()?;
                    this.print_format_b_locale(Clone::clone(&fmt), _t0, Clone::clone(&l))?;
                } else {
                    if (arg.is_instance_of("java/lang/Short")) {
                        let _t0: i16 = (arg).downcast::<Short>().shortValue()?;
                        this.print_format_s_locale(Clone::clone(&fmt), _t0, Clone::clone(&l))?;
                    } else {
                        if (arg.is_instance_of("java/lang/Integer")) {
                            this.print_format_i_locale(Clone::clone(&fmt), (arg).downcast::<i32>(), Clone::clone(&l))?;
                        } else {
                            if (arg.is_instance_of("java/lang/Long")) {
                                this.print_format_l_locale(Clone::clone(&fmt), (arg).downcast::<i64>(), Clone::clone(&l))?;
                            } else {
                                if (arg.is_instance_of("java/math/BigInteger")) {
                                    this.print_format_bigint_locale(Clone::clone(&fmt), Clone::clone(&(arg).downcast::<BigInteger>()), Clone::clone(&l))?;
                                } else {
                                    this.failConversion(this.__get_c(), Clone::clone(&arg))?;
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "printFloat", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printFloat(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if _is_jnull(&arg) {
                this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("null")), Clone::clone(&l))?;
            } else {
                if (arg.is_instance_of("java/lang/Float")) {
                    let _t0: f32 = (arg).downcast::<Float>().floatValue()?;
                    this.print_format_f_locale(Clone::clone(&fmt), _t0, Clone::clone(&l))?;
                } else {
                    if (arg.is_instance_of("java/lang/Double")) {
                        this.print_format_d_locale(Clone::clone(&fmt), (arg).downcast::<f64>(), Clone::clone(&l))?;
                    } else {
                        if (arg.is_instance_of("java/math/BigDecimal")) {
                            this.print_format_bigdec_locale(Clone::clone(&fmt), Clone::clone(&(arg).downcast::<BigDecimal>()), Clone::clone(&l))?;
                        } else {
                            this.failConversion(this.__get_c(), Clone::clone(&arg))?;
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "printDateTime", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printDateTime(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if _is_jnull(&arg) {
                this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("null")), Clone::clone(&l))?;
                return Ok(());
            }
            let mut cal: Object = Object::default();
            if (arg.is_instance_of("java/lang/Long")) {
                let _t0: Calendar = Calendar::getInstance_locale(Clone::clone(&(if _is_jnull(&l) { Locale::US() } else { l })))?;
                let mut cal: Calendar = _t0;
                cal.setTimeInMillis((arg).downcast::<i64>())?;
            } else {
                if (arg.is_instance_of("java/util/Date")) {
                    let _t0: Calendar = Calendar::getInstance_locale(Clone::clone(&(if _is_jnull(&l) { Locale::US() } else { l })))?;
                    let mut cal: Calendar = _t0;
                    cal.setTime(Clone::clone(&(arg).downcast::<Date>()))?;
                } else {
                    if (arg.is_instance_of("java/util/Calendar")) {
                        let _t0: Object = Object::from_any((arg).downcast::<Calendar>().clone());
                        let mut cal = (_t0).downcast::<Calendar>();
                        cal.setLenient((1i32 != 0i32))?;
                    } else {
                        if (arg.is_instance_of("java/time/temporal/TemporalAccessor")) {
                            this.print_format_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&arg), this.__get_c(), Clone::clone(&l))?;
                            return Ok(());
                        }
                        this.failConversion(this.__get_c(), Clone::clone(&arg))?;
                    }
                }
            }
            this.print_format_calend_c_locale(Clone::clone(&fmt), Clone::clone(&cal), this.__get_c(), Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "printCharacter", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printCharacter(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if _is_jnull(&arg) {
                this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("null")), Clone::clone(&l))?;
                return Ok(());
            }
            let mut s: Object = Object::default();
            if (arg.is_instance_of("java/lang/Character")) {
                let _t0 = (arg).downcast::<Character>().toString()?;
                let mut s: String = _t0;
            } else {
                if (arg.is_instance_of("java/lang/Byte")) {
                    let _t0: i8 = (arg).downcast::<Byte>().byteValue()?;
                    let mut i: i8 = _t0;
                    let _t1: bool = Character::isValidCodePoint((i as i32))?;
                    if _t1 {
                        let _t2: Rc<RefCell<Vec<u16>>> = Character::toChars_i((i as i32))?;
                        let mut s = String::new_arr_c(Clone::clone(&_t2))?;
                    } else {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                } else {
                    if (arg.is_instance_of("java/lang/Short")) {
                        let _t0: i16 = (arg).downcast::<Short>().shortValue()?;
                        let mut i: i16 = _t0;
                        let _t1: bool = Character::isValidCodePoint((i as i32))?;
                        if _t1 {
                            let _t2: Rc<RefCell<Vec<u16>>> = Character::toChars_i((i as i32))?;
                            let mut s = String::new_arr_c(Clone::clone(&_t2))?;
                        } else {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    } else {
                        if (arg.is_instance_of("java/lang/Integer")) {
                            let mut i = (arg).downcast::<i32>();
                            let _t0: bool = Character::isValidCodePoint(i)?;
                            if _t0 {
                                let _t1: Rc<RefCell<Vec<u16>>> = Character::toChars_i(i)?;
                                let mut s = String::new_arr_c(Clone::clone(&_t1))?;
                            } else {
                                return Err(JvmError::Custom("athrow".to_owned()));
                            }
                        } else {
                            this.failConversion(this.__get_c(), Clone::clone(&arg))?;
                        }
                    }
                }
            }
            this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&s), Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "printString", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printString(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            if (arg.is_instance_of("java/util/Formattable")) {
                let _t0 = fmt.locale()?;
                if Object::from_any(_t0.clone()) != Object::from_any(l.clone()) {
                    let _t1 = fmt.out()?;
                    fmt = Formatter::new_append_locale(Clone::clone(&_t1), Clone::clone(&l))?;
                }
                if let Some(__f) = arg.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Formatter, i32, i32, i32) -> crate::error::Result<()>>>() { (__f)(Clone::clone(&fmt), this.__get_flags(), this.__get_width(), this.__get_precision())?; }
            } else {
                let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                if _t0 {
                    this.failMismatch(4i32, ((115i32) as u16))?;
                }
                if _is_jnull(&arg) {
                    this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&String::from("null")), Clone::clone(&l))?;
                } else {
                    let _vdispatch1: String = if let Some(__f) = arg.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                    this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&_vdispatch1), Clone::clone(&l))?;
                }
            }
            Ok(())
        }

        #[java_method(name = "printBoolean", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printBoolean(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
        let mut s: String = Default::default();
            if !_is_jnull(&arg) {
                let mut _merged1: String;
                if (arg.is_instance_of("java/lang/Boolean")) {
                    let _t0 = (arg).downcast::<bool>().toString()?;
                    _merged1 = _t0;
                } else {
                    let _t0: String = Boolean::toString_z((1i32 != 0i32))?;
                    _merged1 = _t0;
                }
                s = _merged1;
            } else {
                let _t0: String = Boolean::toString_z((0i32 != 0i32))?;
                s = _t0;
            }
            this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&s), Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "printHashCode", descriptor = "(Ljava/util/Formatter;Ljava/lang/Object;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn printHashCode(&self, mut fmt: Formatter, mut arg: Object, mut l: Locale) -> Result<()> {
            let this = self;
            let mut _merged2: String;
            if _is_jnull(&arg) {
                _merged2 = String::from("null");
            } else {
                let _vdispatch0: i32 = if let Some(__f) = arg.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let _t1: String = Integer::toHexString(_vdispatch0)?;
                _merged2 = _t1;
            }
            let mut s: String = _merged2;
            this.print_format_str_locale(Clone::clone(&fmt), Clone::clone(&s), Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/String;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/lang/String;Ljava/util/Locale;)V
        pub fn print_format_str_locale(&self, mut fmt: Formatter, mut s: String, mut l: Locale) -> Result<()> {
            let this = self;
            let _t0 = s.length()?;
            if this.__get_precision() < _t0 {
                let _t1 = s.substring_i_i(0i32, this.__get_precision())?;
                s = _t1;
            }
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
            if _t1 {
                let _t2 = this.toUpperCaseWithLocale(Clone::clone(&s), Clone::clone(&l))?;
                s = _t2;
            }
            this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(s.clone()))?;
            Ok(())
        }

        #[java_method(name = "toUpperCaseWithLocale", descriptor = "(Ljava/lang/String;Ljava/util/Locale;)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toUpperCaseWithLocale(&self, mut s: String, mut l: Locale) -> Result<String> {
            let this = self;
            let _t0: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(_t0.clone()))?;
            let _t2 = s.toUpperCase_locale(Clone::clone(&(_t1).downcast::<Locale>()))?;
            Ok(_t2)
        }

        #[java_method(name = "appendJustified", descriptor = "(Ljava/lang/Appendable;Ljava/lang/CharSequence;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn appendJustified(&self, mut a: Object, mut cs: Object) -> Result<()> {
            let this = self;
            if this.__get_width() == -1i32 {
                let _vdispatch0: Object = if let Some(_d) = a.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Writer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<PrintStream>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Object>() { _d.append(Clone::clone(&cs))? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&cs))? } else { Default::default() };
                return Ok(());
            }
            let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 1i32)?;
            let mut padRight = (_t0) as i32;
            let _vdispatch1: i32 = if let Some(_d) = cs.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = cs.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = cs.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = cs.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = cs.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = cs.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = cs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut sp = (this.__get_width()).wrapping_sub(_vdispatch1);
            if (padRight!=0) {
                let _vdispatch2: Object = if let Some(_d) = a.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Writer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<PrintStream>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Object>() { _d.append(Clone::clone(&cs))? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&cs))? } else { Default::default() };
            }
            let mut i: i32 = 0i32;
            loop {
                if i >= sp { break; }
                let _vdispatch2: Object = if let Some(_d) = a.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Writer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Object>() { _d.append(((32i32) as u16))? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((32i32) as u16))? } else { Default::default() };
                i = i.wrapping_add(1i32);
            }
            if (padRight==0) {
                let _vdispatch2: Object = if let Some(_d) = a.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<CharBuffer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Writer>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<StringBuilder>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<PrintStream>() { _d.append_seq(Clone::clone(&cs))? } else if let Some(_d) = a.0.as_any().downcast_ref::<Object>() { _d.append(Clone::clone(&cs))? } else if let Some(__f) = a.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&cs))? } else { Default::default() };
            }
            Ok(())
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            let mut sb = StringBuilder::new_str(Clone::clone(&String::from("%")))?;
            let _t0: i32 = Formatter_Flags::remove(this.__get_flags(), 2i32)?;
            let _t1: String = Formatter_Flags::toString(_t0)?;
            let _t2 = sb.append_str(Clone::clone(&_t1))?;
            if (this.__get_index()>0) {
                let _t3 = sb.append_i(this.__get_index())?;
                let _t4 = _t3.append_c(((36i32) as u16))?;
            }
            if this.__get_width() != -1i32 {
                let _t3 = sb.append_i(this.__get_width())?;
            }
            if this.__get_precision() != -1i32 {
                let _t3 = sb.append_c(((46i32) as u16))?;
                let _t4 = _t3.append_i(this.__get_precision())?;
            }
            let _t3: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
            let _t4 = sb.append_c((((if _t3 { 84i32 } else { 116i32 })) as u16))?;
            let _t5: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
            let mut _merged7: u16;
            if _t5 {
                let _t6: u16 = Character::toUpperCase_c(this.__get_c())?;
                _merged7 = _t6;
            } else {
                _merged7 = this.__get_c();
            }
            let _t8 = sb.append_c(_merged7)?;
            let _t9 = sb.toString()?;
            Ok(_t9)
        }

        #[java_method(name = "checkGeneral", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkGeneral(&self) -> Result<()> {
            let this = self;
            let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
            if _t0 {
                this.failMismatch(4i32, this.__get_c())?;
            }
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 1i32)?;
            if _t1 {
                let _t2 = this.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.checkBadFlags(248i32)?;
            Ok(())
        }

        #[java_method(name = "checkDateTime", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkDateTime(&self) -> Result<()> {
            let this = self;
            if this.__get_precision() != -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: bool = Formatter_DateTime::isValid(this.__get_c())?;
            if !(_t0) {
                let _t1 = StringBuilder::new()?.append_str(Clone::clone(&String::from("t")))?;
                let _t2 = _t1.append_c(this.__get_c())?;
                let _t3 = _t2.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.checkBadFlags(252i32)?;
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 1i32)?;
            if _t1 {
                let _t2 = this.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "checkCharacter", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkCharacter(&self) -> Result<()> {
            let this = self;
            if this.__get_precision() != -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.checkBadFlags(252i32)?;
            let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 1i32)?;
            if _t0 {
                let _t1 = this.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "checkInteger", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkInteger(&self) -> Result<()> {
            let this = self;
            this.checkNumeric()?;
            if this.__get_precision() != -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (this.__get_c() as i32) == 100i32 {
                this.checkBadFlags(4i32)?;
            } else {
                if (this.__get_c() as i32) == 111i32 {
                    this.checkBadFlags(64i32)?;
                } else {
                    this.checkBadFlags(64i32)?;
                }
            }
            Ok(())
        }

        #[java_method(name = "checkBadFlags", descriptor = "(I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkBadFlags(&self, mut badFlags: i32) -> Result<()> {
            let this = self;
            if ((this.__get_flags()&badFlags)!=0) {
                this.failMismatch((this.__get_flags()&badFlags), this.__get_c())?;
            }
            Ok(())
        }

        #[java_method(name = "checkFloat", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkFloat(&self) -> Result<()> {
            let this = self;
            this.checkNumeric()?;
            if (this.__get_c() as i32) == 102i32 {
            } else {
                if (this.__get_c() as i32) == 97i32 {
                    this.checkBadFlags(192i32)?;
                } else {
                    if (this.__get_c() as i32) == 101i32 {
                        this.checkBadFlags(64i32)?;
                    } else {
                        if (this.__get_c() as i32) == 103i32 {
                            this.checkBadFlags(4i32)?;
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "checkNumeric", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkNumeric(&self) -> Result<()> {
            let this = self;
            if (this.__get_width()<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if (this.__get_precision()<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: bool = Formatter_Flags::containsAny(this.__get_flags(), 33i32)?;
            if _t0 {
                let _t1 = this.toString()?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 24i32)?;
            let _t2: bool = Formatter_Flags::contains(this.__get_flags(), 33i32)?;
            if _t2 {
                let _t3: String = Formatter_Flags::toString(this.__get_flags())?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "checkText", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn checkText(&self) -> Result<()> {
            let this = self;
            if this.__get_precision() != -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _switch_key = this.__get_c();
            if (this.__get_flags()!=0) {
                let _t0: String = Formatter_Flags::toString(this.__get_flags())?;
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 1i32)?;
            let _t1 = this.toString()?;
            return Err(JvmError::Custom("athrow".to_owned()));
            if this.__get_width() != -1i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t2: String = Formatter_Flags::toString(this.__get_flags())?;
            return Err(JvmError::Custom("athrow".to_owned()));
            if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;BLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;BLjava/util/Locale;)V
        pub fn print_format_b_locale(&self, mut fmt: Formatter, mut value: i8, mut l: Locale) -> Result<()> {
            let this = self;
            let mut v: i64 = (value as i64);
            if (this.__get_c() as i32) == 120i32 {
                v = (v).wrapping_add(256i64);
            }
            this.print_format_l_locale(Clone::clone(&fmt), v, Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;SLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;SLjava/util/Locale;)V
        pub fn print_format_s_locale(&self, mut fmt: Formatter, mut value: i16, mut l: Locale) -> Result<()> {
            let this = self;
            let mut v: i64 = (value as i64);
            v = (v).wrapping_add(65536i64);
            if (((v>(0i64)) as i32-((v)<(0i64)) as i32)<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.print_format_l_locale(Clone::clone(&fmt), v, Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;ILjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;ILjava/util/Locale;)V
        pub fn print_format_i_locale(&self, mut fmt: Formatter, mut value: i32, mut l: Locale) -> Result<()> {
            let this = self;
            let mut v: i64 = (value as i64);
            v = (v).wrapping_add(4294967296i64);
            if (((v>(0i64)) as i32-((v)<(0i64)) as i32)<0) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            this.print_format_l_locale(Clone::clone(&fmt), v, Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;JLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;JLjava/util/Locale;)V
        pub fn print_format_l_locale(&self, mut fmt: Formatter, mut value: i64, mut l: Locale) -> Result<()> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            if (this.__get_c() as i32) == 100i32 {
                let mut neg = ((((value>(0i64)) as i32-((value)<(0i64)) as i32)<0)) as i32;
                let _t0: String = Long::toString_l_i(value, 10i32)?;
                let mut valueStr: String = _t0;
                let _t1 = this.leadingSign(Clone::clone(&sb), (neg != 0i32))?;
                let _t2 = this.adjustWidth(this.__get_width(), this.__get_flags(), (neg != 0i32))?;
                let _t3 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(valueStr.clone()), ((neg!=0) as i32), this.__get_flags(), _t2, Clone::clone(&l))?;
                let _t4 = this.trailingSign(Clone::clone(&sb), (neg != 0i32))?;
            } else {
                if (this.__get_c() as i32) == 111i32 {
                    this.checkBadFlags(152i32)?;
                    let _t0: String = Long::toOctalString(value)?;
                    let mut neg: String = _t0;
                    let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    let mut _merged3: i32;
                    if _t1 {
                        let _t2 = neg.length()?;
                        _merged3 = (_t2).wrapping_add(1i32);
                    } else {
                        let _t2 = neg.length()?;
                        _merged3 = _t2;
                    }
                    let mut valueStr: i32 = _merged3;
                    let _t4: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    if _t4 {
                        let _t5 = sb.append_c(((48i32) as u16))?;
                    }
                    let _t5: bool = Formatter_Flags::contains(this.__get_flags(), 32i32)?;
                    if _t5 {
                        this.trailingZeros(Clone::clone(&sb), (this.__get_width()).wrapping_sub(valueStr))?;
                    }
                    let _t6 = sb.append_str(Clone::clone(&neg))?;
                } else {
                    this.checkBadFlags(152i32)?;
                    let _t0: String = Long::toHexString(value)?;
                    let mut neg: String = _t0;
                    let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    let mut _merged3: i32;
                    if _t1 {
                        let _t2 = neg.length()?;
                        _merged3 = (_t2).wrapping_add(2i32);
                    } else {
                        let _t2 = neg.length()?;
                        _merged3 = _t2;
                    }
                    let mut valueStr: i32 = _merged3;
                    let _t4: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    let _t5: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                    let _t6 = sb.append_str(Clone::clone(&(if _t5 { String::from("0X") } else { String::from("0x") })))?;
                    let _t7: bool = Formatter_Flags::contains(this.__get_flags(), 32i32)?;
                    if _t7 {
                        this.trailingZeros(Clone::clone(&sb), (this.__get_width()).wrapping_sub(valueStr))?;
                    }
                    let _t8: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                    if _t8 {
                        let _t9 = this.toUpperCaseWithLocale(Clone::clone(&neg), Clone::clone(&l))?;
                        neg = _t9;
                    }
                    let _t9 = sb.append_str(Clone::clone(&neg))?;
                }
            }
            this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            Ok(())
        }

        #[java_method(name = "leadingSign", descriptor = "(Ljava/lang/StringBuilder;Z)Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn leadingSign(&self, mut sb: StringBuilder, mut neg: bool) -> Result<StringBuilder> {
            let this = self;
            if !(neg) {
                let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 8i32)?;
                if _t0 {
                    let _t1 = sb.append_c(((43i32) as u16))?;
                } else {
                    let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 16i32)?;
                    let _t2 = sb.append_c(((32i32) as u16))?;
                }
            } else {
                let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 128i32)?;
                if _t0 {
                    let _t1 = sb.append_c(((40i32) as u16))?;
                } else {
                    let _t1 = sb.append_c(((45i32) as u16))?;
                }
            }
            Ok(sb)
        }

        #[java_method(name = "trailingSign", descriptor = "(Ljava/lang/StringBuilder;Z)Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trailingSign(&self, mut sb: StringBuilder, mut neg: bool) -> Result<StringBuilder> {
            let this = self;
            let _t0: bool = Formatter_Flags::contains(this.__get_flags(), 128i32)?;
            if _t0 {
                let _t1 = sb.append_c(((41i32) as u16))?;
            }
            Ok(sb)
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/math/BigInteger;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/math/BigInteger;Ljava/util/Locale;)V
        pub fn print_format_bigint_locale(&self, mut fmt: Formatter, mut value: BigInteger, mut l: Locale) -> Result<()> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            let _t0 = value.signum()?;
            let mut neg = (_t0 == -1i32) as i32;
            let _t1 = value.abs()?;
            let mut v: BigInteger = _t1;
            let _t2 = this.leadingSign(Clone::clone(&sb), (neg != 0i32))?;
            if (this.__get_c() as i32) == 100i32 {
                let _t3 = v.toString()?;
                let _t4 = this.adjustWidth(this.__get_width(), this.__get_flags(), (neg != 0i32))?;
                let _t5 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(_t3.clone()), 0i32, this.__get_flags(), _t4, Clone::clone(&l))?;
            } else {
                if (this.__get_c() as i32) == 111i32 {
                    let _t3 = v.toString_i(8i32)?;
                    let mut s: String = _t3;
                    let _t4 = s.length()?;
                    let _t5 = sb.__super().length()?;
                    let mut len = (_t4).wrapping_add(_t5);
                    let _t6: bool = Formatter_Flags::contains(this.__get_flags(), 128i32)?;
                    if _t6 {
                        len = len.wrapping_add(1i32);
                    }
                    let _t7: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    if _t7 {
                        len = len.wrapping_add(1i32);
                        let _t8 = sb.append_c(((48i32) as u16))?;
                    }
                    let _t8: bool = Formatter_Flags::contains(this.__get_flags(), 32i32)?;
                    if _t8 {
                        this.trailingZeros(Clone::clone(&sb), (this.__get_width()).wrapping_sub(len))?;
                    }
                    let _t9 = sb.append_str(Clone::clone(&s))?;
                } else {
                    let _t3 = v.toString_i(16i32)?;
                    let mut s: String = _t3;
                    let _t4 = s.length()?;
                    let _t5 = sb.__super().length()?;
                    let mut len = (_t4).wrapping_add(_t5);
                    let _t6: bool = Formatter_Flags::contains(this.__get_flags(), 128i32)?;
                    if _t6 {
                        len = len.wrapping_add(1i32);
                    }
                    let _t7: bool = Formatter_Flags::contains(this.__get_flags(), 4i32)?;
                    len = len.wrapping_add(2i32);
                    let _t8: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                    let _t9 = sb.append_str(Clone::clone(&(if _t8 { String::from("0X") } else { String::from("0x") })))?;
                    let _t10: bool = Formatter_Flags::contains(this.__get_flags(), 32i32)?;
                    if _t10 {
                        this.trailingZeros(Clone::clone(&sb), (this.__get_width()).wrapping_sub(len))?;
                    }
                    let _t11: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                    if _t11 {
                        let _t12 = this.toUpperCaseWithLocale(Clone::clone(&s), Clone::clone(&l))?;
                        s = _t12;
                    }
                    let _t12 = sb.append_str(Clone::clone(&s))?;
                }
            }
            let _t3 = value.signum()?;
            let _t4 = this.trailingSign(Clone::clone(&sb), _t3 == -1i32)?;
            this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;FLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;FLjava/util/Locale;)V
        pub fn print_format_f_locale(&self, mut fmt: Formatter, mut value: f32, mut l: Locale) -> Result<()> {
            let this = self;
            this.print_format_d_locale(Clone::clone(&fmt), (value as f64), Clone::clone(&l))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;DLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;DLjava/util/Locale;)V
        pub fn print_format_d_locale(&self, mut fmt: Formatter, mut value: f64, mut l: Locale) -> Result<()> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            let _t0: i32 = Double::compare(value, 0f64)?;
            let mut neg = (_t0 == -1i32) as i32;
            let _t1: bool = Double::isNaN_d(value)?;
            if !(_t1) {
                let _t2: f64 = Math::abs_d(value)?;
                let mut v: f64 = _t2;
                let _t3 = this.leadingSign(Clone::clone(&sb), (neg != 0i32))?;
                let _t4: bool = Double::isInfinite_d(v)?;
                if !(_t4) {
                    this.print_format_sb_d_locale_i_c_i_z(Clone::clone(&fmt), Clone::clone(&sb), v, Clone::clone(&l), this.__get_flags(), this.__get_c(), this.__get_precision(), (neg != 0i32))?;
                } else {
                    let _t5: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                    let _t6 = sb.append_str(Clone::clone(&(if _t5 { String::from("INFINITY") } else { String::from("Infinity") })))?;
                }
                let _t5 = this.trailingSign(Clone::clone(&sb), (neg != 0i32))?;
            } else {
                let _t2: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
                let _t3 = sb.append_str(Clone::clone(&(if _t2 { String::from("NAN") } else { String::from("NaN") })))?;
            }
            this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;DLjava/util/Locale;ICIZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;DLjava/util/Locale;ICIZ)V
        pub fn print_format_sb_d_locale_i_c_i_z(&self, mut fmt: Formatter, mut sb: StringBuilder, mut value: f64, mut l: Locale, mut flags: i32, mut c: u16, mut precision: i32, mut neg: bool) -> Result<()> {
            let this = self;
        let mut exp: Rc<RefCell<Vec<u16>>> = Default::default();
        let mut fd: FormattedFPDecimal = Default::default();
            if (c as i32) == 101i32 {
                let mut prec = (if precision == -1i32 { 6i32 } else { precision });
                let _t0: FormattedFPDecimal = FormattedFPDecimal::valueOf(value, prec, ((101i32) as u16))?;
                fd = _t0;
                let _t1 = fd.getMantissa()?;
                let _t2 = StringBuilder::new()?.append_arr_c(Clone::clone(&_t1))?;
                let mut mant: StringBuilder = _t2;
                this.addZeros(Clone::clone(&mant), prec)?;
                let _t3: bool = Formatter_Flags::contains(flags, 4i32)?;
                if (prec==0) {
                    let _t4 = mant.append_c(((46i32) as u16))?;
                }
                let mut _merged5: Rc<RefCell<Vec<u16>>>;
                if (((value>(0f64)) as i32-((value)<(0f64)) as i32)==0) {
                    let mut _arr4: Rc<RefCell<Vec<u16>>> = Rc::new(RefCell::new(vec![0u16; 3i32 as usize]));
                    _arr4.borrow_mut()[0i32 as usize] = (43i32) as u16;
                    _arr4.borrow_mut()[1i32 as usize] = (48i32) as u16;
                    _arr4.borrow_mut()[2i32 as usize] = (48i32) as u16;
                    _merged5 = _arr4;
                } else {
                    let _t4 = fd.getExponent()?;
                    _merged5 = _t4;
                }
                exp = _merged5;
                let mut newW = this.__get_width();
                if this.__get_width() != -1i32 {
                    let _t6 = this.adjustWidth(((this.__get_width()).wrapping_sub((exp.borrow().len() as i32))).wrapping_sub(1i32), flags, neg)?;
                    newW = _t6;
                }
                let _t6 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(mant.clone()), 0i32, flags, newW, Clone::clone(&l))?;
                let _t7: bool = Formatter_Flags::contains(flags, 2i32)?;
                let _t8 = sb.append_c((((if _t7 { 69i32 } else { 101i32 })) as u16))?;
                let mut sign = (exp.borrow()[0i32 as usize] as i32);
                if sign != 45i32 {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t9 = sb.append_c(((sign) as u16))?;
                this.localizedMagnitudeExp(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&exp), 1i32, Clone::clone(&l))?;
            } else {
                if (c as i32) == 102i32 {
                    let mut prec = (if precision == -1i32 { 6i32 } else { precision });
                    let _t0: FormattedFPDecimal = FormattedFPDecimal::valueOf(value, prec, ((102i32) as u16))?;
                    fd = _t0;
                    let _t1 = fd.getMantissa()?;
                    let _t2 = StringBuilder::new()?.append_arr_c(Clone::clone(&_t1))?;
                    let mut mant: StringBuilder = _t2;
                    this.addZeros(Clone::clone(&mant), prec)?;
                    let _t3: bool = Formatter_Flags::contains(flags, 4i32)?;
                    if (prec==0) {
                        let _t4 = mant.append_c(((46i32) as u16))?;
                    }
                    exp = this.__get_width();
                    if this.__get_width() != -1i32 {
                        let _t4 = this.adjustWidth(this.__get_width(), flags, neg)?;
                        exp = _t4;
                    }
                    let _t4 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(mant.clone()), 0i32, flags, exp, Clone::clone(&l))?;
                } else {
                    if (c as i32) == 103i32 {
                        let mut prec: i32 = precision;
                        if precision == -1i32 {
                            prec = 6i32;
                        } else {
                            if (precision==0) {
                                prec = 1i32;
                            }
                        }
                        let mut mant = StringBuilder::new()?;
                        if (((value>(0f64)) as i32-((value)<(0f64)) as i32)==0) {
                            fd = Object::default();
                            let _t0 = mant.append_c(((48i32) as u16))?;
                            exp = 0i32;
                        } else {
                            let _t0: FormattedFPDecimal = FormattedFPDecimal::valueOf(value, prec, ((103i32) as u16))?;
                            let mut newW: FormattedFPDecimal = _t0;
                            let _t1 = newW.getExponent()?;
                            fd = _t1;
                            let _t2 = newW.getMantissa()?;
                            let _t3 = mant.append_arr_c(Clone::clone(&_t2))?;
                            let _t4 = newW.getExponentRounded()?;
                            exp = _t4;
                        }
                        if !_is_jnull(&fd) {
                            prec = prec.wrapping_sub(1i32);
                        } else {
                            prec = (prec).wrapping_sub((exp).wrapping_add(1i32));
                        }
                        this.addZeros(Clone::clone(&mant), prec)?;
                        let _t0: bool = Formatter_Flags::contains(flags, 4i32)?;
                        if (prec==0) {
                            let _t1 = mant.append_c(((46i32) as u16))?;
                        }
                        let mut newW = this.__get_width();
                        if !_is_jnull(&fd) {
                            let _t1 = this.adjustWidth(((this.__get_width()).wrapping_sub((fd.borrow().len() as i32))).wrapping_sub(1i32), flags, neg)?;
                            newW = _t1;
                        } else {
                            let _t1 = this.adjustWidth(this.__get_width(), flags, neg)?;
                            newW = _t1;
                        }
                        let _t1 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(mant.clone()), 0i32, flags, newW, Clone::clone(&l))?;
                        let _t2: bool = Formatter_Flags::contains(flags, 2i32)?;
                        let _t3 = sb.append_c((((if _t2 { 69i32 } else { 101i32 })) as u16))?;
                        let mut sign = (fd.borrow()[0i32 as usize] as i32);
                        if sign != 45i32 {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                        let _t4 = sb.append_c(((sign) as u16))?;
                        this.localizedMagnitudeExp(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&fd), 1i32, Clone::clone(&l))?;
                    } else {
                        let mut prec: i32 = precision;
                        if precision == -1i32 {
                            prec = 0i32;
                        } else {
                            if (precision==0) {
                                prec = 1i32;
                            }
                        }
                        let _t0 = this.hexDouble(value, prec)?;
                        fd = _t0;
                        let mut mant = StringBuilder::new()?;
                        let _t1: bool = Formatter_Flags::contains(flags, 2i32)?;
                        exp = (_t1) as i32;
                        let _t2 = sb.append_str(Clone::clone(&(if (exp!=0) { String::from("0X") } else { String::from("0x") })))?;
                        let _t3: bool = Formatter_Flags::contains(flags, 32i32)?;
                        let mut newW: i32 = 2i32;
                        let _t4: bool = Formatter_Flags::contains(flags, 16i32)?;
                        let _t5: bool = Formatter_Flags::contains(flags, 8i32)?;
                        if neg {
                            newW = 3i32;
                        }
                        let _t6 = fd.length()?;
                        this.trailingZeros(Clone::clone(&sb), ((this.__get_width()).wrapping_sub(_t6)).wrapping_sub(newW))?;
                        let _t7 = fd.indexOf_i(112i32)?;
                        newW = _t7;
                        if (exp!=0) {
                            let _t8 = fd.substring_i_i(0i32, newW)?;
                            let mut sign: String = _t8;
                            let _t9 = sign.toUpperCase_locale(Clone::clone(&Locale::ROOT()))?;
                            sign = _t9;
                            let _t10 = mant.append_str(Clone::clone(&sign))?;
                        } else {
                            let _t8 = mant.append_seq_i_i(Object::from_any(fd.clone()), 0i32, newW)?;
                        }
                        if (prec!=0) {
                            this.addZeros(Clone::clone(&mant), prec)?;
                        }
                        let _t8 = sb.append_seq(Object::from_any(mant.clone()))?;
                        let _t9 = sb.append_c((((if (exp!=0) { 80i32 } else { 112i32 })) as u16))?;
                        let _t10 = fd.length()?;
                        let _t11 = sb.append_seq_i_i(Object::from_any(fd.clone()), (newW).wrapping_add(1i32), _t10)?;
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "addZeros", descriptor = "(Ljava/lang/StringBuilder;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn addZeros(&self, mut sb: StringBuilder, mut prec: i32) -> Result<()> {
            let this = self;
            let _t0 = sb.__super().length()?;
            let mut len: i32 = _t0;
            let mut i: i32 = 0i32;
            loop {
                if i >= len { break; }
                let _t1 = sb.__super().charAt(i)?;
                if (_t1 as i32) == 46i32 {
                    break;
                }
                i = i.wrapping_add(1i32);
            }
            let mut needDot: i32 = 0i32;
            if i == len {
                needDot = 1i32;
            }
            let mut outPrec = ((len).wrapping_sub(i)).wrapping_sub(((needDot==0) as i32));
            if outPrec > prec {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            if outPrec == prec {
                return Ok(());
            }
            if (needDot!=0) {
                let _t1 = sb.append_c(((46i32) as u16))?;
            }
            this.trailingZeros(Clone::clone(&sb), (prec).wrapping_sub(outPrec))?;
            Ok(())
        }

        #[java_method(name = "hexDouble", descriptor = "(DI)Ljava/lang/String;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn hexDouble(&self, mut d: f64, mut prec: i32) -> Result<String> {
            let this = self;
            let _t0: bool = Double::isFinite(d)?;
            if prec >= 13i32 {
                let _t1: String = Double::toHexString(d)?;
                let _t2 = _t1.substring_i(2i32)?;
                return Ok(_t2);
            }
            if prec > 12i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t1: i32 = Math::getExponent_d(d)?;
            let mut exponent: i32 = _t1;
            let mut subnormal = (exponent == -1023i32) as i32;
            d = (d*Formatter_FormatSpecifier::SCALEUP());
            let _t2: i32 = Math::getExponent_d(d)?;
            exponent = _t2;
            if exponent > 1023i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let mut precision = (1i32).wrapping_add((prec).wrapping_mul(4i32));
            let mut shiftDistance = (53i32).wrapping_sub(precision);
            if shiftDistance >= 53i32 {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            let _t3: i64 = Double::doubleToLongBits(d)?;
            let mut doppel: i64 = _t3;
            let mut newSignif = ((doppel&(9223372036854775807i64))).wrapping_shr((shiftDistance&0x3f) as u32);
            let mut roundingBits = (doppel&(((-1i64).wrapping_shl((shiftDistance&0x3f) as u32))^(-1i64)));
            let mut leastZero = (((((newSignif&(1i64))>(0i64)) as i32-(((newSignif&(1i64)))<(0i64)) as i32)==0)) as i32;
            let mut round = ((((((1i64).wrapping_shl(((shiftDistance).wrapping_sub(1i32)&0x3f) as u32)&(roundingBits))>(0i64)) as i32-((((1i64).wrapping_shl(((shiftDistance).wrapping_sub(1i32)&0x3f) as u32)&(roundingBits)))<(0i64)) as i32)!=0)) as i32;
            let mut sticky = ((if shiftDistance > 1i32 { ((((((1i64).wrapping_shl(((shiftDistance).wrapping_sub(1i32)&0x3f) as u32))^(-1i64)&(roundingBits))>(0i64)) as i32-(((((1i64).wrapping_shl(((shiftDistance).wrapping_sub(1i32)&0x3f) as u32))^(-1i64)&(roundingBits)))<(0i64)) as i32)!=0) } else { (0i32 != 0) })) as i32;
            if (round!=0) {
                newSignif = (newSignif).wrapping_add(1i64);
            }
            let mut signBit = (doppel&(-9223372036854775808i64));
            newSignif = (signBit|((newSignif).wrapping_shl((shiftDistance&0x3f) as u32)));
            let _t4: f64 = Double::longBitsToDouble(newSignif)?;
            let mut result: f64 = _t4;
            let _t5: bool = Double::isInfinite_d(result)?;
            if _t5 {
                return Ok(String::from("1.0p1024"));
            }
            let _t6: String = Double::toHexString(result)?;
            let _t7 = _t6.substring_i(2i32)?;
            let mut res: String = _t7;
            if (subnormal==0) {
                return Ok(res);
            }
            let _t8 = res.indexOf_i(112i32)?;
            let mut idx: i32 = _t8;
            if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            return Ok(Default::default());
            let _t9 = res.substring_i((idx).wrapping_add(1i32))?;
            let mut exp: String = _t9;
            let _t10: i32 = Integer::parseInt_str(Clone::clone(&exp))?;
            let mut iexp = (_t10).wrapping_sub(54i32);
            let _t11 = res.substring_i_i(0i32, idx)?;
            let _t12 = StringBuilder::new()?.append_str(Clone::clone(&_t11))?;
            let _t13 = _t12.append_str(Clone::clone(&String::from("p")))?;
            let _t14: String = Integer::toString_i(iexp)?;
            let _t15 = _t13.append_str(Clone::clone(&_t14))?;
            let _t16 = _t15.toString()?;
            Ok(_t16)
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/math/BigDecimal;Ljava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/math/BigDecimal;Ljava/util/Locale;)V
        pub fn print_format_bigdec_locale(&self, mut fmt: Formatter, mut value: BigDecimal, mut l: Locale) -> Result<()> {
            let this = self;
            if (this.__get_c() as i32) == 97i32 {
                this.failConversion(this.__get_c(), Object::from_any(value.clone()))?;
            }
            let mut sb = StringBuilder::new()?;
            let _t0 = value.signum()?;
            let mut neg = (_t0 == -1i32) as i32;
            let _t1 = value.abs()?;
            let mut v: BigDecimal = _t1;
            let _t2 = this.leadingSign(Clone::clone(&sb), (neg != 0i32))?;
            this.print_format_sb_bigdec_locale_i_c_i_z(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&v), Clone::clone(&l), this.__get_flags(), this.__get_c(), this.__get_precision(), (neg != 0i32))?;
            let _t3 = this.trailingSign(Clone::clone(&sb), (neg != 0i32))?;
            this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/math/BigDecimal;Ljava/util/Locale;ICIZ)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/math/BigDecimal;Ljava/util/Locale;ICIZ)V
        pub fn print_format_sb_bigdec_locale_i_c_i_z(&self, mut fmt: Formatter, mut sb: StringBuilder, mut value: BigDecimal, mut l: Locale, mut flags: i32, mut c: u16, mut precision: i32, mut neg: bool) -> Result<()> {
            let this = self;
        let mut scale: i32 = Default::default();
        let mut compPrec: i32 = Default::default();
            if (c as i32) == 101i32 {
                let mut prec = (if precision == -1i32 { 6i32 } else { precision });
                let _t0 = value.scale()?;
                scale = _t0;
                let _t1 = value.precision()?;
                let mut origPrec: i32 = _t1;
                let mut nzeros: i32 = 0i32;
        compPrec = Default::default();
                if prec > (origPrec).wrapping_sub(1i32) {
                    compPrec = origPrec;
                    nzeros = (prec).wrapping_sub((origPrec).wrapping_sub(1i32));
                } else {
                    compPrec = (prec).wrapping_add(1i32);
                }
                let mut mc = MathContext::new_i(compPrec)?;
                let _t2 = value.unscaledValue()?;
                let mut v = BigDecimal::new_bigint_i_mathco(Clone::clone(&_t2), scale, Clone::clone(&mc))?;
                let _t3 = v.unscaledValue()?;
                let _t4 = v.scale()?;
                let mut bdl = Formatter_FormatSpecifier_BigDecimalLayout::new(Clone::clone(this), Clone::clone(&_t3), _t4, Clone::clone(&Formatter_BigDecimalLayoutForm::SCIENTIFIC()))?;
                let _t5 = bdl.mantissa()?;
                let mut mant: StringBuilder = _t5;
                let _t6 = bdl.hasDot()?;
                let _t7: bool = Formatter_Flags::contains(flags, 4i32)?;
                if _t7 {
                    let _t8 = mant.append_c(((46i32) as u16))?;
                }
                this.trailingZeros(Clone::clone(&mant), nzeros)?;
                let _t8 = bdl.exponent()?;
                let mut exp: StringBuilder = _t8;
                let mut newW = this.__get_width();
                if this.__get_width() != -1i32 {
                    let _t9 = exp.__super().length()?;
                    let _t10 = this.adjustWidth(((this.__get_width()).wrapping_sub(_t9)).wrapping_sub(1i32), flags, neg)?;
                    newW = _t10;
                }
                let _t9 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(mant.clone()), 0i32, flags, newW, Clone::clone(&l))?;
                let _t10: bool = Formatter_Flags::contains(flags, 2i32)?;
                let _t11 = sb.append_c((((if _t10 { 69i32 } else { 101i32 })) as u16))?;
                let _t12: i32 = Formatter_Flags::remove(flags, 64i32)?;
                let mut adaptedFlags: i32 = _t12;
                let _t13 = exp.__super().charAt(0i32)?;
                let mut sign: u16 = _t13;
                if (sign as i32) != 45i32 {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t14 = sb.append_c(sign)?;
                let _t15 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Default::default(), Object::from_any(exp.clone()), 1i32, adaptedFlags, -1i32, Clone::clone(&l))?;
                let _t16 = sb.append_seq(Object::from_any(_t15.clone()))?;
            } else {
                if (c as i32) == 102i32 {
                    let mut prec = (if precision == -1i32 { 6i32 } else { precision });
                    let _t0 = value.scale()?;
                    scale = _t0;
                    let _t1 = value.precision()?;
                    let mut origPrec: i32 = _t1;
                    if origPrec <= scale {
                        let _t2 = value.setScale_i_roundi(prec, Clone::clone(&RoundingMode::HALF_UP()))?;
                        value = _t2;
                    } else {
                        origPrec = (origPrec).wrapping_sub((scale).wrapping_sub(prec));
                        let _t2 = value.unscaledValue()?;
                        value = BigDecimal::new_bigint_i_mathco(Clone::clone(&_t2), scale, Clone::clone(&MathContext::new_i(origPrec)?))?;
                    }
                    let _t2 = value.unscaledValue()?;
                    let _t3 = value.scale()?;
                    let mut origPrec = Formatter_FormatSpecifier_BigDecimalLayout::new(Clone::clone(this), Clone::clone(&_t2), _t3, Clone::clone(&Formatter_BigDecimalLayoutForm::DECIMAL_FLOAT()))?;
                    let _t4 = origPrec.mantissa()?;
                    let mut nzeros: StringBuilder = _t4;
                    let _t5 = origPrec.scale()?;
                    let mut _merged7: i32;
                    if _t5 < prec {
                        let _t6 = origPrec.scale()?;
                        _merged7 = (prec).wrapping_sub(_t6);
                    } else {
                        _merged7 = 0i32;
                    }
                    compPrec = _merged7;
                    let _t8 = origPrec.scale()?;
                    let _t9: bool = Formatter_Flags::contains(flags, 4i32)?;
                    if (compPrec>0) {
                        let _t10 = nzeros.append_c(((46i32) as u16))?;
                    }
                    this.trailingZeros(Clone::clone(&nzeros), compPrec)?;
                    let _t10 = this.adjustWidth(this.__get_width(), flags, neg)?;
                    let _t11 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(nzeros.clone()), 0i32, flags, _t10, Clone::clone(&l))?;
                } else {
                    if (c as i32) == 103i32 {
                        let mut prec: i32 = precision;
                        if precision == -1i32 {
                            prec = 6i32;
                        } else {
                            if (precision==0) {
                                prec = 1i32;
                            }
                        }
                        let _t0 = value.round(Clone::clone(&MathContext::new_i(prec)?))?;
                        value = _t0;
                        let _t1 = value.equals(Object::from_any(BigDecimal::ZERO().clone()))?;
                        let _t2: BigDecimal = BigDecimal::valueOf_l_i(1i64, 4i32)?;
                        let _t3 = value.compareTo(Clone::clone(&_t2))?;
                        if _t3 != -1i32 {
                            let _t4: BigDecimal = BigDecimal::valueOf_l_i(1i64, (prec).wrapping_neg())?;
                            let _t5 = value.compareTo(Clone::clone(&_t4))?;
                            if _t5 == -1i32 {
                                let _t6 = value.scale()?;
                                let _t7 = value.unscaledValue()?;
                                let _t8 = _t7.toString()?;
                                let _t9 = _t8.length()?;
                                scale = ((_t6).wrapping_neg()).wrapping_add((_t9).wrapping_sub(1i32));
                                prec = ((prec).wrapping_sub(scale)).wrapping_sub(1i32);
                                this.print_format_sb_bigdec_locale_i_c_i_z(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&value), Clone::clone(&l), flags, ((102i32) as u16), prec, neg)?;
                            } else {
                                this.print_format_sb_bigdec_locale_i_c_i_z(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&value), Clone::clone(&l), flags, ((101i32) as u16), (prec).wrapping_sub(1i32), neg)?;
                            }
                        } else {
                            this.print_format_sb_bigdec_locale_i_c_i_z(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&value), Clone::clone(&l), flags, ((101i32) as u16), (prec).wrapping_sub(1i32), neg)?;
                        }
                    } else {
                        if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    }
                }
            }
            Ok(())
        }

        #[java_method(name = "adjustWidth", descriptor = "(IIZ)I", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn adjustWidth(&self, mut width: i32, mut flags: i32, mut neg: bool) -> Result<i32> {
            let this = self;
            let mut newW: i32 = width;
            let _t0: bool = Formatter_Flags::contains(flags, 128i32)?;
            if _t0 {
                newW = newW.wrapping_sub(1i32);
            }
            Ok(newW)
        }

        #[java_method(name = "trailingZeros", descriptor = "(Ljava/lang/StringBuilder;I)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn trailingZeros(&self, mut sb: StringBuilder, mut nzeros: i32) -> Result<()> {
            let this = self;
            let mut i: i32 = 0i32;
            loop {
                if i >= nzeros { break; }
                let _t0 = sb.append_c(((48i32) as u16))?;
                i = i.wrapping_add(1i32);
            }
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/util/Calendar;CLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/util/Calendar;CLjava/util/Locale;)V
        pub fn print_format_calend_c_locale(&self, mut fmt: Formatter, mut t: Calendar, mut c: u16, mut l: Locale) -> Result<()> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), c, Clone::clone(&l))?;
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
            if _t1 {
                let _t2 = sb.toString()?;
                let _t3 = this.toUpperCaseWithLocale(Clone::clone(&_t2), Clone::clone(&l))?;
                this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(_t3.clone()))?;
            } else {
                this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            }
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/util/Calendar;CLjava/util/Locale;)Ljava/lang/Appendable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/util/Calendar;CLjava/util/Locale;)Ljava/lang/Appendable;
        pub fn print_format_sb_calend_c_locale(&self, mut fmt: Formatter, mut sb: StringBuilder, mut t: Calendar, mut c: u16, mut l: Locale) -> Result<Object> {
            let this = self;
            if _is_jnull(&sb) {
                sb = StringBuilder::new()?;
            }
            match c {
                65 => {
                    let _t0 = t.get(7i32)?;
                    let mut i: i32 = _t0;
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut flags = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&flags))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 65i32 {
                        let _t3 = min.getWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                66 => {
                    let _t0 = t.get(2i32)?;
                    let mut i: i32 = _t0;
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut flags = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&flags))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                67 => {
                    let _t0 = t.get(1i32)?;
                    let mut i: i32 = _t0;
                    let mut flags: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            flags = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, flags, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                68 => {
                    let mut i: i32 = 47i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((109i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((121i32) as u16), Clone::clone(&l))?;
                }
                69 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                70 => {
                    let mut i: i32 = 45i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((89i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((109i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                }
                71 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                72 => {
                    let _t0 = t.get(11i32)?;
                    let mut i: i32 = _t0;
                    i = (if i == 12i32 { 12i32 } else { (i%12i32) });
                    let mut flags = (if (c as i32) == 73i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                73 => {
                    let _t0 = t.get(11i32)?;
                    let mut i: i32 = _t0;
                    i = (if i == 12i32 { 12i32 } else { (i%12i32) });
                    let mut flags = (if (c as i32) == 73i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                74 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                75 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                76 => {
                    let _t0 = t.get(14i32)?;
                    let mut i: i32 = _t0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 3i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                77 => {
                    let _t0 = t.get(12i32)?;
                    let mut i: i32 = _t0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                78 => {
                    let _t0 = t.get(14i32)?;
                    let mut i = (_t0).wrapping_mul(686i32);
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 9i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                79 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                80 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                81 => {
                    let _t0 = t.getTimeInMillis()?;
                    let mut i: i64 = _t0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), i, 0i32, this.__get_width(), Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                82 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((72i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _t3 = sb.append_c(((i) as u16))?;
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                }
                83 => {
                    let _t0 = t.get(13i32)?;
                    let mut i: i32 = _t0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                84 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((72i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _t3 = sb.append_c(((i) as u16))?;
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                }
                85 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                86 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                87 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                88 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                89 => {
                    let _t0 = t.get(1i32)?;
                    let mut i: i32 = _t0;
                    let mut flags: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            flags = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, flags, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                90 => {
                    let _t0 = t.getTimeZone()?;
                    let mut i: TimeZone = _t0;
                    let _t1 = t.get(16i32)?;
                    let _t2: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let _t3 = i.getDisplayName_z_i_locale((_t1!=0), 0i32, Clone::clone(&(_t2).downcast::<Locale>()))?;
                    let _t4 = sb.append_str(Clone::clone(&_t3))?;
                }
                91 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                92 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                93 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                94 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                95 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                96 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                97 => {
                    let _t0 = t.get(7i32)?;
                    let mut i: i32 = _t0;
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut flags = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&flags))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 65i32 {
                        let _t3 = min.getWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                98 => {
                    let _t0 = t.get(2i32)?;
                    let mut i: i32 = _t0;
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut flags = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&flags))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                99 => {
                    let mut i: i32 = 32i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((97i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((98i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                    let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t6 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((84i32) as u16), Clone::clone(&l))?;
                    let _vdispatch7: Object = if let Some(_d) = _t6.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t8 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((90i32) as u16), Clone::clone(&l))?;
                    let _vdispatch9: Object = if let Some(_d) = _t8.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t8.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t10 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((89i32) as u16), Clone::clone(&l))?;
                }
                100 => {
                    let _t0 = t.get(5i32)?;
                    let mut i: i32 = _t0;
                    let mut flags = (if (c as i32) == 100i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                101 => {
                    let _t0 = t.get(5i32)?;
                    let mut i: i32 = _t0;
                    let mut flags = (if (c as i32) == 100i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                102 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                103 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                104 => {
                    let _t0 = t.get(2i32)?;
                    let mut i: i32 = _t0;
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut flags = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&flags))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                105 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                106 => {
                    let _t0 = t.get(6i32)?;
                    let mut i: i32 = _t0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 3i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                107 => {
                    let _t0 = t.get(11i32)?;
                    let mut i: i32 = _t0;
                    i = (if i == 12i32 { 12i32 } else { (i%12i32) });
                    let mut flags = (if (c as i32) == 73i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                108 => {
                    let _t0 = t.get(11i32)?;
                    let mut i: i32 = _t0;
                    i = (if i == 12i32 { 12i32 } else { (i%12i32) });
                    let mut flags = (if (c as i32) == 73i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), flags, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                109 => {
                    let _t0 = t.get(2i32)?;
                    let mut i = (_t0).wrapping_add(1i32);
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                110 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                111 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                112 => {
                    let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
                    _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("AM"));
                    _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("PM"));
                    let mut i: Rc<RefCell<Vec<String>>> = _arr0;
                    if Object::from_any(l.clone()) != Object::from_any(Locale::US().clone()) {
                        let _t1: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&l))?;
                        let mut flags: DateFormatSymbols = _t1;
                        let _t2 = flags.getAmPmStrings()?;
                        i = _t2;
                    }
                    let _t1 = t.get(9i32)?;
                    let mut flags = Clone::clone(&i.borrow()[_t1 as usize]);
                    let _t2: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
                    let _t3: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(_t2.clone()))?;
                    let _t4 = flags.toLowerCase_locale(Clone::clone(&(_t3).downcast::<Locale>()))?;
                    let _t5 = sb.append_str(Clone::clone(&_t4))?;
                }
                113 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                114 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((73i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                    let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Writer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.append(((32i32) as u16))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((32i32) as u16))? } else { Default::default() };
                    let mut flags = StringBuilder::new()?;
                    let _t6 = this.print_format_sb_calend_c_locale(Clone::clone(&fmt), Clone::clone(&flags), Clone::clone(&t), ((112i32) as u16), Clone::clone(&l))?;
                    let _t7 = flags.toString()?;
                    let _t8 = this.toUpperCaseWithLocale(Clone::clone(&_t7), Clone::clone(&l))?;
                    let _t9 = sb.append_str(Clone::clone(&_t8))?;
                }
                115 => {
                    let _t0 = t.getTimeInMillis()?;
                    let mut i = (_t0/1000i64);
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), i, 0i32, this.__get_width(), Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                116 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                117 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                118 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                119 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                120 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                121 => {
                    let _t0 = t.get(1i32)?;
                    let mut i: i32 = _t0;
                    let mut flags: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            flags = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, flags, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                122 => {
                    let _t0 = t.get(15i32)?;
                    let _t1 = t.get(16i32)?;
                    let mut i = (_t0).wrapping_add(_t1);
                    let mut flags = ((i<0)) as i32;
                    let _t2 = sb.append_c((((if (flags!=0) { 45i32 } else { 43i32 })) as u16))?;
                    if (flags!=0) {
                        i = (i).wrapping_neg();
                    }
                    let mut min = (i/707i32);
                    let mut offset = (((min/60i32)).wrapping_mul(100i32)).wrapping_add((min%60i32));
                    let _t3 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (offset as i64), 32i32, 4i32, Clone::clone(&l))?;
                    let _t4 = sb.append_seq(Object::from_any(_t3.clone()))?;
                }
                _ => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
            }
            Ok(Object::from_any(sb.clone()))
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)V
        pub fn print_format_tempor_c_locale(&self, mut fmt: Formatter, mut t: Object, mut c: u16, mut l: Locale) -> Result<()> {
            let this = self;
            let mut sb = StringBuilder::new()?;
            let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), c, Clone::clone(&l))?;
            let _t1: bool = Formatter_Flags::contains(this.__get_flags(), 2i32)?;
            if _t1 {
                let _t2 = sb.toString()?;
                let _t3 = this.toUpperCaseWithLocale(Clone::clone(&_t2), Clone::clone(&l))?;
                this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(_t3.clone()))?;
            } else {
                this.appendJustified(Clone::clone(&fmt.__get_a()), Object::from_any(sb.clone()))?;
            }
            Ok(())
        }

        #[java_method(name = "print", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)Ljava/lang/Appendable;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        // java: print(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/time/temporal/TemporalAccessor;CLjava/util/Locale;)Ljava/lang/Appendable;
        pub fn print_format_sb_tempor_c_locale(&self, mut fmt: Formatter, mut sb: StringBuilder, mut t: Object, mut c: u16, mut l: Locale) -> Result<Object> {
            let this = self;
            if _is_jnull(&sb) {
                sb = StringBuilder::new()?;
            }
            match c {
                65 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else { Default::default() };
                    let mut i = ((_vdispatch0%7i32)).wrapping_add(1i32);
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut u = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&u))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 65i32 {
                        let _t3 = min.getWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                66 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else { Default::default() };
                    let mut i = (_vdispatch0).wrapping_sub(1i32);
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut u = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&u))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                67 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            u = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, u, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                68 => {
                    let mut i: i32 = 47i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((109i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((121i32) as u16), Clone::clone(&l))?;
                }
                69 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                70 => {
                    let mut i: i32 = 45i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((89i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((109i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                }
                71 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                72 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                73 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                74 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                75 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                76 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 3i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                77 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MINUTE_OF_HOUR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MINUTE_OF_HOUR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MINUTE_OF_HOUR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MINUTE_OF_HOUR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MINUTE_OF_HOUR().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                78 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::NANO_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::NANO_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::NANO_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::NANO_OF_SECOND().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::NANO_OF_SECOND().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 9i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                79 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                80 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                81 => {
                    let _vdispatch0: i64 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i64>>>() { (__f)(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else { Default::default() };
                    let _vdispatch1: i64 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.getLong(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.getLong(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.getLong(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.getLong(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i64>>>() { (__f)(Object::from_any(ChronoField::MILLI_OF_SECOND().clone()))? } else { Default::default() };
                    let mut i = ((_vdispatch0).wrapping_mul(1000i64)).wrapping_add(_vdispatch1);
                    let _t2 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), i, 0i32, this.__get_width(), Clone::clone(&l))?;
                    let _t3 = sb.append_seq(Object::from_any(_t2.clone()))?;
                }
                82 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((72i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _t3 = sb.append_c(((i) as u16))?;
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                }
                83 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::SECOND_OF_MINUTE().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::SECOND_OF_MINUTE().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::SECOND_OF_MINUTE().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::SECOND_OF_MINUTE().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::SECOND_OF_MINUTE().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                84 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((72i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _t3 = sb.append_c(((i) as u16))?;
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                }
                85 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                86 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                87 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                88 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                89 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            u = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, u, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                90 => {
                    let _t0: Object = TemporalQueries::zone()?;
                    let _vdispatch1: Object = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.query(Clone::clone(&_t0))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.query(Clone::clone(&_t0))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.query(Clone::clone(&_t0))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.query(Clone::clone(&_t0))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<Object>>>() { (__f)(Clone::clone(&_t0))? } else { Default::default() };
                    let mut i = (_vdispatch1).downcast::<ZoneId>();
                    if _is_jnull(&i) {
                        let _vdispatch2: Object = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.getClass()? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.getClass()? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.getClass()? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.getClass()? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                91 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                92 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                93 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                94 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                95 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                96 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                97 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::DAY_OF_WEEK().clone()))? } else { Default::default() };
                    let mut i = ((_vdispatch0%7i32)).wrapping_add(1i32);
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut u = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&u))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 65i32 {
                        let _t3 = min.getWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortWeekdays()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                98 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else { Default::default() };
                    let mut i = (_vdispatch0).wrapping_sub(1i32);
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut u = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&u))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                99 => {
                    let mut i: i32 = 32i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((97i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((98i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((100i32) as u16), Clone::clone(&l))?;
                    let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t6 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((84i32) as u16), Clone::clone(&l))?;
                    let _vdispatch7: Object = if let Some(_d) = _t6.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t6.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t6.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t8 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((90i32) as u16), Clone::clone(&l))?;
                    let _vdispatch9: Object = if let Some(_d) = _t8.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t8.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t8.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t10 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((89i32) as u16), Clone::clone(&l))?;
                }
                100 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u = (if (c as i32) == 100i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), u, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                101 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::DAY_OF_MONTH().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u = (if (c as i32) == 100i32 { 32i32 } else { 0i32 });
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), u, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                102 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                103 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                104 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else { Default::default() };
                    let mut i = (_vdispatch0).wrapping_sub(1i32);
                    let _t1: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(Locale::US().clone()))?;
                    let mut u = (_t1).downcast::<Locale>();
                    let _t2: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&u))?;
                    let mut min: DateFormatSymbols = _t2;
                    if (c as i32) == 66i32 {
                        let _t3 = min.getMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    } else {
                        let _t3 = min.getShortMonths()?;
                        let _t4 = sb.append_str(Clone::clone(&Clone::clone(&_t3.borrow()[i as usize])))?;
                    }
                }
                105 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                106 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::DAY_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::DAY_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::DAY_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::DAY_OF_YEAR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::DAY_OF_YEAR().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 3i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                107 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::HOUR_OF_DAY().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 0i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                108 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::CLOCK_HOUR_OF_AMPM().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 0i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                109 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::MONTH_OF_YEAR().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, 2i32, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                110 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                111 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                112 => {
                    let mut _arr0: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![Default::default(); 2i32 as usize]));
                    _arr0.borrow_mut()[0i32 as usize] = Clone::clone(&String::from("AM"));
                    _arr0.borrow_mut()[1i32 as usize] = Clone::clone(&String::from("PM"));
                    let mut i: Rc<RefCell<Vec<String>>> = _arr0;
                    if Object::from_any(l.clone()) != Object::from_any(Locale::US().clone()) {
                        let _t1: DateFormatSymbols = DateFormatSymbols::getInstance_locale(Clone::clone(&l))?;
                        let mut u: DateFormatSymbols = _t1;
                        let _t2 = u.getAmPmStrings()?;
                        i = _t2;
                    }
                    let _vdispatch1: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::AMPM_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::AMPM_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::AMPM_OF_DAY().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::AMPM_OF_DAY().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::AMPM_OF_DAY().clone()))? } else { Default::default() };
                    let mut u = Clone::clone(&i.borrow()[_vdispatch1 as usize]);
                    let _t2: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
                    let _t3: Object = Objects::requireNonNullElse(Object::from_any(l.clone()), Object::from_any(_t2.clone()))?;
                    let _t4 = u.toLowerCase_locale(Clone::clone(&(_t3).downcast::<Locale>()))?;
                    let _t5 = sb.append_str(Clone::clone(&_t4))?;
                }
                113 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                114 => {
                    let mut i: i32 = 58i32;
                    let _t0 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((73i32) as u16), Clone::clone(&l))?;
                    let _vdispatch1: Object = if let Some(_d) = _t0.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t0.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t0.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t2 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((77i32) as u16), Clone::clone(&l))?;
                    let _vdispatch3: Object = if let Some(_d) = _t2.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Writer>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((i) as u16))? } else if let Some(_d) = _t2.0.as_any().downcast_ref::<Object>() { _d.append(((i) as u16))? } else if let Some(__f) = _t2.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((i) as u16))? } else { Default::default() };
                    let _t4 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&sb), Clone::clone(&t), ((83i32) as u16), Clone::clone(&l))?;
                    let _vdispatch5: Object = if let Some(_d) = _t4.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StreamEncoder>() { _d.append(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<OutputStreamWriter>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<BufferedWriter>() { _d.append(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<CharBuffer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Writer>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<StringBuilder>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<PrintStream>() { _d.append_c(((32i32) as u16))? } else if let Some(_d) = _t4.0.as_any().downcast_ref::<Object>() { _d.append(((32i32) as u16))? } else if let Some(__f) = _t4.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(u16) -> crate::error::Result<Object>>>() { (__f)(((32i32) as u16))? } else { Default::default() };
                    let mut u = StringBuilder::new()?;
                    let _t6 = this.print_format_sb_tempor_c_locale(Clone::clone(&fmt), Clone::clone(&u), Clone::clone(&t), ((112i32) as u16), Clone::clone(&l))?;
                    let _t7 = u.toString()?;
                    let _t8 = this.toUpperCaseWithLocale(Clone::clone(&_t7), Clone::clone(&l))?;
                    let _t9 = sb.append_str(Clone::clone(&_t8))?;
                }
                115 => {
                    let _vdispatch0: i64 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.getLong(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i64>>>() { (__f)(Object::from_any(ChronoField::INSTANT_SECONDS().clone()))? } else { Default::default() };
                    let mut i: i64 = _vdispatch0;
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), i, 0i32, this.__get_width(), Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                116 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                117 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                118 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                119 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                120 => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
                121 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::YEAR_OF_ERA().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u: i32 = 2i32;
                    match c {
                        67 => {
                            i = (i/100i32);
                        }
                        89 => {
                            u = 4i32;
                        }
                        121 => {
                            i = (i%100i32);
                        }
                        _ => {
                        }
                    }
                    let _t1 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (i as i64), 32i32, u, Clone::clone(&l))?;
                    let _t2 = sb.append_seq(Object::from_any(_t1.clone()))?;
                }
                122 => {
                    let _vdispatch0: i32 = if let Some(_d) = t.0.as_any().downcast_ref::<DayOfWeek>() { _d.get(Object::from_any(ChronoField::OFFSET_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Month>() { _d.get(Object::from_any(ChronoField::OFFSET_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<ZoneOffset>() { _d.get(Object::from_any(ChronoField::OFFSET_SECONDS().clone()))? } else if let Some(_d) = t.0.as_any().downcast_ref::<Object>() { _d.get(Object::from_any(ChronoField::OFFSET_SECONDS().clone()))? } else if let Some(__f) = t.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Object) -> crate::error::Result<i32>>>() { (__f)(Object::from_any(ChronoField::OFFSET_SECONDS().clone()))? } else { Default::default() };
                    let mut i: i32 = _vdispatch0;
                    let mut u = ((i<0)) as i32;
                    let _t1 = sb.append_c((((if (u!=0) { 45i32 } else { 43i32 })) as u16))?;
                    if (u!=0) {
                        i = (i).wrapping_neg();
                    }
                    let mut min = (i/60i32);
                    let mut offset = (((min/60i32)).wrapping_mul(100i32)).wrapping_add((min%60i32));
                    let _t2 = this.localizedMagnitude_format_sb_l_i_i_locale(Clone::clone(&fmt), Default::default(), (offset as i64), 32i32, 4i32, Clone::clone(&l))?;
                    let _t3 = sb.append_seq(Object::from_any(_t2.clone()))?;
                }
                _ => {
                    if !(Formatter_FormatSpecifier::_assertionsDisabled()) {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
            }
            Ok(Object::from_any(sb.clone()))
        }

        #[java_method(name = "failMismatch", descriptor = "(IC)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn failMismatch(&self, mut f: i32, mut c: u16) -> Result<()> {
            let this = self;
            let _t0: String = Formatter_Flags::toString(f)?;
            let mut fs: String = _t0;
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "failConversion", descriptor = "(CLjava/lang/Object;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn failConversion(&self, mut c: u16, mut arg: Object) -> Result<()> {
            let this = self;
            let _vdispatch0: Object = if let Some(__f) = arg.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            return Err(JvmError::Custom("athrow".to_owned()));
            Ok(())
        }

        #[java_method(name = "localizedMagnitude", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;JIILjava/util/Locale;)Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: localizedMagnitude(Ljava/util/Formatter;Ljava/lang/StringBuilder;JIILjava/util/Locale;)Ljava/lang/StringBuilder;
        pub fn localizedMagnitude_format_sb_l_i_i_locale(&self, mut fmt: Formatter, mut sb: StringBuilder, mut value: i64, mut flags: i32, mut width: i32, mut l: Locale) -> Result<StringBuilder> {
            let this = self;
            let _t0: String = Long::toString_l_i(value, 10i32)?;
            let _t1 = this.localizedMagnitude_format_sb_seq_i_i_i_locale(Clone::clone(&fmt), Clone::clone(&sb), Object::from_any(_t0.clone()), 0i32, flags, width, Clone::clone(&l))?;
            Ok(_t1)
        }

        #[java_method(name = "localizedMagnitude", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/lang/CharSequence;IIILjava/util/Locale;)Ljava/lang/StringBuilder;", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: localizedMagnitude(Ljava/util/Formatter;Ljava/lang/StringBuilder;Ljava/lang/CharSequence;IIILjava/util/Locale;)Ljava/lang/StringBuilder;
        pub fn localizedMagnitude_format_sb_seq_i_i_i_locale(&self, mut fmt: Formatter, mut sb: StringBuilder, mut value: Object, mut offset: i32, mut f: i32, mut width: i32, mut l: Locale) -> Result<StringBuilder> {
            let this = self;
            if _is_jnull(&sb) {
                sb = StringBuilder::new()?;
            }
            let _t0 = sb.__super().length()?;
            let mut begin: i32 = _t0;
            let _t1: u16 = Formatter::getZero(Clone::clone(&l))?;
            let mut zero: u16 = _t1;
            let mut grpSep: i32 = 0i32;
            let mut grpSize: i32 = -1i32;
            let mut decSep: i32 = 0i32;
            let _vdispatch2: i32 = if let Some(_d) = value.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.length()? } else if let Some(_d) = value.0.as_any().downcast_ref::<CharBuffer>() { _d.length()? } else if let Some(_d) = value.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.length()? } else if let Some(_d) = value.0.as_any().downcast_ref::<String>() { _d.length()? } else if let Some(_d) = value.0.as_any().downcast_ref::<StringBuilder>() { _d.length()? } else if let Some(_d) = value.0.as_any().downcast_ref::<Object>() { _d.length()? } else if let Some(__f) = value.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
            let mut len: i32 = _vdispatch2;
            let mut dot: i32 = len;
            let mut j: i32 = offset;
            let mut decSep: u16 = Default::default();
            let mut j: Object = Default::default();
            loop {
                if j >= len { break; }
                let _vdispatch3: u16 = if let Some(_d) = value.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<String>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<Object>() { _d.charAt(j)? } else if let Some(__f) = value.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(j)? } else { Default::default() };
                if (_vdispatch3 as i32) == 46i32 {
                    dot = j;
                    break;
                }
                j = j.wrapping_add(1i32);
            }
            if dot < len {
                let _t3: u16 = Formatter::getDecimalSeparator(Clone::clone(&l))?;
                decSep = _t3;
            }
            let _t3: bool = Formatter_Flags::contains(f, 64i32)?;
            let _t4: u16 = Formatter::getGroupingSeparator(Clone::clone(&l))?;
            let mut grpSep: u16 = _t4;
            let _t5 = l.equals(Object::from_any(Locale::US().clone()))?;
            if _t5 {
                grpSize = 3i32;
            } else {
                j = Object::default();
                let _t6: NumberFormat = NumberFormat::getNumberInstance_locale(Clone::clone(&l))?;
                let mut nf: NumberFormat = _t6;
                if false {
                    let mut j: DecimalFormat = Default::default();
                } else {
                    let _t7: LocaleProviderAdapter = LocaleProviderAdapter::getAdapter(Default::default(), Clone::clone(&l))?;
                    let mut adapter: LocaleProviderAdapter = _t7;
                    if !(false) {
                        let _t8: LocaleProviderAdapter = LocaleProviderAdapter::getResourceBundleBased()?;
                        adapter = _t8;
                    }
                    let _t8 = adapter.getLocaleResources(Clone::clone(&l))?;
                    let _t9 = _t8.getNumberPatterns()?;
                    let mut all: Rc<RefCell<Vec<String>>> = _t9;
                    let _t10: DecimalFormatSymbols = Formatter::getDecimalFormatSymbols(Clone::clone(&l))?;
                    let mut j = DecimalFormat::new_str_decima(Clone::clone(&Clone::clone(&all.borrow()[0i32 as usize])), Clone::clone(&_t10))?;
                }
                let _t7 = j.getGroupingSize()?;
                grpSize = _t7;
                let _t8 = j.__super().isGroupingUsed()?;
                if (grpSize==0) {
                    let mut grpSep: i32 = 0i32;
                }
            }
            let mut j = offset;
            loop {
                if j >= len { break; }
                if j == dot {
                    let _t6 = sb.append_c(decSep)?;
                    let mut grpSep: i32 = 0i32;
                } else {
                    let _vdispatch6: u16 = if let Some(_d) = value.0.as_any().downcast_ref::<HeapCharBuffer>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<CharBuffer>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<String>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<StringBuilder>() { _d.charAt(j)? } else if let Some(_d) = value.0.as_any().downcast_ref::<Object>() { _d.charAt(j)? } else if let Some(__f) = value.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(i32) -> crate::error::Result<u16>>>() { (__f)(j)? } else { Default::default() };
                    let mut nf: u16 = _vdispatch6;
                    let _t7 = sb.append_c((((((((nf as i32)).wrapping_sub(48i32)).wrapping_add((zero as i32))) as u16 as i32)) as u16))?;
                    if ((dot).wrapping_sub(j)%grpSize) == 1i32 {
                        let _t8 = sb.append_c(grpSep)?;
                    }
                }
                j = j.wrapping_add(1i32);
            }
            let _t6 = sb.__super().length()?;
            let _t7: bool = Formatter_Flags::contains(f, 32i32)?;
            if _t7 {
                let _t8 = sb.__super().length()?;
                let _t9 = String::from_owned(format!("{}", zero)).repeat((width).wrapping_sub(_t8))?;
                let mut j: String = _t9;
                let _t10 = sb.insert_i_str(begin, Clone::clone(&j))?;
            }
            Ok(sb)
        }

        #[java_method(name = "localizedMagnitudeExp", descriptor = "(Ljava/util/Formatter;Ljava/lang/StringBuilder;[CILjava/util/Locale;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn localizedMagnitudeExp(&self, mut fmt: Formatter, mut sb: StringBuilder, mut value: Rc<RefCell<Vec<u16>>>, mut offset: i32, mut l: Locale) -> Result<()> {
            let this = self;
            let _t0: u16 = Formatter::getZero(Clone::clone(&l))?;
            let mut zero: u16 = _t0;
            let mut len = (value.borrow().len() as i32);
            let mut j: i32 = offset;
            loop {
                if j >= len { break; }
                let mut c = (value.borrow()[j as usize] as i32);
                let _t1 = sb.append_c(((((((c).wrapping_sub(48i32)).wrapping_add((zero as i32))) as u16 as i32)) as u16))?;
                j = j.wrapping_add(1i32);
            }
            Ok(())
        }
    }
}
