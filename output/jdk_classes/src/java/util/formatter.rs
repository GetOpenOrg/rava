#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/util/Formatter",
    super_class = "java/lang/Object",
    interfaces  = "java/io/Closeable,java/io/Flushable",
    access      = "public final",
    source      = "Formatter.java",
))]
pub struct Formatter {
    #[cfg_attr(any(), java_field(name = "a", descriptor = "Ljava/lang/Appendable;", access = "private"))]
    pub a: Field<Object>,
    #[cfg_attr(any(), java_field(name = "l", descriptor = "Ljava/util/Locale;", access = "private final"))]
    pub l: Field<Object>,
    #[cfg_attr(any(), java_field(name = "lastException", descriptor = "Ljava/io/IOException;", access = "private"))]
    pub lastException: Field<Object>,
}

impl Formatter {
    #[cfg_attr(any(), java_method(name = "getDecimalFormatSymbols", descriptor = "(Ljava/util/Locale;)Ljava/text/DecimalFormatSymbols;", access = "private static"))]
    pub fn getDecimalFormatSymbols(locale: Object) -> Result<Object> {
        let mut dfs: Object = Formatter::DFS();
        let _t0 = dfs.getLocale()?;
        let _t1 = _t0.equals(locale)?;
        return Ok(dfs);
        let _t2: Object = DecimalFormatSymbols::getInstance(locale)?;
        dfs = _t2;
        Formatter::DFS(dfs);
        Ok(dfs)
    }

    #[cfg_attr(any(), java_method(name = "getZero", descriptor = "(Ljava/util/Locale;)C", access = "private static"))]
    pub fn getZero(locale: Object) -> Result<u16> {
        let _t0: Object = Formatter::getDecimalFormatSymbols(locale)?;
        let _t1 = _t0.getZeroDigit()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getDecimalSeparator", descriptor = "(Ljava/util/Locale;)C", access = "private static"))]
    pub fn getDecimalSeparator(locale: Object) -> Result<u16> {
        let _t0: Object = Formatter::getDecimalFormatSymbols(locale)?;
        let _t1 = _t0.getDecimalSeparator()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "getGroupingSeparator", descriptor = "(Ljava/util/Locale;)C", access = "private static"))]
    pub fn getGroupingSeparator(locale: Object) -> Result<u16> {
        let _t0: Object = Formatter::getDecimalFormatSymbols(locale)?;
        let _t1 = _t0.getGroupingSeparator()?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn toCharset(csn: String) -> Result<Object> {
        let _t0: Object = Objects::requireNonNull(csn, String::from("charsetName"))?;
        let _t1: Object = Charset::forName(csn)?;
        return Ok(_t1);
        let mut unused: i32 = todo!("stack underflow");
        return Err(JvmError::Custom(String::from("athrow")));
    }

    #[cfg_attr(any(), java_method(name = "nonNullAppendable", descriptor = "(Ljava/lang/Appendable;)Ljava/lang/Appendable;", access = "private static"))]
    pub fn nonNullAppendable(a: Object) -> Result<Object> {
        return Ok(String::new());
        Ok(a)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Locale;Ljava/lang/Appendable;)V", access = "private"))]
    // java: <init>(Ljava/util/Locale;Ljava/lang/Appendable;)V
    pub fn new__locale_append(l: Object, a: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        /* invokespecial Method java/lang/Object.<init>:()V */
        this.a.set(a);
        this.l.set(l);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V", access = "private"))]
    // java: <init>(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V
    pub fn new__charse_locale_file(charset: Object, l: Object, file: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "()V", access = "public"))]
    // java: <init>()V
    pub fn new() -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Appendable;)V", access = "public"))]
    // java: <init>(Ljava/lang/Appendable;)V
    pub fn new__append(a: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t1: Object = Formatter::nonNullAppendable(a)?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/util/Locale;)V
    pub fn new__locale(l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/Appendable;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/lang/Appendable;Ljava/util/Locale;)V
    pub fn new__append_locale(a: Object, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Formatter::nonNullAppendable(a)?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(fileName: String) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(fileName: String, csn: String) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/lang/String;Ljava/util/Locale;)V
    pub fn new__str_str_locale(fileName: String, csn: String, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Formatter::toCharset(csn)?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/nio/charset/Charset;Ljava/util/Locale;)V
    pub fn new__str_charse_locale(fileName: String, charset: Object, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Objects::requireNonNull(charset, String::from("charset"))?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public"))]
    // java: <init>(Ljava/io/File;)V
    pub fn new__file(file: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/io/File;Ljava/lang/String;)V
    pub fn new__file_str(file: Object, csn: String) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/io/File;Ljava/lang/String;Ljava/util/Locale;)V
    pub fn new__file_str_locale(file: Object, csn: String, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Formatter::toCharset(csn)?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/io/File;Ljava/nio/charset/Charset;Ljava/util/Locale;)V
    pub fn new__file_charse_locale(file: Object, charset: Object, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Objects::requireNonNull(charset, String::from("charset"))?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/nio/charset/Charset;Ljava/util/Locale;Ljava/io/File;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/PrintStream;)V", access = "public"))]
    // java: <init>(Ljava/io/PrintStream;)V
    pub fn new__prints(ps: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t1: Object = Objects::requireNonNull(ps)?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new__output(os: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/lang/String;)V
    pub fn new__output_str(os: Object, csn: String) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        let _t0: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/lang/String;Ljava/util/Locale;)V
    pub fn new__output_str_locale(os: Object, csn: String, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;Ljava/util/Locale;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/nio/charset/Charset;Ljava/util/Locale;)V
    pub fn new__output_charse_locale(os: Object, charset: Object, l: Object) -> Result<Self> {
        let this = Self { a: Field::new(Default::default()), l: Field::new(Default::default()), lastException: Field::new(Default::default()) };
        /* invokespecial Method java/util/Formatter.<init>:(Ljava/util/Locale;Ljava/lang/Appendable;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "locale", descriptor = "()Ljava/util/Locale;", access = "public"))]
    pub fn locale(&self) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        Ok(this.l.get())
    }

    #[cfg_attr(any(), java_method(name = "out", descriptor = "()Ljava/lang/Appendable;", access = "public"))]
    pub fn out(&self) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        Ok(this.a.get())
    }

    #[cfg_attr(any(), java_method(name = "toString", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn toString(&self) -> Result<String> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = this.a.get().toString()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public"))]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.a.get().flush()?;
        let mut ioe: bool = true;
        this.lastException.set(ioe);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        return Ok(());
        this.a.get().close()?;
        /* TODO: aconst_null  */
        true.a.set(this);
        let mut ioe: Object = this.a.get();
        this.lastException.set(ioe);
        /* TODO: aconst_null  */
        this.a.get().a.set(this);
        let mut local_2: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        todo!("stack underflow").a.set(this);
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "ioException", descriptor = "()Ljava/io/IOException;", access = "public"))]
    pub fn ioException(&self) -> Result<Object> {
        let this = self;
        Ok(this.lastException.get())
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;", access = "public"))]
    // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
    pub fn format__str_arr_obj(&self, format: String, args: Vec<Object>) -> Result<Object> {
        let this = self;
        let _t0 = this.format(this.l.get(), format, args)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;", access = "public"))]
    // java: format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
    pub fn format__locale_str_arr_obj(&self, l: Object, format: String, args: Vec<Object>) -> Result<Object> {
        let this = self;
        this.ensureOpen()?;
        let mut last: i32 = -1i32;
        let mut lasto: i32 = -1i32;
        let _t0: Object = Formatter::parse(format)?;
        let mut fsa: Object = _t0;
        let _t1 = fsa.iterator()?;
        let mut local_7: Object = _t1;
        loop {
            let _t0 = local_7.hasNext()?;
            if _t0==0i32 { break; }
            let _t0 = local_7.next()?;
            let mut fs: Object = _t0;
            let _t1 = fs.index()?;
            let mut index: i32 = _t1;
            /* TODO: tableswitch default:213 low:-2 high:0 */
            /* TODO: aconst_null  */
            index.print(fs, this, l)?;
            let _t2 = fs.toString()?;
            return Err(JvmError::Custom(String::from("athrow")));
            /* TODO: aconst_null  */
            this.print(args, args[last as usize].clone(), l)?;
            lasto = lasto.wrapping_add(1i32);
            last = lasto;
            let _t3 = fs.toString()?;
            return Err(JvmError::Custom(String::from("athrow")));
            /* TODO: aconst_null  */
            this.print(args, args[lasto as usize].clone(), l)?;
            last = (index).wrapping_sub(1i32);
            let _t4 = fs.toString()?;
            return Err(JvmError::Custom(String::from("athrow")));
            /* TODO: aconst_null  */
            this.print(args, args[last as usize].clone(), l)?;
            let mut x: Object = fs;
            this.lastException.set(x);
        }
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "parse", descriptor = "(Ljava/lang/String;)Ljava/util/List;", access = "static"))]
    pub fn parse(s: String) -> Result<Object> {
        let mut al: ArrayList<_> = ArrayList::<_>::new()?;
        let mut i: i32 = 0i32;
        let _t0 = s.length()?;
        let mut max: i32 = _t0;
        /* TODO: aconst_null  */
        let mut m: i32 = todo!("stack underflow");
        loop {
            if i >= max { break; }
            let _t0 = s.indexOf(37i32, i)?;
            let mut n: i32 = _t0;
            let _t1 = al.add(Formatter_FixedString::new(s, i, max)?)?;
            let _t2 = al.add(Formatter_FixedString::new(s, i, n)?)?;
            i = (n).wrapping_add(1i32);
            return Err(JvmError::Custom(String::from("athrow")));
            let _t3 = s.charAt(i)?;
            let mut c: i32 = _t3;
            let _t4: bool = Formatter$Conversion::isValid(c)?;
            let _t5 = al.add(Formatter_FormatSpecifier::new(c)?)?;
            i = i.wrapping_add(1i32);
            let _t6 = Formatter::FORMAT_SPECIFIER_PATTERN().matcher(s)?;
            m = _t6;
            let _t7 = m.find(n)?;
            let _t8 = m.start()?;
            let _t9 = al.add(Formatter_FormatSpecifier::new(s, m)?)?;
            let _t10 = m.end()?;
            i = _t10;
            return Err(JvmError::Custom(String::from("athrow")));
        }
        Ok(al)
    }
}
