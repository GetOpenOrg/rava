#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]
use java_runtime::prelude::*;

#[cfg_attr(any(), java_class(
    binary_name = "java/io/FilterOutputStream",
    super_class = "java/io/OutputStream",
    interfaces  = "",
    access      = "public",
    source      = "FilterOutputStream.java",
))]
pub struct FilterOutputStream {
    #[cfg_attr(any(), java_field(name = "out", descriptor = "Ljava/io/OutputStream;", access = "protected"))]
    pub out: Field<Object>,
    #[cfg_attr(any(), java_field(name = "closed", descriptor = "Z", access = "private"))]
    pub closed: Field<bool>,
    #[cfg_attr(any(), java_field(name = "closeLock", descriptor = "Ljava/lang/Object;", access = "private final"))]
    pub closeLock: Field<Object>,
}

impl FilterOutputStream {
    // java: <init>(Ljava/io/OutputStream;)V
    pub fn new(out: Object) -> Result<Self> {
        let this = Self { out: Field::new(Default::default()), closed: Field::new(false), closeLock: Field::new(Default::default()) };
        /* invokespecial Method java/io/OutputStream.<init>:()V */
        this.closeLock.set(Object::new()?);
        this.out.set(out);
        Ok(this)
    }

    // java: write(I)V
    // java: write(I)V
    pub fn write__i(&self, b: i32) -> Result<()> {
        let this = self;
        this.out.get().write(b)?;
        Ok(())
    }

    // java: write([B)V
    // java: write([B)V
    pub fn write__arr_b(&self, b: Vec<i8>) -> Result<()> {
        let this = self;
        this.write(b, 0i32, (b.len() as i32))?;
        Ok(())
    }

    // java: write([BII)V
    // java: write([BII)V
    pub fn write__arr_b_i_i(&self, b: Vec<i8>, off: i32, len: i32) -> Result<()> {
        let this = self;
        let _t0: i32 = Objects::checkFromIndexSize(off, len, (b.len() as i32))?;
        let mut i: i32 = 0i32;
        loop {
            if i >= len { break; }
            this.write(b[(off).wrapping_add(i) as usize])?;
            i = i.wrapping_add(1i32);
        }
        Ok(())
    }

    // java: flush()V
    pub fn flush(&self) -> Result<()> {
        let this = self;
        this.out.get().flush()?;
        Ok(())
    }

    // java: close()V
    pub fn close(&self) -> Result<()> {
        let this = self;
        return Ok(());
        let mut flushException: Object = this.closeLock.get();
        /* TODO: monitorenter  */
        /* TODO: monitorexit  */
        return Ok(());
        this.closed.set(1i32);
        /* TODO: monitorexit  */
        let mut closeException: Object = flushException;
        /* TODO: monitorexit  */
        return Err(JvmError::Custom("athrow".to_owned()));
        /* TODO: aconst_null  */
        flushException = flushException;
        this.flush()?;
        this.out.get().close()?;
        this.out.get().close()?;
        closeException = flushException;
        closeException.addSuppressed(flushException)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        closeException = closeException;
        flushException = closeException;
        return Err(JvmError::Custom("athrow".to_owned()));
        let mut local_3: Object = flushException;
        this.out.get().close()?;
        this.out.get().close()?;
        let mut closeException: Object = flushException;
        closeException.addSuppressed(flushException)?;
        return Err(JvmError::Custom("athrow".to_owned()));
        return Err(JvmError::Custom("athrow".to_owned()));
        Ok(())
    }
}
