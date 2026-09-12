#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/BufferedWriter",
    super_class = "java/io/Writer",
    interfaces  = "",
    access      = "public",
    source      = "BufferedWriter.java",
))]
pub struct BufferedWriter {
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/io/Writer;", access = "private"))]
    pub out: Field<Object>,
    #[cfg_attr(any(), java_field(name = "cb", descriptor = "[C", access = "private"))]
    pub cb: Field<Vec<u16>>,
    #[cfg_attr(any(), java_field(name = "nChars", descriptor = "I", access = "private"))]
    pub nChars: Field<i32>,
    #[cfg_attr(any(), java_field(name = "nextChar", descriptor = "I", access = "private"))]
    pub nextChar: Field<i32>,
    #[cfg_attr(any(), java_field(name = "maxChars", descriptor = "I", access = "private final"))]
    pub maxChars: Field<i32>,
}

impl BufferedWriter {
    #[cfg_attr(any(), java_method(name = "initialBufferSize", descriptor = "()I", access = "private static"))]
    pub fn initialBufferSize() -> Result<i32> {
        let _t0: bool = VM::isBooted()?;
        let _t1: Object = Thread::currentThread()?;
        let _t2 = _t1.isVirtual()?;
        return Ok(512i32);
        Ok(8192i32)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;II)V", access = "private"))]
    // java: <init>(Ljava/io/Writer;II)V
    pub fn new__writer_i_i(out: Object, initialSize: i32, maxSize: i32) -> Result<Self> {
        let this = Self { out: Field::new(Default::default()), cb: Field::new(Default::default()), nChars: Field::new(0), nextChar: Field::new(0), maxChars: Field::new(0) };
        /* invokespecial Method java/io/Writer.<init>:(Ljava/io/Writer;)V */
        return Err(JvmError::Custom(String::from("athrow")));
        this.out.set(out);
        let mut _arr0: Vec<u16> = vec![0u16; initialSize as usize];
        this.cb.set(_arr0);
        this.nChars.set(initialSize);
        this.maxChars.set(maxSize);
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;)V", access = "public"))]
    // java: <init>(Ljava/io/Writer;)V
    pub fn new__writer(out: Object) -> Result<Self> {
        let this = Self { out: Field::new(Default::default()), cb: Field::new(Default::default()), nChars: Field::new(0), nextChar: Field::new(0), maxChars: Field::new(0) };
        let _t0: i32 = BufferedWriter::initialBufferSize()?;
        /* invokespecial Method java/io/BufferedWriter.<init>:(Ljava/io/Writer;II)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "<init>", descriptor = "(Ljava/io/Writer;I)V", access = "public"))]
    // java: <init>(Ljava/io/Writer;I)V
    pub fn new__writer_i(out: Object, sz: i32) -> Result<Self> {
        let this = Self { out: Field::new(Default::default()), cb: Field::new(Default::default()), nChars: Field::new(0), nextChar: Field::new(0), maxChars: Field::new(0) };
        /* invokespecial Method java/io/BufferedWriter.<init>:(Ljava/io/Writer;II)V */
        Ok(this)
    }

    #[cfg_attr(any(), java_method(name = "ensureOpen", descriptor = "()V", access = "private"))]
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "growIfNeeded", descriptor = "(I)V", access = "private"))]
    pub fn growIfNeeded(&self, len: i32) -> Result<()> {
        let this = self;
        let mut neededSize: i32 = ((this.nextChar.get()).wrapping_add(len)).wrapping_add(1i32);
        neededSize = 2147483647i32;
        let _t0 = this.min(neededSize, this.maxChars.get())?;
        let mut newSize: i32 = _t0;
        let _t1: Vec<u16> = Arrays::copyOf(&this.cb.get(), newSize)?;
        this.cb.set(_t1);
        this.nChars.set(newSize);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flushBuffer", descriptor = "()V"))]
    pub fn flushBuffer(&self) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implFlushBuffer()?;
        locker.unlock()?;
        let mut local_3: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_3 = lock;
        /* TODO: monitorenter  */
        this.implFlushBuffer()?;
        /* TODO: monitorexit  */
        let mut local_4: bool = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implFlushBuffer", descriptor = "()V", access = "private"))]
    pub fn implFlushBuffer(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        return Ok(());
        this.out.get().write(this.cb.get(), 0i32, this.nextChar.get())?;
        this.nextChar.set(0i32);
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(I)V", access = "public"))]
    // java: write(I)V
    pub fn write__i(&self, c: i32) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implWrite(c)?;
        locker.unlock()?;
        let mut local_4: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_4 = lock;
        /* TODO: monitorenter  */
        this.implWrite(c)?;
        /* TODO: monitorexit  */
        let mut local_5: bool = local_4;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(I)V", access = "private"))]
    // java: implWrite(I)V
    pub fn implWrite__i(&self, c: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.growIfNeeded(1i32)?;
        this.flushBuffer()?;
        this.nextChar.set((this.nextChar.get()).wrapping_add(1i32));
        /* TODO: i2c  */
        this.cb.get()[this.nextChar.get() as usize] = c;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "min", descriptor = "(II)I", access = "private"))]
    pub fn min(&self, a: i32, b: i32) -> Result<i32> {
        let this = self;
        return Ok(a);
        Ok(b)
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "([CII)V", access = "public"))]
    // java: write([CII)V
    pub fn write__arr_c_i_i(&self, cbuf: Vec<u16>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implWrite(cbuf, off, len)?;
        locker.unlock()?;
        let mut local_6: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_6 = lock;
        /* TODO: monitorenter  */
        this.implWrite(cbuf, off, len)?;
        /* TODO: monitorexit  */
        let mut local_7: bool = local_6;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "([CII)V", access = "private"))]
    // java: implWrite([CII)V
    pub fn implWrite__arr_c_i_i(&self, cbuf: Vec<u16>, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (cbuf.len() as i32))?;
        return Ok(());
        this.flushBuffer()?;
        this.out.get().write(cbuf, off, len)?;
        return Ok(());
        this.growIfNeeded(len)?;
        let mut b: i32 = off;
        let mut t: i32 = (off).wrapping_add(len);
        loop {
            if b >= t { break; }
            let _t0 = this.min((this.nChars.get()).wrapping_sub(this.nextChar.get()), (t).wrapping_sub(b))?;
            let mut d: i32 = _t0;
            System::arraycopy(&cbuf, b, &this.cb.get(), this.nextChar.get(), d)?;
            b = (b).wrapping_add(d);
            this.nextChar.set((this.nextChar.get()).wrapping_add(d));
            this.flushBuffer()?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "write", descriptor = "(Ljava/lang/String;II)V", access = "public"))]
    // java: write(Ljava/lang/String;II)V
    pub fn write__str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implWrite(s, off, len)?;
        locker.unlock()?;
        let mut local_6: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_6 = lock;
        /* TODO: monitorenter  */
        this.implWrite(s, off, len)?;
        /* TODO: monitorexit  */
        let mut local_7: bool = local_6;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implWrite", descriptor = "(Ljava/lang/String;II)V", access = "private"))]
    // java: implWrite(Ljava/lang/String;II)V
    pub fn implWrite__str_i_i(&self, s: String, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        this.growIfNeeded(len)?;
        let mut b: i32 = off;
        let mut t: i32 = (off).wrapping_add(len);
        loop {
            if b >= t { break; }
            let _t0 = this.min((this.nChars.get()).wrapping_sub(this.nextChar.get()), (t).wrapping_sub(b))?;
            let mut d: i32 = _t0;
            s.getChars(b, (b).wrapping_add(d), this.cb.get(), this.nextChar.get())?;
            b = (b).wrapping_add(d);
            this.nextChar.set((this.nextChar.get()).wrapping_add(d));
            this.flushBuffer()?;
        }
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "newLine", descriptor = "()V", access = "public"))]
    pub fn newLine(&self) -> Result<()> {
        let this = self;
        let _t0: String = System::lineSeparator()?;
        this.write(_t0)?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "flush", descriptor = "()V", access = "public"))]
    pub fn flush(&self) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implFlush()?;
        locker.unlock()?;
        let mut local_3: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_3 = lock;
        /* TODO: monitorenter  */
        this.implFlush()?;
        /* TODO: monitorexit  */
        let mut local_4: bool = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implFlush", descriptor = "()V", access = "private"))]
    pub fn implFlush(&self) -> Result<()> {
        let this = self;
        this.flushBuffer()?;
        this.out.get().flush()?;
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "close", descriptor = "()V", access = "public"))]
    pub fn close(&self) -> Result<()> {
        let this = self;
        let mut lock: Object = this.lock.get();
        let mut locker: Object = lock;
        locker.lock()?;
        this.implClose()?;
        locker.unlock()?;
        let mut local_3: bool = true;
        locker.unlock()?;
        return Err(JvmError::Custom(String::from("athrow")));
        local_3 = lock;
        /* TODO: monitorenter  */
        this.implClose()?;
        /* TODO: monitorexit  */
        let mut local_4: bool = local_3;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }

    #[cfg_attr(any(), java_method(name = "implClose", descriptor = "()V", access = "private"))]
    pub fn implClose(&self) -> Result<()> {
        let this = self;
        return Ok(());
        let mut w: Object = this.out.get();
        this.flushBuffer()?;
        w.close()?;
        let mut local_2: Object = w;
        w.close()?;
        let mut local_3: Object = w;
        local_2.addSuppressed(local_3)?;
        return Err(JvmError::Custom(String::from("athrow")));
        /* TODO: aconst_null  */
        this.out.get().out.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").cb.set(this);
        let mut local_4: i32 = todo!("stack underflow");
        /* TODO: aconst_null  */
        todo!("stack underflow").out.set(this);
        /* TODO: aconst_null  */
        todo!("stack underflow").cb.set(this);
        return Err(JvmError::Custom(String::from("athrow")));
        Ok(())
    }
}
