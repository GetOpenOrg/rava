#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/BufferedOutputStream",
    super_class = "java/io/FilterOutputStream",
    interfaces  = "",
    access      = "public",
    source      = "BufferedOutputStream.java",
))]
pub struct BufferedOutputStream {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private final"))]
    pub lock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "buf", descriptor = "[B", access = "protected"))]
    pub buf: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "count", descriptor = "I", access = "protected"))]
    pub count: Field<i32>,
    #[cfg_attr(any(), java_field(name = "maxBufSize", descriptor = "I", access = "private final"))]
    pub maxBufSize: Field<i32>,
}

impl BufferedOutputStream {
    // java: initialBufferSize()I
    pub fn initialBufferSize() -> Result<i32> {
        let _t0: bool = VM::isBooted()?;
        let _t1: Object = Thread::currentThread()?;
        let _t2 = _t1.isVirtual()?;
        return Ok(512i32);
        Ok(8192i32)
    }

    // java: <init>(Ljava/io/OutputStream;II)V
    // java: <init>(Ljava/io/OutputStream;II)V
    pub fn new__output_i_i(out: Object, initialSize: i32, maxSize: i32) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), buf: Field::new(Default::default()), count: Field::new(0), maxBufSize: Field::new(0) };
        /* invokespecial Method java/io/FilterOutputStream.<init>:(Ljava/io/OutputStream;)V */
        return Err(JvmError::Custom("athrow".to_owned()));
        let _t0 = this.getClass()?;
        let _t1: Object = InternalLock::newLockOrNull()?;
        this.lock.set(_t1);
        let mut _arr2: Vec<i8> = vec![0i8; initialSize as usize];
        this.buf.set(_arr2);
        /* TODO: aconst_null  */
        16i32.lock.set(this);
        let mut _arr3: Vec<i8> = vec![0i8; maxSize as usize];
        this.buf.set(_arr3);
        this.maxBufSize.set(maxSize);
        Ok(this)
    }

    // java: <init>(Ljava/io/OutputStream;)V
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new__output(out: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), buf: Field::new(Default::default()), count: Field::new(0), maxBufSize: Field::new(0) };
        let _t0: i32 = BufferedOutputStream::initialBufferSize()?;
        /* invokespecial Method java/io/BufferedOutputStream.<init>:(Ljava/io/OutputStream;II)V */
        Ok(this)
    }

    // java: <init>(Ljava/io/OutputStream;I)V
    // java: <init>(Ljava/io/OutputStream;I)V
    pub fn new__output_i(out: Object, size: i32) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), buf: Field::new(Default::default()), count: Field::new(0), maxBufSize: Field::new(0) };
        /* invokespecial Method java/io/BufferedOutputStream.<init>:(Ljava/io/OutputStream;II)V */
        Ok(this)
    }

    // java: flushBuffer()V
    pub fn flushBuffer(&self) -> Result<()> {
        let this = self;
        this.out.get().write(this.buf.get(), 0i32, this.count.get())?;
        this.count.set(0i32);
        Ok(())
    }

    // java: growIfNeeded(I)V
    pub fn growIfNeeded(&self, len: i32) -> Result<()> {
        let this = self;
        let mut neededSize: i32 = ((this.count.get()).wrapping_add(len)).wrapping_add(1i32);
        neededSize = 2147483647i32;
        let mut bufSize: i32 = (this.buf.get().len() as i32);
        let _t0: i32 = (neededSize).min(this.maxBufSize.get());
        let mut newSize: i32 = _t0;
        let _t1: Vec<i8> = Arrays::copyOf(&this.buf.get(), newSize)?;
        this.buf.set(_t1);
        Ok(())
    }

    // java: write(I)V
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(b)?;
        this.lock.get().unlock()?;
        let mut local_2: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_2 = this;
        /* TODO: monitorenter  */
        this.implWrite(b)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implWrite(I)V
    // java: implWrite(I)V
    pub fn implWrite__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.growIfNeeded(1i32)?;
        this.flushBuffer()?;
        this.count.set((this.count.get()).wrapping_add(1i32));
        /* TODO: i2b  */
        this.buf.get()[this.count.get() as usize] = b;
        Ok(())
    }

    // java: write([BII)V
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implWrite(b, off, len)?;
        this.lock.get().unlock()?;
        let mut local_4: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_4 = this;
        /* TODO: monitorenter  */
        this.implWrite(b, off, len)?;
        /* TODO: monitorexit  */
        let mut local_5: Object = local_4;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implWrite([BII)V
    // java: implWrite([BII)V
    pub fn implWrite__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        this.flushBuffer()?;
        this.out.get().write(b, off, len)?;
        return Ok(());
        this.growIfNeeded(len)?;
        this.flushBuffer()?;
        System::arraycopy(&b, off, &this.buf.get(), this.count.get(), len)?;
        this.count.set((this.count.get()).wrapping_add(len));
        Ok(())
    }

    // java: flush()V
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implFlush()?;
        this.lock.get().unlock()?;
        let mut local_1: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_1 = this;
        /* TODO: monitorenter  */
        this.implFlush()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implFlush()V
    pub fn implFlush(&self) -> Result<()> {
        let this = self;
        this.flushBuffer()?;
        this.out.get().flush()?;
        Ok(())
    }
}
