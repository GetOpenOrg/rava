#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/OutputStreamWriter",
    super_class = "java/io/Writer",
    interfaces  = "",
    access      = "public",
    source      = "OutputStreamWriter.java",
))]
pub struct OutputStreamWriter {
    #[cfg_attr(any(), java_field(name = "se", descriptor = "Lsun/nio/cs/StreamEncoder;", access = "private final"))]
    pub se: Field<Object>,
}

impl OutputStreamWriter {
    #[cfg_attr(any(), java_method(name = "lockFor", descriptor = "(Ljava/io/OutputStreamWriter;)Ljava/lang/Object;", access = "private static"))]
    pub fn lockFor(writer: Object) -> Result<Object> {
        let _t0 = writer.getClass()?;
        let mut clazz: Object = _t0;
        let _t1: Object = InternalLock::newLockOr(writer)?;
        return Ok(_t1);
        Ok(writer)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/lang/String;)V
    pub fn new__output_str(out: Object, charsetName: String) -> Result<Self> {
        let this = Self { se: Field::new(Default::default()) };
        /* invokespecial Method java/io/Writer.<init>:(Ljava/lang/Object;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = OutputStreamWriter::lockFor(this)?;
        let _t1: Object = StreamEncoder::forOutputStreamWriter(out, _t0, charsetName)?;
        this.se.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new__output(out: Object) -> Result<Self> {
        let this = Self { se: Field::new(Default::default()) };
        /* invokespecial Method java/io/Writer.<init>:(Ljava/lang/Object;)V */
        let _t0: Object = OutputStreamWriter::lockFor(this)?;
        let mut ps: Object = out;
        let _t1 = ps.charset()?;
        let _t2: Object = Charset::defaultCharset()?;
        let _t3: Object = StreamEncoder::forOutputStreamWriter(true, _t1, _t2)?;
        out.se.set(_t3);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V
    pub fn new__output_charse(out: Object, cs: Object) -> Result<Self> {
        let this = Self { se: Field::new(Default::default()) };
        /* invokespecial Method java/io/Writer.<init>:(Ljava/lang/Object;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = OutputStreamWriter::lockFor(this)?;
        let _t1: Object = StreamEncoder::forOutputStreamWriter(out, _t0, cs)?;
        this.se.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V
    pub fn new__output_charse(out: Object, enc: Object) -> Result<Self> {
        let this = Self { se: Field::new(Default::default()) };
        /* invokespecial Method java/io/Writer.<init>:(Ljava/lang/Object;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        let _t0: Object = OutputStreamWriter::lockFor(this)?;
        let _t1: Object = StreamEncoder::forOutputStreamWriter(out, _t0, enc)?;
        this.se.set(_t1);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "getEncoding", descriptor = "()Ljava/lang/String;", access = "public"))]
    pub fn getEncoding(&self) -> Result<String> {
        let this = self;
        let _t0 = this.se.get().getEncoding()?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "flushBuffer", descriptor = "()V"))]
    pub fn flushBuffer(&self) -> Result<()> {
        let this = self;
        this.se.get().flushBuffer()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, c: i32) -> Result<()> {
        let this = self;
        this.se.get().write(c)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([CII)V", access = "public"))]
    // java: write([CII)V
    pub fn write__arr_c_i_i(&self, cbuf: Vec<u16>, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.se.get().write(cbuf, off, len)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public"))]
    // java: write(Ljava/lang/String;II)V
    pub fn write__str_i_i(&self, str: String, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.se.get().write(str, off, len)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/Writer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;II)Ljava/io/Writer;
    pub fn append__seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        csq = String::from("null");
        let _t0 = csq.subSequence(start, end)?;
        let _t1 = this.append(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/Writer;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;)Ljava/io/Writer;
    pub fn append__seq(&self, csq: Object) -> Result<Object> {
        let this = self;
        this.se.get().write(csq)?;
        this.se.get().write(String::from_owned(format!("{}", csq)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public"))]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.se.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.se.get().close()?;
        Ok(())
    }
}
