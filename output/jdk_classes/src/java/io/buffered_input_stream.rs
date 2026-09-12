#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/BufferedInputStream",
    super_class = "java/io/FilterInputStream",
    interfaces  = "",
    access      = "public",
    source      = "BufferedInputStream.java",
))]
pub struct BufferedInputStream {
    #[cfg_attr(any(), java_field(name = "lock", descriptor = "Ljdk/internal/misc/InternalLock;", access = "private final"))]
    pub lock: Field<Object>,
    #[cfg_attr(any(), java_field(name = "initialSize", descriptor = "I", access = "private final"))]
    pub initialSize: Field<i32>,
    #[cfg_attr(any(), java_field(name = "buf", descriptor = "[B", access = "protected"))]
    pub buf: Field<Vec<i8>>,
    #[cfg_attr(any(), java_field(name = "count", descriptor = "I", access = "protected"))]
    pub count: Field<i32>,
    #[cfg_attr(any(), java_field(name = "pos", descriptor = "I", access = "protected"))]
    pub pos: Field<i32>,
    #[cfg_attr(any(), java_field(name = "markpos", descriptor = "I", access = "protected"))]
    pub markpos: Field<i32>,
    #[cfg_attr(any(), java_field(name = "marklimit", descriptor = "I", access = "protected"))]
    pub marklimit: Field<i32>,
}

impl BufferedInputStream {
    // java: getInIfOpen()Ljava/io/InputStream;
    pub fn getInIfOpen(&self) -> Result<Object> {
        let this = self;
        let mut input: Object = this.in_.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(input)
    }

    // java: getBufIfOpen(Z)[B
    // java: getBufIfOpen(Z)[B
    pub fn getBufIfOpen__z(&self, allocateIfEmpty: bool) -> Result<Vec<i8>> {
        let this = self;
        let mut buffer: Vec<i8> = this.buf.get();
        let mut _arr0: Vec<i8> = vec![0i8; this.initialSize.get() as usize];
        buffer = _arr0;
        let _t1 = BufferedInputStream::U().compareAndSetReference(this, BufferedInputStream::BUF_OFFSET(), BufferedInputStream::EMPTY(), buffer)?;
        buffer = this.buf.get();
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(buffer)
    }

    // java: getBufIfOpen()[B
    // java: getBufIfOpen()[B
    pub fn getBufIfOpen(&self) -> Result<Vec<i8>> {
        let this = self;
        let _t0 = this.getBufIfOpen(1i32)?;
        Ok(_t0)
    }

    // java: ensureOpen()V
    pub fn ensureOpen(&self) -> Result<()> {
        let this = self;
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: <init>(Ljava/io/InputStream;)V
    // java: <init>(Ljava/io/InputStream;)V
    pub fn new__inputs(in_: Object) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), initialSize: Field::new(0), buf: Field::new(Default::default()), count: Field::new(0), pos: Field::new(0), markpos: Field::new(0), marklimit: Field::new(0) };
        /* invokespecial Method java/io/BufferedInputStream.<init>:(Ljava/io/InputStream;I)V */
        Ok(this)
    }

    // java: <init>(Ljava/io/InputStream;I)V
    // java: <init>(Ljava/io/InputStream;I)V
    pub fn new__inputs_i(in_: Object, size: i32) -> Result<Self> {
        let this = Self { lock: Field::new(Default::default()), initialSize: Field::new(0), buf: Field::new(Default::default()), count: Field::new(0), pos: Field::new(0), markpos: Field::new(0), marklimit: Field::new(0) };
        /* invokespecial Method java/io/FilterInputStream.<init>:(Ljava/io/InputStream;)V */
        this.markpos.set(-1i32);
        return Err(JvmError::Custom("athrow".to_owned()));
        this.initialSize.set(size);
        let _t0 = this.getClass()?;
        let _t1: Object = InternalLock::newLockOrNull()?;
        this.lock.set(_t1);
        this.buf.set(BufferedInputStream::EMPTY());
        /* TODO: aconst_null  */
        2i32.lock.set(this);
        let mut _arr2: Vec<i8> = vec![0i8; size as usize];
        this.buf.set(_arr2);
        Ok(this)
    }

    // java: fill()V
    pub fn fill(&self) -> Result<()> {
        let this = self;
        let _t0 = this.getBufIfOpen()?;
        let mut buffer: Vec<i8> = _t0;
        this.pos.set(0i32);
        let mut sz: i32 = (this.pos.get()).wrapping_sub(this.markpos.get());
        System::arraycopy(&buffer, this.markpos.get(), &buffer, 0i32, sz)?;
        this.pos.set(sz);
        this.markpos.set(0i32);
        this.markpos.set(-1i32);
        this.pos.set(0i32);
        let _t1: i32 = ArraysSupport::newLength(this.pos.get(), 1i32, this.pos.get())?;
        sz = _t1;
        sz = this.marklimit.get();
        let mut _arr2: Vec<i8> = vec![0i8; sz as usize];
        let mut nbuf: Vec<i8> = _arr2;
        System::arraycopy(&buffer, 0i32, &nbuf, 0i32, this.pos.get())?;
        let _t3 = BufferedInputStream::U().compareAndSetReference(this, BufferedInputStream::BUF_OFFSET(), buffer, nbuf)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        buffer = nbuf;
        this.count.set(this.pos.get());
        let _t4 = this.getInIfOpen()?;
        let _t5 = _t4.read(buffer, this.pos.get(), ((buffer.len() as i32)).wrapping_sub(this.pos.get()))?;
        sz = _t5;
        this.count.set((sz).wrapping_add(this.pos.get()));
        Ok(())
    }

    // java: read()I
    // java: read()I
    pub fn read(&self) -> Result<i32> {
        let this = self;
        this.lock.get().lock()?;
        let _t0 = this.implRead()?;
        let mut local_1: i32 = _t0;
        this.lock.get().unlock()?;
        return Ok(local_1);
        let mut local_2: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_1 = this;
        /* TODO: monitorenter  */
        let _t1 = this.implRead()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_3: i32 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: implRead()I
    // java: implRead()I
    pub fn implRead(&self) -> Result<i32> {
        let this = self;
        this.fill()?;
        return Ok(-1i32);
        let _t0 = this.getBufIfOpen()?;
        this.pos.set((this.pos.get()).wrapping_add(1i32));
        Ok((_t0[this.pos.get() as usize]&255i32))
    }

    // java: read1([BII)I
    pub fn read1(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        let mut avail: i32 = (this.count.get()).wrapping_sub(this.pos.get());
        let _t0 = this.getBufIfOpen(0i32)?;
        let _t1: i32 = ((_t0.len() as i32)).max(this.initialSize.get());
        let mut size: i32 = _t1;
        let _t2 = this.getInIfOpen()?;
        let _t3 = _t2.read(b, off, len)?;
        return Ok(_t3);
        this.fill()?;
        avail = (this.count.get()).wrapping_sub(this.pos.get());
        return Ok(-1i32);
        size = len;
        let _t4 = this.getBufIfOpen()?;
        System::arraycopy(&_t4, this.pos.get(), &b, off, size)?;
        this.pos.set((this.pos.get()).wrapping_add(size));
        Ok(size)
    }

    // java: read([BII)I
    // java: read([BII)I
    pub fn read__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        this.lock.get().lock()?;
        let _t0 = this.implRead(b, off, len)?;
        let mut local_4: i32 = _t0;
        this.lock.get().unlock()?;
        return Ok(local_4);
        let mut local_5: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_4 = this;
        /* TODO: monitorenter  */
        let _t1 = this.implRead(b, off, len)?;
        /* TODO: monitorexit  */
        return Ok(local_4);
        let mut local_6: i32 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: implRead([BII)I
    // java: implRead([BII)I
    pub fn implRead__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<i32> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Ok(0i32);
        let mut n: i32 = 0i32;
        let _t0 = this.read1(b, (off).wrapping_add(n), (len).wrapping_sub(n))?;
        let mut nread: i32 = _t0;
        return Ok(n);
        n = (n).wrapping_add(nread);
        return Ok(n);
        let mut input: Object = this.in_.get();
        let _t1 = input.available()?;
        Ok(n)
    }

    // java: skip(J)J
    pub fn skip(&self, n: i64) -> Result<i64> {
        let this = self;
        this.lock.get().lock()?;
        let _t0 = this.implSkip(n)?;
        let mut local_3: i64 = _t0;
        this.lock.get().unlock()?;
        return Ok(local_3);
        let mut local_5: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_3 = this;
        /* TODO: monitorenter  */
        let _t1 = this.implSkip(n)?;
        /* TODO: monitorexit  */
        return Ok(local_3);
        let mut local_6: i64 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: implSkip(J)J
    pub fn implSkip(&self, n: i64) -> Result<i64> {
        let this = self;
        this.ensureOpen()?;
        /* TODO: lcmp  */
        return Ok(0i64);
        let mut avail: i64 = ((this.count.get()).wrapping_sub(this.pos.get()) as i64);
        /* TODO: lcmp  */
        let _t0 = this.getInIfOpen()?;
        let _t1 = _t0.skip(n)?;
        return Ok(_t1);
        this.fill()?;
        avail = ((this.count.get()).wrapping_sub(this.pos.get()) as i64);
        /* TODO: lcmp  */
        return Ok(0i64);
        /* TODO: lcmp  */
        let mut skipped: i64 = n;
        this.pos.set((this.pos.get()).wrapping_add((skipped as i32)));
        Ok(skipped)
    }

    // java: available()I
    pub fn available(&self) -> Result<i32> {
        let this = self;
        this.lock.get().lock()?;
        let _t0 = this.implAvailable()?;
        let mut local_1: i32 = _t0;
        this.lock.get().unlock()?;
        return Ok(local_1);
        let mut local_2: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_1 = this;
        /* TODO: monitorenter  */
        let _t1 = this.implAvailable()?;
        /* TODO: monitorexit  */
        return Ok(local_1);
        let mut local_3: i32 = _t1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: implAvailable()I
    pub fn implAvailable(&self) -> Result<i32> {
        let this = self;
        let mut n: i32 = (this.count.get()).wrapping_sub(this.pos.get());
        let _t0 = this.getInIfOpen()?;
        let _t1 = _t0.available()?;
        let mut avail: i32 = _t1;
        Ok((n).wrapping_add(avail))
    }

    // java: mark(I)V
    pub fn mark(&self, readlimit: i32) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implMark(readlimit)?;
        this.lock.get().unlock()?;
        let mut local_2: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_2 = this;
        /* TODO: monitorenter  */
        this.implMark(readlimit)?;
        /* TODO: monitorexit  */
        let mut local_3: Object = local_2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implMark(I)V
    pub fn implMark(&self, readlimit: i32) -> Result<()> {
        let this = self;
        this.marklimit.set(readlimit);
        this.markpos.set(this.pos.get());
        Ok(())
    }

    // java: reset()V
    pub fn reset(&self) -> Result<()> {
        let this = self;
        this.lock.get().lock()?;
        this.implReset()?;
        this.lock.get().unlock()?;
        let mut local_1: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_1 = this;
        /* TODO: monitorenter  */
        this.implReset()?;
        /* TODO: monitorexit  */
        let mut local_2: Object = local_1;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }

    // java: implReset()V
    pub fn implReset(&self) -> Result<()> {
        let this = self;
        this.ensureOpen()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        this.pos.set(this.markpos.get());
        Ok(())
    }

    // java: markSupported()Z
    pub fn markSupported(&self) -> Result<bool> {
        let this = self;
        Ok(1i32)
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        let mut buffer: Vec<i8> = this.buf.get();
        /* TODO: aconst_null  */
        let _t0 = this.buf.get().compareAndSetReference(BufferedInputStream::U(), this, BufferedInputStream::BUF_OFFSET(), buffer)?;
        let mut input: Object = this.in_.get();
        /* TODO: aconst_null  */
        _t0.in_.set(this);
        input.close()?;
        return Ok(());
    }

    // java: transferTo(Ljava/io/OutputStream;)J
    pub fn transferTo(&self, out: Object) -> Result<i64> {
        let this = self;
        let _t0: Object = Objects::requireNonNull(out, String::from("out"))?;
        this.lock.get().lock()?;
        let _t1 = this.implTransferTo(out)?;
        let mut local_2: i64 = _t1;
        this.lock.get().unlock()?;
        return Ok(local_2);
        let mut local_4: Object = this.lock.get();
        this.lock.get().unlock()?;
        return Err(JvmError::Custom("athrow".to_owned()));
        local_2 = this;
        /* TODO: monitorenter  */
        let _t2 = this.implTransferTo(out)?;
        /* TODO: monitorexit  */
        return Ok(local_2);
        let mut local_5: i64 = _t2;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
    }

    // java: implTransferTo(Ljava/io/OutputStream;)J
    pub fn implTransferTo(&self, out: Object) -> Result<i64> {
        let this = self;
        let _t0 = this.getClass()?;
        let mut avail: i32 = (this.count.get()).wrapping_sub(this.pos.get());
        let _t1 = this.getBufIfOpen()?;
        let _t2: Vec<i8> = Arrays::copyOfRange(&_t1, this.pos.get(), this.count.get())?;
        let mut buffer: Vec<i8> = _t2;
        out.write(buffer)?;
        this.pos.set(this.count.get());
        let _t3 = this.getInIfOpen()?;
        let _t4 = _t3.transferTo(out)?;
        let _t5: i64 = ((avail as i64)).abs();
        return Ok(_t5);
        buffer = avail;
        return Ok(9223372036854775807i64);
        let _t6: i64 = FilterInputStream::transferTo(out)?;
        Ok(_t6)
    }
}
