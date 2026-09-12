#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/PrintStream",
    super_class = "java/io/FilterOutputStream",
    interfaces  = "java/lang/Appendable,java/io/Closeable",
    access      = "public",
    source      = "PrintStream.java",
))]
pub struct PrintStream {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private final"))]
    pub lock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "autoFlush", descriptor = "Z", access = "private final"))]
    pub autoFlush: Field<bool>,
    #[cfg_attr(any(), java_field(name = "trouble", descriptor = "Z", access = "private"))]
    pub trouble: Field<bool>,
    #[cfg_attr(any(), java_field(name = "formatter", descriptor = "Ljava/util/Formatter;", access = "private"))]
    pub formatter: Field<Object>,
    #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private final"))]
    pub charset: Field<Object>,
    #[cfg_attr(any(), java_field(name = "textOut", descriptor = "Ljava/io/BufferedWriter;", access = "private"))]
    pub textOut: Field<Object>,
    #[cfg_attr(any(), java_field(name = "charOut", descriptor = "Ljava/io/OutputStreamWriter;", access = "private"))]
    pub charOut: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closing", descriptor = "Z", access = "private"))]
    pub closing: Field<bool>,
}

impl PrintStream {
    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "private static"))]
    pub fn requireNonNull(obj: Object, message: String) -> Result<Object> {
        panic!("{}", /* NullPointerException::new(message)? */);
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn toCharset(csn: String) -> Result<Object> {
        let _t0: Object = PrintStream::requireNonNull(csn, String::from("charsetName"))?;
        let _t1: Object = Charset::forName(csn)?;
        return Ok(_t1);
        let mut unused: i32 = todo!("stack underflow");
        panic!("{}", /* UnsupportedEncodingException::new(csn)? */);
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/io/OutputStream;)V", access = "private"))]
    // java: <init>(ZLjava/io/OutputStream;)V
    pub fn new__z_output(autoFlush: bool, out: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V */
        this.trouble.set(0i32);
        this.closing.set(0i32);
        this.autoFlush.set(autoFlush);
        let mut ps: Object = out;
        let _t0 = ps.charset()?;
        let _t1: Object = Charset::defaultCharset()?;
        _t0.charset.set(_t1);
        this.charOut.set(OutputStreamWriter::new(this, this.charset.get())?);
        this.textOut.set(BufferedWriter::new(this.charOut.get())?);
        let _t2 = this.getClass()?;
        let _t3: Object = InternalLock::newLockOrNull()?;
        this.lock.set(_t3);
        /* TODO: aconst_null  */
        10i32.lock.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V", access = "private"))]
    // java: <init>(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V
    pub fn new__z_charse_output(autoFlush: bool, charset: Object, out: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new__output(out: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;Z)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Z)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;Z)V
    pub fn new__output_z(out: Object, autoFlush: bool) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::requireNonNull(out, String::from("Null output stream"))?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;ZLjava/lang/String;)V
    pub fn new__output_z_str(out: Object, autoFlush: bool, encoding: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::requireNonNull(out, String::from("Null output stream"))?;
        let _t1: Object = PrintStream::toCharset(encoding)?;
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V", access = "public"))]
    // java: <init>(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V
    pub fn new__output_z_charse(out: Object, autoFlush: bool, charset: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V */
        this.trouble.set(0i32);
        this.closing.set(0i32);
        this.autoFlush.set(autoFlush);
        this.charOut.set(OutputStreamWriter::new(this, charset)?);
        this.textOut.set(BufferedWriter::new(this.charOut.get())?);
        this.charset.set(charset);
        let _t0 = this.getClass()?;
        let _t1: Object = InternalLock::newLockOrNull()?;
        this.lock.set(_t1);
        /* TODO: aconst_null  */
        10i32.lock.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;)V
    pub fn new__str(fileName: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/lang/String;)V
    pub fn new__str_str(fileName: String, csn: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::toCharset(csn)?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public"))]
    // java: <init>(Ljava/lang/String;Ljava/nio/charset/Charset;)V
    pub fn new__str_charse(fileName: String, charset: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::requireNonNull(charset, String::from("charset"))?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public"))]
    // java: <init>(Ljava/io/File;)V
    pub fn new__file(file: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public"))]
    // java: <init>(Ljava/io/File;Ljava/lang/String;)V
    pub fn new__file_str(file: Object, csn: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::toCharset(csn)?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;)V", access = "public"))]
    // java: <init>(Ljava/io/File;Ljava/nio/charset/Charset;)V
    pub fn new__file_charse(file: Object, charset: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: Object = PrintStream::requireNonNull(charset, String::from("charset"))?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        panic!("{}", /* IOException::new(String::from("Stream closed"))? */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public"))]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implFlush()?;
        this.lock.get().unlock()?;
        let mut local_1: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* local_1 */);
        local_1 = this;
        /* TODO: monitorenter  */
        this.implFlush()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", access = "private"))]
    pub fn implFlush(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().flush()?;
        let mut x: i32 = todo!("stack underflow");
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implClose()?;
        this.lock.get().unlock()?;
        let mut local_1: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* local_1 */);
        local_1 = this;
        /* TODO: monitorenter  */
        this.implClose()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implClose", descriptor = "()V", access = "private"))]
    pub fn implClose(&self) -> Result<()> {
        let this = self;
        this.closing.set(1i32);
        this.textOut.get().close()?;
        this.out.get().close()?;
        let mut x: bool = this.closing.get();
        this.trouble.set(1i32);
        /* TODO: aconst_null  */
        todo!("stack underflow").textOut.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").charOut.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").out.set(this);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkError", descriptor = "()Z", access = "public"))]
    pub fn checkError(&self) -> Result<bool> {
        let this = self;
        this.flush()?;
        let mut local_2: Object = this.out.get();
        let mut ps: Object = local_2;
        let _t0 = ps.checkError()?;
        return Ok(_t0);
        Ok(this.trouble.get())
    }

    #[cfg_attr(any(), java_method(name = "setError", descriptor = "()V", access = "protected"))]
    pub fn setError(&self) -> Result<()> {
        let this = self;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "clearError", descriptor = "()V", access = "protected"))]
    pub fn clearError(&self) -> Result<()> {
        let this = self;
        this.trouble.set(0i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(b)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(b)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private"))]
    // java: implWrite(I)V
    pub fn implWrite__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().write(b)?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public"))]
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, buf: Object, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(buf, off, len)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(buf, off, len)?;
        /* TODO: monitorexit  */
        let mut local_5: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_5 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([BII)V", access = "private"))]
    // java: implWrite([BII)V
    pub fn implWrite__arr_b_i_i(&self, buf: Object, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().write(buf, off, len)?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([B)V", access = "public"))]
    // java: write([B)V
    pub fn write__arr_b(&self, buf: Object) -> Result<()> {
        let this = self;
        this.write(buf, 0i32, (buf.len() as i32))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeBytes", descriptor = "([B)V", access = "public"))]
    pub fn writeBytes(&self, buf: Object) -> Result<()> {
        let this = self;
        this.write(buf, 0i32, (buf.len() as i32))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([C)V", access = "private"))]
    // java: write([C)V
    pub fn write__arr_c(&self, buf: Object) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(buf)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(buf)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([C)V", access = "private"))]
    // java: implWrite([C)V
    pub fn implWrite__arr_c(&self, buf: Object) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().write(buf)?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        let mut i: i32 = 0i32;
        loop {
            if i >= (buf.len() as i32) { break; }
            this.out.get().flush()?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeln", descriptor = "([C)V", access = "private"))]
    // java: writeln([C)V
    pub fn writeln__arr_c(&self, buf: Object) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWriteln(buf)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWriteln(buf)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "([C)V", access = "private"))]
    // java: implWriteln([C)V
    pub fn implWriteln__arr_c(&self, buf: Object) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().write(buf)?;
        this.textOut.get().newLine()?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    // java: write(Ljava/lang/String;)V
    pub fn write__str(&self, s: String) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(s)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(s)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    // java: implWrite(Ljava/lang/String;)V
    pub fn implWrite__str(&self, s: String) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().write(s)?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        let _t0 = s.indexOf(10i32)?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeln", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    // java: writeln(Ljava/lang/String;)V
    pub fn writeln__str(&self, s: String) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWriteln(s)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWriteln(s)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    // java: implWriteln(Ljava/lang/String;)V
    pub fn implWriteln__str(&self, s: String) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().write(s)?;
        this.textOut.get().newLine()?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newLine", descriptor = "()V", access = "private"))]
    pub fn newLine(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implNewLine()?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implNewLine()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implNewLine", descriptor = "()V", access = "private"))]
    pub fn implNewLine(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.textOut.get().newLine()?;
        this.textOut.get().flushBuffer()?;
        this.charOut.get().flushBuffer()?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Z)V", access = "public"))]
    // java: print(Z)V
    pub fn print__z(&self, b: bool) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", b)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(C)V", access = "public"))]
    // java: print(C)V
    pub fn print__c(&self, c: u16) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", c)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(I)V", access = "public"))]
    // java: print(I)V
    pub fn print__i(&self, i: i32) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", i)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(J)V", access = "public"))]
    // java: print(J)V
    pub fn print__l(&self, l: i64) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", l)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(F)V", access = "public"))]
    // java: print(F)V
    pub fn print__f(&self, f: f32) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", f)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(D)V", access = "public"))]
    // java: print(D)V
    pub fn print__d(&self, d: f64) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", d)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "([C)V", access = "public"))]
    // java: print([C)V
    pub fn print__arr_c(&self, s: Object) -> Result<()> {
        let this = self;
        this.write(s)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: print(Ljava/lang/String;)V
    pub fn print__str(&self, s: String) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", s)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    // java: print(Ljava/lang/Object;)V
    pub fn print__obj(&self, obj: Object) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", obj)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "()V", access = "public"))]
    // java: println()V
    pub fn println(&self) -> Result<()> {
        let this = self;
        this.newLine()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Z)V", access = "public"))]
    // java: println(Z)V
    pub fn println__z(&self, x: bool) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(C)V", access = "public"))]
    // java: println(C)V
    pub fn println__c(&self, x: u16) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(I)V", access = "public"))]
    // java: println(I)V
    pub fn println__i(&self, x: i32) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(J)V", access = "public"))]
    // java: println(J)V
    pub fn println__l(&self, x: i64) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_3: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_4: java/io/PrintStream = local_3;
        /* TODO: monitorexit  */
        panic!("{}", /* local_4 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(F)V", access = "public"))]
    // java: println(F)V
    pub fn println__f(&self, x: f32) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(D)V", access = "public"))]
    // java: println(D)V
    pub fn println__d(&self, x: f64) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_3: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_4: java/io/PrintStream = local_3;
        /* TODO: monitorexit  */
        panic!("{}", /* local_4 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "([C)V", access = "public"))]
    // java: println([C)V
    pub fn println__arr_c(&self, x: Object) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(x)?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    // java: println(Ljava/lang/String;)V
    pub fn println__str(&self, x: String) -> Result<()> {
        let this = self;
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", x)))?;
        let mut local_2: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(x)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_3: java/io/PrintStream = local_2;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    // java: println(Ljava/lang/Object;)V
    pub fn println__obj(&self, x: Object) -> Result<()> {
        let this = self;
        let mut s: String = String::from_owned(format!("{}", x));
        let _t0 = this.getClass()?;
        this.writeln(String::from_owned(format!("{}", s)))?;
        let mut local_3: java/io/PrintStream = this;
        /* TODO: monitorenter  */
        this.print(s)?;
        this.newLine()?;
        /* TODO: monitorexit  */
        let mut local_4: java/io/PrintStream = local_3;
        /* TODO: monitorexit  */
        panic!("{}", /* local_4 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "printf", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    // java: printf(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
    pub fn printf__str_arr_obj(&self, format: String, args: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.format(format, args)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "printf", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    // java: printf(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
    pub fn printf__locale_str_arr_obj(&self, l: Object, format: String, args: Object) -> Result<Object> {
        let this = self;
        let _t0 = this.format(l, format, args)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    // java: format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
    pub fn format__str_arr_obj(&self, format: String, args: Object) -> Result<Object> {
        let this = self;
        this.lock.get().lock()?;
        this.implFormat(format, args)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implFormat(format, args)?;
        /* TODO: monitorexit  */
        let mut local_4: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_4 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "private"))]
    // java: implFormat(Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn implFormat__str_arr_obj(&self, format: String, args: Object) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = this.formatter.get().locale()?;
        let _t1: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        this.formatter.set(Formatter::new(this)?);
        let _t2: Object = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t3 = this.formatter.get().format(_t2, format, args)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    // java: format(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
    pub fn format__locale_str_arr_obj(&self, l: Object, format: String, args: Object) -> Result<Object> {
        let this = self;
        this.lock.get().lock()?;
        this.implFormat(l, format, args)?;
        this.lock.get().unlock()?;
        let mut x: Object = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implFormat(l, format, args)?;
        /* TODO: monitorexit  */
        let mut local_5: Object = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_5 */);
        x = x;
        let _t0: Object = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V", access = "private"))]
    // java: implFormat(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V
    pub fn implFormat__locale_str_arr_obj(&self, l: Object, format: String, args: Object) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = this.formatter.get().locale()?;
        this.formatter.set(Formatter::new(this, l)?);
        let _t1 = this.formatter.get().format(l, format, args)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;)Ljava/io/PrintStream;
    pub fn append__seq(&self, csq: Object) -> Result<Object> {
        let this = self;
        this.print(String::from_owned(format!("{}", csq)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;", access = "public"))]
    // java: append(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;
    pub fn append__seq_i_i(&self, csq: Object, start: i32, end: i32) -> Result<Object> {
        let this = self;
        csq = String::from("null");
        let _t0 = csq.subSequence(start, end)?;
        let _t1 = this.append(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/io/PrintStream;", access = "public"))]
    // java: append(C)Ljava/io/PrintStream;
    pub fn append__c(&self, c: u16) -> Result<Object> {
        let this = self;
        this.print(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public"))]
    pub fn charset(&self) -> Result<Object> {
        let this = self;
        Ok(this.charset.get())
    }
}
