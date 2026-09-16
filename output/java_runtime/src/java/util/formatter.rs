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
    #[binary_name       = "java/util/Formatter"]
    #[super_class       = "java/lang/Object"]
    #[interfaces        = "java/io/Closeable,java/io/Flushable"]
    #[access            = "public"]
    #[modifiers         = "final"]
    #[generic_signature = ""]
    #[is_abstract       = false]
    #[is_enum           = false]
    #[is_deprecated     = false]
    #[source            = "Formatter.java"]
    #[inner_classes     = "java/util/Locale$Category:java/util/Locale:Category:16409;java/util/Formatter$FormatString:java/util/Formatter:FormatString:1544;java/util/Formatter$FixedString:java/util/Formatter:FixedString:10;java/util/Formatter$Conversion:java/util/Formatter:Conversion:8;java/util/Formatter$FormatSpecifier:java/util/Formatter:FormatSpecifier:8;java/util/Formatter$DateTime:java/util/Formatter:DateTime:8;java/util/Formatter$Flags:java/util/Formatter:Flags:8;java/util/Formatter$BigDecimalLayoutForm:java/util/Formatter:BigDecimalLayoutForm:16409;java/util/Formatter$FormatSpecifier$BigDecimalLayout:java/util/Formatter$FormatSpecifier:BigDecimalLayout:2"]

    // ── 宏展开输入 ──────────────────────────────────────────────
    #[is_interface      = false]
    #[all_supertypes    = "java/io/Closeable;java/io/Flushable;java/lang/Object;java/util/Formatter"]
    #[has_to_string_method = true]

    pub struct Formatter {
        #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Appendable;", access = "private", modifiers = "", is_static = false))]
        pub a: Object,
        #[cfg_attr(any(), java_field(name = "l", descriptor = "Ljava/util/Locale;", access = "private", modifiers = "final", is_static = false))]
        pub l: Locale,
        #[cfg_attr(any(), java_field(name = "lastException", descriptor = "Ljava/io/IOException;", access = "private", modifiers = "", is_static = false))]
        pub lastException: IOException,
    }

    impl Formatter {
        #[cfg_attr(any(), java_field(name = "DFS", descriptor = "Ljava/text/DecimalFormatSymbols;", access = "private", modifiers = "static", is_static = true))]
        // static field: DFS:Ljava/text/DecimalFormatSymbols;
        pub fn DFS() -> DecimalFormatSymbols {
            panic!("stub: java/util/Formatter.DFS:Ljava/text/DecimalFormatSymbols;")
        }

        #[cfg_attr(any(), java_field(name = "FORMAT_SPECIFIER", descriptor = "Ljava/lang/String;", access = "package", modifiers = "static final", is_static = true, constant_value = "%(\\d+\\$)?([-#+ 0,(\\<]*)?(\\d+)?(\\.\\d+)?([tT])?([a-zA-Z%])"))]
        // static field: FORMAT_SPECIFIER:Ljava/lang/String;
        pub fn FORMAT_SPECIFIER() -> String {
            String::from("%(\\d+\\$)?([-#+ 0,(\\<]*)?(\\d+)?(\\.\\d+)?([tT])?([a-zA-Z%])")
        }

        #[cfg_attr(any(), java_field(name = "FORMAT_SPECIFIER_PATTERN", descriptor = "Ljava/util/regex/Pattern;", access = "package", modifiers = "static final", is_static = true))]
        // static field: FORMAT_SPECIFIER_PATTERN:Ljava/util/regex/Pattern;
        pub fn FORMAT_SPECIFIER_PATTERN() -> Pattern {
            panic!("stub: java/util/Formatter.FORMAT_SPECIFIER_PATTERN:Ljava/util/regex/Pattern;")
        }

        #[java_method(name = "getDecimalFormatSymbols", descriptor = "(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecimalFormatSymbols(mut locale: Locale) -> Result<DecimalFormatSymbols> {
            let mut dfs: DecimalFormatSymbols = Formatter::DFS();
            let _t0 = dfs.getLocale()?;
            let _t1 = _t0.equals(Object::from_any(locale.clone()))?;
            if _t1 {
                return Ok(dfs);
            }
            let _t2: DecimalFormatSymbols = DecimalFormatSymbols::getInstance_locale(Clone::clone(&locale))?;
            dfs = _t2;
            Formatter::set_DFS(dfs);
            Ok(dfs)
        }

        #[java_method(name = "getZero", descriptor = "(Ljava/util/Locale;)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getZero(mut locale: Locale) -> Result<u16> {
            let mut _merged2: i32;
            if _is_jnull(&locale) {
                _merged2 = 48i32;
            } else {
                let _t0: DecimalFormatSymbols = Formatter::getDecimalFormatSymbols(Clone::clone(&locale))?;
                let _t1 = _t0.getZeroDigit()?;
                _merged2 = (_t1 as i32);
            }
            Ok(((_merged2) as u16))
        }

        #[java_method(name = "getDecimalSeparator", descriptor = "(Ljava/util/Locale;)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getDecimalSeparator(mut locale: Locale) -> Result<u16> {
            let mut _merged2: i32;
            if _is_jnull(&locale) {
                _merged2 = 46i32;
            } else {
                let _t0: DecimalFormatSymbols = Formatter::getDecimalFormatSymbols(Clone::clone(&locale))?;
                let _t1 = _t0.getDecimalSeparator()?;
                _merged2 = (_t1 as i32);
            }
            Ok(((_merged2) as u16))
        }

        #[java_method(name = "getGroupingSeparator", descriptor = "(Ljava/util/Locale;)C", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn getGroupingSeparator(mut locale: Locale) -> Result<u16> {
            let mut _merged2: i32;
            if _is_jnull(&locale) {
                _merged2 = 44i32;
            } else {
                let _t0: DecimalFormatSymbols = Formatter::getDecimalFormatSymbols(Clone::clone(&locale))?;
                let _t1 = _t0.getGroupingSeparator()?;
                _merged2 = (_t1 as i32);
            }
            Ok(((_merged2) as u16))
        }

        #[java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn toCharset(csn: String) -> Result<Charset> {
            panic!("stub: java/util/Formatter.toCharset:(Ljava/lang/String;)Ljava/nio/charset/Charset;")
        }

        #[java_method(name = "nonNullAppendable", descriptor = "(Ljava/lang/Appendable;)Ljava/lang/Appendable;", access = "private", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn nonNullAppendable(mut a: Object) -> Result<Object> {
            if _is_jnull(&a) {
                return Ok(Object::from_any(StringBuilder::new()?.clone()));
            }
            Ok(a)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;Ljava/lang/Appendable;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/util/Locale;Ljava/lang/Appendable;)V
        pub fn new_locale_append(mut l: Locale, mut a: Object) -> Result<Self> {
            let mut this = Self::default();
            /* invokespecial Method java/lang/Object.<init>:()V (Object no-op) */
            this.__set_a(Clone::clone(&a));
            this.__set_l(Clone::clone(&l));
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException")]
        pub fn new_charse_locale_file(charset: Charset, l: Locale, file: Object) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V")
        }

        #[java_method(name = "<init>", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>()V
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            let _t0: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            this = Formatter::new_locale_append(Clone::clone(&_t0), Object::from_any(StringBuilder::new()?.clone()))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Appendable;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/Appendable;)V
        pub fn new_append(mut a: Object) -> Result<Self> {
            let mut this = Self::default();
            let _t0: Locale = Locale::getDefault_locale(Clone::clone(&Locale_Category::FORMAT()))?;
            let _t1: Object = Formatter::nonNullAppendable(Clone::clone(&a))?;
            this = Formatter::new_locale_append(Clone::clone(&_t0), Clone::clone(&_t1))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_locale(l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/Appendable;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: <init>(Ljava/lang/Appendable;Ljava/util/Locale;)V
        pub fn new_append_locale(mut a: Object, mut l: Locale) -> Result<Self> {
            let mut this = Self::default();
            let _t0: Object = Formatter::nonNullAppendable(Clone::clone(&a))?;
            this = Formatter::new_locale_append(Clone::clone(&l), Clone::clone(&_t0))?;
            Ok(this)
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException")]
        pub fn new_str(fileName: String) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_str_str(fileName: String, csn: String) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/lang/String;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_str_str_locale(fileName: String, csn: String, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn new_str_charse_locale(fileName: String, charset: Charset, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/lang/String;Ljava/nio/charset/Charset;Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException")]
        pub fn new_file(file: Object) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/File;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_file_str(file: Object, csn: String) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/File;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/FileNotFoundException,java/io/UnsupportedEncodingException")]
        pub fn new_file_str_locale(file: Object, csn: String, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/IOException")]
        pub fn new_file_charse_locale(file: Object, charset: Charset, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/File;Ljava/nio/charset/Charset;Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/PrintStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_prints(ps: PrintStream) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/PrintStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output(os: OutputStream) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/OutputStream;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_output_str(os: OutputStream, csn: String) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/OutputStream;Ljava/lang/String;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false, exceptions = "java/io/UnsupportedEncodingException")]
        pub fn new_output_str_locale(os: OutputStream, csn: String, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V")
        }

        #[java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn new_output_charse_locale(os: OutputStream, charset: Charset, l: Locale) -> Result<Self> {
            panic!("stub: java/util/Formatter.<init>:(Ljava/io/OutputStream;Ljava/nio/charset/Charset;Ljava/util/Locale;)V")
        }

        #[java_method(name = "locale", descriptor = "()Ljava/util/Locale;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn locale(&self) -> Result<Locale> {
            let this = self;
            this.ensureOpen()?;
            Ok(this.__get_l())
        }

        #[java_method(name = "out", descriptor = "()Ljava/lang/Appendable;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn out(&self) -> Result<Object> {
            let this = self;
            this.ensureOpen()?;
            Ok(this.__get_a())
        }

        #[java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn toString(&self) -> Result<String> {
            let this = self;
            this.ensureOpen()?;
            let _vdispatch0: String = if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<HeapCharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<StreamEncoder>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<OutputStreamWriter>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<BufferedWriter>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<CharBuffer>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<Writer>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<AbstractStringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<StringBuilder>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<PrintStream>() { _d.toString()? } else if let Some(_d) = this.__get_a().0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = this.__get_a().0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
            Ok(_vdispatch0)
        }

        #[java_method(name = "flush", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn flush(&self) -> Result<()> {
            panic!("stub: java/util/Formatter.flush:()V")
        }

        #[java_method(name = "close", descriptor = "()V", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn close(&self) -> Result<()> {
            panic!("stub: java/util/Formatter.close:()V")
        }

        #[java_method(name = "ensureOpen", descriptor = "()V", access = "private", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ensureOpen(&self) -> Result<()> {
            let this = self;
            if _is_jnull(&this.__get_a()) {
                return Err(JvmError::Custom("athrow".to_owned()));
            }
            Ok(())
        }

        #[java_method(name = "ioException", descriptor = "()Ljava/io/IOException;", access = "public", modifiers = "", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        pub fn ioException(&self) -> Result<IOException> {
            panic!("stub: java/util/Formatter.ioException:()Ljava/io/IOException;")
        }

        #[java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
        pub fn format_str_arr_obj(&self, mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<Formatter> {
            let this = self;
            let _t0 = this.format_locale_str_arr_obj(Clone::clone(&this.__get_l()), Clone::clone(&format), Clone::clone(&args))?;
            Ok(_t0)
        }

        #[java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;", access = "public", modifiers = "varargs", is_static    = false, is_native    = false, is_abstract  = false, is_synthetic = false)]
        // java: format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
        pub fn format_locale_str_arr_obj(&self, mut l: Locale, mut format: String, mut args: Rc<RefCell<Vec<Object>>>) -> Result<Formatter> {
            let this = self;
            this.ensureOpen()?;
            let mut last: i32 = -1i32;
            let mut lasto: i32 = -1i32;
            let _t0: Object = Formatter::parse(Clone::clone(&format))?;
            let mut fsa: Object = _t0;
            let _vdispatch1: Object = if let Some(_d) = fsa.0.as_any().downcast_ref::<AbstractList_RandomAccessSubList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<AbstractList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<ArrayList_SubList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<AbstractSequentialList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<Arrays_ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<AbstractList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<LinkedList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<ArrayList<Object>>() { _d.iterator()? } else if let Some(_d) = fsa.0.as_any().downcast_ref::<Object>() { _d.iterator()? } else if let Some(__f) = fsa.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
            let mut local_7: Object = _vdispatch1;
            loop {
                let _vdispatch2: bool = if let Some(_d) = local_7.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.hasNext()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.hasNext()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<Object>() { _d.hasNext()? } else if let Some(__f) = local_7.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<bool>>>() { (__f)()? } else { Default::default() };
                if !(_vdispatch2) { break; }
                let _vdispatch2: Object = if let Some(_d) = local_7.0.as_any().downcast_ref::<AbstractList_ListItr>() { _d.next()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<ArrayList_ListItr>() { _d.next()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<AbstractList_Itr>() { _d.next()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<ArrayList_Itr>() { _d.next()? } else if let Some(_d) = local_7.0.as_any().downcast_ref::<Object>() { _d.next()? } else if let Some(__f) = local_7.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<Object>>>() { (__f)()? } else { Default::default() };
                let mut fs: Object = _vdispatch2;
                let _vdispatch3: i32 = if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FormatSpecifier>() { _d.index()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FixedString>() { _d.index()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Object>() { _d.index()? } else if let Some(__f) = fs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<i32>>>() { (__f)()? } else { Default::default() };
                let mut index: i32 = _vdispatch3;
                match index {
                    -2 => {
                        if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FormatSpecifier>() { _d.print_format_obj_locale(Clone::clone(this), Clone::clone(&Object::default()), Clone::clone(&l))?; } else if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FixedString>() { _d.print(Clone::clone(this), Clone::clone(&Object::default()), Clone::clone(&l))?; } else if let Some(_d) = fs.0.as_any().downcast_ref::<Object>() { _d.print(Clone::clone(this), Clone::clone(&Object::default()), Clone::clone(&l))?; } else if let Some(__f) = fs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn(Formatter, Object, Locale) -> crate::error::Result<()>>>() { (__f)(Clone::clone(this), Clone::clone(&Object::default()), Clone::clone(&l))?; }
                    }
                    -1 => {
                        if last > ((args.borrow().len() as i32)).wrapping_sub(1i32) {
                            let _vdispatch4: String = if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FormatSpecifier>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FixedString>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = fs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    }
                    0 => {
                        lasto = lasto.wrapping_add(1i32);
                        last = lasto;
                        if lasto > ((args.borrow().len() as i32)).wrapping_sub(1i32) {
                            let _vdispatch4: String = if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FormatSpecifier>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FixedString>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = fs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    }
                    _ => {
                        last = (index).wrapping_sub(1i32);
                        if last > ((args.borrow().len() as i32)).wrapping_sub(1i32) {
                            let _vdispatch4: String = if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FormatSpecifier>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Formatter_FixedString>() { _d.toString()? } else if let Some(_d) = fs.0.as_any().downcast_ref::<Object>() { _d.toString()? } else if let Some(__f) = fs.0.as_any().downcast_ref::<std::rc::Rc<dyn Fn() -> crate::error::Result<String>>>() { (__f)()? } else { Default::default() };
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    }
                }
            }
            Ok(Clone::clone(this))
        }

        #[java_method(name = "parse", descriptor = "(Ljava/lang/String;)Ljava/util/List;", access = "package", modifiers = "static", is_static    = true, is_native    = false, is_abstract  = false, is_synthetic = false, generic_signature = "(Ljava/lang/String;)Ljava/util/List<Ljava/util/Formatter$FormatString;>;")]
        pub fn parse(mut s: String) -> Result<Object> {
            let mut al = ArrayList::<Object>::new()?;
            let mut i: i32 = 0i32;
            let _t0 = s.length()?;
            let mut max: i32 = _t0;
            let mut m: Object = Object::default();
            let mut m: Matcher = Default::default();
            loop {
                if i >= max { break; }
                let _t1 = s.indexOf_i_i(37i32, i)?;
                let mut n: i32 = _t1;
                if (n<0) {
                    let _t2 = al.add_obj(Object::from_any(Formatter_FixedString::new(Clone::clone(&s), i, max)?.clone()))?;
                    break;
                }
                if i != n {
                    let _t2 = al.add_obj(Object::from_any(Formatter_FixedString::new(Clone::clone(&s), i, n)?.clone()))?;
                }
                i = (n).wrapping_add(1i32);
                if i >= max {
                    return Err(JvmError::Custom("athrow".to_owned()));
                }
                let _t2 = s.charAt(i)?;
                let mut c: u16 = _t2;
                let _t3: bool = Formatter_Conversion::isValid(c)?;
                if _t3 {
                    let _t4 = al.add_obj(Object::from_any(Formatter_FormatSpecifier::new_c(c)?.clone()))?;
                    i = i.wrapping_add(1i32);
                } else {
                    if _is_jnull(&m) {
                        let _t4 = Formatter::FORMAT_SPECIFIER_PATTERN().matcher(Object::from_any(s.clone()))?;
                        m = _t4;
                    }
                    let _t4 = m.find_i(n)?;
                    if _t4 {
                        let _t5 = m.start()?;
                        if _t5 == n {
                            let _t6 = al.add_obj(Object::from_any(Formatter_FormatSpecifier::new_str_matche(Clone::clone(&s), Clone::clone(&m))?.clone()))?;
                            let _t7 = m.end()?;
                            i = _t7;
                        } else {
                            return Err(JvmError::Custom("athrow".to_owned()));
                        }
                    } else {
                        return Err(JvmError::Custom("athrow".to_owned()));
                    }
                }
            }
            Ok(Object::from_any(al.clone()))
        }
    }
}
