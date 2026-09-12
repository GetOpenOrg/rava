#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use crate::java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/PrintStream",
    super_class = "java/io/FilterOutputStream",
    interfaces  = "java/lang/Appendable,java/io/Closeable",
    access      = "public",
    source      = "PrintStream.java",
))]
pub struct PrintStream {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private final"))]
    pub lock: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "autoFlush", descriptor = "Z", access = "private final"))]
    pub autoFlush: Field<bool>,
    #[cfg_attr(any(), java_field(name = "trouble", descriptor = "Z", access = "private"))]
    pub trouble: Field<bool>,
    #[cfg_attr(any(), java_field(name = "formatter", descriptor = "Ljava/util/Formatter;", access = "private"))]
    pub formatter: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "charset", descriptor = "Ljava/nio/charset/Charset;", access = "private final"))]
    pub charset: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "textOut", descriptor = "Ljava/io/BufferedWriter;", access = "private"))]
    pub textOut: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "charOut", descriptor = "Ljava/io/OutputStreamWriter;", access = "private"))]
    pub charOut: Field<JvmObject>,
    #[cfg_attr(any(), java_field(name = "closing", descriptor = "Z", access = "private"))]
    pub closing: Field<bool>,
}

impl PrintStream {
    #[cfg_attr(any(), java_method(name = "requireNonNull", descriptor = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;", access = "private static"))]
    pub fn requireNonNull(obj: JvmObject, message: String) -> Result<JvmObject> {
        panic!("{}", /* NullPointerException::new(message)? */);
        Ok(obj)
    }

    #[cfg_attr(any(), java_method(name = "toCharset", descriptor = "(Ljava/lang/String;)Ljava/nio/charset/Charset;", access = "private static"))]
    pub fn toCharset(csn: String) -> Result<JvmObject> {
        let _t0: JvmObject = PrintStream::requireNonNull(csn, String::from("charsetName"))?;
        let _t1: JvmObject = Charset::forName(csn)?;
        return Ok(_t1);
        let mut unused: i32 = /* UNDERFLOW */;
        panic!("{}", /* UnsupportedEncodingException::new(csn)? */);
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/io/OutputStream;)V", access = "private"))]
    pub fn new(autoFlush: bool, out: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V */
        this.trouble.set(0i32);
        this.closing.set(0i32);
        this.autoFlush.set(autoFlush);
        let mut ps: JvmObject = out;
        let _t0 = ps.charset()?;
        let _t1: JvmObject = Charset::defaultCharset()?;
        _t0.charset.set(_t1);
        this.charOut.set(OutputStreamWriter::new(this, this.charset.get())?);
        this.textOut.set(BufferedWriter::new(this.charOut.get())?);
        let _t2 = this.getClass()?;
        let _t3: JvmObject = InternalLock::newLockOrNull()?;
        this.lock.set(_t3);
        /* TODO: aconst_null  */
        10i32.lock.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V", access = "private"))]
    pub fn new(autoFlush: bool, charset: JvmObject, out: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;)V", access = "public"))]
    pub fn new(out: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;Z)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;Z)V", access = "public"))]
    pub fn new(out: JvmObject, autoFlush: bool) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::requireNonNull(out, String::from("Null output stream"))?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/lang/String;)V", access = "public"))]
    pub fn new(out: JvmObject, autoFlush: bool, encoding: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::requireNonNull(out, String::from("Null output stream"))?;
        let _t1: JvmObject = PrintStream::toCharset(encoding)?;
        /* invokespecial Method java/io/PrintStream.<init>:(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/OutputStream;ZLjava/nio/charset/Charset;)V", access = "public"))]
    pub fn new(out: JvmObject, autoFlush: bool, charset: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V */
        this.trouble.set(0i32);
        this.closing.set(0i32);
        this.autoFlush.set(autoFlush);
        this.charOut.set(OutputStreamWriter::new(this, charset)?);
        this.textOut.set(BufferedWriter::new(this.charOut.get())?);
        this.charset.set(charset);
        let _t0 = this.getClass()?;
        let _t1: JvmObject = InternalLock::newLockOrNull()?;
        this.lock.set(_t1);
        /* TODO: aconst_null  */
        10i32.lock.set(this);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn new(fileName: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/String;)V", access = "public"))]
    pub fn new(fileName: String, csn: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::toCharset(csn)?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/nio/charset/Charset;)V", access = "public"))]
    pub fn new(fileName: String, charset: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::requireNonNull(charset, String::from("charset"))?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;)V", access = "public"))]
    pub fn new(file: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/lang/String;)V", access = "public"))]
    pub fn new(file: JvmObject, csn: String) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::toCharset(csn)?;
        /* invokespecial Method java/io/PrintStream.<init>:(ZLjava/nio/charset/Charset;Ljava/io/OutputStream;)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/File;Ljava/nio/charset/Charset;)V", access = "public"))]
    pub fn new(file: JvmObject, charset: JvmObject) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), autoFlush: Field::new(false), trouble: Field::new(false), formatter: Field::new(Default::default()), charset: Field::new(Default::default()), textOut: Field::new(Default::default()), charOut: Field::new(Default::default()), closing: Field::new(false) };
        let _t0: JvmObject = PrintStream::requireNonNull(charset, String::from("charset"))?;
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
        let mut local_1: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* local_1 */);
        local_1 = this;
        /* TODO: monitorenter  */
        this.implFlush()?;
        /* TODO: monitorexit  */
        let mut local_2: JvmObject = local_1;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", access = "private"))]
    pub fn implFlush(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().flush()?;
        let mut x: i32 = /* UNDERFLOW */;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implClose()?;
        this.lock.get().unlock()?;
        let mut local_1: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* local_1 */);
        local_1 = this;
        /* TODO: monitorenter  */
        this.implClose()?;
        /* TODO: monitorexit  */
        let mut local_2: JvmObject = local_1;
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
        /* UNDERFLOW */.textOut.set(this);
        /* TODO: aconst_null  */
        /* UNDERFLOW */.charOut.set(this);
        /* TODO: aconst_null  */
        /* UNDERFLOW */.out.set(this);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "checkError", descriptor = "()Z", access = "public"))]
    pub fn checkError(&self) -> Result<bool> {
        let this = self;
        this.flush()?;
        let mut local_2: JvmObject = this.out.get();
        let mut ps: JvmObject = local_2;
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
    pub fn write(&self, b: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(b)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(b)?;
        /* TODO: monitorexit  */
        let mut local_3: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private"))]
    pub fn implWrite(&self, b: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().write(b)?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([BII)V", access = "public"))]
    pub fn write(&self, buf: JvmObject, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(buf, off, len)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(buf, off, len)?;
        /* TODO: monitorexit  */
        let mut local_5: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_5 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([BII)V", access = "private"))]
    pub fn implWrite(&self, buf: JvmObject, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.out.get().write(buf, off, len)?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([B)V", access = "public"))]
    pub fn write(&self, buf: JvmObject) -> Result<()> {
        let this = self;
        this.write(buf, 0i32, (buf.len() as i32))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "writeBytes", descriptor = "([B)V", access = "public"))]
    pub fn writeBytes(&self, buf: JvmObject) -> Result<()> {
        let this = self;
        this.write(buf, 0i32, (buf.len() as i32))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([C)V", access = "private"))]
    pub fn write(&self, buf: JvmObject) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(buf)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(buf)?;
        /* TODO: monitorexit  */
        let mut local_3: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([C)V", access = "private"))]
    pub fn implWrite(&self, buf: JvmObject) -> Result<()> {
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
    pub fn writeln(&self, buf: JvmObject) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWriteln(buf)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWriteln(buf)?;
        /* TODO: monitorexit  */
        let mut local_3: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "([C)V", access = "private"))]
    pub fn implWriteln(&self, buf: JvmObject) -> Result<()> {
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
    pub fn write(&self, s: String) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(s)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWrite(s)?;
        /* TODO: monitorexit  */
        let mut local_3: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    pub fn implWrite(&self, s: String) -> Result<()> {
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
    pub fn writeln(&self, s: String) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWriteln(s)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implWriteln(s)?;
        /* TODO: monitorexit  */
        let mut local_3: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_3 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWriteln", descriptor = "(Ljava/lang/String;)V", access = "private"))]
    pub fn implWriteln(&self, s: String) -> Result<()> {
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
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implNewLine()?;
        /* TODO: monitorexit  */
        let mut local_2: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_2 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
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
    pub fn print(&self, b: bool) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", b)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(C)V", access = "public"))]
    pub fn print(&self, c: u16) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", c)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(I)V", access = "public"))]
    pub fn print(&self, i: i32) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", i)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(J)V", access = "public"))]
    pub fn print(&self, l: i64) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", l)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(F)V", access = "public"))]
    pub fn print(&self, f: f32) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", f)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(D)V", access = "public"))]
    pub fn print(&self, d: f64) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", d)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "([C)V", access = "public"))]
    pub fn print(&self, s: JvmObject) -> Result<()> {
        let this = self;
        this.write(s)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/String;)V", access = "public"))]
    pub fn print(&self, s: String) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", s)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "print", descriptor = "(Ljava/lang/Object;)V", access = "public"))]
    pub fn print(&self, obj: JvmObject) -> Result<()> {
        let this = self;
        this.write(String::from_owned(format!("{}", obj)))?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "()V", access = "public"))]
    pub fn println(&self) -> Result<()> {
        let this = self;
        this.newLine()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "println", descriptor = "(Z)V", access = "public"))]
    pub fn println(&self, x: bool) -> Result<()> {
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
    pub fn println(&self, x: u16) -> Result<()> {
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
    pub fn println(&self, x: i32) -> Result<()> {
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
    pub fn println(&self, x: i64) -> Result<()> {
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
    pub fn println(&self, x: f32) -> Result<()> {
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
    pub fn println(&self, x: f64) -> Result<()> {
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
    pub fn println(&self, x: JvmObject) -> Result<()> {
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
    pub fn println(&self, x: String) -> Result<()> {
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
    pub fn println(&self, x: JvmObject) -> Result<()> {
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
    pub fn printf(&self, format: String, args: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.format(format, args)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "printf", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    pub fn printf(&self, l: JvmObject, format: String, args: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.format(l, format, args)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    pub fn format(&self, format: String, args: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.lock.get().lock()?;
        this.implFormat(format, args)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implFormat(format, args)?;
        /* TODO: monitorexit  */
        let mut local_4: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_4 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/lang/String;[Ljava/lang/Object;)V", access = "private"))]
    pub fn implFormat(&self, format: String, args: JvmObject) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = this.formatter.get().locale()?;
        let _t1: JvmObject = Locale::getDefault(Locale$Category::FORMAT())?;
        this.formatter.set(Formatter::new(this)?);
        let _t2: JvmObject = Locale::getDefault(Locale$Category::FORMAT())?;
        let _t3 = this.formatter.get().format(_t2, format, args)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "format", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;", access = "public"))]
    pub fn format(&self, l: JvmObject, format: String, args: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.lock.get().lock()?;
        this.implFormat(l, format, args)?;
        this.lock.get().unlock()?;
        let mut x: JvmObject = this.lock.get();
        this.lock.get().unlock()?;
        panic!("{}", /* x */);
        x = this;
        /* TODO: monitorenter  */
        this.implFormat(l, format, args)?;
        /* TODO: monitorexit  */
        let mut local_5: JvmObject = x;
        /* TODO: monitorexit  */
        panic!("{}", /* local_5 */);
        x = x;
        let _t0: JvmObject = Thread::currentThread()?;
        _t0.interrupt()?;
        x = this;
        this.trouble.set(1i32);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "implFormat", descriptor = "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)V", access = "private"))]
    pub fn implFormat(&self, l: JvmObject, format: String, args: JvmObject) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let _t0 = this.formatter.get().locale()?;
        this.formatter.set(Formatter::new(this, l)?);
        let _t1 = this.formatter.get().format(l, format, args)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/io/PrintStream;", access = "public"))]
    pub fn append(&self, csq: JvmObject) -> Result<JvmObject> {
        let this = self;
        this.print(String::from_owned(format!("{}", csq)))?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/io/PrintStream;", access = "public"))]
    pub fn append(&self, csq: JvmObject, start: i32, end: i32) -> Result<JvmObject> {
        let this = self;
        csq = String::from("null");
        let _t0 = csq.subSequence(start, end)?;
        let _t1 = this.append(_t0)?;
        Ok(_t1)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/io/PrintStream;", access = "public"))]
    pub fn append(&self, c: u16) -> Result<JvmObject> {
        let this = self;
        this.print(c)?;
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "charset", descriptor = "()Ljava/nio/charset/Charset;", access = "public"))]
    pub fn charset(&self) -> Result<JvmObject> {
        let this = self;
        Ok(this.charset.get())
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: u16) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.append(arg_0)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject, arg_1: i32, arg_2: i32) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.append(arg_0, arg_1, arg_2)?;
        Ok(_t0)
    }

    #[cfg_attr(any(), java_method(name = "append", descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;", access = "public"))]
    pub fn append(&self, arg_0: JvmObject) -> Result<JvmObject> {
        let this = self;
        let _t0 = this.append(arg_0)?;
        Ok(_t0)
    }
}
